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
  and `render/terminal/demo.tape`, made with pcod
  ([pito-codecs](https://github.com/gmrdad82/pito-codecs)).
- `fit.gif`: `examples/fit.rs`, recorded with prec from
  `render/terminal/fit.tape`, made with pcod
  ([pito-codecs](https://github.com/gmrdad82/pito-codecs)).
- `rows.png`, `spans.png`, `drill.png`: `examples/gallery.rs` with the
  argument `rows`, `spans` or `drill`, stills by prec from
  `render/terminal/still.tape`.
- `draw-time-bars.png`: drawn by pito-compare from `draw-time.toml`, the
  output of `cargo run --release --example bench 200000` on 2026-10-14.

`bin/shots` makes them all again.

## The PITO MD wordmark

`assets/marks/wordmark.svg`, shown at the README's licence section, is the PITO
MD wordmark, copied by `bin/marks` from the PITO MD brand masters at
aa3aee8b232a1b848995bf3d9a012a01d4aff8ec (see `assets/marks/SOURCE`). It is ©
Cătălin Ilinca, all rights reserved, and not covered by the MIT licence (see
TRADEMARKS.md).
