use actix_web::web;
use async_graphql::{Context, InputObject, Object};
use uuid::Uuid;

use crate::database::{forum, reply, thread, user, Database};
use crate::domain::{
    Created, Forum, ForumId, ForumName, Reply, ReplyId, ReplyText, Thread, ThreadId, ThreadName,
    User, UserId, Username,
};

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

        if forum::create(&database.postgres, &forum).await {
            Some(forum)
        } else {
            None
        }
    }

    async fn create_reply<'a>(
        &self,
        ctx: &'a Context<'a>,
        input: CreateReplyInput,
    ) -> Option<Reply> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        let user_id = ctx.data_opt::<UserId>();

        let user_id = match user_id {
            None => return None,
            Some(user_id) => user_id,
        };

        let reply = Reply {
            id: ReplyId(Uuid::new_v4().to_string()),
            created: Created(time::OffsetDateTime::now_utc()),
            created_by: user_id.clone(),
            thread_id: ThreadId(input.reply.thread_id),
            text: ReplyText(input.reply.text),
        };

        if reply::create(&database.postgres, &reply).await {
            Some(reply)
        } else {
            None
        }
    }

    async fn create_thread<'a>(
        &self,
        ctx: &'a Context<'a>,
        input: CreateThreadInput,
    ) -> Option<Thread> {
        let database = ctx
            .data::<web::Data<Database>>()
            .expect("Database not in context");

        let user_id = ctx.data_opt::<UserId>();

        let user_id = match user_id {
            None => return None,
            Some(user_id) => user_id,
        };

        let thread = Thread {
            id: ThreadId(Uuid::new_v4().to_string()),
            created: Created(time::OffsetDateTime::now_utc()),
            created_by: user_id.clone(),
            forum_id: ForumId(input.thread.forum_id),
            name: ThreadName(input.thread.name),
        };

        if thread::create(&database.postgres, &thread).await {
            Some(thread)
        } else {
            None
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

        if user::create(&database.postgres, &user).await {
            Some(user)
        } else {
            None
        }
    }
}

#[derive(InputObject)]
pub struct CreateForumInput {
    forum: ForumInput,
}

#[derive(InputObject)]
pub struct CreateReplyInput {
    reply: ReplyInput,
}

#[derive(InputObject)]
pub struct CreateThreadInput {
    thread: ThreadInput,
}

#[derive(InputObject)]
pub struct CreateUserInput {
    username: String,
}

#[derive(InputObject)]
struct ForumInput {
    name: String,
}

#[derive(InputObject)]
struct ReplyInput {
    text: String,
    thread_id: String,
}

#[derive(InputObject)]
struct ThreadInput {
    name: String,
    forum_id: String,
}
