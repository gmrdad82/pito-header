use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::nav::{Breadcrumb, Nav, NavBar, Place, RULE};
use crate::styles::Styles;
use crate::text::{self, ELLIPSIS, Pen};

const SEPARATOR: &str = " · ";
const SIDE_MIN: u16 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fact<'a> {
    pub text: &'a str,
    pub style: Style,
}

impl<'a> Fact<'a> {
    pub const fn new(text: &'a str, style: Style) -> Self {
        Fact { text, style }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Slot<'a> {
    #[default]
    Empty,
    One(Fact<'a>),
    Parts(&'a [Fact<'a>]),
}

impl<'a> Slot<'a> {
    fn parts(&self) -> &[Fact<'a>] {
        match self {
            Slot::Empty => &[],
            Slot::One(fact) => std::slice::from_ref(fact),
            Slot::Parts(parts) => parts,
        }
    }

    fn width(&self) -> u16 {
        parts_width(self.parts())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Drill {
    #[default]
    Replace,
    Rows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header<'a> {
    nav: &'a Nav,
    styles: Styles,
    title: Option<&'a str>,
    left: Slot<'a>,
    right: Slot<'a>,
    tabs: bool,
    lit: bool,
    underline: bool,
    facts: &'a [Fact<'a>],
    facts_rule: bool,
    separator: &'a str,
    crumbs: bool,
    closing: bool,
    notice: Option<Fact<'a>>,
    drill: Drill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rows {
    title: Option<Rect>,
    tabs: Option<Rect>,
    facts: Option<Rect>,
    crumbs: Option<Rect>,
    closing: Option<Rect>,
    notice: Option<Rect>,
}

impl<'a> Header<'a> {
    pub fn new(nav: &'a Nav) -> Self {
        Header {
            nav,
            styles: Styles::new(),
            title: None,
            left: Slot::Empty,
            right: Slot::Empty,
            tabs: true,
            lit: true,
            underline: false,
            facts: &[],
            facts_rule: false,
            separator: SEPARATOR,
            crumbs: false,
            closing: false,
            notice: None,
            drill: Drill::Replace,
        }
    }

    pub fn styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        self
    }

    pub fn title(mut self, title: Option<&'a str>) -> Self {
        self.title = title;
        self
    }

    pub fn left(mut self, left: Option<Fact<'a>>) -> Self {
        self.left = slot(left.map_or(Slot::Empty, Slot::One));
        self
    }

    pub fn right(mut self, right: Option<Fact<'a>>) -> Self {
        self.right = slot(right.map_or(Slot::Empty, Slot::One));
        self
    }

    pub fn left_parts(mut self, parts: &'a [Fact<'a>]) -> Self {
        self.left = slot(Slot::Parts(parts));
        self
    }

    pub fn right_parts(mut self, parts: &'a [Fact<'a>]) -> Self {
        self.right = slot(Slot::Parts(parts));
        self
    }

    pub fn tabs(mut self, tabs: bool) -> Self {
        self.tabs = tabs;
        self
    }

    pub fn lit(mut self, lit: bool) -> Self {
        self.lit = lit;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn facts(mut self, facts: &'a [Fact<'a>]) -> Self {
        self.facts = facts;
        self
    }

    pub fn facts_rule(mut self, facts_rule: bool) -> Self {
        self.facts_rule = facts_rule;
        self
    }

    pub fn separator(mut self, separator: &'a str) -> Self {
        self.separator = separator;
        self
    }

    pub fn crumbs(mut self, crumbs: bool) -> Self {
        self.crumbs = crumbs;
        self
    }

    pub fn closing(mut self, closing: bool) -> Self {
        self.closing = closing;
        self
    }

    pub fn notice(mut self, notice: Option<Fact<'a>>) -> Self {
        self.notice = notice.filter(|fact| !fact.text.is_empty());
        self
    }

    pub fn drill(mut self, drill: Drill) -> Self {
        self.drill = drill;
        self
    }

    pub fn height(&self) -> u16 {
        let replaced = self.replaced();
        u16::from(self.title.is_some())
            + if self.tabs { self.bar().height() } else { 0 }
            + u16::from(!self.facts.is_empty() && !replaced)
            + u16::from(self.crumbs && !replaced)
            + u16::from(self.closing)
            + u16::from(self.notice.is_some())
    }

    pub fn hit(&self, area: Rect, column: u16, row: u16) -> Option<Place> {
        let tabs = self.rows(area).tabs?;
        let bar = self.bar();
        if self.replaced() && bar.rows(tabs).1.is_some_and(|line| line.y == row) {
            return None;
        }
        bar.hit(tabs, column, row)
    }

    fn replaced(&self) -> bool {
        self.drill == Drill::Replace && self.tabs && self.nav.depth() > 0
    }

