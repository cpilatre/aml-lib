use crate::{ AmlError, HttpsData, SmsData};
use chrono::{ DateTime, Utc };

#[derive(Debug, Default)]
pub struct AmlData {
    /// See [`SmsData::header`] or [`HttpsData::v`]
    pub version: Option<String>,

    /// See [`SmsData::emergency_number`] or [`HttpsData::emergency_number`]
    pub emergency_number: Option<String>,

    /// See [`HttpsData::source`]
    pub source_of_activation: Option<String>,

    /// See [`SmsData::beginning_of_call`] or [`HttpsData::time`]
    pub beginning_of_call: Option<DateTime<Utc>>,

    /// See [`SmsData::latitude`] or [`HttpsData::location_latitude`]
    pub latitude: Option<f64>,

    /// See [`SmsData::longitude`] or [`HttpsData::location_longitude`]
    pub longitude: Option<f64>,

    /// See [`SmsData::time_of_positioning`] or [`HttpsData::location_time`]
    pub time_of_positioning: Option<DateTime<Utc>>,

    /// See [`SmsData::altitude`] or [`HttpsData::location_altitude`]
    pub altitude: Option<f64>,

    /// See [`HttpsData::location_floor`]
    pub floor: Option<f64>,

    /// See [`SmsData::positioning_method`] or [`HttpsData::location_source`]
    pub positioning_method: Option<String>,

    /// See [`SmsData::accuracy`] or [`HttpsData::location_accuracy`]
    pub accuracy: Option<f64>,

    /// See [`SmsData::vertical_accuracy`] or [`HttpsData::location_vertical_accuracy`]
    pub vertical_accuracy: Option<f64>,

    /// See [`SmsData::level_of_confidence`] or [`HttpsData::location_confidence`]
    pub confidence: Option<f64>,

    /// See [`HttpsData::location_bearing`]
    pub bearing: Option<f64>,

    /// See [`HttpsData::location_speed`]
    pub speed: Option<f64>,

    /// See [`HttpsData::device_number`]
    pub device_number: Option<String>,

    /// See [`HttpsData::device_model`]
    pub model: Option<String>,

    /// See [`SmsData::imsi`] or [`HttpsData::device_imsi`]
    pub imsi: Option<String>,

    /// See [`SmsData::imei`] or [`HttpsData::device_imei`]
    pub imei: Option<String>,

    /// See [`HttpsData::device_iccid`]
    pub iccid: Option<String>,

    /// See [`SmsData::home_mcc`] or [`HttpsData::cell_home_mcc`]
    pub home_mcc: Option<String>,

    /// See [`SmsData::home_mnc`] or [`HttpsData::cell_home_mnc`]
    pub home_mnc: Option<String>,

    /// See [`SmsData::network_mcc`] or [`HttpsData::cell_network_mcc`]
    pub network_mcc: Option<String>,

    /// See [`SmsData::network_mnc`] or [`HttpsData::cell_network_mnc`]
    pub network_mnc: Option<String>,

    /// See [`SmsData::language`] or [`HttpsData::device_languages`]
    pub language: Option<String>,

    /// Where the location comes from: `sms` or `https`
    pub transport: String,
}

impl AmlData {
    /// Parse a HTTPS AML message. See [`HttpsData::from_urlencoded`].
    pub fn from_https(payload: &str) -> Result<Self, AmlError> {
        let https_data = HttpsData::from_urlencoded(payload);
        Ok(https_data.into())
    }

    /// Parse a SMS text. See [`SmsData::from_text`].
    pub fn from_text_sms<S: AsRef<str>>(text_sms: S) -> Result<Self, AmlError> {
        let sms_data = SmsData::from_text(text_sms)?;
        Ok(sms_data.into())
    }

    /// Parse a SMS data. See [`SmsData::from_data`].
    pub fn from_data_sms(bin_sms: &[u8]) -> Result<Self, AmlError> {
        let sms_data = SmsData::from_binary(bin_sms)?;
        Ok(sms_data.into())
    }

    /// Parse a base64 encoded SMS data. See [`AmlData::from_data_sms`].
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
            language: https_data.device_languages,
            transport: "https".to_string(),
            ..Default::default()
        }
    }
}

