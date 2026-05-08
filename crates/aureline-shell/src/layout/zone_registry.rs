use core::fmt;

/// A rectangle in logical pixels. Origin is the top-left of the window's
/// client area; y grows downward.
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

    pub const fn right(self) -> u32 {
        self.x.saturating_add(self.width)
    }

    pub const fn bottom(self) -> u32 {
        self.y.saturating_add(self.height)
    }

    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// Canonical shell-zone ids.
///
/// These names are the stable surface identifiers used by shell layout,
/// restore, and UX fixtures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellZoneId {
    TitleContextBar,
    ActivityRail,
    LeftSidebar,
    MainWorkspace,
    RightInspector,
    BottomPanel,
    StatusBar,
    TransientOverlay,
}

impl ShellZoneId {
    pub const fn name(self) -> &'static str {
        match self {
            Self::TitleContextBar => "title_context_bar",
            Self::ActivityRail => "activity_rail",
            Self::LeftSidebar => "left_sidebar",
            Self::MainWorkspace => "main_workspace",
            Self::RightInspector => "right_inspector",
            Self::BottomPanel => "bottom_panel",
            Self::StatusBar => "status_bar",
            Self::TransientOverlay => "transient_overlay",
        }
    }

    pub const ALL: &'static [ShellZoneId] = &[
        Self::TitleContextBar,
        Self::ActivityRail,
        Self::LeftSidebar,
        Self::MainWorkspace,
        Self::RightInspector,
        Self::BottomPanel,
        Self::StatusBar,
        Self::TransientOverlay,
    ];
}

impl fmt::Display for ShellZoneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptiveClass {
    CompactDesktop,
    StandardDesktop,
    ExpandedDesktop,
}

