//! Normalized pipeline run rows, log viewers, artifact browsers, and safe-preview trust classes.
//!
//! This module implements the canonical M5 truth packet for the in-product
//! pipeline viewer: the surface that projects CI / pipeline / check / build /
//! deploy / release runs into a review-lane record without ever pulling raw
//! provider bytes across the support boundary. It binds four pillars into one
//! export-safe record:
//!
//! - **Normalized pipeline run rows** — each [`PipelineRunRow`] carries a run's
//!   target identity, durable review anchor, normalized run status, freshness
//!   class, trigger attribution, and rerun/cancel authority, plus the attention
//!   reasons behind a non-green status, so a run never overstates that it
//!   succeeded and every rerun or cancel stays attributable.
//! - **Log viewers** — each [`LogViewerRow`] records the safe-preview trust
//!   class, safe-open path, stream state, and truncation label for a run's logs,
//!   so a truncated or partial log is labeled rather than presented as complete.
//! - **Artifact browsers** — each [`ArtifactBrowserRow`] records the artifact
//!   kind, safe-preview trust class, safe-open path, size disclosure, freshness,
//!   and retention label for a run's artifacts, so retention-expired or
//!   download-only artifacts narrow their open path instead of pretending the
//!   bytes are live.
//! - **Safe-preview trust classes** — every log and artifact row resolves a
//!   [`SafePreviewTrustClass`] from the frozen architecture vocabulary; because
//!   pipeline content arrives from a provider boundary, `TrustedLocalActive` is
//!   never admissible here.
//!
//! The packet references upstream pipeline-run, log-view, artifact-card,
//! run-control, and trust-class contracts by id rather than embedding their
//! content. Raw run/log/artifact bodies, raw provider payloads, raw URLs, raw
//! absolute paths, credentials, and live provider responses stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json`](../../../../schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md`](../../../../docs/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/`](../../../../fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`PipelineViewerPacket`].
pub const PIPELINE_VIEWER_RECORD_KIND: &str =
    "normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes";

/// Schema version for pipeline viewer records.
pub const PIPELINE_VIEWER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PIPELINE_VIEWER_SCHEMA_REF: &str =
    "schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json";

/// Repo-relative path of the pipeline viewer contract doc.
pub const PIPELINE_VIEWER_DOC_REF: &str =
    "docs/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md";

/// Repo-relative path of the normalized pipeline-run-row contract this lane builds on.
pub const PIPELINE_VIEWER_PIPELINE_RUN_CONTRACT_REF: &str =
    "schemas/ci/pipeline_run_row.schema.json";

/// Repo-relative path of the log-pane view contract reused for log viewers.
pub const PIPELINE_VIEWER_LOG_VIEW_CONTRACT_REF: &str = "schemas/ci/log_view.schema.json";

/// Repo-relative path of the artifact-card contract reused for artifact browsers.
pub const PIPELINE_VIEWER_ARTIFACT_CARD_CONTRACT_REF: &str =
    "schemas/ci/pipeline_artifact_card.schema.json";

/// Repo-relative path of the run-control review contract reused for rerun/cancel authority.
pub const PIPELINE_VIEWER_RUN_CONTROL_CONTRACT_REF: &str =
    "schemas/ci/run_control_review.schema.json";

/// Repo-relative path of the safe-preview trust-class vocabulary this lane re-exports.
pub const PIPELINE_VIEWER_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/trust_class.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const PIPELINE_VIEWER_FIXTURE_DIR: &str =
    "fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes";

/// Repo-relative path of the checked support-export artifact.
pub const PIPELINE_VIEWER_ARTIFACT_REF: &str =
    "artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PIPELINE_VIEWER_SUMMARY_REF: &str =
    "artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md";

/// Normalized run-status class shown for a pipeline run row.
///
/// Re-exported from the upstream pipeline-run-row contract. `unknown` must never
/// be flattened into `failed` or `neutral`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineRunStatus {
    /// Run is queued and has not started.
    Queued,
    /// Run is in progress.
    Running,
    /// Run completed successfully.
    Succeeded,
    /// Run completed with a failure.
    Failed,
    /// Run was cancelled before completion.
    Cancelled,
    /// Run needs an explicit action (for example, a pending approval).
    ActionRequired,
    /// Run was skipped.
    Skipped,
    /// Run exceeded its time budget.
    TimedOut,
    /// Provider returned a status the contract does not recognise yet.
    Unknown,
}

