top = $(shell git rev-parse --show-toplevel)

default: llms cargo-build-release

##
# help: display this help message.
##
.PHONY: help
help:
	@awk '/^##/{a=1-a}a' $(MAKEFILE_LIST) | cut -c3-

##
# llms: generate AI context files fore large language models.
##
llms:
	rustdoc-llms
	cp target/doc/*.json llms.json
	cp target/doc/llms.txt llms.txt

##
# cargo-build-release: Use cargo to build release files for our preferred targets.
##
cargo-build-release:
	cargo build --release --target aarch64-apple-darwin
	git add --force target/aarch64-apple-darwin/release/web-service-count-axum
	cargo build --release --target x86_64-pc-windows-gnu
	git add --force target/x86_64-pc-windows-gnu/release/web-service-count-axum.exe
	TARGET_CC=x86_64-unknown-linux-gnu cargo build --release --target x86_64-unknown-linux-gnu
	git add --force target/x86_64-unknown-linux-gnu/release/web-service-count-axum
	TARGET_CC=x86_64-unknown-linux-musl cargo build --release --target x86_64-unknown-linux-musl
	git add --force target/x86_64-unknown-linux-musl/release/web-service-count-axum

##
# podman-build: Use podman to build the container.
##
.PHONY: podman-build
podman-build:
	#podman machine start
	podman build -t web-service-count-axum "${top}"
