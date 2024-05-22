use anyhow::{Context, Result};
use dotenv::dotenv;
use reqwest::header::AUTHORIZATION;
use reqwest::{self};

// [ENVIRONMENT] Management.
use crate::environment_manager::prelude::*;

// [SERVICE] endpoints.
use crate::service_endpoints::prelude::endpoints;

// [SERVICE] responses types.
use crate::models::response::AuthResponse;

#[derive(Debug)]
pub struct AuthenticationService;

impl AuthenticationService {
    pub async fn init() -> Result<AuthResponse> {
        dotenv().ok();
        ////////////////////////////////////////////
        let client = reqwest::Client::new();
        let config = Config::set_environment();
        let urls = endpoints::ServiceEndpoints::new();
        #[allow(non_snake_case)]
        let AUTH_URL = format!("{}{}", Config::set_environment(), &urls.Authorization.url);
        let auth_token = std::env::var("AUTH_TOKEN").context("[AUTH_TOKEN] NOT found.")?;

        ///////////////////////////////////////////////
        let response = client
            .get(AUTH_URL)
            .header(AUTHORIZATION, format!("Basic {}", auth_token.trim()))
            .send()
            .await
            .context("Error sending request")?;

        /////////////////////////////////////////////
        if response.status().is_success() {
            let auth_response: AuthResponse = response
                .json()
                .await
                .context("Error parsing [AuthResponse]")?;
            println!("\nAUTH_RESPONSE: {:#?}", auth_response);
            Ok(auth_response)
        } else {
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }
}
