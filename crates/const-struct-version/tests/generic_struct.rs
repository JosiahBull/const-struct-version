#![allow(dead_code)]

use const_struct_version::StructVersion;

#[derive(StructVersion)]
struct GenericContainer<T> {
    value: T,
    items: Vec<T>,
}

#[derive(StructVersion)]
struct ConcreteType {
    data: GenericContainer<u32>,
}

#[test]
fn test_generic_struct() {
    let version = <ConcreteType as StructVersion>::version();
    insta::assert_debug_snapshot!(version);
}
