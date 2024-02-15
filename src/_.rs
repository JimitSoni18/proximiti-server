#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginPayload {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pwd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginSuccess {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginResult {
    #[prost(oneof = "login_result::Result", tags = "1, 2")]
    pub result: ::core::option::Option<login_result::Result>,
}
/// Nested message and enum types in `LoginResult`.
pub mod login_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "1")]
        Success(super::LoginSuccess),
        #[prost(enumeration = "super::LoginError", tag = "2")]
        Error(i32),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginError {
    InvalidCredentials = 0,
    AccountNotFound = 1,
    AccountDisabled = 2,
}
impl LoginError {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoginError::InvalidCredentials => "INVALID_CREDENTIALS",
            LoginError::AccountNotFound => "ACCOUNT_NOT_FOUND",
            LoginError::AccountDisabled => "ACCOUNT_DISABLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_CREDENTIALS" => Some(Self::InvalidCredentials),
            "ACCOUNT_NOT_FOUND" => Some(Self::AccountNotFound),
            "ACCOUNT_DISABLED" => Some(Self::AccountDisabled),
            _ => None,
        }
    }
}
