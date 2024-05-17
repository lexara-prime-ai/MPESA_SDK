/*  let headers = new Headers();
    headers.append("Authorization", "Bearer cFJZcjZ6anEwaThMMXp6d1FETUxwWkIzeVBDa2hNc2M6UmYyMkJmWm9nMHFRR2xWOQ==");

    fetch("https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials", { headers })
   .then(response => response.text())
   .then(result => console.log(result))
   .catch(error => console.log(error));
*/

use std::str::FromStr;

use reqwest;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use url::Url;

// [SERVICE] endpoints
mod service_endpoints;
use service_endpoints::endpoints;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let urls = endpoints::ServiceEndpoints::new();

    print!("{:?}", urls.Authorization);

    let response = client
        .get(urls.Authorization.url)
        .header(
            AUTHORIZATION,
            "Bearer cFJZcjZ6anEwaThMMXp6d1FETUxwWkIzeVBDa2hNc2M6UmYyMkJmWm9nMHFRR2xWOQ==",
        )
        .send()
        .await
        .unwrap()
        .json::<Vec<AuthResponse>>()
        .await;

    print!("{:#?}", response);
}
