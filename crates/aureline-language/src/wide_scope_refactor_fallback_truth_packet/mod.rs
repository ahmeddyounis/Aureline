//! Wide-scope refactor fallback truth packet.
//!
//! This module is the language-owned contract for the *safe fallback posture*
//! that a wide-scope or low-confidence transform takes instead of an
//! apply-all on the live workspace. Where the sibling
//! [`crate::typed_refactor_transaction_truth_packet`] certifies that a
//! framework-aware or structured-artifact transform is a typed transaction,
//! and the [`crate::provider_refactor_matrix_truth_packet`] freezes which
//! posture each artifact family may claim, this packet certifies *how a
//! low-confidence or wide-scope transaction reaches source*: a side-branch or
//! worktree apply, a staged apply, or a compare-only review — never an
//! optimistic apply-all on the live workspace once confidence or completeness
//! falls below the frozen threshold.
//!
//! Each artifact-family lane binds a headline `fallback_lane_quality` row
//! naming the acting engine and the refactor class, plus one admission row per
//! fallback dimension:
//!
//! - an **apply-posture** admission — the safe apply posture the lane offers
//!   (side-branch, worktree, staged, compare-only, blocked, or — only when the
//!   scope is narrow, completeness is complete, and confidence is high —
//!   apply-all on the live workspace), co-bound with the target scope, the
//!   typed completeness label, the confidence tier, and the missing-scope
//!   count;
//! - an **impact-packet** admission — the impacted-target and impacted-owner
//!   counts, whether an impact summary is attached, whether a missing-scope
//!   explanation is attached, and the exported impact-packet ref;
//! - a **reviewer-hint** admission — the reviewer / owner routing hint, whether
//!   an owner hint is attached, and the exported review-anchor ref;
//! - a **rollback-path** admission — the rollback route and the exported
//!   checkpoint ref;
//! - a **support-export parity** admission — whether the support / export
//!   channel preserves the refactor lineage and the missing-scope explanation,
//!   plus the exported lineage ref; and
//! - a **provider-disagreement** admission — whether a disagreement keeps the
//!   winning and losing engines both inspectable.
//!
//! The packet reuses the closed provider-family, refactor-class,
//! mutation-scope, completeness, rollback-path, disagreement-visibility,
//! support, evidence, known-limit, downgrade-automation, confidence,
//! promotion-state, and consumer-surface vocabularies frozen by the matrix and
//! picker packets instead of minting a local synonym set, and adds only the
//! apply-posture and reviewer-hint vocabulary the fallback flows need on top.
//! It never weakens the launch-language refactor safety model: a lane that
//! offers an apply-all on the live workspace while confidence or completeness
//! fell below the frozen threshold or the scope is wide, that drops a
//! missing-scope explanation from its impact packet, that drops a reviewer
//! anchor or owner hint, that lets a writing fallback run with no safe
//! rollback path, that drops the refactor lineage or missing-scope explanation
//! from support / export, or that collapses provider disagreement into
//! ranking-only output all narrow the packet below stable instead of
//! publishing.
//!
//! The packet is metadata-only: it never admits raw source bodies, raw refactor
//! diffs, raw generated artifacts, raw notebook outputs, provider payloads,
//! secrets, or ambient credentials past the boundary. It carries opaque ids,
//! closed vocabulary tokens, and export-safe refs only.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::code_action_quick_fix_picker_truth_packet::{
    ArtifactFamilyLaneClass, DisagreementVisibilityClass, MutationScopeClass,
};
use crate::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConsumerSurface, DowngradeAutomationClass, EvidenceClass,
    FindingSeverity, KnownLimitClass, PromotionState, ProviderFamilyClass,
    RefactorTransactionClass, RollbackPathClass, SupportClass,
};

/// Stable record-kind tag for [`WideScopeRefactorFallbackTruthPacket`].
pub const WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_PACKET_RECORD_KIND: &str =
    "wide_scope_refactor_fallback_truth_stable_packet";

/// Stable record-kind tag for [`WideScopeRefactorFallbackTruthSupportExport`].
pub const WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "wide_scope_refactor_fallback_truth_support_export";

/// Integer schema version for the wide-scope refactor fallback truth packet.
pub const WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_REF: &str =
    "schemas/language/wide_scope_refactor_fallback_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_DOC_REF: &str =
    "docs/m5/wide-scope-refactor-side-branch-staged-apply-fallback-reviewer-hints-impact-packets-and-support-export-parity.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m5/wide-scope-refactor-side-branch-staged-apply-fallback-reviewer-hints-impact-packets-and-support-export-parity.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m5/wide_scope_refactor_fallback_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m5/wide_scope_refactor_fallback_truth_packet.json";

