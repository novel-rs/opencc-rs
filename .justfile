update:
    prek auto-update
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
    prek run --all-files
    cargo deny --workspace --all-features --locked check
    cargo clippy --workspace --all-targets --all-features --locked -- --deny warnings

test:
    cargo nextest run --workspace --all-targets --all-features --locked
