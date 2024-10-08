use std::env;

#[derive(Clone)]
pub(crate) struct Env {
    pub(crate) port: u16,
    pub(crate) database_url: String,
    pub(crate) access_token_secret: String,
}

impl Env {
    /// Load environment variables from .env file.
    pub(crate) fn init() {
        dotenvy::dotenv().expect("Not found .env file");
    }

    #[cfg(test)]
    pub(crate) fn init_test() {
        dotenvy::from_filename(".env.test").expect("Not found .env.test file");
    }

    pub(crate) fn new() -> Self {
        Self {
            port: env::var("TODO_API_PORT")
                .expect("TODO_API_PORT must be set")
                .parse()
                .expect("TODO_API_PORT must be a number"),
            database_url: env::var("TODO_API_DATABASE_URL")
                .expect("TODO_API_DATABASE_URL must be set"),
            access_token_secret: env::var("TODO_API_ACCESS_TOKEN_SECRET")
                .expect("TODO_API_ACCESS_TOKEN_SECRET must be set"),
        }
    }
}