impl PipelineRunStatus {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::ActionRequired => "action_required",
            Self::Skipped => "skipped",
            Self::TimedOut => "timed_out",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this status needs at least one explicit attention reason.
    ///
    /// A failing, cancelled, time-out, action-required, or unknown status must
    /// carry a reason so the row never reads as benign when it is not.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::Cancelled | Self::ActionRequired | Self::TimedOut | Self::Unknown
        )
    }
}

/// Freshness class shown for a pipeline run row or artifact.
///
/// Re-exported from the upstream pipeline-run-row and safe-preview contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunFreshness {
    /// Live provider truth.
    AuthoritativeLive,
    /// Warm cached truth.
    WarmCached,
    /// Degraded cached truth.
    DegradedCached,
    /// Stale truth.
    Stale,
    /// Truth could not be verified.
    Unverified,
}

impl RunFreshness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    /// Whether this class is degraded enough that an open path must not depend on live bytes.
    pub const fn narrows_open_path(self) -> bool {
        matches!(self, Self::DegradedCached | Self::Stale | Self::Unverified)
    }
}

/// Rerun/cancel authority a pipeline viewer surface may exercise for a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineViewerRunControlAuthority {
    /// Surface is read-only and never reruns, cancels, or mutates run state.
    ReadOnlyNoControl,
    /// Surface may trigger an attributable rerun.
    AttributableRerun,
    /// Surface may trigger an attributable cancel.
    AttributableCancel,
    /// Surface may trigger an attributable rerun or cancel.
    AttributableRerunAndCancel,
}

impl PipelineViewerRunControlAuthority {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoControl => "read_only_no_control",
            Self::AttributableRerun => "attributable_rerun",
            Self::AttributableCancel => "attributable_cancel",
            Self::AttributableRerunAndCancel => "attributable_rerun_and_cancel",
        }
    }

    /// Whether this authority can mutate upstream run state.
    pub const fn is_mutating(self) -> bool {
        !matches!(self, Self::ReadOnlyNoControl)
    }
}

/// Safe-preview trust class resolved for a log or artifact.
///
/// Uses the frozen architecture spellings verbatim from
/// [`schemas/security/trust_class.schema.json`]. Pipeline content arrives from a
/// provider boundary, so `TrustedLocalActive` is never admissible on a log or
/// artifact row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SafePreviewTrustClass {
    /// Inert text rendered as text.
    RawText,
    /// Rich content rendered after sanitization.
    SanitizedRich,
    /// Trusted local active content; never admissible on the provider boundary.
    TrustedLocalActive,
    /// Active content isolated as remote.
    IsolatedRemoteActive,
}

impl SafePreviewTrustClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawText => "RawText",
            Self::SanitizedRich => "SanitizedRich",
            Self::TrustedLocalActive => "TrustedLocalActive",
            Self::IsolatedRemoteActive => "IsolatedRemoteActive",
        }
    }

    /// Whether this class is admissible on the provider-boundary pipeline surface.
    ///
    /// `TrustedLocalActive` is reserved for trusted local content and must never
    /// label a log or artifact that arrived from a CI / pipeline provider.
    pub const fn admissible_on_provider_boundary(self) -> bool {
        !matches!(self, Self::TrustedLocalActive)
    }
}

/// Safe-open path resolved for a log or artifact.
///
/// Re-exported from the upstream artifact-card contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeOpenPath {
    /// Opens in a structured viewer.
    OpenInStructuredViewer,
    /// Opens in a sanitized safe preview.
    OpenInSafePreviewSanitized,
    /// Opens in a metadata-only safe preview (no body bytes required).
    OpenInSafePreviewMetadataOnly,
    /// Routes to download only; no in-product open.
    DownloadOnlyNoInProductOpen,
    /// No open path is offered.
    DeniedNoOpenPath,
}

