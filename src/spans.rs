use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub fn truncate_spans<'a>(spans: Vec<Span<'a>>, max_width: usize) -> TruncateSpans<'a> {
    TruncateSpans {
        spans,
        max_width,
        ..Default::default()
    }
}

#[derive(Default)]
pub struct TruncateSpans<'a> {
    spans: Vec<Span<'a>>,
    max_width: usize,
    ellipsis: &'a str,
    ellipsis_style: Style,
}

impl<'a> TruncateSpans<'a> {
    pub fn ellipsis(mut self, ellipsis: &'a str) -> Self {
        self.ellipsis = ellipsis;
        self
    }

    pub fn ellipsis_style(mut self, style: Style) -> Self {
        self.ellipsis_style = style;
        self
    }

    pub fn ellipsis_fg(mut self, color: Color) -> Self {
        self.ellipsis_style = self.ellipsis_style.fg(color);
        self
    }

    pub fn ellipsis_bg(mut self, color: Color) -> Self {
        self.ellipsis_style = self.ellipsis_style.bg(color);
        self
    }

    pub fn ellipsis_modifier(mut self, modifier: Modifier) -> Self {
        self.ellipsis_style = self.ellipsis_style.add_modifier(modifier);
        self
    }

    pub fn into_spans(self) -> Vec<Span<'a>> {
        let total_spans = self.spans.len();
        let span_widths: Vec<usize> = self
            .spans
            .iter()
            .map(|s| console::measure_text_width(&s.content))
            .collect();

        if span_widths.iter().sum::<usize>() <= self.max_width {
            return self.spans;
        }

        let ellipsis_width = console::measure_text_width(self.ellipsis);
        if ellipsis_width >= self.max_width {
            let truncated_ellipsis = console::truncate_str(self.ellipsis, self.max_width, "");
            return vec![Span::from(truncated_ellipsis).style(self.ellipsis_style)];
        }

        let mut rest_w = self.max_width;
        rest_w -= ellipsis_width;

        let mut ret = Vec::new();
        let mut exceed = false;
        for (i, span) in self.spans.into_iter().enumerate() {
            let w = span_widths[i];
            ret.push(span);
            if w > rest_w {
                exceed = true;
                break;
            }
            rest_w -= w;
        }

        if !exceed && ret.len() == total_spans {
            return ret;
        }

        let last_span = ret.pop().unwrap();
        let truncated = console::truncate_str(&last_span.content, rest_w, "").to_string();

        if !truncated.is_empty() {
            ret.push(Span::from(truncated).style(last_span.style));
        }

        if !self.ellipsis.is_empty() {
            ret.push(Span::from(self.ellipsis).style(self.ellipsis_style));
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(1, "", vec![Span::raw("a")])]
    #[case(2, "", vec![Span::raw("ab")])]
    #[case(3, "", vec![Span::raw("abc")])]
    #[case(4, "", vec![Span::raw("abc"), Span::raw("d")])]
    #[case(5, "", vec![Span::raw("abc"), Span::raw("de")])]
    #[case(6, "", vec![Span::raw("abc"), Span::raw("def")])]
    #[case(7, "", vec![Span::raw("abc"), Span::raw("def"), Span::raw("g")])]
    #[case(8, "", vec![Span::raw("abc"), Span::raw("def"), Span::raw("gh")])]
    #[case(9, "", vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")])]
    #[case(10, "", vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")])]
    #[case(4, "..", vec![Span::raw("ab"), Span::raw("..")])]
    #[case(5, "..", vec![Span::raw("abc"), Span::raw("..")])]
    #[case(6, "..", vec![Span::raw("abc"), Span::raw("d"), Span::raw("..")])]
    #[case(7, "..", vec![Span::raw("abc"), Span::raw("de"), Span::raw("..")])]
    #[case(8, "..", vec![Span::raw("abc"), Span::raw("def"), Span::raw("..")])]
    #[case(9, "..", vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")])]
    #[case(10, "..", vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")])]
    #[case(1, "...", vec![Span::raw(".")])]
    #[case(2, "...", vec![Span::raw("..")])]
    #[case(3, "...", vec![Span::raw("...")])]
    #[case(4, "...", vec![Span::raw("a"), Span::raw("...")])]
    #[case(5, "...", vec![Span::raw("ab"), Span::raw("...")])]
    #[case(6, "...", vec![Span::raw("abc"), Span::raw("...")])]
    #[case(7, "...", vec![Span::raw("abc"), Span::raw("d"), Span::raw("...")])]
    #[case(8, "...", vec![Span::raw("abc"), Span::raw("de"), Span::raw("...")])]
    #[case(9, "...", vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")])]
    #[case(10, "...", vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")])]
    #[case(6, "....", vec![Span::raw("ab"), Span::raw("....")])]
    #[case(7, "....", vec![Span::raw("abc"), Span::raw("....")])]
    #[case(8, "....", vec![Span::raw("abc"), Span::raw("d"), Span::raw("....")])]
    #[case(9, "....", vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")])]
    #[case(10, "....", vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")])]
    fn test_truncate_spans(
        #[case] max_width: usize,
        #[case] ellipsis: &str,
        #[case] expected: Vec<Span>,
    ) {
        let spans = vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")];
        let actual = truncate_spans(spans, max_width)
            .ellipsis(ellipsis)
            .into_spans();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(1, vec![
        Span::styled(".", ellipsis_style()),
    ])]
    #[case(3, vec![
        Span::styled("a", style1()),
        Span::styled("..", ellipsis_style()),
    ])]
    #[case(5, vec![
        Span::styled("abc", style1()),
        Span::styled("..", ellipsis_style()),
    ])]
    #[case(7, vec![
        Span::styled("abc", style1()),
        Span::styled("de", style2()),
        Span::styled("..", ellipsis_style()),
    ])]
    #[case(8, vec![
        Span::styled("abc", style1()),
        Span::styled("def", style2()),
        Span::styled("..", ellipsis_style()),
    ])]
    #[case(9, vec![
        Span::styled("abc", style1()),
        Span::styled("def", style2()),
        Span::styled("ghi", style3()),
    ])]
    fn test_truncate_spans_with_style(#[case] max_width: usize, #[case] expected: Vec<Span>) {
        let spans = vec![
            Span::styled("abc", style1()),
            Span::styled("def", style2()),
            Span::styled("ghi", style3()),
        ];
        let actual = truncate_spans(spans, max_width)
            .ellipsis("..")
            .ellipsis_style(ellipsis_style())
            .into_spans();
        assert_eq!(actual, expected);
    }

    fn style1() -> Style {
        style(Color::Red, Color::Cyan, Modifier::BOLD)
    }

    fn style2() -> Style {
        style(Color::Green, Color::Magenta, Modifier::ITALIC)
    }

    fn style3() -> Style {
        style(Color::Blue, Color::Yellow, Modifier::UNDERLINED)
    }

    fn ellipsis_style() -> Style {
        style(Color::White, Color::Black, Modifier::DIM)
    }

    fn style(fg: Color, bg: Color, modifier: Modifier) -> Style {
        Style::default().fg(fg).bg(bg).add_modifier(modifier)
    }
}
