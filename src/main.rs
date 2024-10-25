use axum::{
    middleware,
    routing::{delete, get, post},
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
    let app_context = usecase::AppContext::new(env, db_pool);

    let app = app(app_context);

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

fn app(app_context: usecase::AppContext) -> Router {
    let no_auth_routes = Router::new()
        // curl -v -X GET http://localhost:8080/healthz
        .route("/healthz", get(handler::healthz::get))
        // curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d '{"email": "user@example.com", "password": "password"}'
        .route("/users", post(handler::users::post))
        // curl -X POST http://localhost:8080/login -H "Content-Type: application/json" -d '{"email": "user@example.com", "password": "password"}'
        .route("/login", post(handler::login::post));

    let auth_routes = Router::new()
        // curl -X GET http://localhost:8080/tasks -H "Content-Type: application/json" -H "Authorization: Bearer "
        .route("/tasks", get(handler::tasks::list))
        // curl -X POST http://localhost:8080/tasks -H "Content-Type: application/json" -H "Authorization: Bearer " -d '{"title": "task title", "status": "ToDo"}'
        .route("/tasks", post(handler::tasks::post))
        // curl -X DELETE http://localhost:8080/tasks/:task_id -H "Content-Type: application/json" -H "Authorization: Bearer "
        .route("/tasks/:task_id", delete(handler::tasks::delete))
        .route_layer(middleware::from_fn_with_state(
            app_context.clone(),
            auth_middleware::auth,
        ));

    Router::new()
        .nest("/", no_auth_routes)
        .nest("/", auth_routes)
        .with_state(app_context)
}