impl SafeOpenPath {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInStructuredViewer => "open_in_structured_viewer",
            Self::OpenInSafePreviewSanitized => "open_in_safe_preview_sanitized",
            Self::OpenInSafePreviewMetadataOnly => "open_in_safe_preview_metadata_only",
            Self::DownloadOnlyNoInProductOpen => "download_only_no_in_product_open",
            Self::DeniedNoOpenPath => "denied_no_open_path",
        }
    }

    /// Whether this open path renders live body bytes in product.
    ///
    /// Metadata-only, download-only, and denied paths do not depend on live
    /// bytes and stay valid against degraded or stale freshness.
    pub const fn requires_live_bytes(self) -> bool {
        matches!(
            self,
            Self::OpenInStructuredViewer | Self::OpenInSafePreviewSanitized
        )
    }
}

/// Stream state shown for a run's logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogStreamState {
    /// Log is streaming live from a running job.
    LiveStreaming,
    /// Log is complete and replayed from a finished run.
    CompletedReplay,
    /// Only a partial, retained slice of the log is available.
    PartialRetained,
    /// The log is unavailable (for example, retention expired or offline).
    Unavailable,
}

impl LogStreamState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveStreaming => "live_streaming",
            Self::CompletedReplay => "completed_replay",
            Self::PartialRetained => "partial_retained",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether this state must carry an explicit truncation label.
    pub const fn requires_truncation_label(self) -> bool {
        matches!(self, Self::PartialRetained | Self::Unavailable)
    }

    /// Whether this state can still render live body bytes in product.
    pub const fn has_live_bytes(self) -> bool {
        matches!(self, Self::LiveStreaming | Self::CompletedReplay)
    }
}

/// Artifact kind class shown for a run's artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    /// A log bundle.
    LogBundle,
    /// A structured test report.
    TestReport,
    /// A coverage report.
    CoverageReport,
    /// A binary or executable.
    BinaryExecutable,
    /// A container image.
    ContainerImage,
    /// An archive.
    Archive,
    /// A software bill of materials.
    Sbom,
    /// An image / media file.
    ImageMedia,
    /// Any other artifact kind.
    OtherArtifact,
}

