# Rust bindings for the SAP NetWeaver RFC SDK

This crate uses [`bindgen`](https://crates.io/crates/bindgen) to generate bindings for
the SAP NetWeaver RFC libraries.

It consists of two layers of abstraction:

1) It directly exports the wrapped, unsafe C-Functions from the NetWeaver RFC using
   `nwrfc::_unsafe`.
2) It implements an abstraction layer protocol under `nwrfc::protocol`, which allows
   a more object-oriented access to the SAP NetWeaver RFC library.
   This layer is mostly inspired by SAP's own python implementation of the
   SAP NetWeaver RFC wrappers called [PyRFC](https://github.com/sap/pyrfc).

**_NOTE:_** Normal developers should use the abstraction layer protocol any try to avoid
the `nwrfc::_unsafe` layer. Nevertheless, this layer is exported to allow access to
functions which have not (yet) been implemented in the abstraction layer.

## How to include
Because of license limitations, we are not allowed to distribute this crate in
a pre-built fashion. Thus, you'd have to include the git repository as a dependency
instead:

```toml
[dependencies]
nwrfc = { git = "https://github.com/hansingt/rust-nwrfc.git" }
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
