mod env;

fn main() {
    env::Env::init();

    let env = env::Env::new();
    println!("TODO_API_PORT: {}", env.port);
    println!("TODO_API_ACCESS_TOKEN_SECRET: {}", env.access_token_secret);
    println!("TODO_API_DATABASE_URL: {}", env.database_url);
}
