//! Stable migration-center diff, rollback, and unsupported-gap taxonomy truth.
//!
//! This module makes the migration center replacement-grade for switching users
//! on the claimed stable matrix. It mints one governed
//! [`MigrationFlowDisclosureRecord`] per imported source ecosystem that binds,
//! for a single canonical migration identity:
//!
//! - **The diff** — a before/after review shown before apply, every row carrying
//!   both sides and citing one rollback checkpoint.
//! - **The rollback** — a pre-apply checkpoint protecting every touched domain,
//!   with undo and compare routes when (and only when) the evidence is live for
//!   *this* flow.
//! - **The unsupported-gap taxonomy** — the canonical
//!   Exact / Translated / Partial / Shimmed / Unsupported counts, with every
//!   Unsupported and Shimmed gap visible before apply.
//! - **A public claim ceiling** — no row may assert the diff was reviewed,
//!   rollback is available, there are no unsupported gaps, or the import was
//!   full-fidelity unless the product can prove it.
//! - **Automatic narrowing** — a flow missing any pillar of evidence is narrowed
//!   below Stable with a named reason instead of inheriting an adjacent green row.
//! - **Recovery, route, and accessibility parity** — Reopen-report / Compare /
//!   Undo / Review-gaps / Export-support routes, the same flow reachable from the
//!   migration center, settings import history, command palette, and a menu
//!   command, and narration / action labels / affordances reachable in normal,
//!   high-contrast, and zoomed layouts.
//! - **No-account / no-managed-services availability** — every row stays listed
//!   even when identity or managed services are absent.
//!
//! The migration center, settings import history, command palette, diagnostics,
//! support exports, Help/About, and docs read this record verbatim instead of
//! cloning status text. The canonical artifacts for this lane (suggested-output
//! stem `finish-the-migration-center-diff-rollback-and-unsupported`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live migration wizard and corpus builders and pinned on disk under
//!   `fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported/`.
//!
//! The contract narrative is
//! `docs/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    migration_flow_disclosure_corpus, MigrationFlowDisclosureScenario, CORPUS_AS_OF,
};
pub use model::{
    is_canonical_object_ref, required_recovery_actions, AccessibilityDisclosure, BuildError,
    DiffDisclosure, EntryRouteRecord, GapTaxonomy, LayoutMode, LayoutModeDisclosure,
    MigrationClaimCeiling, MigrationFlowDisclosureInput, MigrationFlowDisclosureRecord,
    MigrationRecoveryAction, MigrationRouteSurface, RecoveryActionRole, RecoveryRouteRecord,
    RollbackDisclosure, StableClaimClass, StableNarrowingReason, StableQualification, SurfaceParity,
    UnsupportedGapDisclosure, UpstreamRefs, MIGRATION_FLOW_DISCLOSURE_NOTICE,
    MIGRATION_FLOW_DISCLOSURE_RECORD_KIND, MIGRATION_FLOW_DISCLOSURE_SCHEMA_VERSION,
    MIGRATION_FLOW_DISCLOSURE_SHARED_CONTRACT_REF,
};
