//! Definitions of types and constants for values defined by
//! [ISO/IEC 14496 part 3 (Audio)](https://en.wikipedia.org/wiki/MPEG-4_Part_3).
//!
//! Currently supported,
//!
//!  - [`AudioObjectType`](struct.AudioObjectType.html)
//!  - [`SamplingFrequencyIndex`](struct.SamplingFrequencyIndex.html)
//!  - [`ChannelConfiguration`](struct.ChannelConfiguration.html)

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
    33 LAYER2 "Layer-2",
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

/// A 4-bit sampling frequency index as defined in ISO 14496-3.
///
/// Indices 0x0–0xb map to the 12 standard sampling rates (96 kHz down to
/// 8 kHz). Indices 0xc–0xe are reserved. Index 0xf is excluded from this
/// type as it signals that an explicit 24-bit frequency value follows in the
/// bitstream instead.
///
/// Use [`freq`](Self::freq) to look up the sampling rate in Hz, which returns
/// `None` for reserved indices.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SamplingFrequencyIndex(u8);

impl SamplingFrequencyIndex {
    /// 96 kHz (index 0x0).
    pub const FREQ_96000: Self = Self(0x0);
    /// 88.2 kHz (index 0x1).
    pub const FREQ_88200: Self = Self(0x1);
    /// 64 kHz (index 0x2).
    pub const FREQ_64000: Self = Self(0x2);
    /// 48 kHz (index 0x3).
    pub const FREQ_48000: Self = Self(0x3);
    /// 44.1 kHz (index 0x4).
    pub const FREQ_44100: Self = Self(0x4);
    /// 32 kHz (index 0x5).
    pub const FREQ_32000: Self = Self(0x5);
    /// 24 kHz (index 0x6).
    pub const FREQ_24000: Self = Self(0x6);
    /// 22.05 kHz (index 0x7).
    pub const FREQ_22050: Self = Self(0x7);
    /// 16 kHz (index 0x8).
    pub const FREQ_16000: Self = Self(0x8);
    /// 12 kHz (index 0x9).
    pub const FREQ_12000: Self = Self(0x9);
    /// 11.025 kHz (index 0xa).
    pub const FREQ_11025: Self = Self(0xa);
    /// 8 kHz (index 0xb).
    pub const FREQ_8000: Self = Self(0xb);

    /// Creates a `SamplingFrequencyIndex` from a 4-bit value.
    ///
    /// Panics if `value` is `0xf` (the escape value) or greater than `0xe`.
    /// In const context, an invalid value produces a compile-time error.
    pub const fn new(value: u8) -> Self {
        assert!(value <= 0xe, "SamplingFrequencyIndex: 0xf is the escape value, not a frequency index");
        Self(value)
    }

    /// Returns the sampling rate in Hz, or `None` if the index is reserved
    /// or not yet defined.
    pub fn freq(&self) -> Option<u32> {
        match self.0 {
            0x0 => Some(96000),
            0x1 => Some(88200),
            0x2 => Some(64000),
            0x3 => Some(48000),
            0x4 => Some(44100),
            0x5 => Some(32000),
            0x6 => Some(24000),
            0x7 => Some(22050),
            0x8 => Some(16000),
            0x9 => Some(12000),
            0xa => Some(11025),
            0xb => Some(8000),
            _ => None,
        }
    }
}

/// A channel configuration as defined in ISO 14496-3.
///
/// In ADTS headers the `channel_configuration` field is 3 bits, covering only
/// values 0–7. In `AudioSpecificConfig` (MP4/M4A) the field is 4 bits (0–15),
/// with values 8–14 defined by later amendments to ISO 14496-3.
///
/// This crate provides constants for the original 8 configurations (0–7);
/// callers can define additional constants for the extended layouts as needed.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChannelConfiguration(u8);

impl ChannelConfiguration {
    /// Channel configuration defined by the audio object type specific config.
    pub const OBJECT_TYPE_SPECIFIC_CONFIG: Self = Self(0x0);
    /// Mono (1 channel).
    pub const MONO: Self = Self(0x1);
    /// Stereo (2 channels).
    pub const STEREO: Self = Self(0x2);
    /// 3 channels.
    pub const THREE: Self = Self(0x3);
    /// 4 channels.
    pub const FOUR: Self = Self(0x4);
    /// 5 channels.
    pub const FIVE: Self = Self(0x5);
    /// 5.1 surround (6 channels).
    pub const FIVE_ONE: Self = Self(0x6);
    /// 7.1 surround (8 channels).
    pub const SEVEN_ONE: Self = Self(0x7);

