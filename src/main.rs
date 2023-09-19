#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8000")?;
    zero2prod::startup::run(listener)?.await
}
