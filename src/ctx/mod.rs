mod error;

use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
pub use error::{Error, Result};

#[derive(Clone)]
pub struct Ctx {
	user_id: i64,
}

// Constructor
impl Ctx {
	pub fn root_ctx() -> Self {
		Self { user_id: 0 }
	}
	pub fn new(user_id: i64) -> Result<Self> {
		if user_id == 0 {
			return Err(Error::CtxCannotNewRootCtx);
		} else {
			Ok(Self { user_id })
		}
	}
}

// Accessor
impl Ctx {
	fn user_id(&self) -> i64 {
		self.user_id
	}
}
