use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    // use reqwest to perform HTTP requests in our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Should bind to random port");
    // We receive the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Should bind to address");
    tokio::spawn(server);

    // Return the application address to the caller
    format!("http://127.0.0.1:{port:?}")
}
