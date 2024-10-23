#![no_std]

extern crate alloc;

mod de;
mod package;
mod se;

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests;

pub use self::package::{Package, PackageExport, PackageManifest, Rodata};
