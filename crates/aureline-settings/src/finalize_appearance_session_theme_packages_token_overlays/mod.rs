//! Stable certification for the **appearance-session finalization** across theme
//! packages, token overlays, imported-theme mapping, live-appearance changes,
//! and extension/embedded-surface appearance descriptors.
//!
//! This lane mints one governed [`AppearanceSessionFinalizationCertification`] per
//! posture. The record proves that appearance state — the active theme package,
//! the session summary, the token overlays, the import mapping, the extension
//! inheritance gaps, and the live-apply honesty — is inspectable, exportable,
//! reversible, and cannot silently redefine trust or severity semantics through
//! color alone.
//!
//! A [`AppearanceSessionFinalizationCertification`] binds, for one posture:
//!
//! - **One appearance-session binding** — every row cites the same session id and
//!   revision, so exported packets and diagnostics read one source of truth.
//! - **Versioned theme package manifests** — every active or claimed package
//!   carries a manifest ref, version label, supported modes, density defaults,
//!   motion flags, minimum contrast metadata, provenance, and inheritance
//!   expectations.
//! - **Appearance-session summaries** — active package refs, follow-system state,
//!   theme/mode, accent source, text scale, density, reduced-motion/high-contrast
//!   state, and live-preview checkpoint/rollback information.
//! - **Token-overlay validation by scope** — user, profile, workspace, and policy
//!   overlays are validated; unknown or unsupported tokens are preserved round-trip
//!   as inert or downgraded rather than silently dropped.
//! - **Imported-theme mapping reports** — translated slots, unsupported slots,
//!   syntax coverage, parity notes, and fallback behavior are visible so imported
//!   themes cannot pretend to be full-fidelity packages without evidence.
//! - **Extension/embedded appearance descriptors** — UI-bearing extensions declare
//!   whether they inherit Aureline theme, density, contrast, focus tokens, and
//!   reduced-motion posture, or surface a visible inheritance gap that cannot be
//!   quietly claimed as Stable parity.
//! - **Live-appearance change honesty** — every OS appearance signal declares
//!   whether it applies live, applies behind a checkpoint, requires confirmation,
//!   or requires a disclosed reload/restart, never silently drifting.
//! - **Provenance preservation** — package/source identity, unresolved-slot notes,
//!   overlay-scope lineage, and extension/webview inheritance gaps survive
//!   import/export/sync instead of being flattened into generic profile settings.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason instead of inheriting an adjacent green row.
//!
//! Dashboards, docs, Help/About surfaces, and support exports read this record
//! verbatim instead of cloning status text.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `finalize-appearance-session-theme-packages-token-overlays`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/finalize-appearance-session-theme-packages-token-overlays.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live appearance runtime, and pinned on disk under
//!   `fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays/`.
//!
//! The contract narrative is
//! `docs/ux/m4/finalize-appearance-session-theme-packages-token-overlays.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/finalize-appearance-session-theme-packages-token-overlays.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    appearance_session_finalization_corpus, AppearanceSessionFinalizationScenario, CORPUS_AS_OF,
};
pub use model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure,
    AppearanceSessionBinding, AppearanceSessionFinalizationCertification,
    AppearanceSessionSummaryRow, BuildError, CertificationClaimCeiling, CertificationInput,
    CertificationNarrowingReason, CertificationPillars, CertificationQualification,
    CertificationRecoveryAction, CertificationUpstream, EntryRouteRecord,
    ExtensionAppearanceDescriptorRow, ExtensionInheritanceState, ImportedThemeMappingReportRow,
    LayoutMode, LayoutModeDisclosure, LifecycleMarker, LiveAppearanceAxisClass,
    LiveAppearanceChangeRow, LiveApplyClass, OverlayScopeClass, ProvenanceDimensionClass,
    ProvenancePreservationRow, RecoveryActionRole, RecoveryRouteRecord, RouteSurface,
    StableClaimClass, ThemePackageManifestRow, TokenOverlayValidationRow, TokenPreservationClass,
    APPEARANCE_SESSION_FINALIZATION_NOTICE, APPEARANCE_SESSION_FINALIZATION_RECORD_KIND,
    APPEARANCE_SESSION_FINALIZATION_SCHEMA_VERSION,
    APPEARANCE_SESSION_FINALIZATION_SHARED_CONTRACT_REF,
};
