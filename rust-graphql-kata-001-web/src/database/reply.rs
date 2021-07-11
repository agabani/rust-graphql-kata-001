use crate::database::Identity;
use crate::domain::{Created, Reply, ReplyId, ReplyText, Thread, ThreadId, User, UserId};
use crate::tracing::TraceErrorExt;

#[tracing::instrument(
    skip(executor, reply),
    fields(
        database.reply.id = reply.id.0.as_str(),
        database.thread.id = reply.thread.0.as_str(),
        database.user.id = reply.created_by.0.as_str(),
    )
)]
pub async fn create(executor: &sqlx::Pool<sqlx::Postgres>, reply: &Reply) -> bool {
    sqlx::query!(
        r#"
INSERT INTO reply (public_id, created, created_by_user_id, thread_id, text)
VALUES (
    $1, $2,
    (SELECT U.id
         FROM "user" AS U
         WHERE u.public_id = $3),
    (SELECT T.id
         FROM thread AS T
         WHERE T.public_id = $4),
    $5)
ON CONFLICT DO NOTHING
RETURNING id;;
"#,
        reply.id.0,
        reply.created.0,
        reply.created_by.0,
        reply.thread.0,
        reply.text.0
    )
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")
    .is_some()
}

#[tracing::instrument(
    skip(executor, thread, start, limit),
    fields(
        database.thread.id = thread.id.0.as_str(),
    )
)]
pub async fn list_oldest_by_thread(
    executor: &sqlx::Pool<sqlx::Postgres>,
    thread: &Thread,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Reply>> {
    let records = sqlx::query!(
        r#"
SELECT R.id AS id,
       R.public_id as public_id,
       R.created as created,
       R.text as text,
       U.public_id as user_public_id,
       T.public_id as thread_public_id
FROM reply AS R
        INNER JOIN "user" as U ON U.id = R.created_by_user_id
        INNER JOIN thread AS T ON T.id = R.thread_id
WHERE T.public_id = $1
  AND R.id > $2
ORDER BY R.id ASC
LIMIT $3
            "#,
        thread.id.0,
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

            value: Reply {
                id: ReplyId(record.public_id),
                created: Created(record.created),
                created_by: UserId(record.user_public_id),
                thread: ThreadId(record.thread_public_id),
                text: ReplyText(record.text),
            },
        })
        .collect()
}

#[tracing::instrument(
    skip(executor, thread, start, limit),
    fields(
        database.thread.id = thread.id.0.as_str(),
    )
)]
pub async fn list_newest_by_thread(
    executor: &sqlx::Pool<sqlx::Postgres>,
    thread: &Thread,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Reply>> {
    let records = sqlx::query!(
        r#"
SELECT R.id AS id,
       R.public_id as public_id,
       R.created as created,
       R.text as text,
       U.public_id as user_public_id,
       T.public_id as thread_public_id
FROM reply AS R
        INNER JOIN "user" as U ON U.id = R.created_by_user_id
        INNER JOIN thread AS T ON T.id = R.thread_id
WHERE T.public_id = $1
  AND R.id < $2
ORDER BY R.id DESC
LIMIT $3
            "#,
        thread.id.0,
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

            value: Reply {
                id: ReplyId(record.public_id),
                created: Created(record.created),
                created_by: UserId(record.user_public_id),
                thread: ThreadId(record.thread_public_id),
                text: ReplyText(record.text),
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
pub async fn list_oldest_by_user(
    executor: &sqlx::Pool<sqlx::Postgres>,
    user: &User,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Reply>> {
    let records = sqlx::query!(
        r#"
SELECT R.id AS id,
       R.public_id as public_id,
       R.created as created,
       R.text as text,
       U.public_id as user_public_id,
       T.public_id as thread_public_id
FROM reply AS R
        INNER JOIN "user" as U ON U.id = R.created_by_user_id
        INNER JOIN thread AS T ON T.id = R.thread_id
WHERE U.public_id = $1
  AND R.id > $2
ORDER BY R.id ASC
LIMIT $3
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

            value: Reply {
                id: ReplyId(record.public_id),
                created: Created(record.created),
                created_by: UserId(record.user_public_id),
                thread: ThreadId(record.thread_public_id),
                text: ReplyText(record.text),
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
) -> Vec<Identity<usize, Reply>> {
    let records = sqlx::query!(
        r#"
SELECT R.id AS id,
       R.public_id as public_id,
       R.created as created,
       R.text as text,
       U.public_id as user_public_id,
       T.public_id as thread_public_id
FROM reply AS R
        INNER JOIN "user" as U ON U.id = R.created_by_user_id
        INNER JOIN thread AS T ON T.id = R.thread_id
WHERE U.public_id = $1
  AND R.id < $2
ORDER BY R.id DESC
LIMIT $3
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

            value: Reply {
                id: ReplyId(record.public_id),
                created: Created(record.created),
                created_by: UserId(record.user_public_id),
                thread: ThreadId(record.thread_public_id),
                text: ReplyText(record.text),
            },
        })
        .collect()
}
