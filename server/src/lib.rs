pub mod graphql {
    use async_graphql::http::{
        GraphQLPlaygroundConfig, GraphiQLSource, graphiql_source, playground_source,
    };
    use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
    use axum::extract::State;
    use axum::response::{Html, IntoResponse};

    pub async fn graphql_playground() -> impl IntoResponse {
        Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    }

    pub async fn graphiql() -> impl IntoResponse {
        Html(graphiql_source(
            &GraphiQLSource::build().endpoint("/graphql").finish(),
            None,
        ))
    }
    pub async fn graphql_handler(
        schema: State<ports::graphql::AppSchema>,
        req: GraphQLRequest,
    ) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }
}

pub mod rest {}
