#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String,
};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventInfo {
    pub admin: Address,
    pub title: String,
    pub capacity: u32,
    pub checked_in: u32,
    pub open: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Event,
    CheckedIn(Address),
}

#[contractimpl]
impl Contract {
    pub fn initialize(
        env: Env,
        admin: Address,
        title: String,
        capacity: u32,
    ) -> EventInfo {
        if env.storage().persistent().has(&DataKey::Event) {
            panic!("event already initialized");
        }

        if capacity == 0 {
            panic!("capacity must be greater than zero");
        }

        admin.require_auth();

        let event = EventInfo {
            admin,
            title,
            capacity,
            checked_in: 0,
            open: true,
        };

        env.storage().persistent().set(&DataKey::Event, &event);

        event
    }

    pub fn check_in(env: Env, participant: Address) -> EventInfo {
        participant.require_auth();

        let mut event: EventInfo = env
            .storage()
            .persistent()
            .get(&DataKey::Event)
            .unwrap_or_else(|| panic!("event is not initialized"));

        if !event.open {
            panic!("event is closed");
        }

        if event.checked_in >= event.capacity {
            panic!("event is full");
        }

        let key = DataKey::CheckedIn(participant);

        if env.storage().persistent().has(&key) {
            panic!("participant already checked in");
        }

        env.storage().persistent().set(&key, &true);

        event.checked_in += 1;

        env.storage().persistent().set(&DataKey::Event, &event);

        event
    }

    pub fn close_event(env: Env) -> EventInfo {
        let mut event: EventInfo = env
            .storage()
            .persistent()
            .get(&DataKey::Event)
            .unwrap_or_else(|| panic!("event is not initialized"));

        event.admin.require_auth();

        event.open = false;

        env.storage().persistent().set(&DataKey::Event, &event);

        event
    }

    pub fn has_checked_in(env: Env, participant: Address) -> bool {
        let key = DataKey::CheckedIn(participant);

        env.storage().persistent().has(&key)
    }

    pub fn get_event(env: Env) -> EventInfo {
        env.storage()
            .persistent()
            .get(&DataKey::Event)
            .unwrap_or_else(|| panic!("event is not initialized"))
    }

    pub fn is_initialized(env: Env) -> bool {
        env.storage().persistent().has(&DataKey::Event)
    }
}

mod test;