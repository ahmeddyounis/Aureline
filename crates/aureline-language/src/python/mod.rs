//! Python launch-wedge assistance records.
//!
//! This module owns the first bounded Python hover, definition, references, and
//! rename-preview alpha path. It consumes the shared language-router posture and
//! emits scoped, source-labeled records so editor, CLI, support, and future UI
//! surfaces do not have to infer provider health or rename safety from raw
//! provider output.

mod records;
mod wedge;

pub use records::{
    PythonAccessKindClass, PythonAmbiguityDescriptor, PythonAnchorRef, PythonAnswerLayerClass,
    PythonApplyPostureClass, PythonCheckpointClass, PythonCompletenessClass,
    PythonEnvironmentManagerClass, PythonGeneratedOrExternalStateClass, PythonHoverRecord,
    PythonInlineVisibilityClass, PythonInterpreterContext, PythonInterpreterReadinessClass,
    PythonInterpreterSelectionStateClass, PythonLaunchWedgeSnapshot, PythonOccurrenceSeed,
    PythonProviderSnapshot, PythonReferenceCountSummary, PythonReferenceSetRecord,
    PythonRelationClass, PythonRenameAffectedScopeRow, PythonRenameCheckpointDescriptor,
    PythonRenameCountSummary, PythonRenameCoverageLimitClass, PythonRenameEvidenceBinding,
    PythonRenamePreviewCompletenessClass, PythonRenamePreviewRecord,
    PythonRenamePreviewSchemaVersion, PythonRenameWarningClass, PythonRenameWarningRow,
    PythonResultConfidenceClass, PythonRollbackPathClass, PythonScopeDescriptor,
    PythonSemanticEvidenceBinding, PythonSemanticResultIdentityClass, PythonSemanticResultRecord,
    PythonSemanticResultSchemaVersion, PythonSourceAnchor, PythonSourceAnchorKindClass,
    PythonSymbolKindClass, PythonSymbolSeed, PythonWorkspaceContext,
    PYTHON_NAV_ALPHA_SCHEMA_VERSION,
};
pub use wedge::{PythonLaunchWedge, PythonNavigationError};
