use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_transaction_status::UiTransactionEncoding;

pub async fn get_transaction(signature: &Signature) {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: None,
        max_supported_transaction_version: Some(0),
    };
    
    match rpc_client.get_transaction_with_config(signature, config) {
        Ok(confirmed_tx) => {
            println!("Transaction Information:");
            println!("Signature: {}", signature);
            println!("Slot: {:?}", confirmed_tx.slot);
            println!("Block Time: {:?}", confirmed_tx.block_time);
            
            if let Some(meta) = &confirmed_tx.transaction.meta {
                println!("Fee: {} lamports", meta.fee);
                println!("Status: {:?}", meta.status);
                
                // Handle log messages properly
                println!("\nLog Messages:");
                match &meta.log_messages {
                    solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => {
                        for (i, log) in logs.iter().enumerate() {
                            println!("  {}: {}", i + 1, log);
                        }
                    }
                    _ => println!("  No log messages available"),
                }
            }
            
            println!("\nTransaction processed successfully");
        }
        Err(e) => {
            eprintln!("Error fetching transaction: {}", e);
        }
    }
}
