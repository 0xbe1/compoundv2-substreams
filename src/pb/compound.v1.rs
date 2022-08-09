// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccrueInterestList {
    #[prost(message, repeated, tag="1")]
    pub accrue_interest_list: ::prost::alloc::vec::Vec<AccrueInterest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccrueInterest {
    #[prost(string, tag="1")]
    pub interest_accumulated: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrow_index: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub total_borrows: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="99")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="98")]
    pub block_number: u64,
    #[prost(int64, tag="97")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintList {
    #[prost(message, repeated, tag="1")]
    pub mint_list: ::prost::alloc::vec::Vec<Mint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mint {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub minter: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub mint_amount: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub mint_tokens: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub mint_amount_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="99")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketListed {
    #[prost(bytes="vec", tag="1")]
    pub ctoken: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketListedList {
    #[prost(message, repeated, tag="1")]
    pub market_listed_list: ::prost::alloc::vec::Vec<MarketListed>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketTvl {
    #[prost(bytes="vec", tag="1")]
    pub market: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub tvl: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketTvlList {
    #[prost(message, repeated, tag="1")]
    pub market_tvl_list: ::prost::alloc::vec::Vec<MarketTvl>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketRevenueDelta {
    #[prost(bytes="vec", tag="1")]
    pub market: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub total_revenue: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub protocol_revenue: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub supply_revenue: ::prost::alloc::string::String,
    #[prost(int64, tag="99")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketRevenueDeltaList {
    #[prost(message, repeated, tag="1")]
    pub market_revenue_delta_list: ::prost::alloc::vec::Vec<MarketRevenueDelta>,
}
/// Encoded file descriptor set for the `compound.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb8, 0x19, 0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x2e, 0x76, 0x31,
    0x22, 0x63, 0x0a, 0x12, 0x41, 0x63, 0x63, 0x72, 0x75, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x65,
    0x73, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x4d, 0x0a, 0x14, 0x61, 0x63, 0x63, 0x72, 0x75, 0x65,
    0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73, 0x74, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x2e,
    0x76, 0x31, 0x2e, 0x41, 0x63, 0x63, 0x72, 0x75, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73,
    0x74, 0x52, 0x12, 0x61, 0x63, 0x63, 0x72, 0x75, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73,
    0x74, 0x4c, 0x69, 0x73, 0x74, 0x22, 0xe6, 0x01, 0x0a, 0x0e, 0x41, 0x63, 0x63, 0x72, 0x75, 0x65,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73, 0x74, 0x12, 0x31, 0x0a, 0x14, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x65, 0x73, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x13, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73, 0x74,
    0x41, 0x63, 0x63, 0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x62,
    0x6f, 0x72, 0x72, 0x6f, 0x77, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0b, 0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x23,
    0x0a, 0x0d, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x73, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x42, 0x6f, 0x72, 0x72,
    0x6f, 0x77, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x63,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x21, 0x0a,
    0x0c, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x62, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x61, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x5f,
    0x0a, 0x05, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x73,
    0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x79, 0x6d,
    0x62, 0x6f, 0x6c, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x73, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x73, 0x22,
    0x3a, 0x0a, 0x08, 0x4d, 0x69, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x09, 0x6d,
    0x69, 0x6e, 0x74, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x69, 0x6e,
    0x74, 0x52, 0x08, 0x6d, 0x69, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x22, 0xb6, 0x01, 0x0a, 0x04,
    0x4d, 0x69, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x02, 0x69, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x6d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x1f, 0x0a, 0x0b,
    0x6d, 0x69, 0x6e, 0x74, 0x5f, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0a, 0x6d, 0x69, 0x6e, 0x74, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1f, 0x0a,
    0x0b, 0x6d, 0x69, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0a, 0x6d, 0x69, 0x6e, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x12, 0x26,
    0x0a, 0x0f, 0x6d, 0x69, 0x6e, 0x74, 0x5f, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x75, 0x73,
    0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x6d, 0x69, 0x6e, 0x74, 0x41, 0x6d, 0x6f,
    0x75, 0x6e, 0x74, 0x55, 0x73, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x18, 0x63, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x22, 0x26, 0x0a, 0x0c, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x4c, 0x69,
    0x73, 0x74, 0x65, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x63, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x63, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x5b, 0x0a, 0x10,
    0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x64, 0x4c, 0x69, 0x73, 0x74,
    0x12, 0x47, 0x0a, 0x12, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65,
    0x64, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x61, 0x72, 0x6b, 0x65,
    0x74, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x64, 0x52, 0x10, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x4c,
    0x69, 0x73, 0x74, 0x65, 0x64, 0x4c, 0x69, 0x73, 0x74, 0x22, 0x35, 0x0a, 0x09, 0x4d, 0x61, 0x72,
    0x6b, 0x65, 0x74, 0x54, 0x76, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x12, 0x10,
    0x0a, 0x03, 0x74, 0x76, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x74, 0x76, 0x6c,
    0x22, 0x4f, 0x0a, 0x0d, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x54, 0x76, 0x6c, 0x4c, 0x69, 0x73,
    0x74, 0x12, 0x3e, 0x0a, 0x0f, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x5f, 0x74, 0x76, 0x6c, 0x5f,
    0x6c, 0x69, 0x73, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x75, 0x6e, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x54,
    0x76, 0x6c, 0x52, 0x0d, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x54, 0x76, 0x6c, 0x4c, 0x69, 0x73,
    0x74, 0x22, 0xc1, 0x01, 0x0a, 0x12, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x76, 0x65,
    0x6e, 0x75, 0x65, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x72, 0x6b,
    0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74,
    0x12, 0x23, 0x0a, 0x0d, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x75,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x52, 0x65,
    0x76, 0x65, 0x6e, 0x75, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f,
    0x6c, 0x5f, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52, 0x65, 0x76, 0x65, 0x6e, 0x75, 0x65,
    0x12, 0x25, 0x0a, 0x0e, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x5f, 0x72, 0x65, 0x76, 0x65, 0x6e,
    0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79,
    0x52, 0x65, 0x76, 0x65, 0x6e, 0x75, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x18, 0x63, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x74, 0x0a, 0x16, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x52,
    0x65, 0x76, 0x65, 0x6e, 0x75, 0x65, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x4c, 0x69, 0x73, 0x74, 0x12,
    0x5a, 0x0a, 0x19, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x75,
    0x65, 0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x2e, 0x76, 0x31,
    0x2e, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x76, 0x65, 0x6e, 0x75, 0x65, 0x44, 0x65,
    0x6c, 0x74, 0x61, 0x52, 0x16, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x76, 0x65, 0x6e,
    0x75, 0x65, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x4c, 0x69, 0x73, 0x74, 0x4a, 0xa5, 0x0f, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x40, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04,
    0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x05, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x1a, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x05, 0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08,
    0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x16, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x09, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x09, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x0a, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x09, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x18, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0b, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x0b, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x12, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x0d, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d,
    0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0e, 0x14, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x11, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x11, 0x08,
    0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x13, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x13, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x09,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x10, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x14, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x14, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x15, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x15, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x15, 0x09, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x15, 0x14, 0x15, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x18, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x18, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x19,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x1c, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1c,
    0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x1e, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x1e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x11, 0x12,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x02, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x1f, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12,
    0x03, 0x20, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x20,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x20, 0x09, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x20, 0x17, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x21, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x21, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x21, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x21, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x22,
    0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x22, 0x02, 0x07,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x22, 0x08, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x22, 0x14, 0x16, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x25, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x25, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x26, 0x02,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x26, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x29, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x29, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x02, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2a, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x2d, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04,
    0x2d, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x02, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x2e, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x03, 0x2f, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x09, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x0f, 0x10, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x32, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x32, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03,
    0x33, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x33, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x33, 0x0b, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x15, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x04, 0x36, 0x00, 0x3c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03,
    0x36, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x37, 0x02, 0x07, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x01, 0x12, 0x03, 0x38, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x38, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x38, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x38, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x39, 0x02, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x39, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x39, 0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x39, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03,
    0x12, 0x03, 0x3a, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x3a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3a, 0x09,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3a, 0x1a, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04, 0x12, 0x03, 0x3b, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x04, 0x05, 0x12, 0x03, 0x3b, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x3b, 0x14, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x3e, 0x00,
    0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x3f, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x3f, 0x1e, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x3f, 0x3a, 0x3b, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)