use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub fn highlight_matched_text<'a, T>(t: T) -> HigilightMatchedText<'a>
where
    T: Into<Vec<Span<'a>>>,
{
    HigilightMatchedText {
        spans: t.into(),
        ..Default::default()
    }
}

#[derive(Default)]
pub struct HigilightMatchedText<'a> {
    spans: Vec<Span<'a>>,
    matches: Vec<Range>,
    not_matched_style: Style,
    matched_style: Style,
    ellipsis: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl<'a> HigilightMatchedText<'a> {
    pub fn matched_indices(mut self, indices: Vec<usize>) -> Self {
        self.matches = to_ranges(indices);
        self
    }

    pub fn matched_range(mut self, start: usize, end: usize) -> Self {
        self.matches = vec![Range::new(start, end)];
        self
    }

    pub fn not_matched_style(mut self, style: Style) -> Self {
        self.not_matched_style = style;
        self
    }

    pub fn not_matched_fg(mut self, color: Color) -> Self {
        self.not_matched_style = self.not_matched_style.fg(color);
        self
    }

    pub fn not_matched_bg(mut self, color: Color) -> Self {
        self.not_matched_style = self.not_matched_style.bg(color);
        self
    }

    pub fn not_matched_modifier(mut self, modifier: Modifier) -> Self {
        self.not_matched_style = self.not_matched_style.add_modifier(modifier);
        self
    }

    pub fn matched_style(mut self, style: Style) -> Self {
        self.matched_style = style;
        self
    }

    pub fn matched_fg(mut self, color: Color) -> Self {
        self.matched_style = self.matched_style.fg(color);
        self
    }

    pub fn matched_bg(mut self, color: Color) -> Self {
        self.matched_style = self.matched_style.bg(color);
        self
    }

    pub fn matched_modifier(mut self, modifier: Modifier) -> Self {
        self.matched_style = self.matched_style.add_modifier(modifier);
        self
    }

    pub fn ellipsis(mut self, ellipsis: impl Into<String>) -> Self {
        self.ellipsis = Some(ellipsis.into());
        self
    }

    pub fn into_spans(self) -> Vec<Span<'static>> {
        if self.spans.is_empty() {
            return vec![];
        }

        let total_len: usize = self.spans.iter().map(|s| s.content.len()).sum();

        let (matches_to_use, limit, ellipsis_s) = if let Some(ellipsis) = self.ellipsis {
            let ellipsis_len = ellipsis.len();
            let limit = total_len.saturating_sub(ellipsis_len);
            let mut tmp_matches = self.matches.clone();

            let mut broken = false;
            for (i, r) in self.matches.iter().enumerate() {
                if limit < r.end {
                    let mut new_temp_matches = self.matches[..i].to_vec();
                    if r.start < limit {
                        new_temp_matches.push(Range::new(r.start, total_len));
                    } else {
                        new_temp_matches.push(Range::new(limit, total_len));
                    }
                    tmp_matches = new_temp_matches;
                    broken = true;
                    break;
                }
            }

            if broken {
                let indices: Vec<usize> = tmp_matches.iter().flat_map(|r| r.start..r.end).collect();
                (to_ranges(indices), limit, Some(ellipsis))
            } else {
                (self.matches.clone(), limit, Some(ellipsis))
            }
        } else {
            (self.matches.clone(), total_len, None)
        };

        let mut result_spans = Vec::new();
        let mut current_pos = 0;

        for span in &self.spans {
            if current_pos >= limit {
                break;
            }
            let span_len = span.content.len();
            let effective_span_end = (current_pos + span_len).min(limit);

            let original_style = span.style;
            let mut span_cursor = 0;

            while current_pos + span_cursor < effective_span_end {
                let current_abs_pos = current_pos + span_cursor;

                let next_break = find_next_break(current_abs_pos, &matches_to_use)
                    .unwrap_or(effective_span_end)
                    .min(effective_span_end);

                let end_in_span = next_break - current_pos;

                let content_slice = &span.content[span_cursor..end_in_span];

                if content_slice.is_empty() {
                    span_cursor = end_in_span;
                    continue;
                }

                let is_matched = matches_to_use
                    .iter()
                    .any(|r| r.start <= current_abs_pos && current_abs_pos < r.end);
                let style = if is_matched {
                    original_style.patch(self.matched_style)
                } else {
                    original_style.patch(self.not_matched_style)
                };

                result_spans.push(Span::styled(content_slice.to_string(), style));
                span_cursor = end_in_span;
            }
            current_pos += span_len;
        }

        if let Some(ellipsis) = ellipsis_s {
            let ellipsis_start_pos = limit;
            let is_matched = matches_to_use
                .iter()
                .any(|r| r.start <= ellipsis_start_pos && ellipsis_start_pos < r.end);
            let style = if is_matched {
                self.matched_style
            } else {
                self.not_matched_style
            };
            result_spans.push(Span::styled(ellipsis, style));
        }

        result_spans
    }
}

