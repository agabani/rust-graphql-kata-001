use crate::database::{forum, reply, session, thread, user, Database, Identity};
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

        user::get_by_id(&database.postgres, user_id).await
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
                    (Some(first), None) => {
                        forum::list_oldest(&database.postgres, after, first + 1).await
                    }
                    (None, Some(last)) => {
                        forum::list_newest(&database.postgres, before, last + 1).await
                    }
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
            return user::get_by_id(&database.postgres, &UserId(id)).await;
        }

        if let Some(username) = username {
            return user::get_by_username(&database.postgres, &Username(username)).await;
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

        user::get_by_id(&database.postgres, &self.created_by).await
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
                        thread::list_oldest_by_forum(&database.postgres, self, after, first + 1)
                            .await
                    }
                    (None, Some(last)) => {
                        thread::list_newest_by_forum(&database.postgres, self, before, last + 1)
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

        user::get_by_id(&database.postgres, &self.created_by).await
    }

    async fn text(&self) -> String {
        self.text.0.clone()
    }

    async fn thread<'a>(&self, ctx: &'a Context<'a>) -> Option<Thread> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        thread::get_by_id(&database.postgres, &self.thread).await
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

        user::get_by_id(&database.postgres, &self.created_by).await
    }

    async fn name(&self) -> String {
        self.name.0.clone()
    }

    async fn forum<'a>(&self, ctx: &'a Context<'a>) -> Option<Forum> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        forum::get_by_id(&database.postgres, &self.forum).await
    }

    async fn replies<'a>(
        &self,
        ctx: &'a Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Reply, EmptyFields, EmptyFields>> {
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
                        reply::list_oldest_by_thread(&database.postgres, self, after, first + 1)
                            .await
                    }
                    (None, Some(last)) => {
                        reply::list_newest_by_thread(&database.postgres, self, before, last + 1)
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
impl User {
    async fn id(&self) -> String {
        self.id.0.clone()
    }

    async fn username(&self) -> String {
        self.username.0.clone()
    }

    async fn replies<'a>(
        &self,
        ctx: &'a Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Reply, EmptyFields, EmptyFields>> {
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
                        reply::list_oldest_by_user(&database.postgres, self, after, first + 1).await
                    }
                    (None, Some(last)) => {
                        reply::list_newest_by_user(&database.postgres, self, before, last + 1).await
                    }
                    (None, None) => unreachable!(),
                };

                build_connections(results, first, last)
            },
        )
        .await
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
                        session::list_oldest_by_user(&database.postgres, self, after, first + 1)
                            .await
                    }
                    (None, Some(last)) => {
                        session::list_newest_by_user(&database.postgres, self, before, last + 1)
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

        user::get_by_session(&database.postgres, self).await
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
