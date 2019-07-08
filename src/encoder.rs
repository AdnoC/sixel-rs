use sixel::*;
use status;
use optflags;
use pixelformat::{Pixel, PixelFormatChan};

use std::cell::Cell;
use status::Status;
use std::os::raw;
use std::path::Path;

pub struct Encoder {
    encoder: *mut sixel::Encoder,
}

impl Encoder {
    pub fn new() -> Status<Encoder> {
        use std::ptr;

        let mut encoder: *mut sixel::Encoder  = ptr::null_mut() as *mut _;

        unsafe {
            let result = sixel_encoder_new(&mut encoder,
                                           ptr::null_mut() as *mut Allocator);

            status::from_libsixel(result)?;
        }


        Ok(Encoder { encoder })
    }

    #[deprecated]
    pub fn create() -> Encoder {
        let encoder;
        unsafe {
            encoder = sixel_encoder_create();
        }

        Encoder { encoder }

    }

    pub fn encode_file(&self, source: &Path) -> Status<()> {
        use msc;

        let cstr = msc::path_to_c_str(source)?;

        let result = unsafe { sixel_encoder_encode(self.encoder, cstr.as_ptr()) };
        status::from_libsixel(result)
    }

    pub fn encode_bytes(&self, frame: QuickFrame, palette: Vec<crate::pixelformat::Color3>) -> Status<()> {
        use std::os::raw::c_int;
        use std::os::raw::c_uchar;

        {
            let frame_size = frame.height * frame.width;
            let frame_size = frame_size * frame.format.channels_per_pixel() as usize;
            let palette_size = palette.len() * frame.format.channels_per_pixel() as usize;
            assert_eq!(frame_size, palette_size);
        }

        let result = unsafe {
            sixel_encoder_encode_bytes(self.encoder,
                                       frame.pixels.as_ptr() as *mut c_uchar,
                                       frame.width as c_int,
                                       frame.height as c_int,
                                       frame.format,
                                       palette.as_ptr() as *mut c_uchar,
                                       palette.len() as c_int)
        };
        status::from_libsixel(result)
    }
}

// Optflags
impl Encoder {
    pub fn set_cancel(&self, cancel: Canceller) -> Status<()> {
        let result =
            unsafe { sixel_encoder_set_cancel_flag(self.encoder, (&cancel.flag).as_ptr()) };
        status::from_libsixel(result)
    }

    fn set_opt(&self, opt: Optflag, arg: *const raw::c_char) -> Status<()> {
        let result = unsafe { sixel_encoder_setopt(self.encoder, opt, arg) };
        status::from_libsixel(result)
    }

    pub fn set_output(&self, file: &Path) -> Status<()> {
        use msc;

        let cstr = msc::path_to_c_str(file)?;

        self.set_opt(Optflag::OutFile, cstr.as_ptr())
    }

    pub fn set_bit_mode(&self, mode: optflags::BitMode) -> Status<()> {
        use std::ptr;
        use optflags::BitMode;

        let mode_flag = match mode {
            BitMode::SevenBit => Optflag::UseSevenBitMode,
            BitMode::EightBit => Optflag::UseEightBitMode,
        };

        self.set_opt(mode_flag, ptr::null())
    }

