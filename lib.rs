#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String,
};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certificate {
    pub student: Address,
    pub course: String,
    pub active: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Certificate(Address, String),
}

#[contractimpl]
impl Contract {
    pub fn initialize(env: Env, admin: Address) -> Address {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("contract already initialized");
        }

        admin.require_auth();

        env.storage().persistent().set(&DataKey::Admin, &admin);

        admin
    }

    pub fn issue(env: Env, student: Address, course: String) -> Certificate {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("contract is not initialized"));

        admin.require_auth();

        let key = DataKey::Certificate(student.clone(), course.clone());

        let certificate = Certificate {
            student,
            course,
            active: true,
        };

        env.storage().persistent().set(&key, &certificate);

        certificate
    }

    pub fn revoke(env: Env, student: Address, course: String) -> Certificate {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("contract is not initialized"));

        admin.require_auth();

        let key = DataKey::Certificate(student.clone(), course.clone());

        let mut certificate: Certificate = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("certificate not found"));

        certificate.active = false;

        env.storage().persistent().set(&key, &certificate);

        certificate
    }

    pub fn has_certificate(env: Env, student: Address, course: String) -> bool {
        let key = DataKey::Certificate(student, course);

        let certificate: Option<Certificate> = env.storage().persistent().get(&key);

        match certificate {
            Some(cert) => cert.active,
            None => false,
        }
    }

    pub fn get_certificate(env: Env, student: Address, course: String) -> Certificate {
        let key = DataKey::Certificate(student, course);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("certificate not found"))
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("contract is not initialized"))
    }

    pub fn is_initialized(env: Env) -> bool {
        env.storage().persistent().has(&DataKey::Admin)
    }
}

mod test;