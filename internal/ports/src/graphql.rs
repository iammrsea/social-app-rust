use async_graphql::{EmptySubscription, MergedObject, Schema};

mod user;

use user::{user_mutation::UserMutation, user_query::UserQuery};

#[derive(Default, MergedObject)]
pub struct Query(UserQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(UserMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
