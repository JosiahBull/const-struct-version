#![allow(dead_code)]

use const_struct_version::StructVersion;

#[derive(StructVersion)]
enum EnumUnitVariants {
    A,
    B,
}

#[derive(StructVersion)]
enum EnumUnitVariants2 {
    B,
    A,
}

fn test_enum_unit_variants_changes_hash() {
    let version = <EnumUnitVariants as StructVersion>::version();
    let version2 = <EnumUnitVariants2 as StructVersion>::version();

    // Ensure all are different.
    assert_ne!(version, version2);
}
