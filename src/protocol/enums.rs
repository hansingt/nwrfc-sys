//! This module implements wrappers for the enumerations defined in the SAP NetWeaver RFC library.
//! These enumerations will be used by the abstraction layer protocol, but can also be passed
//! directly to the unsafe functions of the RFC wrapper.
//!
//! They all implement conversion functions from / to the RFC enumeration type as well as
//! [`std::fmt::Display`], which allows to print the name of the value.
use crate::_unsafe::{
    RFCTYPE, RFC_AUTHENTICATION_TYPE, RFC_CALL_TYPE, RFC_CLASS_ATTRIBUTE_TYPE, RFC_DIRECTION,
    RFC_ERROR_GROUP, RFC_METADATA_OBJ_TYPE, RFC_PROTOCOL_TYPE, RFC_RC, RFC_SERVER_STATE,
    RFC_SESSION_EVENT, RFC_UNIT_STATE, _RFCTYPE,
};
use crate::protocol::TypeDesc;
use std::fmt::Formatter;

/// Field or parameter type when describing a structure or function.
///
/// [`Type`] is used in field descriptions [`FieldDescription`] and parameter descriptions
/// [`ParameterDescription`] and denotes the ABAP data type of the corresponding
/// field / parameter.
///
/// [`FieldDescription`]: crate::protocol::FieldDescription
/// [`ParameterDescription`]: crate::protocol::ParameterDescription
///
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Type<'a> {
    /// 1-byte or multibyte character, fixed size of given length, blank padded.
    Char(u32) = RFCTYPE::RFCTYPE_CHAR as u32,
    /// Date (YYYYYMMDD)
    Date = RFCTYPE::RFCTYPE_DATE as u32,
    /// Packed number, any length between 1 and 16 bytes.
    ///
    /// The first parameter defines the length,
    /// the second one the number of decimal places.
    BCD(u32, u32) = RFCTYPE::RFCTYPE_BCD as u32,
    /// Time (HHMMSS)
    Time = RFCTYPE::RFCTYPE_TIME as u32,
    /// Raw data, binary, fixed length of given length, zero padded.
    Byte(u32) = RFCTYPE::RFCTYPE_BYTE as u32,
    /// Internal table.
    /// The parameter defines the type describing the table lines.
    Table(&'a TypeDesc) = RFCTYPE::RFCTYPE_TABLE as u32,
    /// Digits, fixed size of given length, leading zero padded.
    Num(u32) = RFCTYPE::RFCTYPE_NUM as u32,
    /// Floating point, double precision
    Float = RFCTYPE::RFCTYPE_FLOAT as u32,
    /// 4-byte integer
    Int = RFCTYPE::RFCTYPE_INT as u32,
    /// 2-byte integer. Obsolete, not directly supported by ABAP/4.
    Int2 = RFCTYPE::RFCTYPE_INT2 as u32,
    /// 1-byte integer, unsigned. Obsolete, not directly supported by ABAP/4.
    Int1 = RFCTYPE::RFCTYPE_INT1 as u32,
    /// Not supported data type.
    Null = RFCTYPE::RFCTYPE_NULL as u32,
    /// ABAP object.
    ABAPObject = RFCTYPE::RFCTYPE_ABAPOBJECT as u32,
    /// ABAP structure.
    /// The parameter defines the type describing the fields of the structure.
    Structure(&'a TypeDesc) = RFCTYPE::RFCTYPE_STRUCTURE as u32,
    /// IEEE 754r decimal floating point, 8 bytes
    DecF16 = RFCTYPE::RFCTYPE_DECF16 as u32,
    /// IEEE 754r decimal floating point, 16 bytes
    DecF34 = RFCTYPE::RFCTYPE_DECF34 as u32,
    /// No longer used!
    XMLData(u32) = RFCTYPE::RFCTYPE_XMLDATA as u32,
    /// Variable-length, null-terminated string
    String = RFCTYPE::RFCTYPE_STRING as u32,
    /// Variable-length raw string, length in bytes
    XString = RFCTYPE::RFCTYPE_XSTRING as u32,
    /// 8-byte integer
    Int8 = RFCTYPE::RFCTYPE_INT8 as u32,
    /// timestamp/long, 8-byte integer
    UTCLong = RFCTYPE::RFCTYPE_UTCLONG as u32,
    /// timestamp/second, 8-byte integer
    UTCSecond = RFCTYPE::RFCTYPE_UTCSECOND as u32,
    /// timestamp/minute, 8-byte integer
    UTCMinute = RFCTYPE::RFCTYPE_UTCMINUTE as u32,
    /// date/day , 4-byte integer
    DTDay = RFCTYPE::RFCTYPE_DTDAY as u32,
    /// date/week, 4-byte integer
    DTWeek = RFCTYPE::RFCTYPE_DTWEEK as u32,
    /// date/month, 4-byte integer
    DTMonth = RFCTYPE::RFCTYPE_DTMONTH as u32,
    /// time/second, 4-byte integer
    TSecond = RFCTYPE::RFCTYPE_TSECOND as u32,
    /// time/minute, 2-byte integer
    TMinute = RFCTYPE::RFCTYPE_TMINUTE as u32,
    /// calendar day, 2-byte integer
    CDay = RFCTYPE::RFCTYPE_CDAY as u32,
    /// boxed structure, note: not supported by NW RFC lib
    Box = RFCTYPE::RFCTYPE_BOX as u32,
    /// boxed client dependent structure, note: not supported by NW RFC lib
    GenericBox = RFCTYPE::RFCTYPE_GENERIC_BOX as u32,
}

impl<'a> Type<'a> {
    /// todo!
    #[inline]
    pub fn len(&self) -> (u32, u32) {
        match self {
            Type::Structure(t) | Type::Table(t) => {
                match t.inlineable() {
                    // Structure definitions without any pointer fields are inlined.
                    // Thus, the length is calculated by adding the length of the structure
                    true => (t.nuc_length(), t.uc_length()),
                    // Structure will be added as a 64 bit pointer.
                    false => (8, 8),
                }
            }
            Type::CDay => (2, 2),
            Type::Date => (8, 16),
            Type::DTDay | Type::DTWeek | Type::DTMonth => (4, 4),
            Type::DecF16 => (8, 8),
            Type::DecF34 => (16, 16),
            Type::Float => (8, 8),
            Type::Int => (4, 4),
            Type::Int1 => (1, 1),
            Type::Int2 => (2, 2),
            Type::Int8 => (8, 8),
            Type::Time => (6, 12),
            Type::TSecond => (4, 4),
            Type::TMinute => (2, 2),
            Type::String | Type::XString | Type::ABAPObject | Type::Box | Type::GenericBox => {
                (8, 8)
            }
            Type::Char(len) | Type::BCD(len, _) | Type::Num(len) => (*len, *len * 2),
            Type::XMLData(length) | Type::Byte(length) => (*length, *length),
            Type::UTCLong | Type::UTCSecond | Type::UTCMinute => (8, 8),
            Type::Null => (0, 0),
        }
    }

    /// todo!
    #[inline]
    pub fn decimals(&self) -> u32 {
        match self {
            Type::BCD(_, decimals) => *decimals,
            _ => 0,
        }
    }

    /// todo!
    #[inline]
    pub fn type_description(&self) -> Option<&TypeDesc> {
        match self {
            Type::Structure(t) | Type::Table(t) => Some(t),
            _ => None,
        }
    }

    /// todo!
    #[inline]
    pub(crate) fn from_rfc_type(
        rfc_type: RFCTYPE,
        length: u32,
        decimals: u32,
        type_description: Option<&'a TypeDesc>,
    ) -> Self {
        match rfc_type {
            _RFCTYPE::RFCTYPE_CHAR => Type::Char(length),
            _RFCTYPE::RFCTYPE_DATE => Type::Date,
            _RFCTYPE::RFCTYPE_BCD => Type::BCD(length, decimals),
            _RFCTYPE::RFCTYPE_TIME => Type::Time,
            _RFCTYPE::RFCTYPE_BYTE => Type::Byte(length),
            _RFCTYPE::RFCTYPE_TABLE => match type_description {
                None => panic!("Table type without type description is not allowed!"),
                Some(t) => Type::Table(t),
            },
            _RFCTYPE::RFCTYPE_NUM => Type::Num(length),
            _RFCTYPE::RFCTYPE_FLOAT => Type::Float,
            _RFCTYPE::RFCTYPE_INT => Type::Int,
            _RFCTYPE::RFCTYPE_INT2 => Type::Int2,
            _RFCTYPE::RFCTYPE_INT1 => Type::Int1,
            _RFCTYPE::RFCTYPE_NULL => Type::Null,
            _RFCTYPE::RFCTYPE_ABAPOBJECT => Type::ABAPObject,
            _RFCTYPE::RFCTYPE_STRUCTURE => match type_description {
                None => panic!("Structure type without type description is not allowed!"),
                Some(t) => Type::Structure(t),
            },
            _RFCTYPE::RFCTYPE_DECF16 => Type::DecF16,
            _RFCTYPE::RFCTYPE_DECF34 => Type::DecF34,
            _RFCTYPE::RFCTYPE_XMLDATA => Type::XMLData(length),
            _RFCTYPE::RFCTYPE_STRING => Type::String,
            _RFCTYPE::RFCTYPE_XSTRING => Type::XString,
            _RFCTYPE::RFCTYPE_INT8 => Type::Int8,
            _RFCTYPE::RFCTYPE_UTCLONG => Type::UTCLong,
            _RFCTYPE::RFCTYPE_UTCSECOND => Type::UTCSecond,
            _RFCTYPE::RFCTYPE_UTCMINUTE => Type::UTCMinute,
            _RFCTYPE::RFCTYPE_DTDAY => Type::DTDay,
            _RFCTYPE::RFCTYPE_DTWEEK => Type::DTWeek,
            _RFCTYPE::RFCTYPE_DTMONTH => Type::DTMonth,
            _RFCTYPE::RFCTYPE_TSECOND => Type::TSecond,
            _RFCTYPE::RFCTYPE_TMINUTE => Type::TMinute,
            _RFCTYPE::RFCTYPE_CDAY => Type::CDay,
            _RFCTYPE::RFCTYPE_BOX => Type::Box,
            _RFCTYPE::RFCTYPE_GENERIC_BOX => Type::GenericBox,
            _RFCTYPE::_RFCTYPE_max_value => Type::Null,
        }
    }

    #[inline]
    pub(crate) fn inlineable(&self) -> bool {
        match self {
            // Pointer types are never inlineable.
            Type::String | Type::XString | Type::ABAPObject | Type::Box | Type::GenericBox => false,
            // For structures and tables, the corresponding type description determines
            // whether it's inlineable or not.
            Type::Structure(t) | Type::Table(t) => t.inlineable(),
            // Everything else is inlineable.
            _ => true,
        }
    }
}

impl<'a> From<Type<'a>> for RFCTYPE {
    #[inline]
    fn from(value: Type<'a>) -> Self {
        // SAFETY: the values int repr is exactly the same
        // as RFCTYPE. Thus, we can simply transmute it here.
        unsafe { *(&value as *const Type as *const u32 as *const _RFCTYPE) }
    }
}

impl<'a> std::fmt::Display for Type<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Char(len) => write!(f, "Char({})", len),
            Type::Date => write!(f, "Date"),
            Type::BCD(len, decimals) => write!(f, "BCD({}, {})", len, decimals),
            Type::Time => write!(f, "Time"),
            Type::Byte(len) => write!(f, "Byte({})", len),
            Type::Table(t) => write!(f, "Table({})", t.name()),
            Type::Num(len) => write!(f, "Num({})", len),
            Type::Float => write!(f, "Float"),
            Type::Int => write!(f, "Int"),
            Type::Int2 => write!(f, "Int2"),
            Type::Int1 => write!(f, "Int1"),
            Type::Null => write!(f, "Null"),
            Type::ABAPObject => write!(f, "ABAP Object"),
            Type::Structure(t) => write!(f, "Structure({})", t.name()),
            Type::DecF16 => write!(f, "DecF16"),
            Type::DecF34 => write!(f, "DecF34"),
            Type::XMLData(len) => write!(f, "XMLData({})", len),
            Type::String => write!(f, "String"),
            Type::XString => write!(f, "XString"),
            Type::Int8 => write!(f, "Int8"),
            Type::UTCLong => write!(f, "UTC Timestamp"),
            Type::UTCSecond => write!(f, "UTC Seconds"),
            Type::UTCMinute => write!(f, "UTC Minutes"),
            Type::DTDay => write!(f, "Datetime: Day"),
            Type::DTWeek => write!(f, "Datetime: Week"),
            Type::DTMonth => write!(f, "Datetime: Month"),
            Type::TSecond => write!(f, "Time: Second"),
            Type::TMinute => write!(f, "Time: Minute"),
            Type::CDay => write!(f, "Calendar Day"),
            Type::Box => write!(f, "Box"),
            Type::GenericBox => write!(f, "Generic Box"),
        }
    }
}

