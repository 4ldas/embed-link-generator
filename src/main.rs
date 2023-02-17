mod models;
mod handlers;

use std::env::var;
use axum::{
    routing::get,
    Router,
};
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt
};
use tower_http::trace::TraceLayer;
use self::{
    handlers::{
        embed, oembed
    },
    models::AppState
};


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let (root, root_url) = (var("ROOT").unwrap(), var("ROOT_URL").unwrap());


    tracing::info!("Starting on `{}` with root url of `{}`", &root, &root_url);
    axum::Server::bind(&root.parse().unwrap())
        .serve(Router::new()
                .route("/embed", get(embed))
                .route("/oembed", get(oembed))
                .with_state(AppState { root_url })
                .layer(TraceLayer::new_for_http())
            .into_make_service()
        )
        .await
        .unwrap();
}
