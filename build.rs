extern crate bindgen;

use std::env;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
mod config {
    pub const DEFINES: [&str; 8] = [
        "-DNDEBUG",
        "-D_LARGEFILE_SOURCE",
        "-D_CONSOLE",
        "-D_FILE_OFFSET_BITS=64",
        "-DSAPonUNIX",
        "-DSAPwithUNICODE",
        "-DSAPwithTHREADS",
        "-DSAPonLIN",
    ];
    pub const LIBS: [&str; 2] = ["sapnwrfc", "sapucum"];
    pub const LINK_ARGS: [&str; 0] = [];
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod config {
    pub const DEFINES: [&str; 18] = [
        "-DSAPonNT",
        "-D_CRT_NON_CONFORMING_SWPRINTFS",
        "-D_CRT_SECURE_NO_DEPRECATE",
        "-D_CRT_NONSTDC_NO_DEPRECATE",
        "-D_AFXDLL",
        "-DWIN32",
        "-D_WIN32_WINNT=0x0502",
        "-DWIN64",
        "-D_AMD64_",
        "-DNDEBUG",
        "-DSAPwithUNICODE",
        "-DUNICODE",
        "-D_UNICODE",
        "-DSAPwithTHREADS",
        "-D_ATL_ALLOW_CHAR_UNSIGNED",
        "-D_LARGEFILE_SOURCE",
        "-D_CONSOLE",
        "-DSAP_PLATFORM_MAKENAME=ntintel",
    ];
    pub const LIBS: [&str; 2] = ["sapnwrfc", "libsapucum"];
    pub const LINK_ARGS: [&str; 9] = [
        "/NXCOMPAT",
        "/STACK:0x2000000",
        "/SWAPRUN:NET",
        "/DEBUG",
        "/OPT:REF",
        "/DEBUGTYPE:CV,FIXUP",
        "/MACHINE:amd64",
        "/nologo",
        "/LTCG",
    ];
}

const ALLOWED_TYPES: [&str; 24] = [
    "SAP_UC",
    "SAP_RAW",
    "RFC_CHAR",
    "RFC_NUM",
    "RFC_BYTE",
    "RFC_BCD",
    "RFC_INT1",
    "RFC_INT2",
    "RFC_INT",
    "RFC_FLOAT",
    "RFC_DATE",
    "RFC_TIME",
    "RFC_DECF16",
    "RFC_DECF34",
    "RFC_UTCLONG",
    "RFC_UTCSECOND",
    "RFC_UTCMINUTE",
    "RFC_DTDAY",
    "RFC_DTWEEK",
    "RFC_TSECOND",
    "RFC_TMINUTE",
    "RFC_CDAAY",
    "RFC_TID",
    "RFC_UNITTID",
];

fn main() {
    // Get the path to the SAP NWRFC SDK
    let sdk = PathBuf::from(env::var("SAPNWRFC_HOME").expect(
        "SAPNWRFC_HOME environment variable not set! \
                    Please set it to the root directory of the SAP Netweaver RFC SDK.",
    ));

    // Set the path to the libs
    println!(
        "cargo:rustc-link-search={}",
        sdk.join("lib").to_string_lossy()
    );

    // Tell cargo to link against the sapnwrfc libs
    for lib in config::LIBS {
        println!("cargo:rustc-link-lib={lib}");
    }

    // Set additional link args
    for link_arg in config::LINK_ARGS {
        println!("cargo:rustc-link-arg={}", link_arg);
    }

    // Add the bindgen wrapper
    let mut bindings = bindgen::Builder::default()
        // Add custom build arguments for the clang compiler
        .clang_args(config::DEFINES)
        // Build bindings for the sapnwrfc.h header
        .header(sdk.join("include").join("sapnwrfc.h").to_string_lossy())
        // Tell cargo to invalidate the build results if any of the included
        // headers changes
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Derive some standard traits
        .derive_default(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        // Use rust enums as default
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        // Allow all methods starting with Rfc to be exported
        .allowlist_function("Rfc.*")
        // Allow all types starting with RFC_ to be exported
        .allowlist_type("RFC_.*")
        // Don't include the documentation as comments
        .generate_comments(false);

    // Add the types to export
    for allowed_type in ALLOWED_TYPES {
        bindings = bindings.allowlist_type(allowed_type);
    }

    // generate the bindings
    let out_path = PathBuf::from("src");
    bindings
        .generate()
        .expect("Unable to generate library bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write library bindings");
}
