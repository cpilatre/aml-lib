mod aml;
mod https;
mod model;
mod sms;
mod tools;

pub use aml::AmlData;
pub use https::HttpsData;
pub use model::{ SmsDataV1, SmsDataV2 };
pub use sms::SmsData;

#[derive(Debug)]
pub enum AmlError {
    UnimplementedVersion,
    InvalidBase64,
}