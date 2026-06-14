//! Freeze of the source-first preview, preview-runtime, source-map, and
//! browser-runtime inspection matrix for every claimed beta preview/runtime row.
//!
//! Where [`crate::preview_origin`] materializes the per-view truth objects — a
//! [`crate::preview_origin::PreviewOriginDescriptor`] (which runtime produced the
//! view), a [`crate::preview_origin::PreviewTargetDescriptor`] (which target kind
//! is on screen), a [`crate::preview_origin::SourceMappingDescriptor`] (how exact
//! the source mapping is), and a [`crate::preview_origin::BrowserRuntimeSessionOrigin`]
//! (which browser session and what cross-origin/protocol limits apply) — this
//! module binds those objects into a single bounded **qualification matrix**. The
//! matrix is the one canonical answer to "for this claimed preview/runtime
//! surface, what is its preview-session class, source-sync state, target kind,
//! mapping-quality class, browser-runtime attach depth, and round-trip
//! capability — and is the public qualification it claims actually backed by an
//! identified source-sync, target kind, and mapping-quality class?"
//!
//! Each [`PreviewInspectionRow`] reuses the frozen target-kind
//! ([`crate::preview_origin::PreviewTargetClass`]) and mapping-quality
//! ([`crate::preview_origin::SourceMappingQualityClass`]) vocabularies rather than
//! minting synonyms, and adds the matrix-level dimensions this freeze owns:
//! [`PreviewSessionClass`], [`SourceSyncClass`], [`AttachDepthClass`], and
//! [`RoundTripCapabilityClass`]. The matrix *auto-narrows*: a claimed row that
//! cannot identify its source-sync state, target kind, or mapping-quality class
//! must carry an `effective_qualification` strictly below its claim, a recorded
//! narrowing trigger, and a precise degraded label — so a preview/runtime claim
//! never outruns the evidence that backs it.
//!
//! [`PreviewInspectionMatrixPacket::validate`] also refuses a row that lets a
//! runtime-only view masquerade as saved source state, hides mapping uncertainty
//! behind a write-capable claim, auto-upgrades an inspect-only surface into a
//! write-capable designer flow, or commits a high-impact visual edit without
//! previewing the real source diff first.
//!
//! Raw URLs, hostnames, cookies, raw provider payloads, credentials, and raw
//! runtime handles never cross this boundary; the packet carries only typed class
//! tokens, booleans, and opaque evidence refs.
//!
//! The boundary schema is
//! [`schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json`](../../../../schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json).
//! The contract doc is
//! [`docs/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md`](../../../../docs/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md).
//! The protected fixture directory is
//! [`fixtures/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/`](../../../../fixtures/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::preview_origin::{PreviewTargetClass, SourceMappingQualityClass};

/// Stable record-kind tag carried by [`PreviewInspectionMatrixPacket`].
pub const M5_PREVIEW_INSPECTION_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix";

/// Schema version for the preview/runtime inspection matrix.
pub const M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_REF: &str =
    "schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PREVIEW_INSPECTION_MATRIX_DOC_REF: &str =
    "docs/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PREVIEW_INSPECTION_MATRIX_FIXTURE_DIR: &str =
    "fixtures/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PREVIEW_INSPECTION_MATRIX_ARTIFACT_REF: &str =
    "artifacts/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_PREVIEW_INSPECTION_MATRIX_SUMMARY_REF: &str =
    "artifacts/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md";

/// One claimed beta preview/runtime surface a matrix row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewSurface {
    /// Source-first framework preview (the canonical source drives the view).
    SourceFirstFrameworkPreview,
    /// Inspectable visual-surface mapping (component / DOM / widget mapping).
    VisualSurfaceMapping,
    /// Browser-runtime inspection (DOM / CSS / network / storage inspectors).
    BrowserRuntimeInspection,
    /// Device or simulator preview target.
    DeviceOrSimulatorPreview,
    /// Full-stack preview loop spanning client and server.
    FullStackPreviewLoop,
    /// Embedded webview preview hosted in the shell or an extension host.
    EmbeddedWebviewPreview,
    /// Visual-edit transform that previews the real source diff before commit.
    VisualEditTransform,
    /// Support / export projection of the matrix.
    SupportExportProjection,
}

