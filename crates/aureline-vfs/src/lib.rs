//! Virtual filesystem, watcher, canonical-path, alias-set, and
//! save-target prototype.
//!
//! This prototype validates the five-layer filesystem-identity
//! model, watcher posture, conflict-aware save pipeline, and
//! root-capability envelope frozen in
//! `docs/adr/0006-vfs-save-cache-identity.md` and in the cross-
//! surface vocabulary at
//! `docs/filesystem/filesystem_identity_vocabulary.md`. The goal
//! is contract correctness: trigger the exact outcomes and hooks
//! the ADR names against a reviewable synthetic-root fixture
//! table, without taking a dependency on platform-specific
//! filesystem quirks.
//!
//! Five pieces sit behind this crate's public surface:
//!
//! - [`identity`] — the five identity layers (presentation path,
//!   logical workspace identity, canonical filesystem object,
//!   alias set, save-target token) and the frozen token-kind
//!   vocabulary.
//! - [`capabilities`] — the root-capability envelope (flags, root
//!   class, permitted save modes, watcher source, strongest /
//!   fallback identity token kinds) advertised at root attach.
//! - [`synthetic`] — an in-memory model of a workspace root that
//!   owns canonical objects, alias maps, generation tokens, and
//!   permission snapshots. The smoke harness routes through a
//!   synthetic root so emitted save-plan records stay byte-stable
//!   across hosts.
//! - [`roots`] — root adapters behind a shared [`roots::VfsRoot`]
//!   contract, including a local filesystem root and in-memory
//!   virtual/generated document roots for wiring live consumers
//!   without re-deriving identity logic per surface.
//! - [`watcher`] — the watcher-source / watcher-health state
//!   machine. Health transitions emit the
//!   [`watcher::WatcherHealthFrame`] the ADR names.
//! - [`save`] — the conflict-aware save stub. It stages a buffer
//!   snapshot against a [`save::SaveTargetToken`], re-reads the
//!   canonical object's strongest generation token
//!   (compare-before-write), selects the atomic / in-place /
//!   conditional-remote / blocked mode, and records a
//!   [`save::SaveManifest`] with the typed outcome.
//!
//! [`harness`] ties them together: a frozen scenario table drives
//! each ADR failure case (atomic commit, case-only variant,
//! symlink / junction / hardlink alias, Unicode-normalization
//! variant, external-change conflict, review-required gate,
//! read-only block, remote conditional conflict, watcher
//! fallback) and emits one save-plan-example JSON record per
//! scenario plus a structural-metrics aggregate. Metrics are
//! counts only so the committed seeds under
//! `artifacts/fs/save_plan_examples/` stay byte-stable.
//!
//! Known holes (real platform adapters, durable cache identity,
//! mutation-journal wiring, rename planning, review workflow,
//! remote agent transport) live in
//! [`prototypes/vfs/README.md`](https://github.com/ahmeddyounis/Aureline/blob/main/prototypes/vfs/README.md)
//! and are tracked as carry-forward items, not silent
//! capabilities of this prototype.

#![doc(html_root_url = "https://docs.rs/aureline-vfs/0.0.0")]

pub mod capabilities;
pub mod filesystem_mutation_lineage_matrix;
pub mod filesystem_truth_review;
pub mod harness;
pub mod hooks;
pub mod identity;
pub mod identity_beta;
pub mod roots;
pub mod save;
pub mod save_conflict_suite;
pub mod synthetic;
pub mod uri_model;
pub mod watcher;
pub mod watchers;

