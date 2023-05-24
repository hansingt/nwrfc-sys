use crate::_unsafe::{RFC_TID, RFC_UNITID, RFC_UNIT_IDENTIFIER, SAP_UC};
use crate::protocol::UCStr;
use std::fmt;

macro_rules! sap_id {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($rfc_type:ty)) => {
        $(#[$meta])*
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        $vis struct $name($rfc_type);
        impl From<$rfc_type> for $name {
            fn from(id: $rfc_type) -> $name {
                $name { 0: id }
            }
        }
        impl From<$name> for $rfc_type {
            fn from(id: $name) -> $rfc_type {
                id.0
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let id = UCStr::from_slice(&self.0).to_string_lossy();
                write!(f, "{}", id)
            }
        }
    }
}

sap_id! {
    /// todo!
    pub struct TransactionID(RFC_TID)
}

sap_id! {
    /// todo!
    pub struct UnitID(RFC_UNITID)
}

/// For convenience combines a [UnitID] and its type.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct UnitIdentifier(RFC_UNIT_IDENTIFIER);

impl UnitIdentifier {
    /// The type of the unit.
    /// 'T' for "transactional" behavior (unit is executed synchronously),
    /// 'Q' for "queued" behavior (unit is written into a queue and executed asynchronously).
    #[inline]
    pub const fn unit_type(&self) -> char {
        if self.0.unitType == 'T' as SAP_UC {
            'T'
        } else if self.0.unitType == 'Q' as SAP_UC {
            'Q'
        } else {
            panic!("Unknown unit type!")
        }
    }

    // The 32 digit unit ID of the background unit.
    #[inline]
    pub fn unit_id(&self) -> UnitID {
        UnitID::from(self.0.unitID)
    }
}

impl From<RFC_UNIT_IDENTIFIER> for UnitIdentifier {
    fn from(identifier: RFC_UNIT_IDENTIFIER) -> Self {
        Self { 0: identifier }
    }
}

impl From<UnitIdentifier> for RFC_UNIT_IDENTIFIER {
    fn from(identifier: UnitIdentifier) -> Self {
        identifier.0
    }
}
