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
pub mod harness;
pub mod hooks;
pub mod identity;
pub mod roots;
pub mod save;
pub mod synthetic;
pub mod uri_model;
pub mod watcher;

pub use capabilities::{
    AtomicWriteMode, CapabilityFlags, CaseSensitivity, FallbackIdentityTokenKind,
    NormalizationForm, RootCapabilityEnvelope, RootClass, StrongestIdentityTokenKind,
    SymlinkEscapePolicy,
};
pub use hooks::HookCounters;
pub use identity::{
    Alias, AliasKind, AliasSet, CanonicalFilesystemObject, FallbackIdentityToken, IdentityRecord,
    IdentityToken, LogicalWorkspaceIdentity, PresentationPath, TrustState,
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
