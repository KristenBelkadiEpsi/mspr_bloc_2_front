use reqwest::Client;

pub async fn get_qrcode(data: String) -> Vec<u8> {
    let client = Client::new();
    let response = client
        .post("http://172.16.150.30:8080/function/qrcode-go")
        .header("content-type", "image/png")
        .body(data)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    response.into()
}
