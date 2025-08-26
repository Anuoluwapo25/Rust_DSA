// #![no_std]
// use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

// #[contract]
// pub struct Contract;


// #[contractimpl]
// impl Contract {
//     pub fn rev_str(env: Env, str: String) -> String {
//         let bytes = str.len();

//         let mut reverse = Vec::new(&env);

//         for i in (0..len()).rev() {
//             reverse.push_back(bytes.get(&i).unwrap());
            
//             }

//         reverse.into_iter().collet()
//         }
// }

#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec, Symbol, IntoVal};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn rev_str(env: Env, input: String) -> String {
        // Convert the string to a vector of bytes first
        let bytes = Vec::from_array(&env, &input.into_bytes());
        let len = bytes.len();
        
        // Create a new vector for reversed bytes
        let mut reversed_bytes = Vec::new(&env);
        
        // Iterate backwards through the bytes
        for i in (0..len).rev() {
            let byte = bytes.get(i).unwrap();
            reversed_bytes.push_back(byte);
        }
        
        // Convert bytes back to string
        String::from_bytes(&env, reversed_bytes)
    }
}

