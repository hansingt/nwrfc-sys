//! todo!

// Custom macros
#[macro_use]
mod macros;

// Include the generated bindings.
// Export them for direct (unsafe) usage, if the abstraction layer protocol
// does not implement the required functionality.
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
pub mod _unsafe;

// Include and export the NWRFC abstraction layer protocol.
pub mod protocol;
