pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
	InternalError,
	UsernamePasswordError,
    UsernameTaken,
	UnableToCreateUser,
}
