use std::borrow::Cow;
use chrono::{ DateTime, LocalResult, TimeZone, Utc };
use crate::{ millis_to_utc, valid_list, hmac::hmac_sha1 };

const HMAC_FIELD: &str = "hmac";

#[derive(Debug, Default)]
pub struct HttpsData {
    /// This is the version of AML.
    pub v: Option<String>,

    /// Emergency number dialed.
    pub emergency_number: Option<String>,

    /// Source of activation (call or sms).
    pub source: Option<String>,

    /// Version number for thunderbird module.
    pub thunderbird_version: Option<String>,

    /// Date and time of the beginning of call (UTC format).
    pub time: Option<DateTime<Utc>>,

    /// Ground truth latitude (for testing).
    pub gt_location_latitude: Option<f64>,

    /// Ground truth longitude* (for testing).
    pub gt_location_longitude: Option<f64>,

    /// The WGS84 latitude in degrees. Latitude is truncated to 5 decimal points.
    pub location_latitude: Option<f64>,

    /// The WGS84 longitude in degrees. Longitude is truncated to 5 decimal points.
    pub location_longitude: Option<f64>,

    /// The date and time that the handset determined the location area specified in UTC.
    pub location_time: Option<DateTime<Utc>>,

    /// Vertical location in meters.
    pub location_altitude: Option<f64>,

    /// Floor label (as in elevator button floor label - may be non-numeric).
    pub location_floor: Option<f64>,

    /// The method used to determine the location area. String valued with `wifi`, `cell`, `gps` or `unknown`.
    pub location_source: Option<String>,

    /// Location accuracy in meters.
    pub location_accuracy: Option<f64>,

    /// Vertical accuracy in meters.
    pub location_vertical_accuracy: Option<f64>,

    /// Confidence in location accuracy (percent between 0 and 1).
    pub location_confidence: Option<f64>,

    /// Bearing in degrees.
    pub location_bearing: Option<f64>,

    /// Speed in meters/second.
    pub location_speed: Option<f64>,

    /// Device phone number (may be mising).
    pub device_number: Option<String>,

    /// Device model.
    pub device_model: Option<String>,

    /// The SIM card identifier of the handset that has made the emergency call.
    pub device_imsi: Option<String>,

    /// The identifier of the handset that made the emergency call.
    pub device_imei: Option<String>,

    /// Integrated Circuit Card Identification number. 
    pub device_iccid: Option<String>,

    /// Home Mobile Country Code.
    pub cell_home_mcc: Option<i32>,

    /// Home mobile Network Code.
    pub cell_home_mnc: Option<i32>,

    /// Mobile Country Code, used to determine the network country that the emergency call was made on.
    pub cell_network_mcc: Option<i32>,

    /// Mobile Network Code, used to determine the mobile network used to make the emergency call.
    pub cell_network_mnc: Option<i32>,

    /// BCP 47 language tags (comma separated), in order from highest priority to lowest
    pub device_languages: Option<String>,

    /// Car crash date time
    pub adr_carcrash_time: Option<DateTime<Utc>>,

    /// Message Authentification Code
    pub hmac: Option<String>,
}

impl HttpsData {
    /// Verify the `hmac` field to authenticate the message.
    /// Assumes that HMAC is the last of the fields.
    ///
    /// ```
    /// use aml_lib::HttpsData;
    /// 
    /// const KEY: &str = "AML";
    ///
    /// let https = String::from(r#"v=1&device_number=%2B33611223344&location_latitude=0.85732&location_longitude=-4.26325&location_time=1604912121000&location_accuracy=10.4&location_source=GPS&location_certainty=83&hmac=f64c70eb238bb239e00e8ac8c023bf2b5d3c41dd"#);
    /// if HttpsData::is_authenticated(&https, KEY.as_bytes()) {
    ///     let data = HttpsData::from_urlencoded(&https);
    ///     /* Do something */
    /// }
    /// ```
    pub fn is_authenticated<S: AsRef<str>>(payload: S, key: &[u8]) -> bool {
        let splitted: Vec<&str> = payload.as_ref().split(&format!("&{}=", HMAC_FIELD)).collect();

        if splitted.len() != 2 { 
            return false; 
        }

        let message = splitted[0];
        let hmac = hex::encode(hmac_sha1(key, message.as_bytes()));

        hmac.eq(splitted[1])
    }

