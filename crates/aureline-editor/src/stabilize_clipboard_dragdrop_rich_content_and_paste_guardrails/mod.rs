//! Stabilized transfer-safety truth for clipboard, drag/drop, rich content,
//! paste guardrails, large transfers, and named undo groups.
//!
//! This module owns the stable cross-surface packet that editor, terminal,
//! notebook, docs, shell, and support flows consume when they need to explain
//! what representation is copied, which boundary a paste targets, which verb a
//! drop will commit, and which undo/recovery path a mutation registers.
//!
//! ## Honesty invariants
//!
//! The builder refuses to mint a packet that would hide transfer semantics:
//!
//! - **Plain text survives by default.** Rich or rendered copy is additive and
//!   never the only reachable representation when source text exists.
//! - **Boundary crossings are labeled before commit.** Remote clipboard writes,
//!   support links, private paths, and production shell pastes show boundary
//!   labels and policy gates before changing state.
//! - **High-risk paste is reviewed.** Multiline shell paste requires bracketed
//!   paste, disabled automatic submit, and explicit confirmation.
//! - **Drop targets advertise verbs.** Move, copy, attach, open, import, and
//!   split semantics show insertion indicators, modifier meanings, and a
//!   keyboard command route before commit.
//! - **Mutations are named.** Paste, drop, attach, import, split, AI apply,
//!   settings import, and multi-file replace actions register named undo groups
//!   with source attribution and recovery surfaces.
//! - **Large transfers stay interruptible.** Long paste, output attach, import,
//!   and broad replace flows show progress, cancellation, and completion
//!   summaries.
//! - **Rich content is inspectable.** Sanitized/rich surfaces expose trust class,
//!   raw inspection, and plain-text copy.
//!
//! Canonical artifacts for this lane use the
//! `stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails` stem:
//!
//! - [`model`] defines the governed packet, closed vocabularies, and builder.
//! - [`corpus`] defines the deterministic fixture matrix under
//!   `fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails/`.
//! - The boundary schema is `schemas/ux/transfer-safety.schema.json`.
//! - The contract narrative is
//!   `docs/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails.md`.
//! - The release-evidence packet is
//!   `artifacts/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails.md`.

pub mod corpus;
pub mod model;

pub use corpus::{transfer_safety_corpus, TransferSafetyScenario, TRANSFER_SAFETY_CORPUS_AS_OF};
pub use model::{
    BoundaryClass, BoundaryContext, BuildError as TransferSafetyBuildError, DropPreview, DropVerb,
    LargeTransferFeedback, PasteGuardrail, RecoveryClass, RepresentationTruth, RichContentTrust,
    RichTrustClass, SensitiveReview, SurfaceProjection, TransferActionClass,
    TransferRepresentationClass, TransferSafetyInput, TransferSafetyPacket, TransferSurfaceClass,
    UndoGroupTruth, TRANSFER_SAFETY_PACKET_RECORD_KIND, TRANSFER_SAFETY_SCHEMA_REF,
    TRANSFER_SAFETY_SCHEMA_VERSION,
};
