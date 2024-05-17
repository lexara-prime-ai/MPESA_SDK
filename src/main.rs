/*
    This crate provides a streamlined interface for
    integrating M-Pesa, a widely used mobile money
    service, into your applications.

    Author: Irfan Ghat
    License: MIT
*/

use service_requests::authentication::AuthenticationService;

// Register all modules
mod models;
mod service_endpoints;
mod service_requests;

fn main() {
    AuthenticationService::init();
}
