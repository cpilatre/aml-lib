use crate::{ AmlError, SmsDataV2, SmsDataV1, HttpsData, SmsData, };
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
        match sms_data {
            SmsData::V1(v1) => Ok(v1.into()),
            SmsData::V2(v2) => Ok(v2.into()),
        }
    }

    pub fn from_data_sms(data_sms: &[u8]) -> Result<Self, AmlError> {
        let sms_data = SmsData::from_data(data_sms)?;
        match sms_data {
            SmsData::V1(v1) => Ok(v1.into()),
            SmsData::V2(v2) => Ok(v2.into()),
        }
    }

    pub fn from_base64_sms<S: AsRef<[u8]>>(base64_sms: S)-> Result<Self, AmlError> {
        match base64::decode(base64_sms) {
            Ok(bin_sms) => Self::from_data_sms(&bin_sms),
            Err(_) => Err(AmlError::InvalidBase64),
        }
    }
}

impl From<SmsDataV1> for AmlData {
    fn from(sms_v1: SmsDataV1) -> Self {
        AmlData {
            version: sms_v1.header,
            latitude: sms_v1.latitude,
            longitude: sms_v1.longitude,
            time_of_positioning: sms_v1.time_of_positioning,
            positioning_method: sms_v1.positioning_method,
            accuracy: sms_v1.radius,
            confidence: sms_v1.level_of_confidence,
            imsi: sms_v1.imsi,
            imei: sms_v1.imei,
            network_mcc: sms_v1.network_mcc,
            network_mnc: sms_v1.network_mnc,
            transport: "sms".to_string(),
            ..Default::default()
        }
    }
}

impl From<SmsDataV2> for AmlData {
    fn from(sms_v2: SmsDataV2) -> Self {
        AmlData {
            version: sms_v2.header,
            emergency_number: sms_v2.emergency_number,
            beginning_of_call: sms_v2.beginning_of_call,
            latitude: sms_v2.latitude,
            longitude: sms_v2.longitude,
            accuracy: sms_v2.accuracy,
            time_of_positioning: sms_v2.time_of_positioning,
            confidence: sms_v2.level_of_confidence,
            altitude: sms_v2.altitude,
            vertical_accuracy: sms_v2.vertical_accuracy,
            positioning_method: sms_v2.positioning_method,
            imei: sms_v2.imei,
            network_mcc: sms_v2.network_mcc,
            network_mnc: sms_v2.network_mnc,
            home_mcc: sms_v2.home_mcc,
            home_mnc: sms_v2.home_mnc,
            language: sms_v2.language,
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

