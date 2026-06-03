//! Template, starter, and prebuild entry disclosure stable truth.
//!
//! A [`TemplateStarterPrebuildEntryRecord`] is the single governed record that
//! Start Center, CLI / headless entry, docs, and support packets read **before**
//! a template, starter, or prebuild is applied. It discloses what the accelerator
//! is, where it came from, what it will do, what it will not do yet, and how to
//! bypass it — keeping entry disclosure honest and separate from scaffolding.
//!
//! The desktop shell, diagnostics, support exports, Help/About, and docs read
//! this record verbatim instead of cloning status text. The canonical artifacts
//! for this lane are:
//!
//! - [`model`] — the governed record, closed vocabularies, builder, and honesty
//!   invariants. Boundary schema:
//!   `schemas/ux/template-prebuild-entry.schema.json`.
//! - [`corpus`] — deterministic drill corpus pinned under
//!   `fixtures/ux/m4/stabilize-template-starter-prebuild-entry/`.
//!
//! The contract narrative is
//! `docs/ux/m4/stabilize-template-starter-prebuild-entry.md`; the release-evidence
//! packet is `artifacts/ux/m4/stabilize-template-starter-prebuild-entry.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    template_starter_prebuild_entry_corpus, TemplateStarterPrebuildEntryScenario, CORPUS_AS_OF,
};
pub use model::{
    AcceleratorIdentity, BuildError, BypassPath, BypassPathClass, CleanupRollback,
    CredentialProvisioningClass, EntryKind, ExtensionInstallClass, FailureOutcomeClass,
    FailureSummary, FreshnessClass, FreshnessReview, HostBoundaryClass, ManagedServiceClass,
    NetworkEgressClass, RemoteProvisioningClass, ResultingMode, RuntimeReview, RuntimeScopeClass,
    SetupActionClass, SetupReview, SideEffectEnvelope, SourceClass, SourceReview, SupportClass,
    SupportExportMetadata, SupportReview, TemplateStarterPrebuildEntryRecord, TrustAuthBoundaries,
    TrustPostureClass, TEMPLATE_STARTER_PREBUILD_ENTRY_NOTICE,
    TEMPLATE_STARTER_PREBUILD_ENTRY_RECORD_KIND, TEMPLATE_STARTER_PREBUILD_ENTRY_SCHEMA_VERSION,
    TEMPLATE_STARTER_PREBUILD_ENTRY_SHARED_CONTRACT_REF,
};
