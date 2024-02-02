use crate::crypt;

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

// Debug for test
#[derive(Debug)]
pub enum Error {
	EntityNotFound { entity: &'static str, id: i64 },

	// -- Modules
	Store(store::Error),
	Crypt(crypt::Error),

	// -- Externals
	Sqlx(sqlx::Error),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl From<sqlx::Error> for Error {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}

impl From<store::Error> for Error {
	fn from(value: store::Error) -> Self {
		Self::Store(value)
	}
}

impl From<crypt::Error> for Error {
	fn from(value: crypt::Error) -> Self {
		Self::Crypt(value)
	}
}

impl std::error::Error for Error {}
