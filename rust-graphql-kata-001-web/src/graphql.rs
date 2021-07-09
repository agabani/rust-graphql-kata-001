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
}
