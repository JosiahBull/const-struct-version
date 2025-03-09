#![allow(de)]

use const_struct_version::StructVersion;

#[derive(StructVersion)]
enum EnumDiscriminants {
    A = 1,
    B = 2,
}

#[derive(StructVersion)]
enum EnumDiscriminants2 {
    A,
    B,
}

#[derive(StructVersion)]
enum EnumDiscriminants3 {
    A = 2,
    B = 1,
}

fn test_enum_discriminats_changes_hash() {
    let version = <EnumDiscriminants as StructVersion>::version();
    let version2 = <EnumDiscriminants2 as StructVersion>::version();
    let version3 = <EnumDiscriminants3 as StructVersion>::version();

    // Ensure all are different.
    assert_ne!(version, version2);
    assert_ne!(version, version3);
    assert_ne!(version2, version3);
}
