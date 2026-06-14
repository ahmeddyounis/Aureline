//! Typed refactor transaction truth packet.
//!
//! This module is the language-owned contract that generalizes the
//! launch-language refactor transaction model onto the new M5 framework
//! packs and structured artifacts. Where the sibling
//! [`crate::provider_refactor_matrix_truth_packet`] freezes which posture
//! each artifact family may claim, and the
//! [`crate::code_action_quick_fix_picker_truth_packet`] certifies the
//! picker entry the user invokes, this packet certifies the *transaction*
//! itself: a framework-aware or structured-artifact transform is a typed
//! transaction rather than an optimistic multi-file edit. Each transaction
//! carries the refactor id, engine identity, target scope, the missing-scope
//! set, the confidence tier, grouped hunks with impact and ownership hints,
//! a validation plan, a generated-asset handling policy, an apply pipeline
//! that reuses the normal save pipeline and mutation journal, and a rollback
//! checkpoint.
//!
//! Each artifact-family lane binds a headline `transaction_lane_quality`
//! row naming the acting engine and the refactor class, plus one admission
//! row per transaction dimension:
//!
//! - a **target-scope** admission — the target scope the transaction
//!   reaches, the count of targets left out of scope (the missing-scope
//!   set), and the typed completeness label that keeps the preview honest;
//! - a **grouped-hunks** admission — the count of grouped hunks the preview
//!   groups, whether an impact summary is attached, and whether an
//!   ownership hint is attached;
//! - a **validation-plan** admission — the validation plan that runs around
//!   the transaction plus the exported plan ref;
//! - a **generated-asset policy** admission — whether the lane is not
//!   generated, must regenerate before edit, may edit with a regeneration
//!   replay, is edit-blocked, or is compare-only;
//! - an **apply-pipeline** admission — whether the apply reuses the save
//!   pipeline and mutation journal, preserves source fidelity, and refuses
//!   any privileged fast path around the transaction; and
//! - a **rollback-checkpoint** admission — the rollback route and the
//!   exported checkpoint ref; plus
//! - a **provider-disagreement** admission — whether a disagreement keeps
//!   the winning and losing engines both inspectable.
//!
//! The packet reuses the closed provider-family, refactor-class,
//! mutation-scope, generated-artifact policy, rollback-path,
//! preview-completeness, disagreement-visibility, support, evidence,
//! known-limit, downgrade-automation, confidence, promotion-state, and
//! consumer-surface vocabularies frozen by the matrix and picker packets
//! instead of minting a local synonym set, and adds only the validation-plan
//! and apply-pipeline vocabulary the transactions need on top. It never
//! weakens the launch-language refactor safety model: a transaction whose
//! preview overclaims completeness while leaving targets out of scope, whose
//! grouped hunks carry no impact summary, whose validation plan exports no
//! plan ref, whose apply bypasses the save pipeline or mutation journal or
//! source fidelity, that takes a privileged fast path, that mutates without
//! exporting a rollback checkpoint ref, that treats generated source as
//! ordinary text, or that collapses provider disagreement into ranking-only
//! output all narrow the packet below stable instead of publishing.
//!
//! The packet is metadata-only: it never admits raw source bodies, raw
//! refactor diffs, raw generated artifacts, raw notebook outputs, provider
//! payloads, secrets, or ambient credentials past the boundary. It carries
//! opaque ids, closed vocabulary tokens, and export-safe refs only.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::code_action_quick_fix_picker_truth_packet::{
    ArtifactFamilyLaneClass, DisagreementVisibilityClass, MutationScopeClass,
};
use crate::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConsumerSurface, DowngradeAutomationClass, EvidenceClass,
    FindingSeverity, GeneratedArtifactPolicyClass, KnownLimitClass, PromotionState,
    ProviderFamilyClass, RefactorTransactionClass, RollbackPathClass, SupportClass,
};

/// Stable record-kind tag for [`TypedRefactorTransactionTruthPacket`].
pub const TYPED_REFACTOR_TRANSACTION_TRUTH_PACKET_RECORD_KIND: &str =
    "typed_refactor_transaction_truth_stable_packet";

/// Stable record-kind tag for [`TypedRefactorTransactionTruthSupportExport`].
pub const TYPED_REFACTOR_TRANSACTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "typed_refactor_transaction_truth_support_export";

/// Integer schema version for the typed refactor transaction truth packet.
pub const TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_REF: &str =
    "schemas/language/typed_refactor_transaction_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const TYPED_REFACTOR_TRANSACTION_TRUTH_DOC_REF: &str =
    "docs/m5/typed-refactor-transactions-completeness-labels-generated-artifact-policy-validation-plans-and-rollback-checkpoints.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const TYPED_REFACTOR_TRANSACTION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m5/typed-refactor-transactions-completeness-labels-generated-artifact-policy-validation-plans-and-rollback-checkpoints.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const TYPED_REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m5/typed_refactor_transaction_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const TYPED_REFACTOR_TRANSACTION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m5/typed_refactor_transaction_truth_packet.json";

