//cli module

pub mod track;

use clap::{Parser, Subcommand};
use crate::cli::track_wallet;

#[derive(Parser)]
#[command(name = "BlockHawk")]

pub struct Cli {
  #[derive(Subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Track {address: String},
}

impl Cli {
  pub fn run(&self) {
    match &self.command {
      Commands::Track{address} => track_wallet(address.clone())
    }
  }
}