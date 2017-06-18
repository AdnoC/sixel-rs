use sixel;

use std::os::raw::c_uint;
use std::os::raw::c_int;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    RGB555,
    RGB565,
    RGB888,
    BGR555,
    BGR565,
    BGR888,
    ARGB8888,
    RGBA8888,
    ABGR8888,
    BGRA8888,
    G1,
    G2,
    G4,
    G8,
    AG88,
    GA88,
    PAL1,
    PAL2,
    PAL4,
    PAL8,
}

impl PixelFormat {
    pub fn to_libsixel(self) -> c_uint {
        use self::PixelFormat::*;
        use sixel::*;

        match self {
            RGB555 => SIXEL_PIXELFORMAT_RGB555,
            RGB565 => SIXEL_PIXELFORMAT_RGB565,
            RGB888 => SIXEL_PIXELFORMAT_RGB888,
            BGR555 => SIXEL_PIXELFORMAT_BGR555,
            BGR565 => SIXEL_PIXELFORMAT_BGR565,
            BGR888 => SIXEL_PIXELFORMAT_BGR888,
            ARGB8888 => SIXEL_PIXELFORMAT_ARGB8888,
            RGBA8888 => SIXEL_PIXELFORMAT_RGBA8888,
            ABGR8888 => SIXEL_PIXELFORMAT_ABGR8888,
            BGRA8888 => SIXEL_PIXELFORMAT_BGRA8888,
            G1 => SIXEL_PIXELFORMAT_G1,
            G2 => SIXEL_PIXELFORMAT_G2,
            G4 => SIXEL_PIXELFORMAT_G4,
            G8 => SIXEL_PIXELFORMAT_G8,
            AG88 => SIXEL_PIXELFORMAT_AG88,
            GA88 => SIXEL_PIXELFORMAT_GA88,
            PAL1 => SIXEL_PIXELFORMAT_PAL1,
            PAL2 => SIXEL_PIXELFORMAT_PAL2,
            PAL4 => SIXEL_PIXELFORMAT_PAL4,
            PAL8 => SIXEL_PIXELFORMAT_PAL8,
        }
    }

    pub fn channels_per_pixel(self) -> c_int {
        unsafe { sixel::sixel_helper_compute_depth(self.to_libsixel() as c_int) }
    }
}

// pub trait Color {
//     fn depth(&self) -> usize;
//
//     fn at(&self, index: usize) -> u8 {
//         assert!(self.depth() < index);
//         self.all()[index]
//     }
//
//     fn set(&mut self, index: usize, val: u8) {
//         assert!(self.depth() < index);
//         self.all_mut()[index] = val;
//     }
//     fn all(&self) -> &[u8];
//
//     fn all_mut(&mut self) -> &mut [u8];
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// #[repr(C)]
// pub struct Color1([u8; 1]);
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// #[repr(C)]
// pub struct Color2([u8; 2]);
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// #[repr(C)]
// pub struct Color3([u8; 3]);
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// #[repr(C)]
// pub struct Color4([u8; 4]);
//
// impl Color for Color1 {
//     fn depth(&self) -> usize {
//         1
//     }
//
//     fn all(&self) -> &[u8] {
//         &self.0
//     }
//
//     fn all_mut(&mut self) -> &mut [u8] {
//         &mut self.0
//     }
// }
// impl Color for Color2 {
//     fn depth(&self) -> usize {
//         2
//     }
//
//     fn all(&self) -> &[u8] {
//         &self.0
//     }
//
//     fn all_mut(&mut self) -> &mut [u8] {
//         &mut self.0
//     }
// }
// impl Color for Color3 {
//     fn depth(&self) -> usize {
//         3
//     }
//
//     fn all(&self) -> &[u8] {
//         &self.0
//     }
//
//     fn all_mut(&mut self) -> &mut [u8] {
//         &mut self.0
//     }
// }
// impl Color for Color4 {
//     fn depth(&self) -> usize {
//         4
//     }
//
//     fn all(&self) -> &[u8] {
//         &self.0
//     }
//
//     fn all_mut(&mut self) -> &mut [u8] {
//         &mut self.0
//     }
// }

// Piston Image library used for reference for this bit. Thanks

pub trait Pixel {
    fn num_channels() -> u8;

    fn channels(&self) -> &[u8];

    fn channels_mut(&mut self) -> &mut [u8];

    fn from_slice<'a>(&'a [u8]) -> &'a Self;

    fn from_slice_mut<'a>(&'a mut [u8]) -> &'a mut Self;
}

macro_rules! define_colors {
    {$(
            $ident: ident,
            $channels: expr;
      )*} => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[repr(C)]
            pub struct $ident {
                pub data: [u8; $channels]
            }

            impl Pixel for $ident {
                fn num_channels() -> u8 {
                    $channels
                }

                fn channels(&self) -> &[u8] {
                    &self.data
                }

                fn channels_mut(&mut self) -> &mut [u8] {
                    &mut self.data
                }

                fn from_slice<'a>(slice: &'a [u8]) -> &'a $ident {
                    use std::mem;
                    assert_eq!(slice.len(), $channels);
                    unsafe {
                        mem::transmute(slice.as_ptr())
                    }
                }

                fn from_slice_mut<'a>(slice: &'a mut [u8]) -> &'a mut $ident {
                    use std::mem;
                    assert_eq!(slice.len(), $channels);
                    unsafe {
                        mem::transmute(slice.as_ptr())
                    }
                }

            }
         )*
    }
}

define_colors!{
    Color1, 1;
    Color2, 2;
    Color3, 3;
    Color4, 4;
}
