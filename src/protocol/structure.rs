use crate::_unsafe::{RfcDestroyStructure, RFC_ERROR_INFO, RFC_STRUCTURE_HANDLE};
use crate::protocol::utils;
use std::mem::ManuallyDrop;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Structure {
    pub(crate) handle: RFC_STRUCTURE_HANDLE,
}

impl Drop for Structure {
    fn drop(&mut self) {
        let mut error_info = RFC_ERROR_INFO::default();
        unsafe {
            RfcDestroyStructure(self.handle, &mut error_info);
        }
        match utils::check_rc(&error_info) {
            Ok(_) => {}
            Err(e) => panic!("Error while destroying structure: {}", e),
        }
    }
}

impl From<RFC_STRUCTURE_HANDLE> for Structure {
    #[inline(always)]
    fn from(handle: RFC_STRUCTURE_HANDLE) -> Self {
        Self { handle }
    }
}

impl From<Structure> for RFC_STRUCTURE_HANDLE {
    #[inline(always)]
    #[must_use]
    fn from(s: Structure) -> RFC_STRUCTURE_HANDLE {
        let s_ = ManuallyDrop::new(s);
        s_.handle
    }
}
