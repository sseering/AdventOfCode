MAKEFLAGS += --no-builtin-rules --no-builtin-variables --no-print-directory

.PHONY: clean clean_rust build build_rust default
.SILENT: clean clean_rust build build_rust default

default:
	echo Available targets: clean build

clean: clean_rust

clean_rust:
	find . -type f -name Cargo.toml -execdir cargo clean ";"
	echo "(no output means success)"

build: build_rust

build_rust:
	find . -type f -name Cargo.toml -execdir cargo build ";"

