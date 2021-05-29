use crate::{seconds_to_utc, valid_list, AmlError, SmsDataV1, SmsDataV2};
use chrono::{DateTime, LocalResult, NaiveDateTime, TimeZone, Utc};
use std::collections::HashMap;

const DATETIME_FORMAT: &str = "%Y%m%d%H%M%S";

#[derive(Debug)]
pub enum SmsData {
    V1(SmsDataV1),
    V2(SmsDataV2),
}

impl SmsData {
    /// Parse a SMS data. This method must never fail.
    ///
    /// # Example
    ///
    /// ```ignore
    ///     let input = "415193D98BEDD8F4DEECE6A2C962B7DA8E7DEEB56232990B86A3D9623B39B92783EDE86F784F068BD560B6D80C1683E568B81D7BDCB3E176F076EFB89BA77B39DCCD56A3C966B15D39DD9BD570B2590E56CBC168B21A4DB66B8FC7BD590CB66BBBC73D990DB66BB37B31D90C";
    ///     let decoded = hex::decode(input).expect("Decoding failed");
    ///
    ///     let sms_data = SmsData::from_data(&decoded).unwrap();
    ///     if let SmsData::V1(sms) = sms_data {
    ///         assert_eq!(sms.latitude, Some(37.42175));
    ///     }
    /// ```
    pub fn from_data(bin_sms: &[u8]) -> Result<Self, AmlError> {
        let raw_sms: Vec<u8>;
        let text_sms: &str;

        raw_sms = Self::decode_7to8(bin_sms);
        text_sms = std::str::from_utf8(&raw_sms).unwrap_or_default();
        Self::from_text(text_sms)
    }

    /// Parse a SMS text. This method must never fail.
    ///
    /// # Example
    ///
    /// ```ignore
    ///     let sms_text = String::from(r#"A"ML=1;lt=48.82639;lg=-2.36619;rd=52;top=20191112112928;lc=68;pm=G;si=208201771948415;ei=353472104343540;mcc=208;mnc=20;ml=128"#);
    ///
    ///     let sms_data = SmsData::from_text(&sms_text).unwrap();
    ///     if let SmsData::V1(sms) = sms_data {
    ///         assert_eq!(sms.latitude, Some(48.82639));
    ///     }
    /// ```
    pub fn from_text<S: AsRef<str>>(text_sms: S) -> Result<Self, AmlError> {
        let properties = Self::get_properties(text_sms.as_ref());

        match properties.get(r#"A"ML"#) {
            Some(&"1") => Ok(Self::from_text_v1(properties)),
            Some(&"2") => Ok(Self::from_text_v2(properties)),
            _ => Err(AmlError::UnimplementedVersion),
        }
    }

    fn from_text_v1(properties: HashMap<&str, &str>) -> Self {
        let mut sms_v1: SmsDataV1 = Default::default();

        for (key, value) in properties {
            match (key, value) {
                (r#"A"ML"#, _) => sms_v1.header = Some(value.to_string()),
                ("lg", _) => sms_v1.longitude = value.parse::<f64>().ok(),
                ("lt", _) => sms_v1.latitude = value.parse::<f64>().ok(),
                ("rd", _) => sms_v1.radius = value.parse::<f64>().ok(),
                ("top", _) => {
                    if let Ok(ndt) = NaiveDateTime::parse_from_str(&value, DATETIME_FORMAT) {
                        sms_v1.time_of_positioning = Some(DateTime::<Utc>::from_utc(ndt, Utc));
                    }
                }
                ("lc", _) => sms_v1.level_of_confidence = value.parse::<f64>().ok(),
                ("pm", _) => {
                    sms_v1.positioning_method =
                        valid_list!(value.to_uppercase(), "G", "W", "C", "U")
                }
                ("si", _) => sms_v1.imsi = Some(value.to_string()),
                ("ei", _) => sms_v1.imei = Some(value.to_string()),
                ("mcc", _) => sms_v1.network_mcc = Some(value.to_string()),
                ("mnc", _) => sms_v1.network_mnc = Some(value.to_string()),
                ("ml", _) => sms_v1.message_length = value.parse::<usize>().ok(),
                (_, _) => (),
            }
        }

        Self::V1(sms_v1)
    }

    fn from_text_v2(properties: HashMap<&str, &str>) -> Self {
        let mut sms_v2: SmsDataV2 = Default::default();
        let (mut et_opt, mut lt_opt): (Option<i64>, Option<i64>) = Default::default();

        for (key, value) in properties {
            match (key, value) {
                (r#"A"ML"#, _) => sms_v2.header = Some(value.to_string()),
                ("en", _) => sms_v2.emergency_number = Some(value.to_string()),
                ("et", _) => et_opt = value.parse::<i64>().ok(),
                ("lo", _) => {
                    let mut values: Vec<Option<f64>> =
                        value.split(',').map(|i| i.parse::<f64>().ok()).collect();
                    values.resize(3, None);
                    sms_v2.latitude = values[0];
                    sms_v2.longitude = values[1];
                    sms_v2.accuracy = values[2];
                }
                ("lt", _) => lt_opt = value.parse::<i64>().ok(),
                ("lc", _) => sms_v2.level_of_confidence = value.parse::<f64>().ok(),
                ("lz", _) => {
                    let mut values: Vec<Option<f64>> =
                        value.split(',').map(|i| i.parse::<f64>().ok()).collect();
                    values.resize(2, None);
                    sms_v2.altitude = values[0];
                    sms_v2.vertical_accuracy = values[1];
                }
                ("ls", _) => {
                    sms_v2.positioning_method =
                        valid_list!(value.to_uppercase(), "G", "W", "C", "U", "F")
                }
                ("ei", _) => sms_v2.imei = Some(value.to_string()),
                ("nc", _) => {
                    sms_v2.network_mcc = value.get(..3).map(|s| s.to_string());
                    sms_v2.network_mnc = value.get(3..).map(|s| s.to_string());
                }
                ("hc", _) => {
                    sms_v2.home_mcc = value.get(..3).map(|s| s.to_string());
                    sms_v2.home_mnc = value.get(3..).map(|s| s.to_string());
                }
                ("lg", _) => sms_v2.language = Some(value.to_string()),
                (_, _) => (),
            }
        }

        if let Some(et) = et_opt {
            sms_v2.beginning_of_call = seconds_to_utc!(et);
            if let Some(lt) = lt_opt {
                sms_v2.time_of_positioning = seconds_to_utc!(et + lt);
            }
        }

        Self::V2(sms_v2)
    }

    fn get_properties(s: &str) -> HashMap<&str, &str> {
        s.split(';')
            .map(|property| {
                let key_value: Vec<&str> = property.split('=').collect();
                (key_value[0].trim(), key_value[1].trim())
            })
            .filter(|key_val| !key_val.0.is_empty() && !key_val.1.is_empty())
            .collect()
    }

    // The definition of the 7 bit encoding can be found in ETSI TS 123 038 (see clause 6.1.2.1.1 specifically)
    fn decode_7to8(raw_bytes: &[u8]) -> Vec<u8> {
        let (mut bits_len, mut bits) = (0_u8, 0_u8);
        let mut out = Vec::<u8>::with_capacity(raw_bytes.len() << 1);

        for byte in raw_bytes {
            out.push(((byte << bits_len) | bits) & 0x7F);
            bits = byte >> (7 - bits_len);
            bits_len += 1;

            if bits_len == 7 {
                out.push(bits);
                bits = 0;
                bits_len = 0;
            }
        }

        out
    }
}
