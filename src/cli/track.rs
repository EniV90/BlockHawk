// track wallet command

use crate::models::wallet::Wallet;

pub fn track_wallet(address: String) {
  match Wallet::new(address) {
    Ok(wallet) => wallet.track(),
    Err(err) => eprintln!("Error: {}" err)
  }
}