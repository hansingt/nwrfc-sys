use crate::_unsafe::{
    RfcAddException, RfcAddParameter, RfcCreateFunctionDesc, RfcDestroyFunctionDesc,
    RfcGetExceptionCount, RfcGetExceptionDescByIndex, RfcGetExceptionDescByName,
    RfcGetFunctionName, RfcGetParameterCount, RfcGetParameterDescByIndex,
    RfcGetParameterDescByName, RFC_ABAP_NAME, RFC_ERROR_INFO, RFC_EXCEPTION_DESC,
    RFC_FUNCTION_DESC_HANDLE, RFC_PARAMETER_DESC, _RFC_FUNCTION_DESC_HANDLE,
};
use crate::protocol::{
    utils, ExceptionDescription, ParameterDescription, ParameterDirection, ReturnCode, RfcResult,
    UCStr, UCString,
};
use std::mem::ManuallyDrop;
use std::ops::Deref;

/// todo!
#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct FuncDesc {
    pub(crate) handle: _RFC_FUNCTION_DESC_HANDLE,
}

/// todo!
#[derive(Debug)]
pub struct ParameterIterator<'a> {
    desc: &'a FuncDesc,
    index: usize,
    direction: ParameterDirection,
}

impl<'a> Iterator for ParameterIterator<'a> {
    type Item = ParameterDescription<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = self.desc.get_parameter_by_index(self.index);
            self.index += 1;
            match result {
                None => return None,
                Some(param) => {
                    if param.direction() == self.direction {
                        return Some(param);
                    }
                }
            }
        }
    }
}

/// todo!
#[derive(Debug)]
pub struct ExceptionIterator<'a> {
    desc: &'a FuncDesc,
    index: usize,
}

impl<'a> Iterator for ExceptionIterator<'a> {
    type Item = ExceptionDescription;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.desc.get_exception_by_index(self.index);
        self.index += 1;
        result
    }
}

impl FuncDesc {
    #[inline]
    #[must_use]
    pub(crate) const unsafe fn from_handle<'a>(handle: RFC_FUNCTION_DESC_HANDLE) -> &'a Self {
        // SAFETY: A `RFC_FUNCTION_DESC_HANDLE` is a `*mut *mut c_void`,
        // and a `FuncDesc` is `repr(transparent)` with a `*const c_void`.
        // Thus, it is safe to cast a `RFC_FUNCTION_DESC_HANDLE` to a `*const FuncDesc`.
        // De-referencing it is then safe, as long as the `RFC_FUNCTION_DESC_HANDLE` is a
        // valid pointer.
        // The lifetime is bound by `'a` for which the caller is responsible to make
        // sure, that the `handle` remains valid.
        &*(handle as *const Self)
    }

    #[inline(always)]
    fn _as_handle(&self) -> RFC_FUNCTION_DESC_HANDLE {
        // SAFETY: SAP API requires a mutable pointer even for non-modifying operations.
        //
        // We are not doing any modifying operations to the type description
        // in this type. Thus, it is safe to cast the handle to a mutable pointer here.
        &self.handle as *const _RFC_FUNCTION_DESC_HANDLE as *mut _RFC_FUNCTION_DESC_HANDLE
    }

    /// todo!
    pub fn name(&self) -> String {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut uc_name = RFC_ABAP_NAME::default();
        // SAFETY: As long as our handle is valid, the call is safe.
        unsafe {
            RfcGetFunctionName(self._as_handle(), uc_name.as_mut_ptr(), &mut error_info);
        }
        // According to the docs, this function should always return successfully.
        // Panic, if it doesn't.
        match utils::check_rc(&error_info) {
            Ok(_) => UCStr::from_slice(&uc_name).to_string_lossy(),
            Err(e) => panic!("Error getting the name of the function: {}", e),
        }
    }

    /// todo!
    pub fn parameter_count(&self) -> usize {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut param_count = 0;
        // SAFETY: As long as our handle is valid, the call is safe.
        unsafe {
            RfcGetParameterCount(self._as_handle(), &mut param_count, &mut error_info);
        }
        // According to the docs, this function should always return successfully.
        // Panic, if it doesn't.
        match utils::check_rc(&error_info) {
            Ok(_) => param_count as usize,
            Err(e) => panic!(
                "Error getting the number of parameters from function '{}': {}",
                self.name(),
                e
            ),
        }
    }

    /// todo!
    #[inline]
    pub fn parameters(&self, direction: ParameterDirection) -> ParameterIterator {
        ParameterIterator {
            desc: self,
            index: 0,
            direction,
        }
    }

