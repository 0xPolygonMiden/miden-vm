extern crate lalrpop;

use rustc_version::{version_meta, Channel};

fn main() {
    lalrpop::process_root().unwrap();

    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=nightly")
    }
}
