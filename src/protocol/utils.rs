//! todo!

use crate::_unsafe::{RFC_ERROR_INFO, RFC_RC};
use crate::protocol::RfcResult;

/// todo!
pub fn check_rc(error_info: &RFC_ERROR_INFO) -> RfcResult<()> {
    match error_info.code {
        RFC_RC::RFC_OK => Ok(()),
        _ => Err(error_info.into()),
    }
}
