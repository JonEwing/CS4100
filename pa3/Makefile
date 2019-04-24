target/release/pa3: src/main.rs
	cargo build --release

.PHONY: build

build: target/release/pa3

.PHONY: run

run: target/release/pa3
	target/release/pa3 $F

.PHONY: clean

clean:
	cargo clean
