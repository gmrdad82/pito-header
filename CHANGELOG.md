# Changelog

## Unreleased

### Changed

- The brand reads PITO MD in the README, TRADEMARKS.md and the demo GIF's licence note.

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

## v0.1.5 — 2026-10-05

### Added

- `Section::short_spans` gives a section's abbreviation several styled spans, drawn in the short and lone forms as `Section::spans` draws the full one; the last of `short` and `short_spans` wins.

## v0.1.4 — 2026-10-05

### Added

- `Section::spans` gives a section several styled spans that draw in their own styles over the selected section's emphasis, abbreviate and clip like a plain label, and are hit-tested as one cell.

### Changed

- `Group` and `Section` share one name type and the breadcrumb's three trails share one rule for which crumbs they show, with no change to the API or the drawing.

## v0.1.3 — 2026-10-05

### Fixed

- The tenth section's label draws its key, 0, instead of 10, so it reads "0 Requests", and clicks on it still land on section ten.

## v0.1.2 — 2026-10-04

### Added

- `Nav::selected` reads the section any group remembers.
- `NavKeys::swallow_digits` takes digits past the last section as a no-op.
- The title's slots take several styled parts.

### Fixed

- Text on a rule keeps exactly its own style instead of inheriting the rule's modifiers.

## v0.1.1 — 2026-10-04

### Added

- `Drill::Rows` keeps the old rows when drilling in, and `Breadcrumb::centred` serves apps that draw their own.

### Changed

- Drilling in swaps the sections row for a centred breadcrumb rule in the accent and steps the facts and breadcrumb rows aside.

## v0.1.0 — 2026-10-04

### Added

- A ratatui header with groups of numbered sections in two nested rows that abbreviate to fit.
- Drill-in with a breadcrumb that keeps the list's selection.
- Optional title, facts and notice rows, click hits, and configurable keys that map to actions without taking input.
