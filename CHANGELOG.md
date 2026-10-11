# Changelog

## v0.2.0 — 2026-10-05

### Added

- A section's label changes in place: `Nav::section_mut` with `set_name`, `set_spans`, `set_short` and `set_short_spans`.
- Owned strings draw as facts and slot parts: `facts_pairs`, `left_pairs` and `right_pairs`.

### Changed

- `Key` carries Alt, so Alt+q is not Back, and Ctrl+Alt plus a character passes through as that character, so AltGr input still types.
- `Action`, `Step`, `Key`, `Drill` and `NavKeys` are `#[non_exhaustive]`, and `NavKeys` is built from `NavKeys::HEY`, `NavKeys::NONE` or `NavKeys::default()` with its const methods.
- The facts row skips empty facts and elides only when the room is short.

### Fixed

- Every clipping edge of the title slots, facts, breadcrumb and hit test, and control characters are measured as they are drawn.
- `cycle(0)` stays put.
