use std::{convert::Infallible, sync::Arc};

use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};
use tracing::instrument;

pub mod error;

use crate::{
	crypt::{
		pwd::{hash_password, verify_password},
		token::create_token,
	},
	models::sql,
	AppState,
};

pub struct UserService(sql::Db);

pub struct AuthUser {
	id: i64,
	password: String,
	username: String,
	profile_picture_url: Option<String>,
}

#[derive(Serialize)]
pub struct ListUser {
	username: String,
	#[serde(rename = "camelCase", skip_serializing_if = "Option::is_none")]
	profile_picture_url: Option<String>,
	id: i64,
}

#[derive(Serialize)]
pub struct ListOnlineUser {
	#[serde(rename = "camelCase")]
	conversation_id: i64,
	username: String,
	#[serde(rename = "camelCase", skip_serializing_if = "Option::is_none")]
	profile_picture_url: Option<String>,
	id: i64,
}

impl AuthUser {
	#[inline]
	fn into_authenticated_user(self, token: String) -> AuthenticatedUser {
		let AuthUser {
			profile_picture_url,
			username,
			id,
			..
		} = self;
		AuthenticatedUser {
			id,
			username,
			token,
			profile_picture_url,
		}
	}
}

#[derive(Deserialize)]
pub struct User {
	pub username: String,
	pub password: String,
}

#[derive(Serialize)]
pub struct AuthenticatedUser {
	id: i64,
	token: String,
	username: String,
	#[serde(rename = "camelCase", skip_serializing_if = "Option::is_none")]
	profile_picture_url: Option<String>,
}

impl UserService {
	#[inline]
	pub async fn verify_password(&self, user: User) -> error::Result<AuthenticatedUser> {
		let auth_user = sqlx::query_as!(
			AuthUser,
			"SELECT id, password, username, profile_picture_url from users where username = $1",
			user.username
		)
		.fetch_one(&self.0)
		.await
		.map_err(|why| match why {
			sqlx::Error::RowNotFound => error::Error::UsernamePasswordError,
			_ => error::Error::InternalError,
		})?;
		// TODO: trace
		verify_password(&user.password, &auth_user.password).map_err(|why| match why {
			crate::crypt::pwd::Error::InvalidPassword => error::Error::UsernamePasswordError,
			crate::crypt::pwd::Error::Other => error::Error::InternalError,
		})?;

		// TODO: trace
		let token = create_token(auth_user.id).map_err(|_| error::Error::InternalError)?;

		Ok(auth_user.into_authenticated_user(token))
	}

	#[inline]
	pub async fn add_user(&self, user: User) -> error::Result<AuthenticatedUser> {
		let hashed_pwd = hash_password(&user.password).map_err(|_| error::Error::InternalError)?;

		let exists = sqlx::query_scalar!(
			r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1) as "bool!""#,
			user.username
		)
		.fetch_one(&self.0)
		.await
		.map_err(|_| error::Error::InternalError)?;

		if exists {
			return Err(error::Error::UsernameTaken);
		}

		struct NewUser {
			id: i64,
			profile_picture_url: Option<String>,
			username: String,
		}

		let new_user = sqlx::query_as!(
			NewUser,
			"INSERT INTO users (username, password) values ($1, $2) RETURNING id, profile_picture_url, username",
			user.username,
			hashed_pwd,
		)
		.fetch_one(&self.0)
		.await
		.map_err(|_| error::Error::UnableToCreateUser)?;

		// TODO: trace
		let token = create_token(new_user.id).map_err(|_| error::Error::InternalError)?;

		let NewUser {
			username,
			profile_picture_url,
			id,
		} = new_user;

		Ok(AuthenticatedUser {
			id,
			profile_picture_url,
			username,
			token,
		})
	}

	#[inline]
	pub async fn list_online(&self, current_user_id: i64) -> error::Result<Vec<ListOnlineUser>> {
		let _ = sqlx::query_as!(
			ListOnlineUser,
			r#"
SELECT
	uc.id as conversation_id,
	u.id,
	u.username,
	u.profile_picture_url
FROM user_conversations uc
JOIN users u
ON u.id = CASE
	WHEN uc.user1_id = $1 THEN user2_id
	ELSE user1_id
END
WHERE (
	uc.user1_id = $1
	OR uc.user2_id = $1
)
AND u.online
AND NOT EXISTS (
	SELECT
		1
	FROM user_conversation_blocks
	WHERE blocked_user = $1
	AND blocked_by_user = u.id
)
"#,
			current_user_id,
		);
		todo!()
	}

	#[inline]
	#[instrument(skip(self))]
	pub async fn search_by_username(
		&self,
		current_user_id: i64,
		mut search: String,
	) -> error::Result<Vec<ListUser>> {
		search.push('%');
		sqlx::query_as!(
			ListUser,
			r#"
SELECT
	username, profile_picture_url, id
FROM users
WHERE username LIKE $1
AND id != $2
AND NOT EXISTS (
	SELECT
		1
	FROM user_conversations
	WHERE (user1_id = $2 AND user2_id = users.id)
	OR (user1_id = users.id AND user2_id = $2)
)
AND NOT EXISTS (
	SELECT
		1
	FROM user_conversation_blocks
	WHERE blocked_user = $2
	AND blocked_by_user = users.id
)
AND NOT EXISTS (
	SELECT
		1
	FROM user_requests
	WHERE sender_id = $2
)
"#,
			search,
			current_user_id,
		)
		.fetch_all(&self.0)
		.await
		.map_err(|why| {
			tracing::error!(%why, "friend-query-failed");
			error::Error::InternalError
		})
	}
}

impl FromRequestParts<Arc<AppState>> for UserService {
	type Rejection = Infallible;

	async fn from_request_parts(
		_: &mut Parts,
		state: &Arc<AppState>,
	) -> Result<Self, Self::Rejection> {
		Ok(UserService(state.model.db()))
	}
}
