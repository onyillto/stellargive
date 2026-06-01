//! Integration tests for campaign expiration and time-based state transitions.
//!
//! These tests validate that the contract correctly rejects donations after a
//! campaign's deadline has passed, and that the edge case at exactly the
//! deadline timestamp behaves consistently with the contract's `>` comparison.

use soroban_sdk::{
    symbol_short, testutils::Address as _, token, Address, Env, String, Vec,
};

mod helpers;
use helpers::{register_and_setup, set_timestamp, single_ben};
use stellar_give::{CampaignStatus, ContractError, StellarGiveContract, StellarGiveContractClient};

/// Donating after the deadline must fail with `CampaignNotActive`.
#[test]
fn test_donate_after_deadline() {
    let (env, client, creator, beneficiary, donor, _admin, token_client, _token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);

    let bens = single_ben(&env, &beneficiary);
    let deadline = 1_100_u64; // 100 seconds in the future

    let campaign_id = client.create_campaign(
        &creator,
        &bens,
        &String::from_str(&env, "Short Relief"),
        &String::from_str(&env, "https://example.com/meta"),
        &symbol_short!("relief"),
        &10_000_000_i128,
        &deadline,
        &token_client.address,
        &None,
    );

    // Advance time strictly past the deadline
    set_timestamp(&env, 1_101);

    let result = client.try_donate(
        &donor,
        &campaign_id,
        &1_000_000_i128,
        &false,
        &None,
    );

    assert_eq!(result, Err(Ok(ContractError::CampaignNotActive)));
}

/// Donating *exactly* at the deadline timestamp should still succeed.
///
/// The contract uses `now > campaign.deadline` (strict greater-than) for
/// expiry detection, so `now == deadline` keeps the campaign active.
#[test]
fn test_donate_exactly_at_deadline() {
    let (env, client, creator, beneficiary, donor, _admin, token_client, _token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);

    let bens = single_ben(&env, &beneficiary);
    let deadline = 1_100_u64;

    let campaign_id = client.create_campaign(
        &creator,
        &bens,
        &String::from_str(&env, "Edge Relief"),
        &String::from_str(&env, "https://example.com/meta"),
        &symbol_short!("relief"),
        &10_000_000_i128,
        &deadline,
        &token_client.address,
        &None,
    );

    // Set time to exactly the deadline
    set_timestamp(&env, 1_100);

    // Should succeed — contract uses strict `>` check
    let result = client.try_donate(
        &donor,
        &campaign_id,
        &1_000_000_i128,
        &false,
        &None,
    );

    assert!(result.is_ok(), "Donation at exactly the deadline should succeed");

    // Verify campaign is still active (not expired) with raised amount updated
    let campaign = client.get_campaign(&campaign_id);
    assert_eq!(campaign.raised_amount, 1_000_000);
    assert_eq!(campaign.status, CampaignStatus::Active);
}

/// `get_campaign` should derive `Expired` status once the ledger time passes
/// the deadline, even if the stored status is still `Active`.
#[test]
fn test_status_transitions_from_active_to_expired() {
    let (env, client, creator, beneficiary, _donor, _admin, token_client, _token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);

    let bens = single_ben(&env, &beneficiary);
    let deadline = 1_100_u64;

    let campaign_id = client.create_campaign(
        &creator,
        &bens,
        &String::from_str(&env, "Expiry Check"),
        &String::from_str(&env, "https://example.com/meta"),
        &symbol_short!("relief"),
        &10_000_000_i128,
        &deadline,
        &token_client.address,
        &None,
    );

    // Before deadline: should be Active
    let campaign = client.get_campaign(&campaign_id);
    assert_eq!(campaign.status, CampaignStatus::Active);

    // Exactly at deadline: still Active (strict >)
    set_timestamp(&env, 1_100);
    let campaign = client.get_campaign(&campaign_id);
    assert_eq!(campaign.status, CampaignStatus::Active);

    // One second past deadline: Expired
    set_timestamp(&env, 1_101);
    let campaign = client.get_campaign(&campaign_id);
    assert_eq!(campaign.status, CampaignStatus::Expired);
}
