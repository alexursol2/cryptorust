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
    // he function declaration contains the type of the return value (in this case u64) indicated by ->
  }

  // start here

  #[storage_mapper("dna_digits")]
  fn dna_digits(&self) -> SingleValueMapper<u8>;

  #[storage_mapper("zombies_count")]
  fn zombies_count(&self) -> SingleValueMapper<usize>;

  #[view] // function, meaning it's only viewing the data but not modifying it
  #[storage_mapper("zombies")]
  fn zombies(&self, id: &usize) -> SingleValueMapper<Zombie<Self::Api>>;
}
