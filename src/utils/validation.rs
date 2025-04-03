use solana_sdk::pubkey::Pubkey;
use log::error;

// validate if the wallet address is a valid pubic key
pub fn is_valid_pubkey(address: &str) -> bool {
  if let Ok(_) =address.parse::<Pubkey>() {
    true
  } else {
    error!("Invalid solana wallet address: {}", address);
    false
  }
}