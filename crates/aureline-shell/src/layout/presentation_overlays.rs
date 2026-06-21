//! Placement geometry for presentation overlays on the canonical shell zones.
//!
//! Presentation overlays — the presenter bar, agenda / waypoint rail, spotlight
//! frame, speaker-notes tray, audience strip, breakaway banner, and provenance
//! strip — are a **thin layer over the existing pane-and-navigation system**,
//! never a second workspace shell. This module places each overlay surface onto
//! an existing [`ShellZoneId`] from the live [`ZoneRegistryLayout`] instead of
//! minting new top-level chrome, and proves the placement never removes or
//! resizes an underlying pane:
//!
//! - the presenter bar and provenance strip ride the title / context bar so the
//!   file, symbol, branch, workspace, and boundary labels stay visible;
//! - the agenda / waypoint rail attaches to the left sidebar, floating into the
//!   transient overlay only when the sidebar is collapsed on narrow widths;
//! - the spotlight frame is a strict **inset** within the main workspace pane —
//!   it dims the surroundings without ever replacing the pane;
//! - the speaker-notes tray attaches to the right inspector (then the bottom
//!   panel, then a floated panel) so it never steals the editor's space;
//! - the audience strip rides the status bar, and the breakaway banner floats in
//!   the transient overlay zone.
//!
//! When a preferred host zone is collapsed by the responsive zone registry, the
//! placement falls back to a floated rect inside the always-present transient
//! overlay zone rather than forcing the zone back open — keeping presentation a
//! guest of the layout, not its owner.

use crate::layout::zone_registry::{Rect, ShellZoneId, ZoneRegistryLayout};

/// Inset (logical px) the spotlight frame keeps inside the main workspace pane.
const SPOTLIGHT_INSET_PX: u32 = 24;
/// Width (logical px) of the waypoint rail when it floats into the overlay zone.
const FLOATED_RAIL_WIDTH_PX: u32 = 280;
/// Width (logical px) of the speaker-notes tray when it floats.
const FLOATED_NOTES_WIDTH_PX: u32 = 360;
/// Height (logical px) of the speaker-notes tray when it floats.
const FLOATED_NOTES_HEIGHT_PX: u32 = 220;
/// Height (logical px) of the breakaway banner floated below the title bar.
const BREAKAWAY_BANNER_HEIGHT_PX: u32 = 32;

/// One presentation overlay surface placed over the existing shell zones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationOverlaySurface {
    /// Presenter bar: guided-mode controls (zoom presets, spotlight, notes, exit).
    PresenterBar,
    /// Provenance strip: file / symbol / branch / workspace + boundary labels.
    ProvenanceStrip,
    /// Agenda / waypoint rail: prepared anchors in a predictable order.
    WaypointRail,
    /// Spotlight frame: directs attention as an inset within the main workspace.
    SpotlightFrame,
    /// Speaker-notes tray: presenter-only prompts, local by default.
    SpeakerNotesTray,
    /// Audience strip / follow chip: who is following or broken away.
    AudienceStrip,
    /// Breakaway banner: the durable "browsing independently" banner.
    BreakawayBanner,
}

impl PresentationOverlaySurface {
    /// Every overlay surface, in placement order.
    pub const ALL: [Self; 7] = [
        Self::PresenterBar,
        Self::ProvenanceStrip,
        Self::WaypointRail,
        Self::SpotlightFrame,
        Self::SpeakerNotesTray,
        Self::AudienceStrip,
        Self::BreakawayBanner,
    ];

