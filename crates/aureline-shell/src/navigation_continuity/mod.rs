//! Desktop breadcrumb, outline, bookmark, history, and peek chrome bound to the
//! navigation-continuity matrix.
//!
//! This module is the desktop first consumer of
//! [`NavigationContinuityBindingPacket`](aureline_search::NavigationContinuityBindingPacket).
//! It projects a compact, inspectable per-surface set of continuity cues that
//! reuse the canonical continuity artifacts verbatim, so the shell's breadcrumb
//! bar, outline tree, bookmark gutter, back/forward, recent-locations, and peek
//! overlay land on the same anchors that the back/forward, session-restore, and
//! support-replay consumers see.
//!
//! Crucially, the projection does **not** flatten a drifted, missing, scope-
//! unavailable, or archived artifact into a plain "open": the cue carries the
//! visible drift reason and recovery choices so the chrome renders an explicit,
//! recoverable cue instead of silently relocating to a nearby symbol, line, or
//! document. The shell mints no new anchor identity; it preserves the canonical
//! and resolved target refs, the durable result identity, the origin/return
//! anchor, and the drift state.

use aureline_navigation::{
    NavigationContinuityArtifactKind, NavigationContinuitySurface, NavigationDriftState,
};
use aureline_search::{ContinuityConsumerClass, HistoryRole, NavigationContinuityBindingPacket};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`NavigationContinuityProjectionSet`].
pub const NAVIGATION_CONTINUITY_PROJECTION_SET_RECORD_KIND: &str =
    "navigation_continuity_projection_set";

/// Schema version for [`NavigationContinuityProjectionSet`].
pub const NAVIGATION_CONTINUITY_PROJECTION_SET_SCHEMA_VERSION: u32 = 1;

/// One desktop continuity cue bound to a single continuity artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityCue {
    /// Artifact id reused from the substrate.
    pub artifact_id: String,
    /// Artifact kind reused from the substrate.
    pub artifact_kind: NavigationContinuityArtifactKind,
    /// Surface that owns the cue.
    pub surface: NavigationContinuitySurface,
    /// Display title preserved verbatim, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Canonical anchor identity resolved before any remap rules ran.
    pub canonical_target_ref: String,
    /// Resolved target ref, when the anchor still resolves.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_target_ref: Option<String>,
    /// Origin/return anchor for history and peek cues.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_target_ref: Option<String>,
    /// Recent-navigation role for history cues.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_role: Option<HistoryRole>,
    /// Drift state reused from the substrate.
    pub drift_state: NavigationDriftState,
    /// True when the cue must render a visible, recoverable drift state.
    pub needs_visible_reason: bool,
    /// User-visible drift reason; present iff the cue needs a visible reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_reason: Option<String>,
    /// User-visible recovery choices; present iff the cue needs a visible reason.
    pub recovery_choices: Vec<String>,
    /// True when the cue preserves authority (never widened by routing).
    pub authority_preserved: bool,
}

impl ContinuityCue {
    /// True when the cue keeps an attributable anchor and, if it needs a visible
    /// reason, a non-empty drift reason and recovery choices — never a silent
    /// relocation to a nearby target.
    pub fn is_attributable(&self) -> bool {
        !self.canonical_target_ref.is_empty()
            && self.authority_preserved
            && (!self.needs_visible_reason
                || (self
                    .drift_reason
                    .as_ref()
                    .is_some_and(|reason| !reason.trim().is_empty())
                    && !self.recovery_choices.is_empty()
                    && self.resolved_target_ref.is_none()))
    }
}

/// Per-surface group of continuity cues.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuitySurfaceProjection {
    /// Surface token reused from the substrate.
    pub surface: NavigationContinuitySurface,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Continuity cues, in artifact order.
    pub cues: Vec<ContinuityCue>,
}

impl ContinuitySurfaceProjection {
    /// Number of cues on this surface that render a visible drift state.
    pub fn drifted_cue_count(&self) -> usize {
        self.cues
            .iter()
            .filter(|cue| cue.needs_visible_reason)
            .count()
    }
}

/// Desktop projection of the navigation-continuity matrix across all surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityProjectionSet {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet id the desktop ingests verbatim.
    pub ingested_packet_id: String,
    /// Per-surface projections, in substrate order.
    pub surfaces: Vec<ContinuitySurfaceProjection>,
}

impl NavigationContinuityProjectionSet {
    /// Returns the projection for one surface, if present.
    pub fn surface_for(
        &self,
        surface: NavigationContinuitySurface,
    ) -> Option<&ContinuitySurfaceProjection> {
        self.surfaces
            .iter()
            .find(|projection| projection.surface == surface)
    }

    /// True when every surface is bound and every cue keeps an attributable
    /// anchor with visible, recoverable drift states.
    pub fn reuses_substrate(&self) -> bool {
        !self.surfaces.is_empty()
            && self.surfaces.iter().all(|surface| {
                !surface.cues.is_empty() && surface.cues.iter().all(ContinuityCue::is_attributable)
            })
    }
}

