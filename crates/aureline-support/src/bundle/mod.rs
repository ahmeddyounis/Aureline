//! Support-bundle manifest, redaction defaults, local preview, and exact-build capture.
//!
//! See [`crate`] for the seed's posture, what it owns, and what it does
//! not own. The submodules are intentionally narrow:
//!
//! - [`exact_build`] — quotes the canonical `aureline_build_info` record so
//!   the manifest's build identity matches the running binary verbatim.
//! - [`redaction`] — the local-first default redaction profile vocabulary
//!   and rule refs. Mirrors `support.redaction.local_first_default`.
//! - [`vocabulary`] — frozen string tokens shared by the manifest, the
//!   shell copy, and the docs (data class, redaction state, decision
//!   class, exclusion reason, ...).
//! - [`manifest`] — the [`SupportBundleManifest`] record and the
//!   [`SupportBundlePreviewItem`] row.
//! - [`preview`] — [`SupportBundlePreviewBuilder`] and
//!   [`SupportBundlePreview`]: the live local-preview projection the
//!   chrome renders before any export step.

pub mod exact_build;
pub mod manifest;
pub mod preview;
pub mod redaction;
pub mod vocabulary;

pub use exact_build::ExactBuildCapture;
pub use manifest::{
    ActionPolicySourceContext, ActionReconstructionContext, ActionabilityImpact,
    ActionabilityWarning, BuildIdentity, CollectionContext, ExcludedClass, FileSectionIdentity,
    ParityBinding, PolicyContext, PolicyLock, PolicyNote, PreviewClassificationSummary,
    PreviewExportParity, Redaction, RedactionControl, RedactionReport, ReopenAfterExportPath,
    ReviewDecision, SecretScanSummary, SizeEstimate, SupportBundleManifest,
    SupportBundlePreviewItem, COLLECTION_SCHEMA_VERSION, SUPPORT_BUNDLE_MANIFEST_RECORD_KIND,
    SUPPORT_BUNDLE_PREVIEW_ITEM_RECORD_KIND,
};
pub use preview::{
    ActionReconstructionSeed, PreviewItemSeed, SupportBundlePreview, SupportBundlePreviewBuilder,
    SupportBundlePreviewError, SUPPORT_BUNDLE_PREVIEW_RECORD_KIND,
    SUPPORT_BUNDLE_PREVIEW_SEED_SCOPE_NOTICE,
};
pub use redaction::LocalFirstDefaults;
pub use vocabulary::{
    ActionabilityImpactClass, ActorClass, DiagnosticDataClass, ExcludedReasonClass,
    HighRiskContentClass, PolicyNoteSeverity, RedactionState, ReleaseChannelClass,
    ReviewDecidedByClass, ReviewDecisionClass, SecretScanState, TrustState,
};
