use std::sync::OnceLock;

use error::{Error, Result};

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(
			|Error::ConfigMissingEnv(name)| {
				panic!("FATAL - config missing environment: {name}");
			},
		)
	})
}

#[allow(non_snake_case)]
pub struct Config {
	pub STATIC_DIR: String,
	pub DB_URL: String,
}

impl Config {
	fn load_from_env() -> Result<Self> {
		Ok(Self {
			STATIC_DIR: get_env("SERVICE_STATIC_DIR")?,
			DB_URL: get_env("SERVICE_DB_URL")?,
		})
	}
}

fn get_env(name: &'static str) -> Result<String> {
	std::env::var(name).map_err(|_| Error::ConfigMissingEnv(name.to_string()))
}

mod error {
	pub type Result<T> = core::result::Result<T, Error>;

	pub enum Error {
		ConfigMissingEnv(String),
	}
}
