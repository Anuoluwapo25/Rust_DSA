#![cfg(test)]
use crate::hello::{Contract, ContractClient};
use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let words = client.rev_str(&String::from_str(&env, "Dev"));
    assert_eq!(words, String::from_str(env.clone(), "veD"));
}
