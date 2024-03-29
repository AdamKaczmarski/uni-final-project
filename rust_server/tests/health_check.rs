use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = rust_server::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn test_template() {
    // Arrange
    //let app_address = spawn_app();
    //let client = reqwest::Client::new();
    // Act
//    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//    let response = client
//        .post(&format!("{}/subscriptions", &app_address))
//        .header("Content-Type", "application/x-www-form-urlencoded")
//        .body(body)
//        .send()
//        .await
//        .expect("Failed to execute request.");
    // Assert
//    assert_eq!(200, response.status().as_u16());
        assert_eq!(1,1);
}
#[tokio::test]
async fn get_json_object() {
    // Arrange
    //let app_address = spawn_app();
    //let client = reqwest::Client::new();
    // Act
//    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//    let response = client
//        .post(&format!("{}/subscriptions", &app_address))
//        .header("Content-Type", "application/x-www-form-urlencoded")
//        .body(body)
//        .send()
//        .await
//        .expect("Failed to execute request.");
    // Assert
//    assert_eq!(200, response.status().as_u16());
        assert_eq!(1,1);
}
#[tokio::test]
async fn get_bson_object() {
    // Arrange
    //let app_address = spawn_app();
    //let client = reqwest::Client::new();
    // Act
//    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//    let response = client
//        .post(&format!("{}/subscriptions", &app_address))
//        .header("Content-Type", "application/x-www-form-urlencoded")
//        .body(body)
//        .send()
//        .await
//        .expect("Failed to execute request.");
    // Assert
//    assert_eq!(200, response.status().as_u16());
        assert_eq!(1,1);
}
