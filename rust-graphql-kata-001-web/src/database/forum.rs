use crate::database::Identity;
use crate::domain::{Created, Forum, ForumId, ForumName, UserId};
use crate::tracing::TraceErrorExt;

#[tracing::instrument(
    skip(executor, forum),
    fields(
        database.forum.id = forum.id.0.as_str(),
        database.forum.name = forum.name.0.as_str(),
        database.user.id = forum.created_by.0.as_str(),
    )
)]
pub async fn create(executor: &sqlx::Pool<sqlx::Postgres>, forum: &Forum) -> bool {
    sqlx::query!(
        r#"
INSERT INTO forum (public_id, created, created_by_user_id, name)
VALUES (
    $1, $2,
    (SELECT U.id
         FROM "user" AS U
         WHERE u.public_id = $3),
    $4)
ON CONFLICT DO NOTHING
RETURNING id;
"#,
        forum.id.0,
        forum.created.0,
        forum.created_by.0,
        forum.name.0
    )
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")
    .is_some()
}

#[tracing::instrument(
    skip(executor, forum_id),
    fields(
        database.forum.id = forum_id.0.as_str(),
    )
)]
pub async fn get_by_id(executor: &sqlx::Pool<sqlx::Postgres>, forum_id: &ForumId) -> Option<Forum> {
    let record = sqlx::query!(
        r#"
SELECT F.public_id as public_id,
       F.created as created,
       F.name as name,
       U.public_id as user_public_id
FROM forum as F
        INNER JOIN "user" as U ON U.id = F.created_by_user_id
WHERE F.public_id = $1;
"#,
        forum_id.0
    )
    .fetch_optional(executor)
    .await
    .trace_err()
    .expect("Failed to run database query")?;

    Some(Forum {
        id: ForumId(record.public_id),
        created: Created(record.created),
        created_by: UserId(record.user_public_id),
        name: ForumName(record.name),
    })
}

#[tracing::instrument(skip(executor))]
pub async fn list_oldest(
    executor: &sqlx::Pool<sqlx::Postgres>,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Forum>> {
    let records = sqlx::query!(
        r#"
SELECT F.id as id,
       F.public_id as public_id,
       F.created as created,
       F.name as name,
       U.public_id as user_public_id
FROM forum as F
        INNER JOIN "user" as U ON U.id = F.created_by_user_id
WHERE F.id > $1
ORDER BY F.id ASC
LIMIT $2;
"#,
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
            value: Forum {
                id: ForumId(record.public_id),
                created: Created(record.created),
                created_by: UserId(record.user_public_id),
                name: ForumName(record.name),
            },
        })
        .collect()
}

#[tracing::instrument(skip(executor))]
pub async fn list_newest(
    executor: &sqlx::Pool<sqlx::Postgres>,
    start: usize,
    limit: usize,
) -> Vec<Identity<usize, Forum>> {
    let records = sqlx::query!(
        r#"
SELECT F.id as id,
       F.public_id as public_id,
       F.created as created,
       F.name as name,
       U.public_id as user_public_id
FROM forum as F
        INNER JOIN "user" as U ON U.id = F.created_by_user_id
WHERE F.id < $1
ORDER BY F.id DESC
LIMIT $2;
"#,
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
            value: Forum {
                id: ForumId(record.public_id),
                created: Created(record.created),
                created_by: UserId(record.user_public_id),
                name: ForumName(record.name),
            },
        })
        .collect()
}
