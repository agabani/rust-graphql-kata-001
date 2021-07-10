use crate::database::Database;
use crate::domain::{User, UserId, Username};
use actix_web::web;
use async_graphql::{Context, InputObject, Object, Result};
use uuid::Uuid;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user<'a>(&self, ctx: &'a Context<'a>, input: CreateUserInput) -> Option<User> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        let id = Uuid::new_v4();

        let user = User {
            id: UserId(id.to_string()),
            username: Username(input.username),
        };

        match database.create_user(&user).await {
            true => Some(user),
            false => None,
        }
    }
}

#[derive(InputObject)]
pub struct CreateUserInput {
    username: String,
}
