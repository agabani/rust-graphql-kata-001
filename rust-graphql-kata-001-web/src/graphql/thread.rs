use actix_web::web;
use async_graphql::connection::{query, Connection, EmptyFields};
use async_graphql::{Context, Object, Result};

use crate::database::{forum, reply, user, Database};
use crate::domain::{Forum, Reply, Thread, User};
use crate::graphql::query::{build_connections, decode_cursor};

#[Object]
impl Thread {
    async fn id(&self) -> &str {
        &self.id.0
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

    async fn name(&self) -> &str {
        &self.name.0
    }

    async fn forum<'a>(&self, ctx: &'a Context<'a>) -> Option<Forum> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        forum::get_by_id(&database.postgres, &self.forum_id).await
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
