use actix_web::web;
use async_graphql::{Context, Object};

use crate::database::{thread, user, Database};
use crate::domain::{Reply, Thread, User};

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
