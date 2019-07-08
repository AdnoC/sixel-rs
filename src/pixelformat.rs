use sixel::PixelFormat;

use std::os::raw::c_int;

pub trait PixelFormatChan {
    fn channels_per_pixel(self) -> c_int;
}

impl PixelFormatChan for PixelFormat {
    fn channels_per_pixel(self) -> c_int {
        unsafe { sixel::sixel_helper_compute_depth(self) }
    }
}




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
