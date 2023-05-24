macro_rules! sap_enum {
    ($rfc_type:ty, $(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident = $val:path,)*
    }) => {
        $(#[$meta])*
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        $vis enum $name {
            $($(#[$vmeta])* $vname = $val as isize,)*
        }
        impl From<$rfc_type> for $name {
            fn from(value: $rfc_type) -> Self {
                match value {
                    $($val => Self::$vname,)*
                }
            }
        }
        impl From<&$name> for $rfc_type {
            fn from(value: &$name) -> Self {
                match value {
                    $($name::$vname => $val,)*
                }
            }
        }
        impl From<$name> for $rfc_type {
            #[inline]
            fn from(value: $name) -> Self {
                (&value).into()
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$vname => write!(f, "{}", stringify!($vname)),)*
                }
            }
        }
    }
}
