# Makefile

all: build

build:
	@cargo build

build-release:
	@cargo build --release

test:
	@cargo test --all

fmt:
	@cargo fmt --all

lint:
	@cargo fmt --all -- --check
	@cargo clippy

.PHONY: build test fmt lint
