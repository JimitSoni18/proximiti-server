use std::{marker::PhantomData, sync::LazyLock};

#[allow(unused)]
pub struct Config {
	pub db_url: String,
	pub s3_url: String,
	pub cache_url: String,
	pub pwd_key: String,
	pub token_duration_sec: u16,
	pub web_folder: String,
	pub token_secret: String,
	_priv: PhantomData<()>,
}

impl Config {
    pub fn check(&self) -> PhantomData<()> {
        self._priv
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
	let mut db_url: Option<String> = None;
	let mut s3_url: Option<String> = None;
	let mut cache_url: Option<String> = None;
	let mut pwd_key: Option<String> = None;
	let mut token_duration_sec: Option<u16> = None;
	let mut web_folder: Option<String> = None;
	let mut token_secret: Option<String> = None;
	for item in dotenvy::dotenv_iter().unwrap() {
		let (key, val) = item.unwrap();
		match key.as_str() {
			"DATABASE_URL" => db_url = Some(val),
			"S3_URL" => s3_url = Some(val),
			"CACHE_URL" => cache_url = Some(val),
			"PASSWORD_SECRET" => pwd_key = Some(val),
            "HMAC_SECRET" => token_secret = Some(val),
			"TOKEN_DURATION_SECONDS" => token_duration_sec = Some(val.parse().unwrap()),
			"WEB_FOLDER" => web_folder = Some(val),
			_ => {}
		}
	}
	Config {
		db_url: db_url.unwrap(),
		s3_url: s3_url.unwrap(),
		cache_url: cache_url.unwrap(),
		pwd_key: pwd_key.unwrap(),
		token_duration_sec: token_duration_sec.unwrap(),
		web_folder: web_folder.unwrap(),
		token_secret: token_secret.unwrap(),
		_priv: PhantomData,
	}
});
