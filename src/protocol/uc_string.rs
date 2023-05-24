use crate::_unsafe::{RfcUTF8ToSAPUC, RFC_ERROR_INFO, RFC_RC, SAP_UC};
use crate::protocol::RfcError;
use crate::protocol::UCStr;
use std::ffi::c_uint;
use std::ops::{Deref, DerefMut};

/// todo!
#[repr(transparent)]
#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UCString {
    uc: Vec<SAP_UC>,
}

fn string_to_sap_uc<T: AsRef<str>>(s: T, len: usize) -> Vec<SAP_UC> {
    let mut error_info = RFC_ERROR_INFO::default();
    let mut buffer_len = len as c_uint;
    let mut buffer = Vec::with_capacity(len);
    let mut result_len = 0;
    let rc = unsafe {
        RfcUTF8ToSAPUC(
            s.as_ref().as_ptr(),
            s.as_ref().len() as c_uint,
            buffer.as_mut_ptr(),
            &mut buffer_len,
            &mut result_len,
            &mut error_info,
        )
    };
    match rc {
        RFC_RC::RFC_OK => {
            // SAFETY: We know, that the result length must be smaller than the
            // length of the buffer. Thus, setting the length is safe.
            unsafe { buffer.set_len(buffer_len as usize) }
            buffer
        }
        RFC_RC::RFC_BUFFER_TOO_SMALL => {
            drop(buffer);
            string_to_sap_uc(s.as_ref(), buffer_len as usize)
        }
        _ => panic!(
            "Unexpected error while converting the string \"{}\" to SAP unicode: {}",
            s.as_ref(),
            RfcError::from(error_info)
        ),
    }
}

impl UCString {
    /// todo!
    #[inline(always)]
    pub const fn new() -> Self {
        Self { uc: Vec::new() }
    }
}

impl<T: AsRef<str>> From<T> for UCString {
    fn from(s: T) -> Self {
        Self {
            uc: string_to_sap_uc(s.as_ref(), s.as_ref().len()),
        }
    }
}

impl Deref for UCString {
    type Target = UCStr;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        UCStr::from_slice(&self.uc)
    }
}

impl DerefMut for UCString {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        UCStr::from_slice_mut(&mut self.uc)
    }
}
