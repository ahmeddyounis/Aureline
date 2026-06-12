//! Finalized work-item linkage with branch/review/status traceability and
//! publish-later continuity.
//!
//! This module consumes provider work-item detail, transition, and offline-handoff
//! records and projects them into the review-workspace stabilization context. It
//! produces a governed work-item detail surface with explicit provider-owned IDs,
//! freshness disclosure, and a visible distinction between provider-authoritative
//! state, local-draft state, sync-pending state, and offline-captured state.
//!
//! Status-transition sheets preview exact provider side effects before commit,
//! including comment creation, status or assignee mutation, label changes, and
//! branch/review link creation or updates. Users can save a local draft note or
//! offline handoff packet instead of publishing immediately.
//!
//! Offline handoff packets are first-class: they preserve selected evidence,
//! redaction choices, destination/provider target, queued actions, and
//! publish-later continuity across restart, reconnect, and export/import flows.
//!
//! The record family includes:
//!
//! - [`WorkItemLinkageFinalizationRecord`] — stable identity binding workspace,
//!   stabilization, and linkage finalization state.
//! - [`WorkItemDetailSurfaceRecord`] — governed work-item detail surface with
//!   canonical provider-owned ID, explicit freshness, and write-mode disclosure.
//! - [`StatusTransitionSheetRecord`] — status-transition sheet that previews
//!   exact provider side effects before commit.
//! - [`OfflineHandoffContinuityRecord`] — offline-handoff continuity record that
//!   preserves publish-later continuity across restart, reconnect, and export.
//! - [`WorkItemBranchLinkRecord`] — explicit branch-to-work-item link with
//!   traceability and preview-before-publish semantics.
//! - [`WorkItemReviewLinkRecord`] — explicit review-to-work-item link with
//!   traceability and preview-before-publish semantics.
//! - [`PublishLaterContinuityRecord`] — publish-later queue continuity record.
//! - [`WorkItemLinkageCommandRecord`] — command-graph operations surfaced to the
//!   inspector.
//! - [`WorkItemLinkageSupportExportPacket`] — redaction-safe support export.
//! - [`WorkItemLinkageInspectionRecord`] — compact boolean projection for CLI
//!   and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/finalize_issue_and_work_item_linkage_with_branch.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/finalize-issue-and-work-item-linkage-with-branch/`.

use std::collections::BTreeSet;
use std::fmt;

use aureline_provider::WorkItemObjectRowRecord;
use serde::{Deserialize, Serialize};

use crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationPacket;
use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every work-item linkage finalization record.
pub const WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`WorkItemLinkageFinalizationPacket`].
pub const WORK_ITEM_LINKAGE_FINALIZATION_PACKET_RECORD_KIND: &str =
    "review_work_item_linkage_finalization_packet";

/// Stable record-kind tag for [`WorkItemLinkageFinalizationRecord`].
pub const WORK_ITEM_LINKAGE_FINALIZATION_RECORD_KIND: &str =
    "review_work_item_linkage_finalization_record";

/// Stable record-kind tag for [`WorkItemDetailSurfaceRecord`].
pub const WORK_ITEM_DETAIL_SURFACE_RECORD_KIND: &str = "review_work_item_detail_surface_record";

/// Stable record-kind tag for [`StatusTransitionSheetRecord`].
pub const STATUS_TRANSITION_SHEET_RECORD_KIND: &str = "review_status_transition_sheet_record";

/// Stable record-kind tag for [`OfflineHandoffContinuityRecord`].
pub const OFFLINE_HANDOFF_CONTINUITY_RECORD_KIND: &str = "review_offline_handoff_continuity_record";

/// Stable record-kind tag for [`WorkItemBranchLinkRecord`].
pub const WORK_ITEM_BRANCH_LINK_RECORD_KIND: &str = "review_branch_work_item_link_record";

/// Stable record-kind tag for [`WorkItemReviewLinkRecord`].
pub const WORK_ITEM_REVIEW_LINK_RECORD_KIND: &str = "review_review_work_item_link_record";

/// Stable record-kind tag for [`PublishLaterContinuityRecord`].
pub const PUBLISH_LATER_CONTINUITY_RECORD_KIND: &str = "review_publish_later_continuity_record";

/// Stable record-kind tag for [`WorkItemLinkageCommandRecord`].
pub const WORK_ITEM_LINKAGE_COMMAND_RECORD_KIND: &str = "review_work_item_linkage_command_record";

/// Stable record-kind tag for [`WorkItemLinkageSupportExportPacket`].
pub const WORK_ITEM_LINKAGE_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_work_item_linkage_support_export_packet";

/// Stable record-kind tag for [`WorkItemLinkageInspectionRecord`].
pub const WORK_ITEM_LINKAGE_INSPECTION_RECORD_KIND: &str =
    "review_work_item_linkage_inspection_record";

/// Closed set of finalization states.
pub const FINALIZATION_STATES: &[&str] = &[
    "finalized_current",
    "finalized_stale_provider_overlay",
    "finalized_partial_work_item_scope",
    "finalized_diverged_requires_review",
    "finalized_offline_handoff_only",
];

/// Closed set of write-mode disclosure classes.
pub const WRITE_MODE_DISCLOSURE_CLASSES: &[&str] = &[
    "read_only",
    "comment_or_link",
    "full_edit",
    "offline_capture_only",
    "policy_blocked",
];

/// Closed set of linkage command classes.
pub const WORK_ITEM_LINKAGE_COMMAND_CLASSES: &[&str] = &[
    "preview_linkage",
    "confirm_status_transition",
    "save_local_draft",
    "queue_publish_later",
    "export_offline_handoff",
    "refresh_provider_overlay",
    "invalidate_linkage",
];

/// Closed set of consumer surfaces for linkage packets.
pub const WORK_ITEM_LINKAGE_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "review_landing_strip",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
    "offline_handoff",
];

