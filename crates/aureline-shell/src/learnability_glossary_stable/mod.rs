//! Stable learnability, glossary, and contextual docs/help truth for switching
//! rows.
//!
//! This module makes the learnability layer replacement-grade for switching
//! users on the claimed stable matrix. It mints one governed
//! [`LearnabilityDisclosureRecord`] per imported source ecosystem (the switching
//! cohort) that binds, for a single canonical switching identity:
//!
//! - **The why-now card** — an inline, dismissible card grounded in
//!   command/file/symbol truth that explains why the flow matters now.
//! - **The glossary chips** — incumbent term → Aureline term mappings, each
//!   citing a stable command/file/symbol/docs anchor.
//! - **The contextual docs/help** — docs/help nodes reachable in place without
//!   losing the switcher's focus.
//! - **The posture** — opt-in and non-blocking: the layer never forces a
//!   tutorial funnel before first useful work and preserves exact focus return.
//! - **The guided-affordance lifecycle markers** — any guided tour / learning /
//!   teaching affordance carries its own `Preview`/`Beta`/`Stable` marker and
//!   support boundary, so it never implies stable coverage by adjacency.
//! - **The privacy posture** — dismissals, resume entries, and the learning
//!   digest stay user-owned and local-first, never repo-visible or
//!   telemetry-grade.
//! - **A public claim ceiling** — no row over-claims any pillar.
//! - **Automatic narrowing** — a row missing any pillar is narrowed below Stable
//!   with a named reason instead of inheriting an adjacent green row.
//! - **Recovery, route, and accessibility parity** — the same row reachable from
//!   the switching row, the docs/help browser, the command palette, and a menu
//!   command, keyboard-first, in normal / high-contrast / zoomed layouts.
//! - **No-account / no-managed-services availability**.
//!
//! The switching row, docs/help browser, command palette, diagnostics, support
//! exports, Help/About, and docs read this record verbatim instead of cloning
//! status text. The canonical artifacts for this lane (suggested-output stem
//! `promote-learnability-glossary-and-contextual-docs-help-guidance`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/promote-learnability-glossary-and-contextual-docs-help-guidance.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live migration scoreboard, migration wizard, and learning-mode manifest /
//!   surface, and pinned on disk under
//!   `fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance/`.
//!
//! The contract narrative is
//! `docs/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance.md`;
//! the release-evidence packet is
//! `artifacts/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance.md`.

pub mod corpus;
pub mod model;

pub use corpus::{learnability_disclosure_corpus, LearnabilityDisclosureScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, is_command_file_symbol_anchor, required_recovery_actions,
    AccessibilityDisclosure, BuildError, ContextualDocsDisclosure, EntryRouteRecord, GlossaryChip,
    GuidedAffordanceDisclosure, GuidedAffordanceKind, LayoutMode, LayoutModeDisclosure,
    LearnabilityClaimCeiling, LearnabilityDisclosureInput, LearnabilityDisclosureRecord,
    LearnabilityPosture, LearnabilityRecoveryAction, LearnabilityRouteSurface,
    LearningStatePrivacyPosture, LifecycleMarker, RecoveryActionRole, RecoveryRouteRecord,
    StableClaimClass, StableNarrowingReason, StableQualification, SurfaceParity, UpstreamRefs,
    WhyNowCard, LEARNABILITY_DISCLOSURE_NOTICE, LEARNABILITY_DISCLOSURE_RECORD_KIND,
    LEARNABILITY_DISCLOSURE_SCHEMA_VERSION, LEARNABILITY_DISCLOSURE_SHARED_CONTRACT_REF,
};
