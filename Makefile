SHELL=/bin/bash

export PVER ?= 0.1.0


# Building the app
build: Cargo.toml
	cargo build


# Building Cargo.toml from template
Cargo.toml: Cargo.template.toml
	@rm -f Cargo.lock
	@envsubst '$${PVER}' \
		< Cargo.template.toml \
		> Cargo.toml


# Remove Cargos
.PHONY: clean
clean:
	rm -f Cargo.toml
	rm -f Cargo.lock

# Remove all targets
.PHONY: clean_target
clean_target:
	rm -rf target

# Remove both - cargos and targets
.PHONY: clean_all
clean_all: clean clean_target


# Build all the elements in workspace: crates, examples, tests
.PHONY: build_all
build_all: build_ws build_ex

# Build workspace only
.PHONY: build_ws
build_ws: Cargo.toml
	@cargo build --workspace

# Build examples only
.PHONY: build_ex
build_ex: Cargo.toml
	@cargo build --examples
