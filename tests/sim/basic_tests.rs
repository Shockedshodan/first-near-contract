use near_sdk::serde_json::json;
use near_sdk_sim::DEFAULT_GAS;
use crate::utils::init;

#[test]
fn simulate_is_hash_exist() {
    let (root, contract, _alice) = init();
    let response: bool = root.view(
        contract.account_id(),
        "is_hash_exist",
        &json!({
            "hash": "B5D4045C3F466FA91FE2CC6ABE79232A1A57CDF104F7A26E716E0A1E2789DF78".to_string(),
        }).to_string().into_bytes(),
    ).unwrap_json();

   assert_eq!(response, false)
}

#[test]
fn simulate_hash_array() {
    let (root, contract, _alice) = init();
    let simple_vector = vec![1u8, 2,3,4,5];
    let response: String = root.call(contract.account_id(), "store_hash", &json!({
            "raw_vector": simple_vector,
        }).to_string().into_bytes(), DEFAULT_GAS, 0).unwrap_json();

    let res: bool = root.view(
        contract.account_id(),
        "is_hash_exist",
        &json!({
            "hash": response.to_string(),
        }).to_string().into_bytes(),
    ).unwrap_json();

    assert_eq!(res, true)
}


