extern crate alloc;

/// Instantiates a test with Miden standard library included.
#[macro_export]
macro_rules! build_test {
    ($($params:tt)+) => {{
        let mut test = test_utils::build_test_by_mode!(false, $($params)+);
        test.host_libraries = vec![::alloc::boxed::Box::new(::miden_stdlib::StdLibrary::default())];
        test
    }}
}

/// Instantiates a test in debug mode with Miden standard library included.
#[macro_export]
macro_rules! build_debug_test {
    ($($params:tt)+) => {{
        use ::miden_stdlib::StdLibrary;
        let mut test = test_utils::build_test_by_mode!(true, $($params)+);
        test.host_libraries = vec![::alloc::boxed::Box::new(StdLibrary::default())];
        test
    }}
}

mod collections;
mod crypto;
mod mast_forest_merge;
mod math;
mod mem;
mod sys;
