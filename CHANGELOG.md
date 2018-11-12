# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

### Added

- Add `assert_json_include`. It does partial matching the same way the old `assert_json_eq` did.

### Changed

- Change `assert_json_eq` do exact matching. If the two values are not exactly the same, it'll panic.

### Removed

N/A

### Fixed

N/A

## 0.1.0 - 2018-10-17

Initial release.
