[![Build Status][travis-badge]][travis]

[travis-badge]: https://travis-ci.org/ChorusOne/celo-light-client.svg?branch=main
[travis]: https://travis-ci.org/ChorusOne/celo-light-client/

# Celo Light Client
The library provides a [lightest-sync](https://docs.celo.org/celo-codebase/protocol/consensus/ultralight-sync) mode, that enables a quick, secure, and cheap way to synchronize IBFT consensus state with a Celo Blockchain node.

The codebase is split into two parts:
* `library` - a subset of Celo Blockchain building blocks, such as data structures (ie. Header, IBFT) or functionalities (ie. serialization, consensus state management)
* `contract` - an (optional) [IBC](https://docs.cosmos.network/master/ibc/overview.html) compatible light client contract, intended to be run on Cosmos Blockchain as WASM binary

**ultralight-sync explained**

In the nutshell, the validator set for the current epoch is computed by downloading the last header of each previous epoch and applying the validator set diff. The latest block header is then verified by checking that at least two-thirds of the validator set for the current epoch signed the block header.
Ultralight mode download approximately 30,000 times fewer headers than light nodes in order to sync the latest block (assuming 3-second block periods and 1-day epochs).

### Example
An example program that utilizes `lightest-sync` library is placed in the `examples/lightest-sync`. It uses [celo-blockchain node](https://github.com/celo-org/celo-blockchain) to fetch epoch headers, built up the validator set, and verify the latest available header.

You may spawn up example program via:
```
$ docker-compose up --abort-on-container-exit
```

### Light Client
The CosmWasm contract is gated by `wasm-contract` feature:
```
$ rustup target add wasm32-unknown-unknown
$ cargo build --release --features wasm-contract --target wasm32-unknown-unknown
```

To compile optimized binary run:
```
$ make wasm-optimized
$ stat target/wasm32-unknown-unknown/release/celo.wasm
```

### Demo
[![asciicast](https://asciinema.org/a/411776.svg)](https://asciinema.org/a/411776)
