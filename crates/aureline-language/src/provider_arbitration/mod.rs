//! Language-intelligence arbitration and provider-health inspector.
//!
//! This module owns the inspector contract for protected language-action
//! lanes (definition, references, rename, formatting, organize-imports, and
//! code-action). It binds the per-provider health-state row read by editor
//! chrome, quick-fix previews, diagnostics detail, command results,
//! CLI/headless inspect, and support export together with the per-lane
//! arbitration decision that records provider order, the chosen winner, the
//! confidence outcome, the disagreement disclosure, the downgraded-promise
//! copy, the fallback label, the apply gate, and the back-references into
//! the existing capability-negotiation, result-provenance, and router-decision
//! records.
//!
//! The module does not apply edits, dispatch requests, or replace the
//! existing router. It freezes the boundary shape every inspector consumer
//! reads so that provider preference cannot hide conflicts, stale state, or
//! scope warnings, crash-looping providers remain visible with retry and
//! isolate controls, and wide-scope rename or refactor work routes through
//! preview or side-branch review whenever semantic completeness is partial.

mod records;

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

pub use records::{
    ApplyGateClass, ArbitrationDecisionAggregateCounts, ArbitrationDecisionRecord,
    ArbitrationDecisionReportRow, ArbitrationDecisionSchemaVersion, ArbitrationInspectorBetaReport,
    CompletenessClass as ArbitrationCompletenessClass, ConfidenceOutcomeClass, ConflictClass,
    ConsumerRoutingRow, ConsumerSurfaceClass, DisagreementBlock, DisagreementVisibilityClass,
    DowngradedPromiseBlock, DowngradedPromiseReasonClass, EpochBinding as ArbitrationEpochBinding,
    EpochRoleClass as ArbitrationEpochRoleClass, FallbackLabelClass, FaultDomainClass,
    FreshnessClass as ArbitrationFreshnessClass, HealthState as ArbitrationHealthState,
    IsolateActionClass, LaneSupportClass, LaneSupportRow, LanguageActionLaneClass,
    LinkedRecordRefs, LocalityClass as ArbitrationLocalityClass,
    PolicyContext as ArbitrationPolicyContext, ProviderArbitrationCorpus,
    ProviderArbitrationCorpusEntry, ProviderFamily, ProviderHealthStateRecord,
    ProviderHealthStateSchemaVersion, ProviderOrderRow, ProviderRoleClass, RecoveryHintClass,
    RedactionClass as ArbitrationRedactionClass, RequestedAuthorityFloorClass, RetryActionClass,
    RetryIsolateControls, ScopeClaimClass as ArbitrationScopeClaimClass,
    TrustState as ArbitrationTrustState, ARBITRATION_DECISION_RECORD_KIND,
    ARBITRATION_DECISION_SCHEMA_VERSION, PROVIDER_HEALTH_STATE_RECORD_KIND,
    PROVIDER_HEALTH_STATE_SCHEMA_VERSION,
};

/// Repository-relative documentation ref for the inspector beta contract.
pub const PROVIDER_ARBITRATION_BETA_DOC_REF: &str = "docs/language/m3/provider_arbitration_beta.md";

/// Repository-relative schema ref for the provider-health-state row.
pub const PROVIDER_HEALTH_STATE_SCHEMA_REF: &str =
    "schemas/language/provider_health_state.schema.json";

/// Repository-relative schema ref for the arbitration-decision record.
pub const ARBITRATION_DECISION_SCHEMA_REF: &str =
    "schemas/language/arbitration_decision.schema.json";

/// Directory containing the checked-in arbitration inspector corpus.
pub const PROVIDER_ARBITRATION_CORPUS_DIR: &str = "fixtures/language/m3/provider_arbitration";

/// Directory containing the multi-provider arbitration proof corpus used to
/// qualify claimed beta language rows.
pub const PROVIDER_ARBITRATION_PROOF_CORPUS_DIR: &str =
    "fixtures/language/m3/provider_arbitration_corpus";

/// Repository-relative documentation ref for the beta claim qualification
/// contract.
pub const PROVIDER_ARBITRATION_CLAIM_QUALIFICATION_DOC_REF: &str =
    "docs/language/m3/provider_arbitration_claim_qualification.md";

/// Repository-relative artifact ref for the downgraded semantic-claims matrix.
pub const DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_ARTIFACT_REF: &str =
    "artifacts/language/m3/downgraded_semantic_claims_matrix.json";

/// Repository-relative artifact ref for the human-readable arbitration proof
/// report.
pub const PROVIDER_ARBITRATION_PROOF_REPORT_ARTIFACT_REF: &str =
    "artifacts/language/m3/provider_arbitration_report.md";

/// Stable record-kind tag for inspector corpus reports.
pub const PROVIDER_ARBITRATION_BETA_REPORT_RECORD_KIND: &str =
    "language_provider_arbitration_beta_report";

/// Stable record-kind tag for proof-corpus reports.
pub const PROVIDER_ARBITRATION_PROOF_REPORT_RECORD_KIND: &str =
    "language_provider_arbitration_proof_report";

/// Stable record-kind tag for downgraded-semantic-claims matrix rows.
pub const DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_ROW_RECORD_KIND: &str =
    "language_downgraded_semantic_claim_row";

/// Stable record-kind tag for downgraded-semantic-claims matrix envelopes.
pub const DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_RECORD_KIND: &str =
    "language_downgraded_semantic_claims_matrix";

const LSP_FRAMEWORK_DEFINITION_DISAGREEMENT_PATH: &str =
    "fixtures/language/m3/provider_arbitration/lsp_framework_definition_disagreement.yaml";
