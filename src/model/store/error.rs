pub type Result<T> = core::result::Result<T, Error>;

// Debug for test
#[derive(Debug)]
pub enum Error {
	FailedToCreatePool(String),
}
