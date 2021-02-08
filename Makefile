wasm-optimized:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown

wasm:
	cargo build --release --target wasm32-unknown-unknown

example:
	docker-compose up --abort-on-container-exit