const GRAPH_PARTIAL_REFERENCES_SCOPE_NARROWED_PATH: &str =
    "fixtures/language/m3/provider_arbitration/graph_partial_references_scope_narrowed.yaml";
const NOTEBOOK_RENAME_PREVIEW_REQUIRED_PATH: &str =
    "fixtures/language/m3/provider_arbitration/notebook_rename_preview_required.yaml";
const FORMATTING_FALLBACK_TO_TEXT_PATH: &str =
    "fixtures/language/m3/provider_arbitration/formatting_fallback_to_text.yaml";
const ORGANIZE_IMPORTS_CRASH_LOOP_QUARANTINED_PATH: &str =
    "fixtures/language/m3/provider_arbitration/organize_imports_crash_loop_quarantined.yaml";
const CODE_ACTION_WIDE_SCOPE_SIDE_BRANCH_PATH: &str =
    "fixtures/language/m3/provider_arbitration/code_action_wide_scope_side_branch.yaml";

const PROOF_DEFINITION_AGREE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/definition_all_providers_agree_exact.yaml";
const PROOF_DEFINITION_DISAGREEMENT_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/definition_target_set_disagreement_preview.yaml";
const PROOF_DEFINITION_STALE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/definition_stale_cache_reuse_labeled.yaml";
const PROOF_REFERENCES_AGREE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/references_all_providers_agree_exact.yaml";
const PROOF_REFERENCES_SCOPE_DISAGREEMENT_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/references_scope_coverage_disagreement_partial.yaml";
const PROOF_REFERENCES_IMPORTED_SNAPSHOT_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/references_imported_snapshot_partial.yaml";
const PROOF_RENAME_AGREE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/rename_all_providers_agree_exact.yaml";
const PROOF_RENAME_WIDE_SCOPE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/rename_wide_scope_side_branch_required.yaml";
const PROOF_RENAME_TEXT_FALLBACK_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/rename_text_fallback_labeled_heuristic.yaml";
const PROOF_FORMATTING_AGREE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/formatting_all_providers_agree_exact.yaml";
const PROOF_FORMATTING_TEXT_FALLBACK_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/formatting_text_fallback_labeled_heuristic.yaml";
const PROOF_FORMATTING_STALE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/formatting_stale_cache_reuse_labeled.yaml";
const PROOF_ORGANIZE_IMPORTS_AGREE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/organize_imports_all_providers_agree_exact.yaml";
const PROOF_ORGANIZE_IMPORTS_CRASH_LOOP_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/organize_imports_language_server_crash_loop_blocked.yaml";
const PROOF_CODE_ACTION_AGREE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/code_action_all_providers_agree_exact.yaml";
const PROOF_CODE_ACTION_SIDE_BRANCH_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/code_action_edit_safety_disagreement_side_branch.yaml";
const PROOF_CODE_ACTION_PARTIAL_SCOPE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/code_action_partial_scope_preview_required.yaml";
const PROOF_CRASH_LOOP_FRAMEWORK_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/crash_loop_framework_pack_blocked.yaml";
const PROOF_CRASH_LOOP_NOTEBOOK_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/crash_loop_notebook_adapter_blocked.yaml";
const PROOF_PREFERENCE_PRESERVES_CONFLICT_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/provider_preference_reorder_preserves_conflict.yaml";
const PROOF_PREFERENCE_PRESERVES_STALE_PATH: &str =
    "fixtures/language/m3/provider_arbitration_corpus/provider_preference_reorder_preserves_stale_warning.yaml";

const CURRENT_PROVIDER_ARBITRATION_FIXTURES: &[(&str, &str)] = &[
    (
        LSP_FRAMEWORK_DEFINITION_DISAGREEMENT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration/lsp_framework_definition_disagreement.yaml"
        )),
    ),
    (
        GRAPH_PARTIAL_REFERENCES_SCOPE_NARROWED_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration/graph_partial_references_scope_narrowed.yaml"
        )),
    ),
    (
        NOTEBOOK_RENAME_PREVIEW_REQUIRED_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration/notebook_rename_preview_required.yaml"
        )),
    ),
    (
        FORMATTING_FALLBACK_TO_TEXT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration/formatting_fallback_to_text.yaml"
        )),
    ),
    (
        ORGANIZE_IMPORTS_CRASH_LOOP_QUARANTINED_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration/organize_imports_crash_loop_quarantined.yaml"
        )),
    ),
    (
        CODE_ACTION_WIDE_SCOPE_SIDE_BRANCH_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration/code_action_wide_scope_side_branch.yaml"
        )),
    ),
];

