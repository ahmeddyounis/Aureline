//! Stable lock for **desktop handoff, file-association, protocol-handler
//! ownership, embedded auth-return-path, and system-browser default
//! conformance**.
//!
//! This module makes the OS-originated desktop entry and return paths
//! replacement-grade on the claimed-stable matrix. It mints one governed
//! [`DesktopHandoffConformanceRecord`] per claimed handoff posture that binds,
//! for a single entry-path identity:
//!
//! - **Typed target intent** — the literal target, the source-locator /
//!   deep-link intent, the requested action, the resulting mode, and the
//!   canonical object identity are preserved end to end instead of reopening a
//!   generic shell or the wrong install.
//! - **Handler ownership** — the owning channel and build are explicit,
//!   side-by-side Stable / Preview / Beta / portable / admin-owned channels are
//!   enumerated, ownership never degrades into last-writer-wins, and handler
//!   spoofing fails closed.
//! - **System-browser default conformance** — claimed identity and auth rows
//!   default to system-browser handoff; any other path surfaces the exception
//!   explicitly with target scope, return path, and recovery behaviour.
//! - **Trust / profile / tenant review** — review precedes any widened
//!   authority or resumed remote action; authority is never widened silently.
//! - **Truthful recovery** — a moved, removable, network, or missing target
//!   renders a recoverable placeholder with its last-seen identity, an
//!   unsaved-local-state posture, and explicit Locate / Open cached context /
//!   Close placeholder actions.
//! - **Per-OS conformance** — macOS, Windows, and Linux profiles each carry
//!   current proof.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason.
//!
//! The desktop handoff review, the CLI inspector, Help/About, and the
//! diagnostics support export read this record verbatim instead of cloning
//! status text. The entry-surface vocabulary, the handler-ownership classes,
//! the target-availability classes, and the system-browser exception classes
//! are **not** reinvented here: the record projects the live
//! [`crate::platform_integration`] native desktop contract packet, the
//! [`crate::deeplink::native_handoff`] handoff vocabulary, and the
//! [`crate::system_browser_return_paths`] auth-return page.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `finalize-desktop-handoff-file-association-protocol-handler-embedded`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/finalize-desktop-handoff-file-association-protocol-handler-embedded.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live native desktop contract packet, and pinned on disk under
//!   `fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded/`.
//!
//! The contract narrative is
//! `docs/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded.md`;
//! the release-evidence packet is
//! `artifacts/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    desktop_handoff_conformance_corpus, DesktopHandoffConformanceScenario, CORPUS_AS_OF,
};
pub use model::{
    required_recovery_routes, AuthDefaultPosture, BuildError, ChannelClass,
    DesktopHandoffConformanceInput, DesktopHandoffConformanceRecord, EntryPathClass,
    HandlerOwnershipDisclosure, HandoffClaimCeiling, HandoffNarrowingReason, HandoffPillars,
    HandoffQualification, HandoffRecoveryAction, HandoffSurfaceProjection,
    HandoffSurfaceProjectionInput, HandoffTruthSurface, HandoffUpstream, PlatformConformanceRow,
    PlatformProfileClass, TargetRecoveryPosture, TrustReviewPosture, TypedTargetIntent,
    DESKTOP_HANDOFF_CONFORMANCE_NOTICE, DESKTOP_HANDOFF_CONFORMANCE_RECORD_KIND,
    DESKTOP_HANDOFF_CONFORMANCE_SCHEMA_VERSION, DESKTOP_HANDOFF_CONFORMANCE_SHARED_CONTRACT_REF,
};
