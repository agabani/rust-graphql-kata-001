use crate::domain::{Session, User, UserId, Username};
use crate::tracing::TraceErrorExt;

#[tracing::instrument(
    skip(executor, user),
    fields(
        database.user.id = user.id.0.as_str(),
        database.user.username = user.username.0.as_str(),
    )
)]
pub async fn create_user(executor: &sqlx::Pool<sqlx::Postgres>, user: &User) -> bool {
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
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")
    .is_some()
}

#[tracing::instrument(
    skip(executor, user_id),
    fields(
        database.user.id = user_id.0.as_str(),
    )
)]
pub async fn get_by_id(executor: &sqlx::Pool<sqlx::Postgres>, user_id: &UserId) -> Option<User> {
    let record = sqlx::query!(
        r#"
SELECT U.public_id AS id,
       U.username as username
FROM "user" as U
WHERE U.public_id = $1;
"#,
        user_id.0
    )
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")?;

    Some(User {
        id: UserId(record.id),
        username: Username(record.username),
    })
}

#[tracing::instrument(
    skip(executor, session),
    fields(
        database.session.id = session.id.0.as_str(),
    )
)]
pub async fn get_by_session(
    executor: &sqlx::Pool<sqlx::Postgres>,
    session: &Session,
) -> Option<User> {
    let record = sqlx::query!(
        r#"
SELECT U.public_id as id,
       U.username as username
FROM "user" as U
        INNER JOIN session as S ON U.id = S.user_id
WHERE S.public_id = $1;
"#,
        session.id.0
    )
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")?;

    Some(User {
        id: UserId(record.id),
        username: Username(record.username),
    })
}

#[tracing::instrument(
    skip(executor, username),
    fields(
        database.user.username = username.0.as_str(),
    )
)]
pub async fn get_by_username(
    executor: &sqlx::Pool<sqlx::Postgres>,
    username: &Username,
) -> Option<User> {
    let record = sqlx::query!(
        r#"
SELECT U.public_id AS id,
       U.username as username
FROM "user" as U
WHERE U.username = $1;
"#,
        username.0
    )
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")?;

    Some(User {
        id: UserId(record.id),
        username: Username(record.username),
    })
}
