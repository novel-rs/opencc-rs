update:
    cargo upgrade --incompatible
    cargo update
    cd opencc-sys
    cargo upgrade --incompatible
    cargo update
    cd ..

fmt:
    cargo +nightly fmt
    prettier --write .
    taplo fmt *toml
    just --fmt --unstable

check:
    pre-commit run --all-files
    cargo deny --workspace --all-features check
    cargo clippy --workspace --all-targets --all-features -- --deny warnings

test:
    cargo nextest run --workspace --all-targets --all-features --locked
