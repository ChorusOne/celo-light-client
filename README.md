# celo-light-client-lib
... TODO ...

slidedeck: https://docs.google.com/presentation/d/1fGcO_DqXXuGJjRViSOwxcZVteQO9Xapa3FhZT1_gZy0/edit#slide=id.gb1c8621faf_0_194

## How to?
```
# first terminal window
$ git clone https://github.com/celo-org/celo-blockchain.git && cd celo-blockchain
$ go run build/ci.go install ./cmd/geth && ./build/bin/geth  --maxpeers 50 --light.maxpeers 20 --syncmode lightest --rpc  console

# second terminal window
$ git clone https://github.com/ChorusOne/celo-light-client-lib.git && cd celo-light-client-lib

$ RUST_LOG=info cargo run --example lightest-sync
$ cargo test -- --nocapture
```
