TARG=--target=wasm32-wasi

.PHONY: build run clean guest
build: guest
	cargo build

run: build
	cargo run

clean:
	cargo clean
	rm Cargo.lock

guest:
	cd guest; cargo build $(TARG) 

.PHONY: release run-rel
release:
	cd guest; cargo build $(TARG) --release; cd ..
	cargo build --release

run-rel: release
	cargo run --release

