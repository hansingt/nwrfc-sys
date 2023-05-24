use crate::_unsafe::RFC_FIELD_DESC;
use crate::protocol::{Type, TypeDesc, UCStr};
use std::marker::PhantomData;

/// todo!
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FieldDescription<'a> {
    handle: RFC_FIELD_DESC,
    _type_ref: PhantomData<Type<'a>>,
}

impl<'a> FieldDescription<'a> {
    /// todo!
    #[inline]
    pub fn name(&self) -> String {
        UCStr::from_slice(&self.handle.name).to_string_lossy()
    }

    /// todo!
    #[inline]
    pub fn field_type(&self) -> Type {
        let type_description = if self.handle.typeDescHandle.is_null() {
            None
        } else {
            Some(unsafe { TypeDesc::from_handle::<'a>(self.handle.typeDescHandle) })
        };
        Type::from_rfc_type(
            self.handle.type_,
            self.length(),
            self.decimals(),
            type_description,
        )
    }

    /// todo!
    #[inline]
    pub fn length(&self) -> u32 {
        self.handle.nucLength
    }

    /// todo!
    #[inline]
    pub fn decimals(&self) -> u32 {
        self.handle.decimals
    }
}

impl<'a> From<RFC_FIELD_DESC> for FieldDescription<'a> {
    #[inline]
    fn from(handle: RFC_FIELD_DESC) -> Self {
        Self {
            handle,
            _type_ref: PhantomData::default(),
        }
    }
}

impl<'a> From<FieldDescription<'a>> for RFC_FIELD_DESC {
    fn from(value: FieldDescription<'a>) -> Self {
        value.handle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{RfcResult, TypeDescription};

    #[test]
    fn test_multiple_fields() -> RfcResult<()> {
        let type1 = TypeDescription::new("TYPE1")?;
        let mut type2 = TypeDescription::new("TYPE2")?;
        type2.add_field("FIELD1", Type::Structure(&type1))?;
        type2.add_field("FIELD2", Type::Structure(&type1))?;

        type2.get("FIELD1").expect("FIELD1 not found!");
        type2.get("FIELD2").expect("FIELD2 not found!");
        Ok(())
    }
}