sap_enum! {
    RFC_RC,
    /// Return codes used by the SAP NetWeaver RFC functions.
    ///
    /// Return codes are used by all functions that do not directly return a handle.
    /// Also used as error indicator in the structure [`RfcError::code`].
    ///
    /// [`RfcError::code`]: crate::protocol::RfcError::code
    ///
    pub enum ReturnCode {
        /// Everything O.K. Used by every function
        Ok = RFC_RC::RFC_OK,
        /// Error in Network & Communication layer
        CommunicationFailure = RFC_RC::RFC_COMMUNICATION_FAILURE,
        /// Unable to logon to SAP system. Invalid password, user locked, etc.
        LogonFailure = RFC_RC::RFC_LOGON_FAILURE,
        /// SAP system runtime error (SYSTEM_FAILURE): Shortdump on the backend side
        ABAPRuntimeFailure = RFC_RC::RFC_ABAP_RUNTIME_FAILURE,
        /// The called function module raised an E-, A- or X-Message
        ABAPMessage = RFC_RC::RFC_ABAP_MESSAGE,
        /// The called function module raised an Exception (RAISE or MESSAGE ... RAISING)
        ABAPException = RFC_RC::RFC_ABAP_EXCEPTION,
        /// Connection closed by the other side
        Closed = RFC_RC::RFC_CLOSED,
        /// No longer used
        Canceled = RFC_RC::RFC_CANCELED,
        /// The operation timed out
        Timeout = RFC_RC::RFC_TIMEOUT,
        /// Memory insufficient
        MemoryInsuffcient = RFC_RC::RFC_MEMORY_INSUFFICIENT,
        /// Version mismatch
        VersionMismatch = RFC_RC::RFC_VERSION_MISMATCH,
        /// The received data has an unsupported format
        InvalidProtocol = RFC_RC::RFC_INVALID_PROTOCOL,
        /// A problem while serializing or deserializing RFM parameters
        SerializationFailure = RFC_RC::RFC_SERIALIZATION_FAILURE,
        /// An invalid handle was passed to an API call
        InvalidHandle = RFC_RC::RFC_INVALID_HANDLE,
        /// RfcListenAndDispatch did not receive an RFC request during the timeout period
        Retry = RFC_RC::RFC_RETRY,
        /// Error in external custom code. (E.g. in the function handlers or tRFC handlers.)
        /// Results in SYSTEM_FAILURE
        ExternalFailure = RFC_RC::RFC_EXTERNAL_FAILURE,
        /// Inbound tRFC Call already executed (needs to be returned from RFC_ON_CHECK_TRANSACTION
        /// in case the TID is already known and successfully processed before.)
        Executed = RFC_RC::RFC_EXECUTED,
        /// Function or structure definition not found (Metadata API)
        NotFound = RFC_RC::RFC_NOT_FOUND,
        /// The operation is not supported on that handle
        NotSupported = RFC_RC::RFC_NOT_SUPPORTED,
        /// The operation is not supported on that handle at the current point of time
        /// (e.g. trying a callback on a server handle, while not in a call)
        IllegalState = RFC_RC::RFC_ILLEGAL_STATE,
        /// An invalid parameter was passed to an API call, (e.g. invalid name, type or length)
        InvalidParameter = RFC_RC::RFC_INVALID_PARAMETER,
        /// Codepage conversion error
        CodepageConversionFailure = RFC_RC::RFC_CODEPAGE_CONVERSION_FAILURE,
        /// Error while converting a parameter to the correct data type
        ConversionFailure = RFC_RC::RFC_CONVERSION_FAILURE,
        /// The given buffer was to small to hold the entire parameter. Data has been truncated.
        BufferTooSmall = RFC_RC::RFC_BUFFER_TOO_SMALL,
        /// Trying to move the current position before the first row of the table
        TableMoveBOF = RFC_RC::RFC_TABLE_MOVE_BOF,
        /// Trying to move the current position after the last row of the table
        TableMoveEOF = RFC_RC::RFC_TABLE_MOVE_EOF,
        /// Failed to start and attach SAPGUI to the RFC connection
        StartSAPGUIFailure = RFC_RC::RFC_START_SAPGUI_FAILURE,
        /// The called function module raised a class based exception
        ABAPClassException = RFC_RC::RFC_ABAP_CLASS_EXCEPTION,
        /// "Something" went wrong, but I don't know what...
        UnknownError = RFC_RC::RFC_UNKNOWN_ERROR,
        /// Authorization check error
        AuthorizationFailure = RFC_RC::RFC_AUTHORIZATION_FAILURE,
        /// The authentication handler (RFC_ON_AUTHENTICATION_CHECK) failed to authenticate
        /// the user trying to log on
        AuthenticationFailure = RFC_RC::RFC_AUTHENTICATION_FAILURE,
        /// Error when dealing with functions provided by the cryptolibrary
        CryptolibFailure = RFC_RC::RFC_CRYPTOLIB_FAILURE,
        /// Error when dealing with io functions, streams etc
        IOFailure = RFC_RC::RFC_IO_FAILURE,
        /// Requesting or freeing critical sections or mutex failed
        LockingFailure = RFC_RC::RFC_LOCKING_FAILURE,
        /// Don't use
        _MaxValure = RFC_RC::_RFC_RC_max_value,
    }
}
impl Default for ReturnCode {
    fn default() -> Self {
        Self::Ok
    }
}

