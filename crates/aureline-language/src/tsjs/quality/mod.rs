//! TypeScript and JavaScript quality-tool seed flows.
//!
//! This module owns the bounded formatter, linter, and test-adapter alpha path
//! for the TS/JS launch wedge. It normalizes quality findings into the shared
//! diagnostic bus and exposes execution-plane task hooks so UI, CLI, support,
//! and replay surfaces consume one provenance model.

mod records;
mod wedge;

pub use records::{
    TsJsQualityActionClass, TsJsQualityAggregateCounts, TsJsQualityAlphaSchemaVersion,
    TsJsQualityDiagnosticSeed, TsJsQualityExecutionPlaneProjection, TsJsQualityExecutionTaskHook,
    TsJsQualityPreviewRequirementClass, TsJsQualityRerunPostureClass, TsJsQualitySafetyClass,
    TsJsQualitySeedSnapshot, TsJsQualitySnapshot, TsJsQualitySnapshotRequest,
    TsJsQualityTaskHookSeed, TsJsQualityToolKindClass, TsJsQualityToolStatusRow,
    TsJsQualityTriggerClass, TSJS_QUALITY_ALPHA_SCHEMA_VERSION,
};
pub use wedge::TsJsQualityWedge;
