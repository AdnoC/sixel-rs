use sixel::*;
use status;
use status::Status;
use optflags;
use std::os::raw::c_int;
use std::os::raw::c_char;
use std::path::Path;

pub struct Encoder {
    encoder: *mut sixel_encoder_t
}

// TODO: Get working with stack values
// Drop impl means it won't work as-is
pub struct Canceller {
    flag: *mut c_int
}

impl Canceller {
    pub fn new() -> Canceller {
        let flag: Box<c_int> = Box::new(0);
        let flag = Box::into_raw(flag);
        
        Canceller { flag }
    }

    pub fn cancel(&self) {
        unsafe { *self.flag = 1; }
    }

    pub fn reset(&self) {
        unsafe { *self.flag = 0; }
    }
}

impl Drop for Canceller {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.flag);
        }
    }
}

impl Encoder {

    pub fn new() -> Status<Encoder> {
        use std::ptr;

        let mut encoder: *mut sixel_encoder_t = ptr::null_mut() as *mut _;

        unsafe {
            let result = sixel_encoder_new(&mut encoder,
                                           ptr::null_mut() as *mut sixel_allocator_t);

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
            result = sixel_encoder_set_cancel_flag(self.encoder, cancel.flag);
        }
        status::from_libsixel(result)
    }

    fn set_opt(&self, opt: optflags::OptflagUnderlying, arg: *const c_char) -> Status<()> {
        let result;
        unsafe {
            result = sixel_encoder_setopt(self.encoder,
                                          opt as c_int,
                                          arg);
        }
        status::from_libsixel(result)
    }

    pub fn set_output(&self, file: &Path) -> Status<()>{
        use msc;

        let cstr = msc::path_to_c_str(file)?;

        self.set_opt(SIXEL_OPTFLAG_OUTPUT, cstr.as_ptr())
    }

    pub fn set_bit_mode(&self, mode: optflags::BitMode) -> Status<()> {
        use std::ptr;
        use optflags::BitMode;

        let mode_flag = match mode {
            BitMode::SevenBit => SIXEL_OPTFLAG_7BIT_MODE,
            BitMode::EightBit => SIXEL_OPTFLAG_8BIT_MODE
        };

        self.set_opt(mode_flag, ptr::null())
    }

    pub fn enable_gri_arg_limit(&self) -> Status<()> {
        use std::ptr;

        self.set_opt(SIXEL_OPTFLAG_HAS_GRI_ARG_LIMIT, ptr::null())
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

// type OptflagRaw = u8;

// pub enum ColorOption {
//     Default,
//     Monochrome,
//     Builtin,
//     Mapfile,
//     Highcolor
// }

// impl ColorOption {
//     #[doc(hidden)]
//     pub fn from_sixel(opt: OptflagRaw) -> ColorOption {
//         match opt {
//             SIXEL_OPT
//         }
//     }
//
// }
