use async_graphql::EmptySubscription;
use axum::{
    Router,
    routing::{get, post},
    serve,
};

use infra::storage::StorageEngine;
use ports::{
    app_service::AppService,
    graphql::{AppSchema, Mutation, Query},
};
use server::graphql::{graphiql, graphql_handler, graphql_playground};
use shared::config::Config;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app_service = AppService::build(StorageEngine::MongoDB).await;

    let schema = AppSchema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(app_service)
        .finish();

    let router = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/playground", get(graphql_playground))
        .route("/graphiql", get(graphiql))
        .with_state(schema);

    let config = Config::build();

    info!("GraphQL API at http//localhost:{}/graphql", config.port);

    info!(
        "GraphQL Playground at http//localhost:{}/playground",
        config.port
    );
    info!("Axum server at http//localhost:{}", config.port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();
    serve(listener, router).await.unwrap();
}
