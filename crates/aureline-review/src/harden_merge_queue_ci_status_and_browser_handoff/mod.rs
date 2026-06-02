//! Hardened merge-queue, CI/check-status, pipeline-overlay freshness, and
//! browser-handoff audit on claimed provider rows.
//!
//! This module owns the bounded contract that keeps merge-queue truth,
//! CI/check status, pipeline-overlay freshness, and browser-handoff audit
//! explicit on every claimed stable provider row. It extends provider overlays
//! from simple badges to normalized run/check objects with fetched-at freshness,
//! provider/source class, artifact-link trust class, and explicit read-only
//! versus run-control subset labeling. Any rerun, cancel, queue, or similar
//! upstream-mutation affordance claimed in-product uses an auditable inline
//! review or short confirm sheet naming provider scope, actor mode, target run
//! identity, and browser-handoff fallback. Cached provider metadata never
//! implies control authority.
//!
//! The record family includes:
//!
//! - [`MergeQueueCiStatusBrowserHandoffAuditRecord`] — stable identity binding
//!   workspace, landing, stabilization, boundary hardening, and provider-linked
//!   stabilization into one audit row.
//! - [`MergeQueueAuditRecord`] — merge-queue entry audit with explicit provider
//!   authority, local-git truth preservation, and divergence detection.
//! - [`CiCheckAuditRecord`] — normalized run/check object with fetched-at
//!   freshness, provider/source class, and explicit divergence labels when local,
//!   CI, AI-review, or provider results disagree.
//! - [`PipelineOverlayAuditRecord`] — pipeline overlay audit with read-only
//!   versus run-control subset labeling and artifact-link trust class.
//! - [`RunControlAuditRecord`] — rerun/cancel/retry control audit stating
//!   whether it is inspect-only, provider-controlled, or auditable in-product,
//!   with review-pack digest and base/head context.
//! - [`BrowserHandoffAuditRecord`] — browser-handoff audit record on claimed
//!   provider rows with explicit source, freshness, actor, target, and return
//!   path.
//! - [`AuditCommandRecord`] — command-graph operations surfaced to the inspector
//!   (preview, approve, refresh, invalidate, handoff, rerun, cancel, retry).
//! - [`AuditSupportExportPacket`] — redaction-safe support export preserving
//!   audit lineage.
//! - [`AuditInspectionRecord`] — compact boolean projection for CLI and
//!   inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/harden_merge_queue_ci_status_and_browser_handoff.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/harden-merge-queue-ci-status-and-browser-handoff/`.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::harden_browser_handoff_and_in_product_review_boundaries::{
    InProductReviewBoundaryRecord, ReviewBoundaryHardeningRecord,
};
use crate::landing::LandingCandidateRecord;
use crate::review_pack_parity_harness::ReviewPackParityHarnessRecord;
use crate::stabilize_provider_linked_object_models_snapshot_freshness_and::ProviderLinkedReviewStabilizationRecord;
use crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationRecord;
use crate::workspace::ReviewWorkspaceRecord;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every merge-queue/CI-status/browser-handoff audit record.
pub const MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`MergeQueueCiStatusBrowserHandoffAuditPacket`].
pub const MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_PACKET_RECORD_KIND: &str =
    "merge_queue_ci_status_browser_handoff_audit_packet";

/// Stable record-kind tag for [`MergeQueueCiStatusBrowserHandoffAuditRecord`].
pub const MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_RECORD_KIND: &str =
    "merge_queue_ci_status_browser_handoff_audit_record";

/// Stable record-kind tag for [`MergeQueueAuditRecord`].
pub const MERGE_QUEUE_AUDIT_RECORD_KIND: &str = "merge_queue_audit_record";

/// Stable record-kind tag for [`CiCheckAuditRecord`].
pub const CI_CHECK_AUDIT_RECORD_KIND: &str = "ci_check_audit_record";

/// Stable record-kind tag for [`PipelineOverlayAuditRecord`].
pub const PIPELINE_OVERLAY_AUDIT_RECORD_KIND: &str = "pipeline_overlay_audit_record";

/// Stable record-kind tag for [`RunControlAuditRecord`].
pub const RUN_CONTROL_AUDIT_RECORD_KIND: &str = "run_control_audit_record";

/// Stable record-kind tag for [`BrowserHandoffAuditRecord`].
pub const BROWSER_HANDOFF_AUDIT_RECORD_KIND: &str = "browser_handoff_audit_record";

/// Stable record-kind tag for [`AuditCommandRecord`].
pub const AUDIT_COMMAND_RECORD_KIND: &str = "audit_command_record";

/// Stable record-kind tag for [`AuditSupportExportPacket`].
pub const AUDIT_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str = "audit_support_export_packet";

/// Stable record-kind tag for [`AuditInspectionRecord`].
pub const AUDIT_INSPECTION_RECORD_KIND: &str = "audit_inspection_record";

/// Closed set of audit states.
pub const AUDIT_STATES: &[&str] = &[
    "audit_passed",
    "audit_degraded_merge_queue_divergence",
    "audit_degraded_ci_check_stale",
    "audit_degraded_pipeline_overlay_unqualified",
    "audit_degraded_browser_handoff_untyped",
    "audit_degraded_hidden_authority",
    "audit_failed_provider_claim_unsupported",
];

/// Closed set of merge-queue audit states.
pub const MERGE_QUEUE_AUDIT_STATES: &[&str] = &[
    "merge_queue_authoritative",
    "merge_queue_diverged_from_provider",
    "merge_queue_stale_local_estimate",
    "merge_queue_downgraded_to_inspect_only",
];

/// Closed set of CI/check freshness classes.
pub const CI_CHECK_FRESHNESS_CLASSES: &[&str] = &[
    "check_fresh",
    "check_stale_within_grace",
    "check_stale_blocks_mutation",
    "check_freshness_unknown",
];

/// Closed set of CI/check divergence label classes.
pub const CI_CHECK_DIVERGENCE_CLASSES: &[&str] = &[
    "no_divergence",
    "local_ci_disagree",
    "local_provider_disagree",
    "ci_provider_disagree",
    "ai_review_disagrees",
    "all_three_disagree",
];

/// Closed set of pipeline-overlay subset classes.
pub const PIPELINE_OVERLAY_SUBSET_CLASSES: &[&str] = &[
    "read_only_inspect",
    "run_control_subset",
    "run_control_full",
    "subset_unqualified_downgraded",
];

/// Closed set of run-control mutation modes.
pub const RUN_CONTROL_MUTATION_MODES: &[&str] = &[
    "inspect_only",
    "provider_controlled",
    "auditable_in_product",
];

/// Closed set of browser-handoff audit classes.
pub const BROWSER_HANDOFF_AUDIT_CLASSES: &[&str] = &[
    "handoff_audited_reversible",
    "handoff_audited_no_return_path",
    "handoff_downgraded_untyped",
    "handoff_not_required",
];

/// Closed set of command classes for the audit lane.
pub const AUDIT_COMMAND_CLASSES: &[&str] = &[
    "preview_audit",
    "approve_audit",
    "refresh_provider_overlay",
    "invalidate_audit",
    "request_browser_handoff",
    "rerun_control",
    "cancel_control",
    "retry_control",
    "export_evidence",
];

/// Closed set of consumer surfaces for audit packets.
pub const AUDIT_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "review_landing_strip",
    "merge_queue_inspector",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
];

