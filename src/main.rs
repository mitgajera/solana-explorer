mod block;
mod transaction;
mod account;
mod rpc;

use clap::{Parser, Subcommand};
use solana_sdk::{clock::Slot, signature};
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "Solana Explorer")]
#[command(about = "Mini CLI to explore Solana blockchain", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Block {
        #[arg(long)]
        slot: u64
    },

    Tx {
        #[arg(long)]
        signature: String,
    },

    Addr {
        #[arg(long)]
        address: String,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Block { slot } => {
            let slot: Slot = slot.into();
            block::get_block(slot).await;
        },
        Commands::Tx { signature } => {
            let sig = signature::Signature::from_str(&signature).unwrap();
            transaction::get_transaction(&sig).await;
        },
        Commands::Addr { address } => {
            account::get_account(&address).await;
        }
    }
}