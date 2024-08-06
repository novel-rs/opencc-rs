# Changelog

All notable changes to this project will be documented in this file.

## [opencc-rs-v0.4.0] - 2024-08-06

### 🚜 Refactor

- Use std::sync::LazyLock

### 📚 Documentation

- Update changelog
- Update changelog
- Update README.md

### 🎨 Styling

- Reformatting the code

### 🧪 Testing

- Use testresult

### ⚙️ Miscellaneous Tasks

- _(ci)_ Remove unnecessary operations
- _(ci)_ Coverage is no longer run during pull_request
- _(ci)_ Fix fmt check
- Upgrade various project files
- Bump OpenCC
- Upgrade various project files
- Update .gitignore
- _(ci)_ Try to fix release-plz failed
- _(ci)_ Try to fix release-plz failed
- _(ci)_ Try to fix release-plz failed

## [opencc-rs-0.3.3] - 2024-04-11

### 🐛 Bug Fixes

- _(ci)_ Use macos-14 instead of macos-latest

### 🚜 Refactor

- Change variable names

### 📚 Documentation

- Update changelog
- Update changelog

### ⚙️ Miscellaneous Tasks

- Add workspace resolver
- Update .pre-commit-config.yaml
- Bump opencc to 1.1.7
- Upgrade various project files
- Update deny.toml
- Upgrade various project files
- _(release)_ Prepare new version
- _(release)_ Prepare new version

## [opencc-rs-0.3.2] - 2023-07-31

### 🚜 Refactor

- Refactor code slightly

### ⚙️ Miscellaneous Tasks

- Update changelog
- _(release)_ Prepare for opencc-sys-0.1.9+1.1.6
- _(release)_ Prepare for opencc-rs-0.3.2

## [opencc-rs-0.3.1] - 2023-07-11

### 📚 Documentation

- Remove an unnecessary TODO
- Add msrv badge

### ⚙️ Miscellaneous Tasks

- _(ci)_ Enable check semver
- Remove redundant install action
- Add git-cliff to generate changelog
- _(ci)_ Remove outdated action
- _(ci)_ Remove semver-checks directory
- Use cargo-nextest
- Update cliff.toml
- Update changelog
- Update cliff.toml
- Update .justfile
- Record minimum supported Rust version in metadata
- Correct incorrect manifest field
- _(release)_ Prepare for opencc-sys-0.1.8+1.1.6
- _(release)_ Prepare for opencc-rs-0.3.1

## [opencc-rs-0.3.0] - 2023-03-04

### 🚀 Features

- Support for cross compilation

### 🐛 Bug Fixes

- Build failed without python env on macos ([#7](https://github.com/novel-rs/opencc-rs/issues/7))

### 🚜 Refactor

- Modify the parameter of OpenCC::new

### 📚 Documentation

- Update changelog
- Update README.md
- Change some comment
- Update changelog

### ⚙️ Miscellaneous Tasks

- Change publish
- Add cargo-semver-checks-action
- Remove outdated action schedule
- Add changelog
- Bump version
- Fix check semver
- Disable check semver
- Enable check semver
- Add cargo-semver-checks install action
- Disable default-features for all crates
- Remove opencc-sys license and README.md
- Add license and README.md for opencc-sys
- Bump version
- Bump version
- Change dependabot schedule to weekly
- _(ci)_ Change version-tag-prefix
- _(ci)_ Try to fix check semver failed
- _(ci)_ Disable check semver

## [opencc-rs-0.2.0] - 2023-01-23

### 🚀 Features

- Allow multiple OpenCC configurations

### 🐛 Bug Fixes

- Wrong opencc-sys version

### 🚜 Refactor

- Use once_cell instead of lazy_static
- Use link-cplusplus

### 🧪 Testing

- Add t2tw test
- Fix wrong line breaks on Windows

### ⚙️ Miscellaneous Tasks

- Update pre-commit config
- Remove rustfmt.toml
- Use stable rustfmt

### Build

- _(deps)_ Bump taiki-e/install-action from 1 to 2 ([#3](https://github.com/novel-rs/opencc-rs/issues/3))

## [opencc-rs-0.1.4] - 2022-12-06

### 🚀 Features

- Initial

### 📚 Documentation

- Add badge
- Add some comment
- Add som doc

### 🧪 Testing

- Add more tests
- Fix wrong package name

### ⚙️ Miscellaneous Tasks

- Update README.md
- Update README.md
- Remove CHANGELOG.md
- Bump version
- Add example
- Change .pre-commit-config.yaml
- Update pre-commot config
- Change publish.yml
- Remove dependabot gitsubmodule
- _(ci)_ Add including branches and ignore coverage failed
- _(ci)_ Add tags-ignore
- Bump version

### Build

- Fix windows build
- Fix windows build
- Add doc link
- Change package name
- Bump version
- Bump version, add README.md and change keywords

### Ci

- Remove --locked
- Fix build failed
- Remove aarch64-apple-darwin target
- Remove unnecessary submodules downloads
- Rename test.yml to build.yml
- Add publish.yml
- Fix publish
- Fix publish
