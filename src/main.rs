use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use handler::auth_middleware;
use sqlx::postgres::PgPoolOptions;

mod env;
mod handler;
mod id;
mod password;
mod task;
mod usecase;
mod user;

#[tokio::main]
async fn main() {
    env::Env::init();
    let env = env::Env::new();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.port))
        .await
        .expect("Failed to bind to port");

    let db_pool = db_pool(&env).await;
    let app_state = usecase::AppState::new(env, db_pool);

    let app = app(app_state);

    axum::serve(listener, app).await.unwrap();
}

async fn db_pool(env: &env::Env) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .idle_timeout(std::time::Duration::from_secs(5))
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&env.database_url)
        .await
        .expect("Failed to connect to DB")
}

fn app(app_state: usecase::AppState) -> Router {
    let no_auth_routes = Router::new()
        // curl -v -X GET http://localhost:8080/healthz
        .route("/healthz", get(handler::get_healthz))
        // curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d '{"email": "user@example.com", "password": "password"}'
        .route("/users", post(handler::create_user))
        // curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d '{"email": "user@example.com", "password": "password"}'
        .route("/login", post(handler::login));

    let auth_routes = Router::new()
        // curl -X POST http://localhost:8080/tasks -H "Content-Type: application/json" -H "Authorization: Bearer " -d '{"title": "task title", "status": "ToDo"}'
        .route("/tasks", post(handler::create_task))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware::auth,
        ));

    Router::new()
        .nest("/", no_auth_routes)
        .nest("/", auth_routes)
        .with_state(app_state)
}
