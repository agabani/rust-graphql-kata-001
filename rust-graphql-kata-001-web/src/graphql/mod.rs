mod mutation;
mod query;

use crate::graphql::mutation::MutationRoot;
use crate::graphql::query::QueryRoot;
use async_graphql::{EmptySubscription, Schema};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build() -> GraphQLSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}
