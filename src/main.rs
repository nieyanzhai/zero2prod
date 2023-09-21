use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::configuration::get_configurations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    let configurations = get_configurations().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configurations.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", configurations.application_port))?;
    zero2prod::startup::run(listener, connection_pool)?.await
}
