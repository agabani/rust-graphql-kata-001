use crate::database::Database;
use crate::domain::{Created, Forum, ForumId, ForumName, User, UserId, Username};
use actix_web::web;
use async_graphql::{Context, InputObject, Object, Result};
use uuid::Uuid;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_forum<'a>(
        &self,
        ctx: &'a Context<'a>,
        input: CreateForumInput,
    ) -> Option<Forum> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        let user_id = ctx.data_opt::<UserId>();

        let user_id = match user_id {
            None => return None,
            Some(user_id) => user_id,
        };

        let forum = Forum {
            id: ForumId(Uuid::new_v4().to_string()),
            created: Created(time::OffsetDateTime::now_utc()),
            created_by: user_id.clone(),
            name: ForumName(input.forum.name),
        };

        match database.create_forum(&forum).await {
            true => Some(forum),
            false => None,
        }
    }

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
pub struct CreateForumInput {
    forum: ForumInput,
}

#[derive(InputObject)]
struct ForumInput {
    name: String,
}

#[derive(InputObject)]
pub struct CreateUserInput {
    username: String,
}