pub use capabilities::{
    AtomicWriteMode, CapabilityFlags, CaseSensitivity, FallbackIdentityTokenKind,
    NormalizationForm, RootCapabilityEnvelope, RootClass, StrongestIdentityTokenKind,
    SymlinkEscapePolicy,
};
pub use filesystem_mutation_lineage_matrix::{
    seeded_filesystem_mutation_lineage_matrix_fixtures,
    seeded_filesystem_mutation_lineage_matrix_packet, validate_filesystem_mutation_lineage_fixture,
    validate_filesystem_mutation_lineage_matrix, ConnectivityState as MatrixConnectivityState,
    CorruptionState as MatrixCorruptionState, CoverageFlags as MatrixCoverageFlags,
    FilesystemMutationLineageMatrixPacket, MatrixFixture, MatrixRootClass, MatrixRow,
    MatrixValidationReport, MatrixValidationViolation,
    PathIdentityClass as MatrixPathIdentityClass,
    ReconciliationPosture as MatrixReconciliationPosture, SaveFallback as MatrixSaveFallback,
    SourceContractRefs as MatrixSourceContractRefs, SurfaceClass as MatrixSurfaceClass,
    UndoClass as MatrixUndoClass, WatchState as MatrixWatchState,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_DOC_REF, FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_DIR,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_MANIFEST_REF,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_RECORD_KIND,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_REF, FILESYSTEM_MUTATION_LINEAGE_MATRIX_REPORT_REF,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_REF,
    FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
};
pub use filesystem_truth_review::{
    seeded_filesystem_truth_review_fixtures, seeded_filesystem_truth_review_packet,
    validate_filesystem_truth_review_fixture, validate_filesystem_truth_review_packet,
    BoundaryCrossingKind, ExternalChangeReviewRecord, FilesystemTruthReviewFixture,
    FilesystemTruthReviewPacket, IgnoreResolutionDrawerRecord, IgnoreSourceClass,
    IgnoreSourceEntry, IgnoreVisibilityClass, MetadataConsequence, MetadataDeltaKind,
    MetadataDeltaNote, ReviewScenarioRecord, ReviewSurfaceValidationReport,
    ReviewSurfaceValidationViolation, WatchFidelityStripRecord, WatchGuaranteeImpact, WatchMode,
    WatchReason, FILESYSTEM_TRUTH_REVIEW_ARTIFACT_REF, FILESYSTEM_TRUTH_REVIEW_DOC_REF,
    FILESYSTEM_TRUTH_REVIEW_FIXTURE_DIR, FILESYSTEM_TRUTH_REVIEW_FIXTURE_MANIFEST_REF,
    FILESYSTEM_TRUTH_REVIEW_FIXTURE_RECORD_KIND, FILESYSTEM_TRUTH_REVIEW_PACKET_RECORD_KIND,
    FILESYSTEM_TRUTH_REVIEW_REPORT_REF, FILESYSTEM_TRUTH_REVIEW_SCHEMA_REF,
    FILESYSTEM_TRUTH_REVIEW_SCHEMA_VERSION,
};
pub use hooks::HookCounters;
pub use identity::{
    compare_external_change, derive_path_truth_chip, filesystem_identity_reference_set,
    inspect_aliases, review_save_target, Alias, AliasInspectionEntry, AliasInspectionRecord,
    AliasKind, AliasSet, CanonicalFilesystemObject, ExternalChangeCompareOutcome,
    ExternalChangeCompareRecord, ExternalChangeContentKind, ExternalChangeDiff,
    ExternalChangeDiffAvailability, ExternalChangeResolutionAction, FallbackIdentityToken,
    FilesystemIdentityReferenceSet, IdentityRecord, IdentityToken, LogicalWorkspaceIdentity,
    PathTruthChip, PathTruthClass, PermissionSummary, PresentationPath, SaveTargetReviewBlocker,
    SaveTargetReviewRecord, TrustState,
};
pub use roots::{
    LocalFilesystemRoot, LocalFilesystemRootError, RootIoError, RootResolveError, VfsRoot,
    VirtualDocumentKind, VirtualDocumentRoot, VirtualDocumentRootError, VirtualDocumentSpec,
};
pub use save::{
    CompareBeforeWriteGenerationToken, GenerationToken, GenerationTokenKind, OpenError,
    PermissionSnapshot, SaveManifest, SaveOutcome, SavePlan, SaveRequest, SaveTargetToken,
};
pub use synthetic::{SyntheticRoot, SyntheticRootBuilder, Workspace};
pub use uri_model::{HierarchicalUriRef, UriError, VfsUri};
pub use watcher::{WatcherHealth, WatcherHealthFrame, WatcherRegistry, WatcherSource};
pub use watchers::{
    VfsChangeEvent, VfsChangeKind, WatcherEvent, WatcherService, WatcherServiceOptions,
};
