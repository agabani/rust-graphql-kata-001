use crate::database::Identity;
use crate::domain::{Created, Session, SessionId, User, UserAgent};
use crate::tracing::TraceErrorExt;

#[tracing::instrument(
    skip(executor, user, start, limit),
    fields(
      database.user.id = user.id.0.as_str(),
    )
)]
pub async fn list_oldest_by_user(
    executor: &sqlx::Pool<sqlx::Postgres>,
    user: &User,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Session>> {
    let records = sqlx::query!(
        r#"
SELECT S.id AS id,
       S.public_id AS public_id,
       S.user_agent as user_agent,
       S.created as created
FROM session AS S
        INNER JOIN "user" as U ON U.id = S.user_id
WHERE U.public_id = $1
  AND S.id > $2
ORDER BY S.id ASC
LIMIT $3;
"#,
        user.id.0,
        start as i64,
        limit as i64
    )
    .fetch_all(executor)
    .await
    .trace_err()
    .expect("Failed to run database query");

    records
        .into_iter()
        .map(|record| Identity {
            id: record.id as usize,
            value: Session {
                id: SessionId(record.public_id),
                user_agent: UserAgent(record.user_agent),
                created: Created(record.created),
            },
        })
        .collect()
}

#[tracing::instrument(
    skip(executor, user, start, limit),
    fields(
        database.user.id = user.id.0.as_str(),
    )
)]
pub async fn list_newest_by_user(
    executor: &sqlx::Pool<sqlx::Postgres>,
    user: &User,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Session>> {
    let records = sqlx::query!(
        r#"
SELECT S.id AS id,
       S.public_id AS public_id,
       S.user_agent as user_agent,
       S.created as created
FROM session AS S
        INNER JOIN "user" as U ON U.id = S.user_id
WHERE U.public_id = $1
  AND S.id < $2
ORDER BY S.id DESC
LIMIT $3;
"#,
        user.id.0,
        start as i64,
        limit as i64
    )
    .fetch_all(executor)
    .await
    .trace_err()
    .expect("Failed to run database query");

    records
        .into_iter()
        .map(|record| Identity {
            id: record.id as usize,
            value: Session {
                id: SessionId(record.public_id),
                user_agent: UserAgent(record.user_agent),
                created: Created(record.created),
            },
        })
        .collect()
}
