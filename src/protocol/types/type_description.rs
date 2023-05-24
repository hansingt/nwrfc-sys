use crate::_unsafe::{
    RfcAddTypeField, RfcCreateTypeDesc, RfcDestroyTypeDesc, RfcGetFieldCount,
    RfcGetFieldDescByIndex, RfcGetFieldDescByName, RfcGetTypeLength, RfcGetTypeName,
    RfcSetTypeLength, RFC_ABAP_NAME, RFC_ERROR_INFO, RFC_FIELD_DESC, RFC_TYPE_DESC_HANDLE,
    _RFC_TYPE_DESC_HANDLE,
};
use crate::protocol::{utils, FieldDescription, ReturnCode, RfcResult, Type, UCStr, UCString};
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr;

/// Type description from a borrowed reference
/// todo!
#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct TypeDesc {
    pub(crate) handle: _RFC_TYPE_DESC_HANDLE,
}

impl TypeDesc {
    /// todo!
    pub unsafe fn from_handle<'a>(handle: RFC_TYPE_DESC_HANDLE) -> &'a Self {
        // SAFETY: A `RFC_TYPE_DESC_HANDLE` is a `*mut _RFC_TYPE_DESC_HANDLE` and
        // `TypeDesc` is `repr(transparent)` with a `_RFC_TYPE_DESC_HANDLE` field.
        //
        // Thus, as long, as the given handle points to a valid _RFC_TYPE_DESC_HANDLE,
        // it is safe, to de-reference the pointer. Creating a new reference is then
        // safe (in std), because it came from a reference. The lifetime is bound by 'b,
        // which must be shorter than the lifetime of the handle itself ('a).
        &*(handle as *const Self)
    }

    fn _as_handle(&self) -> RFC_TYPE_DESC_HANDLE {
        &self.handle as *const _RFC_TYPE_DESC_HANDLE as *mut _RFC_TYPE_DESC_HANDLE
    }

    /// todo!
    pub fn name(&self) -> String {
        let mut uc_name = RFC_ABAP_NAME::default();
        let mut error_info = RFC_ERROR_INFO::default();
        unsafe {
            RfcGetTypeName(self._as_handle(), uc_name.as_mut_ptr(), &mut error_info);
        }
        utils::check_rc(&error_info).expect("Unable to get the name of the type");
        UCStr::from_slice(&uc_name).to_string_lossy()
    }

    /// todo!
    pub fn len(&self) -> usize {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut field_count = 0;
        unsafe {
            RfcGetFieldCount(self._as_handle(), &mut field_count, &mut error_info);
        }
        utils::check_rc(&error_info).expect("Unable to get the number of fields of the type");
        field_count as usize
    }

    ///todo!
    pub fn get<T: AsRef<str>>(&self, field_name: T) -> Option<FieldDescription> {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut desc = RFC_FIELD_DESC::default();
        let uc_name = UCString::from(field_name.as_ref());
        unsafe {
            RfcGetFieldDescByName(
                self._as_handle(),
                uc_name.as_ptr(),
                &mut desc,
                &mut error_info,
            );
        }
        match utils::check_rc(&error_info) {
            Ok(_) => Some(desc.into()),
            Err(e) => match e.code {
                ReturnCode::NotFound => None,
                _ => panic!(
                    "Unknown error while getting field {} from type {}: {}",
                    field_name.as_ref(),
                    self.name(),
                    e
                ),
            },
        }
    }

    /// todo!
    pub fn get_by_index<'a, 'b: 'a>(&'a self, index: u32) -> Option<FieldDescription<'b>> {
        if index as usize >= self.len() {
            // Index out of range. This is not handled correctly by the NWRFC libs.
            None
        } else {
            let mut error_info = RFC_ERROR_INFO::default();
            let mut desc = RFC_FIELD_DESC::default();
            unsafe {
                RfcGetFieldDescByIndex(self._as_handle(), index, &mut desc, &mut error_info);
            }
            match utils::check_rc(&error_info) {
                Ok(_) => Some(desc.into()),
                Err(e) => match e.code {
                    ReturnCode::NotFound => None,
                    _ => panic!(
                        "Unknown error while getting field {} from type {}: {}",
                        index,
                        self.name(),
                        e
                    ),
                },
            }
        }
    }

    pub(crate) fn byte_lengths(&self) -> (u32, u32) {
        let mut error_info = RFC_ERROR_INFO::default();
        let (mut nuc_length, mut uc_length) = (0, 0);
        unsafe {
            RfcGetTypeLength(
                self._as_handle(),
                &mut nuc_length,
                &mut uc_length,
                &mut error_info,
            );
        }
        utils::check_rc(&error_info).expect("Error getting the length of the type");
        (nuc_length, uc_length)
    }

    /// todo!
    pub fn nuc_length(&self) -> u32 {
        self.byte_lengths().0
    }

    /// todo!
    pub fn uc_length(&self) -> u32 {
        self.byte_lengths().1
    }

    pub(crate) fn inlineable(&self) -> bool {
        for field in self {
            if !field.field_type().inlineable() {
                return false;
            }
        }
        true
    }
}

