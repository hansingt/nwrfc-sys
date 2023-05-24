//! This module implements wrappers for all the types defined in the SAP NetWeaver RFC library.
mod connection_attributes;
mod connection_parameter;
mod date_time;
mod error;
mod exception;
mod field_description;
mod ids;
mod parameter_description;
mod type_description;

use crate::_unsafe::{
    RFC_BCD, RFC_BYTE, RFC_CDAY, RFC_DECF16, RFC_DECF34, RFC_DTDAY, RFC_DTMONTH, RFC_DTWEEK,
    RFC_FLOAT, RFC_INT, RFC_INT1, RFC_INT2, RFC_NUM, RFC_TMINUTE, RFC_TSECOND, RFC_UTCLONG,
    RFC_UTCMINUTE, RFC_UTCSECOND,
};

// Export aliases for the types which do not require any wrapping.
pub type RfcNum = RFC_NUM;
pub type RfcByte = RFC_BYTE;
pub type RfcBCD = RFC_BCD;
pub type RfcInt1 = RFC_INT1;
pub type RfcInt2 = RFC_INT2;
pub type RfcInt = RFC_INT;
pub type RfcFloat = RFC_FLOAT;
pub type RfcDecF16 = RFC_DECF16;
pub type RfcDecF34 = RFC_DECF34;
pub type RfcUTCLong = RFC_UTCLONG;
pub type RfcUTCSecond = RFC_UTCSECOND;
pub type RfcUTCMinute = RFC_UTCMINUTE;
pub type RfcDTDay = RFC_DTDAY;
pub type RfcDTWeek = RFC_DTWEEK;
pub type RfcDTMonth = RFC_DTMONTH;
pub type RfcTSecond = RFC_TSECOND;
pub type RfcTMinute = RFC_TMINUTE;
pub type RfcCDay = RFC_CDAY;

pub use connection_attributes::ConnectionAttributes;
pub use connection_parameter::ConnectionParameters;
pub use date_time::*;
pub use error::{RfcError, RfcResult};
pub use exception::ExceptionDescription;
pub use field_description::FieldDescription;
pub use ids::{TransactionID, UnitID, UnitIdentifier};
pub use parameter_description::ParameterDescription;
pub use type_description::{TypeDesc, TypeDescription};
