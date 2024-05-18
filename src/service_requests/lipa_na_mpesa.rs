use anyhow::{Context, Ok, Result};
use dotenv::dotenv;
use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

// [SERVICE] endpoints.
use crate::service_endpoints::endpoints;

// [SERVICE] request types.
use crate::models::request::LipaNaMpesaServiceRequest;

// [SERVICE] responses types.
use crate::models::response::AuthResponse;

// [SERVICES] i.e AuthenticationService, LipaNaMpesaService etc.
use crate::service_requests::authentication::AuthenticationService;
use crate::utils::encoder::Encoder;
use crate::utils::timestamp::TimestampGenerator;

///////////////////////////////////
//// Define Transaction Types ////
/////////////////////////////////
pub enum TransactionTypes {
    CustomerPayBillOnline,
    CustomerBuyGoodsOnline,
}

// Enable enum variants to be used as strings.
impl TransactionTypes {
    fn as_str(&self) -> &'static str {
        match self {
            TransactionTypes::CustomerBuyGoodsOnline => "CustomerBuyGoodsOnline",
            TransactionTypes::CustomerPayBillOnline => "CustomerPayBillOnline",
        }
    }
}

////////////////////////////////////
///// MPESA Express handler. //////
//////////////////////////////////
pub struct LipaNaMpesaService;

impl LipaNaMpesaService {
    pub async fn init(
        amount: u64,
        phonenumber: u64,
        account_reference: String,
        transaction_description: String,
    ) -> Result<String> {
        dotenv().ok();
        let client = reqwest::Client::new();

        /////////////////////////////////////
        ///////////// [CONFIG] /////////////
        ////////////////////////////////////
        let urls = endpoints::ServiceEndpoints::new();
        let auth_response = AuthenticationService::init().await.unwrap();
        let auth_token = format!("Bearer {}", auth_response.access_token);

        println!("{}", auth_token);
        let business_short_code = std::env::var("BUSINESS_SHORT_CODE")
            .expect("[SHORT_CODE] NOT found!")
            .trim()
            .parse::<u64>()
            .unwrap();
        let passkey = std::env::var("PASSKEY").expect("[PASSKEY] NOT found!");
        let timestamp = TimestampGenerator::init().unwrap();
        let password = Encoder::init(business_short_code, passkey, &timestamp).unwrap();
        let callback_url = std::env::var("CALLBACK_URL").expect("[CALLBACK_URL] NOT found!");

        //////////////////////////////////////////c
        ///////////// Define [PAYLOD] ///////////
        ////////////////////////////////////////
        let payload = serde_json::to_string(&LipaNaMpesaServiceRequest {
            BusinessShortCode: business_short_code,
            // Password -> Base64 Encode (Business Short Code + PassKey + Timestamp).
            Password: password,
            Timestamp: timestamp,
            TransactionType: TransactionTypes::CustomerPayBillOnline.as_str().to_string(),
            Amount: amount,
            PartyA: phonenumber,
            PartyB: business_short_code,
            PhoneNumber: phonenumber,
            CallBackURL: callback_url,
            AccountReference: account_reference,
            TransactionDesc: transaction_description,
        })
        .unwrap();

        /////////////////////////////////////
        /////////// [DEBUGGING] /////////////
        /////////////////////////////////////
        // dbg!(auth_token.clone());
        // dbg!(format!("Basic {:?}", auth_token));
        // dbg!(urls.Authorization.clone());
        // dbg!(payload.clone());
        // dbg!("{:#?}", payload.clone());
        ////////////////////////////////////

        let response = client
            .post(urls.MpesaExpress.url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, auth_token)
            .body(payload)
            .send()
            .await
            .context("Error sending request")?;

        if response.status().is_success() {
            let lipa_na_mpesa_response = response.text().await.context("Error parsing response")?;
            Ok(lipa_na_mpesa_response)
        } else {
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }
}

/////////////////
//// NOTES /////
///////////////

/*
    CustomerPayBillOnline and CustomerBuyGoodsOnline are enum variants.

    Every variant of an enum is assigned to a single integer value.
    This is known as the discriminant.
    Currently, discriminants are only allowed to be integers, not arbitrary types like &'static str,
    although that may change in the future.
*/