    /// Stable token recorded in placements and bindings.
    pub const fn name(self) -> &'static str {
        match self {
            Self::PresenterBar => "presenter_bar",
            Self::ProvenanceStrip => "provenance_strip",
            Self::WaypointRail => "waypoint_rail",
            Self::SpotlightFrame => "spotlight_frame",
            Self::SpeakerNotesTray => "speaker_notes_tray",
            Self::AudienceStrip => "audience_strip",
            Self::BreakawayBanner => "breakaway_banner",
        }
    }

    /// The shell zone this surface prefers to ride. A presentation overlay never
    /// invents a top-level zone; it attaches to one of the canonical zones.
    pub const fn preferred_host_zone(self) -> ShellZoneId {
        match self {
            Self::PresenterBar | Self::ProvenanceStrip => ShellZoneId::TitleContextBar,
            Self::WaypointRail => ShellZoneId::LeftSidebar,
            Self::SpotlightFrame => ShellZoneId::MainWorkspace,
            Self::SpeakerNotesTray => ShellZoneId::RightInspector,
            Self::AudienceStrip => ShellZoneId::StatusBar,
            Self::BreakawayBanner => ShellZoneId::TransientOverlay,
        }
    }
}

/// A resolved placement of one overlay surface over the live zone layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PresentationOverlayPlacement {
    /// The overlay surface placed.
    pub surface: PresentationOverlaySurface,
    /// The shell zone the surface is hosted on.
    pub host_zone: ShellZoneId,
    /// The rect, in window-local logical px, the surface occupies.
    pub rect: Rect,
    /// True when the preferred host zone was collapsed and the surface fell back
    /// to a floated rect in the transient overlay zone.
    pub is_fallback_placement: bool,
}

impl PresentationOverlayPlacement {
    /// Overlays never replace an underlying pane; they layer over it.
    pub const fn replaces_underlying_pane(self) -> bool {
        false
    }

    /// The underlying pane stays visible beneath every overlay placement.
    pub const fn underlying_pane_visible(self) -> bool {
        true
    }

    /// Whether the placement rect lies wholly within the window bounds.
    pub fn within_window(self, layout: &ZoneRegistryLayout) -> bool {
        rect_contains(layout.window, self.rect)
    }
}

/// Place each active overlay surface onto the live zone layout.
///
/// The main workspace pane is never shrunk or removed: the spotlight frame is a
/// strict inset within it, and every other surface rides another zone or floats
/// in the transient overlay zone.
pub fn plan_presentation_overlays(
    layout: &ZoneRegistryLayout,
    active: &[PresentationOverlaySurface],
) -> Vec<PresentationOverlayPlacement> {
    let share_title_bar = active.contains(&PresentationOverlaySurface::PresenterBar)
        && active.contains(&PresentationOverlaySurface::ProvenanceStrip);

    active
        .iter()
        .map(|&surface| place_surface(layout, surface, share_title_bar))
        .collect()
}

/// Whether the main workspace pane is preserved under the given placements: no
/// placement occupies the main workspace except the spotlight frame, which must
/// be a strict inset within it rather than a replacement.
pub fn main_workspace_preserved(
    layout: &ZoneRegistryLayout,
    placements: &[PresentationOverlayPlacement],
) -> bool {
    let main = layout.main_workspace;
    placements.iter().all(|placement| {
        if placement.host_zone != ShellZoneId::MainWorkspace {
            return true;
        }
        placement.surface == PresentationOverlaySurface::SpotlightFrame
            && rect_strictly_inside(placement.rect, main)
    })
}

fn place_surface(
    layout: &ZoneRegistryLayout,
    surface: PresentationOverlaySurface,
    share_title_bar: bool,
) -> PresentationOverlayPlacement {
    use PresentationOverlaySurface as S;
    match surface {
        S::PresenterBar => title_bar_placement(layout, surface, share_title_bar, false),
        S::ProvenanceStrip => title_bar_placement(layout, surface, share_title_bar, true),
        S::WaypointRail => with_fallback(
            layout,
            surface,
            ShellZoneId::LeftSidebar,
            floated_rail_rect(layout),
        ),
        S::SpotlightFrame => PresentationOverlayPlacement {
            surface,
            host_zone: ShellZoneId::MainWorkspace,
            rect: inset(layout.main_workspace, SPOTLIGHT_INSET_PX),
            is_fallback_placement: false,
        },
        S::SpeakerNotesTray => notes_tray_placement(layout, surface),
        S::AudienceStrip => PresentationOverlayPlacement {
            surface,
            host_zone: ShellZoneId::StatusBar,
            rect: layout.status_bar,
            is_fallback_placement: false,
        },
        S::BreakawayBanner => PresentationOverlayPlacement {
            surface,
            host_zone: ShellZoneId::TransientOverlay,
            rect: breakaway_banner_rect(layout),
            is_fallback_placement: false,
        },
    }
}

