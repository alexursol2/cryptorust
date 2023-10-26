multiversx_sc::imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait ZombieHelper: storage::Storage {
    fn check_above_level(&self, level: u16, zombie_id: usize) {
        let my_zombie = self.zombies(&zombie_id).get();
        require!(my_zombie.level >= level, "Zombie is too low level");
    }

    #[endpoint]
    fn change_name(&self, zombie_id: usize, name: ManagedBuffer) {
        self.check_above_level(2u16, zombie_id);
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.zombie_owner(&zombie_id).get(),
            "Only the owner of the zombie can perform this operation"
        );
        self.zombies(&zombie_id)
            .update(|my_zombie| my_zombie.name = name);
    }

    #[endpoint]
    fn change_dna(&self, zombie_id: usize, dna: u64) {
        self.check_above_level(20u16, zombie_id);
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.zombie_owner(&zombie_id).get(),
            "Only the owner of the zombie can perform this operation"
        );
        self.zombies(&zombie_id)
            .update(|my_zombie| my_zombie.dna = dna);
    }
}