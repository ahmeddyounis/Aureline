//! Launch-language refactor preview, validation, and rollback evidence.
//!
//! This module owns the beta evidence lane for claimed launch-language
//! refactors. It does not apply edits. Instead it records the preview target
//! set, semantic certainty, fallback label, validation posture, and grouped
//! rollback lineage that editor, graph, review, and support surfaces consume
//! before any multi-file semantic change can mutate the workspace.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Integer schema version for language refactor preview records.
pub type RefactorPreviewSchemaVersion = u32;

/// Integer schema version for language refactor validation results.
pub type RefactorValidationResultSchemaVersion = u32;

/// Schema version used by [`RefactorPreviewRecord`].
pub const REFACTOR_PREVIEW_SCHEMA_VERSION: RefactorPreviewSchemaVersion = 1;

/// Schema version used by [`RefactorValidationResult`].
pub const REFACTOR_VALIDATION_RESULT_SCHEMA_VERSION: RefactorValidationResultSchemaVersion = 1;

/// Stable record-kind tag for preview records.
pub const REFACTOR_PREVIEW_RECORD_KIND: &str = "language_refactor_preview_record";

/// Stable record-kind tag for validation-result records.
pub const REFACTOR_VALIDATION_RESULT_RECORD_KIND: &str =
    "language_refactor_validation_result_record";

/// Stable record-kind tag for corpus reports.
pub const REFACTOR_PREVIEW_BETA_REPORT_RECORD_KIND: &str = "language_refactor_preview_beta_report";

/// Repository-relative schema ref for preview records.
pub const REFACTOR_PREVIEW_SCHEMA_REF: &str = "schemas/language/refactor_preview.schema.json";

/// Repository-relative schema ref for validation-result records.
pub const REFACTOR_VALIDATION_RESULT_SCHEMA_REF: &str =
    "schemas/language/refactor_validation_result.schema.json";

/// Repository-relative documentation ref for the beta refactor evidence lane.
pub const REFACTOR_PREVIEW_BETA_DOC_REF: &str = "docs/language/refactor_preview_beta.md";

/// Directory containing the checked-in launch-language refactor corpus.
pub const REFACTOR_PREVIEW_CORPUS_DIR: &str = "fixtures/language/refactor_preview_and_rollback";

const PYTHON_RENAME_WARM_SEMANTIC_PATH: &str =
    "fixtures/language/refactor_preview_and_rollback/python_rename_warm_semantic.yaml";
const TSJS_EXTRACT_PARTIAL_INDEX_PATH: &str =
    "fixtures/language/refactor_preview_and_rollback/tsjs_extract_partial_index.yaml";
const PYTHON_MOVE_CACHED_SEMANTIC_PATH: &str =
    "fixtures/language/refactor_preview_and_rollback/python_move_cached_semantic.yaml";
const TSJS_UPDATE_IMPORTS_REMOTE_ASSISTED_PATH: &str =
    "fixtures/language/refactor_preview_and_rollback/tsjs_update_imports_remote_assisted.yaml";
const PYTHON_SIGNATURE_GENERATED_LIMIT_PATH: &str =
    "fixtures/language/refactor_preview_and_rollback/python_signature_generated_limit.yaml";
const TSJS_MOVE_POLICY_LIMITED_TEXT_PATH: &str =
    "fixtures/language/refactor_preview_and_rollback/tsjs_move_policy_limited_text.yaml";

const CURRENT_REFACTOR_PREVIEW_FIXTURES: &[(&str, &str)] = &[
    (
        PYTHON_RENAME_WARM_SEMANTIC_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/refactor_preview_and_rollback/python_rename_warm_semantic.yaml"
        )),
    ),
    (
        TSJS_EXTRACT_PARTIAL_INDEX_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/refactor_preview_and_rollback/tsjs_extract_partial_index.yaml"
        )),
    ),
    (
        PYTHON_MOVE_CACHED_SEMANTIC_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/refactor_preview_and_rollback/python_move_cached_semantic.yaml"
        )),
    ),
    (
        TSJS_UPDATE_IMPORTS_REMOTE_ASSISTED_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/refactor_preview_and_rollback/tsjs_update_imports_remote_assisted.yaml"
        )),
    ),
    (
        PYTHON_SIGNATURE_GENERATED_LIMIT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/refactor_preview_and_rollback/python_signature_generated_limit.yaml"
        )),
    ),
    (
        TSJS_MOVE_POLICY_LIMITED_TEXT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/refactor_preview_and_rollback/tsjs_move_policy_limited_text.yaml"
        )),
    ),
];

/// Semantic change class covered by a refactor preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorClass {
    /// Symbol or identifier rename with semantic occurrence discovery.
    RenameSymbol,
    /// Extract function, method, type, or module-level helper.
    ExtractFunction,
    /// Move symbol, file, module, or exported object.
    MoveSymbol,
    /// Rewrite import statements or exported import surfaces.
    UpdateImports,
    /// Change a callable signature and update cross-file call sites.
    CrossFileSignatureChange,
}

/// Launch-language support claim attached to the corpus row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorSupportClaimClass {
    /// The row backs a claimed beta refactor class.
    BetaClaimed,
    /// The row is admitted only with a visible partial or fallback label.
    DowngradedPartial,
    /// The row is not supported for apply and remains inspect-only.
    Unsupported,
}

/// Runtime condition under which the preview was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorRuntimeConditionClass {
    /// Semantic provider and graph are current for the requested scope.
    WarmSemantic,
    /// Semantic provider or graph covers only part of the requested scope.
    PartialIndex,
    /// Preview uses cached semantic evidence with a visible freshness label.
    CachedSemantic,
    /// Preview relies on a qualified remote helper or workspace agent.
    RemoteAssisted,
}

