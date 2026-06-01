//! Shared test helpers for integration tests.
//!
//! Mirrors the `setup()` and `set_timestamp()` helpers from the inline
//! `#[cfg(test)]` module in `lib.rs`, exposed for use by external test files.

use soroban_sdk::{
    testutils::Address as _,
    token, Address, Env, Vec,
};
use stellar_give::{StellarGiveContract, StellarGiveContractClient};

/// Set the ledger timestamp for time-dependent test scenarios.
pub fn set_timestamp(env: &Env, timestamp: u64) {
    let mut ledger = env.ledger().get();
    ledger.timestamp = timestamp;
    env.ledger().set(ledger);
}

/// Standard test setup: registers a token, mints balances, deploys the
/// contract, and initializes it. All auths are mocked.
pub fn register_and_setup() -> (
    Env,
    StellarGiveContractClient<'static>,
    Address,
    Address,
    Address,
    Address,
    token::Client<'static>,
    token::StellarAssetClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let creator = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let donor = Address::generate(&env);
    let platform_admin = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = token::Client::new(&env, &token_id.address());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id.address());

    // Mint enough for all test scenarios (1_000 XLM equivalent).
    token_admin_client.mint(&donor, &1_000_000_000_000);
    token_admin_client.mint(&creator, &1_000_000_000_000);

    let contract_id = env.register_contract(None, StellarGiveContract);
    let client = StellarGiveContractClient::new(&env, &contract_id);
    client.initialize(&platform_admin);

    (
        env,
        client,
        creator,
        beneficiary,
        donor,
        platform_admin,
        token_client,
        token_admin_client,
    )
}

/// Creates a single-beneficiary vector with 100% share (10_000 bps).
pub fn single_ben(env: &Env, beneficiary: &Address) -> Vec<(Address, u32)> {
    let mut bens = Vec::new(env);
    bens.push_back((beneficiary.clone(), 10_000_u32));
    bens
}
