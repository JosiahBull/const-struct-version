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

fn main() {
    let version = <EnumTupleVariants as StructVersion>::version();
    let version2 = <EnumTupleVariants2 as StructVersion>::version();

    // Ensure all are different.
    assert_ne!(version, version2);
}
