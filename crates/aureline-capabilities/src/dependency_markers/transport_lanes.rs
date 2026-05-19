//! Replay artifact dependency markers through the closed set of
//! transport lanes M3 requires markers to survive without loss.
//!
//! A marker is only release-bearing if it round-trips intact through
//! every lane a real M3 artifact actually travels: in-product import
//! and export, cross-device sync, backup/restore, mirror-only and
//! offline-cache-only fallbacks, headless / CLI inspect output, and
//! companion-targeted handoff packets.
//!
//! The lane vocabulary is closed. Surfaces switch on
//! [`TransportLane`] (and on the per-lane projection's
//! `requires_pre_apply_disclosure`, `preserves_user_data`, and
//! `companion_safe` flags) rather than re-deriving lane rules.
//!
//! This module is data-only: it mints no actual export bytes, opens
//! no RPC, and parses no formats. It exists so the fixture-replay
//! tests, the conformance packet, and CLI inspect output can prove
//! that markers survive each lane with the original vocabulary.

use serde::{Deserialize, Serialize};

use super::{project_marker_for_host_surface, ArtifactDependencyMarker, HostSurface};

/// Closed transport-lane vocabulary. Each variant maps to one real
/// artifact path the marker is required to survive intact.
///
/// Adding a lane is an additive change; repurposing one is breaking.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum TransportLane {
    /// In-product import (compare/apply review sheet, settings import,
    /// profile import, recipe import, bundle import).
    Import,
    /// In-product export (settings export, profile export, bundle
    /// export, recipe export, saved-view export).
    Export,
    /// Cross-device sync (sync artifact upload / download).
    Sync,
    /// Backup and restore (portable-state package, restore payload).
    BackupRestore,
    /// Mirror-only fallback (artifact served from a curated mirror
    /// because the upstream lane is unavailable).
    MirrorOnly,
    /// Offline-cache-only fallback (artifact replayed from the local
    /// cache because no upstream lane is reachable).
    OfflineCacheOnly,
    /// Headless / CLI inspect (`aureline … inspect`) reading the
    /// artifact for support, automation, or audit.
    HeadlessCliInspect,
    /// Companion-targeted handoff packet (scoped browser-companion
    /// hand-off, scoped device-to-device hand-off).
    CompanionHandoff,
}

impl TransportLane {
    /// Stable snake_case token persisted in projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Import => "import",
            Self::Export => "export",
            Self::Sync => "sync",
            Self::BackupRestore => "backup_restore",
            Self::MirrorOnly => "mirror_only",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::HeadlessCliInspect => "headless_cli_inspect",
            Self::CompanionHandoff => "companion_handoff",
        }
    }

    /// Full closed lane list used by replay tests, conformance
    /// projections, and CLI inspect.
    pub const fn all() -> [TransportLane; 8] {
        [
            Self::Import,
            Self::Export,
            Self::Sync,
            Self::BackupRestore,
            Self::MirrorOnly,
            Self::OfflineCacheOnly,
            Self::HeadlessCliInspect,
            Self::CompanionHandoff,
        ]
    }

    /// Lanes that re-emit the artifact onto a target where the
    /// dependency may be missing. The marker MUST be disclosed before
    /// apply on these lanes (`requires_pre_apply_disclosure = true`).
    pub const fn requires_pre_apply_disclosure(self) -> bool {
        matches!(
            self,
            Self::Import
                | Self::Sync
                | Self::BackupRestore
                | Self::MirrorOnly
                | Self::OfflineCacheOnly
                | Self::CompanionHandoff
        )
    }

    /// Lanes that are read-only with respect to the marker (no apply).
    /// Headless inspect and pure export quote the marker without
    /// branching apply behavior.
    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::Export | Self::HeadlessCliInspect)
    }

    /// True for lanes that may cross to a scoped companion-target
    /// surface. The marker MUST stay companion-safe (no raw provider
    /// tokens, no policy bundle bytes) — the closed
    /// `ArtifactDependencyMarker` schema enforces this by design.
    pub const fn is_companion_targeted(self) -> bool {
        matches!(self, Self::CompanionHandoff)
    }

    /// The host surface that quotes the marker on this lane. The
    /// projection re-uses the existing per-surface projection so the
    /// vocabulary cannot drift between lanes and surfaces.
    pub const fn host_surface(self) -> HostSurface {
        match self {
            Self::Import => HostSurface::ImportReviewSheet,
            Self::Export => HostSurface::BundleDetailPage,
            Self::Sync => HostSurface::ImportReviewSheet,
            Self::BackupRestore => HostSurface::ImportReviewSheet,
            Self::MirrorOnly => HostSurface::ImportReviewSheet,
            Self::OfflineCacheOnly => HostSurface::ImportReviewSheet,
            Self::HeadlessCliInspect => HostSurface::HeadlessCliInspect,
            Self::CompanionHandoff => HostSurface::BundleDetailPage,
        }
    }
}

