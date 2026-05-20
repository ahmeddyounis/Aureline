//! Corpus manifest types for the repository-acquisition and bootstrap
//! truth drill suite.
//!
//! The manifest is the single source of truth for the corpus. Each
//! positive drill names the fixture that binds one source locator, one
//! checkout plan, and zero or more bootstrap-queue items, and pins the
//! `RepositoryAcquisitionBetaProjection` truth that fixture must produce.
//! Each negative drill names a fixture whose projection MUST FAIL with an
//! error whose message contains `expected_failure_substring`.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/workspace/m3/bootstrap_truth_corpus";

/// Root manifest document for the bootstrap-truth drill corpus.
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

/// Detailed per-guardrail expectation. When present on a positive drill,
/// the runner asserts every one of the six guardrail predicates. Most
/// drills omit it (the runner then only asserts `expected_guardrails_all_hold`),
/// but the "silent setup is caught" drill pins the individual predicate
/// that must flip to `false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardrailExpectations {
    pub clone_not_confused_with_open: bool,
    pub no_implicit_repo_code_execution: bool,
    pub bootstrap_items_attributed: bool,
    pub browse_safe_inspection_available: bool,
    pub mirror_not_masquerading_as_live: bool,
    pub no_hidden_trust_elevation: bool,
}

/// Single positive drill spec: the fixture MUST parse, project, and
/// satisfy every projection expectation listed here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Reviewer-facing acquisition class (mirror / air-gap / interrupted /
    /// shallow / partial / sparse / submodule / lfs / silent-setup ...).
    pub acquisition_class: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,

    /// Expected `AcquisitionSurface` token. The fixture's declared surface
    /// MUST match this exactly so a drill cannot silently change which
    /// client it stands for.
    pub expected_surface: String,
    /// Expected `AcquisitionVerb` token. Open / clone / import / open-archive
    /// / resume stay distinct verbs.
    pub expected_acquisition_verb: String,
    /// Expected `LocatorClass` token.
    pub expected_locator_class: String,
    /// Expected `TransportClass` token, when one is resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_transport_class: Option<String>,

    /// Expected `CheckoutModeClass` token.
    pub expected_checkout_mode: String,
    /// Expected `partial_or_sparse` flag on the checkout shape.
    pub expected_partial_or_sparse: bool,
    /// Expected `SubmodulePolicyClass` token.
    pub expected_submodule_policy: String,
    /// Expected `LfsPolicyClass` token.
    pub expected_lfs_policy: String,
    /// Expected `ExpectedCostBand` token.
    pub expected_cost_band: String,

    /// Expected `CredentialPostureClass` token.
    pub expected_credential_posture: String,
    /// Expected `reauth_required` flag on the credential posture.
    pub expected_credential_reauth_required: bool,

    /// Expected presence of an interrupted-recovery card.
    pub expected_interrupted: bool,
    /// Expected ordered list of interrupted-recovery branches. Order is
    /// significant so Resume / Discard / Open-read-only stay distinguishable.
    #[serde(default)]
    pub expected_interrupted_branches: Vec<String>,
    /// Expected `DiscardPosture` token on the interrupted-recovery card,
    /// when the drill is interrupted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_discard_posture: Option<String>,
    /// Expected `open_read_only_available` flag on the interrupted-recovery
    /// card, when the drill is interrupted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_open_read_only_available: Option<bool>,

    /// Expected count of follow-up bootstrap items that remain manual.
    pub expected_manual_followup_count: u64,
    /// Expected ordered list of `AcquisitionHonestyLabel` tokens. Order is
    /// significant.
    #[serde(default)]
    pub expected_honesty_labels: Vec<String>,

    /// Expected `guardrails.all_hold()`.
    pub expected_guardrails_all_hold: bool,
    /// Optional detailed per-guardrail expectation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_guardrails: Option<GuardrailExpectations>,

    /// Expected `surface_must_disclose_acquisition()`.
    pub expected_surface_must_disclose: bool,
    /// Expected `evidence_packet.every_item_attributed`.
    pub expected_every_item_attributed: bool,
}

/// Single negative drill spec: the fixture MUST FAIL projection with an
/// error whose message contains `expected_failure_substring`.
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
