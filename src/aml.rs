use crate::{ AmlError, HttpsData, SmsData};
use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub struct AmlData {
    pub version: Option<String>,
    pub emergency_number: Option<String>,
    pub source_of_activation: Option<String>,
    pub beginning_of_call: Option<DateTime<Utc>>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub time_of_positioning: Option<DateTime<Utc>>,
    pub altitude: Option<f64>,
    pub floor: Option<f64>,
    pub positioning_method: Option<String>,
    pub accuracy: Option<f64>,
    pub vertical_accuracy: Option<f64>,
    pub confidence: Option<f64>,
    pub bearing: Option<f64>,
    pub speed: Option<f64>,
    pub device_number: Option<String>,
    pub model: Option<String>,
    pub imsi: Option<String>,
    pub imei: Option<String>,
    pub iccid: Option<String>,
    pub home_mcc: Option<String>,
    pub home_mnc: Option<String>,
    pub network_mcc: Option<String>,
    pub network_mnc: Option<String>,
    pub language: Option<String>,
    pub transport: String,
}

impl AmlData {
    pub fn from_https(payload: &str) -> Result<Self, AmlError> {
        let https_data = HttpsData::from_urlencoded(payload);
        Ok(https_data.into())
    }

    pub fn from_text_sms<S: AsRef<str>>(text_sms: S) -> Result<Self, AmlError> {
        let sms_data = SmsData::from_text(text_sms)?;
        Ok(sms_data.into())
    }

    pub fn from_data_sms(data_sms: &[u8]) -> Result<Self, AmlError> {
        let sms_data = SmsData::from_data(data_sms)?;
        Ok(sms_data.into())
    }

    pub fn from_base64_sms<S: AsRef<[u8]>>(base64_sms: S)-> Result<Self, AmlError> {
        match base64::decode(base64_sms) {
            Ok(bin_sms) => Self::from_data_sms(&bin_sms),
            Err(_) => Err(AmlError::InvalidBase64),
        }
    }
}

impl From<SmsData> for AmlData {
    fn from(sms: SmsData) -> Self {
        AmlData {
            version: sms.header,
            emergency_number: sms.emergency_number,
            beginning_of_call: sms.beginning_of_call,
            latitude: sms.latitude,
            longitude: sms.longitude,
            accuracy: sms.accuracy,
            time_of_positioning: sms.time_of_positioning,
            confidence: sms.level_of_confidence,
            altitude: sms.altitude,
            vertical_accuracy: sms.vertical_accuracy,
            positioning_method: sms.positioning_method,
            imsi: sms.imsi,
            imei: sms.imei,
            network_mcc: sms.network_mcc,
            network_mnc: sms.network_mnc,
            home_mcc: sms.home_mcc,
            home_mnc: sms.home_mnc,
            language: sms.language,
            transport: "sms".to_string(),
            ..Default::default()
        }
    }
}

impl From<HttpsData> for AmlData {
    fn from(https_data: HttpsData) -> Self {
        AmlData {
            version: https_data.v,
            emergency_number: https_data.emergency_number,
            source_of_activation: https_data.source,
            beginning_of_call: https_data.time,
            latitude: https_data.location_latitude,
            longitude: https_data.location_longitude,
            time_of_positioning: https_data.location_time,
            altitude: https_data.location_altitude,
            floor: https_data.location_floor,
            positioning_method: https_data.location_source,
            accuracy: https_data.location_accuracy,
            vertical_accuracy: https_data.location_vertical_accuracy,
            confidence: https_data.location_confidence,
            bearing: https_data.location_bearing,
            speed: https_data.location_speed,
            device_number: https_data.device_number,
            model: https_data.device_model,
            imsi: https_data.device_imsi,
            imei: https_data.device_imei,
            iccid: https_data.device_iccid,
            home_mcc: https_data.cell_home_mcc,
            home_mnc: https_data.cell_home_mnc,
            network_mcc: https_data.cell_network_mcc,
            network_mnc: https_data.cell_network_mnc,
            transport: "https".to_string(),
            ..Default::default()
        }
    }
}