/// Source of truth used to assemble the preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorSemanticSourceClass {
    /// Current semantic engine produced the target set.
    SemanticCurrent,
    /// Semantic engine produced only a partial target set.
    SemanticPartial,
    /// Cached semantic evidence produced the target set.
    SemanticCached,
    /// Text or syntax fallback produced the target set.
    TextualFallback,
    /// A qualified remote helper produced semantic target evidence.
    RemoteSemanticAssisted,
}

impl RefactorSemanticSourceClass {
    /// Returns true when the preview needs an explicit fallback or source label.
    pub const fn requires_fallback_label(self) -> bool {
        !matches!(self, Self::SemanticCurrent)
    }

    /// Returns true when the source can back a green beta claim.
    pub const fn can_back_green_claim(self) -> bool {
        matches!(self, Self::SemanticCurrent | Self::RemoteSemanticAssisted)
    }
}

/// Confidence tier assigned to the preview target set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorConfidenceClass {
    /// High confidence for the requested scope.
    High,
    /// Medium confidence with a visible caveat.
    Medium,
    /// Low confidence; apply should be blocked or heavily reviewed.
    Low,
    /// Confidence is insufficient for any apply path.
    Blocked,
}

impl RefactorConfidenceClass {
    /// Returns true when the tier can back a claimed beta apply path.
    pub const fn can_back_apply(self) -> bool {
        matches!(self, Self::High | Self::Medium)
    }
}

/// Reason that a fallback, source, or partial-scope label is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorFallbackReasonClass {
    /// No fallback or source caveat applies.
    None,
    /// Semantic index covers only part of the requested scope.
    PartialIndex,
    /// Semantic engine evidence is stale.
    SemanticEngineStale,
    /// Semantic engine is missing for the requested row.
    SemanticEngineMissing,
    /// Policy narrowed or blocked semantic evidence.
    PolicyLimited,
    /// The row has not been qualified for the requested refactor class.
    NotQualifiedForRow,
    /// Remote helper or workspace shard is unavailable.
    RemoteUnavailable,
    /// Remote helper supplied qualified semantic evidence.
    RemoteAssistedSemantic,
    /// Generated artifact boundaries limited the preview.
    GeneratedArtifactLimit,
}

/// Generated-artifact posture attached to a preview note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedArtifactPostureClass {
    /// No generated artifact is involved.
    None,
    /// Generated artifacts are counted but authored source remains the mutation target.
    CountedAuthoredOnly,
    /// Generated artifacts are compare-only during preview.
    CompareOnly,
    /// Generated artifacts must be regenerated before replay.
    RegenerateRequired,
    /// Generated artifacts block apply until reviewed.
    BlockedPendingReview,
}

/// Dependency or import impact class declared by the preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorDependencyImpactClass {
    /// No dependency or import impact applies.
    None,
    /// Import statements or export barrels change.
    ImportRewrite,
    /// Public API signature or call surface changes.
    PublicApiSignature,
    /// Package, module, or project boundary may be affected.
    PackageBoundary,
    /// Generated configuration or metadata may change.
    GeneratedConfig,
}

/// Validation hook available after preview or apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorValidationHookClass {
    /// Refreshes the semantic snapshot.
    RefreshSemanticSnapshot,
    /// Re-runs diagnostics from the owning provider.
    RerunDiagnostics,
    /// Runs a type checker.
    RunTypecheck,
    /// Runs unit or targeted tests.
    RunUnitTests,
    /// Reviews generated outputs.
    InspectGeneratedOutputs,
    /// Reviews dependency or import graph effects.
    InspectDependencyGraph,
    /// Revalidates with a remote helper.
    RemoteAgentRevalidation,
    /// No automatic validation path exists.
    NoAutomaticValidationAvailable,
}

/// Validation state reported for a preview or corpus row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorValidationStateClass {
    /// Validation passed without warnings.
    Passed,
    /// Validation passed with caveats.
    PassedWithWarnings,
    /// Validation has not run yet.
    Pending,
    /// Validation ran and failed.
    Failed,
    /// Validation is blocked by missing prerequisites.
    Blocked,
    /// Validation is unsupported for this row.
    Unsupported,
}

impl RefactorValidationStateClass {
    /// Returns true when the state can back a green corpus row.
    pub const fn is_green_acceptable(self) -> bool {
        matches!(self, Self::Passed | Self::PassedWithWarnings)
    }
}

/// Rollback route declared by a preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorRollbackPathClass {
    /// Exact undo is available through a local-history checkpoint.
    ExactUndoViaLocalHistoryCheckpoint,
    /// Revert is available as a compensating workspace diff.
    CompensatingRevertViaWorkspaceDiff,
    /// Grouped mutation-journal entry owns the revert.
    GroupedMutationJournalRevert,
    /// Generated artifacts must regenerate before replay.
    RegenerateFirstThenReplay,
    /// Manual review is required before an automatic route can be claimed.
    ManualReviewRequiredNoAutomaticPath,
    /// No safe automatic rollback exists.
    NoSafeRollbackAvailable,
}

/// Grouped lineage path reused by the preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupedMutationLineageClass {
    /// Mutation remains within one buffer and uses ordinary undo.
    SingleBufferUndo,
    /// Local history captures a grouped checkpoint.
    LocalHistoryCheckpointGroup,
    /// Mutation journal captures the grouped entry.
    MutationJournalGroupedEntry,
    /// Local history and mutation journal are linked for the same group.
    LocalHistoryAndMutationJournal,
    /// Row is support-export evidence only and cannot apply.
    SupportExportOnly,
}

impl GroupedMutationLineageClass {
    /// Returns true when lineage is strong enough for a multi-file apply path.
    pub const fn is_grouped_apply_lineage(self) -> bool {
        matches!(
            self,
            Self::MutationJournalGroupedEntry | Self::LocalHistoryAndMutationJournal
        )
    }
}

