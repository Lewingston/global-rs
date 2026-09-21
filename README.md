
# global-rs

This crate provides macros for portable storing global static objects on platforms with threading support and without thrading support (e.g. wasm-unknown-unknown).

## How to use

Add the crate to the dependencies in the `cargo.toml`.

```toml
[dependencies]
global = { git = "git@github.com:Lewingston/global-rs.git" }
```

Use the `global::shared` macro to declare the name and the type of the variable.
Use `global::get_or_init` to init the global static variable.

```rust
struct GlobalData {
}

global::shared(GLOBAL, GlobalData);

fn get_global_data() -> std::sync::Arc<GlobalData> {

  global::get_or_init(GLOBAL, create_global_data)
}

fn create_global_data() -> GlobalData {

    GlobalData {}
}
```
