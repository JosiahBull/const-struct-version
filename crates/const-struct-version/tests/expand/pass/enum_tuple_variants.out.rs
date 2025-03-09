#![cfg_attr(rustfmt, rustfmt_skip)]
use const_struct_version::StructVersion;
enum EnumTupleVariants {
    A(u32),
    B(f32),
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for EnumTupleVariants {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("A");
            hasher.update("0");
            hasher
                .update(
                    <u32 as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            hasher.update("B");
            hasher.update("0");
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
impl EnumTupleVariants {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
enum EnumTupleVariants2 {
    B(f32),
    A(u32),
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for EnumTupleVariants2 {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("B");
            hasher.update("0");
            hasher
                .update(
                    <f32 as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            hasher.update("A");
            hasher.update("0");
            hasher
                .update(
                    <u32 as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl EnumTupleVariants2 {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "main"]
#[doc(hidden)]
pub const main: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("main"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "/workspaces/const-struct-version/crates/const-struct-version/tests/expand/pass/enum_tuple_variants.rs",
        start_line: 16usize,
        start_col: 4usize,
        end_line: 16usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(main())),
};
#[allow(dead_code)]
fn main() {
    let version = <EnumTupleVariants as StructVersion>::version();
    let version2 = <EnumTupleVariants2 as StructVersion>::version();
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
    test::test_main_static(&[&main])
}
