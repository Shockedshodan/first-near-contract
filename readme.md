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

```bash
cargo test
```