sap_enum! {
    RFC_ERROR_GROUP,
    /// Error groups used by the SAP NetWeaver RFC functions.
    ///
    /// Groups several error conditions together, depending on the "layer" to which they belong.
    // Used in the structure [`RfcError::group`].
    ///
    /// [`RfcError::group`]: crate::protocol::RfcError::group
    ///
    pub enum ErrorGroup {
        /// OK
        Ok = RFC_ERROR_GROUP::OK,
        /// ABAP Exception raised in ABAP function modules
        ABAPApplicationFailure = RFC_ERROR_GROUP::ABAP_APPLICATION_FAILURE,
        /// ABAP Message raised in ABAP function modules or
        /// in ABAP runtime of the backend (e.g Kernel)
        ABAPRuntimeFailure = RFC_ERROR_GROUP::ABAP_RUNTIME_FAILURE,
        /// Error message raised when logon fails
        LogonFailure = RFC_ERROR_GROUP::LOGON_FAILURE,
        /// Problems with the network connection (or backend broke down and killed the connection)
        CommunicationFailure = RFC_ERROR_GROUP::COMMUNICATION_FAILURE,
        /// Problems in the RFC runtime of the external program (i.e "this" library)
        ExternalRuntimeFailure = RFC_ERROR_GROUP::EXTERNAL_RUNTIME_FAILURE,
        /// Problems in the external program (e.g in the external server implementation)
        ExternalApplicationFailure = RFC_ERROR_GROUP::EXTERNAL_APPLICATION_FAILURE,
        /// Problems raised in the authorization check handler provided by
        /// the external server implementation
        ExternalAuthorizationFailure = RFC_ERROR_GROUP::EXTERNAL_AUTHORIZATION_FAILURE,
        /// Problems raised by the authentication handler (RFC_ON_AUTHENTICATION_CHECK)
        ExtenralAuthenticationFailure = RFC_ERROR_GROUP::EXTERNAL_AUTHENTICATION_FAILURE,
        /// Problems when dealing with functions provided by the cryptolibrary
        CryptolibFailure = RFC_ERROR_GROUP::CRYPTOLIB_FAILURE,
        /// Requesting or freeing critical sections or mutex failed
        LockingFailure = RFC_ERROR_GROUP::LOCKING_FAILURE,
    }
}
impl Default for ErrorGroup {
    fn default() -> Self {
        Self::Ok
    }
}

