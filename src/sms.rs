use std::collections::HashMap;
use chrono::{DateTime, LocalResult, NaiveDateTime, TimeZone, Utc};
use crate::{seconds_to_utc, valid_list, AmlError};

const DATETIME_FORMAT: &str = "%Y%m%d%H%M%S";

#[derive(Debug, Default)]
pub struct  SmsData {
    /// The header shall appear at the beginning of the SMS message.
    /// This is the version of AML.
    pub header: Option<String>,

    /// The emergency number dialed (i.e. 112, 911, ...).
    pub emergency_number: Option<String>,

    /// The beginning of the emergency call (UTC).
    pub beginning_of_call: Option<DateTime<Utc>>,

    /// The WGS84 latitude in degrees. Latitude is truncated to 5 decimal points.
    pub latitude: Option<f64>,

    /// The WGS84 longitude in degrees. Longitude is truncated to 5 decimal points.
    pub longitude: Option<f64>,

    /// Accuracy of location in meters. An accuracy of 0 represents unknown.
    pub accuracy: Option<f64>,

    /// The date and time that the handset determined the location area specified in UTC.
    /// This field may be ignored if location or beginning of call fields are valued to None.
    pub time_of_positioning: Option<DateTime<Utc>>,

    /// The Level of Confidence is a percentage probability that the mobile handset is within the area being communicated.
    pub level_of_confidence: Option<f64>,

    /// Vertical location in meters (truncated to 1 decimal point).
    /// This field may be ignored if location field is valued to None.
    pub altitude: Option<f64>,

    /// Vertical accuracy in meters (truncated to 1 decimal point).
    /// Accuracy of 0 represents unknown.
    /// This field may be ignored if location field is valued to None.
    pub vertical_accuracy: Option<f64>,

    /// The method used to determine the location area.
    /// One char string valued with `"W"` (wifi), `"C"` (cell), `"G"` (GNSS), `"F"` (fused) or `"U"` (unknown).
    /// This field may be ignored if location fields are valued to None.
    pub positioning_method: Option<String>,

    /// The SIM card identifier of the handset that has made the emergency call.
    pub imsi: Option<String>,

    /// The identifier of the handset that made the emergency call.
    pub imei: Option<String>,

    /// Mobile Country Code, used to determine the network country that the emergency call was made on.
    pub network_mcc: Option<usize>,

    /// Mobile Network Code, used to determine the mobile network used to make the emergency call.
    pub network_mnc: Option<usize>,

    /// Home Mobile Country Code.
    pub home_mcc: Option<usize>,

    /// Home Mobile Network Code.
    pub home_mnc: Option<usize>,

    /// Language tags (IETF BCP 47).
    pub languages: Option<String>,

    /// (v1) The length of the entire SMS message including the header and the length attribute.
    pub message_length: Option<usize>,    

    /// SMS AML is validated for v1 if message length is equal to message_length.
    /// For v2, SMS AML is always validated. 
    pub is_validated: bool,
}

impl SmsData {
    /// Parse a SMS data.
    ///
    /// # Example
    ///
    /// ```
    /// use aml_lib::SmsData;
    /// 
    /// let input = "415193D98BEDD8F4DEECE6A2C962B7DA8E7DEEB56232990B86A3D9623B39B92783EDE86F784F068BD560B6D80C1683E568B81D7BDCB3E176F076EFB89BA77B39DCCD56A3C966B15D39DD9BD570B2590E56CBC168B21A4DB66B8FC7BD590CB66BBBC73D990DB66BB37B31D90C";
    /// let decoded = hex::decode(input).expect("Decoding failed");
    ///
    /// let sms_data = SmsData::from_data(&decoded);
    /// if let Ok(sms) = sms_data {
    ///     assert_eq!(sms.latitude, Some(37.42175));
    /// }
    /// ```
    pub fn from_data(bin_sms: &[u8]) -> Result<Self, AmlError> {
        let raw_sms: Vec<u8>;
        let text_sms: &str;

        raw_sms = Self::decode_7to8(bin_sms);
        text_sms = std::str::from_utf8(&raw_sms).unwrap_or_default();
        Self::from_text(text_sms)
    }

