use crate::domain::{Session, User};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

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

    async fn current_user(&self) -> User {
        User {
            id: "id".to_string(),
            username: "username".to_string(),
        }
    }
}

#[Object]
impl User {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn username(&self) -> String {
        self.username.clone()
    }

    async fn sessions(&self, id: Option<String>) -> Vec<Session> {
        let data = vec![
            Session {
                id: "id 1".to_string(),
                user_agent: "user_agent".to_string(),
            },
            Session {
                id: "id 2".to_string(),
                user_agent: "user_agent".to_string(),
            },
        ];

        match id {
            None => data,
            Some(id) => data
                .iter()
                .filter(|d| d.id.contains(&id))
                .cloned()
                .collect::<Vec<_>>(),
        }
    }
}

#[Object]
impl Session {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn user_agent(&self) -> String {
        self.id.clone()
    }
}