    /// todo!
    pub fn get_parameter_by_name<T: AsRef<str>>(&self, name: T) -> Option<ParameterDescription> {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut param_desc = RFC_PARAMETER_DESC::default();
        let uc_name = UCString::from(&name);
        // SAFETY: As long as our handle is valid, the call is safe.
        unsafe {
            RfcGetParameterDescByName(
                self._as_handle(),
                uc_name.as_ptr(),
                &mut param_desc,
                &mut error_info,
            )
        };
        // According to the docs, this function should always return successfully.
        // Panic, if it doesn't.
        match utils::check_rc(&error_info) {
            Ok(_) => Some(param_desc.into()),
            Err(e) => match e.code {
                ReturnCode::InvalidParameter => None,
                _ => panic!(
                    "Error getting parameter '{}' from function '{}': {}",
                    name.as_ref(),
                    self.name(),
                    e
                ),
            },
        }
    }

    /// todo!
    pub fn get_parameter_by_index<'a, 'b: 'a>(
        &'a self,
        idx: usize,
    ) -> Option<ParameterDescription<'b>> {
        if idx >= self.parameter_count() {
            None
        } else {
            let mut error_info = RFC_ERROR_INFO::default();
            let mut param_desc = RFC_PARAMETER_DESC::default();
            // SAFETY: As long as our handle is valid, the call is safe.
            unsafe {
                RfcGetParameterDescByIndex(
                    self._as_handle(),
                    idx as u32,
                    &mut param_desc,
                    &mut error_info,
                );
            }
            // According to the docs, this function should always return successfully.
            // Panic, if it doesn't.
            match utils::check_rc(&error_info) {
                Ok(_) => Some(param_desc.into()),
                Err(e) => match e.code {
                    ReturnCode::InvalidParameter => None,
                    _ => panic!(
                        "Error getting parameter '{}' from function '{}': {}",
                        idx,
                        self.name(),
                        e
                    ),
                },
            }
        }
    }

    /// todo!
    pub fn exception_count(&self) -> usize {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut count = 0;
        // SAFETY: As long as our handle is valid, the call is safe.
        unsafe {
            RfcGetExceptionCount(self._as_handle(), &mut count, &mut error_info);
        }
        // According to the docs, this function should always return successfully.
        // Panic, if it doesn't.
        match utils::check_rc(&error_info) {
            Ok(_) => count as usize,
            Err(e) => panic!(
                "Error getting the number of exceptions from function '{}': {}",
                self.name(),
                e
            ),
        }
    }

    /// todo!
    #[inline]
    pub fn exceptions(&self) -> ExceptionIterator {
        ExceptionIterator {
            desc: self,
            index: 0,
        }
    }

    /// todo!
    pub fn get_exception_by_name<T: AsRef<str>>(&self, name: T) -> Option<ExceptionDescription> {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut desc = RFC_EXCEPTION_DESC::default();
        let uc_name = UCString::from(&name);
        // SAFETY: As long as our handle is valid, the call is safe.
        unsafe {
            RfcGetExceptionDescByName(
                self._as_handle(),
                uc_name.as_ptr(),
                &mut desc,
                &mut error_info,
            )
        };
        // According to the docs, this function should always return successfully.
        // Panic, if it doesn't.
        match utils::check_rc(&error_info) {
            Ok(_) => Some(desc.into()),
            Err(e) => match e.code {
                ReturnCode::InvalidParameter => None,
                _ => panic!(
                    "Error getting exception '{}' from function '{}': {}",
                    name.as_ref(),
                    self.name(),
                    e
                ),
            },
        }
    }

    /// todo!
    pub fn get_exception_by_index(&self, idx: usize) -> Option<ExceptionDescription> {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut desc = RFC_EXCEPTION_DESC::default();
        // SAFETY: As long as our handle is valid, the call is safe.
        unsafe {
            RfcGetExceptionDescByIndex(self._as_handle(), idx as u32, &mut desc, &mut error_info);
        }
        // According to the docs, this function should always return successfully.
        // Panic, if it doesn't.
        match utils::check_rc(&error_info) {
            Ok(_) => Some(desc.into()),
            Err(e) => match e.code {
                ReturnCode::InvalidParameter => None,
                _ => panic!(
                    "Error getting exception '{}' from function '{}': {}",
                    idx,
                    self.name(),
                    e
                ),
            },
        }
    }
}

/// Metadata description of a function module.
///
/// Can either be requested from the DDIC using [`Connection::describe_function`] or created
/// as new function using the [`new`] method. If the function description was requested from
/// DDIC, modification is not permitted.
/// If the function description is created using [`new`], you can use [`add_parameter`] and
/// [`add_exception`] to add parameters and exceptions to it.
///
/// **_NOTE:_** After this description was used to create a [`Function`], modifications are not
/// permitted anymore!
///
/// # Examples
/// ```rust
/// use nwrfc::protocol::{FunctionDescription, ParameterDescription, Type, ParameterDirection};
///
/// let mut func_desc = FunctionDescription::new("TEST").expect("Unable to create function");
/// let param = ParameterDescription::new(
///     "TEST_PARAM",
///     ParameterDirection::Import,
///     Type::Char(1),
/// ).expect("Unable to create parameter");
/// func_desc.add_parameter(&param).expect("Error adding the parameter to the function");
/// ```
///
/// [`Connection::describe_function`]: crate::protocol::Connection::describe_function
/// [`new`]: FunctionDescription::new
/// [`add_parameter`]: FunctionDescription::add_parameter
/// [`add_exception`]: FunctionDescription::add_exception
///
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FunctionDescription {
    handle: RFC_FUNCTION_DESC_HANDLE,
}

