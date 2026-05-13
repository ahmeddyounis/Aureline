//! TypeScript and JavaScript launch-wedge assistance records.
//!
//! This module owns the first bounded TS/JS hover, definition, references, and
//! rename-preview alpha path. It consumes the shared language-router posture and
//! emits scoped, source-labeled records so editor, CLI, support, and future UI
//! surfaces do not have to infer provider health or rename safety from raw
//! provider output.

mod records;
mod wedge;

pub use records::{
    TsJsAccessKindClass, TsJsAmbiguityDescriptor, TsJsAnchorRef, TsJsAnswerLayerClass,
    TsJsApplyPostureClass, TsJsCheckpointClass, TsJsCompletenessClass,
    TsJsGeneratedOrExternalStateClass, TsJsHoverRecord, TsJsInlineVisibilityClass,
    TsJsLaunchWedgeSnapshot, TsJsOccurrenceSeed, TsJsProviderSnapshot, TsJsReferenceCountSummary,
    TsJsReferenceSetRecord, TsJsRelationClass, TsJsRenameAffectedScopeRow,
    TsJsRenameCheckpointDescriptor, TsJsRenameCountSummary, TsJsRenameCoverageLimitClass,
    TsJsRenameEvidenceBinding, TsJsRenamePreviewCompletenessClass, TsJsRenamePreviewRecord,
    TsJsRenamePreviewSchemaVersion, TsJsRenameWarningClass, TsJsRenameWarningRow,
    TsJsResultConfidenceClass, TsJsRollbackPathClass, TsJsScopeDescriptor,
    TsJsSemanticEvidenceBinding, TsJsSemanticResultIdentityClass, TsJsSemanticResultRecord,
    TsJsSemanticResultSchemaVersion, TsJsSourceAnchor, TsJsSourceAnchorKindClass,
    TsJsSymbolKindClass, TsJsSymbolSeed, TsJsWorkspaceContext, TSJS_NAV_ALPHA_SCHEMA_VERSION,
};
pub use wedge::{TsJsLaunchWedge, TsJsNavigationError};
