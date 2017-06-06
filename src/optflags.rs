// use sixel::*;
use std::path::Path;

#[doc(hidden)]
pub type OptflagUnderlying = u8;

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub enum Optflag {
//     Input,
//     Output,
//     Outfile,
//     SevenBitMode,
//     EightBitMode,
//     HasGriArgLimit,
//     Colors,
//     Mapfile,
//     Monochrome,
//     Insecure,
//     Invert,
//     HighColor,
//     UseMacro,
//     MacroNumber,
//     ComplexionScore,
//     IgnoreDelay,
//     Static,
//     Diffusion,
//     FindLargest,
//     SelectColor,
//     Crop,
//     Width,
//     Height,
//     Resampling,
//     Quality,
//     Loopmode,
//     PaletteType,
//     BuiltinPalette,
//     EncodePolicy,
//     Bgcolor,
//     Penetrate,
//     PipeMode,
//     Verbose,
//     Version,
//     Help
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BitMode {
    SevenBit,
    EightBit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinPalette {
    XTerm16,
    XTerm256,
    VT340Mono,
    VT340Color,
    Gray1,
    Gray2,
    Gray4,
    Gray8,
}

impl BuiltinPalette {
    pub fn to_str(self) -> &'static str {
        use self::BuiltinPalette::*;

        match self {
            XTerm16 => "xterm16",
            XTerm256 => "xterm256",
            VT340Mono => "vt340mono",
            VT340Color => "vt340color",
            Gray1 => "gray1",
            Gray2 => "gray2",
            Gray4 => "gray4",
            Gray8 => "gray8",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorOption<'a> {
    Monochrome,
    Builtin(&'a str),
    Mapfile(&'a Path),
    Highcolor,
}

impl<'a> ColorOption<'a> {
    // So that you can select a builtin color option using enums instead of strings
    pub fn builtin_palette(palette: self::BuiltinPalette) -> ColorOption<'static> {
        ColorOption::Builtin(palette.to_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DiffusionMethod {
    Auto,
    None,
    FS,
    Atkinson,
    Jajuni,
    Stucki,
    Burkes,
}

impl DiffusionMethod {
    pub fn to_str(self) -> &'static str {
        use self::DiffusionMethod::*;

        match self {
            Auto => "auto",
            None => "none",
            FS => "fs",
            Atkinson => "atkinson",
            Jajuni => "jajuni",
            Stucki => "stucki",
            Burkes => "burkes",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FindLargestOpt {
    Auto,
    Norm,
    Lum,
}

impl FindLargestOpt {
    pub fn to_str(self) -> &'static str {
        use self::FindLargestOpt::*;

        match self {
            Auto => "auto",
            Norm => "norm",
            Lum => "lum",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorSelectionMethod {
    Auto,
    Center,
    Average,
    Histogram,
}

impl ColorSelectionMethod {
    pub fn to_str(self) -> &'static str {
        use self::ColorSelectionMethod::*;

        match self {
            Auto => "auto",
            Center => "center",
            Average => "average",
            Histogram => "histogram",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SizeSpecification {
    Auto,
    Pixel(u64),
    Percent(u64),
}

impl ToString for SizeSpecification {
    fn to_string(&self) -> String {
        use self::SizeSpecification::*;

        match *self {
            Auto => "auto".to_owned(),
            Pixel(size) => format!("{}px", size),
            Percent(size) => format!("{}%", size),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResampleMethod {
    Nearest,
    Gaussian,
    Hanning,
    Hamming,
    Bilinear,
    Welsh,
    Bicubic,
    Lanczos2,
    Lanczos3,
    Lanczos4,
}

impl ResampleMethod {
    pub fn to_str(self) -> &'static str {
        use self::ResampleMethod::*;

        match self {
            Nearest => "nearest",
            Gaussian => "gaussian",
            Hanning => "hanning",
            Hamming => "hamming",
            Bilinear => "bilinear",
            Welsh => "welsh",
            Bicubic => "bicubic",
            Lanczos2 => "lanczos2",
            Lanczos3 => "lanczos3",
            Lanczos4 => "lanczos4",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Quality {
    Auto,
    High,
    Low,
    Full,
}

impl Quality {
    pub fn to_str(self) -> &'static str {
        use self::Quality::*;

        match self {
            Auto => "auto",
            High => "high",
            Low => "low",
            Full => "full",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LoopMode {
    Auto,
    Force,
    Disable,
}

impl LoopMode {
    pub fn to_str(self) -> &'static str {
        use self::LoopMode::*;

        match self {
            Auto => "auto",
            Force => "force",
            Disable => "disable",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PaletteType {
    Auto,
    HLS,
    RGB,
}

impl PaletteType {
    pub fn to_str(self) -> &'static str {
        use self::PaletteType::*;

        match self {
            Auto => "auto",
            HLS => "hls",
            RGB => "rgb",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EncodePolicy {
    Auto,
    Fast,
    Size,
}

impl EncodePolicy {
    pub fn to_str(self) -> &'static str {
        use self::EncodePolicy::*;

        match self {
            Auto => "auto",
            Fast => "fast",
            Size => "size",
        }
    }
}
