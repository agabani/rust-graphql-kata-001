use crate::database::Database;
use crate::domain::{Session, User, UserId, Username};
use actix_web::web;
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Result, Schema};

pub type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build() -> GraphQLSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

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
impl User {
    async fn id(&self) -> String {
        self.id.0.clone()
    }

    async fn username(&self) -> String {
        self.username.0.clone()
    }

    async fn sessions<'a>(&self, ctx: &'a Context<'a>, id: Option<String>) -> Vec<Session> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        database.get_sessions_by_user(self).await
    }

    async fn numbers(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, i32, EmptyFields, EmptyFields>> {
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(10000);
                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }

                let mut connection = Connection::new(start > 0, end < 10000);
                connection.append(
                    (start..end)
                        .into_iter()
                        .map(|n| Edge::with_additional_fields(n, n as i32, EmptyFields)),
                );
                Ok(connection)
            },
        )
        .await
    }
}

#[Object]
impl Session {
    async fn id(&self) -> String {
        self.id.0.clone()
    }

    async fn user_agent(&self) -> String {
        self.user_agent.0.clone()
    }

    async fn user<'a>(&self, ctx: &'a Context<'a>) -> Option<User> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        database.get_user_by_session(self).await
    }
}
