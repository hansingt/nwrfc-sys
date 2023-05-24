use crate::_unsafe::RFC_ATTRIBUTES;
use crate::protocol::{TraceLevel, UCStr};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Structure returned by [`Connection::get_attributes`] giving some
/// information about the partner system on the other side of this RFC connection.
///
/// [`Connection::get_attributes`]: crate::protocol::Connection::get_attributes
///
#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConnectionAttributes {
    attrs: RFC_ATTRIBUTES,
}

impl ConnectionAttributes {
    #[inline]
    #[must_use]
    pub(crate) const fn from_attrs(attrs: RFC_ATTRIBUTES) -> Self {
        Self { attrs }
    }

    /// RFC destination
    #[inline]
    pub fn dest(&self) -> String {
        UCStr::from_slice(&self.attrs.dest).to_string_lossy()
    }

    /// Own host name
    #[inline]
    pub fn host(&self) -> String {
        UCStr::from_slice(&self.attrs.host).to_string_lossy()
    }

    /// Partner host name
    #[inline]
    pub fn partner_host(&self) -> String {
        UCStr::from_slice(&self.attrs.partnerHost).to_string_lossy()
    }

    /// R/3 system number
    #[inline]
    pub fn sys_number(&self) -> String {
        UCStr::from_slice(&self.attrs.sysNumber).to_string_lossy()
    }

    /// R/3 system ID
    #[inline]
    pub fn sys_id(&self) -> String {
        UCStr::from_slice(&self.attrs.sysId).to_string_lossy()
    }

    /// Client ("Mandant")
    #[inline]
    pub fn client(&self) -> String {
        UCStr::from_slice(&self.attrs.client).to_string_lossy()
    }

    /// User
    #[inline]
    pub fn user(&self) -> String {
        UCStr::from_slice(&self.attrs.user).to_string_lossy()
    }

    /// Language
    #[inline]
    pub fn language(&self) -> String {
        UCStr::from_slice(&self.attrs.language).to_string_lossy()
    }

    /// Trace level (0-3)
    #[inline]
    pub fn trace(&self) -> TraceLevel {
        let level = UCStr::from_slice(&self.attrs.trace).to_string_lossy();
        TraceLevel::try_from(level).expect("Invalid trace level from connection attributes")
    }

    /// 2 characters ISO langauge code
    #[inline]
    pub fn iso_language(&self) -> String {
        UCStr::from_slice(&self.attrs.isoLanguage).to_string_lossy()
    }

    /// Own code page
    pub fn codepage(&self) -> String {
        UCStr::from_slice(&self.attrs.codepage).to_string_lossy()
    }

    /// Partner code page
    #[inline]
    pub fn partner_codepage(&self) -> String {
        UCStr::from_slice(&self.attrs.partnerCodepage).to_string_lossy()
    }

    /// RFC Client (C) or RFC Server (S)
    #[inline]
    pub fn rfc_role(&self) -> String {
        UCStr::from_slice(&self.attrs.rfcRole).to_string_lossy()
    }

    /// Own system type: R/2 (2), R/3 (3), External (E), Registered External (R),
    #[inline]
    pub fn system_type(&self) -> String {
        UCStr::from_slice(&self.attrs.type_).to_string_lossy()
    }

    /// Partner system type: R/2 (2), R/3 (3), External (E), Registered External (R),
    #[inline]
    pub fn partner_system_type(&self) -> String {
        UCStr::from_slice(&self.attrs.partnerType).to_string_lossy()
    }

    /// Own system release
    #[inline]
    pub fn release(&self) -> String {
        UCStr::from_slice(&self.attrs.rel).to_string_lossy()
    }
    /// Partner system release
    #[inline]
    pub fn partner_release(&self) -> String {
        UCStr::from_slice(&self.attrs.partnerRel).to_string_lossy()
    }

    /// Partner kernel release
    #[inline]
    pub fn partner_kernel_release(&self) -> String {
        UCStr::from_slice(&self.attrs.kernelRel).to_string_lossy()
    }
    /// CPI-C conversion ID
    pub fn cpic_conversion_id(&self) -> String {
        UCStr::from_slice(&self.attrs.cpicConvId).to_string_lossy()
    }
    /// Name of the calling ABAP program (report, module pool)
    pub fn program_name(&self) -> String {
        UCStr::from_slice(&self.attrs.progName).to_string_lossy()
    }

    /// Number of bytes per character in the partners current codepage.
    ///
    /// **_Note:_** This is different from the semantics of the PCS parameter.
    pub fn partner_bytes_per_char(&self) -> u32 {
        let s = UCStr::from_slice(&self.attrs.partnerBytesPerChar).to_string_lossy();
        s.parse()
            .expect("Unable to parse partner bytes per character")
    }

    /// Partner system code page
    pub fn partner_system_codepage(&self) -> String {
        UCStr::from_slice(&self.attrs.partnerSystemCodepage).to_string_lossy()
    }
    /// Partner IP
    pub fn partner_ip(&self) -> Ipv4Addr {
        let s = UCStr::from_slice(&self.attrs.partnerIP).to_string_lossy();
        s.parse().expect("Unable to parse partner IPv4 address")
    }
    /// Partner IPv6
    pub fn partner_ipv6(&self) -> Ipv6Addr {
        let s = UCStr::from_slice(&self.attrs.partnerIPv6).to_string_lossy();
        s.parse().expect("Unable to parse partner IPv6 address")
    }
}

impl From<RFC_ATTRIBUTES> for ConnectionAttributes {
    #[inline(always)]
    fn from(value: RFC_ATTRIBUTES) -> Self {
        ConnectionAttributes::from_attrs(value)
    }
}
