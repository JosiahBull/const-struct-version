use const_struct_version::StructVersion;
pub struct PublicStruct {
    pub field: u32,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for PublicStruct {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("field");
            hasher.update(<u32 as _const_struct_version::StructVersion>::version().as_bytes());
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl PublicStruct {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    pub fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
struct PrivateStruct {
    field: u32,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for PrivateStruct {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("field");
            hasher.update(<u32 as _const_struct_version::StructVersion>::version().as_bytes());
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl PrivateStruct {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
pub(crate) struct PublicCrateStruct {
    field: u32,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for PublicCrateStruct {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("field");
            hasher.update(<u32 as _const_struct_version::StructVersion>::version().as_bytes());
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl PublicCrateStruct {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    pub(crate) fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
pub enum PublicEnum {
    Variant1,
    Variant2,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for PublicEnum {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("Variant1");
            hasher.update("Variant2");
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl PublicEnum {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    pub fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
enum PrivateEnum {
    Variant1,
    Variant2,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for PrivateEnum {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("Variant1");
            hasher.update("Variant2");
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl PrivateEnum {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
pub(crate) enum PublicCrateEnum {
    Variant1,
    Variant2,
}
#[doc(hidden)]
const _: () = {
    extern crate const_struct_version as _const_struct_version;
    use _const_struct_version::__private::sha1::Digest as _;
    /// Automatically derived implementation of StructVersion
    #[automatically_derived]
    impl _const_struct_version::StructVersion for PublicCrateEnum {
        fn version() -> String {
            let mut hasher = _const_struct_version::__private::sha1::Sha1::new();
            hasher.update("Variant1");
            hasher.update("Variant2");
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:x}", hasher.finalize()));
                res
            })
        }
    }
};
impl PublicCrateEnum {
    /// Returns a cached version of the structure's hash
    /// This is computed once and stored in a OnceLock for efficient access
    pub(crate) fn version_cached() -> &'static str {
        extern crate const_struct_version as _const_struct_version;
        static VERSION: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        VERSION.get_or_init(|| <Self as _const_struct_version::StructVersion>::version())
    }
}
#[allow(dead_code)]
fn main() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
