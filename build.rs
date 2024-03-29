extern crate bindgen;

use std::env;
use std::path::PathBuf;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
mod config {
    pub const DEFINES: [&str; 15] = [
        "-DSAPonNT",
        "-D_CRT_NON_CONFORMING_SWPRINTFS",
        "-D_CRT_SECURE_NO_DEPRECATE",
        "-D_CRT_NONSTDC_NO_DEPRECATE",
        "-D_AFXDLL",
        "-DWIN32",
        "-D_WIN32_WINNT=0x0502",
        "-D_X86_",
        "-DBCDASM",
        "-DNDEBUG",
        "-DSAPwithUNICODE",
        "-DUNICODE",
        "-D_UNICODE",
        "-DSAPwithTHREADS",
        "-D_ATL_ALLOW_CHAR_UNSIGNED",
    ];
    pub const LIBS: [&str; 13] = [
        "ole32.lib",
        "oleauth32.lib",
        "uuid.lib",
        "kernel32.lib",
        "advapi32.lib",
        "user32.lib",
        "gdi32.lib",
        "winspool.lib",
        "ws2_32.lib",
        "comdlg32.lib",
        "shell32.lib",
        "sapnwrfc.lib",
        "libsapucum.lib",
    ];
    pub const LINK_ARGS: [&str; 9] = [
        "-NXCOMPAT",
        "-STACK:0x800000",
        "-SWAPRUN:NET",
        "-Opt:REF",
        "-DEBUGTYPE:CV",
        "-LARGEADDRESSAWARE",
        "-MACHINE:x86",
        "-nologo",
        "-LTCG",
    ];
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
    ];
    pub const LIBS: [&str; 13] = [
        "ole32.lib",
        "oleauth32.lib",
        "uuid.lib",
        "kernel32.lib",
        "advapi32.lib",
        "user32.lib",
        "gdi32.lib",
        "winspool.lib",
        "ws2_32.lib",
        "comdlg32.lib",
        "shell32.lib",
        "sapnwrfc.lib",
        "libsapucum.lib",
    ];
    pub const LINK_ARGS: [&str; 9] = [
        "-NXCOMPAT",
        "-STACK:0x2000000",
        "-SWAPRUN:NET",
        "-DEBUG",
        "-OPT:REF",
        "-DEBUGTYPE:CV,FIXUP",
        "-MACHINE:amd64",
        "-nologo",
        "-LTCG",
    ];
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod config {
    pub const DEFINES: [&str; 1] = [
        "-DSAPwithUNICODE",
    ];
    pub const LIBS: [&str; 2] = ["sapnwrfc", "sapucum"];
    pub const LINK_ARGS: [&str; 0] = [];
}

#[cfg(all(target_os = "linux", target_arch = "x86"))]
mod config {
    pub const DEFINES: [&str; 2] = [
        "-DSAPwithUNICODE",
        "-m32",
    ];
    pub const LIBS: [&str; 2] = ["sapnwrfc", "sapucum"];
    pub const LINK_ARGS: [&str; 0] = [];
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod config {
    pub const DEFINES: [&str; 1] = ["-DSAPwithUNICODE"];
    pub const LIBS: [&str; 2] = ["sapnwrfc", "sapucum"];
    pub const LINK_ARGS: [&str; 0] = [];
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
        .header(sdk.join("include").join("sapdecf.h").to_string_lossy())
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
