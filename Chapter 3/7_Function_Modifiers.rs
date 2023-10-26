multiversx_sc::imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait ZombieHelper: storage::Storage {
    fn check_above_level(&self, level: u16, zombie_id: usize){
        let my_zombie = self.zombies(&zombie_id).get(); // don`t forget about get
        require!(my_zombie.level >= level, "Zombie is too low level");
    }
}
