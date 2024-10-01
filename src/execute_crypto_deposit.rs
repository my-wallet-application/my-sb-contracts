use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "exec-crypto-deposit")]
pub struct ExecuteCryptoDepositSbModel {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(string, tag = "2")]
    pub client_id: String,
    #[prost(string, tag = "3")]
    pub asset_id: String,
    #[prost(string, tag = "4")]
    pub transaction_id: String,
    #[prost(sint64, tag = "5")]
    pub timestamp: i64,
}
