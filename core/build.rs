fn main() {
    // Needed until we have an MSRV of 1.80+
    println!("cargo::rustc-check-cfg=cfg(loom)");
}
