# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/ghismary/weather-utils/compare/v0.3.0...v0.4.0) - 2026-06-10

### Added

- Add heat index computation
- Add dew point temperature computation

### Fixed

- Fix the "Humidity at a glance" link in the README.md file

### Other

- Implement new type pattern for `RelativeHumidity` to ensure that the value of the relative humidity is between 0 and
  100%
- Refactor the temperature types
- Update rstest and add more_assert dev-dependencies
- Improve code coverage
- Update CI workflows
- Add zed tasks

## [0.3.0](https://github.com/ghismary/weather-utils/compare/v0.2.1...v0.3.0) - 2025-02-03

### Added

- Add GitHub status badge and Codecov integration
- Add PartialEq implementations & Improve tests

### Fixed

- [**breaking**] fix the spelling of "Fahrenheit"
- [**breaking**] fix the spelling of "Celsius"

## [0.2.1](https://github.com/ghismary/weather-utils/compare/v0.2.0...v0.2.1) - 2024-12-02

### Fixed

- update repository links to GitHub
