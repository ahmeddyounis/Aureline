//! Corpus manifest types for the warm-start, prebuild, and live-resume drill
//! suite.
//!
//! The manifest is the single source of truth for the corpus. Each positive
//! drill names a fixture that carries one warm-start choice card and pins the
//! truth the card must reproduce — the source class, support class, runtime/host
//! class, the entry lanes it offers, per-lane availability, the snapshot
//! freshness / age / invalidation facts, the environment-starter setup location,
//! the local-safe default, and the honesty marker. Each negative drill names a
//! contract-valid base card and a typed tamper that, applied to the card, MUST
//! raise a finding whose message contains `expected_failure_substring`, so a
//! warm-start path that presents a stale snapshot as a live resume, masquerades a
//! networked lane as a local open, drops a same-weight escape hatch, lets the
//! default widen trust, or hides a managed attach stays rejected before a beta
//! warm-start row hardens.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/workspace/m3/warm_start_and_live_resume";

/// Root manifest document for the warm-start and live-resume corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusManifest {
    /// Stable corpus identifier.
    pub corpus_id: String,
    /// Manifest schema version.
    pub schema_version: u32,
    /// Reviewer-facing description.
    pub description: String,
    /// Positive drill specs.
    pub positive_drills: Vec<PositiveDrillSpec>,
    /// Negative drill specs.
    pub negative_drills: Vec<NegativeDrillSpec>,
}

/// Single positive drill spec: the fixture card MUST be contract-valid and
/// satisfy every expectation listed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Marketed beta warm-start row this drill stands in for.
    pub row_label: String,
    /// Plain-language scenario the row represents.
    pub scenario: String,
    /// Pinned expectations the card must reproduce.
    pub expect: PositiveExpect,
}

/// Pinned expectations for one positive drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveExpect {
    /// Expected card source class token.
    pub source_class: String,
    /// Expected card support class token.
    pub support_class: String,
    /// Expected runtime / host model token.
    pub runtime_or_host_model: String,
    /// Expected local-first claim.
    pub local_first: bool,
    /// Expected safest-next-action path token (must be local-safe).
    pub safest_next_action: String,
    /// Expected environment-starter setup-location token.
    pub setup_location_class: String,
    /// Expected honesty marker.
    pub honesty_marker_present: bool,
    /// Path tokens the card MUST offer as lanes.
    pub present_lanes: Vec<String>,
    /// Whether the card carries snapshot facts.
    pub snapshot_present: bool,
    /// Expected snapshot freshness token (snapshot drills only).
    #[serde(default)]
    pub snapshot_freshness: Option<String>,
    /// Expected snapshot age token (snapshot drills only).
    #[serde(default)]
    pub snapshot_age_class: Option<String>,
    /// Whether the snapshot must carry an invalidation reason.
    #[serde(default)]
    pub snapshot_invalidation_reason_present: Option<bool>,
    /// Per-lane availability the card must reproduce.
    #[serde(default)]
    pub lane_availability: Vec<LaneAvailabilityExpect>,
    /// Marker that must be absent from the serialized card, proving redaction.
    #[serde(default)]
    pub redacted_marker_absent: Option<String>,
}

/// A pinned availability for one named lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneAvailabilityExpect {
    /// Lane path token.
    pub path: String,
    /// Expected availability token for that lane.
    pub availability: String,
}

/// Single negative drill spec: the recorded tamper applied to the contract-valid
/// base card MUST raise a finding whose message contains
/// `expected_failure_substring`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegativeDrillSpec {
    /// Stable drill id.
    pub drill_id: String,
    /// Base-card fixture path relative to the corpus directory.
    pub fixture: String,
    /// Marketed beta warm-start row this drill protects.
    pub row_label: String,
    /// Tamper applied to the card before re-checking the contract.
    pub tamper: Tamper,
    /// Substring that must appear in the resulting contract finding.
    pub expected_failure_substring: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}

/// Typed tamper applied to a contract-valid [`WarmStartChoiceCard`] so the corpus
/// can prove the warm-start contract rejects each unsafe regression without
/// hand-writing an entire malformed card.
///
/// [`WarmStartChoiceCard`]: aureline_shell::start_center::warm_start_choice::WarmStartChoiceCard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tamper {
    /// Make a stale snapshot's live-resume lane immediately takeable.
    StaleSnapshotResumeTakeable,
    /// Drop the invalidation reason from a stale snapshot.
    StaleSnapshotMissingReason,
    /// Make a networked lane advertise a local-safe side-effect class.
    RemoteLaneMasqueradesAsLocal,
    /// Give the open-minimal escape hatch a network side effect.
    EscapeHatchHasSideEffect,
    /// Point the safest next action at a non-local-safe lane.
    SafestActionNotLocalSafe,
    /// Flip the default action to widen trust.
    DefaultWidensTrust,
    /// Drop the same-weight flag from the set-up-later lane on a local-first card.
    LocalFirstEscapeHatchNotSameWeight,
    /// Remove the bypass routes from a starter that runs setup.
    EnvironmentStarterMissingBypass,
    /// Remove the defer routes from a starter that runs setup.
    EnvironmentStarterMissingDefer,
    /// Hide a managed/remote attach from the side-effect summary.
    ManagedAttachUndisclosed,
    /// Drift the source-class token away from the source class.
    SourceClassTokenDrift,
    /// Clear the honesty marker on a card with a stale snapshot.
    HonestyMarkerInconsistent,
}

impl Tamper {
    /// Returns the stable snake_case token for this tamper.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleSnapshotResumeTakeable => "stale_snapshot_resume_takeable",
            Self::StaleSnapshotMissingReason => "stale_snapshot_missing_reason",
            Self::RemoteLaneMasqueradesAsLocal => "remote_lane_masquerades_as_local",
            Self::EscapeHatchHasSideEffect => "escape_hatch_has_side_effect",
            Self::SafestActionNotLocalSafe => "safest_action_not_local_safe",
            Self::DefaultWidensTrust => "default_widens_trust",
            Self::LocalFirstEscapeHatchNotSameWeight => "local_first_escape_hatch_not_same_weight",
            Self::EnvironmentStarterMissingBypass => "environment_starter_missing_bypass",
            Self::EnvironmentStarterMissingDefer => "environment_starter_missing_defer",
            Self::ManagedAttachUndisclosed => "managed_attach_undisclosed",
            Self::SourceClassTokenDrift => "source_class_token_drift",
            Self::HonestyMarkerInconsistent => "honesty_marker_inconsistent",
        }
    }
}