impl PreviewSurface {
    /// Every claimed surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SourceFirstFrameworkPreview,
        Self::VisualSurfaceMapping,
        Self::BrowserRuntimeInspection,
        Self::DeviceOrSimulatorPreview,
        Self::FullStackPreviewLoop,
        Self::EmbeddedWebviewPreview,
        Self::VisualEditTransform,
        Self::SupportExportProjection,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFirstFrameworkPreview => "source_first_framework_preview",
            Self::VisualSurfaceMapping => "visual_surface_mapping",
            Self::BrowserRuntimeInspection => "browser_runtime_inspection",
            Self::DeviceOrSimulatorPreview => "device_or_simulator_preview",
            Self::FullStackPreviewLoop => "full_stack_preview_loop",
            Self::EmbeddedWebviewPreview => "embedded_webview_preview",
            Self::VisualEditTransform => "visual_edit_transform",
            Self::SupportExportProjection => "support_export_projection",
        }
    }
}

/// Closed preview-session vocabulary. Names the kind of preview session a row
/// presents; distinct from the per-view origin descriptor, this is the
/// matrix-level session classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewSessionClass {
    /// Source-bound live preview — canonical source drives a live, derivative view.
    SourceBoundLivePreview,
    /// Runtime-backed inspection — a live runtime is attached for inspection; the
    /// view reflects runtime state, never saved source state by itself.
    RuntimeBackedInspection,
    /// Snapshot projection — a captured / pinned static view with no live runtime.
    SnapshotProjection,
    /// Device-tethered session — a simulator or physical device over a transport.
    DeviceTetheredSession,
    /// Embedded renderer session — a renderer hosted inside the shell / extension host.
    EmbeddedRendererSession,
    /// External handoff session — the preview was handed off to the system browser.
    ExternalHandoffSession,
}

impl PreviewSessionClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceBoundLivePreview => "source_bound_live_preview",
            Self::RuntimeBackedInspection => "runtime_backed_inspection",
            Self::SnapshotProjection => "snapshot_projection",
            Self::DeviceTetheredSession => "device_tethered_session",
            Self::EmbeddedRendererSession => "embedded_renderer_session",
            Self::ExternalHandoffSession => "external_handoff_session",
        }
    }

    /// True when the session is backed by a live runtime whose state can diverge
    /// from saved source and so must never masquerade as saved source state.
    pub const fn is_runtime_backed(self) -> bool {
        matches!(
            self,
            Self::RuntimeBackedInspection | Self::DeviceTetheredSession
        )
    }
}

/// Closed source-sync vocabulary. Names how the derivative preview relates to the
/// canonical source right now. Source remains canonical; the preview is
/// derivative — this class keeps that relationship honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceSyncClass {
    /// The preview reflects the current canonical source.
    InSyncFromSource,
    /// The source changed; the preview is queued for rebuild but not yet refreshed.
    PendingRebuild,
    /// The preview has drifted from the canonical source (stale).
    DriftedFromSource,
    /// The view is runtime-only with no saved-source backing; it must declare so
    /// and never claim to be saved source state.
    RuntimeOnlyNoSource,
    /// The sync state could not be identified; a claimed row in this state must
    /// auto-narrow before promotion.
    UnidentifiedSourceSync,
}

impl SourceSyncClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSyncFromSource => "in_sync_from_source",
            Self::PendingRebuild => "pending_rebuild",
            Self::DriftedFromSource => "drifted_from_source",
            Self::RuntimeOnlyNoSource => "runtime_only_no_source",
            Self::UnidentifiedSourceSync => "unidentified_source_sync",
        }
    }

    /// True when the sync state could not be identified.
    pub const fn is_unidentified(self) -> bool {
        matches!(self, Self::UnidentifiedSourceSync)
    }

    /// True when the view is runtime-only with no saved-source backing.
    pub const fn is_runtime_only(self) -> bool {
        matches!(self, Self::RuntimeOnlyNoSource)
    }
}

