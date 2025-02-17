.PHONY: all clean lint test

all:
	cargo +nightly-2024-03-21 build --release -Z build-std=core,alloc --target armv6k-nintendo-3ds --manifest-path reader_core/Cargo.toml
	make -C 3gx
	mkdir -p out
	cp 3gx/build/3gx.3gx out/default.3gx

clean:
	cargo clean --manifest-path reader_core/Cargo.toml
	make clean -C 3gx
	rm -rf out

lint:
	cargo +nightly-2024-03-21 clippy --release -Z build-std=core,alloc --target armv6k-nintendo-3ds --manifest-path reader_core/Cargo.toml

test:
	cargo test --manifest-path reader_core/Cargo.toml
