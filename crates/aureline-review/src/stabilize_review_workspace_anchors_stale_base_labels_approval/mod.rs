//! Stabilized review workspace anchors, stale-base labels, approval invalidation,
//! and mergeability truth for daily-driver review lanes.
//!
//! This module binds review-workspace anchors, landing-candidate truths,
//! review-pack digest, base/head identity, required-check vocabulary, and
//! ownership signals into a single coherent stabilization packet. Every truth
//! axis remains separable and inspectable; stale-pack, partial-scope, and
//! slice-omitted states may not inherit green from adjacent provider rows.
//!
//! The record family includes:
//!
//! - [`ReviewStabilizationRecord`] — stable identity binding workspace, landing,
//!   review pack, and ownership signals.
//! - [`ReviewAnchorStabilityRecord`] — durable anchor bound to the exact
//!   review-pack digest, base/head identity, and required-check vocabulary.
//! - [`StaleBaseLabelRecord`] — explicit stale-base label with divergence class
//!   and exact base revision identity.
//! - [`ApprovalInvalidationRecord`] — approval invalidation with triggering cause
//!   and replayable local-CI/AI-review evidence.
//! - [`MergeabilityTruthRecord`] — mergeability truth bound to required-check
//!   vocabulary and freshness state.
//! - [`OwnershipSignalRecord`] — ownership split between advisory/graph-derived
//!   and enforceable/CODEOWNERS-or-provider-policy classes.
//! - [`ReviewBundleExportRecord`] / [`ReviewBundleImportRecord`] — offline
//!   review bundle preserving review-pack version, divergence labels, and
//!   replayable evidence.
//! - [`OfflineHandoffRecord`] — offline handoff with explicit freshness,
//!   ownership, and return path.
//! - [`ReviewStabilizationCommandRecord`] — command-graph operations surfaced to
//!   the inspector.
//! - [`ReviewStabilizationSupportExportPacket`] — redaction-safe support export.
//! - [`ReviewStabilizationInspectionRecord`] — compact boolean projection for
//!   CLI and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/review_stabilization.schema.json`. Canonical fixtures live
//! under `fixtures/review/m4/stabilize-review-workspace-anchors-stale-base-labels-approval/`.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::landing::LandingCandidatePacket;
use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every review-stabilization record.
pub const REVIEW_STABILIZATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ReviewStabilizationPacket`].
pub const REVIEW_STABILIZATION_PACKET_RECORD_KIND: &str = "review_stabilization_packet";

/// Stable record-kind tag for [`ReviewStabilizationRecord`].
pub const REVIEW_STABILIZATION_RECORD_KIND: &str = "review_stabilization_record";

/// Stable record-kind tag for [`ReviewAnchorStabilityRecord`].
pub const REVIEW_ANCHOR_STABILITY_RECORD_KIND: &str = "review_anchor_stability_record";

/// Stable record-kind tag for [`StaleBaseLabelRecord`].
pub const STALE_BASE_LABEL_RECORD_KIND: &str = "review_stale_base_label_record";

/// Stable record-kind tag for [`ApprovalInvalidationRecord`].
pub const APPROVAL_INVALIDATION_RECORD_KIND: &str = "review_approval_invalidation_record";

/// Stable record-kind tag for [`MergeabilityTruthRecord`].
pub const MERGEABILITY_TRUTH_RECORD_KIND: &str = "review_mergeability_truth_record";

/// Stable record-kind tag for [`OwnershipSignalRecord`].
pub const OWNERSHIP_SIGNAL_RECORD_KIND: &str = "review_ownership_signal_record";

/// Stable record-kind tag for [`ReviewBundleExportRecord`].
pub const REVIEW_BUNDLE_EXPORT_RECORD_KIND: &str = "review_bundle_export_record";

/// Stable record-kind tag for [`ReviewBundleImportRecord`].
pub const REVIEW_BUNDLE_IMPORT_RECORD_KIND: &str = "review_bundle_import_record";

/// Stable record-kind tag for [`OfflineHandoffRecord`].
pub const OFFLINE_HANDOFF_RECORD_KIND: &str = "review_offline_handoff_record";

/// Stable record-kind tag for [`ReviewStabilizationCommandRecord`].
pub const REVIEW_STABILIZATION_COMMAND_RECORD_KIND: &str = "review_stabilization_command_record";

/// Stable record-kind tag for [`ReviewStabilizationSupportExportPacket`].
pub const REVIEW_STABILIZATION_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_stabilization_support_export_packet";

/// Stable record-kind tag for [`ReviewStabilizationInspectionRecord`].
pub const REVIEW_STABILIZATION_INSPECTION_RECORD_KIND: &str =
    "review_stabilization_inspection_record";

/// Closed set of stabilization states.
pub const STABILIZATION_STATES: &[&str] = &[
    "stabilized_current",
    "stabilized_stale_pack",
    "stabilized_partial_scope",
    "stabilized_slice_omitted",
    "stabilized_diverged_requires_review",
];

/// Closed set of anchor stability classes.
pub const ANCHOR_STABILITY_CLASSES: &[&str] = &[
    "anchor_bound_exact",
    "anchor_bound_stale_pack",
    "anchor_bound_partial_scope",
    "anchor_drifted_requires_review",
];

/// Closed set of stale-base label classes.
pub const STALE_BASE_LABEL_CLASSES: &[&str] = &[
    "base_current",
    "base_stale_within_grace",
    "base_stale_blocks_landing",
    "base_diverged_requires_rebase",
    "base_diverged_requires_merge",
];

/// Closed set of approval-invalidation trigger classes.
pub const APPROVAL_INVALIDATION_TRIGGER_CLASSES: &[&str] = &[
    "head_changed_after_approval",
    "base_changed_after_approval",
    "review_pack_changed_after_approval",
    "check_failure_after_approval",
    "manual_invalidation_by_reviewer",
];

/// Closed set of mergeability truth classes.
pub const MERGEABILITY_TRUTH_CLASSES: &[&str] = &[
    "mergeable",
    "not_mergeable_blocking",
    "mergeable_pending_eligibility",
    "mergeability_unknown_requires_review",
];

/// Closed set of ownership signal classes.
pub const OWNERSHIP_SIGNAL_CLASSES: &[&str] = &[
    "advisory_graph_derived",
    "enforceable_codeowners_policy",
    "enforceable_provider_policy",
    "advisory_and_enforceable_match",
    "advisory_and_enforceable_conflict",
];

/// Closed set of bundle export states.
pub const BUNDLE_EXPORT_STATES: &[&str] = &[
    "export_ready",
    "export_in_progress",
    "export_completed",
    "export_failed",
];

/// Closed set of bundle import states.
pub const BUNDLE_IMPORT_STATES: &[&str] = &[
    "import_ready",
    "import_in_progress",
    "import_completed",
    "import_failed",
    "import_rejected_version_mismatch",
];

/// Closed set of offline handoff states.
pub const OFFLINE_HANDOFF_STATES: &[&str] = &[
    "handoff_ready",
    "handoff_transferred",
    "handoff_received",
    "handoff_replay_verified",
];

/// Closed set of replay evidence classes.
pub const REPLAY_EVIDENCE_CLASSES: &[&str] = &[
    "local_ci_evidence",
    "ai_review_evidence",
    "human_review_evidence",
    "provider_check_evidence",
];

/// Closed set of divergence label classes.
pub const DIVERGENCE_LABEL_CLASSES: &[&str] = &[
    "no_divergence",
    "local_ahead",
    "remote_ahead",
    "diverged_requires_rebase",
    "diverged_requires_merge",
];

/// Closed set of command classes for the stabilization lane.
pub const REVIEW_STABILIZATION_COMMAND_CLASSES: &[&str] = &[
    "preview_stabilization",
    "approve_stabilization",
    "invalidate_anchors",
    "refresh_base_truth",
    "refresh_mergeability",
    "export_review_bundle",
    "import_review_bundle",
    "request_offline_handoff",
    "replay_evidence",
    "publish_to_provider",
];

/// Closed set of consumer surfaces for stabilization packets.
pub const REVIEW_STABILIZATION_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "review_landing_strip",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
    "offline_handoff",
];

/// Closed set of invalidation reasons that mark a stabilization stale.
pub const REVIEW_STABILIZATION_INVALIDATION_REASONS: &[&str] = &[
    "stale_base",
    "checks_stale",
    "approval_invalidated",
    "policy_blocked",
    "review_pack_version_changed",
    "worktree_scope_changed",
    "environment_capsule_changed",
    "provider_overlay_stale",
    "anchor_drifted",
    "partial_scope_omission",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a review stabilization to materialize on top of a beta
/// review-workspace packet and a landing-candidate packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStabilizationInput {
    /// Stable stabilization identity.
    pub stabilization_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Stabilization state from the closed vocabulary.
    pub stabilization_state: String,
    /// Review-pack digest pinned at stabilization time.
    pub review_pack_digest_ref: String,
    /// Base revision ref the stabilization was computed against.
    pub base_revision_ref: String,
    /// Head revision ref the stabilization was computed against.
    pub head_revision_ref: String,
    /// Required-check identifiers from the workspace check-freshness rows.
    pub required_check_ids: Vec<String>,
    /// Anchor stability inputs for every anchor in the workspace.
    pub anchor_stabilities: Vec<ReviewAnchorStabilityInput>,
    /// Stale-base label input.
    pub stale_base_label: StaleBaseLabelInput,
    /// Approval invalidation input, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_invalidation: Option<ApprovalInvalidationInput>,
    /// Mergeability truth input.
    pub mergeability_truth: MergeabilityTruthInput,
    /// Ownership signal inputs.
    pub ownership_signals: Vec<OwnershipSignalInput>,
    /// Optional bundle export input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_export: Option<ReviewBundleExportInput>,
    /// Optional bundle import input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_import: Option<ReviewBundleImportInput>,
    /// Optional offline handoff input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_handoff: Option<OfflineHandoffInput>,
    /// Command-graph operations defined for this stabilization.
    pub commands: Vec<ReviewStabilizationCommandInput>,
    /// Support/export envelope input.
    pub support_export: ReviewStabilizationSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing the stability of one review anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAnchorStabilityInput {
    /// Anchor id ref from the workspace beta packet.
    pub anchor_id_ref: String,
    /// Anchor stability class from the closed vocabulary.
    pub anchor_stability_class: String,
    /// Review-pack digest that produced this anchor binding.
    pub bound_review_pack_digest_ref: String,
    /// Base revision ref at anchor binding time.
    pub bound_base_revision_ref: String,
    /// Head revision ref at anchor binding time.
    pub bound_head_revision_ref: String,
    /// Required-check vocabulary snapshot at binding time.
    pub bound_required_check_ids: Vec<String>,
    /// True when the anchor excludes provider ids from its stable hash.
    pub provider_excluded_from_anchor_hash: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a stale-base label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleBaseLabelInput {
    /// Stale-base label class from the closed vocabulary.
    pub stale_base_label_class: String,
    /// Base revision ref at label time.
    pub base_revision_ref: String,
    /// Head revision ref at label time.
    pub head_revision_ref: String,
    /// Divergence label class at label time.
    pub divergence_label_class: String,
    /// True when the stale base blocks landing.
    pub blocks_landing: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing an approval invalidation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalInvalidationInput {
    /// Approval invalidation trigger class from the closed vocabulary.
    pub invalidation_trigger_class: String,
    /// Actor that triggered the invalidation, when known.
    pub actor_ref: String,
    /// Timestamp of the invalidation event.
    pub invalidated_at: String,
    /// Replayable evidence refs (local-CI/AI-review/human-review).
    pub replay_evidence_refs: Vec<String>,
    /// Evidence classes for each replay evidence ref.
    pub replay_evidence_classes: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing mergeability truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeabilityTruthInput {
    /// Mergeability truth class from the closed vocabulary.
    pub mergeability_truth_class: String,
    /// Required-check identifiers that factor into mergeability.
    pub required_check_ids: Vec<String>,
    /// Checks-freshness state at mergeability observation time.
    pub checks_freshness_state: String,
    /// True when mergeability is provider-authoritative.
    pub provider_authoritative: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing an ownership signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipSignalInput {
    /// Ownership signal class from the closed vocabulary.
    pub ownership_signal_class: String,
    /// Scope path or pattern the ownership applies to.
    pub scope_pattern: String,
    /// Owner ref (team, individual, or policy identifier).
    pub owner_ref: String,
    /// True when this signal is enforceable (CODEOWNERS or provider policy).
    pub enforceable: bool,
    /// True when this signal is advisory (graph-derived or heuristic).
    pub advisory: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a review bundle export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewBundleExportInput {
    /// Stable bundle export identity.
    pub bundle_export_id: String,
    /// Bundle export state from the closed vocabulary.
    pub export_state: String,
    /// Review-pack version preserved in the bundle.
    pub review_pack_version: String,
    /// Divergence labels preserved in the bundle.
    pub divergence_labels: Vec<String>,
    /// Replayable evidence refs included in the bundle.
    pub replay_evidence_refs: Vec<String>,
    /// Consumer surfaces that can read this bundle.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a review bundle import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewBundleImportInput {
    /// Stable bundle import identity.
    pub bundle_import_id: String,
    /// Bundle import state from the closed vocabulary.
    pub import_state: String,
    /// Review-pack version expected by the import.
    pub expected_review_pack_version: String,
    /// Divergence labels expected by the import.
    pub expected_divergence_labels: Vec<String>,
    /// Consumer surfaces that can read this bundle.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing an offline handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineHandoffInput {
    /// Stable offline handoff identity.
    pub handoff_id: String,
    /// Offline handoff state from the closed vocabulary.
    pub handoff_state: String,
    /// Freshness class at handoff time.
    pub freshness_class: String,
    /// Actor that minted the handoff.
    pub actor_ref: String,
    /// Return anchor ref for reversible handoff.
    pub return_anchor_ref: String,
    /// Review-pack version preserved in the handoff.
    pub review_pack_version: String,
    /// Divergence labels preserved in the handoff.
    pub divergence_labels: Vec<String>,
    /// Replayable evidence refs included in the handoff.
    pub replay_evidence_refs: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing one command-graph operation for a review stabilization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStabilizationCommandInput {
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

/// Input row for the review-stabilization support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStabilizationSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the stabilization.
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

/// Review-stabilization record materialized from input plus workspace and
/// landing truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStabilizationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable stabilization identity.
    pub stabilization_id: String,
    /// Review workspace this stabilization belongs to.
    pub review_workspace_id_ref: String,
    /// Landing candidate this stabilization binds.
    pub landing_candidate_id_ref: String,
    /// Stabilization state.
    pub stabilization_state: String,
    /// Review-pack digest pinned at stabilization time.
    pub review_pack_digest_ref: String,
    /// Base revision ref.
    pub base_revision_ref: String,
    /// Head revision ref.
    pub head_revision_ref: String,
    /// Required-check identifiers.
    pub required_check_ids: Vec<String>,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing stabilization approval.
    pub blocked_reasons: Vec<String>,
    /// True when the stabilization is actionable from the current state.
    pub actionable: bool,
    /// Timestamp the stabilization was frozen.
    pub generated_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Anchor stability record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAnchorStabilityRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this anchor stability belongs to.
    pub stabilization_id_ref: String,
    /// Anchor id ref from the workspace.
    pub anchor_id_ref: String,
    /// Anchor stability class.
    pub anchor_stability_class: String,
    /// Review-pack digest that produced this anchor binding.
    pub bound_review_pack_digest_ref: String,
    /// Base revision ref at anchor binding time.
    pub bound_base_revision_ref: String,
    /// Head revision ref at anchor binding time.
    pub bound_head_revision_ref: String,
    /// Required-check vocabulary snapshot at binding time.
    pub bound_required_check_ids: Vec<String>,
    /// True when the anchor excludes provider ids from its stable hash.
    pub provider_excluded_from_anchor_hash: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Stale-base label record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleBaseLabelRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this label belongs to.
    pub stabilization_id_ref: String,
    /// Stale-base label class.
    pub stale_base_label_class: String,
    /// Base revision ref at label time.
    pub base_revision_ref: String,
    /// Head revision ref at label time.
    pub head_revision_ref: String,
    /// Divergence label class.
    pub divergence_label_class: String,
    /// True when the stale base blocks landing.
    pub blocks_landing: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Approval invalidation record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalInvalidationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this invalidation belongs to.
    pub stabilization_id_ref: String,
    /// Approval invalidation trigger class.
    pub invalidation_trigger_class: String,
    /// Actor that triggered the invalidation.
    pub actor_ref: String,
    /// Timestamp of the invalidation event.
    pub invalidated_at: String,
    /// Replayable evidence refs.
    pub replay_evidence_refs: Vec<String>,
    /// Evidence classes for each replay evidence ref.
    pub replay_evidence_classes: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Mergeability truth record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeabilityTruthRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this mergeability truth belongs to.
    pub stabilization_id_ref: String,
    /// Mergeability truth class.
    pub mergeability_truth_class: String,
    /// Required-check identifiers.
    pub required_check_ids: Vec<String>,
    /// Checks-freshness state.
    pub checks_freshness_state: String,
    /// True when mergeability is provider-authoritative.
    pub provider_authoritative: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Ownership signal record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipSignalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this ownership signal belongs to.
    pub stabilization_id_ref: String,
    /// Ownership signal class.
    pub ownership_signal_class: String,
    /// Scope path or pattern.
    pub scope_pattern: String,
    /// Owner ref.
    pub owner_ref: String,
    /// True when this signal is enforceable.
    pub enforceable: bool,
    /// True when this signal is advisory.
    pub advisory: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Review bundle export record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewBundleExportRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this export belongs to.
    pub stabilization_id_ref: String,
    /// Stable bundle export identity.
    pub bundle_export_id: String,
    /// Bundle export state.
    pub export_state: String,
    /// Review-pack version preserved in the bundle.
    pub review_pack_version: String,
    /// Divergence labels preserved in the bundle.
    pub divergence_labels: Vec<String>,
    /// Replayable evidence refs included in the bundle.
    pub replay_evidence_refs: Vec<String>,
    /// Consumer surfaces that can read this bundle.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Review bundle import record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewBundleImportRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this import belongs to.
    pub stabilization_id_ref: String,
    /// Stable bundle import identity.
    pub bundle_import_id: String,
    /// Bundle import state.
    pub import_state: String,
    /// Review-pack version expected by the import.
    pub expected_review_pack_version: String,
    /// Divergence labels expected by the import.
    pub expected_divergence_labels: Vec<String>,
    /// Consumer surfaces that can read this bundle.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Offline handoff record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineHandoffRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this handoff belongs to.
    pub stabilization_id_ref: String,
    /// Stable offline handoff identity.
    pub handoff_id: String,
    /// Offline handoff state.
    pub handoff_state: String,
    /// Freshness class at handoff time.
    pub freshness_class: String,
    /// Actor that minted the handoff.
    pub actor_ref: String,
    /// Return anchor ref for reversible handoff.
    pub return_anchor_ref: String,
    /// Review-pack version preserved in the handoff.
    pub review_pack_version: String,
    /// Divergence labels preserved in the handoff.
    pub divergence_labels: Vec<String>,
    /// Replayable evidence refs included in the handoff.
    pub replay_evidence_refs: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Command-graph operation record for a review stabilization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStabilizationCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Stabilization this command belongs to.
    pub stabilization_id_ref: String,
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
    /// True when the command is actionable from the current stabilization state.
    pub actionable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet for the review-stabilization lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStabilizationSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stabilization this packet exports.
    pub stabilization_id_ref: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Landing candidate this packet exports.
    pub landing_candidate_id_ref: String,
    /// Stable context ref used to reopen the stabilization.
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
pub struct ReviewStabilizationInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization inspected by this row.
    pub stabilization_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the stabilization is current.
    pub stabilized_current: bool,
    /// True when the stabilization is stale-pack.
    pub stabilized_stale_pack: bool,
    /// True when the stabilization is partial-scope.
    pub stabilized_partial_scope: bool,
    /// True when the stabilization is slice-omitted.
    pub stabilized_slice_omitted: bool,
    /// True when the stabilization is diverged and requires review.
    pub stabilized_diverged_requires_review: bool,
    /// True when every anchor is bound exact.
    pub all_anchors_bound_exact: bool,
    /// True when at least one anchor is drifted.
    pub any_anchor_drifted: bool,
    /// True when the stale base blocks landing.
    pub stale_base_blocks_landing: bool,
    /// True when approval is invalidated.
    pub approval_invalidated: bool,
    /// True when mergeability is blocking.
    pub mergeability_blocking: bool,
    /// True when mergeability is provider-authoritative.
    pub mergeability_provider_authoritative: bool,
    /// True when at least one ownership signal is enforceable.
    pub enforceable_ownership_present: bool,
    /// True when at least one ownership signal is advisory.
    pub advisory_ownership_present: bool,
    /// True when advisory and enforceable ownership signals conflict.
    pub ownership_conflict_present: bool,
    /// True when a bundle export is present.
    pub bundle_export_present: bool,
    /// True when a bundle import is present.
    pub bundle_import_present: bool,
    /// True when an offline handoff is present.
    pub offline_handoff_present: bool,
    /// True when the stabilization is actionable.
    pub actionable: bool,
    /// True when the stabilization is invalidated by any reason.
    pub invalidated: bool,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// Number of anchor stability records.
    pub anchor_stability_count: usize,
    /// Number of ownership signal records.
    pub ownership_signal_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the stabilization context.
    pub support_export_reopenable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Review-stabilization packet consumed by review surfaces and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStabilizationPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the packet.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Review workspace summary copied from the beta packet.
    pub review_workspace: crate::workspace::ReviewWorkspaceRecord,
    /// Landing candidate summary copied from the landing packet.
    pub landing_candidate: crate::landing::LandingCandidateRecord,
    /// Review-stabilization record.
    pub stabilization: ReviewStabilizationRecord,
    /// Anchor stability records.
    pub anchor_stabilities: Vec<ReviewAnchorStabilityRecord>,
    /// Stale-base label record.
    pub stale_base_label: StaleBaseLabelRecord,
    /// Optional approval invalidation record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_invalidation: Option<ApprovalInvalidationRecord>,
    /// Mergeability truth record.
    pub mergeability_truth: MergeabilityTruthRecord,
    /// Ownership signal records.
    pub ownership_signals: Vec<OwnershipSignalRecord>,
    /// Optional bundle export record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_export: Option<ReviewBundleExportRecord>,
    /// Optional bundle import record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_import: Option<ReviewBundleImportRecord>,
    /// Optional offline handoff record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_handoff: Option<OfflineHandoffRecord>,
    /// Command-graph operation records.
    pub commands: Vec<ReviewStabilizationCommandRecord>,
    /// Support/export packet.
    pub support_export: ReviewStabilizationSupportExportPacket,
    /// Inspection row.
    pub inspection: ReviewStabilizationInspectionRecord,
}

impl ReviewStabilizationPacket {
    /// Builds a review-stabilization packet from a beta review-workspace packet,
    /// a landing-candidate packet, and stabilization input.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewStabilizationValidationError`] when the input violates a
    /// review-stabilization invariant.
    pub fn from_workspace_and_landing_packets(
        input: ReviewStabilizationInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
        landing_packet: &LandingCandidatePacket,
    ) -> Result<Self, ReviewStabilizationValidationError> {
        validate_input(&input, workspace_packet, landing_packet)?;

        let stabilization = stabilization_record(&input, workspace_packet, landing_packet);
        let anchor_stabilities = input
            .anchor_stabilities
            .iter()
            .map(|a| anchor_stability_record(a, &stabilization))
            .collect::<Vec<_>>();
        let stale_base_label = stale_base_label_record(&input.stale_base_label, &stabilization);
        let approval_invalidation = input
            .approval_invalidation
            .as_ref()
            .map(|i| approval_invalidation_record(i, &stabilization));
        let mergeability_truth =
            mergeability_truth_record(&input.mergeability_truth, &stabilization);
        let ownership_signals = input
            .ownership_signals
            .iter()
            .map(|o| ownership_signal_record(o, &stabilization))
            .collect::<Vec<_>>();
        let bundle_export = input
            .bundle_export
            .as_ref()
            .map(|b| bundle_export_record(b, &stabilization));
        let bundle_import = input
            .bundle_import
            .as_ref()
            .map(|b| bundle_import_record(b, &stabilization));
        let offline_handoff = input
            .offline_handoff
            .as_ref()
            .map(|h| offline_handoff_record(h, &stabilization));
        let commands = input
            .commands
            .iter()
            .map(|command| stabilization_command_record(command, &stabilization))
            .collect::<Vec<_>>();
        let support_export = stabilization_support_export_packet(
            &input.support_export,
            &stabilization,
            workspace_packet,
            landing_packet,
            &commands,
        );
        let inspection = stabilization_inspection_record(
            &stabilization,
            &anchor_stabilities,
            &stale_base_label,
            approval_invalidation.as_ref(),
            &mergeability_truth,
            &ownership_signals,
            bundle_export.as_ref(),
            bundle_import.as_ref(),
            offline_handoff.as_ref(),
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: REVIEW_STABILIZATION_PACKET_RECORD_KIND.to_string(),
            schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            landing_candidate: landing_packet.landing_candidate.clone(),
            stabilization,
            anchor_stabilities,
            stale_base_label,
            approval_invalidation,
            mergeability_truth,
            ownership_signals,
            bundle_export,
            bundle_import,
            offline_handoff,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the review-stabilization invariants.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewStabilizationValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), ReviewStabilizationValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            REVIEW_STABILIZATION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq(
            self.schema_version,
            REVIEW_STABILIZATION_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_stabilization_record(
            &self.stabilization,
            &self.review_workspace.review_workspace_id,
            &self.landing_candidate.landing_candidate_id,
        )?;
        for anchor in &self.anchor_stabilities {
            validate_anchor_stability_record(anchor, &self.stabilization.stabilization_id)?;
        }
        validate_stale_base_label_record(
            &self.stale_base_label,
            &self.stabilization.stabilization_id,
        )?;
        if let Some(invalidation) = &self.approval_invalidation {
            validate_approval_invalidation_record(
                invalidation,
                &self.stabilization.stabilization_id,
            )?;
        }
        validate_mergeability_truth_record(
            &self.mergeability_truth,
            &self.stabilization.stabilization_id,
        )?;
        for signal in &self.ownership_signals {
            validate_ownership_signal_record(signal, &self.stabilization.stabilization_id)?;
        }
        if let Some(export) = &self.bundle_export {
            validate_bundle_export_record(export, &self.stabilization.stabilization_id)?;
        }
        if let Some(import) = &self.bundle_import {
            validate_bundle_import_record(import, &self.stabilization.stabilization_id)?;
        }
        if let Some(handoff) = &self.offline_handoff {
            validate_offline_handoff_record(handoff, &self.stabilization.stabilization_id)?;
        }
        for command in &self.commands {
            validate_command_record(command, &self.stabilization.stabilization_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.stabilization,
            &self.commands,
        )?;
        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when stabilization-truth axes are surfaced as separable
    /// inspectable truths.
    pub fn truths_are_separable(&self) -> bool {
        contains_token(STABILIZATION_STATES, &self.stabilization.stabilization_state)
            && contains_token(
                STALE_BASE_LABEL_CLASSES,
                &self.stale_base_label.stale_base_label_class,
            )
            && contains_token(
                MERGEABILITY_TRUTH_CLASSES,
                &self.mergeability_truth.mergeability_truth_class,
            )
            && self
                .ownership_signals
                .iter()
                .all(|s| contains_token(OWNERSHIP_SIGNAL_CLASSES, &s.ownership_signal_class))
    }

    /// Returns true when no raw escape hatch crosses the support boundary.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_url_export_allowed
            && !self.support_export.raw_provider_payload_export_allowed
    }

    /// Returns true when every anchor is bound to the exact review-pack digest
    /// and base/head identity.
    pub fn anchors_bound_exact(&self) -> bool {
        self.anchor_stabilities.iter().all(|a| {
            a.anchor_stability_class == "anchor_bound_exact"
                && a.provider_excluded_from_anchor_hash
        })
    }

    /// Returns true when every ownership signal is at least advisory or
    /// enforceable. Conflict signals that are both are permitted as explicit
    /// conflict records.
    pub fn ownership_signals_properly_split(&self) -> bool {
        self.ownership_signals
            .iter()
            .all(|s| s.advisory || s.enforceable)
    }
}

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewStabilizationProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Stabilization identity.
    pub stabilization_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Landing candidate identity.
    pub landing_candidate_id: String,
    /// Stabilization state.
    pub stabilization_state: String,
    /// True when every anchor is bound exact.
    pub all_anchors_bound_exact: bool,
    /// True when stale base blocks landing.
    pub stale_base_blocks_landing: bool,
    /// True when approval is invalidated.
    pub approval_invalidated: bool,
    /// True when mergeability is blocking.
    pub mergeability_blocking: bool,
    /// True when ownership signals conflict.
    pub ownership_conflict_present: bool,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// Command count.
    pub command_count: usize,
    /// True when support/export can reopen the stabilization context.
    pub support_export_reopenable: bool,
    /// Consumer surfaces wired through the support export.
    pub consumer_surfaces: Vec<String>,
}

