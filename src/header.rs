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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header<'a> {
    nav: &'a Nav,
    styles: Styles,
    title: Option<&'a str>,
    left: Option<Fact<'a>>,
    right: Option<Fact<'a>>,
    tabs: bool,
    lit: bool,
    underline: bool,
    facts: &'a [Fact<'a>],
    facts_rule: bool,
    separator: &'a str,
    crumbs: bool,
    closing: bool,
    notice: Option<Fact<'a>>,
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
            left: None,
            right: None,
            tabs: true,
            lit: true,
            underline: false,
            facts: &[],
            facts_rule: false,
            separator: SEPARATOR,
            crumbs: false,
            closing: false,
            notice: None,
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
        self.left = left.filter(|fact| !fact.text.is_empty());
        self
    }

    pub fn right(mut self, right: Option<Fact<'a>>) -> Self {
        self.right = right.filter(|fact| !fact.text.is_empty());
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

    pub fn height(&self) -> u16 {
        u16::from(self.title.is_some())
            + if self.tabs { self.bar().height() } else { 0 }
            + u16::from(!self.facts.is_empty())
            + u16::from(self.crumbs)
            + u16::from(self.closing)
            + u16::from(self.notice.is_some())
    }

    pub fn hit(&self, area: Rect, column: u16, row: u16) -> Option<Place> {
        self.bar().hit(self.rows(area).tabs?, column, row)
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
            facts: take(!self.facts.is_empty(), 1),
            crumbs: take(self.crumbs, 1),
            closing: take(self.closing, 1),
            notice: take(self.notice.is_some(), 1),
        }
    }

    fn draw_title(&self, buf: &mut Buffer, line: Rect, title: &str) {
        let Some(mut pen) = Pen::new(buf, line, line.x, line.y) else {
            return;
        };
        pen.fill(RULE, self.styles.rule);
        let lit = self.styles.lit();
        let wide = text::width(title).saturating_add(2).min(line.width);
        let start = line.x + (line.width - wide) / 2;
        if let Some(mut pen) = Pen::new(buf, line, start, line.y) {
            pen.put(" ", lit);
            pen.clip(title, lit, wide.saturating_sub(2));
            pen.put(" ", lit);
        }
        let side = start.saturating_sub(line.x).saturating_sub(2);
        if side < SIDE_MIN {
            return;
        }
        if let Some(left) = self.left
            && let Some(mut pen) = Pen::new(buf, line, line.x + 1, line.y)
        {
            pen.put(" ", left.style);
            pen.clip(left.text, left.style, side - 2);
            pen.put(" ", left.style);
        }
        if let Some(right) = self.right {
            let wide = text::width(right.text).min(side - 2) + 2;
            let x = line.right().saturating_sub(1 + wide);
            if let Some(mut pen) = Pen::new(buf, line, x, line.y) {
                pen.put(" ", right.style);
                pen.clip(right.text, right.style, wide - 2);
                pen.put(" ", right.style);
            }
        }
    }

    fn draw_facts(&self, buf: &mut Buffer, line: Rect) {
        if self.facts_rule
            && let Some(mut pen) = Pen::new(buf, line, line.x, line.y)
        {
            pen.fill(RULE, self.styles.rule);
        }
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
        let Some(mut pen) = Pen::new(buf, line, start, line.y) else {
            return;
        };
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
        }
    }

    fn draw_notice(&self, buf: &mut Buffer, line: Rect, notice: Fact) {
        if let Some(mut pen) = Pen::new(buf, line, line.x, line.y) {
            let room = pen.room();
            pen.clip(notice.text, notice.style, room);
        }
    }
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
            self.bar().render(tabs, buf);
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
