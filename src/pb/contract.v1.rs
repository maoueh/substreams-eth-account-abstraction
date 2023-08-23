// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOperations {
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<UserOperation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOperation {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub init_code: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub call_data: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub call_gas_limit: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub verification_gas_limit: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub pre_verification_gas: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub max_fee_per_gas: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub max_priority_fee_per_gas: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub paymaster_and_data: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub signature: ::prost::alloc::string::String,
    /// Extra data
    #[prost(string, tag="12")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="13")]
    pub call_index: u64,
}
// @@protoc_insertion_point(module)
