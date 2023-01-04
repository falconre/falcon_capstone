![build](https://img.shields.io/github/workflow/status/falconre/falcon_capstone/rust)

# falcon_capstone

This is a fork of Mm7's capstone bindings for Rust, which can be found [here](https://github.com/Mm7/capstone-rust/).

### Requirements
- Rust edition 2021 (1.56+)

### Usage
#### Vendored

In Cargo.toml include
```toml
[dependencies]
falcon_capstone = { git = "https://github.com/marirs/falcon_capstone", branch = "master", features = ["vendored"] }
```

#### Non Vendored

#### Requirements for non vendored compile
- Libcapstone
  - linux: apt-get install -y libcapstone-dev
  - mac: brew install capstone
- Clang/LLVM

In Cargo.toml include
```toml
[dependencies]
falcon_capstone = { git = "https://github.com/marirs/falcon_capstone", branch = "master" }
```

#### Troubleshooting
For any reason if its not getting compiled in macOS complaining about `capstone/capstone.h not found` although you have it, compile using the following method:
```bash
CPATH="/usr/local/include" LIBRARY_PATH="/usr/local/lib" cargo b
```
assuming that `/usr/local/include` has the header files of capstone and `/usr/local/lib` having the capstone lib files.
This is because CLANG in macOS has its `include` and `lib` path in a different folder location which might not be in the search path.

---
License: LGPL-3