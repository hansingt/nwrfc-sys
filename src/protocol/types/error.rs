use crate::_unsafe::RFC_ERROR_INFO;
use crate::protocol::enums::{ErrorGroup, ReturnCode};
use crate::protocol::UCStr;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

/// Detailed information about the error that has occurred.
///
/// Used in all functions of the NW RFC library to return detailed information about
/// an error that has just occurred. This can be an error that the communication partner
/// sent back to us, an error that occurred in the network layer or operating system,
/// an internal error in the NW RFC library or an error that the application programmer
/// (i.e. you) has committed...
///
/// Within a server function implementation, the application programmer (you) can return
/// this structure to the RFC library in order to specify the error type & message that
/// you want to send back to the backend.
#[derive(Debug, Clone)]
pub struct RfcError {
    /// Error code.
    pub code: ReturnCode,
    /// Error group.
    pub group: ErrorGroup,
    /// Error key
    pub key: String,
    /// Error message
    pub message: String,
    /// ABAP message ID, or class
    pub abap_msg_class: String,
    /// ABAP message type, e.g. 'E', 'A', or 'X'
    pub abap_msg_type: String,
    /// ABAP message number
    pub abap_msg_number: String,
    /// ABAP message details field 1, corresponds to SY-MSGV1
    pub abap_msg_v1: String,
    /// ABAP message details field 2, corresponds to SY-MSGV2
    pub abap_msg_v2: String,
    /// ABAP message details field 3, corresponds to SY-MSGV3
    pub abap_msg_v3: String,
    /// ABAP message details field 4, corresponds to SY-MSGV4
    pub abap_msg_v4: String,
}

impl Default for RfcError {
    fn default() -> Self {
        Self {
            code: ReturnCode::default(),
            group: ErrorGroup::default(),
            key: "".to_string(),
            message: "".to_string(),
            abap_msg_class: "".to_string(),
            abap_msg_type: "".to_string(),
            abap_msg_number: "".to_string(),
            abap_msg_v1: "".to_string(),
            abap_msg_v2: "".to_string(),
            abap_msg_v3: "".to_string(),
            abap_msg_v4: "".to_string(),
        }
    }
}

impl fmt::Display for RfcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "RFC-Error:")?;
        writeln!(f, "\tcode: {}", self.code)?;
        writeln!(f, "\tgroup: {}", self.group)?;
        writeln!(f, "\tkey: {}", self.key)?;
        writeln!(f, "\tmessage: {}", self.message)?;
        writeln!(f, "\tabapMsgClass: {}", self.abap_msg_class)?;
        writeln!(f, "\tabapMsgType: {}", self.abap_msg_type)?;
        writeln!(f, "\tabapMsgNumber: {}", self.abap_msg_number)?;
        writeln!(f, "\tabapMsgV1: {}", self.abap_msg_v1)?;
        writeln!(f, "\tabapMsgV2: {}", self.abap_msg_v2)?;
        writeln!(f, "\tabapMsgV3: {}", self.abap_msg_v3)?;
        writeln!(f, "\tabapMsgV4: {}", self.abap_msg_v4)
    }
}

impl Error for RfcError {}

impl From<&RFC_ERROR_INFO> for RfcError {
    fn from(value: &RFC_ERROR_INFO) -> Self {
        Self {
            code: value.code.into(),
            group: value.group.into(),
            key: UCStr::from_slice(&value.key).to_string_lossy(),
            message: UCStr::from_slice(&value.key).to_string_lossy(),
            abap_msg_class: UCStr::from_slice(&value.abapMsgClass).to_string_lossy(),
            abap_msg_type: UCStr::from_slice(&value.abapMsgType).to_string_lossy(),
            abap_msg_number: UCStr::from_slice(&value.abapMsgNumber).to_string_lossy(),
            abap_msg_v1: UCStr::from_slice(&value.abapMsgV1).to_string_lossy(),
            abap_msg_v2: UCStr::from_slice(&value.abapMsgV2).to_string_lossy(),
            abap_msg_v3: UCStr::from_slice(&value.abapMsgV3).to_string_lossy(),
            abap_msg_v4: UCStr::from_slice(&value.abapMsgV4).to_string_lossy(),
        }
    }
}

impl From<RFC_ERROR_INFO> for RfcError {
    fn from(value: RFC_ERROR_INFO) -> Self {
        (&value).into()
    }
}

impl TryFrom<&RfcError> for RFC_ERROR_INFO {
    type Error = RfcError;

    fn try_from(value: &RfcError) -> Result<Self, Self::Error> {
        let mut result = RFC_ERROR_INFO::default();
        result.code = value.code.into();
        result.group = value.group.into();
        UCStr::from_slice_mut(&mut result.key).write(&value.key)?;
        UCStr::from_slice_mut(&mut result.message).write(&value.message)?;
        UCStr::from_slice_mut(&mut result.abapMsgClass).write(&value.abap_msg_class)?;
        UCStr::from_slice_mut(&mut result.abapMsgType).write(&value.abap_msg_type)?;
        UCStr::from_slice_mut(&mut result.abapMsgNumber).write(&value.abap_msg_number)?;
        UCStr::from_slice_mut(&mut result.abapMsgV1).write(&value.abap_msg_v1)?;
        UCStr::from_slice_mut(&mut result.abapMsgV2).write(&value.abap_msg_v2)?;
        UCStr::from_slice_mut(&mut result.abapMsgV3).write(&value.abap_msg_v3)?;
        UCStr::from_slice_mut(&mut result.abapMsgV4).write(&value.abap_msg_v4)?;
        Ok(result)
    }
}

/// Result returned by all SAP NetWeaver RFC functions.
///
/// This result either contains the generic type `T` in case the execution
/// as successful. Or it returns a [`RfcError`] which will describe the actual
/// error that has happened and give additional information about that error.
pub type RfcResult<T> = Result<T, RfcError>;