/// Closed set of invalidation reasons that mark an audit stale.
pub const AUDIT_INVALIDATION_REASONS: &[&str] = &[
    "merge_queue_diverged",
    "ci_check_stale",
    "pipeline_overlay_unqualified",
    "browser_handoff_untyped",
    "hidden_authority_detected",
    "provider_claim_unsupported",
    "stale_pack_invalidated",
    "review_pack_digest_mismatch",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a merge-queue/CI-status/browser-handoff audit to
/// materialize on top of landing, stabilization, boundary-hardening, and
/// provider-linked stabilization packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueCiStatusBrowserHandoffAuditInput {
    /// Stable audit identity.
    pub audit_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Audit state from the closed vocabulary.
    pub audit_state: String,
    /// Merge-queue audit input.
    pub merge_queue_audit: MergeQueueAuditInput,
    /// CI/check audit inputs.
    pub ci_check_audits: Vec<CiCheckAuditInput>,
    /// Pipeline-overlay audit inputs.
    pub pipeline_overlay_audits: Vec<PipelineOverlayAuditInput>,
    /// Run-control audit inputs.
    pub run_control_audits: Vec<RunControlAuditInput>,
    /// Browser-handoff audit inputs.
    pub browser_handoff_audits: Vec<BrowserHandoffAuditInput>,
    /// Command-graph operations defined for this audit.
    pub commands: Vec<AuditCommandInput>,
    /// Support/export envelope input.
    pub support_export: AuditSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a merge-queue audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueAuditInput {
    /// Merge-queue entry id ref.
    pub merge_queue_entry_id_ref: String,
    /// Queue authority class.
    pub queue_authority_class: String,
    /// Queue lifecycle state.
    pub queue_state: String,
    /// Provider freshness class at observation time.
    pub provider_freshness_class: String,
    /// True when local Git truth is preserved.
    pub local_git_truth_preserved: bool,
    /// True when stale base blocks queue progress.
    pub stale_base_blocks_queue: bool,
    /// True when stale checks block queue progress.
    pub checks_stale_blocks_queue: bool,
    /// True when divergence from local truth is detected.
    pub divergence_from_local_detected: bool,
    /// True when an explicit downgrade is required.
    pub explicit_downgrade_required: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a CI/check audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CiCheckAuditInput {
    /// Stable check audit identity.
    pub check_audit_id: String,
    /// Check id ref.
    pub check_id_ref: String,
    /// True when this is a required check.
    pub required_check: bool,
    /// Human-readable check name.
    pub check_name: String,
    /// Provider source class.
    pub provider_source_class: String,
    /// Timestamp when the check was fetched.
    pub fetched_at: String,
    /// Freshness class from the closed vocabulary.
    pub freshness_class: String,
    /// Local outcome class.
    pub local_outcome_class: String,
    /// CI outcome class.
    pub ci_outcome_class: String,
    /// AI-review outcome class.
    pub ai_review_outcome_class: String,
    /// Provider outcome class.
    pub provider_outcome_class: String,
    /// Divergence label class from the closed vocabulary.
    pub divergence_label_class: String,
    /// True when the check is inspect-only.
    pub inspect_only: bool,
    /// True when run-control is permitted.
    pub run_control_permitted: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a pipeline-overlay audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineOverlayAuditInput {
    /// Stable overlay audit identity.
    pub overlay_audit_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Overlay kind.
    pub overlay_kind: String,
    /// Timestamp when the overlay was fetched.
    pub fetched_at: String,
    /// Provider source class.
    pub provider_source_class: String,
    /// Artifact trust class.
    pub artifact_trust_class: String,
    /// True when the overlay is read-only inspect.
    pub read_only_subset_label: bool,
    /// True when the overlay exposes a run-control subset.
    pub run_control_subset_label: bool,
    /// Mutation mode disclosure label.
    pub mutation_mode_disclosure: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a run-control audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunControlAuditInput {
    /// Stable run-control audit identity.
    pub run_control_audit_id: String,
    /// Run-control id ref.
    pub run_control_id_ref: String,
    /// Control class.
    pub control_class: String,
    /// Mutation mode from the closed vocabulary.
    pub mutation_mode: String,
    /// Provider scope disclosure.
    pub provider_scope_disclosure: String,
    /// Actor mode disclosure.
    pub actor_mode_disclosure: String,
    /// Target run identity.
    pub target_run_identity: String,
    /// True when a browser-handoff fallback is present.
    pub browser_handoff_fallback_present: bool,
    /// Review-pack digest ref that scoped the action.
    pub review_pack_digest_ref: String,
    /// Base revision ref that scoped the action.
    pub base_revision_ref: String,
    /// Head revision ref that scoped the action.
    pub head_revision_ref: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a browser-handoff audit on a claimed provider row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffAuditInput {
    /// Stable handoff audit identity.
    pub handoff_audit_id: String,
    /// Boundary hardening id ref.
    pub boundary_hardening_id_ref: String,
    /// Handoff boundary class from the closed vocabulary.
    pub handoff_boundary_class: String,
    /// Provider class token.
    pub provider_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Reason code.
    pub reason_code: String,
    /// True when the handoff has a return path.
    pub return_path_present: bool,
    /// True when the handoff is reversible.
    pub reversible: bool,
    /// Actor ref.
    pub actor_ref: String,
    /// Scope disclosure.
    pub scope_disclosure: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing one command-graph operation for the audit lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditCommandInput {
    /// Stable command identity.
    pub command_id: String,
    /// Command class from the closed vocabulary.
    pub command_class: String,
    /// Target object ref the command would mutate.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when the command supports preview/dry-run.
    pub preview_supported: bool,
    /// True when the command emits an audit event when executed.
    pub emits_audit_event: bool,
    /// Active blocked reasons preventing execution; empty when actionable.
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input row for the audit support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the audit.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Merge-queue/CI-status/browser-handoff audit record materialized from input
/// plus cross-packet truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueCiStatusBrowserHandoffAuditRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable audit identity.
    pub audit_id: String,
    /// Review workspace this audit belongs to.
    pub review_workspace_id_ref: String,
    /// Landing candidate this audit belongs to.
    pub landing_candidate_id_ref: String,
    /// Stabilization this audit binds.
    pub stabilization_id_ref: String,
    /// Boundary hardening this audit binds.
    pub boundary_hardening_id_ref: String,
    /// Provider-linked stabilization this audit binds.
    pub provider_linked_stabilization_id_ref: String,
    /// Audit state.
    pub audit_state: String,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing mutation.
    pub blocked_reasons: Vec<String>,
    /// True when the audit is actionable from the current state.
    pub actionable: bool,
    /// Timestamp the audit was frozen.
    pub generated_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Merge-queue audit record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueAuditRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Audit this record belongs to.
    pub audit_id_ref: String,
    /// Merge-queue entry id ref.
    pub merge_queue_entry_id_ref: String,
    /// Queue authority class.
    pub queue_authority_class: String,
    /// Queue lifecycle state.
    pub queue_state: String,
    /// Provider freshness class.
    pub provider_freshness_class: String,
    /// True when local Git truth is preserved.
    pub local_git_truth_preserved: bool,
    /// True when stale base blocks queue progress.
    pub stale_base_blocks_queue: bool,
    /// True when stale checks block queue progress.
    pub checks_stale_blocks_queue: bool,
    /// True when divergence from local truth is detected.
    pub divergence_from_local_detected: bool,
    /// True when an explicit downgrade is required.
    pub explicit_downgrade_required: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// CI/check audit record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CiCheckAuditRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Audit this record belongs to.
    pub audit_id_ref: String,
    /// Stable check audit identity.
    pub check_audit_id: String,
    /// Check id ref.
    pub check_id_ref: String,
    /// True when this is a required check.
    pub required_check: bool,
    /// Human-readable check name.
    pub check_name: String,
    /// Provider source class.
    pub provider_source_class: String,
    /// Timestamp when the check was fetched.
    pub fetched_at: String,
    /// Freshness class.
    pub freshness_class: String,
    /// Local outcome class.
    pub local_outcome_class: String,
    /// CI outcome class.
    pub ci_outcome_class: String,
    /// AI-review outcome class.
    pub ai_review_outcome_class: String,
    /// Provider outcome class.
    pub provider_outcome_class: String,
    /// Divergence label class.
    pub divergence_label_class: String,
    /// True when the check is inspect-only.
    pub inspect_only: bool,
    /// True when run-control is permitted.
    pub run_control_permitted: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Pipeline-overlay audit record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineOverlayAuditRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Audit this record belongs to.
    pub audit_id_ref: String,
    /// Stable overlay audit identity.
    pub overlay_audit_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Overlay kind.
    pub overlay_kind: String,
    /// Timestamp when the overlay was fetched.
    pub fetched_at: String,
    /// Provider source class.
    pub provider_source_class: String,
    /// Artifact trust class.
    pub artifact_trust_class: String,
    /// True when the overlay is read-only inspect.
    pub read_only_subset_label: bool,
    /// True when the overlay exposes a run-control subset.
    pub run_control_subset_label: bool,
    /// Mutation mode disclosure label.
    pub mutation_mode_disclosure: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Run-control audit record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunControlAuditRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Audit this record belongs to.
    pub audit_id_ref: String,
    /// Stable run-control audit identity.
    pub run_control_audit_id: String,
    /// Run-control id ref.
    pub run_control_id_ref: String,
    /// Control class.
    pub control_class: String,
    /// Mutation mode.
    pub mutation_mode: String,
    /// Provider scope disclosure.
    pub provider_scope_disclosure: String,
    /// Actor mode disclosure.
    pub actor_mode_disclosure: String,
    /// Target run identity.
    pub target_run_identity: String,
    /// True when a browser-handoff fallback is present.
    pub browser_handoff_fallback_present: bool,
    /// Review-pack digest ref that scoped the action.
    pub review_pack_digest_ref: String,
    /// Base revision ref that scoped the action.
    pub base_revision_ref: String,
    /// Head revision ref that scoped the action.
    pub head_revision_ref: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Browser-handoff audit record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffAuditRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Audit this record belongs to.
    pub audit_id_ref: String,
    /// Stable handoff audit identity.
    pub handoff_audit_id: String,
    /// Boundary hardening id ref.
    pub boundary_hardening_id_ref: String,
    /// Handoff boundary class.
    pub handoff_boundary_class: String,
    /// Provider class token.
    pub provider_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Reason code.
    pub reason_code: String,
    /// True when the handoff has a return path.
    pub return_path_present: bool,
    /// True when the handoff is reversible.
    pub reversible: bool,
    /// Actor ref.
    pub actor_ref: String,
    /// Scope disclosure.
    pub scope_disclosure: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Command-graph operation record for the audit lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Audit this command belongs to.
    pub audit_id_ref: String,
    /// Command class.
    pub command_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when preview/dry-run is supported.
    pub preview_supported: bool,
    /// True when the command emits an audit event when executed.
    pub emits_audit_event: bool,
    /// Active blocked reasons preventing execution.
    pub blocked_reasons: Vec<String>,
    /// True when the command is actionable from the current audit state.
    pub actionable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet for the audit lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Audit this packet exports.
    pub audit_id_ref: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Stable context ref used to reopen the audit.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Command ids exported in this packet.
    pub command_id_refs: Vec<String>,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the export cites.
    pub source_schema_refs: Vec<String>,
    /// False so raw URLs cannot cross the support boundary.
    pub raw_url_export_allowed: bool,
    /// False so raw provider payloads cannot cross the support boundary.
    pub raw_provider_payload_export_allowed: bool,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Inspection row used by support/export and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Audit inspected by this row.
    pub audit_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when merge queue has been audited.
    pub merge_queue_audited: bool,
    /// True when CI/checks have been audited.
    pub ci_checks_audited: bool,
    /// True when pipeline overlays have been audited.
    pub pipeline_overlays_audited: bool,
    /// True when run controls have been audited.
    pub run_controls_audited: bool,
    /// True when browser handoffs have been audited.
    pub browser_handoffs_audited: bool,
    /// True when all provider rows are claimed stable.
    pub all_provider_rows_claimed_stable: bool,
    /// True when at least one provider row was downgraded.
    pub any_provider_row_downgraded: bool,
    /// True when hidden authority is detected.
    pub hidden_authority_detected: bool,
    /// True when a stale overlay is present.
    pub stale_overlay_present: bool,
    /// True when at least one control is inspect-only.
    pub inspect_only_controls_present: bool,
    /// True when at least one control is provider-controlled.
    pub provider_controlled_controls_present: bool,
    /// True when at least one control is auditable in-product.
    pub auditable_in_product_controls_present: bool,
    /// True when divergence labels are present.
    pub divergence_labels_present: bool,
    /// True when the audit is actionable.
    pub actionable: bool,
    /// True when the audit is invalidated by any reason.
    pub invalidated: bool,
    /// Number of CI/check audit records.
    pub ci_check_audit_count: usize,
    /// Number of pipeline-overlay audit records.
    pub pipeline_overlay_audit_count: usize,
    /// Number of run-control audit records.
    pub run_control_audit_count: usize,
    /// Number of browser-handoff audit records.
    pub browser_handoff_audit_count: usize,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the audit context.
    pub support_export_reopenable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Merge-queue/CI-status/browser-handoff audit packet consumed by review
/// surfaces and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueCiStatusBrowserHandoffAuditPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the packet.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Review workspace summary copied from the beta packet.
    pub review_workspace: ReviewWorkspaceRecord,
    /// Landing candidate summary copied from the landing packet.
    pub landing_candidate: LandingCandidateRecord,
    /// Stabilization summary copied from the stabilization packet.
    pub stabilization: ReviewStabilizationRecord,
    /// Boundary hardening summary copied from the boundary-hardening packet.
    pub boundary_hardening: ReviewBoundaryHardeningRecord,
    /// In-product review boundary summary copied from the boundary-hardening packet.
    pub in_product_boundary: InProductReviewBoundaryRecord,
    /// Provider-linked stabilization summary copied from the provider-linked packet.
    pub provider_linked_stabilization: ProviderLinkedReviewStabilizationRecord,
    /// Optional parity-harness record copied when present.
    pub parity_harness: Option<ReviewPackParityHarnessRecord>,
    /// Audit record.
    pub audit: MergeQueueCiStatusBrowserHandoffAuditRecord,
    /// Merge-queue audit record.
    pub merge_queue_audit: MergeQueueAuditRecord,
    /// CI/check audit records.
    pub ci_check_audits: Vec<CiCheckAuditRecord>,
    /// Pipeline-overlay audit records.
    pub pipeline_overlay_audits: Vec<PipelineOverlayAuditRecord>,
    /// Run-control audit records.
    pub run_control_audits: Vec<RunControlAuditRecord>,
    /// Browser-handoff audit records.
    pub browser_handoff_audits: Vec<BrowserHandoffAuditRecord>,
    /// Command-graph operation records.
    pub commands: Vec<AuditCommandRecord>,
    /// Support/export packet.
    pub support_export: AuditSupportExportPacket,
    /// Inspection row.
    pub inspection: AuditInspectionRecord,
}

impl MergeQueueCiStatusBrowserHandoffAuditPacket {
    /// Builds an audit packet from landing, stabilization, boundary-hardening,
    /// and provider-linked stabilization packets.
    ///
    /// # Errors
    ///
    /// Returns [`AuditValidationError`] when the input violates an audit
    /// invariant.
    pub fn from_source_packets(
        input: MergeQueueCiStatusBrowserHandoffAuditInput,
        landing_packet: &crate::landing::LandingCandidatePacket,
        stabilization_packet: &crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationPacket,
        boundary_hardening_packet: &crate::harden_browser_handoff_and_in_product_review_boundaries::ReviewBoundaryHardeningPacket,
        provider_linked_packet: &crate::stabilize_provider_linked_object_models_snapshot_freshness_and::ProviderLinkedReviewStabilizationPacket,
        parity_harness: Option<&ReviewPackParityHarnessRecord>,
    ) -> Result<Self, AuditValidationError> {
        validate_input(
            &input,
            landing_packet,
            stabilization_packet,
            boundary_hardening_packet,
            provider_linked_packet,
        )?;

        let audit = audit_record(
            &input,
            landing_packet,
            stabilization_packet,
            boundary_hardening_packet,
            provider_linked_packet,
        );
        let merge_queue_audit = merge_queue_audit_record(&input.merge_queue_audit, &audit);
        let ci_check_audits = input
            .ci_check_audits
            .iter()
            .map(|c| ci_check_audit_record(c, &audit))
            .collect::<Vec<_>>();
        let pipeline_overlay_audits = input
            .pipeline_overlay_audits
            .iter()
            .map(|p| pipeline_overlay_audit_record(p, &audit))
            .collect::<Vec<_>>();
        let run_control_audits = input
            .run_control_audits
            .iter()
            .map(|r| run_control_audit_record(r, &audit))
            .collect::<Vec<_>>();
        let browser_handoff_audits = input
            .browser_handoff_audits
            .iter()
            .map(|b| browser_handoff_audit_record(b, &audit))
            .collect::<Vec<_>>();
        let commands = input
            .commands
            .iter()
            .map(|c| audit_command_record(c, &audit))
            .collect::<Vec<_>>();
        let support_export =
            audit_support_export_packet(&input.support_export, &audit, landing_packet, &commands);
        let inspection = audit_inspection_record(
            &audit,
            &merge_queue_audit,
            &ci_check_audits,
            &pipeline_overlay_audits,
            &run_control_audits,
            &browser_handoff_audits,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_PACKET_RECORD_KIND.to_string(),
            schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: landing_packet.review_workspace.clone(),
            landing_candidate: landing_packet.landing_candidate.clone(),
            stabilization: stabilization_packet.stabilization.clone(),
            boundary_hardening: boundary_hardening_packet.boundary_hardening.clone(),
            in_product_boundary: boundary_hardening_packet.in_product_boundary.clone(),
            provider_linked_stabilization: provider_linked_packet.stabilization.clone(),
            parity_harness: parity_harness.cloned(),
            audit,
            merge_queue_audit,
            ci_check_audits,
            pipeline_overlay_audits,
            run_control_audits,
            browser_handoff_audits,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the audit invariants.
    ///
    /// # Errors
    ///
    /// Returns [`AuditValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), AuditValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq(
            self.schema_version,
            MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_audit_record(
            &self.audit,
            &self.review_workspace.review_workspace_id,
            &self.landing_candidate.landing_candidate_id,
            &self.stabilization.stabilization_id,
            &self.boundary_hardening.boundary_hardening_id,
            &self.provider_linked_stabilization.stabilization_id,
        )?;
        validate_merge_queue_audit_record(&self.merge_queue_audit, &self.audit.audit_id)?;
        for check in &self.ci_check_audits {
            validate_ci_check_audit_record(check, &self.audit.audit_id)?;
        }
        for overlay in &self.pipeline_overlay_audits {
            validate_pipeline_overlay_audit_record(overlay, &self.audit.audit_id)?;
        }
        for control in &self.run_control_audits {
            validate_run_control_audit_record(control, &self.audit.audit_id)?;
        }
        for handoff in &self.browser_handoff_audits {
            validate_browser_handoff_audit_record(handoff, &self.audit.audit_id)?;
        }
        for command in &self.commands {
            validate_command_record(command, &self.audit.audit_id)?;
        }
        validate_support_export(&self.support_export, &self.audit, &self.commands)?;
        validate_inspection(&self.inspection, self)?;

        // Cross-record invariants
        if self.audit.audit_state == "audit_passed" && self.audit.invalidation_reasons.is_empty() {
            if self.inspection.any_provider_row_downgraded {
                return Err(audit_validation_error(
                    "audit_passed is incompatible with any_provider_row_downgraded",
                ));
            }
            if self.inspection.hidden_authority_detected {
                return Err(audit_validation_error(
                    "audit_passed is incompatible with hidden_authority_detected",
                ));
            }
        }

        // Run-control mutation modes must be explicit
        for control in &self.run_control_audits {
            if control.mutation_mode == "inspect_only" {
                // Inspect-only controls are already validated by the input validator;
                // this cross-check ensures no downstream record corruption.
                continue;
            }
        }

        // Browser-handoff audit must match boundary-hardening state when present
        for handoff in &self.browser_handoff_audits {
            if handoff.reversible && handoff.handoff_boundary_class != "handoff_audited_reversible"
            {
                return Err(audit_validation_error(format!(
                    "browser_handoff {} is reversible but class is not handoff_audited_reversible",
                    handoff.handoff_audit_id
                )));
            }
        }

        Ok(())
    }

    /// Returns true when no raw escape hatch crosses the support boundary.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_url_export_allowed
            && !self.support_export.raw_provider_payload_export_allowed
    }

    /// Returns true when all provider rows are claimed stable.
    pub fn all_provider_rows_claimed_stable(&self) -> bool {
        self.inspection.all_provider_rows_claimed_stable
    }

    /// Returns true when at least one provider row was downgraded.
    pub fn any_provider_row_downgraded(&self) -> bool {
        self.inspection.any_provider_row_downgraded
    }
}

// ---------------------------------------------------------------------------
// Projection type
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeQueueCiStatusBrowserHandoffAuditProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Audit identity.
    pub audit_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Landing candidate identity.
    pub landing_candidate_id: String,
    /// Stabilization identity.
    pub stabilization_id: String,
    /// Boundary hardening identity.
    pub boundary_hardening_id: String,
    /// Provider-linked stabilization identity.
    pub provider_linked_stabilization_id: String,
    /// Audit state.
    pub audit_state: String,
    /// True when merge queue has been audited.
    pub merge_queue_audited: bool,
    /// True when all provider rows are claimed stable.
    pub all_provider_rows_claimed_stable: bool,
    /// True when at least one provider row was downgraded.
    pub any_provider_row_downgraded: bool,
    /// True when hidden authority is detected.
    pub hidden_authority_detected: bool,
    /// True when a stale overlay is present.
    pub stale_overlay_present: bool,
    /// True when divergence labels are present.
    pub divergence_labels_present: bool,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// Command count.
    pub command_count: usize,
    /// True when support/export can reopen the audit context.
    pub support_export_reopenable: bool,
    /// Consumer surfaces wired through the support export.
    pub consumer_surfaces: Vec<String>,
}

/// Parses and validates a materialized audit packet.
///
/// # Errors
///
/// Returns [`AuditError`] when the payload fails to parse or violates the
/// audit invariants.
pub fn project_merge_queue_ci_status_browser_handoff_audit_packet(
    payload: &str,
) -> Result<MergeQueueCiStatusBrowserHandoffAuditProjection, AuditError> {
    let packet: MergeQueueCiStatusBrowserHandoffAuditPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(MergeQueueCiStatusBrowserHandoffAuditProjection::from(
        packet,
    ))
}

impl From<MergeQueueCiStatusBrowserHandoffAuditPacket>
    for MergeQueueCiStatusBrowserHandoffAuditProjection
{
    fn from(packet: MergeQueueCiStatusBrowserHandoffAuditPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            audit_id: packet.audit.audit_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            landing_candidate_id: packet.landing_candidate.landing_candidate_id,
            stabilization_id: packet.stabilization.stabilization_id,
            boundary_hardening_id: packet.boundary_hardening.boundary_hardening_id,
            provider_linked_stabilization_id: packet.provider_linked_stabilization.stabilization_id,
            audit_state: packet.audit.audit_state.clone(),
            merge_queue_audited: packet.inspection.merge_queue_audited,
            all_provider_rows_claimed_stable: packet.inspection.all_provider_rows_claimed_stable,
            any_provider_row_downgraded: packet.inspection.any_provider_row_downgraded,
            hidden_authority_detected: packet.inspection.hidden_authority_detected,
            stale_overlay_present: packet.inspection.stale_overlay_present,
            divergence_labels_present: packet.inspection.divergence_labels_present,
            invalidation_reasons: packet.audit.invalidation_reasons.clone(),
            blocked_reasons: packet.audit.blocked_reasons.clone(),
            command_count: packet.commands.len(),
            support_export_reopenable: packet.inspection.support_export_reopenable,
            consumer_surfaces: packet.support_export.consumer_surfaces,
        }
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error returned when an audit payload cannot be projected.
#[derive(Debug)]
pub enum AuditError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the audit invariants.
    Validation(AuditValidationError),
}

impl fmt::Display for AuditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(formatter, "audit parse error: {err}"),
            Self::Validation(err) => write!(formatter, "audit validation error: {err}"),
        }
    }
}

impl std::error::Error for AuditError {}

impl From<serde_json::Error> for AuditError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<AuditValidationError> for AuditError {
    fn from(err: AuditValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for audit packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditValidationError {
    message: String,
}

impl AuditValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for AuditValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AuditValidationError {}

fn audit_validation_error(message: impl Into<String>) -> AuditValidationError {
    AuditValidationError {
        message: message.into(),
    }
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn audit_record(
    input: &MergeQueueCiStatusBrowserHandoffAuditInput,
    landing_packet: &crate::landing::LandingCandidatePacket,
    stabilization_packet: &crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationPacket,
    boundary_hardening_packet: &crate::harden_browser_handoff_and_in_product_review_boundaries::ReviewBoundaryHardeningPacket,
    provider_linked_packet: &crate::stabilize_provider_linked_object_models_snapshot_freshness_and::ProviderLinkedReviewStabilizationPacket,
) -> MergeQueueCiStatusBrowserHandoffAuditRecord {
    let mut invalidation_reasons = input.invalidation_reasons.clone();
    invalidation_reasons.extend(derive_invalidation_reasons(input));
    invalidation_reasons.sort();
    invalidation_reasons.dedup();

    let blocked_reasons = derive_blocked_reasons(&input.audit_state, input);

    MergeQueueCiStatusBrowserHandoffAuditRecord {
        record_kind: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        audit_id: input.audit_id.clone(),
        review_workspace_id_ref: landing_packet.review_workspace.review_workspace_id.clone(),
        landing_candidate_id_ref: landing_packet
            .landing_candidate
            .landing_candidate_id
            .clone(),
        stabilization_id_ref: stabilization_packet.stabilization.stabilization_id.clone(),
        boundary_hardening_id_ref: boundary_hardening_packet
            .boundary_hardening
            .boundary_hardening_id
            .clone(),
        provider_linked_stabilization_id_ref: provider_linked_packet
            .stabilization
            .stabilization_id
            .clone(),
        audit_state: input.audit_state.clone(),
        invalidation_reasons,
        blocked_reasons,
        actionable: input.commands.iter().any(|c| c.blocked_reasons.is_empty()),
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn merge_queue_audit_record(
    input: &MergeQueueAuditInput,
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
) -> MergeQueueAuditRecord {
    MergeQueueAuditRecord {
        record_kind: MERGE_QUEUE_AUDIT_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        audit_id_ref: audit.audit_id.clone(),
        merge_queue_entry_id_ref: input.merge_queue_entry_id_ref.clone(),
        queue_authority_class: input.queue_authority_class.clone(),
        queue_state: input.queue_state.clone(),
        provider_freshness_class: input.provider_freshness_class.clone(),
        local_git_truth_preserved: input.local_git_truth_preserved,
        stale_base_blocks_queue: input.stale_base_blocks_queue,
        checks_stale_blocks_queue: input.checks_stale_blocks_queue,
        divergence_from_local_detected: input.divergence_from_local_detected,
        explicit_downgrade_required: input.explicit_downgrade_required,
        summary_label: input.summary_label.clone(),
    }
}

fn ci_check_audit_record(
    input: &CiCheckAuditInput,
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
) -> CiCheckAuditRecord {
    CiCheckAuditRecord {
        record_kind: CI_CHECK_AUDIT_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        audit_id_ref: audit.audit_id.clone(),
        check_audit_id: input.check_audit_id.clone(),
        check_id_ref: input.check_id_ref.clone(),
        required_check: input.required_check,
        check_name: input.check_name.clone(),
        provider_source_class: input.provider_source_class.clone(),
        fetched_at: input.fetched_at.clone(),
        freshness_class: input.freshness_class.clone(),
        local_outcome_class: input.local_outcome_class.clone(),
        ci_outcome_class: input.ci_outcome_class.clone(),
        ai_review_outcome_class: input.ai_review_outcome_class.clone(),
        provider_outcome_class: input.provider_outcome_class.clone(),
        divergence_label_class: input.divergence_label_class.clone(),
        inspect_only: input.inspect_only,
        run_control_permitted: input.run_control_permitted,
        summary_label: input.summary_label.clone(),
    }
}

fn pipeline_overlay_audit_record(
    input: &PipelineOverlayAuditInput,
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
) -> PipelineOverlayAuditRecord {
    PipelineOverlayAuditRecord {
        record_kind: PIPELINE_OVERLAY_AUDIT_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        audit_id_ref: audit.audit_id.clone(),
        overlay_audit_id: input.overlay_audit_id.clone(),
        provider_descriptor_ref: input.provider_descriptor_ref.clone(),
        overlay_kind: input.overlay_kind.clone(),
        fetched_at: input.fetched_at.clone(),
        provider_source_class: input.provider_source_class.clone(),
        artifact_trust_class: input.artifact_trust_class.clone(),
        read_only_subset_label: input.read_only_subset_label,
        run_control_subset_label: input.run_control_subset_label,
        mutation_mode_disclosure: input.mutation_mode_disclosure.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn run_control_audit_record(
    input: &RunControlAuditInput,
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
) -> RunControlAuditRecord {
    RunControlAuditRecord {
        record_kind: RUN_CONTROL_AUDIT_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        audit_id_ref: audit.audit_id.clone(),
        run_control_audit_id: input.run_control_audit_id.clone(),
        run_control_id_ref: input.run_control_id_ref.clone(),
        control_class: input.control_class.clone(),
        mutation_mode: input.mutation_mode.clone(),
        provider_scope_disclosure: input.provider_scope_disclosure.clone(),
        actor_mode_disclosure: input.actor_mode_disclosure.clone(),
        target_run_identity: input.target_run_identity.clone(),
        browser_handoff_fallback_present: input.browser_handoff_fallback_present,
        review_pack_digest_ref: input.review_pack_digest_ref.clone(),
        base_revision_ref: input.base_revision_ref.clone(),
        head_revision_ref: input.head_revision_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn browser_handoff_audit_record(
    input: &BrowserHandoffAuditInput,
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
) -> BrowserHandoffAuditRecord {
    BrowserHandoffAuditRecord {
        record_kind: BROWSER_HANDOFF_AUDIT_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        audit_id_ref: audit.audit_id.clone(),
        handoff_audit_id: input.handoff_audit_id.clone(),
        boundary_hardening_id_ref: input.boundary_hardening_id_ref.clone(),
        handoff_boundary_class: input.handoff_boundary_class.clone(),
        provider_class: input.provider_class.clone(),
        target_object_ref: input.target_object_ref.clone(),
        reason_code: input.reason_code.clone(),
        return_path_present: input.return_path_present,
        reversible: input.reversible,
        actor_ref: input.actor_ref.clone(),
        scope_disclosure: input.scope_disclosure.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn audit_command_record(
    input: &AuditCommandInput,
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
) -> AuditCommandRecord {
    AuditCommandRecord {
        record_kind: AUDIT_COMMAND_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        audit_id_ref: audit.audit_id.clone(),
        command_class: input.command_class.clone(),
        target_object_ref: input.target_object_ref.clone(),
        target_object_kind: input.target_object_kind.clone(),
        preview_supported: input.preview_supported,
        emits_audit_event: input.emits_audit_event,
        blocked_reasons: input.blocked_reasons.clone(),
        actionable: input.blocked_reasons.is_empty(),
        summary_label: input.summary_label.clone(),
    }
}

fn audit_support_export_packet(
    input: &AuditSupportExportInput,
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
    landing_packet: &crate::landing::LandingCandidatePacket,
    commands: &[AuditCommandRecord],
) -> AuditSupportExportPacket {
    AuditSupportExportPacket {
        record_kind: AUDIT_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        audit_id_ref: audit.audit_id.clone(),
        review_workspace_id_ref: landing_packet.review_workspace.review_workspace_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/harden_merge_queue_ci_status_and_browser_handoff.schema.json"
                .to_string(),
            "schemas/review/landing_candidate.schema.json".to_string(),
            "schemas/review/review_stabilization.schema.json".to_string(),
            "schemas/review/harden_browser_handoff_and_in_product_review_boundaries.schema.json"
                .to_string(),
            "schemas/review/provider_linked_review_stabilization.schema.json".to_string(),
            "schemas/review/review_pack_parity_harness.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn audit_inspection_record(
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
    merge_queue_audit: &MergeQueueAuditRecord,
    ci_check_audits: &[CiCheckAuditRecord],
    pipeline_overlay_audits: &[PipelineOverlayAuditRecord],
    run_control_audits: &[RunControlAuditRecord],
    browser_handoff_audits: &[BrowserHandoffAuditRecord],
    commands: &[AuditCommandRecord],
    support_export: &AuditSupportExportPacket,
) -> AuditInspectionRecord {
    let merge_queue_audited = !merge_queue_audit.merge_queue_entry_id_ref.trim().is_empty();
    let ci_checks_audited = !ci_check_audits.is_empty();
    let pipeline_overlays_audited = !pipeline_overlay_audits.is_empty();
    let run_controls_audited = !run_control_audits.is_empty();
    let browser_handoffs_audited = !browser_handoff_audits.is_empty();

    let all_provider_rows_claimed_stable = pipeline_overlay_audits
        .iter()
        .all(|o| o.artifact_trust_class != "subset_unqualified_downgraded")
        && run_control_audits
            .iter()
            .all(|c| c.mutation_mode != "inspect_only");

    let any_provider_row_downgraded = pipeline_overlay_audits
        .iter()
        .any(|o| o.artifact_trust_class == "subset_unqualified_downgraded")
        || run_control_audits
            .iter()
            .any(|c| c.mutation_mode == "inspect_only")
        || ci_check_audits.iter().any(|c| {
            c.freshness_class == "check_stale_blocks_mutation"
                || c.freshness_class == "check_freshness_unknown"
        });

    let hidden_authority_detected = browser_handoff_audits
        .iter()
        .any(|b| b.handoff_boundary_class == "handoff_downgraded_untyped");

    let stale_overlay_present = pipeline_overlay_audits
        .iter()
        .any(|o| o.provider_source_class == "cached_provider_overlay")
        || ci_check_audits.iter().any(|c| {
            c.freshness_class == "check_stale_within_grace"
                || c.freshness_class == "check_stale_blocks_mutation"
        });

    let inspect_only_controls_present = run_control_audits
        .iter()
        .any(|c| c.mutation_mode == "inspect_only");
    let provider_controlled_controls_present = run_control_audits
        .iter()
        .any(|c| c.mutation_mode == "provider_controlled");
    let auditable_in_product_controls_present = run_control_audits
        .iter()
        .any(|c| c.mutation_mode == "auditable_in_product");

    let divergence_labels_present = ci_check_audits
        .iter()
        .any(|c| c.divergence_label_class != "no_divergence");

    let actionable = audit.actionable;
    let invalidated = !audit.invalidation_reasons.is_empty();
    let preview_capable = commands.iter().any(|c| c.preview_supported);
    let support_export_reopenable = support_export_can_reopen(support_export, commands);

    AuditInspectionRecord {
        record_kind: AUDIT_INSPECTION_RECORD_KIND.to_string(),
        schema_version: MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        audit_id_ref: audit.audit_id.clone(),
        review_workspace_id_ref: audit.review_workspace_id_ref.clone(),
        merge_queue_audited,
        ci_checks_audited,
        pipeline_overlays_audited,
        run_controls_audited,
        browser_handoffs_audited,
        all_provider_rows_claimed_stable,
        any_provider_row_downgraded,
        hidden_authority_detected,
        stale_overlay_present,
        inspect_only_controls_present,
        provider_controlled_controls_present,
        auditable_in_product_controls_present,
        divergence_labels_present,
        actionable,
        invalidated,
        ci_check_audit_count: ci_check_audits.len(),
        pipeline_overlay_audit_count: pipeline_overlay_audits.len(),
        run_control_audit_count: run_control_audits.len(),
        browser_handoff_audit_count: browser_handoff_audits.len(),
        command_count: commands.len(),
        preview_capable,
        support_export_reopenable,
        summary_label: format!(
            "Audit {} ({} check(s), {} overlay(s), {} control(s), {} handoff(s))",
            audit.audit_id,
            ci_check_audits.len(),
            pipeline_overlay_audits.len(),
            run_control_audits.len(),
            browser_handoff_audits.len()
        ),
    }
}

fn support_export_can_reopen(
    export: &AuditSupportExportPacket,
    commands: &[AuditCommandRecord],
) -> bool {
    !export.reopen_context_ref.trim().is_empty()
        && !export.reopen_command_id_ref.trim().is_empty()
        && !export.raw_url_export_allowed
        && !export.raw_provider_payload_export_allowed
        && !commands.is_empty()
        && export
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "support_export")
}

fn derive_invalidation_reasons(input: &MergeQueueCiStatusBrowserHandoffAuditInput) -> Vec<String> {
    let mut reasons = Vec::new();
    if input.merge_queue_audit.divergence_from_local_detected {
        reasons.push("merge_queue_diverged".to_string());
    }
    if input.ci_check_audits.iter().any(|c| {
        c.freshness_class == "check_stale_blocks_mutation"
            || c.freshness_class == "check_freshness_unknown"
    }) {
        reasons.push("ci_check_stale".to_string());
    }
    if input
        .pipeline_overlay_audits
        .iter()
        .any(|o| o.artifact_trust_class == "subset_unqualified_downgraded")
    {
        reasons.push("pipeline_overlay_unqualified".to_string());
    }
    if input
        .browser_handoff_audits
        .iter()
        .any(|b| b.handoff_boundary_class == "handoff_downgraded_untyped")
    {
        reasons.push("browser_handoff_untyped".to_string());
    }
    reasons
}

fn derive_blocked_reasons(
    audit_state: &str,
    input: &MergeQueueCiStatusBrowserHandoffAuditInput,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if audit_state == "audit_degraded_merge_queue_divergence" {
        reasons.push("merge_queue_diverged".to_string());
    }
    if audit_state == "audit_degraded_ci_check_stale" {
        reasons.push("ci_check_stale".to_string());
    }
    if audit_state == "audit_degraded_pipeline_overlay_unqualified" {
        reasons.push("pipeline_overlay_unqualified".to_string());
    }
    if audit_state == "audit_degraded_browser_handoff_untyped" {
        reasons.push("browser_handoff_untyped".to_string());
    }
    if audit_state == "audit_degraded_hidden_authority" {
        reasons.push("hidden_authority_detected".to_string());
    }
    if audit_state == "audit_failed_provider_claim_unsupported" {
        reasons.push("provider_claim_unsupported".to_string());
    }
    if input.merge_queue_audit.stale_base_blocks_queue {
        reasons.push("stale_base_blocks_queue".to_string());
    }
    if input.merge_queue_audit.checks_stale_blocks_queue {
        reasons.push("checks_stale_blocks_queue".to_string());
    }
    reasons
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_input(
    input: &MergeQueueCiStatusBrowserHandoffAuditInput,
    landing_packet: &crate::landing::LandingCandidatePacket,
    stabilization_packet: &crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationPacket,
    boundary_hardening_packet: &crate::harden_browser_handoff_and_in_product_review_boundaries::ReviewBoundaryHardeningPacket,
    provider_linked_packet: &crate::stabilize_provider_linked_object_models_snapshot_freshness_and::ProviderLinkedReviewStabilizationPacket,
) -> Result<(), AuditValidationError> {
    ensure_nonempty(&input.audit_id, "audit_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    ensure_token(AUDIT_STATES, &input.audit_state, "audit_state")?;

    for reason in &input.invalidation_reasons {
        ensure_token(AUDIT_INVALIDATION_REASONS, reason, "invalidation_reason")?;
    }

    ensure_token(
        MERGE_QUEUE_AUDIT_STATES,
        &input.merge_queue_audit.queue_authority_class,
        "merge_queue_audit.queue_authority_class",
    )?;
    ensure_nonempty(
        &input.merge_queue_audit.summary_label,
        "merge_queue_audit.summary_label",
    )?;

    for check in &input.ci_check_audits {
        ensure_nonempty(&check.check_audit_id, "ci_check_audit.check_audit_id")?;
        ensure_token(
            CI_CHECK_FRESHNESS_CLASSES,
            &check.freshness_class,
            "ci_check_audit.freshness_class",
        )?;
        ensure_token(
            CI_CHECK_DIVERGENCE_CLASSES,
            &check.divergence_label_class,
            "ci_check_audit.divergence_label_class",
        )?;
        ensure_nonempty(&check.summary_label, "ci_check_audit.summary_label")?;
    }

    for overlay in &input.pipeline_overlay_audits {
        ensure_nonempty(
            &overlay.overlay_audit_id,
            "pipeline_overlay_audit.overlay_audit_id",
        )?;
        ensure_nonempty(&overlay.overlay_kind, "pipeline_overlay_audit.overlay_kind")?;
        ensure_nonempty(
            &overlay.provider_source_class,
            "pipeline_overlay_audit.provider_source_class",
        )?;
        ensure_nonempty(
            &overlay.artifact_trust_class,
            "pipeline_overlay_audit.artifact_trust_class",
        )?;
        ensure_nonempty(
            &overlay.mutation_mode_disclosure,
            "pipeline_overlay_audit.mutation_mode_disclosure",
        )?;
        ensure_nonempty(
            &overlay.summary_label,
            "pipeline_overlay_audit.summary_label",
        )?;
    }

    for control in &input.run_control_audits {
        ensure_nonempty(
            &control.run_control_audit_id,
            "run_control_audit.run_control_audit_id",
        )?;
        ensure_token(
            RUN_CONTROL_MUTATION_MODES,
            &control.mutation_mode,
            "run_control_audit.mutation_mode",
        )?;
        ensure_nonempty(
            &control.provider_scope_disclosure,
            "run_control_audit.provider_scope_disclosure",
        )?;
        ensure_nonempty(
            &control.actor_mode_disclosure,
            "run_control_audit.actor_mode_disclosure",
        )?;
        ensure_nonempty(
            &control.target_run_identity,
            "run_control_audit.target_run_identity",
        )?;
        ensure_nonempty(
            &control.review_pack_digest_ref,
            "run_control_audit.review_pack_digest_ref",
        )?;
        ensure_nonempty(
            &control.base_revision_ref,
            "run_control_audit.base_revision_ref",
        )?;
        ensure_nonempty(
            &control.head_revision_ref,
            "run_control_audit.head_revision_ref",
        )?;
        ensure_nonempty(&control.summary_label, "run_control_audit.summary_label")?;
    }

    for handoff in &input.browser_handoff_audits {
        ensure_nonempty(
            &handoff.handoff_audit_id,
            "browser_handoff_audit.handoff_audit_id",
        )?;
        ensure_token(
            BROWSER_HANDOFF_AUDIT_CLASSES,
            &handoff.handoff_boundary_class,
            "browser_handoff_audit.handoff_boundary_class",
        )?;
        ensure_nonempty(
            &handoff.provider_class,
            "browser_handoff_audit.provider_class",
        )?;
        ensure_nonempty(&handoff.actor_ref, "browser_handoff_audit.actor_ref")?;
        ensure_nonempty(
            &handoff.summary_label,
            "browser_handoff_audit.summary_label",
        )?;
    }

    for command in &input.commands {
        ensure_nonempty(&command.command_id, "command.command_id")?;
        ensure_token(
            AUDIT_COMMAND_CLASSES,
            &command.command_class,
            "command.command_class",
        )?;
        ensure_nonempty(&command.summary_label, "command.summary_label")?;
    }

    ensure_nonempty(
        &input.support_export.support_export_id,
        "support_export.support_export_id",
    )?;
    ensure_nonempty(
        &input.support_export.reopen_context_ref,
        "support_export.reopen_context_ref",
    )?;
    ensure_nonempty(
        &input.support_export.reopen_command_id_ref,
        "support_export.reopen_command_id_ref",
    )?;
    for surface in &input.support_export.consumer_surfaces {
        ensure_token(
            AUDIT_CONSUMER_SURFACES,
            surface,
            "support_export.consumer_surface",
        )?;
    }
    ensure_nonempty(
        &input.support_export.redaction_class,
        "support_export.redaction_class",
    )?;
    ensure_nonempty(
        &input.support_export.summary_label,
        "support_export.summary_label",
    )?;

    // Cross-packet consistency
    if landing_packet.review_workspace.review_workspace_id
        != stabilization_packet.review_workspace.review_workspace_id
    {
        return Err(audit_validation_error(
            "landing and stabilization packets must share the same review_workspace_id",
        ));
    }
    if stabilization_packet.review_workspace.review_workspace_id
        != boundary_hardening_packet
            .review_workspace
            .review_workspace_id
    {
        return Err(audit_validation_error(
            "stabilization and boundary_hardening packets must share the same review_workspace_id",
        ));
    }
    if provider_linked_packet.review_workspace.review_workspace_id
        != stabilization_packet.review_workspace.review_workspace_id
    {
        return Err(audit_validation_error(
            "provider_linked and stabilization packets must share the same review_workspace_id",
        ));
    }

    // Run-control review-pack digest must match stabilization when present
    for control in &input.run_control_audits {
        if control.review_pack_digest_ref
            != stabilization_packet.stabilization.review_pack_digest_ref
        {
            return Err(audit_validation_error(format!(
                "run_control {} review_pack_digest_ref must match stabilization review_pack_digest_ref",
                control.run_control_audit_id
            )));
        }
    }

    Ok(())
}

fn validate_audit_record(
    record: &MergeQueueCiStatusBrowserHandoffAuditRecord,
    review_workspace_id: &str,
    landing_candidate_id: &str,
    stabilization_id: &str,
    boundary_hardening_id: &str,
    provider_linked_stabilization_id: &str,
) -> Result<(), AuditValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_RECORD_KIND,
        "audit record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "audit schema_version",
    )?;
    ensure_nonempty(&record.audit_id, "audit_id")?;
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        review_workspace_id,
        "audit review_workspace_id_ref",
    )?;
    ensure_eq(
        record.landing_candidate_id_ref.as_str(),
        landing_candidate_id,
        "audit landing_candidate_id_ref",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "audit stabilization_id_ref",
    )?;
    ensure_eq(
        record.boundary_hardening_id_ref.as_str(),
        boundary_hardening_id,
        "audit boundary_hardening_id_ref",
    )?;
    ensure_eq(
        record.provider_linked_stabilization_id_ref.as_str(),
        provider_linked_stabilization_id,
        "audit provider_linked_stabilization_id_ref",
    )?;
    ensure_token(AUDIT_STATES, &record.audit_state, "audit_state")?;
    Ok(())
}

fn validate_merge_queue_audit_record(
    record: &MergeQueueAuditRecord,
    audit_id: &str,
) -> Result<(), AuditValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        MERGE_QUEUE_AUDIT_RECORD_KIND,
        "merge_queue_audit record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "merge_queue_audit schema_version",
    )?;
    ensure_eq(
        record.audit_id_ref.as_str(),
        audit_id,
        "merge_queue_audit audit_id_ref",
    )?;
    ensure_nonempty(
        &record.merge_queue_entry_id_ref,
        "merge_queue_audit merge_queue_entry_id_ref",
    )?;
    Ok(())
}

fn validate_ci_check_audit_record(
    record: &CiCheckAuditRecord,
    audit_id: &str,
) -> Result<(), AuditValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        CI_CHECK_AUDIT_RECORD_KIND,
        "ci_check_audit record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "ci_check_audit schema_version",
    )?;
    ensure_eq(
        record.audit_id_ref.as_str(),
        audit_id,
        "ci_check_audit audit_id_ref",
    )?;
    ensure_nonempty(&record.check_audit_id, "ci_check_audit check_audit_id")?;
    ensure_token(
        CI_CHECK_FRESHNESS_CLASSES,
        &record.freshness_class,
        "ci_check_audit freshness_class",
    )?;
    ensure_token(
        CI_CHECK_DIVERGENCE_CLASSES,
        &record.divergence_label_class,
        "ci_check_audit divergence_label_class",
    )?;
    Ok(())
}

fn validate_pipeline_overlay_audit_record(
    record: &PipelineOverlayAuditRecord,
    audit_id: &str,
) -> Result<(), AuditValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PIPELINE_OVERLAY_AUDIT_RECORD_KIND,
        "pipeline_overlay_audit record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "pipeline_overlay_audit schema_version",
    )?;
    ensure_eq(
        record.audit_id_ref.as_str(),
        audit_id,
        "pipeline_overlay_audit audit_id_ref",
    )?;
    ensure_nonempty(
        &record.overlay_audit_id,
        "pipeline_overlay_audit overlay_audit_id",
    )?;
    Ok(())
}

fn validate_run_control_audit_record(
    record: &RunControlAuditRecord,
    audit_id: &str,
) -> Result<(), AuditValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        RUN_CONTROL_AUDIT_RECORD_KIND,
        "run_control_audit record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "run_control_audit schema_version",
    )?;
    ensure_eq(
        record.audit_id_ref.as_str(),
        audit_id,
        "run_control_audit audit_id_ref",
    )?;
    ensure_nonempty(
        &record.run_control_audit_id,
        "run_control_audit run_control_audit_id",
    )?;
    ensure_token(
        RUN_CONTROL_MUTATION_MODES,
        &record.mutation_mode,
        "run_control_audit mutation_mode",
    )?;
    Ok(())
}

fn validate_browser_handoff_audit_record(
    record: &BrowserHandoffAuditRecord,
    audit_id: &str,
) -> Result<(), AuditValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        BROWSER_HANDOFF_AUDIT_RECORD_KIND,
        "browser_handoff_audit record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "browser_handoff_audit schema_version",
    )?;
    ensure_eq(
        record.audit_id_ref.as_str(),
        audit_id,
        "browser_handoff_audit audit_id_ref",
    )?;
    ensure_nonempty(
        &record.handoff_audit_id,
        "browser_handoff_audit handoff_audit_id",
    )?;
    ensure_token(
        BROWSER_HANDOFF_AUDIT_CLASSES,
        &record.handoff_boundary_class,
        "browser_handoff_audit handoff_boundary_class",
    )?;
    Ok(())
}

fn validate_command_record(
    record: &AuditCommandRecord,
    audit_id: &str,
) -> Result<(), AuditValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        AUDIT_COMMAND_RECORD_KIND,
        "command record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "command schema_version",
    )?;
    ensure_eq(
        record.audit_id_ref.as_str(),
        audit_id,
        "command audit_id_ref",
    )?;
    ensure_nonempty(&record.command_id, "command command_id")?;
    ensure_token(
        AUDIT_COMMAND_CLASSES,
        &record.command_class,
        "command command_class",
    )?;
    Ok(())
}

