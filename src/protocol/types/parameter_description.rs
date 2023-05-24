//! todo!

use crate::_unsafe::{
    RFC_ABAP_NAME, RFC_PARAMETER_DEFVALUE, RFC_PARAMETER_DESC, SAP_UC, _RFC_TYPE_DESC_HANDLE,
};
use crate::protocol::{ParameterDirection, RfcResult, Type, TypeDesc, UCStr};
use std::marker::PhantomData;
use std::ptr;

/// todo!
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParameterDescription<'a> {
    desc: RFC_PARAMETER_DESC,
    _type_ref: PhantomData<Type<'a>>,
}

impl<'a> ParameterDescription<'a> {
    /// todo!
    pub fn new<N: AsRef<str>>(
        name: N,
        direction: ParameterDirection,
        parameter_type: Type<'a>,
    ) -> RfcResult<Self> {
        let (nuc_length, uc_length) = parameter_type.len();
        let mut handle = RFC_PARAMETER_DESC {
            name: RFC_ABAP_NAME::default(),
            direction: direction.into(),
            nucLength: nuc_length,
            ucLength: uc_length,
            decimals: parameter_type.decimals(),
            typeDescHandle: match parameter_type.type_description() {
                None => ptr::null_mut(),
                // SAFETY: We are not modifying this type description
                // and we don't pass modifiable references out. Thus, it's
                // safe to cast the reference to a mutable reference to
                // allow getting the handle.
                Some(t) => &t.handle as *const _RFC_TYPE_DESC_HANDLE as *mut _RFC_TYPE_DESC_HANDLE,
            },
            defaultValue: RFC_PARAMETER_DEFVALUE::default(),
            parameterText: [0 as SAP_UC; 80],
            optional: 0,
            type_: parameter_type.into(),
            extendedDescription: ptr::null_mut(),
        };
        UCStr::from_slice_mut(&mut handle.name).write(name)?;
        Ok(handle.into())
    }

    /// todo!
    #[inline]
    pub fn name(&self) -> String {
        UCStr::from_slice(&self.desc.name).to_string_lossy()
    }

    /// todo!
    #[inline]
    pub fn parameter_type(&self) -> Type {
        let type_desc = if self.desc.typeDescHandle.is_null() {
            None
        } else {
            Some(unsafe { TypeDesc::from_handle::<'a>(self.desc.typeDescHandle) })
        };
        Type::from_rfc_type(self.desc.type_, self.len(), self.decimals(), type_desc)
    }

    /// todo!
    #[inline]
    pub fn direction(&self) -> ParameterDirection {
        self.desc.direction.into()
    }

    /// todo!
    #[inline]
    pub fn len(&self) -> u32 {
        self.desc.nucLength
    }

    /// todo!
    #[inline]
    pub fn decimals(&self) -> u32 {
        self.desc.decimals
    }

    /// todo!
    #[inline]
    pub fn default_value(&self) -> String {
        UCStr::from_slice(&self.desc.defaultValue).to_string_lossy()
    }

    #[inline]
    pub fn set_default_value<T: AsRef<str>>(&mut self, default: T) -> RfcResult<()> {
        match UCStr::from_slice_mut(&mut self.desc.defaultValue).write(default) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// todo!
    #[inline]
    pub fn description(&self) -> String {
        UCStr::from_slice(&self.desc.parameterText).to_string_lossy()
    }

    /// todo!
    #[inline]
    pub fn optional(&self) -> bool {
        self.desc.optional != 0
    }

    /// todo!
    #[inline]
    pub fn set_optional(&mut self, optional: bool) {
        self.desc.optional = if optional { 1 } else { 0 }
    }
}

impl<'a> From<RFC_PARAMETER_DESC> for ParameterDescription<'a> {
    #[inline(always)]
    fn from(desc: RFC_PARAMETER_DESC) -> Self {
        Self {
            desc,
            _type_ref: PhantomData::default(),
        }
    }
}

impl<'a> From<&ParameterDescription<'a>> for RFC_PARAMETER_DESC {
    #[inline(always)]
    fn from(desc: &ParameterDescription) -> Self {
        desc.desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() -> RfcResult<()> {
        let name = "TEST";
        let param_desc =
            ParameterDescription::new(name, ParameterDirection::Import, Type::Char(1))?;
        assert_eq!(param_desc.name(), name);
        Ok(())
    }
}
