use crate::_unsafe::RFC_EXCEPTION_DESC;
use crate::protocol::UCStr;

/// Structure for reading [`get_exception_by_index`] or [`get_exception_by_name`]
/// or defining [`add_exception`] the properties of an exception key in a function module.
///
/// [`get_exception_by_index`]: crate::protocol::FunctionDescription::get_exception_by_index
/// [`get_exception_by_name`]: crate::protocol::FunctionDescription::get_exception_by_name
/// [`add_exception`]: crate::protocol::FunctionDescription::add_exception
///
#[repr(transparent)]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ExceptionDescription {
    desc: RFC_EXCEPTION_DESC,
}

impl ExceptionDescription {
    /// todo!
    pub fn key(&self) -> String {
        UCStr::from_slice(&self.desc.key).to_string_lossy()
    }

    /// todo!
    pub fn message(&self) -> String {
        UCStr::from_slice(&self.desc.message).to_string_lossy()
    }
}

impl From<RFC_EXCEPTION_DESC> for ExceptionDescription {
    #[inline(always)]
    fn from(desc: RFC_EXCEPTION_DESC) -> Self {
        Self { desc }
    }
}

impl From<ExceptionDescription> for RFC_EXCEPTION_DESC {
    #[inline(always)]
    fn from(desc: ExceptionDescription) -> Self {
        desc.desc
    }
}