fn title_bar_placement(
    layout: &ZoneRegistryLayout,
    surface: PresentationOverlaySurface,
    share: bool,
    is_provenance: bool,
) -> PresentationOverlayPlacement {
    let bar = layout.title_context_bar;
    let rect = if share {
        // Provenance keeps the left half so source identity stays visible; the
        // presenter controls take the right half.
        let half = bar.width / 2;
        if is_provenance {
            Rect::new(bar.x, bar.y, half, bar.height)
        } else {
            Rect::new(bar.x.saturating_add(half), bar.y, bar.width.saturating_sub(half), bar.height)
        }
    } else {
        bar
    };
    PresentationOverlayPlacement {
        surface,
        host_zone: ShellZoneId::TitleContextBar,
        rect,
        is_fallback_placement: false,
    }
}

fn notes_tray_placement(
    layout: &ZoneRegistryLayout,
    surface: PresentationOverlaySurface,
) -> PresentationOverlayPlacement {
    if let Some(rect) = layout.right_inspector {
        return PresentationOverlayPlacement {
            surface,
            host_zone: ShellZoneId::RightInspector,
            rect,
            is_fallback_placement: false,
        };
    }
    if let Some(rect) = layout.bottom_panel {
        return PresentationOverlayPlacement {
            surface,
            host_zone: ShellZoneId::BottomPanel,
            rect,
            is_fallback_placement: false,
        };
    }
    PresentationOverlayPlacement {
        surface,
        host_zone: ShellZoneId::TransientOverlay,
        rect: floated_notes_rect(layout),
        is_fallback_placement: true,
    }
}

fn with_fallback(
    layout: &ZoneRegistryLayout,
    surface: PresentationOverlaySurface,
    preferred: ShellZoneId,
    floated: Rect,
) -> PresentationOverlayPlacement {
    match layout.zone(preferred) {
        Some(rect) => PresentationOverlayPlacement {
            surface,
            host_zone: preferred,
            rect,
            is_fallback_placement: false,
        },
        None => PresentationOverlayPlacement {
            surface,
            host_zone: ShellZoneId::TransientOverlay,
            rect: floated,
            is_fallback_placement: true,
        },
    }
}

fn content_band(layout: &ZoneRegistryLayout) -> (u32, u32) {
    let top = layout.title_context_bar.bottom();
    let bottom = layout.status_bar.y.max(top);
    (top, bottom.saturating_sub(top))
}

fn floated_rail_rect(layout: &ZoneRegistryLayout) -> Rect {
    let (top, height) = content_band(layout);
    let width = FLOATED_RAIL_WIDTH_PX.min(layout.window.width);
    Rect::new(layout.window.x, top, width, height)
}

fn floated_notes_rect(layout: &ZoneRegistryLayout) -> Rect {
    let (_, band_height) = content_band(layout);
    let width = FLOATED_NOTES_WIDTH_PX.min(layout.window.width);
    let height = FLOATED_NOTES_HEIGHT_PX.min(band_height);
    let x = layout.window.right().saturating_sub(width);
    let y = layout.status_bar.y.saturating_sub(height);
    Rect::new(x, y, width, height)
}

fn breakaway_banner_rect(layout: &ZoneRegistryLayout) -> Rect {
    let top = layout.title_context_bar.bottom();
    let height = BREAKAWAY_BANNER_HEIGHT_PX.min(content_band(layout).1);
    Rect::new(layout.window.x, top, layout.window.width, height)
}

fn inset(rect: Rect, pad: u32) -> Rect {
    let pad = pad.min(rect.width / 3).min(rect.height / 3);
    Rect::new(
        rect.x.saturating_add(pad),
        rect.y.saturating_add(pad),
        rect.width.saturating_sub(pad.saturating_mul(2)),
        rect.height.saturating_sub(pad.saturating_mul(2)),
    )
}

