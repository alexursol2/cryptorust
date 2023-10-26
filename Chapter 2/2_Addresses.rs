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
      self.new_zombie_event(*id, &name, dna);
      self.zombies(id).set(Zombie { name, dna });
      *id +=1;
    });
  }

  #[view]
  fn generate_random_dna(&self) -> u64{
    let mut rand_source = RandomnessSource::new();
    let dna_digits = self.dna_digits().get();
    let max_dna_value = u64::pow(10u64, dna_digits as u32);
    rand_source.next_u64_in_range(0u64, max_dna_value)
  }

  #[endpoint]
  fn create_random_zombie(&self, name: ManagedBuffer){
    let rand_dna = self.generate_random_dna();
    self.create_zombie(name, rand_dna);
  }

  #[event("new_zombie_event")]
  fn new_zombie_event(
    &self,
    #[indexed] zombie_id: usize,
    name: &ManagedBuffer,
    #[indexed] dna: u64,
  );

  #[view]
  #[storage_mapper("dna_digits")]
  fn dna_digits(&self) -> SingleValueMapper<u8>;

  #[view]
  #[storage_mapper("zombies_count")]
  fn zombies_count(&self) -> SingleValueMapper<usize>;

  #[view]
  #[storage_mapper("zombies")]
  fn zombies(&self, id: &usize) -> SingleValueMapper<Zombie<Self::Api>>;

  #[storage_mapper("zombie_owner")] // Create a new indexed SingleValueMapper storage named zombie_owner for a ManagedAddress. 
  // The index named id should be of usize type.
  fn zombie_owner(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

  #[storage_mapper("owned_zombies")] // Create a new indexed UnorderedSetMapper storage named owned_zombies for a usize. 
  // The index named owner should be of &ManagedAddress type.
  fn owned_zombies(&self, owner: &ManagedAddress) -> UnorderedSetMapper<usize>;
}
