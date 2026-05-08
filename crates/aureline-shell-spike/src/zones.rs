//! Composited shell frame — zone layout.
//!
//! The spike paints four placeholder zones into one window. Each zone has
//! a stable id, a fixed stacking order, and a single invalidation entry
//! point. The zone set is intentionally small; the editor, panels, and
//! command palette are out of scope for this spike.

use core::fmt;

/// A rectangle in logical (pre-scale) device coordinates. Origin is the
/// top-left of the window's client area; y grows downward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn area(self) -> u32 {
        self.width.saturating_mul(self.height)
    }

    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// The zones composited into the shell frame. Ordering is significant:
/// later variants paint on top of earlier variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZoneId {
    TitleBar,
    Sidebar,
    EditorViewport,
    StatusBar,
}

impl ZoneId {
    pub const fn name(self) -> &'static str {
        match self {
            Self::TitleBar => "title_bar",
            Self::Sidebar => "sidebar",
            Self::EditorViewport => "editor_viewport",
            Self::StatusBar => "status_bar",
        }
    }

    /// The stacking order for the zone. Higher paints on top.
    pub const fn z_order(self) -> u8 {
        match self {
            Self::EditorViewport => 0,
            Self::Sidebar => 1,
            Self::StatusBar => 2,
            Self::TitleBar => 3,
        }
    }

    pub const ALL: &'static [ZoneId] = &[
        Self::TitleBar,
        Self::Sidebar,
        Self::EditorViewport,
        Self::StatusBar,
    ];
}

impl fmt::Display for ZoneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The shell frame: one window, four zones, one damage entry point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellFrame {
    pub window: Rect,
    pub title_bar: Rect,
    pub sidebar: Rect,
    pub editor_viewport: Rect,
    pub status_bar: Rect,
}

impl ShellFrame {
    /// Default shell frame layout used by the fixture scene. Numbers are
    /// deterministic so the emitted trace is byte-stable across runs.
    pub const FIXTURE_WINDOW_WIDTH: u32 = 1280;
    pub const FIXTURE_WINDOW_HEIGHT: u32 = 800;

    pub const TITLE_BAR_HEIGHT: u32 = 28;
    pub const STATUS_BAR_HEIGHT: u32 = 22;
    pub const SIDEBAR_WIDTH: u32 = 240;

    /// Lay the frame out for a window of the given size.
    pub const fn lay_out(window_width: u32, window_height: u32) -> Self {
        let window = Rect::new(0, 0, window_width, window_height);
        let title_bar = Rect::new(0, 0, window_width, Self::TITLE_BAR_HEIGHT);
        let status_y = window_height.saturating_sub(Self::STATUS_BAR_HEIGHT);
        let status_bar = Rect::new(0, status_y, window_width, Self::STATUS_BAR_HEIGHT);
        let body_y = Self::TITLE_BAR_HEIGHT;
        let body_height = status_y.saturating_sub(body_y);
        let sidebar = Rect::new(0, body_y, Self::SIDEBAR_WIDTH, body_height);
        let editor_viewport = Rect::new(
            Self::SIDEBAR_WIDTH,
            body_y,
            window_width.saturating_sub(Self::SIDEBAR_WIDTH),
            body_height,
        );
        Self {
            window,
            title_bar,
            sidebar,
            editor_viewport,
            status_bar,
        }
    }

    pub const fn fixture() -> Self {
        Self::lay_out(Self::FIXTURE_WINDOW_WIDTH, Self::FIXTURE_WINDOW_HEIGHT)
    }

    pub fn zone(&self, id: ZoneId) -> Rect {
        match id {
            ZoneId::TitleBar => self.title_bar,
            ZoneId::Sidebar => self.sidebar,
            ZoneId::EditorViewport => self.editor_viewport,
            ZoneId::StatusBar => self.status_bar,
        }
    }

    /// Zones painted in their canonical (ADR-0002 two-layer) order.
    pub fn zones_in_paint_order(&self) -> [(ZoneId, Rect); 4] {
        let mut zones = [
            (ZoneId::TitleBar, self.title_bar),
            (ZoneId::Sidebar, self.sidebar),
            (ZoneId::EditorViewport, self.editor_viewport),
            (ZoneId::StatusBar, self.status_bar),
        ];
        zones.sort_by_key(|(id, _)| id.z_order());
        zones
    }
}

#[cfg(test)]
mod tests {
    use super::{Rect, ShellFrame, ZoneId};

    #[test]
    fn fixture_layout_covers_the_window_without_gaps() {
        let frame = ShellFrame::fixture();
        // Title bar pins to the top.
        assert_eq!(frame.title_bar.y, 0);
        // Status bar pins to the bottom.
        assert_eq!(
            frame.status_bar.y + frame.status_bar.height,
            ShellFrame::FIXTURE_WINDOW_HEIGHT
        );
        // Sidebar and editor viewport meet at the sidebar's right edge.
        assert_eq!(
            frame.sidebar.x + frame.sidebar.width,
            frame.editor_viewport.x
        );
        // Body region exactly spans between the title and status bars.
        assert_eq!(frame.editor_viewport.y, ShellFrame::TITLE_BAR_HEIGHT);
        assert_eq!(
            frame.editor_viewport.y + frame.editor_viewport.height,
            ShellFrame::FIXTURE_WINDOW_HEIGHT - ShellFrame::STATUS_BAR_HEIGHT
        );
    }

    #[test]
    fn zones_paint_in_adr_order() {
        let frame = ShellFrame::fixture();
        let order: Vec<_> = frame
            .zones_in_paint_order()
            .iter()
            .map(|(id, _)| *id)
            .collect();
        assert_eq!(
            order,
            vec![
                ZoneId::EditorViewport,
                ZoneId::Sidebar,
                ZoneId::StatusBar,
                ZoneId::TitleBar,
            ]
        );
    }

    #[test]
    fn rect_area_saturates_on_overflow() {
        let big = Rect::new(0, 0, u32::MAX, u32::MAX);
        assert_eq!(big.area(), u32::MAX);
    }
}
