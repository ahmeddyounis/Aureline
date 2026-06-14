//! Code-action and quick-fix picker truth packet.
//!
//! This module is the language-owned contract for the code-action and
//! quick-fix pickers across the new M5 artifact families — framework
//! packs, notebook cells, docs artifacts, request/structured artifacts,
//! config artifacts, and generated source. Where the sibling
//! [`crate::provider_refactor_matrix_truth_packet`] freezes which posture
//! each artifact family may claim and the
//! [`crate::semantic_result_arbitration_truth_packet`] keeps the *result*
//! each surface anchors honest, this packet certifies the *picker entry*
//! the user actually invokes: which provider is acting, how wide the
//! mutation reaches, whether a typed preview is required before apply,
//! which validation hook runs, and which manual / fallback path stays
//! visible when the acting provider is partial, stale, or low confidence.
//!
//! Each lane binds a headline `picker_lane_quality` row naming the acting
//! provider plus one admission row per picker dimension:
//!
//! - an **apply-posture** admission — the central output: whether a
//!   mutating action is `inline_safe`, `preview_required`, `compare_only`,
//!   or `blocked_pending_review`, co-bound with the mutation scope, the
//!   validation hook, the typed preview completeness label, the preview
//!   hash the action packet exports, and the rollback checkpoint ref the
//!   action packet exports;
//! - a **generated-asset policy** admission — whether the lane is not
//!   generated, must regenerate before edit, may edit with a regeneration
//!   replay, is edit-blocked, or is compare-only;
//! - a **fallback-path** admission — the manual-fix / repair / regenerate
//!   / broaden-review guidance that stays visible, or an explicit
//!   none-needed / disabled posture;
//! - a **provider-disagreement** admission — whether a disagreement keeps
//!   the winning and losing providers both inspectable, surfaces an
//!   unresolved disagreement, records a policy override, or (forbidden)
//!   collapses the loser into ranking-only output; and
//! - a **rollback-checkpoint** admission — the rollback route the picker
//!   may claim.
//!
//! The packet reuses the closed provider-family, generated-artifact
//! policy, rollback-path, preview-completeness, support, evidence,
//! known-limit, downgrade-automation, confidence, promotion-state, and
//! consumer-surface vocabularies frozen by the
//! [`crate::provider_refactor_matrix_truth_packet`] matrix instead of
//! minting a local synonym set, and adds only the apply-posture,
//! mutation-scope, validation-hook, fallback-path, and disagreement
//! vocabulary the pickers need on top. It never weakens the launch-language
//! refactor safety model: a mutating apply that widens into generated or
//! structured artifacts without a preview, a preview-required action with
//! no preview hash or completeness label, a mutating apply with no
//! checkpoint ref, a disagreement collapsed to ranking-only, or a hidden
//! manual-fix path on a low-confidence provider all narrow the packet
//! below stable instead of publishing.
//!
//! The packet is metadata-only: it never admits raw source bodies, raw
//! notebook outputs, raw generated artifacts, refactor diffs, provider
//! payloads, secrets, or ambient credentials past the boundary. It carries
//! opaque ids, closed vocabulary tokens, and export-safe refs only.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConsumerSurface, DowngradeAutomationClass, EvidenceClass,
    FindingSeverity, GeneratedArtifactPolicyClass, KnownLimitClass, PromotionState,
    ProviderFamilyClass, RollbackPathClass, SupportClass,
};

/// Stable record-kind tag for [`CodeActionQuickFixPickerTruthPacket`].
pub const CODE_ACTION_QUICK_FIX_PICKER_TRUTH_PACKET_RECORD_KIND: &str =
    "code_action_quick_fix_picker_truth_stable_packet";

/// Stable record-kind tag for [`CodeActionQuickFixPickerTruthSupportExport`].
pub const CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "code_action_quick_fix_picker_truth_support_export";

/// Integer schema version for the code-action / quick-fix picker truth packet.
pub const CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_REF: &str =
    "schemas/language/code_action_quick_fix_picker_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const CODE_ACTION_QUICK_FIX_PICKER_TRUTH_DOC_REF: &str =
    "docs/m5/code-action-and-quick-fix-pickers-acting-provider-mutation-scope-and-validation-hooks.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const CODE_ACTION_QUICK_FIX_PICKER_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m5/code-action-and-quick-fix-pickers-acting-provider-mutation-scope-and-validation-hooks.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const CODE_ACTION_QUICK_FIX_PICKER_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m5/code_action_quick_fix_picker_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const CODE_ACTION_QUICK_FIX_PICKER_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m5/code_action_quick_fix_picker_truth_packet.json";

/// Closed artifact-family lane vocabulary for the new M5 picker surfaces.
/// Every required lane MUST have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamilyLaneClass {
    /// Framework analyzer / framework-pack lane.
    FrameworkPackLane,
    /// Notebook-aware cell semantics lane.
    NotebookCellLane,
    /// Docs / markup artifact lane.
    DocsArtifactLane,
    /// API request / structured-artifact lane.
    RequestArtifactLane,
    /// Config / infra artifact lane.
    ConfigArtifactLane,
    /// Generated / scaffolded source bridge lane.
    GeneratedSourceLane,
}

