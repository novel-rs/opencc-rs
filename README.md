# opencc-rs

[![Test](https://github.com/novel-rs/opencc-rs/actions/workflows/test.yml/badge.svg)](https://github.com/novel-rs/opencc-rs/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/novel-rs/opencc-rs/graph/badge.svg?token=96TJ1OIF3P)](https://codecov.io/gh/novel-rs/opencc-rs)
[![docs.rs](https://img.shields.io/docsrs/opencc-rs)](https://docs.rs/opencc-rs)
[![Crates.io](https://img.shields.io/crates/l/opencc-rs)](https://github.com/novel-rs/opencc-rs)
[![Crates.io](https://img.shields.io/crates/v/opencc-rs)](https://crates.io/crates/opencc-rs)

---

OpenCC bindings for Rust

## Platform

- Windows
- Linux
- macOS

## Dependencies

- Clang
- CMake
- Python / Python3

## Build OpenCC

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release \
-DBUILD_TESTING=OFF \
-DBUILD_DOCUMENTATION=OFF \
-DBUILD_SHARED_LIBS=OFF \
-DOPENCC_ENABLE_INSTALL=OFF \
-DENABLE_GTEST=OFF \
-DENABLE_BENCHMARK=OFF \
-DBUILD_OPENCC_JIEBA_PLUGIN=OFF \
-DBUILD_PYTHON=OFF \
-DUSE_SYSTEM_DARTS=OFF \
-DUSE_SYSTEM_GOOGLE_BENCHMARK=OFF \
-DUSE_SYSTEM_GTEST=OFF \
-DUSE_SYSTEM_MARISA=OFF \
-DUSE_SYSTEM_PYBIND11=OFF \
-DUSE_SYSTEM_RAPIDJSON=OFF \
-DUSE_SYSTEM_TCLAP=OFF

cmake --build build --config Release -j16
```

## Contributing

You should read [CONTRIBUTING](https://github.com/novel-rs/opencc-rs/blob/main/CONTRIBUTING.md) first

## License

All the code in this repository is released under **[Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)** and **[MIT license](https://opensource.org/licenses/MIT)**
