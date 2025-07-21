use solana_client::rpc_client::RpcClient;
use solana_sdk::clock::Slot;
use solana_transaction_status::EncodedTransaction;

pub async fn get_block(slot: Slot) {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    
    match rpc_client.get_block(slot) {
        Ok(block) => {
            println!("Block Information for Slot {}:", slot);
            println!("Block Height: {:?}", block.block_height);
            println!("Block Time: {:?}", block.block_time);
            println!("Blockhash: {}", block.blockhash);
            println!("Parent Slot: {}", block.parent_slot);
            println!("Number of Transactions: {}", block.transactions.len());
            
            // Display first few transaction signatures
            println!("\nFirst 5 Transaction Signatures:");
            for (i, tx) in block.transactions.iter().take(5).enumerate() {
                match &tx.transaction {
                    EncodedTransaction::Json(ui_tx) => {
                        println!("  {}: {:?}", i + 1, ui_tx.signatures);
                    }
                    _ => {
                        println!("  {}: Transaction encoding not supported", i + 1);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching block: {}", e);
        }
    }
}
