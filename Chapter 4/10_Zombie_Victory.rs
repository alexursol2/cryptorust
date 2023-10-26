multiversx_sc::imports!();

use crate::{storage, zombie_factory, zombie_feeding, zombie_helper};

#[multiversx_sc::module]
pub trait ZombieAttack:
    storage::Storage
    + zombie_feeding::ZombieFeeding
    + zombie_factory::ZombieFactory
    + zombie_helper::ZombieHelper
{
    fn rand_mod(&self, modulus: u8) -> u8 {
        let mut rand_source = RandomnessSource::new();
        rand_source.next_u8() % modulus
    }

    #[endpoint]
    fn attack(&self, zombie_id: usize, target_id: usize){
        let caller = self.blockchain().get_caller();
        self.check_zombie_belongs_to_caller(zombie_id, &caller);
        let rand = self.rand_mod(100u8);
        let attack_victory_probability = self.attack_victory_probability().get();
        // the storage value of attack_victory_probability in a variable with the same name
        if rand <= attack_victory_probability {
            self.zombies(&zombie_id).update(|my_zombie| {
                my_zombie.win_count += 1;
                my_zombie.level += 1;
            });
            // increase win_count and level by 1

            let mut enemy_dna = 0;
            self.zombies(&target_id).update(|enemy_zombie| {
                enemy_zombie.loss_count += 1;
                enemy_dna = enemy_zombie.dna;
            });
            // increase loss_count by 1 and set enemy_dna equal to enemy_zombie.dna
            self.feed_and_multiply(zombie_id, enemy_dna, ManagedBuffer::from(b"zombie"));
            // It doesn't actually do anything at the moment
        }
    }
}
