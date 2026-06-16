use soroban_sdk::{
    symbol_short, String,
};

mod helpers;
use helpers::{register_and_setup, set_timestamp, single_ben};
use stellar_give::ContractError;

#[test]
fn test_long_title_rejected() {
    let (env, client, creator, beneficiary, _donor, _admin, token_client, _token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);
    let bens = single_ben(&env, &beneficiary);

    // Create a title with 10,000 characters
    extern crate std;
    let mut long_title_raw = std::string::String::new();
    for _ in 0..10_000 {
        long_title_raw.push('A');
    }
    let long_title = String::from_str(&env, &long_title_raw);

    let result = client.try_create_campaign(
        &creator,
        &bens,
        &long_title,
        &String::from_str(&env, "A test campaign description."),
        &String::from_str(&env, "https://example.com/meta"),
        &symbol_short!("relief"),
        &10_000_000_i128,
        &2_000_u64,
        &token_client.address,
        &None,
    );

    assert_eq!(result, Err(Ok(ContractError::InvalidTitle)));
}

#[test]
fn test_gas_cost_max_title() {
    let (env, client, creator, beneficiary, _donor, _admin, token_client, _token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);
    let bens = single_ben(&env, &beneficiary);

    // Max allowed title length is 50
    let title_50 = String::from_str(&env, "12345678901234567890123456789012345678901234567890");

    env.budget().reset_default();
    let before_cpu = env.budget().cpu_instruction_cost();
    let before_mem = env.budget().memory_bytes_cost();

    let result = client.try_create_campaign(
        &creator,
        &bens,
        &title_50,
        &String::from_str(&env, "A test campaign description."),
        &String::from_str(&env, "https://example.com/meta"),
        &symbol_short!("relief"),
        &10_000_000_i128,
        &2_000_u64,
        &token_client.address,
        &None,
    );

    let after_cpu = env.budget().cpu_instruction_cost();
    let after_mem = env.budget().memory_bytes_cost();

    assert!(result.is_ok());
    
    // Print results for documentation (will be visible in test output with --nocapture)
    extern crate std;
    std::println!("Gas for max title (50 chars): CPU: {}, MEM: {}", after_cpu - before_cpu, after_mem - before_mem);
}

#[test]
fn test_unicode_title_edge_cases() {
    let (env, client, creator, beneficiary, _donor, _admin, token_client, _token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);
    let bens = single_ben(&env, &beneficiary);

    let test_cases = [
        ("🎉 Campaign 🚀", "Emoji title"),
        ("مهمة إغاثة", "Arabic (RTL) title"),
        ("Camp\u{200B}aign", "Zero-width space"),
        ("汉语 Campaign", "Chinese and Latin"),
    ];

    for (raw_title, description) in test_cases {
        let title = String::from_str(&env, raw_title);
        let result = client.try_create_campaign(
            &creator,
            &bens,
            &title,
            &String::from_str(&env, "A test campaign description."),
            &String::from_str(&env, "https://example.com/meta"),
            &symbol_short!("relief"),
            &10_000_000_i128,
            &2_000_u64,
            &token_client.address,
            &None,
        );

        assert!(result.is_ok(), "Failed for {}: {}", description, raw_title);
    }
}
