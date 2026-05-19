//! Corpus manifest types for the history-rewrite drill suite.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/git/m3/history_rewrite_corpus";

/// Root manifest document for the history-rewrite drill corpus.
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

/// Single positive drill spec: the fixture must parse, validate,
/// project, and match every projection expectation listed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Reviewer-facing category (e.g. `cherry_pick_conflict`).
    pub category: String,
    /// Sub-axes the drill exercises (e.g. `["cherry_pick", "skip_semantics"]`).
    #[serde(default)]
    pub covers: Vec<String>,
    /// Expected record kind on the projection (matches
    /// `*_RECORD_KIND` tokens from `aureline-git`).
    pub expected_record_kind: String,
    /// Expected operation kind on the projection.
    pub expected_operation_kind: String,
    /// Expected lifecycle state.
    pub expected_lifecycle: String,
    /// Expected `destructive_gate_satisfied` flag.
    pub expected_destructive_gate_satisfied: bool,
    /// Expected recovery posture class on the projection.
    pub expected_recovery_posture_class: String,
    /// Expected next-safe-path classes on the projection. Order is not
    /// significant; the runner asserts set equality.
    #[serde(default)]
    pub expected_next_safe_path_classes: Vec<String>,
    /// Expected blocks-summary prefixes (e.g. `policy_admin_lock:`)
    /// asserted as `starts_with` against the projection's
    /// `blocks_summary`. Order is not significant.
    #[serde(default)]
    pub expected_blocks_summary_starts: Vec<String>,
    /// Expected audit-event ids on the projection. The runner asserts
    /// the projection's `audit_event_ids` is a superset of this list.
    #[serde(default)]
    pub expected_audit_event_ids: Vec<String>,
}

/// Single negative drill spec: the fixture MUST FAIL validation with
/// an error whose message contains `expected_failure_substring`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegativeDrillSpec {
    /// Stable drill id.
    pub drill_id: String,
    /// Fixture path relative to the corpus directory.
    pub fixture: String,
    /// Substring that must appear in the validation failure message.
    pub expected_failure_substring: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}
