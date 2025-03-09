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

fn main() {
    let version = <ConcreteType as StructVersion>::version();
}
