# The Idea
Since Celo is a fork of Ethereum, we could utilze the p2p protocol to create
a neat relayer. Basically the p2p is a transport layer (rlp) which based on
a given message/request returns the arbitrary data (think of slice of bytes),
which then have to be parsed according to client protocol (celo, eth, les,
istanbul etc).

I managed to get the ethereum node running, where the connection is established
succesfully. So in theory we could send some requests to the full-node (asking
for block stream etc). Minor issue: the ping/pong timeout doesn't seem to work
properly.

How to do it:
```
git clone https://github.com/openethereum/openethereum
cp main.rs openethereum/util/network-devp2p/
# add env_logger = "0.5" to dependencies
RUST_LOG=debug RUST_BACKTRACE=1 cargo run dev

git clone https://github.com/ethereum/go-ethereum.git
make geth
./build/bin/geth --rinkeby console
```

In the code you'd have to setup correct enode, so:
```
geth console
> admin.nodeInfo
```
