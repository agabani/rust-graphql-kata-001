use crate::database::{Database, Identity};
use crate::domain::{Forum, Reply, Session, Thread, User, UserId, Username};
use actix_web::web;
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Object, Result};
use std::str::FromStr;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn current_user<'a>(&self, ctx: &'a Context<'a>) -> Option<User> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        let username = ctx.data_opt::<UserId>();

        let user_id = match username {
            None => return None,
            Some(user_id) => user_id,
        };

        database.get_user_by_id(user_id).await
    }

    async fn forums<'a>(
        &self,
        ctx: &'a Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Forum, EmptyFields, EmptyFields>> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        query(
            after,
            before,
            first,
            last,
            |after: Option<String>, before: Option<String>, first, last| async move {
                let after = after.and_then(decode_cursor).unwrap_or(usize::MIN);
                let before = before.and_then(decode_cursor).unwrap_or(u32::MAX as usize);

                let first = if first.is_none() && last.is_none() {
                    Some(10)
                } else {
                    first
                };

                let results = match (first, last) {
                    (Some(_), Some(_)) => todo!("Bad request..."),
                    (Some(first), None) => database.get_forums_oldest(after, first + 1).await,
                    (None, Some(last)) => database.get_forums_newest(before, last + 1).await,
                    (None, None) => unreachable!(),
                };

                build_connections(results, first, last)
            },
        )
        .await
    }

    async fn user<'a>(
        &self,
        ctx: &'a Context<'a>,
        id: Option<String>,
        username: Option<String>,
    ) -> Option<User> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        if let Some(id) = id {
            return database.get_user_by_id(&UserId(id)).await;
        }

        if let Some(username) = username {
            return database.get_user_by_username(&Username(username)).await;
        }

        None
    }
}

#[Object]
impl Forum {
    async fn id(&self) -> String {
        self.id.0.clone()
    }

    async fn created(&self) -> String {
        self.created.is8601()
    }

    async fn created_by<'a>(&self, ctx: &'a Context<'a>) -> Option<User> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        database.get_user_by_id(&self.created_by).await
    }

    async fn name(&self) -> String {
        self.name.0.clone()
    }

    async fn threads<'a>(
        &self,
        ctx: &'a Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Thread, EmptyFields, EmptyFields>> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        query(
            after,
            before,
            first,
            last,
            |after: Option<String>, before: Option<String>, first, last| async move {
                let after = after.and_then(decode_cursor).unwrap_or(usize::MIN);
                let before = before.and_then(decode_cursor).unwrap_or(u32::MAX as usize);

                let first = if first.is_none() && last.is_none() {
                    Some(10)
                } else {
                    first
                };

                let results = match (first, last) {
                    (Some(_), Some(_)) => todo!("Bad request..."),
                    (Some(first), None) => {
                        database
                            .get_threads_by_forum_oldest(self, after, first + 1)
                            .await
                    }
                    (None, Some(last)) => {
                        database
                            .get_threads_by_forum_newest(self, before, last + 1)
                            .await
                    }
                    (None, None) => unreachable!(),
                };

                build_connections(results, first, last)
            },
        )
        .await
    }
}

#[Object]
impl Reply {
    async fn id(&self) -> String {
        self.id.0.clone()
    }

    async fn created(&self) -> String {
        self.created.is8601()
    }

    async fn created_by<'a>(&self, ctx: &'a Context<'a>) -> Option<User> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        database.get_user_by_id(&self.created_by).await
    }

    async fn text(&self) -> String {
        self.text.0.clone()
    }

    async fn thread<'a>(&self, ctx: &'a Context<'a>) -> Option<Thread> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        database.get_thread_by_id(&self.thread).await
    }
}

#[Object]
impl Thread {
    async fn id(&self) -> String {
        self.id.0.clone()
    }

    async fn created(&self) -> String {
        self.created.is8601()
    }

    async fn created_by<'a>(&self, ctx: &'a Context<'a>) -> Option<User> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        database.get_user_by_id(&self.created_by).await
    }

    async fn name(&self) -> String {
        self.name.0.clone()
    }

    async fn forum<'a>(&self, ctx: &'a Context<'a>) -> Option<Forum> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        database.get_forum_by_id(&self.forum).await
    }
}

#[Object]
impl User {
    async fn id(&self) -> String {
        self.id.0.clone()
    }

    async fn username(&self) -> String {
        self.username.0.clone()
    }

    async fn sessions<'a>(
        &self,
        ctx: &'a Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Session, EmptyFields, EmptyFields>> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        query(
            after,
            before,
            first,
            last,
            |after: Option<String>, before: Option<String>, first, last| async move {
                let after = after.and_then(decode_cursor).unwrap_or(usize::MIN);
                let before = before.and_then(decode_cursor).unwrap_or(u32::MAX as usize);

                let first = if first.is_none() && last.is_none() {
                    Some(10)
                } else {
                    first
                };

                let results = match (first, last) {
                    (Some(_), Some(_)) => todo!("Bad request..."),
                    (Some(first), None) => {
                        database
                            .get_sessions_by_user_oldest(self, after, first + 1)
                            .await
                    }
                    (None, Some(last)) => {
                        database
                            .get_sessions_by_user_newest(self, before, last + 1)
                            .await
                    }
                    (None, None) => unreachable!(),
                };

                build_connections(results, first, last)
            },
        )
        .await
    }
}

#[Object]
impl Session {
    async fn created(&self) -> String {
        self.created.is8601()
    }

    async fn id(&self) -> String {
        self.id.0.clone()
    }

    async fn user<'a>(&self, ctx: &'a Context<'a>) -> Option<User> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        database.get_user_by_session(self).await
    }

    async fn user_agent(&self) -> String {
        self.user_agent.0.clone()
    }
}

fn build_connections<T>(
    results: Vec<Identity<usize, T>>,
    first: Option<usize>,
    last: Option<usize>,
) -> Result<Connection<String, T, EmptyFields, EmptyFields>> {
    let page_info = match (first, last) {
        (Some(first), None) => {
            let has_previous_page = false;
            let has_next_page = results.len() > first;
            let results = results.into_iter().take(first).collect::<Vec<_>>();
            (results, has_previous_page, has_next_page)
        }
        (None, Some(last)) => {
            let has_previous_page = results.len() > last;
            let has_next_page = false;
            let results = results.into_iter().take(last).rev().collect::<Vec<_>>();
            (results, has_previous_page, has_next_page)
        }
        _ => unreachable!(),
    };

    let mut connection = Connection::new(page_info.1, page_info.2);
    connection.append(page_info.0.into_iter().map(|item| {
        Edge::with_additional_fields(base64::encode(item.id.to_string()), item.value, EmptyFields)
    }));
    Ok(connection)
}

fn decode_cursor<T: FromStr, V: AsRef<[u8]>>(value: V) -> Option<T> {
    base64::decode(value)
        .ok()
        .and_then(|utf8_bytes| String::from_utf8_lossy(&utf8_bytes).parse().ok())
}
