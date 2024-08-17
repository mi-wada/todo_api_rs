use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};

mod env;

#[tokio::main]
async fn main() {
    env::Env::init();
    let env = env::Env::new();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port))
        .await
        .unwrap();
    let app = Router::new()
        // curl -v -X GET http://localhost:8080/healthz
        .route("/healthz", get(get_healthz))
        .with_state(AppState { env });

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    env: env::Env,
}

async fn get_healthz(State(state): State<AppState>) -> impl IntoResponse {
    StatusCode::OK
}