/// Closed fallback-row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackRowClass {
    /// The lane's headline qualification row binding acting engine, refactor
    /// class, and support.
    FallbackLaneQuality,
    /// Apply-posture admission row co-binding posture, target scope, typed
    /// completeness label, confidence tier, and missing-scope count.
    ApplyPostureAdmission,
    /// Impact-packet admission row binding the impacted target/owner counts,
    /// the impact summary, and the missing-scope explanation.
    ImpactPacketAdmission,
    /// Reviewer-hint admission row binding the reviewer/owner routing hint and
    /// the review anchor.
    ReviewerHintAdmission,
    /// Rollback-path admission row binding one rollback route.
    RollbackPathAdmission,
    /// Support-export parity admission row binding the lineage and missing-scope
    /// preservation posture.
    SupportExportParityAdmission,
    /// Provider-disagreement admission row binding one disagreement visibility.
    ProviderDisagreementAdmission,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl FallbackRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FallbackLaneQuality => "fallback_lane_quality",
            Self::ApplyPostureAdmission => "apply_posture_admission",
            Self::ImpactPacketAdmission => "impact_packet_admission",
            Self::ReviewerHintAdmission => "reviewer_hint_admission",
            Self::RollbackPathAdmission => "rollback_path_admission",
            Self::SupportExportParityAdmission => "support_export_parity_admission",
            Self::ProviderDisagreementAdmission => "provider_disagreement_admission",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when the row class must name a concrete acting engine and refactor
    /// class.
    pub const fn requires_engine_identity(self) -> bool {
        matches!(self, Self::FallbackLaneQuality)
    }
}

/// Closed apply-posture vocabulary. An `apply_posture_admission` row binds
/// exactly one posture. This is the central fallback safety output: how a
/// wide-scope or low-confidence transform reaches source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyFallbackPostureClass {
    /// The transform applies onto a dedicated side branch for review.
    SideBranchApply,
    /// The transform applies into an isolated worktree for review.
    WorktreeApply,
    /// The transform applies in reviewed stages on the live workspace.
    StagedApply,
    /// The transform is compare-only; it shows a diff but never applies.
    CompareOnlyReview,
    /// The transform applies all hunks on the live workspace at once. Permitted
    /// only when the scope is narrow, completeness is complete, and confidence
    /// is high.
    ApplyAllOnLiveWorkspace,
    /// The transform is blocked pending broader review.
    BlockedPendingReview,
    /// Row is not an apply-posture admission row.
    NotApplicable,
    /// Row has no bound posture; this never qualifies certified for a row class
    /// that requires a binding.
    PostureUnbound,
}

impl ApplyFallbackPostureClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SideBranchApply => "side_branch_apply",
            Self::WorktreeApply => "worktree_apply",
            Self::StagedApply => "staged_apply",
            Self::CompareOnlyReview => "compare_only_review",
            Self::ApplyAllOnLiveWorkspace => "apply_all_on_live_workspace",
            Self::BlockedPendingReview => "blocked_pending_review",
            Self::NotApplicable => "not_applicable",
            Self::PostureUnbound => "posture_unbound",
        }
    }

    /// True when this posture is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::PostureUnbound)
    }

    /// True when this posture is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::PostureUnbound)
    }

    /// True when this posture is one of the safe fallbacks (side-branch,
    /// worktree, staged, compare-only, or blocked).
    pub const fn is_safe_fallback(self) -> bool {
        matches!(
            self,
            Self::SideBranchApply
                | Self::WorktreeApply
                | Self::StagedApply
                | Self::CompareOnlyReview
                | Self::BlockedPendingReview
        )
    }

    /// True when this posture is the apply-all-on-live-workspace posture, which
    /// is only permitted under the frozen narrow / complete / high-confidence
    /// threshold.
    pub const fn is_apply_all_on_live(self) -> bool {
        matches!(self, Self::ApplyAllOnLiveWorkspace)
    }

    /// True when this posture actually writes source, so it must carry a safe
    /// rollback path.
    pub const fn writes_source(self) -> bool {
        matches!(
            self,
            Self::SideBranchApply
                | Self::WorktreeApply
                | Self::StagedApply
                | Self::ApplyAllOnLiveWorkspace
        )
    }
}

