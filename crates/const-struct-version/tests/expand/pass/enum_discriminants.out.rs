#![cfg_attr(rustfmt, rustfmt_skip)]
use const_struct_version::StructVersion;
enum EnumDiscriminants {
    A = 1,
    B = 2,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for EnumDiscriminants {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("A");
            hasher.update("=1");
            hasher.update("B");
            hasher.update("=2");
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl EnumDiscriminants {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
enum EnumDiscriminants2 {
    A,
    B,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for EnumDiscriminants2 {
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
impl EnumDiscriminants2 {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
enum EnumDiscriminants3 {
    A = 2,
    B = 1,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for EnumDiscriminants3 {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("A");
            hasher.update("=2");
            hasher.update("B");
            hasher.update("=1");
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl EnumDiscriminants3 {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
#[allow(dead_code)]
fn main() {
    let version = <EnumDiscriminants as StructVersion>::version();
    let version2 = <EnumDiscriminants2 as StructVersion>::version();
    let version3 = <EnumDiscriminants3 as StructVersion>::version();
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
    match (&version, &version3) {
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
    match (&version2, &version3) {
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
