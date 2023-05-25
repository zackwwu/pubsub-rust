.PHONY: test
test: rust-test ## Test Go code

.PHONY: lint
lint: rust-lint ## Perform lint checks on code, updates files


.PHONY: help

COL_RED=$(shell [ -n "${TERM}" ] && tput setaf 1)
COL_GREEN=$(shell [ -n "${TERM}" ] && tput setaf 2)
COL_CYAN=$(shell [ -n "${TERM}" ] && tput setaf 6)
COL_GREY=$(shell [ -n "${TERM}" ] && tput setaf 8)
COL_RESET=$(shell [ -n "${TERM}" ] && tput sgr0)

help:
	@# Explicitly grep ./Makefile to avoid any included config files
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' ./Makefile | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

rust-test:
	@echo "Running tests..."
	@cargo llvm-cov

rust-lint:
	@echo "Running lints..."
	@cargo clippy --all -- -D warnings

.DEFAULT_GOAL := help
