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