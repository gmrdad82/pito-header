mod common;

use common::{STYLES, draw, nav, read, show, widest};
use pito_header::{Breadcrumb, Drill, Fact, Group, Header, Nav, NavBar, Place, Section};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

const GREEN: Style = Style::new().fg(Color::Green);
const MAGENTA: Style = Style::new().fg(Color::Magenta);
const GRAY: Style = Style::new().fg(Color::DarkGray);
const RED: Style = Style::new().fg(Color::Red);

fn deep() -> Nav {
    let mut nav = nav();
    nav.go(9);
    nav.open("nightly", 4);
    nav
}

const FACTS: [Fact; 3] = [
    Fact::new("12 checks", GREEN),
    Fact::new("1 running", MAGENTA),
    Fact::new("synced just now", GRAY),
];

fn full<'a>(nav: &'a Nav) -> Header<'a> {
    Header::new(nav)
        .styles(STYLES)
        .title(Some("Desk"))
        .left(Some(Fact::new("? help", GRAY)))
        .right(Some(Fact::new("trial: 9 days left", GRAY)))
        .facts(&FACTS)
        .facts_rule(true)
        .crumbs(true)
        .notice(Some(Fact::new(
            "Stop schedule nightly? enter confirms · esc cancels",
            RED,
        )))
        .underline(true)
}

fn rows<'a>(nav: &'a Nav) -> Header<'a> {
    full(nav).drill(Drill::Rows)
}

#[test]
fn the_full_header_at_eighty_columns() {
    let nav = deep();
    let header = rows(&nav);
    assert_eq!(header.height(), 6);
    let drawn = draw(header, 80, 6);
    assert_eq!(
        drawn.text,
        [
            "─ ? help ──────────────────────────── Desk ──────────────── trial: 9 days left ─",
            "                            Mail    Library    Work",
            "                   7 Imports & exports  8 Schedules  9 Checks",
            "─────────────────── 12 checks · 1 running · synced just now ────────────────────",
            "─ Checks / nightly ─────────────────────────────────────────────────────────────",
            "Stop schedule nightly? enter confirms · esc cancels",
        ],
        "\n{}",
        show(&drawn)
    );
    assert_eq!(
        drawn.marks,
        [
            "rmmmmmmmmrrrrrrrrrrrrrrrrrrrrrrrrrrrrAAAAAArrrrrrrrrrrrrrrrmmmmmmmmmmmmmmmmmmmmr",
            "                           mmmmmm  mmmmmmmmm  AAAAAA",
            "                  mummmmmmmmmmmmmmmmmmmmummmmmmmmmmmAUAAAAAAAA",
            "rrrrrrrrrrrrrrrrrrrmgggggggggmmmaaaaaaaaammmmmmmmmmmmmmmmmmmrrrrrrrrrrrrrrrrrrrr",
            "rrAAAAAAAAAAAAAAAArrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        ],
        "\n{}",
        show(&drawn)
    );
}

#[test]
fn narrow_widths_abbreviate_and_never_wrap() {
    let nav = deep();
    let drawn = draw(rows(&nav), 30, 6);
    assert_eq!(
        drawn.text,
        [
            "─ ? help ─── Desk ─ trial: … ─",
            "   Mail    Library    Work",
            "        7  8  9 Checks",
            "─ 12 checks · 1 running · s… ─",
            "─ Checks / nightly ───────────",
            "Stop schedule nightly? enter …",
        ],
        "\n{}",
        show(&drawn)
    );
    let drawn = draw(rows(&nav), 16, 6);
    assert_eq!(
        drawn.text,
        [
            "───── Desk ─────",
            " M    L    Jobs",
            " 7  8  9 Checks",
            "─ 12 checks… ───",
            "─ … / nightly ──",
            "Stop schedule n…",
        ],
        "\n{}",
        show(&drawn)
    );
}

#[test]
fn every_width_fits_its_rows() {
    let nav = deep();
    for width in 0..=160 {
        for (header, height) in [(rows(&nav), 6), (full(&nav), 4)] {
            assert_eq!(header.height(), height);
            let drawn = draw(header, width.max(1), header.height());
            assert!(widest(&drawn) <= usize::from(width.max(1)), "{width}");
            assert_eq!(drawn.text.len(), usize::from(height));
        }
    }
}

#[test]
fn sections_shorten_step_by_step() {
    let nav = deep();
    let row = |width: u16| draw(NavBar::new(&nav).styles(STYLES), width, 2).text[1].clone();
    assert_eq!(row(60).trim(), "7 Imports & exports  8 Schedules  9 Checks");
    assert_eq!(row(40).trim(), "7 Imports  8 Schedules  9 Checks");
    assert_eq!(row(24).trim(), "7  8  9 Checks");
    assert_eq!(row(10).trim(), "7  8  9");
    assert_eq!(row(4), " 7");
}

#[test]
fn a_dimmed_bar_lights_only_the_group() {
    let nav = deep();
    let drawn = draw(NavBar::new(&nav).styles(STYLES).lit(false), 50, 2);
    assert!(drawn.marks[0].contains('A'));
    assert!(!drawn.marks[1].contains('A'), "\n{}", show(&drawn));
}

#[test]
fn one_group_draws_one_row_and_no_groups_draw_none() {
    let single = Nav::new(vec![
        Group::new("Main")
            .section(Section::new("Items"))
            .section(Section::new("Settings")),
    ]);
    let bar = NavBar::new(&single).styles(STYLES);
    assert_eq!(bar.height(), 1);
    assert_eq!(draw(bar, 30, 1).text, ["     1 Items  2 Settings"]);
    let empty = Nav::new(Vec::new());
    let header = Header::new(&empty).styles(STYLES).title(Some("Dashboard"));
    assert_eq!(header.height(), 1);
    assert_eq!(draw(header, 21, 1).text, ["───── Dashboard ─────"]);
}

const TRIAL: [Fact; 2] = [Fact::new("trial: ", GRAY), Fact::new("9 days", MAGENTA)];
const HELP: [Fact; 2] = [Fact::new("?", MAGENTA), Fact::new(" help", GRAY)];

fn slots<'a>(nav: &'a Nav) -> Header<'a> {
    Header::new(nav)
        .styles(STYLES)
        .tabs(false)
        .title(Some("Desk"))
        .left_parts(&HELP)
        .right_parts(&TRIAL)
}

