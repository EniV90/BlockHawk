use std::io;
mod service;
mod utils;
mod models;
use log::error;
use service::solana::SolanaClient;
use utils::{logger, validation};
use tokio;

#[tokio::main]
async fn main() {
    //initialize logger
    logger::init_logger();
   let solana = SolanaClient::new("https://api.mainnet-beta.solana.com");

   println!("Enter solana wallet address to check balance:");

   let mut wallet_address = String::new();
   io::stdin().read_line(&mut wallet_address).expect("Failed to read input");
   let wallet_address = wallet_address.trim(); 

   //validate wallet address
   if !validation::is_valid_pubkey(wallet_address) {
        return;
   }

   log::info! ("🕵🏽‍♂️ Checking balance for {}", wallet_address);
   if let Err(e) = solana.get_balance(wallet_address).await {
    error!("❌ Error fatching balance: {}", e)
   }

   
   log::info! ("🕵🏽‍♂️ Fetching recent transactions... {}", wallet_address);
   if let Err(e) = solana.get_recent_transactions(wallet_address) {
    error!("❌ Error fatching transactions: {}", e)
   }
   }




