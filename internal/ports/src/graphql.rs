use async_graphql::{EmptySubscription, MergedObject, Schema};

mod user;

#[derive(Default, MergedObject)]
pub struct Query(user::UserQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(user::UserMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
