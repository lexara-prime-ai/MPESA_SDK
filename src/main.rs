// let headers = new Headers();
// headers.append("Authorization", "Bearer cFJZcjZ6anEwaThMMXp6d1FETUxwWkIzeVBDa2hNc2M6UmYyMkJmWm9nMHFRR2xWOQ==");
// fetch("https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials", { headers })
//   .then(response => response.text())
//   .then(result => console.log(result))
//   .catch(error => console.log(error));

use reqwest;
use reqwest::header;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Todo {
    pub userId: i32,
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let endpoint = "https://jsonplaceholder.typicode.com/todos";

    let response = client
        .get(endpoint)
        .send()
        .await
        .unwrap()
        .json::<Vec<Todo>>()
        .await;

    print!("{:#?}", response);
}