/// Type description from an owned reference
/// todo!
#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct TypeDescription {
    handle: RFC_TYPE_DESC_HANDLE,
}

impl TypeDescription {
    /// todo!
    pub fn new<T: AsRef<str>>(name: T) -> RfcResult<Self> {
        let mut error_info = RFC_ERROR_INFO::default();
        let uc_name = UCString::from(name);
        let handle = unsafe { RfcCreateTypeDesc(uc_name.as_ptr(), &mut error_info) };
        utils::check_rc(&error_info)?;
        Ok(Self::from(handle))
    }

    /// todo!
    pub fn add_field<T: AsRef<str>>(&mut self, name: T, field_type: Type) -> RfcResult<()> {
        // Calculate new type size
        let (nuc_length, uc_length) = field_type.len();
        // Search for the next number that is dividable by the given field length.
        // This might add some padding bytes in case, the current length is not dividable.
        let nuc_offset = (self.nuc_length() as f64 / nuc_length as f64).ceil() as u32;
        let uc_offset = (self.uc_length() as f64 / uc_length as f64).ceil() as u32;

        let mut field_desc = RFC_FIELD_DESC {
            name: RFC_ABAP_NAME::default(),
            nucLength: nuc_length,
            nucOffset: nuc_offset,
            ucLength: uc_length,
            ucOffset: uc_offset,
            decimals: field_type.decimals(),
            typeDescHandle: match field_type.type_description() {
                None => ptr::null_mut(),
                // SAFETY: We are not modifying this type description
                // and we don't pass modifiable references out. Thus, it's
                // safe to cast the reference to a mutable reference to
                // allow getting the handle.
                Some(t) => t._as_handle(),
            },
            type_: field_type.into(),
            extendedDescription: ptr::null_mut(),
        };
        UCStr::from_slice_mut(&mut field_desc.name).write(name)?;

        // Add it to the type
        let mut error_info = RFC_ERROR_INFO::default();
        unsafe {
            RfcAddTypeField(self.handle, &field_desc, &mut error_info);
        }
        utils::check_rc(&error_info)?;
        // Set the new type length
        unsafe {
            RfcSetTypeLength(self.handle, nuc_offset, uc_offset, &mut error_info);
        }
        utils::check_rc(&error_info)?;
        Ok(())
    }
}

impl Drop for TypeDescription {
    fn drop(&mut self) {
        let mut error_info = RFC_ERROR_INFO::default();
        unsafe {
            RfcDestroyTypeDesc(self.handle, &mut error_info);
        }
    }
}

impl From<TypeDescription> for RFC_TYPE_DESC_HANDLE {
    #[inline(always)]
    #[must_use]
    fn from(value: TypeDescription) -> Self {
        // Prevent dropping the type description handle
        let s = ManuallyDrop::new(value);
        s.handle
    }
}

impl From<RFC_TYPE_DESC_HANDLE> for TypeDescription {
    #[inline(always)]
    fn from(handle: RFC_TYPE_DESC_HANDLE) -> Self {
        Self { handle }
    }
}

impl Deref for TypeDescription {
    type Target = TypeDesc;

    fn deref(&self) -> &Self::Target {
        unsafe { TypeDesc::from_handle(self.handle) }
    }
}

/// todo!
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TypeIter<'a> {
    desc: &'a TypeDesc,
    index: u32,
}

impl<'a> Iterator for TypeIter<'a> {
    type Item = FieldDescription<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.desc.get_by_index(self.index);
        self.index += 1;
        result
    }
}

impl<'a, 'b: 'a> IntoIterator for &'a TypeDesc {
    type Item = <TypeIter<'a> as Iterator>::Item;
    type IntoIter = TypeIter<'a>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        TypeIter {
            desc: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a TypeDescription {
    type Item = <&'a TypeDesc as IntoIterator>::Item;
    type IntoIter = <&'a TypeDesc as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        (self as &TypeDesc).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() -> RfcResult<()> {
        let mut type_desc = TypeDescription::new("TEST")?;
        type_desc.add_field("TEST", Type::Char(1))?;

        for field in &type_desc {
            assert_eq!(field.name(), "TEST");
        }
        Ok(())
    }
}
