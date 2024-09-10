use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "bid-ask")]
#[derive(Serialize, Deserialize)]
pub struct BidAskSbModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
}

impl service_sdk::rust_extensions::sorted_vec::EntityWithStrKey for BidAskSbModel {
    fn get_key(&self) -> &str {
        self.id.as_str()
    }
}
