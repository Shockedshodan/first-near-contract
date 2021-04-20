First contract
===================
##Task
Create Rust Smart Contract:
- Function that receives not a null array ```(Vec<u8>)```
- Calculate sha256 hash 
- Check if hash exist in the storage: if not - revert, else save to storage
- Return hash as String
- Cover with unit tests
- Cover with integration tests


## Building
To build run:
```bash
./build.sh
```

## Testing
Local testing
```bash
cargo test
```
Test deployed example contract

- Save null vector. Contract will panic because you can't save null vector.
```bash
near call firstcontract.shockedshodan.testnet store_hash '{"raw_vector": []}'  --accountId accountId-example.testnet
```
- Save non null vector. 
```bash
near call firstcontract.shockedshodan.testnet store_hash '{"raw_vector": [1, 2, 3, 4]}'  --accountId accountId-example.testnet
```
- Check is hash exist in storage. Should return true.
```bash
near view firstcontract.shockedshodan.testnet is_hash_exist '{"hash": "9F64A747E1B97F131FABB6B447296C9B6F0201E79FB3C5356E6C77E89B6A806A"}'
```
