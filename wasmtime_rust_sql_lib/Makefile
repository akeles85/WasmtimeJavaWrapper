TARG=--target=wasm32-wasi

.PHONY: build run clean sql_udf
build: sql_udf
	cargo build $(TARG)

run: build
	cargo run

clean:
	cargo clean
	rm Cargo.lock

sql_udf:
	cd sql_udf; cargo build $(TARG) 

.PHONY: release run-rel
release:
	cd sql_udf; cargo build $(TARG) --release; cd ..
	cargo build --release

run-rel: release
	cargo run --release