/// Parses and validates a materialized review-stabilization packet.
///
/// # Errors
///
/// Returns [`ReviewStabilizationError`] when the payload fails to parse or
/// violates the review-stabilization invariants.
pub fn project_review_stabilization_packet(
    payload: &str,
) -> Result<ReviewStabilizationProjection, ReviewStabilizationError> {
    let packet: ReviewStabilizationPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(ReviewStabilizationProjection::from(packet))
}

impl From<ReviewStabilizationPacket> for ReviewStabilizationProjection {
    fn from(packet: ReviewStabilizationPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            stabilization_id: packet.stabilization.stabilization_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            landing_candidate_id: packet.landing_candidate.landing_candidate_id,
            stabilization_state: packet.stabilization.stabilization_state,
            all_anchors_bound_exact: packet.inspection.all_anchors_bound_exact,
            stale_base_blocks_landing: packet.inspection.stale_base_blocks_landing,
            approval_invalidated: packet.inspection.approval_invalidated,
            mergeability_blocking: packet.inspection.mergeability_blocking,
            ownership_conflict_present: packet.inspection.ownership_conflict_present,
            invalidation_reasons: packet.stabilization.invalidation_reasons.clone(),
            blocked_reasons: packet.stabilization.blocked_reasons.clone(),
            command_count: packet.commands.len(),
            support_export_reopenable: packet.inspection.support_export_reopenable,
            consumer_surfaces: packet.support_export.consumer_surfaces,
        }
    }
}

