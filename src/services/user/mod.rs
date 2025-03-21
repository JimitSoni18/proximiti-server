use serde::{Deserialize, Serialize};

use crate::{
	crypt::{
		pwd::{hash_password, verify_password},
		token::create_token,
	},
	models::relational,
};

pub struct UserService;

pub mod error;

pub struct AuthUser {
	id: i64,
	password: String,
	username: String,
	profile_picture_url: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
	id: i64,
	token: String,
	username: String,
	profile_picture_url: Option<String>,
}

impl UserService {
	pub async fn verify_password(
		db: &relational::Db,
		user: User,
	) -> error::Result<AuthenticatedUser> {
		let auth_user = sqlx::query_as!(
			AuthUser,
			"SELECT id, password, username, profile_picture_url from users where username = $1",
			user.username
		)
		.fetch_one(db)
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

	#[tracing::instrument(skip_all)]
	pub async fn add_user(db: &relational::Db, user: User) -> error::Result<AuthenticatedUser> {
		let hashed_pwd = hash_password(&user.password).map_err(|_| error::Error::InternalError)?;

		let exists = sqlx::query_scalar!(
			r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1) as "bool!""#,
			user.username
		)
		.fetch_one(db)
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
		.fetch_one(db)
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
}
