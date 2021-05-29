use crate::{millis_to_utc, valid_list};
use chrono::{DateTime, LocalResult, TimeZone, Utc};
use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct HttpsData {
    pub v: Option<String>,
    pub emergency_number: Option<String>,
    pub source: Option<String>,
    pub thunderbird_version: Option<String>,
    pub time: Option<DateTime<Utc>>,
    pub gt_location_latitude: Option<f64>,
    pub gt_location_longitude: Option<f64>,
    pub location_latitude: Option<f64>,
    pub location_longitude: Option<f64>,
    pub location_time: Option<DateTime<Utc>>,
    pub location_altitude: Option<f64>,
    pub location_floor: Option<f64>,
    pub location_source: Option<String>,
    pub location_accuracy: Option<f64>,
    pub location_vertical_accuracy: Option<f64>,
    pub location_confidence: Option<f64>,
    pub location_bearing: Option<f64>,
    pub location_speed: Option<f64>,
    pub device_number: Option<String>,
    pub device_model: Option<String>,
    pub device_imsi: Option<String>,
    pub device_imei: Option<String>,
    pub device_iccid: Option<String>,
    pub cell_home_mcc: Option<String>,
    pub cell_home_mnc: Option<String>,
    pub cell_network_mcc: Option<String>,
    pub cell_network_mnc: Option<String>,
}

impl HttpsData {
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

                ("cell_home_mcc", val) => https_data.cell_home_mcc = Some(val.to_string()),
                ("cell_home_mnc", val) => https_data.cell_home_mnc = Some(val.to_string()),
                ("cell_network_mcc", val) => https_data.cell_network_mcc = Some(val.to_string()),
                ("cell_network_mnc", val) => https_data.cell_network_mnc = Some(val.to_string()),
                (_, _) => (),
            }
        }

        https_data
    }
}