/// Apply posture exposed by the preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorApplyPostureClass {
    /// User may apply after inspecting the preview.
    ReadyAfterPreview,
    /// User may inspect but apply remains downgraded or narrowed.
    PreviewOnlyDowngraded,
    /// Apply is blocked until provider or index freshness is restored.
    BlockedPendingRefresh,
    /// Apply is blocked until scope is expanded or narrowed explicitly.
    BlockedPendingScopeExpansion,
    /// Apply is blocked because the row is unsupported.
    BlockedUnsupported,
    /// Row is inspect-only.
    InspectOnly,
}

impl RefactorApplyPostureClass {
    /// Returns true when the row can mutate after review.
    pub const fn can_apply_after_review(self) -> bool {
        matches!(self, Self::ReadyAfterPreview | Self::PreviewOnlyDowngraded)
    }
}

/// Shiproom row state for the corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorCorpusRowState {
    /// Row is green for its claimed beta class.
    Green,
    /// Row is intentionally downgraded with visible labels.
    Downgraded,
    /// Row is unsupported and cannot apply.
    Unsupported,
}

/// One validation check state in a validation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorValidationCheckClass {
    /// Check passed.
    Passed,
    /// Check failed.
    Failed,
    /// Check does not apply to this row.
    NotApplicable,
}

/// Severity assigned to a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorValidationSeverityClass {
    /// Informational finding.
    Info,
    /// Non-blocking warning.
    Warning,
    /// Blocking validation error.
    Error,
}

/// Rollback drill outcome declared by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorRollbackDrillOutcomeClass {
    /// Grouped rollback drill passed.
    Passed,
    /// Grouped rollback drill failed.
    Failed,
    /// Drill did not run because apply was blocked.
    NotRunBlocked,
    /// Drill is unnecessary because the mutation stays inside one buffer.
    NotRequiredSingleBuffer,
}

/// Epoch role cited by refactor preview evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorEpochRoleClass {
    /// Workspace scope epoch.
    WorkspaceScope,
    /// Language semantic-model epoch.
    LanguageSemanticModel,
    /// Build graph epoch.
    BuildGraph,
    /// Search or index epoch.
    SearchIndexEpoch,
    /// Generated-artifact lineage epoch.
    GeneratedArtifactLineage,
    /// Policy-bundle epoch.
    PolicyBundle,
    /// Remote helper or agent session epoch.
    RemoteAgentSession,
}

/// Workspace trust state attached to the preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorTrustState {
    /// Workspace is trusted for the declared operation.
    Trusted,
    /// Workspace is restricted and mutation must be narrowed or blocked.
    Restricted,
}

/// Redaction class for preview and validation evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorRedactionClass {
    /// Metadata-only evidence is safe by default.
    MetadataSafeDefault,
    /// Evidence is restricted to the local operator.
    OperatorOnlyRestricted,
    /// Evidence is restricted to internal support.
    InternalSupportRestricted,
}

/// Epoch binding cited by a refactor preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorEpochBinding {
    /// Role of the cited epoch.
    pub epoch_role_class: RefactorEpochRoleClass,
    /// Opaque epoch reference.
    pub epoch_ref: String,
}

/// Policy context attached to a refactor preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPolicyContext {
    /// Policy epoch that admitted or narrowed the preview.
    pub policy_epoch: String,
    /// Workspace trust state.
    pub trust_state: RefactorTrustState,
    /// Execution context used to evaluate provider authority.
    pub execution_context_id: String,
}

/// Target set that a refactor preview proposes to inspect or mutate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorTargetSet {
    /// Opaque target-set reference.
    pub target_set_ref: String,
    /// Number of files affected by the target set.
    pub affected_file_count: u32,
    /// Number of symbols affected by the target set.
    pub affected_symbol_count: u32,
    /// Number of references affected by the target set.
    pub affected_reference_count: u32,
    /// Number of generated files present in or adjacent to the target set.
    pub generated_file_count: u32,
    /// Number of dependency or import notes attached to the preview.
    pub dependency_note_count: u32,
    /// Count of scopes omitted from semantic certainty.
    pub omitted_scope_count: u32,
    /// Opaque file-set reference.
    pub file_set_ref: String,
    /// Opaque symbol references represented in the target set.
    pub symbol_refs: Vec<String>,
    /// Opaque scope refs that were omitted or blocked.
    pub omitted_scope_refs: Vec<String>,
    /// Reasons the target set is partial.
    pub partial_scope_reason_classes: Vec<RefactorFallbackReasonClass>,
    /// Export-safe target-set summary.
    pub summary: String,
}

impl RefactorTargetSet {
    /// Returns true when the preview affects more than the active buffer.
    pub const fn mutates_more_than_active_buffer(&self) -> bool {
        self.affected_file_count > 1
    }
}

/// Label rendered when semantic certainty is partial, cached, remote-assisted, or textual.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorFallbackLabel {
    /// Whether the label is required for honest rendering.
    pub required: bool,
    /// Human-readable redaction-safe label.
    pub label: String,
    /// Reasons the label is required.
    pub reason_classes: Vec<RefactorFallbackReasonClass>,
    /// Export-safe label summary.
    pub summary: String,
}

impl RefactorFallbackLabel {
    /// Returns true when the label has enough detail for a required disclosure.
    pub fn is_actionable(&self) -> bool {
        self.required
            && !self.label.trim().is_empty()
            && !self.summary.trim().is_empty()
            && self
                .reason_classes
                .iter()
                .any(|reason| *reason != RefactorFallbackReasonClass::None)
    }
}

/// Generated artifact or dependency note attached to the preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorGeneratedDependencyNote {
    /// Stable note id.
    pub note_id: String,
    /// Generated-artifact posture.
    pub generated_artifact_posture_class: GeneratedArtifactPostureClass,
    /// Dependency or import impact class.
    pub dependency_impact_class: RefactorDependencyImpactClass,
    /// Number of affected files or generated outputs behind the note.
    pub affected_count: u32,
    /// Export-safe note summary.
    pub summary: String,
}

