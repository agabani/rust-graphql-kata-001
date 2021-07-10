use crate::domain::{Created, Session, SessionId, User, UserAgent, UserId, Username};
use crate::tracing::TraceErrorExt;

pub struct Database {
    postgres: sqlx::Pool<sqlx::Postgres>,
}

impl Database {
    pub fn new(postgres: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { postgres }
    }

    #[tracing::instrument(
        skip(self, user_id),
        fields(
            database.user_id = user_id.0.as_str(),
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
            database.username = username.0.as_str(),
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
            database.user_id = user.id.0.as_str(),
        )
    )]
    pub async fn get_sessions_by_user(&self, user: &User) -> Vec<Session> {
        let records = sqlx::query!(
            r#"
SELECT S.public_id AS id,
       S.user_agent as user_agent,
       S.created as created
FROM session AS S
        INNER JOIN "user" as U ON U.id = S.user_id
WHERE U.public_id = $1
            "#,
            user.id.0
        )
        .fetch_all(&self.postgres)
        .await
        .trace_err()
        .expect("Failed to run database query");

        records
            .into_iter()
            .map(|record| Session {
                id: SessionId(record.id),
                user_agent: UserAgent(record.user_agent),
                created: Created(record.created),
            })
            .collect()
    }
}
