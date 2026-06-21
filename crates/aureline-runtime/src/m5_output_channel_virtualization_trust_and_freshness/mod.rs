//! Canonical per-channel truth for M5 output channels: stream-first searchable
//! virtualization, content trust classes, pin/export controls, and live-vs-cached
//! -vs-stale channel freshness with run/step/provider/artifact lineage intact.
//!
//! Where [`crate::m5_execution_evidence_causality_matrix`] froze the *lane* matrix
//! — one row per Problems/output/execution-evidence **surface family** —,
//! [`crate::m5_problem_records_source_task_correlation_and_rerun_jump_parity`] froze
//! the **individual Problems row**, and
//! [`crate::m5_execution_evidence_projection_overlays`] froze the **projected
//! overlay**, this module freezes the **individual output channel**: a raw log
//! stream, a trusted structured report, an HTML report bundle, a generated artifact,
//! or a trace/profile output rendered into the shell, terminal, Problems panel,
//! review surface, timeline, support bundle, or AI-evidence consumer. Each
//! [`OutputChannelRecord`] binds its channel to the *original*
//! run/step/provider/artifact lineage, the stream-first virtualization profile that
//! keeps a large log searchable and exportable without full materialization, the
//! content trust class and pin/export controls that keep safe-preview distinct from
//! active/open-in-external content, and the live/cached/stale freshness with
//! fetched-at and provider-unreachable cues — so a large log never forces full
//! materialization into shell memory, a user can always tell raw / safe-preview /
//! trusted-structured / untrusted-active content apart before copying, exporting, or
//! opening it, and a provider-backed channel can never masquerade as live after a
//! freshness threshold or a lost connection.
//!
//! The channel speaks the **same** frozen vocabulary as the causality matrix
//! ([`ClaimPosture`], [`OriginClass`], [`OutputChannelClass`], [`ConfidenceTier`],
//! [`FreshnessState`], [`ReopenTarget`], [`ProofCurrency`],
//! [`VerificationFreshness`]) rather than forking a private channel truth model.
//! Reuse the canonical run/step/provider refs, generated-artifact ids, output
//! channels, and evidence packets already landed earlier; this module binds them to
//! one inspectable, reopenable channel.
//!
//! Re-derivation rules ([`OutputChannelRecord::narrow`]):
//!
//! * Every channel keeps its **canonical channel id and origin run/step/provider
//!   identity reopenable on demand** on every surface it renders: the output pane,
//!   a Problems row, a review overlay, and a support export must all be able to
//!   answer "which run, which step, which provider produced this channel" without
//!   stitching raw logs.
//! * Every **large** channel stays **stream-first, searchable, and exportable**
//!   with stable chunk ids and bounded memory; a large channel that cannot stream,
//!   search, bound memory, or export without full materialization floors rather than
//!   pretending it can be safely opened whole.
//! * Every channel carries an explicit **content trust class** and keeps
//!   **safe-preview distinct from active/open-in-external**: untrusted active content
//!   never opens externally without confirmation, an export never leaks active
//!   content, and the trust boundary is never blurred merely for a cleaner UI.
//! * **Provider-backed/imported** channels disclose **fetched-at** and
//!   **provider-unreachable** cues and never claim live local authority after a
//!   freshness threshold or a connection loss.
//! * A channel that flattens channel/run/provider lineage, hides lineage from a
//!   surface, drops a heuristic raw-output backlink, loses its reopen-to-origin
//!   path, lets a rendering surface overclaim, blurs the trust boundary, forces full
//!   materialization on export, or lets a provider-backed channel masquerade as live
//!   floors to [`ChannelClaim::Unreconstructable`] and keeps a raw-output / keyboard
//!   fallback rather than rendering a clean-but-false channel. Stale/labelled gaps
//!   hold a first-party channel at [`ChannelClaim::Narrowed`] (still reopenable).
//!   Labs/unadvertised channels make no public claim and are never widened.
//!
//! [`M5OutputChannelSetPacket::validate`] confirms the packet is well-formed and
//! honest: header/identity/redaction/freshness are present, every payload kind, every
//! content trust class, and every rendering surface is represented, overlay channels
//! name their provider, no rendering surface overclaims its channel, a floored channel
//! keeps a raw fallback, at least one channel demonstrates the auto-narrowing rule,
//! and no raw boundary material crosses the export. Downstream shell, terminal,
//! Problems, debug, pipeline, notebook, support-export, AI-evidence, and docs surfaces
//! ingest this packet rather than inventing a parallel channel model.
//!
//! Raw stdout/stderr bytes, command lines, provider log bodies, env bodies, absolute
//! paths, URLs, and secrets never cross this boundary; the packet carries only typed
//! class tokens, chunk counts/byte sizes, booleans, opaque ids, and redaction-aware
//! reviewable labels.
//!
//! The boundary schema is
//! [`schemas/tooling/m5-output-channels.schema.json`](../../../../schemas/tooling/m5-output-channels.schema.json).
//! The contract doc is
//! [`docs/tooling/m5-output-channels.md`](../../../../docs/tooling/m5-output-channels.md).
//! The canonical support export is
//! [`artifacts/tooling/m5-output-channels/support_export.json`](../../../../artifacts/tooling/m5-output-channels/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/tooling/m5-output-channels/`](../../../../fixtures/tooling/m5-output-channels/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_execution_evidence_causality_matrix::{
    json_contains_forbidden_boundary_material, label_is_generic, parse_rfc3339_to_epoch_seconds,
    ClaimPosture, ConfidenceTier, FreshnessState, OriginClass, OutputChannelClass, ProofCurrency,
    ReopenTarget, VerificationFreshness,
};

/// Stable record-kind tag carried by [`M5OutputChannelSetPacket`].
pub const M5_OUTPUT_CHANNELS_RECORD_KIND: &str = "m5_output_channel_set_packet";

/// Schema version for the channel set.
pub const M5_OUTPUT_CHANNELS_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_OUTPUT_CHANNELS_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical channel-set packet.
pub const M5_OUTPUT_CHANNELS_PACKET_ID: &str = "m5-output-channels:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_OUTPUT_CHANNELS_SCHEMA_REF: &str = "schemas/tooling/m5-output-channels.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_OUTPUT_CHANNELS_DOC_REF: &str = "docs/tooling/m5-output-channels.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_OUTPUT_CHANNELS_SUPPORT_EXPORT_REF: &str =
    "artifacts/tooling/m5-output-channels/support_export.json";

/// Repo-relative path of the generated certification report.
pub const M5_OUTPUT_CHANNELS_REPORT_REF: &str = "artifacts/tooling/m5-output-channels/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_OUTPUT_CHANNELS_FIXTURE_DIR: &str = "fixtures/tooling/m5-output-channels";

/// Chunk-count threshold above which a channel is large enough that stream-first
/// virtualization is mandatory rather than optional.
pub const LARGE_CHANNEL_CHUNK_THRESHOLD: u64 = 256;

/// Allowed packet redaction-class tokens.
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

/// Deterministic seed timestamp for the canonical packet and report.
const SEED_AS_OF: &str = "2026-06-21T00:00:00Z";

// --------------------------------------------------------------------------- //
// Frozen channel taxonomies (mirror the boundary schema).
// --------------------------------------------------------------------------- //

/// What an output channel carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelPayloadKind {
    /// Raw stdout/stderr log stream.
    RawLogStream,
    /// Structured machine-readable report (test JSON, lint SARIF-like, …).
    StructuredReport,
    /// Self-contained HTML report bundle.
    HtmlReportBundle,
    /// Generated binary/opaque artifact.
    GeneratedArtifact,
    /// Trace / profile output (flamegraph, trace file, …).
    TraceProfileOutput,
}

impl ChannelPayloadKind {
    /// Every payload kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RawLogStream,
        Self::StructuredReport,
        Self::HtmlReportBundle,
        Self::GeneratedArtifact,
        Self::TraceProfileOutput,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawLogStream => "raw_log_stream",
            Self::StructuredReport => "structured_report",
            Self::HtmlReportBundle => "html_report_bundle",
            Self::GeneratedArtifact => "generated_artifact",
            Self::TraceProfileOutput => "trace_profile_output",
        }
    }
}

