use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

use crate::ContextSbModel;

pub const TYPE_CTX_KEY: &str = "type";
pub const CRYPTO_DEPOSIT_TYPE_CTX_VALUE: &str = "crypto-deposit";
pub const TX_ID_CTX_KEY: &str = "tx_id";

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "balance-update")]
pub struct BalanceUpdateNotificationSbModel {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(sint64, tag = "2")]
    pub timestamp: i64,
    #[prost(string, tag = "3")]
    pub who: String,
    #[prost(message, repeated, tag = "4")]
    pub updates: Vec<BalanceUpdateSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateSbModel {
    #[prost(string, tag = "1")]
    pub wallet: String,
    #[prost(double, tag = "2")]
    pub delta: f64,
    #[prost(double, tag = "3")]
    pub balance: f64,
    #[prost(string, tag = "4")]
    pub client_id: String,
    #[prost(message, repeated, tag = "5")]
    pub ctx: Vec<ContextSbModel>,
}

impl BalanceUpdateNotificationSbModel {
    pub fn fill_ctx_as_crypto_deposit_operation(
        &mut self,
        client_id: String,
        wallet_id: String,
        delta: f64,
        balance: f64,
        blockchain_transaction_id: String,
    ) {
        self.updates.push(BalanceUpdateSbModel {
            wallet: wallet_id,
            balance,
            delta,
            client_id,
            ctx: vec![
                ContextSbModel {
                    key: TYPE_CTX_KEY.to_string(),
                    value: CRYPTO_DEPOSIT_TYPE_CTX_VALUE.to_string(),
                },
                ContextSbModel {
                    key: TX_ID_CTX_KEY.to_string(),
                    value: blockchain_transaction_id,
                },
            ],
        });
    }

    pub fn is_crypto_deposit(&self) -> Option<CryptoDepositInfo> {
        if self.updates.len() != 1 {
            return None;
        }

        let first_el = &self.updates.get(0).unwrap();
        let mut is_crypto_deposit = false;
        let mut tx_id = None;
        for ctx in &first_el.ctx {
            if ctx.key == TYPE_CTX_KEY && ctx.value == CRYPTO_DEPOSIT_TYPE_CTX_VALUE {
                is_crypto_deposit = true;
            }

            if ctx.key == TX_ID_CTX_KEY {
                tx_id = Some(ctx.value.as_str());
            }
        }

        if is_crypto_deposit {
            if let Some(tx_id) = tx_id {
                return CryptoDepositInfo {
                    client_id: &first_el.client_id,
                    delta: first_el.delta,
                    balance: first_el.balance,
                    tx_id,
                    wallet_id: &first_el.wallet,
                }
                .into();
            }
        }

        None
    }
}

pub struct CryptoDepositInfo<'s> {
    pub client_id: &'s str,
    pub delta: f64,
    pub balance: f64,
    pub tx_id: &'s str,
    pub wallet_id: &'s str,
}
