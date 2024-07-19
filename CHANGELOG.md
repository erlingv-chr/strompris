# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2024-07-19
### Changed
- Errors are now collected in a single enum, that passes through errors from dependencies. This makes
it easier to develop using the `?` operator. This is considered to be a breaking change.
- Cleaned up a bunch of manual map-functions in the Date-type.

## [0.2.0] - 2024-07-15

### Added

- Tests
- A check in Strompris.get_prices() to see if the date is after
the minimum acceptable date. 
- A new error type: StromprisError
- HourlyPrice derives PartialEq
- Strompris.get_prices() returns a fitting error message when trying
to get prices from the future

### Changed

- Changed the signature of Strompris.get_prices() so that it takes
a DateLike structure, instead of integers for year, month and day.
- Moved the blocking test from strompris.rs to lib.rs
- The return type of Strompris.get_prices() to Result<Vec<HourlyPrice>, StromprisError>.


## [0.1.0] - 2024-07-15
- This is the original MVP of the project