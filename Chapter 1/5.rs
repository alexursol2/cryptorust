#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Zombie<M: ManagedTypeApi> {
  name: ManagedBuffer<M>,
  dna: u64,
}

#[multiversx_sc::contract]
pub trait ZombiesContract {

  #[init]
  fn init(&self) {
    self.dna_digits().set(16u8);
    self.zombies_count().set(1usize);
    // unindexed SingleValueMapper storage named zombies_count and set it up with 1usize the init
  }

  // start here

  #[storage_mapper("dna_digits")]
  fn dna_digits(&self) -> SingleValueMapper<u8>;

  #[storage_mapper("zombies_count")]
  fn zombies_count(&self) -> SingleValueMapper<usize>;
  // Storage mappers can also be unindexed when the data we want to store consist of a single element

  #[storage_mapper("zombies")]
  fn zombies(&self, id: usize) -> SingleValueMapper<Zombie<Self::Api>>;
  // Remember we mentioned the contract is already aware of managed types? 
  // It can be accessed wth: Self::Api in stead of M: ManagedTypeApi.
}
