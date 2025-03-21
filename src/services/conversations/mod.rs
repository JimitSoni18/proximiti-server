use std::error::Error;

use crate::models::relational;

pub struct ConversationService;

#[derive(sqlx::Type)]
struct OtherUserInfo {
	id: i64,
	username: String,
    profile_picture_url: Option<String>,
}

struct ConversationList {
	id: i64,
	other_user: Option<OtherUserInfo>,
}

impl ConversationService {
	async fn list(db: &relational::Db, user_id: i64) -> Result<Vec<ConversationList>, Box<dyn Error>> {
		let _ = sqlx::query_as!(
            ConversationList,
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
		).fetch_all(db).await;
		todo!();
	}
}