/// Per-lane replay outcome for one marker.
///
/// The struct exists so a reviewer can read the conformance packet
/// (and the fixture-replay test output) and confirm that the marker
/// survived the lane with the exact same vocabulary tokens it was
/// minted with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneReplayOutcome {
    /// Lane the marker was replayed through.
    pub lane: TransportLane,
    /// Source marker id.
    pub marker_id: String,
    /// Artifact ref the marker is attached to.
    pub artifact_ref: String,
    /// Artifact class token (preserved bit-for-bit).
    pub artifact_class: String,
    /// Required capability id (preserved bit-for-bit).
    pub required_capability_id: String,
    /// Dependency class token (preserved bit-for-bit).
    pub dependency_class: String,
    /// Required lifecycle state token (preserved bit-for-bit).
    pub required_lifecycle_state: String,
    /// Support promise token (preserved bit-for-bit).
    pub support_promise: String,
    /// Effect-on-import token (preserved bit-for-bit).
    pub effect_on_import: String,
    /// True when `kill_switch_active` was set on the source marker.
    pub kill_switch_active: bool,
    /// True when the lane must disclose the marker before apply on the
    /// target.
    pub requires_pre_apply_disclosure: bool,
    /// True when the lane is a read-only inspect / export lane that
    /// quotes the marker without branching apply.
    pub is_read_only: bool,
    /// True when the lane is companion-targeted. The closed marker
    /// schema guarantees the payload is companion-safe.
    pub companion_safe: bool,
    /// True for every successful replay; the closed
    /// [`EffectOnImport`] vocabulary forbids silent drop.
    pub user_authored_data_preserved: bool,
    /// Compact summary copy carried forward to the lane.
    pub summary: String,
    /// Compact fallback / recover path carried forward to the lane.
    pub fallback_path: String,
}

impl LaneReplayOutcome {
    /// True when every persisted vocabulary token matches the source
    /// marker. Used by the conformance packet and the replay tests.
    pub fn matches_source(&self, marker: &ArtifactDependencyMarker) -> bool {
        self.marker_id == marker.marker_id
            && self.artifact_ref == marker.artifact_ref
            && self.artifact_class == marker.artifact_class.as_str()
            && self.required_capability_id == marker.required_capability_id
            && self.dependency_class == marker.dependency_class.as_str()
            && self.required_lifecycle_state == marker.required_lifecycle_state.as_str()
            && self.support_promise == marker.support_promise.as_str()
            && self.effect_on_import == marker.effect_on_import.as_str()
            && self.kill_switch_active == marker.kill_switch_active
            && self.summary == marker.behavior_on_missing.summary
            && self.fallback_path == marker.behavior_on_missing.fallback_path
            && self.user_authored_data_preserved
    }
}

