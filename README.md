[![Build Status][travis-badge]][travis]

[travis-badge]: https://travis-ci.org/ChorusOne/celo-light-client.svg?branch=master

# Celo Light Client
Implementation of celo light client in rust compilable to wasm. It is written to be integrated with CosmWasm readily, and is optimized to run in a constrained environment of a smart contract.

## How to?
NOTE: In order to run the light client example you need to spawn [celo-blockchain](https://github.com/celo-org/celo-blockchain) node.

### Quick setup (via `docker-compose`)
```
$ make example
```

### Manual setup
```
# first terminal window
$ git clone https://github.com/celo-org/celo-blockchain.git && cd celo-blockchain
$ go run build/ci.go install ./cmd/geth && ./build/bin/geth  --maxpeers 50 --light.maxpeers 20 --syncmode lightest --rpc  --ws --wsport 3334 --wsapi eth,net,web3 console

# second terminal window
$ git clone https://github.com/ChorusOne/celo-light-client.git && cd celo-light-client

$ RUST_LOG=info cargo run --example lightest-sync -- --fast
$ cargo test -- --nocapture
```

### Compiling to wasm
```
$ rustup target add wasm32-unknown-unknown
$ make wasm
$ stat target/wasm32-unknown-unknown/release/celo.wasm
```

## How does it work?
This library reflects the [lightest-sync](https://docs.celo.org/celo-codebase/protocol/consensus/ultralight-sync) mode, where the validator set for the current epoch is computed by downloading the last header of each previous epoch and applying the validator set diff. The latest block header is then verified by checking that at least two-thirds of the validator set for the current epoch signed the block header.

The validator set is being stored in the local KV-store so that after a restart, only new epoch blocks are processed.