/// Closed reviewer-hint vocabulary. A `reviewer_hint_admission` row binds
/// exactly one reviewer / owner routing hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerHintClass {
    /// Route the fallback to the CODEOWNERS owner of the impacted paths.
    CodeownersReviewer,
    /// Route the fallback to the recent author / blame owner.
    RecentAuthorReviewer,
    /// Route the fallback to the owning team.
    OwningTeamReviewer,
    /// Manual reviewer assignment is required before the fallback applies.
    ManualAssignmentRequired,
    /// No reviewer is required for this low-risk fallback.
    NoReviewerRequired,
    /// Row is not a reviewer-hint admission row.
    NotApplicable,
    /// Row has no bound reviewer hint; this never qualifies certified for a row
    /// class that requires a binding.
    HintUnbound,
}

impl ReviewerHintClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeownersReviewer => "codeowners_reviewer",
            Self::RecentAuthorReviewer => "recent_author_reviewer",
            Self::OwningTeamReviewer => "owning_team_reviewer",
            Self::ManualAssignmentRequired => "manual_assignment_required",
            Self::NoReviewerRequired => "no_reviewer_required",
            Self::NotApplicable => "not_applicable",
            Self::HintUnbound => "hint_unbound",
        }
    }

    /// True when this hint is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::HintUnbound)
    }

    /// True when this hint is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::HintUnbound)
    }

    /// True when this hint routes to a reviewer, so the row must export a review
    /// anchor and carry an owner hint.
    pub const fn requires_review_anchor(self) -> bool {
        self.is_concrete() && !matches!(self, Self::NoReviewerRequired)
    }
}

