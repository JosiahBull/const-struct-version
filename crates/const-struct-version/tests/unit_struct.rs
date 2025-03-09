use const_struct_version::StructVersion;

#[derive(StructVersion)]
struct UnitStruct;

#[derive(StructVersion)]
struct UnitStruct2;

#[test]
fn test_unit_structs_give_same_hash() {
    let version = <UnitStruct as StructVersion>::version();
    let version2 = <UnitStruct2 as StructVersion>::version();

    insta::assert_debug_snapshot!(version);

    // Ensure these are the same.
    assert_eq!(version, version2);
}
