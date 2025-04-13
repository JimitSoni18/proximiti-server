use std::{convert::Infallible, sync::Arc};

use axum::extract::FromRequest;

use crate::{models::sql, AppState};

pub mod error;

pub struct ConversationService(sql::Db);

impl FromRequest<Arc<AppState>> for ConversationService {
	type Rejection = Infallible;

	async fn from_request(
		_: axum::extract::Request,
		state: &Arc<AppState>,
	) -> Result<Self, Self::Rejection> {
		Ok(Self(state.model.db()))
	}
}

#[derive(sqlx::Type, serde::Serialize)]
struct OtherUserInfo {
	id: i64,
	username: String,
	#[serde(rename = "camelCase", skip_serializing_if = "Option::is_none")]
	profile_picture_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Conversation {
	id: i64,
	#[serde(rename = "camelCase", skip_serializing_if = "Option::is_none")]
	other_user: Option<OtherUserInfo>,
}

impl ConversationService {
	pub async fn list(&self, user_id: i64) -> error::Result<Vec<Conversation>> {
		sqlx::query_as!(
			Conversation,
			r#"
SELECT
	uc.id,
	(CASE
		WHEN user1_id = $1 THEN user2_id
		ELSE user1_id
	END,
	u.username, u.profile_picture_url) AS "other_user: OtherUserInfo"
FROM user_conversations uc
LEFT JOIN users u
ON u.id = CASE
	WHEN user1_id = $1 THEN user2_id
	ELSE user1_id
END
"#,
			user_id,
		)
		.fetch_all(&self.0)
		.await
		.map_err(|_| error::Error::Internal)
	}
}
