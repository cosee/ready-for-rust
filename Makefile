
test: ## run all tests with cargo
	cargo test

build: ## build binary with cargo
	cargo build
	@echo " -> Binary is located in ./target/debug/ready-for-rust"

run: ## run with cargo
	cargo run

help:
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make \033[36m<target>\033[0m\n"} /^[0-9a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
