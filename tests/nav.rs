mod common;

use common::nav;
use pito_header::{Action, Group, Key, Nav, NavKeys, Place, Section, Step};

fn place(group: usize, section: usize, number: usize) -> Place {
    Place {
        group,
        section,
        number,
    }
}

#[test]
fn a_new_nav_starts_on_the_first_section() {
    let nav = nav();
    assert_eq!(nav.place(), place(0, 0, 1));
    assert_eq!(nav.section().map(Section::name), Some("Inbox"));
    assert_eq!(nav.count(), 9);
    assert_eq!(nav.depth(), 0);
}

#[test]
fn digits_jump_across_groups_by_their_running_number() {
    let mut nav = nav();
    assert_eq!(nav.key(Key::Char('5')), Some(Step::Moved(place(1, 1, 5))));
    assert_eq!(nav.section().map(Section::name), Some("Collections"));
    assert_eq!(nav.key(Key::Char('9')), Some(Step::Moved(place(2, 2, 9))));
    assert_eq!(nav.key(Key::Char('1')), Some(Step::Moved(place(0, 0, 1))));
}

#[test]
fn a_digit_past_the_last_section_does_nothing() {
    let mut nav = nav();
    assert_eq!(nav.action(Key::Char('0')), None);
    assert_eq!(nav.key(Key::Char('0')), None);
    assert_eq!(nav.place(), place(0, 0, 1));
}

#[test]
fn a_digit_past_the_last_section_is_swallowed_on_request() {
    let mut nav = nav().keys(NavKeys::HEY.swallow_digits(true));
    nav.go(4);
    for digit in ['0', '1'] {
        let past = digit == '0';
        let action = nav.action(Key::Char(digit));
        assert_eq!(action == Some(Action::Swallow), past, "{digit}");
    }
    assert_eq!(nav.key(Key::Char('0')), Some(Step::Swallowed));
    assert_eq!(nav.apply(Action::Swallow), Some(Step::Swallowed));
    assert_eq!(nav.place(), place(1, 0, 4));
    assert_eq!(nav.key(Key::Char('9')), Some(Step::Moved(place(2, 2, 9))));
    assert_eq!(nav.action(Key::Char('x')), None);
}

#[test]
fn swallowing_needs_the_digits() {
    let nav = nav().keys(NavKeys::HEY.digits(false).swallow_digits(true));
    assert_eq!(nav.action(Key::Char('0')), None);
    assert_eq!(nav.action(Key::Char('1')), None);
    let defaults = [NavKeys::HEY, NavKeys::NONE, NavKeys::default()];
    assert!(defaults.iter().all(|keys| !keys.swallow_digits));
}

#[test]
fn every_group_remembers_its_section() {
    let mut nav = nav();
    assert_eq!(nav.selected(0), Some(0));
    assert_eq!(nav.selected(2), Some(0));
    nav.go(8);
    nav.go(5);
    assert_eq!(nav.selected(2), Some(1));
    assert_eq!(nav.selected(1), Some(1));
    assert_eq!(nav.selected(0), Some(0));
    assert_eq!(nav.selected(3), None);
    let section = nav.selected(2).unwrap();
    assert_eq!(nav.go_to(2, section), Some(place(2, 1, 8)));
    let mut empty = Nav::new(vec![
        Group::new("Empty"),
        Group::new("Mail").section(Section::new("Inbox")),
    ]);
    assert_eq!(empty.selected(0), None);
    assert_eq!(empty.selected(1), Some(0));
    assert_eq!(empty.go(1), Some(place(1, 0, 1)));
}

#[test]
fn zero_is_the_tenth_section() {
    let mut nav = Nav::new(vec![(1..=10).fold(Group::new("All"), |group, n| {
        group.section(Section::new(format!("S{n}")))
    })]);
    assert_eq!(nav.action(Key::Char('0')), Some(Action::Go(10)));
    assert_eq!(nav.key(Key::Char('0')), Some(Step::Moved(place(0, 9, 10))));
}

#[test]
fn tab_and_backtab_cycle_groups_and_return_to_the_last_section() {
    let mut nav = nav();
    nav.key(Key::Char('2'));
    assert_eq!(nav.key(Key::Tab), Some(Step::Moved(place(1, 0, 4))));
    nav.key(Key::Char('6'));
    assert_eq!(nav.key(Key::Tab), Some(Step::Moved(place(2, 0, 7))));
    assert_eq!(nav.key(Key::Tab), Some(Step::Moved(place(0, 1, 2))));
    assert_eq!(nav.key(Key::BackTab), Some(Step::Moved(place(2, 0, 7))));
    assert_eq!(nav.key(Key::BackTab), Some(Step::Moved(place(1, 2, 6))));
}

#[test]
fn brackets_step_sections_within_the_group_and_wrap() {
    let mut nav = nav();
    nav.key(Key::Char('4'));
    assert_eq!(nav.key(Key::Char(']')), Some(Step::Moved(place(1, 1, 5))));
    assert_eq!(nav.key(Key::Char(']')), Some(Step::Moved(place(1, 2, 6))));
    assert_eq!(nav.key(Key::Char(']')), Some(Step::Moved(place(1, 0, 4))));
    assert_eq!(nav.key(Key::Char('[')), Some(Step::Moved(place(1, 2, 6))));
}

#[test]
fn with_one_group_tab_steps_sections() {
    let mut nav = Nav::new(vec![
        Group::new("Main")
            .section(Section::new("Items"))
            .section(Section::new("Settings")),
    ]);
    assert_eq!(nav.action(Key::Tab), Some(Action::NextSection));
    assert_eq!(nav.key(Key::Tab), Some(Step::Moved(place(0, 1, 2))));
    assert_eq!(nav.key(Key::Tab), Some(Step::Moved(place(0, 0, 1))));
    assert_eq!(nav.key(Key::BackTab), Some(Step::Moved(place(0, 1, 2))));
}

