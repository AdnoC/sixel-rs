// use sixel::*;

#[doc(hidden)]
pub type OptflagUnderlying = u8;

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

pub enum BitMode {
    SevenBit,
    EightBit
}
