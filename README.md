# mpeg4-audio-const

Definitions of types and constants for values defined by ISO/IEC 14496-3
(MPEG-4 Audio).

## Overview

This crate provides newtype wrappers for the fields found in
`AudioSpecificConfig` and ADTS headers:

 - `AudioObjectType` - AAC profile - Main, LC, SSR, LTP, and others defined by the spec
 - `SamplingFrequencyIndex` - Sampling rate index - 12 standard rates (96 kHz down to 8 kHz)
 - `ChannelConfiguration` - Channel layout - mono through 7.1 surround and beyond

## Usage

```rust
use mpeg4_audio_const::{AudioObjectType, ChannelConfiguration, SamplingFrequencyIndex};
use std::convert::TryFrom;

let profile  = AudioObjectType::try_from(2).unwrap();
assert_eq!(profile, AudioObjectType::AAC_LC);

let freq     = SamplingFrequencyIndex::new(0x3);  // 48 kHz
let channels = ChannelConfiguration::new(0x2); // stereo

assert_eq!(freq.freq(), Some(48000));
assert_eq!(channels, ChannelConfiguration::STEREO);
```

`AudioObjectType` implements `TryFrom<u8>`, rejecting the escape value (31)
and values above 95.
`SamplingFrequencyIndex::new()` is a `const fn` that rejects the escape value
(`0xf`). `SamplingFrequencyIndex::freq()` returns the rate in Hz as
`Option<u32>` (`None` for reserved indices).
`ChannelConfiguration::new()` is a `const fn` that rejects values above `0xf`.

## Field widths by context

ADTS frame headers allow a limited subset of these header values; MP$/M4A allows for a wider field and can therefore
accept a wider range of values

| Type | ADTS          | AudioSpecificConfig (MP4/M4A)                      |
|---|---------------|----------------------------------------------------|
| `AudioObjectType` | 2 bits (0-3)  | 5 bits (1-31, extensible)                          |
| `SamplingFrequencyIndex` | 4 bits (0-15) | 4 bits (0-15; 0xf = explicit 24-bit value follows) |
| `ChannelConfiguration` | 3 bits (0-7)  | 4 bits (0-15)                                      |
