#[cfg(all(feature = "metal", target_arch = "aarch64", target_os = "macos"))]
pub mod metal;

#[cfg(all(
    feature = "webgpu",
    any(all(target_arch = "aarch64", target_os = "macos"), target_family = "wasm")
))]
pub mod webgpu;
