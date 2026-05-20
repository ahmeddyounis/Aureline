//! Corpus manifest types for the scaffold and generated-project safety
//! drill suite.
//!
//! The manifest is the single source of truth for the corpus. Each positive
//! drill names the fixture that binds one signed template / generator
//! descriptor, one scaffold plan, and zero-or-one scaffold run, and pins the
//! `ScaffoldSafetyBetaProjection` truth that fixture must produce — provider
//! / signature / generation identity, declared side effects, create-empty /
//! set-up-later / rollback handoffs, honesty labels, guardrails, and the
//! disclosure verdict. Each negative drill names a fixture whose projection
//! MUST FAIL with an error whose message contains
//! `expected_failure_substring`, so undeclared-hook execution, sibling
//! descriptor / plan binding, and smuggled "declared" tasks stay rejected
//! before any beta creation row hardens.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/workspace/m3/scaffold_safety_corpus";

/// Root manifest document for the scaffold-safety drill corpus.
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

/// Detailed per-guardrail expectation. When present on a positive drill, the
/// runner asserts every one of the seven guardrail predicates that
/// [`aureline_workspace::ScaffoldSafetyGuardrails`] reports. Most drills omit
/// it (the runner then only asserts `expected_guardrails_all_hold`), but the
/// "caught" drills pin the individual predicate that must flip to `false` so
/// a regression that stops catching writes-before-review, hidden side
/// effects, or a hidden project database is detected here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardrailExpectations {
    pub no_writes_before_review: bool,
    pub side_effects_declared_before_execution: bool,
    pub side_effects_attributable_after_rollback: bool,
    pub no_undeclared_hooks_or_bootstrap: bool,
    pub generated_output_is_plain_workspace_content: bool,
    pub rollback_boundary_visible: bool,
    pub ai_extension_uses_governed_surface: bool,
}

/// Single positive drill spec: the fixture MUST parse, project, and satisfy
/// every projection expectation listed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Reviewer-facing scaffold class (first-party / extension / AI / import
    /// / mirror / policy-blocked / failure / caught ...).
    pub scaffold_class: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,

    /// Expected `ScaffoldSurface` token. The fixture's declared surface MUST
    /// match this exactly so a drill cannot silently change which client it
    /// stands for.
    pub expected_surface: String,
    /// Expected `TemplateProviderClass` token.
    pub expected_provider_class: String,
    /// Expected `DescriptorSignatureState` token.
    pub expected_signature_state: String,
    /// Expected `GenerationKindClass` token.
    pub expected_generation_kind: String,
    /// Expected `GenerationVerb` token. Create-project / generate-into-existing
    /// / update-regenerate stay distinct verbs.
    pub expected_generation_verb: String,
    /// Expected `EgressPostureClass` token.
    pub expected_egress_posture: String,
    /// Expected `TrustExpectationClass` token.
    pub expected_trust_expectation: String,
    /// Expected `SourceDistributionClass` token, asserted against the fixture
    /// descriptor's provenance block so mirrored / offline / imported rows
    /// keep their distribution truth rather than flattening into local files.
    pub expected_source_distribution_class: String,

    /// Expected declared side-effect classes (hook / network / registry /
    /// remote-image / dependency). Compared as a sorted set.
    #[serde(default)]
    pub expected_declared_side_effect_classes: Vec<String>,

    /// Expected `setup_handoff.create_empty_available`.
    pub expected_create_empty_available: bool,
    /// Expected `setup_handoff.set_up_later_available`.
    pub expected_set_up_later_available: bool,
    /// Expected `RollbackBoundaryClass` token on the setup handoff.
    pub expected_rollback_boundary: String,
    /// Expected `setup_handoff.rollback_automatic`.
    pub expected_rollback_automatic: bool,

    /// Expected presence of a run summary (the scaffold run has executed).
    pub expected_has_run: bool,
    /// Expected `ScaffoldOutcomeClass` token on the run, when one is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_run_outcome: Option<String>,

    /// Expected `ScaffoldHonestyLabel` tokens. Compared as a sorted set.
    #[serde(default)]
    pub expected_honesty_labels: Vec<String>,

    /// Expected `guardrails.all_hold()`.
    pub expected_guardrails_all_hold: bool,
    /// Optional detailed per-guardrail expectation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_guardrails: Option<GuardrailExpectations>,

    /// Expected `surface_must_disclose_generation()`.
    pub expected_surface_must_disclose: bool,
}

/// Single negative drill spec: the fixture MUST FAIL projection with an error
/// whose message contains `expected_failure_substring`.
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
