#![no_std]

use soroban_sdk::{contractimpl, Address, BytesN, Env};

pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    pub fn register_attestation(
        _env: Env,
        _batch_id: BytesN<32>,
        _proof_hash: BytesN<32>,
        _off_chain_uri: String,
        _collector: Address,
        _timestamp: u64,
    ) {
    }

    pub fn has_attestation(_env: Env, _batch_id: BytesN<32>) -> bool {
        false
    }
}
