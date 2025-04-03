// token struct

#[derive(Debug)]

pub struct Token {
  pub name: String,
  pub symbol: String,
  pub address: String
  
}

impl Token {
  pub fn new(name: String, symbol: String, address: String) -> Self {
    Token {name, symbol, address}
  }

  pub fn display_info(&self) {
    println!(
      "Token: {} ({}) at {}",
      self.name, self.symbol, self.address
    );
  }
}