/// Error returned when a review-stabilization payload cannot be projected.
#[derive(Debug)]
pub enum ReviewStabilizationError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the review-stabilization invariants.
    Validation(ReviewStabilizationValidationError),
}

impl fmt::Display for ReviewStabilizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(formatter, "review stabilization parse error: {err}"),
            Self::Validation(err) => {
                write!(formatter, "review stabilization validation error: {err}")
            }
        }
    }
}

impl std::error::Error for ReviewStabilizationError {}

impl From<serde_json::Error> for ReviewStabilizationError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<ReviewStabilizationValidationError> for ReviewStabilizationError {
    fn from(err: ReviewStabilizationValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for review-stabilization packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewStabilizationValidationError {
    message: String,
}

impl ReviewStabilizationValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ReviewStabilizationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ReviewStabilizationValidationError {}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn stabilization_record(
    input: &ReviewStabilizationInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    landing_packet: &LandingCandidatePacket,
) -> ReviewStabilizationRecord {
    let mut invalidation_reasons = input.invalidation_reasons.clone();
    invalidation_reasons.extend(derive_invalidation_reasons(
        &input.stale_base_label.stale_base_label_class,
        &input.mergeability_truth.checks_freshness_state,
        input.approval_invalidation.as_ref(),
    ));
    invalidation_reasons.sort();
    invalidation_reasons.dedup();

    let blocked_reasons = derive_blocked_reasons(
        &input.stabilization_state,
        &input.stale_base_label.stale_base_label_class,
        &input.mergeability_truth.mergeability_truth_class,
        input.approval_invalidation.as_ref(),
        &invalidation_reasons,
    );

    ReviewStabilizationRecord {
        record_kind: REVIEW_STABILIZATION_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id: input.stabilization_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        landing_candidate_id_ref: landing_packet.landing_candidate.landing_candidate_id.clone(),
        stabilization_state: input.stabilization_state.clone(),
        review_pack_digest_ref: input.review_pack_digest_ref.clone(),
        base_revision_ref: input.base_revision_ref.clone(),
        head_revision_ref: input.head_revision_ref.clone(),
        required_check_ids: input.required_check_ids.clone(),
        invalidation_reasons,
        blocked_reasons,
        actionable: input.commands.iter().any(|c| c.blocked_reasons.is_empty()),
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn anchor_stability_record(
    input: &ReviewAnchorStabilityInput,
    stabilization: &ReviewStabilizationRecord,
) -> ReviewAnchorStabilityRecord {
    ReviewAnchorStabilityRecord {
        record_kind: REVIEW_ANCHOR_STABILITY_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        anchor_id_ref: input.anchor_id_ref.clone(),
        anchor_stability_class: input.anchor_stability_class.clone(),
        bound_review_pack_digest_ref: input.bound_review_pack_digest_ref.clone(),
        bound_base_revision_ref: input.bound_base_revision_ref.clone(),
        bound_head_revision_ref: input.bound_head_revision_ref.clone(),
        bound_required_check_ids: input.bound_required_check_ids.clone(),
        provider_excluded_from_anchor_hash: input.provider_excluded_from_anchor_hash,
        summary_label: input.summary_label.clone(),
    }
}

fn stale_base_label_record(
    input: &StaleBaseLabelInput,
    stabilization: &ReviewStabilizationRecord,
) -> StaleBaseLabelRecord {
    StaleBaseLabelRecord {
        record_kind: STALE_BASE_LABEL_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        stale_base_label_class: input.stale_base_label_class.clone(),
        base_revision_ref: input.base_revision_ref.clone(),
        head_revision_ref: input.head_revision_ref.clone(),
        divergence_label_class: input.divergence_label_class.clone(),
        blocks_landing: input.blocks_landing,
        summary_label: input.summary_label.clone(),
    }
}

fn approval_invalidation_record(
    input: &ApprovalInvalidationInput,
    stabilization: &ReviewStabilizationRecord,
) -> ApprovalInvalidationRecord {
    ApprovalInvalidationRecord {
        record_kind: APPROVAL_INVALIDATION_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        invalidation_trigger_class: input.invalidation_trigger_class.clone(),
        actor_ref: input.actor_ref.clone(),
        invalidated_at: input.invalidated_at.clone(),
        replay_evidence_refs: input.replay_evidence_refs.clone(),
        replay_evidence_classes: input.replay_evidence_classes.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn mergeability_truth_record(
    input: &MergeabilityTruthInput,
    stabilization: &ReviewStabilizationRecord,
) -> MergeabilityTruthRecord {
    MergeabilityTruthRecord {
        record_kind: MERGEABILITY_TRUTH_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        mergeability_truth_class: input.mergeability_truth_class.clone(),
        required_check_ids: input.required_check_ids.clone(),
        checks_freshness_state: input.checks_freshness_state.clone(),
        provider_authoritative: input.provider_authoritative,
        summary_label: input.summary_label.clone(),
    }
}

fn ownership_signal_record(
    input: &OwnershipSignalInput,
    stabilization: &ReviewStabilizationRecord,
) -> OwnershipSignalRecord {
    OwnershipSignalRecord {
        record_kind: OWNERSHIP_SIGNAL_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        ownership_signal_class: input.ownership_signal_class.clone(),
        scope_pattern: input.scope_pattern.clone(),
        owner_ref: input.owner_ref.clone(),
        enforceable: input.enforceable,
        advisory: input.advisory,
        summary_label: input.summary_label.clone(),
    }
}

fn bundle_export_record(
    input: &ReviewBundleExportInput,
    stabilization: &ReviewStabilizationRecord,
) -> ReviewBundleExportRecord {
    ReviewBundleExportRecord {
        record_kind: REVIEW_BUNDLE_EXPORT_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        bundle_export_id: input.bundle_export_id.clone(),
        export_state: input.export_state.clone(),
        review_pack_version: input.review_pack_version.clone(),
        divergence_labels: input.divergence_labels.clone(),
        replay_evidence_refs: input.replay_evidence_refs.clone(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn bundle_import_record(
    input: &ReviewBundleImportInput,
    stabilization: &ReviewStabilizationRecord,
) -> ReviewBundleImportRecord {
    ReviewBundleImportRecord {
        record_kind: REVIEW_BUNDLE_IMPORT_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        bundle_import_id: input.bundle_import_id.clone(),
        import_state: input.import_state.clone(),
        expected_review_pack_version: input.expected_review_pack_version.clone(),
        expected_divergence_labels: input.expected_divergence_labels.clone(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn offline_handoff_record(
    input: &OfflineHandoffInput,
    stabilization: &ReviewStabilizationRecord,
) -> OfflineHandoffRecord {
    OfflineHandoffRecord {
        record_kind: OFFLINE_HANDOFF_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        handoff_id: input.handoff_id.clone(),
        handoff_state: input.handoff_state.clone(),
        freshness_class: input.freshness_class.clone(),
        actor_ref: input.actor_ref.clone(),
        return_anchor_ref: input.return_anchor_ref.clone(),
        review_pack_version: input.review_pack_version.clone(),
        divergence_labels: input.divergence_labels.clone(),
        replay_evidence_refs: input.replay_evidence_refs.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn stabilization_command_record(
    input: &ReviewStabilizationCommandInput,
    stabilization: &ReviewStabilizationRecord,
) -> ReviewStabilizationCommandRecord {
    ReviewStabilizationCommandRecord {
        record_kind: REVIEW_STABILIZATION_COMMAND_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        stabilization_id_ref: stabilization.stabilization_id.clone(),
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

fn stabilization_support_export_packet(
    input: &ReviewStabilizationSupportExportInput,
    stabilization: &ReviewStabilizationRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    landing_packet: &LandingCandidatePacket,
    commands: &[ReviewStabilizationCommandRecord],
) -> ReviewStabilizationSupportExportPacket {
    ReviewStabilizationSupportExportPacket {
        record_kind: REVIEW_STABILIZATION_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        landing_candidate_id_ref: landing_packet.landing_candidate.landing_candidate_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/review_stabilization.schema.json".to_string(),
            "schemas/review/review_workspace.schema.json".to_string(),
            "schemas/review/landing_candidate.schema.json".to_string(),
            "schemas/review/anchor_id_alpha.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn stabilization_inspection_record(
    stabilization: &ReviewStabilizationRecord,
    anchor_stabilities: &[ReviewAnchorStabilityRecord],
    stale_base_label: &StaleBaseLabelRecord,
    approval_invalidation: Option<&ApprovalInvalidationRecord>,
    mergeability_truth: &MergeabilityTruthRecord,
    ownership_signals: &[OwnershipSignalRecord],
    bundle_export: Option<&ReviewBundleExportRecord>,
    bundle_import: Option<&ReviewBundleImportRecord>,
    offline_handoff: Option<&OfflineHandoffRecord>,
    commands: &[ReviewStabilizationCommandRecord],
    support_export: &ReviewStabilizationSupportExportPacket,
) -> ReviewStabilizationInspectionRecord {
    let stabilized_current = stabilization.stabilization_state == "stabilized_current";
    let stabilized_stale_pack = stabilization.stabilization_state == "stabilized_stale_pack";
    let stabilized_partial_scope = stabilization.stabilization_state == "stabilized_partial_scope";
    let stabilized_slice_omitted =
        stabilization.stabilization_state == "stabilized_slice_omitted";
    let stabilized_diverged_requires_review =
        stabilization.stabilization_state == "stabilized_diverged_requires_review";
    let all_anchors_bound_exact = anchor_stabilities
        .iter()
        .all(|a| a.anchor_stability_class == "anchor_bound_exact");
    let any_anchor_drifted = anchor_stabilities
        .iter()
        .any(|a| a.anchor_stability_class == "anchor_drifted_requires_review");
    let stale_base_blocks_landing = stale_base_label.blocks_landing;
    let approval_invalidated = approval_invalidation.is_some();
    let mergeability_blocking = mergeability_truth.mergeability_truth_class == "not_mergeable_blocking";
    let mergeability_provider_authoritative = mergeability_truth.provider_authoritative;
    let enforceable_ownership_present = ownership_signals.iter().any(|s| s.enforceable);
    let advisory_ownership_present = ownership_signals.iter().any(|s| s.advisory);
    let ownership_conflict_present = ownership_signals.iter().any(|s| {
        s.ownership_signal_class == "advisory_and_enforceable_conflict"
    });
    let bundle_export_present = bundle_export.is_some();
    let bundle_import_present = bundle_import.is_some();
    let offline_handoff_present = offline_handoff.is_some();
    let actionable = stabilization.actionable;
    let invalidated = !stabilization.invalidation_reasons.is_empty();
    let preview_capable = commands.iter().any(|c| c.preview_supported);
    let support_export_reopenable = support_export_can_reopen(support_export, commands);

    ReviewStabilizationInspectionRecord {
        record_kind: REVIEW_STABILIZATION_INSPECTION_RECORD_KIND.to_string(),
        schema_version: REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        review_workspace_id_ref: stabilization.review_workspace_id_ref.clone(),
        stabilized_current,
        stabilized_stale_pack,
        stabilized_partial_scope,
        stabilized_slice_omitted,
        stabilized_diverged_requires_review,
        all_anchors_bound_exact,
        any_anchor_drifted,
        stale_base_blocks_landing,
        approval_invalidated,
        mergeability_blocking,
        mergeability_provider_authoritative,
        enforceable_ownership_present,
        advisory_ownership_present,
        ownership_conflict_present,
        bundle_export_present,
        bundle_import_present,
        offline_handoff_present,
        actionable,
        invalidated,
        command_count: commands.len(),
        anchor_stability_count: anchor_stabilities.len(),
        ownership_signal_count: ownership_signals.len(),
        preview_capable,
        support_export_reopenable,
        summary_label: format!(
            "Review stabilization {} ({} command(s), {} anchor(s))",
            stabilization.stabilization_id,
            commands.len(),
            anchor_stabilities.len()
        ),
    }
}

fn support_export_can_reopen(
    export: &ReviewStabilizationSupportExportPacket,
    commands: &[ReviewStabilizationCommandRecord],
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

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_input(
    input: &ReviewStabilizationInput,
    _workspace_packet: &ReviewWorkspaceBetaPacket,
    landing_packet: &LandingCandidatePacket,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_nonempty(&input.stabilization_id, "stabilization_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.review_pack_digest_ref, "review_pack_digest_ref")?;
    ensure_nonempty(&input.base_revision_ref, "base_revision_ref")?;
    ensure_nonempty(&input.head_revision_ref, "head_revision_ref")?;
    ensure_token(
        STABILIZATION_STATES,
        &input.stabilization_state,
        "stabilization_state",
    )?;

    if input.base_revision_ref != landing_packet.landing_candidate.base_revision_ref {
        return Err(stabilization_validation_error(
            "stabilization base_revision_ref must match landing candidate base_revision_ref",
        ));
    }
    if input.head_revision_ref != landing_packet.landing_candidate.head_revision_ref {
        return Err(stabilization_validation_error(
            "stabilization head_revision_ref must match landing candidate head_revision_ref",
        ));
    }
    if input.review_pack_digest_ref != landing_packet.landing_candidate.review_pack_digest_ref {
        return Err(stabilization_validation_error(
            "stabilization review_pack_digest_ref must match landing candidate review_pack_digest_ref",
        ));
    }

    for anchor in &input.anchor_stabilities {
        ensure_nonempty(&anchor.anchor_id_ref, "anchor_id_ref")?;
        ensure_token(
            ANCHOR_STABILITY_CLASSES,
            &anchor.anchor_stability_class,
            "anchor_stability_class",
        )?;
        ensure_nonempty(
            &anchor.bound_review_pack_digest_ref,
            "bound_review_pack_digest_ref",
        )?;
    }

    ensure_token(
        STALE_BASE_LABEL_CLASSES,
        &input.stale_base_label.stale_base_label_class,
        "stale_base_label_class",
    )?;

    if let Some(invalidation) = &input.approval_invalidation {
        ensure_token(
            APPROVAL_INVALIDATION_TRIGGER_CLASSES,
            &invalidation.invalidation_trigger_class,
            "invalidation_trigger_class",
        )?;
        ensure_nonempty(&invalidation.invalidated_at, "invalidated_at")?;
        if invalidation.replay_evidence_refs.len() != invalidation.replay_evidence_classes.len() {
            return Err(stabilization_validation_error(
                "replay_evidence_refs and replay_evidence_classes must have the same length",
            ));
        }
        for class in &invalidation.replay_evidence_classes {
            ensure_token(REPLAY_EVIDENCE_CLASSES, class, "replay_evidence_class")?;
        }
    }

    ensure_token(
        MERGEABILITY_TRUTH_CLASSES,
        &input.mergeability_truth.mergeability_truth_class,
        "mergeability_truth_class",
    )?;

    for signal in &input.ownership_signals {
        ensure_token(
            OWNERSHIP_SIGNAL_CLASSES,
            &signal.ownership_signal_class,
            "ownership_signal_class",
        )?;
        if !signal.advisory && !signal.enforceable {
            return Err(stabilization_validation_error(
                "ownership signal must be advisory or enforceable (or both when match/conflict)",
            ));
        }
    }

    if let Some(export) = &input.bundle_export {
        ensure_token(BUNDLE_EXPORT_STATES, &export.export_state, "export_state")?;
    }
    if let Some(import) = &input.bundle_import {
        ensure_token(BUNDLE_IMPORT_STATES, &import.import_state, "import_state")?;
    }
    if let Some(handoff) = &input.offline_handoff {
        ensure_token(OFFLINE_HANDOFF_STATES, &handoff.handoff_state, "handoff_state")?;
    }

    for command in &input.commands {
        ensure_token(
            REVIEW_STABILIZATION_COMMAND_CLASSES,
            &command.command_class,
            "command_class",
        )?;
    }

    ensure_nonempty(&input.support_export.support_export_id, "support_export_id")?;
    Ok(())
}

fn validate_stabilization_record(
    record: &ReviewStabilizationRecord,
    review_workspace_id: &str,
    landing_candidate_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        REVIEW_STABILIZATION_RECORD_KIND,
        "stabilization record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        REVIEW_STABILIZATION_SCHEMA_VERSION,
        "stabilization schema_version",
    )?;
    ensure_nonempty(&record.stabilization_id, "stabilization_id")?;
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        review_workspace_id,
        "stabilization review_workspace_id_ref",
    )?;
    ensure_eq(
        record.landing_candidate_id_ref.as_str(),
        landing_candidate_id,
        "stabilization landing_candidate_id_ref",
    )?;
    ensure_token(
        STABILIZATION_STATES,
        &record.stabilization_state,
        "stabilization_state",
    )?;
    Ok(())
}

fn validate_anchor_stability_record(
    record: &ReviewAnchorStabilityRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        REVIEW_ANCHOR_STABILITY_RECORD_KIND,
        "anchor_stability record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "anchor_stability stabilization_id_ref",
    )?;
    ensure_token(
        ANCHOR_STABILITY_CLASSES,
        &record.anchor_stability_class,
        "anchor_stability_class",
    )?;
    Ok(())
}

fn validate_stale_base_label_record(
    record: &StaleBaseLabelRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STALE_BASE_LABEL_RECORD_KIND,
        "stale_base_label record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "stale_base_label stabilization_id_ref",
    )?;
    ensure_token(
        STALE_BASE_LABEL_CLASSES,
        &record.stale_base_label_class,
        "stale_base_label_class",
    )?;
    Ok(())
}

fn validate_approval_invalidation_record(
    record: &ApprovalInvalidationRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        APPROVAL_INVALIDATION_RECORD_KIND,
        "approval_invalidation record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "approval_invalidation stabilization_id_ref",
    )?;
    ensure_token(
        APPROVAL_INVALIDATION_TRIGGER_CLASSES,
        &record.invalidation_trigger_class,
        "invalidation_trigger_class",
    )?;
    Ok(())
}

fn validate_mergeability_truth_record(
    record: &MergeabilityTruthRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        MERGEABILITY_TRUTH_RECORD_KIND,
        "mergeability_truth record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "mergeability_truth stabilization_id_ref",
    )?;
    ensure_token(
        MERGEABILITY_TRUTH_CLASSES,
        &record.mergeability_truth_class,
        "mergeability_truth_class",
    )?;
    Ok(())
}

fn validate_ownership_signal_record(
    record: &OwnershipSignalRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        OWNERSHIP_SIGNAL_RECORD_KIND,
        "ownership_signal record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "ownership_signal stabilization_id_ref",
    )?;
    ensure_token(
        OWNERSHIP_SIGNAL_CLASSES,
        &record.ownership_signal_class,
        "ownership_signal_class",
    )?;
    Ok(())
}

fn validate_bundle_export_record(
    record: &ReviewBundleExportRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        REVIEW_BUNDLE_EXPORT_RECORD_KIND,
        "bundle_export record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "bundle_export stabilization_id_ref",
    )?;
    ensure_token(BUNDLE_EXPORT_STATES, &record.export_state, "export_state")?;
    Ok(())
}

fn validate_bundle_import_record(
    record: &ReviewBundleImportRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        REVIEW_BUNDLE_IMPORT_RECORD_KIND,
        "bundle_import record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "bundle_import stabilization_id_ref",
    )?;
    ensure_token(BUNDLE_IMPORT_STATES, &record.import_state, "import_state")?;
    Ok(())
}

fn validate_offline_handoff_record(
    record: &OfflineHandoffRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        OFFLINE_HANDOFF_RECORD_KIND,
        "offline_handoff record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "offline_handoff stabilization_id_ref",
    )?;
    ensure_token(OFFLINE_HANDOFF_STATES, &record.handoff_state, "handoff_state")?;
    Ok(())
}

fn validate_command_record(
    record: &ReviewStabilizationCommandRecord,
    stabilization_id: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        REVIEW_STABILIZATION_COMMAND_RECORD_KIND,
        "command record_kind",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "command stabilization_id_ref",
    )?;
    ensure_token(
        REVIEW_STABILIZATION_COMMAND_CLASSES,
        &record.command_class,
        "command_class",
    )?;
    Ok(())
}

fn validate_support_export(
    export: &ReviewStabilizationSupportExportPacket,
    stabilization: &ReviewStabilizationRecord,
    commands: &[ReviewStabilizationCommandRecord],
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        REVIEW_STABILIZATION_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export record_kind",
    )?;
    ensure_eq(
        export.stabilization_id_ref.as_str(),
        stabilization.stabilization_id.as_str(),
        "support_export stabilization_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        stabilization.review_workspace_id_ref.as_str(),
        "support_export review_workspace_id_ref",
    )?;
    ensure_eq(
        export.landing_candidate_id_ref.as_str(),
        stabilization.landing_candidate_id_ref.as_str(),
        "support_export landing_candidate_id_ref",
    )?;
    if export.raw_url_export_allowed {
        return Err(stabilization_validation_error(
            "support_export raw_url_export_allowed must be false",
        ));
    }
    if export.raw_provider_payload_export_allowed {
        return Err(stabilization_validation_error(
            "support_export raw_provider_payload_export_allowed must be false",
        ));
    }
    if export.command_id_refs.len() != commands.len() {
        return Err(stabilization_validation_error(
            "support_export command_id_refs length must match commands length",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &ReviewStabilizationInspectionRecord,
    packet: &ReviewStabilizationPacket,
) -> Result<(), ReviewStabilizationValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        REVIEW_STABILIZATION_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.stabilization_id_ref.as_str(),
        packet.stabilization.stabilization_id.as_str(),
        "inspection stabilization_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.review_workspace.review_workspace_id.as_str(),
        "inspection review_workspace_id_ref",
    )?;
    if inspection.command_count != packet.commands.len() {
        return Err(stabilization_validation_error(
            "inspection command_count must match commands length",
        ));
    }
    if inspection.anchor_stability_count != packet.anchor_stabilities.len() {
        return Err(stabilization_validation_error(
            "inspection anchor_stability_count must match anchor_stabilities length",
        ));
    }
    if inspection.ownership_signal_count != packet.ownership_signals.len() {
        return Err(stabilization_validation_error(
            "inspection ownership_signal_count must match ownership_signals length",
        ));
    }
    if inspection.stale_base_blocks_landing != packet.stale_base_label.blocks_landing {
        return Err(stabilization_validation_error(
            "inspection stale_base_blocks_landing must match stale_base_label.blocks_landing",
        ));
    }
    if inspection.approval_invalidated != packet.approval_invalidation.is_some() {
        return Err(stabilization_validation_error(
            "inspection approval_invalidated must match presence of approval_invalidation",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Derivation helpers
// ---------------------------------------------------------------------------

fn derive_invalidation_reasons(
    stale_base_label_class: &str,
    checks_freshness_state: &str,
    approval_invalidation: Option<&ApprovalInvalidationInput>,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if stale_base_label_class == "base_stale_blocks_landing" {
        reasons.push("stale_base".to_string());
    }
    if checks_freshness_state == "checks_stale_blocks_landing" {
        reasons.push("checks_stale".to_string());
    }
    if approval_invalidation.is_some() {
        reasons.push("approval_invalidated".to_string());
    }
    reasons
}

fn derive_blocked_reasons(
    stabilization_state: &str,
    stale_base_label_class: &str,
    mergeability_truth_class: &str,
    approval_invalidation: Option<&ApprovalInvalidationInput>,
    invalidation_reasons: &[String],
) -> Vec<String> {
    let mut reasons = Vec::new();
    if stabilization_state == "stabilized_stale_pack" {
        reasons.push("review_pack_version_drift".to_string());
    }
    if stabilization_state == "stabilized_partial_scope" {
        reasons.push("worktree_scope_changed".to_string());
    }
    if stabilization_state == "stabilized_slice_omitted" {
        reasons.push("partial_scope_omission".to_string());
    }
    if stale_base_label_class == "base_stale_blocks_landing" {
        reasons.push("base_revision_stale".to_string());
    }
    if mergeability_truth_class == "not_mergeable_blocking" {
        reasons.push("mergeability_blocking".to_string());
    }
    if approval_invalidation.is_some() {
        reasons.push("approval_invalidated".to_string());
    }
    for reason in invalidation_reasons {
        if reason == "policy_blocked" {
            reasons.push("policy_blocked".to_string());
        }
    }
    reasons.sort();
    reasons.dedup();
    reasons
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn stabilization_validation_error(message: impl Into<String>) -> ReviewStabilizationValidationError {
    ReviewStabilizationValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), ReviewStabilizationValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(stabilization_validation_error(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(
    value: &str,
    field: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    if value.trim().is_empty() {
        return Err(stabilization_validation_error(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), ReviewStabilizationValidationError> {
    if !tokens.contains(&value) {
        return Err(stabilization_validation_error(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}

fn contains_token(tokens: &[&str], value: &str) -> bool {
    tokens.contains(&value)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_are_nonempty() {
        assert!(!STABILIZATION_STATES.is_empty());
        assert!(!ANCHOR_STABILITY_CLASSES.is_empty());
        assert!(!STALE_BASE_LABEL_CLASSES.is_empty());
        assert!(!APPROVAL_INVALIDATION_TRIGGER_CLASSES.is_empty());
        assert!(!MERGEABILITY_TRUTH_CLASSES.is_empty());
        assert!(!OWNERSHIP_SIGNAL_CLASSES.is_empty());
        assert!(!BUNDLE_EXPORT_STATES.is_empty());
        assert!(!BUNDLE_IMPORT_STATES.is_empty());
        assert!(!OFFLINE_HANDOFF_STATES.is_empty());
        assert!(!REPLAY_EVIDENCE_CLASSES.is_empty());
        assert!(!DIVERGENCE_LABEL_CLASSES.is_empty());
        assert!(!REVIEW_STABILIZATION_COMMAND_CLASSES.is_empty());
        assert!(!REVIEW_STABILIZATION_CONSUMER_SURFACES.is_empty());
        assert!(!REVIEW_STABILIZATION_INVALIDATION_REASONS.is_empty());
    }
}
