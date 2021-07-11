use actix_web::web;
use async_graphql::{Context, Object};

use crate::database::{user, Database};
use crate::domain::{Session, User};

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
