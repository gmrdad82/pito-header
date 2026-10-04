# pito-header

The top of a pito terminal app, as a small ratatui 0.30 crate: groups of
numbered sections drawn as two nested rows, drill-in with a breadcrumb, and
optional title, facts and notice rows. The look is in the style of HEY's
terminal UI. It has no app logic and no words of its own: the app passes in
every name, every style and every key.

```toml
pito-header = { git = "https://github.com/gmrdad82/pito-header", tag = "v0.1.0" }
```

Turn on the `crossterm` feature for `Key::from(crossterm::event::KeyEvent)`
(crossterm 0.29); without it the crate has no backend dependency.

## What it does

- **Groups and numbered sections, as data.** Sections are numbered 1, 2, 3…
  across all groups, so a digit jumps anywhere; `0` is the tenth.
- **Two nested rows,** the groups over the current group's sections,
  centred, the current one in the accent and bold. When the width runs out
  they abbreviate step by step and never wrap: full names, then each
  section's short name, then only the current one named, then numbers only.
  A group falls back to its short name, then its first letter.
- **Drill-in.** `open(title, selected)` goes one level deeper and remembers
  the list's selection; back returns it, so the list comes back where it
  was. Each section keeps its own stack while you move around. The
  breadcrumb rule ("Checks / nightly") elides the middle, then the head,
  when it doesn't fit.
- **Keys map to actions; nothing is taken behind the app's back.**
  `action(key)` only looks; `apply(action)` moves; `key(key)` does both.
  Every key set is configurable through `NavKeys` (`NavKeys::NONE` takes
  nothing), so `tab`, `[ ]`, digits or `q` can keep other meanings in an app.
  Back at the top level is never an action, so `esc` stays the app's there.
  With a single group, `tab` steps sections.
- **A header with no sections** (a dashboard) draws only the rows it's given.
- **Optional rows** in `Header`: a title rule with the name in the middle
  and a left and a right slot (they drop when there's no room), the nav
  rows, a facts row (each fact styled by the app, on a rule or plain), the
  breadcrumb rule, a closing rule, and a notice row (a result, a warning, or
  a confirm question).
- **Clicks:** `hit(area, column, row)` names the group or section under the
  pointer; the crate never reads the mouse itself.
- **Widths by cell:** Unicode widths throughout, so diacritics (ă, î, ș, ț),
  "…" and wide glyphs measure and clip cleanly.
- **Cheap:** drawing writes straight into the buffer, with no allocation
  and no clock; well under a millisecond at 150×40 (`cargo run --release
  --example bench`).

## The API

```text
pub enum Key { Char(char), Ctrl(char), Tab, BackTab, Enter, Esc, Backspace,
               Left, Right, Up, Down, Other }
pub struct Styles { accent, muted, rule }              // all Style::new() by default
Section::new(name).short(short)
Group::new(name).short(short).section(section)
Nav::new(groups).keys(NavKeys)
  place() -> Place { group, section, number }          // number is 1-based, 0 when empty
  section(), count(), go(number), go_to(group, section), cycle(by), step(by)
  open(title, selected), back() -> Option<usize>, close() -> Option<usize>
  depth(), title(), crumbs(), breadcrumb() -> String
  action(Key) -> Option<Action>, apply(Action) -> Option<Step>, key(Key) -> Option<Step>
pub enum Action { NextGroup, PrevGroup, NextSection, PrevSection, Go(usize), Back }
pub enum Step { Moved(Place), Back { selected } }
NavKeys { next_group, prev_group, next_section, prev_section, back: &'static [Key], digits }
  NavKeys::HEY: tab / backtab, ] / [, digits, esc / q          NavKeys::NONE
NavBar::new(&nav).styles(..).lit(bool).groups(bool).underline(bool); height(), hit(..)
Breadcrumb::new(&nav).styles(..)
Fact::new(text, style)
Header::new(&nav).styles(..).title(..).left(..).right(..).tabs(bool).lit(bool)
  .underline(bool).facts(&[Fact]).facts_rule(bool).separator(..).crumbs(bool)
  .closing(bool).notice(Option<Fact>); height(), hit(..)
```

`lit(false)` keeps the group lit but dims the current section, for a page
that sits over every section (help, settings). `underline(true)` underlines
the section numbers.

## Example

```rust,standalone_crate
use pito_header::{Fact, Group, Header, Key, Nav, Section, Step, Styles};
use ratatui::{Frame, layout::Rect, style::{Color, Modifier, Style}};

fn nav() -> Nav {
    Nav::new(vec![
        Group::new("Mail").section(Section::new("Inbox")).section(Section::new("Drafts")),
        Group::new("Library").section(Section::new("Items & notes").short("Items")),
    ])
}

fn key(nav: &mut Nav, key: Key, selected: &mut usize) {
    match nav.key(key) {
        Some(Step::Back { selected: kept }) => *selected = kept,
        Some(Step::Moved(_)) => *selected = 0,
        None if key == Key::Enter => nav.open(format!("item {selected}"), *selected),
        None => {}
    }
}

fn draw(frame: &mut Frame, nav: &Nav) {
    let dim = Style::new().add_modifier(Modifier::DIM);
    let styles = Styles::new().accent(Style::new().fg(Color::Magenta)).muted(dim).rule(dim);
    let facts = [Fact::new("12 items", dim), Fact::new("synced just now", dim)];
    let header = Header::new(nav)
        .styles(styles)
        .title(Some("Desk"))
        .right(Some(Fact::new("? help", dim)))
        .facts(&facts)
        .crumbs(true);
    let area = frame.area();
    frame.render_widget(header, Rect { height: header.height(), ..area });
}
```

## Development

`bin/gate` runs `cargo fmt --check`, `cargo clippy --all-targets
--all-features -- -D warnings`, `cargo test --all-features` (the tests draw
through ratatui's `TestBackend`, and this README's example compiles as a
doctest) and builds the bench.

## Licence

MIT, see [LICENSE](LICENSE). The look is in the style of HEY's terminal UI;
see [NOTICE.md](NOTICE.md).
