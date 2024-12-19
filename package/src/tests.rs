use std::format;

use proptest::{
    prelude::*,
    test_runner::{Config, TestRunner},
};

use super::*;

#[ignore = "update the binary atfer the changes in the Miden package format are settled"]
#[test]
fn basic_wallet_package_deserialization() {
    // Test for the https://github.com/0xPolygonMiden/compiler/issues/347
    // The included Miden package file is built at
    // https://github.com/0xPolygonMiden/compiler/blob/6cd29e17b34c5abef7f6328c33af06f8bf203344/tests/integration/src/rust_masm_tests/rust_sdk.rs#L48-L63

    let bytes = include_bytes!("../tests/data/basic_wallet.masp");

    let package = Package::read_from_bytes(bytes).unwrap();
    // dbg!(&package.manifest);
    assert_eq!(package.name, "basic_wallet");
}

#[test]
fn package_serialization_roundtrip() {
    let cases = 128; // since the test is quite expensive, 128 cases should be enough to cover all edge cases
                     // (default is 256)
    TestRunner::new(Config::with_cases(cases))
        .run(&any::<Package>(), move |package| {
            let bytes = package.write_to_bytes().unwrap();
            let deserialized = Package::read_from_bytes(&bytes).unwrap();
            prop_assert_eq!(package, deserialized);
            Ok(())
        })
        .unwrap();
}
