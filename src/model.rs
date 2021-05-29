use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub struct SmsDataV1 {
    /// The header shall appear at the beginning of the SMS message.
    pub header: Option<String>,

    /// The WGS84 latitude.
    pub latitude: Option<f64>,

    /// The WGS84 longitude.
    pub longitude: Option<f64>,

    /// The radius of the location area in metres.
    pub radius: Option<f64>,

    /// The date and time that the handset determined the location area specified in UTC (Greenwich).
    pub time_of_positioning: Option<DateTime<Utc>>,

    /// The Level of Confidence is a percentage probability that the mobile handset is within the area being communicated.
    pub level_of_confidence: Option<f64>,

    /// The method used to determine the location area.
    pub positioning_method: Option<String>,

    /// The SIM card identifier of the handset that has made the emergency call.
    pub imsi: Option<String>,

    /// The identifier of the handset that made the emergency call.
    pub imei: Option<String>,

    /// Mobile Country Code, used to determine the network country that the emergency call was made on.
    pub network_mcc: Option<String>,

    /// Mobile Network Code, used to determine the mobile network used to make the emergency call.
    pub network_mnc: Option<String>,

    /// The length of the entire SMS message including the header and the length attribute.
    pub message_length: Option<usize>,
}

#[derive(Debug, Default)]
pub struct SmsDataV2 {
    /// The header shall appear at the beginning of the SMS message.
    pub header: Option<String>,

    pub emergency_number: Option<String>,

    pub beginning_of_call: Option<DateTime<Utc>>,

    /// The WGS84 latitude.
    pub latitude: Option<f64>,

    /// The WGS84 longitude.
    pub longitude: Option<f64>,

    /// The radius of the location area in metres.
    pub accuracy: Option<f64>,

    /// The date and time that the handset determined the location area specified in UTC (Greenwich).
    pub time_of_positioning: Option<DateTime<Utc>>,

    /// The Level of Confidence is a percentage probability that the mobile handset is within the area being communicated.
    pub level_of_confidence: Option<f64>,

    pub altitude: Option<f64>,

    pub vertical_accuracy: Option<f64>,

    /// The method used to determine the location area.
    pub positioning_method: Option<String>,

    /// The identifier of the handset that made the emergency call.
    pub imei: Option<String>,

    /// Mobile Country Code, used to determine the network country that the emergency call was made on.
    pub network_mcc: Option<String>,

    /// Mobile Network Code, used to determine the mobile network used to make the emergency call.
    pub network_mnc: Option<String>,

    /// Home mobile Country Code,
    pub home_mcc: Option<String>,

    /// Home mobile Network Code,
    pub home_mnc: Option<String>,

    /// Language tags (IETF BCP 47)
    pub language: Option<String>,
}
