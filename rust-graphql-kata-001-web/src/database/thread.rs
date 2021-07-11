use crate::database::Identity;
use crate::domain::{Created, Forum, ForumId, Thread, ThreadId, ThreadName, UserId};
use crate::tracing::TraceErrorExt;

#[tracing::instrument(
    skip(executor, thread),
    fields(
        database.forum.id = thread.forum.0.as_str(),
        database.thread.id = thread.id.0.as_str(),
        database.thread.name = thread.name.0.as_str(),
        database.user.id = thread.created_by.0.as_str(),
    )
)]
pub async fn create(executor: &sqlx::Pool<sqlx::Postgres>, thread: &Thread) -> bool {
    sqlx::query!(
        r#"
INSERT INTO thread (public_id, created, created_by_user_id, name, forum_id)
VALUES (
    $1, $2,
    (SELECT U.id
         FROM "user" AS U
         WHERE u.public_id = $3),
    $4,
    (SELECT F.id
         FROM forum AS F
         WHERE F.public_id = $5))
ON CONFLICT DO NOTHING
RETURNING id;;
"#,
        thread.id.0,
        thread.created.0,
        thread.created_by.0,
        thread.name.0,
        thread.forum.0
    )
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")
    .is_some()
}

#[tracing::instrument(
    skip(executor, thread_id),
    fields(
        database.thread.id = thread_id.0.as_str(),
    )
)]
pub async fn get_by_id(
    executor: &sqlx::Pool<sqlx::Postgres>,
    thread_id: &ThreadId,
) -> Option<Thread> {
    let record = sqlx::query!(
        r#"
SELECT T.public_id as public_id,
       T.created as created,
       T.name as name,
       F.public_id as forum_public_id,
       U.public_id as user_public_id
FROM thread as T
        INNER JOIN forum AS F ON F.id = T.forum_id
        INNER JOIN "user" as U ON U.id = T.created_by_user_id
WHERE T.public_id = $1
            "#,
        thread_id.0
    )
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")?;

    Some(Thread {
        id: ThreadId(record.public_id),
        created: Created(record.created),
        created_by: UserId(record.user_public_id),
        forum: ForumId(record.forum_public_id),
        name: ThreadName(record.name),
    })
}

#[tracing::instrument(
    skip(executor, forum, start, limit),
    fields(
        database.forum.id = forum.id.0.as_str(),
    )
)]
pub async fn list_oldest_by_forum(
    executor: &sqlx::Pool<sqlx::Postgres>,
    forum: &Forum,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Thread>> {
    let records = sqlx::query!(
        r#"
SELECT T.id AS id,
       T.public_id as public_id,
       T.created as created,
       T.name as name,
       U.public_id as user_public_id,
       F.public_id as forum_public_id
FROM thread AS T
        INNER JOIN "user" as U ON U.id = T.created_by_user_id
        INNER JOIN forum AS F ON F.id = T.forum_id
WHERE F.public_id = $1
  AND T.id > $2
ORDER BY T.id ASC
LIMIT $3
            "#,
        forum.id.0,
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

            value: Thread {
                id: ThreadId(record.public_id),
                created: Created(record.created),
                created_by: UserId(record.user_public_id),
                forum: ForumId(record.forum_public_id),
                name: ThreadName(record.name),
            },
        })
        .collect()
}

#[tracing::instrument(
    skip(executor, forum, start, limit),
    fields(
        database.forum.id = forum.id.0.as_str(),
    )
)]
pub async fn list_newest_by_forum(
    executor: &sqlx::Pool<sqlx::Postgres>,
    forum: &Forum,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Thread>> {
    let records = sqlx::query!(
        r#"
SELECT T.id AS id,
       T.public_id as public_id,
       T.created as created,
       T.name as name,
       U.public_id as user_public_id,
       F.public_id as forum_public_id
FROM thread AS T
        INNER JOIN "user" as U ON U.id = T.created_by_user_id
        INNER JOIN forum AS F ON F.id = T.forum_id
WHERE F.public_id = $1
  AND T.id < $2
ORDER BY T.id DESC
LIMIT $3
            "#,
        forum.id.0,
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

            value: Thread {
                id: ThreadId(record.public_id),
                created: Created(record.created),
                created_by: UserId(record.user_public_id),
                forum: ForumId(record.forum_public_id),
                name: ThreadName(record.name),
            },
        })
        .collect()
}
