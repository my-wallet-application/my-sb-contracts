use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "bid-ask")]
#[derive(Serialize, Deserialize)]
pub struct BidAskSbModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub src_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(double, tag = "4")]
    pub bid: f64,
    #[prost(double, tag = "5")]
    pub ask: f64,
}

impl service_sdk::rust_extensions::sorted_vec::EntityWithStrKey for BidAskSbModel {
    fn get_key(&self) -> &str {
        self.id.as_str()
    }
}
