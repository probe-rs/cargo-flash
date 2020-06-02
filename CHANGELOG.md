# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `Cargo.toml` metadata parsing for specifying the chip (see https://github.com/probe-rs/cargo-flash/pull/31).

### Changed

- Improved error logging by a large marging!
- Cleaned up some of the logging output. Mostly beauty stuff.

### Fixed

## [0.7.0]

### Added

### Changed

### Fixed

## [0.6.0]

### Added

- Add a `--speed` setting to configure protocol speed in kHz.
- Upgrade to probe-rs 0.6.0 which fixes some bugs that appeared within cargo-flash (see [CHANGELOG](https://github.com/probe-rs/probe-rs/blob/master/CHANGELOG.md))
- Add a `--restore-unwritten` flag which makes the flashing procedure restore all bytes that have been erased in the sectore erase but are not actually in the writeable sections of the ELF data.
- Add an `--elf` setting to point to a specific ELF binary instead of a cargo one.
- Add a `--work-dir` for cargo flash to operate in.

## [0.5.0]

### Added

- Adds support for JLink and JTag based flashing.
- Add the possibility to select the debug protocol (SWD/JTAG) with `--protocol`.
- Added the possibility to set the log level via the `--log` argument.

### Changed

### Fixed

- Fix a bug where `--probe-index` would be handed to cargo build accidentially.
- Logs are now always shown, even with progressbars enabled.
  Before progressbars would behave weirdly and errors would not be shown.
  Now this is handled properly and any output is shown above the progress bars.

### Known issues

- Some chips do not reset automatically after flashing
- The STM32L0 cores have issues with flashing.

## [0.4.0]

### Added

- A basic GDB server was added \o/ You can either use the provided `gdb-server` binary or use `cargo flash --gdb` to first flash the target and then open a GDB session. There is many more new options which you can list with `cargo flash --help`.
- A flag to disable progressbars was added. Error reporting was broken because of progressbar overdraw. Now one can disable progress bars to see errors. In the long run this has to be fixed.

### Changed

### Fixed

## [0.3.0]

Improved flashing for `cargo-flash` considering speed and useability.

### Added

- Added CMSIS-Pack powered flashing. This feature essentially enables to flash any ARM core which can also be flashed by ARM Keil.
- Added progress bars for flash progress indication.
- Added `nrf-recover` feature that unlocks nRF52 chips through Nordic's custom `AP`

### Changed

### Fixed

- Various bugfixes

## [0.2.0]
- Introduce cargo-flash which can automatically build & flash the target elf file.

[Unreleased]: https://github.com/probe-rs/probe-rs/compare/v0.7.0...master
[0.7.0]: https://github.com/probe-rs/probe-rs/releases/tag/v0.6.0...v0.7.0
[0.6.0]: https://github.com/probe-rs/probe-rs/releases/tag/v0.5.0...v0.6.0
[0.5.0]: https://github.com/probe-rs/probe-rs/releases/tag/v0.4.0...v0.5.0
[0.4.0]: https://github.com/probe-rs/probe-rs/releases/tag/v0.4.0