/// The content trust class a channel surfaces before copy/export/open. Keeping these
/// distinct is what lets the shell warn before opening active content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentTrustClass {
    /// Raw bytes; previewable as inert text, never executed.
    Raw,
    /// Rendered safe preview (sanitized, no active content).
    SafePreview,
    /// Structured report from a trusted parser.
    TrustedStructured,
    /// Untrusted active content (HTML bundle, embedded scripts) that must open in an
    /// external/sandboxed viewer with confirmation.
    UntrustedActive,
}

impl ContentTrustClass {
    /// Every trust class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Raw,
        Self::SafePreview,
        Self::TrustedStructured,
        Self::UntrustedActive,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::SafePreview => "safe_preview",
            Self::TrustedStructured => "trusted_structured",
            Self::UntrustedActive => "untrusted_active",
        }
    }

    /// Whether this class is untrusted active content, which must never open
    /// externally without confirmation and must never leak through an export.
    pub const fn is_active_content(self) -> bool {
        matches!(self, Self::UntrustedActive)
    }
}

/// A surface on which an output channel is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelSurface {
    /// Output pane.
    OutputPane,
    /// Terminal pane.
    TerminalPane,
    /// Problems panel.
    ProblemsPanel,
    /// Diff / review overlay.
    DiffReviewOverlay,
    /// Activity-center timeline / history.
    TimelineHistory,
    /// Support export bundle.
    SupportExport,
    /// AI-evidence consumer.
    AiEvidence,
}

impl ChannelSurface {
    /// Every rendering surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::OutputPane,
        Self::TerminalPane,
        Self::ProblemsPanel,
        Self::DiffReviewOverlay,
        Self::TimelineHistory,
        Self::SupportExport,
        Self::AiEvidence,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutputPane => "output_pane",
            Self::TerminalPane => "terminal_pane",
            Self::ProblemsPanel => "problems_panel",
            Self::DiffReviewOverlay => "diff_review_overlay",
            Self::TimelineHistory => "timeline_history",
            Self::SupportExport => "support_export",
            Self::AiEvidence => "ai_evidence",
        }
    }
}

// --------------------------------------------------------------------------- //
// Derived channel-claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim a channel renders. A higher rank asserts more authority, so a
/// narrowed or floored channel must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelClaim {
    /// Lineage/identity/virtualization/trust broken; the channel surfaces a
    /// raw-output backlink or keyboard fallback instead of a clean-but-false channel.
    #[serde(rename = "channel_unreconstructable")]
    Unreconstructable,
    /// Remote/pipeline/imported channel; attributable and reopenable but never claims
    /// live local authority.
    #[serde(rename = "channel_read_only_overlay")]
    ReadOnlyOverlay,
    /// A first-party channel held below certified by a stale/labelled gap, but lineage
    /// stays reopenable.
    #[serde(rename = "channel_narrowed")]
    Narrowed,
    /// Full first-party lineage preserved, virtualized, trust-honest, fresh,
    /// reopenable.
    #[serde(rename = "channel_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "channel_labs_not_claimed")]
    LabsNotClaimed,
}

impl ChannelClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unreconstructable => "channel_unreconstructable",
            Self::ReadOnlyOverlay => "channel_read_only_overlay",
            Self::Narrowed => "channel_narrowed",
            Self::Certified => "channel_certified",
            Self::LabsNotClaimed => "channel_labs_not_claimed",
        }
    }

    /// Monotonic rank, or `None` for the non-claiming Labs token.
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::Unreconstructable => Some(0),
            Self::ReadOnlyOverlay => Some(1),
            Self::Narrowed => Some(2),
            Self::Certified => Some(3),
            Self::LabsNotClaimed => None,
        }
    }

    /// Whether rendering `rendered` would overclaim relative to this effective claim.
    /// A rendering surface must never render wider than the channel's effective claim;
    /// the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: ChannelClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a channel fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelNarrowingReason {
    /// Channel lost its stable canonical channel ref.
    #[serde(rename = "channel_identity_flattened")]
    ChannelIdentityFlattened,
    /// Origin run/step lineage flattened away from the channel.
    #[serde(rename = "run_step_lineage_flattened")]
    RunStepLineageFlattened,
    /// Provider identity flattened away from the channel.
    #[serde(rename = "provider_identity_flattened")]
    ProviderIdentityFlattened,
    /// Lineage cannot be revealed on demand on some rendering surface.
    #[serde(rename = "lineage_not_visible")]
    LineageNotVisible,
    /// Reopen-to-origin lost; only a keyboard fallback remains.
    #[serde(rename = "reopen_target_lost")]
    ReopenTargetLost,
    /// Heuristic channel without a raw-output backlink.
    #[serde(rename = "raw_output_backlink_missing")]
    RawBacklinkMissing,
    /// A large log is not stream-first / searchable.
    #[serde(rename = "stream_not_virtualized")]
    StreamNotVirtualized,
    /// A large log does not bound its retained memory.
    #[serde(rename = "unbounded_memory")]
    UnboundedMemory,
    /// Exporting a large log would force full materialization into shell memory.
    #[serde(rename = "export_forces_full_materialization")]
    ExportForcesFullMaterialization,
    /// Safe-preview versus active/open-in-external boundary blurred.
    #[serde(rename = "trust_boundary_blurred")]
    TrustBoundaryBlurred,
    /// Untrusted active content opens externally without confirmation.
    #[serde(rename = "active_content_auto_opens")]
    ActiveContentAutoOpens,
    /// Export would leak active/untrusted content.
    #[serde(rename = "export_unsafe")]
    ExportUnsafe,
    /// A rendering surface renders a claim wider than the effective claim.
    #[serde(rename = "surface_overclaims")]
    SurfaceOverclaims,
    /// Imported/remote/pipeline channel claims live local authority.
    #[serde(rename = "imported_channel_claims_live")]
    ImportedChannelClaimsLive,
    /// Provider-backed channel masquerades as live after a freshness threshold or
    /// connection loss.
    #[serde(rename = "stale_channel_claims_live")]
    StaleChannelClaimsLive,
    /// Channel content missing.
    #[serde(rename = "channel_content_missing")]
    ChannelContentMissing,
    /// Content trust class not surfaced before copy/export/open.
    #[serde(rename = "trust_class_unlabeled")]
    TrustClassUnlabeled,
    /// Chunk ids/ranges not stable across a refresh of a large log.
    #[serde(rename = "chunk_ids_unstable")]
    ChunkIdsUnstable,
    /// Follow/scroll mode unavailable on a large log.
    #[serde(rename = "follow_mode_unavailable")]
    FollowModeUnavailable,
    /// No safe-preview path before opening the channel.
    #[serde(rename = "safe_preview_unavailable")]
    SafePreviewUnavailable,
    /// Pin control unavailable.
    #[serde(rename = "pin_control_unavailable")]
    PinControlUnavailable,
    /// Export control unavailable.
    #[serde(rename = "export_control_unavailable")]
    ExportControlUnavailable,
    /// Provider-backed channel without a fetched-at cue.
    #[serde(rename = "fetched_at_missing")]
    FetchedAtMissing,
    /// Provider unreachable but the cue is not surfaced.
    #[serde(rename = "provider_unreachable_unmarked")]
    ProviderUnreachableUnmarked,
    /// Channel freshness state not surfaced.
    #[serde(rename = "freshness_unlabeled")]
    FreshnessUnlabeled,
    /// Confidence tier not surfaced.
    #[serde(rename = "confidence_unlabeled")]
    ConfidenceUnlabeled,
    /// Superseded-by-newer-run state not marked.
    #[serde(rename = "superseded_state_not_marked")]
    SupersededNotMarked,
    /// First-party channel content stale.
    #[serde(rename = "channel_stale")]
    StaleEvidence,
    /// Verification proof stale or window elapsed.
    #[serde(rename = "verification_proof_stale")]
    StaleProof,
    /// Verification proof missing.
    #[serde(rename = "verification_proof_missing")]
    MissingProof,
}

