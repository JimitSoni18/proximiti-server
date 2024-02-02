pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	DateFailParse(String),
	FailToB64uDecode,
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}
