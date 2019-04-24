target/release/pa4: src/main.rs
	cargo build --release

.PHONY: build

build: target/release/pa4

.PHONY: run

run: target/release/pa4
	target/release/pa4 $F

.PHONY: clean

clean:
	cargo clean