impl ChannelNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelIdentityFlattened => "channel_identity_flattened",
            Self::RunStepLineageFlattened => "run_step_lineage_flattened",
            Self::ProviderIdentityFlattened => "provider_identity_flattened",
            Self::LineageNotVisible => "lineage_not_visible",
            Self::ReopenTargetLost => "reopen_target_lost",
            Self::RawBacklinkMissing => "raw_output_backlink_missing",
            Self::StreamNotVirtualized => "stream_not_virtualized",
            Self::UnboundedMemory => "unbounded_memory",
            Self::ExportForcesFullMaterialization => "export_forces_full_materialization",
            Self::TrustBoundaryBlurred => "trust_boundary_blurred",
            Self::ActiveContentAutoOpens => "active_content_auto_opens",
            Self::ExportUnsafe => "export_unsafe",
            Self::SurfaceOverclaims => "surface_overclaims",
            Self::ImportedChannelClaimsLive => "imported_channel_claims_live",
            Self::StaleChannelClaimsLive => "stale_channel_claims_live",
            Self::ChannelContentMissing => "channel_content_missing",
            Self::TrustClassUnlabeled => "trust_class_unlabeled",
            Self::ChunkIdsUnstable => "chunk_ids_unstable",
            Self::FollowModeUnavailable => "follow_mode_unavailable",
            Self::SafePreviewUnavailable => "safe_preview_unavailable",
            Self::PinControlUnavailable => "pin_control_unavailable",
            Self::ExportControlUnavailable => "export_control_unavailable",
            Self::FetchedAtMissing => "fetched_at_missing",
            Self::ProviderUnreachableUnmarked => "provider_unreachable_unmarked",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::ConfidenceUnlabeled => "confidence_unlabeled",
            Self::SupersededNotMarked => "superseded_state_not_marked",
            Self::StaleEvidence => "channel_stale",
            Self::StaleProof => "verification_proof_stale",
            Self::MissingProof => "verification_proof_missing",
        }
    }

    /// Whether this reason floors a channel to [`ChannelClaim::Unreconstructable`].
    /// Each floor reason breaks the "stay reopenable / never flatten lineage / never
    /// open active content silently / never masquerade as live" contract outright
    /// rather than merely aging out.
    pub const fn is_floor(self) -> bool {
        matches!(
            self,
            Self::ChannelIdentityFlattened
                | Self::RunStepLineageFlattened
                | Self::ProviderIdentityFlattened
                | Self::LineageNotVisible
                | Self::ReopenTargetLost
                | Self::RawBacklinkMissing
                | Self::StreamNotVirtualized
                | Self::UnboundedMemory
                | Self::ExportForcesFullMaterialization
                | Self::TrustBoundaryBlurred
                | Self::ActiveContentAutoOpens
                | Self::ExportUnsafe
                | Self::SurfaceOverclaims
                | Self::ImportedChannelClaimsLive
                | Self::StaleChannelClaimsLive
                | Self::ChannelContentMissing
        )
    }

    /// Deterministic ordering index so recorded reason lists are stable across runs.
    /// Floor reasons sort first so the headline trigger is the most severe.
    const fn order_index(self) -> u8 {
        match self {
            Self::ChannelIdentityFlattened => 0,
            Self::RunStepLineageFlattened => 1,
            Self::ProviderIdentityFlattened => 2,
            Self::LineageNotVisible => 3,
            Self::ReopenTargetLost => 4,
            Self::RawBacklinkMissing => 5,
            Self::StreamNotVirtualized => 6,
            Self::UnboundedMemory => 7,
            Self::ExportForcesFullMaterialization => 8,
            Self::TrustBoundaryBlurred => 9,
            Self::ActiveContentAutoOpens => 10,
            Self::ExportUnsafe => 11,
            Self::SurfaceOverclaims => 12,
            Self::ImportedChannelClaimsLive => 13,
            Self::StaleChannelClaimsLive => 14,
            Self::ChannelContentMissing => 15,
            Self::TrustClassUnlabeled => 16,
            Self::ChunkIdsUnstable => 17,
            Self::FollowModeUnavailable => 18,
            Self::SafePreviewUnavailable => 19,
            Self::PinControlUnavailable => 20,
            Self::ExportControlUnavailable => 21,
            Self::FetchedAtMissing => 22,
            Self::ProviderUnreachableUnmarked => 23,
            Self::FreshnessUnlabeled => 24,
            Self::ConfidenceUnlabeled => 25,
            Self::SupersededNotMarked => 26,
            Self::StaleEvidence => 27,
            Self::StaleProof => 28,
            Self::MissingProof => 29,
        }
    }
}

/// Sort reasons by their canonical order and drop duplicates.
fn order_reasons(mut reasons: Vec<ChannelNarrowingReason>) -> Vec<ChannelNarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Channel sub-objects.
// --------------------------------------------------------------------------- //

/// Stable identifiers binding a channel to its origin. Lineage is reconstructed from
/// these refs, never inferred from freeform display text. Absent refs serialize as
/// `null` so the schema's required keys stay present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelLineage {
    /// Execution-context ref (required).
    pub execution_context_ref: String,
    /// The channel's own stable canonical ref (required for a real channel).
    pub canonical_channel_ref: Option<String>,
    /// Origin run ref.
    pub origin_run_ref: Option<String>,
    /// Origin step ref.
    pub origin_step_ref: Option<String>,
    /// Provider ref (required for remote/pipeline/imported channels).
    pub provider_ref: Option<String>,
    /// Generated-artifact ref backing the channel.
    pub artifact_ref: Option<String>,
    /// Evidence-bundle / packet ref.
    pub evidence_packet_ref: Option<String>,
    /// Raw-output backlink ref.
    pub raw_output_backlink_ref: Option<String>,
}

/// The stream-first virtualization profile keeping a large log searchable and
/// exportable without full materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualizationProfile {
    /// Whether this channel is large enough that virtualization is mandatory.
    pub large_log: bool,
    /// Channel is read as a stream of windowed chunks rather than one buffer.
    pub stream_first: bool,
    /// Chunk windows are searchable without materializing the whole log.
    pub searchable: bool,
    /// Chunk ids/ranges stay stable across a refresh.
    pub stable_chunk_ids: bool,
    /// Follow/scroll (tail) mode is supported.
    pub follow_mode_supported: bool,
    /// Retained memory is bounded by a window rather than the whole log.
    pub bounded_memory: bool,
    /// Export streams to disk rather than materializing the whole log into memory.
    pub exportable_without_full_materialization: bool,
    /// Total chunk count produced by the run.
    pub total_chunk_count: u64,
    /// Chunks retained in the in-memory window.
    pub retained_window_chunks: u64,
    /// Approximate total byte size of the channel.
    pub approx_total_bytes: u64,
    /// Maximum bytes retained in memory at once.
    pub max_retained_bytes: u64,
}

/// The content trust class and pin/export controls a channel surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelAccessControls {
    /// The content trust class is surfaced before copy/export/open.
    pub trust_class_labeled: bool,
    /// A safe-preview path exists before opening the channel.
    pub safe_preview_available: bool,
    /// The channel can be pinned so it is not evicted.
    pub pin_supported: bool,
    /// The channel can be exported.
    pub export_supported: bool,
    /// Export is safe: it does not leak active content.
    pub export_is_safe: bool,
    /// Untrusted active content requires confirmation before opening externally.
    pub open_in_external_requires_confirmation: bool,
    /// The safe-preview versus active/open-in-external boundary is preserved.
    pub trust_boundary_preserved: bool,
}

/// The live/cached/stale freshness and provider-reachability cues a channel carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelFreshness {
    /// Whether this channel is provider-backed or imported.
    pub provider_backed: bool,
    /// A fetched-at cue is present (required for provider-backed channels).
    pub fetched_at_present: bool,
    /// The provider is currently reachable.
    pub provider_reachable: bool,
    /// When unreachable, the provider-unreachable cue is surfaced.
    pub provider_unreachable_marked: bool,
    /// The channel does not claim live local authority once stale or disconnected.
    pub live_state_honest: bool,
}

