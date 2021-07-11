pub mod forum;
pub mod reply;
pub mod session;
pub mod thread;

use crate::domain::{Session, User, UserId, Username};
use crate::tracing::TraceErrorExt;

pub struct Database {
    pub postgres: sqlx::Pool<sqlx::Postgres>,
}

pub struct Identity<I, T> {
    pub id: I,
    pub value: T,
}

impl Database {
    pub fn new(postgres: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { postgres }
    }

    #[tracing::instrument(
        skip(self, user_id),
        fields(
            database.user.id = user_id.0.as_str(),
        )
    )]
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

    #[tracing::instrument(
    skip(self, username),
        fields(
            database.user.username = username.0.as_str(),
        )
    )]
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

    pub async fn get_user_by_session(&self, session: &Session) -> Option<User> {
        let record = sqlx::query!(
            r#"
SELECT U.public_id as id,
       U.username as username
FROM "user" as U
        INNER JOIN session as S ON U.id = S.user_id
WHERE S.public_id = $1
            "#,
            session.id.0
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

    #[tracing::instrument(
        skip(self, user),
        fields(
            database.user.id = user.id.0.as_str(),
            database.user.username = user.username.0.as_str(),
        )
    )]
    pub async fn create_user(&self, user: &User) -> bool {
        sqlx::query!(
            r#"
INSERT INTO "user" (public_id, username)
VALUES ($1, $2)
ON CONFLICT DO NOTHING
RETURNING id;
"#,
            user.id.0,
            user.username.0
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("Failed to run database query")
        .is_some()
    }
}