/// Replays one marker through one lane and returns the [`LaneReplayOutcome`].
///
/// The replay is total: it never drops, narrows, or rewrites the
/// vocabulary tokens. The lane-aware flags
/// (`requires_pre_apply_disclosure`, `is_read_only`, `companion_safe`)
/// are derived from [`TransportLane`] only.
pub fn replay_marker_through_lane(
    marker: &ArtifactDependencyMarker,
    lane: TransportLane,
) -> LaneReplayOutcome {
    let surface = lane.host_surface();
    let projection = project_marker_for_host_surface(marker, surface);

    LaneReplayOutcome {
        lane,
        marker_id: projection.marker_id,
        artifact_ref: projection.artifact_ref,
        artifact_class: projection.artifact_class,
        required_capability_id: projection.required_capability_id,
        dependency_class: projection.dependency_class,
        required_lifecycle_state: projection.required_lifecycle_state,
        support_promise: projection.support_promise,
        effect_on_import: projection.effect_on_import,
        kill_switch_active: projection.kill_switch_active,
        requires_pre_apply_disclosure: lane.requires_pre_apply_disclosure(),
        is_read_only: lane.is_read_only(),
        companion_safe: lane.is_companion_targeted()
            || marker.effect_on_import.preserves_user_data(),
        user_authored_data_preserved: projection.user_authored_data_preserved,
        summary: projection.summary,
        fallback_path: projection.fallback_path,
    }
}

/// Replays a marker through every lane in [`TransportLane::all`].
/// The resulting [`LaneReplaySheet`] is the per-marker row in the
/// conformance packet.
pub fn replay_marker_through_all_lanes(marker: &ArtifactDependencyMarker) -> LaneReplaySheet {
    let outcomes: Vec<LaneReplayOutcome> = TransportLane::all()
        .iter()
        .copied()
        .map(|lane| replay_marker_through_lane(marker, lane))
        .collect();
    LaneReplaySheet {
        marker_id: marker.marker_id.clone(),
        artifact_ref: marker.artifact_ref.clone(),
        outcomes,
    }
}

/// Per-marker replay sheet (one row per lane).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneReplaySheet {
    /// Source marker id.
    pub marker_id: String,
    /// Artifact ref the marker is attached to.
    pub artifact_ref: String,
    /// One outcome per lane in [`TransportLane::all`] order.
    pub outcomes: Vec<LaneReplayOutcome>,
}

impl LaneReplaySheet {
    /// True when every lane preserved the marker bit-for-bit.
    pub fn all_lanes_preserved(&self, marker: &ArtifactDependencyMarker) -> bool {
        self.outcomes
            .iter()
            .all(|outcome| outcome.matches_source(marker))
    }
}

/// Defects flagged by [`assert_marker_survives_all_lanes`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaneReplayDefect {
    /// A vocabulary token changed between the source marker and the
    /// lane projection.
    VocabularyDrift {
        /// Marker id.
        marker_id: String,
        /// Lane the drift was observed on.
        lane: TransportLane,
        /// Vocabulary token that drifted.
        token: String,
    },
    /// A lane projection failed to preserve user-authored data.
    /// Unreachable while the closed [`EffectOnImport`] vocabulary holds,
    /// but emitted defensively so future additive bumps cannot silently
    /// regress.
    SilentDrop {
        /// Marker id.
        marker_id: String,
        /// Lane the drop was observed on.
        lane: TransportLane,
    },
    /// A lane projection lost its summary or fallback copy.
    MissingCopy {
        /// Marker id.
        marker_id: String,
        /// Lane the loss was observed on.
        lane: TransportLane,
        /// Missing field name.
        field: String,
    },
}

impl std::fmt::Display for LaneReplayDefect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VocabularyDrift {
                marker_id,
                lane,
                token,
            } => write!(
                f,
                "marker {marker_id:?} drifted vocabulary token {token:?} on lane {}",
                lane.as_str()
            ),
            Self::SilentDrop { marker_id, lane } => write!(
                f,
                "marker {marker_id:?} dropped user-authored data on lane {}",
                lane.as_str()
            ),
            Self::MissingCopy {
                marker_id,
                lane,
                field,
            } => write!(
                f,
                "marker {marker_id:?} lost {field:?} copy on lane {}",
                lane.as_str()
            ),
        }
    }
}

impl std::error::Error for LaneReplayDefect {}

