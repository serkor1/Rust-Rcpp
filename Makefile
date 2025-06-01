SHELL := /bin/bash
.PHONY: help

help: ## List all options
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

build-lib: ## Cleans and builds Rust library with C++ bindings as api.h
	cd src/pkg && cargo clean && cargo build && cbindgen --lang c++ --output api.h

build-pkg: ## Builds R package with {Rcpp} and {pak}
	Rscript -e "Rcpp::compileAttributes()"
	Rscript -e "devtools::document()"
	Rscript -e "pak::pak()"

test-pkg: ## Test the exported functions
	Rscript -e "testthat::test_local()"

test-rust: ## Test the Rust implementation
	cd src/pkg && cargo test

purge: ## Delete all non-tracked files
	git clean -d -x -f 
	cd src/pkg && cargo clean