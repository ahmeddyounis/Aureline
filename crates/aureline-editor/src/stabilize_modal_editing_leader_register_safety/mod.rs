//! Stabilized modal editing, leader-key discovery, register routing, macro-safe
//! replay, and keyboard-mode downgrade truth.
//!
//! This module owns the bounded alpha contract that governs how modal state,
//! sequence guides, register routes, macro replay review, and surface downgrade
//! truth are exposed to editor chrome, settings, help, support exports, and
//! docs. It intentionally models the bounded alpha contract rather than a full
//! modal-editor implementation.
//!
//! ## Honesty invariants
//!
//! The builder refuses to mint a packet that would hide modal state, silently
//! approximate destructive behavior, or leave invisible traps. Each violation is
//! a [`BuildError`](model::BuildError), not a warning, so a dishonest projection
//! fails the row instead of shipping:
//!
//! - **Modal state is product state.** The mode strip, sequence guide,
//!   register picker, operator-pending overlay, and macro replay review are all
//!   keyboard-reachable and screen-reader narratable.
//! - **Register routes fail closed.** Blocked or unsupported routes (remote
//!   clipboard bridge, policy-blocked, etc.) must not silently fall back to a
//!   nearby route.
//! - **Macro replays are bounded.** Replays that cross files, mutate settings,
//!   invoke run-capable commands, or depend on unstable timing must promote to a
//!   reviewable recipe or be rejected with a reason.
//! - **Sequence states are visible.** Partial, ambiguous, blocked, and
//!   unsupported sequence guides carry a visible note and accessibility
//!   announcement.
//! - **Recovery paths exist.** Keymap diagnostics, command palette search, and
//!   safe-mode reset must be reachable from every modal surface.
//! - **Surface downgrades are labeled and reversible.** IME, accessibility,
//!   browser-companion, restricted-mode, and large-file surfaces that narrow
//!   modal fidelity must label the gap before key meaning changes and must
//!   provide a reversible restore path.
//! - **Import regressions are closed vocabulary.** Imported keymap outcomes are
//!   labeled exact, translated, partial, shimmed, or unsupported, never simply
//!   `imported`.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `stabilize-modal-editing-leader-register-safety`) are:
//!
//! - [`model`] — the governed packet, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/editor/mode_state_record.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live builder and pinned on disk under
//!   `fixtures/editor/m4/stabilize-modal-editing-leader-register-safety/`.
//!
//! The contract narrative is
//! `docs/editor/m4/stabilize-modal-editing-leader-register-safety.md`;
//! the release-evidence packet is
//! `artifacts/editor/m4/stabilize-modal-editing-leader-register-safety.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    modal_editing_safety_corpus, ModalEditingSafetyScenario, MODAL_EDITING_SAFETY_CORPUS_AS_OF,
};
pub use model::{
    BuildError, KeymapImportOutcomeClass, KeymapImportRegressionRecord, ModalEditingSafetyInput,
    ModalEditingSafetyPacket, SurfaceDowngradeKind, SurfaceDowngradeRecord,
    MODAL_CUE_LATENCY_BUDGET_MICROS, MODAL_EDITING_SAFETY_PACKET_RECORD_KIND,
    MODAL_EDITING_SAFETY_SCHEMA_REF, MODAL_EDITING_SAFETY_SCHEMA_VERSION,
};
