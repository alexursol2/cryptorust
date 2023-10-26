multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Zombie<M: ManagedTypeApi> {
  pub name: ManagedBuffer<M>,
  pub dna: u64,
  pub level: u16,
  pub ready_time: u64,
  // we're going to add 2 new features to our zombies: level and ready_time — 
  // the latter will be used to implement a cooldown timer to limit how often a zombie can feed
}
