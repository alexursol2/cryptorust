multiversx_sc::imports!();
// multiversx_sc::derive_imports!(); will not be required in this case since 
// we will not define any new struct type that will need the encode / decode implementations for serialization

use crate::{storage, zombie_factory, zombie_feeding, zombie_helper};
// Import the storage, zombie_factory, zombie_feeding and zombie_helper trait files

#[multiversx_sc::module]
pub trait ZombieAttack:
// Declare a new module supertrait called ZombieAttack that implements Storage, ZombieFeeding, ZombieFactory and ZombieHelper
    storage::Storage
    + zombie_feeding::ZombieFeeding
    + zombie_factory::ZombieFactory
    + zombie_helper::ZombieHelper
{
}