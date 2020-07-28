//! Definitions of types and constants for values defined by
//! [ISO/IEC 14496 part 3 (Audio)](https://en.wikipedia.org/wiki/MPEG-4_Part_3).
//!
//! Currently supported,
//!
//!  - [`AudioObjectType`](struct.AudioObjectType.html)

use std::convert::TryFrom;
use std::fmt;

/// Represents an error converting a `u8` into an `AudioObjectType`
#[derive(PartialEq, Debug)]
pub enum AudioObjectTypeError {
    /// Tried to convert the 'escape value', `31`, into an `AudioObjectType` (this is not a legitimate
    /// AOT value but instead is used as part of encoding the field value.
    EscapeValue,
    /// Only values 95 and under can be legitimate Audio Object Types.
    TooLarge(u8),
}

/// Represents an
/// [audio object type](https://en.wikipedia.org/wiki/MPEG-4_Part_3#MPEG-4_Audio_Object_Types)
/// indicator value.
///
/// This type can be constructed from a `u8`,
///
/// ```rust
/// # use mpeg4_audio_const::*;
/// # use std::convert::TryFrom;
/// assert_eq!(AudioObjectType::AAC_LC, AudioObjectType::try_from(2).unwrap());
/// assert_eq!(2u8, AudioObjectType::AAC_LC.into());
/// ```
///
/// and will accept values that are 'reserved' in the spec,
///
/// ```rust
/// # use mpeg4_audio_const::*;
/// # use std::convert::TryFrom;
/// assert_eq!("RESERVED(95)", format!("{:?}", AudioObjectType::try_from(95).unwrap()));
/// ```
///
/// but disallows values that can't legitimately be represented because they are too large
/// (the maximum representable a-o-t value is `96`) and also disallows the 'escape value' (value
/// `31` see [`AOT_ESCAPE_VALUE`](constant.AOT_ESCAPE_VALUE.html)) which is used as part of the
/// encoding scheme for the a-o-t field rather than as a distinct field value.
///
/// ```rust
/// # use mpeg4_audio_const::*;
/// # use std::convert::TryFrom;
/// assert_eq!(Err(AudioObjectTypeError::EscapeValue), AudioObjectType::try_from(31));
/// assert_eq!(Err(AudioObjectTypeError::TooLarge(97)), AudioObjectType::try_from(97));
/// ```
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct AudioObjectType(u8);

/// This value, `31`, is not used as an _audio object type_, but is instead used in the encoding of
/// any _audio object type_ value greater than or equal to `32`.
pub const AOT_ESCAPE_VALUE: u8 = 0b_11111;

impl From<AudioObjectType> for u8 {
    fn from(v: AudioObjectType) -> Self {
        v.0
    }
}
impl TryFrom<u8> for AudioObjectType {
    type Error = AudioObjectTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            AOT_ESCAPE_VALUE => Err(AudioObjectTypeError::EscapeValue),
            96..=255 => Err(AudioObjectTypeError::TooLarge(value)),
            _ => Ok(AudioObjectType(value)),
        }
    }
}

macro_rules! implement_aot {
    (
        $( $tag:literal $id:ident $desc:literal ),* ,
    ) => {

        impl AudioObjectType {
            $(
                #[doc=$desc]
                pub const $id: AudioObjectType = AudioObjectType($tag);
            )*
        }

        impl fmt::Debug for AudioObjectType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    $(
                        $tag => write!(f, "{}({})", stringify!($id), $tag)
                    ),* ,
                    _ => write!(f, "RESERVED({})", self.0),
                }
            }
        }
    }
}

implement_aot! {
    0 NULL "Null",
    1 AAC_MAIN "AAC main",
    3 AAC_SSR "AAC SSR",
    4 AAC_LTP "AAC LTP",
    7 TWIN_VQ "TwinVQ",
    8 CELP "CELP",
    9 HVXC "HVXC",
    5 SBR "SBR",
    6 AAC_SCALABLE "AAC Scalable",
    2 AAC_LC "AAC LC",
    12 TTSI "TTSI",
    13 MAIN_SYNTHETIC "Main synthetic",
    14 WAVETABLE_SYNTHESIS "Wavetable synthesis",
    15 GENERAL_MIDI "General MIDI",
    16 ALGORITHMIC_SYNTHESIS_AND_AUDIO_FX "Algorithmic Synthesis and Audio FX",
    17 ER_AAC_LC "ER AAC LC",
    19 ER_AAC_LTP "ER AAC LTP",
    20 ER_AAC_SCALABLE "ER AAC Scalable",
    21 ER_TWIN_VQ "ER TwinVQ",
    22 ER_BSAC "ER BSAC",
    23 ER_AAC_LD "ER AAC LD",
    24 ER_CELP "ER CELP",
    25 ER_HVXC "ER HVXC",
    26 ER_HILN "ER HILN",
    27 ER_PARAMETRIC "ER Parametric",
    28 SSC "SSC",
    29 PS "PS",
    30 MPEG_SURROUND "MPEG Surround",
    // 31 - 'escape' - deliberately skipped
    32 LAYER1 "Layer-1",
    34 LAYER3 "Layer-3",
    35 DST "DST",
    36 ALS "ALS",
    37 SLS "SLS",
    38 SLS_NON_CORE "SLS non-core",
    39 ER_AAC_ELD "ER AAC ELD",
    40 SMR_SIMPLE "SMR Simple",
    41 SMR_MAIN "SMR Main",
    42 USAC "Unified Speech and Audio Coding",
    43 SAOC "Spatial Audio Object Coding",
    44 LD_MPEG_SURROUND "Low Delay MPEG Surround",
    45 SAOC_DE "Spatial Audio Object Coding Dialogue Enhancement",
    46 AUDIO_SYNC "Audio synchronization tool",
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn escape_value() {
        assert_eq!(
            Err(AudioObjectTypeError::EscapeValue),
            AudioObjectType::try_from(AOT_ESCAPE_VALUE)
        );
    }
}