/// Validation summary embedded in a preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorValidationSummary {
    /// Validation-result record id.
    pub validation_result_ref: String,
    /// Validation state.
    pub validation_state_class: RefactorValidationStateClass,
    /// Validation hooks available or required.
    pub validation_hook_classes: Vec<RefactorValidationHookClass>,
    /// Count of likely fallout risks.
    pub fallout_risk_count: u32,
    /// Count of blocking validation defects.
    pub blocking_defect_count: u32,
    /// Export-safe validation summary.
    pub summary: String,
}

/// Rollback handle and grouped lineage declared by a preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorRollbackHandle {
    /// Opaque rollback-handle ref when rollback exists.
    pub rollback_handle_ref: Option<String>,
    /// Rollback path class.
    pub rollback_path_class: RefactorRollbackPathClass,
    /// Local-history checkpoint ref when captured.
    pub checkpoint_ref: Option<String>,
    /// Local-history group ref when the mutation spans more than one member.
    pub local_history_group_ref: Option<String>,
    /// Mutation-journal ref when a grouped mutation is recorded.
    pub mutation_journal_ref: Option<String>,
    /// Grouped lineage class.
    pub grouped_mutation_lineage_class: GroupedMutationLineageClass,
    /// Rollback drill outcome.
    pub drill_outcome_class: RefactorRollbackDrillOutcomeClass,
    /// Export-safe rollback summary.
    pub summary: String,
}

impl RefactorRollbackHandle {
    /// Returns true when the handle can support a multi-file apply path.
    pub fn is_grouped_rollback_ready(&self) -> bool {
        self.rollback_handle_ref.is_some()
            && self.checkpoint_ref.is_some()
            && self.local_history_group_ref.is_some()
            && self.mutation_journal_ref.is_some()
            && self
                .grouped_mutation_lineage_class
                .is_grouped_apply_lineage()
            && self.drill_outcome_class == RefactorRollbackDrillOutcomeClass::Passed
    }
}

/// Evidence refs connecting language preview truth to shared product surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorEvidenceBinding {
    /// Editor refactor-preview packet ref.
    pub editor_refactor_preview_ref: String,
    /// Navigation rename-preview ref when the refactor is a rename.
    pub navigation_rename_preview_ref: Option<String>,
    /// Graph impact packet ref when available.
    pub graph_impact_ref: Option<String>,
    /// Review packet ref used by diff or batch review.
    pub review_packet_ref: String,
    /// Support-export ref for the row.
    pub support_export_ref: String,
    /// Shared mutation lineage refs.
    pub mutation_lineage_refs: Vec<String>,
    /// Schema refs that bound the row.
    pub schema_refs: Vec<String>,
}

/// Exportable refactor preview record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub refactor_preview_schema_version: RefactorPreviewSchemaVersion,
    /// Stable preview id.
    pub preview_id: String,
    /// Launch-language row this preview qualifies.
    pub language_row_ref: String,
    /// Refactor class under evaluation.
    pub refactor_class: RefactorClass,
    /// Support claim posture.
    pub support_claim_class: RefactorSupportClaimClass,
    /// Runtime condition exercised by the row.
    pub runtime_condition_class: RefactorRuntimeConditionClass,
    /// Source class used to assemble the preview.
    pub semantic_source_class: RefactorSemanticSourceClass,
    /// Confidence tier.
    pub confidence_class: RefactorConfidenceClass,
    /// Apply posture.
    pub apply_posture_class: RefactorApplyPostureClass,
    /// Target set.
    pub target_set: RefactorTargetSet,
    /// Fallback or partial-source label.
    pub fallback_label: RefactorFallbackLabel,
    /// Generated artifact and dependency notes.
    pub generated_dependency_notes: Vec<RefactorGeneratedDependencyNote>,
    /// Validation summary.
    pub validation_summary: RefactorValidationSummary,
    /// Rollback handle.
    pub rollback_handle: RefactorRollbackHandle,
    /// Evidence binding.
    pub evidence_binding: RefactorEvidenceBinding,
    /// Epoch bindings.
    pub epoch_bindings: Vec<RefactorEpochBinding>,
    /// Policy context.
    pub policy_context: RefactorPolicyContext,
    /// Redaction class.
    pub redaction_class: RefactorRedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl RefactorPreviewRecord {
    /// Returns true when the preview must disclose partial or fallback truth.
    pub const fn requires_fallback_label(&self) -> bool {
        self.semantic_source_class.requires_fallback_label()
    }

    /// Returns true when the row can mutate after review.
    pub const fn can_apply_after_review(&self) -> bool {
        self.apply_posture_class.can_apply_after_review()
    }
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Finding severity.
    pub severity_class: RefactorValidationSeverityClass,
    /// Field associated with the finding.
    pub field_name: String,
    /// Export-safe finding summary.
    pub summary: String,
}

/// Exportable validation result for a refactor preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorValidationResult {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub refactor_validation_result_schema_version: RefactorValidationResultSchemaVersion,
    /// Stable validation-result id.
    pub validation_result_id: String,
    /// Preview id under validation.
    pub preview_ref: String,
    /// Shiproom row state.
    pub corpus_row_state: RefactorCorpusRowState,
    /// Validation state.
    pub validation_state_class: RefactorValidationStateClass,
    /// Whether preview target-set truth passed validation.
    pub preview_truth_check_class: RefactorValidationCheckClass,
    /// Whether fallback labeling passed validation.
    pub fallback_label_check_class: RefactorValidationCheckClass,
    /// Whether rollback lineage passed validation.
    pub rollback_lineage_check_class: RefactorValidationCheckClass,
    /// Whether support-export visibility passed validation.
    pub support_export_check_class: RefactorValidationCheckClass,
    /// Rollback drill outcome.
    pub rollback_drill_outcome_class: RefactorRollbackDrillOutcomeClass,
    /// Validation findings.
    pub findings: Vec<RefactorValidationFinding>,
    /// Support-export ref for the row.
    pub support_export_ref: String,
    /// Validation timestamp.
    pub validated_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

