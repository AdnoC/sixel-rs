#![allow(dead_code)]

extern crate sixel_sys as sixel;

pub mod status;
pub mod encoder;
// Should it be pub?
pub mod optflags;
mod msc;
pub mod pixelformat;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
