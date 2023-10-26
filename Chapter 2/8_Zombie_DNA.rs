multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage, zombie_factory};

#[multiversx_sc::module]
pub trait ZombieFeeding:
    storage::Storage
    + zombie_factory::ZombieFactory
    // import storages and zombiefactory and to make ZombieFeeding a supertrait by adding Storages and ZombieFactory to its definition.
{
    #[endpoint]
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64) {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.zombie_owner(&zombie_id).get(), // check address for existing
            "Only the owner of the zombie can perform this operation"
        );
        let my_zombie = self.zombies(&zombie_id).get(); 
        // declare a local Zombie named my_zombie (which will take the zombie with that certain id from the storage zombie)
    }
}
