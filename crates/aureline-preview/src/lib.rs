//! Representation-labeled safe-preview and copy/export seed wedge.
//!
//! This crate is the M1 bounded prototype that proves three risky preview
//! lanes can carry an honest representation label, a paired copy/export
//! action set, and a typed downgrade story end to end on one live shell
//! row:
//!
//! - **Risky text** — bidi controls, invisible formatters, and mixed-script
//!   confusables in plain UTF-8 text. The detector outcome that drives
//!   labelling comes verbatim from [`aureline_content_safety`]; this crate
//!   does not re-derive what is suspicious.
//! - **Oversized artifacts** — files / logs / captures whose visible body is
//!   only a windowed slice of the source. The wedge surfaces typed scope and
//!   transform tokens so a rendered preview can never silently claim full
//!   fidelity.
//! - **Generated content** — model-produced summaries / diffs / explanations.
//!   The wedge pins the on-screen representation to `generated`, requires a
//!   citation anchor list for any quoted authoritative bytes, and refuses to
//!   advertise a `copy_raw` action when no canonical source exists.
//!
//! Every preview the wedge emits ([`safe_preview::SafePreviewRecord`])
//! carries:
//!
//! - a [`safe_preview::PrototypeLabel`] chip so the chrome cannot quietly
//!   drop the wedge label;
//! - a typed [`safe_preview::ContentClass`] so support exports name the
//!   risky / oversized / generated lane;
//! - the [`aureline_content_safety::TrustClass`] of the source surface;
//!   the wedge MUST NOT mint a synonym;
//! - a [`safe_preview::OriginClass`] (user-authored vs generated vs unknown)
//!   so a `generated` preview never silently inherits raw-source identity;
//! - a [`safe_preview::CurrentlyVisibleRepresentation`] that names which
//!   representation the user is presently looking at; and
//! - a paired list of [`safe_preview::CopyExportOption`] rows, each
//!   carrying the
//!   [`aureline_content_safety::RepresentationActionId`] and
//!   [`aureline_content_safety::RepresentationClass`] vocabulary verbatim,
//!   plus typed scope / transform / omission tokens and an honest
//!   [`safe_preview::ShareSafety`] posture.
//!
//! The wedge is bounded:
//!
//! - It does not rewrite the broader content viewer; it only owns the
//!   preview record and the named shell consumer at
//!   [`crates/aureline-shell/src/safe_preview_card/`](../../../crates/aureline-shell/src/safe_preview_card/mod.rs).
//! - It does not invent new representation, action, scope, transform, or
//!   omission tokens. Every closed vocabulary mirrors the schema in
//!   [`/schemas/ux/representation_copy_export.schema.json`](../../../schemas/ux/representation_copy_export.schema.json)
//!   and the parity contract in
//!   [`/docs/ux/copy_export_representation_parity.md`](../../../docs/ux/copy_export_representation_parity.md).
//! - It does not silently widen authority. Calling
//!   [`safe_preview::SafePreviewRecord::validate`] surfaces every
//!   representation-honesty rule the spec freezes (e.g. risky text MUST
//!   offer both `copy_raw` and `copy_escaped` paired, generated previews
//!   MUST NOT advertise `copy_raw` without a canonical-source anchor,
//!   oversized previews MUST name a scope token other than
//!   `loaded_materialized_set` when the visible slice is windowed).
//!
//! The reviewer-facing landing page is
//! [`/docs/ux/m1_safe_preview_and_copy_export.md`](../../../docs/ux/m1_safe_preview_and_copy_export.md).

#![doc(html_root_url = "https://docs.rs/aureline-preview/0.0.0")]

pub mod freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix;
pub mod preview_origin;
pub mod safe_preview;

pub use freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix::{
    current_m5_preview_inspection_matrix_export, AttachDepthClass, MatrixConsumerProjection,
    MatrixEvidenceFreshness, MatrixGuardrails, PreviewInspectionMatrixArtifactError,
    PreviewInspectionMatrixPacket, PreviewInspectionMatrixPacketInput,
    PreviewInspectionMatrixViolation, PreviewInspectionRow, PreviewMatrixDowngradeTrigger,
    PreviewMatrixQualificationClass, PreviewSessionClass, PreviewSurface,
    RoundTripCapabilityClass, SourceSyncClass, M5_PREVIEW_INSPECTION_MATRIX_ARTIFACT_REF,
    M5_PREVIEW_INSPECTION_MATRIX_DOC_REF, M5_PREVIEW_INSPECTION_MATRIX_FIXTURE_DIR,
    M5_PREVIEW_INSPECTION_MATRIX_RECORD_KIND, M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_REF,
    M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_VERSION, M5_PREVIEW_INSPECTION_MATRIX_SUMMARY_REF,
};

pub use preview_origin::{
    BrowserRuntimeSessionOrigin, BrowserSessionOriginClass, BrowserSessionScopeClass,
    CrossOriginPostureClass, DeviceCapabilityClass, HotReloadEventClass, HotReloadStateDescriptor,
    HotReloadStateRecoveryRoute, HotReloadUnderlyingStateClass, MutationActionKind,
    MutationBlastClass, MutationReviewRequirement, PreviewLaneClass, PreviewOriginClass,
    PreviewOriginDescriptor, PreviewOriginFinding, PreviewOriginLifecyclePhase,
    PreviewOriginSharingPosture, PreviewTargetClass, PreviewTargetDescriptor,
    PreviewTargetReducedCapabilityReason, ProtocolPostureClass, RuntimeMutationActionPlan,
    SourceMappingDescriptor, SourceMappingQualityClass, BROWSER_SESSION_ORIGIN_RECORD_KIND,
    BROWSER_SESSION_ORIGIN_SCHEMA_VERSION, HOT_RELOAD_STATE_DESCRIPTOR_RECORD_KIND,
    HOT_RELOAD_STATE_DESCRIPTOR_SCHEMA_VERSION, PREVIEW_ORIGIN_DESCRIPTOR_RECORD_KIND,
    PREVIEW_ORIGIN_DESCRIPTOR_SCHEMA_VERSION, PREVIEW_TARGET_DESCRIPTOR_RECORD_KIND,
    PREVIEW_TARGET_DESCRIPTOR_SCHEMA_VERSION, RUNTIME_MUTATION_ACTION_PLAN_RECORD_KIND,
    RUNTIME_MUTATION_ACTION_PLAN_SCHEMA_VERSION,
};
pub use safe_preview::{
    build_generated_content_preview, build_oversized_artifact_preview, build_risky_text_preview,
    ContentClass, CopyExportActionKind, CopyExportOption, CurrentlyVisibleRepresentation,
    GeneratedContentInput, OmissionReason, OmissionSummary, OriginClass, OversizedArtifactInput,
    PrototypeLabel, RiskyTextInput, SafePreviewClaimLimit, SafePreviewInvariantViolation,
    SafePreviewRecord, ScopeClass, ShareSafety, TransformKind, SAFE_PREVIEW_RECORD_KIND,
    SAFE_PREVIEW_SCHEMA_VERSION,
};
