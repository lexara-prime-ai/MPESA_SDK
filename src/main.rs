/*  let headers = new Headers();
    headers.append("Authorization", "Bearer cFJZcjZ6anEwaThMMXp6d1FETUxwWkIzeVBDa2hNc2M6UmYyMkJmWm9nMHFRR2xWOQ==");

    fetch("https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials", { headers })
   .then(response => response.text())
   .then(result => console.log(result))
   .catch(error => console.log(error));
*/

use reqwest;
use reqwest::header::AUTHORIZATION;

// [SERVICE] endpoints.
mod service_endpoints;
use service_endpoints::endpoints;

// [SERVICE] responses types.
mod models;
use models::response::{AuthResponse, AuthResponseError};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let urls = endpoints::ServiceEndpoints::new();

    print!("{:?}", urls.Authorization);

    let result = client
        .get(urls.Authorization.url)
        .header(
            AUTHORIZATION,
            "Basic cFJZcjZ6anEwaThMMXp6d1FETUxwWkIzeVBDa2hNc2M6UmYyMkJmWm9nMHFRR2xWOQ==",
        )
        .send()
        .await
        .unwrap()
        .json::<AuthResponseError>()
        .await;

    print!("{:#?}", result);
}
