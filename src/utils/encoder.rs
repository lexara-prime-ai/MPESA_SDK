use anyhow::{Error, Ok};
use base64::{engine::general_purpose::STANDARD, Engine};

pub struct Encoder;

impl Encoder {
    pub fn init(short_code: u64, passkey: String, timestamp: &String) -> Result<String, Error> {
        let format_string = format!("{}{}{}", short_code, passkey, timestamp);
        let password = STANDARD.encode(format_string);
        Ok(password)
    }
}
