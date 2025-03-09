#![allow(dead_code)]

use const_struct_version::StructVersion;

#[derive(StructVersion)]
struct TupleStruct(u32, f32);

#[derive(StructVersion)]
struct TupleStruct2(f32, u32);

fn test_tuple_struct_ordering_changes_hash() {
    let version = <TupleStruct as StructVersion>::version();
    let version2 = <TupleStruct2 as StructVersion>::version();

    // Ensure all are different.
    assert_ne!(version, version2);
}
