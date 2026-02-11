# justfile

default: check

build:
    cargo build --profile dev-release

check:
    cargo fmt -- --check
    cargo clippy -- -D warnings
    cargo test

install:
    cargo install --path .

release:
    cargo build --release
