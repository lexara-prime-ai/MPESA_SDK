#[derive(Debug)]
pub enum Environment {
    Live,
    Sandbox,
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Config<'c> {
    BASE_URL: String,
    TOKEN: &'c String,
}

impl<'c> Config<'c> {
    pub fn new(environment: Environment, token: &'c String) -> Self {
        match environment {
            Environment::Live => Self {
                // To do -> Update existing URL with [LIVE] url.
                BASE_URL: "https://sandbox.safaricom.co.ke/".to_string(),
                TOKEN: token,
            },
            Environment::Sandbox => Self {
                BASE_URL: "https://sandbox.safaricom.co.ke/".to_string(),
                TOKEN: token,
            },
        }
    }
}