/// One checked-in corpus entry with its preview and validation result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewCorpusEntry {
    /// Repository-relative fixture ref.
    pub fixture_ref: String,
    /// Preview record under test.
    pub preview: RefactorPreviewRecord,
    /// Validation result expected for the preview.
    pub validation_result: RefactorValidationResult,
}

/// Checked-in corpus of refactor preview rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewCorpus {
    /// Corpus entries.
    pub entries: Vec<RefactorPreviewCorpusEntry>,
}

/// Aggregate counts for a corpus report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewAggregateCounts {
    /// Total row count.
    pub total_rows: u32,
    /// Green row count.
    pub green_rows: u32,
    /// Downgraded row count.
    pub downgraded_rows: u32,
    /// Unsupported row count.
    pub unsupported_rows: u32,
    /// Rows with required fallback or source labels.
    pub fallback_labeled_rows: u32,
    /// Rows with grouped rollback handles.
    pub grouped_rollback_rows: u32,
}

/// One row in a refactor preview corpus report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewReportRow {
    /// Repository-relative fixture ref.
    pub fixture_ref: String,
    /// Preview id.
    pub preview_id: String,
    /// Launch-language row ref.
    pub language_row_ref: String,
    /// Refactor class.
    pub refactor_class: RefactorClass,
    /// Support claim posture.
    pub support_claim_class: RefactorSupportClaimClass,
    /// Runtime condition.
    pub runtime_condition_class: RefactorRuntimeConditionClass,
    /// Shiproom row state.
    pub corpus_row_state: RefactorCorpusRowState,
    /// Source class.
    pub semantic_source_class: RefactorSemanticSourceClass,
    /// Confidence tier.
    pub confidence_class: RefactorConfidenceClass,
    /// Fallback or source label.
    pub fallback_label: String,
    /// Affected file count.
    pub affected_file_count: u32,
    /// Affected symbol count.
    pub affected_symbol_count: u32,
    /// Validation state.
    pub validation_state_class: RefactorValidationStateClass,
    /// Rollback drill outcome.
    pub rollback_drill_outcome_class: RefactorRollbackDrillOutcomeClass,
    /// Rollback handle ref.
    pub rollback_handle_ref: Option<String>,
    /// Support-export ref.
    pub support_export_ref: String,
}

impl RefactorPreviewReportRow {
    fn from_entry(entry: &RefactorPreviewCorpusEntry) -> Self {
        let preview = &entry.preview;
        let result = &entry.validation_result;
        Self {
            fixture_ref: entry.fixture_ref.clone(),
            preview_id: preview.preview_id.clone(),
            language_row_ref: preview.language_row_ref.clone(),
            refactor_class: preview.refactor_class,
            support_claim_class: preview.support_claim_class,
            runtime_condition_class: preview.runtime_condition_class,
            corpus_row_state: result.corpus_row_state,
            semantic_source_class: preview.semantic_source_class,
            confidence_class: preview.confidence_class,
            fallback_label: preview.fallback_label.label.clone(),
            affected_file_count: preview.target_set.affected_file_count,
            affected_symbol_count: preview.target_set.affected_symbol_count,
            validation_state_class: result.validation_state_class,
            rollback_drill_outcome_class: result.rollback_drill_outcome_class,
            rollback_handle_ref: preview.rollback_handle.rollback_handle_ref.clone(),
            support_export_ref: result.support_export_ref.clone(),
        }
    }
}

/// Corpus report consumed by release, support, and shiproom evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewBetaReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Report id.
    pub report_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Documentation ref.
    pub doc_ref: String,
    /// Preview schema ref.
    pub preview_schema_ref: String,
    /// Validation-result schema ref.
    pub validation_result_schema_ref: String,
    /// True when raw patches and source bodies are excluded.
    pub raw_payload_excluded: bool,
    /// True when private source material is excluded.
    pub raw_private_material_excluded: bool,
    /// Aggregate counts.
    pub aggregate_counts: RefactorPreviewAggregateCounts,
    /// Report rows.
    pub rows: Vec<RefactorPreviewReportRow>,
}

impl RefactorPreviewBetaReport {
    /// Returns true when the report is safe to include in support evidence.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.doc_ref == REFACTOR_PREVIEW_BETA_DOC_REF
            && self.preview_schema_ref == REFACTOR_PREVIEW_SCHEMA_REF
            && self.validation_result_schema_ref == REFACTOR_VALIDATION_RESULT_SCHEMA_REF
            && !self.rows.is_empty()
            && self.aggregate_counts.total_rows == self.rows.len() as u32
    }
}

/// Validation defect emitted by the corpus evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewValidationDefect {
    /// Fixture ref where the defect was found.
    pub fixture_ref: String,
    /// Preview id when available.
    pub preview_id: Option<String>,
    /// Stable check id.
    pub check_id: String,
    /// Field associated with the defect.
    pub field_name: String,
    /// Export-safe defect summary.
    pub summary: String,
}

/// Error returned when a corpus fails validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewCorpusValidationReport {
    /// Validation defects.
    pub defects: Vec<RefactorPreviewValidationDefect>,
}

impl RefactorPreviewCorpusValidationReport {
    /// Returns true when no defects were found.
    pub fn is_empty(&self) -> bool {
        self.defects.is_empty()
    }
}

impl fmt::Display for RefactorPreviewCorpusValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.defects.is_empty() {
            return write!(f, "refactor preview corpus valid");
        }
        write!(
            f,
            "refactor preview corpus has {} validation defect(s)",
            self.defects.len()
        )
    }
}

impl Error for RefactorPreviewCorpusValidationReport {}

/// Evaluates the checked-in refactor preview corpus.
#[derive(Debug, Default, Clone, Copy)]
pub struct RefactorPreviewEvaluator;

