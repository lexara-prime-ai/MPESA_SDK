#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponseError {
    pub requestId: String,
    pub errorCode: String,
    pub errorMessage: String,
}