impl ArtifactFamilyLaneClass {
    /// Every required artifact-family lane, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::FrameworkPackLane,
        Self::NotebookCellLane,
        Self::DocsArtifactLane,
        Self::RequestArtifactLane,
        Self::ConfigArtifactLane,
        Self::GeneratedSourceLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackLane => "framework_pack_lane",
            Self::NotebookCellLane => "notebook_cell_lane",
            Self::DocsArtifactLane => "docs_artifact_lane",
            Self::RequestArtifactLane => "request_artifact_lane",
            Self::ConfigArtifactLane => "config_artifact_lane",
            Self::GeneratedSourceLane => "generated_source_lane",
        }
    }
}

/// Closed picker-row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PickerRowClass {
    /// The lane's headline qualification row binding acting provider and support.
    PickerLaneQuality,
    /// Apply-posture admission row co-binding posture, mutation scope, and hook.
    ApplyPostureAdmission,
    /// Generated-asset policy admission row binding one generated-asset policy.
    GeneratedAssetPolicyAdmission,
    /// Fallback / manual-path admission row binding one fallback path.
    FallbackPathAdmission,
    /// Provider-disagreement admission row binding one disagreement visibility.
    ProviderDisagreementAdmission,
    /// Rollback-checkpoint admission row binding one rollback route.
    RollbackCheckpointAdmission,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl PickerRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PickerLaneQuality => "picker_lane_quality",
            Self::ApplyPostureAdmission => "apply_posture_admission",
            Self::GeneratedAssetPolicyAdmission => "generated_asset_policy_admission",
            Self::FallbackPathAdmission => "fallback_path_admission",
            Self::ProviderDisagreementAdmission => "provider_disagreement_admission",
            Self::RollbackCheckpointAdmission => "rollback_checkpoint_admission",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when the row class must name a concrete acting provider family.
    pub const fn requires_acting_provider(self) -> bool {
        matches!(self, Self::PickerLaneQuality)
    }
}

/// Closed apply-posture vocabulary. An `apply_posture_admission` row binds
/// exactly one posture. This is the central picker output: how the user is
/// allowed to apply a mutating code action or quick fix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyPostureClass {
    /// The action may apply inline without a preview step.
    InlineSafe,
    /// The action requires a typed preview before apply.
    PreviewRequired,
    /// The action is compare-only; it shows a diff but never applies.
    CompareOnly,
    /// The action is blocked pending broader review.
    BlockedPendingReview,
    /// Row is not an apply-posture admission row.
    NotApplicable,
    /// Row has no bound posture; this never qualifies certified for a row
    /// class that requires a binding.
    PostureUnbound,
}

impl ApplyPostureClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineSafe => "inline_safe",
            Self::PreviewRequired => "preview_required",
            Self::CompareOnly => "compare_only",
            Self::BlockedPendingReview => "blocked_pending_review",
            Self::NotApplicable => "not_applicable",
            Self::PostureUnbound => "posture_unbound",
        }
    }

    /// True when this posture is a concrete, bound output.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::PostureUnbound)
    }

    /// True when this posture is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::PostureUnbound)
    }

    /// True when this posture requires a typed preview before any apply.
    pub const fn requires_preview(self) -> bool {
        matches!(self, Self::PreviewRequired | Self::CompareOnly)
    }

    /// True when this posture actually writes to source on apply.
    pub const fn applies_mutation(self) -> bool {
        matches!(self, Self::InlineSafe | Self::PreviewRequired)
    }
}

/// Closed mutation-scope vocabulary. The `apply_posture_admission` row
/// co-binds exactly one mutation scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationScopeClass {
    /// The action does not mutate source.
    NoMutation,
    /// The action mutates a single file.
    SingleFileScope,
    /// The action mutates multiple files in the same artifact family.
    MultiFileScope,
    /// The action reaches across artifact families.
    CrossArtifactScope,
    /// The action reaches into generated source.
    GeneratedArtifactScope,
    /// The action reaches into a structured (API / infra / config) artifact.
    StructuredArtifactScope,
    /// The action reaches workspace-wide.
    WorkspaceWideScope,
    /// Row is not an apply-posture admission row.
    NotApplicable,
    /// Row has no bound scope; this never qualifies certified for a row
    /// class that requires a binding.
    ScopeUnbound,
}

