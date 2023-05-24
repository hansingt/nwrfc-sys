use crate::_unsafe::{strnlenU16, RfcSAPUCToUTF8, RfcUTF8ToSAPUC, RFC_ERROR_INFO, RFC_RC, SAP_UC};
use crate::protocol::RfcResult;
use std::ffi::c_uint;
use std::mem::size_of;
use std::slice;

/// todo!
#[repr(transparent)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UCStr {
    uc: [SAP_UC],
}

fn sap_uc_to_string_with_len(uc: &[SAP_UC], len: usize) -> String {
    let mut error_info = RFC_ERROR_INFO::default();
    let mut buffer_len = len as c_uint;
    let mut buffer = Vec::with_capacity(len);
    let mut result_len = 0;
    let rc = unsafe {
        RfcSAPUCToUTF8(
            uc.as_ptr(),
            strnlenU16(uc.as_ptr(), uc.len()) as c_uint,
            buffer.as_mut_ptr(),
            &mut buffer_len,
            &mut result_len,
            &mut error_info,
        )
    };
    match rc {
        RFC_RC::RFC_OK => {
            // SAFETY: We know, that buffer now contains valid UTF-8 and that result_len
            // can't be larger than the length of the buffer. Thus, it is safe to
            // set the vectors length and convert it into a rust string unchecked.
            unsafe {
                buffer.set_len(result_len as usize);
                String::from_utf8_unchecked(buffer)
            }
        }
        _ => {
            // According to the docs, the only error, that can occurr is when the UTF-8 buffer
            // is too small. Thus, we simply assume this error here and retry with the new buffer
            // length.
            sap_uc_to_string_with_len(uc, buffer_len as usize)
        }
    }
}

const fn strnlen(s: *const SAP_UC, maxlen: usize) -> usize {
    let mut len = 0;
    while unsafe { *s.add(len) } != 0 && len < maxlen {
        len += 1
    }
    len
}

const fn strlen(s: *const SAP_UC) -> usize {
    strnlen(s, isize::MAX as usize / size_of::<SAP_UC>())
}

impl UCStr {
    /// todo!
    #[inline(always)]
    pub const fn from_slice(uc: &[SAP_UC]) -> &UCStr {
        // SAFETY: Transmuting a slice of SAP_UCs to a UCStr
        // is safe, because UCStr internally is [SAP_UC].
        // De-referencing the pointer obtained is safe, because it comes
        // from a reference. Create a new reference is then safe, because
        // it's lifetime is bound to the lifetime of the original reference.
        unsafe { &*(uc as *const [SAP_UC] as *const UCStr) }
    }

    /// todo!
    #[inline(always)]
    pub fn from_slice_mut(uc: &mut [SAP_UC]) -> &mut UCStr {
        // SAFETY: See from_slice_unchecked
        unsafe { &mut *(uc as *mut [SAP_UC] as *mut UCStr) }
    }

    /// todo!
    #[inline(always)]
    pub const unsafe fn from_ptr_with_nul<'a>(ptr: *const SAP_UC) -> &'a UCStr {
        // SAFETY: The caller provides a pointer to a valid sap unicode string, which is
        // nul-terminated and whose contents do not change during the lifetime of the
        // UCStr returned.
        //
        // Thus, it is safe to calculate the length (nul-terminator exists) and construct
        // a slice from the pointer and cast it's reference to UCStr.
        UCStr::from_slice(slice::from_raw_parts(ptr, strlen(ptr)))
    }

    /// todo!
    #[inline]
    pub fn to_string_lossy(&self) -> String {
        sap_uc_to_string_with_len(&self.uc, self.uc.len())
    }

    /// todo!
    pub fn write<T: AsRef<str>>(&mut self, s: T) -> RfcResult<usize> {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut buffer_len = self.uc.len() as c_uint;
        let mut result_len = 0;
        let rc = unsafe {
            RfcUTF8ToSAPUC(
                s.as_ref().as_ptr(),
                s.as_ref().len() as c_uint,
                self.uc.as_mut_ptr(),
                &mut buffer_len,
                &mut result_len,
                &mut error_info,
            )
        };
        match rc {
            RFC_RC::RFC_OK => Ok(result_len as usize),
            _ => Err(error_info.into()),
        }
    }

    /// todo!
    pub fn write_without_nul<T: AsRef<str>>(&mut self, s: T) -> RfcResult<usize> {
        let mut tmp_buffer = vec![0; self.uc.len() + 1];
        let result_len = UCStr::from_slice_mut(&mut tmp_buffer).write(s)?;
        self.uc.copy_from_slice(&tmp_buffer[..self.uc.len()]);
        Ok(result_len)
    }

    /// todo!
    #[inline(always)]
    #[must_use]
    pub const fn as_ptr(&self) -> *const SAP_UC {
        self.uc.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice() {
        let a = [
            'H' as SAP_UC,
            'e' as SAP_UC,
            'l' as SAP_UC,
            'l' as SAP_UC,
            'o' as SAP_UC,
        ];
        let uc_str = UCStr::from_slice(&a);
        assert_eq!(uc_str.to_string_lossy(), "Hello");
    }

    #[test]
    fn test_write() {
        let mut a = [0; 12];
        UCStr::from_slice_mut(&mut a)
            .write("Hello World")
            .expect("Unable to write the string");
        assert_eq!(UCStr::from_slice(&a).to_string_lossy(), "Hello World");
    }

    #[test]
    fn test_write_without_nul() {
        let mut a = [0; 11];
        UCStr::from_slice_mut(&mut a)
            .write_without_nul("Hello World")
            .expect("Unable to write the string");
        assert_eq!(a[10], 'd' as SAP_UC);
    }
}
