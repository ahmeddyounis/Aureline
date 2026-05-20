//! Corpus manifest types for the command-truth and palette-authority drill suite.
//!
//! The manifest is the single source of truth for the corpus. Each positive
//! drill names a scenario fixture and pins the command-authority truth it must
//! reproduce — the canonical command id, lifecycle state, preview/approval
//! posture, the invocation surfaces that must stay in parity, the automation
//! labels that must remain honest, the agreed enablement decision, and whether
//! the invocation lineage reconstructs end to end. Each negative drill names a
//! scenario fixture whose validation MUST FAIL with an error whose message
//! contains `expected_failure_substring`, so a surface that widens authority,
//! suppresses preview/approval, lies about its automation labels, breaks alias
//! canonicalization, or drops a lineage join stays rejected before a beta command
//! row hardens.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/commands/m3/command_truth_and_authority";

/// Drill kind discriminator.
pub mod drill_kind {
    /// A standalone command-authority scenario fixture.
    pub const COMMAND_AUTHORITY_SCENARIO: &str = "command_authority_scenario";
}

/// Root manifest document for the command-truth and palette-authority corpus.
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
    /// Drill kind (`command_authority_scenario`).
    pub kind: String,

    /// Expected canonical command id.
    pub expected_command_id: String,
    /// Expected descriptor lifecycle state.
    pub expected_lifecycle_state: String,
    /// Expected declared preview class.
    pub expected_preview_class: String,
    /// Expected declared approval posture class.
    pub expected_approval_posture_class: String,
    /// Expected agreed enablement decision class across surfaces.
    pub expected_enablement_decision_class: String,

    /// Surface classes that must be covered by the scenario.
    #[serde(default)]
    pub expected_surface_classes: Vec<String>,
    /// Automation labels that must be present and honest.
    #[serde(default)]
    pub expected_automation_labels: Vec<String>,

    /// Whether the lineage chain must reconstruct end to end.
    pub expected_lineage_complete: bool,
    /// Whether the command's effect class requires a reversible rollback handle.
    pub expected_rollback_required: bool,
}

/// Single negative drill spec: the fixture MUST FAIL validation with an error
/// whose message contains `expected_failure_substring`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegativeDrillSpec {
    /// Stable drill id.
    pub drill_id: String,
    /// Fixture path relative to the corpus directory.
    pub fixture: String,
    /// Drill kind (`command_authority_scenario`).
    pub kind: String,
    /// Substring that must appear in the validation failure message.
    pub expected_failure_substring: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}