impl MutationScopeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutation => "no_mutation",
            Self::SingleFileScope => "single_file_scope",
            Self::MultiFileScope => "multi_file_scope",
            Self::CrossArtifactScope => "cross_artifact_scope",
            Self::GeneratedArtifactScope => "generated_artifact_scope",
            Self::StructuredArtifactScope => "structured_artifact_scope",
            Self::WorkspaceWideScope => "workspace_wide_scope",
            Self::NotApplicable => "not_applicable",
            Self::ScopeUnbound => "scope_unbound",
        }
    }

    /// True when this scope is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ScopeUnbound)
    }

    /// True when this scope is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ScopeUnbound)
    }

    /// True when this scope writes to source.
    pub const fn is_mutating(self) -> bool {
        !matches!(
            self,
            Self::NoMutation | Self::NotApplicable | Self::ScopeUnbound
        )
    }

    /// True when this scope reaches into generated, structured, cross-artifact,
    /// or workspace-wide territory that one-click apply must not widen into
    /// without a preview.
    pub const fn widens_into_protected_artifacts(self) -> bool {
        matches!(
            self,
            Self::CrossArtifactScope
                | Self::GeneratedArtifactScope
                | Self::StructuredArtifactScope
                | Self::WorkspaceWideScope
        )
    }
}

/// Closed validation-hook vocabulary. The `apply_posture_admission` row
/// co-binds exactly one validation hook that runs around the action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationHookClass {
    /// No validation hook is required.
    NoneRequired,
    /// A compiler / build check runs.
    BuildCheck,
    /// A test suite runs.
    TestSuite,
    /// A type check runs.
    TypeCheck,
    /// A lint / format pass runs.
    LintFormat,
    /// A schema-validation pass runs.
    SchemaValidate,
    /// A framework-specific check runs.
    FrameworkCheck,
    /// Manual review is the only validation path.
    ManualReviewOnly,
    /// Row is not an apply-posture admission row.
    NotApplicable,
    /// Row has no bound hook; this never qualifies certified for a row
    /// class that requires a binding.
    HookUnbound,
}

impl ValidationHookClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::BuildCheck => "build_check",
            Self::TestSuite => "test_suite",
            Self::TypeCheck => "type_check",
            Self::LintFormat => "lint_format",
            Self::SchemaValidate => "schema_validate",
            Self::FrameworkCheck => "framework_check",
            Self::ManualReviewOnly => "manual_review_only",
            Self::NotApplicable => "not_applicable",
            Self::HookUnbound => "hook_unbound",
        }
    }

    /// True when this hook is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::HookUnbound)
    }

    /// True when this hook is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::HookUnbound)
    }
}

/// Closed fallback / manual-path vocabulary. A `fallback_path_admission`
/// row binds exactly one fallback path that stays visible before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackPathClass {
    /// No fallback is needed; the action is fully supported.
    NoneNeeded,
    /// Manual-fix guidance stays visible.
    ManualFixGuidance,
    /// Repair guidance is surfaced.
    RepairGuidanceSurfaced,
    /// Regenerate-first guidance is surfaced.
    RegenerateFirstGuidance,
    /// Broaden-review guidance is surfaced.
    BroadenReviewGuidance,
    /// The action is disabled with no fallback offered.
    DisabledNoFallback,
    /// Row is not a fallback-path admission row.
    NotApplicable,
    /// Row has no bound fallback; this never qualifies certified for a row
    /// class that requires a binding.
    FallbackUnbound,
}

impl FallbackPathClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneNeeded => "none_needed",
            Self::ManualFixGuidance => "manual_fix_guidance",
            Self::RepairGuidanceSurfaced => "repair_guidance_surfaced",
            Self::RegenerateFirstGuidance => "regenerate_first_guidance",
            Self::BroadenReviewGuidance => "broaden_review_guidance",
            Self::DisabledNoFallback => "disabled_no_fallback",
            Self::NotApplicable => "not_applicable",
            Self::FallbackUnbound => "fallback_unbound",
        }
    }

    /// True when this fallback is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::FallbackUnbound)
    }

    /// True when this fallback is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::FallbackUnbound)
    }

    /// True when this fallback offers no visible manual / repair guidance.
    pub const fn hides_guidance(self) -> bool {
        matches!(self, Self::NoneNeeded | Self::DisabledNoFallback)
    }
}

/// Closed provider-disagreement visibility vocabulary. A
/// `provider_disagreement_admission` row binds exactly one visibility. The
/// losing provider and downgrade reason stay inspectable; disagreement is
/// never collapsed into a ranking-only result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisagreementVisibilityClass {
    /// A single provider answered; no disagreement exists.
    SingleProviderNoDisagreement,
    /// Providers disagreed; the winner and loser both stay inspectable.
    WinnerLoserBothInspectable,
    /// Providers disagreed and the disagreement is surfaced unresolved.
    UnresolvedSurfaced,
    /// A policy / trust override decided the result and is recorded.
    PolicyOverrideRecorded,
    /// Forbidden: the losing provider is collapsed into ranking-only output.
    RankingOnlyCollapsed,
    /// Row is not a provider-disagreement admission row.
    NotApplicable,
    /// Row has no bound visibility; this never qualifies certified for a row
    /// class that requires a binding.
    VisibilityUnbound,
}

