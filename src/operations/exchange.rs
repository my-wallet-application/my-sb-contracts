use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "exchange-operation")]
pub struct ExchangeOperationSbModel {
    #[prost(string, tag = "1")]
    pub operation_id: String,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(sint64, tag = "3")]
    pub timestamp: i64,
    #[prost(string, tag = "4")]
    pub who: String,
    #[prost(message, tag = "5")]
    pub sell_wallet: Option<ExchangeWalletUpdate>,
    #[prost(message, tag = "6")]
    pub buy_wallet: Option<ExchangeWalletUpdate>,
    #[prost(message, tag = "7")]
    pub commission: Option<ExchangeWalletUpdate>,
}

#[derive(Clone, ::prost::Message)]
pub struct ExchangeWalletUpdate {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(string, tag = "2")]
    pub wallet_id: String,
    #[prost(double, tag = "3")]
    pub delta: f64,
    #[prost(double, tag = "4")]
    pub balance_after_operation: f64,
}