/// Closed browser-runtime attach-depth vocabulary. Names how deep the runtime
/// inspection reaches so a shallow attach never advertises full DOM/CSS/network/
/// storage inspection by silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachDepthClass {
    /// No runtime attach; the row is source-first only.
    NoAttach,
    /// DOM tree inspection only.
    DomOnly,
    /// DOM plus computed styles (CSS).
    DomAndStyles,
    /// DOM, styles, and network activity.
    DomStylesNetwork,
    /// DOM, styles, network, and storage — the full inspection depth.
    DomStylesNetworkStorage,
    /// The target is not a browser-runtime surface; attach depth does not apply.
    NotApplicableNonBrowser,
}

impl AttachDepthClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAttach => "no_attach",
            Self::DomOnly => "dom_only",
            Self::DomAndStyles => "dom_and_styles",
            Self::DomStylesNetwork => "dom_styles_network",
            Self::DomStylesNetworkStorage => "dom_styles_network_storage",
            Self::NotApplicableNonBrowser => "not_applicable_non_browser",
        }
    }

    /// True when the attach depth reaches at least into the live DOM.
    pub const fn is_attached(self) -> bool {
        matches!(
            self,
            Self::DomOnly
                | Self::DomAndStyles
                | Self::DomStylesNetwork
                | Self::DomStylesNetworkStorage
        )
    }
}

/// Closed round-trip capability vocabulary. Names whether a visual action against
/// the row is an exact source round-trip, an approximate one, inspect-only, a
/// source-only fallback, or has no path back to source at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundTripCapabilityClass {
    /// A visual edit maps to an exact canonical-source diff, previewed before commit.
    ExactSourceRoundTrip,
    /// A visual edit maps approximately to source; the diff is previewed before commit.
    ApproximateSourceRoundTrip,
    /// Inspect-only; the surface never writes back to source.
    InspectOnlyNoWrite,
    /// No visual write-back; edits fall back to editing the source directly.
    SourceOnlyFallback,
    /// No mapping back to source at all (e.g. a captured snapshot).
    NoRoundTrip,
}

impl RoundTripCapabilityClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSourceRoundTrip => "exact_source_round_trip",
            Self::ApproximateSourceRoundTrip => "approximate_source_round_trip",
            Self::InspectOnlyNoWrite => "inspect_only_no_write",
            Self::SourceOnlyFallback => "source_only_fallback",
            Self::NoRoundTrip => "no_round_trip",
        }
    }

    /// True when a visual edit against this row writes back to source.
    pub const fn writes_back_to_source(self) -> bool {
        matches!(
            self,
            Self::ExactSourceRoundTrip | Self::ApproximateSourceRoundTrip
        )
    }

    /// True when the surface must remain inspect-only and never auto-upgrade into
    /// a write-capable designer flow.
    pub const fn is_inspect_only(self) -> bool {
        matches!(self, Self::InspectOnlyNoWrite | Self::NoRoundTrip)
    }
}

/// Closed qualification vocabulary the matrix freezes for claimed preview/runtime
/// rows. Higher means a stronger public claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewMatrixQualificationClass {
    /// Promoted, durable, publicly claimed.
    Stable,
    /// Publicly claimed but still hardening.
    Beta,
    /// Narrow public preview.
    Preview,
    /// Internal / experimental; not a public claim.
    Experimental,
    /// Held below preview pending evidence.
    Held,
    /// Not available on this surface.
    Unavailable,
}

impl PreviewMatrixQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Held => "held",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether this class is a publicly claimed lane (Stable, Beta, or Preview).
    pub const fn is_claimed(self) -> bool {
        matches!(self, Self::Stable | Self::Beta | Self::Preview)
    }

    /// Ordinal rank used to compare claim severity; higher is a stronger claim, so
    /// a narrowing must move strictly lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unavailable => 0,
            Self::Held => 1,
            Self::Experimental => 2,
            Self::Preview => 3,
            Self::Beta => 4,
            Self::Stable => 5,
        }
    }
}

