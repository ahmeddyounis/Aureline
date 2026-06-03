//! Stabilized orientation aids: breadcrumbs, multi-cursor, fold summaries,
//! minimap/overview markers, and constrained-profile downgrade truth.
//!
//! This module owns the bounded stable contract that governs how orientation
//! aids are exposed to editor chrome, settings, help, support exports, and docs.
//! It intentionally models the bounded stable contract rather than a full
//! orientation-aid implementation.
//!
//! ## Honesty invariants
//!
//! The builder refuses to mint a packet that would hide orientation state,
//! silently compress critical markers, or leave invisible traps. Each violation
//! is a [`BuildError`](model::BuildError), not a warning, so a dishonest
//! projection fails the row instead of shipping:
//!
//! - **Multi-cursor state is visible.** Caret count, posture, grouped undo
//!   semantics, and explicit command routes are surfaced before any transform.
//! - **Fold summaries preserve hidden critical state.** Diagnostics, conflicts,
//!   trust warnings, search hits, and review comments inside a folded region
//!   must remain discoverable through the summary and a detail route.
//! - **Breadcrumb continuity is preserved.** Path state stays aligned with
//!   canonical file identity and semantic freshness across moves, renames, and
//!   back/forward navigation.
//! - **Overview aids are optional and suppressible.** Minimap, overview ruler,
//!   and gutter rail markers correspond only to inspectable sources and must
//!   name alternate routes when degraded or disabled.
//! - **Constrained profiles label their downgrade.** Large-file mode,
//!   reduced-motion, high-contrast, battery-saver, and restricted-mode postures
//!   that narrow orientation aids must carry a visible degraded-state label.
//! - **No aid is the sole carrier of critical state.** If a profile suppresses
//!   an aid, the downgrade is explicit and Problems, Search, Review, and Outline
//!   remain available.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `stabilize-orientation-aids-breadcrumbs-folds-minimap`) are:
//!
//! - [`model`] — the governed packet, its builder, and the honesty invariants.
//!   The boundary schema is `schemas/editor/orientation_aid_state.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live builder and pinned on disk under
//!   `fixtures/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap/`.
//!
//! The contract narrative is
//! `docs/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap.md`;
//! the release-evidence packet is
//! `artifacts/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    orientation_aids_stability_corpus, OrientationAidsStabilityScenario,
    ORIENTATION_AIDS_STABILITY_CORPUS_AS_OF,
};
pub use model::{
    BuildError, OrientationAidsStabilityInput, OrientationAidsStabilityPacket,
    ORIENTATION_AID_FILE_SWITCH_BUDGET_MICROS, ORIENTATION_AID_LATENCY_BUDGET_MICROS,
    ORIENTATION_AID_SCROLL_BUDGET_MICROS, ORIENTATION_AID_TYPING_BUDGET_MICROS,
    ORIENTATION_AIDS_STABILITY_PACKET_RECORD_KIND, ORIENTATION_AIDS_STABILITY_SCHEMA_REF,
    ORIENTATION_AIDS_STABILITY_SCHEMA_VERSION,
};
