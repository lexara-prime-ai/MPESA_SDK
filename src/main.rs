/*
    This crate provides a streamlined interface for
    integrating M-Pesa, a widely used mobile money
    service, into your applications.

    Author: Irfan Ghat
    License: MIT
*/
#![allow(unused)]

use tokio::main;

use service_requests::{authentication::AuthenticationService, lipa_na_mpesa::LipaNaMpesaService};

// Register all modules
mod models;
mod service_endpoints;
mod service_requests;
mod utils;

#[main]
async fn main() {
    //////////////// Examples //////////////////

    ////////////////////////////////////////////
    //// Initiate [Authentication] service ////
    //////////////////////////////////////////
    // let auth_result = AuthenticationService::init();
    // println!("{:?}", auth_result.await.unwrap());

    /////////////////////////////////////////
    //// Initiate [LipaNaMpesa] service ////
    ////////////////////////////////////////
    let lipa_na_mpesa_result = LipaNaMpesaService::init(
        1,
        254741542352,
        "CompanyNameLTD".to_string(),
        "The payment has been processed successfully".to_string(),
    );
    println!("{:?}", lipa_na_mpesa_result.await);
}
