/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

use near_sdk::{AccountId, Promise, Timestamp};
use near_sdk::{env, near_bindgen, setup_alloc};
//log,
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::BorshStorageKey;
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Vouchers,
    AccountVouchers,
    // SubAccount { account_hash: Vec<u8> },
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)] // Default, Debug,
#[serde(crate = "near_sdk::serde")]
pub struct Voucher {
    id: String,
    deposit_amount: u128,
    create_date: Timestamp,
    expire_date: Option<Timestamp>,
    hash: String,
    is_used: bool,
    used_by: AccountId,
}


impl Default for Voucher {
    fn default() -> Self {
        Self {
            id: String::new(),
            deposit_amount: env::attached_deposit(),
            create_date: env::block_timestamp(),
            expire_date: None,
            hash: String::new(),
            used_by: String::new(),
            is_used: false,
        }
    }
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VoucherContract {
    vouchers: LookupMap<AccountId, UnorderedSet<Voucher>>,
}

impl Default for VoucherContract {
    fn default() -> Self {
        Self {
            vouchers: LookupMap::new(StorageKeys::Vouchers),
        }
    }
}

#[near_bindgen]
impl VoucherContract {
    pub fn user_vouchers(&self, account_id: String) -> Vec<Voucher> {
        match self.vouchers.get(&account_id) {
            Some(vouchers) => vouchers.to_vec(),
            None => vec![],
        }
    }

    #[payable]
    pub fn add_voucher(&mut self, hash: String, id: String, expire_date: Option<Timestamp>) {
        assert!(env::attached_deposit() > 0, "You should attach Deposit");
        assert_eq!(hash.len(), 64, "Wrong Hash");
        assert_eq!(id.len(), 12, "Wrong ID");

        let mut user_vouchers = match self.vouchers.get(&env::predecessor_account_id()) {
            Some(vouchers) => vouchers,
            None => UnorderedSet::new(StorageKeys::AccountVouchers),
        };

        user_vouchers.insert(&Voucher {
            hash,
            id,
            expire_date,
            ..Voucher::default()
        });
        self.vouchers.insert(&env::predecessor_account_id(), &user_vouchers);
    }

    pub fn remove_voucher(&mut self, id: String) {
        let mut vouchers = match self.vouchers.get(&env::predecessor_account_id()) {
            Some(vouchers) => vouchers,
            None => panic!("Vouchers not found!"),
        };

        let selected_voucher = vouchers.iter().find(|v| *v.id == id).unwrap();

        // return rest voucher balance to user
        if selected_voucher.deposit_amount > 0 {
            Promise::new(env::predecessor_account_id()).transfer(selected_voucher.deposit_amount);
        }

        // remove voucher
        vouchers.remove(&selected_voucher);
        self.vouchers.insert(&env::predecessor_account_id(), &vouchers);
    }

    pub fn transfer(&mut self, key: String, id: String, account_id: String, pay_amount: String) {
        assert_eq!(key.len(), 64, "Wrong Hash");
        assert_eq!(id.len(), 12, "Wrong ID");

        let pay_amount = pay_amount.parse().unwrap();
        assert!(pay_amount > 0, "Wrong Payment amount");

        let user_vouchers = match self.vouchers.get(&account_id) {
            Some(vouchers) => vouchers,
            None => panic!("Vouchers not found!"),
        };

        let hashed_key = env::sha256(key.as_bytes());
        let hashed_key_hex = hex::encode(&hashed_key);

        let mut voucher = user_vouchers.iter().find(|v| *v.hash == hashed_key_hex).expect("User voucher not found");
        assert!(voucher.deposit_amount >= pay_amount, "Too big amount for this voucher!");
        match self.vouchers.get(&account_id) {
            Some(vouchers) => vouchers,
            None => panic!("Vouchers not found!"),
        };

        assert_eq!(voucher.used_by.len(), 0, "Voucher already used 1 !");
        assert_eq!(voucher.is_used, false, "Voucher already used 2 !");
        voucher.used_by = env::predecessor_account_id().clone();
        voucher.is_used = true;
        self.vouchers.insert(&account_id, &user_vouchers);

        Promise::new(env::predecessor_account_id()).transfer(pay_amount);

        // send rest to owner
        let rest_amount = voucher.deposit_amount - pay_amount;
        if rest_amount > 0 {
            Promise::new(account_id).transfer(rest_amount);
        }
    }
}


/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use near_sdk::{testing_env, VMContext};
    use near_sdk::MockedBlockchain;

    use super::*;

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn get_all_vouchers() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = VoucherContract::default();
        assert_eq!(contract.all_vouchers().len(), 0);

        // let hash = String::from("1");
        // let id = String::from("1");
        // contract.add_voucher(hash, id);

        assert_eq!(contract.all_vouchers().len(), 1);
    }
}

