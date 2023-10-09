use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{configuration, telemetry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configurations =
        configuration::get_configurations().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(configurations.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configurations.application.host, configurations.application.port
    ))?;
    zero2prod::startup::run(listener, connection_pool)?.await
}
