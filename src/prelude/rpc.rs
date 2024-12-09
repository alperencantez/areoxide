use serde_json::Value;
use std::error::Error;
use reqwest::Client;

use super::types::{AreonBlock, RpcRequest, RpcResponse};

pub struct RpcClient {
    rpc_url: String,
    client: Client,
}

impl RpcClient {
    pub fn init(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_block_number(&self) -> Result<u64, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_blockNumber",
            params: vec![],
            id: 1,
        };

        let response: RpcResponse<String> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?
            .json::<RpcResponse<String>>()
            .await?;

        let block_number = u64::from_str_radix(response.result.trim_start_matches("0x"), 16)?;
        Ok(block_number)
    }

    pub async fn get_balance(&self, address: String) -> Result<String, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_getBalance",
            params: vec![Value::String(address), Value::String("latest".to_string())],
            id: 1,
        };
    
        let response: RpcResponse<String> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?
            .json::<RpcResponse<String>>()
            .await?;

        Ok(response.result)
    }


    pub async fn chain_id(&self) -> Result<String, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_chainId",
            params: vec![],
            id: 1,
        };
    
        let response: RpcResponse<String> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?
            .json::<RpcResponse<String>>()
            .await?;

        Ok(response.result)
    }

    pub async fn gas_price(&self) -> Result<String, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_gasPrice",
            params: vec![],
            id: 1,
        };
    
        let response: RpcResponse<String> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?
            .json::<RpcResponse<String>>()
            .await?;

        Ok(response.result)
    }

    pub async fn get_block_by_number(&self, block_number: String, show_txs_in_detail: bool) -> Result<AreonBlock, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_getBlockByNumber",
            params: vec![Value::String(block_number), Value::Bool(show_txs_in_detail)],
            id: 1,
        };
    
        let response: RpcResponse<AreonBlock> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?
            .json::<RpcResponse<AreonBlock>>()
            .await?;

        Ok(response.result)
    }

    pub async fn get_block_by_hash(&self, block_hash: String, show_txs_in_detail: bool) -> Result<AreonBlock, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_getBlockByHash",
            params: vec![Value::String(block_hash), Value::Bool(show_txs_in_detail)],
            id: 1,
        };
    
        let response: RpcResponse<AreonBlock> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?
            .json::<RpcResponse<AreonBlock>>()
            .await?;

        Ok(response.result)
    }


}