/// The channel-integrity invariants every channel re-derives rather than trusting a
/// grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelIntegrity {
    /// Origin run/step lineage survives into the channel.
    pub preserves_run_step_lineage: bool,
    /// Provider identity survives into the channel.
    pub preserves_provider_identity: bool,
    /// Origin lineage can be revealed on demand on every rendering surface.
    pub lineage_visible_on_demand: bool,
    /// The freshness state is surfaced rather than hidden.
    pub freshness_state_labeled: bool,
    /// The confidence tier is surfaced rather than hidden.
    pub confidence_label_visible: bool,
    /// Superseded state stays marked.
    pub superseded_state_marked: bool,
    /// Imported channels stay read-only.
    pub imported_channel_read_only: bool,
    /// A heuristic channel keeps a raw-output backlink.
    pub raw_output_backlink_present: bool,
}

/// Certification-proof currency for a channel (distinct from the channel's own
/// freshness state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelVerification {
    /// Currency of the certification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the channel.
    pub proof_ref: Option<String>,
}

/// One surface that renders a channel, with the claim it shows and whether it can
/// reveal the origin lineage on demand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelRendering {
    /// The rendering surface.
    pub surface: ChannelSurface,
    /// The claim this surface renders.
    pub rendered_claim: ChannelClaim,
    /// Whether the origin run/step/provider lineage is revealable here.
    pub lineage_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical channel this surface re-renders.
    pub source_channel_ref: String,
}

// --------------------------------------------------------------------------- //
// Channel + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) output channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputChannelRecord {
    /// Stable channel id.
    pub channel_id: String,
    /// What the channel carries.
    pub payload_kind: ChannelPayloadKind,
    /// Producer/transport class.
    pub channel_class: OutputChannelClass,
    /// Content trust class.
    pub trust_class: ContentTrustClass,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Whether the channel is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// How the run/evidence originated.
    pub origin_class: OriginClass,
    /// Declared confidence tier.
    pub declared_confidence_tier: ConfidenceTier,
    /// Declared freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared reopen target.
    pub declared_reopen_target: ReopenTarget,
    /// Stable origin-lineage block.
    pub lineage: ChannelLineage,
    /// Stream-first virtualization block.
    pub virtualization: VirtualizationProfile,
    /// Trust-class + pin/export control block.
    pub access: ChannelAccessControls,
    /// Live/cached/stale freshness block.
    pub freshness: ChannelFreshness,
    /// Channel-integrity invariant block.
    pub integrity: ChannelIntegrity,
    /// Certification-proof block.
    pub verification: ChannelVerification,
    /// Surfaces that render this channel.
    pub renderings: Vec<ChannelRendering>,
}

/// The re-derived channel decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelDecision {
    /// The headline claim the channel is eligible to make.
    pub claimed_channel_claim: ChannelClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_channel_claim: ChannelClaim,
    /// Ordered, de-duplicated reasons the channel fails to hold its headline.
    pub active_narrowing_reasons: Vec<ChannelNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl ChannelDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<ChannelNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this channel would overclaim.
    pub fn surface_overclaims(&self, rendered: ChannelClaim) -> bool {
        self.effective_channel_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(claimed: ChannelClaim, reasons: &[ChannelNarrowingReason]) -> ChannelClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        ChannelClaim::Unreconstructable
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, ChannelClaim::ReadOnlyOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we can
        // no longer certify even the read-only overlay, so it floors.
        ChannelClaim::Unreconstructable
    } else {
        ChannelClaim::Narrowed
    }
}