const CURRENT_PROVIDER_ARBITRATION_PROOF_FIXTURES: &[(&str, &str)] = &[
    (
        PROOF_DEFINITION_AGREE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/definition_all_providers_agree_exact.yaml"
        )),
    ),
    (
        PROOF_DEFINITION_DISAGREEMENT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/definition_target_set_disagreement_preview.yaml"
        )),
    ),
    (
        PROOF_DEFINITION_STALE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/definition_stale_cache_reuse_labeled.yaml"
        )),
    ),
    (
        PROOF_REFERENCES_AGREE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/references_all_providers_agree_exact.yaml"
        )),
    ),
    (
        PROOF_REFERENCES_SCOPE_DISAGREEMENT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/references_scope_coverage_disagreement_partial.yaml"
        )),
    ),
    (
        PROOF_REFERENCES_IMPORTED_SNAPSHOT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/references_imported_snapshot_partial.yaml"
        )),
    ),
    (
        PROOF_RENAME_AGREE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/rename_all_providers_agree_exact.yaml"
        )),
    ),
    (
        PROOF_RENAME_WIDE_SCOPE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/rename_wide_scope_side_branch_required.yaml"
        )),
    ),
    (
        PROOF_RENAME_TEXT_FALLBACK_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/rename_text_fallback_labeled_heuristic.yaml"
        )),
    ),
    (
        PROOF_FORMATTING_AGREE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/formatting_all_providers_agree_exact.yaml"
        )),
    ),
    (
        PROOF_FORMATTING_TEXT_FALLBACK_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/formatting_text_fallback_labeled_heuristic.yaml"
        )),
    ),
    (
        PROOF_FORMATTING_STALE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/formatting_stale_cache_reuse_labeled.yaml"
        )),
    ),
    (
        PROOF_ORGANIZE_IMPORTS_AGREE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/organize_imports_all_providers_agree_exact.yaml"
        )),
    ),
    (
        PROOF_ORGANIZE_IMPORTS_CRASH_LOOP_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/organize_imports_language_server_crash_loop_blocked.yaml"
        )),
    ),
    (
        PROOF_CODE_ACTION_AGREE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/code_action_all_providers_agree_exact.yaml"
        )),
    ),
    (
        PROOF_CODE_ACTION_SIDE_BRANCH_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/code_action_edit_safety_disagreement_side_branch.yaml"
        )),
    ),
    (
        PROOF_CODE_ACTION_PARTIAL_SCOPE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/code_action_partial_scope_preview_required.yaml"
        )),
    ),
    (
        PROOF_CRASH_LOOP_FRAMEWORK_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/crash_loop_framework_pack_blocked.yaml"
        )),
    ),
    (
        PROOF_CRASH_LOOP_NOTEBOOK_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/crash_loop_notebook_adapter_blocked.yaml"
        )),
    ),
    (
        PROOF_PREFERENCE_PRESERVES_CONFLICT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/provider_preference_reorder_preserves_conflict.yaml"
        )),
    ),
    (
        PROOF_PREFERENCE_PRESERVES_STALE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m3/provider_arbitration_corpus/provider_preference_reorder_preserves_stale_warning.yaml"
        )),
    ),
];

/// Loads one inspector corpus entry from YAML.
pub fn load_provider_arbitration_case(
    yaml: &str,
) -> Result<ProviderArbitrationCorpusEntry, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the current checked-in inspector corpus.
pub fn current_provider_arbitration_corpus() -> Result<ProviderArbitrationCorpus, serde_yaml::Error>
{
    CURRENT_PROVIDER_ARBITRATION_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<ProviderArbitrationCorpusEntry>(yaml).map(|mut entry| {
                entry.fixture_ref = (*fixture_ref).to_owned();
                entry
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|entries| ProviderArbitrationCorpus { entries })
}

/// Returns fixture refs included in the checked-in inspector corpus.
pub fn current_provider_arbitration_fixture_refs() -> impl Iterator<Item = &'static str> {
    CURRENT_PROVIDER_ARBITRATION_FIXTURES
        .iter()
        .map(|(fixture_ref, _)| *fixture_ref)
}

/// Loads the multi-provider arbitration proof corpus.
pub fn current_provider_arbitration_proof_corpus(
) -> Result<ProviderArbitrationCorpus, serde_yaml::Error> {
    CURRENT_PROVIDER_ARBITRATION_PROOF_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<ProviderArbitrationCorpusEntry>(yaml).map(|mut entry| {
                entry.fixture_ref = (*fixture_ref).to_owned();
                entry
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|entries| ProviderArbitrationCorpus { entries })
}

/// Returns fixture refs included in the arbitration proof corpus.
pub fn current_provider_arbitration_proof_fixture_refs() -> impl Iterator<Item = &'static str> {
    CURRENT_PROVIDER_ARBITRATION_PROOF_FIXTURES
        .iter()
        .map(|(fixture_ref, _)| *fixture_ref)
}

/// Closed scenario vocabulary for the downgraded semantic-claims matrix.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ProofScenarioClass {
    /// Every admissible provider agreed on the answer.
    ProviderAgreement,
    /// Providers disagreed on the target set or scope.
    ProviderDisagreement,
    /// Negotiated scope is narrower than requested.
    PartialScope,
    /// An imported snapshot was the only admissible source.
    ImportedSnapshot,
    /// Result was served from a warm cache with explicit stale label.
    StaleCacheReuse,
    /// A provider was quarantined after a crash loop.
    ProviderCrashLoop,
    /// Wide-scope rename routed through preview or side-branch review.
    WideScopeRename,
    /// Result fell back to text/syntax fallback with explicit label.
    TextFallback,
    /// User preference re-ordered providers without hiding warnings.
    ProviderPreferenceReorder,
}

/// Closed claim-status vocabulary for the downgraded semantic-claims matrix.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ClaimStatusClass {
    /// Exact, complete, live lane that may stay marketed as a full claim.
    QualifiedForBetaClaim,
    /// Lane is downgraded but the downgrade is disclosed end-to-end.
    DowngradedDiscloseAndProceed,
    /// Lane is blocked; apply gate refuses to proceed.
    BlockedForRecovery,
}

/// One row in the downgraded semantic-claims matrix.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DowngradedSemanticClaimRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Repository-relative fixture ref.
    pub fixture_ref: String,
    /// Arbitration decision id.
    pub arbitration_decision_id: String,
    /// Lane covered by the decision.
    pub language_action_lane_class: LanguageActionLaneClass,
    /// Scenario class.
    pub proof_scenario_class: ProofScenarioClass,
    /// Confidence outcome class.
    pub confidence_outcome_class: ConfidenceOutcomeClass,
    /// Apply gate class.
    pub apply_gate_class: ApplyGateClass,
    /// Fallback label class.
    pub fallback_label_class: FallbackLabelClass,
    /// Downgraded-promise reason class.
    pub downgraded_promise_reason_class: DowngradedPromiseReasonClass,
    /// Conflict class.
    pub conflict_class: ConflictClass,
    /// Claim status class.
    pub claim_status_class: ClaimStatusClass,
}

