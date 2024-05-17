#![allow(unused)]

/// The following list contains the required URLs.
const AUTHORIZATION_URL: &str =
    "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials";
const CUSTOMER_TO_BUSINESS_URL: &str = "https://sandbox.safaricom.co.ke/mpesa/c2b/v1/registerurl";
const MPESA_EXPRESS_URL: &str = "https://sandbox.safaricom.co.ke/mpesa/stkpush/v1/processrequest";

#[derive(Debug, Clone)]
pub enum RequestTypes {
    GET,
    POST,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct EndpointMetadata {
    pub url: String,
    pub requestType: RequestTypes,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ServiceEndpoints {
    pub Authorization: EndpointMetadata,
    pub CustomerToBusiness: EndpointMetadata,
    pub MpesaExpress: EndpointMetadata,
}

impl ServiceEndpoints {
    pub fn new() -> Self {
        ServiceEndpoints {
            Authorization: EndpointMetadata {
                url: AUTHORIZATION_URL.to_owned(),
                requestType: RequestTypes::GET,
            },
            CustomerToBusiness: EndpointMetadata {
                url: CUSTOMER_TO_BUSINESS_URL.to_owned(),
                requestType: RequestTypes::POST,
            },
            MpesaExpress: EndpointMetadata {
                url: MPESA_EXPRESS_URL.to_owned(),
                requestType: RequestTypes::POST,
            },
        }
    }
}
