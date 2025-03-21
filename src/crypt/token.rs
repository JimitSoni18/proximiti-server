// deps
use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::Mac as _;
use time::{Duration, OffsetDateTime};

// internal
use crate::config::CONFIG;

pub mod error {
	pub type Result<T> = core::result::Result<T, Error>;
	pub enum Error {
		VerificationFailed,
		TokenExpired,
		Other,
	}
	impl From<serde_json::Error> for Error {
		fn from(_: serde_json::Error) -> Self {
			Self::VerificationFailed
		}
	}
	impl From<hmac::digest::MacError> for Error {
		fn from(_: hmac::digest::MacError) -> Self {
			Self::VerificationFailed
		}
	}
	impl From<hmac::digest::crypto_common::InvalidLength> for Error {
		fn from(_: hmac::digest::crypto_common::InvalidLength) -> Self {
			Self::Other
		}
	}
}

type BlakeMac = hmac::SimpleHmac<blake3::Hasher>;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
	#[serde(with = "time::serde::timestamp")]
	exp: OffsetDateTime,
	user_id: i64,
}

fn get_mac(token_str: &str) -> error::Result<BlakeMac> {
	let mut mac = BlakeMac::new_from_slice(CONFIG.token_secret.as_bytes())?;
	mac.update(token_str.as_bytes());
	Ok(mac)
}

#[tracing::instrument(skip_all)]
pub fn create_token(user_id: i64) -> error::Result<String> {
	let claims = STANDARD.encode(
		serde_json::to_string(&Claims {
			exp: OffsetDateTime::now_utc()
				.saturating_add(Duration::seconds(CONFIG.token_duration_sec.into())),
			user_id,
		})?
		.as_bytes(),
	);
	let mac = get_mac(&claims)?;
	let sign = STANDARD.encode(mac.finalize().into_bytes());
	Ok(format!("{claims}.{sign}"))
}

pub fn verify_token(token: &str) -> error::Result<i64> {
	let (claims, sign) = token
		.split_once('.')
		.ok_or(error::Error::VerificationFailed)?;

	let mac = get_mac(claims)?;
	mac.verify_slice(sign.as_bytes())?;

	let token = serde_json::from_str::<Claims>(claims)?;

	if token.exp >= OffsetDateTime::now_utc() {
		return Err(error::Error::TokenExpired);
	}

	Ok(token.user_id)
}
