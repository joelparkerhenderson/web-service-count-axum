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
# cargo-build-release:: Use cargo to build release files for our preferred targets.
##
cargo-build-release: cargo-build-release-mac cargo-build-release-linux cargo-build-release-windows

##
# cargo-build-release-mac: Use cargo to build release for our preferred macOS target.
##
cargo-build-release-mac:
	cargo build --release --target aarch64-apple-darwin
	git add --force target/aarch64-apple-darwin/release/web-service-count-axum

##
# cargo-build-release-mac: Use cargo to build release for our preferred Linux target.
##
cargo-build-release-linux:
	TARGET_CC=x86_64-unknown-linux-gnu cargo build --release --target x86_64-unknown-linux-gnu
	git add --force target/x86_64-unknown-linux-gnu/release/web-service-count-axum

##
# cargo-build-release-mac: Use cargo to build release for our preferred Windows target.
##
cargo-build-release-windows:
	cargo build --release --target=x86_64-pc-windows-gnu
	git add --force target/x86_64-pc-windows-gnu/release/web-service-count-axum.exe                    
