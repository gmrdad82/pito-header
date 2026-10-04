mod common;

use common::{STYLES, draw, nav, show, widest};
use pito_header::{Breadcrumb, Drill, Fact, Group, Header, Key, Nav, Place, Section, Step};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
};

const GRAY: Style = Style::new().fg(Color::DarkGray);
const FACTS: [Fact; 2] = [
    Fact::new("12 owners", GRAY),
    Fact::new("synced just now", GRAY),
];

fn owners() -> Nav {
    let mut nav = nav();
    nav.go(6);
    nav
}

fn header<'a>(nav: &'a Nav) -> Header<'a> {
    Header::new(nav)
        .styles(STYLES)
        .title(Some("Desk"))
        .facts(&FACTS)
        .facts_rule(true)
        .crumbs(true)
        .closing(true)
}

fn frame(nav: &Nav, width: u16) -> Vec<String> {
    let header = header(nav);
    draw(header, width, header.height()).text
}

const TOP: [&str; 6] = [
    "───────────────────────────────────── Desk ─────────────────────────────────────",
    "                            Mail    Library    Work",
    "                  4 Items & notes  5 Collections  6 Ownership",
    "───────────────────────── 12 owners · synced just now ──────────────────────────",
    "─ Ownership ────────────────────────────────────────────────────────────────────",
    "────────────────────────────────────────────────────────────────────────────────",
];

#[test]
fn at_the_top_level_nothing_changes() {
    let nav = owners();
    assert_eq!(header(&nav).height(), 6);
    assert_eq!(frame(&nav, 80), TOP);
    let rows = header(&nav).drill(Drill::Rows);
    assert_eq!(draw(rows, 80, 6).text, TOP);
}

#[test]
fn one_level_deep_the_sections_row_becomes_the_breadcrumb_rule() {
    let mut nav = owners();
    nav.open("build queue", 3);
    let header = header(&nav);
    assert_eq!(header.height(), 4);
    let drawn = draw(header, 80, 4);
    assert_eq!(
        drawn.text,
        [
            "───────────────────────────────────── Desk ─────────────────────────────────────",
            "                            Mail    Library    Work",
            "─────────────────────────── Ownership / build queue ────────────────────────────",
            "────────────────────────────────────────────────────────────────────────────────",
        ],
        "\n{}",
        show(&drawn)
    );
    assert_eq!(
        drawn.marks[2],
        "rrrrrrrrrrrrrrrrrrrrrrrrrrrrAAAAAAAAAAAAAAAAAAAAAAArrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
        "\n{}",
        show(&drawn)
    );
}

#[test]
fn two_levels_deep_and_narrow_widths_abbreviate_the_breadcrumb() {
    let mut nav = owners();
    nav.open("build queue", 3);
    nav.open("nightly run", 1);
    assert_eq!(
        frame(&nav, 80)[2],
        "──────────────────── Ownership / build queue / nightly run ─────────────────────"
    );
    assert_eq!(
        frame(&nav, 40)[2],
        "───── Ownership / … / nightly run ──────"
    );
    assert_eq!(frame(&nav, 24)[2], "─── … / nightly run ────");
    assert_eq!(frame(&nav, 12)[2], "─ … / nig… ─");
    assert_eq!(frame(&nav, 4)[2], "────");
    for width in 1..=120 {
        let header = header(&nav);
        let drawn = draw(header, width, header.height());
        assert!(widest(&drawn) <= usize::from(width), "{width}");
        assert_eq!(drawn.text.len(), 4);
    }
}

#[test]
fn a_romanian_title_measures_by_cell() {
    let mut nav = Nav::new(vec![
        Group::new("Mesaje")
            .section(Section::new("Căsuță"))
            .section(Section::new("Înștiințări")),
        Group::new("Setări").section(Section::new("Cont")),
    ]);
    nav.go(2);
    nav.open("Ședință în Brașov, țară", 0);
    assert_eq!(
        frame(&nav, 50),
        [
            "────────────────────── Desk ──────────────────────",
            "                 Mesaje    Setări",
            "───── Înștiințări / Ședință în Brașov, țară ──────",
            "──────────────────────────────────────────────────",
        ]
    );
    assert_eq!(frame(&nav, 24)[2], "─ … / Ședință în Braș… ─");
}

#[test]
fn back_restores_the_rows_and_the_selection() {
    let mut nav = owners();
    nav.open("build queue", 3);
    nav.open("nightly run", 1);
    assert_eq!(nav.key(Key::Esc), Some(Step::Back { selected: 1 }));
    assert_eq!(header(&nav).height(), 4);
    assert_eq!(nav.key(Key::Esc), Some(Step::Back { selected: 3 }));
    assert_eq!(header(&nav).height(), 6);
    assert_eq!(frame(&nav, 80), TOP);
}

#[test]
fn clicks_find_groups_but_not_the_breadcrumb_rule() {
    let mut nav = owners();
    let area = Rect::new(0, 0, 80, 6);
    let ownership = Some(Place {
        group: 1,
        section: 2,
        number: 6,
    });
    let mail = Some(Place {
        group: 0,
        section: 0,
        number: 1,
    });
    assert_eq!(header(&nav).hit(area, 50, 2), ownership);
    nav.open("build queue", 3);
    let deep = header(&nav);
    assert_eq!(deep.hit(area, 30, 1), mail);
    assert_eq!(deep.hit(area, 50, 2), None);
    assert_eq!(deep.hit(area, 50, 3), None);
    assert_eq!(deep.drill(Drill::Rows).hit(area, 50, 2), ownership);
}

#[test]
fn the_rows_drill_keeps_every_row() {
    let mut nav = owners();
    nav.open("build queue", 3);
    let rows = header(&nav).drill(Drill::Rows);
    assert_eq!(rows.height(), 6);
    let drawn = draw(rows, 80, 6).text;
    assert_eq!(drawn[2], TOP[2]);
    assert_eq!(drawn[3], TOP[3]);
    assert_eq!(
        drawn[4],
        "─ Ownership / build queue ──────────────────────────────────────────────────────"
    );
}

#[test]
fn without_the_tabs_nothing_is_replaced() {
    let mut nav = owners();
    nav.open("build queue", 3);
    let plain = header(&nav).tabs(false);
    assert_eq!(plain.height(), 4);
    let drawn = draw(plain, 40, 4).text;
    assert_eq!(drawn[1], "───── 12 owners · synced just now ──────");
    assert_eq!(drawn[2], "─ Ownership / build queue ──────────────");
}

#[test]
fn a_single_group_draws_only_the_breadcrumb_rule() {
    let mut nav = Nav::new(vec![
        Group::new("Main")
            .section(Section::new("Items"))
            .section(Section::new("Settings")),
    ]);
    nav.open("Gamer", 0);
    let header = Header::new(&nav).styles(STYLES);
    assert_eq!(header.height(), 1);
    assert_eq!(draw(header, 21, 1).text, ["─── Items / Gamer ───"]);
}

#[test]
fn the_breadcrumb_widget_centres_on_request() {
    let mut nav = owners();
    nav.open("build queue", 3);
    let crumb = |centred: bool| {
        let widget = Breadcrumb::new(&nav).styles(STYLES).centred(centred);
        draw(widget, 31, 1).text[0].clone()
    };
    assert_eq!(crumb(false), "─ Ownership / build queue ─────");
    assert_eq!(crumb(true), "─── Ownership / build queue ───");
}
