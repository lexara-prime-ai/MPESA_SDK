use dotenv::dotenv;
use reqwest;
use reqwest::header::AUTHORIZATION;

// [SERVICE] endpoints.
use crate::models::response::AuthResponse;
use crate::service_endpoints::endpoints;

// [SERVICE] responses types.

pub struct AuthenticationService;

impl AuthenticationService {
    #[tokio::main]
    pub async fn init() {
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
}
