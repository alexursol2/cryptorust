multiversx_sc::imports!();

use crate::{storage, zombie_factory, zombie_feeding, zombie_helper};

#[multiversx_sc::module]
pub trait ZombieAttack:
    storage::Storage
    + zombie_feeding::ZombieFeeding
    + zombie_factory::ZombieFactory
    + zombie_helper::ZombieHelper
{
    fn rand_mod(&self, modulus: u8) -> u8{
        let mut rand_source = RandomnessSource::new(); // remember the formula of Randomness
        rand_source.next_u8() % modulus
    }
}
