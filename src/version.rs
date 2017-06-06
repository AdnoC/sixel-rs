use semver_parser::version::{Version, parse};
use sixel::{LIBSIXEL_VERSION, LIBSIXEL_ABI_VERSION};

use std::str;
use std::ffi::CStr;

fn remove_version_suffix<'a>(version: &'a str) -> &'a str{
    version.trim_right_matches("\u{0}")
}

lazy_static! {
    static ref SIXEL_VERSION_STR: &'static str = {
        let ver_str = CStr::from_bytes_with_nul(LIBSIXEL_VERSION)
            .unwrap()
            .to_str()
            .unwrap();

        remove_version_suffix(ver_str)
    };

    pub static ref SIXEL_VERSION: Version = parse(&SIXEL_VERSION_STR).unwrap();

    static ref SIXEL_ABI_VERSION_STR: String = {
        let ver_str = CStr::from_bytes_with_nul(LIBSIXEL_ABI_VERSION)
            .unwrap()
            .to_str()
            .unwrap();

        let ver_str = ver_str.replace(":", ".").clone();
        remove_version_suffix(&ver_str).to_owned()
    };

    pub static ref SIXEL_ABI_VERSION: Version = parse(&SIXEL_ABI_VERSION_STR).unwrap();
}

#[cfg(test)]
mod tests {
    use semver_parser::version::{Version, parse};
    use sixel::{LIBSIXEL_VERSION, LIBSIXEL_ABI_VERSION};
    use std::str;
    use std::ops::Deref;

    use super::*;

    #[test]
    fn from_utf8_versions() {
        let lib_eq = (&LIBSIXEL_VERSION).iter()
            .zip(SIXEL_VERSION_STR.as_bytes())
            .all(|(a, b)| a == b);
        assert!(lib_eq);

        let abi_eq = (&LIBSIXEL_VERSION).iter()
            .zip(SIXEL_VERSION_STR.as_bytes())
            .all(|(a, b)| a == b);
        assert!(abi_eq);
    }

    #[test]
    fn lib_versions() {
        let lib_version_str = str::from_utf8(LIBSIXEL_VERSION).unwrap();
        // println!("lazy_str: {:?}", *SIXEL_VERSION_STR);
        version_same(&*SIXEL_VERSION, lib_version_str);
    }

    #[test]
    fn abi_versions() {
        let lib_version_str = str::from_utf8(LIBSIXEL_ABI_VERSION).unwrap();
        let lib_version_str = lib_version_str.replace(":", ".");
        // println!("lazy_str: {:?}", *SIXEL_ABI_VERSION_STR);
        version_same(&*SIXEL_ABI_VERSION, &lib_version_str);
    }

    fn version_same<T>(lazy: T, expected: &str) where
         T: Deref<Target=Version>
    {
        let lib_version_str = remove_version_suffix(expected);
        // println!("test_str: {:?}", lib_version_str);
        let lib_version = parse(lib_version_str).unwrap();
        // println!("lazy: {:?}", *lazy);
        // println!("test: {:?}", lib_version);
        assert_eq!(*lazy, lib_version);
    }

}