/// Closed downgrade-trigger vocabulary. Names why a claimed row narrowed below its
/// claim; the chrome quotes the trigger verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewMatrixDowngradeTrigger {
    /// The source-sync state could not be identified.
    UnidentifiedSourceSync,
    /// The target kind could not be identified.
    UnidentifiedTargetKind,
    /// The mapping-quality class could not be identified.
    UnidentifiedMappingQuality,
    /// The surface is inspect-only and cannot back a write-capable claim.
    InspectOnlyScope,
    /// A cross-origin / protocol limit blocks part of the inspection.
    CrossOriginBlocked,
    /// The view is runtime-only with no saved-source backing.
    RuntimeOnlyNoSource,
    /// Policy narrowed the surface below its claim.
    PolicyNarrowed,
    /// An upstream dependency narrowed and dragged this row down with it.
    UpstreamDependencyNarrowed,
}

impl PreviewMatrixDowngradeTrigger {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnidentifiedSourceSync => "unidentified_source_sync",
            Self::UnidentifiedTargetKind => "unidentified_target_kind",
            Self::UnidentifiedMappingQuality => "unidentified_mapping_quality",
            Self::InspectOnlyScope => "inspect_only_scope",
            Self::CrossOriginBlocked => "cross_origin_blocked",
            Self::RuntimeOnlyNoSource => "runtime_only_no_source",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// One claimed beta preview/runtime row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewInspectionRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed preview/runtime surface.
    pub surface: PreviewSurface,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Matrix-level preview-session class.
    pub preview_session_class: PreviewSessionClass,
    /// Source-sync state of the derivative preview against canonical source.
    pub source_sync_class: SourceSyncClass,
    /// Identified target kind, reusing the frozen target vocabulary. `None` means
    /// the target kind could not be identified and forces narrowing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_kind: Option<PreviewTargetClass>,
    /// Identified mapping-quality class, reusing the frozen mapping vocabulary.
    /// `None` means the mapping quality could not be identified and forces
    /// narrowing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping_quality: Option<SourceMappingQualityClass>,
    /// Browser-runtime attach depth.
    pub attach_depth_class: AttachDepthClass,
    /// Round-trip capability for visual actions against the row.
    pub round_trip_capability: RoundTripCapabilityClass,
    /// Headline qualification publicly claimed for this row.
    pub claimed_qualification: PreviewMatrixQualificationClass,
    /// Effective qualification after auto-narrowing; equals the claim when every
    /// identity dimension is present, and ranks strictly below it otherwise.
    pub effective_qualification: PreviewMatrixQualificationClass,
    /// True when a live runtime backs the view.
    pub runtime_backed: bool,
    /// True when the row claims the view is saved source state. A runtime-only
    /// view must never set this true.
    pub claims_saved_source: bool,
    /// True when a visual edit against the row writes back to source.
    pub write_capable: bool,
    /// True when a write-capable visual edit previews the real source diff before
    /// commit.
    pub previews_source_diff_before_commit: bool,
    /// Trigger that fired the narrowing, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<PreviewMatrixDowngradeTrigger>,
    /// Precise degraded label, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl PreviewInspectionRow {
    /// Whether this row carries a public claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_qualification.is_claimed()
    }

    /// Whether every required identity dimension (source-sync, target kind,
    /// mapping-quality) is identified.
    pub fn identity_complete(&self) -> bool {
        !self.source_sync_class.is_unidentified()
            && self.target_kind.is_some()
            && self.mapping_quality.is_some()
    }

    /// Whether the row must narrow below its claim because an identity dimension
    /// is missing.
    pub fn needs_narrowing(&self) -> bool {
        !self.identity_complete()
    }

    /// Whether the effective qualification and narrowing evidence are consistent.
    ///
    /// When every identity dimension is present the effective qualification equals
    /// the claim; otherwise it must rank strictly below the claim and carry both a
    /// recorded narrowing trigger and a precise degraded label.
    pub fn narrowing_consistent(&self) -> bool {
        if self.needs_narrowing() {
            self.effective_qualification.rank() < self.claimed_qualification.rank()
                && self.narrow_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_qualification == self.claimed_qualification
        }
    }

    /// Whether a runtime-only view is honestly labeled rather than passed off as
    /// saved source state.
    pub fn runtime_masquerade_ok(&self) -> bool {
        if self.source_sync_class.is_runtime_only() {
            self.runtime_backed && !self.claims_saved_source
        } else {
            true
        }
    }

    /// Whether the inspect-only posture is preserved (no silent write-up).
    pub fn inspect_only_ok(&self) -> bool {
        if self.round_trip_capability.is_inspect_only() {
            !self.write_capable
        } else {
            true
        }
    }

    /// Whether a write-capable visual edit previews the real source diff first.
    pub fn source_diff_preview_ok(&self) -> bool {
        if self.write_capable {
            self.previews_source_diff_before_commit
                && self.round_trip_capability.writes_back_to_source()
        } else {
            true
        }
    }

    /// Whether the attach depth is consistent with the identified target kind.
    ///
    /// Browser-runtime targets must declare a real attach depth; non-browser
    /// targets must declare `not_applicable_non_browser`.
    pub fn attach_depth_consistent(&self) -> bool {
        match self.target_kind {
            Some(target) if target.participates_in_browser_runtime() => {
                self.attach_depth_class != AttachDepthClass::NotApplicableNonBrowser
            }
            Some(_) => self.attach_depth_class == AttachDepthClass::NotApplicableNonBrowser,
            None => true,
        }
    }

    /// Whether an exact source round-trip is backed by an exact mapping quality.
    pub fn round_trip_mapping_consistent(&self) -> bool {
        if matches!(
            self.round_trip_capability,
            RoundTripCapabilityClass::ExactSourceRoundTrip
        ) {
            self.mapping_quality == Some(SourceMappingQualityClass::Exact)
        } else {
            true
        }
    }

    /// Whether every dimension required to record this row is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.narrowing_consistent()
            && self.runtime_masquerade_ok()
            && self.inspect_only_ok()
            && self.source_diff_preview_ok()
            && self.attach_depth_consistent()
            && self.round_trip_mapping_consistent()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixGuardrails {
    /// Source remains canonical; the preview is derivative, never a second
    /// writable truth model.
    pub source_canonical_preview_derivative: bool,
    /// Runtime-backed inspection never masquerades as saved source state.
    pub runtime_inspection_never_masquerades_as_source: bool,
    /// Mapping uncertainty is never hidden by runtime or extension-private wording.
    pub mapping_uncertainty_never_hidden: bool,
    /// Inspect-only rows are never auto-upgraded into write-capable designer flows.
    pub inspect_only_never_auto_upgraded_to_write: bool,
    /// High-impact visual edits preview the real source diff before commit.
    pub visual_edits_preview_source_diff_before_commit: bool,
    /// Embedded preview/browser boundaries are not blurred into product authority.
    pub embedded_boundaries_not_blurred_into_product: bool,
    /// Any row lacking an identified source-sync, target kind, or mapping-quality
    /// class auto-narrows below its claim.
    pub rows_auto_narrow_on_unidentified_dimension: bool,
}

