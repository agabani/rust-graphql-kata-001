use crate::domain::{User, UserId, Username};
use crate::tracing::TraceErrorExt;

pub struct Database {
    postgres: sqlx::Pool<sqlx::Postgres>,
}

impl Database {
    pub fn new(postgres: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { postgres }
    }

    pub async fn get_user_by_id(&self, user_id: &UserId) -> Option<User> {
        let record = sqlx::query!(
            r#"
SELECT U.public_id AS id,
       U.username as username
FROM "user" as U
WHERE U.public_id = $1
            "#,
            user_id.0
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("Failed to run database query")?;

        Some(User {
            id: UserId(record.id),
            username: Username(record.username),
        })
    }

    pub async fn get_user_by_username(&self, username: &Username) -> Option<User> {
        let record = sqlx::query!(
            r#"
SELECT U.public_id AS id,
       U.username as username
FROM "user" as U
WHERE U.username = $1
            "#,
            username.0
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("Failed to run database query")?;

        Some(User {
            id: UserId(record.id),
            username: Username(record.username),
        })
    }
}
