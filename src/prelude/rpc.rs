use reqwest::Client;
use serde_json::Value;
use std::error::Error;

use super::types::*;

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

    pub async fn get_block_by_number(
        &self,
        block_number: String,
        show_txs_in_detail: bool,
    ) -> Result<AreonBlock, Box<dyn Error>> {
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

    pub async fn get_block_by_hash(
        &self,
        block_hash: String,
        show_txs_in_detail: bool,
    ) -> Result<AreonBlock, Box<dyn Error>> {
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

    pub async fn get_transaction_by_hash(&self, tx_hash: String) -> Result<AreonTx, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_getTransactionByHash",
            params: vec![Value::String(tx_hash)],
            id: 1,
        };

        let response: RpcResponse<AreonTx> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?
            .json::<RpcResponse<AreonTx>>()
            .await?;

        Ok(response.result)
    }

    pub async fn get_transaction_receipt(&self, tx_hash: String) -> Result<AreonTxReceipt, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_getTransactionReceipt",
            params: vec![Value::String(tx_hash)],
            id: 1,
        };

        let response: RpcResponse<AreonTxReceipt> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?
            .json::<RpcResponse<AreonTxReceipt>>()
            .await?;

        Ok(response.result)
    }

    /// Retrieves the bytecode of a smart contract from Areon Network.
    ///
    /// # Parameters
    /// - `address`: The address of the smart contract in hexadecimal format (e.g., `"0x1234..."`).
    /// - `block_tag`: Specifies the block context:
    ///   - `"latest"`: Use the latest block.
    ///   - `"earliest"`: Use the genesis block.
    ///   - `"pending"`: Use the pending state.
    ///   - `"safe"`: Use the safe state.
    ///   - `"finalized"`: Use the finalized state.
    ///   - A block number as a hexadecimal string (e.g., `"0x10"`).
    ///
    /// # Returns
    /// - `Ok(String)`: The bytecode of the contract as a hexadecimal string.
    /// - `Err(String)`: An error message if the request fails or the contract doesn't exist.
    ///
    /// # Example
    /// ```
    /// let client = areoxide::prelude::RpcClient::init("<RPC_URL>");
    /// let address = "0x1234567890abcdef1234567890abcdef12345678";
    /// let block_tag = Some("latest");
    /// match client::get_code(address, block_tag) {
    ///     Ok(bytecode) => println!("Contract bytecode: {}", bytecode),
    ///     Err(e) => eprintln!("Failed to retrieve bytecode: {}", e),
    /// }
    /// ```
    ///
    /// # Notes
    /// - If the address doesn't contain a contract, the function returns an `0x` string.
    /// - Ensure the node you're querying supports the `eth_getCode` RPC method.
    pub async fn get_code(&self, address: String, block: String) -> Result<String, Box<dyn Error>> {
        let request: RpcRequest<'_> = RpcRequest {
            jsonrpc: "2.0",
            method: "eth_getCode",
            params: vec![Value::String(address), Value::String(block)],
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
}