    /// Creates a `ChannelConfiguration` from a 4-bit value.
    ///
    /// Panics if `value` is greater than `0xf`.
    /// In const context, an invalid value produces a compile-time error.
    pub const fn new(value: u8) -> Self {
        assert!(value <= 0xf, "ChannelConfiguration: expected a 4 bit value");
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn aot_escape_value() {
        assert_eq!(
            Err(AudioObjectTypeError::EscapeValue),
            AudioObjectType::try_from(AOT_ESCAPE_VALUE)
        );
    }

    #[test]
    fn aot_too_large() {
        assert_eq!(
            Err(AudioObjectTypeError::TooLarge(96)),
            AudioObjectType::try_from(96)
        );
    }

    #[test]
    fn aot_valid() {
        assert_eq!(AudioObjectType::AAC_LC, AudioObjectType::try_from(2).unwrap());
        assert_eq!(2u8, AudioObjectType::AAC_LC.into());
    }

    #[test]
    fn sampling_frequency_known_indices() {
        assert_eq!(SamplingFrequencyIndex::FREQ_96000.freq(), Some(96000));
        assert_eq!(SamplingFrequencyIndex::FREQ_88200.freq(), Some(88200));
        assert_eq!(SamplingFrequencyIndex::FREQ_64000.freq(), Some(64000));
        assert_eq!(SamplingFrequencyIndex::FREQ_48000.freq(), Some(48000));
        assert_eq!(SamplingFrequencyIndex::FREQ_44100.freq(), Some(44100));
        assert_eq!(SamplingFrequencyIndex::FREQ_32000.freq(), Some(32000));
        assert_eq!(SamplingFrequencyIndex::FREQ_24000.freq(), Some(24000));
        assert_eq!(SamplingFrequencyIndex::FREQ_22050.freq(), Some(22050));
        assert_eq!(SamplingFrequencyIndex::FREQ_16000.freq(), Some(16000));
        assert_eq!(SamplingFrequencyIndex::FREQ_12000.freq(), Some(12000));
        assert_eq!(SamplingFrequencyIndex::FREQ_11025.freq(), Some(11025));
        assert_eq!(SamplingFrequencyIndex::FREQ_8000.freq(), Some(8000));
    }

    #[test]
    fn sampling_frequency_reserved_indices() {
        assert_eq!(SamplingFrequencyIndex::new(0xc).freq(), None);
        assert_eq!(SamplingFrequencyIndex::new(0xd).freq(), None);
        assert_eq!(SamplingFrequencyIndex::new(0xe).freq(), None);
    }

    #[test]
    fn sampling_frequency_new_valid() {
        assert_eq!(SamplingFrequencyIndex::new(0x3), SamplingFrequencyIndex::FREQ_48000);
    }

    #[test]
    #[should_panic]
    fn sampling_frequency_new_escape() {
        SamplingFrequencyIndex::new(0xf);
    }

    #[test]
    #[should_panic]
    fn sampling_frequency_new_too_large() {
        SamplingFrequencyIndex::new(0x10);
    }

    #[test]
    fn channel_configuration_valid() {
        assert_eq!(ChannelConfiguration::MONO, ChannelConfiguration::new(1));
        assert_eq!(ChannelConfiguration::STEREO, ChannelConfiguration::new(2));
        assert_eq!(ChannelConfiguration::FIVE_ONE, ChannelConfiguration::new(6));
        assert_eq!(ChannelConfiguration::SEVEN_ONE, ChannelConfiguration::new(7));
    }

    #[test]
    fn channel_configuration_reserved() {
        // Values 8-15 are valid 4-bit values, just not assigned constants
        let _ = ChannelConfiguration::new(0xf);
    }

    #[test]
    #[should_panic]
    fn channel_configuration_too_large() {
        ChannelConfiguration::new(0x10);
    }
}
