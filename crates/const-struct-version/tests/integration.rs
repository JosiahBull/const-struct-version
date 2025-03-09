use const_struct_version::StructVersion;

#[allow(dead_code)]
#[test]
fn test_that_different_name_same_hash() {
    #[derive(StructVersion)]
    struct Cart1 {
        value: i32,
    }

    #[derive(StructVersion)]
    struct Cart2 {
        value: i32,
    }

    let version1 = <Cart1 as StructVersion>::version();
    let version2 = <Cart2 as StructVersion>::version();

    assert_eq!(version1, version2);
}
