VERSION ?= $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
export DEVFORGE_VERSION := $(VERSION)

CARGO ?= cargo

.PHONY: build test install uninstall clean all lint fmt fmt-check vet coverage

build:
	$(CARGO) build --release --locked
	mkdir -p bin
	cp target/release/devforge bin/devforge

test:
	$(CARGO) test --all-features --locked

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint:
	$(CARGO) clippy --all-targets --all-features --locked -- -D warnings

vet: lint

coverage:
	@command -v cargo-llvm-cov >/dev/null 2>&1 || { echo "cargo-llvm-cov not installed. Run: cargo install cargo-llvm-cov"; exit 1; }
	$(CARGO) llvm-cov --all-features --lcov --output-path coverage.lcov

install:
	$(CARGO) install --path . --locked --force

uninstall:
	-$(CARGO) uninstall devforge

clean:
	$(CARGO) clean
	rm -rf bin dist coverage.out coverage.html coverage.lcov

all: clean fmt-check lint build test
