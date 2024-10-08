use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "audit-log")]
#[derive(Serialize, Deserialize)]
pub struct AuditLogSbContract {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(int64, tag = "2")]
    pub moment: i64,
    #[prost(string, tag = "3")]
    pub who: String,
    #[prost(string, tag = "4")]
    pub message: String,
    #[prost(string, tag = "6")]
    pub tech_data: String,
}
