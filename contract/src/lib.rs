use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedSet;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

pub type HashedVector = Vec<u8>;
const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HashStorage {
    hash_storage: UnorderedSet<HashedVector>
}

#[near_bindgen]
impl HashStorage {
    pub fn store_hash(&mut self, raw_vector: Vec<u8>) -> String {
        if raw_vector.len() == 0 {
            env::panic(b"Vec is null.");
        }
        let hashed_vector = env::sha256(&*raw_vector);
        let is_hash_exist = self.hash_storage.contains(&hashed_vector);

        if is_hash_exist {
            env::panic(b"Hash already exist.");
        } else {
            self.hash_storage.insert(&hashed_vector);
            //I dont think this is a correct way
            let mut stringified_hash = String::with_capacity(hashed_vector.len() * 2);
            for n in hashed_vector.iter() {
                stringified_hash.push(HEX_CHARS_UPPER[(n >> 4) as usize] as char);
                stringified_hash.push(HEX_CHARS_UPPER[(n & 0x0F) as usize] as char);
            }
            stringified_hash
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn byte_array_abc() -> Vec<u8> {
        vec![0x41, 0x42, 0x43]
    }
    // fn byte_array_def() -> Vec<u8> {
    //     vec![0x44, 0x45, 0x46]
    // }
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
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
    #[should_panic(
    expected = r#"Vec is null."#
    )]
    fn recieve_null_array_and_panic() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = HashStorage { hash_storage: UnorderedSet::new(vec![]) };
        let _store_hash = contract.store_hash(vec![]);
    }
    #[test]
    fn hash_array() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = HashStorage { hash_storage: UnorderedSet::new(vec![]) };
        let vector_abc = byte_array_abc();
        let store_hash_result = contract.store_hash(vector_abc);
        assert!(store_hash_result.len() > 0,"Store hash result string length less than 1.");
    }
    #[test]
    #[should_panic(
    expected = r#"Hash already exist."#
    )]
    fn hash_same_array_twice_and_panic() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = HashStorage { hash_storage: UnorderedSet::new(vec![]) };
        let vector_abc = byte_array_abc();
        let store_hash_result = contract.store_hash(Vec::from(&*vector_abc));
        assert!(store_hash_result.len() > 0,"Store hash result string length less than 1.");
        contract.store_hash(vector_abc);
    }



}
