use aml_lib::{AmlData, SmsData, HttpsData};

#[test]
fn from_text_sms() {
    let sms_text = String::from(
        r#"A"ML=1;lt=48.82639;lg=-2.36619;rd=52;top=20191112112928;lc=68;pm=G;si=208201771948415;ei=353472104343540;mcc=208;mnc=20;ml=128"#,
    );

    let sms_data = SmsData::from_text(&sms_text);
    if let Ok(sms) = sms_data {
        assert!(sms.latitude == Some(48.82639), "Parsing failed : {:?}", sms);
    } else {
        panic!("Error text SMS");
    }

    let aml = AmlData::from_text_sms(&sms_text).unwrap();
    assert!(
        aml.imei == Some("353472104343540".to_string()),
        "Parsing failed : {:?}",
        aml
    );
}

#[test]
fn from_text_sms_v2() {
    let sms_text = String::from(
        r#"A"ML=2;en=+15555555555;et=1593187189;lo=-37.42175,-122.08461,2000.1;lt=-9999;lc=68;lz=-100.1,100.1;ls=G;ei=358239059042542;nc=310260;hc=310260;lg=en-US"#,
    );

    let sms_data = SmsData::from_text(&sms_text);
    if let Ok(sms) = sms_data {
        assert!(
            sms.latitude == Some(-37.42175),
            "Parsing failed : {:?}",
            sms
        );
    } else {
        panic!("Error AML SMS v2");
    }
}

#[test]
fn from_data_sms() {
    let input = "415193D98BEDD8F4DEECE6A2C962B7DA8E7DEEB56232990B86A3D9623B39B92783EDE86F784F068BD560B6D80C1683E568B81D7BDCB3E176F076EFB89BA77B39DCCD56A3C966B15D39DD9BD570B2590E56CBC168B21A4DB66B8FC7BD590CB66BBBC73D990DB66BB37B31D90C";
    let decoded = hex::decode(input).expect("Decoding failed");

    let sms_data = SmsData::from_data(&decoded);
    if let Ok(sms) = sms_data {
        assert!(sms.latitude == Some(37.42175), "Parsing failed : {:?}", sms);
    } else {
        panic!("Error data SMS");
    }

    let aml = AmlData::from_data_sms(&decoded).unwrap();
    assert!(
        aml.imei == Some("358239059042542".to_string()),
        "Parsing failed : {:?}",
        aml
    );
}

#[test]
fn from_https() {
    let https = r#"v=1&device_number=%2B447477593102&location_latitude=55.85732&location_longitude=-4.26325&location_time=1476189444435&location_accuracy=10.4&location_source=GPS&location_certainty=83&location_altitude=0.0&location_floor=5&device_model=ABC+ABC+Detente+530&device_imei=354773072099116&device_imsi=234159176307582&device_os=AOS&cell_carrier=&cell_home_mcc=234&cell_home_mnc=15&cell_network_mcc=234&cell_network_mnc=15&cell_id=0213454321"#;

    let aml = AmlData::from_https(&https).unwrap();
    assert!(
        aml.positioning_method == Some("gps".to_string()),
        "Parsing failed : {:?}",
        aml
    );
}

#[test]
fn authenticate() {
    let https = String::from(r#"v=1&device_number=%2B33611223344&location_latitude=0.85732&location_longitude=-4.26325&location_time=1604912121000&location_accuracy=10.4&location_source=GPS&location_certainty=83&hmac=f64c70eb238bb239e00e8ac8c023bf2b5d3c41dd"#);

    assert!(HttpsData::is_authenticated(https, "AML".as_bytes()));
}