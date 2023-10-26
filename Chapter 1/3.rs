#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// start here

#[multiversx_sc::contract]
pub trait ZombiesContract {

  #[init]
  fn init(&self) {
    self.dna_digits().set(16u8); // and than set the value of it to 16
  }

  #[storage_mapper("dna_digits")] // which is something we call a procedural macro. 
  // At compile time, this line adds a body to the function that defines its behaviour as a storage mapper. 
  fn dna_digits(&self) -> SingleValueMapper<u8>; 
  // we created a storage for a u8 named dna_digits 
}