impl DisagreementVisibilityClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleProviderNoDisagreement => "single_provider_no_disagreement",
            Self::WinnerLoserBothInspectable => "winner_loser_both_inspectable",
            Self::UnresolvedSurfaced => "unresolved_surfaced",
            Self::PolicyOverrideRecorded => "policy_override_recorded",
            Self::RankingOnlyCollapsed => "ranking_only_collapsed",
            Self::NotApplicable => "not_applicable",
            Self::VisibilityUnbound => "visibility_unbound",
        }
    }

    /// True when this visibility is a concrete, bound value. The forbidden
    /// `ranking_only_collapsed` value is concrete so that the dedicated
    /// collapse finding fires rather than a generic missing-binding finding.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::VisibilityUnbound)
    }

    /// True when this visibility is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::VisibilityUnbound)
    }

    /// True when this visibility collapses the losing provider into a
    /// ranking-only result.
    pub const fn collapses_loser(self) -> bool {
        matches!(self, Self::RankingOnlyCollapsed)
    }
}

/// Closed validation-finding vocabulary for the picker packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required artifact-family lane has no row.
    MissingArtifactFamilyLaneCoverage,
    /// A lane claiming certified is missing an apply-posture admission.
    MissingApplyPostureCoverage,
    /// A lane claiming certified is missing a generated-asset policy admission.
    MissingGeneratedAssetPolicyCoverage,
    /// A lane claiming certified is missing a fallback-path admission.
    MissingFallbackPathCoverage,
    /// A lane claiming certified is missing a provider-disagreement admission.
    MissingProviderDisagreementCoverage,
    /// A lane claiming certified is missing a rollback-checkpoint admission.
    MissingRollbackCheckpointCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A headline row has no concrete acting provider.
    MissingActingProvider,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// An apply-posture admission row has no bound posture.
    MissingApplyPostureClass,
    /// An apply-posture admission row has no bound mutation scope.
    MissingMutationScopeClass,
    /// An apply-posture admission row has no bound validation hook.
    MissingValidationHookClass,
    /// A generated-asset policy admission row has no bound policy.
    MissingGeneratedAssetPolicyClass,
    /// A fallback-path admission row has no bound fallback.
    MissingFallbackPathClass,
    /// A provider-disagreement admission row has no bound visibility.
    MissingDisagreementVisibilityClass,
    /// A rollback-checkpoint admission row has no bound route.
    MissingRollbackCheckpointClass,
    /// A row claims certified while one or more bindings is unbound.
    CertifiedWithUnboundBinding,
    /// A row narrowed below certified drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// An apply-posture admission row drops its posture binding.
    ApplyPostureNotApplicable,
    /// A non-apply-posture row binds an apply posture.
    ApplyPostureNotPermittedOnRowClass,
    /// An apply-posture admission row drops its mutation-scope binding.
    MutationScopeNotApplicable,
    /// A non-apply-posture row binds a mutation scope.
    MutationScopeNotPermittedOnRowClass,
    /// An apply-posture admission row drops its validation-hook binding.
    ValidationHookNotApplicable,
    /// A non-apply-posture row binds a validation hook.
    ValidationHookNotPermittedOnRowClass,
    /// A generated-asset policy admission row drops its policy binding.
    GeneratedAssetPolicyNotApplicable,
    /// A non-generated-asset-policy row binds a generated-asset policy.
    GeneratedAssetPolicyNotPermittedOnRowClass,
    /// A fallback-path admission row drops its fallback binding.
    FallbackPathNotApplicable,
    /// A non-fallback-path row binds a fallback path.
    FallbackPathNotPermittedOnRowClass,
    /// A provider-disagreement admission row drops its visibility binding.
    DisagreementVisibilityNotApplicable,
    /// A non-provider-disagreement row binds a disagreement visibility.
    DisagreementVisibilityNotPermittedOnRowClass,
    /// A rollback-checkpoint admission row drops its route binding.
    RollbackCheckpointNotApplicable,
    /// A non-rollback-checkpoint row binds a rollback route.
    RollbackCheckpointNotPermittedOnRowClass,
    /// A one-click inline apply widens scope into generated / structured /
    /// cross-artifact / workspace territory without a preview.
    InlineApplyWidensScopeWithoutPreview,
    /// A preview-required action carries no preview hash ref.
    MissingPreviewHashRef,
    /// A preview-required action carries no typed preview completeness label.
    MissingPreviewCompletenessLabel,
    /// A mutating apply carries no rollback checkpoint ref.
    MissingCheckpointRef,
    /// A headline row names a concrete provider with no acting-provider label.
    MissingActingProviderLabel,
    /// A disagreement is collapsed into ranking-only output.
    DisagreementCollapsedToRankingOnly,
    /// A low-confidence provider hides its manual-fix / repair guidance.
    ManualFixGuidanceHidden,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops picker truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the acting-provider vocabulary.
    ActingProviderVocabularyCollapsed,
    /// A projection collapses the apply-posture vocabulary.
    ApplyPostureVocabularyCollapsed,
    /// A projection collapses the mutation-scope vocabulary.
    MutationScopeVocabularyCollapsed,
    /// A projection collapses the validation-hook vocabulary.
    ValidationHookVocabularyCollapsed,
    /// A projection collapses the generated-asset policy vocabulary.
    GeneratedAssetPolicyVocabularyCollapsed,
    /// A projection collapses the fallback-path vocabulary.
    FallbackPathVocabularyCollapsed,
    /// A projection collapses the disagreement-visibility vocabulary.
    DisagreementVisibilityVocabularyCollapsed,
    /// A projection collapses the rollback-checkpoint vocabulary.
    RollbackCheckpointVocabularyCollapsed,
    /// A projection collapses the preview-completeness vocabulary.
    PreviewCompletenessVocabularyCollapsed,
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
            Self::MissingArtifactFamilyLaneCoverage => "missing_artifact_family_lane_coverage",
            Self::MissingApplyPostureCoverage => "missing_apply_posture_coverage",
            Self::MissingGeneratedAssetPolicyCoverage => "missing_generated_asset_policy_coverage",
            Self::MissingFallbackPathCoverage => "missing_fallback_path_coverage",
            Self::MissingProviderDisagreementCoverage => "missing_provider_disagreement_coverage",
            Self::MissingRollbackCheckpointCoverage => "missing_rollback_checkpoint_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingActingProvider => "missing_acting_provider",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingApplyPostureClass => "missing_apply_posture_class",
            Self::MissingMutationScopeClass => "missing_mutation_scope_class",
            Self::MissingValidationHookClass => "missing_validation_hook_class",
            Self::MissingGeneratedAssetPolicyClass => "missing_generated_asset_policy_class",
            Self::MissingFallbackPathClass => "missing_fallback_path_class",
            Self::MissingDisagreementVisibilityClass => "missing_disagreement_visibility_class",
            Self::MissingRollbackCheckpointClass => "missing_rollback_checkpoint_class",
            Self::CertifiedWithUnboundBinding => "certified_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::ApplyPostureNotApplicable => "apply_posture_not_applicable",
            Self::ApplyPostureNotPermittedOnRowClass => "apply_posture_not_permitted_on_row_class",
            Self::MutationScopeNotApplicable => "mutation_scope_not_applicable",
            Self::MutationScopeNotPermittedOnRowClass => {
                "mutation_scope_not_permitted_on_row_class"
            }
            Self::ValidationHookNotApplicable => "validation_hook_not_applicable",
            Self::ValidationHookNotPermittedOnRowClass => {
                "validation_hook_not_permitted_on_row_class"
            }
            Self::GeneratedAssetPolicyNotApplicable => "generated_asset_policy_not_applicable",
            Self::GeneratedAssetPolicyNotPermittedOnRowClass => {
                "generated_asset_policy_not_permitted_on_row_class"
            }
            Self::FallbackPathNotApplicable => "fallback_path_not_applicable",
            Self::FallbackPathNotPermittedOnRowClass => "fallback_path_not_permitted_on_row_class",
            Self::DisagreementVisibilityNotApplicable => "disagreement_visibility_not_applicable",
            Self::DisagreementVisibilityNotPermittedOnRowClass => {
                "disagreement_visibility_not_permitted_on_row_class"
            }
            Self::RollbackCheckpointNotApplicable => "rollback_checkpoint_not_applicable",
            Self::RollbackCheckpointNotPermittedOnRowClass => {
                "rollback_checkpoint_not_permitted_on_row_class"
            }
            Self::InlineApplyWidensScopeWithoutPreview => {
                "inline_apply_widens_scope_without_preview"
            }
            Self::MissingPreviewHashRef => "missing_preview_hash_ref",
            Self::MissingPreviewCompletenessLabel => "missing_preview_completeness_label",
            Self::MissingCheckpointRef => "missing_checkpoint_ref",
            Self::MissingActingProviderLabel => "missing_acting_provider_label",
            Self::DisagreementCollapsedToRankingOnly => "disagreement_collapsed_to_ranking_only",
            Self::ManualFixGuidanceHidden => "manual_fix_guidance_hidden",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::ActingProviderVocabularyCollapsed => "acting_provider_vocabulary_collapsed",
            Self::ApplyPostureVocabularyCollapsed => "apply_posture_vocabulary_collapsed",
            Self::MutationScopeVocabularyCollapsed => "mutation_scope_vocabulary_collapsed",
            Self::ValidationHookVocabularyCollapsed => "validation_hook_vocabulary_collapsed",
            Self::GeneratedAssetPolicyVocabularyCollapsed => {
                "generated_asset_policy_vocabulary_collapsed"
            }
            Self::FallbackPathVocabularyCollapsed => "fallback_path_vocabulary_collapsed",
            Self::DisagreementVisibilityVocabularyCollapsed => {
                "disagreement_visibility_vocabulary_collapsed"
            }
            Self::RollbackCheckpointVocabularyCollapsed => {
                "rollback_checkpoint_vocabulary_collapsed"
            }
            Self::PreviewCompletenessVocabularyCollapsed => {
                "preview_completeness_vocabulary_collapsed"
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

/// One picker row binding an artifact-family lane to the posture, scope,
/// validation hook, generated-asset policy, fallback path, disagreement
/// visibility, and rollback checkpoint its code-action / quick-fix entries
/// may claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PickerRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Artifact-family lane this row certifies.
    pub lane_class: ArtifactFamilyLaneClass,
    /// Row class.
    pub row_class: PickerRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Acting provider family (or `not_applicable`).
    pub acting_provider_class: ProviderFamilyClass,
    /// Apply posture (or `not_applicable`).
    pub apply_posture_class: ApplyPostureClass,
    /// Mutation scope (or `not_applicable`).
    pub mutation_scope_class: MutationScopeClass,
    /// Validation hook (or `not_applicable`).
    pub validation_hook_class: ValidationHookClass,
    /// Generated-asset policy (or `not_applicable`).
    pub generated_asset_policy_class: GeneratedArtifactPolicyClass,
    /// Fallback / manual path (or `not_applicable`).
    pub fallback_path_class: FallbackPathClass,
    /// Provider-disagreement visibility (or `not_applicable`).
    pub disagreement_visibility_class: DisagreementVisibilityClass,
    /// Rollback checkpoint route (or `not_applicable`).
    pub rollback_checkpoint_class: RollbackPathClass,
    /// Typed preview completeness label co-bound on the apply-posture row
    /// when a preview is required (or `not_applicable`).
    pub preview_completeness_class: CompletenessClass,
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
    /// Redaction-safe display label for the acting provider, exported by the
    /// action packet. Required on a headline row that names a concrete provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acting_provider_label: Option<String>,
    /// Opaque preview content-hash ref the action packet exports. Required on
    /// an apply-posture row whose posture requires a preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_hash_ref: Option<String>,
    /// Opaque rollback checkpoint ref the action packet exports. Required on
    /// an apply-posture row whose mutating posture actually applies.
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

impl PickerRow {
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
pub struct PickerConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Picker packet id consumed by the projection.
    pub picker_packet_id_ref: String,
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
    /// True when the acting-provider vocabulary is preserved verbatim.
    pub preserves_acting_provider_vocabulary: bool,
    /// True when the apply-posture vocabulary is preserved verbatim.
    pub preserves_apply_posture_vocabulary: bool,
    /// True when the mutation-scope vocabulary is preserved verbatim.
    pub preserves_mutation_scope_vocabulary: bool,
    /// True when the validation-hook vocabulary is preserved verbatim.
    pub preserves_validation_hook_vocabulary: bool,
    /// True when the generated-asset policy vocabulary is preserved verbatim.
    pub preserves_generated_asset_policy_vocabulary: bool,
    /// True when the fallback-path vocabulary is preserved verbatim.
    pub preserves_fallback_path_vocabulary: bool,
    /// True when the disagreement-visibility vocabulary is preserved verbatim.
    pub preserves_disagreement_visibility_vocabulary: bool,
    /// True when the rollback-checkpoint vocabulary is preserved verbatim.
    pub preserves_rollback_checkpoint_vocabulary: bool,
    /// True when the preview-completeness vocabulary is preserved verbatim.
    pub preserves_preview_completeness_vocabulary: bool,
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

impl PickerConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.picker_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_acting_provider_vocabulary
            && self.preserves_apply_posture_vocabulary
            && self.preserves_mutation_scope_vocabulary
            && self.preserves_validation_hook_vocabulary
            && self.preserves_generated_asset_policy_vocabulary
            && self.preserves_fallback_path_vocabulary
            && self.preserves_disagreement_visibility_vocabulary
            && self.preserves_rollback_checkpoint_vocabulary
            && self.preserves_preview_completeness_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`CodeActionQuickFixPickerTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionQuickFixPickerTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Artifact-family lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ArtifactFamilyLaneClass>,
    /// Picker rows.
    #[serde(default)]
    pub rows: Vec<PickerRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<PickerConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet freezing the code-action / quick-fix pickers across
/// the M5 framework, notebook, docs, request, config, and generated lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionQuickFixPickerTruthPacket {
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
    /// Picker rows.
    #[serde(default)]
    pub rows: Vec<PickerRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<PickerConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl CodeActionQuickFixPickerTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: CodeActionQuickFixPickerTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: CODE_ACTION_QUICK_FIX_PICKER_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable picker invariants.
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

    /// Returns the unique acting-provider tokens observed across rows.
    pub fn acting_provider_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.acting_provider_class.as_str())
    }

    /// Returns the unique apply-posture tokens observed across rows.
    pub fn apply_posture_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.apply_posture_class.as_str())
    }

    /// Returns the unique mutation-scope tokens observed across rows.
    pub fn mutation_scope_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.mutation_scope_class.as_str())
    }

    /// Returns the unique validation-hook tokens observed across rows.
    pub fn validation_hook_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.validation_hook_class.as_str())
    }

    /// Returns the unique generated-asset policy tokens observed across rows.
    pub fn generated_asset_policy_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.generated_asset_policy_class.as_str())
    }

    /// Returns the unique fallback-path tokens observed across rows.
    pub fn fallback_path_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.fallback_path_class.as_str())
    }

    /// Returns the unique disagreement-visibility tokens observed across rows.
    pub fn disagreement_visibility_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.disagreement_visibility_class.as_str())
    }

    /// Returns the unique rollback-checkpoint tokens observed across rows.
    pub fn rollback_checkpoint_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.rollback_checkpoint_class.as_str())
    }

    /// Returns the unique preview-completeness tokens observed across rows.
    pub fn preview_completeness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.preview_completeness_class.as_str())
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

    fn unique_tokens(&self, project: impl Fn(&PickerRow) -> &'static str) -> Vec<&'static str> {
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
    ) -> CodeActionQuickFixPickerTruthSupportExport {
        CodeActionQuickFixPickerTruthSupportExport {
            record_kind: CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            picker_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            picker_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != CODE_ACTION_QUICK_FIX_PICKER_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "picker packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "picker packet has the wrong schema version",
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
                FindingKind::MissingArtifactFamilyLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered artifact-family lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingArtifactFamilyLaneCoverage,
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

    fn append_per_row_findings(&self, row: &PickerRow, findings: &mut Vec<ValidationFinding>) {
        if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                format!("row {} identity or timestamp is empty", row.row_id),
            ));
        }
        if !row.raw_source_material_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::RawSourceMaterialPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits raw source bodies past the boundary",
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
        if row.row_class.requires_acting_provider() && !row.acting_provider_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingActingProvider,
                FindingSeverity::Blocker,
                format!("row {} must name a concrete acting provider", row.row_id),
            ));
        }
        if row.row_class.requires_acting_provider()
            && row.acting_provider_class.is_concrete()
            && !PickerRow::has_label(&row.acting_provider_label)
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingActingProviderLabel,
                FindingSeverity::Blocker,
                format!(
                    "row {} names a concrete acting provider but exports no acting-provider label",
                    row.row_id
                ),
            ));
        }

        if matches!(row.support_class, SupportClass::Certified) && !row.all_bindings_satisfied() {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims certified while a binding (support, acting provider, known limit, downgrade automation, or evidence) is unbound",
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
        self.append_picker_safety_findings(row, findings);

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
        row: &PickerRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let is_apply = matches!(row.row_class, PickerRowClass::ApplyPostureAdmission);
        let is_generated = matches!(row.row_class, PickerRowClass::GeneratedAssetPolicyAdmission);
        let is_fallback = matches!(row.row_class, PickerRowClass::FallbackPathAdmission);
        let is_disagreement =
            matches!(row.row_class, PickerRowClass::ProviderDisagreementAdmission);
        let is_rollback = matches!(row.row_class, PickerRowClass::RollbackCheckpointAdmission);

        // Apply-posture dimension (owner co-binds posture, scope, and hook).
        if is_apply && !row.apply_posture_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingApplyPostureClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound apply posture", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::ApplyPostureNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is an apply_posture_admission but has no bound posture",
                    row.row_id
                ),
            ));
        }
        if !is_apply && !row.apply_posture_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ApplyPostureNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds apply posture {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.apply_posture_class.as_str()
                ),
            ));
        }

        // Mutation-scope dimension (co-bound on the apply-posture row).
        if is_apply && !row.mutation_scope_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingMutationScopeClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound mutation scope", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::MutationScopeNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is an apply_posture_admission but has no bound mutation scope",
                    row.row_id
                ),
            ));
        }
        if !is_apply && !row.mutation_scope_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::MutationScopeNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds mutation scope {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.mutation_scope_class.as_str()
                ),
            ));
        }

        // Validation-hook dimension (co-bound on the apply-posture row).
        if is_apply && !row.validation_hook_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingValidationHookClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound validation hook", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::ValidationHookNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is an apply_posture_admission but has no bound validation hook",
                    row.row_id
                ),
            ));
        }
        if !is_apply && !row.validation_hook_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ValidationHookNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds validation hook {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.validation_hook_class.as_str()
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

        // Fallback-path dimension.
        if is_fallback && !row.fallback_path_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingFallbackPathClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound fallback path", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::FallbackPathNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a fallback_path_admission but has no bound fallback",
                    row.row_id
                ),
            ));
        }
        if !is_fallback && !row.fallback_path_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::FallbackPathNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds fallback path {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.fallback_path_class.as_str()
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
    }

    fn append_picker_safety_findings(
        &self,
        row: &PickerRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        // Apply-posture safety: a mutating apply must not widen scope into
        // protected artifacts without a preview, a preview-required action must
        // export a preview hash and a typed completeness label, and a mutating
        // apply that actually writes must export a rollback checkpoint ref.
        if matches!(row.row_class, PickerRowClass::ApplyPostureAdmission)
            && row.apply_posture_class.is_concrete()
            && row.mutation_scope_class.is_concrete()
        {
            if matches!(row.apply_posture_class, ApplyPostureClass::InlineSafe)
                && row.mutation_scope_class.widens_into_protected_artifacts()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::InlineApplyWidensScopeWithoutPreview,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} applies inline while widening into {} without a preview",
                        row.row_id,
                        row.mutation_scope_class.as_str()
                    ),
                ));
            }

            if row.apply_posture_class.requires_preview() {
                if !PickerRow::has_label(&row.preview_hash_ref) {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingPreviewHashRef,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} requires a preview but exports no preview hash ref",
                            row.row_id
                        ),
                    ));
                }
                if !row.preview_completeness_class.is_concrete() {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingPreviewCompletenessLabel,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} requires a preview but carries no typed completeness label",
                            row.row_id
                        ),
                    ));
                }
            }

            if row.mutation_scope_class.is_mutating()
                && row.apply_posture_class.applies_mutation()
                && !PickerRow::has_label(&row.checkpoint_ref)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingCheckpointRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} applies a mutating action but exports no rollback checkpoint ref",
                        row.row_id
                    ),
                ));
            }
        }

        // Provider disagreement must never collapse the loser into ranking-only.
        if matches!(row.row_class, PickerRowClass::ProviderDisagreementAdmission)
            && row.disagreement_visibility_class.collapses_loser()
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

        // A low-confidence provider must keep its manual-fix / repair guidance
        // visible; it may not present a none-needed or disabled fallback.
        if matches!(row.row_class, PickerRowClass::FallbackPathAdmission)
            && matches!(row.confidence_class, ConfidenceClass::LowConfidence)
            && row.fallback_path_class.hides_guidance()
        {
            findings.push(ValidationFinding::new(
                FindingKind::ManualFixGuidanceHidden,
                FindingSeverity::Blocker,
                format!(
                    "row {} hides manual-fix / repair guidance while the acting provider is low confidence",
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
                && matches!(row.row_class, PickerRowClass::PickerLaneQuality)
                && matches!(row.support_class, SupportClass::Certified)
        });
        if !lane_claims_stable {
            return;
        }

        let required: [(PickerRowClass, FindingKind, &str); 5] = [
            (
                PickerRowClass::ApplyPostureAdmission,
                FindingKind::MissingApplyPostureCoverage,
                "apply_posture_admission",
            ),
            (
                PickerRowClass::GeneratedAssetPolicyAdmission,
                FindingKind::MissingGeneratedAssetPolicyCoverage,
                "generated_asset_policy_admission",
            ),
            (
                PickerRowClass::FallbackPathAdmission,
                FindingKind::MissingFallbackPathCoverage,
                "fallback_path_admission",
            ),
            (
                PickerRowClass::ProviderDisagreementAdmission,
                FindingKind::MissingProviderDisagreementCoverage,
                "provider_disagreement_admission",
            ),
            (
                PickerRowClass::RollbackCheckpointAdmission,
                FindingKind::MissingRollbackCheckpointCoverage,
                "rollback_checkpoint_admission",
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
        projection: &PickerConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve picker truth",
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
                projection.preserves_acting_provider_vocabulary,
                FindingKind::ActingProviderVocabularyCollapsed,
                "acting-provider",
            ),
            (
                projection.preserves_apply_posture_vocabulary,
                FindingKind::ApplyPostureVocabularyCollapsed,
                "apply-posture",
            ),
            (
                projection.preserves_mutation_scope_vocabulary,
                FindingKind::MutationScopeVocabularyCollapsed,
                "mutation-scope",
            ),
            (
                projection.preserves_validation_hook_vocabulary,
                FindingKind::ValidationHookVocabularyCollapsed,
                "validation-hook",
            ),
            (
                projection.preserves_generated_asset_policy_vocabulary,
                FindingKind::GeneratedAssetPolicyVocabularyCollapsed,
                "generated-asset-policy",
            ),
            (
                projection.preserves_fallback_path_vocabulary,
                FindingKind::FallbackPathVocabularyCollapsed,
                "fallback-path",
            ),
            (
                projection.preserves_disagreement_visibility_vocabulary,
                FindingKind::DisagreementVisibilityVocabularyCollapsed,
                "disagreement-visibility",
            ),
            (
                projection.preserves_rollback_checkpoint_vocabulary,
                FindingKind::RollbackCheckpointVocabularyCollapsed,
                "rollback-checkpoint",
            ),
            (
                projection.preserves_preview_completeness_vocabulary,
                FindingKind::PreviewCompletenessVocabularyCollapsed,
                "preview-completeness",
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
pub struct CodeActionQuickFixPickerTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub picker_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub picker_packet: CodeActionQuickFixPickerTruthPacket,
}

impl CodeActionQuickFixPickerTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_VERSION
            && self.picker_packet_id_ref == self.picker_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.picker_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable picker packet.
#[derive(Debug)]
pub enum CodeActionQuickFixPickerTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for CodeActionQuickFixPickerTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(formatter, "picker packet parse failed: {error}"),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "picker packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for CodeActionQuickFixPickerTruthArtifactError {}

/// Returns the checked-in stable code-action / quick-fix picker truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_code_action_quick_fix_picker_truth_packet(
) -> Result<CodeActionQuickFixPickerTruthPacket, CodeActionQuickFixPickerTruthArtifactError> {
    let packet: CodeActionQuickFixPickerTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m5/code_action_quick_fix_picker_truth_packet.json"
    )))
    .map_err(CodeActionQuickFixPickerTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(CodeActionQuickFixPickerTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests;
