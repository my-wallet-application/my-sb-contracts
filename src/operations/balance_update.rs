use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

use crate::{ContextSbModel, ExecuteCryptoDepositSbModel};

pub const TYPE_CTX_KEY: &str = "type";
pub const CRYPTO_DEPOSIT_TYPE_CTX_VALUE: &str = "crypto-deposit";
pub const TX_ID_CTX_KEY: &str = "tx_id";
pub const CRYPTO_DEPOSIT_ADDRESS_KEY: &str = "deposit_address";
pub const CRYPTO_DEPOSIT_ADDRESS_MEMO_KEY: &str = "deposit_address_memo";

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "balance-update")]
pub struct BalanceUpdateOperationSbModel {
    #[prost(string, tag = "1")]
    pub operation_id: String,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(sint64, tag = "3")]
    pub timestamp: i64,
    #[prost(string, tag = "4")]
    pub who: String,
    #[prost(message, repeated, tag = "5")]
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

impl BalanceUpdateOperationSbModel {
    pub fn fill_ctx_as_crypto_deposit_operation(
        &mut self,
        model: &ExecuteCryptoDepositSbModel,
        balance: f64,
    ) {
        let address = model.deposit_address.as_ref().unwrap();
        self.updates.push(BalanceUpdateSbModel {
            wallet: model.asset_id.to_string(),
            balance,
            delta: model.amount,
            client_id: model.client_id.to_string(),
            ctx: vec![
                ContextSbModel {
                    key: TYPE_CTX_KEY.to_string(),
                    value: CRYPTO_DEPOSIT_TYPE_CTX_VALUE.to_string(),
                },
                ContextSbModel {
                    key: TX_ID_CTX_KEY.to_string(),
                    value: model.blockchain_transaction_id.to_string(),
                },
                ContextSbModel {
                    key: CRYPTO_DEPOSIT_ADDRESS_KEY.to_string(),
                    value: address.address.to_string(),
                },
                ContextSbModel {
                    key: CRYPTO_DEPOSIT_ADDRESS_MEMO_KEY.to_string(),
                    value: address.memo.to_string(),
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

        let mut crypto_deposit_address = None;
        let mut crypto_deposit_address_memo = None;
        for ctx in &first_el.ctx {
            if ctx.key == TYPE_CTX_KEY && ctx.value == CRYPTO_DEPOSIT_TYPE_CTX_VALUE {
                is_crypto_deposit = true;
            }

            if ctx.key == TX_ID_CTX_KEY {
                tx_id = Some(ctx.value.as_str());
            }

            if ctx.key == CRYPTO_DEPOSIT_ADDRESS_KEY {
                crypto_deposit_address = Some(ctx.value.as_str());
            }

            if ctx.key == CRYPTO_DEPOSIT_ADDRESS_MEMO_KEY {
                crypto_deposit_address_memo = Some(ctx.value.as_str());
            }
        }

        if is_crypto_deposit {
            if let Some(tx_id) = tx_id {
                if crypto_deposit_address.is_none() {
                    panic!("Crypto deposit address is missing {:?}", self);
                }

                if crypto_deposit_address_memo.is_none() {
                    panic!("Crypto deposit address memo is missing {:?}", self);
                }
                return CryptoDepositInfo {
                    client_id: &first_el.client_id,
                    delta: first_el.delta,
                    balance: first_el.balance,
                    tx_id,
                    wallet_id: &first_el.wallet,
                    crypto_deposit_address: crypto_deposit_address.unwrap(),
                    crypto_deposit_address_memo: crypto_deposit_address_memo.unwrap(),
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
    pub crypto_deposit_address: &'s str,
    pub crypto_deposit_address_memo: &'s str,
}
