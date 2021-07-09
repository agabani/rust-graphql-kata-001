mod test_server;

#[actix_rt::test]
async fn health_check_liveness_works() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/liveness", test_server.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
}

#[actix_rt::test]
async fn health_check_readiness_works() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/readiness", test_server.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
}
