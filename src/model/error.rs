use super::store;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
	// -- Modules
	Store(store::Error),
}

impl From<store::Error> for Error {
	fn from(value: store::Error) -> Self {
		Self::Store(value)
	}
}
