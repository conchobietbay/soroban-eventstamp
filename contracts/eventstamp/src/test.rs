#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

#[test]
fn test_initialize_and_check_in() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let participant = Address::generate(&env);
    let title = String::from_str(&env, "Stellar Builder Meetup");

    assert_eq!(client.is_initialized(), false);

    let event = client.initialize(&admin, &title, &50);

    assert_eq!(event.admin, admin);
    assert_eq!(event.title, title);
    assert_eq!(event.capacity, 50);
    assert_eq!(event.checked_in, 0);
    assert_eq!(event.open, true);

    assert_eq!(client.is_initialized(), true);
    assert_eq!(client.has_checked_in(&participant), false);

    let updated = client.check_in(&participant);

    assert_eq!(updated.checked_in, 1);
    assert_eq!(updated.open, true);
    assert_eq!(client.has_checked_in(&participant), true);

    let saved = client.get_event();

    assert_eq!(saved.checked_in, 1);
    assert_eq!(saved.capacity, 50);
}

#[test]
fn test_close_event() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let title = String::from_str(&env, "Rust on Stellar");

    client.initialize(&admin, &title, &10);

    let closed = client.close_event();

    assert_eq!(closed.open, false);

    let saved = client.get_event();

    assert_eq!(saved.open, false);
}