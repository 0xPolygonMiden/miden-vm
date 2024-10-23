use std::{dbg, sync::Arc};

use assembly::Report;

use super::*;

#[ignore = "not implemented"]
#[test]
fn packaging_serialization() {
    let package = example_package().unwrap();
    bitcode::serialize(package.as_ref()).unwrap();
    todo!("do it via roundtrip in proptest")
}

#[ignore = "not implemented"]
#[test]
fn packaging_deserialization() -> Result<(), Report> {
    let _expected = example_package()?;
    todo!("do it via roundtrip in proptest")

    // let mut bytes = vec![];
    // expected
    //     .write_to(&mut bytes, midenc_session::OutputMode::Binary, &context.session)
    //     .into_diagnostic()?;

    // let package = Package::read_from_bytes(bytes)?;

    // assert_eq!(package.name, expected.name);
    // assert_eq!(package.digest, expected.digest);
    // assert_eq!(package.rodata, expected.rodata);
    // assert_eq!(package.manifest, expected.manifest);
    // assert!(package.is_program());

    // // Verify rodata serialization
    // assert!(!package.rodata.is_empty());
    // let expected_rodata_offset = PtrDesc::from_ptr(65536 * 4);
    // let foo_data = package
    //     .rodata
    //     .iter()
    //     .find(|rodata| rodata.start == expected_rodata_offset)
    //     .unwrap();
    // let foo_bytes = foo_data.data.as_slice();

    // let foo_ty = StructType::new([Type::U8, Type::U32, Type::U64]);
    // let offset_u8 = foo_ty.get(0).offset as usize;
    // let offset_u32 = foo_ty.get(1).offset as usize;
    // let offset_u64 = foo_ty.get(2).offset as usize;
    // assert_eq!(foo_bytes[offset_u8], 1);
    // assert_eq!(
    //     u32::from_be_bytes([
    //         foo_bytes[offset_u32],
    //         foo_bytes[offset_u32 + 1],
    //         foo_bytes[offset_u32 + 2],
    //         foo_bytes[offset_u32 + 3]
    //     ]),
    //     2
    // );
    // assert_eq!(
    //     u32::from_be_bytes([
    //         foo_bytes[offset_u64],
    //         foo_bytes[offset_u64 + 1],
    //         foo_bytes[offset_u64 + 2],
    //         foo_bytes[offset_u64 + 3]
    //     ]),
    //     0
    // );
    // assert_eq!(
    //     u32::from_be_bytes([
    //         foo_bytes[offset_u64 + 4],
    //         foo_bytes[offset_u64 + 5],
    //         foo_bytes[offset_u64 + 6],
    //         foo_bytes[offset_u64 + 7]
    //     ]),
    //     3
    // );

    // // Verify the MAST
    // let expected = expected.unwrap_program();
    // let program = package.unwrap_program();
    // assert_eq!(program.hash(), expected.hash());
    // assert_eq!(program.mast_forest(), expected.mast_forest());

    // Ok(())
}

fn example_package() -> Result<Arc<Package>, Report> {
    todo!()
    //     use midenc_hir::ProgramBuilder;

    //     // Build a simple program
    //     let mut builder = ProgramBuilder::new(&context.session.diagnostics);

    //     // Build test module with fib function
    //     let mut mb = builder.module("test");
    //     midenc_hir::testing::fib1(mb.as_mut(), context);

    //     // Ensure we have an example data segment or two to work with
    //     let foo_ty = StructType::new([Type::U8, Type::U32, Type::U64]);
    //     // Initialize the struct with some data
    //     let offset_u8 = foo_ty.get(0).offset as usize;
    //     let offset_u32 = foo_ty.get(1).offset as usize;
    //     let offset_u64 = foo_ty.get(2).offset as usize;
    //     let foo_ty = Type::Struct(foo_ty);
    //     let foo_size = foo_ty.size_in_bytes();
    //     let mut data = vec![0u8; foo_size];
    //     unsafe {
    //         let data_ptr_range = data.as_mut_ptr_range();
    //         core::ptr::write(data_ptr_range.start.byte_add(offset_u8), 1u8);
    //         core::ptr::write(data_ptr_range.start.byte_add(offset_u32).cast(),
    // 2u32.to_be_bytes());         core::ptr::write(data_ptr_range.start.byte_add(offset_u64).
    // cast(), 0u32.to_be_bytes()); // hi bits         core::ptr::write(data_ptr_range.start.
    // byte_add(offset_u64 + 4).cast(), 3u32.to_be_bytes());         // lo bits
    //     }
    //     mb.declare_data_segment(65536 * 4, foo_size as u32, data, true)?;

    //     mb.build().expect("unexpected error constructing test module");

    //     // Link the program
    //     let mut program = builder
    //         .with_entrypoint("test::fib".parse().unwrap())
    //         .link()
    //         .expect("failed to link program");

    //     program.add_library(StdLibrary::default().into());

    //     // Compile the program
    //     let mut compiler = crate::MasmCompiler::new(&context.session);
    //     let program = compiler.compile(program).expect("compilation failed").unwrap_executable();

    //     // Assemble the program
    //     let masm_artifact = MasmArtifact::Executable(program);
    //     let mast_artifact = masm_artifact.assemble(&context.session)?;

    //     // Package the program
    //     Ok(Arc::new(Package::new(mast_artifact, &masm_artifact, &context.session)))
}

#[ignore = "update the binary atfer the changes in the Miden package format are settled"]
#[test]
fn basic_wallet_package_deserialization() {
    // Test for the https://github.com/0xPolygonMiden/compiler/issues/347
    // The included Miden package file is built at
    // https://github.com/0xPolygonMiden/compiler/blob/6cd29e17b34c5abef7f6328c33af06f8bf203344/tests/integration/src/rust_masm_tests/rust_sdk.rs#L48-L63

    let bytes = include_bytes!("../tests/data/basic_wallet.masp");

    let package = Package::read_from_bytes(bytes).unwrap();
    dbg!(&package.manifest);
    assert_eq!(package.name, "basic_wallet");
}