impl OutputChannelRecord {
    /// Whether this channel is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this channel is an inherently read-only overlay origin.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin_class.is_overlay()
    }

    /// Whether this channel is large enough that stream-first virtualization is
    /// mandatory rather than optional.
    pub fn requires_virtualization(&self) -> bool {
        self.virtualization.large_log
            || self.virtualization.total_chunk_count > LARGE_CHANNEL_CHUNK_THRESHOLD
    }

    /// The headline channel claim this channel is eligible to make.
    pub fn claimed_claim(&self) -> ChannelClaim {
        if self.is_labs() {
            ChannelClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            ChannelClaim::ReadOnlyOverlay
        } else {
            ChannelClaim::Certified
        }
    }

    /// Whether this channel's confidence is one of the explicit heuristic tiers, which
    /// must keep a raw-output backlink.
    fn is_heuristic(&self) -> bool {
        self.declared_confidence_tier.is_heuristic_tier()
    }

    /// Reasons that hold independently of how the rendering surfaces render — the
    /// intrinsic lineage/virtualization/trust/freshness gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<ChannelNarrowingReason> {
        use ChannelNarrowingReason as R;
        let integ = &self.integrity;
        let virt = &self.virtualization;
        let access = &self.access;
        let fresh = &self.freshness;
        let overlay = self.is_overlay_origin();
        let requires_virt = self.requires_virtualization();
        let mut reasons: Vec<R> = Vec::new();

        // Channel identity + origin lineage.
        if self.channel_class.is_real_channel() && !opt_present(&self.lineage.canonical_channel_ref)
        {
            reasons.push(R::ChannelIdentityFlattened);
        }
        if !integ.preserves_run_step_lineage {
            reasons.push(R::RunStepLineageFlattened);
        }
        if !integ.preserves_provider_identity {
            reasons.push(R::ProviderIdentityFlattened);
        }
        if !integ.lineage_visible_on_demand || self.renderings.iter().any(|r| !r.lineage_visible) {
            reasons.push(R::LineageNotVisible);
        }

        // A heuristic channel must keep a raw-output backlink and a tier label.
        if self.is_heuristic() && !integ.raw_output_backlink_present {
            reasons.push(R::RawBacklinkMissing);
        }
        if !integ.confidence_label_visible {
            reasons.push(R::ConfidenceUnlabeled);
        }

        // Stream-first virtualization is mandatory for large logs.
        if requires_virt {
            if !virt.stream_first || !virt.searchable {
                reasons.push(R::StreamNotVirtualized);
            }
            if !virt.bounded_memory {
                reasons.push(R::UnboundedMemory);
            }
            if access.export_supported && !virt.exportable_without_full_materialization {
                reasons.push(R::ExportForcesFullMaterialization);
            }
            if !virt.stable_chunk_ids {
                reasons.push(R::ChunkIdsUnstable);
            }
            if !virt.follow_mode_supported {
                reasons.push(R::FollowModeUnavailable);
            }
        }

        // Content trust classes and pin/export controls.
        if !access.trust_class_labeled {
            reasons.push(R::TrustClassUnlabeled);
        }
        if !access.trust_boundary_preserved {
            reasons.push(R::TrustBoundaryBlurred);
        }
        if self.trust_class.is_active_content() && !access.open_in_external_requires_confirmation {
            reasons.push(R::ActiveContentAutoOpens);
        }
        if access.export_supported && !access.export_is_safe {
            reasons.push(R::ExportUnsafe);
        }
        if !access.safe_preview_available {
            reasons.push(R::SafePreviewUnavailable);
        }
        if !access.pin_supported {
            reasons.push(R::PinControlUnavailable);
        }
        if !access.export_supported {
            reasons.push(R::ExportControlUnavailable);
        }

        // Reopen-to-origin must survive.
        if matches!(
            self.declared_reopen_target,
            ReopenTarget::NoneKeyboardFallback
        ) {
            reasons.push(R::ReopenTargetLost);
        }

        // Freshness must be labelled, and provider-backed channels disclose cues and
        // never masquerade as live.
        if !integ.freshness_state_labeled {
            reasons.push(R::FreshnessUnlabeled);
        }
        if fresh.provider_backed {
            if !fresh.live_state_honest {
                reasons.push(R::StaleChannelClaimsLive);
            }
            if !fresh.fetched_at_present {
                reasons.push(R::FetchedAtMissing);
            }
            if !fresh.provider_reachable && !fresh.provider_unreachable_marked {
                reasons.push(R::ProviderUnreachableUnmarked);
            }
        }
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::ChannelContentMissing),
            FreshnessState::SupersededByNewerRun if !integ.superseded_state_marked => {
                reasons.push(R::SupersededNotMarked);
            }
            // An overlay snapshot is expected to be cached/stale; a first-party live
            // channel showing a stale snapshot has aged out of currency.
            FreshnessState::StaleExpired if !overlay => reasons.push(R::StaleEvidence),
            _ => {}
        }

        // Certification-proof currency (distinct from the channel's own freshness).
        match self.verification.proof_currency {
            ProofCurrency::MissingProof => reasons.push(R::MissingProof),
            ProofCurrency::StaleExpired | ProofCurrency::RequiresReview => {
                reasons.push(R::StaleProof);
            }
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow if stale_window => {
                reasons.push(R::StaleProof);
            }
            _ => {}
        }

        // Imported/remote/pipeline channels must stay read-only.
        if overlay && !integ.imported_channel_read_only {
            reasons.push(R::ImportedChannelClaimsLive);
        }

        reasons
    }

    /// Every reason this channel fails to hold its headline claim, including a
    /// rendering surface that overclaims relative to the intrinsic effective claim.
    pub fn channel_reasons(&self, stale_window: bool) -> Vec<ChannelNarrowingReason> {
        let claimed = self.claimed_claim();
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic = derive_effective(claimed, &reasons);
        if self
            .renderings
            .iter()
            .any(|r| intrinsic.overclaims_as(r.rendered_claim))
        {
            reasons.push(ChannelNarrowingReason::SurfaceOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive the effective channel claim, reasons, and narrowed flag.
    pub fn narrow(&self, stale_window: bool) -> ChannelDecision {
        let claimed = self.claimed_claim();

        // Labs/unadvertised channels make no public claim, so they never accrue
        // governance narrowing; they hold their non-claiming token.
        if matches!(claimed, ChannelClaim::LabsNotClaimed) {
            return ChannelDecision {
                claimed_channel_claim: ChannelClaim::LabsNotClaimed,
                effective_channel_claim: ChannelClaim::LabsNotClaimed,
                active_narrowing_reasons: Vec::new(),
                narrowed: false,
            };
        }

        let reasons = self.channel_reasons(stale_window);
        let effective = derive_effective(claimed, &reasons);
        let narrowed = matches!(
            (effective.rank(), claimed.rank()),
            (Some(eff), Some(claim)) if eff < claim
        );

        ChannelDecision {
            claimed_channel_claim: claimed,
            effective_channel_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// The effective confidence tier: a floored channel cannot assert a tier beyond
    /// unmapped/needs-review.
    pub fn effective_confidence(&self, effective: ChannelClaim) -> ConfidenceTier {
        if matches!(effective, ChannelClaim::Unreconstructable) {
            ConfidenceTier::UnmappedRequiresReview
        } else {
            self.declared_confidence_tier
        }
    }

    /// A precise, non-generic reviewer label for a narrowed/floored channel.
    pub fn narrowed_label(&self, decision: &ChannelDecision) -> Option<String> {
        if !decision.narrowed {
            return None;
        }
        let trigger = decision
            .downgrade_trigger()
            .map_or("narrowed", ChannelNarrowingReason::as_str)
            .replace('_', " ");
        let reopen = self.declared_reopen_target.as_str().replace('_', " ");
        let claimed = decision.claimed_channel_claim.as_str();
        let effective = decision.effective_channel_claim;
        let label = if matches!(effective, ChannelClaim::Unreconstructable) {
            format!(
                "Floored to {} below the {claimed} claim: {trigger}; the {reopen} stays reopenable rather than rendering a clean-but-false channel",
                effective.as_str()
            )
        } else {
            format!(
                "Held at {} below the {claimed} claim: {trigger}; lineage stays reopenable via the {reopen} until re-verified",
                effective.as_str()
            )
        };
        Some(label)
    }

    /// Whether a non-labs channel that floors keeps a reopen fallback rather than
    /// hiding lineage behind a clean-but-false claim.
    fn floored_keeps_fallback(&self, effective: ChannelClaim) -> bool {
        if !matches!(effective, ChannelClaim::Unreconstructable) {
            return true;
        }
        self.declared_reopen_target.is_raw_fallback()
            || self.integrity.raw_output_backlink_present
            || opt_present(&self.lineage.raw_output_backlink_ref)
    }

    /// Whether any rendering surface renders wider than the channel's effective claim.
    fn surface_overclaims(&self, effective: ChannelClaim) -> bool {
        self.renderings
            .iter()
            .any(|r| effective.overclaims_as(r.rendered_claim))
    }

    /// Structural checks that hold independently of the narrowing derivation.
    fn structural_violations(&self, out: &mut Vec<M5OutputChannelViolation>) {
        if self.channel_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.lineage.execution_context_ref.trim().is_empty()
        {
            out.push(M5OutputChannelViolation::ChannelMissingIdentity);
        }
        if self.is_overlay_origin() && !opt_present(&self.lineage.provider_ref) {
            out.push(M5OutputChannelViolation::OverlayMissingProviderRef);
        }
        if self.renderings.is_empty() {
            out.push(M5OutputChannelViolation::ChannelMissingRendering);
        }
        for rendering in &self.renderings {
            if rendering.source_channel_ref.trim().is_empty() {
                out.push(M5OutputChannelViolation::RenderingMissingSourceRef);
            }
        }
    }
}

/// Whether an optional ref is present and non-empty.
fn opt_present(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|inner| !inner.trim().is_empty())
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for an [`M5OutputChannelSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OutputChannelSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-channel rows.
    pub channels: Vec<OutputChannelRecord>,
}

/// Export-safe M5 output-channel set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OutputChannelSetPacket {
    /// Record kind; must equal [`M5_OUTPUT_CHANNELS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_OUTPUT_CHANNELS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_OUTPUT_CHANNELS_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-channel rows.
    pub channels: Vec<OutputChannelRecord>,
}

/// The distribution of effective channel claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelClaimDistribution {
    /// Channels effective at [`ChannelClaim::Certified`].
    pub certified: usize,
    /// Channels effective at [`ChannelClaim::Narrowed`].
    pub narrowed: usize,
    /// Channels effective at [`ChannelClaim::ReadOnlyOverlay`].
    pub overlay: usize,
    /// Channels effective at [`ChannelClaim::Unreconstructable`].
    pub unreconstructable: usize,
    /// Channels effective at [`ChannelClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5OutputChannelSetPacket {
    /// Builds a channel-set packet, sealing the record-kind, schema, and taxonomy
    /// version constants.
    pub fn new(input: M5OutputChannelSetInput) -> Self {
        Self {
            record_kind: M5_OUTPUT_CHANNELS_RECORD_KIND.to_owned(),
            schema_version: M5_OUTPUT_CHANNELS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_OUTPUT_CHANNELS_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            channels: input.channels,
        }
    }

    /// Whether the verification window has elapsed by `as_of`.
    pub fn freshness_stale_at(&self, as_of: &str) -> bool {
        if !self.verification_freshness.auto_downgrade_on_stale {
            return false;
        }
        let last =
            parse_rfc3339_to_epoch_seconds(&self.verification_freshness.last_verification_refresh);
        let now = parse_rfc3339_to_epoch_seconds(as_of);
        match (last, now) {
            (Some(last), Some(now)) => {
                now - last
                    > i64::from(self.verification_freshness.verification_freshness_slo_hours) * 3600
            }
            _ => false,
        }
    }

    /// Whether the window has elapsed by the packet's own `as_of`.
    pub fn stale_window(&self) -> bool {
        self.freshness_stale_at(&self.as_of)
    }

    /// Re-derive the decision for every channel, paired with its id.
    pub fn decisions(&self) -> Vec<(String, ChannelDecision)> {
        let stale_window = self.stale_window();
        self.channels
            .iter()
            .map(|c| (c.channel_id.clone(), c.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective channel claims.
    pub fn claim_distribution(&self) -> ChannelClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = ChannelClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            unreconstructable: 0,
            labs: 0,
        };
        for c in &self.channels {
            match c.narrow(stale_window).effective_channel_claim {
                ChannelClaim::Certified => dist.certified += 1,
                ChannelClaim::Narrowed => dist.narrowed += 1,
                ChannelClaim::ReadOnlyOverlay => dist.overlay += 1,
                ChannelClaim::Unreconstructable => dist.unreconstructable += 1,
                ChannelClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of channels whose effective claim ranks below their claimed claim.
    pub fn narrowed_channel_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.channels
            .iter()
            .filter(|c| c.narrow(stale_window).narrowed)
            .count()
    }

    /// Payload kinds represented by some channel.
    pub fn represented_payload_kinds(&self) -> BTreeSet<ChannelPayloadKind> {
        self.channels.iter().map(|c| c.payload_kind).collect()
    }

    /// Content trust classes represented by some channel.
    pub fn represented_trust_classes(&self) -> BTreeSet<ContentTrustClass> {
        self.channels.iter().map(|c| c.trust_class).collect()
    }

    /// Rendering surfaces represented by some channel.
    pub fn represented_surfaces(&self) -> BTreeSet<ChannelSurface> {
        self.channels
            .iter()
            .flat_map(|c| c.renderings.iter().map(|r| r.surface))
            .collect()
    }

    /// Validate the output-channel invariants.
    pub fn validate(&self) -> Vec<M5OutputChannelViolation> {
        use M5OutputChannelViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_OUTPUT_CHANNELS_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_OUTPUT_CHANNELS_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_OUTPUT_CHANNELS_TAXONOMY_VERSION {
            violations.push(V::WrongTaxonomyVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(V::MissingIdentity);
        }
        if !REDACTION_CLASS_TOKENS.contains(&self.redaction_class_token.as_str()) {
            violations.push(V::InvalidRedactionClass);
        }
        if self.verification_freshness.verification_freshness_slo_hours == 0
            || self
                .verification_freshness
                .last_verification_refresh
                .trim()
                .is_empty()
        {
            violations.push(V::EvidenceFreshnessIncomplete);
        }
        if self.channels.is_empty() {
            violations.push(V::EmptyChannels);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for c in &self.channels {
            if !seen.insert(c.channel_id.as_str()) {
                violations.push(V::DuplicateChannelId);
            }
        }

        let kinds = self.represented_payload_kinds();
        if ChannelPayloadKind::ALL.iter().any(|k| !kinds.contains(k)) {
            violations.push(V::ChannelPayloadKindMissing);
        }
        let trust = self.represented_trust_classes();
        if ContentTrustClass::ALL.iter().any(|t| !trust.contains(t)) {
            violations.push(V::ChannelTrustClassMissing);
        }
        let surfaces = self.represented_surfaces();
        if ChannelSurface::ALL.iter().any(|s| !surfaces.contains(s)) {
            violations.push(V::ChannelSurfaceMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for c in &self.channels {
            c.structural_violations(&mut violations);
            let decision = c.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || c.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedChannelMissingLabelOrTrigger);
                }
            }
            if !c.floored_keeps_fallback(decision.effective_channel_claim) {
                violations.push(V::FlooredChannelLosesFallback);
            }
            if c.surface_overclaims(decision.effective_channel_claim) {
                violations.push(V::RenderingSurfaceOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedChannelCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("output-channel packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("output-channel packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Output-Channel Virtualization, Trust, and Freshness\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Channels: {}\n", self.channels.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} read-only overlay, {} unreconstructable, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.unreconstructable, dist.labs
        ));

        out.push_str("| Channel | Payload | Trust | Origin | Claimed | Effective | Confidence |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for c in &self.channels {
            let decision = c.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                c.channel_id,
                c.payload_kind.as_str(),
                c.trust_class.as_str(),
                c.origin_class.as_str(),
                decision.claimed_channel_claim.as_str(),
                decision.effective_channel_claim.as_str(),
                c.effective_confidence(decision.effective_channel_claim)
                    .as_str(),
            ));
        }

        out.push('\n');
        for c in &self.channels {
            let decision = c.narrow(stale_window);
            if let Some(label) = c.narrowed_label(&decision) {
                out.push_str(&format!("- Narrowed: `{}` — {}\n", c.channel_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or validate.
#[derive(Debug)]
pub enum M5OutputChannelArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5OutputChannelViolation>),
}

impl fmt::Display for M5OutputChannelArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(f, "output-channel support export parse error: {err}")
            }
            Self::Validation(violations) => write!(
                f,
                "output-channel support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for M5OutputChannelArtifactError {}

/// Invariant violations reported by [`M5OutputChannelSetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OutputChannelViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Taxonomy version is wrong.
    WrongTaxonomyVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Redaction-class token is not one of the allowed values.
    InvalidRedactionClass,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// The packet carries no channels.
    EmptyChannels,
    /// Two channels share an id.
    DuplicateChannelId,
    /// A required payload kind is unrepresented.
    ChannelPayloadKindMissing,
    /// A required content trust class is unrepresented.
    ChannelTrustClassMissing,
    /// A required rendering surface is unrepresented.
    ChannelSurfaceMissing,
    /// A channel is missing its id, label, or execution-context ref.
    ChannelMissingIdentity,
    /// An overlay-origin channel does not name its provider.
    OverlayMissingProviderRef,
    /// A channel renders on no surface.
    ChannelMissingRendering,
    /// A rendering is missing its source-channel backlink.
    RenderingMissingSourceRef,
    /// A floored channel lost its raw-output / keyboard reopen fallback.
    FlooredChannelLosesFallback,
    /// A narrowed channel is missing its precise label or trigger.
    NarrowedChannelMissingLabelOrTrigger,
    /// A rendering surface renders wider than the channel's effective claim.
    RenderingSurfaceOverclaims,
    /// No channel demonstrates the auto-narrowing rule.
    DowngradedChannelCaseMissing,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl M5OutputChannelViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyChannels => "empty_channels",
            Self::DuplicateChannelId => "duplicate_channel_id",
            Self::ChannelPayloadKindMissing => "channel_payload_kind_missing",
            Self::ChannelTrustClassMissing => "channel_trust_class_missing",
            Self::ChannelSurfaceMissing => "channel_surface_missing",
            Self::ChannelMissingIdentity => "channel_missing_identity",
            Self::OverlayMissingProviderRef => "overlay_missing_provider_ref",
            Self::ChannelMissingRendering => "channel_missing_rendering",
            Self::RenderingMissingSourceRef => "rendering_missing_source_ref",
            Self::FlooredChannelLosesFallback => "floored_channel_loses_fallback",
            Self::NarrowedChannelMissingLabelOrTrigger => {
                "narrowed_channel_missing_label_or_trigger"
            }
            Self::RenderingSurfaceOverclaims => "rendering_surface_overclaims",
            Self::DowngradedChannelCaseMissing => "downgraded_channel_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked-in canonical support export.
///
/// This is the canonical entry point downstream shell, terminal, Problems, debug,
/// pipeline, notebook, support-export, AI-evidence, and docs surfaces use to ingest
/// the frozen channel set instead of cloning provider-local channel state.
///
/// # Errors
///
/// Returns [`M5OutputChannelArtifactError`] when the artifact cannot be parsed or
/// fails validation.
pub fn current_m5_output_channel_set(
) -> Result<M5OutputChannelSetPacket, M5OutputChannelArtifactError> {
    let packet: M5OutputChannelSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/tooling/m5-output-channels/support_export.json"
    )))
    .map_err(M5OutputChannelArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5OutputChannelArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded channel set: the in-crate source of truth the checked-in
/// support export and report are regenerated from.
pub fn seeded_m5_output_channel_set() -> M5OutputChannelSetPacket {
    M5OutputChannelSetPacket::new(M5OutputChannelSetInput {
        packet_id: M5_OUTPUT_CHANNELS_PACKET_ID.to_owned(),
        label: "M5 output channels — stream-first virtualization, trust classes, pin/export, and stale/live truth".to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        channels: seed_channels(),
    })
}

/// Renderings that show a `claim` cleanly across the named surfaces.
fn renderings(
    source_ref: &str,
    claim: ChannelClaim,
    surfaces: &[ChannelSurface],
    read_only: bool,
) -> Vec<ChannelRendering> {
    surfaces
        .iter()
        .map(|&surface| ChannelRendering {
            surface,
            rendered_claim: claim,
            lineage_visible: true,
            read_only,
            source_channel_ref: source_ref.to_owned(),
        })
        .collect()
}

/// A clean first-party integrity block.
fn clean_integrity() -> ChannelIntegrity {
    ChannelIntegrity {
        preserves_run_step_lineage: true,
        preserves_provider_identity: true,
        lineage_visible_on_demand: true,
        freshness_state_labeled: true,
        confidence_label_visible: true,
        superseded_state_marked: true,
        imported_channel_read_only: true,
        raw_output_backlink_present: true,
    }
}

/// A large stream-first virtualization block (searchable, bounded, exportable).
fn large_virtualization(total_chunks: u64) -> VirtualizationProfile {
    VirtualizationProfile {
        large_log: true,
        stream_first: true,
        searchable: true,
        stable_chunk_ids: true,
        follow_mode_supported: true,
        bounded_memory: true,
        exportable_without_full_materialization: true,
        total_chunk_count: total_chunks,
        retained_window_chunks: 256,
        approx_total_bytes: total_chunks * 65_536,
        max_retained_bytes: 16_777_216,
    }
}

/// A small (non-stream) virtualization block; stream-first is not required.
fn small_virtualization(total_chunks: u64) -> VirtualizationProfile {
    VirtualizationProfile {
        large_log: false,
        stream_first: false,
        searchable: true,
        stable_chunk_ids: true,
        follow_mode_supported: false,
        bounded_memory: true,
        exportable_without_full_materialization: true,
        total_chunk_count: total_chunks,
        retained_window_chunks: total_chunks,
        approx_total_bytes: total_chunks * 4_096,
        max_retained_bytes: total_chunks * 4_096,
    }
}

/// A clean trust/access block for a given trust class.
fn clean_access(trust: ContentTrustClass) -> ChannelAccessControls {
    ChannelAccessControls {
        trust_class_labeled: true,
        safe_preview_available: true,
        pin_supported: true,
        export_supported: true,
        export_is_safe: true,
        open_in_external_requires_confirmation: trust.is_active_content(),
        trust_boundary_preserved: true,
    }
}

/// A first-party (non-provider) freshness block.
fn local_freshness() -> ChannelFreshness {
    ChannelFreshness {
        provider_backed: false,
        fetched_at_present: false,
        provider_reachable: true,
        provider_unreachable_marked: false,
        live_state_honest: true,
    }
}

/// A provider-backed freshness block with a fetched-at cue and honest live state.
fn provider_freshness() -> ChannelFreshness {
    ChannelFreshness {
        provider_backed: true,
        fetched_at_present: true,
        provider_reachable: true,
        provider_unreachable_marked: false,
        live_state_honest: true,
    }
}

/// A verified-current proof block.
fn verified(proof_ref: &str) -> ChannelVerification {
    ChannelVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

fn seed_channels() -> Vec<OutputChannelRecord> {
    vec![
        // 1. Large raw log stream from a local test — certified, virtualized.
        OutputChannelRecord {
            channel_id: "channel:raw-log-local-test:0001".to_owned(),
            payload_kind: ChannelPayloadKind::RawLogStream,
            channel_class: OutputChannelClass::TaskTestDebugOutput,
            trust_class: ContentTrustClass::Raw,
            label_summary:
                "Large raw test log stream rendered stream-first, searchable, and exportable without full materialization."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OutputChannel,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                canonical_channel_ref: Some("channel.local.test.raw.0001".to_owned()),
                origin_run_ref: Some("run.local.test.0001".to_owned()),
                origin_step_ref: Some("step.local.test.0001".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.test.log.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.test.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.0001".to_owned()),
            },
            virtualization: large_virtualization(4_096),
            access: clean_access(ContentTrustClass::Raw),
            freshness: local_freshness(),
            integrity: clean_integrity(),
            verification: verified("proof.local.test.raw.0001"),
            renderings: renderings(
                "channel:raw-log-local-test:0001",
                ChannelClaim::Certified,
                &[
                    ChannelSurface::OutputPane,
                    ChannelSurface::TerminalPane,
                    ChannelSurface::SupportExport,
                ],
                false,
            ),
        },
        // 2. Trusted structured test report from a local test — certified.
        OutputChannelRecord {
            channel_id: "channel:structured-report-local-test:0001".to_owned(),
            payload_kind: ChannelPayloadKind::StructuredReport,
            channel_class: OutputChannelClass::TaskTestDebugOutput,
            trust_class: ContentTrustClass::TrustedStructured,
            label_summary:
                "Trusted structured test report parsed into Problems and the output pane with a raw-output fallback."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                canonical_channel_ref: Some("channel.local.test.report.0001".to_owned()),
                origin_run_ref: Some("run.local.test.0002".to_owned()),
                origin_step_ref: Some("step.local.test.0002".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.test.report.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.test.0002".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.0002".to_owned()),
            },
            virtualization: small_virtualization(4),
            access: clean_access(ContentTrustClass::TrustedStructured),
            freshness: local_freshness(),
            integrity: clean_integrity(),
            verification: verified("proof.local.test.report.0001"),
            renderings: renderings(
                "channel:structured-report-local-test:0001",
                ChannelClaim::Certified,
                &[
                    ChannelSurface::ProblemsPanel,
                    ChannelSurface::OutputPane,
                    ChannelSurface::AiEvidence,
                ],
                false,
            ),
        },
        // 3. HTML report bundle from a local task — certified, untrusted-active but
        //    gated behind confirmation with a safe export.
        OutputChannelRecord {
            channel_id: "channel:html-bundle-local-task:0001".to_owned(),
            payload_kind: ChannelPayloadKind::HtmlReportBundle,
            channel_class: OutputChannelClass::EvidenceBundle,
            trust_class: ContentTrustClass::UntrustedActive,
            label_summary:
                "Untrusted HTML report bundle that opens in an external viewer only after confirmation, never blurring the safe-preview boundary."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::GeneratedArtifact,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                canonical_channel_ref: Some("channel.local.task.html.0001".to_owned()),
                origin_run_ref: Some("run.local.task.0001".to_owned()),
                origin_step_ref: Some("step.local.task.0001".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.task.html.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.task.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.0001".to_owned()),
            },
            virtualization: small_virtualization(1),
            access: clean_access(ContentTrustClass::UntrustedActive),
            freshness: local_freshness(),
            integrity: clean_integrity(),
            verification: verified("proof.local.task.html.0001"),
            renderings: renderings(
                "channel:html-bundle-local-task:0001",
                ChannelClaim::Certified,
                &[ChannelSurface::OutputPane, ChannelSurface::DiffReviewOverlay],
                false,
            ),
        },
        // 4. Trace/profile output from a local task — certified, safe-preview.
        OutputChannelRecord {
            channel_id: "channel:trace-profile-local-task:0001".to_owned(),
            payload_kind: ChannelPayloadKind::TraceProfileOutput,
            channel_class: OutputChannelClass::TaskTestDebugOutput,
            trust_class: ContentTrustClass::SafePreview,
            label_summary:
                "Trace/profile output rendered as a safe preview card with a reopenable artifact and timeline entry."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::GeneratedArtifact,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                canonical_channel_ref: Some("channel.local.task.trace.0001".to_owned()),
                origin_run_ref: Some("run.local.task.0002".to_owned()),
                origin_step_ref: Some("step.local.task.0002".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.task.trace.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.task.0002".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.0002".to_owned()),
            },
            virtualization: small_virtualization(16),
            access: clean_access(ContentTrustClass::SafePreview),
            freshness: local_freshness(),
            integrity: clean_integrity(),
            verification: verified("proof.local.task.trace.0001"),
            renderings: renderings(
                "channel:trace-profile-local-task:0001",
                ChannelClaim::Certified,
                &[ChannelSurface::OutputPane, ChannelSurface::TimelineHistory],
                false,
            ),
        },
        // 5. Generated artifact from a local test — certified, safe-preview.
        OutputChannelRecord {
            channel_id: "channel:artifact-local-test:0001".to_owned(),
            payload_kind: ChannelPayloadKind::GeneratedArtifact,
            channel_class: OutputChannelClass::EvidenceBundle,
            trust_class: ContentTrustClass::SafePreview,
            label_summary:
                "Generated binary artifact shown as a safe metadata preview, pinnable and exportable without opening active content."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::GeneratedArtifact,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                canonical_channel_ref: Some("channel.local.test.artifact.0001".to_owned()),
                origin_run_ref: Some("run.local.test.0003".to_owned()),
                origin_step_ref: Some("step.local.test.0003".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.test.bin.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.test.0003".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.0003".to_owned()),
            },
            virtualization: small_virtualization(1),
            access: clean_access(ContentTrustClass::SafePreview),
            freshness: local_freshness(),
            integrity: clean_integrity(),
            verification: verified("proof.local.test.artifact.0001"),
            renderings: renderings(
                "channel:artifact-local-test:0001",
                ChannelClaim::Certified,
                &[ChannelSurface::OutputPane, ChannelSurface::SupportExport],
                false,
            ),
        },
        // 6. Large raw log from a local task — narrowed by a stale verification proof.
        OutputChannelRecord {
            channel_id: "channel:raw-log-local-task:0001".to_owned(),
            payload_kind: ChannelPayloadKind::RawLogStream,
            channel_class: OutputChannelClass::TaskTestDebugOutput,
            trust_class: ContentTrustClass::Raw,
            label_summary:
                "Large raw task log stream held below certified by a stale verification proof, still virtualized and reopenable."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::OutputChannel,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                canonical_channel_ref: Some("channel.local.task.raw.0001".to_owned()),
                origin_run_ref: Some("run.local.task.0003".to_owned()),
                origin_step_ref: Some("step.local.task.0003".to_owned()),
                provider_ref: None,
                artifact_ref: Some("artifact.local.task.log.0001".to_owned()),
                evidence_packet_ref: Some("evidence.local.task.0003".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.0003".to_owned()),
            },
            virtualization: large_virtualization(8_192),
            access: clean_access(ContentTrustClass::Raw),
            freshness: local_freshness(),
            integrity: clean_integrity(),
            verification: ChannelVerification {
                proof_currency: ProofCurrency::StaleExpired,
                proof_ref: Some("proof.local.task.raw.0001".to_owned()),
            },
            renderings: renderings(
                "channel:raw-log-local-task:0001",
                ChannelClaim::Narrowed,
                &[ChannelSurface::OutputPane, ChannelSurface::TerminalPane],
                false,
            ),
        },
        // 7. Large raw log from a pipeline provider — read-only overlay.
        OutputChannelRecord {
            channel_id: "channel:raw-log-pipeline-provider:0001".to_owned(),
            payload_kind: ChannelPayloadKind::RawLogStream,
            channel_class: OutputChannelClass::RemoteProviderImportedOutput,
            trust_class: ContentTrustClass::Raw,
            label_summary:
                "Pipeline provider raw log surfaced read-only with a fetched-at cue, cached within window and never claiming live local authority."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::PipelineProviderRun,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::ProviderRunPage,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.remote.pipeline.primary".to_owned(),
                canonical_channel_ref: Some("channel.pipeline.provider.raw.0001".to_owned()),
                origin_run_ref: Some("run.pipeline.provider.0001".to_owned()),
                origin_step_ref: Some("step.pipeline.provider.0001".to_owned()),
                provider_ref: Some("provider.pipeline.ci.0001".to_owned()),
                artifact_ref: Some("artifact.pipeline.provider.log.0001".to_owned()),
                evidence_packet_ref: Some("evidence.pipeline.provider.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.pipeline.provider.0001".to_owned()),
            },
            virtualization: large_virtualization(2_048),
            access: clean_access(ContentTrustClass::Raw),
            freshness: provider_freshness(),
            integrity: clean_integrity(),
            verification: ChannelVerification {
                proof_currency: ProofCurrency::ImportedCurrent,
                proof_ref: Some("proof.pipeline.provider.0001".to_owned()),
            },
            renderings: renderings(
                "channel:raw-log-pipeline-provider:0001",
                ChannelClaim::ReadOnlyOverlay,
                &[
                    ChannelSurface::OutputPane,
                    ChannelSurface::DiffReviewOverlay,
                    ChannelSurface::TimelineHistory,
                ],
                true,
            ),
        },
        // 8. Imported provider structured report — read-only overlay.
        OutputChannelRecord {
            channel_id: "channel:structured-report-imported-provider:0001".to_owned(),
            payload_kind: ChannelPayloadKind::StructuredReport,
            channel_class: OutputChannelClass::RemoteProviderImportedOutput,
            trust_class: ContentTrustClass::TrustedStructured,
            label_summary:
                "Imported provider structured report surfaced read-only, cached within window with a fetched-at cue and provider run-page reopen."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::ImportedProviderEvidence,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::ProviderRunPage,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.remote.import.primary".to_owned(),
                canonical_channel_ref: Some("channel.import.provider.report.0001".to_owned()),
                origin_run_ref: Some("run.import.provider.0001".to_owned()),
                origin_step_ref: None,
                provider_ref: Some("provider.import.scanner.0001".to_owned()),
                artifact_ref: Some("artifact.import.provider.report.0001".to_owned()),
                evidence_packet_ref: Some("evidence.import.provider.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.import.provider.0001".to_owned()),
            },
            virtualization: small_virtualization(8),
            access: clean_access(ContentTrustClass::TrustedStructured),
            freshness: provider_freshness(),
            integrity: clean_integrity(),
            verification: ChannelVerification {
                proof_currency: ProofCurrency::ImportedCurrent,
                proof_ref: Some("proof.import.provider.0001".to_owned()),
            },
            renderings: renderings(
                "channel:structured-report-imported-provider:0001",
                ChannelClaim::ReadOnlyOverlay,
                &[
                    ChannelSurface::ProblemsPanel,
                    ChannelSurface::SupportExport,
                    ChannelSurface::AiEvidence,
                ],
                true,
            ),
        },
        // 9. Labs HTML report bundle — makes no public claim.
        OutputChannelRecord {
            channel_id: "channel:html-bundle-labs:0001".to_owned(),
            payload_kind: ChannelPayloadKind::HtmlReportBundle,
            channel_class: OutputChannelClass::TaskTestDebugOutput,
            trust_class: ContentTrustClass::UntrustedActive,
            label_summary:
                "Labs HTML report bundle; unadvertised, makes no public claim and is never widened."
                    .to_owned(),
            claim_posture: ClaimPosture::LabsUnadvertised,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::HeuristicMedium,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            lineage: ChannelLineage {
                execution_context_ref: "exec-context.local.workspace.labs".to_owned(),
                canonical_channel_ref: Some("channel.local.labs.html.0001".to_owned()),
                origin_run_ref: Some("run.local.labs.0001".to_owned()),
                origin_step_ref: None,
                provider_ref: None,
                artifact_ref: None,
                evidence_packet_ref: None,
                raw_output_backlink_ref: Some("raw.local.labs.0001".to_owned()),
            },
            virtualization: small_virtualization(1),
            access: clean_access(ContentTrustClass::UntrustedActive),
            freshness: local_freshness(),
            integrity: clean_integrity(),
            verification: ChannelVerification {
                proof_currency: ProofCurrency::RequiresReview,
                proof_ref: None,
            },
            renderings: renderings(
                "channel:html-bundle-labs:0001",
                ChannelClaim::LabsNotClaimed,
                &[ChannelSurface::OutputPane],
                false,
            ),
        },
    ]
}