sap_enum! {
    RFC_UNIT_STATE,
    /// Processing status of a logical unit of work (LOW).
    ///
    /// Used in [`RfcGetUnitState`] for inquiring the processing status of a background Unit that
    /// we (or someone else) sent into this backend.
    ///
    /// [`RfcGetUnitState`]: crate::_unsafe::RfcGetUnitState
    ///
    pub enum UnitState {
        /// No information for this unit ID and unit type can be found in the target system.
        /// If you are sure, that target system, unit ID and unit type are correct,
        /// it means that your previous attempt did not even reach the target system.
        /// Send the unit again. However, if you get this status after the Confirm step has
        /// already been executed, it means that everything is ok. Don't re-execute in this case!
        UnitNotFound = RFC_UNIT_STATE::RFC_UNIT_NOT_FOUND,
        /// Backend system is still in the process of persisting (or executing if type 'T')
        /// the payload data. Give it some more time and check the state again later.
        /// If this takes "too long", an admin should probably have a look at why there is
        /// no progress here.
        UnitInProcress = RFC_UNIT_STATE::RFC_UNIT_IN_PROCESS,
        /// Data has been persisted (or executed if type 'T') ok on receiver side.
        /// Confirm event may now be triggered.
        UnitCommitted = RFC_UNIT_STATE::RFC_UNIT_COMMITTED,
        /// An error of any type has occurred. Unit needs to be resent.
        UnitRolledBack = RFC_UNIT_STATE::RFC_UNIT_ROLLED_BACK,
        /// Temporary state between the Confirm event and the time, when the status data
        /// will be erased for good. Nothing to be done.
        /// Just delete the payload and status information on your side.
        UnitConfirmed = RFC_UNIT_STATE::RFC_UNIT_CONFIRMED,
    }
}

