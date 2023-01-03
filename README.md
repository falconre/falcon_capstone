![build](https://img.shields.io/github/workflow/status/falconre/falcon_capstone/rust)

# falcon_capstone

This is a fork of Mm7's capstone bindings for Rust, which can be found [here](https://github.com/Mm7/capstone-rust/).

### Usage
- Vendored
```toml
[dependencies]
falcon_capstone = { git = "https://github.com/marirs/falcon_capstone", branch = "master", features = ["vendored"] }
```

- Non Vendored
```toml
[dependencies]
falcon_capstone = { git = "https://github.com/marirs/falcon_capstone", branch = "master" }
```

---
License: LGPL-3