    /// Parse a SMS text.
    ///
    /// # Example
    ///
    /// ```
    /// use aml_lib::SmsData;
    /// 
    /// let sms_text = String::from(r#"A"ML=1;lt=48.82639;lg=-2.36619;rd=52;top=20191112112928;lc=68;pm=G;si=208201771948415;ei=353472104343540;mcc=208;mnc=20;ml=126"#);
    ///
    /// let sms_data = SmsData::from_text(&sms_text);
    /// if let Ok(sms) = sms_data {
    ///     assert_eq!(sms.latitude, Some(48.82639));
    /// }
    /// ```
    pub fn from_text<S: AsRef<str>>(text_sms: S) -> Result<Self, AmlError> {
        let properties = Self::get_properties(text_sms.as_ref());

        match properties.get(r#"A"ML"#) {
            Some(&"1") => {
                let mut sms_data = Self::from_text_v1(properties);
                if let Some(len) = sms_data.message_length {
                    sms_data.is_validated = len == text_sms.as_ref().len();
                };
                Ok(sms_data)
            },
            Some(&"2") => {
                let mut sms_data = Self::from_text_v2(properties);
                // By default AML SMS v2 is validate
                sms_data.is_validated = true;
                Ok(sms_data)
            },
            _ => Err(AmlError::UnimplementedVersion),
        }
    }

    fn from_text_v1(properties: HashMap<&str, &str>) -> Self {
        let mut sms: SmsData = Default::default();

        for (key, value) in properties {
            match (key, value) {
                (r#"A"ML"#, _) => sms.header = Some(value.to_string()),
                ("lg", _) => sms.longitude = value.parse::<f64>().ok(),
                ("lt", _) => sms.latitude = value.parse::<f64>().ok(),
                ("rd", _) => sms.accuracy = value.parse::<f64>().ok(),
                ("top", _) => {
                    if let Ok(ndt) = NaiveDateTime::parse_from_str(&value, DATETIME_FORMAT) {
                        sms.time_of_positioning = Some(DateTime::<Utc>::from_utc(ndt, Utc));
                    }
                }
                ("lc", _) => sms.level_of_confidence = value.parse::<f64>().ok(),
                ("pm", _) => {
                    sms.positioning_method =
                        valid_list!(value.to_uppercase(), "G", "W", "C", "U")
                }
                ("si", _) => sms.imsi = Some(value.to_string()),
                ("ei", _) => sms.imei = Some(value.to_string()),
                ("mcc", _) => sms.network_mcc = value.parse::<usize>().ok(),
                ("mnc", _) => sms.network_mnc = value.parse::<usize>().ok(), //Some(value.to_string()),
                ("ml", _) => sms.message_length = value.parse::<usize>().ok(),
                (_, _) => (),
            }
        }

        sms
    }

    fn from_text_v2(properties: HashMap<&str, &str>) -> Self {
        let mut sms: SmsData = Default::default();
        let (mut et_opt, mut lt_opt): (Option<i64>, Option<i64>) = Default::default();

        for (key, value) in properties {
            match (key, value) {
                (r#"A"ML"#, _) => sms.header = Some(value.to_string()),
                ("en", _) => sms.emergency_number = Some(value.to_string()),
                ("et", _) => et_opt = value.parse::<i64>().ok(),
                ("lo", _) => {
                    let mut values: Vec<Option<f64>> =
                        value.split(',').map(|i| i.parse::<f64>().ok()).collect();
                    values.resize(3, None);
                    sms.latitude = values[0];
                    sms.longitude = values[1];
                    sms.accuracy = values[2];
                }
                ("lt", _) => lt_opt = value.parse::<i64>().ok(),
                ("lc", _) => sms.level_of_confidence = value.parse::<f64>().ok(),
                ("lz", _) => {
                    let mut values: Vec<Option<f64>> =
                        value.split(',').map(|i| i.parse::<f64>().ok()).collect();
                    values.resize(2, None);
                    sms.altitude = values[0];
                    sms.vertical_accuracy = values[1];
                }
                ("ls", _) => {
                    sms.positioning_method =
                        valid_list!(value.to_uppercase(), "G", "W", "C", "U", "F")
                }
                ("ei", _) => sms.imei = Some(value.to_string()),
                ("nc", _) => {
                    sms.network_mcc = value.get(..3).and_then(|s| s.parse::<usize>().ok());
                    sms.network_mnc = value.get(3..).and_then(|s| s.parse::<usize>().ok()); 
                }
                ("hc", _) => {
                    sms.home_mcc = value.get(..3).and_then(|s| s.parse::<usize>().ok());
                    sms.home_mnc = value.get(3..).and_then(|s| s.parse::<usize>().ok());
                }
                ("lg", _) => sms.languages = Some(value.to_string()),
                (_, _) => (),
            }
        }

        if let Some(et) = et_opt {
            sms.beginning_of_call = seconds_to_utc!(et);
            if let Some(lt) = lt_opt {
                sms.time_of_positioning = seconds_to_utc!(et + lt);
            }
        }

        sms
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
