multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage, zombie_factory};

mod crypto_kitties_proxy {
    // Define a module proxy called crypto_kitties_proxy and put the get_kitty endpoint inside it. 
    // Don't forget to add also multiversx_sc::imports!(); and multiversx_sc::derive_imports!(); to the proxy module 
    // together with the #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)] macro to the Kitty struct.
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
    pub struct Kitty {
        pub is_gestating: bool,
        pub is_ready: bool,
        pub cooldown_index: u64,
        pub next_action_at: u64,
        pub siring_with_id: u64,
        pub birth_time: u64,
        pub matron_id: u64,
        pub sire_id: u64,
        pub generation: u64,
        pub genes: u64,
    }

    #[multiversx_sc::proxy]
    pub trait CryptoKitties {
        #[endpoint]
        fn get_kitty(&self, id: usize) -> Kitty;
    }
}

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
        let max_dna_value = u64::pow(10u64, dna_digits as u32);
        let verified_target_dna = target_dna % max_dna_value;
        let new_dna = (my_zombie.dna + verified_target_dna) / 2;
        self.create_zombie(caller, ManagedBuffer::from(b"NoName"), new_dna);
    }
    // proxy function inside the ZombieFeeding module
    #[proxy]
    fn kitty_proxy(&self, to: ManagedAddress) -> crypto_kitties_proxy::Proxy<Self::Api>;
}
