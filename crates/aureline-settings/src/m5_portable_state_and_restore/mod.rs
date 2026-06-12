//! Portable-state export/import, restore-provenance, and missing-dependency contract.
//!
//! The module mints one governed record for M5-owned portable artifacts. The
//! record names each artifact class's portability disposition (portable,
//! redacted, restore-only, or machine-local) with explicit exclusions for
//! secrets, live handles, and machine-unique state; carries restore-provenance
//! cards with schema-migration labels (exact, compatible, layout-only,
//! recovered-drafts, or evidence-only); and keeps missing-extension,
//! missing-remote-target, and unsupported-client dependencies visible as
//! placeholders instead of silently dropping the affected surface.

pub mod corpus;
pub mod model;

#[cfg(test)]
mod tests;

pub use corpus::{portable_state_restore_corpus, PortableStateRestoreScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, BuildError, ExclusionReason, M5PortableStateRestoreCertification,
    M5PortableStateRestoreInput, MigrationLabel, MissingDependencyKind,
    MissingDependencyPlaceholder, NarrowingReason, PortabilityDisposition, PortableArtifactClass,
    PortablePackageClassRow, PortableRestoreClaim, PortableRestorePillars,
    PortableRestoreQualification, RestoreProvenanceCard, SurfaceClass, SurfaceTruthRow,
    M5_PORTABLE_STATE_RESTORE_RECORD_KIND, M5_PORTABLE_STATE_RESTORE_SCHEMA_VERSION,
    M5_PORTABLE_STATE_RESTORE_SHARED_CONTRACT_REF,
};
