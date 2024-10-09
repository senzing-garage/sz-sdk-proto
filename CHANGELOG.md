# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], [markdownlint],
and this project adheres to [Semantic Versioning].

## [Unreleased]

- ...

## [0.7.9] - 2024-10-08

### Changed in 0.7.9

- Updated dependencies
- Regenerated language stubs

## [0.7.8] - 2024-10-01

### Added in 0.7.8

- `PreprocessRecord`

## [0.7.7] - 2024-08-30

### Changed in 0.7.7

- From `BuildOutDegree` to `BuildOutDegrees`

## [0.7.6] - 2024-06-14

### Changed in 0.7.6

- From `GetConfigList` to `GetConfigs`
- From `RecordList` to `RecordKeys`
- From exclusions to avoidance

## [0.7.5] - 2024-05-08

### Added in 0.7.5

- `GetFeature`

## [0.7.4] - 2024-05-07

### Added in 0.7.4

- `FindInterestingEntitiesByEntityId`
- `FindInterestingEntitiesByRecordId`

## [0.7.3] - 2024-05-03

### Deleted in 0.7.3

- `GetRepositoryLastModifiedTime`

## [0.7.2] - 2024-04-26

### Changed in 0.7.2

- Renamed `CheckDatabasePerformance` to `CheckDatastorePerformance`
- Added `CheckDatabasePerformance`

## [0.7.1] - 2024-04-25

### Changed in 0.7.1

- Reinstated `Reinitialize`

## [0.7.0] - 2024-04-19

### Changed in 0.7.0

- Updated to Senzing API V4 method signatures

## [0.6.3] - 2024-02-29

### Changed in 0.6.3

- Fix go package structure

## [0.6.2] - 2024-02-29

### Changed in 0.6.2

- Fix go package structure

## [0.6.1] - 2024-02-29

### Changed in 0.6.1

- Added G2Diagnostic.PurgeRepository()

## [0.6.0] - 2024-02-26

### Changed in 0.6.0

- Updated dependencies
- Deleted methods not used in V4

## [0.5.0] - 2024-01-26

### Changed in 0.5.0

- Renamed module to `github.com/senzing-garage/g2-sdk-proto`
- Update dependencies
  - google.golang.org/grpc v1.61.0
  - google.golang.org/protobuf v1.32.0

## [0.4.3] - 2023-10-15

### Changed in 0.4.3

- Refactor to [template-go](https://github.com/senzing-garage/template-go)

## [0.4.2] - 2023-10-13

### Changed in 0.4.2

- Update dependencies
  - google.golang.org/grpc v1.58.3

### Deleted in 0.4.2

- `g2product.ValidateLicenseFile`
- `g2product.ValidateLicenseStringBase64`

## [0.4.1] - 2023-10-12

### Changed in 0.4.1

- Changed from `int` to `int64` where required by the SenzingAPI

## [0.4.0] - 2023-09-25

### Changed in 0.4.0

- Remove deprecated functions
  See commented functions in [g2api](https://github.com/senzing-garage/g2-sdk-go/blob/495326c4451851f9e9a04ac23a150b3d48bc2ad1/g2api/main.go).

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

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[markdownlint]: https://dlaa.me/markdownlint/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