/// Closed set of invalidation reasons.
pub const WORK_ITEM_LINKAGE_INVALIDATION_REASONS: &[&str] = &[
    "stale_provider_overlay",
    "work_item_scope_changed",
    "branch_link_drifted",
    "review_link_drifted",
    "status_transition_stale",
    "offline_handoff_superseded",
    "publish_later_queue_changed",
    "policy_blocked",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a work-item linkage finalization to materialize on top of a
/// beta review-workspace packet, a review-stabilization packet, and provider
/// work-item records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkageFinalizationInput {
    /// Stable finalization identity.
    pub linkage_finalization_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Finalization state from the closed vocabulary.
    pub finalization_state: String,
    /// Work-item detail surface inputs.
    pub detail_surfaces: Vec<WorkItemDetailSurfaceInput>,
    /// Status-transition sheet inputs.
    pub transition_sheets: Vec<StatusTransitionSheetInput>,
    /// Offline-handoff continuity inputs.
    pub offline_handoff_continuities: Vec<OfflineHandoffContinuityInput>,
    /// Branch-to-work-item link inputs.
    pub branch_links: Vec<WorkItemBranchLinkInput>,
    /// Review-to-work-item link inputs.
    pub review_links: Vec<WorkItemReviewLinkInput>,
    /// Publish-later continuity inputs.
    pub publish_later_continuities: Vec<PublishLaterContinuityInput>,
    /// Command-graph operations.
    pub commands: Vec<WorkItemLinkageCommandInput>,
    /// Support/export envelope input.
    pub support_export: WorkItemLinkageSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing one governed work-item detail surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemDetailSurfaceInput {
    /// Stable detail surface identity.
    pub detail_surface_id: String,
    /// Ref to the provider work-item detail record.
    pub provider_work_item_detail_record_id_ref: String,
    /// Shared provider-owned work-item row vocabulary projected from the detail record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_row: Option<WorkItemObjectRowRecord>,
    /// Canonical provider-owned ID visible to the user.
    pub canonical_provider_id: String,
    /// Provider-owned opaque object ID.
    pub provider_owned_id: String,
    /// Write-mode disclosure class.
    pub write_mode_disclosure_class: String,
    /// Row posture class copied from provider.
    pub row_posture_class: String,
    /// Freshness class.
    pub freshness_class: String,
    /// Freshness observed timestamp.
    pub freshness_observed_at: String,
    /// True when provider-authoritative state is visible.
    pub provider_authoritative_state_visible: bool,
    /// True when local-draft state is visible.
    pub local_draft_state_visible: bool,
    /// True when sync-pending state is visible.
    pub sync_pending_state_visible: bool,
    /// True when offline-captured state is visible.
    pub offline_captured_state_visible: bool,
    /// Branch or worktree locator ref.
    pub branch_local_locator_ref: String,
    /// Review workspace record ref.
    pub review_workspace_record_id_ref: String,
    /// Validation evidence refs.
    pub validation_evidence_record_id_refs: Vec<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one status-transition sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusTransitionSheetInput {
    /// Stable transition sheet identity.
    pub transition_sheet_id: String,
    /// Ref to the provider status-transition packet.
    pub provider_status_transition_packet_record_id_ref: String,
    /// Ref to the provider transition-review sheet.
    pub provider_transition_review_record_id_ref: String,
    /// Previewed side effects.
    pub previewed_side_effects: Vec<PreviewedSideEffectInput>,
    /// Confirm action available.
    pub confirm_action_available: bool,
    /// Export action available.
    pub export_action_available: bool,
    /// Cancel action available.
    pub cancel_action_available: bool,
    /// Whether local draft is preserved on publish failure.
    pub local_draft_preserved_on_failure: bool,
    /// Optional publish-later continuity ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_continuity_ref: Option<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// One previewed side effect in a status-transition sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewedSideEffectInput {
    /// Side-effect id.
    pub side_effect_id: String,
    /// Side-effect class.
    pub side_effect_class: String,
    /// Target account or scope.
    pub target_scope: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one offline-handoff continuity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineHandoffContinuityInput {
    /// Stable offline-handoff continuity identity.
    pub offline_handoff_continuity_id: String,
    /// Ref to the provider offline-handoff packet.
    pub provider_offline_handoff_packet_record_id_ref: String,
    /// True when the packet survives restart.
    pub survives_restart: bool,
    /// True when the packet survives reconnect.
    pub survives_reconnect: bool,
    /// True when the packet survives export/import.
    pub survives_export_import: bool,
    /// Destination provider target.
    pub destination_provider_target: String,
    /// Queued actions preserved.
    pub queued_actions_preserved: bool,
    /// Redaction choices preserved.
    pub redaction_choices_preserved: bool,
    /// Selected evidence preserved.
    pub selected_evidence_preserved: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one branch-to-work-item link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemBranchLinkInput {
    /// Stable branch link identity.
    pub branch_link_id: String,
    /// Work-item detail surface ref.
    pub work_item_detail_surface_id_ref: String,
    /// Branch local locator ref.
    pub branch_local_locator_ref: String,
    /// Issue-to-branch link class.
    pub issue_to_branch_link_class: String,
    /// Link source class.
    pub link_source_class: String,
    /// Link freshness class.
    pub link_freshness_class: String,
    /// True when the link is previewable before publish.
    pub previewable_before_publish: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one review-to-work-item link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemReviewLinkInput {
    /// Stable review link identity.
    pub review_link_id: String,
    /// Work-item detail surface ref.
    pub work_item_detail_surface_id_ref: String,
    /// Review workspace record ref.
    pub review_workspace_record_id_ref: String,
    /// Linked review class.
    pub linked_review_class: String,
    /// Link source class.
    pub link_source_class: String,
    /// Link freshness class.
    pub link_freshness_class: String,
    /// True when the link is previewable before publish.
    pub previewable_before_publish: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one publish-later continuity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishLaterContinuityInput {
    /// Stable publish-later continuity identity.
    pub continuity_id: String,
    /// Publish-later queue item ref.
    pub publish_later_queue_item_record_id_ref: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Queue state.
    pub queue_state: String,
    /// Drain state.
    pub drain_state: String,
    /// Retry route class.
    pub retry_route_class: String,
    /// True when continuity survives restart.
    pub survives_restart: bool,
    /// True when continuity survives reconnect.
    pub survives_reconnect: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one command-graph operation for work-item linkage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkageCommandInput {
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
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input row for the work-item linkage support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkageSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the linkage finalization.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Work-item linkage finalization record materialized from input plus workspace
/// and stabilization truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkageFinalizationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable finalization identity.
    pub linkage_finalization_id: String,
    /// Review workspace this finalization belongs to.
    pub review_workspace_id_ref: String,
    /// Stabilization this finalization binds.
    pub stabilization_id_ref: String,
    /// Finalization state.
    pub finalization_state: String,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing finalization approval.
    pub blocked_reasons: Vec<String>,
    /// True when the finalization is actionable from the current state.
    pub actionable: bool,
    /// Timestamp the finalization was frozen.
    pub generated_at: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Governed work-item detail surface record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemDetailSurfaceRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable detail surface identity.
    pub detail_surface_id: String,
    /// Linkage finalization this surface belongs to.
    pub linkage_finalization_id_ref: String,
    /// Ref to the provider work-item detail record.
    pub provider_work_item_detail_record_id_ref: String,
    /// Shared provider-owned work-item row vocabulary projected from the detail record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_row: Option<WorkItemObjectRowRecord>,
    /// Canonical provider-owned ID visible to the user.
    pub canonical_provider_id: String,
    /// Provider-owned opaque object ID.
    pub provider_owned_id: String,
    /// Write-mode disclosure class.
    pub write_mode_disclosure_class: String,
    /// Row posture class.
    pub row_posture_class: String,
    /// Freshness class.
    pub freshness_class: String,
    /// Freshness observed timestamp.
    pub freshness_observed_at: String,
    /// True when provider-authoritative state is visible.
    pub provider_authoritative_state_visible: bool,
    /// True when local-draft state is visible.
    pub local_draft_state_visible: bool,
    /// True when sync-pending state is visible.
    pub sync_pending_state_visible: bool,
    /// True when offline-captured state is visible.
    pub offline_captured_state_visible: bool,
    /// Branch or worktree locator ref.
    pub branch_local_locator_ref: String,
    /// Review workspace record ref.
    pub review_workspace_record_id_ref: String,
    /// Validation evidence refs.
    pub validation_evidence_record_id_refs: Vec<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Status-transition sheet record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusTransitionSheetRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable transition sheet identity.
    pub transition_sheet_id: String,
    /// Linkage finalization this sheet belongs to.
    pub linkage_finalization_id_ref: String,
    /// Work-item detail surface ref.
    pub work_item_detail_surface_id_ref: String,
    /// Ref to the provider status-transition packet.
    pub provider_status_transition_packet_record_id_ref: String,
    /// Ref to the provider transition-review sheet.
    pub provider_transition_review_record_id_ref: String,
    /// Previewed side effects.
    pub previewed_side_effects: Vec<PreviewedSideEffectRecord>,
    /// Confirm action available.
    pub confirm_action_available: bool,
    /// Export action available.
    pub export_action_available: bool,
    /// Cancel action available.
    pub cancel_action_available: bool,
    /// Whether local draft is preserved on publish failure.
    pub local_draft_preserved_on_failure: bool,
    /// Optional publish-later continuity ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_continuity_ref: Option<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// One previewed side effect in a status-transition sheet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewedSideEffectRecord {
    /// Side-effect id.
    pub side_effect_id: String,
    /// Side-effect class.
    pub side_effect_class: String,
    /// Target account or scope.
    pub target_scope: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Offline-handoff continuity record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineHandoffContinuityRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable offline-handoff continuity identity.
    pub offline_handoff_continuity_id: String,
    /// Linkage finalization this continuity belongs to.
    pub linkage_finalization_id_ref: String,
    /// Ref to the provider offline-handoff packet.
    pub provider_offline_handoff_packet_record_id_ref: String,
    /// True when the packet survives restart.
    pub survives_restart: bool,
    /// True when the packet survives reconnect.
    pub survives_reconnect: bool,
    /// True when the packet survives export/import.
    pub survives_export_import: bool,
    /// Destination provider target.
    pub destination_provider_target: String,
    /// Queued actions preserved.
    pub queued_actions_preserved: bool,
    /// Redaction choices preserved.
    pub redaction_choices_preserved: bool,
    /// Selected evidence preserved.
    pub selected_evidence_preserved: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Branch-to-work-item link record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemBranchLinkRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable branch link identity.
    pub branch_link_id: String,
    /// Linkage finalization this link belongs to.
    pub linkage_finalization_id_ref: String,
    /// Work-item detail surface ref.
    pub work_item_detail_surface_id_ref: String,
    /// Branch local locator ref.
    pub branch_local_locator_ref: String,
    /// Issue-to-branch link class.
    pub issue_to_branch_link_class: String,
    /// Link source class.
    pub link_source_class: String,
    /// Link freshness class.
    pub link_freshness_class: String,
    /// True when the link is previewable before publish.
    pub previewable_before_publish: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Review-to-work-item link record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemReviewLinkRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable review link identity.
    pub review_link_id: String,
    /// Linkage finalization this link belongs to.
    pub linkage_finalization_id_ref: String,
    /// Work-item detail surface ref.
    pub work_item_detail_surface_id_ref: String,
    /// Review workspace record ref.
    pub review_workspace_record_id_ref: String,
    /// Linked review class.
    pub linked_review_class: String,
    /// Link source class.
    pub link_source_class: String,
    /// Link freshness class.
    pub link_freshness_class: String,
    /// True when the link is previewable before publish.
    pub previewable_before_publish: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Publish-later continuity record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishLaterContinuityRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable publish-later continuity identity.
    pub continuity_id: String,
    /// Linkage finalization this continuity belongs to.
    pub linkage_finalization_id_ref: String,
    /// Publish-later queue item ref.
    pub publish_later_queue_item_record_id_ref: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Queue state.
    pub queue_state: String,
    /// Drain state.
    pub drain_state: String,
    /// Retry route class.
    pub retry_route_class: String,
    /// True when continuity survives restart.
    pub survives_restart: bool,
    /// True when continuity survives reconnect.
    pub survives_reconnect: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Command-graph operation record for work-item linkage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkageCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Linkage finalization this command belongs to.
    pub linkage_finalization_id_ref: String,
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
    /// True when the command is actionable from the current finalization state.
    pub actionable: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Support/export packet for the work-item linkage lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkageSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Linkage finalization this packet exports.
    pub linkage_finalization_id_ref: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Stabilization this packet exports.
    pub stabilization_id_ref: String,
    /// Stable context ref used to reopen the finalization.
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
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Inspection row used by support/export and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkageInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Linkage finalization inspected by this row.
    pub linkage_finalization_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the finalization is current.
    pub finalized_current: bool,
    /// True when the finalization is stale due to provider overlay.
    pub finalized_stale_provider_overlay: bool,
    /// True when the finalization is partial scope.
    pub finalized_partial_work_item_scope: bool,
    /// True when the finalization is diverged and requires review.
    pub finalized_diverged_requires_review: bool,
    /// True when the finalization is offline-handoff only.
    pub finalized_offline_handoff_only: bool,
    /// True when at least one detail surface is provider authoritative.
    pub provider_authoritative_surface_present: bool,
    /// True when at least one detail surface is local draft.
    pub local_draft_surface_present: bool,
    /// True when at least one detail surface is sync pending.
    pub sync_pending_surface_present: bool,
    /// True when at least one detail surface is offline captured.
    pub offline_captured_surface_present: bool,
    /// True when every detail surface discloses its write mode.
    pub write_mode_disclosed_on_all_surfaces: bool,
    /// True when at least one transition sheet is present.
    pub transition_sheet_present: bool,
    /// True when at least one transition sheet preserves local draft on failure.
    pub local_draft_preserved_on_failure: bool,
    /// True when at least one offline-handoff continuity record survives restart.
    pub offline_handoff_survives_restart: bool,
    /// True when at least one offline-handoff continuity record survives reconnect.
    pub offline_handoff_survives_reconnect: bool,
    /// True when at least one branch link is previewable before publish.
    pub branch_link_previewable: bool,
    /// True when at least one review link is previewable before publish.
    pub review_link_previewable: bool,
    /// True when at least one publish-later continuity record survives restart.
    pub publish_later_survives_restart: bool,
    /// True when at least one publish-later continuity record survives reconnect.
    pub publish_later_survives_reconnect: bool,
    /// True when the finalization is actionable.
    pub actionable: bool,
    /// True when the finalization is invalidated by any reason.
    pub invalidated: bool,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// Number of detail surface records.
    pub detail_surface_count: usize,
    /// Number of transition sheet records.
    pub transition_sheet_count: usize,
    /// Number of offline-handoff continuity records.
    pub offline_handoff_continuity_count: usize,
    /// Number of branch link records.
    pub branch_link_count: usize,
    /// Number of review link records.
    pub review_link_count: usize,
    /// Number of publish-later continuity records.
    pub publish_later_continuity_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the finalization context.
    pub support_export_reopenable: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Work-item linkage finalization packet consumed by review surfaces and support
/// exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkageFinalizationPacket {
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
    /// Stabilization summary copied from the stabilization packet.
    pub stabilization: crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationRecord,
    /// Work-item linkage finalization record.
    pub linkage_finalization: WorkItemLinkageFinalizationRecord,
    /// Work-item detail surface records.
    pub detail_surfaces: Vec<WorkItemDetailSurfaceRecord>,
    /// Status-transition sheet records.
    pub transition_sheets: Vec<StatusTransitionSheetRecord>,
    /// Offline-handoff continuity records.
    pub offline_handoff_continuities: Vec<OfflineHandoffContinuityRecord>,
    /// Branch-to-work-item link records.
    pub branch_links: Vec<WorkItemBranchLinkRecord>,
    /// Review-to-work-item link records.
    pub review_links: Vec<WorkItemReviewLinkRecord>,
    /// Publish-later continuity records.
    pub publish_later_continuities: Vec<PublishLaterContinuityRecord>,
    /// Command-graph operation records.
    pub commands: Vec<WorkItemLinkageCommandRecord>,
    /// Support/export packet.
    pub support_export: WorkItemLinkageSupportExportPacket,
    /// Inspection row.
    pub inspection: WorkItemLinkageInspectionRecord,
}

impl WorkItemLinkageFinalizationPacket {
    /// Builds a work-item linkage finalization packet from a beta review-workspace
    /// packet, a review-stabilization packet, and linkage finalization input.
    ///
    /// # Errors
    ///
    /// Returns [`WorkItemLinkageFinalizationValidationError`] when the input
    /// violates a linkage finalization invariant.
    pub fn from_workspace_and_stabilization_packets(
        input: WorkItemLinkageFinalizationInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
        stabilization_packet: &ReviewStabilizationPacket,
    ) -> Result<Self, WorkItemLinkageFinalizationValidationError> {
        validate_input(&input, workspace_packet, stabilization_packet)?;

        let linkage_finalization =
            linkage_finalization_record(&input, workspace_packet, stabilization_packet);
        let detail_surfaces = input
            .detail_surfaces
            .iter()
            .map(|d| detail_surface_record(d, &linkage_finalization))
            .collect::<Vec<_>>();
        let transition_sheets = input
            .transition_sheets
            .iter()
            .map(|t| transition_sheet_record(t, &linkage_finalization))
            .collect::<Vec<_>>();
        let offline_handoff_continuities = input
            .offline_handoff_continuities
            .iter()
            .map(|o| offline_handoff_continuity_record(o, &linkage_finalization))
            .collect::<Vec<_>>();
        let branch_links = input
            .branch_links
            .iter()
            .map(|b| branch_link_record(b, &linkage_finalization))
            .collect::<Vec<_>>();
        let review_links = input
            .review_links
            .iter()
            .map(|r| review_link_record(r, &linkage_finalization))
            .collect::<Vec<_>>();
        let publish_later_continuities = input
            .publish_later_continuities
            .iter()
            .map(|p| publish_later_continuity_record(p, &linkage_finalization))
            .collect::<Vec<_>>();
        let commands = input
            .commands
            .iter()
            .map(|command| linkage_command_record(command, &linkage_finalization))
            .collect::<Vec<_>>();
        let support_export = linkage_support_export_packet(
            &input.support_export,
            &linkage_finalization,
            workspace_packet,
            stabilization_packet,
            &commands,
        );
        let inspection = linkage_inspection_record(
            &linkage_finalization,
            &detail_surfaces,
            &transition_sheets,
            &offline_handoff_continuities,
            &branch_links,
            &review_links,
            &publish_later_continuities,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: WORK_ITEM_LINKAGE_FINALIZATION_PACKET_RECORD_KIND.to_string(),
            schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            stabilization: stabilization_packet.stabilization.clone(),
            linkage_finalization,
            detail_surfaces,
            transition_sheets,
            offline_handoff_continuities,
            branch_links,
            review_links,
            publish_later_continuities,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the work-item linkage finalization invariants.
    ///
    /// # Errors
    ///
    /// Returns [`WorkItemLinkageFinalizationValidationError`] when an invariant
    /// is violated.
    pub fn validate(&self) -> Result<(), WorkItemLinkageFinalizationValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            WORK_ITEM_LINKAGE_FINALIZATION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq(
            self.schema_version,
            WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_linkage_finalization_record(
            &self.linkage_finalization,
            &self.review_workspace.review_workspace_id,
            &self.stabilization.stabilization_id,
        )?;
        for surface in &self.detail_surfaces {
            validate_detail_surface_record(
                surface,
                &self.linkage_finalization.linkage_finalization_id,
            )?;
        }
        for sheet in &self.transition_sheets {
            validate_transition_sheet_record(
                sheet,
                &self.linkage_finalization.linkage_finalization_id,
            )?;
        }
        for offline in &self.offline_handoff_continuities {
            validate_offline_handoff_continuity_record(
                offline,
                &self.linkage_finalization.linkage_finalization_id,
            )?;
        }
        for link in &self.branch_links {
            validate_branch_link_record(link, &self.linkage_finalization.linkage_finalization_id)?;
        }
        for link in &self.review_links {
            validate_review_link_record(link, &self.linkage_finalization.linkage_finalization_id)?;
        }
        for continuity in &self.publish_later_continuities {
            validate_publish_later_continuity_record(
                continuity,
                &self.linkage_finalization.linkage_finalization_id,
            )?;
        }
        for command in &self.commands {
            validate_command_record(command, &self.linkage_finalization.linkage_finalization_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.linkage_finalization,
            &self.commands,
        )?;
        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when every detail surface discloses its write mode.
    pub fn write_modes_disclosed(&self) -> bool {
        self.detail_surfaces.iter().all(|surface| {
            contains_token(
                WRITE_MODE_DISCLOSURE_CLASSES,
                &surface.write_mode_disclosure_class,
            )
        })
    }

    /// Returns true when no raw escape hatch crosses the support boundary.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_url_export_allowed
            && !self.support_export.raw_provider_payload_export_allowed
    }

    /// Returns true when provider-authoritative, local-draft, sync-pending, and
    /// offline-captured states are all visible where claimed.
    pub fn state_distinctions_visible(&self) -> bool {
        self.detail_surfaces.iter().all(|surface| {
            let expected_visible = match surface.row_posture_class.as_str() {
                "provider_authoritative" => surface.provider_authoritative_state_visible,
                "local_draft" => surface.local_draft_state_visible,
                "queued" => surface.sync_pending_state_visible,
                "offline_captured" => surface.offline_captured_state_visible,
                _ => true,
            };
            expected_visible
        })
    }

    /// Returns true when offline handoff packets survive restart and reconnect.
    pub fn offline_handoff_continuity_preserved(&self) -> bool {
        self.offline_handoff_continuities
            .iter()
            .all(|o| o.survives_restart && o.survives_reconnect && o.survives_export_import)
    }

    /// Returns true when branch names, review links, and status updates are
    /// previewable before publish.
    pub fn mutations_previewable_before_publish(&self) -> bool {
        self.branch_links
            .iter()
            .all(|b| b.previewable_before_publish)
            && self
                .review_links
                .iter()
                .all(|r| r.previewable_before_publish)
            && self
                .transition_sheets
                .iter()
                .all(|t| t.export_action_available && t.cancel_action_available)
    }
}

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemLinkageFinalizationProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Linkage finalization identity.
    pub linkage_finalization_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Stabilization identity.
    pub stabilization_id: String,
    /// Finalization state.
    pub finalization_state: String,
    /// True when provider-authoritative surface is present.
    pub provider_authoritative_surface_present: bool,
    /// True when local-draft surface is present.
    pub local_draft_surface_present: bool,
    /// True when sync-pending surface is present.
    pub sync_pending_surface_present: bool,
    /// True when offline-captured surface is present.
    pub offline_captured_surface_present: bool,
    /// True when write mode is disclosed on all surfaces.
    pub write_mode_disclosed_on_all_surfaces: bool,
    /// True when transition sheet is present.
    pub transition_sheet_present: bool,
    /// True when offline handoff survives restart.
    pub offline_handoff_survives_restart: bool,
    /// True when mutations are previewable before publish.
    pub mutations_previewable_before_publish: bool,
    /// True when publish-later continuity survives restart.
    pub publish_later_survives_restart: bool,
    /// True when publish-later continuity survives reconnect.
    pub publish_later_survives_reconnect: bool,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// Command count.
    pub command_count: usize,
    /// True when support/export can reopen the finalization context.
    pub support_export_reopenable: bool,
    /// Consumer surfaces wired through the support export.
    pub consumer_surfaces: Vec<String>,
}

/// Parses and validates a materialized work-item linkage finalization packet.
///
/// # Errors
///
/// Returns [`WorkItemLinkageFinalizationError`] when the payload fails to parse
/// or violates the linkage finalization invariants.
pub fn project_work_item_linkage_finalization_packet(
    payload: &str,
) -> Result<WorkItemLinkageFinalizationProjection, WorkItemLinkageFinalizationError> {
    let packet: WorkItemLinkageFinalizationPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(WorkItemLinkageFinalizationProjection::from(packet))
}

impl From<WorkItemLinkageFinalizationPacket> for WorkItemLinkageFinalizationProjection {
    fn from(packet: WorkItemLinkageFinalizationPacket) -> Self {
        Self {
            packet_id: packet.packet_id.clone(),
            linkage_finalization_id: packet.linkage_finalization.linkage_finalization_id.clone(),
            review_workspace_id: packet.review_workspace.review_workspace_id.clone(),
            stabilization_id: packet.stabilization.stabilization_id.clone(),
            finalization_state: packet.linkage_finalization.finalization_state.clone(),
            provider_authoritative_surface_present: packet
                .inspection
                .provider_authoritative_surface_present,
            local_draft_surface_present: packet.inspection.local_draft_surface_present,
            sync_pending_surface_present: packet.inspection.sync_pending_surface_present,
            offline_captured_surface_present: packet.inspection.offline_captured_surface_present,
            write_mode_disclosed_on_all_surfaces: packet
                .inspection
                .write_mode_disclosed_on_all_surfaces,
            transition_sheet_present: packet.inspection.transition_sheet_present,
            offline_handoff_survives_restart: packet.inspection.offline_handoff_survives_restart,
            mutations_previewable_before_publish: packet.mutations_previewable_before_publish(),
            publish_later_survives_restart: packet.inspection.publish_later_survives_restart,
            publish_later_survives_reconnect: packet.inspection.publish_later_survives_reconnect,
            invalidation_reasons: packet.linkage_finalization.invalidation_reasons.clone(),
            blocked_reasons: packet.linkage_finalization.blocked_reasons.clone(),
            command_count: packet.commands.len(),
            support_export_reopenable: packet.inspection.support_export_reopenable,
            consumer_surfaces: packet.support_export.consumer_surfaces.clone(),
        }
    }
}

/// Error returned when a work-item linkage finalization payload cannot be projected.
#[derive(Debug)]
pub enum WorkItemLinkageFinalizationError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the linkage finalization invariants.
    Validation(WorkItemLinkageFinalizationValidationError),
}

impl fmt::Display for WorkItemLinkageFinalizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(
                formatter,
                "work item linkage finalization parse error: {err}"
            ),
            Self::Validation(err) => {
                write!(
                    formatter,
                    "work item linkage finalization validation error: {err}"
                )
            }
        }
    }
}

impl std::error::Error for WorkItemLinkageFinalizationError {}

impl From<serde_json::Error> for WorkItemLinkageFinalizationError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<WorkItemLinkageFinalizationValidationError> for WorkItemLinkageFinalizationError {
    fn from(err: WorkItemLinkageFinalizationValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for work-item linkage finalization packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemLinkageFinalizationValidationError {
    message: String,
}

impl WorkItemLinkageFinalizationValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for WorkItemLinkageFinalizationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for WorkItemLinkageFinalizationValidationError {}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn linkage_finalization_validation_error(
    message: impl Into<String>,
) -> WorkItemLinkageFinalizationValidationError {
    WorkItemLinkageFinalizationValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(linkage_finalization_validation_error(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(
    value: &str,
    field: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    if value.trim().is_empty() {
        return Err(linkage_finalization_validation_error(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    if !tokens.contains(&value) {
        return Err(linkage_finalization_validation_error(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}

fn contains_token(tokens: &[&str], value: &str) -> bool {
    tokens.contains(&value)
}

// ---------------------------------------------------------------------------
// Constructor helpers
// ---------------------------------------------------------------------------

fn linkage_finalization_record(
    input: &WorkItemLinkageFinalizationInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    stabilization_packet: &ReviewStabilizationPacket,
) -> WorkItemLinkageFinalizationRecord {
    let blocked_reasons =
        derive_blocked_reasons(&input.finalization_state, &input.invalidation_reasons);
    WorkItemLinkageFinalizationRecord {
        record_kind: WORK_ITEM_LINKAGE_FINALIZATION_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        linkage_finalization_id: input.linkage_finalization_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        stabilization_id_ref: stabilization_packet.stabilization.stabilization_id.clone(),
        finalization_state: input.finalization_state.clone(),
        invalidation_reasons: input.invalidation_reasons.clone(),
        blocked_reasons,
        actionable: input.finalization_state == "finalized_current"
            && input.invalidation_reasons.is_empty(),
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn detail_surface_record(
    input: &WorkItemDetailSurfaceInput,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
) -> WorkItemDetailSurfaceRecord {
    WorkItemDetailSurfaceRecord {
        record_kind: WORK_ITEM_DETAIL_SURFACE_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        detail_surface_id: input.detail_surface_id.clone(),
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
        provider_work_item_detail_record_id_ref: input
            .provider_work_item_detail_record_id_ref
            .clone(),
        object_row: input.object_row.clone(),
        canonical_provider_id: input.canonical_provider_id.clone(),
        provider_owned_id: input.provider_owned_id.clone(),
        write_mode_disclosure_class: input.write_mode_disclosure_class.clone(),
        row_posture_class: input.row_posture_class.clone(),
        freshness_class: input.freshness_class.clone(),
        freshness_observed_at: input.freshness_observed_at.clone(),
        provider_authoritative_state_visible: input.provider_authoritative_state_visible,
        local_draft_state_visible: input.local_draft_state_visible,
        sync_pending_state_visible: input.sync_pending_state_visible,
        offline_captured_state_visible: input.offline_captured_state_visible,
        branch_local_locator_ref: input.branch_local_locator_ref.clone(),
        review_workspace_record_id_ref: input.review_workspace_record_id_ref.clone(),
        validation_evidence_record_id_refs: input.validation_evidence_record_id_refs.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn transition_sheet_record(
    input: &StatusTransitionSheetInput,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
) -> StatusTransitionSheetRecord {
    StatusTransitionSheetRecord {
        record_kind: STATUS_TRANSITION_SHEET_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        transition_sheet_id: input.transition_sheet_id.clone(),
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
        work_item_detail_surface_id_ref: input.transition_sheet_id.clone(),
        provider_status_transition_packet_record_id_ref: input
            .provider_status_transition_packet_record_id_ref
            .clone(),
        provider_transition_review_record_id_ref: input
            .provider_transition_review_record_id_ref
            .clone(),
        previewed_side_effects: input
            .previewed_side_effects
            .iter()
            .map(|s| PreviewedSideEffectRecord {
                side_effect_id: s.side_effect_id.clone(),
                side_effect_class: s.side_effect_class.clone(),
                target_scope: s.target_scope.clone(),
                summary_label: s.summary_label.clone(),
            })
            .collect(),
        confirm_action_available: input.confirm_action_available,
        export_action_available: input.export_action_available,
        cancel_action_available: input.cancel_action_available,
        local_draft_preserved_on_failure: input.local_draft_preserved_on_failure,
        publish_later_continuity_ref: input.publish_later_continuity_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn offline_handoff_continuity_record(
    input: &OfflineHandoffContinuityInput,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
) -> OfflineHandoffContinuityRecord {
    OfflineHandoffContinuityRecord {
        record_kind: OFFLINE_HANDOFF_CONTINUITY_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        offline_handoff_continuity_id: input.offline_handoff_continuity_id.clone(),
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
        provider_offline_handoff_packet_record_id_ref: input
            .provider_offline_handoff_packet_record_id_ref
            .clone(),
        survives_restart: input.survives_restart,
        survives_reconnect: input.survives_reconnect,
        survives_export_import: input.survives_export_import,
        destination_provider_target: input.destination_provider_target.clone(),
        queued_actions_preserved: input.queued_actions_preserved,
        redaction_choices_preserved: input.redaction_choices_preserved,
        selected_evidence_preserved: input.selected_evidence_preserved,
        summary_label: input.summary_label.clone(),
    }
}

fn branch_link_record(
    input: &WorkItemBranchLinkInput,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
) -> WorkItemBranchLinkRecord {
    WorkItemBranchLinkRecord {
        record_kind: WORK_ITEM_BRANCH_LINK_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        branch_link_id: input.branch_link_id.clone(),
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
        work_item_detail_surface_id_ref: input.work_item_detail_surface_id_ref.clone(),
        branch_local_locator_ref: input.branch_local_locator_ref.clone(),
        issue_to_branch_link_class: input.issue_to_branch_link_class.clone(),
        link_source_class: input.link_source_class.clone(),
        link_freshness_class: input.link_freshness_class.clone(),
        previewable_before_publish: input.previewable_before_publish,
        summary_label: input.summary_label.clone(),
    }
}

fn review_link_record(
    input: &WorkItemReviewLinkInput,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
) -> WorkItemReviewLinkRecord {
    WorkItemReviewLinkRecord {
        record_kind: WORK_ITEM_REVIEW_LINK_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        review_link_id: input.review_link_id.clone(),
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
        work_item_detail_surface_id_ref: input.work_item_detail_surface_id_ref.clone(),
        review_workspace_record_id_ref: input.review_workspace_record_id_ref.clone(),
        linked_review_class: input.linked_review_class.clone(),
        link_source_class: input.link_source_class.clone(),
        link_freshness_class: input.link_freshness_class.clone(),
        previewable_before_publish: input.previewable_before_publish,
        summary_label: input.summary_label.clone(),
    }
}

fn publish_later_continuity_record(
    input: &PublishLaterContinuityInput,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
) -> PublishLaterContinuityRecord {
    PublishLaterContinuityRecord {
        record_kind: PUBLISH_LATER_CONTINUITY_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        continuity_id: input.continuity_id.clone(),
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
        publish_later_queue_item_record_id_ref: input
            .publish_later_queue_item_record_id_ref
            .clone(),
        provider_descriptor_ref: input.provider_descriptor_ref.clone(),
        queue_state: input.queue_state.clone(),
        drain_state: input.drain_state.clone(),
        retry_route_class: input.retry_route_class.clone(),
        survives_restart: input.survives_restart,
        survives_reconnect: input.survives_reconnect,
        summary_label: input.summary_label.clone(),
    }
}

fn linkage_command_record(
    input: &WorkItemLinkageCommandInput,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
) -> WorkItemLinkageCommandRecord {
    WorkItemLinkageCommandRecord {
        record_kind: WORK_ITEM_LINKAGE_COMMAND_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
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

fn linkage_support_export_packet(
    input: &WorkItemLinkageSupportExportInput,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    stabilization_packet: &ReviewStabilizationPacket,
    commands: &[WorkItemLinkageCommandRecord],
) -> WorkItemLinkageSupportExportPacket {
    WorkItemLinkageSupportExportPacket {
        record_kind: WORK_ITEM_LINKAGE_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        stabilization_id_ref: stabilization_packet.stabilization.stabilization_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/finalize_issue_and_work_item_linkage_with_branch.schema.json"
                .to_string(),
            "schemas/work_items/work_item_detail.schema.json".to_string(),
            "schemas/work_items/status_transition_packet.schema.json".to_string(),
            "schemas/providers/offline_handoff_packet.schema.json".to_string(),
            "schemas/providers/publish_later_queue_alpha.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn linkage_inspection_record(
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
    detail_surfaces: &[WorkItemDetailSurfaceRecord],
    transition_sheets: &[StatusTransitionSheetRecord],
    offline_handoff_continuities: &[OfflineHandoffContinuityRecord],
    branch_links: &[WorkItemBranchLinkRecord],
    review_links: &[WorkItemReviewLinkRecord],
    publish_later_continuities: &[PublishLaterContinuityRecord],
    commands: &[WorkItemLinkageCommandRecord],
    support_export: &WorkItemLinkageSupportExportPacket,
) -> WorkItemLinkageInspectionRecord {
    let finalized_current = linkage_finalization.finalization_state == "finalized_current";
    let finalized_stale_provider_overlay =
        linkage_finalization.finalization_state == "finalized_stale_provider_overlay";
    let finalized_partial_work_item_scope =
        linkage_finalization.finalization_state == "finalized_partial_work_item_scope";
    let finalized_diverged_requires_review =
        linkage_finalization.finalization_state == "finalized_diverged_requires_review";
    let finalized_offline_handoff_only =
        linkage_finalization.finalization_state == "finalized_offline_handoff_only";

    let provider_authoritative_surface_present = detail_surfaces
        .iter()
        .any(|s| s.provider_authoritative_state_visible);
    let local_draft_surface_present = detail_surfaces.iter().any(|s| s.local_draft_state_visible);
    let sync_pending_surface_present = detail_surfaces.iter().any(|s| s.sync_pending_state_visible);
    let offline_captured_surface_present = detail_surfaces
        .iter()
        .any(|s| s.offline_captured_state_visible);

    let write_mode_disclosed_on_all_surfaces = detail_surfaces.iter().all(|s| {
        contains_token(
            WRITE_MODE_DISCLOSURE_CLASSES,
            &s.write_mode_disclosure_class,
        )
    });

    let transition_sheet_present = !transition_sheets.is_empty();
    let local_draft_preserved_on_failure = transition_sheets
        .iter()
        .all(|t| t.local_draft_preserved_on_failure);

    let offline_handoff_survives_restart = offline_handoff_continuities
        .iter()
        .any(|o| o.survives_restart);
    let offline_handoff_survives_reconnect = offline_handoff_continuities
        .iter()
        .any(|o| o.survives_reconnect);

    let branch_link_previewable = branch_links.iter().any(|b| b.previewable_before_publish);
    let review_link_previewable = review_links.iter().any(|r| r.previewable_before_publish);

    let publish_later_survives_restart = publish_later_continuities
        .iter()
        .any(|p| p.survives_restart);
    let publish_later_survives_reconnect = publish_later_continuities
        .iter()
        .any(|p| p.survives_reconnect);

    let preview_capable = commands.iter().any(|c| c.preview_supported);
    let support_export_reopenable = !support_export.reopen_context_ref.trim().is_empty()
        && !support_export.reopen_command_id_ref.trim().is_empty();

    WorkItemLinkageInspectionRecord {
        record_kind: WORK_ITEM_LINKAGE_INSPECTION_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        linkage_finalization_id_ref: linkage_finalization.linkage_finalization_id.clone(),
        review_workspace_id_ref: linkage_finalization.review_workspace_id_ref.clone(),
        finalized_current,
        finalized_stale_provider_overlay,
        finalized_partial_work_item_scope,
        finalized_diverged_requires_review,
        finalized_offline_handoff_only,
        provider_authoritative_surface_present,
        local_draft_surface_present,
        sync_pending_surface_present,
        offline_captured_surface_present,
        write_mode_disclosed_on_all_surfaces,
        transition_sheet_present,
        local_draft_preserved_on_failure,
        offline_handoff_survives_restart,
        offline_handoff_survives_reconnect,
        branch_link_previewable,
        review_link_previewable,
        publish_later_survives_restart,
        publish_later_survives_reconnect,
        actionable: linkage_finalization.actionable,
        invalidated: !linkage_finalization.invalidation_reasons.is_empty(),
        command_count: commands.len(),
        detail_surface_count: detail_surfaces.len(),
        transition_sheet_count: transition_sheets.len(),
        offline_handoff_continuity_count: offline_handoff_continuities.len(),
        branch_link_count: branch_links.len(),
        review_link_count: review_links.len(),
        publish_later_continuity_count: publish_later_continuities.len(),
        preview_capable,
        support_export_reopenable,
        summary_label: format!(
            "{} detail surface(s), {} transition sheet(s), {} offline handoff(s), {} branch link(s), {} review link(s), {} publish-later continuity row(s)",
            detail_surfaces.len(),
            transition_sheets.len(),
            offline_handoff_continuities.len(),
            branch_links.len(),
            review_links.len(),
            publish_later_continuities.len()
        ),
    }
}

// ---------------------------------------------------------------------------
// Input validation
// ---------------------------------------------------------------------------

fn validate_input(
    input: &WorkItemLinkageFinalizationInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    stabilization_packet: &ReviewStabilizationPacket,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_token(
        FINALIZATION_STATES,
        &input.finalization_state,
        "finalization_state",
    )?;

    if input.detail_surfaces.is_empty() {
        return Err(linkage_finalization_validation_error(
            "detail_surfaces must include at least one work-item detail surface input",
        ));
    }

    let detail_ids: BTreeSet<_> = input
        .detail_surfaces
        .iter()
        .map(|d| d.detail_surface_id.as_str())
        .collect();
    if detail_ids.len() != input.detail_surfaces.len() {
        return Err(linkage_finalization_validation_error(
            "detail_surface_ids must be unique",
        ));
    }

    for surface in &input.detail_surfaces {
        ensure_nonempty(
            &surface.detail_surface_id,
            "detail_surface.detail_surface_id",
        )?;
        ensure_nonempty(
            &surface.canonical_provider_id,
            "detail_surface.canonical_provider_id",
        )?;
        ensure_nonempty(
            &surface.provider_owned_id,
            "detail_surface.provider_owned_id",
        )?;
        ensure_token(
            WRITE_MODE_DISCLOSURE_CLASSES,
            &surface.write_mode_disclosure_class,
            "detail_surface.write_mode_disclosure_class",
        )?;
    }

    for sheet in &input.transition_sheets {
        ensure_nonempty(
            &sheet.transition_sheet_id,
            "transition_sheet.transition_sheet_id",
        )?;
        if !detail_ids.contains(sheet.transition_sheet_id.as_str()) {
            return Err(linkage_finalization_validation_error(format!(
                "transition_sheet {} must reference a known detail_surface",
                sheet.transition_sheet_id
            )));
        }
    }

    for command in &input.commands {
        ensure_token(
            WORK_ITEM_LINKAGE_COMMAND_CLASSES,
            &command.command_class,
            "command.command_class",
        )?;
    }

    if !workspace_packet
        .review_workspace
        .review_workspace_id
        .trim()
        .is_empty()
        && !stabilization_packet
            .stabilization
            .stabilization_id
            .trim()
            .is_empty()
    {
        // workspace and stabilization are present; ok
    } else {
        return Err(linkage_finalization_validation_error(
            "workspace and stabilization packets must carry valid identities",
        ));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Record validation
// ---------------------------------------------------------------------------

fn validate_linkage_finalization_record(
    record: &WorkItemLinkageFinalizationRecord,
    workspace_id: &str,
    stabilization_id: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        WORK_ITEM_LINKAGE_FINALIZATION_RECORD_KIND,
        "linkage_finalization record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        "linkage_finalization schema_version",
    )?;
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        workspace_id,
        "linkage_finalization review_workspace_id_ref",
    )?;
    ensure_eq(
        record.stabilization_id_ref.as_str(),
        stabilization_id,
        "linkage_finalization stabilization_id_ref",
    )?;
    ensure_token(
        FINALIZATION_STATES,
        &record.finalization_state,
        "linkage_finalization finalization_state",
    )?;
    Ok(())
}

fn validate_detail_surface_record(
    record: &WorkItemDetailSurfaceRecord,
    linkage_finalization_id: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        WORK_ITEM_DETAIL_SURFACE_RECORD_KIND,
        "detail_surface record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        "detail_surface schema_version",
    )?;
    ensure_eq(
        record.linkage_finalization_id_ref.as_str(),
        linkage_finalization_id,
        "detail_surface linkage_finalization_id_ref",
    )?;
    ensure_nonempty(
        &record.canonical_provider_id,
        "detail_surface canonical_provider_id",
    )?;
    ensure_nonempty(
        &record.provider_owned_id,
        "detail_surface provider_owned_id",
    )?;
    ensure_token(
        WRITE_MODE_DISCLOSURE_CLASSES,
        &record.write_mode_disclosure_class,
        "detail_surface write_mode_disclosure_class",
    )?;
    Ok(())
}

fn validate_transition_sheet_record(
    record: &StatusTransitionSheetRecord,
    linkage_finalization_id: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STATUS_TRANSITION_SHEET_RECORD_KIND,
        "transition_sheet record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        "transition_sheet schema_version",
    )?;
    ensure_eq(
        record.linkage_finalization_id_ref.as_str(),
        linkage_finalization_id,
        "transition_sheet linkage_finalization_id_ref",
    )?;
    Ok(())
}

fn validate_offline_handoff_continuity_record(
    record: &OfflineHandoffContinuityRecord,
    linkage_finalization_id: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        OFFLINE_HANDOFF_CONTINUITY_RECORD_KIND,
        "offline_handoff_continuity record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        "offline_handoff_continuity schema_version",
    )?;
    ensure_eq(
        record.linkage_finalization_id_ref.as_str(),
        linkage_finalization_id,
        "offline_handoff_continuity linkage_finalization_id_ref",
    )?;
    Ok(())
}

fn validate_branch_link_record(
    record: &WorkItemBranchLinkRecord,
    linkage_finalization_id: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        WORK_ITEM_BRANCH_LINK_RECORD_KIND,
        "branch_link record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        "branch_link schema_version",
    )?;
    ensure_eq(
        record.linkage_finalization_id_ref.as_str(),
        linkage_finalization_id,
        "branch_link linkage_finalization_id_ref",
    )?;
    Ok(())
}

fn validate_review_link_record(
    record: &WorkItemReviewLinkRecord,
    linkage_finalization_id: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        WORK_ITEM_REVIEW_LINK_RECORD_KIND,
        "review_link record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        "review_link schema_version",
    )?;
    ensure_eq(
        record.linkage_finalization_id_ref.as_str(),
        linkage_finalization_id,
        "review_link linkage_finalization_id_ref",
    )?;
    Ok(())
}

fn validate_publish_later_continuity_record(
    record: &PublishLaterContinuityRecord,
    linkage_finalization_id: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PUBLISH_LATER_CONTINUITY_RECORD_KIND,
        "publish_later_continuity record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        "publish_later_continuity schema_version",
    )?;
    ensure_eq(
        record.linkage_finalization_id_ref.as_str(),
        linkage_finalization_id,
        "publish_later_continuity linkage_finalization_id_ref",
    )?;
    Ok(())
}

fn validate_command_record(
    record: &WorkItemLinkageCommandRecord,
    linkage_finalization_id: &str,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        WORK_ITEM_LINKAGE_COMMAND_RECORD_KIND,
        "command record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
        "command schema_version",
    )?;
    ensure_eq(
        record.linkage_finalization_id_ref.as_str(),
        linkage_finalization_id,
        "command linkage_finalization_id_ref",
    )?;
    ensure_token(
        WORK_ITEM_LINKAGE_COMMAND_CLASSES,
        &record.command_class,
        "command command_class",
    )?;
    Ok(())
}

fn validate_support_export(
    export: &WorkItemLinkageSupportExportPacket,
    linkage_finalization: &WorkItemLinkageFinalizationRecord,
    commands: &[WorkItemLinkageCommandRecord],
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        WORK_ITEM_LINKAGE_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export record_kind",
    )?;
    ensure_eq(
        export.linkage_finalization_id_ref.as_str(),
        linkage_finalization.linkage_finalization_id.as_str(),
        "support_export linkage_finalization_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        linkage_finalization.review_workspace_id_ref.as_str(),
        "support_export review_workspace_id_ref",
    )?;
    ensure_eq(
        export.stabilization_id_ref.as_str(),
        linkage_finalization.stabilization_id_ref.as_str(),
        "support_export stabilization_id_ref",
    )?;
    if export.raw_url_export_allowed {
        return Err(linkage_finalization_validation_error(
            "support_export raw_url_export_allowed must be false",
        ));
    }
    if export.raw_provider_payload_export_allowed {
        return Err(linkage_finalization_validation_error(
            "support_export raw_provider_payload_export_allowed must be false",
        ));
    }
    if export.command_id_refs.len() != commands.len() {
        return Err(linkage_finalization_validation_error(
            "support_export command_id_refs length must match commands length",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &WorkItemLinkageInspectionRecord,
    packet: &WorkItemLinkageFinalizationPacket,
) -> Result<(), WorkItemLinkageFinalizationValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        WORK_ITEM_LINKAGE_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.linkage_finalization_id_ref.as_str(),
        packet.linkage_finalization.linkage_finalization_id.as_str(),
        "inspection linkage_finalization_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.review_workspace.review_workspace_id.as_str(),
        "inspection review_workspace_id_ref",
    )?;
    if inspection.command_count != packet.commands.len() {
        return Err(linkage_finalization_validation_error(
            "inspection command_count must match commands length",
        ));
    }
    if inspection.detail_surface_count != packet.detail_surfaces.len() {
        return Err(linkage_finalization_validation_error(
            "inspection detail_surface_count must match detail_surfaces length",
        ));
    }
    if inspection.transition_sheet_count != packet.transition_sheets.len() {
        return Err(linkage_finalization_validation_error(
            "inspection transition_sheet_count must match transition_sheets length",
        ));
    }
    if inspection.offline_handoff_continuity_count != packet.offline_handoff_continuities.len() {
        return Err(linkage_finalization_validation_error(
            "inspection offline_handoff_continuity_count must match offline_handoff_continuities length",
        ));
    }
    if inspection.branch_link_count != packet.branch_links.len() {
        return Err(linkage_finalization_validation_error(
            "inspection branch_link_count must match branch_links length",
        ));
    }
    if inspection.review_link_count != packet.review_links.len() {
        return Err(linkage_finalization_validation_error(
            "inspection review_link_count must match review_links length",
        ));
    }
    if inspection.publish_later_continuity_count != packet.publish_later_continuities.len() {
        return Err(linkage_finalization_validation_error(
            "inspection publish_later_continuity_count must match publish_later_continuities length",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Derivation helpers
// ---------------------------------------------------------------------------

fn derive_blocked_reasons(
    finalization_state: &str,
    invalidation_reasons: &[String],
) -> Vec<String> {
    let mut reasons = Vec::new();
    if finalization_state == "finalized_stale_provider_overlay" {
        reasons.push("stale_provider_overlay".to_string());
    }
    if finalization_state == "finalized_partial_work_item_scope" {
        reasons.push("partial_work_item_scope".to_string());
    }
    if finalization_state == "finalized_diverged_requires_review" {
        reasons.push("diverged_requires_review".to_string());
    }
    if finalization_state == "finalized_offline_handoff_only" {
        reasons.push("offline_handoff_only".to_string());
    }
    for reason in invalidation_reasons {
        reasons.push(reason.clone());
    }
    reasons.sort();
    reasons.dedup();
    reasons
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_provider::{project_work_item_object_row, seeded_work_item_transition_beta_page};

    fn sample_object_row() -> WorkItemObjectRowRecord {
        let page = seeded_work_item_transition_beta_page();
        project_work_item_object_row(&page.detail_records[0])
    }

    #[test]
    fn constants_are_nonempty() {
        assert!(!FINALIZATION_STATES.is_empty());
        assert!(!WRITE_MODE_DISCLOSURE_CLASSES.is_empty());
        assert!(!WORK_ITEM_LINKAGE_COMMAND_CLASSES.is_empty());
        assert!(!WORK_ITEM_LINKAGE_CONSUMER_SURFACES.is_empty());
        assert!(!WORK_ITEM_LINKAGE_INVALIDATION_REASONS.is_empty());
    }

    #[test]
    fn validation_rejects_empty_packet_id() {
        let input = WorkItemLinkageFinalizationInput {
            linkage_finalization_id: "linkage:001".to_string(),
            packet_id: "".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            finalization_state: "finalized_current".to_string(),
            detail_surfaces: vec![WorkItemDetailSurfaceInput {
                detail_surface_id: "detail:001".to_string(),
                provider_work_item_detail_record_id_ref: "provider:detail:001".to_string(),
                object_row: Some(sample_object_row()),
                canonical_provider_id: "AUR-001".to_string(),
                provider_owned_id: "provider:issue:001".to_string(),
                write_mode_disclosure_class: "full_edit".to_string(),
                row_posture_class: "provider_authoritative".to_string(),
                freshness_class: "live_authoritative_fresh".to_string(),
                freshness_observed_at: "2026-05-27T10:00:00Z".to_string(),
                provider_authoritative_state_visible: true,
                local_draft_state_visible: false,
                sync_pending_state_visible: false,
                offline_captured_state_visible: false,
                branch_local_locator_ref: "branch:main".to_string(),
                review_workspace_record_id_ref: "workspace:001".to_string(),
                validation_evidence_record_id_refs: vec![],
                summary_label: "Detail surface".to_string(),
            }],
            transition_sheets: vec![],
            offline_handoff_continuities: vec![],
            branch_links: vec![],
            review_links: vec![],
            publish_later_continuities: vec![],
            commands: vec![],
            support_export: WorkItemLinkageSupportExportInput {
                support_export_id: "export:001".to_string(),
                reopen_context_ref: "context:001".to_string(),
                reopen_command_id_ref: "cmd:001".to_string(),
                consumer_surfaces: vec!["review_workspace_inspector".to_string()],
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Support export".to_string(),
            },
            invalidation_reasons: vec![],
            summary_label: "Test".to_string(),
        };

        let workspace = workspace_beta_packet();
        let stabilization = stabilization_packet();
        let result = WorkItemLinkageFinalizationPacket::from_workspace_and_stabilization_packets(
            input,
            &workspace,
            &stabilization,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message().contains("packet_id must not be empty"));
    }

    #[test]
    fn validation_rejects_invalid_finalization_state() {
        let input = WorkItemLinkageFinalizationInput {
            linkage_finalization_id: "linkage:001".to_string(),
            packet_id: "packet:001".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            finalization_state: "invalid_state".to_string(),
            detail_surfaces: vec![WorkItemDetailSurfaceInput {
                detail_surface_id: "detail:001".to_string(),
                provider_work_item_detail_record_id_ref: "provider:detail:001".to_string(),
                object_row: Some(sample_object_row()),
                canonical_provider_id: "AUR-001".to_string(),
                provider_owned_id: "provider:issue:001".to_string(),
                write_mode_disclosure_class: "full_edit".to_string(),
                row_posture_class: "provider_authoritative".to_string(),
                freshness_class: "live_authoritative_fresh".to_string(),
                freshness_observed_at: "2026-05-27T10:00:00Z".to_string(),
                provider_authoritative_state_visible: true,
                local_draft_state_visible: false,
                sync_pending_state_visible: false,
                offline_captured_state_visible: false,
                branch_local_locator_ref: "branch:main".to_string(),
                review_workspace_record_id_ref: "workspace:001".to_string(),
                validation_evidence_record_id_refs: vec![],
                summary_label: "Detail surface".to_string(),
            }],
            transition_sheets: vec![],
            offline_handoff_continuities: vec![],
            branch_links: vec![],
            review_links: vec![],
            publish_later_continuities: vec![],
            commands: vec![],
            support_export: WorkItemLinkageSupportExportInput {
                support_export_id: "export:001".to_string(),
                reopen_context_ref: "context:001".to_string(),
                reopen_command_id_ref: "cmd:001".to_string(),
                consumer_surfaces: vec!["review_workspace_inspector".to_string()],
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Support export".to_string(),
            },
            invalidation_reasons: vec![],
            summary_label: "Test".to_string(),
        };

        let workspace = workspace_beta_packet();
        let stabilization = stabilization_packet();
        let result = WorkItemLinkageFinalizationPacket::from_workspace_and_stabilization_packets(
            input,
            &workspace,
            &stabilization,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message().contains("finalization_state must be one of"));
    }

    #[test]
    fn packet_round_trips_through_json() {
        let input = WorkItemLinkageFinalizationInput {
            linkage_finalization_id: "linkage:001".to_string(),
            packet_id: "packet:001".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            finalization_state: "finalized_current".to_string(),
            detail_surfaces: vec![WorkItemDetailSurfaceInput {
                detail_surface_id: "detail:001".to_string(),
                provider_work_item_detail_record_id_ref: "provider:detail:001".to_string(),
                object_row: Some(sample_object_row()),
                canonical_provider_id: "AUR-001".to_string(),
                provider_owned_id: "provider:issue:001".to_string(),
                write_mode_disclosure_class: "full_edit".to_string(),
                row_posture_class: "provider_authoritative".to_string(),
                freshness_class: "live_authoritative_fresh".to_string(),
                freshness_observed_at: "2026-05-27T10:00:00Z".to_string(),
                provider_authoritative_state_visible: true,
                local_draft_state_visible: false,
                sync_pending_state_visible: false,
                offline_captured_state_visible: false,
                branch_local_locator_ref: "branch:main".to_string(),
                review_workspace_record_id_ref: "workspace:001".to_string(),
                validation_evidence_record_id_refs: vec![],
                summary_label: "Detail surface".to_string(),
            }],
            transition_sheets: vec![StatusTransitionSheetInput {
                transition_sheet_id: "detail:001".to_string(),
                provider_status_transition_packet_record_id_ref: "provider:transition:001"
                    .to_string(),
                provider_transition_review_record_id_ref: "provider:review:001".to_string(),
                previewed_side_effects: vec![PreviewedSideEffectInput {
                    side_effect_id: "effect:001".to_string(),
                    side_effect_class: "comment_creation".to_string(),
                    target_scope: "provider:issue:001".to_string(),
                    summary_label: "Create comment".to_string(),
                }],
                confirm_action_available: true,
                export_action_available: true,
                cancel_action_available: true,
                local_draft_preserved_on_failure: true,
                publish_later_continuity_ref: None,
                summary_label: "Transition sheet".to_string(),
            }],
            offline_handoff_continuities: vec![],
            branch_links: vec![WorkItemBranchLinkInput {
                branch_link_id: "branch_link:001".to_string(),
                work_item_detail_surface_id_ref: "detail:001".to_string(),
                branch_local_locator_ref: "branch:main".to_string(),
                issue_to_branch_link_class: "linked_provider_branch_overlay_fetched".to_string(),
                link_source_class: "provider_authoritative_overlay".to_string(),
                link_freshness_class: "live_authoritative_fresh".to_string(),
                previewable_before_publish: true,
                summary_label: "Branch link".to_string(),
            }],
            review_links: vec![WorkItemReviewLinkInput {
                review_link_id: "review_link:001".to_string(),
                work_item_detail_surface_id_ref: "detail:001".to_string(),
                review_workspace_record_id_ref: "workspace:001".to_string(),
                linked_review_class: "linked_review_workspace_with_provider_overlay".to_string(),
                link_source_class: "provider_authoritative_overlay".to_string(),
                link_freshness_class: "live_authoritative_fresh".to_string(),
                previewable_before_publish: true,
                summary_label: "Review link".to_string(),
            }],
            publish_later_continuities: vec![],
            commands: vec![WorkItemLinkageCommandInput {
                command_id: "cmd:001".to_string(),
                command_class: "preview_linkage".to_string(),
                target_object_ref: "detail:001".to_string(),
                target_object_kind: "work_item_detail_surface".to_string(),
                preview_supported: true,
                emits_audit_event: false,
                blocked_reasons: vec![],
                summary_label: "Preview linkage".to_string(),
            }],
            support_export: WorkItemLinkageSupportExportInput {
                support_export_id: "export:001".to_string(),
                reopen_context_ref: "context:001".to_string(),
                reopen_command_id_ref: "cmd:001".to_string(),
                consumer_surfaces: vec!["review_workspace_inspector".to_string()],
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Support export".to_string(),
            },
            invalidation_reasons: vec![],
            summary_label: "Test".to_string(),
        };

        let workspace = workspace_beta_packet();
        let stabilization = stabilization_packet();
        let packet = WorkItemLinkageFinalizationPacket::from_workspace_and_stabilization_packets(
            input,
            &workspace,
            &stabilization,
        )
        .expect("packet must construct");

        let json = serde_json::to_string(&packet).expect("serialization must succeed");
        let reparsed: WorkItemLinkageFinalizationPacket =
            serde_json::from_str(&json).expect("re-deserialization must succeed");
        reparsed.validate().expect("round-trip must validate");
        assert_eq!(packet.packet_id, reparsed.packet_id);
        assert_eq!(packet.detail_surfaces.len(), reparsed.detail_surfaces.len());
    }

    fn workspace_beta_packet() -> ReviewWorkspaceBetaPacket {
        use crate::workspace::{
            ReviewLocalLocator, ReviewPolicyContext, ReviewWorkspaceRecord,
            ReviewWorkspaceSupportExportPacket,
        };

        let workspace = ReviewWorkspaceRecord {
            record_kind: "review_workspace_record".to_string(),
            review_workspace_schema_version: 1,
            review_workspace_id: "workspace:001".to_string(),
            review_workspace_source_class: "local_git".to_string(),
            provider_authority_class: "local_authoritative".to_string(),
            review_workspace_lifecycle_state: "open".to_string(),
            local_locator: Some(ReviewLocalLocator {
                workspace_id_ref: "workspace:001".to_string(),
                branch_or_worktree_ref: "branch:main".to_string(),
                base_revision_ref: None,
                head_revision_ref: None,
            }),
            provider_overlay: None,
            imported_bundle_envelope: None,
            browser_handoff_envelope: None,
            policy_context: ReviewPolicyContext {
                policy_epoch: "epoch:001".to_string(),
                trust_state: "trusted".to_string(),
                execution_context_id: None,
                workspace_trust_state_class: "trusted".to_string(),
            },
            client_scopes: vec!["desktop".to_string()],
            redaction_class: "metadata_safe_default".to_string(),
            freshness_class: "fresh".to_string(),
            summary_label: "Workspace".to_string(),
            created_at: "2026-05-27T10:00:00Z".to_string(),
            updated_at: "2026-05-27T10:00:00Z".to_string(),
            archived_at: None,
            hosted_review_inbox_record_id_ref: None,
            merge_policy_record_id_ref: None,
        };

        ReviewWorkspaceBetaPacket {
            record_kind: "review_workspace_beta_packet".to_string(),
            schema_version: 1,
            packet_id: "packet:workspace:001".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            review_workspace: workspace,
            diff_entries: vec![],
            durable_comment_anchors: vec![],
            object_lineage: vec![],
            check_freshness: vec![],
            browser_handoff: None,
            support_export: ReviewWorkspaceSupportExportPacket {
                record_kind: "review_workspace_support_export_packet".to_string(),
                schema_version: 1,
                support_export_id: "export:001".to_string(),
                review_workspace_id_ref: "workspace:001".to_string(),
                reopen_context_ref: "context:001".to_string(),
                reopen_command_id_ref: "cmd:001".to_string(),
                durable_comment_anchor_refs: vec![],
                check_freshness_refs: vec![],
                object_lineage_refs: vec![],
                browser_handoff_ref: None,
                consumer_surfaces: vec!["review_workspace_inspector".to_string()],
                source_schema_refs: vec![],
                raw_comment_body_export_allowed: false,
                raw_url_export_allowed: false,
                raw_source_body_export_allowed: false,
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Support export".to_string(),
            },
            inspection: crate::workspace::ReviewWorkspaceBetaInspectionRecord {
                record_kind: "review_workspace_beta_inspection_record".to_string(),
                schema_version: 1,
                review_workspace_id_ref: "workspace:001".to_string(),
                durable_comment_anchor_count: 0,
                object_lineage_count: 0,
                check_freshness_count: 0,
                anchor_identity_preserved: true,
                object_lineage_preserved: true,
                check_freshness_browser_independent: true,
                typed_reversible_browser_handoff_present: false,
                support_export_reopenable: true,
                raw_escape_hatches_absent: true,
                operator_truth_current: true,
                stale_check_blocks_operator_truth: false,
                summary_label: "Inspection".to_string(),
            },
        }
    }

    fn stabilization_packet() -> ReviewStabilizationPacket {
        use crate::landing::LandingCandidateRecord;
        use crate::stabilize_review_workspace_anchors_stale_base_labels_approval::{
            MergeabilityTruthRecord, ReviewStabilizationRecord,
            ReviewStabilizationSupportExportPacket, StaleBaseLabelRecord,
        };

        let landing_candidate = LandingCandidateRecord {
            record_kind: "landing_candidate_record".to_string(),
            schema_version: 1,
            landing_candidate_id: "landing:001".to_string(),
            review_workspace_id_ref: "workspace:001".to_string(),
            target_branch_ref: "branch:main".to_string(),
            base_revision_ref: "base:001".to_string(),
            head_revision_ref: "head:001".to_string(),
            change_object_ref: "change:001".to_string(),
            worktree_identity_ref: "worktree:001".to_string(),
            review_pack_digest_ref: "digest:001".to_string(),
            environment_capsule_digest_ref: "capsule:001".to_string(),
            merge_strategy_class: "merge".to_string(),
            landing_authority_class: "local_authoritative".to_string(),
            mergeable_state: "mergeable".to_string(),
            eligibility_state: "eligible".to_string(),
            stale_base_state: "base_current".to_string(),
            checks_freshness_state: "current".to_string(),
            approval_state: "approved".to_string(),
            policy_block_state: "none".to_string(),
            required_check_ids: vec![],
            invalidation_reasons: vec![],
            blocked_reasons: vec![],
            landing_requires_explicit_candidate: true,
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            summary_label: "Landing candidate".to_string(),
        };

        let stabilization = ReviewStabilizationRecord {
            record_kind: "review_stabilization_record".to_string(),
            schema_version: 1,
            stabilization_id: "stabilization:001".to_string(),
            review_workspace_id_ref: "workspace:001".to_string(),
            landing_candidate_id_ref: "landing:001".to_string(),
            stabilization_state: "stabilized_current".to_string(),
            review_pack_digest_ref: "digest:001".to_string(),
            base_revision_ref: "base:001".to_string(),
            head_revision_ref: "head:001".to_string(),
            required_check_ids: vec![],
            invalidation_reasons: vec![],
            blocked_reasons: vec![],
            actionable: true,
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            summary_label: "Stabilization".to_string(),
        };

        ReviewStabilizationPacket {
            record_kind: "review_stabilization_packet".to_string(),
            schema_version: 1,
            packet_id: "packet:stabilization:001".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            review_workspace: workspace_beta_packet().review_workspace,
            landing_candidate,
            stabilization,
            anchor_stabilities: vec![],
            stale_base_label: StaleBaseLabelRecord {
                record_kind: "review_stale_base_label_record".to_string(),
                schema_version: 1,
                stabilization_id_ref: "stabilization:001".to_string(),
                stale_base_label_class: "base_current".to_string(),
                base_revision_ref: "base:001".to_string(),
                head_revision_ref: "head:001".to_string(),
                divergence_label_class: "no_divergence".to_string(),
                blocks_landing: false,
                summary_label: "Stale base".to_string(),
            },
            approval_invalidation: None,
            mergeability_truth: MergeabilityTruthRecord {
                record_kind: "review_mergeability_truth_record".to_string(),
                schema_version: 1,
                stabilization_id_ref: "stabilization:001".to_string(),
                mergeability_truth_class: "mergeable".to_string(),
                required_check_ids: vec![],
                checks_freshness_state: "current".to_string(),
                provider_authoritative: false,
                summary_label: "Mergeability".to_string(),
            },
            ownership_signals: vec![],
            bundle_export: None,
            bundle_import: None,
            offline_handoff: None,
            commands: vec![],
            support_export: ReviewStabilizationSupportExportPacket {
                record_kind: "review_stabilization_support_export_packet".to_string(),
                schema_version: 1,
                support_export_id: "export:001".to_string(),
                stabilization_id_ref: "stabilization:001".to_string(),
                review_workspace_id_ref: "workspace:001".to_string(),
                landing_candidate_id_ref: "landing:001".to_string(),
                reopen_context_ref: "context:001".to_string(),
                reopen_command_id_ref: "cmd:001".to_string(),
                command_id_refs: vec![],
                consumer_surfaces: vec!["review_workspace_inspector".to_string()],
                source_schema_refs: vec![],
                raw_url_export_allowed: false,
                raw_provider_payload_export_allowed: false,
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Support export".to_string(),
            },
            inspection: crate::stabilize_review_workspace_anchors_stale_base_labels_approval::ReviewStabilizationInspectionRecord {
                record_kind: "review_stabilization_inspection_record".to_string(),
                schema_version: 1,
                stabilization_id_ref: "stabilization:001".to_string(),
                review_workspace_id_ref: "workspace:001".to_string(),
                stabilized_current: true,
                stabilized_stale_pack: false,
                stabilized_partial_scope: false,
                stabilized_slice_omitted: false,
                stabilized_diverged_requires_review: false,
                all_anchors_bound_exact: true,
                any_anchor_drifted: false,
                stale_base_blocks_landing: false,
                approval_invalidated: false,
                mergeability_blocking: false,
                mergeability_provider_authoritative: false,
                enforceable_ownership_present: false,
                advisory_ownership_present: false,
                ownership_conflict_present: false,
                bundle_export_present: false,
                bundle_import_present: false,
                offline_handoff_present: false,
                actionable: true,
                invalidated: false,
                command_count: 0,
                anchor_stability_count: 0,
                ownership_signal_count: 0,
                preview_capable: false,
                support_export_reopenable: true,
                summary_label: "Inspection".to_string(),
            },
        }
    }
}
