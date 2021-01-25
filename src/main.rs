use asrtemplate::run;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", "8088");
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
