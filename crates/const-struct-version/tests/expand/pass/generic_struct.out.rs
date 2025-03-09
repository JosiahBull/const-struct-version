#![cfg_attr(rustfmt, rustfmt_skip)]
use const_struct_version::StructVersion;
struct GenericContainer<T> {
    value: T,
    items: Vec<T>,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl<T: StructVersion> _const_struct_version::StructVersion for GenericContainer<T> {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("value");
            hasher
                .update(
                    <T as _const_struct_version::StructVersion>::version().as_bytes(),
                );
            hasher.update("items");
            hasher
                .update(
                    <Vec<T> as _const_struct_version::StructVersion>::version()
                        .as_bytes(),
                );
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl<T: StructVersion> GenericContainer<T> {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
struct ConcreteType {
    data: GenericContainer<u32>,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for ConcreteType {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("data");
            hasher
                .update(
                    <GenericContainer<
                        u32,
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
impl ConcreteType {
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
        source_file: "/workspaces/const-struct-version/crates/const-struct-version/tests/expand/pass/generic_struct.rs",
        start_line: 15usize,
        start_col: 4usize,
        end_line: 15usize,
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
    let version = <ConcreteType as StructVersion>::version();
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&main])
}
