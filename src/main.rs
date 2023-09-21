use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, telemetry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configurations =
        configuration::get_configurations().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configurations.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", configurations.application_port))?;
    zero2prod::startup::run(listener, connection_pool)?.await
}
