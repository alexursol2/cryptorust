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

  fn create_zombie(&self, owner: ManagedAddress, name: ManagedBuffer, dna: u64) {
    self.zombies_count().update(|id| {
      self.new_zombie_event(*id, &name, dna);
      self.zombies(id).set(Zombie { name, dna });
      self.owned_zombies(&owner).insert(*id);
      self.zombie_owner(id).set(owner);
      *id += 1;
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
    let caller = self.blockchain().get_caller();
    require!(
    // перевірка чи є адреса
      self.owned_zombies(&caller).is_empty(),
      "You already own a zombie"
    );
    let rand_dna = self.generate_random_dna();
    self.create_zombie(caller, name, rand_dna);
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

  #[storage_mapper("zombie_owner")]
  fn zombie_owner(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

  #[storage_mapper("owned_zombies")]
  fn owned_zombies(&self, owner: &ManagedAddress) -> UnorderedSetMapper<usize>;
}