fn rect_contains(outer: Rect, inner: Rect) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.right() <= outer.right()
        && inner.bottom() <= outer.bottom()
}

fn rect_strictly_inside(inner: Rect, outer: Rect) -> bool {
    !inner.is_empty() && inner != outer && rect_contains(outer, inner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::zone_registry::{ZoneDefaults, ZoneRegistry, ZoneRegistryInput};

    fn expanded_layout() -> ZoneRegistryLayout {
        ZoneRegistry::new(ZoneDefaults::standard()).layout(ZoneRegistryInput {
            window_width: 1920,
            window_height: 1080,
            split_heavy: false,
            main_workspace_min_width_override: None,
        })
    }

    fn compact_layout() -> ZoneRegistryLayout {
        ZoneRegistry::new(ZoneDefaults::standard()).layout(ZoneRegistryInput {
            window_width: 1024,
            window_height: 720,
            split_heavy: false,
            main_workspace_min_width_override: None,
        })
    }

    #[test]
    fn every_surface_attaches_to_an_existing_zone_and_keeps_the_pane() {
        let layout = expanded_layout();
        let placements = plan_presentation_overlays(&layout, &PresentationOverlaySurface::ALL);
        assert_eq!(placements.len(), PresentationOverlaySurface::ALL.len());
        for placement in &placements {
            assert!(!placement.replaces_underlying_pane());
            assert!(placement.underlying_pane_visible());
            assert!(
                placement.within_window(&layout),
                "{} escapes the window",
                placement.surface.name()
            );
        }
        assert!(main_workspace_preserved(&layout, &placements));
    }

    #[test]
    fn spotlight_is_a_strict_inset_within_the_main_workspace() {
        let layout = expanded_layout();
        let placements =
            plan_presentation_overlays(&layout, &[PresentationOverlaySurface::SpotlightFrame]);
        let spotlight = placements[0];
        assert_eq!(spotlight.host_zone, ShellZoneId::MainWorkspace);
        assert!(rect_strictly_inside(spotlight.rect, layout.main_workspace));
        assert_ne!(spotlight.rect, layout.main_workspace);
    }

    #[test]
    fn provenance_and_presenter_share_the_title_bar_without_overlapping() {
        let layout = expanded_layout();
        let placements = plan_presentation_overlays(
            &layout,
            &[
                PresentationOverlaySurface::PresenterBar,
                PresentationOverlaySurface::ProvenanceStrip,
            ],
        );
        let presenter = placements
            .iter()
            .find(|p| p.surface == PresentationOverlaySurface::PresenterBar)
            .unwrap();
        let provenance = placements
            .iter()
            .find(|p| p.surface == PresentationOverlaySurface::ProvenanceStrip)
            .unwrap();
        assert_eq!(presenter.host_zone, ShellZoneId::TitleContextBar);
        assert_eq!(provenance.host_zone, ShellZoneId::TitleContextBar);
        // Provenance keeps the left edge; the two halves do not overlap.
        assert_eq!(provenance.rect.x, layout.title_context_bar.x);
        assert!(provenance.rect.right() <= presenter.rect.x);
    }

    #[test]
    fn waypoint_rail_floats_into_overlay_when_sidebar_is_collapsed() {
        let layout = compact_layout();
        assert!(
            layout.left_sidebar.is_none(),
            "compact layout should collapse the sidebar"
        );
        let placements =
            plan_presentation_overlays(&layout, &[PresentationOverlaySurface::WaypointRail]);
        let rail = placements[0];
        assert!(rail.is_fallback_placement);
        assert_eq!(rail.host_zone, ShellZoneId::TransientOverlay);
        assert!(rail.within_window(&layout));
    }

    #[test]
    fn notes_tray_falls_back_off_the_inspector_without_taking_the_editor() {
        let layout = compact_layout();
        let placements =
            plan_presentation_overlays(&layout, &[PresentationOverlaySurface::SpeakerNotesTray]);
        let tray = placements[0];
        assert_ne!(tray.host_zone, ShellZoneId::MainWorkspace);
        assert!(tray.within_window(&layout));
    }
}
