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

fn main() {
    let version = <NestedStruct as StructVersion>::version();
}