    fn bar(&self) -> NavBar<'a> {
        NavBar::new(self.nav)
            .styles(self.styles)
            .lit(self.lit)
            .underline(self.underline)
    }

    fn rows(&self, area: Rect) -> Rows {
        let mut y = area.y;
        let mut take = |on: bool, height: u16| {
            if !on || height == 0 || y >= area.bottom() {
                return None;
            }
            let height = height.min(area.bottom() - y);
            let row = Rect::new(area.x, y, area.width, height);
            y += height;
            Some(row)
        };
        Rows {
            title: take(self.title.is_some(), 1),
            tabs: take(self.tabs, self.bar().height()),
            facts: take(!self.facts.is_empty() && !self.replaced(), 1),
            crumbs: take(self.crumbs && !self.replaced(), 1),
            closing: take(self.closing, 1),
            notice: take(self.notice.is_some(), 1),
        }
    }

    fn draw_title(&self, buf: &mut Buffer, line: Rect, title: &str) {
        let Some(mut pen) = Pen::new(buf, line, line.x, line.y) else {
            return;
        };
        let rule = self.styles.rule;
        let lit = self.styles.lit();
        let wide = text::width(title).saturating_add(2).min(line.width);
        let start = line.x + (line.width - wide) / 2;
        let side = start.saturating_sub(line.x).saturating_sub(2);
        let sides = side >= SIDE_MIN;
        if sides && self.left != Slot::Empty {
            pen.fill_to(line.x + 1, RULE, rule);
            label(&mut pen, self.left.parts(), side - 2);
        }
        pen.fill_to(start, RULE, rule);
        label(&mut pen, &[Fact::new(title, lit)], wide.saturating_sub(2));
        if sides && self.right != Slot::Empty {
            let wide = self.right.width().min(side - 2) + 2;
            pen.fill_to(line.right().saturating_sub(1 + wide), RULE, rule);
            label(&mut pen, self.right.parts(), wide - 2);
        }
        pen.fill(RULE, rule);
    }

    fn draw_tabs(&self, buf: &mut Buffer, tabs: Rect) {
        let bar = self.bar();
        if !self.replaced() {
            return bar.render(tabs, buf);
        }
        let (groups, sections) = bar.rows(tabs);
        if let Some(line) = groups {
            bar.draw_groups(buf, line);
        }
        if let Some(line) = sections {
            Breadcrumb::new(self.nav)
                .styles(self.styles)
                .centred(true)
                .render(line, buf);
        }
    }

    fn draw_facts(&self, buf: &mut Buffer, line: Rect) {
        let pad = if self.facts_rule { 1 } else { 0 };
        let separator = text::width(self.separator);
        let inner = self
            .facts
            .iter()
            .enumerate()
            .fold(0u16, |total, (index, fact)| {
                let gap = if index > 0 { separator } else { 0 };
                total
                    .saturating_add(gap)
                    .saturating_add(text::width(fact.text))
            });
        let shown = inner.min(line.width.saturating_sub(4 * pad));
        let block = shown + 2 * pad;
        let start = line.x + line.width.saturating_sub(block) / 2;
        let rule = self.styles.rule;
        let Some(mut pen) = Pen::new(buf, line, line.x, line.y) else {
            return;
        };
        if self.facts_rule {
            pen.fill_to(start, RULE, rule);
        } else {
            pen.skip(start - line.x);
        }
        let muted = self.styles.muted;
        if pad > 0 {
            pen.put(" ", muted);
        }
        let end = pen.x.saturating_add(shown);
        for (index, fact) in self.facts.iter().enumerate() {
            if index > 0 {
                if end.saturating_sub(pen.x) < separator + 2 {
                    pen.clip(ELLIPSIS, muted, end.saturating_sub(pen.x));
                    break;
                }
                pen.put(self.separator, muted);
            }
            pen.clip(fact.text, fact.style, end.saturating_sub(pen.x));
        }
        if pad > 0 {
            pen.put(" ", muted);
            pen.fill(RULE, rule);
        }
    }

    fn draw_notice(&self, buf: &mut Buffer, line: Rect, notice: Fact) {
        if let Some(mut pen) = Pen::new(buf, line, line.x, line.y) {
            let room = pen.room();
            pen.clip(notice.text, notice.style, room);
        }
    }
}

fn slot(slot: Slot) -> Slot {
    if slot.width() == 0 { Slot::Empty } else { slot }
}

fn parts_width(parts: &[Fact]) -> u16 {
    parts.iter().fold(0u16, |total, part| {
        total.saturating_add(text::width(part.text))
    })
}

fn label(pen: &mut Pen, parts: &[Fact], room: u16) {
    let (Some(first), Some(last)) = (parts.first(), parts.last()) else {
        return;
    };
    pen.put(" ", first.style);
    if parts_width(parts) <= room {
        for part in parts {
            pen.put(part.text, part.style);
        }
    } else if room > 0 {
        let stop = pen.x.saturating_add(room - 1);
        let mut style = first.style;
        for part in parts {
            if pen.x >= stop {
                break;
            }
            style = part.style;
            let start = pen.x;
            pen.putn(part.text, part.style, stop - pen.x);
            if pen.x - start < text::width(part.text) {
                break;
            }
        }
        pen.x = pen.x.min(stop);
        pen.put(ELLIPSIS, style);
    }
    pen.put(" ", last.style);
}

impl Widget for Header<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &Header<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = self.rows(area);
        if let (Some(line), Some(title)) = (rows.title, self.title) {
            self.draw_title(buf, line, title);
        }
        if let Some(tabs) = rows.tabs {
            self.draw_tabs(buf, tabs);
        }
        if let Some(line) = rows.facts {
            self.draw_facts(buf, line);
        }
        if let Some(line) = rows.crumbs {
            Breadcrumb::new(self.nav)
                .styles(self.styles)
                .render(line, buf);
        }
        if let Some(line) = rows.closing
            && let Some(mut pen) = Pen::new(buf, line, line.x, line.y)
        {
            pen.fill(RULE, self.styles.rule);
        }
        if let (Some(line), Some(notice)) = (rows.notice, self.notice) {
            self.draw_notice(buf, line, notice);
        }
    }
}