/// Projects the desktop continuity cues from the navigation-continuity packet.
///
/// The projection reuses the canonical anchors, drift states, reasons, and
/// origin/return refs verbatim; it mints no new anchor identity, so the desktop
/// chrome shares one truth with the history, restore, and support consumers.
pub fn project_navigation_continuity(
    packet: &NavigationContinuityBindingPacket,
) -> NavigationContinuityProjectionSet {
    let surfaces = packet
        .surfaces
        .iter()
        .map(|surface| ContinuitySurfaceProjection {
            surface: surface.surface,
            surface_label: surface.surface_label.clone(),
            cues: surface
                .artifacts
                .iter()
                .map(|artifact| ContinuityCue {
                    artifact_id: artifact.artifact_id.clone(),
                    artifact_kind: artifact.artifact_kind,
                    surface: artifact.surface,
                    label: artifact.label.clone(),
                    canonical_target_ref: artifact.canonical_target_ref.clone(),
                    resolved_target_ref: artifact.resolved_target_ref.clone(),
                    origin_target_ref: artifact.origin_target_ref.clone(),
                    history_role: artifact.history_role,
                    drift_state: artifact.drift_state,
                    needs_visible_reason: artifact.requires_visible_reason(),
                    drift_reason: artifact.drift_reason.clone(),
                    recovery_choices: artifact.recovery_choices.clone(),
                    authority_preserved: artifact.authority_not_widened,
                })
                .collect(),
        })
        .collect();

    NavigationContinuityProjectionSet {
        record_kind: NAVIGATION_CONTINUITY_PROJECTION_SET_RECORD_KIND.to_string(),
        schema_version: NAVIGATION_CONTINUITY_PROJECTION_SET_SCHEMA_VERSION,
        ingested_packet_id: packet.packet_id.clone(),
        surfaces,
    }
}

/// True when the packet names the desktop a first consumer that reuses the same
/// continuity objects without widening authority.
pub fn shell_is_continuity_first_consumer(packet: &NavigationContinuityBindingPacket) -> bool {
    packet.consumer_projections.iter().any(|projection| {
        projection.consumer == ContinuityConsumerClass::ProductUi
            && projection.reuses_same_continuity_objects
            && projection.preserves_drift_vocabulary
            && projection.preserves_drift_reasons
            && projection.preserves_origin_destination
            && !projection.widens_authority
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_search::{
        seeded_navigation_continuity_packet, seeded_workset_drift_navigation_continuity_packet,
    };

    #[test]
    fn projects_a_surface_for_every_continuity_surface() {
        let packet = seeded_navigation_continuity_packet();
        let set = project_navigation_continuity(&packet);
        assert_eq!(
            set.record_kind,
            NAVIGATION_CONTINUITY_PROJECTION_SET_RECORD_KIND
        );
        assert_eq!(set.ingested_packet_id, packet.packet_id);
        assert_eq!(set.surfaces.len(), packet.surfaces.len());
        assert!(set.reuses_substrate());
        for surface in [
            NavigationContinuitySurface::Editor,
            NavigationContinuitySurface::Diff,
            NavigationContinuitySurface::Notebook,
            NavigationContinuitySurface::Docs,
            NavigationContinuitySurface::Search,
            NavigationContinuitySurface::Topology,
        ] {
            assert!(set.surface_for(surface).is_some(), "missing {surface:?}");
        }
        assert!(shell_is_continuity_first_consumer(&packet));
    }

    #[test]
    fn drifted_cues_render_a_visible_recoverable_reason() {
        // A drifted/missing/archived artifact never flattens into a plain open:
        // the cue keeps the visible reason and recovery choices the chrome must
        // render, and never claims a resolved target.
        let packet = seeded_navigation_continuity_packet();
        let set = project_navigation_continuity(&packet);
        let drifted: Vec<_> = set
            .surfaces
            .iter()
            .flat_map(|surface| &surface.cues)
            .filter(|cue| cue.needs_visible_reason)
            .collect();
        assert!(!drifted.is_empty());
        for cue in drifted {
            assert!(cue.drift_reason.is_some());
            assert!(!cue.recovery_choices.is_empty());
            assert!(cue.resolved_target_ref.is_none());
        }
    }

    #[test]
    fn history_cues_keep_origin_and_role() {
        let packet = seeded_navigation_continuity_packet();
        let set = project_navigation_continuity(&packet);
        let history: Vec<_> = set
            .surfaces
            .iter()
            .flat_map(|surface| &surface.cues)
            .filter(|cue| {
                cue.artifact_kind == NavigationContinuityArtifactKind::NavigationHistoryEntry
            })
            .collect();
        assert!(!history.is_empty());
        for cue in history {
            assert!(cue.history_role.is_some());
            assert!(cue
                .origin_target_ref
                .as_ref()
                .is_some_and(|origin| origin != &cue.canonical_target_ref));
        }
    }

    #[test]
    fn workset_drift_projection_shows_more_visible_drift() {
        let canonical = project_navigation_continuity(&seeded_navigation_continuity_packet());
        let drifted =
            project_navigation_continuity(&seeded_workset_drift_navigation_continuity_packet());
        assert!(drifted.reuses_substrate());
        let canonical_drift: usize = canonical
            .surfaces
            .iter()
            .map(ContinuitySurfaceProjection::drifted_cue_count)
            .sum();
        let drifted_drift: usize = drifted
            .surfaces
            .iter()
            .map(ContinuitySurfaceProjection::drifted_cue_count)
            .sum();
        assert!(drifted_drift > canonical_drift);
    }

    #[test]
    fn round_trips_through_json() {
        let packet = seeded_navigation_continuity_packet();
        let set = project_navigation_continuity(&packet);
        let json = serde_json::to_string(&set).expect("projection serializes");
        let round_trip: NavigationContinuityProjectionSet =
            serde_json::from_str(&json).expect("projection deserializes");
        assert_eq!(round_trip, set);
    }
}
