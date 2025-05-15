use reqwest::{Client, Response};

pub async fn get_qrcode(data: String) -> Response {
    let client = Client::new();
    client
        .post("http://172.16.150.30:8080/function/pg-list-user")
        .header("content-type", "application/octet-stream")
        .body(data)
        .send()
        .await
        .unwrap()
}