sap_enum! {
    RFC_CALL_TYPE,
    /// Used in [`RfcGetServerContext`] for inquiring the type of
    /// an incoming function call from the backend.
    ///
    /// [`RfcGetServerContext`]: crate::_unsafe::RfcGetServerContext
    pub enum CallType {
        /// It's a standard synchronous RFC call.
        Synchronous = RFC_CALL_TYPE::RFC_SYNCHRONOUS,
        /// This function call is part of a transactional LUW (tRFC).
        Transactional = RFC_CALL_TYPE::RFC_TRANSACTIONAL,
        /// This function call is part of a queued LUW (qRFC).
        Queued = RFC_CALL_TYPE::RFC_QUEUED,
        /// This function call is part of a background LUW (bgRFC).
        BackgroundUnit = RFC_CALL_TYPE::RFC_BACKGROUND_UNIT,
    }
}

sap_enum! {
    RFC_AUTHENTICATION_TYPE,
    /// Type of authentication method used by the backend authentication handler
    /// ([`RFC_ON_AUTHENTICATION_CHECK`]).
    ///
    /// [`RFC_ON_AUTHENTICATION_CHECK`]: crate::_unsafe::RFC_ON_AUTHENTICATION_CHECK
    pub enum AuthenticationType {
        /// No authentication data was provided
        None = RFC_AUTHENTICATION_TYPE::RFC_AUTH_NONE,
        /// Authentication with user and password
        Basic = RFC_AUTHENTICATION_TYPE::RFC_AUTH_BASIC,
        /// Authentication with x509 certificate
        X509 = RFC_AUTHENTICATION_TYPE::RFC_AUTH_X509,
        /// Authentication with assertion ticket
        SSO = RFC_AUTHENTICATION_TYPE::RFC_AUTH_SSO,
    }
}

