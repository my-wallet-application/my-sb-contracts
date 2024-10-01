use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

use crate::ContextSbModel;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "balance-update")]
pub struct BalanceUpdateNotificationSbModel {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(sint64, tag = "2")]
    pub timestamp: i64,
    #[prost(message, repeated, tag = "3")]
    pub updates: Vec<BalanceUpdateSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateSbModel {
    #[prost(string, tag = "1")]
    pub wallet: String,
    #[prost(double, tag = "2")]
    pub balance: f64,
    #[prost(string, tag = "3")]
    pub client_id: String,
    #[prost(message, repeated, tag = "4")]
    pub ctx: Vec<ContextSbModel>,
}