impl MatrixGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_canonical_preview_derivative
            && self.runtime_inspection_never_masquerades_as_source
            && self.mapping_uncertainty_never_hidden
            && self.inspect_only_never_auto_upgraded_to_write
            && self.visual_edits_preview_source_diff_before_commit
            && self.embedded_boundaries_not_blurred_into_product
            && self.rows_auto_narrow_on_unidentified_dimension
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixConsumerProjection {
    /// Product surfaces ingest this matrix instead of cloning preview/runtime text.
    pub product_ingests_matrix: bool,
    /// Docs/help ingests the same matrix.
    pub docs_help_ingests_matrix: bool,
    /// Diagnostics ingests the same matrix.
    pub diagnostics_ingests_matrix: bool,
    /// Extension/provider conformance ingests the same matrix.
    pub extension_provider_conformance_ingests_matrix: bool,
    /// Release-control surfaces ingest the same matrix.
    pub release_control_ingests_matrix: bool,
    /// Narrowed rows are visibly labeled below current in every surface.
    pub narrowed_rows_labeled_below_current: bool,
}

impl MatrixConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_matrix
            && self.docs_help_ingests_matrix
            && self.diagnostics_ingests_matrix
            && self.extension_provider_conformance_ingests_matrix
            && self.release_control_ingests_matrix
            && self.narrowed_rows_labeled_below_current
    }
}