#[test]
fn the_title_slots_take_several_styled_parts() {
    let nav = nav();
    let drawn = draw(slots(&nav), 50, 1);
    assert_eq!(
        drawn.text,
        ["─ ? help ───────────── Desk ────── trial: 9 days ─"],
        "\n{}",
        show(&drawn)
    );
    assert_eq!(
        drawn.marks,
        ["raammmmmmrrrrrrrrrrrrrAAAAAArrrrrrmmmmmmmmaaaaaaar"],
        "\n{}",
        show(&drawn)
    );
}

#[test]
fn several_parts_clip_with_the_ellipsis_in_the_cut_part() {
    let nav = nav();
    let drawn = draw(slots(&nav), 34, 1);
    assert_eq!(
        drawn.text,
        ["─ ? help ───── Desk ─ trial: 9 … ─"],
        "\n{}",
        show(&drawn)
    );
    assert_eq!(
        drawn.marks,
        ["raammmmmmrrrrrAAAAAArmmmmmmmmaaaar"],
        "\n{}",
        show(&drawn)
    );
    let cut = [Fact::new("ab界", GRAY), Fact::new("c", MAGENTA)];
    let header = Header::new(&nav)
        .styles(STYLES)
        .tabs(false)
        .title(Some("Desk"))
        .right_parts(&cut);
    assert_eq!(draw(header, 22, 1).text, ["──────── Desk ─ ab… ──"]);
}

#[test]
fn empty_parts_and_the_single_fact_replace_each_other() {
    let nav = nav();
    let blank = [Fact::new("", GRAY)];
    let none = Header::new(&nav)
        .styles(STYLES)
        .tabs(false)
        .title(Some("Desk"))
        .left_parts(&[])
        .right_parts(&blank);
    assert_eq!(draw(none, 30, 1).text, ["──────────── Desk ────────────"]);
    let single = slots(&nav).right(Some(Fact::new("trial", GRAY))).left(None);
    assert_eq!(draw(single, 30, 1).text, ["──────────── Desk ──── trial ─"]);
}

