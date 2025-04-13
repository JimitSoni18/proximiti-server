// std
use std::sync::LazyLock;

// deps
use argon2::{
	password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
	Argon2, PasswordHash,
};

pub static ARGON2_INSTANCE: LazyLock<Argon2> = LazyLock::new(Argon2::default);

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
	InvalidPassword,
	Other,
}

pub fn hash_password(password: impl AsRef<[u8]>) -> Result<String> {
	let salt = SaltString::generate(&mut OsRng);
	ARGON2_INSTANCE
		.hash_password(password.as_ref(), &salt)
		.map(|hash| hash.to_string())
		.map_err(|_| Error::Other)
}

// TODO: take solid type instead of impl AsRef
pub fn verify_password(password: impl AsRef<[u8]>, password_hash: &str) -> Result<()> {
	let hash = PasswordHash::new(password_hash)?;
	ARGON2_INSTANCE.verify_password(password.as_ref(), &hash)?;
	Ok(())
}

impl From<argon2::password_hash::Error> for Error {
	// TODO: trace
	fn from(value: argon2::password_hash::Error) -> Self {
		match value {
			argon2::password_hash::Error::Password => Self::InvalidPassword,
			argon2::password_hash::Error::B64Encoding(_) => Self::Other,
			why => Self::Other,
		}
	}
}
