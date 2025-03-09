use const_struct_version::StructVersion;

#[derive(StructVersion, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    #[serde(rename = "userId")]
    id: String,
    login_count: u32,
}

#[derive(StructVersion, serde::Deserialize)]
struct User2 {
    #[serde(rename = "userId")]
    id: String,
    login_count: u32,
}

#[derive(StructVersion, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct User3 {
    id: String,
    login_count: u32,
}

#[test]
fn main() {
    let version = <User as StructVersion>::version();
    let version2 = <User2 as StructVersion>::version();
    let version3 = <User3 as StructVersion>::version();

    // Ensure all are different.
    assert_ne!(version, version2);
    assert_ne!(version, version3);
    assert_ne!(version2, version3);
}
