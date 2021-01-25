use std::net::TcpListener;
use asrtemplate::run;


 fn spawn_app() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

// #[actix_rt::test]
// async fn health_check_works() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();

//     // Act
//     let response = client
//         // Use the returned application address
//         .get(&format!("{}/health_check", &app.address))
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     // Assert
//     assert!(response.status().is_success());
//     assert_eq!(Some(0), response.content_length());
// }
