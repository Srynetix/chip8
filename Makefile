# Makefile

all: build

build:
	@cargo build

test:
	@cargo test --all

fmt:
	@cargo fmt --all

lint:
	@cargo fmt --all -- --check
	@cargo clippy

.PHONY: build test fmt lint