    /// Parse a HTTPS AML message. That assumes it is an URL encoded string. 
    ///
    /// ```
    /// use aml_lib::HttpsData;
    /// 
    /// let https = String::from(r#"v=1&device_number=%2B33611223344&location_latitude=0.85732&location_longitude=-4.26325&location_time=1604912121000&location_accuracy=10.4&location_source=GPS&location_certainty=83&hmac=f64c70eb238bb239e00e8ac8c023bf2b5d3c41dd"#);
    /// 
    /// let https_data = HttpsData::from_urlencoded(&https);
    /// assert_eq!(https_data.location_latitude, Some(0.85732));
    /// ```    
    pub fn from_urlencoded<S: AsRef<str>>(payload: S) -> Self {
        let mut https_data: HttpsData = Default::default();

        let attributes: Vec<(Cow<str>, Cow<str>)> =
            url::form_urlencoded::parse(payload.as_ref().as_bytes())
                .into_iter()
                .collect();

        for (key, value) in attributes {
            match (key.as_ref(), value.as_ref().trim()) {
                ("v", val) => https_data.v = Some(val.to_string()),
                ("emergency_number", val) => https_data.emergency_number = Some(val.to_string()),
                ("source", val) => {
                    https_data.source = valid_list!(val.to_lowercase(), "call", "sms")
                }
                ("thunderbird_version", val) => {
                    https_data.thunderbird_version = Some(val.to_string())
                }
                ("time", val) => https_data.time = millis_to_utc!(val),

                ("gt_location_latitude", val) => {
                    https_data.gt_location_latitude = val.parse::<f64>().ok()
                }
                ("gt_location_longitude", val) => {
                    https_data.gt_location_longitude = val.parse::<f64>().ok()
                }

                ("location_latitude", val) => {
                    https_data.location_latitude = val.parse::<f64>().ok()
                }
                ("location_longitude", val) => {
                    https_data.location_longitude = val.parse::<f64>().ok()
                }
                ("location_time", val) => https_data.location_time = millis_to_utc!(val),
                ("location_altitude", val) => {
                    https_data.location_altitude = val.parse::<f64>().ok()
                }
                ("location_source", val) => {
                    https_data.location_source =
                        valid_list!(val.to_lowercase(), "gps", "wifi", "cell", "unknown")
                }
                ("location_accuracy", val) => {
                    https_data.location_accuracy = val.parse::<f64>().ok()
                }
                ("location_vertical_accuracy", val) => {
                    https_data.location_vertical_accuracy = val.parse::<f64>().ok()
                }
                ("location_confidence", val) => {
                    https_data.location_confidence = val.parse::<f64>().ok()
                }
                ("location_bearing", val) => https_data.location_bearing = val.parse::<f64>().ok(),
                ("location_speed", val) => https_data.location_speed = val.parse::<f64>().ok(),

                ("device_number", val) => https_data.device_number = Some(val.to_string()),
                ("device_model", val) => https_data.device_model = Some(val.to_string()),
                ("device_imsi", val) => https_data.device_imsi = Some(val.to_string()),
                ("device_imei", val) => https_data.device_imei = Some(val.to_string()),
                ("device_iccid", val) => https_data.device_iccid = Some(val.to_string()),

                ("cell_home_mcc", val) => https_data.cell_home_mcc = val.parse::<i32>().ok(),
                ("cell_home_mnc", val) => https_data.cell_home_mnc = val.parse::<i32>().ok(),
                ("cell_network_mcc", val) => https_data.cell_network_mcc = val.parse::<i32>().ok(),
                ("cell_network_mnc", val) => https_data.cell_network_mnc = val.parse::<i32>().ok(),
                
                ("device_languages", val) => https_data.device_languages = Some(val.to_string()),
                ("adr_carcrash_time", val) => https_data.adr_carcrash_time = millis_to_utc!(val),
                ("hmac", val) => https_data.hmac = Some(val.to_string()),

                (_, _) => (),
            }
        }

        https_data
    }
}
