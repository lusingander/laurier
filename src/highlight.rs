use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub fn highlight_matched_text(s: &str) -> HigilightMatchedText {
    HigilightMatchedText {
        s,
        matches: Vec::new(),
        not_matched_style: Style::default(),
        matched_style: Style::default().fg(Color::Red),
        ellipsis: None,
    }
}

pub struct HigilightMatchedText<'a> {
    s: &'a str,
    matches: Vec<Range>,
    not_matched_style: Style,
    matched_style: Style,
    ellipsis: Option<&'a str>,
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

    pub fn matched_style(mut self, style: Style) -> Self {
        self.matched_style = style;
        self
    }

    pub fn ellipsis(mut self, ellipsis: &'a str) -> Self {
        self.ellipsis = Some(ellipsis);
        self
    }

    pub fn into_spans(self) -> Vec<Span<'a>> {
        let mut matches;
        let (s, matches) = if let Some(e) = self.ellipsis {
            matches = Vec::new();
            let el = e.len();
            let sl = self.s.len();
            for r in &self.matches {
                if sl - el < r.end {
                    if r.start < sl - el {
                        matches.push(Range::new(r.start, sl));
                    } else {
                        matches.push(Range::new(sl - el, sl));
                    }
                    break;
                } else {
                    matches.push(*r);
                }
            }
            (self.s, &matches)
        } else {
            (self.s, &self.matches)
        };

        let mut spans = Vec::new();
        let mut start = 0;
        for range in matches {
            if start < range.start {
                let span = Span::styled(&s[start..range.start], self.not_matched_style);
                spans.push(span);
            }
            let span = Span::styled(&s[range.start..range.end], self.matched_style);
            spans.push(span);
            start = range.end;
        }
        if !&s[start..].is_empty() {
            let span = Span::styled(&s[start..], self.not_matched_style);
            spans.push(span);
        }
        spans
    }
}

fn to_ranges(mut indices: Vec<usize>) -> Vec<Range> {
    if indices.is_empty() {
        return Vec::new();
    }

    indices.sort_unstable();
    indices.dedup();

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
        let not_matched_style = Style::default();
        let matched_style = Style::default().fg(Color::Red);
        let actual = highlight_matched_text(s)
            .matched_indices(vec![2, 3, 4, 7, 9, 10]) // "cde", "h", "jk"
            .into_spans();
        let expected = vec![
            Span::styled("ab", not_matched_style),
            Span::styled("cde", matched_style),
            Span::styled("fg", not_matched_style),
            Span::styled("h", matched_style),
            Span::styled("i", not_matched_style),
            Span::styled("jk", matched_style),
            Span::styled("lmn", not_matched_style),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_matched_text_matched_range() {
        let s = "abcdef";
        let not_matched_style = Style::default();
        let matched_style = Style::default().fg(Color::Red);
        let actual = highlight_matched_text(s)
            .matched_range(2, 4) // "cd"
            .into_spans();
        let expected = vec![
            Span::styled("ab", not_matched_style),
            Span::styled("cd", matched_style),
            Span::styled("ef", not_matched_style),
        ];
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
        let actual = highlight_matched_text(s)
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
        let actual = highlight_matched_text(s)
            .matched_indices(vec![3, 4, 5]) // "def"
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
        let actual = highlight_matched_text(s)
            .matched_indices(vec![3, 4, 5, 6]) // "def."
            .ellipsis("...")
            .into_spans();
        let expected = vec![
            Span::styled("abc", not_matched_style),
            Span::styled("def...", matched_style),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_matched_text_ellipsis_3() {
        let s = "abcdef...";
        let not_matched_style = Style::default();
        let matched_style = Style::default().fg(Color::Red);
        let actual = highlight_matched_text(s)
            .matched_indices(vec![0, 1, 7, 10, 11]) // "ab", ".", "??"
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
        let actual = highlight_matched_text(s)
            .matched_indices(vec![3, 4, 5, 9, 10, 11]) // "def", "???"
            .ellipsis("...")
            .into_spans();
        let expected = vec![
            Span::styled("abc", not_matched_style),
            Span::styled("def", matched_style),
            Span::styled("...", matched_style),
        ];
        assert_eq!(actual, expected);
    }
}
