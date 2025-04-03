// wallet struct

#[derive(Debug)]


pub struct Wallet {
  pub address: String
}

impl Wallet {
  pub fn new(address:String) -> Result<Self, & 'static str> {
    if address.len() == 44 {
      Ok(Wallet {address})
    } else {
      Err("Invalid Solana address")
    }
  }

  pub fn tract(&self) {
    println!("Tracking wallet: {}", self.address)
  }
}