use asrtemplate::main;
use tokio::*;

#[actix_rt::test]
async fn health_check_works() {
    spawn_app();

    // spawn_app().await.expect("Failed to spawn our app.");
    // let client = reqest::Client::new();

    // let response = client
    //     .get("http://127.0.0.1:8800/health_check")
    //     .send()
    //     .await
    //     .expect("Failed to execute request.");

    // assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let server = asrtemplate::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