/// Downgraded semantic-claims matrix envelope.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DowngradedSemanticClaimsMatrix {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Matrix id.
    pub matrix_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Documentation ref for the claim-qualification contract.
    pub claim_qualification_doc_ref: String,
    /// Source corpus directory.
    pub corpus_dir: String,
    /// Provider-health-state schema ref.
    pub provider_health_state_schema_ref: String,
    /// Arbitration-decision schema ref.
    pub arbitration_decision_schema_ref: String,
    /// True when raw provider payloads are excluded.
    pub raw_payload_excluded: bool,
    /// True when private source material is excluded.
    pub raw_private_material_excluded: bool,
    /// Per-row claim rows.
    pub rows: Vec<DowngradedSemanticClaimRow>,
}

/// Classifies a corpus entry into a scenario class.
pub fn classify_proof_scenario(entry: &ProviderArbitrationCorpusEntry) -> ProofScenarioClass {
    let decision = &entry.arbitration_decision;
    let any_imported_snapshot = entry
        .provider_health_states
        .iter()
        .any(|row| row.locality_class == ArbitrationLocalityClass::ImportedSnapshot);
    let any_crash_loop = entry
        .provider_health_states
        .iter()
        .any(|row| row.health_state == ArbitrationHealthState::CrashLoopQuarantined);
    let is_user_preference = decision
        .arbitration_decision_id
        .contains("preference_preserves");

    if any_crash_loop {
        return ProofScenarioClass::ProviderCrashLoop;
    }
    if is_user_preference {
        return ProofScenarioClass::ProviderPreferenceReorder;
    }
    if decision.fallback_label_class == FallbackLabelClass::TextFallback {
        return ProofScenarioClass::TextFallback;
    }
    if any_imported_snapshot {
        return ProofScenarioClass::ImportedSnapshot;
    }
    if decision.language_action_lane_class == LanguageActionLaneClass::Rename
        && matches!(
            decision.requested_scope_claim_class,
            ArbitrationScopeClaimClass::ActiveWorkset | ArbitrationScopeClaimClass::WholeWorkspace
        )
    {
        return ProofScenarioClass::WideScopeRename;
    }
    if decision.disagreement_block.conflict_class != ConflictClass::None {
        return ProofScenarioClass::ProviderDisagreement;
    }
    if matches!(
        decision.confidence_outcome_class,
        ConfidenceOutcomeClass::Stale
    ) || decision
        .downgraded_promise_block
        .downgraded_promise_reason_class
        == DowngradedPromiseReasonClass::StaleCacheReuse
    {
        return ProofScenarioClass::StaleCacheReuse;
    }
    if decision.negotiated_completeness_class
        == ArbitrationCompletenessClass::PartialForClaimedScope
    {
        return ProofScenarioClass::PartialScope;
    }
    ProofScenarioClass::ProviderAgreement
}

/// Classifies a corpus entry into a claim-status class.
pub fn classify_claim_status(entry: &ProviderArbitrationCorpusEntry) -> ClaimStatusClass {
    let decision = &entry.arbitration_decision;
    match decision.confidence_outcome_class {
        ConfidenceOutcomeClass::Exact => {
            if decision.disagreement_block.conflict_class == ConflictClass::None
                && decision.apply_gate_class == ApplyGateClass::ReadyToApply
            {
                ClaimStatusClass::QualifiedForBetaClaim
            } else {
                ClaimStatusClass::DowngradedDiscloseAndProceed
            }
        }
        ConfidenceOutcomeClass::Unavailable => ClaimStatusClass::BlockedForRecovery,
        _ => ClaimStatusClass::DowngradedDiscloseAndProceed,
    }
}

/// Builds the downgraded semantic-claims matrix for the given corpus.
pub fn build_downgraded_semantic_claims_matrix(
    corpus: &ProviderArbitrationCorpus,
    matrix_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> DowngradedSemanticClaimsMatrix {
    let rows = corpus
        .entries
        .iter()
        .map(|entry| DowngradedSemanticClaimRow {
            record_kind: DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_ROW_RECORD_KIND.to_owned(),
            fixture_ref: entry.fixture_ref.clone(),
            arbitration_decision_id: entry.arbitration_decision.arbitration_decision_id.clone(),
            language_action_lane_class: entry.arbitration_decision.language_action_lane_class,
            proof_scenario_class: classify_proof_scenario(entry),
            confidence_outcome_class: entry.arbitration_decision.confidence_outcome_class,
            apply_gate_class: entry.arbitration_decision.apply_gate_class,
            fallback_label_class: entry.arbitration_decision.fallback_label_class,
            downgraded_promise_reason_class: entry
                .arbitration_decision
                .downgraded_promise_block
                .downgraded_promise_reason_class,
            conflict_class: entry.arbitration_decision.disagreement_block.conflict_class,
            claim_status_class: classify_claim_status(entry),
        })
        .collect::<Vec<_>>();

    DowngradedSemanticClaimsMatrix {
        record_kind: DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_RECORD_KIND.to_owned(),
        matrix_id: matrix_id.into(),
        captured_at: captured_at.into(),
        claim_qualification_doc_ref: PROVIDER_ARBITRATION_CLAIM_QUALIFICATION_DOC_REF.to_owned(),
        corpus_dir: PROVIDER_ARBITRATION_PROOF_CORPUS_DIR.to_owned(),
        provider_health_state_schema_ref: PROVIDER_HEALTH_STATE_SCHEMA_REF.to_owned(),
        arbitration_decision_schema_ref: ARBITRATION_DECISION_SCHEMA_REF.to_owned(),
        raw_payload_excluded: true,
        raw_private_material_excluded: true,
        rows,
    }
}

/// Validation defect emitted by the inspector evaluator.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArbitrationCorpusValidationDefect {
    /// Fixture ref where the defect was found.
    pub fixture_ref: String,
    /// Arbitration decision id when available.
    pub arbitration_decision_id: Option<String>,
    /// Stable check id.
    pub check_id: String,
    /// Field associated with the defect.
    pub field_name: String,
    /// Export-safe defect summary.
    pub summary: String,
}

