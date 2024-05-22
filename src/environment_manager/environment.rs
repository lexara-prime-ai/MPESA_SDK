#[derive(Debug)]
pub enum Environment {
    Live,
    Sandbox,
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Config {
    BASE_URL: String,
}

impl Config {
    pub fn new(environment: Environment) -> Self {
        match environment {
            Environment::Live => Self {
                // To do -> Update existing URL with [LIVE] url.
                BASE_URL: "https://live.safaricom.co.ke/".to_string(),
            },
            Environment::Sandbox => Self {
                BASE_URL: "https://sandbox.safaricom.co.ke/".to_string(),
            },
        }
    }

    pub fn get_environment() -> String {
        #[allow(non_snake_case)]
        let ENVIRONMENT = std::env::var("ENVIRONMENT")
            .expect("Failed to retrieve [ENVIRONMENT].")
            .trim()
            .to_string();

        ENVIRONMENT.to_lowercase()
    }

    pub fn set_environment() -> String {
        let env_config = Config::get_environment();
        let environment = match env_config.as_str() {
            "live" => Environment::Live,
            _ => Environment::Sandbox,
        };

        let config = Config::new(environment);

        config.BASE_URL
    }
}
