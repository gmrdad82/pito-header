# Notices

The look of this crate (the centred rules, the nested numbered tabs, the
accent-lit current tab and the breadcrumb rule) is in the style of HEY's terminal UI by 37signals
([basecamp/hey-cli](https://github.com/basecamp/hey-cli)). No code or text is
copied from it.

## Images

The images in `docs/images/` are © 2026 Cătălin Ilinca, under the MIT licence
like the code (see LICENSE); each carries a `.license` file beside it. They
show only this crate's own examples and sample words, no real data.

- `demo.gif`: `examples/demo.rs`, recorded with prec from `render/terminal.toml`
  and `render/terminal/demo.tape`, made with pconv.
- `fit.gif`: `examples/fit.rs`, recorded with prec from
  `render/terminal/fit.tape`, made with pconv.
- `rows.png`, `spans.png`, `drill.png`: `examples/gallery.rs` with the
  argument `rows`, `spans` or `drill`, stills by prec from
  `render/terminal/still.tape`.
- `draw-time-bars.png`: drawn by pito-compare from `draw-time.toml`, the
  output of `cargo run --release --example bench 200000` on 2026-10-14.

`bin/shots` makes them all again.

## The PITO MD logo

`assets/marks/pitomd.svg`, shown at the README's licence section, is the PITO
MD logo, copied by `bin/marks` from the PITO MD brand masters at
fe97f221c8c071389283144c416f4fcde87b3958 (see `assets/marks/SOURCE`). It is ©
Cătălin Ilinca, all rights reserved, and not covered by the MIT licence (see
TRADEMARKS.md).
