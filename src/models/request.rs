use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipaNaMpesaServiceRequest {
    pub BusinessShortCode: u64,
    pub Password: String,
    pub Timestamp: String,
    pub TransactionType: String,
    pub Amount: u64,
    // Customer's phone no. -> 254708374149
    pub PartyA: u64,
    // Same as <BusinessShortCode>
    pub PartyB: u64,
    pub PhoneNumber: u64,
    pub CallBackURL: String,
    pub AccountReference: String,
    pub TransactionDesc: String,
}