#[test]
fn rows_are_optional() {
    let nav = deep();
    let tabs = Header::new(&nav).styles(STYLES);
    assert_eq!(tabs.height(), 2);
    let none = Header::new(&nav).styles(STYLES).tabs(false);
    assert_eq!(none.height(), 0);
    let plain = Header::new(&nav)
        .styles(STYLES)
        .tabs(false)
        .facts(&FACTS[..1])
        .closing(true);
    assert_eq!(plain.height(), 2);
    assert_eq!(
        draw(plain, 20, 2).text,
        ["     12 checks", "────────────────────"]
    );
}

#[test]
fn clicks_land_on_the_label_under_them() {
    let nav = deep();
    let header = rows(&nav);
    let area = Rect::new(0, 0, 80, 7);
    assert_eq!(
        header.hit(area, 30, 1),
        Some(Place {
            group: 0,
            section: 0,
            number: 1
        })
    );
    assert_eq!(
        header.hit(area, 40, 1),
        Some(Place {
            group: 1,
            section: 0,
            number: 4
        })
    );
    assert_eq!(
        header.hit(area, 20, 2),
        Some(Place {
            group: 2,
            section: 0,
            number: 7
        })
    );
    assert_eq!(
        header.hit(area, 52, 2),
        Some(Place {
            group: 2,
            section: 2,
            number: 9
        })
    );
    assert_eq!(header.hit(area, 0, 2), None);
    assert_eq!(header.hit(area, 30, 0), None);
    assert_eq!(header.hit(area, 30, 4), None);
}

#[test]
fn the_breadcrumb_elides_the_middle_then_the_head() {
    let mut nav = deep();
    nav.open("step three", 0);
    let crumb = |width: u16| draw(Breadcrumb::new(&nav).styles(STYLES), width, 1).text[0].clone();
    assert_eq!(crumb(40), "─ Checks / nightly / step three ────────");
    assert_eq!(crumb(28), "─ Checks / … / step three ──");
    assert_eq!(crumb(20), "─ … / step three ───");
    assert_eq!(crumb(12), "─ … / ste… ─");
    assert_eq!(crumb(3), "───");
}

#[test]
fn diacritics_and_wide_glyphs_measure_and_clip_by_cell() {
    let nav = Nav::new(vec![
        Group::new("Mesaje")
            .section(Section::new("Căsuță"))
            .section(Section::new("Înștiințări"))
            .section(Section::new("Setări")),
        Group::new("日本").section(Section::new("項目")),
    ]);
    let drawn = draw(NavBar::new(&nav).styles(STYLES), 40, 2);
    assert_eq!(drawn.text[0], "             Mesaje    日本");
    assert_eq!(drawn.text[1], "   1 Căsuță  2 Înștiințări  3 Setări");
    let fact = [Fact::new("Ultima sincronizare: acum ș ț ă î", GRAY)];
    let drawn = draw(Header::new(&nav).tabs(false).facts(&fact), 20, 1);
    assert_eq!(drawn.text[0], "Ultima sincronizare…");
    let wide = [Fact::new("日本語のテキスト", GRAY)];
    let drawn = draw(Header::new(&nav).tabs(false).facts(&wide), 9, 1);
    assert_eq!(drawn.text[0], "日本語の…");
}

#[test]
fn drawing_stays_inside_its_area() {
    let nav = deep();
    let mut buffer = Buffer::empty(Rect::new(0, 0, 40, 10));
    let area = Rect::new(5, 3, 20, 4);
    full(&nav).render(area, &mut buffer);
    let drawn = read(&buffer);
    for (y, line) in drawn.text.iter().enumerate() {
        let y = y as u16;
        if !(3..7).contains(&y) {
            assert!(line.is_empty(), "row {y}: {line:?}");
        } else {
            assert!(line.starts_with("     "), "row {y}: {line:?}");
            assert!(
                unicode_width::UnicodeWidthStr::width(line.as_str()) <= 25,
                "row {y}: {line:?}"
            );
        }
    }
}
