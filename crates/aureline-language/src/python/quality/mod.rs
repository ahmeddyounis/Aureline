//! Python quality-tool seed flows.
//!
//! This module owns the bounded formatter, linter, and test-adapter alpha path
//! for the Python launch wedge. It normalizes quality findings into the shared
//! diagnostic bus and exposes execution-plane task hooks that preserve Python
//! interpreter selection, tool availability, preview posture, and rerun truth.

mod records;
mod wedge;

pub use records::{
    PythonQualityActionClass, PythonQualityAggregateCounts, PythonQualityAlphaSchemaVersion,
    PythonQualityDiagnosticSeed, PythonQualityExecutionPlaneProjection,
    PythonQualityExecutionTaskHook, PythonQualityPreviewRequirementClass,
    PythonQualityRerunPostureClass, PythonQualitySafetyClass, PythonQualitySeedSnapshot,
    PythonQualitySnapshot, PythonQualitySnapshotRequest, PythonQualityTaskHookSeed,
    PythonQualityToolKindClass, PythonQualityToolStatusRow, PythonQualityTriggerClass,
    PYTHON_QUALITY_ALPHA_SCHEMA_VERSION,
};
pub use wedge::PythonQualityWedge;
