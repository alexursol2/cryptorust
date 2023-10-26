#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)] // procedural macro
pub struct Zombie<M: ManagedTypeApi> { //This generic argument M is telling us that it is of ManagedTypeApi type, 
//a custom type defined in the MultiversX framework used in declaring the data types that don't use memory allocation such as 
//ManagedBuffer. Every time a structure, trait or enum contains a datatype such as ManagedBuffer or any other managed type, the generic 
//with restriction <M: ManagedTypeApi> needs to be added to the declaration of it, and the generic <M> to the data type itself
  name: ManagedBuffer<M>,
  dna: u64,
}

#[multiversx_sc::contract]
pub trait ZombiesContract {

  #[init]
  fn init(&self) {
    self.dna_digits().set(16u8);
  }

  #[storage_mapper("dna_digits")]
  fn dna_digits(&self) -> SingleValueMapper<u8>;

  // start here
}
