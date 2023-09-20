use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let configurations = get_configurations().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configurations.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", configurations.application_port))?;
    zero2prod::startup::run(listener, connection_pool)?.await
}
