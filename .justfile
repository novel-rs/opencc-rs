update:
    prek update
    cargo upgrade --incompatible
    cargo update
    cd opencc-sys
    cargo upgrade --incompatible
    cargo update
    cd ..

fmt:
    cargo +nightly fmt
    biome format --write .
    taplo fmt *toml
    just --fmt

check:
    prek run --all-files
    biome ci .
    cargo deny --workspace --all-features --locked check
    cargo clippy --workspace --all-targets --all-features --locked -- --deny warnings

test:
    cargo nextest run --workspace --all-targets --all-features --locked
