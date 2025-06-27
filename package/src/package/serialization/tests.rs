use std::format;

use miden_core::utils::{Deserializable, Serializable};
use proptest::{
    prelude::*,
    test_runner::{Config, TestRunner},
};

use super::*;

#[test]
fn package_serialization_roundtrip() {
    // since the test is quite expensive, 128 cases should be enough to cover all edge cases
    // (default is 256)
    let cases = 128;
    TestRunner::new(Config::with_cases(cases))
        .run(&any::<Package>(), move |package| {
            let bytes = package.to_bytes();
            let deserialized = Package::read_from_bytes(&bytes).unwrap();
            prop_assert_eq!(package, deserialized);
            Ok(())
        })
        .unwrap();
}
