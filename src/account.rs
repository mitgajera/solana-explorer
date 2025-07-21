use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn get_account(address: &str) {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    
    match Pubkey::from_str(address) {
        Ok(pubkey) => {
            println!("Account Information:");
            println!("Address: {}", address);
            println!("Valid Solana address format: ✓");
            
            match rpc_client.get_account(&pubkey) {
                Ok(account) => {
                    println!("Account Status: ACTIVE");
                    println!("Balance: {} lamports ({:.9} SOL)", 
                        account.lamports, 
                        account.lamports as f64 / 1_000_000_000.0
                    );
                    println!("Owner: {}", account.owner);
                    println!("Executable: {}", account.executable);
                    println!("Rent Epoch: {}", account.rent_epoch);
                    println!("Data Length: {} bytes", account.data.len());
                    
                    // Show first few bytes of data if available
                    if !account.data.is_empty() {
                        println!("First 32 bytes of data: {:?}", 
                            &account.data[..std::cmp::min(32, account.data.len())]);
                    }
                }
                Err(e) => {
                    println!("Account Status: NOT FOUND ON-CHAIN");
                    println!("Reason: {}", e);
                    println!("\nThis could mean:");
                    println!("  • The address has never received any SOL");
                    println!("  • The address has never been used in any transaction");
                    println!("  • The account was closed and its SOL returned");
                    println!("\nThe address is valid but inactive. To activate it:");
                    println!("  • Send some SOL to this address");
                    println!("  • Use it in a transaction");
                    
                    // Check if it's a Program Derived Address (PDA)
                    if is_likely_pda(&pubkey) {
                        println!("\nNote: This appears to be a Program Derived Address (PDA)");
                        println!("PDAs are deterministically generated and may not be funded yet.");
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Invalid address format: {}", e);
            println!("Solana addresses should be base58 encoded and 32 bytes long");
        }
    }
}

// Helper function to detect if an address might be a PDA
fn is_likely_pda(pubkey: &Pubkey) -> bool {
    // This is a simple heuristic - real PDA detection would require knowing the program and seeds
    // PDAs often have specific patterns or are generated deterministically
    let bytes = pubkey.to_bytes();
    
    // Check if it's on the ed25519 curve (PDAs are not)
    // This is a simplified check
    bytes[31] & 0x80 == 0
}
