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

#[test]
fn main() {
    let version = <EnumUnitVariants as StructVersion>::version();
    let version2 = <EnumUnitVariants2 as StructVersion>::version();

    // Ensure all are different.
    assert_ne!(version, version2);
}
