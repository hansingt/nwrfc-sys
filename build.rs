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

fn set_ld_library_path(lib_dir: PathBuf) {
    let library_path = env::var("LD_LIBRARY_PATH").unwrap_or(String::from(""));
    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH={}:{}",
        lib_dir.to_string_lossy(),
        library_path
    );
}

fn main() {
    // Get the path to the SAP NWRFC SDK
    let sdk = PathBuf::from(env::var("SAPNWRFC_HOME").expect(
        "SAPNWRFC_HOME environment variable not set! \
                    Please set it to the root directory of the SAP Netweaver RFC SDK.",
    ));
    let lib_dir = sdk.join("lib");

    // Set the path to the libs
    println!("cargo:rustc-link-search={}", lib_dir.to_string_lossy());

    // On linux, we need to set the LD_LIBRARY_PATH to the sapnwrfc libs
    // e.g. for tests
    #[cfg(target_os = "linux")]
    set_ld_library_path(lib_dir);

    // Tell cargo to link against the sapnwrfc libs
    for lib in config::LIBS {
        println!("cargo:rustc-link-lib={lib}");
    }

    // Set additional link args
    for link_arg in config::LINK_ARGS {
        println!("cargo:rustc-link-arg={}", link_arg);
    }

    // Add the bindgen wrapper
    let bindings = bindgen::Builder::default()
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
        // Use rust enums as default.
        // But make them exhaustive, as we need to re-build the wrappers for
        // every NW RFC lib version anyway.
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        // Don't include the documentation as comments
        .generate_comments(true);

    // generate the bindings
    let out_path = PathBuf::from("src");
    bindings
        .generate()
        .expect("Unable to generate library bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write library bindings");
}
