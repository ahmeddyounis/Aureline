//! TypeScript and JavaScript launch-wedge assistance records.
//!
//! This module owns the first bounded TS/JS hover, definition, references,
//! rename-preview, formatter, linter, and test-adapter alpha paths. It consumes
//! the shared language-router and diagnostic-bus postures and emits scoped,
//! source-labeled records so editor, CLI, support, and future UI surfaces do not
//! have to infer provider health, quality-tool state, or mutation safety from
//! raw provider output.

pub mod quality;
mod records;
mod wedge;

pub use quality::{
    TsJsQualityActionClass, TsJsQualityAggregateCounts, TsJsQualityAlphaSchemaVersion,
    TsJsQualityDiagnosticSeed, TsJsQualityExecutionPlaneProjection, TsJsQualityExecutionTaskHook,
    TsJsQualityPreviewRequirementClass, TsJsQualityRerunPostureClass, TsJsQualitySafetyClass,
    TsJsQualitySeedSnapshot, TsJsQualitySnapshot, TsJsQualitySnapshotRequest,
    TsJsQualityTaskHookSeed, TsJsQualityToolKindClass, TsJsQualityToolStatusRow,
    TsJsQualityTriggerClass, TsJsQualityWedge, TSJS_QUALITY_ALPHA_SCHEMA_VERSION,
};
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
