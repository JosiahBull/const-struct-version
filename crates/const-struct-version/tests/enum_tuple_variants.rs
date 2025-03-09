#![allow(dead_code)]

use const_struct_version::StructVersion;

#[derive(StructVersion)]
enum EnumTupleVariants {
    A(u32),
    B(f32),
}

#[derive(StructVersion)]
enum EnumTupleVariants2 {
    B(f32),
    A(u32),
}

#[test]
fn test_enum_ordering_changes_hash() {
    let version = <EnumTupleVariants as StructVersion>::version();
    let version2 = <EnumTupleVariants2 as StructVersion>::version();

    insta::assert_debug_snapshot!(version);
    insta::assert_debug_snapshot!(version2);

    // Ensure all are different.
    assert_ne!(version, version2);
}
