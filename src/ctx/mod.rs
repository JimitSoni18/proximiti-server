mod error;

use error::{Error, Result};

pub struct Ctx {
	user_id: i64,
}

impl Ctx {
	pub fn new(user_id: i64) -> Result<Self> {
		Ok(Self { user_id })
	}
}
