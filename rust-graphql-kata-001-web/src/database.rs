use crate::domain::{
    Created, Forum, ForumId, ForumName, Session, SessionId, Thread, ThreadId, ThreadName, User,
    UserAgent, UserId, Username,
};
use crate::tracing::TraceErrorExt;

pub struct Database {
    postgres: sqlx::Pool<sqlx::Postgres>,
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
        skip(self, user, start, limit),
        fields(
            database.user.id = user.id.0.as_str(),
        )
    )]
    pub async fn get_sessions_by_user_oldest(
        &self,
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
LIMIT $3
            "#,
            user.id.0,
            start as i64,
            limit as i64
        )
        .fetch_all(&self.postgres)
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
        skip(self, user, start, limit),
        fields(
            database.user.id = user.id.0.as_str(),
        )
    )]
    pub async fn get_sessions_by_user_newest(
        &self,
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
LIMIT $3
            "#,
            user.id.0,
            start as i64,
            limit as i64
        )
        .fetch_all(&self.postgres)
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

    #[tracing::instrument(skip(self))]
    pub async fn get_forums_oldest(
        &self,
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
LIMIT $2
"#,
            start as i64,
            limit as i64
        )
        .fetch_all(&self.postgres)
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

    #[tracing::instrument(skip(self))]
    pub async fn get_forums_newest(
        &self,
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
LIMIT $2
"#,
            start as i64,
            limit as i64
        )
        .fetch_all(&self.postgres)
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

    #[tracing::instrument(
        skip(self, forum),
        fields(
            database.forum.id = forum.id.0.as_str(),
            database.forum.name = forum.name.0.as_str(),
            database.user.id = forum.created_by.0.as_str(),
        )
    )]
    pub async fn create_forum(&self, forum: &Forum) -> bool {
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
RETURNING id;;
"#,
            forum.id.0,
            forum.created.0,
            forum.created_by.0,
            forum.name.0
        )
        .fetch_optional(&self.postgres)
        .await
        .trace_err()
        .expect("Failed to run database query")
        .is_some()
    }

    #[tracing::instrument(
    skip(self, forum_id),
        fields(
         database.forum.id = forum_id.0.as_str(),
        )
    )]
    pub async fn get_forum_by_id(&self, forum_id: &ForumId) -> Option<Forum> {
        let record = sqlx::query!(
            r#"
SELECT F.public_id as public_id,
       F.created as created,
       F.name as name,
       U.public_id as user_public_id
FROM forum as F
        INNER JOIN "user" as U ON U.id = F.created_by_user_id
WHERE F.public_id = $1
            "#,
            forum_id.0
        )
        .fetch_optional(&self.postgres)
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

    #[tracing::instrument(
    skip(self, forum, start, limit),
        fields(
           database.forum.id = forum.id.0.as_str(),
        )
    )]
    pub async fn get_threads_by_forum_oldest(
        &self,
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
        .fetch_all(&self.postgres)
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
    skip(self, forum, start, limit),
        fields(
           database.forum.id = forum.id.0.as_str(),
        )
    )]
    pub async fn get_threads_by_forum_newest(
        &self,
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
        .fetch_all(&self.postgres)
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
}
