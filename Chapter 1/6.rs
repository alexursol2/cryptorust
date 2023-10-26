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
        // This line is updating the zombies_count in the contract's storage. It uses the update method on the zombies_count 
        // storage mapper. Inside the closure, it increments the value pointed to by id (which is a reference to the current count of
        // zombies). This indicates that a new zombie is being created, so the count needs to be incremented
        self.zombies(id).set(Zombie { name, dna });
        // This line is creating a new Zombie object and storing it in the contract's storage. It uses the set method on the zombies 
        // storage mapper, where id is used as the key
        *id +=1;
        // when you pass a variable as a reference, you need to dereference it to modify the actual value it points to
      });
    }

    // start here

    #[storage_mapper("dna_digits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("zombies_count")]
    fn zombies_count(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("zombies")]
    fn zombies(&self, id: &usize) -> SingleValueMapper<Zombie<Self::Api>>;
  }
