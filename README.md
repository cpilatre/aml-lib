# aml-lib

[![Build Status](https://github.com/cpilatre/aml-lib/actions/workflows/build.yml/badge.svg)](https://github.com/cpilatre/aml-lib/actions?query=workflow%3A%22build%22)
[![Test Status](https://github.com/cpilatre/aml-lib/actions/workflows/test.yml/badge.svg)](https://github.com/cpilatre/aml-lib/actions?query=workflow%3A%22test%22)

## About

aml-lib is a 100% Rust library for dealing with AML (Advanced Mobile Location) messages.

For v1 SMS and HTTPS see [ETSI TS 103 625 V1.1.1 (2019-12)](https://www.etsi.org/deliver/etsi_ts/103600_103699/103625/01.01.01_60/ts_103625v010101p.pdf).

## Main features

- SMS AML v1 and V2 compliance
- Accepts text and data SMS (with binary or Base64 encoded sources)
- HTTPS AML with hmac-sha1 authentification
- Provides a generic AML format

## Installation

Manually add `aml-lib` to your `Cargo.toml` file :

```toml
[dependencies]
aml-lib = "*"
```

Or use [cargo-edit](https://crates.io/crates/cargo-edit) :

```bash
cargo add aml-lib
```

## Usage

- If you receive AML SMS only, you could use `SmsData` structure to parse text sms
 (send by Apple device) or data SMS (send by Android device).

```rust
use aml_lib::SmsData;

let sms_text = String::from(r#"A"ML=1;lt=48.82639;lg=-2.36619;rd=52;top=20191112112928;lc=68;pm=G;si=208201771948415;ei=353472104343540;mcc=208;mnc=20;ml=126"#);
    
let sms_data = SmsData::from_text(&sms_text);
if let Ok(sms) = sms_data {
    println!("{:#?}", sms);
}
```

- If you receive HTTPS AML message only, you could use `HttpsData` structure.

```rust
use aml_lib::HttpsData;

const KEY: &str = "AML";

let https = String::from(r#"v=1&device_number=%2B33611223344&location_latitude=0.85732&location_longitude=-4.26325&location_time=1604912121000&location_accuracy=10.4&location_source=GPS&location_certainty=83&hmac=f64c70eb238bb239e00e8ac8c023bf2b5d3c41dd"#);
if HttpsData::is_authenticated(&https, KEY.as_bytes()) {
    let https_data = HttpsData::from_urlencoded(&https);
    println!("{:#?}", https_data)
} 
```

- For both, SMS and HTTPS, you could use `AmlData` DTO structure to standardize
 AML data.

```rust
use aml_lib::AmlData;

let sms_text = r#"A"ML=1;lt=48.82639;lg=-2.36619;rd=52;top=20191112112928;lc=68;pm=G;si=208201771948415;ei=353472104343540;mcc=208;mnc=20;ml=126"#;
let aml_data_1 = AmlData::from_text_sms(&sms_text);
println!("{:?}", aml_data_1);
```

```rust
use aml_lib::AmlData;

let base64_data_sms = "QVGT2Yvt2PTe7OaiyWK32o597rViMpkLhqPZYjs5uSeD7ehveE8Gi9VgttgMFoPlaLgde9yz4Xbwdu+4m6d7OdzNVqPJZrFdOd2b1XCyWQ5Wy8FoshpNtmuPx71ZDLZru8c9mQ22a7N7MdkM";
let aml_data_2 = AmlData::from_base64_sms(base64_data_sms);
println!("{:?}", aml_data_2);
```

```rust
use aml_lib::{HttpsData, AmlData};

let https = r#"v=1&device_number=%2B33611223344&location_latitude=0.85732&location_longitude=-4.26325&location_time=1604912121000&location_accuracy=10.4&location_source=GPS&location_certainty=83&hmac=f64c70eb238bb239e00e8ac8c023bf2b5d3c41dd"#;
let https_data = HttpsData::from_urlencoded(https);
let aml_data_3: AmlData = https_data.into();
println!("{:?}", aml_data_3);
```

```rust
use aml_lib::{SmsData, AmlData};

let sms_data = SmsData::from_text(sms_text).unwrap();
let aml_data_4 = AmlData::from(sms_data);
println!("{:?}", aml_data_4);
```

## Release History

A short list of features, fixes and changes for each release is available in [CHANGELOG.md](https://github.com/cpilatre/aml-lib/blob/main/CHANGELOG.md).

## Contributing

Anyone is welcome to submit issues and pull requests.

## License

See [LICENSE](LICENSE).