/// Report returned when the inspector evaluator validates a corpus.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArbitrationCorpusValidationReport {
    /// Validation defects.
    pub defects: Vec<ArbitrationCorpusValidationDefect>,
}

impl ArbitrationCorpusValidationReport {
    /// Returns true when no defects were found.
    pub fn is_empty(&self) -> bool {
        self.defects.is_empty()
    }
}

impl fmt::Display for ArbitrationCorpusValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.defects.is_empty() {
            return write!(f, "arbitration inspector corpus valid");
        }
        write!(
            f,
            "arbitration inspector corpus has {} validation defect(s)",
            self.defects.len()
        )
    }
}

impl Error for ArbitrationCorpusValidationReport {}

/// Evaluates the checked-in arbitration inspector corpus.
#[derive(Debug, Default, Clone, Copy)]
pub struct ArbitrationInspector;

impl ArbitrationInspector {
    /// Creates a new inspector evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a corpus and returns all defects.
    pub fn validate(
        &self,
        corpus: &ProviderArbitrationCorpus,
    ) -> ArbitrationCorpusValidationReport {
        let mut defects = Vec::new();
        let mut fixture_refs = BTreeSet::new();
        let mut decision_ids = BTreeSet::new();
        let mut health_ids = BTreeSet::new();
        let mut covered_lanes = BTreeSet::new();
        let mut covered_outcomes = BTreeSet::new();
        let mut covered_gates = BTreeSet::new();
        let mut covered_consumers = BTreeSet::new();

        for entry in &corpus.entries {
            self.validate_entry(
                entry,
                &mut defects,
                &mut fixture_refs,
                &mut decision_ids,
                &mut health_ids,
                &mut covered_lanes,
                &mut covered_outcomes,
                &mut covered_gates,
                &mut covered_consumers,
            );
        }

        for required_lane in [
            LanguageActionLaneClass::Definition,
            LanguageActionLaneClass::References,
            LanguageActionLaneClass::Rename,
            LanguageActionLaneClass::Formatting,
            LanguageActionLaneClass::OrganizeImports,
            LanguageActionLaneClass::CodeAction,
        ] {
            if !covered_lanes.contains(&required_lane) {
                defects.push(corpus_defect(
                    "corpus.lane_missing",
                    "entries",
                    format!("corpus must cover {required_lane:?}"),
                ));
            }
        }

        for required_outcome in [
            ConfidenceOutcomeClass::Exact,
            ConfidenceOutcomeClass::Heuristic,
            ConfidenceOutcomeClass::Partial,
            ConfidenceOutcomeClass::Stale,
            ConfidenceOutcomeClass::Unavailable,
        ] {
            if !covered_outcomes.contains(&required_outcome) {
                defects.push(corpus_defect(
                    "corpus.confidence_outcome_missing",
                    "decision.confidence_outcome_class",
                    format!("corpus must cover {required_outcome:?}"),
                ));
            }
        }

        for required_gate in [
            ApplyGateClass::PreviewRequired,
            ApplyGateClass::SideBranchRequired,
            ApplyGateClass::BlockedForHealth,
        ] {
            if !covered_gates.contains(&required_gate) {
                defects.push(corpus_defect(
                    "corpus.apply_gate_missing",
                    "decision.apply_gate_class",
                    format!("corpus must cover {required_gate:?}"),
                ));
            }
        }

        for required_consumer in [
            ConsumerSurfaceClass::EditorChrome,
            ConsumerSurfaceClass::QuickFixPreview,
            ConsumerSurfaceClass::DiagnosticsDetail,
            ConsumerSurfaceClass::CommandResult,
            ConsumerSurfaceClass::CliHeadlessInspect,
            ConsumerSurfaceClass::SupportExport,
        ] {
            if !covered_consumers.contains(&required_consumer) {
                defects.push(corpus_defect(
                    "corpus.consumer_routing_missing",
                    "decision.consumer_routing_rows",
                    format!("corpus must route to {required_consumer:?}"),
                ));
            }
        }

        if !corpus.entries.iter().any(|entry| {
            entry.arbitration_decision.disagreement_block.conflict_class != ConflictClass::None
        }) {
            defects.push(corpus_defect(
                "corpus.conflict_class_missing",
                "decision.disagreement_block.conflict_class",
                "corpus must include at least one entry where providers diverge",
            ));
        }

        if !corpus.entries.iter().any(|entry| {
            entry
                .provider_health_states
                .iter()
                .any(|row| row.health_state == ArbitrationHealthState::CrashLoopQuarantined)
        }) {
            defects.push(corpus_defect(
                "corpus.crash_loop_row_missing",
                "provider_health_states.health_state",
                "corpus must include at least one crash-loop-quarantined provider row",
            ));
        }

        ArbitrationCorpusValidationReport { defects }
    }

