use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "exec-crypto-deposit")]
pub struct ExecuteCryptoDepositSbModel {
    #[prost(string, tag = "1")]
    pub operation_id: String,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(string, tag = "3")]
    pub client_id: String,
    #[prost(string, tag = "4")]
    pub asset_id: String,
    #[prost(double, tag = "5")]
    pub amount: f64,
    #[prost(message, tag = "6")]
    pub deposit_address: Option<CryptoDepositAddressGrpcModel>,
    #[prost(string, tag = "7")]
    pub blockchain_transaction_id: String,
    #[prost(sint64, tag = "8")]
    pub timestamp: i64,
    #[prost(string, tag = "9")]
    pub who: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoDepositAddressGrpcModel {
    #[prost(string, tag = "1")]
    pub address: String,
    #[prost(string, tag = "2")]
    pub memo: String,
}
