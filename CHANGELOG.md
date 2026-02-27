# Changelog

## Unreleased

### Added
- `AudioObjectType::LAYER2` (Object Type ID 33, "Layer-2"), which was missing.
- `SamplingFrequencyIndex` type with constants for the 12 standard sampling
  rate indices, a `freq()` method to look up the rate in Hz.
- `ChannelConfiguration` type with constants for channel layouts 0–7
  (mono through 7.1 surround).
