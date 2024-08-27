#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextSbModel {
    #[prost(string, tag = "1")]
    pub key: String,
    #[prost(string, tag = "2")]
    pub value: String,
}
