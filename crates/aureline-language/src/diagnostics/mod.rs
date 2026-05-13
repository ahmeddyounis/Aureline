//! Diagnostic bus records and launch-language diagnostic projections.
//!
//! This module owns the alpha diagnostic bus for language-derived findings.
//! The bus normalizes compiler, language-server, linter, scanner-import,
//! cached, and partial diagnostics into one envelope while reusing the
//! language-router provider health, scope, locality, and redaction vocabulary.

mod bus;
mod records;

pub use bus::{DiagnosticBus, DiagnosticBusSnapshotRequest};
pub use records::{
    DiagnosticAnchor, DiagnosticAnchorRemapStateClass, DiagnosticBusAggregateCounts,
    DiagnosticBusSchemaVersion, DiagnosticBusSnapshot, DiagnosticEnvelope,
    DiagnosticEvidencePlaneClass, DiagnosticEvidenceRef, DiagnosticEvidenceRoleClass,
    DiagnosticFreshness, DiagnosticFreshnessClass, DiagnosticOriginClass,
    DiagnosticProviderAvailabilityRow, DiagnosticScope, DiagnosticSeverityClass,
    DiagnosticSourceDescriptor, DiagnosticSourceFamily, DiagnosticSurfaceClass,
    DiagnosticSurfaceProjection, DIAGNOSTIC_BUS_SCHEMA_VERSION,
};
