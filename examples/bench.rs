use std::hint::black_box;
use std::time::{Duration, Instant};

use pito_header::{Fact, Group, Header, Nav, Section, Styles};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

const WIDTH: u16 = 150;
const HEIGHT: u16 = 40;

fn main() {
    let frames: u32 = std::env::args()
        .nth(1)
        .and_then(|text| text.parse().ok())
        .unwrap_or(20_000);
    let accent = Style::new().fg(Color::Rgb(0xff, 0xcf, 0x5c));
    let dim = Style::new().add_modifier(Modifier::DIM);
    let styles = Styles::new().accent(accent).muted(dim).rule(dim);
    let mut nav = Nav::new(vec![
        Group::new("Mail")
            .section(Section::new("Inbox"))
            .section(Section::new("Drafts"))
            .section(Section::new("Archive")),
        Group::new("Library")
            .section(Section::new("Items & notes").short("Items"))
            .section(Section::new("Collections").short("Sets"))
            .section(Section::new("Ownership").short("Owners")),
        Group::new("Work")
            .section(Section::new("Imports & exports").short("Imports"))
            .section(Section::new("Schedules"))
            .section(Section::new("Checks")),
    ]);
    nav.go(9);
    nav.open("nightly", 4);
    nav.open("step 3", 2);
    let facts = [
        Fact::new("12 checks", Style::new().fg(Color::Green)),
        Fact::new("1 running", accent),
        Fact::new("2 failed", Style::new().fg(Color::Red)),
        Fact::new("synced just now", dim),
    ];
    let area = Rect::new(0, 0, WIDTH, HEIGHT);
    let mut buffer = Buffer::empty(area);
    let mut total = Duration::ZERO;
    let mut worst = Duration::ZERO;
    for frame in 0..frames {
        buffer.reset();
        let started = Instant::now();
        let header = Header::new(&nav)
            .styles(styles)
            .title(Some("Desk"))
            .left(Some(Fact::new("? help", dim)))
            .right(Some(Fact::new("trial: 9 days left", dim)))
            .facts(&facts)
            .facts_rule(true)
            .crumbs(true)
            .underline(true)
            .notice((frame % 2 == 0).then_some(Fact::new("Saved.", Style::new().fg(Color::Green))));
        header.render(Rect::new(0, 0, WIDTH, header.height()), &mut buffer);
        let took = started.elapsed();
        black_box(&buffer);
        total += took;
        worst = worst.max(took);
    }
    let mean = total / frames.max(1);
    println!(
        "pito-header bench: {frames} frames at {WIDTH}x{HEIGHT}: mean {:.1} µs, worst {:.1} µs",
        mean.as_secs_f64() * 1e6,
        worst.as_secs_f64() * 1e6
    );
}
