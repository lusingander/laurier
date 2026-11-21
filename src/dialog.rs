use ratatui_core::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Color,
    widgets::Widget,
};

type DialogContent<'a> = Box<dyn FnOnce(Rect, &mut Buffer) + 'a>;

pub struct Dialog<'a> {
    content: DialogContent<'a>,
    margin: Margin,
    bg: Color,
}

impl<'a> Dialog<'a> {
    pub fn new<W: Widget + 'a>(content: W) -> Dialog<'a> {
        Dialog {
            content: Box::new(move |area, buf| content.render(area, buf)),
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

impl Widget for Dialog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_dialog(area, buf);
    }
}

impl Dialog<'_> {
    fn render_dialog(self, area: Rect, buf: &mut Buffer) {
        let outer = outer_rect(area, self.margin).intersection(*buf.area());
        for y in outer.top()..outer.bottom() {
            for x in outer.left()..outer.right() {
                let cell = &mut buf[(x, y)];
                cell.reset();
                cell.set_bg(self.bg);
            }
        }
        (self.content)(area, buf);
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

#[cfg(test)]
mod tests {
    use ratatui_core::{
        style::{Modifier, Style},
        text::Span,
    };

    use super::*;

    #[test]
    fn test_render_clears_margin_and_preserves_content_style() {
        let mut buffer = Buffer::with_lines(["xxxxxxxx"; 5]);
        buffer.set_style(
            *buffer.area(),
            Style::default()
                .fg(Color::Red)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );
        buffer[(1, 1)].set_skip(true);

        Dialog::new(Span::styled("ok", Style::default().fg(Color::Yellow)))
            .margin(Margin::new(1, 1))
            .bg(Color::Blue)
            .render(Rect::new(2, 2, 2, 1), &mut buffer);

        let mut expected =
            Buffer::with_lines(["xxxxxxxx", "x    xxx", "x ok xxx", "x    xxx", "xxxxxxxx"]);
        expected.set_style(
            *expected.area(),
            Style::default()
                .fg(Color::Red)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );
        expected.set_style(
            Rect::new(1, 1, 4, 3),
            Style::default()
                .fg(Color::Reset)
                .bg(Color::Blue)
                .remove_modifier(Modifier::BOLD),
        );
        expected.set_style(Rect::new(2, 2, 2, 1), Style::default().fg(Color::Yellow));
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_clips_margin_to_buffer() {
        let mut buffer = Buffer::with_lines(["xxxx"; 3]);

        Dialog::new(Span::raw("ok"))
            .margin(Margin::new(1, 1))
            .bg(Color::Blue)
            .render(Rect::new(2, 2, 2, 1), &mut buffer);

        let mut expected = Buffer::with_lines(["xxxx", "x   ", "x ok"]);
        expected.set_style(Rect::new(1, 1, 3, 2), Style::default().bg(Color::Blue));
        assert_eq!(buffer, expected);
    }
}
