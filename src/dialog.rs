use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Stylize},
    widgets::{Block, Clear, Widget, WidgetRef},
};

pub struct Dialog<'a> {
    content: Box<dyn WidgetRef + 'a>,
    margin: Margin,
    bg: Color,
}

impl<'a> Dialog<'a> {
    pub fn new(content: Box<dyn WidgetRef + 'a>) -> Dialog<'a> {
        Dialog {
            content,
            margin: Margin::default(),
            bg: Color::default(),
        }
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }
}

impl WidgetRef for Dialog<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.render_dialog(area, buf);
    }
}

impl Dialog<'_> {
    fn render_dialog(&self, area: Rect, buf: &mut Buffer) {
        let outer = outer_rect(area, self.margin);
        Clear.render(outer, buf);
        Block::default().bg(self.bg).render(outer, buf);
        self.content.render_ref(area, buf);
    }
}

fn outer_rect(r: Rect, margin: Margin) -> Rect {
    let doubled_margin_horizontal = margin.horizontal.saturating_mul(2);
    let doubled_margin_vertical = margin.vertical.saturating_mul(2);
    Rect {
        x: r.x.saturating_sub(margin.horizontal),
        y: r.y.saturating_sub(margin.vertical),
        width: r.width.saturating_add(doubled_margin_horizontal),
        height: r.height.saturating_add(doubled_margin_vertical),
    }
}
