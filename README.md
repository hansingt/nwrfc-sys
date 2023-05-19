# Unsafe Rust bindings for the SAP NetWeaver RFC SDK

This crate uses [`bindgen`](https://crates.io/crates/bindgen) to generate bindings for the
SAP NetWeaver RFC C-SDK and exports the unsafe bindings for direct usage.

> **_NOTE_**: Normal developers should use the
[`sapnwrfc`](https://github.com/hansingt/sapnwrfc-rust) crate instead, which is built upon
this crate and implements safe wrappers for most of the functions
and types exported by this crate.

This crate allows for direct interaction with SAP systems using the RFC protocol.
It does not implement any error handling or type conversion / checking.
Thus, all types exported by this crate are direct wrappers of the corresponding
C structs and all functions are the direct unsafe calls to the C functions.

## How to include
Because of license limitations, we are not allowed to distribute this crate in
a pre-built fashion. Thus, you'd have to include the git repository as a dependency
instead:

```toml
[dependencies]
sapnwrfc-sys = { git = "https://github.com/hansingt/sapnwrfc-sys.git" }
```

## How to build the bindings
Besides including the repository as mentioned above, the SAP NetWeaver RFC SDK is
required as well. It can be downloaded from the SAP Support Portal as described at
https://support.sap.com/en/product/connectors/nwrfcsdk.html

After downloading and unpacking the SDK, you need to set the `SAPNWRFC_HOME` variable
to the root directory of the NetWeaver RFC SDK. Afterward, you can build the bindings
and the crate by simply calling `cargo build`.

## License
This crate is licensed under the MIT License. For details see the [LICENSE](LICENSE)
file.