impl RefactorPreviewEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a corpus and returns all defects.
    pub fn validate(
        &self,
        corpus: &RefactorPreviewCorpus,
    ) -> RefactorPreviewCorpusValidationReport {
        let mut defects = Vec::new();
        let mut fixture_refs = BTreeSet::new();
        let mut preview_ids = BTreeSet::new();
        let mut validation_ids = BTreeSet::new();
        let mut refactor_classes = BTreeSet::new();
        let mut runtime_conditions = BTreeSet::new();

        for entry in &corpus.entries {
            self.validate_entry(
                entry,
                &mut defects,
                &mut fixture_refs,
                &mut preview_ids,
                &mut validation_ids,
                &mut refactor_classes,
                &mut runtime_conditions,
            );
        }

        for required_class in [
            RefactorClass::RenameSymbol,
            RefactorClass::ExtractFunction,
            RefactorClass::MoveSymbol,
            RefactorClass::UpdateImports,
            RefactorClass::CrossFileSignatureChange,
        ] {
            if !refactor_classes.contains(&required_class) {
                defects.push(corpus_defect(
                    "corpus.refactor_class_missing",
                    "entries",
                    format!("corpus must cover {required_class:?}"),
                ));
            }
        }

        for required_condition in [
            RefactorRuntimeConditionClass::WarmSemantic,
            RefactorRuntimeConditionClass::PartialIndex,
            RefactorRuntimeConditionClass::CachedSemantic,
            RefactorRuntimeConditionClass::RemoteAssisted,
        ] {
            if !runtime_conditions.contains(&required_condition) {
                defects.push(corpus_defect(
                    "corpus.runtime_condition_missing",
                    "entries",
                    format!("corpus must cover {required_condition:?}"),
                ));
            }
        }

        if !corpus
            .entries
            .iter()
            .any(|entry| entry.validation_result.corpus_row_state == RefactorCorpusRowState::Green)
        {
            defects.push(corpus_defect(
                "corpus.green_row_missing",
                "validation_result.corpus_row_state",
                "corpus must contain at least one green row",
            ));
        }

        if !corpus.entries.iter().any(|entry| {
            entry.validation_result.corpus_row_state == RefactorCorpusRowState::Downgraded
        }) {
            defects.push(corpus_defect(
                "corpus.downgraded_row_missing",
                "validation_result.corpus_row_state",
                "corpus must contain at least one downgraded row",
            ));
        }

        if !corpus.entries.iter().any(|entry| {
            entry.validation_result.corpus_row_state == RefactorCorpusRowState::Unsupported
        }) {
            defects.push(corpus_defect(
                "corpus.unsupported_row_missing",
                "validation_result.corpus_row_state",
                "corpus must contain at least one unsupported row",
            ));
        }

        RefactorPreviewCorpusValidationReport { defects }
    }

    /// Builds a corpus report when validation passes.
    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &RefactorPreviewCorpus,
    ) -> Result<RefactorPreviewBetaReport, RefactorPreviewCorpusValidationReport> {
        let validation = self.validate(corpus);
        if !validation.is_empty() {
            return Err(validation);
        }

        let rows = corpus
            .entries
            .iter()
            .map(RefactorPreviewReportRow::from_entry)
            .collect::<Vec<_>>();
        let aggregate_counts = aggregate_counts(corpus);

        Ok(RefactorPreviewBetaReport {
            record_kind: REFACTOR_PREVIEW_BETA_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            doc_ref: REFACTOR_PREVIEW_BETA_DOC_REF.to_owned(),
            preview_schema_ref: REFACTOR_PREVIEW_SCHEMA_REF.to_owned(),
            validation_result_schema_ref: REFACTOR_VALIDATION_RESULT_SCHEMA_REF.to_owned(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            aggregate_counts,
            rows,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_entry(
        &self,
        entry: &RefactorPreviewCorpusEntry,
        defects: &mut Vec<RefactorPreviewValidationDefect>,
        fixture_refs: &mut BTreeSet<String>,
        preview_ids: &mut BTreeSet<String>,
        validation_ids: &mut BTreeSet<String>,
        refactor_classes: &mut BTreeSet<RefactorClass>,
        runtime_conditions: &mut BTreeSet<RefactorRuntimeConditionClass>,
    ) {
        let preview = &entry.preview;
        let result = &entry.validation_result;
        let fixture_ref = entry.fixture_ref.as_str();
        let preview_id = Some(preview.preview_id.clone());

        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "fixture_ref.duplicate",
                "fixture_ref",
                "fixture refs must be unique",
            ));
        }
        if !preview_ids.insert(preview.preview_id.clone()) {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "preview_id.duplicate",
                "preview.preview_id",
                "preview ids must be unique",
            ));
        }
        if !validation_ids.insert(result.validation_result_id.clone()) {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "validation_result_id.duplicate",
                "validation_result.validation_result_id",
                "validation result ids must be unique",
            ));
        }

        refactor_classes.insert(preview.refactor_class);
        runtime_conditions.insert(preview.runtime_condition_class);

        if !entry.fixture_ref.starts_with(REFACTOR_PREVIEW_CORPUS_DIR) {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "fixture_ref.not_corpus_relative",
                "fixture_ref",
                "fixture ref must live in the checked-in refactor preview corpus",
            ));
        }

        if preview.record_kind != REFACTOR_PREVIEW_RECORD_KIND {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "preview.record_kind",
                "preview.record_kind",
                "preview record_kind must match the language refactor preview contract",
            ));
        }
        if preview.refactor_preview_schema_version != REFACTOR_PREVIEW_SCHEMA_VERSION {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "preview.schema_version",
                "preview.refactor_preview_schema_version",
                "preview schema version must match the checked-in schema",
            ));
        }
        if result.record_kind != REFACTOR_VALIDATION_RESULT_RECORD_KIND {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "validation_result.record_kind",
                "validation_result.record_kind",
                "validation result record_kind must match the contract",
            ));
        }
        if result.refactor_validation_result_schema_version
            != REFACTOR_VALIDATION_RESULT_SCHEMA_VERSION
        {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "validation_result.schema_version",
                "validation_result.refactor_validation_result_schema_version",
                "validation result schema version must match the checked-in schema",
            ));
        }
        if result.preview_ref != preview.preview_id {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "validation_result.preview_ref_mismatch",
                "validation_result.preview_ref",
                "validation result must cite the preview id it validates",
            ));
        }
        if result.validation_result_id != preview.validation_summary.validation_result_ref {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "validation_result.summary_ref_mismatch",
                "preview.validation_summary.validation_result_ref",
                "preview summary must cite the validation result id",
            ));
        }
        if result.support_export_ref != preview.evidence_binding.support_export_ref {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "support_export.ref_mismatch",
                "support_export_ref",
                "preview and validation result must share one support export ref",
            ));
        }

        self.validate_preview_truth(entry, defects);
        self.validate_fallback_label(entry, defects);
        self.validate_rollback_lineage(entry, defects);
        self.validate_generated_and_dependency_notes(entry, defects);
        self.validate_support_export(entry, defects);
    }

    fn validate_preview_truth(
        &self,
        entry: &RefactorPreviewCorpusEntry,
        defects: &mut Vec<RefactorPreviewValidationDefect>,
    ) {
        let preview = &entry.preview;
        let result = &entry.validation_result;
        let fixture_ref = entry.fixture_ref.as_str();
        let preview_id = Some(preview.preview_id.clone());

        if preview.target_set.affected_file_count == 0 {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "target_set.affected_file_count_zero",
                "preview.target_set.affected_file_count",
                "preview must name affected files before apply",
            ));
        }
        if preview.target_set.affected_symbol_count == 0 {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "target_set.affected_symbol_count_zero",
                "preview.target_set.affected_symbol_count",
                "preview must name affected symbols before apply",
            ));
        }

        if preview.support_claim_class == RefactorSupportClaimClass::BetaClaimed {
            if result.corpus_row_state != RefactorCorpusRowState::Green {
                defects.push(entry_defect(
                    fixture_ref,
                    preview_id.clone(),
                    "claim.green_state_required",
                    "validation_result.corpus_row_state",
                    "claimed beta rows must validate as green",
                ));
            }
            if !preview.semantic_source_class.can_back_green_claim() {
                defects.push(entry_defect(
                    fixture_ref,
                    preview_id.clone(),
                    "claim.semantic_source_not_green",
                    "preview.semantic_source_class",
                    "claimed beta rows cannot be backed by textual, partial, or stale cached truth",
                ));
            }
            if !preview.confidence_class.can_back_apply() {
                defects.push(entry_defect(
                    fixture_ref,
                    preview_id.clone(),
                    "claim.confidence_not_applyable",
                    "preview.confidence_class",
                    "claimed beta rows need high or medium confidence",
                ));
            }
            if !result.validation_state_class.is_green_acceptable() {
                defects.push(entry_defect(
                    fixture_ref,
                    preview_id.clone(),
                    "claim.validation_not_passing",
                    "validation_result.validation_state_class",
                    "claimed beta rows need passing validation",
                ));
            }
        }

        if preview.semantic_source_class == RefactorSemanticSourceClass::TextualFallback
            && preview.support_claim_class == RefactorSupportClaimClass::BetaClaimed
        {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "textual_fallback.masquerades_as_semantic",
                "preview.semantic_source_class",
                "textual fallback must not masquerade as semantic certainty",
            ));
        }

        if preview.target_set.omitted_scope_count > 0
            && preview.target_set.partial_scope_reason_classes.is_empty()
        {
            defects.push(entry_defect(
                fixture_ref,
                preview_id,
                "target_set.partial_reasons_missing",
                "preview.target_set.partial_scope_reason_classes",
                "omitted scopes must cite partial-scope reasons",
            ));
        }
    }

    fn validate_fallback_label(
        &self,
        entry: &RefactorPreviewCorpusEntry,
        defects: &mut Vec<RefactorPreviewValidationDefect>,
    ) {
        let preview = &entry.preview;
        let fixture_ref = entry.fixture_ref.as_str();
        let preview_id = Some(preview.preview_id.clone());

        if preview.requires_fallback_label() && !preview.fallback_label.is_actionable() {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "fallback_label.required_but_not_actionable",
                "preview.fallback_label",
                "partial, cached, remote-assisted, and textual previews must carry an explicit label",
            ));
        }

        if !preview.requires_fallback_label()
            && preview.fallback_label.required
            && preview
                .fallback_label
                .reason_classes
                .iter()
                .all(|reason| *reason == RefactorFallbackReasonClass::None)
        {
            defects.push(entry_defect(
                fixture_ref,
                preview_id,
                "fallback_label.unnecessary_required_label",
                "preview.fallback_label.required",
                "current semantic previews should not force a fallback label without a reason",
            ));
        }
    }

    fn validate_rollback_lineage(
        &self,
        entry: &RefactorPreviewCorpusEntry,
        defects: &mut Vec<RefactorPreviewValidationDefect>,
    ) {
        let preview = &entry.preview;
        let result = &entry.validation_result;
        let fixture_ref = entry.fixture_ref.as_str();
        let preview_id = Some(preview.preview_id.clone());

        if preview.target_set.mutates_more_than_active_buffer()
            && preview.can_apply_after_review()
            && !preview.rollback_handle.is_grouped_rollback_ready()
        {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "rollback.grouped_handle_missing",
                "preview.rollback_handle",
                "multi-file applyable previews must cite grouped local-history and mutation-journal rollback lineage",
            ));
        }

        if preview.rollback_handle.drill_outcome_class != result.rollback_drill_outcome_class {
            defects.push(entry_defect(
                fixture_ref,
                preview_id,
                "rollback.drill_outcome_mismatch",
                "validation_result.rollback_drill_outcome_class",
                "validation result must quote the preview rollback drill outcome",
            ));
        }
    }

    fn validate_generated_and_dependency_notes(
        &self,
        entry: &RefactorPreviewCorpusEntry,
        defects: &mut Vec<RefactorPreviewValidationDefect>,
    ) {
        let preview = &entry.preview;
        let fixture_ref = entry.fixture_ref.as_str();
        let preview_id = Some(preview.preview_id.clone());

        if preview.target_set.generated_file_count > 0
            && !preview.generated_dependency_notes.iter().any(|note| {
                note.generated_artifact_posture_class != GeneratedArtifactPostureClass::None
            })
        {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "generated_artifact.note_missing",
                "preview.generated_dependency_notes",
                "generated-file impact must be represented by generated artifact notes",
            ));
        }

        if preview.target_set.dependency_note_count > 0
            && !preview
                .generated_dependency_notes
                .iter()
                .any(|note| note.dependency_impact_class != RefactorDependencyImpactClass::None)
        {
            defects.push(entry_defect(
                fixture_ref,
                preview_id,
                "dependency_impact.note_missing",
                "preview.generated_dependency_notes",
                "dependency or import impact must be represented by dependency notes",
            ));
        }
    }

    fn validate_support_export(
        &self,
        entry: &RefactorPreviewCorpusEntry,
        defects: &mut Vec<RefactorPreviewValidationDefect>,
    ) {
        let preview = &entry.preview;
        let result = &entry.validation_result;
        let fixture_ref = entry.fixture_ref.as_str();
        let preview_id = Some(preview.preview_id.clone());

        if result.support_export_check_class != RefactorValidationCheckClass::Passed {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "support_export.check_not_passed",
                "validation_result.support_export_check_class",
                "support/export visibility must pass for every corpus row",
            ));
        }
        if result.support_export_ref.trim().is_empty() {
            defects.push(entry_defect(
                fixture_ref,
                preview_id.clone(),
                "support_export.ref_missing",
                "validation_result.support_export_ref",
                "support/export ref must be present",
            ));
        }
        if preview
            .evidence_binding
            .editor_refactor_preview_ref
            .trim()
            .is_empty()
            || preview.evidence_binding.review_packet_ref.trim().is_empty()
            || preview.evidence_binding.schema_refs.is_empty()
        {
            defects.push(entry_defect(
                fixture_ref,
                preview_id,
                "evidence_binding.shared_refs_missing",
                "preview.evidence_binding",
                "preview must bind editor preview, review packet, support export, and schema refs",
            ));
        }
    }
}

