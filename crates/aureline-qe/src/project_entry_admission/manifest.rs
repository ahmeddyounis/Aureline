//! Corpus manifest types for the project-entry and workspace-admission drill
//! suite.
//!
//! The manifest is the single source of truth for the corpus. Each positive
//! drill names a fixture that carries one [`ProjectEntryReviewRequest`] and pins
//! the entry truth the built review record must reproduce — the verb-specific
//! review sheet, the source-labelled access class, the first-useful entry source
//! and landing surface, the resulting mode, the primary next action, the
//! destination-collision posture, the Blocking now / Recommended soon / Optional
//! later readiness grouping, the work the entry deliberately defers, and (for
//! imports) the inspect/write posture. Each negative drill names a fixture and a
//! typed tamper that, applied to the built record, MUST raise a contract finding
//! whose message contains `expected_failure_substring`, so an entry path that
//! widens trust, leaks credentials, writes before review, drops the collision
//! choice, drifts a cross-surface parity row, loses failed-attempt inputs, or
//! lets detection auto-trust / auto-install stays rejected before a beta entry
//! row hardens.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/workspace/m3/project_entry_and_admission";

/// Root manifest document for the project-entry and admission corpus.
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

/// Single positive drill spec: the fixture request MUST build a contract-valid
/// review record that satisfies every expectation listed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Marketed beta switching row this drill stands in for.
    pub row_label: String,
    /// Pinned expectations the built record must reproduce.
    pub expect: PositiveExpect,
}

/// Pinned expectations for one positive drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveExpect {
    /// Expected verb-specific review sheet kind.
    pub review_sheet_kind: String,
    /// Expected source-labelled access class.
    pub source_access_class: String,
    /// Expected first-useful entry source on the checkpoint route.
    pub first_useful_entry_source: String,
    /// Expected first landing surface.
    pub landing_surface: String,
    /// Expected resulting mode.
    pub resulting_mode: String,
    /// Expected primary next action on the post-entry handoff card.
    pub primary_next_action: String,
    /// Expected destination-collision class (absent means no collision review).
    #[serde(default)]
    pub collision_class: Option<String>,
    /// Expected explicit-choice requirement when a collision review is present.
    #[serde(default)]
    pub collision_requires_explicit_choice: Option<bool>,
    /// Expected count of Blocking now readiness tasks.
    pub blocking_now_count: usize,
    /// Expected count of Recommended soon readiness tasks.
    pub recommended_soon_count: usize,
    /// Expected count of Optional later readiness tasks.
    pub optional_later_count: usize,
    /// Deferred-work classes that must appear on the handoff card.
    #[serde(default)]
    pub deferred_work_present: Vec<String>,
    /// Expected inspect-only posture (imports only).
    #[serde(default)]
    pub import_inspect_only: Option<bool>,
    /// Expected import write-behavior class (imports only).
    #[serde(default)]
    pub import_write_behavior_class: Option<String>,
    /// Marker that must be absent from the serialized record, proving redaction.
    #[serde(default)]
    pub redacted_marker_absent: Option<String>,
}

/// Single negative drill spec: the recorded tamper applied to the built record
/// MUST raise a contract finding whose message contains
/// `expected_failure_substring`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegativeDrillSpec {
    /// Stable drill id.
    pub drill_id: String,
    /// Fixture path relative to the corpus directory.
    pub fixture: String,
    /// Marketed beta switching row this drill protects.
    pub row_label: String,
    /// Tamper applied to the built record before re-checking the contract.
    pub tamper: Tamper,
    /// Substring that must appear in the resulting contract finding.
    pub expected_failure_substring: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}

/// Typed tamper applied to a built [`ProjectEntryReviewRecord`] so the corpus can
/// prove the entry contract rejects each unsafe regression without hand-writing
/// an entire malformed record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tamper {
    /// Make a clone review claim it grants trust on materialization.
    CloneGrantsTrust,
    /// Expose credentials inside the normalized clone remote label.
    CloneExposesCredentials,
    /// Let an import write durably before its review admits it.
    ImportWritesBeforeReview,
    /// Let an inspect-only import advertise a write behaviour.
    ImportInspectAdvertisesWrite,
    /// Drop the explicit-choice requirement from a destination collision.
    CollisionSkipsExplicitChoice,
    /// Drift a cross-surface parity row off the reviewed verb/mode.
    SurfaceParityDrift,
    /// Drop the failed-attempt input-preservation guarantee.
    FailureRepairDropsInputs,
    /// Let the route auto-trust the workspace.
    RouteAutoTrust,
    /// Let the route auto-install setup.
    RouteAutoInstall,
    /// Mismatch the review sheet kind against the entry verb.
    ReviewSheetMismatch,
}

impl Tamper {
    /// Returns the stable snake_case token for this tamper.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CloneGrantsTrust => "clone_grants_trust",
            Self::CloneExposesCredentials => "clone_exposes_credentials",
            Self::ImportWritesBeforeReview => "import_writes_before_review",
            Self::ImportInspectAdvertisesWrite => "import_inspect_advertises_write",
            Self::CollisionSkipsExplicitChoice => "collision_skips_explicit_choice",
            Self::SurfaceParityDrift => "surface_parity_drift",
            Self::FailureRepairDropsInputs => "failure_repair_drops_inputs",
            Self::RouteAutoTrust => "route_auto_trust",
            Self::RouteAutoInstall => "route_auto_install",
            Self::ReviewSheetMismatch => "review_sheet_mismatch",
        }
    }
}
