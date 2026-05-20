//! Corpus manifest types for the portable-state and restore-provenance drill
//! suite.
//!
//! The manifest is the single source of truth for the corpus. Each positive
//! drill names a fixture and pins the restore-provenance truth it must
//! reproduce — source event, schema outcome, resulting fidelity, the
//! controlled downgrade label, the missing-surface dependencies that reopen as
//! placeholders, whether high-risk classes stay named exclusions, and whether
//! the prior artifact stays available for compare/export. Migration drills add
//! the package-level expectations the alpha->beta projection must keep
//! (separated layers, machine-local exclusion, path/host redaction). Each
//! negative drill names a fixture whose validation MUST FAIL with an error
//! whose message contains `expected_failure_substring`, so a restore that
//! widens meaning, hides the prior artifact, strands a placeholder, or reuses a
//! placeholder id stays rejected before a beta continuity row hardens.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/workspace/m3/portable_state_and_restore_conformance";

/// Drill kind discriminator.
///
/// `restore_provenance_card` drills carry a [`WorkspaceRestoreProvenanceCard`]
/// directly; `alpha_migration` drills carry an older
/// [`PortableStateAlphaPackage`] that is migrated forward through
/// `WorkspacePortableStatePackage::from_alpha_package` before the projection is
/// checked.
///
/// [`WorkspaceRestoreProvenanceCard`]: aureline_workspace::WorkspaceRestoreProvenanceCard
/// [`PortableStateAlphaPackage`]: aureline_workspace::PortableStateAlphaPackage
pub mod drill_kind {
    /// A standalone restore-provenance card fixture.
    pub const RESTORE_PROVENANCE_CARD: &str = "restore_provenance_card";
    /// An alpha portable-state package migrated to the beta boundary.
    pub const ALPHA_MIGRATION: &str = "alpha_migration";
}

/// Root manifest document for the portable-state / restore-provenance corpus.
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

/// Single positive drill spec: the fixture MUST parse, validate, and satisfy
/// every projection expectation listed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Drill kind (`restore_provenance_card` or `alpha_migration`).
    pub kind: String,
    /// Reviewer-facing restore class label.
    pub restore_class: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,

    /// Expected `RestoreSourceEvent` token.
    pub expected_source_event: String,
    /// Expected `WorkspaceSchemaOutcome` token.
    pub expected_schema_outcome: String,
    /// Expected `WorkspaceRestoreFidelity` token.
    pub expected_resulting_fidelity: String,
    /// Expected controlled downgrade display label, asserted against
    /// `WorkspaceRestoreFidelity::display_label` so docs / help / claim
    /// language quote the same label the runtime renders.
    pub expected_downgrade_label: String,

    /// Expected `MissingSurfaceDependency` tokens, compared as a sorted set
    /// against the card's missing-surface placeholders.
    #[serde(default)]
    pub expected_missing_surface_dependencies: Vec<String>,

    /// Whether the card must name secrets, delegated approvals, live authority,
    /// and machine-unique trust anchors as intentional exclusions.
    pub expected_named_exclusions: bool,
    /// Whether the prior artifact must stay available through compare/export
    /// refs (always required when the schema outcome is manual review).
    pub expected_requires_compare_export: bool,

    /// Migration only: expected path-redaction availability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_path_redaction_available: Option<bool>,
    /// Migration only: expected host-redaction availability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_host_redaction_available: Option<bool>,
    /// Migration only: whether machine-local hints stay excluded from export.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_machine_local_excluded: Option<bool>,
    /// Migration only: whether the four required state layers stay separated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_required_layers_present: Option<bool>,
}

/// Single negative drill spec: the fixture MUST FAIL validation with an error
/// whose message contains `expected_failure_substring`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegativeDrillSpec {
    /// Stable drill id.
    pub drill_id: String,
    /// Fixture path relative to the corpus directory.
    pub fixture: String,
    /// Drill kind (`restore_provenance_card`).
    pub kind: String,
    /// Substring that must appear in the validation failure message.
    pub expected_failure_substring: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}
