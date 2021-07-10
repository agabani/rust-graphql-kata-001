use crate::domain::{Session, User, UserAgent, Username};
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Result, Schema};

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
            username: Username("username".to_string()),
        }
    }
}

#[Object]
impl User {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn username(&self) -> String {
        self.username.0.clone()
    }

    async fn sessions(&self, id: Option<String>) -> Vec<Session> {
        let data = vec![
            Session {
                id: "id 1".to_string(),
                user_agent: UserAgent("user_agent".to_string()),
            },
            Session {
                id: "id 2".to_string(),
                user_agent: UserAgent("user_agent".to_string()),
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
        self.id.clone()
    }

    async fn user_agent(&self) -> String {
        self.id.clone()
    }

    async fn user(&self) -> Option<User> {
        match self.id.as_str() {
            "id 1" | "id 2" => Some(User {
                id: "id".to_string(),
                username: Username("username".to_string()),
            }),
            _ => None,
        }
    }
}
