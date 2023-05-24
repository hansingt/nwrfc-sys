use crate::_unsafe::{
    RfcCreateFunction, RfcDestroyFunction, RfcSetParameterActive, RFC_ERROR_INFO,
    RFC_FUNCTION_HANDLE, _RFC_FUNCTION_DESC_HANDLE,
};
use crate::protocol::{utils, FuncDesc, RfcError, RfcResult, UCString};
use std::mem::ManuallyDrop;

#[repr(transparent)]
#[derive(Debug, Hash)]
pub struct Function {
    handle: RFC_FUNCTION_HANDLE,
}

impl Function {
    /// todo!
    pub fn set_parameter_active<N: AsRef<str>>(&mut self, name: N, active: bool) -> RfcResult<()> {
        let mut error_info = RFC_ERROR_INFO::default();
        let parameter_name = UCString::from(name);
        let is_active = if active { 1 } else { 0 };
        unsafe {
            RfcSetParameterActive(
                self.handle,
                parameter_name.as_ptr(),
                is_active,
                &mut error_info,
            );
        }
        utils::check_rc(&error_info)
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        let mut error_info = RFC_ERROR_INFO::default();
        unsafe { RfcDestroyFunction(self.handle, &mut error_info) };
        match utils::check_rc(&error_info) {
            Err(e) => panic!("Error while destroying function: {}", e),
            Ok(_) => {}
        }
    }
}

impl TryFrom<&FuncDesc> for Function {
    type Error = RfcError;

    fn try_from(desc: &FuncDesc) -> Result<Self, Self::Error> {
        let mut error_info = RFC_ERROR_INFO::default();
        let handle = unsafe {
            // SAFETY: SAP API requires a mutable pointer, even if the pointer
            // is not mutated by the function. Thus, it is safe to cast the function
            // description to a mutable reference to get the function description handle here.
            let func_desc =
                &desc.handle as *const _RFC_FUNCTION_DESC_HANDLE as *mut _RFC_FUNCTION_DESC_HANDLE;
            RfcCreateFunction(func_desc, &mut error_info)
        };
        match utils::check_rc(&error_info) {
            Ok(_) => Ok(handle.into()),
            Err(e) => Err(e),
        }
    }
}

impl From<RFC_FUNCTION_HANDLE> for Function {
    fn from(handle: RFC_FUNCTION_HANDLE) -> Self {
        Self { handle }
    }
}

impl From<Function> for RFC_FUNCTION_HANDLE {
    fn from(func: Function) -> RFC_FUNCTION_HANDLE {
        let s = ManuallyDrop::new(func);
        s.handle
    }
}