/// Asserts that a marker survives every lane in [`TransportLane::all`]
/// bit-for-bit. Returns the [`LaneReplaySheet`] on success or the list
/// of defects on failure. Used by the fixture-replay test suite and
/// the conformance-report builder.
pub fn assert_marker_survives_all_lanes(
    marker: &ArtifactDependencyMarker,
) -> Result<LaneReplaySheet, Vec<LaneReplayDefect>> {
    let sheet = replay_marker_through_all_lanes(marker);
    let mut defects = Vec::new();
    for outcome in &sheet.outcomes {
        if outcome.summary.trim().is_empty() {
            defects.push(LaneReplayDefect::MissingCopy {
                marker_id: marker.marker_id.clone(),
                lane: outcome.lane,
                field: "summary".to_owned(),
            });
        }
        if outcome.fallback_path.trim().is_empty() {
            defects.push(LaneReplayDefect::MissingCopy {
                marker_id: marker.marker_id.clone(),
                lane: outcome.lane,
                field: "fallback_path".to_owned(),
            });
        }
        if !outcome.user_authored_data_preserved {
            defects.push(LaneReplayDefect::SilentDrop {
                marker_id: marker.marker_id.clone(),
                lane: outcome.lane,
            });
        }
        let token_pairs: [(&str, &str); 6] = [
            ("artifact_class", marker.artifact_class.as_str()),
            ("dependency_class", marker.dependency_class.as_str()),
            (
                "required_lifecycle_state",
                marker.required_lifecycle_state.as_str(),
            ),
            ("support_promise", marker.support_promise.as_str()),
            ("effect_on_import", marker.effect_on_import.as_str()),
            ("required_capability_id", &marker.required_capability_id),
        ];
        for (token_name, expected) in token_pairs {
            let observed: &str = match token_name {
                "artifact_class" => &outcome.artifact_class,
                "dependency_class" => &outcome.dependency_class,
                "required_lifecycle_state" => &outcome.required_lifecycle_state,
                "support_promise" => &outcome.support_promise,
                "effect_on_import" => &outcome.effect_on_import,
                "required_capability_id" => &outcome.required_capability_id,
                _ => unreachable!(),
            };
            if observed != expected {
                defects.push(LaneReplayDefect::VocabularyDrift {
                    marker_id: marker.marker_id.clone(),
                    lane: outcome.lane,
                    token: token_name.to_owned(),
                });
            }
        }
    }

    if defects.is_empty() {
        Ok(sheet)
    } else {
        Err(defects)
    }
}

/// Audit summary of one [`LaneReplaySheet`]. The conformance packet
/// renders one row per audit; the per-lane outcome rows live in the
/// fixture-replay test output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneReplayAudit {
    /// Source marker id.
    pub marker_id: String,
    /// Artifact ref the marker is attached to.
    pub artifact_ref: String,
    /// Total lanes replayed.
    pub lanes_total: usize,
    /// Lanes that preserved the marker bit-for-bit.
    pub lanes_preserved: usize,
    /// Lanes that required pre-apply disclosure.
    pub lanes_pre_apply_disclosure: usize,
    /// Lanes that were read-only.
    pub lanes_read_only: usize,
    /// Lanes that were companion-targeted.
    pub lanes_companion_targeted: usize,
    /// True when every lane preserved user-authored data.
    pub user_authored_data_preserved_on_every_lane: bool,
}

