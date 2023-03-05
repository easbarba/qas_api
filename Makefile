.DEFAULT_GOAL := test

RUNNER ?= podman
NAME := qas

build:
	cargo build --release --verbose

run:
	cargo run

test:
	cargo test --verbose

.PHONY: build run test

deps:
	cargo fetch

fmt:
	cargo fmt

lint:
	cargo clippy

grab:
archive:

pub:
	cargo publish

img:
	${RUNNER} build --file ./Dockerfile --tag ${USER}/${NAME}:$(shell cat .version)
	${RUNNER} run --rm ${USER}/${NAME}:$(shell cat .version)

.PHONY: deps test fmt lint pub img run build grab archive
