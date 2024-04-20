# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### ⚙️ Miscellaneous Tasks

- _(ci)_ Remove unnecessary operations

## [opencc-rs-0.3.3] - 2024-04-11

### 🐛 Bug Fixes

- _(ci)_ Use macos-14 instead of macos-latest

### 🚜 Refactor

- Change variable names

### ⚙️ Miscellaneous Tasks

- Upgrade various project files
- Update deny.toml
- Upgrade various project files
- Bump opencc to 1.1.7
- Update .pre-commit-config.yaml
- Add workspace resolver

## [opencc-rs-0.3.2] - 2023-07-31

### 🚜 Refactor

- Refactor code slightly

### ⚙️ Miscellaneous Tasks

- Update changelog

## [opencc-rs-0.3.1] - 2023-07-11

### 📚 Documentation

- Add msrv badge
- Remove an unnecessary TODO

### ⚙️ Miscellaneous Tasks

- Correct incorrect manifest field
- Record minimum supported Rust version in metadata
- Update .justfile
- Update cliff.toml
- Update changelog
- Update cliff.toml
- Use cargo-nextest
- _(ci)_ Remove semver-checks directory
- _(ci)_ Remove outdated action
- Add git-cliff to generate changelog
- Remove redundant install action
- _(ci)_ Enable check semver

## [opencc-rs-0.3.0] - 2023-03-04

### ⛰️ Features

- Support for cross compilation

### 🐛 Bug Fixes

- Build failed without python env on macos ([#7](https://github.com/novel-rs/opencc-rs/issues/7))

### 🚜 Refactor

- Modify the parameter of OpenCC::new

### 📚 Documentation

- Change some comment
- Update README.md

### ⚙️ Miscellaneous Tasks

- _(ci)_ Disable check semver
- _(ci)_ Try to fix check semver failed
- _(ci)_ Change version-tag-prefix
- Change dependabot schedule to weekly
- Bump version
- Bump version
- Add license and README.md for opencc-sys
- Remove opencc-sys license and README.md
- Disable default-features for all crates
- Add cargo-semver-checks install action
- Enable check semver
- Disable check semver
- Fix check semver
- Bump version
- Add changelog
- Remove outdated action schedule
- Add cargo-semver-checks-action
- Change publish

## [opencc-rs-0.2.0] - 2023-01-23

### ⛰️ Features

- Allow multiple OpenCC configurations

### 🐛 Bug Fixes

- Wrong opencc-sys version

### 🚜 Refactor

- Use link-cplusplus
- Use once_cell instead of lazy_static

### 🧪 Testing

- Fix wrong line breaks on Windows
- Add t2tw test

### ⚙️ Miscellaneous Tasks

- Use stable rustfmt
- Remove rustfmt.toml
- Update pre-commit config

### Build

- _(deps)_ Bump taiki-e/install-action from 1 to 2 ([#3](https://github.com/novel-rs/opencc-rs/issues/3))

## [opencc-rs-0.1.4] - 2022-12-06

### ⛰️ Features

- Initial

### 📚 Documentation

- Add som doc
- Add some comment
- Add badge

### 🧪 Testing

- Fix wrong package name
- Add more tests

### ⚙️ Miscellaneous Tasks

- Bump version
- _(ci)_ Add tags-ignore
- _(ci)_ Add including branches and ignore coverage failed
- Remove dependabot gitsubmodule
- Change publish.yml
- Update pre-commot config
- Change .pre-commit-config.yaml
- Add example
- Bump version
- Remove CHANGELOG.md
- Update README.md
- Update README.md

### Build

- Bump version, add README.md and change keywords
- Bump version
- Change package name
- Add doc link
- Fix windows build
- Fix windows build

### Ci

- Fix publish
- Fix publish
- Add publish.yml
- Rename test.yml to build.yml
- Remove unnecessary submodules downloads
- Remove aarch64-apple-darwin target
- Fix build failed
- Remove --locked