/// Loads one refactor preview corpus entry from YAML.
pub fn load_refactor_preview_case(
    yaml: &str,
) -> Result<RefactorPreviewCorpusEntry, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the current checked-in refactor preview corpus.
pub fn current_refactor_preview_corpus() -> Result<RefactorPreviewCorpus, serde_yaml::Error> {
    CURRENT_REFACTOR_PREVIEW_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<RefactorPreviewCorpusEntry>(yaml).map(|mut entry| {
                entry.fixture_ref = (*fixture_ref).to_owned();
                entry
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|entries| RefactorPreviewCorpus { entries })
}

/// Returns fixture refs included in the checked-in refactor preview corpus.
pub fn current_refactor_preview_fixture_refs() -> impl Iterator<Item = &'static str> {
    CURRENT_REFACTOR_PREVIEW_FIXTURES
        .iter()
        .map(|(fixture_ref, _)| *fixture_ref)
}

fn aggregate_counts(corpus: &RefactorPreviewCorpus) -> RefactorPreviewAggregateCounts {
    let mut aggregate = RefactorPreviewAggregateCounts {
        total_rows: corpus.entries.len() as u32,
        green_rows: 0,
        downgraded_rows: 0,
        unsupported_rows: 0,
        fallback_labeled_rows: 0,
        grouped_rollback_rows: 0,
    };

    for entry in &corpus.entries {
        match entry.validation_result.corpus_row_state {
            RefactorCorpusRowState::Green => aggregate.green_rows += 1,
            RefactorCorpusRowState::Downgraded => aggregate.downgraded_rows += 1,
            RefactorCorpusRowState::Unsupported => aggregate.unsupported_rows += 1,
        }
        if entry.preview.fallback_label.required {
            aggregate.fallback_labeled_rows += 1;
        }
        if entry.preview.rollback_handle.is_grouped_rollback_ready() {
            aggregate.grouped_rollback_rows += 1;
        }
    }

    aggregate
}

