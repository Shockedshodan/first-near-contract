use near_sdk_sim::{init_simulator, to_yocto, STORAGE_AMOUNT, UserAccount};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CONTRACT_WASM_BYTES => "res/re_04.wasm",
}


const CONTRACT_ID: &str = "contract";

pub fn init() -> (UserAccount, UserAccount, UserAccount) {
    let root = init_simulator(None);

    let contract = root.deploy(
        &CONTRACT_WASM_BYTES,
        CONTRACT_ID.to_string(),
        STORAGE_AMOUNT
    );

    let alice = root.create_user(
        "alice".to_string(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}
