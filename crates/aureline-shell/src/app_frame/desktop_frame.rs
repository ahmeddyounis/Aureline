use crate::layout::zone_registry::{Rect, ShellZoneId, ZoneDefaults, ZoneRegistry, ZoneRegistryInput, ZoneRegistryLayout};

/// A live desktop frame: zone registry + placeholder occupant slots + focus.
#[derive(Debug, Clone)]
pub struct DesktopFrame {
    registry: ZoneRegistry,
    layout: ZoneRegistryLayout,
    focused_zone: ShellZoneId,
}

impl DesktopFrame {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let registry = ZoneRegistry::new(ZoneDefaults::standard());
        let layout = registry.layout(ZoneRegistryInput {
            window_width,
            window_height,
        });
        Self {
            registry,
            layout,
            focused_zone: ShellZoneId::MainWorkspace,
        }
    }

    pub fn relayout(&mut self, window_width: u32, window_height: u32) {
        self.layout = self.registry.layout(ZoneRegistryInput {
            window_width,
            window_height,
        });
        if self.layout.zone(self.focused_zone).is_none() {
            self.focused_zone = ShellZoneId::MainWorkspace;
        }
    }

    pub const fn focused_zone(&self) -> ShellZoneId {
        self.focused_zone
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
            ShellZoneId::StatusBar => &["status.slot.recovery.primary", "status.slot.extension.scoped"],
            ShellZoneId::TransientOverlay => &["slot.overlay.command_palette", "slot.overlay.dialog_or_sheet"],
        }
    }

    pub fn slot_rects_within_zone(&self, zone: ShellZoneId, zone_rect: Rect) -> Vec<(&'static str, Rect)> {
        let slots = self.slot_ids_for_zone(zone);
        if slots.is_empty() || zone_rect.is_empty() {
            return Vec::new();
        }

        let padding = 8;
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
            &["status.slot.recovery.primary", "status.slot.extension.scoped"]
        );
    }
}
