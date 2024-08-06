# Contribution guidelines

First off, thank you for considering contributing to opencc-rs.

If your contribution is not straightforward, please first discuss the change you wish to make by
creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the [issue tracker](https://github.com/novel-rs/opencc-rs/issues),
please check that it has not already been reported by searching for some related keywords.

## Pull requests

Try to do one pull request per change.

## Commit Message Format

This project adheres to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
A specification for adding human and machine readable meaning to commit messages.

### Commit Message Header

```plain
<type>(<scope>): <short summary>
  │       │             │
  │       │             └─⫸ Summary in present tense. Not capitalized. No period at the end.
  │       │
  │       └─⫸ Commit Scope
  │
  └─⫸ Commit Type: feat|fix|build|ci|docs|perf|refactor|test|chore
```

#### Type

| feat     | Features                 | A new feature                                             |
| -------- | ------------------------ | --------------------------------------------------------- |
| fix      | Bug Fixes                | A bug fix                                                 |
| docs     | Documentation            | Documentation only changes                                |
| style    | Styles                   | Changes that do not affect the meaning of the code\       |
| refactor | Code Refactoring         | A code change that neither fixes a bug nor adds a feature |
| perf     | Performance Improvements | A code change that improves performance                   |
| test     | Tests                    | Adding missing tests or correcting existing tests         |
| chore    | Chores                   | Other changes that don't modify src or test files         |
| revert   | Reverts                  | Reverts a previous commit                                 |

## Developing

This is no different than other Rust projects.

```shell
git clone --recursive https://github.com/novel-rs/opencc-rs
cd opencc-rs
cargo test
```

This project uses [pre-commit](https://github.com/pre-commit/pre-commit), which you should install before committing.

```shell
pre-commit install
```