    /// Builds a corpus report when validation passes.
    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &ProviderArbitrationCorpus,
    ) -> Result<ArbitrationInspectorBetaReport, ArbitrationCorpusValidationReport> {
        let validation = self.validate(corpus);
        if !validation.is_empty() {
            return Err(validation);
        }

        let rows = corpus
            .entries
            .iter()
            .map(ArbitrationDecisionReportRow::from_entry)
            .collect::<Vec<_>>();
        let aggregate_counts = aggregate_counts(corpus);

        Ok(ArbitrationInspectorBetaReport {
            record_kind: PROVIDER_ARBITRATION_BETA_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            doc_ref: PROVIDER_ARBITRATION_BETA_DOC_REF.to_owned(),
            provider_health_state_schema_ref: PROVIDER_HEALTH_STATE_SCHEMA_REF.to_owned(),
            arbitration_decision_schema_ref: ARBITRATION_DECISION_SCHEMA_REF.to_owned(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            aggregate_counts,
            rows,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_entry(
        &self,
        entry: &ProviderArbitrationCorpusEntry,
        defects: &mut Vec<ArbitrationCorpusValidationDefect>,
        fixture_refs: &mut BTreeSet<String>,
        decision_ids: &mut BTreeSet<String>,
        health_ids: &mut BTreeSet<String>,
        covered_lanes: &mut BTreeSet<LanguageActionLaneClass>,
        covered_outcomes: &mut BTreeSet<ConfidenceOutcomeClass>,
        covered_gates: &mut BTreeSet<ApplyGateClass>,
        covered_consumers: &mut BTreeSet<ConsumerSurfaceClass>,
    ) {
        let decision = &entry.arbitration_decision;
        let fixture_ref = entry.fixture_ref.as_str();
        let decision_id = Some(decision.arbitration_decision_id.clone());

        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "fixture_ref.duplicate",
                "fixture_ref",
                "fixture refs must be unique",
            ));
        }
        if !decision_ids.insert(decision.arbitration_decision_id.clone()) {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "arbitration_decision_id.duplicate",
                "arbitration_decision.arbitration_decision_id",
                "arbitration decision ids must be unique",
            ));
        }

        if !entry
            .fixture_ref
            .starts_with(PROVIDER_ARBITRATION_CORPUS_DIR)
        {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "fixture_ref.not_corpus_relative",
                "fixture_ref",
                "fixture ref must live in the checked-in arbitration inspector corpus",
            ));
        }

        if decision.record_kind != ARBITRATION_DECISION_RECORD_KIND {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "decision.record_kind",
                "arbitration_decision.record_kind",
                "arbitration decision record_kind must match the inspector contract",
            ));
        }
        if decision.arbitration_decision_schema_version != ARBITRATION_DECISION_SCHEMA_VERSION {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "decision.schema_version",
                "arbitration_decision.arbitration_decision_schema_version",
                "arbitration decision schema version must match the checked-in schema",
            ));
        }

        covered_lanes.insert(decision.language_action_lane_class);
        covered_outcomes.insert(decision.confidence_outcome_class);
        covered_gates.insert(decision.apply_gate_class);
        for routing in &decision.consumer_routing_rows {
            covered_consumers.insert(routing.consumer_surface_class);
        }

        self.validate_decision_invariants(entry, defects);
        self.validate_provider_order(entry, defects);
        self.validate_health_states(entry, defects, health_ids);
        self.validate_consumer_coverage(entry, defects);
    }

    fn validate_decision_invariants(
        &self,
        entry: &ProviderArbitrationCorpusEntry,
        defects: &mut Vec<ArbitrationCorpusValidationDefect>,
    ) {
        let decision = &entry.arbitration_decision;
        let fixture_ref = entry.fixture_ref.as_str();
        let decision_id = Some(decision.arbitration_decision_id.clone());

        match decision.confidence_outcome_class {
            ConfidenceOutcomeClass::Exact => {
                if decision.negotiated_completeness_class
                    != ArbitrationCompletenessClass::CompleteForClaimedScope
                {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "decision.exact_requires_complete_scope",
                        "negotiated_completeness_class",
                        "exact outcomes require complete-for-claimed-scope coverage",
                    ));
                }
                if decision.negotiated_freshness_class
                    != ArbitrationFreshnessClass::AuthoritativeLive
                {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "decision.exact_requires_live_freshness",
                        "negotiated_freshness_class",
                        "exact outcomes require authoritative_live freshness",
                    ));
                }
                if decision.fallback_label_class != FallbackLabelClass::None {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "decision.exact_must_not_label_fallback",
                        "fallback_label_class",
                        "exact outcomes must not carry a fallback label",
                    ));
                }
                if decision
                    .downgraded_promise_block
                    .downgraded_promise_reason_class
                    != DowngradedPromiseReasonClass::None
                {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "decision.exact_must_not_downgrade",
                        "downgraded_promise_block.downgraded_promise_reason_class",
                        "exact outcomes must not carry a downgraded-promise reason",
                    ));
                }
            }
            ConfidenceOutcomeClass::Unavailable => {
                if decision.chosen_provider_id.is_some() {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "decision.unavailable_must_not_choose_provider",
                        "chosen_provider_id",
                        "unavailable outcomes must not name a winning provider",
                    ));
                }
                if !matches!(
                    decision.apply_gate_class,
                    ApplyGateClass::BlockedForHealth
                        | ApplyGateClass::BlockedForPartialScope
                        | ApplyGateClass::BlockedForDisagreement
                        | ApplyGateClass::InspectOnly
                ) {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "decision.unavailable_requires_blocked_gate",
                        "apply_gate_class",
                        "unavailable outcomes must enforce a blocked or inspect-only gate",
                    ));
                }
            }
            ConfidenceOutcomeClass::Heuristic
            | ConfidenceOutcomeClass::Partial
            | ConfidenceOutcomeClass::Stale => {
                if decision.fallback_label_class == FallbackLabelClass::None {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "decision.non_exact_requires_fallback_label",
                        "fallback_label_class",
                        "heuristic, partial, and stale outcomes must carry a fallback label",
                    ));
                }
            }
        }

        if decision.confidence_outcome_class == ConfidenceOutcomeClass::Partial
            && decision.negotiated_completeness_class
                != ArbitrationCompletenessClass::PartialForClaimedScope
        {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "decision.partial_requires_partial_completeness",
                "negotiated_completeness_class",
                "partial outcomes must declare partial completeness",
            ));
        }

        if decision.language_action_lane_class == LanguageActionLaneClass::Rename
            && matches!(
                decision.requested_scope_claim_class,
                ArbitrationScopeClaimClass::ActiveWorkset
                    | ArbitrationScopeClaimClass::WholeWorkspace
            )
            && decision.negotiated_completeness_class
                == ArbitrationCompletenessClass::PartialForClaimedScope
            && !matches!(
                decision.apply_gate_class,
                ApplyGateClass::PreviewRequired
                    | ApplyGateClass::SideBranchRequired
                    | ApplyGateClass::BlockedForPartialScope
                    | ApplyGateClass::InspectOnly
            )
        {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "decision.wide_scope_rename_requires_preview_gate",
                "apply_gate_class",
                "wide-scope rename with partial completeness must route through preview or side-branch review",
            ));
        }

        if decision.disagreement_block.conflict_class != ConflictClass::None
            && decision.apply_gate_class == ApplyGateClass::ReadyToApply
        {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "decision.conflict_blocks_ready_apply",
                "apply_gate_class",
                "non-empty conflict class must block ready-to-apply",
            ));
        }

        if decision.disagreement_block.conflict_class != ConflictClass::None
            && decision.disagreement_block.disagreement_visibility_class
                == DisagreementVisibilityClass::None
        {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "decision.conflict_requires_visible_disagreement",
                "disagreement_block.disagreement_visibility_class",
                "non-empty conflict must surface through a visible disagreement panel",
            ));
        }
    }

    fn validate_provider_order(
        &self,
        entry: &ProviderArbitrationCorpusEntry,
        defects: &mut Vec<ArbitrationCorpusValidationDefect>,
    ) {
        let decision = &entry.arbitration_decision;
        let fixture_ref = entry.fixture_ref.as_str();
        let decision_id = Some(decision.arbitration_decision_id.clone());

        let mut seen_ranks = BTreeSet::new();
        for row in &decision.provider_order_rows {
            if row.rank == 0 {
                defects.push(entry_defect(
                    fixture_ref,
                    decision_id.clone(),
                    "provider_order.rank_zero",
                    "provider_order_rows.rank",
                    "provider order ranks must be 1-based",
                ));
            }
            if !seen_ranks.insert(row.rank) {
                defects.push(entry_defect(
                    fixture_ref,
                    decision_id.clone(),
                    "provider_order.rank_duplicate",
                    "provider_order_rows.rank",
                    "provider order ranks must be unique within a decision",
                ));
            }
        }

        let chosen_in_order = if let Some(chosen) = decision.chosen_provider_id.as_deref() {
            decision
                .provider_order_rows
                .iter()
                .any(|row| row.provider_id == chosen)
        } else {
            true
        };
        if !chosen_in_order {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "provider_order.chosen_not_listed",
                "provider_order_rows",
                "chosen provider must appear in the provider order rows",
            ));
        }

        let referenced_health_refs: BTreeSet<&str> = decision
            .provider_order_rows
            .iter()
            .map(|row| row.provider_health_state_ref.as_str())
            .collect();
        let provided_health_refs: BTreeSet<&str> = entry
            .provider_health_states
            .iter()
            .map(|row| row.provider_health_state_id.as_str())
            .collect();
        for missing in referenced_health_refs.difference(&provided_health_refs) {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "provider_order.health_ref_unresolved",
                "provider_order_rows.provider_health_state_ref",
                format!(
                    "provider order references health state {missing} but no matching row is bundled"
                ),
            ));
        }
    }

    fn validate_health_states(
        &self,
        entry: &ProviderArbitrationCorpusEntry,
        defects: &mut Vec<ArbitrationCorpusValidationDefect>,
        health_ids: &mut BTreeSet<String>,
    ) {
        let decision = &entry.arbitration_decision;
        let fixture_ref = entry.fixture_ref.as_str();
        let decision_id = Some(decision.arbitration_decision_id.clone());

        if entry.provider_health_states.is_empty() {
            defects.push(entry_defect(
                fixture_ref,
                decision_id.clone(),
                "provider_health_states.empty",
                "provider_health_states",
                "every arbitration decision must bundle the provider-health rows it references",
            ));
        }

        for row in &entry.provider_health_states {
            if row.record_kind != PROVIDER_HEALTH_STATE_RECORD_KIND {
                defects.push(entry_defect(
                    fixture_ref,
                    decision_id.clone(),
                    "provider_health_state.record_kind",
                    "provider_health_states.record_kind",
                    "provider health state record_kind must match the inspector contract",
                ));
            }
            if row.provider_health_state_schema_version != PROVIDER_HEALTH_STATE_SCHEMA_VERSION {
                defects.push(entry_defect(
                    fixture_ref,
                    decision_id.clone(),
                    "provider_health_state.schema_version",
                    "provider_health_states.provider_health_state_schema_version",
                    "provider health state schema version must match the checked-in schema",
                ));
            }
            if !health_ids.insert(row.provider_health_state_id.clone()) {
                defects.push(entry_defect(
                    fixture_ref,
                    decision_id.clone(),
                    "provider_health_state.id_duplicate",
                    "provider_health_states.provider_health_state_id",
                    "provider-health-state ids must be unique across the corpus",
                ));
            }

            if row.health_state == ArbitrationHealthState::CrashLoopQuarantined {
                if row.quarantine_ref.is_none() {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "provider_health_state.quarantine_ref_missing",
                        "provider_health_states.quarantine_ref",
                        "quarantined providers must carry a quarantine reference",
                    ));
                }
                if row.downgraded_promise_block.downgraded_promise_reason_class
                    != DowngradedPromiseReasonClass::CrashLoopExcluded
                {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "provider_health_state.crash_loop_promise_mismatch",
                        "provider_health_states.downgraded_promise_block",
                        "quarantined providers must surface a crash-loop-excluded downgrade",
                    ));
                }
                if row.retry_isolate_controls.retry_action_class == RetryActionClass::NotAvailable
                    && row.retry_isolate_controls.isolate_action_class
                        == IsolateActionClass::NotAvailable
                {
                    defects.push(entry_defect(
                        fixture_ref,
                        decision_id.clone(),
                        "provider_health_state.quarantine_actions_missing",
                        "provider_health_states.retry_isolate_controls",
                        "quarantined providers must expose at least one retry or isolate action",
                    ));
                }
            }

            if row.provider_family == ProviderFamily::AiAssist
                && row.provider_role_class != ProviderRoleClass::AssistOnly
            {
                defects.push(entry_defect(
                    fixture_ref,
                    decision_id.clone(),
                    "provider_health_state.ai_assist_must_be_assist_only",
                    "provider_health_states.provider_role_class",
                    "AI assist providers must remain in assist-only role",
                ));
            }
        }
    }

    fn validate_consumer_coverage(
        &self,
        entry: &ProviderArbitrationCorpusEntry,
        defects: &mut Vec<ArbitrationCorpusValidationDefect>,
    ) {
        let decision = &entry.arbitration_decision;
        let fixture_ref = entry.fixture_ref.as_str();
        let decision_id = Some(decision.arbitration_decision_id.clone());

        let mut seen = BTreeSet::new();
        for routing in &decision.consumer_routing_rows {
            if !seen.insert(routing.consumer_surface_class) {
                defects.push(entry_defect(
                    fixture_ref,
                    decision_id.clone(),
                    "consumer_routing.duplicate",
                    "consumer_routing_rows.consumer_surface_class",
                    "consumer routing rows must be unique within a decision",
                ));
            }
        }

        let required = [
            ConsumerSurfaceClass::EditorChrome,
            ConsumerSurfaceClass::CommandResult,
            ConsumerSurfaceClass::CliHeadlessInspect,
            ConsumerSurfaceClass::SupportExport,
        ];
        for required_consumer in required {
            if !seen.contains(&required_consumer) {
                defects.push(entry_defect(
                    fixture_ref,
                    decision_id.clone(),
                    "consumer_routing.required_consumer_missing",
                    "consumer_routing_rows",
                    format!(
                        "every decision must route to {required_consumer:?} so the same record feeds every consumer"
                    ),
                ));
            }
        }
    }
}