impl FunctionDescription {
    /// Creates an empty function description with the given name.
    ///
    /// Add parameter descriptions and exception descriptions to this function description
    /// using the [`add_parameter`] and [`add_exception`] methods.
    ///
    /// After the handle was used for creating a container, any modifications are forbidden.
    ///
    /// [`add_parameter`]: FunctionDescription::add_parameter
    /// [`add_exception`]: FunctionDescription::add_exception
    ///
    pub fn new<T: AsRef<str>>(name: T) -> RfcResult<Self> {
        let mut error_info = RFC_ERROR_INFO::default();
        let uc_name = UCString::from(name);
        let handle = unsafe { RfcCreateFunctionDesc(uc_name.as_ptr(), &mut error_info) };
        utils::check_rc(&error_info)?;
        // SAFETY: We know, that our handle is non-null, aligned and points to a valid
        // _RFC_FUNCTION_DESC_HANDLE.
        // Thus, it is safe, to de-reference it and use the value pointed to in our
        // FunctionDescription.
        Ok(Self::from(handle))
    }

    /// todo!
    pub fn add_parameter(&mut self, param: &ParameterDescription) -> RfcResult<()> {
        let mut error_info = RFC_ERROR_INFO::default();
        // SAFETY: We know, that our handle is valid. Thus, this call is safe.
        unsafe {
            RfcAddParameter(self.handle, &param.into(), &mut error_info);
        }
        utils::check_rc(&error_info)
    }

    /// todo!
    pub fn add_exception(&mut self, exception: ExceptionDescription) -> RfcResult<()> {
        let mut error_info = RFC_ERROR_INFO::default();
        let exception_desc = exception.into();
        // SAFETY: We know, that our handle is valid. Thus, this call is safe.
        unsafe {
            RfcAddException(self.handle, &exception_desc, &mut error_info);
        }
        utils::check_rc(&error_info)
    }
}

impl From<FunctionDescription> for RFC_FUNCTION_DESC_HANDLE {
    #[inline]
    #[must_use]
    fn from(desc: FunctionDescription) -> Self {
        // Prevent from destroying the RFC_FUNCTION_DESC_HANDLE.
        let s = ManuallyDrop::new(desc);
        s.handle
    }
}

impl From<RFC_FUNCTION_DESC_HANDLE> for FunctionDescription {
    #[inline]
    fn from(handle: RFC_FUNCTION_DESC_HANDLE) -> Self {
        Self { handle }
    }
}

impl Drop for FunctionDescription {
    fn drop(&mut self) {
        let mut error_info = RFC_ERROR_INFO::default();
        // SAFETY: We know, that our handle is valid. Thus, this call is safe.
        unsafe {
            RfcDestroyFunctionDesc(self.handle, &mut error_info);
        }
        // No much we can do here. Thus, simply panic, if the drop fails.
        utils::check_rc(&error_info).expect("Error destroying the function description");
    }
}

impl Deref for FunctionDescription {
    type Target = FuncDesc;

    fn deref(&self) -> &Self::Target {
        // SAFETY: We are converting into an un-mutable reference.
        // We do not modify the function description through this reference.
        // Thus, it is safe to cast to a mutable reference, to get the function description handle.
        unsafe { FuncDesc::from_handle(self.handle) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{ParameterDirection, Type};

    #[test]
    fn add_parameter() {
        let mut func_desc = FunctionDescription::new("TEST").expect("Unable to create function");
        let param =
            ParameterDescription::new("TEST_PARAM", ParameterDirection::Import, Type::Char(1))
                .expect("Unable to create parameter");
        func_desc
            .add_parameter(&param)
            .expect("Error adding the parameter to the function");
    }

    #[test]
    fn get_parameter_by_name() -> RfcResult<()> {
        let mut desc = FunctionDescription::new("TEST")?;
        let param = ParameterDescription::new("TEST", ParameterDirection::Import, Type::Char(1))?;
        desc.add_parameter(&param)?;

        desc.get_parameter_by_name("TEST")
            .expect("Parameter not found!");
        assert!(desc.get_parameter_by_name("NOT_EXISTING").is_none());
        assert!(desc
            .get_parameter_by_name("PARAMETER_NAME_WHICH_IS_TOO_LONG")
            .is_none());

        Ok(())
    }
}
