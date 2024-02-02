pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_login;
pub mod routes_static;

mod error;

pub use error::{Error, Result};
use tower_cookies::{Cookie, Cookies};

use crate::crypt::token::generate_web_token;

pub const AUTH_TOKEN: &'static str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
	let token = generate_web_token(user, salt)?;

	let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
	cookie.set_http_only(true);
	cookie.set_path("/");
	cookies.add(cookie);

	Ok(())
}

fn remove_token_cookie() {}
