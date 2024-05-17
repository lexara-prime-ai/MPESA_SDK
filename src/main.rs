/*
    This crate provides a streamlined interface for 
    integrating M-Pesa, a widely used mobile money 
    service, into your applications.

    Author: Irfan Ghat
    License: MIT
*/

use dotenv::dotenv;
use reqwest;
use reqwest::header::AUTHORIZATION;

// [SERVICE] endpoints.
mod service_endpoints;
use service_endpoints::endpoints;

// [SERVICE] responses types.
mod models;
#[allow(unused)]
use models::response::{AuthResponse, AuthResponseError};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = reqwest::Client::new();
    let urls = endpoints::ServiceEndpoints::new();
    let auth_token = std::env::var("AUTH_TOKEN").expect("[AUTH_TOKEN] NOT found.");

    // dbg!(auth_token.clone());
    // dbg!(format!("Basic {:?}", auth_token));
    // dbg!(urls.Authorization.clone());

    match client
        .get(urls.Authorization.url)
        .header(AUTHORIZATION, format!("Basic {}", auth_token.trim()))
        .send()
        .await
    {
        Ok(results) => match results.json::<AuthResponse>().await {
            Ok(auth_response) => {
                println!("\nAUTH_RESPONSE: {:#?}", auth_response);
            }
            Err(e) => {
                println!("\nError parsing [AuthResponse]: {:#?}", e);
            }
        },
        Err(e) => {
            println!("\nError sending request: {:#?}", e);
        }
    }
}
