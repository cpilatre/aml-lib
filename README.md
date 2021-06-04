# aml-lib

[![Build Status](https://github.com/cpilatre/aml-lib/actions/workflows/build.yml/badge.svg)](https://github.com/cpilatre/aml-lib/actions?query=workflow%3A%22build%22)
[![Test Status](https://github.com/cpilatre/aml-lib/actions/workflows/test.yml/badge.svg)](https://github.com/cpilatre/aml-lib/actions?query=workflow%3A%22test%22)

## About

aml-lib is a 100% Rust library for dealing with AML (Advanced Mobile Location) messages.

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

## Release History

A short list of features, fixes and changes for each release is available in [CHANGELOG.md](https://github.com/cpilatre/aml-lib/blob/main/CHANGELOG.md).

## Contributing

Anyone is welcome to submit issues and pull requests.

## License

See [LICENSE](LICENSE).
