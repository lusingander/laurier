use ratatui_core::layout::{Constraint, Layout, Rect};

pub fn calc_centered_area(base_area: Rect, area_width: u16, area_height: u16) -> Rect {
    let vertical_pad = base_area.height.saturating_sub(area_height) / 2;
    let vertical_layout = Layout::vertical(Constraint::from_lengths([
        vertical_pad,
        area_height,
        vertical_pad,
    ]))
    .split(base_area);

    let horizontal_pad = base_area.width.saturating_sub(area_width) / 2;
    Layout::horizontal(Constraint::from_lengths([
        horizontal_pad,
        area_width,
        horizontal_pad,
    ]))
    .split(vertical_layout[1])[1]
}