    pub fn enable_gri_arg_limit(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::HasGRIArgLimit, ptr::null())
    }

    pub fn set_num_colors_str(&self, num_colors: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(num_colors.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::NumColors, cstr.as_ptr())
    }

    // Calls Encoder::set_colors, but allocates a new String
    pub fn set_num_colors(&self, num_colors: u8) -> Status<()> {
        self.set_num_colors_str(&num_colors.to_string())
    }

    pub fn set_color_option<'a>(&self, option: optflags::ColorOption<'a>) -> Status<()> {
        use optflags::ColorOption::*;
        match option {
            Monochrome => self.use_monochrome(),
            Builtin(palette) => self.use_builtin_palette(palette),
            Mapfile(file) => self.use_mapfile(file),
            Highcolor => self.use_high_color(),

        }
    }

    fn use_mapfile(&self, file: &Path) -> Status<()> {
        use msc;

        let cstr = msc::path_to_c_str(file)?;

        self.set_opt(Optflag::Mapfile, cstr.as_ptr())
    }

    fn use_monochrome(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::Monochrome, ptr::null())
    }

    fn use_high_color(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::UseHighColor, ptr::null())
    }

    fn use_builtin_palette(&self, option: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(option.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::BuiltinPalette, cstr.as_ptr())
    }

    pub fn set_diffusion_str(&self, method: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(method.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::Diffusion, cstr.as_ptr())
    }

    pub fn set_diffusion(&self, method: optflags::DiffusionMethod) -> Status<()> {
        self.set_diffusion_str(method.to_str())
    }

    pub fn set_find_largest_str(&self, option: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(option.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::FindLargest, cstr.as_ptr())
    }

    pub fn set_find_largest(&self, opt: optflags::FindLargestOpt) -> Status<()> {
        self.set_find_largest_str(opt.to_str())
    }

    pub fn set_color_select_str(&self, opt: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(opt.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::SelectColor, cstr.as_ptr())
    }

    pub fn set_color_select(&self, meth: optflags::ColorSelectionMethod) -> Status<()> {
        self.set_color_select_str(meth.to_str())
    }

    pub fn set_crop_str(&self, crop: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(crop.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::CropRegion, cstr.as_ptr())
    }

    pub fn set_crop(&self, width: i64, height: i64, x: i64, y: i64) -> Status<()> {
        let crop_str = format!("{}x{}+{}+{}", width, height, x, y);
        self.set_crop_str(&crop_str)
    }

    pub fn set_width_str(&self, width: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(width.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::Width, cstr.as_ptr())
    }

    pub fn set_width(&self, width: optflags::SizeSpecification) -> Status<()> {
        self.set_width_str(&width.to_string())
    }

    pub fn set_height_str(&self, height: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(height.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::Height, cstr.as_ptr())
    }

    pub fn set_height(&self, height: optflags::SizeSpecification) -> Status<()> {
        self.set_height_str(&height.to_string())
    }

    pub fn set_resampling_str(&self, meth: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(meth.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::Resampling, cstr.as_ptr())
    }

    pub fn set_resampling(&self, meth: optflags::ResampleMethod) -> Status<()> {
        self.set_resampling_str(meth.to_str())
    }

    pub fn set_quality_str(&self, opt: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(opt.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::QualityMode, cstr.as_ptr())
    }

    pub fn set_quality(&self, opt: optflags::Quality) -> Status<()> {
        self.set_quality_str(opt.to_str())
    }

    pub fn set_loopmode_str(&self, mode: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(mode.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::LoopMode, cstr.as_ptr())
    }

    pub fn set_loopmode(&self, mode: optflags::LoopMode) -> Status<()> {
        self.set_loopmode_str(mode.to_str())
    }

    pub fn set_palette_type_str(&self, opt: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(opt.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::PaletteType, cstr.as_ptr())
    }

    pub fn set_palette_type(&self, opt: optflags::PaletteType) -> Status<()> {
        self.set_palette_type_str(opt.to_str())
    }

    pub fn set_background_color_str(&self, color: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(color.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::BackgroundColor, cstr.as_ptr())
    }

    pub fn set_background_color(&self, red: u8, green: u8, blue: u8) -> Status<()> {
        let color_str = format!("#{:0>3}{:0>3}{:0>3}", red, green, blue);

        self.set_background_color_str(&color_str)
    }

    pub fn use_insecure(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::Insecure, ptr::null())
    }

    pub fn use_invert(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::InvertBackground, ptr::null())
    }

    pub fn use_macro(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::UseMacro, ptr::null())
    }

    pub fn set_macro_number_str(&self, num: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(num.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::UseMacro, cstr.as_ptr())
    }

    pub fn set_macro_number(&self, num: i64) -> Status<()> {
        let num_str = format!("{}", num);
        self.set_macro_number_str(&num_str)
    }

    pub fn ignore_delay(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::IgnoreGIFDelay, ptr::null())
    }

    pub fn use_verbose(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::Verbose, ptr::null())
    }

    pub fn use_static(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::StaticGIF, ptr::null())
    }

    pub fn use_penetrate(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::PenetrateScreen, ptr::null())
    }

    pub fn set_encode_policy_str(&self, pol: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(pol.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::EncodingPolicy, cstr.as_ptr())
    }

    pub fn set_encode_policy(&self, pol: optflags::EncodePolicy) -> Status<()> {
        self.set_encode_policy_str(pol.to_str())
    }

    pub fn set_complexion_score_str(&self, score: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(score.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(Optflag::ComplexionScore, cstr.as_ptr())
    }

    pub fn set_complexion_score(&self, score: i64) -> Status<()> {
        let score_str = format!("{}", score);
        self.set_complexion_score_str(&score_str)
    }

    pub fn use_pipe_mode(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(Optflag::PipeInput, ptr::null())
    }
}

impl Clone for Encoder {
    fn clone(&self) -> Encoder {
        unsafe {
            sixel_encoder_ref(self.encoder);
        }

        Encoder { encoder: self.encoder }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            sixel_encoder_unref(self.encoder);
        }
    }
}


