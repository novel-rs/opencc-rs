fmt:
    cargo +nightly fmt
    prettier --write .
    taplo fmt *toml
    just --fmt --unstable

update:
    cargo upgrade --incompatible
    cargo update
    cd opencc-sys
    cargo upgrade
    cargo update
    cd ..

check:
    pre-commit run --all-files
    cargo deny --workspace check
    cargo +nightly udeps --workspace --all-targets
    cargo clippy --workspace --all-targets -- --deny clippy::all

build:
    cargo build --workspace --all-targets

test:
    cargo nextest run --workspace --all-targets

changelog:
    git cliff -o CHANGELOG.md
    prettier --write CHANGELOG.md