sap_enum! {
    RFC_PROTOCOL_TYPE,
    /// Used in state information in order to indicate the different types of
    /// RFC programs, RFC Server types, etc.
    pub enum ProtocolType {
        /// Unspecified
        Unknown = RFC_PROTOCOL_TYPE::RFC_UNKOWN,
        /// RFC Client
        Client = RFC_PROTOCOL_TYPE::RFC_CLIENT,
        /// Started RFC Server
        StartedServer = RFC_PROTOCOL_TYPE::RFC_STARTED_SERVER,
        /// Registered RFC Server
        RegisteredServer = RFC_PROTOCOL_TYPE::RFC_REGISTERED_SERVER,
        /// Multi-count registered RFC Server
        MultiCoundRegisteredServer = RFC_PROTOCOL_TYPE::RFC_MULTI_COUNT_REGISTERED_SERVER,
        /// TCP Client
        TCPSocketClient = RFC_PROTOCOL_TYPE::RFC_TCP_SOCKET_CLIENT,
        /// TCP Server
        TCPSocketServer = RFC_PROTOCOL_TYPE::RFC_TCP_SOCKET_SERVER,
        /// Websocket RFC Client
        WebsocketClient = RFC_PROTOCOL_TYPE::RFC_WEBSOCKET_CLIENT,
        /// Websocket RFC Server
        WebsocketServer = RFC_PROTOCOL_TYPE::RFC_WEBSOCKET_SERVER,
        /// Websocket RFC Client
        ProxyWebsocketClient = RFC_PROTOCOL_TYPE::RFC_PROXY_WEBSOCKET_CLIENT,
    }
}

