# Changelog

## 0.6.1 - 2025-03-24

### Fixed

- Fix bug where schemes aren't generated with hashes correctly

## 0.6.0 - 2024-12-20

### Added

- Add basic documentation and tests for `Color` struct

### Changed

- Update distance formula to more directly use the Euclidean distance
  formula

### Fixed

- Add a lightness and saturation adjustment function for >= `base08` and
  increase the lightness if the weighting is too low

## 0.5.0 - 2024-10-06

### Changed

- Use latest `tinted-builder` crate

## 0.4.0 - 2024-09-23

### Added

- Add error fallback for unsupported `SchemeVariant`

### Changed

- Export `System` and `Variant` variants from `tinted-builder` and
  remove enum definitions in `tinted-scheme-extractor` crate

## 0.3.2 - 2024-07-12

### Fixed

- Use latest tinted-builder which wraps all printed scheme properties in
  double quotes

## 0.3.1 - 2024-07-12

### Fixed

- Use latest tinted-builder which also has a `.to_hex` prefix hash fix

## 0.3.0 - 2024-07-12

### Fixed

- Don't return hash when returning value with `.to_hex` method

## 0.2.2 - 2024-06-18

### Fixed

- Update `tinted-builder` package

## 0.2.1 - 2024-06-18

### Fixed

- Update `tinted-builder` package since it includes `Scheme` struct
  `Serialize` bugfix

## 0.2.0 - 2024-06-16

### Added

- Add Base24 support

## 0.1.0 - 2024-06-16

- Initial release
