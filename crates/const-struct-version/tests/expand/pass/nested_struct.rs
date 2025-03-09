#![allow(dead_code)]

use const_struct_version::StructVersion;

#[derive(StructVersion)]
struct NestedStruct {
    a: u32,
    b: f32,
    c: NestedStruct2,
}

#[derive(StructVersion)]
struct NestedStruct2 {
    a: u32,
    b: f32,
}

fn test_nested_structs_work() {
    let version = <NestedStruct as StructVersion>::version();
}