sap_enum! {
    RFC_SERVER_STATE,
    /// Used in state information in order to indicate the current state of an RFC Server.
    pub enum ServerState {
        /// The server object has been created, but nothing has been done with it yet.
        Initial = RFC_SERVER_STATE::RFC_SERVER_INITIAL,
        /// The server has been started, but startup is not yet complete and the server is
        /// not yet able to receive/serve requests. Should quickly switch to `Running` or `Broken`.
        Starting = RFC_SERVER_STATE::RFC_SERVER_STARTING,
        /// Means at least one registration is still able to accept request from the gateway
        /// (in case of Registered Server), or that the server port is open and listening
        /// (in case of TCP Socket Server).
        Running = RFC_SERVER_STATE::RFC_SERVER_RUNNING,
        /// Means that all registrations are dead, e.g. because of gateway being down
        /// (in case of Registered Server), or that for some reason server port could
        /// not be opened (in case of TCP Socket Server).
        Broken = RFC_SERVER_STATE::RFC_SERVER_BROKEN,
        /// The server has been stopped via [`RfcShutdownServer`] (with a timeout > 0) and is
        /// still busy processing ongoing requests. It is however no longer accepting new requests.
        /// Should switch to `Stopped`, once the ongoing requests are finished.
        ///
        /// [`RfcShutdownServer`]: crate::_unsafe::RfcShutdownServer
        ///
        Stopping = RFC_SERVER_STATE::RFC_SERVER_STOPPING,
        /// The server has been stopped via [`RfcShutdownServer`] and is currently not processing
        /// nor accepting any requests. The object, however, is still valid and can be started
        /// again anytime with [`RfcLaunchServer`].
        ///
        /// [`RfcShutdownServer`]: crate::_unsafe::RfcShutdownServer
        /// [`RfcLaunchServer`]: crate::_unsafe::RfcLaunchServer
        ///
        Stopped = RFC_SERVER_STATE::RFC_SERVER_STOPPED,
    }
}

sap_enum! {
    RFC_SESSION_EVENT,
    /// Used in a server session change listener to notify the application whenever a new user
    /// session on the server gets started or ends.
    pub enum SessionEvent {
        /// A new stateful user session has been created on the server.
        /// This can be done either by the server itself via
        /// [`RfcSetServerStateful`], or by the backend via function module
        /// `RFC_SET_REG_SERVER_PROPERTY`.
        ///
        /// [`RfcSetServerStateful`]: crate::_unsafe::RfcSetServerStateful
        ///
        Created = RFC_SESSION_EVENT::RFC_SESSION_CREATED,
        /// A function call came in from the backend and started processing.
        /// This event can probably be ignored by 99% of the applications.
        Activated = RFC_SESSION_EVENT::RFC_SESSION_ACTIVATED,
        /// A function call completed processing.
        /// This event can probably be ignored by 99% of the applications.
        Passivated = RFC_SESSION_EVENT::RFC_SESSION_PASSIVATED,
        /// A stateful user session has been destroyed, either by the server itself via
        /// [`RfcSetServerStateful`], or by the backend via function module
        /// `RFC_SET_REG_SERVER_PROPERTY`, or because the connection was closed
        /// (e.g. the corresponding ABAP user session ended or explicitly closed the connection),
        /// or because the connection was broken by network error/system failure etc.
        /// The application should now clean up all memory/resources allocated
        /// for the given session ID.
        ///
        /// [`RfcSetServerStateful`]: crate::_unsafe::RfcSetServerStateful
        ///
        Destroyed = RFC_SESSION_EVENT::RFC_SESSION_DESTROYED,
    }
}