fn aggregate_counts(corpus: &ProviderArbitrationCorpus) -> ArbitrationDecisionAggregateCounts {
    let mut aggregate = ArbitrationDecisionAggregateCounts {
        total_rows: corpus.entries.len() as u32,
        exact_rows: 0,
        heuristic_rows: 0,
        partial_rows: 0,
        stale_rows: 0,
        unavailable_rows: 0,
        conflict_rows: 0,
        quarantined_provider_rows: 0,
        preview_gated_rows: 0,
        side_branch_gated_rows: 0,
    };

    for entry in &corpus.entries {
        match entry.arbitration_decision.confidence_outcome_class {
            ConfidenceOutcomeClass::Exact => aggregate.exact_rows += 1,
            ConfidenceOutcomeClass::Heuristic => aggregate.heuristic_rows += 1,
            ConfidenceOutcomeClass::Partial => aggregate.partial_rows += 1,
            ConfidenceOutcomeClass::Stale => aggregate.stale_rows += 1,
            ConfidenceOutcomeClass::Unavailable => aggregate.unavailable_rows += 1,
        }
        if entry.arbitration_decision.disagreement_block.conflict_class != ConflictClass::None {
            aggregate.conflict_rows += 1;
        }
        if entry
            .provider_health_states
            .iter()
            .any(|row| row.health_state == ArbitrationHealthState::CrashLoopQuarantined)
        {
            aggregate.quarantined_provider_rows += 1;
        }
        match entry.arbitration_decision.apply_gate_class {
            ApplyGateClass::PreviewRequired => aggregate.preview_gated_rows += 1,
            ApplyGateClass::SideBranchRequired => aggregate.side_branch_gated_rows += 1,
            _ => {}
        }
    }

    aggregate
}

fn entry_defect(
    fixture_ref: &str,
    arbitration_decision_id: Option<String>,
    check_id: &str,
    field_name: &str,
    summary: impl Into<String>,
) -> ArbitrationCorpusValidationDefect {
    ArbitrationCorpusValidationDefect {
        fixture_ref: fixture_ref.to_owned(),
        arbitration_decision_id,
        check_id: check_id.to_owned(),
        field_name: field_name.to_owned(),
        summary: summary.into(),
    }
}

fn corpus_defect(
    check_id: &str,
    field_name: &str,
    summary: impl Into<String>,
) -> ArbitrationCorpusValidationDefect {
    ArbitrationCorpusValidationDefect {
        fixture_ref: String::new(),
        arbitration_decision_id: None,
        check_id: check_id.to_owned(),
        field_name: field_name.to_owned(),
        summary: summary.into(),
    }
}
