//! Crash incident trails for alpha supportability.
//!
//! This crate owns the first runtime-shaped contract that joins a crash
//! envelope, exact-build symbolication status, trace IDs, and support-bundle
//! manifest linkage into one redaction-safe incident trail. The trail is
//! intentionally metadata-first: raw dumps, raw memory, raw stack bodies,
//! command lines, paths, and secrets stay out of the record.

#![doc(html_root_url = "https://docs.rs/aureline-crash/0.0.0")]

pub mod envelope;
pub mod incident_trail;
pub mod symbolication;

pub use envelope::{
    bind_crash_envelope, CrashEnvelopeBindingInputs, CrashEnvelopeSymbolBinding,
    ManifestArtifactFamilyClass, ManifestModuleKind, ManifestRedactionClass, ManifestStorageClass,
    ModuleBindingRow, ModuleBindingState, ReleaseChannelClass, SupportExportPostureClass,
    SymbolBindingState, SymbolManifest, SymbolManifestModule,
    CRASH_ENVELOPE_SYMBOL_BINDING_RECORD_KIND, SYMBOL_MANIFEST_DOC_REF,
    SYMBOL_MANIFEST_RECORD_KIND, SYMBOL_MANIFEST_SCHEMA_REF, SYMBOL_MANIFEST_SCHEMA_VERSION,
};
pub use incident_trail::{
    CrashDumpManifest, CrashEnvelope, CrashFrame, CrashIncidentTrail, CrashIncidentTrailInputs,
    CrashModule, CrashModuleIdentity, IncidentEvidenceKind, IncidentEvidenceRef,
    ModuleIncidentSummary, ModuleMappingQuality, NextSafeAction, NextSafeActionKind,
    SupportBundleLinkage, SupportBundleLinkageState, SymbolicatedModuleResult, SymbolicationReport,
    SymbolicationState, CRASH_INCIDENT_TRAIL_RECORD_KIND, CRASH_INCIDENT_TRAIL_SCHEMA_VERSION,
};
pub use symbolication::{
    symbolicate_exact_build, ExactBuildSymbolicationError, ExactBuildSymbolicationInput,
    InTreeSymbolFile, InTreeSymbolFrame, InTreeSymbolModule, SYMBOLICATION_REPORT_RECORD_KIND,
};