sap_enum! {
    RFC_DIRECTION,
    /// Used in [`ParameterDescription::direction`] for specifying the direction of a
    /// function module parameter.
    ///
    /// [`ParameterDescription::direction`]: crate::protocol::ParameterDescription::direction
    ///
    pub enum ParameterDirection {
        /// Import parameter. This corresponds to ABAP IMPORTING parameter.
        Import = RFC_DIRECTION::RFC_IMPORT,
        /// Export parameter. This corresponds to ABAP EXPORTING parameter.
        Export = RFC_DIRECTION::RFC_EXPORT,
        /// Import and export parameter. This corresponds to ABAP CHANGING parameter.
        Changing = RFC_DIRECTION::RFC_CHANGING,
        /// Table parameter. This corresponds to ABAP TABLES parameter.
        Tables = RFC_DIRECTION::RFC_TABLES,
    }
}

sap_enum! {
    RFC_CLASS_ATTRIBUTE_TYPE,
    /// Determines the type of an ABAP Object attribute.
    pub enum ClassAttributeType {
        /// Instance attribute (object member)
        Instance = RFC_CLASS_ATTRIBUTE_TYPE::RFC_CLASS_ATTRIBUTE_INSTANCE,
        /// Class attribute (global)
        Class = RFC_CLASS_ATTRIBUTE_TYPE::RFC_CLASS_ATTRIBUTE_CLASS,
        /// A constant
        Constant = RFC_CLASS_ATTRIBUTE_TYPE::RFC_CLASS_ATTRIBUTE_CONSTANT,
    }
}

sap_enum! {
    RFC_METADATA_OBJ_TYPE,
    /// Indicates whether in a call to [`RfcGetMetadataQueryFailedEntry`] or
    /// [`RfcGetMetadataQuerySucceededEntry`] you are interested in the error/success message for a
    /// function module ([`MetadataObjectType::Function`]),
    /// structure/table ([`MetadataObjectType::Type`]),
    /// or ABAP Class ([`MetadataObjectType::Class`]).
    /// It needs to be passed to the above two functions.
    ///
    /// [`RfcGetMetadataQueryFailedEntry`]: crate::_unsafe::RfcGetMetadataQueryFailedEntry
    /// [`RfcGetMetadataQuerySucceededEntry`]: crate::_unsafe::RfcGetMetadataQuerySucceededEntry
    ///
    pub enum MetadataObjectType {
        /// Request the metadata of a function module
        Function = RFC_METADATA_OBJ_TYPE::RFC_METADATA_FUNCTION,
        /// Request the metadata of a structure / table
        Type = RFC_METADATA_OBJ_TYPE::RFC_METADATA_TYPE,
        /// Request the metadata of an ABAP class
        Class = RFC_METADATA_OBJ_TYPE::RFC_METADATA_CLASS,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TraceLevel {
    Off = 0,
    Brief = 1,
    Verbose = 2,
    Full = 3,
}

#[derive(Debug)]
pub struct InvalidTraceLevel {}

impl std::fmt::Display for InvalidTraceLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid trace level!")
    }
}

impl TryFrom<&str> for TraceLevel {
    type Error = InvalidTraceLevel;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(TraceLevel::Off),
            "1" => Ok(TraceLevel::Brief),
            "2" => Ok(TraceLevel::Verbose),
            "3" => Ok(TraceLevel::Full),
            _ => Err(InvalidTraceLevel {}),
        }
    }
}

impl TryFrom<String> for TraceLevel {
    type Error = InvalidTraceLevel;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<&TraceLevel> for &'static str {
    fn from(value: &TraceLevel) -> Self {
        match value {
            TraceLevel::Off => "0",
            TraceLevel::Brief => "1",
            TraceLevel::Verbose => "2",
            TraceLevel::Full => "3",
        }
    }
}

impl std::fmt::Display for TraceLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.into();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::TypeDescription;

    #[test]
    fn display_type() {
        let t = Type::Char(1);
        let t_str = format!("{}", t);
        assert_eq!(t_str, "Char(1)");

        let type_desc = TypeDescription::new("TEST").expect("Could not create new type");
        let t = Type::Structure(&type_desc);
        let t_str = format!("{}", t);
        assert_eq!(t_str, "Structure(TEST)")
    }
}
