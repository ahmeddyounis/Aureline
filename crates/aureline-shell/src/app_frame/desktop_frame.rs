use crate::layout::split_tree::{PaneId, SplitAxis, SplitTree};
use crate::layout::zone_registry::{
    Rect, ShellZoneId, ZoneDefaults, ZoneRegistry, ZoneRegistryInput, ZoneRegistryLayout,
};

/// Stable editor-tab identifier scoped to a single [`DesktopFrame`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EditorTabId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveFallbackMode {
    FullChrome,
    CompactShell,
    SplitShell,
    NarrowWidthSheet,
    VeryNarrowCompare,
}

impl ResponsiveFallbackMode {
    pub const fn name(self) -> &'static str {
        match self {
            Self::FullChrome => "full_chrome",
            Self::CompactShell => "compact_shell",
            Self::SplitShell => "split_shell",
            Self::NarrowWidthSheet => "narrow_width_sheet",
            Self::VeryNarrowCompare => "very_narrow_compare",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitViolation {
    pub requested_group_count: usize,
    pub attempted_per_group_width: u32,
    pub main_workspace_minimum_width: u32,
    pub main_workspace_available_width: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewEditorGroupOutcome {
    Created { new_group: PaneId },
    WouldViolateMinimum(SplitViolation),
}

#[derive(Debug, Clone)]
pub struct EditorGroupLayout {
    pub group_id: PaneId,
    pub rect: Rect,
    pub tab_count: u32,
    pub active_tab: Option<EditorTabId>,
    pub tabbed_compare_active: bool,
}

#[derive(Debug, Clone)]
struct EditorGroupState {
    group_id: PaneId,
    tabs: Vec<EditorTabId>,
    active_tab: Option<EditorTabId>,
    tabbed_compare_active: bool,
}

/// A live desktop frame: zone registry + placeholder occupant slots + focus.
#[derive(Debug, Clone)]
pub struct DesktopFrame {
    registry: ZoneRegistry,
    layout: ZoneRegistryLayout,
    focused_zone: ShellZoneId,
    editor_splits: SplitTree,
    focused_editor_group: PaneId,
    editor_groups: Vec<EditorGroupState>,
    next_tab_id: u64,
    last_compare_fallback_violation: Option<SplitViolation>,
}

impl DesktopFrame {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let registry = ZoneRegistry::new(ZoneDefaults::standard());
        let editor_splits = SplitTree::single();
        let focused_editor_group = editor_splits.root_leaf();
        let main_min = registry.defaults().main_workspace_min_width;
        let required_main_min = main_min.saturating_mul(editor_splits.leaf_count() as u32);
        let layout = registry.layout(ZoneRegistryInput {
            window_width,
            window_height,
            split_heavy: editor_splits.leaf_count() >= 2,
            main_workspace_min_width_override: Some(required_main_min),
        });
        Self {
            registry,
            layout,
            focused_zone: ShellZoneId::MainWorkspace,
            editor_splits,
            focused_editor_group,
            editor_groups: vec![EditorGroupState {
                group_id: focused_editor_group,
                tabs: Vec::new(),
                active_tab: None,
                tabbed_compare_active: false,
            }],
            next_tab_id: 1,
            last_compare_fallback_violation: None,
        }
    }

    pub fn relayout(&mut self, window_width: u32, window_height: u32) {
        let main_min = self.registry.defaults().main_workspace_min_width;
        let required_main_min = main_min.saturating_mul(self.editor_splits.leaf_count() as u32);
        self.layout = self.registry.layout(ZoneRegistryInput {
            window_width,
            window_height,
            split_heavy: self.editor_splits.leaf_count() >= 2,
            main_workspace_min_width_override: Some(required_main_min),
        });
        if self.layout.zone(self.focused_zone).is_none() {
            self.focused_zone = ShellZoneId::MainWorkspace;
        }
        self.ensure_focused_editor_group_is_visible();
    }

    pub const fn focused_zone(&self) -> ShellZoneId {
        self.focused_zone
    }

    pub fn focus_zone(&mut self, zone: ShellZoneId) {
        if self.layout.zone(zone).is_some() {
            self.focused_zone = zone;
        } else {
            self.focused_zone = ShellZoneId::MainWorkspace;
        }
    }

    pub const fn focused_editor_group(&self) -> PaneId {
        self.focused_editor_group
    }

    pub fn editor_group_layouts(&self) -> Vec<EditorGroupLayout> {
        let groups_in_order = self.editor_splits.leaf_ids_in_order();
        let visible = self.visible_editor_groups(&groups_in_order);
        if visible.is_empty() {
            return Vec::new();
        }

        let rects = if visible.len() == groups_in_order.len() {
            self.editor_splits
                .layout_with_min_width(
                    self.layout.main_workspace,
                    self.registry.defaults().main_workspace_min_width,
                )
                .unwrap_or_else(|_| equal_vertical_slices(self.layout.main_workspace, &visible))
        } else {
            equal_vertical_slices(self.layout.main_workspace, &visible)
        };

        rects
            .into_iter()
            .filter_map(|(group_id, rect)| {
                let state = self.editor_groups.iter().find(|g| g.group_id == group_id)?;
                Some(EditorGroupLayout {
                    group_id,
                    rect,
                    tab_count: state.tabs.len() as u32,
                    active_tab: state.active_tab,
                    tabbed_compare_active: state.tabbed_compare_active,
                })
            })
            .collect()
    }

    pub fn responsive_fallback_modes(&self) -> Vec<ResponsiveFallbackMode> {
        let mut modes = Vec::new();
        match self.layout.adaptive_class {
            crate::layout::zone_registry::AdaptiveClass::CompactDesktop => {
                modes.push(ResponsiveFallbackMode::CompactShell);
                modes.push(ResponsiveFallbackMode::NarrowWidthSheet);
            }
            crate::layout::zone_registry::AdaptiveClass::StandardDesktop => {
                modes.push(ResponsiveFallbackMode::FullChrome);
            }
            crate::layout::zone_registry::AdaptiveClass::ExpandedDesktop => {
                modes.push(ResponsiveFallbackMode::FullChrome);
            }
        }

        if self.editor_splits.leaf_count() >= 2 {
            modes.push(ResponsiveFallbackMode::SplitShell);
        }
        if self.last_compare_fallback_violation.is_some() {
            modes.push(ResponsiveFallbackMode::VeryNarrowCompare);
        }
        modes
    }

    pub fn open_placeholder_tab(&mut self) {
        let _ = self.open_tab();
    }

    /// Opens a new tab in the focused editor group and returns its stable id.
    pub fn open_tab(&mut self) -> Option<EditorTabId> {
        self.open_tab_in_group(self.focused_editor_group)
    }

    /// Opens a new tab in `group` and returns its stable id.
    pub fn open_tab_in_group(&mut self, group: PaneId) -> Option<EditorTabId> {
        let state = self
            .editor_groups
            .iter_mut()
            .find(|g| g.group_id == group)?;
        let id = EditorTabId(self.next_tab_id);
        self.next_tab_id = self.next_tab_id.saturating_add(1);
        state.tabs.push(id);
        state.active_tab = Some(id);
        Some(id)
    }

    /// Returns the ordered tab ids for `group`.
    pub fn tab_ids(&self, group: PaneId) -> Vec<EditorTabId> {
        self.editor_groups
            .iter()
            .find(|g| g.group_id == group)
            .map(|g| g.tabs.clone())
            .unwrap_or_default()
    }

    /// Returns the active tab id for `group`, if any.
    pub fn active_tab_id(&self, group: PaneId) -> Option<EditorTabId> {
        self.editor_groups
            .iter()
            .find(|g| g.group_id == group)
            .and_then(|g| g.active_tab)
    }

    /// Sets the active tab for `group`. Returns `true` when the tab belonged to the group.
    pub fn set_active_tab(&mut self, group: PaneId, tab: EditorTabId) -> bool {
        let Some(state) = self.editor_groups.iter_mut().find(|g| g.group_id == group) else {
            return false;
        };
        if !state.tabs.iter().any(|id| *id == tab) {
            return false;
        }
        state.active_tab = Some(tab);
        true
    }

    /// Closes the active tab for `group` and returns the closed tab id.
    pub fn close_active_tab(&mut self, group: PaneId) -> Option<EditorTabId> {
        let state = self
            .editor_groups
            .iter_mut()
            .find(|g| g.group_id == group)?;
        let active = state.active_tab?;
        let idx = state.tabs.iter().position(|id| *id == active)?;
        state.tabs.remove(idx);
        state.active_tab = if state.tabs.is_empty() {
            None
        } else if idx == 0 {
            state.tabs.first().copied()
        } else {
            state.tabs.get(idx.saturating_sub(1)).copied()
        };
        Some(active)
    }

    pub fn focus_next_editor_group(&mut self) {
        if self.focused_zone != ShellZoneId::MainWorkspace {
            self.focused_zone = ShellZoneId::MainWorkspace;
        }
        let groups_in_order = self.editor_splits.leaf_ids_in_order();
        let visible = self.visible_editor_groups(&groups_in_order);
        if visible.is_empty() {
            return;
        }
        let current = visible
            .iter()
            .position(|id| *id == self.focused_editor_group);
        let next = match current {
            Some(i) => visible[(i + 1) % visible.len()],
            None => visible[0],
        };
        self.focused_editor_group = next;
    }

    pub fn focus_editor_group(&mut self, group: PaneId) {
        if !self.editor_splits.contains_leaf(group) {
            return;
        }
        self.focused_zone = ShellZoneId::MainWorkspace;
        self.focused_editor_group = group;
        self.ensure_focused_editor_group_is_visible();
    }

    pub fn request_split_focused_editor_group(&mut self) -> NewEditorGroupOutcome {
        self.last_compare_fallback_violation = None;
        let window_width = self.layout.window.width;
        let window_height = self.layout.window.height;

        let current_count = self.editor_splits.leaf_count();
        let requested = current_count.saturating_add(1);
        let per_group_min = self.registry.defaults().main_workspace_min_width;
        let required_min = per_group_min.saturating_mul(requested as u32);

        let prospective = self.registry.layout(ZoneRegistryInput {
            window_width,
            window_height,
            split_heavy: requested >= 2,
            main_workspace_min_width_override: Some(required_min),
        });

        let available = prospective.main_workspace.width;
        let attempted_per_group = if requested == 0 {
            0
        } else {
            available / requested as u32
        };

        if attempted_per_group < per_group_min {
            let violation = SplitViolation {
                requested_group_count: requested,
                attempted_per_group_width: attempted_per_group,
                main_workspace_minimum_width: per_group_min,
                main_workspace_available_width: available,
            };
            self.last_compare_fallback_violation = Some(violation);
            return NewEditorGroupOutcome::WouldViolateMinimum(violation);
        }

        let Some(new_group) = self
            .editor_splits
            .split_leaf(self.focused_editor_group, SplitAxis::Vertical)
        else {
            let violation = SplitViolation {
                requested_group_count: requested,
                attempted_per_group_width: attempted_per_group,
                main_workspace_minimum_width: per_group_min,
                main_workspace_available_width: available,
            };
            self.last_compare_fallback_violation = Some(violation);
            return NewEditorGroupOutcome::WouldViolateMinimum(violation);
        };

        self.editor_groups.push(EditorGroupState {
            group_id: new_group,
            tabs: Vec::new(),
            active_tab: None,
            tabbed_compare_active: false,
        });
        self.focused_zone = ShellZoneId::MainWorkspace;
        self.focused_editor_group = new_group;
        self.layout = prospective;
        self.ensure_focused_editor_group_is_visible();
        NewEditorGroupOutcome::Created { new_group }
    }

    pub fn close_focused_editor_group(&mut self) -> bool {
        if self.editor_splits.leaf_count() <= 1 {
            return false;
        }
        let to_remove = self.focused_editor_group;
        if !self.editor_splits.remove_leaf(to_remove) {
            return false;
        }
        self.editor_groups.retain(|g| g.group_id != to_remove);
        let groups_in_order = self.editor_splits.leaf_ids_in_order();
        self.focused_editor_group = groups_in_order.first().copied().unwrap_or(to_remove);

        let window_width = self.layout.window.width;
        let window_height = self.layout.window.height;
        self.relayout(window_width, window_height);
        true
    }

    pub fn engage_tabbed_compare_fallback(&mut self) {
        if let Some(group) = self
            .editor_groups
            .iter_mut()
            .find(|g| g.group_id == self.focused_editor_group)
        {
            group.tabbed_compare_active = true;
        }
    }

    pub fn clear_compare_fallback_marker(&mut self) {
        self.last_compare_fallback_violation = None;
    }

    pub fn focus_next(&mut self) {
        let mut zones: Vec<ShellZoneId> = Vec::new();
        for zone in ShellZoneId::ALL {
            let zone = *zone;
            if zone == ShellZoneId::TransientOverlay {
                continue;
            }
            if self.layout.zone(zone).is_some() {
                zones.push(zone);
            }
        }
        if zones.is_empty() {
            return;
        }
        let current = zones.iter().position(|z| *z == self.focused_zone);
        let next = match current {
            Some(i) => zones[(i + 1) % zones.len()],
            None => zones[0],
        };
        self.focused_zone = next;
    }

    pub fn layout(&self) -> &ZoneRegistryLayout {
        &self.layout
    }

    /// Placeholder slot ids per zone. These are stable attachment points for
    /// later shell surfaces.
    pub fn slot_ids_for_zone(&self, zone: ShellZoneId) -> &'static [&'static str] {
        match zone {
            ShellZoneId::TitleContextBar => &["slot.title_context_bar.identity"],
            ShellZoneId::ActivityRail => &["slot.activity_rail.primary_routes"],
            ShellZoneId::LeftSidebar => &["slot.sidebar.section_surface"],
            ShellZoneId::MainWorkspace => &[
                "slot.main_workspace.working_set",
                "slot.main_workspace.review_surface",
            ],
            ShellZoneId::RightInspector => &["slot.right_inspector.contextual_detail"],
            ShellZoneId::BottomPanel => &["slot.bottom_panel.tool_panels"],
            ShellZoneId::StatusBar => &[
                "status.slot.recovery.primary",
                "status.slot.context.workspace",
                "status.slot.context.execution",
                "status.slot.work.summary",
                "status.slot.metadata.file",
                "status.slot.extension.scoped",
            ],
            ShellZoneId::TransientOverlay => &[
                "slot.overlay.command_palette",
                "slot.overlay.dialog_or_sheet",
            ],
        }
    }

    pub fn slot_rects_within_zone(
        &self,
        zone: ShellZoneId,
        zone_rect: Rect,
        zone_inset_px: u32,
    ) -> Vec<(&'static str, Rect)> {
        let slots = self.slot_ids_for_zone(zone);
        if slots.is_empty() || zone_rect.is_empty() {
            return Vec::new();
        }

        let padding = zone_inset_px;
        let inner = Rect::new(
            zone_rect.x.saturating_add(padding),
            zone_rect.y.saturating_add(padding),
            zone_rect.width.saturating_sub(padding * 2),
            zone_rect.height.saturating_sub(padding * 2),
        );
        if inner.is_empty() {
            return Vec::new();
        }

        let n = slots.len() as u32;
        if zone == ShellZoneId::StatusBar {
            let col_w = (inner.width / n).max(1);
            return slots
                .iter()
                .enumerate()
                .map(|(i, id)| {
                    let x = inner.x.saturating_add(col_w.saturating_mul(i as u32));
                    let width = if i + 1 == slots.len() {
                        inner.right().saturating_sub(x)
                    } else {
                        col_w
                    };
                    (*id, Rect::new(x, inner.y, width, inner.height))
                })
                .collect();
        }

        let row_h = (inner.height / n).max(1);
        slots
            .iter()
            .enumerate()
            .map(|(i, id)| {
                let y = inner.y.saturating_add(row_h.saturating_mul(i as u32));
                let height = if i + 1 == slots.len() {
                    inner.bottom().saturating_sub(y)
                } else {
                    row_h
                };
                (*id, Rect::new(inner.x, y, inner.width, height))
            })
            .collect()
    }

    fn visible_editor_groups(&self, groups_in_order: &[PaneId]) -> Vec<PaneId> {
        let per_group_min = self.registry.defaults().main_workspace_min_width;
        if per_group_min == 0 {
            return groups_in_order.to_vec();
        }
        let max_visible = (self.layout.main_workspace.width / per_group_min).max(1) as usize;
        groups_in_order.iter().copied().take(max_visible).collect()
    }

    fn ensure_focused_editor_group_is_visible(&mut self) {
        let groups_in_order = self.editor_splits.leaf_ids_in_order();
        let visible = self.visible_editor_groups(&groups_in_order);
        if visible.is_empty() {
            return;
        }
        if visible.iter().any(|id| *id == self.focused_editor_group) {
            return;
        }
        self.focused_editor_group = visible[visible.len().saturating_sub(1)];
    }
}

fn equal_vertical_slices(container: Rect, group_ids: &[PaneId]) -> Vec<(PaneId, Rect)> {
    if group_ids.is_empty() || container.is_empty() {
        return Vec::new();
    }
    let n = group_ids.len() as u32;
    let base_w = (container.width / n).max(1);
    let mut out = Vec::with_capacity(group_ids.len());
    let mut x = container.x;
    for (idx, group_id) in group_ids.iter().copied().enumerate() {
        let width = if idx + 1 == group_ids.len() {
            container.right().saturating_sub(x)
        } else {
            base_w
        };
        out.push((group_id, Rect::new(x, container.y, width, container.height)));
        x = x.saturating_add(width);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_slot_ids_match_fixture_contract() {
        let frame = DesktopFrame::new(1920, 1080);

        assert_eq!(
            frame.slot_ids_for_zone(ShellZoneId::TitleContextBar),
            &["slot.title_context_bar.identity"]
        );
        assert_eq!(
            frame.slot_ids_for_zone(ShellZoneId::ActivityRail),
            &["slot.activity_rail.primary_routes"]
        );
        assert_eq!(
            frame.slot_ids_for_zone(ShellZoneId::LeftSidebar),
            &["slot.sidebar.section_surface"]
        );
        assert_eq!(
            frame.slot_ids_for_zone(ShellZoneId::MainWorkspace),
            &[
                "slot.main_workspace.working_set",
                "slot.main_workspace.review_surface",
            ]
        );
        assert_eq!(
            frame.slot_ids_for_zone(ShellZoneId::RightInspector),
            &["slot.right_inspector.contextual_detail"]
        );
        assert_eq!(
            frame.slot_ids_for_zone(ShellZoneId::BottomPanel),
            &["slot.bottom_panel.tool_panels"]
        );
        assert_eq!(
            frame.slot_ids_for_zone(ShellZoneId::StatusBar),
            &[
                "status.slot.recovery.primary",
                "status.slot.context.workspace",
                "status.slot.context.execution",
                "status.slot.work.summary",
                "status.slot.metadata.file",
                "status.slot.extension.scoped"
            ]
        );
    }

    #[test]
    fn status_bar_slots_are_horizontal_slices() {
        let frame = DesktopFrame::new(600, 240);
        let rects =
            frame.slot_rects_within_zone(ShellZoneId::StatusBar, Rect::new(10, 200, 480, 24), 0);

        assert_eq!(rects.len(), 6);
        assert!(rects.windows(2).all(|pair| pair[0].1.y == pair[1].1.y));
        assert!(rects.windows(2).all(|pair| pair[0].1.x < pair[1].1.x));
        assert!(rects.iter().all(|(_, rect)| rect.height == 24));
        assert_eq!(rects.first().map(|(_, rect)| rect.x), Some(10));
        assert_eq!(rects.last().map(|(_, rect)| rect.right()), Some(490));
    }
}
