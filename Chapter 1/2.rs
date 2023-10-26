#![no_std] // to avoid importing Rust standard library and bloating smart contract size

multiversx_sc::imports!();
multiversx_sc::derive_imports!();
// libraries

#[multiversx_sc::contract] // is a procedural macro that defines the ZombiesContract trait as a contract
pub trait ZombiesContract {

  #[init] // Every contract has a mandatory function init, marked with #[init]
  fn init(&self) {
    
  }

  
}