fn entry_defect(
    fixture_ref: &str,
    preview_id: Option<String>,
    check_id: &str,
    field_name: &str,
    summary: impl Into<String>,
) -> RefactorPreviewValidationDefect {
    RefactorPreviewValidationDefect {
        fixture_ref: fixture_ref.to_owned(),
        preview_id,
        check_id: check_id.to_owned(),
        field_name: field_name.to_owned(),
        summary: summary.into(),
    }
}

fn corpus_defect(
    check_id: &str,
    field_name: &str,
    summary: impl Into<String>,
) -> RefactorPreviewValidationDefect {
    RefactorPreviewValidationDefect {
        fixture_ref: REFACTOR_PREVIEW_CORPUS_DIR.to_owned(),
        preview_id: None,
        check_id: check_id.to_owned(),
        field_name: field_name.to_owned(),
        summary: summary.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_corpus_validates() {
        let corpus = current_refactor_preview_corpus().expect("corpus parses");
        let report = RefactorPreviewEvaluator::new()
            .report(
                "language:refactor-preview:report:test",
                "2026-05-18T09:00:00Z",
                &corpus,
            )
            .expect("corpus validates");
        assert!(report.is_export_safe());
        assert_eq!(
            report.aggregate_counts.total_rows,
            corpus.entries.len() as u32
        );
    }
}
