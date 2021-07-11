use std::str::FromStr;

use actix_web::web;
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Object, Result};

use crate::database::{forum, user, Database, Identity};
use crate::domain::{Forum, User, UserId, Username};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
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

pub fn build_connections<T>(
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

pub fn decode_cursor<T: FromStr, V: AsRef<[u8]>>(value: V) -> Option<T> {
    base64::decode(value)
        .ok()
        .and_then(|utf8_bytes| String::from_utf8_lossy(&utf8_bytes).parse().ok())
}
