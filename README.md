# MPESA SDK for Rust

This crate provides a streamlined interface for integrating **M-Pesa**, a widely used _mobile money service_, into your Rust applications. The SDK focuses on both **safety** and **speed**, making it easy for developers to interact with **Safaricom's Daraja**/**M-PESA API**.

**Author:** _Irfan Ghat_  
**License:** _MIT_

## Installation

To use the **MPESA SDK** in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
mpesa_sdk = "0.1.1"  # Current version [BETA]
```

## Modules

-   `authentication`: Handles authentication with the MPESA API.
-   `lipa_na_mpesa`: Manages Lipa na MPESA transactions.

## Environment Management

The SDK supports configuration through environment variables, making it easier to manage different environments (e.g., sandbox, production).

### Environment Variables

Set the following environment variables in your `.env` file or directly in your environment:

```env
USAGE_TOKEN=YOUR_USAGE_TOKEN
ENVIRONMENT=sandbox # For production use live.
AUTH_TOKEN=YOUR_AUTH_TOKEN
BUSINESS_SHORT_CODE=YOUR_SHORT_CODE
CALLBACK_URL=https://mydomain.com/path
PASSKEY=YOUR_PASS_KEY
``` 

## Example Usage

Below are examples of how to use the Authentication and Lipa na MPESA services provided by the SDK.

### Authentication Service

To initiate the Authentication service, use the `AuthenticationService::init` method. This method will handle the authentication process and return a result containing the authentication details.

```rust
use tokio::main;
use service_requests::authentication::AuthenticationService;

#[main]
async fn main() {
    let auth_result = AuthenticationService::init();
    match auth_result.await {
        Ok(auth_details) => println!("Authentication successful: {:?}", auth_details),
        Err(e) => eprintln!("Authentication failed: {:?}", e),
    }
}
```

### Lipa na MPESA Service

To initiate a Lipa na MPESA transaction, use the `LipaNaMpesaService::init` method. Provide the necessary parameters such as amount, phone number, company name, and transaction description.

```rust
use tokio::main;
use service_requests::lipa_na_mpesa::LipaNaMpesaService;

#[main]
async fn main() {
    let lipa_na_mpesa_result = LipaNaMpesaService::init(
        1, // Amount to transact
        254123456789, // Phone number in international format
        "CompanyNameLTD".to_string(), // Company name
        "The payment has been processed successfully".to_string() // Transaction description
    );

    match lipa_na_mpesa_result.await {
        Ok(transaction_details) => println!("Lipa na MPESA transaction successful: {:?}", transaction_details),
        Err(e) => eprintln!("Lipa na MPESA transaction failed: {:?}", e),
    }
}
``` 

## Detailed Module Descriptions

### Authentication Module

The `authentication` module provides methods to authenticate your application with the MPESA API. This is necessary to obtain the access token required for making further API calls.

**Methods:**

-   `AuthenticationService::init()`: Initiates the authentication process.

**Usage:**

```rust
use service_requests::authentication::AuthenticationService;

async fn authenticate() {
    let auth_result = AuthenticationService::init().await;
    match auth_result {
        Ok(auth_details) => println!("Authentication successful: {:?}", auth_details),
        Err(e) => eprintln!("Authentication failed: {:?}", e),
    }
}
``` 

### Lipa na MPESA Module

The `lipa_na_mpesa` module handles the Lipa na MPESA transactions. This service allows you to initiate a payment request to a customer's phone.

**Methods:**

-   `LipaNaMpesaService::init(amount: u32, phone_number: u64, company_name: String, description: String)`: Initiates a Lipa na MPESA transaction.

**Usage:**

```rust
use service_requests::lipa_na_mpesa::LipaNaMpesaService;

async fn initiate_payment() {
    let lipa_na_mpesa_result = LipaNaMpesaService::init(
        1, // Amount to transact
        254123456789, // Phone number in international format
        "CompanyNameLTD".to_string(), // Company name
        "The payment has been processed successfully".to_string() // Transaction description
    ).await;

    match lipa_na_mpesa_result {
        Ok(transaction_details) => println!("Transaction successful: {:?}", transaction_details),
        Err(e) => eprintln!("Transaction failed: {:?}", e),
    }
}
``` 

## Configuration Example

Below is an example of how the Lipa na MPESA service has been configured to read from the environment variables:

```rust
use service_requests::authentication::AuthenticationService;
use service_requests::lipa_na_mpesa::LipaNaMpesaService;
use endpoints::ServiceEndpoints;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string());
    let business_short_code = env::var("BUSINESS_SHORT_CODE").expect("BUSINESS_SHORT_CODE must be set");
    let callback_url = env::var("CALLBACK_URL").expect("CALLBACK_URL must be set");
    let passkey = env::var("PASSKEY").expect("PASSKEY must be set");

    // Configure MPESA Express URL
    let urls = ServiceEndpoints::new();
    let mpesa_express_url = format!("{}{}", environment, &urls.MpesaExpress.url);

    // Authenticate
    let auth_response = AuthenticationService::init().await.unwrap();
    let auth_token = format!("Bearer {}", auth_response.access_token);

    // Initiate Lipa na MPESA transaction
    let lipa_na_mpesa_result = LipaNaMpesaService::init(
        1, // Amount to transact
        254123456789, // Phone number in international format
        "CompanyNameLTD".to_string(), // Company name
        "The payment has been processed successfully".to_string() // Transaction description
    ).await;

    match lipa_na_mpesa_result {
        Ok(transaction_details) => println!("Transaction successful: {:?}", transaction_details),
        Err(e) => eprintln!("Transaction failed: {:?}", e),
    }
}
``` 

## Future Plans

### Current and Upcoming Implementations

Currently, the SDK exposes functions for **Consumer-to-Business** (_C2B_) interactions, allowing for payments from customers to businesses. This includes services like Lipa na MPESA.

In the near _future_, **Business-to-Business** (_B2B_) transactions will also be implemented. This will enable transactions between businesses, expanding the capabilities of the SDK to cover more use cases.

### Secure Callback Endpoint

Additionally, users who opt in will have a secure endpoint for the callback URL generated and configured on **Render Cloud** (_@render-inc_). This endpoint will handle the _**JSON dumps**_ after all transactions, reducing the effort required to set up a secure endpoint/domain.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub if you have any suggestions or improvements.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Support

For any questions or support, please contact the author at irfanghat@gmail.com.
