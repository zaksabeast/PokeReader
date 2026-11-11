# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Configurable input bindings for every PokeReader action: pause, frame step,
  resume, overlay toggle and lock toggle.
- Separate keyboard and gamepad binding profiles, with device selection
  (`KB+Pad`, `KB`, `Pad`, `Off`).
- In-plugin **Controls** menu with binding recording, chord support, conflict
  detection, gameplay-leak warnings, per-action clearing, presets, a test view
  and manual save.
- Versioned configuration file persisted to
  `sdmc:/luma/plugins/PokeReader-input.cfg` (fallback
  `sdmc:/PokeReader-input.cfg`), with migration from unversioned files and
  protection against overwriting newer files.
- Conflict-free preset based on the New 3DS `ZL`/`ZR` buttons, which the
  supported games do not read.
- Frame step and resume are deferred until the bound input is released, so the
  game never samples the button that triggered the action.
- `INPUT_CONFIGURATION.md` with the platform model, editor usage, file format
  and known limitations.

### Changed

- Pause, frame step, resume, overlay toggle and lock toggle now go through the
  input configuration instead of hardcoded button checks in the C hook.
- Control hints in the overlay show the configured bindings.
- The game is no longer resumed by the hardcoded `A`/`Start`/`R` buttons when a
  custom resume binding is configured.

### Fixed

- PokeReader no longer has to share buttons with the emulated game: users can
  choose inputs the game ignores instead of relying on Citra's mapping.

### Notes

- PokeReader runs inside the 3DS process and cannot read PC keyboard/gamepad
  input directly. Keyboard and gamepad keys must be mapped to 3DS buttons in
  Citra (or come from the physical console). See `INPUT_CONFIGURATION.md`.
- No changes were made to MT RNG, TinyMT, game memory access, hunting or
  inspection logic.
