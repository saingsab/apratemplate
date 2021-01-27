// use asrtemplate::run;
// use std::net::TcpListener;

// async fn spawn_app() {
//     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
//     let port = listener.local_addr().unwrap().port();
//     let address = format!("http://127.0.0.1:{}", port);
//     // let mut configuration = get_configuration().expect("Failed to read configuration.");
//     let server = run(listener);
//     let _ = tokio::spawn();
// }
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

// #[actix_rt::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//     // Assrange
//     let app_address = spawn_app();
//     let client = reqwest::Client::new();
//     let body = "name=%20guin&email=ursula_le_guin%40gmail.com";

//     // Act
//     let response = client
//         .post(&format!("{}/subscriptions", &app_address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     assert_eq!(200, response.status().as_u16());
// }
