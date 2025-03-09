#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]
use const_struct_version::StructVersion;
struct NestedStruct {
    a: u32,
    b: f32,
    c: NestedStruct2,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for NestedStruct {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("a");
            hasher
                .update(
                    <u32 as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            hasher.update("b");
            hasher
                .update(
                    <f32 as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            hasher.update("c");
            hasher
                .update(
                    <NestedStruct2 as _const_struct_version::StructVersion>::version()
                        .as_bytes(),
                );
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl NestedStruct {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
struct NestedStruct2 {
    a: u32,
    b: f32,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for NestedStruct2 {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("a");
            hasher
                .update(
                    <u32 as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            hasher.update("b");
            hasher
                .update(
                    <f32 as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl NestedStruct2 {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
fn test_nested_structs_work() {
    let version = <NestedStruct as StructVersion>::version();
    ::insta::_macro_support::assert_snapshot(
            (
                ::insta::_macro_support::AutoName,
                #[allow(clippy::redundant_closure_call)]
                (|v| ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{0:#?}", v));
                    res
                }))(&version)
                    .as_str(),
            )
                .into(),
            {
                use ::insta::_macro_support::{env, option_env};
                const WORKSPACE_ROOT: ::insta::_macro_support::Workspace = if let Some(
                    root,
                ) = ::core::option::Option::None::<&'static str> {
                    ::insta::_macro_support::Workspace::UseAsIs(root)
                } else {
                    ::insta::_macro_support::Workspace::DetectWithCargo(
                        "/workspaces/const-struct-version/target/tests/const-struct-version_b539mcipdmp",
                    )
                };
                ::insta::_macro_support::get_cargo_workspace(WORKSPACE_ROOT)
            }
                .as_path(),
            {
                fn f() {}
                fn type_name_of_val<T>(_: T) -> &'static str {
                    ::insta::_macro_support::any::type_name::<T>()
                }
                let mut name = type_name_of_val(f).strip_suffix("::f").unwrap_or("");
                while let Some(rest) = name.strip_suffix("::{{closure}}") {
                    name = rest;
                }
                name
            },
            "const_struct_version_r3ar9ntvg7",
            "/workspaces/const-struct-version/crates/const-struct-version/tests/expand/pass/nested_struct.rs",
            20u32,
            "version",
        )
        .unwrap();
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