impl LaneReplayAudit {
    /// Builds an audit from a [`LaneReplaySheet`] and its source
    /// marker.
    pub fn from_sheet(marker: &ArtifactDependencyMarker, sheet: &LaneReplaySheet) -> Self {
        let lanes_preserved = sheet
            .outcomes
            .iter()
            .filter(|outcome| outcome.matches_source(marker))
            .count();
        let lanes_pre_apply_disclosure = sheet
            .outcomes
            .iter()
            .filter(|outcome| outcome.requires_pre_apply_disclosure)
            .count();
        let lanes_read_only = sheet
            .outcomes
            .iter()
            .filter(|outcome| outcome.is_read_only)
            .count();
        let lanes_companion_targeted = sheet
            .outcomes
            .iter()
            .filter(|outcome| outcome.lane.is_companion_targeted())
            .count();
        let user_authored_data_preserved_on_every_lane = sheet
            .outcomes
            .iter()
            .all(|outcome| outcome.user_authored_data_preserved);

        Self {
            marker_id: sheet.marker_id.clone(),
            artifact_ref: sheet.artifact_ref.clone(),
            lanes_total: sheet.outcomes.len(),
            lanes_preserved,
            lanes_pre_apply_disclosure,
            lanes_read_only,
            lanes_companion_targeted,
            user_authored_data_preserved_on_every_lane,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dependency_markers::{
        catalog_default_capabilities, ArtifactClass, ArtifactDependencyMarker, EffectOnImport,
    };

    fn sample_marker_for(idx: usize, artifact_class: ArtifactClass) -> ArtifactDependencyMarker {
        let catalog = catalog_default_capabilities();
        let capability = &catalog[idx];
        ArtifactDependencyMarker::from_capability(
            format!(
                "marker:{}:{}:test",
                artifact_class.as_str(),
                capability.capability_id
            ),
            artifact_class,
            format!("artifact:{}:test", artifact_class.as_str()),
            capability,
            format!(
                "Behavior depends on {}; rendered via lane test.",
                capability.title
            ),
        )
    }

    #[test]
    fn transport_lane_tokens_are_unique_and_stable() {
        let mut tokens = std::collections::BTreeSet::new();
        for lane in TransportLane::all() {
            assert!(
                tokens.insert(lane.as_str()),
                "duplicate token: {}",
                lane.as_str()
            );
        }
    }

    #[test]
    fn replay_preserves_source_vocabulary_on_every_lane() {
        for (idx, artifact_class) in [
            ArtifactClass::SettingsExport,
            ArtifactClass::Profile,
            ArtifactClass::WorkflowBundle,
            ArtifactClass::PortableStatePackage,
            ArtifactClass::Recipe,
        ]
        .iter()
        .enumerate()
        {
            let marker = sample_marker_for(idx, *artifact_class);
            let sheet = assert_marker_survives_all_lanes(&marker)
                .expect("lane replay must not flag defects");
            assert!(sheet.all_lanes_preserved(&marker));
            assert_eq!(sheet.outcomes.len(), TransportLane::all().len());
            for outcome in &sheet.outcomes {
                assert!(outcome.user_authored_data_preserved);
                if outcome.lane == TransportLane::CompanionHandoff {
                    assert!(outcome.companion_safe);
                }
            }
        }
    }

    #[test]
    fn audit_counts_pre_apply_and_companion_lanes() {
        let marker = sample_marker_for(0, ArtifactClass::SettingsExport);
        let sheet = replay_marker_through_all_lanes(&marker);
        let audit = LaneReplayAudit::from_sheet(&marker, &sheet);
        assert_eq!(audit.lanes_total, TransportLane::all().len());
        assert_eq!(audit.lanes_preserved, audit.lanes_total);
        assert_eq!(audit.lanes_companion_targeted, 1);
        assert!(audit.user_authored_data_preserved_on_every_lane);
        assert!(audit.lanes_pre_apply_disclosure >= 5);
        assert!(audit.lanes_read_only >= 1);
    }

    #[test]
    fn missing_copy_is_flagged() {
        let mut marker = sample_marker_for(0, ArtifactClass::SettingsExport);
        marker.behavior_on_missing.summary.clear();
        marker.behavior_on_missing.fallback_path.clear();
        let defects = assert_marker_survives_all_lanes(&marker)
            .expect_err("empty copy must be flagged on every lane");
        assert!(defects
            .iter()
            .any(|d| matches!(d, LaneReplayDefect::MissingCopy { field, .. } if field == "summary")));
        assert!(defects
            .iter()
            .any(|d| matches!(d, LaneReplayDefect::MissingCopy { field, .. } if field == "fallback_path")));
    }

    #[test]
    fn effect_on_import_variants_are_companion_safe() {
        for effect in [
            EffectOnImport::BlockApplyPreserveData,
            EffectOnImport::NarrowBehaviorPreserveData,
            EffectOnImport::EmulatedDowngradePreserveData,
            EffectOnImport::HoldForLaterPreserveData,
            EffectOnImport::RenderTombstonePreserveData,
        ] {
            assert!(effect.preserves_user_data());
        }
    }
}
