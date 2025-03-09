use const_struct_version::StructVersion;

#[derive(StructVersion)]
pub struct PublicStruct {
    pub field: u32,
}

#[derive(StructVersion)]
struct PrivateStruct {
    field: u32,
}

#[derive(StructVersion)]
pub(crate) struct PublicCrateStruct {
    field: u32,
}

#[derive(StructVersion)]
pub enum PublicEnum {
    Variant1,
    Variant2,
}

#[derive(StructVersion)]
enum PrivateEnum {
    Variant1,
    Variant2,
}

#[derive(StructVersion)]
pub(crate) enum PublicCrateEnum {
    Variant1,
    Variant2,
}

fn main() {}