impl AdaptiveClass {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CompactDesktop => "compact_desktop",
            Self::StandardDesktop => "standard_desktop",
            Self::ExpandedDesktop => "expanded_desktop",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneDefaults {
    pub title_context_bar_height: u32,
    pub status_bar_height: u32,
    pub activity_rail_width: u32,
    pub activity_rail_min_width: u32,
    pub left_sidebar_width: u32,
    pub right_inspector_width: u32,
    pub bottom_panel_height: u32,
    pub main_workspace_min_width: u32,
}

impl ZoneDefaults {
    pub const fn standard() -> Self {
        Self {
            title_context_bar_height: 32,
            status_bar_height: 24,
            activity_rail_width: 48,
            activity_rail_min_width: 44,
            left_sidebar_width: 280,
            right_inspector_width: 360,
            bottom_panel_height: 280,
            main_workspace_min_width: 420,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneRegistryInput {
    pub window_width: u32,
    pub window_height: u32,
    /// When true, the shell is in a split-heavy posture (≥ 2 editor groups).
    /// This allows the zone registry to collapse optional chrome earlier,
    /// preserving minimum useful widths for editor groups.
    pub split_heavy: bool,
    /// Optional override for the minimum useful width of the main workspace.
    /// Used by split-heavy editor-group layouts to ensure the zone registry
    /// collapses optional chrome before rendering unusable narrow panes.
    pub main_workspace_min_width_override: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneRegistryLayout {
    pub adaptive_class: AdaptiveClass,
    pub window: Rect,
    pub title_context_bar: Rect,
    pub activity_rail: Rect,
    pub left_sidebar: Option<Rect>,
    pub main_workspace: Rect,
    pub right_inspector: Option<Rect>,
    pub bottom_panel: Option<Rect>,
    pub status_bar: Rect,
    pub transient_overlay: Rect,
}

impl ZoneRegistryLayout {
    pub fn zone(&self, id: ShellZoneId) -> Option<Rect> {
        match id {
            ShellZoneId::TitleContextBar => Some(self.title_context_bar),
            ShellZoneId::ActivityRail => Some(self.activity_rail),
            ShellZoneId::LeftSidebar => self.left_sidebar,
            ShellZoneId::MainWorkspace => Some(self.main_workspace),
            ShellZoneId::RightInspector => self.right_inspector,
            ShellZoneId::BottomPanel => self.bottom_panel,
            ShellZoneId::StatusBar => Some(self.status_bar),
            ShellZoneId::TransientOverlay => Some(self.transient_overlay),
        }
    }
}

/// Canonical shell-zone registry.
///
/// This is the single owner of the top-level desktop shell zone layout. Higher
/// level surfaces attach occupants through these zones (and their slot
/// families) instead of hard-coding new top-level chrome.
#[derive(Debug, Clone, Copy)]
pub struct ZoneRegistry {
    defaults: ZoneDefaults,
}

impl ZoneRegistry {
    pub const fn new(defaults: ZoneDefaults) -> Self {
        Self { defaults }
    }

    pub const fn defaults(&self) -> ZoneDefaults {
        self.defaults
    }

    pub fn layout(&self, input: ZoneRegistryInput) -> ZoneRegistryLayout {
        let adaptive_class = classify_adaptive_class(input.window_width);
        let window = Rect::new(0, 0, input.window_width, input.window_height);
        let main_workspace_min_width = input
            .main_workspace_min_width_override
            .unwrap_or(self.defaults.main_workspace_min_width);

        let title_h = self
            .defaults
            .title_context_bar_height
            .min(input.window_height);
        let status_h = self
            .defaults
            .status_bar_height
            .min(input.window_height.saturating_sub(title_h));

        let title_context_bar = Rect::new(0, 0, input.window_width, title_h);
        let status_y = input.window_height.saturating_sub(status_h);
        let status_bar = Rect::new(0, status_y, input.window_width, status_h);

        let content_y = title_h;
        let content_height = status_y.saturating_sub(content_y);

        let mut activity_rail_width = self.defaults.activity_rail_width.min(input.window_width);
        if input.window_width < activity_rail_width {
            activity_rail_width = input.window_width;
        }

        let mut left_sidebar_visible = matches!(
            adaptive_class,
            AdaptiveClass::StandardDesktop | AdaptiveClass::ExpandedDesktop
        );
        let mut right_inspector_visible = matches!(adaptive_class, AdaptiveClass::ExpandedDesktop);
        let mut bottom_panel_visible = matches!(
            adaptive_class,
            AdaptiveClass::StandardDesktop | AdaptiveClass::ExpandedDesktop
        );

        // In split-heavy posture, collapse optional side chrome earlier on
        // constrained widths so editor groups retain minimum useful width.
        if input.split_heavy
            && matches!(
                adaptive_class,
                AdaptiveClass::StandardDesktop | AdaptiveClass::CompactDesktop
            )
        {
            left_sidebar_visible = false;
            right_inspector_visible = false;
        }

        let mut bottom_panel_height = if bottom_panel_visible {
            self.defaults.bottom_panel_height.min(content_height)
        } else {
            0
        };

        let main_row_height = content_height.saturating_sub(bottom_panel_height);

        // Collapse in priority order before violating the main-workspace minimum.
        //
        // Order: inspector -> sidebar -> shrink rail to minimum.
        let workspace_available = |rail_w: u32, sidebar: bool, inspector: bool| -> u32 {
            input
                .window_width
                .saturating_sub(rail_w)
                .saturating_sub(if sidebar {
                    self.defaults.left_sidebar_width
                } else {
                    0
                })
                .saturating_sub(if inspector {
                    self.defaults.right_inspector_width
                } else {
                    0
                })
        };

        if workspace_available(
            activity_rail_width,
            left_sidebar_visible,
            right_inspector_visible,
        ) < main_workspace_min_width
        {
            right_inspector_visible = false;
        }

        if workspace_available(
            activity_rail_width,
            left_sidebar_visible,
            right_inspector_visible,
        ) < main_workspace_min_width
        {
            left_sidebar_visible = false;
        }

        if workspace_available(
            activity_rail_width,
            left_sidebar_visible,
            right_inspector_visible,
        ) < main_workspace_min_width
        {
            activity_rail_width = activity_rail_width.min(self.defaults.activity_rail_min_width);
        }

        // If the window is too short to host a meaningful bottom panel, collapse it.
        if bottom_panel_visible && main_row_height < 120 {
            bottom_panel_visible = false;
            bottom_panel_height = 0;
        }

        let activity_rail = Rect::new(0, content_y, activity_rail_width, content_height);

        let work_x = activity_rail_width;
        let work_width = input.window_width.saturating_sub(work_x);

        let bottom_panel = if bottom_panel_visible && bottom_panel_height > 0 {
            Some(Rect::new(
                work_x,
                content_y.saturating_add(main_row_height),
                work_width,
                bottom_panel_height,
            ))
        } else {
            None
        };

        let mut x = work_x;
        let left_sidebar = if left_sidebar_visible && self.defaults.left_sidebar_width > 0 {
            let width = self.defaults.left_sidebar_width.min(work_width);
            let rect = Rect::new(x, content_y, width, main_row_height);
            x = x.saturating_add(width);
            Some(rect)
        } else {
            None
        };

        let inspector_width = if right_inspector_visible {
            self.defaults.right_inspector_width.min(work_width)
        } else {
            0
        };

        let main_workspace_width = work_width
            .saturating_sub(x.saturating_sub(work_x))
            .saturating_sub(inspector_width);

        let main_workspace = Rect::new(x, content_y, main_workspace_width, main_row_height);
        let right_inspector = if right_inspector_visible && inspector_width > 0 {
            Some(Rect::new(
                x.saturating_add(main_workspace_width),
                content_y,
                inspector_width,
                main_row_height,
            ))
        } else {
            None
        };

        // The overlay zone exists even when nothing is shown; it is the full
        // window bounds for window-local transient surfaces.
        let transient_overlay = window;

        ZoneRegistryLayout {
            adaptive_class,
            window,
            title_context_bar,
            activity_rail,
            left_sidebar,
            main_workspace,
            right_inspector,
            bottom_panel,
            status_bar,
            transient_overlay,
        }
    }
}

pub const fn classify_adaptive_class(window_width: u32) -> AdaptiveClass {
    if window_width < 1280 {
        AdaptiveClass::CompactDesktop
    } else if window_width < 1600 {
        AdaptiveClass::StandardDesktop
    } else {
        AdaptiveClass::ExpandedDesktop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expanded_desktop_default_renders_all_canonical_zones() {
        let registry = ZoneRegistry::new(ZoneDefaults::standard());
        let layout = registry.layout(ZoneRegistryInput {
            window_width: 1920,
            window_height: 1080,
            split_heavy: false,
            main_workspace_min_width_override: None,
        });

        assert_eq!(layout.adaptive_class.name(), "expanded_desktop");
        for zone in [
            ShellZoneId::TitleContextBar,
            ShellZoneId::ActivityRail,
            ShellZoneId::LeftSidebar,
            ShellZoneId::MainWorkspace,
            ShellZoneId::RightInspector,
            ShellZoneId::BottomPanel,
            ShellZoneId::StatusBar,
        ] {
            assert!(layout.zone(zone).is_some(), "missing zone {zone}");
        }
    }

    #[test]
    fn expanded_desktop_default_geometry_matches_fixture_defaults() {
        let registry = ZoneRegistry::new(ZoneDefaults::standard());
        let layout = registry.layout(ZoneRegistryInput {
            window_width: 1920,
            window_height: 1080,
            split_heavy: false,
            main_workspace_min_width_override: None,
        });

        assert_eq!(layout.title_context_bar, Rect::new(0, 0, 1920, 32));
        assert_eq!(layout.activity_rail, Rect::new(0, 32, 48, 1024));
        assert_eq!(layout.left_sidebar, Some(Rect::new(48, 32, 280, 744)));
        assert_eq!(layout.main_workspace, Rect::new(328, 32, 1232, 744));
        assert_eq!(layout.right_inspector, Some(Rect::new(1560, 32, 360, 744)));
        assert_eq!(layout.bottom_panel, Some(Rect::new(48, 776, 1872, 280)));
        assert_eq!(layout.status_bar, Rect::new(0, 1056, 1920, 24));
        assert_eq!(layout.transient_overlay, Rect::new(0, 0, 1920, 1080));
    }

    #[test]
    fn layout_never_overlaps_title_or_status_bars() {
        let registry = ZoneRegistry::new(ZoneDefaults::standard());
        let layout = registry.layout(ZoneRegistryInput {
            window_width: 1920,
            window_height: 1080,
            split_heavy: false,
            main_workspace_min_width_override: None,
        });

        let title_bottom = layout.title_context_bar.bottom();
        let status_top = layout.status_bar.y;
        for zone in [
            ShellZoneId::ActivityRail,
            ShellZoneId::LeftSidebar,
            ShellZoneId::MainWorkspace,
            ShellZoneId::RightInspector,
            ShellZoneId::BottomPanel,
        ] {
            if let Some(rect) = layout.zone(zone) {
                assert!(rect.y >= title_bottom, "zone {zone} overlaps title");
                assert!(rect.bottom() <= status_top, "zone {zone} overlaps status");
            }
        }
    }

    #[test]
    fn standard_desktop_hides_inspector_by_default() {
        let registry = ZoneRegistry::new(ZoneDefaults::standard());
        let layout = registry.layout(ZoneRegistryInput {
            window_width: 1440,
            window_height: 900,
            split_heavy: false,
            main_workspace_min_width_override: None,
        });
        assert_eq!(layout.adaptive_class.name(), "standard_desktop");
        assert!(layout.right_inspector.is_none());
        assert!(layout.left_sidebar.is_some());
        assert!(layout.bottom_panel.is_some());
    }
}
