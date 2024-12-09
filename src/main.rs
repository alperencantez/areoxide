use areoxide::prelude::{rpc::RpcClient, util};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    test_drive().await;
}

async fn test_drive() {
    const RPC_URL: &str = "https://mainnet-rpc.areon.network";
    let client: RpcClient = RpcClient::init(RPC_URL);

    match client
        .get_balance("0xdd0446989c851A76Bf03e10a04eae1488e58a0D9".to_string())
        .await
    {
        Ok(balance) => {
            println!("Balance: {:#?}", util::hexadecimal_str_to_decimal_str(balance).unwrap())
        }
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }

    match client.get_block_number().await {
        Ok(block_number) => println!("Last Block Number: {:#?}", block_number),
        Err(e) => eprintln!("Error fetching last block number: {}", e),
    }

    match client.get_block_by_number("8420156".to_string(), false).await {
        Ok(block_number) => println!("Custom Block Number: {:#?}", block_number),
        Err(e) => eprintln!("Error fetching block number: {}", e),
    }

    println!("----- The same block but retrieved with hash -----");

    match client
        .get_block_by_hash(
            "0x67a384763b3b986363694c48b710c4a61d92e684ff65bc39ce3f843cc0ea35f2".to_string(),
            false,
        )
        .await
    {
        Ok(block_number) => println!("Custom Block by Hash: {:#?}", block_number),
        Err(e) => eprintln!("Error fetching block by hash: {}", e),
    }

    match client.chain_id().await {
        Ok(chain_id) => println!(
            "Areon Chain ID: {:#?}",
            util::hexadecimal_str_to_decimal_str(chain_id).unwrap()
        ),
        Err(e) => eprintln!("Error fetching chain id: {}", e),
    }

    match client.gas_price().await {
        Ok(gas) => println!("Gas Price: {:#?}", util::hexadecimal_str_to_decimal_str(gas).unwrap()),
        Err(e) => eprintln!("Error fetching gas price: {}", e),
    }
}
