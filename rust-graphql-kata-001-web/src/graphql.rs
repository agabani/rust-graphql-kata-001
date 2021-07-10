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
            |after, before, first, last| async move {
                let sessions = match (first, last) {
                    (Some(first), None) => {
                        let first = first;
                        (
                            database.get_sessions_by_user_oldest(self, first + 1).await,
                            first,
                            true,
                        )
                    }
                    (None, Some(last)) => {
                        let last = last;
                        (
                            database.get_sessions_by_user_newest(self, last + 1).await,
                            last,
                            false,
                        )
                    }
                    (None, None) => {
                        let limit = first.unwrap_or(100);

                        (
                            database.get_sessions_by_user_oldest(self, limit + 1).await,
                            limit,
                            true,
                        )
                    }
                    _ => todo!("Bad query"),
                };

                let has_page = sessions.0.len() > sessions.1;

                let mut connection =
                    Connection::new(has_page && !sessions.2, has_page && sessions.2);
                connection.append(sessions.0.into_iter().take(sessions.1).map(|session| {
                    Edge::with_additional_fields(session.id.0.clone(), session, EmptyFields)
                }));
                Ok(connection)
            },
        )
        .await
    }
}

#[Object]
impl Session {
    async fn created(&self) -> String {
        self.created.0.format("%Y-%m-%dT%H:%M:%S.%NZ")
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