#[test]
fn drilling_in_builds_a_breadcrumb_and_back_returns_the_selection() {
    let mut nav = nav();
    nav.key(Key::Char('9'));
    nav.open("nightly", 4);
    nav.open("step 3", 2);
    assert_eq!(nav.depth(), 2);
    assert_eq!(nav.title(), Some("step 3"));
    assert_eq!(nav.breadcrumb(), "Checks / nightly / step 3");
    assert_eq!(nav.key(Key::Esc), Some(Step::Back { selected: 2 }));
    assert_eq!(nav.key(Key::Char('q')), Some(Step::Back { selected: 4 }));
    assert_eq!(nav.breadcrumb(), "Checks");
}

#[test]
fn back_at_the_top_is_not_taken() {
    let mut nav = nav();
    assert_eq!(nav.action(Key::Esc), None);
    assert_eq!(nav.key(Key::Char('q')), None);
}

#[test]
fn each_section_keeps_its_own_depth() {
    let mut nav = nav();
    nav.open("first", 7);
    nav.key(Key::Char('4'));
    assert_eq!(nav.depth(), 0);
    nav.key(Key::Char('1'));
    assert_eq!(nav.breadcrumb(), "Inbox / first");
    assert_eq!(nav.close(), Some(7));
    assert_eq!(nav.depth(), 0);
}

#[test]
fn keys_are_configurable_and_never_taken_without_a_match() {
    const KEYS: NavKeys = NavKeys::NONE
        .next_group(&[Key::Right])
        .prev_group(&[Key::Left])
        .next_section(&[Key::Down])
        .prev_section(&[Key::Up])
        .back(&[Key::Esc, Key::Char('q'), Key::Char('Q'), Key::Backspace]);
    let mut nav = nav().keys(KEYS);
    assert_eq!(nav.action(Key::Tab), None);
    assert_eq!(nav.action(Key::Char(']')), None);
    assert_eq!(nav.action(Key::Char('3')), None);
    assert_eq!(nav.key(Key::Right), Some(Step::Moved(place(1, 0, 4))));
    assert_eq!(nav.key(Key::Down), Some(Step::Moved(place(1, 1, 5))));
    nav.open("detail", 3);
    assert_eq!(nav.key(Key::Backspace), Some(Step::Back { selected: 3 }));
    nav.open("detail", 1);
    assert_eq!(nav.key(Key::Char('Q')), Some(Step::Back { selected: 1 }));
}

#[test]
fn action_only_looks_and_apply_moves() {
    let mut nav = nav();
    assert_eq!(nav.action(Key::Char('8')), Some(Action::Go(8)));
    assert_eq!(nav.place(), place(0, 0, 1));
    assert_eq!(nav.apply(Action::Go(8)), Some(Step::Moved(place(2, 1, 8))));
}

#[test]
fn no_keys_at_all_leaves_every_key_to_the_app() {
    let nav = nav().keys(NavKeys::NONE);
    for key in [
        Key::Tab,
        Key::BackTab,
        Key::Char('['),
        Key::Char(']'),
        Key::Char('1'),
        Key::Esc,
        Key::Char('q'),
        Key::Enter,
    ] {
        assert_eq!(nav.action(key), None, "{key:?}");
    }
}

#[test]
fn a_nav_without_sections_answers_nothing_and_never_panics() {
    let mut nav = Nav::new(Vec::new());
    assert_eq!(nav.place(), place(0, 0, 0));
    assert_eq!(nav.count(), 0);
    for key in [
        Key::Tab,
        Key::BackTab,
        Key::Char(']'),
        Key::Char('1'),
        Key::Esc,
    ] {
        assert_eq!(nav.key(key), None);
    }
    nav.open("ignored", 1);
    assert_eq!(nav.depth(), 0);
    assert_eq!(nav.breadcrumb(), "");
}

#[test]
fn empty_groups_are_skipped_by_tab() {
    let mut nav = Nav::new(vec![
        Group::new("One").section(Section::new("A")),
        Group::new("Empty"),
        Group::new("Two").section(Section::new("B")),
    ]);
    assert_eq!(nav.key(Key::Tab), Some(Step::Moved(place(2, 0, 2))));
    assert_eq!(nav.key(Key::Tab), Some(Step::Moved(place(0, 0, 1))));
    assert_eq!(nav.go_to(1, 0), None);
}

#[cfg(feature = "crossterm")]
#[test]
fn crossterm_keys_convert() {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
    let key = |code, modifiers| Key::from(KeyEvent::new(code, modifiers));
    assert_eq!(
        key(KeyCode::Char('c'), KeyModifiers::CONTROL),
        Key::Ctrl('c')
    );
    assert_eq!(
        key(KeyCode::Char('C'), KeyModifiers::CONTROL),
        Key::Ctrl('c')
    );
    assert_eq!(key(KeyCode::Char('Q'), KeyModifiers::SHIFT), Key::Char('Q'));
    assert_eq!(key(KeyCode::Tab, KeyModifiers::SHIFT), Key::BackTab);
    assert_eq!(key(KeyCode::BackTab, KeyModifiers::SHIFT), Key::BackTab);
    assert_eq!(key(KeyCode::Backspace, KeyModifiers::NONE), Key::Backspace);
    assert_eq!(key(KeyCode::F(5), KeyModifiers::NONE), Key::Other);
    let mut release = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
    release.kind = KeyEventKind::Release;
    assert_eq!(Key::from(release), Key::Other);
}
