#[cfg(all(feature = "metal", target_arch = "aarch64", target_os = "macos"))]
pub mod metal;

#[cfg(all(feature = "webgpu"))]
pub mod webgpu;