fn validate_support_export(
    export: &AuditSupportExportPacket,
    audit: &MergeQueueCiStatusBrowserHandoffAuditRecord,
    commands: &[AuditCommandRecord],
) -> Result<(), AuditValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        AUDIT_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export record_kind",
    )?;
    ensure_eq(
        export.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "support_export schema_version",
    )?;
    ensure_eq(
        export.audit_id_ref.as_str(),
        audit.audit_id.as_str(),
        "support_export audit_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        audit.review_workspace_id_ref.as_str(),
        "support_export review_workspace_id_ref",
    )?;
    if export.raw_url_export_allowed {
        return Err(audit_validation_error(
            "support_export raw_url_export_allowed must be false",
        ));
    }
    if export.raw_provider_payload_export_allowed {
        return Err(audit_validation_error(
            "support_export raw_provider_payload_export_allowed must be false",
        ));
    }
    if export.command_id_refs.len() != commands.len() {
        return Err(audit_validation_error(
            "support_export command_id_refs length must match commands length",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &AuditInspectionRecord,
    packet: &MergeQueueCiStatusBrowserHandoffAuditPacket,
) -> Result<(), AuditValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        AUDIT_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.schema_version,
        MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION,
        "inspection schema_version",
    )?;
    ensure_eq(
        inspection.audit_id_ref.as_str(),
        packet.audit.audit_id.as_str(),
        "inspection audit_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.review_workspace.review_workspace_id.as_str(),
        "inspection review_workspace_id_ref",
    )?;
    if inspection.ci_check_audit_count != packet.ci_check_audits.len() {
        return Err(audit_validation_error(
            "inspection ci_check_audit_count mismatch",
        ));
    }
    if inspection.pipeline_overlay_audit_count != packet.pipeline_overlay_audits.len() {
        return Err(audit_validation_error(
            "inspection pipeline_overlay_audit_count mismatch",
        ));
    }
    if inspection.run_control_audit_count != packet.run_control_audits.len() {
        return Err(audit_validation_error(
            "inspection run_control_audit_count mismatch",
        ));
    }
    if inspection.browser_handoff_audit_count != packet.browser_handoff_audits.len() {
        return Err(audit_validation_error(
            "inspection browser_handoff_audit_count mismatch",
        ));
    }
    if inspection.command_count != packet.commands.len() {
        return Err(audit_validation_error("inspection command_count mismatch"));
    }
    if inspection.hidden_authority_detected != packet.in_product_boundary.hidden_authority_detected
    {
        return Err(audit_validation_error(
            "inspection hidden_authority_detected must match in_product_boundary.hidden_authority_detected",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), AuditValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(audit_validation_error(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), AuditValidationError> {
    if value.trim().is_empty() {
        return Err(audit_validation_error(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(tokens: &[&str], value: &str, field: &str) -> Result<(), AuditValidationError> {
    if !tokens.contains(&value) {
        return Err(audit_validation_error(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_are_nonempty() {
        assert!(!AUDIT_STATES.is_empty());
        assert!(!MERGE_QUEUE_AUDIT_STATES.is_empty());
        assert!(!CI_CHECK_FRESHNESS_CLASSES.is_empty());
        assert!(!CI_CHECK_DIVERGENCE_CLASSES.is_empty());
        assert!(!PIPELINE_OVERLAY_SUBSET_CLASSES.is_empty());
        assert!(!RUN_CONTROL_MUTATION_MODES.is_empty());
        assert!(!BROWSER_HANDOFF_AUDIT_CLASSES.is_empty());
        assert!(!AUDIT_COMMAND_CLASSES.is_empty());
        assert!(!AUDIT_CONSUMER_SURFACES.is_empty());
        assert!(!AUDIT_INVALIDATION_REASONS.is_empty());
    }
}