/// Returns true when the target scope is wide enough to forbid an apply-all on
/// the live workspace. Single-file and no-mutation scopes are narrow; anything
/// that crosses files, artifacts, or the workspace is wide.
const fn scope_is_wide(scope: MutationScopeClass) -> bool {
    matches!(
        scope,
        MutationScopeClass::MultiFileScope
            | MutationScopeClass::CrossArtifactScope
            | MutationScopeClass::GeneratedArtifactScope
            | MutationScopeClass::StructuredArtifactScope
            | MutationScopeClass::WorkspaceWideScope
    )
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

/// Returns true when a rollback route is a safe, recoverable route. A writing
/// fallback may not run with `no_safe_rollback_available`.
const fn rollback_is_safe(rollback: RollbackPathClass) -> bool {
    matches!(
        rollback,
        RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint
            | RollbackPathClass::CompensatingRevertViaWorkspaceDiff
            | RollbackPathClass::GroupedMutationJournalRevert
            | RollbackPathClass::RegenerateFirstThenReplay
            | RollbackPathClass::ManualReviewRequiredNoAutomaticPath
    )
}

/// Closed validation-finding vocabulary for the fallback packet.
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
    /// A lane claiming certified is missing an apply-posture admission.
    MissingApplyPostureCoverage,
    /// A lane claiming certified is missing an impact-packet admission.
    MissingImpactPacketCoverage,
    /// A lane claiming certified is missing a reviewer-hint admission.
    MissingReviewerHintCoverage,
    /// A lane claiming certified is missing a rollback-path admission.
    MissingRollbackPathCoverage,
    /// A lane claiming certified is missing a support-export parity admission.
    MissingSupportExportParityCoverage,
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
    /// An apply-posture admission row has no bound posture.
    MissingApplyPostureClass,
    /// An apply-posture admission row has no bound target scope.
    MissingTargetScopeClass,
    /// An apply-posture admission row has no typed completeness label.
    MissingScopeCompletenessLabel,
    /// A reviewer-hint admission row has no bound reviewer hint.
    MissingReviewerHintClass,
    /// A rollback-path admission row has no bound route.
    MissingRollbackPathClass,
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
    /// An apply-posture admission row drops its posture binding.
    ApplyPostureNotApplicable,
    /// A non-apply-posture row binds an apply posture.
    ApplyPostureNotPermittedOnRowClass,
    /// A non-apply-posture row binds a target scope.
    TargetScopeNotPermittedOnRowClass,
    /// A reviewer-hint admission row drops its hint binding.
    ReviewerHintNotApplicable,
    /// A non-reviewer-hint row binds a reviewer hint.
    ReviewerHintNotPermittedOnRowClass,
    /// A rollback-path admission row drops its route binding.
    RollbackPathNotApplicable,
    /// A non-rollback-path row binds a rollback route.
    RollbackPathNotPermittedOnRowClass,
    /// A provider-disagreement admission row drops its visibility binding.
    DisagreementVisibilityNotApplicable,
    /// A non-provider-disagreement row binds a disagreement visibility.
    DisagreementVisibilityNotPermittedOnRowClass,
    /// An apply-posture admission offers apply-all on the live workspace while
    /// confidence, completeness, or scope is below the frozen threshold.
    UnsafeApplyAllBelowThreshold,
    /// An apply-posture admission labels the preview complete while leaving
    /// targets out of scope.
    ScopeCompletenessOverclaimed,
    /// An impact-packet admission documents no impacted targets.
    EmptyImpactPacket,
    /// An impact-packet admission documents impacted targets with no impact
    /// summary.
    MissingImpactSummary,
    /// An impact-packet admission documents impacted targets but exports no
    /// impact-packet ref.
    MissingImpactPacketRef,
    /// The impact packet drops the missing-scope explanation for a lane whose
    /// apply posture left targets out of scope.
    ImpactPacketDropsMissingScope,
    /// A reviewer-hint admission routes to a reviewer but exports no review
    /// anchor ref.
    MissingReviewAnchorRef,
    /// A reviewer-hint admission routes to a reviewer but attaches no owner
    /// hint.
    MissingOwnerHint,
    /// A rollback-path admission claims an automatic route but exports no
    /// checkpoint ref.
    MissingCheckpointRef,
    /// A writing fallback runs with no safe rollback path.
    WritingFallbackWithoutSafeRollback,
    /// A support-export parity admission drops the refactor lineage.
    SupportExportDropsLineage,
    /// A support-export parity admission drops the missing-scope explanation.
    SupportExportDropsMissingScope,
    /// A support-export parity admission exports no lineage ref.
    MissingLineageRef,
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
    /// A consumer projection remints or drops fallback truth.
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
    /// A projection collapses the confidence vocabulary.
    ConfidenceVocabularyCollapsed,
    /// A projection collapses the apply-posture vocabulary.
    ApplyPostureVocabularyCollapsed,
    /// A projection collapses the reviewer-hint vocabulary.
    ReviewerHintVocabularyCollapsed,
    /// A projection collapses the rollback-path vocabulary.
    RollbackPathVocabularyCollapsed,
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
            Self::MissingApplyPostureCoverage => "missing_apply_posture_coverage",
            Self::MissingImpactPacketCoverage => "missing_impact_packet_coverage",
            Self::MissingReviewerHintCoverage => "missing_reviewer_hint_coverage",
            Self::MissingRollbackPathCoverage => "missing_rollback_path_coverage",
            Self::MissingSupportExportParityCoverage => "missing_support_export_parity_coverage",
            Self::MissingProviderDisagreementCoverage => "missing_provider_disagreement_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingEngineIdentity => "missing_engine_identity",
            Self::MissingEngineIdentityLabel => "missing_engine_identity_label",
            Self::MissingRefactorClass => "missing_refactor_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::MissingApplyPostureClass => "missing_apply_posture_class",
            Self::MissingTargetScopeClass => "missing_target_scope_class",
            Self::MissingScopeCompletenessLabel => "missing_scope_completeness_label",
            Self::MissingReviewerHintClass => "missing_reviewer_hint_class",
            Self::MissingRollbackPathClass => "missing_rollback_path_class",
            Self::MissingDisagreementVisibilityClass => "missing_disagreement_visibility_class",
            Self::CertifiedWithUnboundBinding => "certified_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::ApplyPostureNotApplicable => "apply_posture_not_applicable",
            Self::ApplyPostureNotPermittedOnRowClass => "apply_posture_not_permitted_on_row_class",
            Self::TargetScopeNotPermittedOnRowClass => "target_scope_not_permitted_on_row_class",
            Self::ReviewerHintNotApplicable => "reviewer_hint_not_applicable",
            Self::ReviewerHintNotPermittedOnRowClass => "reviewer_hint_not_permitted_on_row_class",
            Self::RollbackPathNotApplicable => "rollback_path_not_applicable",
            Self::RollbackPathNotPermittedOnRowClass => "rollback_path_not_permitted_on_row_class",
            Self::DisagreementVisibilityNotApplicable => "disagreement_visibility_not_applicable",
            Self::DisagreementVisibilityNotPermittedOnRowClass => {
                "disagreement_visibility_not_permitted_on_row_class"
            }
            Self::UnsafeApplyAllBelowThreshold => "unsafe_apply_all_below_threshold",
            Self::ScopeCompletenessOverclaimed => "scope_completeness_overclaimed",
            Self::EmptyImpactPacket => "empty_impact_packet",
            Self::MissingImpactSummary => "missing_impact_summary",
            Self::MissingImpactPacketRef => "missing_impact_packet_ref",
            Self::ImpactPacketDropsMissingScope => "impact_packet_drops_missing_scope",
            Self::MissingReviewAnchorRef => "missing_review_anchor_ref",
            Self::MissingOwnerHint => "missing_owner_hint",
            Self::MissingCheckpointRef => "missing_checkpoint_ref",
            Self::WritingFallbackWithoutSafeRollback => "writing_fallback_without_safe_rollback",
            Self::SupportExportDropsLineage => "support_export_drops_lineage",
            Self::SupportExportDropsMissingScope => "support_export_drops_missing_scope",
            Self::MissingLineageRef => "missing_lineage_ref",
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
            Self::ConfidenceVocabularyCollapsed => "confidence_vocabulary_collapsed",
            Self::ApplyPostureVocabularyCollapsed => "apply_posture_vocabulary_collapsed",
            Self::ReviewerHintVocabularyCollapsed => "reviewer_hint_vocabulary_collapsed",
            Self::RollbackPathVocabularyCollapsed => "rollback_path_vocabulary_collapsed",
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

/// One fallback row binding an artifact-family lane to the engine, refactor
/// class, apply posture, target scope, completeness, confidence, impact packet,
/// reviewer hint, rollback path, support-export parity, and disagreement
/// visibility its safe fallback posture may claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Artifact-family lane this row certifies.
    pub lane_class: ArtifactFamilyLaneClass,
    /// Row class.
    pub row_class: FallbackRowClass,
    /// Stable refactor (transaction) id the lane's rows share.
    pub refactor_id: String,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Acting engine family (or `not_applicable`).
    pub acting_provider_class: ProviderFamilyClass,
    /// Refactor class (or `not_applicable`).
    pub refactor_class: RefactorTransactionClass,
    /// Apply posture (or `not_applicable`).
    pub apply_posture_class: ApplyFallbackPostureClass,
    /// Target scope co-bound on the apply-posture row (or `not_applicable`).
    pub target_scope_class: MutationScopeClass,
    /// Typed scope-completeness label co-bound on the apply-posture row.
    pub scope_completeness_class: CompletenessClass,
    /// Confidence tier for the row; the apply-posture gate reads the
    /// apply-posture row's tier.
    pub confidence_class: ConfidenceClass,
    /// Count of targets left out of the transform scope (the missing-scope set).
    #[serde(default)]
    pub missing_scope_count: u32,
    /// Count of impacted targets the impact packet documents.
    #[serde(default)]
    pub impacted_target_count: u32,
    /// Count of impacted owners the impact packet documents.
    #[serde(default)]
    pub impacted_owner_count: u32,
    /// True when an impact summary is attached to the impact packet.
    #[serde(default)]
    pub impact_summary_present: bool,
    /// True when a missing-scope explanation is attached to the impact packet.
    #[serde(default)]
    pub missing_scope_explanation_present: bool,
    /// Reviewer / owner routing hint (or `not_applicable`).
    pub reviewer_hint_class: ReviewerHintClass,
    /// True when an owner hint is attached to the reviewer-hint row.
    #[serde(default)]
    pub owner_hint_present: bool,
    /// Rollback path (or `not_applicable`).
    pub rollback_path_class: RollbackPathClass,
    /// True when the support-export parity row preserves the refactor lineage.
    #[serde(default)]
    pub preserves_refactor_lineage: bool,
    /// True when the support-export parity row preserves the missing-scope
    /// explanation.
    #[serde(default)]
    pub preserves_missing_scope_explanation: bool,
    /// Provider-disagreement visibility (or `not_applicable`).
    pub disagreement_visibility_class: DisagreementVisibilityClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
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
    /// Opaque impact-packet ref. Required on an impact-packet row that documents
    /// impacted targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_packet_ref: Option<String>,
    /// Opaque review-anchor ref. Required on a reviewer-hint row that routes to
    /// a reviewer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_anchor_ref: Option<String>,
    /// Opaque rollback checkpoint ref. Required on a rollback-path row whose
    /// route is an automatic checkpoint route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Opaque lineage ref the support-export parity row exports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_ref: Option<String>,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl FallbackRow {
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
pub struct FallbackConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Fallback packet id consumed by the projection.
    pub fallback_packet_id_ref: String,
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
    /// True when the confidence vocabulary is preserved verbatim.
    pub preserves_confidence_vocabulary: bool,
    /// True when the apply-posture vocabulary is preserved verbatim.
    pub preserves_apply_posture_vocabulary: bool,
    /// True when the reviewer-hint vocabulary is preserved verbatim.
    pub preserves_reviewer_hint_vocabulary: bool,
    /// True when the rollback-path vocabulary is preserved verbatim.
    pub preserves_rollback_path_vocabulary: bool,
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

impl FallbackConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.fallback_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_engine_identity_vocabulary
            && self.preserves_refactor_class_vocabulary
            && self.preserves_target_scope_vocabulary
            && self.preserves_scope_completeness_vocabulary
            && self.preserves_confidence_vocabulary
            && self.preserves_apply_posture_vocabulary
            && self.preserves_reviewer_hint_vocabulary
            && self.preserves_rollback_path_vocabulary
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

/// Constructor input for [`WideScopeRefactorFallbackTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WideScopeRefactorFallbackTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Artifact-family lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ArtifactFamilyLaneClass>,
    /// Fallback rows.
    #[serde(default)]
    pub rows: Vec<FallbackRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<FallbackConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying the safe fallback posture that a wide-scope
/// or low-confidence transform takes across the M5 framework, notebook, docs,
/// request, config, and generated lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WideScopeRefactorFallbackTruthPacket {
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
    /// Fallback rows.
    #[serde(default)]
    pub rows: Vec<FallbackRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<FallbackConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl WideScopeRefactorFallbackTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: WideScopeRefactorFallbackTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable fallback invariants.
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

    /// Returns the unique apply-posture tokens observed across rows.
    pub fn apply_posture_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.apply_posture_class.as_str())
    }

    /// Returns the unique target-scope tokens observed across rows.
    pub fn target_scope_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.target_scope_class.as_str())
    }

    /// Returns the unique scope-completeness tokens observed across rows.
    pub fn scope_completeness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.scope_completeness_class.as_str())
    }

    /// Returns the unique confidence tokens observed across rows.
    pub fn confidence_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.confidence_class.as_str())
    }

    /// Returns the unique reviewer-hint tokens observed across rows.
    pub fn reviewer_hint_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.reviewer_hint_class.as_str())
    }

    /// Returns the unique rollback-path tokens observed across rows.
    pub fn rollback_path_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.rollback_path_class.as_str())
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

    fn unique_tokens(&self, project: impl Fn(&FallbackRow) -> &'static str) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(project(row));
        }
        set.into_iter().collect()
    }

    /// Builds a support export wrapping the exact packet shown to product
    /// surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> WideScopeRefactorFallbackTruthSupportExport {
        WideScopeRefactorFallbackTruthSupportExport {
            record_kind: WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            fallback_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            fallback_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "fallback packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "fallback packet has the wrong schema version",
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
            self.append_per_lane_safety_findings(*lane, &mut findings);
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

    fn append_per_row_findings(&self, row: &FallbackRow, findings: &mut Vec<ValidationFinding>) {
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
                && !FallbackRow::has_label(&row.engine_identity_label)
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
        self.append_row_safety_findings(row, findings);
    }

    fn append_dimension_gating_findings(
        &self,
        row: &FallbackRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let is_posture = matches!(row.row_class, FallbackRowClass::ApplyPostureAdmission);
        let is_reviewer = matches!(row.row_class, FallbackRowClass::ReviewerHintAdmission);
        let is_rollback = matches!(row.row_class, FallbackRowClass::RollbackPathAdmission);
        let is_disagreement = matches!(
            row.row_class,
            FallbackRowClass::ProviderDisagreementAdmission
        );

        // Apply-posture dimension (owner co-binds posture, scope, completeness).
        if is_posture {
            if !row.apply_posture_class.is_concrete() {
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
            if !row.target_scope_class.is_concrete() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingTargetScopeClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an apply_posture_admission but has no bound target scope",
                        row.row_id
                    ),
                ));
            }
            if !row.scope_completeness_class.is_concrete() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingScopeCompletenessLabel,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an apply_posture_admission but carries no typed completeness label",
                        row.row_id
                    ),
                ));
            }
        }
        if !is_posture && !row.apply_posture_class.is_inactive() {
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
        if !is_posture && !row.target_scope_class.is_inactive() {
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

        // Reviewer-hint dimension.
        if is_reviewer && !row.reviewer_hint_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingReviewerHintClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound reviewer hint", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::ReviewerHintNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a reviewer_hint_admission but has no bound hint",
                    row.row_id
                ),
            ));
        }
        if !is_reviewer && !row.reviewer_hint_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ReviewerHintNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds reviewer hint {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.reviewer_hint_class.as_str()
                ),
            ));
        }

        // Rollback-path dimension.
        if is_rollback && !row.rollback_path_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRollbackPathClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound rollback route", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::RollbackPathNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a rollback_path_admission but has no bound route",
                    row.row_id
                ),
            ));
        }
        if !is_rollback && !row.rollback_path_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackPathNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds rollback route {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.rollback_path_class.as_str()
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

    fn append_row_safety_findings(&self, row: &FallbackRow, findings: &mut Vec<ValidationFinding>) {
        // Apply-posture safety: apply-all on the live workspace is only ever
        // permitted for a narrow, complete, high-confidence transform.
        if matches!(row.row_class, FallbackRowClass::ApplyPostureAdmission)
            && row.apply_posture_class.is_apply_all_on_live()
        {
            let confident = matches!(row.confidence_class, ConfidenceClass::HighConfidence);
            let complete = matches!(row.scope_completeness_class, CompletenessClass::Complete);
            let narrow = !scope_is_wide(row.target_scope_class);
            if !(confident && complete && narrow) {
                findings.push(ValidationFinding::new(
                    FindingKind::UnsafeApplyAllBelowThreshold,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} offers apply-all on the live workspace while confidence ({}), completeness ({}), or scope ({}) is below the frozen threshold",
                        row.row_id,
                        row.confidence_class.as_str(),
                        row.scope_completeness_class.as_str(),
                        row.target_scope_class.as_str()
                    ),
                ));
            }
        }

        // Apply-posture safety: a preview may not claim complete while leaving
        // targets out of scope.
        if matches!(row.row_class, FallbackRowClass::ApplyPostureAdmission)
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

        // Impact-packet safety: a real impact packet documents at least one
        // impacted target and attaches an impact summary and a packet ref.
        if matches!(row.row_class, FallbackRowClass::ImpactPacketAdmission) {
            if row.impacted_target_count == 0 {
                findings.push(ValidationFinding::new(
                    FindingKind::EmptyImpactPacket,
                    FindingSeverity::Blocker,
                    format!("row {} documents no impacted targets", row.row_id),
                ));
            } else {
                if !row.impact_summary_present {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingImpactSummary,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} documents impacted targets but attaches no impact summary",
                            row.row_id
                        ),
                    ));
                }
                if !FallbackRow::has_label(&row.impact_packet_ref) {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingImpactPacketRef,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} documents impacted targets but exports no impact-packet ref",
                            row.row_id
                        ),
                    ));
                }
            }
        }

        // Reviewer-hint safety: a hint that routes to a reviewer exports a
        // review anchor and carries an owner hint.
        if matches!(row.row_class, FallbackRowClass::ReviewerHintAdmission)
            && row.reviewer_hint_class.requires_review_anchor()
        {
            if !FallbackRow::has_label(&row.review_anchor_ref) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingReviewAnchorRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} routes to a reviewer but exports no review-anchor ref",
                        row.row_id
                    ),
                ));
            }
            if !row.owner_hint_present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingOwnerHint,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} routes to a reviewer but attaches no owner hint",
                        row.row_id
                    ),
                ));
            }
        }

        // Rollback-path safety: an automatic checkpoint route exports a
        // checkpoint ref.
        if matches!(row.row_class, FallbackRowClass::RollbackPathAdmission)
            && row.rollback_path_class.is_concrete()
            && rollback_requires_checkpoint_ref(row.rollback_path_class)
            && !FallbackRow::has_label(&row.checkpoint_ref)
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

        // Support-export parity safety: the support / export channel preserves
        // the refactor lineage and the missing-scope explanation and exports a
        // lineage ref.
        if matches!(
            row.row_class,
            FallbackRowClass::SupportExportParityAdmission
        ) {
            if !row.preserves_refactor_lineage {
                findings.push(ValidationFinding::new(
                    FindingKind::SupportExportDropsLineage,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} drops the refactor lineage from support/export",
                        row.row_id
                    ),
                ));
            }
            if !row.preserves_missing_scope_explanation {
                findings.push(ValidationFinding::new(
                    FindingKind::SupportExportDropsMissingScope,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} drops the missing-scope explanation from support/export",
                        row.row_id
                    ),
                ));
            }
            if !FallbackRow::has_label(&row.lineage_ref) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLineageRef,
                    FindingSeverity::Blocker,
                    format!("row {} exports no refactor-lineage ref", row.row_id),
                ));
            }
        }

        // Provider disagreement must never collapse the loser into ranking-only.
        if matches!(
            row.row_class,
            FallbackRowClass::ProviderDisagreementAdmission
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
                && matches!(row.row_class, FallbackRowClass::FallbackLaneQuality)
                && matches!(row.support_class, SupportClass::Certified)
        });
        if !lane_claims_stable {
            return;
        }

        let required: [(FallbackRowClass, FindingKind, &str); 6] = [
            (
                FallbackRowClass::ApplyPostureAdmission,
                FindingKind::MissingApplyPostureCoverage,
                "apply_posture_admission",
            ),
            (
                FallbackRowClass::ImpactPacketAdmission,
                FindingKind::MissingImpactPacketCoverage,
                "impact_packet_admission",
            ),
            (
                FallbackRowClass::ReviewerHintAdmission,
                FindingKind::MissingReviewerHintCoverage,
                "reviewer_hint_admission",
            ),
            (
                FallbackRowClass::RollbackPathAdmission,
                FindingKind::MissingRollbackPathCoverage,
                "rollback_path_admission",
            ),
            (
                FallbackRowClass::SupportExportParityAdmission,
                FindingKind::MissingSupportExportParityCoverage,
                "support_export_parity_admission",
            ),
            (
                FallbackRowClass::ProviderDisagreementAdmission,
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

    fn append_per_lane_safety_findings(
        &self,
        lane: ArtifactFamilyLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let posture_row = self.rows.iter().find(|row| {
            row.lane_class == lane
                && matches!(row.row_class, FallbackRowClass::ApplyPostureAdmission)
        });
        let Some(posture_row) = posture_row else {
            return;
        };

        let writes = posture_row.apply_posture_class.is_concrete()
            && posture_row.apply_posture_class.writes_source();
        let lane_has_missing_scope = posture_row.missing_scope_count > 0;

        // A writing fallback must carry a safe rollback path.
        if writes {
            if let Some(rollback_row) = self.rows.iter().find(|row| {
                row.lane_class == lane
                    && matches!(row.row_class, FallbackRowClass::RollbackPathAdmission)
                    && row.rollback_path_class.is_concrete()
            }) {
                if !rollback_is_safe(rollback_row.rollback_path_class) {
                    findings.push(ValidationFinding::new(
                        FindingKind::WritingFallbackWithoutSafeRollback,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} writes source under posture {} but its rollback route {} offers no safe recovery",
                            lane.as_str(),
                            posture_row.apply_posture_class.as_str(),
                            rollback_row.rollback_path_class.as_str()
                        ),
                    ));
                }
            }
        }

        // When the lane left targets out of scope, its impact packet must carry
        // the missing-scope explanation.
        if lane_has_missing_scope {
            if let Some(impact_row) = self.rows.iter().find(|row| {
                row.lane_class == lane
                    && matches!(row.row_class, FallbackRowClass::ImpactPacketAdmission)
            }) {
                if !impact_row.missing_scope_explanation_present {
                    findings.push(ValidationFinding::new(
                        FindingKind::ImpactPacketDropsMissingScope,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} left {} targets out of scope but its impact packet attaches no missing-scope explanation",
                            lane.as_str(),
                            posture_row.missing_scope_count
                        ),
                    ));
                }
            }
        }
    }

    fn append_projection_findings(
        &self,
        projection: &FallbackConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve fallback truth",
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
                projection.preserves_confidence_vocabulary,
                FindingKind::ConfidenceVocabularyCollapsed,
                "confidence",
            ),
            (
                projection.preserves_apply_posture_vocabulary,
                FindingKind::ApplyPostureVocabularyCollapsed,
                "apply-posture",
            ),
            (
                projection.preserves_reviewer_hint_vocabulary,
                FindingKind::ReviewerHintVocabularyCollapsed,
                "reviewer-hint",
            ),
            (
                projection.preserves_rollback_path_vocabulary,
                FindingKind::RollbackPathVocabularyCollapsed,
                "rollback-path",
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
pub struct WideScopeRefactorFallbackTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub fallback_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub fallback_packet: WideScopeRefactorFallbackTruthPacket,
}

impl WideScopeRefactorFallbackTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_VERSION
            && self.fallback_packet_id_ref == self.fallback_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.fallback_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable fallback packet.
#[derive(Debug)]
pub enum WideScopeRefactorFallbackTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for WideScopeRefactorFallbackTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(formatter, "fallback packet parse failed: {error}"),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "fallback packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for WideScopeRefactorFallbackTruthArtifactError {}

/// Returns the checked-in stable wide-scope refactor fallback truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or
/// validate.
pub fn current_stable_wide_scope_refactor_fallback_truth_packet(
) -> Result<WideScopeRefactorFallbackTruthPacket, WideScopeRefactorFallbackTruthArtifactError> {
    let packet: WideScopeRefactorFallbackTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m5/wide_scope_refactor_fallback_truth_packet.json"
    )))
    .map_err(WideScopeRefactorFallbackTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(WideScopeRefactorFallbackTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests;
