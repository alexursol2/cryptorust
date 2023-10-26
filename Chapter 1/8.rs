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
  }

  fn create_zombie(&self, name: ManagedBuffer, dna: u64) {
    self.zombies_count().update(|id| {
      self.zombies(id).set(Zombie { name, dna });
      *id +=1;
    });
  }

  #[view]
  fn generate_random_dna(&self) -> u64{
    // start here
  }

  #[endpoint] // offers access for the function to be called by user. 
  // Without this annotation the function can be called only by another function inside the contract.
  fn create_random_zombie(&self, name: ManagedBuffer){
  }

  #[storage_mapper("dna_digits")]
  fn dna_digits(&self) -> SingleValueMapper<u8>;

  #[storage_mapper("zombies_count")]
  fn zombies_count(&self) -> SingleValueMapper<usize>;

  #[view]
  #[storage_mapper("zombies")]
  fn zombies(&self, id: &usize) -> SingleValueMapper<Zombie<Self::Api>>;
}
