//! Code-action and quick-fix alpha records.
//!
//! This module owns the first runtime contract for language-derived repairs
//! and source actions. It consumes diagnostic provenance and provider
//! freshness, then exposes side-effect classes, preview admission, named undo
//! groups, and surface projections without applying edits directly.

mod records;

pub use records::{
    ActionClass, ApplyPostureClass, BlockingReasonClass, CodeActionAdmissionRecord,
    CodeActionAlphaAggregateCounts, CodeActionAlphaSchemaVersion, CodeActionAlphaSnapshot,
    CodeActionCatalog, CodeActionContentIntegrityReview, CodeActionContractError,
    CodeActionEpochBinding, CodeActionEpochRoleClass, CodeActionFreshnessClass,
    CodeActionMutationCounts, CodeActionPolicyContext, CodeActionProviderDescriptor,
    CodeActionRecord, CodeActionRefactorScopeBinding, CodeActionSafetyClass,
    CodeActionScopeWideningReview, CodeActionSideEffectClass, CodeActionSnapshotRequest,
    CodeActionSourceKindClass, CodeActionSupportClass, CodeActionSurfaceClass,
    CodeActionSurfaceProjection, CodeActionTrustState, CodeActionUndoGroup,
    CodeActionValidationPlan, MutationScopeClass, PreviewRequirementClass,
    RefactorScopeAdmissionClass, RefactorScopeCandidate, RefactorScopeTargetRow, ReplayHintClass,
    ScopeWideningReviewTriggerClass, SemanticLayerStateClass, UndoReversalClass,
    ValidationHintClass, CODE_ACTION_ALPHA_SCHEMA_VERSION,
};
