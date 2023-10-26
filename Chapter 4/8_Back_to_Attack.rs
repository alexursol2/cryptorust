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
        let caller = self.blockchain().get_caller(); // retreive the caller of the contract
        self.check_zombie_belongs_to_caller(zombie_id, &caller);
        let rand = self.rand_mod(100u8);
    }
}
