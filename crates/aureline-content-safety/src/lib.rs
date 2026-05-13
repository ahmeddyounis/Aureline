//! Shared suspicious-content detector and representation-labeled transfer
//! records for safe preview.
//!
//! This crate provides one shared detector contract that every preview-capable
//! surface can map to: editor content, save-review, docs/help pages, install
//! review, support export, and so on. It does not invent a parallel evidence
//! vocabulary — record kinds and field names follow the boundary schemas in
//! `/schemas/security/trust_class.schema.json` and
//! `/schemas/security/text_representation_policy.schema.json` so a single
//! detector outcome can be consumed by any later preview surface without
//! re-parsing the raw content independently.
//!
//! Scope:
//!
//! - Detect bidi control codepoints, invisible/zero-width formatting, and
//!   mixed-script confusable identifiers in plain UTF-8 text.
//! - Build [`SuspiciousContentCaseRecord`] and
//!   [`SurfaceTrustResolutionRecord`] payloads that match the boundary
//!   schemas verbatim.
//! - Build representation-labeled [`RepresentationTransferRecord`] payloads
//!   so copy/export affordances can be surfaced as raw vs escaped instead of
//!   a generic "Copy" or "Export".
//! - Provide an `escape_for_safe_inspection` helper that turns suspicious
//!   codepoints into stable `\u{XXXX}` escapes so an escaped-copy path is
//!   reachable wherever the detector finds something.
//!
//! Out of scope (deferred to later lanes):
//!
//! - Whole-script confusable scoring across full Unicode tables.
//! - Sandboxing / iframe / process boundaries for `IsolatedRemoteActive`.
//! - Final UI chrome for warning badges and trust-class badges.
//! - Raw-vs-rendered divergence detection on rendered HTML/Markdown trees;
//!   only token-level findings on plain text are produced here.

#![doc(html_root_url = "https://docs.rs/aureline-content-safety/0.0.0")]

pub mod detector;
pub mod records;
pub mod suspicious_text;
pub mod transfer;

pub use detector::{
    detect_suspicious_content, escape_for_safe_inspection, has_suspicious_content,
    DetectorOutcomeClass, SuspiciousContentDetection, SuspiciousFinding,
};
pub use records::{
    LabelExamples, SurfaceFamily, SurfaceTrustResolutionRecord, SuspiciousContentCaseRecord,
    SuspiciousContentClass, SuspiciousContentFindingRecord, TrustClass,
};
pub use suspicious_text::{
    project_suspicious_text_core_surfaces, SuspiciousTextAnchor, SuspiciousTextLocationKind,
    SuspiciousTextProjectionSeed, SuspiciousTextSafeExport, SuspiciousTextSurfaceKind,
    SuspiciousTextSurfacePacket, SuspiciousTextSurfaceProjection, SuspiciousTextTransferChoice,
    SuspiciousTextWarning, SuspiciousTextWarningAction, CORE_SOURCE_SURFACES,
    SUSPICIOUS_TEXT_SURFACE_PACKET_SCHEMA_VERSION,
};
pub use transfer::{
    BodyPosture, RepresentationActionId, RepresentationClass, RepresentationTransferRecord,
};
