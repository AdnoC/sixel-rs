extern crate sixel_sys as sixel;
extern crate semver_parser;

#[macro_use]
extern crate lazy_static;

pub mod version;
pub mod status;
pub mod encoder;
mod optflags;
mod msc;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
