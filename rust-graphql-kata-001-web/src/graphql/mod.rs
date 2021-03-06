use async_graphql::{EmptySubscription, Schema};

use crate::graphql::mutation::MutationRoot;
use crate::graphql::query::QueryRoot;

mod forum;
mod mutation;
mod query;
mod reply;
mod session;
mod thread;
mod user;

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build() -> GraphQLSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}
