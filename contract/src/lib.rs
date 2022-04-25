use near_sdk::{AccountId, assert_one_yocto, Promise, Timestamp};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::BorshStorageKey;
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::U128;

setup_alloc!();

// Max voucher deposit: 1000 NEAR
const MAX_DEPOSIT: u128 = 1_000_000_000_000_000_000_000_000_000;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Vouchers,
    AccountVouchers,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Voucher {
    id: String,
    payment_type: String,
    deposit_amount: u128,
    paid_amount: u128,
    create_date: Timestamp,
    expire_date: Option<Timestamp>,
    hash: String,
    used_by: Option<AccountId>,
}

impl Default for Voucher {
    fn default() -> Self {
        Self {
            id: String::new(),
            payment_type: String::new(),
            deposit_amount: env::attached_deposit(),
            paid_amount: 0,
            create_date: env::block_timestamp(),
            expire_date: None,
            hash: String::new(),
            used_by: None,
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
    /*
     * Get all user payment vouchers
     */
    pub fn user_vouchers(&self, account_id: String) -> Vec<Voucher> {
        match self.vouchers.get(&account_id) {
            Some(vouchers) => vouchers.to_vec(),
            None => vec![],
        }
    }

    /*
     * Add new payment voucher.
     */
    #[payable]
    pub fn add_voucher(&mut self, hash_list: Vec<String>, id_list: Vec<String>, expire_date: Option<Timestamp>, payment_type: String) {
        assert!(env::attached_deposit() > 0, "You should attach some Deposit");
        assert!(env::attached_deposit() <= MAX_DEPOSIT, "Please attach less than 1000 NEAR");
        let deposit_amount = env::attached_deposit() / id_list.len() as u128;

        if payment_type != "static" && expire_date.is_none() {
            panic!("Specify unlock date");
        }
        if expire_date.is_some() && expire_date.unwrap() <= env::block_timestamp() {
            panic!("Wrong expire date");
        }

        let owner_id = env::predecessor_account_id();
        let mut user_vouchers = match self.vouchers.get(&env::predecessor_account_id()) {
            Some(vouchers) => vouchers,
            None => UnorderedSet::new(StorageKeys::AccountVouchers),
        };

        for (index, id) in id_list.into_iter().enumerate() {
            let hash = hash_list[index].to_string();
            assert_eq!(id.len(), 12, "Error: Wrong ID value");
            assert_eq!(hash.len(), 64, "Error: Wrong Hash value");

            user_vouchers.insert(&Voucher {
                hash: hash.to_string(),
                id: id.to_string(),
                payment_type: payment_type.to_string(),
                expire_date,
                deposit_amount,
                ..Voucher::default()
            });
            self.vouchers.insert(&owner_id, &user_vouchers);
        }
    }

    /*
     * Remove payment voucher.
     */
    #[payable]
    pub fn remove_voucher(&mut self, id: String) {
        assert_one_yocto();
        let mut user_vouchers = match self.vouchers.get(&env::predecessor_account_id()) {
            Some(vouchers) => vouchers,
            None => panic!("Error: User vouchers not found"),
        };

        let voucher = user_vouchers.iter().find(|v| *v.id == id).unwrap();

        if voucher.used_by.is_none() || voucher.deposit_amount == voucher.paid_amount {
            if voucher.deposit_amount != voucher.paid_amount {
                let payment_diff = voucher.deposit_amount - voucher.paid_amount;
                Promise::new(env::predecessor_account_id()).transfer(payment_diff);
            }

            user_vouchers.remove(&voucher);
            self.vouchers.insert(&env::predecessor_account_id(), &user_vouchers);
        } else {
            panic!("Voucher locked and can't be removed");
        }
    }

    /*
     * Get available claim amount for voucher with linear unlock.
     */
    pub fn linear_claim_amount(&self, voucher: &Voucher) -> u128 {
        if voucher.payment_type == "linear" {
            let time_diff = u128::from((voucher.expire_date.unwrap() - voucher.create_date) / 1_000_000_000);
            let one_sec_reward = voucher.deposit_amount / time_diff;
            let seconds_from_start = u128::from((env::block_timestamp() - voucher.create_date) / 1_000_000_000);
            let unlocked = seconds_from_start * one_sec_reward;
            return unlocked - voucher.paid_amount;
        }
        voucher.deposit_amount - voucher.paid_amount
    }

    /*
     * Get information about payment voucher (except private key).
     */
    pub fn voucher_info(&self, id: String, account_id: String) -> (Voucher, U128) {
        let user_vouchers = match self.vouchers.get(&account_id) {
            Some(vouchers) => vouchers,
            None => panic!("Voucher not exists"),
        };
        let mut voucher = user_vouchers.iter().find(|v| *v.id == id).expect("Voucher not found");
        voucher.hash = String::new();
        let claimable = self.linear_claim_amount(&voucher);

        (voucher, claimable.into())
    }

    /*
     * Use voucher - get payment from voucher.
     */
    pub fn transfer(&mut self, key: String, id: String, account_id: String) -> U128 {
        assert_eq!(key.len(), 64, "Wrong Hash value");
        assert_eq!(id.len(), 12, "Wrong ID value");

        // Get all user vouchers
        let mut user_vouchers = match self.vouchers.get(&account_id) {
            Some(vouchers) => vouchers,
            None => panic!("Voucher not exists"),
        };

        // Get voucher by hash
        let hashed_key = env::sha256(key.as_bytes());
        let hashed_key_hex = hex::encode(&hashed_key);
        let mut voucher = user_vouchers.iter().find(|v| *v.hash == hashed_key_hex).expect("Voucher not found");

        if voucher.used_by.is_none() || voucher.used_by == Some(env::predecessor_account_id()) {
            let withdraw_amount;
            if voucher.payment_type == "static" {
                match voucher.expire_date {
                    Some(timestamp) => assert!(timestamp >= env::block_timestamp(), "Voucher expired"),
                    None => (),
                };
                assert!(voucher.paid_amount < 1, "Voucher already used");
                withdraw_amount = voucher.deposit_amount;
            } else {
                withdraw_amount = self.linear_claim_amount(&voucher);
                if withdraw_amount <= 0 {
                    panic!("Error: Nothing to claim");
                }
            }

            // Remove previous voucher
            user_vouchers.remove(&voucher);

            // Add updated voucher
            voucher.used_by = Some(env::predecessor_account_id());
            voucher.paid_amount += withdraw_amount;
            user_vouchers.insert(&voucher);
            self.vouchers.insert(&account_id, &user_vouchers);

            // Send payment
            Promise::new(env::predecessor_account_id()).transfer(withdraw_amount);
            return withdraw_amount.into();
        } else {
            panic!("You can't use this voucher");
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
    use std::convert::TryInto;

    use near_sdk::{testing_env, VMContext};
    use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::VMContextBuilder;

    use super::*;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".try_into().unwrap())
            .block_timestamp(99)
            .is_view(is_view)
            .build()
    }

    fn _add_voucher_internal(id: String, expire_date: Option<Timestamp>) -> (String, String) {
        let mut context = get_context(false);
        context.attached_deposit = 1;
        testing_env!(context.clone());

        let mut contract = VoucherContract::default();

        let private_key: String = String::from("x".repeat(64));
        let hashed_key = env::sha256(private_key.as_bytes());
        let hash_list = vec![hex::encode(&hashed_key)];
        let id_list = vec![id.to_string()];

        contract.add_voucher(hash_list, id_list, expire_date, String::from("static"));
        (private_key, id)
    }

    #[test]
    fn get_all_vouchers() {
        let context = get_context(true);
        testing_env!(context.clone());

        let contract = VoucherContract::default();
        assert_eq!(contract.user_vouchers(context.predecessor_account_id).len(), 0);
    }

    #[test]
    fn add_voucher() {
        let context = get_context(false);
        testing_env!(context.clone());

        let contract = VoucherContract::default();
        _add_voucher_internal(String::from("123456789012"),None);
        _add_voucher_internal(String::from("123456789013"),None);
        assert_eq!(contract.user_vouchers(context.predecessor_account_id).len(), 2);
    }

    #[test]
    fn remove_voucher() {
        let mut context = get_context(false);
        context.attached_deposit = 1;
        testing_env!(context.clone());

        let mut contract = VoucherContract::default();
        let (_, id) = _add_voucher_internal(String::from("123456789014"),None);
        contract.remove_voucher(id);

        assert_eq!(contract.user_vouchers(context.predecessor_account_id).len(), 0);
    }

    #[test]
    fn transfer() {
        let context = get_context(false);
        testing_env!(context.clone());

        let mut contract = VoucherContract::default();
        let (private_key, id) = _add_voucher_internal(String::from("123456789015"),None);
        let predecessor = context.predecessor_account_id;

        contract.transfer(private_key, id, predecessor.clone());

        let voucher = contract.user_vouchers(predecessor.clone());
        assert_eq!(voucher.len(), 1, "Voucher not found");
        assert_eq!(voucher[0].deposit_amount, 1, "Wrong deposit amount");
        assert_eq!(voucher[0].paid_amount, 1, "Wrong paid amount");
        assert_eq!(voucher[0].used_by, Some(predecessor.clone()), "Wrong used_by");
        assert_eq!(voucher[0].expire_date, None, "Wrong expire_date");
    }

    #[test]
    #[should_panic(expected = "Wrong expire date")]
    fn transfer_expired() {
        let context = get_context(false);
        testing_env!(context.clone());

        let mut contract = VoucherContract::default();
        let expired_date: Option<Timestamp> = Some(context.block_timestamp);
        let (private_key, id) = _add_voucher_internal(String::from("123456789016"),expired_date);
        let predecessor = context.predecessor_account_id;

        contract.transfer(private_key, id, predecessor.clone());
    }
}

