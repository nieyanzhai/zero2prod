use actix_web::web;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_configurations().expect("Failed to read configuration.");
    let pg_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let db_pool = web::Data::new(pg_pool);

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", settings.application_port))?;
    zero2prod::startup::run(listener, db_pool)?.await
}
