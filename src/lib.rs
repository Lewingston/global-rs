
#[cfg(not(target_arch = "wasm32"))]
mod default;

#[cfg(target_arch =  "wasm32")]
mod wasm32;
