//! Corpus manifest types for the workflow-bundle lifecycle drill suite.
//!
//! The manifest is the single source of truth for the corpus. Each positive
//! drill names a fixture and pins the workflow-bundle truth it must reproduce —
//! the bundle/source/status classes, the effective badge after evidence,
//! dependency, and mirror checks, the support claim it is allowed to imply, the
//! mirror/offline posture, the granular drift / removal / override counts, the
//! review and drift-resolution actions, whether removal preserves user-owned
//! assets, whether the rollback checkpoint restores bundle-owned state, and the
//! capability-dependency and lifecycle-sensitive markers it must propagate. Each
//! negative drill names a fixture whose validation MUST FAIL with an error whose
//! message contains `expected_failure_substring`, so a bundle that over-claims
//! stale evidence, endangers a user-owned asset, widens trust, hides the diff,
//! or leaks raw secrets stays rejected before a beta bundle row hardens.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/workspace/m3/workflow_bundle_lifecycle";

/// Drill kind discriminator.
pub mod drill_kind {
    /// A standalone workflow-bundle review packet fixture.
    pub const WORKFLOW_BUNDLE_REVIEW: &str = "workflow_bundle_review";
}

/// Root manifest document for the workflow-bundle lifecycle corpus.
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

/// Single positive drill spec: the fixture MUST parse, validate, project, and
/// satisfy every expectation listed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Drill kind (`workflow_bundle_review`).
    pub kind: String,
    /// Reviewer-facing source class.
    pub source_class: String,
    /// Reviewer-facing bundle (product row) class.
    pub bundle_class: String,
    /// Lifecycle flows the drill exercises.
    #[serde(default)]
    pub lifecycle_flows: Vec<String>,

    /// Expected product row class.
    pub expected_bundle_class: String,
    /// Expected source class.
    pub expected_source_class: String,
    /// Expected operational status class.
    pub expected_status_class: String,
    /// Expected support class.
    pub expected_support_class: String,
    /// Expected effective badge after evidence / dependency / mirror checks.
    pub expected_effective_badge_class: String,
    /// Expected support claim the badge may imply.
    pub expected_support_claim_class: String,
    /// Expected evidence freshness class.
    pub expected_evidence_freshness_class: String,
    /// Expected certification state class.
    pub expected_certification_state_class: String,
    /// Expected retest-required flag.
    pub expected_retest_required: bool,
    /// Expected mirror/offline packaging posture.
    pub expected_mirror_posture_class: String,

    /// Whether all required install/update diff axes must be present.
    pub expected_required_diff_axes_complete: bool,
    /// Whether the bundle guardrails must pass.
    pub expected_guardrails_pass: bool,
    /// Whether any raw export path is allowed (must stay false).
    pub expected_raw_export_allowed: bool,

    /// Expected number of granular drift rows.
    pub expected_drift_entry_count: usize,
    /// Expected number of removable assets in the remove review.
    pub expected_removable_asset_count: usize,
    /// Expected number of retained local overrides.
    pub expected_retained_override_count: usize,

    /// Review action ids that must be present in the projection.
    #[serde(default)]
    pub expected_review_actions_present: Vec<String>,
    /// Drift resolve action ids that must be present in the projection.
    #[serde(default)]
    pub expected_resolve_actions_present: Vec<String>,

    /// Whether removal must preserve user-owned assets.
    pub expected_preserves_user_owned_assets: bool,
    /// Whether the rollback checkpoint must restore bundle-owned state.
    pub expected_rollback_restores_bundle_owned: bool,

    /// Capability-dependency markers that must propagate across surfaces.
    #[serde(default)]
    pub expected_capability_dependency_markers: Vec<String>,
    /// Lifecycle-sensitive dependencies that must propagate across surfaces.
    #[serde(default)]
    pub expected_lifecycle_sensitive_dependencies: Vec<String>,
}

/// Single negative drill spec: the fixture MUST FAIL validation with an error
/// whose message contains `expected_failure_substring`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegativeDrillSpec {
    /// Stable drill id.
    pub drill_id: String,
    /// Fixture path relative to the corpus directory.
    pub fixture: String,
    /// Drill kind (`workflow_bundle_review`).
    pub kind: String,
    /// Substring that must appear in the validation failure message.
    pub expected_failure_substring: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}
