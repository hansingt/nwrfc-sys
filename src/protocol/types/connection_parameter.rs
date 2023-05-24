use crate::_unsafe::{RFC_CONNECTION_PARAMETER, SAP_UC};
use crate::protocol::{UCStr, UCString};
use std::mem::ManuallyDrop;
use std::ptr;

/// todo!
#[derive(Debug, Default, Clone)]
pub struct ConnectionParameters(Vec<RFC_CONNECTION_PARAMETER>);

impl ConnectionParameters {
    /// todo!
    pub fn push<N: AsRef<str>, V: AsRef<str>>(&mut self, name: N, value: V) {
        // We need to make sure, that rust does not drop our SAP unicode vectors
        // as we are storing only the pointers to it
        let name = ManuallyDrop::new(UCString::from(name));
        let value = ManuallyDrop::new(UCString::from(value));
        self.0.push(RFC_CONNECTION_PARAMETER {
            name: name.as_ptr(),
            value: value.as_ptr(),
        })
    }

    /// todo!
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            0: Vec::with_capacity(capacity),
        }
    }

    /// todo!
    #[inline]
    pub fn as_ptr(&self) -> *const RFC_CONNECTION_PARAMETER {
        self.0.as_ptr()
    }

    /// todo!
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// todo!
    #[inline]
    pub fn get(&self, index: usize) -> Option<(String, String)> {
        match self.0.get(index) {
            None => None,
            Some(param) => unsafe {
                let name = UCStr::from_ptr_with_nul(param.name);
                let value = UCStr::from_ptr_with_nul(param.value);
                Some((name.to_string_lossy(), value.to_string_lossy()))
            },
        }
    }

    /// todo!
    #[inline]
    pub fn iter(&self) -> ConnectionParameterIterator {
        ConnectionParameterIterator {
            index: 0,
            params: self,
        }
    }
}

impl Drop for ConnectionParameters {
    fn drop(&mut self) {
        // Explicitly drop the SAP unicode strings from the connection parameters
        for param in &self.0 {
            // SAFETY: We know, that the name and value of the parameters are
            // valid, because we own them. For the same reason it's safe to cast them
            // from a *const to *mut.
            unsafe {
                ptr::drop_in_place(param.name as *mut SAP_UC);
                ptr::drop_in_place(param.value as *mut SAP_UC);
            }
        }
    }
}

impl<N: AsRef<str>, V: AsRef<str>> From<&[(N, V)]> for ConnectionParameters {
    fn from(params: &[(N, V)]) -> Self {
        let mut result = Self::with_capacity(params.len());
        for (name, value) in params {
            result.push(name, value);
        }
        result
    }
}

pub struct ConnectionParameterIterator<'a> {
    index: usize,
    params: &'a ConnectionParameters,
}
impl<'a> Iterator for ConnectionParameterIterator<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.params.get(self.index);
        self.index += 1;
        res
    }
}

impl<'a> IntoIterator for &'a ConnectionParameters {
    type Item = (String, String);
    type IntoIter = ConnectionParameterIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_strings() {
        let params = [
            ("ASHOST", "fuubar.example.com"),
            ("SYSNR", "37"),
            ("CLIENT", "000"),
            ("USER", "test"),
        ];
        let cparams = ConnectionParameters::from(params.as_slice());
        for (param, check) in cparams.iter().zip(params) {
            assert_eq!(check.0, param.0);
            assert_eq!(check.1, param.1);
        }
    }
}