impl ArtifactKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogBundle => "log_bundle",
            Self::TestReport => "test_report",
            Self::CoverageReport => "coverage_report",
            Self::BinaryExecutable => "binary_executable",
            Self::ContainerImage => "container_image",
            Self::Archive => "archive",
            Self::Sbom => "sbom",
            Self::ImageMedia => "image_media",
            Self::OtherArtifact => "other_artifact",
        }
    }

    /// Whether this kind is opaque bytes that must route to download-only or denied.
    ///
    /// Binary, executable, container, and archive artifacts have no safe
    /// in-product render and must not claim an in-product open path.
    pub const fn is_download_only_bytes(self) -> bool {
        matches!(
            self,
            Self::BinaryExecutable | Self::ContainerImage | Self::Archive
        )
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineViewerDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A run status was surfaced without verification.
    RunStatusUnverified,
    /// A safe-preview trust class narrowed.
    SafePreviewTrustNarrowed,
    /// A log was truncated or partial without an explicit label.
    LogTruncationUnlabeled,
    /// An artifact's retention expired and its bytes are gone.
    ArtifactRetentionExpired,
    /// A rerun/cancel authority was revoked.
    RunControlAuthorityRevoked,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified pipeline-viewer boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl PipelineViewerDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::RunStatusUnverified,
        Self::SafePreviewTrustNarrowed,
        Self::LogTruncationUnlabeled,
        Self::ArtifactRetentionExpired,
        Self::RunControlAuthorityRevoked,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::RunStatusUnverified => "run_status_unverified",
            Self::SafePreviewTrustNarrowed => "safe_preview_trust_narrowed",
            Self::LogTruncationUnlabeled => "log_truncation_unlabeled",
            Self::ArtifactRetentionExpired => "artifact_retention_expired",
            Self::RunControlAuthorityRevoked => "run_control_authority_revoked",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project this lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineViewerConsumerSurface {
    /// Pipeline viewer panel.
    PipelineViewer,
    /// Runs panel.
    RunsPanel,
    /// Log pane.
    LogPane,
    /// Artifact browser.
    ArtifactBrowser,
    /// Review workspace header.
    ReviewWorkspaceHeader,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl PipelineViewerConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PipelineViewer,
        Self::RunsPanel,
        Self::LogPane,
        Self::ArtifactBrowser,
        Self::ReviewWorkspaceHeader,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PipelineViewer => "pipeline_viewer",
            Self::RunsPanel => "runs_panel",
            Self::LogPane => "log_pane",
            Self::ArtifactBrowser => "artifact_browser",
            Self::ReviewWorkspaceHeader => "review_workspace_header",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One normalized pipeline run row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineRunRow {
    /// Stable run id.
    pub run_id: String,
    /// Human-readable target identity (what the run is for).
    pub target_identity_label: String,
    /// Durable review anchor id bound to this run.
    pub durable_anchor_id: String,
    /// Human-readable pipeline / workflow label.
    pub pipeline_label: String,
    /// Normalized run status.
    pub run_status: PipelineRunStatus,
    /// Freshness class shown for the run.
    pub freshness: RunFreshness,
    /// Human-readable trigger attribution (who or what started the run).
    pub trigger_attribution_label: String,
    /// Rerun/cancel authority the surface may exercise for the run.
    pub run_control_authority: PipelineViewerRunControlAuthority,
    /// Human-readable run-status summary.
    pub status_summary: String,
    /// Attention reasons; required and non-empty when the status needs attention.
    pub attention_reasons: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

/// One log viewer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogViewerRow {
    /// Run id this log belongs to.
    pub run_id: String,
    /// Stable log view id.
    pub view_id: String,
    /// Human-readable, redaction-aware log label.
    pub log_label: String,
    /// Stream state shown for the log.
    pub stream_state: LogStreamState,
    /// Safe-preview trust class resolved for the log.
    pub safe_preview_trust_class: SafePreviewTrustClass,
    /// Safe-open path resolved for the log.
    pub safe_open_path: SafeOpenPath,
    /// Truncation label; required and non-empty when the stream state requires it.
    pub truncation_label: String,
}

/// One artifact browser row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactBrowserRow {
    /// Run id this artifact belongs to.
    pub run_id: String,
    /// Stable artifact id.
    pub artifact_id: String,
    /// Human-readable, redaction-aware artifact label.
    pub artifact_label: String,
    /// Artifact kind class.
    pub artifact_kind: ArtifactKind,
    /// Safe-preview trust class resolved for the artifact.
    pub safe_preview_trust_class: SafePreviewTrustClass,
    /// Safe-open path resolved for the artifact.
    pub safe_open_path: SafeOpenPath,
    /// Freshness class shown for the artifact.
    pub freshness: RunFreshness,
    /// Human-readable size disclosure.
    pub size_disclosure_label: String,
    /// Human-readable retention label.
    pub retention_label: String,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineViewerTrustReview {
    /// Run status is explicit, never implied.
    pub run_status_explicit: bool,
    /// Run status is never overstated relative to its attention reasons.
    pub run_status_never_overstated: bool,
    /// Safe-preview trust class is shown for every log and artifact.
    pub safe_preview_trust_class_explicit: bool,
    /// Provider-boundary logs and artifacts never resolve `TrustedLocalActive`.
    pub active_content_never_trusted_local: bool,
    /// Log truncation is labeled, never silently presented as complete.
    pub log_truncation_labeled_not_hidden: bool,
    /// Artifact retention is labeled, never silently presented as live.
    pub artifact_retention_labeled_not_hidden: bool,
    /// Freshness is explicit and narrows the safe-open path when degraded.
    pub freshness_explicit_and_narrows_open_path: bool,
    /// Rerun/cancel authority is explicit and every action stays attributable.
    pub rerun_cancel_authority_explicit_and_attributable: bool,
    /// No pipeline viewer surface creates hidden write scope.
    pub no_hidden_write_scope: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl PipelineViewerTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.run_status_explicit
            && self.run_status_never_overstated
            && self.safe_preview_trust_class_explicit
            && self.active_content_never_trusted_local
            && self.log_truncation_labeled_not_hidden
            && self.artifact_retention_labeled_not_hidden
            && self.freshness_explicit_and_narrows_open_path
            && self.rerun_cancel_authority_explicit_and_attributable
            && self.no_hidden_write_scope
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineViewerConsumerProjection {
    /// Pipeline viewer shows the normalized run status.
    pub pipeline_viewer_shows_run_status: bool,
    /// Pipeline viewer shows freshness truth.
    pub pipeline_viewer_shows_freshness: bool,
    /// Runs panel shows attention reasons for non-green runs.
    pub runs_panel_shows_attention_reasons: bool,
    /// Log pane shows the safe-preview trust class.
    pub log_pane_shows_safe_preview_trust_class: bool,
    /// Log pane shows truncation labels.
    pub log_pane_shows_truncation: bool,
    /// Artifact browser shows the safe-open path.
    pub artifact_browser_shows_safe_open_path: bool,
    /// Artifact browser shows retention truth.
    pub artifact_browser_shows_retention: bool,
    /// Run-control surface shows authority and attribution.
    pub run_control_shows_authority_and_attribution: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_truth: bool,
    /// Preview / Labs lanes are labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified: bool,
}

impl PipelineViewerConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.pipeline_viewer_shows_run_status
            && self.pipeline_viewer_shows_freshness
            && self.runs_panel_shows_attention_reasons
            && self.log_pane_shows_safe_preview_trust_class
            && self.log_pane_shows_truncation
            && self.artifact_browser_shows_safe_open_path
            && self.artifact_browser_shows_retention
            && self.run_control_shows_authority_and_attribution
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.preview_labs_label_for_unqualified
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineViewerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PipelineViewerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineViewerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Normalized pipeline run rows.
    pub run_rows: Vec<PipelineRunRow>,
    /// Log viewer rows.
    pub log_views: Vec<LogViewerRow>,
    /// Artifact browser rows.
    pub artifact_cards: Vec<ArtifactBrowserRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PipelineViewerDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<PipelineViewerConsumerSurface>,
    /// Trust review block.
    pub trust_review: PipelineViewerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PipelineViewerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PipelineViewerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe normalized pipeline run rows, log viewers, artifact browsers, and safe-preview packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineViewerPacket {
    /// Record kind; must equal [`PIPELINE_VIEWER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PIPELINE_VIEWER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Normalized pipeline run rows.
    pub run_rows: Vec<PipelineRunRow>,
    /// Log viewer rows.
    pub log_views: Vec<LogViewerRow>,
    /// Artifact browser rows.
    pub artifact_cards: Vec<ArtifactBrowserRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PipelineViewerDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<PipelineViewerConsumerSurface>,
    /// Trust review block.
    pub trust_review: PipelineViewerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PipelineViewerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PipelineViewerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PipelineViewerPacket {
    /// Builds a pipeline viewer packet from stable-lane input.
    pub fn new(input: PipelineViewerPacketInput) -> Self {
        Self {
            record_kind: PIPELINE_VIEWER_RECORD_KIND.to_owned(),
            schema_version: PIPELINE_VIEWER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            run_rows: input.run_rows,
            log_views: input.log_views,
            artifact_cards: input.artifact_cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the pipeline viewer invariants.
    pub fn validate(&self) -> Vec<PipelineViewerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PIPELINE_VIEWER_RECORD_KIND {
            violations.push(PipelineViewerViolation::WrongRecordKind);
        }
        if self.schema_version != PIPELINE_VIEWER_SCHEMA_VERSION {
            violations.push(PipelineViewerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PipelineViewerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(PipelineViewerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(PipelineViewerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_run_rows(self, &mut violations);
        validate_log_views(self, &mut violations);
        validate_artifact_cards(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(PipelineViewerViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(PipelineViewerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(PipelineViewerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("pipeline viewer packet serializes"),
        ) {
            violations.push(PipelineViewerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("pipeline viewer packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let succeeded_runs = self
            .run_rows
            .iter()
            .filter(|row| row.run_status == PipelineRunStatus::Succeeded)
            .count();
        let attention_runs = self
            .run_rows
            .iter()
            .filter(|row| row.run_status.requires_attention_reason())
            .count();
        let truncated_logs = self
            .log_views
            .iter()
            .filter(|row| row.stream_state.requires_truncation_label())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Normalized Pipeline Run Rows, Log Viewers, Artifact Browsers, and Safe-Preview Trust Classes\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Run rows: {} ({} succeeded, {} needing attention)\n",
            self.run_rows.len(),
            succeeded_runs,
            attention_runs
        ));
        out.push_str(&format!(
            "- Log viewers: {} ({} truncated or partial)\n",
            self.log_views.len(),
            truncated_logs
        ));
        out.push_str(&format!(
            "- Artifact browsers: {}\n",
            self.artifact_cards.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Run rows\n\n");
        for row in &self.run_rows {
            out.push_str(&format!(
                "- **{}** ({}) → anchor `{}`: status `{}`, freshness `{}`, authority `{}`\n",
                row.target_identity_label,
                row.pipeline_label,
                row.durable_anchor_id,
                row.run_status.as_str(),
                row.freshness.as_str(),
                row.run_control_authority.as_str()
            ));
        }

        out.push_str("\n## Log viewers\n\n");
        for row in &self.log_views {
            out.push_str(&format!(
                "- `{}` on `{}`: stream `{}`, trust `{}`, open `{}`\n",
                row.view_id,
                row.run_id,
                row.stream_state.as_str(),
                row.safe_preview_trust_class.as_str(),
                row.safe_open_path.as_str()
            ));
        }

        out.push_str("\n## Artifact browsers\n\n");
        for row in &self.artifact_cards {
            out.push_str(&format!(
                "- `{}` on `{}`: kind `{}`, trust `{}`, open `{}`, freshness `{}`\n",
                row.artifact_id,
                row.run_id,
                row.artifact_kind.as_str(),
                row.safe_preview_trust_class.as_str(),
                row.safe_open_path.as_str(),
                row.freshness.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in pipeline viewer export.
#[derive(Debug)]
pub enum PipelineViewerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PipelineViewerViolation>),
}

impl fmt::Display for PipelineViewerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "pipeline viewer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "pipeline viewer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PipelineViewerArtifactError {}

/// Validation failures emitted by [`PipelineViewerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipelineViewerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No run rows are present.
    RunRowsMissing,
    /// A run row is incomplete.
    RunRowIncomplete,
    /// A non-green run is missing its attention reasons.
    AttentionReasonMissing,
    /// A run row has no log viewer row.
    RunMissingLogView,
    /// A log or artifact row references a run id with no run row.
    OrphanRowReference,
    /// No log viewer rows are present.
    LogViewsMissing,
    /// A log viewer row is incomplete.
    LogViewRowIncomplete,
    /// A partial or unavailable log is missing its truncation label.
    TruncationLabelMissing,
    /// No artifact browser rows are present.
    ArtifactCardsMissing,
    /// An artifact browser row is incomplete.
    ArtifactCardRowIncomplete,
    /// A log or artifact resolved `TrustedLocalActive` on the provider boundary.
    ActiveContentTrustInadmissible,
    /// A safe-open path renders live bytes against degraded freshness.
    SafeOpenPathOverstatesFreshness,
    /// An opaque-byte artifact claims an in-product open path.
    DownloadOnlyArtifactOpensInProduct,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PipelineViewerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RunRowsMissing => "run_rows_missing",
            Self::RunRowIncomplete => "run_row_incomplete",
            Self::AttentionReasonMissing => "attention_reason_missing",
            Self::RunMissingLogView => "run_missing_log_view",
            Self::OrphanRowReference => "orphan_row_reference",
            Self::LogViewsMissing => "log_views_missing",
            Self::LogViewRowIncomplete => "log_view_row_incomplete",
            Self::TruncationLabelMissing => "truncation_label_missing",
            Self::ArtifactCardsMissing => "artifact_cards_missing",
            Self::ArtifactCardRowIncomplete => "artifact_card_row_incomplete",
            Self::ActiveContentTrustInadmissible => "active_content_trust_inadmissible",
            Self::SafeOpenPathOverstatesFreshness => "safe_open_path_overstates_freshness",
            Self::DownloadOnlyArtifactOpensInProduct => "download_only_artifact_opens_in_product",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable pipeline viewer export.
pub fn current_pipeline_viewer_export() -> Result<PipelineViewerPacket, PipelineViewerArtifactError>
{
    let packet: PipelineViewerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/support_export.json"
    )))
    .map_err(PipelineViewerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PipelineViewerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &PipelineViewerPacket,
    violations: &mut Vec<PipelineViewerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PIPELINE_VIEWER_SCHEMA_REF,
        PIPELINE_VIEWER_DOC_REF,
        PIPELINE_VIEWER_PIPELINE_RUN_CONTRACT_REF,
        PIPELINE_VIEWER_LOG_VIEW_CONTRACT_REF,
        PIPELINE_VIEWER_ARTIFACT_CARD_CONTRACT_REF,
        PIPELINE_VIEWER_RUN_CONTROL_CONTRACT_REF,
        PIPELINE_VIEWER_TRUST_CLASS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PipelineViewerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_run_rows(packet: &PipelineViewerPacket, violations: &mut Vec<PipelineViewerViolation>) {
    if packet.run_rows.is_empty() {
        violations.push(PipelineViewerViolation::RunRowsMissing);
        return;
    }

    let logged_runs: BTreeSet<&str> = packet
        .log_views
        .iter()
        .map(|row| row.run_id.as_str())
        .collect();

    for row in &packet.run_rows {
        if row.run_id.trim().is_empty()
            || row.target_identity_label.trim().is_empty()
            || row.durable_anchor_id.trim().is_empty()
            || row.pipeline_label.trim().is_empty()
            || row.trigger_attribution_label.trim().is_empty()
            || row.status_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(PipelineViewerViolation::RunRowIncomplete);
        }
        if row.run_status.requires_attention_reason() && row.attention_reasons.is_empty() {
            violations.push(PipelineViewerViolation::AttentionReasonMissing);
        }
        if !row.run_id.trim().is_empty() && !logged_runs.contains(row.run_id.as_str()) {
            violations.push(PipelineViewerViolation::RunMissingLogView);
        }
    }
}

fn validate_log_views(
    packet: &PipelineViewerPacket,
    violations: &mut Vec<PipelineViewerViolation>,
) {
    if packet.log_views.is_empty() {
        violations.push(PipelineViewerViolation::LogViewsMissing);
        return;
    }

    let run_ids: BTreeSet<&str> = packet
        .run_rows
        .iter()
        .map(|row| row.run_id.as_str())
        .collect();

    for row in &packet.log_views {
        if row.run_id.trim().is_empty()
            || row.view_id.trim().is_empty()
            || row.log_label.trim().is_empty()
        {
            violations.push(PipelineViewerViolation::LogViewRowIncomplete);
        }
        if !row.run_id.trim().is_empty() && !run_ids.contains(row.run_id.as_str()) {
            violations.push(PipelineViewerViolation::OrphanRowReference);
        }
        if !row
            .safe_preview_trust_class
            .admissible_on_provider_boundary()
        {
            violations.push(PipelineViewerViolation::ActiveContentTrustInadmissible);
        }
        if row.stream_state.requires_truncation_label() && row.truncation_label.trim().is_empty() {
            violations.push(PipelineViewerViolation::TruncationLabelMissing);
        }
        if !row.stream_state.has_live_bytes() && row.safe_open_path.requires_live_bytes() {
            violations.push(PipelineViewerViolation::SafeOpenPathOverstatesFreshness);
        }
    }
}

fn validate_artifact_cards(
    packet: &PipelineViewerPacket,
    violations: &mut Vec<PipelineViewerViolation>,
) {
    if packet.artifact_cards.is_empty() {
        violations.push(PipelineViewerViolation::ArtifactCardsMissing);
        return;
    }

    let run_ids: BTreeSet<&str> = packet
        .run_rows
        .iter()
        .map(|row| row.run_id.as_str())
        .collect();

    for row in &packet.artifact_cards {
        if row.run_id.trim().is_empty()
            || row.artifact_id.trim().is_empty()
            || row.artifact_label.trim().is_empty()
            || row.size_disclosure_label.trim().is_empty()
            || row.retention_label.trim().is_empty()
        {
            violations.push(PipelineViewerViolation::ArtifactCardRowIncomplete);
        }
        if !row.run_id.trim().is_empty() && !run_ids.contains(row.run_id.as_str()) {
            violations.push(PipelineViewerViolation::OrphanRowReference);
        }
        if !row
            .safe_preview_trust_class
            .admissible_on_provider_boundary()
        {
            violations.push(PipelineViewerViolation::ActiveContentTrustInadmissible);
        }
        if row.freshness.narrows_open_path() && row.safe_open_path.requires_live_bytes() {
            violations.push(PipelineViewerViolation::SafeOpenPathOverstatesFreshness);
        }
        if row.artifact_kind.is_download_only_bytes() && row.safe_open_path.requires_live_bytes() {
            violations.push(PipelineViewerViolation::DownloadOnlyArtifactOpensInProduct);
        }
    }
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
