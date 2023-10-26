multiversx_sc::imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait ZombieHelper: storage::Storage {
    fn check_above_level(&self, level: u16, zombie_id: usize) {
        let my_zombie = self.zombies(&zombie_id).get();
        require!(my_zombie.level >= level, "Zombie is too low level");
    }

    fn check_zombie_belongs_to_caller(&self, zombie_id: usize, caller: &ManagedAddress) {
        require!(
            caller == &self.zombie_owner(&zombie_id).get(),
            "Only the owner of the zombie can perform this operation"
        );
        // The body should contain the require from the code above with caller being received as parameter
    }

    #[endpoint]
    fn change_name(&self, zombie_id: usize, name: ManagedBuffer) {
        // Change the function definition of change_dna and change_name to use this function instead of the straight up call
        self.check_above_level(2u16, zombie_id);
        let caller = self.blockchain().get_caller();
        self.check_zombie_belongs_to_caller(zombie_id, &caller);
        self.zombies(&zombie_id)
            .update(|my_zombie| my_zombie.name = name);
    }

    #[endpoint]
    fn change_dna(&self, zombie_id: usize, dna: u64) {
        self.check_above_level(20u16, zombie_id);
        let caller = self.blockchain().get_caller();
        self.check_zombie_belongs_to_caller(zombie_id, &caller);
        self.zombies(&zombie_id)
            .update(|my_zombie| my_zombie.dna = dna);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn level_up(&self, zombie_id: usize){
        let payment_amount = self.call_value().egld_value();
        let fee = self.level_up_fee().get();
        require!(payment_amount == (&fee).into(), "Payment must be must be 0.001 EGLD");
        self.zombies(&zombie_id).update(|my_zombie| my_zombie.level += 1);
    }

    #[only_owner]
    #[endpoint]
    fn withdraw(&self) {
        let caller_address = self.blockchain().get_caller();
        let collected_fees = self.collected_fees().get();
        self.send().direct_egld(&caller_address, &collected_fees);
        self.collected_fees().clear();
    }
}