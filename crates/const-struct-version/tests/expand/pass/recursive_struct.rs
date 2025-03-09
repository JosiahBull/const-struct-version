use const_struct_version::StructVersion;

#[derive(StructVersion)]
struct TreeNode {
    value: i32,
    children: Vec<Box<TreeNode>>,
}

fn main() {
    let version = <TreeNode as StructVersion>::version();
}
