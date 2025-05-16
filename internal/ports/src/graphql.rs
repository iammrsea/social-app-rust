use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

mod user;

#[derive(Default, MergedObject)]
pub struct Query(user::UserQuery);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