/// Closed transaction-row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionRowClass {
    /// The lane's headline qualification row binding acting engine, refactor
    /// class, and support.
    TransactionLaneQuality,
    /// Target-scope admission row binding scope, missing-scope set, and the
    /// typed completeness label.
    TargetScopeAdmission,
    /// Grouped-hunks admission row binding hunk grouping, impact summary, and
    /// ownership hint.
    GroupedHunksAdmission,
    /// Validation-plan admission row binding one validation plan.
    ValidationPlanAdmission,
    /// Generated-asset policy admission row binding one generated-asset policy.
    GeneratedAssetPolicyAdmission,
    /// Apply-pipeline admission row binding the save-pipeline / journal /
    /// source-fidelity posture.
    ApplyPipelineAdmission,
    /// Rollback-checkpoint admission row binding one rollback route.
    RollbackCheckpointAdmission,
    /// Provider-disagreement admission row binding one disagreement visibility.
    ProviderDisagreementAdmission,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl TransactionRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransactionLaneQuality => "transaction_lane_quality",
            Self::TargetScopeAdmission => "target_scope_admission",
            Self::GroupedHunksAdmission => "grouped_hunks_admission",
            Self::ValidationPlanAdmission => "validation_plan_admission",
            Self::GeneratedAssetPolicyAdmission => "generated_asset_policy_admission",
            Self::ApplyPipelineAdmission => "apply_pipeline_admission",
            Self::RollbackCheckpointAdmission => "rollback_checkpoint_admission",
            Self::ProviderDisagreementAdmission => "provider_disagreement_admission",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when the row class must name a concrete acting engine and refactor
    /// class.
    pub const fn requires_engine_identity(self) -> bool {
        matches!(self, Self::TransactionLaneQuality)
    }
}

/// Closed validation-plan vocabulary. A `validation_plan_admission` row binds
/// exactly one validation plan that runs around the transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationPlanClass {
    /// No validation plan is required.
    NoPlanRequired,
    /// A compiler / build check then a test suite run.
    BuildThenTest,
    /// A type check then a build check run.
    TypeThenBuild,
    /// A test suite runs.
    TestSuitePlan,
    /// A schema-validation pass runs.
    SchemaValidatePlan,
    /// A framework-specific check runs.
    FrameworkCheckPlan,
    /// A lint / format pass runs.
    LintFormatPlan,
    /// Manual review is the only validation path.
    ManualReviewPlan,
    /// Row is not a validation-plan admission row.
    NotApplicable,
    /// Row has no bound plan; this never qualifies certified for a row class
    /// that requires a binding.
    PlanUnbound,
}

impl ValidationPlanClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPlanRequired => "no_plan_required",
            Self::BuildThenTest => "build_then_test",
            Self::TypeThenBuild => "type_then_build",
            Self::TestSuitePlan => "test_suite_plan",
            Self::SchemaValidatePlan => "schema_validate_plan",
            Self::FrameworkCheckPlan => "framework_check_plan",
            Self::LintFormatPlan => "lint_format_plan",
            Self::ManualReviewPlan => "manual_review_plan",
            Self::NotApplicable => "not_applicable",
            Self::PlanUnbound => "plan_unbound",
        }
    }

    /// True when this plan is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::PlanUnbound)
    }

    /// True when this plan is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::PlanUnbound)
    }

    /// True when this plan runs at least one step and so must export a plan ref.
    pub const fn requires_plan_ref(self) -> bool {
        self.is_concrete() && !matches!(self, Self::NoPlanRequired)
    }
}

/// Closed apply-pipeline vocabulary. An `apply_pipeline_admission` row binds
/// exactly one apply pipeline. This is the central transaction safety output:
/// how the transaction reaches source on apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyPipelineClass {
    /// The transaction applies through the normal save pipeline with a grouped
    /// mutation-journal entry.
    SavePipelineWithJournal,
    /// The transaction previews first, then applies through the save pipeline.
    PreviewThenSavePipeline,
    /// The transaction is compare-only; it shows a diff but never applies.
    CompareOnlyNoApply,
    /// The transaction is blocked pending broader review.
    BlockedPendingReview,
    /// Row is not an apply-pipeline admission row.
    NotApplicable,
    /// Row has no bound pipeline; this never qualifies certified for a row
    /// class that requires a binding.
    PipelineUnbound,
}

impl ApplyPipelineClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SavePipelineWithJournal => "save_pipeline_with_journal",
            Self::PreviewThenSavePipeline => "preview_then_save_pipeline",
            Self::CompareOnlyNoApply => "compare_only_no_apply",
            Self::BlockedPendingReview => "blocked_pending_review",
            Self::NotApplicable => "not_applicable",
            Self::PipelineUnbound => "pipeline_unbound",
        }
    }

    /// True when this pipeline is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::PipelineUnbound)
    }

    /// True when this pipeline is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::PipelineUnbound)
    }

    /// True when this pipeline actually writes to source on apply, so it must
    /// reuse the save pipeline and mutation journal.
    pub const fn applies_mutation(self) -> bool {
        matches!(
            self,
            Self::SavePipelineWithJournal | Self::PreviewThenSavePipeline
        )
    }
}

/// Returns true when a rollback route is an automatic checkpoint route, so the
/// admission row must export an opaque checkpoint ref.
const fn rollback_requires_checkpoint_ref(rollback: RollbackPathClass) -> bool {
    matches!(
        rollback,
        RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint
            | RollbackPathClass::CompensatingRevertViaWorkspaceDiff
            | RollbackPathClass::GroupedMutationJournalRevert
    )
}

