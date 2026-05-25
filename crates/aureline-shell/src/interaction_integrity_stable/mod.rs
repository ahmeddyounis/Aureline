//! Stable lock for **focus, current item, selection, anchor, activation,
//! keyboard parity, and collection-state semantics** on claimed-stable dense
//! shell surfaces.
//!
//! This module makes dense-surface interaction replacement-grade on the
//! claimed-stable matrix. It mints one governed [`InteractionParityRecord`] per
//! interaction posture that binds, for a single dense-surface identity:
//!
//! - **Distinct coordination states** — `Focus`, `Current item`, `Selection`,
//!   `Anchor`, and `Activation` are separate state objects keyed by stable
//!   object identity, never collapsed and never keyed by row index or DOM
//!   position.
//! - **Identity that survives asynchronous updates** — streaming inserts,
//!   sort/filter/pagination refresh, background indexing, and extension-view
//!   replacement preserve focus and selection by stable object id.
//! - **Complete focus return** — every dialog, sheet, palette, popover, inline
//!   rename, placeholder card, inspector, pane close, and split reflow records a
//!   focus-return target and returns to the invoker, its row, or the nearest
//!   safe ancestor/sibling — never the document body, an off-screen surface, or
//!   a different window.
//! - **A complete keyboard model** — single-tab-stop or roving-tabindex with
//!   Arrow moving the current item, Space toggling selection where supported,
//!   Enter triggering a discoverable default, and Home/End/Page preserving
//!   anchor semantics without silently firing destructive actions.
//! - **No focus theft** — background indexing, streamed rows, notifications,
//!   banners, diagnostics, and multi-window updates never steal focus; when the
//!   focused object disappears, focus moves to the nearest safe sibling or
//!   parent and the reason is announced.
//! - **Complete accessibility cues** — selected-count narration, position-in-set
//!   cues, and blocked/read-only row cues across normal / high-contrast /
//!   zoomed layouts.
//! - **Per-OS conformance** — macOS, Windows, and Linux each carry current proof.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason.
//! - **Recovery, route, and accessibility parity** and **no-account /
//!   no-managed-services availability**.
//!
//! The shell collection surface, the keyboard-help reference, the CLI inspector,
//! Help/About, and the diagnostics support export read this record verbatim
//! instead of cloning status text. The shared object-interaction vocabulary, the
//! batch-scope truth, and the focus-return grammar are **not** reinvented here:
//! the record is a genuine projection of the live interaction-integrity packet
//! in [`crate::interaction_integrity`].
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `harden-focus-selection-keyboard-parity-and-collection-state`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/harden-focus-selection-keyboard-parity-and-collection-state.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live interaction-integrity packet, and pinned on disk under
//!   `fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state/`.
//!
//! The contract narrative is
//! `docs/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state.md`;
//! the release-evidence packet is
//! `artifacts/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state.md`.

pub mod corpus;
pub mod model;

pub use corpus::{interaction_parity_corpus, InteractionParityScenario, CORPUS_AS_OF};
pub use model::{
    required_recovery_routes, AsyncUpdateClass, AsyncUpdateRow, BuildError, CoordinationStateKind,
    CoordinationStateModel, DisappearanceResolution, FocusReturnRow, FocusReturnTrigger,
    InteractionA11yCues, InteractionClaimCeiling, InteractionNarrowingReason, InteractionParityInput,
    InteractionParityRecord, InteractionPillars, InteractionQualification, InteractionRecoveryAction,
    InteractionSurfaceClass, InteractionSurfaceProjection, InteractionSurfaceProjectionInput,
    InteractionTruthSurface, InteractionUpstream, KeyboardModelClass, KeyboardModelRow,
    PlatformConformanceRow, PlatformProfileClass, INTERACTION_PARITY_NOTICE,
    INTERACTION_PARITY_RECORD_KIND, INTERACTION_PARITY_SCHEMA_VERSION,
    INTERACTION_PARITY_SHARED_CONTRACT_REF,
};
