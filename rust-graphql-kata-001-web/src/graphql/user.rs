use actix_web::web;
use async_graphql::connection::{query, Connection, EmptyFields};
use async_graphql::{Context, Object, Result};

use crate::database::{reply, session, Database};
use crate::domain::{Reply, Session, User};
use crate::graphql::query::{build_connections, decode_cursor};

#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id.0
    }

    async fn username(&self) -> &str {
        &self.username.0
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