/// Evidence freshness block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixEvidenceFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically narrows claimed rows.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PreviewInspectionMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewInspectionMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Per-row qualifications.
    pub rows: Vec<PreviewInspectionRow>,
    /// Guardrail invariants block.
    pub guardrails: MatrixGuardrails,
    /// Consumer projection block.
    pub consumer_projection: MatrixConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: MatrixEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe source-first preview / browser-runtime inspection matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewInspectionMatrixPacket {
    /// Record kind; must equal [`M5_PREVIEW_INSPECTION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Per-row qualifications.
    pub rows: Vec<PreviewInspectionRow>,
    /// Guardrail invariants block.
    pub guardrails: MatrixGuardrails,
    /// Consumer projection block.
    pub consumer_projection: MatrixConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: MatrixEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PreviewInspectionMatrixPacket {
    /// Builds a preview/runtime inspection matrix packet.
    pub fn new(input: PreviewInspectionMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_PREVIEW_INSPECTION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some row in this matrix.
    pub fn represented_surfaces(&self) -> BTreeSet<PreviewSurface> {
        self.rows.iter().map(|row| row.surface).collect()
    }

    /// Count of rows whose effective qualification was narrowed below its claim.
    pub fn narrowed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_narrowing()).count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Validates the preview/runtime inspection matrix invariants.
    pub fn validate(&self) -> Vec<PreviewInspectionMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PREVIEW_INSPECTION_MATRIX_RECORD_KIND {
            violations.push(PreviewInspectionMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_VERSION {
            violations.push(PreviewInspectionMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PreviewInspectionMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_evidence_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("preview inspection matrix packet serializes"),
        ) {
            violations.push(PreviewInspectionMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("preview inspection matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Source-First Preview / Browser-Runtime Inspection Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} narrowed)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.narrowed_row_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            PreviewSurface::ALL.len()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_qualification.as_str(),
                row.effective_qualification.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - session=`{}` source_sync=`{}` target=`{}` mapping=`{}`\n",
                row.preview_session_class.as_str(),
                row.source_sync_class.as_str(),
                row.target_kind
                    .map_or("unidentified", PreviewTargetClass::as_str),
                row.mapping_quality
                    .map_or("unidentified", SourceMappingQualityClass::as_str),
            ));
            out.push_str(&format!(
                "  - attach_depth=`{}` round_trip=`{}` write_capable={}\n",
                row.attach_depth_class.as_str(),
                row.round_trip_capability.as_str(),
                row.write_capable,
            ));
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in matrix export.
#[derive(Debug)]
pub enum PreviewInspectionMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PreviewInspectionMatrixViolation>),
}

impl fmt::Display for PreviewInspectionMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "preview inspection matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "preview inspection matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PreviewInspectionMatrixArtifactError {}

/// Validation failures emitted by [`PreviewInspectionMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreviewInspectionMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required preview/runtime surface is represented by no row.
    RequiredSurfaceMissing,
    /// No row demonstrates auto-narrowing on an unidentified identity dimension.
    NarrowedRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not narrowed below its claim despite a missing dimension.
    RowNotNarrowedOnUnidentifiedDimension,
    /// A narrowed row lacks a precise degraded label or narrowing trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// A runtime-only view masquerades as saved source state.
    RuntimeOnlyMasqueradesAsSource,
    /// An inspect-only row claims write capability.
    InspectOnlyRowClaimsWrite,
    /// A write-capable visual edit does not preview the real source diff first.
    WriteCapableRowSkipsSourceDiffPreview,
    /// Attach depth is inconsistent with the identified target kind.
    AttachDepthInconsistentWithTarget,
    /// An exact round-trip is not backed by an exact mapping quality.
    ExactRoundTripWithoutExactMapping,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// A claimed row is missing required evidence refs.
    ClaimedRowMissingEvidence,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PreviewInspectionMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::NarrowedRowCaseMissing => "narrowed_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotNarrowedOnUnidentifiedDimension => {
                "row_not_narrowed_on_unidentified_dimension"
            }
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::RuntimeOnlyMasqueradesAsSource => "runtime_only_masquerades_as_source",
            Self::InspectOnlyRowClaimsWrite => "inspect_only_row_claims_write",
            Self::WriteCapableRowSkipsSourceDiffPreview => {
                "write_capable_row_skips_source_diff_preview"
            }
            Self::AttachDepthInconsistentWithTarget => "attach_depth_inconsistent_with_target",
            Self::ExactRoundTripWithoutExactMapping => "exact_round_trip_without_exact_mapping",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::ClaimedRowMissingEvidence => "claimed_row_missing_evidence",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable matrix export.
pub fn current_m5_preview_inspection_matrix_export(
) -> Result<PreviewInspectionMatrixPacket, PreviewInspectionMatrixArtifactError> {
    let packet: PreviewInspectionMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/support_export.json"
    )))
    .map_err(PreviewInspectionMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PreviewInspectionMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &PreviewInspectionMatrixPacket,
    violations: &mut Vec<PreviewInspectionMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_REF,
        M5_PREVIEW_INSPECTION_MATRIX_DOC_REF,
        M5_PREVIEW_INSPECTION_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PreviewInspectionMatrixViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &PreviewInspectionMatrixPacket,
    violations: &mut Vec<PreviewInspectionMatrixViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in PreviewSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(PreviewInspectionMatrixViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_narrowing() && row.narrowing_consistent())
    {
        violations.push(PreviewInspectionMatrixViolation::NarrowedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &PreviewInspectionMatrixPacket,
    violations: &mut Vec<PreviewInspectionMatrixViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(PreviewInspectionMatrixViolation::RowIncomplete);
        }
        if row.needs_narrowing()
            && row.effective_qualification.rank() >= row.claimed_qualification.rank()
        {
            violations
                .push(PreviewInspectionMatrixViolation::RowNotNarrowedOnUnidentifiedDimension);
        }
        if row.needs_narrowing()
            && (row.narrow_trigger.is_none()
                || !row
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(PreviewInspectionMatrixViolation::NarrowedRowMissingLabelOrTrigger);
        }
        if !row.runtime_masquerade_ok() {
            violations.push(PreviewInspectionMatrixViolation::RuntimeOnlyMasqueradesAsSource);
        }
        if !row.inspect_only_ok() {
            violations.push(PreviewInspectionMatrixViolation::InspectOnlyRowClaimsWrite);
        }
        if !row.source_diff_preview_ok() {
            violations
                .push(PreviewInspectionMatrixViolation::WriteCapableRowSkipsSourceDiffPreview);
        }
        if !row.attach_depth_consistent() {
            violations.push(PreviewInspectionMatrixViolation::AttachDepthInconsistentWithTarget);
        }
        if !row.round_trip_mapping_consistent() {
            violations.push(PreviewInspectionMatrixViolation::ExactRoundTripWithoutExactMapping);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(PreviewInspectionMatrixViolation::RowEvidenceMissing);
        }
        if row.is_claimed() && row.evidence_refs.is_empty() {
            violations.push(PreviewInspectionMatrixViolation::ClaimedRowMissingEvidence);
        }
    }
}

fn validate_guardrails(
    packet: &PreviewInspectionMatrixPacket,
    violations: &mut Vec<PreviewInspectionMatrixViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(PreviewInspectionMatrixViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &PreviewInspectionMatrixPacket,
    violations: &mut Vec<PreviewInspectionMatrixViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(PreviewInspectionMatrixViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_evidence_freshness(
    packet: &PreviewInspectionMatrixPacket,
    violations: &mut Vec<PreviewInspectionMatrixViolation>,
) {
    if packet.evidence_freshness.evidence_freshness_slo_hours == 0
        || packet
            .evidence_freshness
            .last_evidence_refresh
            .trim()
            .is_empty()
    {
        violations.push(PreviewInspectionMatrixViolation::EvidenceFreshnessIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise narrowing truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "narrowed"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
