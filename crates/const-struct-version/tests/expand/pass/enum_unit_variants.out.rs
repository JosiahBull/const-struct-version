#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]
use const_struct_version::StructVersion;
enum EnumUnitVariants {
    A,
    B,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for EnumUnitVariants {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("A");
            hasher.update("B");
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl EnumUnitVariants {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
enum EnumUnitVariants2 {
    B,
    A,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for EnumUnitVariants2 {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("B");
            hasher.update("A");
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl EnumUnitVariants2 {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
fn test_enum_unit_variants_changes_hash() {
    let version = <EnumUnitVariants as StructVersion>::version();
    let version2 = <EnumUnitVariants2 as StructVersion>::version();
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
            "const_struct_version_9ddwkljzw5",
            "/workspaces/const-struct-version/crates/const-struct-version/tests/expand/pass/enum_unit_variants.rs",
            21u32,
            "version",
        )
        .unwrap();
    ::insta::_macro_support::assert_snapshot(
            (
                ::insta::_macro_support::AutoName,
                #[allow(clippy::redundant_closure_call)]
                (|v| ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{0:#?}", v));
                    res
                }))(&version2)
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
            "const_struct_version_9ddwkljzw5",
            "/workspaces/const-struct-version/crates/const-struct-version/tests/expand/pass/enum_unit_variants.rs",
            22u32,
            "version2",
        )
        .unwrap();
    match (&version, &version2) {
        (left_val, right_val) => {
            if *left_val == *right_val {
                let kind = ::core::panicking::AssertKind::Ne;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