/// Closed validation-finding vocabulary for the transaction packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// A row carries no refactor (transaction) id.
    MissingRefactorId,
    /// Required artifact-family lane has no row.
    MissingLaneCoverage,
    /// A lane claiming certified is missing a target-scope admission.
    MissingTargetScopeCoverage,
    /// A lane claiming certified is missing a grouped-hunks admission.
    MissingGroupedHunksCoverage,
    /// A lane claiming certified is missing a validation-plan admission.
    MissingValidationPlanCoverage,
    /// A lane claiming certified is missing a generated-asset policy admission.
    MissingGeneratedAssetPolicyCoverage,
    /// A lane claiming certified is missing an apply-pipeline admission.
    MissingApplyPipelineCoverage,
    /// A lane claiming certified is missing a rollback-checkpoint admission.
    MissingRollbackCheckpointCoverage,
    /// A lane claiming certified is missing a provider-disagreement admission.
    MissingProviderDisagreementCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A headline row has no concrete acting engine.
    MissingEngineIdentity,
    /// A headline row names a concrete engine with no engine-identity label.
    MissingEngineIdentityLabel,
    /// A headline row has no concrete refactor class.
    MissingRefactorClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A target-scope admission row has no bound target scope.
    MissingTargetScopeClass,
    /// A target-scope admission row has no typed completeness label.
    MissingScopeCompletenessLabel,
    /// A validation-plan admission row has no bound plan.
    MissingValidationPlanClass,
    /// A validation-plan admission row runs steps but exports no plan ref.
    MissingValidationPlanRef,
    /// A generated-asset policy admission row has no bound policy.
    MissingGeneratedAssetPolicyClass,
    /// An apply-pipeline admission row has no bound pipeline.
    MissingApplyPipelineClass,
    /// A rollback-checkpoint admission row has no bound route.
    MissingRollbackCheckpointClass,
    /// A provider-disagreement admission row has no bound visibility.
    MissingDisagreementVisibilityClass,
    /// A row claims certified while one or more bindings is unbound.
    CertifiedWithUnboundBinding,
    /// A row narrowed below certified drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A target-scope admission row drops its scope binding.
    TargetScopeNotApplicable,
    /// A non-target-scope row binds a target scope.
    TargetScopeNotPermittedOnRowClass,
    /// A validation-plan admission row drops its plan binding.
    ValidationPlanNotApplicable,
    /// A non-validation-plan row binds a validation plan.
    ValidationPlanNotPermittedOnRowClass,
    /// A generated-asset policy admission row drops its policy binding.
    GeneratedAssetPolicyNotApplicable,
    /// A non-generated-asset-policy row binds a generated-asset policy.
    GeneratedAssetPolicyNotPermittedOnRowClass,
    /// An apply-pipeline admission row drops its pipeline binding.
    ApplyPipelineNotApplicable,
    /// A non-apply-pipeline row binds an apply pipeline.
    ApplyPipelineNotPermittedOnRowClass,
    /// A rollback-checkpoint admission row drops its route binding.
    RollbackCheckpointNotApplicable,
    /// A non-rollback-checkpoint row binds a rollback route.
    RollbackCheckpointNotPermittedOnRowClass,
    /// A provider-disagreement admission row drops its visibility binding.
    DisagreementVisibilityNotApplicable,
    /// A non-provider-disagreement row binds a disagreement visibility.
    DisagreementVisibilityNotPermittedOnRowClass,
    /// A preview claims complete while leaving targets out of scope.
    ScopeCompletenessOverclaimed,
    /// A grouped-hunks admission row groups no hunks.
    MissingGroupedHunkGrouping,
    /// A grouped-hunks admission row groups hunks with no impact summary.
    MissingImpactSummary,
    /// A grouped-hunks admission row groups hunks with no ownership hint.
    MissingOwnershipHint,
    /// A mutating apply pipeline does not reuse the save pipeline.
    ApplyPipelineBypassesSavePipeline,
    /// A mutating apply pipeline does not reuse the mutation journal.
    ApplyPipelineBypassesMutationJournal,
    /// An apply pipeline does not preserve source fidelity.
    SourceFidelityBypassed,
    /// An apply pipeline takes a privileged fast path around the transaction.
    PrivilegedFastPathNotPermitted,
    /// A mutating transaction exports no rollback checkpoint ref.
    MissingCheckpointRef,
    /// A generated-source lane treats generated artifacts as ordinary text.
    GeneratedPolicyBypassed,
    /// A disagreement is collapsed into ranking-only output.
    DisagreementCollapsedToRankingOnly,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops transaction truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the engine-identity vocabulary.
    EngineIdentityVocabularyCollapsed,
    /// A projection collapses the refactor-class vocabulary.
    RefactorClassVocabularyCollapsed,
    /// A projection collapses the target-scope vocabulary.
    TargetScopeVocabularyCollapsed,
    /// A projection collapses the scope-completeness vocabulary.
    ScopeCompletenessVocabularyCollapsed,
    /// A projection collapses the validation-plan vocabulary.
    ValidationPlanVocabularyCollapsed,
    /// A projection collapses the generated-asset policy vocabulary.
    GeneratedAssetPolicyVocabularyCollapsed,
    /// A projection collapses the apply-pipeline vocabulary.
    ApplyPipelineVocabularyCollapsed,
    /// A projection collapses the rollback-checkpoint vocabulary.
    RollbackCheckpointVocabularyCollapsed,
    /// A projection collapses the disagreement-visibility vocabulary.
    DisagreementVisibilityVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingRefactorId => "missing_refactor_id",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::MissingTargetScopeCoverage => "missing_target_scope_coverage",
            Self::MissingGroupedHunksCoverage => "missing_grouped_hunks_coverage",
            Self::MissingValidationPlanCoverage => "missing_validation_plan_coverage",
            Self::MissingGeneratedAssetPolicyCoverage => "missing_generated_asset_policy_coverage",
            Self::MissingApplyPipelineCoverage => "missing_apply_pipeline_coverage",
            Self::MissingRollbackCheckpointCoverage => "missing_rollback_checkpoint_coverage",
            Self::MissingProviderDisagreementCoverage => "missing_provider_disagreement_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingEngineIdentity => "missing_engine_identity",
            Self::MissingEngineIdentityLabel => "missing_engine_identity_label",
            Self::MissingRefactorClass => "missing_refactor_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::MissingTargetScopeClass => "missing_target_scope_class",
            Self::MissingScopeCompletenessLabel => "missing_scope_completeness_label",
            Self::MissingValidationPlanClass => "missing_validation_plan_class",
            Self::MissingValidationPlanRef => "missing_validation_plan_ref",
            Self::MissingGeneratedAssetPolicyClass => "missing_generated_asset_policy_class",
            Self::MissingApplyPipelineClass => "missing_apply_pipeline_class",
            Self::MissingRollbackCheckpointClass => "missing_rollback_checkpoint_class",
            Self::MissingDisagreementVisibilityClass => "missing_disagreement_visibility_class",
            Self::CertifiedWithUnboundBinding => "certified_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::TargetScopeNotApplicable => "target_scope_not_applicable",
            Self::TargetScopeNotPermittedOnRowClass => "target_scope_not_permitted_on_row_class",
            Self::ValidationPlanNotApplicable => "validation_plan_not_applicable",
            Self::ValidationPlanNotPermittedOnRowClass => {
                "validation_plan_not_permitted_on_row_class"
            }
            Self::GeneratedAssetPolicyNotApplicable => "generated_asset_policy_not_applicable",
            Self::GeneratedAssetPolicyNotPermittedOnRowClass => {
                "generated_asset_policy_not_permitted_on_row_class"
            }
            Self::ApplyPipelineNotApplicable => "apply_pipeline_not_applicable",
            Self::ApplyPipelineNotPermittedOnRowClass => {
                "apply_pipeline_not_permitted_on_row_class"
            }
            Self::RollbackCheckpointNotApplicable => "rollback_checkpoint_not_applicable",
            Self::RollbackCheckpointNotPermittedOnRowClass => {
                "rollback_checkpoint_not_permitted_on_row_class"
            }
            Self::DisagreementVisibilityNotApplicable => "disagreement_visibility_not_applicable",
            Self::DisagreementVisibilityNotPermittedOnRowClass => {
                "disagreement_visibility_not_permitted_on_row_class"
            }
            Self::ScopeCompletenessOverclaimed => "scope_completeness_overclaimed",
            Self::MissingGroupedHunkGrouping => "missing_grouped_hunk_grouping",
            Self::MissingImpactSummary => "missing_impact_summary",
            Self::MissingOwnershipHint => "missing_ownership_hint",
            Self::ApplyPipelineBypassesSavePipeline => "apply_pipeline_bypasses_save_pipeline",
            Self::ApplyPipelineBypassesMutationJournal => {
                "apply_pipeline_bypasses_mutation_journal"
            }
            Self::SourceFidelityBypassed => "source_fidelity_bypassed",
            Self::PrivilegedFastPathNotPermitted => "privileged_fast_path_not_permitted",
            Self::MissingCheckpointRef => "missing_checkpoint_ref",
            Self::GeneratedPolicyBypassed => "generated_policy_bypassed",
            Self::DisagreementCollapsedToRankingOnly => "disagreement_collapsed_to_ranking_only",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::EngineIdentityVocabularyCollapsed => "engine_identity_vocabulary_collapsed",
            Self::RefactorClassVocabularyCollapsed => "refactor_class_vocabulary_collapsed",
            Self::TargetScopeVocabularyCollapsed => "target_scope_vocabulary_collapsed",
            Self::ScopeCompletenessVocabularyCollapsed => "scope_completeness_vocabulary_collapsed",
            Self::ValidationPlanVocabularyCollapsed => "validation_plan_vocabulary_collapsed",
            Self::GeneratedAssetPolicyVocabularyCollapsed => {
                "generated_asset_policy_vocabulary_collapsed"
            }
            Self::ApplyPipelineVocabularyCollapsed => "apply_pipeline_vocabulary_collapsed",
            Self::RollbackCheckpointVocabularyCollapsed => {
                "rollback_checkpoint_vocabulary_collapsed"
            }
            Self::DisagreementVisibilityVocabularyCollapsed => {
                "disagreement_visibility_vocabulary_collapsed"
            }
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One transaction row binding an artifact-family lane to the engine, refactor
/// class, target scope, missing-scope set, grouped hunks, validation plan,
/// generated-asset policy, apply pipeline, rollback checkpoint, and
/// disagreement visibility its typed refactor transaction may claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Artifact-family lane this row certifies.
    pub lane_class: ArtifactFamilyLaneClass,
    /// Row class.
    pub row_class: TransactionRowClass,
    /// Stable refactor (transaction) id the lane's rows share.
    pub refactor_id: String,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Acting engine family (or `not_applicable`).
    pub acting_provider_class: ProviderFamilyClass,
    /// Refactor class (or `not_applicable`).
    pub refactor_class: RefactorTransactionClass,
    /// Target scope (or `not_applicable`).
    pub target_scope_class: MutationScopeClass,
    /// Typed scope-completeness label co-bound on the target-scope row.
    pub scope_completeness_class: CompletenessClass,
    /// Count of targets left out of the transaction scope.
    #[serde(default)]
    pub missing_scope_count: u32,
    /// Count of grouped hunks the preview groups.
    #[serde(default)]
    pub grouped_hunk_count: u32,
    /// True when an impact summary is attached to the grouped hunks.
    #[serde(default)]
    pub impact_summary_present: bool,
    /// True when an ownership hint is attached to the grouped hunks.
    #[serde(default)]
    pub ownership_hint_present: bool,
    /// Validation plan (or `not_applicable`).
    pub validation_plan_class: ValidationPlanClass,
    /// Generated-asset policy (or `not_applicable`).
    pub generated_asset_policy_class: GeneratedArtifactPolicyClass,
    /// Apply pipeline (or `not_applicable`).
    pub apply_pipeline_class: ApplyPipelineClass,
    /// True when the apply reuses the normal save pipeline.
    #[serde(default)]
    pub reuses_save_pipeline: bool,
    /// True when the apply reuses the mutation journal.
    #[serde(default)]
    pub reuses_mutation_journal: bool,
    /// True when the apply preserves source fidelity.
    #[serde(default)]
    pub source_fidelity_preserved: bool,
    /// True (forbidden) when the apply takes a privileged fast path around the
    /// transaction.
    #[serde(default)]
    pub privileged_fast_path: bool,
    /// Rollback checkpoint route (or `not_applicable`).
    pub rollback_checkpoint_class: RollbackPathClass,
    /// Provider-disagreement visibility (or `not_applicable`).
    pub disagreement_visibility_class: DisagreementVisibilityClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not `certified`,
    /// declares a non-`none_declared` known limit, or binds a non-`none`
    /// automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// Redaction-safe display label for the acting engine. Required on a
    /// headline row that names a concrete engine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine_identity_label: Option<String>,
    /// Opaque validation-plan ref the transaction exports. Required on a
    /// validation-plan row whose plan runs at least one step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_plan_ref: Option<String>,
    /// Opaque rollback checkpoint ref the transaction exports. Required on a
    /// rollback-checkpoint row whose route is an automatic checkpoint route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl TransactionRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.acting_provider_class.is_bound()
    }

    fn has_label(label: &Option<String>) -> bool {
        label.as_ref().is_some_and(|value| !value.trim().is_empty())
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Transaction packet id consumed by the projection.
    pub transaction_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the lane vocabulary is preserved verbatim.
    pub preserves_lane_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the engine-identity vocabulary is preserved verbatim.
    pub preserves_engine_identity_vocabulary: bool,
    /// True when the refactor-class vocabulary is preserved verbatim.
    pub preserves_refactor_class_vocabulary: bool,
    /// True when the target-scope vocabulary is preserved verbatim.
    pub preserves_target_scope_vocabulary: bool,
    /// True when the scope-completeness vocabulary is preserved verbatim.
    pub preserves_scope_completeness_vocabulary: bool,
    /// True when the validation-plan vocabulary is preserved verbatim.
    pub preserves_validation_plan_vocabulary: bool,
    /// True when the generated-asset policy vocabulary is preserved verbatim.
    pub preserves_generated_asset_policy_vocabulary: bool,
    /// True when the apply-pipeline vocabulary is preserved verbatim.
    pub preserves_apply_pipeline_vocabulary: bool,
    /// True when the rollback-checkpoint vocabulary is preserved verbatim.
    pub preserves_rollback_checkpoint_vocabulary: bool,
    /// True when the disagreement-visibility vocabulary is preserved verbatim.
    pub preserves_disagreement_visibility_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl TransactionConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.transaction_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_engine_identity_vocabulary
            && self.preserves_refactor_class_vocabulary
            && self.preserves_target_scope_vocabulary
            && self.preserves_scope_completeness_vocabulary
            && self.preserves_validation_plan_vocabulary
            && self.preserves_generated_asset_policy_vocabulary
            && self.preserves_apply_pipeline_vocabulary
            && self.preserves_rollback_checkpoint_vocabulary
            && self.preserves_disagreement_visibility_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`TypedRefactorTransactionTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedRefactorTransactionTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Artifact-family lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ArtifactFamilyLaneClass>,
    /// Transaction rows.
    #[serde(default)]
    pub rows: Vec<TransactionRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<TransactionConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet generalizing the launch-language refactor transaction
/// model onto the M5 framework, notebook, docs, request, config, and generated
/// lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedRefactorTransactionTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Artifact-family lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ArtifactFamilyLaneClass>,
    /// Transaction rows.
    #[serde(default)]
    pub rows: Vec<TransactionRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<TransactionConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl TypedRefactorTransactionTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: TypedRefactorTransactionTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: TYPED_REFACTOR_TRANSACTION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable transaction invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.lane_class.as_str())
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.row_class.as_str())
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.support_class.as_str())
    }

    /// Returns the unique engine-identity (acting provider) tokens.
    pub fn engine_identity_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.acting_provider_class.as_str())
    }

    /// Returns the unique refactor-class tokens observed across rows.
    pub fn refactor_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.refactor_class.as_str())
    }

    /// Returns the unique target-scope tokens observed across rows.
    pub fn target_scope_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.target_scope_class.as_str())
    }

    /// Returns the unique scope-completeness tokens observed across rows.
    pub fn scope_completeness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.scope_completeness_class.as_str())
    }

    /// Returns the unique validation-plan tokens observed across rows.
    pub fn validation_plan_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.validation_plan_class.as_str())
    }

    /// Returns the unique generated-asset policy tokens observed across rows.
    pub fn generated_asset_policy_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.generated_asset_policy_class.as_str())
    }

    /// Returns the unique apply-pipeline tokens observed across rows.
    pub fn apply_pipeline_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.apply_pipeline_class.as_str())
    }

    /// Returns the unique rollback-checkpoint tokens observed across rows.
    pub fn rollback_checkpoint_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.rollback_checkpoint_class.as_str())
    }

    /// Returns the unique disagreement-visibility tokens observed across rows.
    pub fn disagreement_visibility_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.disagreement_visibility_class.as_str())
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.known_limit_class.as_str())
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.downgrade_automation_class.as_str())
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.evidence_class.as_str())
    }

    fn unique_tokens(
        &self,
        project: impl Fn(&TransactionRow) -> &'static str,
    ) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(project(row));
        }
        set.into_iter().collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> TypedRefactorTransactionTruthSupportExport {
        TypedRefactorTransactionTruthSupportExport {
            record_kind: TYPED_REFACTOR_TRANSACTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            transaction_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            transaction_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != TYPED_REFACTOR_TRANSACTION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "transaction packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "transaction packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered artifact-family lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers artifact-family lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            self.append_per_row_findings(row, &mut findings);
        }

        for lane in &self.covered_lanes {
            self.append_per_lane_coverage_findings(*lane, &mut findings);
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            self.append_projection_findings(projection, &mut findings);
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }

    fn append_per_row_findings(&self, row: &TransactionRow, findings: &mut Vec<ValidationFinding>) {
        if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                format!("row {} identity or timestamp is empty", row.row_id),
            ));
        }
        if row.refactor_id.trim().is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRefactorId,
                FindingSeverity::Blocker,
                format!("row {} carries no refactor (transaction) id", row.row_id),
            ));
        }
        if !row.raw_source_material_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::RawSourceMaterialPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits raw source bodies or refactor diffs past the boundary",
                    row.row_id
                ),
            ));
        }
        if !row.secrets_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::SecretsPresent,
                FindingSeverity::Blocker,
                format!("row {} admits secrets past the boundary", row.row_id),
            ));
        }
        if !row.ambient_authority_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::AmbientAuthorityPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits ambient authority/credentials past the boundary",
                    row.row_id
                ),
            ));
        }

        if !row.support_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSupportClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound support class", row.row_id),
            ));
        }
        if !row.known_limit_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingKnownLimit,
                FindingSeverity::Blocker,
                format!("row {} has no bound known-limit class", row.row_id),
            ));
        }
        if !row.downgrade_automation_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDowngradeAutomation,
                FindingSeverity::Blocker,
                format!("row {} has no bound downgrade-automation class", row.row_id),
            ));
        }
        if !row.evidence_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound evidence class", row.row_id),
            ));
        }

        if row.row_class.requires_engine_identity() {
            if !row.acting_provider_class.is_concrete() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEngineIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} must name a concrete acting engine", row.row_id),
                ));
            }
            if row.acting_provider_class.is_concrete()
                && !TransactionRow::has_label(&row.engine_identity_label)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEngineIdentityLabel,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} names a concrete engine but exports no engine-identity label",
                        row.row_id
                    ),
                ));
            }
            if !row.refactor_class.is_concrete() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRefactorClass,
                    FindingSeverity::Blocker,
                    format!("row {} must name a concrete refactor class", row.row_id),
                ));
            }
        }

        if matches!(row.support_class, SupportClass::Certified) && !row.all_bindings_satisfied() {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims certified while a binding (support, acting engine, known limit, downgrade automation, or evidence) is unbound",
                    row.row_id
                ),
            ));
        }

        if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
            findings.push(ValidationFinding::new(
                FindingKind::NarrowedRowMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} has support class {} without a disclosure ref",
                    row.row_id,
                    row.support_class.as_str()
                ),
            ));
        }
        if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
            findings.push(ValidationFinding::new(
                FindingKind::KnownLimitMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} discloses known limit {} without a disclosure ref",
                    row.row_id,
                    row.known_limit_class.as_str()
                ),
            ));
        }
        if row
            .downgrade_automation_class
            .requires_explicit_disclosure()
            && row.disclosure_ref.is_none()
        {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeAutomationMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} binds downgrade automation {} without a disclosure ref",
                    row.row_id,
                    row.downgrade_automation_class.as_str()
                ),
            ));
        }

        if row.evidence_refs.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceRefs,
                FindingSeverity::Blocker,
                format!("row {} carries no evidence refs", row.row_id),
            ));
        }

        self.append_dimension_gating_findings(row, findings);
        self.append_transaction_safety_findings(row, findings);

        if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
            && matches!(row.support_class, SupportClass::Certified)
        {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Warning,
                format!(
                    "row {} claims certified at low_confidence; narrowing until evidence grows",
                    row.row_id
                ),
            ));
        }
    }

    fn append_dimension_gating_findings(
        &self,
        row: &TransactionRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let is_target = matches!(row.row_class, TransactionRowClass::TargetScopeAdmission);
        let is_plan = matches!(row.row_class, TransactionRowClass::ValidationPlanAdmission);
        let is_generated = matches!(
            row.row_class,
            TransactionRowClass::GeneratedAssetPolicyAdmission
        );
        let is_pipeline = matches!(row.row_class, TransactionRowClass::ApplyPipelineAdmission);
        let is_rollback = matches!(
            row.row_class,
            TransactionRowClass::RollbackCheckpointAdmission
        );
        let is_disagreement = matches!(
            row.row_class,
            TransactionRowClass::ProviderDisagreementAdmission
        );

        // Target-scope dimension (owner co-binds scope and completeness label).
        if is_target {
            if !row.target_scope_class.is_concrete() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingTargetScopeClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound target scope", row.row_id),
                ));
                findings.push(ValidationFinding::new(
                    FindingKind::TargetScopeNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a target_scope_admission but has no bound scope",
                        row.row_id
                    ),
                ));
            }
            if !row.scope_completeness_class.is_concrete() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingScopeCompletenessLabel,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a target_scope_admission but carries no typed completeness label",
                        row.row_id
                    ),
                ));
            }
        }
        if !is_target && !row.target_scope_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::TargetScopeNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds target scope {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.target_scope_class.as_str()
                ),
            ));
        }

        // Validation-plan dimension.
        if is_plan && !row.validation_plan_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingValidationPlanClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound validation plan", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::ValidationPlanNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a validation_plan_admission but has no bound plan",
                    row.row_id
                ),
            ));
        }
        if !is_plan && !row.validation_plan_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ValidationPlanNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds validation plan {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.validation_plan_class.as_str()
                ),
            ));
        }

        // Generated-asset policy dimension.
        if is_generated && !row.generated_asset_policy_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingGeneratedAssetPolicyClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound generated-asset policy", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::GeneratedAssetPolicyNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a generated_asset_policy_admission but has no bound policy",
                    row.row_id
                ),
            ));
        }
        if !is_generated && !row.generated_asset_policy_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::GeneratedAssetPolicyNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds generated-asset policy {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.generated_asset_policy_class.as_str()
                ),
            ));
        }

        // Apply-pipeline dimension.
        if is_pipeline && !row.apply_pipeline_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingApplyPipelineClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound apply pipeline", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::ApplyPipelineNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is an apply_pipeline_admission but has no bound pipeline",
                    row.row_id
                ),
            ));
        }
        if !is_pipeline && !row.apply_pipeline_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ApplyPipelineNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds apply pipeline {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.apply_pipeline_class.as_str()
                ),
            ));
        }

        // Rollback-checkpoint dimension.
        if is_rollback && !row.rollback_checkpoint_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRollbackCheckpointClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound rollback checkpoint route", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::RollbackCheckpointNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a rollback_checkpoint_admission but has no bound route",
                    row.row_id
                ),
            ));
        }
        if !is_rollback && !row.rollback_checkpoint_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackCheckpointNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds rollback checkpoint route {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.rollback_checkpoint_class.as_str()
                ),
            ));
        }

        // Provider-disagreement dimension.
        if is_disagreement && !row.disagreement_visibility_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDisagreementVisibilityClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound disagreement visibility", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::DisagreementVisibilityNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a provider_disagreement_admission but has no bound visibility",
                    row.row_id
                ),
            ));
        }
        if !is_disagreement && !row.disagreement_visibility_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::DisagreementVisibilityNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds disagreement visibility {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.disagreement_visibility_class.as_str()
                ),
            ));
        }
    }

    fn append_transaction_safety_findings(
        &self,
        row: &TransactionRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        // Target-scope safety: a preview may not claim complete while leaving
        // targets out of scope.
        if matches!(row.row_class, TransactionRowClass::TargetScopeAdmission)
            && row.scope_completeness_class.is_concrete()
            && matches!(row.scope_completeness_class, CompletenessClass::Complete)
            && row.missing_scope_count > 0
        {
            findings.push(ValidationFinding::new(
                FindingKind::ScopeCompletenessOverclaimed,
                FindingSeverity::Blocker,
                format!(
                    "row {} labels the preview complete while {} targets stay out of scope",
                    row.row_id, row.missing_scope_count
                ),
            ));
        }

        // Grouped-hunks safety: a real transaction groups at least one hunk and
        // attaches an impact summary and ownership hint.
        if matches!(row.row_class, TransactionRowClass::GroupedHunksAdmission) {
            if row.grouped_hunk_count == 0 {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingGroupedHunkGrouping,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} groups no hunks for the transaction preview",
                        row.row_id
                    ),
                ));
            } else {
                if !row.impact_summary_present {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingImpactSummary,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} groups hunks but attaches no impact summary",
                            row.row_id
                        ),
                    ));
                }
                if !row.ownership_hint_present {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingOwnershipHint,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} groups hunks but attaches no ownership hint",
                            row.row_id
                        ),
                    ));
                }
            }
        }

        // Validation-plan safety: a plan that runs at least one step exports a
        // plan ref.
        if matches!(row.row_class, TransactionRowClass::ValidationPlanAdmission)
            && row.validation_plan_class.requires_plan_ref()
            && !TransactionRow::has_label(&row.validation_plan_ref)
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingValidationPlanRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} runs a validation plan but exports no plan ref",
                    row.row_id
                ),
            ));
        }

        // Generated-asset policy safety: a generated-source lane may not treat
        // generated artifacts as ordinary text.
        if matches!(
            row.row_class,
            TransactionRowClass::GeneratedAssetPolicyAdmission
        ) && row.lane_class == ArtifactFamilyLaneClass::GeneratedSourceLane
            && row.generated_asset_policy_class.is_concrete()
            && matches!(
                row.generated_asset_policy_class,
                GeneratedArtifactPolicyClass::NotGenerated
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::GeneratedPolicyBypassed,
                FindingSeverity::Blocker,
                format!(
                    "row {} treats generated source as ordinary text instead of applying regenerate/compare/block policy",
                    row.row_id
                ),
            ));
        }

        // Apply-pipeline safety: a mutating apply reuses the save pipeline and
        // mutation journal, preserves source fidelity, and refuses a privileged
        // fast path around the transaction.
        if matches!(row.row_class, TransactionRowClass::ApplyPipelineAdmission)
            && row.apply_pipeline_class.is_concrete()
        {
            if row.apply_pipeline_class.applies_mutation() {
                if !row.reuses_save_pipeline {
                    findings.push(ValidationFinding::new(
                        FindingKind::ApplyPipelineBypassesSavePipeline,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} applies a mutating transaction without reusing the save pipeline",
                            row.row_id
                        ),
                    ));
                }
                if !row.reuses_mutation_journal {
                    findings.push(ValidationFinding::new(
                        FindingKind::ApplyPipelineBypassesMutationJournal,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} applies a mutating transaction without reusing the mutation journal",
                            row.row_id
                        ),
                    ));
                }
            }
            if !row.source_fidelity_preserved {
                findings.push(ValidationFinding::new(
                    FindingKind::SourceFidelityBypassed,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} applies a transaction without preserving source fidelity",
                        row.row_id
                    ),
                ));
            }
            if row.privileged_fast_path {
                findings.push(ValidationFinding::new(
                    FindingKind::PrivilegedFastPathNotPermitted,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} takes a privileged fast path around the refactor transaction",
                        row.row_id
                    ),
                ));
            }
        }

        // Rollback-checkpoint safety: a route that owns an automatic checkpoint
        // must export a checkpoint ref.
        if matches!(
            row.row_class,
            TransactionRowClass::RollbackCheckpointAdmission
        ) && row.rollback_checkpoint_class.is_concrete()
            && rollback_requires_checkpoint_ref(row.rollback_checkpoint_class)
            && !TransactionRow::has_label(&row.checkpoint_ref)
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingCheckpointRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims an automatic rollback route but exports no checkpoint ref",
                    row.row_id
                ),
            ));
        }

        // Provider disagreement must never collapse the loser into ranking-only.
        if matches!(
            row.row_class,
            TransactionRowClass::ProviderDisagreementAdmission
        ) && row.disagreement_visibility_class.collapses_loser()
        {
            findings.push(ValidationFinding::new(
                FindingKind::DisagreementCollapsedToRankingOnly,
                FindingSeverity::Blocker,
                format!(
                    "row {} collapses provider disagreement into ranking-only output",
                    row.row_id
                ),
            ));
        }
    }

    fn append_per_lane_coverage_findings(
        &self,
        lane: ArtifactFamilyLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_stable = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(row.row_class, TransactionRowClass::TransactionLaneQuality)
                && matches!(row.support_class, SupportClass::Certified)
        });
        if !lane_claims_stable {
            return;
        }

        let required: [(TransactionRowClass, FindingKind, &str); 7] = [
            (
                TransactionRowClass::TargetScopeAdmission,
                FindingKind::MissingTargetScopeCoverage,
                "target_scope_admission",
            ),
            (
                TransactionRowClass::GroupedHunksAdmission,
                FindingKind::MissingGroupedHunksCoverage,
                "grouped_hunks_admission",
            ),
            (
                TransactionRowClass::ValidationPlanAdmission,
                FindingKind::MissingValidationPlanCoverage,
                "validation_plan_admission",
            ),
            (
                TransactionRowClass::GeneratedAssetPolicyAdmission,
                FindingKind::MissingGeneratedAssetPolicyCoverage,
                "generated_asset_policy_admission",
            ),
            (
                TransactionRowClass::ApplyPipelineAdmission,
                FindingKind::MissingApplyPipelineCoverage,
                "apply_pipeline_admission",
            ),
            (
                TransactionRowClass::RollbackCheckpointAdmission,
                FindingKind::MissingRollbackCheckpointCoverage,
                "rollback_checkpoint_admission",
            ),
            (
                TransactionRowClass::ProviderDisagreementAdmission,
                FindingKind::MissingProviderDisagreementCoverage,
                "provider_disagreement_admission",
            ),
        ];

        for (row_class, finding_kind, label) in required {
            let covered = self
                .rows
                .iter()
                .any(|row| row.lane_class == lane && row.row_class == row_class);
            if !covered {
                findings.push(ValidationFinding::new(
                    finding_kind,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims certified but has no {} row",
                        lane.as_str(),
                        label
                    ),
                ));
            }
        }
    }

    fn append_projection_findings(
        &self,
        projection: &TransactionConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve transaction truth",
                    projection.projection_ref
                ),
            ));
        }
        let collapses: [(bool, FindingKind, &str); 15] = [
            (
                projection.preserves_lane_vocabulary,
                FindingKind::LaneVocabularyCollapsed,
                "lane",
            ),
            (
                projection.preserves_row_class_vocabulary,
                FindingKind::RowClassVocabularyCollapsed,
                "row-class",
            ),
            (
                projection.preserves_support_class_vocabulary,
                FindingKind::SupportClassVocabularyCollapsed,
                "support-class",
            ),
            (
                projection.preserves_engine_identity_vocabulary,
                FindingKind::EngineIdentityVocabularyCollapsed,
                "engine-identity",
            ),
            (
                projection.preserves_refactor_class_vocabulary,
                FindingKind::RefactorClassVocabularyCollapsed,
                "refactor-class",
            ),
            (
                projection.preserves_target_scope_vocabulary,
                FindingKind::TargetScopeVocabularyCollapsed,
                "target-scope",
            ),
            (
                projection.preserves_scope_completeness_vocabulary,
                FindingKind::ScopeCompletenessVocabularyCollapsed,
                "scope-completeness",
            ),
            (
                projection.preserves_validation_plan_vocabulary,
                FindingKind::ValidationPlanVocabularyCollapsed,
                "validation-plan",
            ),
            (
                projection.preserves_generated_asset_policy_vocabulary,
                FindingKind::GeneratedAssetPolicyVocabularyCollapsed,
                "generated-asset-policy",
            ),
            (
                projection.preserves_apply_pipeline_vocabulary,
                FindingKind::ApplyPipelineVocabularyCollapsed,
                "apply-pipeline",
            ),
            (
                projection.preserves_rollback_checkpoint_vocabulary,
                FindingKind::RollbackCheckpointVocabularyCollapsed,
                "rollback-checkpoint",
            ),
            (
                projection.preserves_disagreement_visibility_vocabulary,
                FindingKind::DisagreementVisibilityVocabularyCollapsed,
                "disagreement-visibility",
            ),
            (
                projection.preserves_known_limit_vocabulary,
                FindingKind::KnownLimitVocabularyCollapsed,
                "known-limit",
            ),
            (
                projection.preserves_downgrade_automation_vocabulary,
                FindingKind::DowngradeAutomationVocabularyCollapsed,
                "downgrade-automation",
            ),
            (
                projection.preserves_evidence_class_vocabulary,
                FindingKind::EvidenceClassVocabularyCollapsed,
                "evidence-class",
            ),
        ];
        for (preserved, finding_kind, label) in collapses {
            if !preserved {
                findings.push(ValidationFinding::new(
                    finding_kind,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the {} vocabulary",
                        projection.projection_ref, label
                    ),
                ));
            }
        }
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedRefactorTransactionTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub transaction_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub transaction_packet: TypedRefactorTransactionTruthPacket,
}

impl TypedRefactorTransactionTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == TYPED_REFACTOR_TRANSACTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION
            && self.transaction_packet_id_ref == self.transaction_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.transaction_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable transaction packet.
#[derive(Debug)]
pub enum TypedRefactorTransactionTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for TypedRefactorTransactionTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(formatter, "transaction packet parse failed: {error}"),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "transaction packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for TypedRefactorTransactionTruthArtifactError {}

/// Returns the checked-in stable typed refactor transaction truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_typed_refactor_transaction_truth_packet(
) -> Result<TypedRefactorTransactionTruthPacket, TypedRefactorTransactionTruthArtifactError> {
    let packet: TypedRefactorTransactionTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m5/typed_refactor_transaction_truth_packet.json"
    )))
    .map_err(TypedRefactorTransactionTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(TypedRefactorTransactionTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests;
