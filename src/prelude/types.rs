use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct RpcRequest<'a> {
    pub jsonrpc: &'a str,
    pub method: &'a str,
    pub params: Vec<Value>,
    pub id: u64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: i64,
    pub result: T,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreonBlock {
    pub base_fee_per_gas: String,
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub hash: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: String,
    pub parent_hash: String,
    pub receipts_root: String,
    #[serde(rename = "sha3Uncles")]
    pub sha3uncles: String,
    pub size: String,
    pub state_root: String,
    pub timestamp: String,
    pub total_difficulty: String,
    pub transactions: Vec<Value>,
    pub transactions_root: String,
    pub uncles: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreonTx {
    pub block_hash: String,
    pub block_number: String,
    pub from: String,
    pub gas: String,
    pub gas_price: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: String,
    pub transaction_index: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub chain_id: String,
    pub v: String,
    pub r: String,
    pub s: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreonTxReceipt {
    pub block_hash: String,
    pub block_number: String,
    pub contract_address: Value,
    pub cumulative_gas_used: String,
    pub from: String,
    pub gas_used: String,
    pub logs: Vec<Value>,
    pub logs_bloom: String,
    pub status: String,
    pub to: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
