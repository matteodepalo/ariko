.PHONY: test build flash clean

RUST_NIGHTLY := PATH="/opt/rust-nightly/bin:$${PATH}"

test:
	$(RUST_NIGHTLY) cargo test --target x86_64-unknown-linux-gnu --no-default-features --features simulator

build:
	$(RUST_NIGHTLY) cargo build --release

flash: build
	./deploy.sh

clean:
	cargo clean
