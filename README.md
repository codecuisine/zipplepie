# substrate-zk

A Substrate-based chain supporting `ZK-SNARK` verification.

## Download substrate-zk
```
git clone --recurse-submodules https://github.com/Zkvers/substrate-zk
```

## Environment
Rust version:   
```
1.65.0-aarch64-apple-darwin (default)  
rustc 1.65.0 (897e37553 2022-11-02)
```
## Build and Run verification on Substrate-based chain
- **groth16**: refer to `substrate-zk/zk-tutorials/proof-system/theory_to_practice` where you can build the full process which using `bellman` to verify the proof generated by `snarkjs`
- **plonk**: coming soon...