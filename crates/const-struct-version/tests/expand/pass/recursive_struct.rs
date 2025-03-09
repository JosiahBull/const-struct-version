#![allow(dead_code)]

use const_struct_version::StructVersion;

#[derive(StructVersion)]
struct TreeNode {
    value: i32,
    #[allow(clippy::vec_box)]
    children: Vec<Box<TreeNode>>,
}

fn test_tree_node_works() {
    let version = <TreeNode as StructVersion>::version();
}
