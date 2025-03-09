use const_struct_version::StructVersion;

// TODO: Eventually I'd like named fields to be sorted by field name, so they are consistent across
// differnet orderings.

#[derive(StructVersion)]
struct NamedFieldsStruct {
    a: u32,
    b: f32,
}

#[derive(StructVersion)]
struct NamedFieldsStruct2 {
    b: f32,
    a: u32,
}

fn main() {
    let version = <NamedFieldsStruct as StructVersion>::version();
    let version2 = <NamedFieldsStruct2 as StructVersion>::version();

    // Ensure all are different.
    assert_ne!(version, version2);
}
