use asrtemplate::configuration::get_configuration;
use asrtemplate::run;
use sqlx::PgPool;
// use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let _connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // Here we choose to bind explicitly to localhost, 127.0.0.1, for security
    // reasons. This binding may cause issues in some environments. For example,
    // it causes connectivity issues running in WSL2, where you cannot reach the
    // server when it is bound to WSL2's localhost interface. As a workaround,
    // you can choose to bind to all interfaces, 0.0.0.0, instead, but be aware
    // of the security implications when you expose the server on all interfaces.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
