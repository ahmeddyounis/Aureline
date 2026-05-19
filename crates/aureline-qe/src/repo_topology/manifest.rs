//! Corpus manifest types for the repo-topology drill suite.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/workspace/m3/repo_topology_corpus";

/// Root manifest document for the repo-topology drill corpus.
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

/// Single positive drill spec: the fixture MUST parse, project, and
/// satisfy every projection expectation listed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Reviewer-facing topology class (matches a
    /// `CompletenessStateClass` token where applicable).
    pub topology_class: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
    /// Expected surface token (`workspace`, `search`, `blame`, ...).
    pub expected_surface: String,
    /// Expected `repo_root_kind` token on the projection.
    pub expected_repo_root_kind: String,
    /// Expected `may_claim_full_coverage` on the projection.
    pub expected_may_claim_full_coverage: bool,
    /// Expected ordered list of `full_coverage_blockers`. Order is
    /// significant: the runner asserts the observed and expected
    /// lists are equal.
    #[serde(default)]
    pub expected_full_coverage_blockers: Vec<String>,
    /// Expected ordered list of `required_affordances`. Order is
    /// significant.
    #[serde(default)]
    pub expected_required_affordances: Vec<String>,
    /// Expected `mutation_target` token.
    pub expected_mutation_target: String,
    /// Expected `body_export_posture` token.
    pub expected_body_export_posture: String,
    /// Expected ordered list of `honesty_labels`. Order is
    /// significant.
    #[serde(default)]
    pub expected_honesty_labels: Vec<String>,
}

/// Single negative drill spec: the fixture MUST FAIL projection with
/// an error whose message contains `expected_failure_substring`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegativeDrillSpec {
    /// Stable drill id.
    pub drill_id: String,
    /// Fixture path relative to the corpus directory.
    pub fixture: String,
    /// Substring that must appear in the projection failure message.
    pub expected_failure_substring: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}
