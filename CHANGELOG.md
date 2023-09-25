# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
[markdownlint](https://dlaa.me/markdownlint/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- ...

## [0.4.0] - 2023-09-25

### Changed in 0.4.0

- Remove deprecated functions
  See commented functions in [g2api](https://github.com/Senzing/g2-sdk-go/blob/495326c4451851f9e9a04ac23a150b3d48bc2ad1/g2api/main.go).

## [0.3.3] - 2023-05-26

### Fixed in 0.3.3

- Corrected g2config.Load() signature
- Updated dependencies
  - google.golang.org/protobuf v1.30.0

## [0.3.2] - 2023-03-10

### Fixed in 0.3.2

- Syntax fix

## [0.3.1] - 2023-03-10

### Fixed in 0.3.1

- Syntax fix

## [0.3.0] - 2023-03-09

### Added to 0.3.0

- `StreamExportCSVEntityReport()`
- `StreamExportJSONEntityReport()`

## [0.2.0] - 2023-01-04

### Added to 0.2.0

- Extracted `go` directory from `example_generated_source_code`
- Refactored `example_generated_source_code` directory
- Simplified documentation for generating source code

### Deleted in 0.2.0

- Removed `GetLastException()`, `GetLastExceptionCode()`, and `ClearLastException()` methods

## [0.1.0] - 2022-10-28

### Added to 0.1.0

- `.proto` files matching Senzing 3.3.2
