//! todo!

use crate::_unsafe::{
    RfcCancel, RfcCloseConnection, RfcGetConnectionAttributes, RfcGetFunctionDesc, RfcGetTypeDesc,
    RfcOpenConnection, RfcPing, RFC_ATTRIBUTES, RFC_CONNECTION_HANDLE, RFC_ERROR_INFO, RFC_RC,
};
use crate::protocol::{
    utils, ConnectionAttributes, ConnectionParameters, FunctionDescription, RfcResult,
    TypeDescription, UCString,
};
use std::ffi::c_uint;
use std::ptr;

/// todo!
#[derive(Debug)]
pub struct Connection {
    params: ConnectionParameters,
    handle: RFC_CONNECTION_HANDLE,
}

impl Connection {
    /// todo!
    pub fn open(params: ConnectionParameters) -> RfcResult<Self> {
        let mut result = Self {
            params,
            handle: ptr::null_mut(),
        };
        result._open()?;
        Ok(result)
    }

    fn _open(&mut self) -> RfcResult<()> {
        let mut error_info = RFC_ERROR_INFO::default();
        let handle = unsafe {
            RfcOpenConnection(
                self.params.as_ptr(),
                self.params.len() as c_uint,
                &mut error_info,
            )
        };
        match utils::check_rc(&error_info) {
            Ok(_) => {
                self.handle = handle;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// todo!
    pub fn is_alive(&self) -> bool {
        !self.handle.is_null()
    }

    /// Closes the RFC connection
    ///
    /// Can be used to close the connection, when it is no longer needed.
    /// Returns an error in case, closing the connection fails.
    pub fn close(&mut self) -> RfcResult<()> {
        if !self.is_alive() {
            return Ok(());
        }
        let mut error_info = RFC_ERROR_INFO::default();
        unsafe { RfcCloseConnection(self.handle, &mut error_info) };
        utils::check_rc(&error_info)?;
        self.handle = ptr::null_mut();
        Ok(())
    }

    /// todo!
    pub fn reopen(&mut self) -> RfcResult<()> {
        self.close()?;
        self._open()
    }

    /// Cancels the RFC call which is currently being called over this connection and closes it.
    ///
    /// Needs to be called from a different thread than the one currently executing the RFC call.
    /// Returns an error in case the RFC call could not be canceled.
    pub fn cancel(&mut self) -> RfcResult<()> {
        if !self.is_alive() {
            return Ok(());
        }
        let mut error_info = RFC_ERROR_INFO::default();
        unsafe { RfcCancel(self.handle, &mut error_info) };
        utils::check_rc(&error_info)?;
        self.handle = ptr::null_mut();
        Ok(())
    }

    /// Ping the remote communication partner.
    ///
    /// Sends a ping to the backend in order to check, whether the connection is still alive.
    /// Returns an error, if the connection is broken and not alive anymore.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nwrfc::protocol::Connection;
    /// # let params: &[(&str, &str)] = &[
    /// #     ("ASHOST", "foo.sap.example.com"),
    /// #     ("SYSNR", "42"),
    /// #     ("CLIENT", "1337"),
    /// #     ("USER", "cooluser"),
    /// #     ("PASSWD", "secret"),
    /// #     ("TRACE", TraceLevel::Full.into()),
    /// # ];
    /// let con = Connection::open(params.into())?;
    /// match con.ping() {
    ///     Ok(_) => print!("Connection alive!"),
    ///     Err(e) => print!("Error in connection: {}", e),
    /// };
    /// ```
    pub fn ping(&self) -> RfcResult<()> {
        let mut error_info = RFC_ERROR_INFO::default();
        let rc = unsafe { RfcPing(self.handle, &mut error_info) };
        match rc {
            RFC_RC::RFC_OK => Ok(()),
            _ => Err((&error_info).into()),
        }
    }

    /// See documentation of [`ConnectionAttributes`].
    pub fn attributes(&self) -> RfcResult<ConnectionAttributes> {
        let mut error_info = RFC_ERROR_INFO::default();
        let mut attributes = RFC_ATTRIBUTES::default();
        unsafe { RfcGetConnectionAttributes(self.handle, &mut attributes, &mut error_info) };
        match utils::check_rc(&error_info) {
            Err(e) => Err(e),
            Ok(_) => Ok(attributes.into()),
        }
    }

    /// Returns the type description that is valid for the connected system.
    ///
    /// If the type description is already in the repository cache for that system ID, it will be
    /// returned immediately (from the cache), otherwise it will be looked up in the system's DDIC
    /// using the connection. The result from the DDIC lookup will then be placed into the cache
    /// for later use.
    ///
    /// The RFC Runtime maintains a cache for every R/3 System ID, as the meta data could be
    /// different from R/3 release to R/3 release.
    ///
    /// **_NOTE_**: Normally it should not be necessary to lookup separate structure descriptions.
    /// They are already looked up as part of the function module, in which they are used.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nwrfc::protocol::Connection;
    /// # let params: &[(&str, &str)] = &[
    /// #     ("ASHOST", "foo.sap.example.com"),
    /// #     ("SYSNR", "42"),
    /// #     ("CLIENT", "1337"),
    /// #     ("USER", "cooluser"),
    /// #     ("PASSWD", "secret"),
    /// #     ("TRACE", TraceLevel::Full.into()),
    /// # ];
    /// let con = Connection::open(params.into())?;
    ///
    /// let type_desc = con.describe_type("BAPIRET2")?;
    /// for field in &type_desc {
    ///     println!("{}: {}", field.name(), field.field_type());
    /// }
    /// ```
    pub fn describe_type<N: AsRef<str>>(&self, name: N) -> RfcResult<TypeDescription> {
        let uc_name = UCString::from(name);
        let mut error_info = RFC_ERROR_INFO::default();
        let handle = unsafe { RfcGetTypeDesc(self.handle, uc_name.as_ptr(), &mut error_info) };
        match utils::check_rc(&error_info) {
            Ok(_) => Ok(handle.into()),
            Err(e) => Err(e),
        }
    }

    /// Returns the function description that is valid for the system to which rfcHandle points to.
    ///
    /// If the function description is already in the repository cache for that system ID,
    /// it will be returned immediately (from the cache), otherwise it will be looked up in the
    /// system's DDIC using this [`Connection`]. The result from the DDIC lookup will then be
    /// placed into the cache for later use.
    ///
    /// The RFC Runtime maintains a cache for every R/3 System ID, as the meta data could be
    /// different from R/3 release to R/3 release.
    /// This is the main API that should be used.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nwrfc::protocol::Connection;
    /// # let params: &[(&str, &str)] = &[
    /// #     ("ASHOST", "foo.sap.example.com"),
    /// #     ("SYSNR", "42"),
    /// #     ("CLIENT", "1337"),
    /// #     ("USER", "cooluser"),
    /// #     ("PASSWD", "secret"),
    /// #     ("TRACE", TraceLevel::Full.into()),
    /// # ];
    /// let con = Connection::open(params.into())?;
    ///
    /// let function_description = con.describe_function("BAPI_MATERIAL_SAVEDATA")?;
    /// for parameter in function_description.parameters(ParameterDirection::Import) {
    ///     println!("{}: {}", parameter.name(), parameter.parameter_type());
    /// }
    /// ```
    pub fn describe_function<N: AsRef<str>>(&self, name: N) -> RfcResult<FunctionDescription> {
        let mut error_info = RFC_ERROR_INFO::default();
        let uc_name = UCString::from(name);
        let handle = unsafe { RfcGetFunctionDesc(self.handle, uc_name.as_ptr(), &mut error_info) };
        utils::check_rc(&error_info)?;
        Ok(handle.into())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        match self.close() {
            Ok(_) => {}
            Err(e) => panic!("Error closing the connection while dropping: {}", e),
        }
    }
}
