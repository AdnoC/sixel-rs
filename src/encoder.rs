use sixel::*;
use std::cell::Cell;
use status;
use status::Status;
use optflags;
use std::os::raw::c_int;
use std::os::raw::c_char;
use std::path::Path;

pub struct Encoder {
    encoder: *mut sixel_encoder_t,
}

impl Encoder {
    pub fn new() -> Status<Encoder> {
        use std::ptr;

        let mut encoder: *mut sixel_encoder_t = ptr::null_mut() as *mut _;

        unsafe {
            let result = sixel_encoder_new(&mut encoder, ptr::null_mut() as *mut sixel_allocator_t);

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

    pub fn set_cancel(&self, cancel: Canceller) -> Status<()> {
        let result;
        unsafe {
            result = sixel_encoder_set_cancel_flag(self.encoder, (&cancel.flag).as_ptr());
        }
        status::from_libsixel(result)
    }

    fn set_opt(&self, opt: optflags::OptflagUnderlying, arg: *const c_char) -> Status<()> {
        let result;
        unsafe {
            result = sixel_encoder_setopt(self.encoder, opt as c_int, arg);
        }
        status::from_libsixel(result)
    }

    pub fn set_output(&self, file: &Path) -> Status<()> {
        use msc;

        let cstr = msc::path_to_c_str(file)?;

        self.set_opt(SIXEL_OPTFLAG_OUTPUT, cstr.as_ptr())
    }

    pub fn set_bit_mode(&self, mode: optflags::BitMode) -> Status<()> {
        use std::ptr;
        use optflags::BitMode;

        let mode_flag = match mode {
            BitMode::SevenBit => SIXEL_OPTFLAG_7BIT_MODE,
            BitMode::EightBit => SIXEL_OPTFLAG_8BIT_MODE,
        };

        self.set_opt(mode_flag, ptr::null())
    }

    pub fn enable_gri_arg_limit(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_HAS_GRI_ARG_LIMIT, ptr::null())
    }

    pub fn set_num_colors_str(&self, num_colors: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(num_colors.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(SIXEL_OPTFLAG_COLORS, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_MAPFILE, cstr.as_ptr())
    }

    fn use_monochrome(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_MONOCHROME, ptr::null())
    }

    fn use_high_color(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_HIGH_COLOR, ptr::null())
    }

    fn use_builtin_palette(&self, option: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(option.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(SIXEL_OPTFLAG_BUILTIN_PALETTE, cstr.as_ptr())
    }

    pub fn set_diffusion_str(&self, method: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(method.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(SIXEL_OPTFLAG_DIFFUSION, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_FIND_LARGEST, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_SELECT_COLOR, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_CROP, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_WIDTH, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_HEIGHT, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_RESAMPLING, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_QUALITY, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_LOOPMODE, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_PALETTE_TYPE, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_BGCOLOR, cstr.as_ptr())
    }

    pub fn set_background_color(&self, red: u8, green: u8, blue: u8) -> Status<()> {
        let color_str = format!("#{:0>3}{:0>3}{:0>3}", red, green, blue);

        self.set_background_color_str(&color_str)
    }

    pub fn use_insecure(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_INSECURE, ptr::null())
    }

    pub fn use_invert(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_INVERT, ptr::null())
    }

    pub fn use_macro(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_USE_MACRO, ptr::null())
    }

    pub fn set_macro_number_str(&self, num: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(num.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(SIXEL_OPTFLAG_MACRO_NUMBER, cstr.as_ptr())
    }

    pub fn set_macro_number(&self, num: i64) -> Status<()> {
        let num_str = format!("{}", num);
        self.set_macro_number_str(&num_str)
    }

    pub fn ignore_delay(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_IGNORE_DELAY, ptr::null())
    }

    pub fn use_verbose(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_VERBOSE, ptr::null())
    }

    pub fn use_statuc(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_STATIC, ptr::null())
    }

    pub fn use_penetrate(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_PENETRATE, ptr::null())
    }

    pub fn set_encode_policy_str(&self, pol: &str) -> Status<()> {
        use std::ffi::CString;

        let cstr = match CString::new(pol.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(status::Error::BadArgument),
        };

        self.set_opt(SIXEL_OPTFLAG_ENCODE_POLICY, cstr.as_ptr())
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

        self.set_opt(SIXEL_OPTFLAG_COMPLEXION_SCORE, cstr.as_ptr())
    }

    pub fn set_complexion_score(&self, score: i64) -> Status<()> {
        let score_str = format!("{}", score);
        self.set_complexion_score_str(&score_str)
    }

    pub fn use_pipe_mode(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_PIPE_MODE, ptr::null())
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
    flag: Box<Cell<c_int>>
}

impl Canceller {
    pub fn new() -> Canceller {
        let flag: Box<Cell<c_int>> = Box::new(Cell::new(0));
        // let flag_raw = Box::into_raw(flag);
        // let flag;
        // unsafe {
        //     flag: &'a Cell<c_int> = &*flag_raw;
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
