use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;

mod env;
mod handler;
mod id;
mod password;
mod task;
mod user;

#[tokio::main]
async fn main() {
    env::Env::init();
    let env = env::Env::new();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .idle_timeout(std::time::Duration::from_secs(5))
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&env.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port))
        .await
        .unwrap();
    let app = Router::new()
        // curl -v -X GET http://localhost:8080/healthz
        .route("/healthz", get(handler::get_healthz))
        // curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d '{"email": "user@example.com", "password": "password"}'
        .route("/users", post(handler::create_user))
        // curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d '{"email": "user@example.com", "password": "password"}'
        .route("/login", post(handler::login))
        .with_state(handler::AppState::new(env, db_pool));

    axum::serve(listener, app).await.unwrap();
}
