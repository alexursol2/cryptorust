multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage, zombie_factory};

#[multiversx_sc::module]
pub trait ZombieFeeding:
    storage::Storage
    + zombie_factory::ZombieFactory
{
    #[endpoint]
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64) {
      let caller = self.blockchain().get_caller();
      require!(
          caller == self.zombie_owner(&zombie_id).get(),
          "Only the owner of the zombie can perform this operation"
      );
      let my_zombie = self.zombies(&zombie_id).get();
      let dna_digits = self.dna_digits().get();
      let max_dna_value = u64::pow(10u64, dna_digits as u32); // take the parameter from another file
      let verified_target_dna = target_dna % max_dna_value;
      let new_dna = (my_zombie.dna + verified_target_dna) / 2;
      self.create_zombie(caller, ManagedBuffer::from(b"NoName"), new_dna);
    }
}
