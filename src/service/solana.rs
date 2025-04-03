//interacts with solana rpc


use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use chrono::{DateTime, Utc};
use reqwest;
use serde_json::Value;
use crate::utils::formatter::format_usd;

//Define struct for interacting with solana
pub struct SolanaClient {
  rpc: RpcClient, //Holds rpc connection
}

async fn get_sol_price() -> Result<f64, Box<dyn std::error::Error>> {
  let url = "https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd";
  let resp: Value = reqwest::get(url).await?.json().await?;
  // let price = resp["solana"]["usd"].as_f64().unwrap_or(0.0);

  match resp["solana"]["usd"].as_f64() {
    Some(price) => Ok(price),
    None => Err("Failed to fetch SOL price".into())
  }
 
}

impl SolanaClient {
  // constructor function to create client 
  pub fn new(rpc_url: &str) -> Self {
    SolanaClient {
      rpc: RpcClient::new(rpc_url.to_string()),
    }
  }
  // function to get wallet balance
  pub async fn get_balance (&self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pubkey = address.parse::<Pubkey>()?;
    let balance = self.rpc.get_balance(&pubkey)? as f64 / 1_000_000_000.0;  //query balance from solana and converts lampots to sol
    
    //fetch sol price
    let sol_price= get_sol_price().await?;
    let balance_usd = balance * sol_price;

    println!(
      "\n💰 Balance: {:.3} SOL {}",
      balance,
      format_usd(balance_usd) 
  );
    Ok(())

  }



  //function to fetch the last limit transaction for a wallet
  pub fn get_recent_transactions(&self, wallet_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pubkey = wallet_address.parse::<Pubkey>()?;

    let config = GetConfirmedSignaturesForAddress2Config {
      before: None, 
      until: None,
      limit: Some(10),
      commitment: None,
    };

    let signatures = self.rpc.get_signatures_for_address_with_config(&pubkey, config)?;

    if signatures.is_empty() {
      println!("No transactions found in this wallet.");
    return Ok(())
    }

    println!("\n RecENT transactions for {}:", wallet_address);

    for sig in signatures {
    let _time_stamp = match sig.block_time {
      Some(time) => {
        let date_time = DateTime::<Utc>::from_timestamp(time, 0)
        .expect("Valid timestamp");
        date_time.format("%Y-%m-%d %H:%M:%S UTC").to_string()
      }
      None =>"Unknown TimeStamp".to_string()
    };

    println!("📌 Signature: {}\n ⏰ Timestamp: {}\n ✅ Status: {:?}\n", sig.signature, _time_stamp, sig.confirmation_status)
    }
    Ok(())
 }
}

