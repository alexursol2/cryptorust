#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;
mod zombie;
mod zombie_factory;
mod zombie_feeding; // import of files

#[multiversx_sc::contract]
pub trait ZombiesContract: zombie_factory::ZombieFactory + zombie_feeding::ZombieFeeding + storage::Storage 
// Update the definition of our ZombieContract contract supertrait to contain all the other module traits
{
    #[init]
    fn init(&self) {
        self.dna_digits().set(16u8);
    }
}
