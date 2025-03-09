#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]
use const_struct_version::StructVersion;
struct TreeNode {
    value: i32,
    #[allow(clippy::vec_box)]
    children: Vec<Box<TreeNode>>,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for TreeNode {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("value");
            hasher
                .update(
                    <i32 as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            hasher.update("children");
            hasher
                .update(
                    <Vec<
                        Box<TreeNode>,
                    > as _const_struct_version::StructVersion>::version()
                        .as_bytes(),
                );
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl TreeNode {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
fn test_tree_node_works() {
    let version = <TreeNode as StructVersion>::version();
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
