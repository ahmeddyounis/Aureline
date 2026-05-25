//! Stable lock for **suspicious-content, safe-preview, copy/export, and
//! representation cues on shell-adjacent surfaces**.
//!
//! This module makes safe preview replacement-grade on the claimed-stable matrix
//! for the shell-adjacent surfaces that can render or hand off ambiguous content.
//! It mints one governed [`SafePreviewRecord`] per shell-adjacent surface posture
//! that binds, for a single surface identity:
//!
//! - **The consumed trust-class ladder** — the [`TrustClass`] and the
//!   [`DetectorOutcomeClass`] come straight from [`aureline_content_safety`]; this
//!   lane never invents a parallel evidence vocabulary.
//! - **Explicit representation cues** — a raw/reveal affordance, a representation
//!   label, and `Copy raw` / `Copy rendered` / `Copy escaped` choices that stay
//!   explicit whenever rendered meaning can differ from source bytes.
//! - **Surfaced suspicious-content findings** — each finding keeps its reveal
//!   affordances and a reachable escaped-copy path.
//! - **Cue survival across carriers** — the trust class, representation label, and
//!   suspicious-content warning survive the notification, activity-center,
//!   browser-handoff, support-export, and screenshot/evidence carriers without
//!   flattening to a generic preview.
//! - **A stricter boundary before commit** — install, attach, approve, publish,
//!   delete, and open-external actions may enforce a stricter preview class than
//!   ordinary browsing, but must show that boundary before the user commits.
//! - **Complete accessibility cues**, **per-OS conformance**, a **public claim
//!   ceiling**, **automatic narrowing** below Stable with a named reason,
//!   **recovery / route / accessibility parity**, and **no-account /
//!   no-managed-services availability**.
//!
//! The shell surface, the activity center, the CLI inspector, Help/About, and the
//! diagnostics support export read this record verbatim instead of cloning status
//! text. The trust-class ladder, the suspicious-content detector, and the
//! representation-transfer grammar are **not** reinvented here: each record is a
//! genuine projection of the live detector and representation builders in
//! [`aureline_content_safety`].
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `certify-suspicious-content-safe-preview-copy-export-and`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/certify-suspicious-content-safe-preview-copy-export-and.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live content-safety detector, and pinned on disk under
//!   `fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and/`.
//!
//! The contract narrative is
//! `docs/ux/m4/certify-suspicious-content-safe-preview-copy-export-and.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/certify-suspicious-content-safe-preview-copy-export-and.md`.

pub mod corpus;
pub mod model;

pub use corpus::{safe_preview_corpus, SafePreviewScenario, CORPUS_AS_OF};
pub use model::{
    required_recovery_routes, BuildError, CueCarrier, CueCarrierRow, PlatformConformanceRow,
    PlatformProfileClass, RepresentationChoiceRow, RepresentationCues, RepresentationCuesInput,
    SafePreviewA11yCues, SafePreviewClaimCeiling, SafePreviewInput, SafePreviewNarrowingReason,
    SafePreviewPillars, SafePreviewQualification, SafePreviewRecord, SafePreviewRecoveryAction,
    SafePreviewSurfaceProjection, SafePreviewSurfaceProjectionInput, SafePreviewTruthSurface,
    SafePreviewUpstream, ShellAdjacentSurface, StricterBoundary, SuspiciousFindingRow,
    CONTENT_SAFETY_CONTRACT_REF, SAFE_PREVIEW_NOTICE, SAFE_PREVIEW_RECORD_KIND,
    SAFE_PREVIEW_SCHEMA_VERSION, SAFE_PREVIEW_SHARED_CONTRACT_REF,
};
