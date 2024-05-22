/*
    This crate provides a streamlined interface for
    integrating M-Pesa, a widely used mobile money
    service, into your applications.

    Author: Irfan Ghat
    License: MIT
*/
#![allow(unused)]

use tokio::main;

// Register all modules
mod environment_manager;

mod models;
mod service_endpoints;

mod service_requests;
use service_requests::{prelude::*, *};

mod utils;
use utils::{prelude::*, *};

#[main]
async fn main() {
    display_version();

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
        254712345678,
        "CompanyNameLTD".to_string(),
        "The payment has been processed successfully".to_string(),
    );
    println!("{:?}", lipa_na_mpesa_result.await);
}
