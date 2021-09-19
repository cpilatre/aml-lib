mod aml;
mod https;
mod sms;
mod tools;
mod hmac;

pub use aml::AmlData;
pub use https::HttpsData;
pub use sms::SmsData;

#[derive(Debug)]
pub enum AmlError {
    /// You have tried to parse an unimplemented version of SMS AML.
    UnimplementedVersion,

    /// You have tried to parse an corrumpted base64 SMS data.
    InvalidBase64,
}

impl std::error::Error for AmlError {}

impl std::fmt::Display for AmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match *self {
            AmlError::UnimplementedVersion => {
                String::from("You have tried to parse an unimplemented version of SMS AML")
            }
            AmlError::InvalidBase64 => {
                String::from("You have tried to parse an corrumpted base64 SMS data")
            }
        };
        write!(f, "Error: {}", text)
    }
}