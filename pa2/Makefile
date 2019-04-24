target/release/pa2: src/main.rs
	cargo build --release

.PHONY: build

build: target/release/pa2

.PHONY: run

run: target/release/pa2
	target/release/pa2 $F

.PHONY: clean

clean:
	cargo clean