// TODO: Get working with stack values
pub struct Canceller {
    flag: Box<Cell<raw::c_int>>,
}

impl Canceller {
    pub fn new() -> Canceller {
        let flag: Box<Cell<raw::c_int>> = Box::new(Cell::new(0));
        // let flag_raw = Box::into_raw(flag);
        // let flag;
        // unsafe {
        //     flag: &'a Cell<raw::c_int> = &*flag_raw;
        // }

        Canceller { flag }
    }

    pub fn cancel(&self) {
        self.flag.set(1);
        // unsafe {
        //     // *self.flag = 1;
        // }
    }

    pub fn reset(&self) {
        self.flag.set(0);
        // unsafe {
        //     // *self.flag = 0;
        // }
    }
}

// impl<'a> Drop for Canceller<'a> {
//     fn drop(&mut self) {
//         unsafe {
//             Box::from_raw(self.flag.as_ptr());
//         }
//     }
// }

pub struct QuickFrameBuilder {
    width: usize,
    height: usize,
    format: PixelFormat,
}

impl QuickFrameBuilder {
    pub fn new() -> QuickFrameBuilder {
        QuickFrameBuilder {
            width: 0,
            height: 0,
            format: PixelFormat::RGB888,
        }
    }

    pub fn width(mut self, width: usize) -> QuickFrameBuilder {
        self.width = width;
        self
    }
    pub fn height(mut self, height: usize) -> QuickFrameBuilder {
        self.height = height;
        self
    }
    pub fn format(mut self, format: PixelFormat) -> QuickFrameBuilder {
        self.format = format;
        self
    }

    pub fn finalize(self) -> QuickFrame {
        let depth = self.format.channels_per_pixel() as usize;
        let size = self.width * self.height * depth;
        let pixels: Vec<u8> = Vec::with_capacity(size);

        QuickFrame {
            width: self.width,
            height: self.height,
            format: self.format,
            pixels,
        }
    }
    pub fn pixels(self, pixels: Vec<u8>) -> QuickFrame {
        let depth = self.format.channels_per_pixel() as usize;
        let size = self.width * self.height * depth;

        assert_eq!(size, pixels.len());

        QuickFrame {
            width: self.width,
            height: self.height,
            format: self.format,
            pixels,
        }
    }
}

pub struct QuickFrame {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    format: PixelFormat,
}

impl QuickFrame {
    pub fn row(&self, row: usize) -> &[u8] {
        let row_len = self.width * self.format.channels_per_pixel() as usize;

        let row_start = (row - 1) * row_len;
        let next_row_start = row * row_len;

        &self.pixels[row_start..next_row_start]
    }
    pub fn row_mut(&mut self, row: usize) -> &mut [u8] {
        let row_len = self.width * self.format.channels_per_pixel() as usize;

        let row_start = (row - 1) * row_len;
        let next_row_start = row * row_len;

        &mut self.pixels[row_start..next_row_start]
    }

    pub fn pixel(&self, row: usize, column: usize) -> &[u8] {
        let depth = self.format.channels_per_pixel() as usize;
        let row_len = self.width * depth;

        let row_start = (row - 1) * row_len;
        let col_pos = column * depth;

        let start = row_start + col_pos;
        let end = start + depth;

        &self.pixels[start..end]
    }

    pub fn pixel_mut(&mut self, row: usize, column: usize) -> &mut [u8] {
        let depth = self.format.channels_per_pixel() as usize;
        let row_len = self.width * depth;

        let row_start = (row - 1) * row_len;
        let col_pos = column * depth;

        let start = row_start + col_pos;
        let end = start + depth;

        &mut self.pixels[start..end]
    }

    pub fn color(&self, row: usize, column: usize, depth: usize) -> u8 {
        let pix_depth = self.format.channels_per_pixel() as usize;

        assert!(depth < pix_depth,
                "Gave a depth of {} when a pixel is only {} bytes deep",
                depth,
                pix_depth);

        let row_len = self.width * pix_depth;

        let row_start = (row - 1) * row_len;
        let col_pos = column * pix_depth;

        self.pixels[row_start + col_pos + depth]
    }

    pub fn set_color(&mut self, row: usize, column: usize, depth: usize, color: u8) {
        let pix_depth = self.format.channels_per_pixel() as usize;

        assert!(depth < pix_depth,
                "Gave a depth of {} when a pixel is only {} bytes deep",
                depth,
                pix_depth);

        let row_len = self.width * pix_depth;

        let row_start = (row - 1) * row_len;
        let col_pos = column * pix_depth;

        self.pixels[row_start + col_pos + depth] = color;
    }
}