fn find_next_break(pos: usize, matches: &[Range]) -> Option<usize> {
    matches
        .iter()
        .flat_map(|r| [r.start, r.end])
        .filter(|&b| b > pos)
        .min()
}

fn to_ranges(indices: Vec<usize>) -> Vec<Range> {
    if indices.is_empty() {
        return Vec::new();
    }
    let indices = sort_and_dedup(indices);

    let mut ranges = Vec::new();
    let mut start = indices[0];
    let mut end = indices[0] + 1;
    for i in indices.into_iter().skip(1) {
        if i == end {
            end = i + 1;
        } else {
            ranges.push(Range::new(start, end));
            start = i;
            end = i + 1;
        }
    }
    ranges.push(Range::new(start, end));
    ranges
}

fn sort_and_dedup(mut indices: Vec<usize>) -> Vec<usize> {
    indices.sort_unstable();
    indices.dedup();
    indices
}

#[cfg(test)]
mod tests {
    use ratatui::style::{Color, Modifier};
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(vec![], vec![])]
    #[case(vec![1], vec![Range::new(1, 2)])]
    #[case(vec![1, 2, 3], vec![Range::new(1, 4)])]
    #[case(vec![1, 3, 5], vec![Range::new(1, 2), Range::new(3, 4), Range::new(5, 6)])]
    #[case(vec![1, 2, 3, 5, 6, 7, 9, 10, 11], vec![Range::new(1, 4), Range::new(5, 8), Range::new(9, 12)])]
    #[case(vec![5, 10, 2, 3, 3, 7, 9, 5, 10, 1, 11, 6], vec![Range::new(1, 4), Range::new(5, 8), Range::new(9, 12)])]
    fn test_to_ranges(#[case] indices: Vec<usize>, #[case] expected: Vec<Range>) {
        let ranges = to_ranges(indices);
        assert_eq!(ranges, expected);
    }

    #[test]
    fn test_highlight_matched_text_matched_indices() {
        let s = "abcdefghijklmn";
        let actual = highlight_matched_text(vec![s.into()])
            .matched_indices(vec![2, 3, 4, 7, 9, 10]) // "cde", "h", "jk"
            .into_spans();
        let expected = vec![
            Span::raw("ab"),
            Span::raw("cde"),
            Span::raw("fg"),
            Span::raw("h"),
            Span::raw("i"),
            Span::raw("jk"),
            Span::raw("lmn"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_matched_text_matched_range() {
        let s = "abcdef";
        let actual = highlight_matched_text(vec![s.into()])
            .matched_range(2, 4) // "cd"
            .into_spans();
        let expected = vec![Span::raw("ab"), Span::raw("cd"), Span::raw("ef")];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_matched_text_styles() {
        let s = "abcdef";
        let not_matched_style = Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::ITALIC);
        let matched_style = Style::default()
            .fg(Color::Yellow)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD);
        let actual = highlight_matched_text(vec![s.into()])
            .matched_indices(vec![0, 1, 5]) // "ab", "f"
            .not_matched_style(not_matched_style)
            .matched_style(matched_style)
            .into_spans();
        let expected = vec![
            Span::styled("ab", matched_style),
            Span::styled("cde", not_matched_style),
            Span::styled("f", matched_style),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_matched_text_ellipsis_1() {
        let s = "abcdef...";
        let not_matched_style = Style::default();
        let matched_style = Style::default().fg(Color::Red);
        let actual = highlight_matched_text(vec![s.into()])
            .matched_indices(vec![3, 4, 5]) // "def"
            .not_matched_style(not_matched_style)
            .matched_style(matched_style)
            .ellipsis("...")
            .into_spans();
        let expected = vec![
            Span::styled("abc", not_matched_style),
            Span::styled("def", matched_style),
            Span::styled("...", not_matched_style),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_matched_text_ellipsis_2() {
        let s = "abcdef...";
        let not_matched_style = Style::default();
        let matched_style = Style::default().fg(Color::Red);
        let actual = highlight_matched_text(vec![s.into()])
            .matched_indices(vec![3, 4, 5, 6]) // "def."
            .not_matched_style(not_matched_style)
            .matched_style(matched_style)
            .ellipsis("...")
            .into_spans();
        let expected = vec![
            Span::styled("abc", not_matched_style),
            Span::styled("def", matched_style),
            Span::styled("...", matched_style),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_matched_text_ellipsis_3() {
        let s = "abcdef...";
        let not_matched_style = Style::default();
        let matched_style = Style::default().fg(Color::Red);
        let actual = highlight_matched_text(vec![s.into()])
            .matched_indices(vec![0, 1, 7, 10, 11]) // "ab", ".", "??"
            .not_matched_style(not_matched_style)
            .matched_style(matched_style)
            .ellipsis("...")
            .into_spans();
        let expected = vec![
            Span::styled("ab", matched_style),
            Span::styled("cdef", not_matched_style),
            Span::styled("...", matched_style),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_matched_text_ellipsis_4() {
        let s = "abcdef...";
        let not_matched_style = Style::default();
        let matched_style = Style::default().fg(Color::Red);
        let actual = highlight_matched_text(vec![s.into()])
            .matched_indices(vec![3, 4, 5, 9, 10, 11]) // "def", "???"
            .not_matched_style(not_matched_style)
            .matched_style(matched_style)
            .ellipsis("...")
            .into_spans();
        let expected = vec![
            Span::styled("abc", not_matched_style),
            Span::styled("def", matched_style),
            Span::styled("...", matched_style),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_with_spans_no_style() {
        let s = vec![Span::raw("abc"), Span::raw("def"), Span::raw("ghi")];
        let actual = highlight_matched_text(s)
            .matched_indices(vec![1, 2, 5, 6]) // "bc", "fg"
            .into_spans();
        let expected = vec![
            Span::raw("a"),
            Span::raw("bc"),
            Span::raw("de"),
            Span::raw("f"),
            Span::raw("g"),
            Span::raw("hi"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_with_spans_with_style() {
        let s = vec![
            Span::styled(
                "abc",
                Style::default()
                    .fg(Color::Blue)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "def",
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ),
            Span::styled(
                "ghi",
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ];
        let not_matched_style = Style::default().fg(Color::DarkGray);
        let matched_style = Style::default().bg(Color::Yellow);

        let actual = highlight_matched_text(s.clone())
            .matched_indices(vec![1, 2, 5, 6]) // "bc", "fg"
            .not_matched_style(not_matched_style)
            .matched_style(matched_style)
            .into_spans();

        let expected = vec![
            Span::styled("a", s[0].style.patch(not_matched_style)),
            Span::styled("bc", s[0].style.patch(matched_style)),
            Span::styled("de", s[1].style.patch(not_matched_style)),
            Span::styled("f", s[1].style.patch(matched_style)),
            Span::styled("g", s[2].style.patch(matched_style)),
            Span::styled("hi", s[2].style.patch(not_matched_style)),
        ];
        assert_eq!(actual, expected);
    }
}
