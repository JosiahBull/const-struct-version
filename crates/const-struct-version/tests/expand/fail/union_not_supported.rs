use const_struct_version::StructVersion;

#[derive(StructVersion)]
union TestUnion {
    a: u32,
    b: f32,
}

fn main() {}
