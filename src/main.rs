use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;

#[derive(Clone)]
struct App {
    //db: Arc<Mutex<()>>,
}

#[derive(serde::Deserialize)]
struct Payload {
    //
}

#[derive(serde::Deserialize)]
struct Env {
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;

    let app_state = App {
        // db
    };

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/healthz", get(health_handler))
        .route("/state", post(state_handler))
        .with_state(app_state)
        .layer(tower::ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([0, 0, 0, 0], env.port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health_handler() -> &'static str {
    "OK"
}

async fn state_handler(State(_): State<App>, Json(_): Json<Payload>) -> Json<serde_json::Value> {
    Json(serde_json::Value::String("hello world".to_string()))
}
