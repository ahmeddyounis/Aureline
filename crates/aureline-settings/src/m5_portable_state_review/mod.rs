//! Portable-state export/import review sheets, redaction manifests, producer
//! provenance, and compare-before-restore.
//!
//! This module mints one governed record for the *review* shown before an
//! M5-owned portable-state package is exported or before its contents are
//! restored. The record names each selected artifact class's data-class label
//! (local-only, portable, shared, redacted, or machine-local) with explicit
//! exclusion reasons; carries a redaction manifest naming what was stripped and
//! how; records producer/build provenance and schema versions; and — for an
//! import — carries a compare-before-restore summary of added/removed/changed
//! panes and surfaces, missing dependency classes, excluded secrets/handles, and
//! path/host redaction.
//!
//! It reuses the portability vocabulary from
//! [`crate::m5_portable_state_and_restore`] rather than inventing surface-local
//! language, and its fail-closed gate makes a dishonest review sheet
//! unconstructable: secrets never cross as portable/shared, exclusions are never
//! silently dropped, redacted classes always carry a manifest entry, and an
//! import review always carries its comparison.

pub mod corpus;
pub mod model;

#[cfg(test)]
mod tests;

pub use corpus::{portable_state_review_corpus, PortableStateReviewScenario, CORPUS_AS_OF};
pub use model::{
    BuildError, ChangeCounts, ChecksumState, CompareSummary, DataClassLabel, HostProvenanceClass,
    M5PortableStateReviewInput, M5PortableStateReviewSheet, ProducerProvenance,
    RedactionManifestEntry, RedactionTechnique, ReviewClassRow, ReviewConsumerSurface,
    ReviewDirection, ReviewNarrowingReason, ReviewPillars, ReviewQualification, ReviewReadiness,
    ReviewSurfaceRow, SignatureState, M5_PORTABLE_STATE_REVIEW_RECORD_KIND,
    M5_PORTABLE_STATE_REVIEW_SCHEMA_VERSION, M5_PORTABLE_STATE_REVIEW_SHARED_CONTRACT_REF,
};
