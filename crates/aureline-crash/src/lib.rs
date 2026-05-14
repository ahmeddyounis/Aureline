//! Crash incident trails for alpha supportability.
//!
//! This crate owns the first runtime-shaped contract that joins a crash
//! envelope, exact-build symbolication status, trace IDs, and support-bundle
//! manifest linkage into one redaction-safe incident trail. The trail is
//! intentionally metadata-first: raw dumps, raw memory, raw stack bodies,
//! command lines, paths, and secrets stay out of the record.

#![doc(html_root_url = "https://docs.rs/aureline-crash/0.0.0")]

pub mod incident_trail;

pub use incident_trail::{
    CrashDumpManifest, CrashEnvelope, CrashIncidentTrail, CrashIncidentTrailInputs, CrashModule,
    IncidentEvidenceKind, IncidentEvidenceRef, ModuleIncidentSummary, ModuleMappingQuality,
    NextSafeAction, NextSafeActionKind, SupportBundleLinkage, SupportBundleLinkageState,
    SymbolicatedModuleResult, SymbolicationReport, SymbolicationState,
    CRASH_INCIDENT_TRAIL_RECORD_KIND, CRASH_INCIDENT_TRAIL_SCHEMA_VERSION,
};
