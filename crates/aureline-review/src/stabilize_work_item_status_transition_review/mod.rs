//! Stabilized work-item status-transition review, offline handoff packets, and
//! publish-later continuity for provider-linked lanes.
//!
//! This module turns provider work-item detail, transition, and offline-handoff
//! beta records into one stable review packet. Every claimed provider lane MUST
//! show exact provider side effects before mutation — comments, status changes,
//! assignee changes, label changes, and branch/review link updates — and MUST
//! preserve offline handoff packets across restart, reconnect, and export.
//!
//! The record family includes:
//!
//! - [`StableWorkItemStatusTransitionRecord`] — stable identity binding
//!   workspace, provider object rows, and explicit mutation modes.
//! - [`StableWorkItemDetailRecord`] — governed work-item detail header with
//!   canonical provider-owned ID, explicit freshness, and write-mode disclosure.
//! - [`StableStatusTransitionSheetRecord`] — status-transition sheet that
//!   previews exact provider side effects before commit.
//! - [`StableOfflineHandoffRecord`] — durable offline handoff packet preserving
//!   canonical work-item IDs, selected evidence, redaction choices, queued
//!   actions, publish target, expiry, and retry/export/discard semantics.
//! - [`StablePublishLaterContinuityRecord`] — publish-later queue continuity.
//! - [`StableWorkItemCommandRecord`] — command-graph operations surfaced to the
//!   inspector.
//! - [`StableWorkItemSupportExportPacket`] — redaction-safe support export.
//! - [`StableWorkItemInspectionRecord`] — compact boolean projection for CLI
//!   and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/stable_work_item_status_transition_review.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/stabilize-work-item-status-transition-review/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable work-item status-transition review record.
pub const STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every stable work-item record.
pub const STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF: &str =
    "review:stable_work_item_status_transition_review:v1";

/// Stable record-kind tag for [`StableWorkItemStatusTransitionPacket`].
pub const STABLE_WORK_ITEM_STATUS_TRANSITION_PACKET_RECORD_KIND: &str =
    "review_stable_work_item_status_transition_packet";

/// Stable record-kind tag for [`StableWorkItemStatusTransitionRecord`].
pub const STABLE_WORK_ITEM_STATUS_TRANSITION_RECORD_KIND: &str =
    "review_stable_work_item_status_transition_record";

/// Stable record-kind tag for [`StableWorkItemDetailRecord`].
pub const STABLE_WORK_ITEM_DETAIL_RECORD_KIND: &str = "review_stable_work_item_detail_record";

/// Stable record-kind tag for [`StableStatusTransitionSheetRecord`].
pub const STABLE_STATUS_TRANSITION_SHEET_RECORD_KIND: &str =
    "review_stable_status_transition_sheet_record";

/// Stable record-kind tag for [`StableOfflineHandoffRecord`].
pub const STABLE_OFFLINE_HANDOFF_RECORD_KIND: &str = "review_stable_offline_handoff_record";

/// Stable record-kind tag for [`StablePublishLaterContinuityRecord`].
pub const STABLE_PUBLISH_LATER_CONTINUITY_RECORD_KIND: &str =
    "review_stable_publish_later_continuity_record";

/// Stable record-kind tag for [`StableWorkItemCommandRecord`].
pub const STABLE_WORK_ITEM_COMMAND_RECORD_KIND: &str = "review_stable_work_item_command_record";

/// Stable record-kind tag for [`StableWorkItemSupportExportPacket`].
pub const STABLE_WORK_ITEM_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_stable_work_item_support_export_packet";

/// Stable record-kind tag for [`StableWorkItemInspectionRecord`].
pub const STABLE_WORK_ITEM_INSPECTION_RECORD_KIND: &str =
    "review_stable_work_item_inspection_record";

/// Closed set of explicit mutation mode classes.
pub const WORK_ITEM_MUTATION_MODE_CLASSES: &[&str] = &[
    "local_draft",
    "publish_later",
    "publish_now",
    "open_in_provider",
];

/// Closed set of work-item row posture classes.
pub const WORK_ITEM_ROW_POSTURE_CLASSES: &[&str] = &[
    "provider_authoritative",
    "cached_stale",
    "read_only",
    "policy_blocked",
    "local_draft",
    "queued",
    "offline_captured",
];

/// Closed set of side-effect classes previewed in transition sheets.
pub const SIDE_EFFECT_CLASSES: &[&str] = &[
    "comment_create",
    "status_change",
    "assignee_change",
    "label_add",
    "label_remove",
    "branch_link_create",
    "branch_link_update",
    "review_link_create",
    "review_link_update",
];

/// Closed set of offline handoff states.
pub const OFFLINE_HANDOFF_STATES: &[&str] = &[
    "handoff_ready",
    "handoff_queued",
    "handoff_published",
    "handoff_expired",
    "handoff_retrying",
    "handoff_exported",
    "handoff_discarded",
];

/// Closed set of publish-later continuity states.
pub const PUBLISH_LATER_CONTINUITY_STATES: &[&str] = &[
    "queued_pending",
    "queued_ready",
    "queued_published",
    "queued_failed",
    "queued_cancelled",
];

/// Closed set of consumer surfaces for stable work-item packets.
pub const STABLE_WORK_ITEM_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "work_item_lane",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
    "offline_handoff",
];

/// Closed set of invalidation reasons.
pub const STABLE_WORK_ITEM_INVALIDATION_REASONS: &[&str] = &[
    "stale_provider_overlay",
    "work_item_scope_changed",
    "status_transition_stale",
    "offline_handoff_superseded",
    "publish_later_queue_changed",
    "policy_blocked",
    "provider_drift",
];

/// Closed set of command classes.
pub const STABLE_WORK_ITEM_COMMAND_CLASSES: &[&str] = &[
    "preview_status_transition",
    "confirm_publish_now",
    "save_local_draft",
    "queue_publish_later",
    "export_offline_handoff",
    "open_in_provider",
    "refresh_provider_overlay",
    "discard_handoff",
    "retry_handoff",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable work-item status-transition review to materialize
/// on top of a beta review-workspace packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemStatusTransitionInput {
    /// Stable status-transition identity.
    pub status_transition_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Work-item detail inputs.
    pub work_item_details: Vec<StableWorkItemDetailInput>,
    /// Status-transition sheet inputs.
    pub transition_sheets: Vec<StableStatusTransitionSheetInput>,
    /// Offline-handoff inputs.
    pub offline_handoffs: Vec<StableOfflineHandoffInput>,
    /// Publish-later continuity inputs.
    pub publish_later_continuities: Vec<StablePublishLaterContinuityInput>,
    /// Command-graph operations.
    pub commands: Vec<StableWorkItemCommandInput>,
    /// Support/export envelope input.
    pub support_export: StableWorkItemSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one governed work-item detail header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemDetailInput {
    /// Stable detail identity.
    pub detail_id: String,
    /// Canonical provider-owned work-item ID.
    pub canonical_work_item_id: String,
    /// Provider-owned opaque object ID.
    pub provider_owned_id: String,
    /// Write-mode disclosure class.
    pub write_mode_disclosure_class: String,
    /// Row posture class.
    pub row_posture_class: String,
    /// Freshness class.
    pub freshness_class: String,
    /// True when provider-authoritative state is visible.
    pub provider_authoritative_state_visible: bool,
    /// True when local-draft state is visible.
    pub local_draft_state_visible: bool,
    /// True when sync-pending state is visible.
    pub sync_pending_state_visible: bool,
    /// True when offline-captured state is visible.
    pub offline_captured_state_visible: bool,
    /// Mutation mode.
    pub mutation_mode: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one status-transition sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableStatusTransitionSheetInput {
    /// Stable transition sheet identity.
    pub transition_sheet_id: String,
    /// Work-item detail ref.
    pub work_item_detail_id_ref: String,
    /// Previewed side effects.
    pub previewed_side_effects: Vec<StablePreviewedSideEffectInput>,
    /// Confirm action available.
    pub confirm_action_available: bool,
    /// Export action available.
    pub export_action_available: bool,
    /// Cancel action available.
    pub cancel_action_available: bool,
    /// Local draft preserved on failure.
    pub local_draft_preserved_on_failure: bool,
    /// Optional publish-later continuity ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_continuity_ref: Option<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// One previewed side effect in a status-transition sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePreviewedSideEffectInput {
    /// Side-effect id.
    pub side_effect_id: String,
    /// Side-effect class.
    pub side_effect_class: String,
    /// Target scope.
    pub target_scope: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one durable offline handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableOfflineHandoffInput {
    /// Stable offline-handoff identity.
    pub offline_handoff_id: String,
    /// Canonical work-item IDs preserved.
    pub canonical_work_item_ids: Vec<String>,
    /// Selected evidence refs.
    pub selected_evidence_refs: Vec<String>,
    /// Redaction choices.
    pub redaction_choices: Vec<String>,
    /// Queued actions.
    pub queued_actions: Vec<String>,
    /// Publish target.
    pub publish_target: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Retry semantics class.
    pub retry_semantics: String,
    /// Export semantics class.
    pub export_semantics: String,
    /// Discard semantics class.
    pub discard_semantics: String,
    /// True when the packet survives restart.
    pub survives_restart: bool,
    /// True when the packet survives reconnect.
    pub survives_reconnect: bool,
    /// True when the packet survives export/import.
    pub survives_export_import: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one publish-later continuity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePublishLaterContinuityInput {
    /// Stable continuity identity.
    pub continuity_id: String,
    /// Work-item detail ref.
    pub work_item_detail_id_ref: String,
    /// Continuity state.
    pub continuity_state: String,
    /// Queued action refs.
    pub queued_action_refs: Vec<String>,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing one command-graph operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemCommandInput {
    /// Stable command identity.
    pub command_id: String,
    /// Command class.
    pub command_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when preview/dry-run is supported.
    pub preview_supported: bool,
    /// True when the command emits an audit event.
    pub emits_audit_event: bool,
    /// Active blocked reasons.
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Input describing the support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the review.
    pub reopen_context_ref: String,
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

/// Stable work-item status-transition review record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemStatusTransitionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable status-transition identity.
    pub status_transition_id: String,
    /// Review workspace this review belongs to.
    pub review_workspace_id_ref: String,
    /// Number of work-item details.
    pub work_item_detail_count: usize,
    /// Number of transition sheets.
    pub transition_sheet_count: usize,
    /// Number of offline handoffs.
    pub offline_handoff_count: usize,
    /// Number of publish-later continuities.
    pub publish_later_continuity_count: usize,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// True when the review is actionable.
    pub actionable: bool,
    /// Timestamp the review was frozen.
    pub generated_at: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Stable work-item detail record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemDetailRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable detail identity.
    pub detail_id: String,
    /// Canonical provider-owned work-item ID.
    pub canonical_work_item_id: String,
    /// Provider-owned opaque object ID.
    pub provider_owned_id: String,
    /// Write-mode disclosure class.
    pub write_mode_disclosure_class: String,
    /// Row posture class.
    pub row_posture_class: String,
    /// Freshness class.
    pub freshness_class: String,
    /// True when provider-authoritative state is visible.
    pub provider_authoritative_state_visible: bool,
    /// True when local-draft state is visible.
    pub local_draft_state_visible: bool,
    /// True when sync-pending state is visible.
    pub sync_pending_state_visible: bool,
    /// True when offline-captured state is visible.
    pub offline_captured_state_visible: bool,
    /// Mutation mode.
    pub mutation_mode: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Stable status-transition sheet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableStatusTransitionSheetRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable transition sheet identity.
    pub transition_sheet_id: String,
    /// Work-item detail ref.
    pub work_item_detail_id_ref: String,
    /// Previewed side effects.
    pub previewed_side_effects: Vec<StablePreviewedSideEffectRecord>,
    /// Confirm action available.
    pub confirm_action_available: bool,
    /// Export action available.
    pub export_action_available: bool,
    /// Cancel action available.
    pub cancel_action_available: bool,
    /// Local draft preserved on failure.
    pub local_draft_preserved_on_failure: bool,
    /// Optional publish-later continuity ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_continuity_ref: Option<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// One previewed side effect record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePreviewedSideEffectRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Side-effect id.
    pub side_effect_id: String,
    /// Side-effect class.
    pub side_effect_class: String,
    /// Target scope.
    pub target_scope: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Stable offline handoff record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableOfflineHandoffRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable offline-handoff identity.
    pub offline_handoff_id: String,
    /// Canonical work-item IDs preserved.
    pub canonical_work_item_ids: Vec<String>,
    /// Selected evidence refs.
    pub selected_evidence_refs: Vec<String>,
    /// Redaction choices.
    pub redaction_choices: Vec<String>,
    /// Queued actions.
    pub queued_actions: Vec<String>,
    /// Publish target.
    pub publish_target: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Retry semantics class.
    pub retry_semantics: String,
    /// Export semantics class.
    pub export_semantics: String,
    /// Discard semantics class.
    pub discard_semantics: String,
    /// True when the packet survives restart.
    pub survives_restart: bool,
    /// True when the packet survives reconnect.
    pub survives_reconnect: bool,
    /// True when the packet survives export/import.
    pub survives_export_import: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Stable publish-later continuity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePublishLaterContinuityRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable continuity identity.
    pub continuity_id: String,
    /// Work-item detail ref.
    pub work_item_detail_id_ref: String,
    /// Continuity state.
    pub continuity_state: String,
    /// Queued action refs.
    pub queued_action_refs: Vec<String>,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Stable command record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable command identity.
    pub command_id: String,
    /// Command class.
    pub command_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when preview/dry-run is supported.
    pub preview_supported: bool,
    /// True when the command emits an audit event.
    pub emits_audit_event: bool,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// True when the command is actionable.
    pub actionable: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Support/export packet for the stable work-item lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the review.
    pub reopen_context_ref: String,
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
pub struct StableWorkItemInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Status transition inspected by this row.
    pub status_transition_id_ref: String,
    /// True when at least one work-item detail shows provider-authoritative state.
    pub provider_authoritative_state_visible: bool,
    /// True when at least one work-item detail shows local-draft state.
    pub local_draft_state_visible: bool,
    /// True when at least one work-item detail shows sync-pending state.
    pub sync_pending_state_visible: bool,
    /// True when at least one work-item detail shows offline-captured state.
    pub offline_captured_state_visible: bool,
    /// True when at least one transition sheet has confirm available.
    pub confirm_action_available: bool,
    /// True when at least one offline handoff survives restart.
    pub offline_handoff_survives_restart: bool,
    /// True when at least one offline handoff survives reconnect.
    pub offline_handoff_survives_reconnect: bool,
    /// True when at least one publish-later continuity is pending.
    pub publish_later_pending: bool,
    /// True when the review is actionable.
    pub actionable: bool,
    /// True when the review is invalidated by any reason.
    pub invalidated: bool,
    /// Number of work-item details.
    pub work_item_detail_count: usize,
    /// Number of transition sheets.
    pub transition_sheet_count: usize,
    /// Number of offline handoffs.
    pub offline_handoff_count: usize,
    /// Number of publish-later continuities.
    pub publish_later_continuity_count: usize,
    /// Number of commands.
    pub command_count: usize,
    /// True when at least one command supports preview.
    pub preview_capable: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable work-item status-transition packet consumed by review surfaces and
/// support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWorkItemStatusTransitionPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Review workspace summary copied from the beta packet.
    pub review_workspace: crate::workspace::ReviewWorkspaceRecord,
    /// Stable status-transition review record.
    pub status_transition: StableWorkItemStatusTransitionRecord,
    /// Work-item detail records.
    pub work_item_details: Vec<StableWorkItemDetailRecord>,
    /// Status-transition sheet records.
    pub transition_sheets: Vec<StableStatusTransitionSheetRecord>,
    /// Offline handoff records.
    pub offline_handoffs: Vec<StableOfflineHandoffRecord>,
    /// Publish-later continuity records.
    pub publish_later_continuities: Vec<StablePublishLaterContinuityRecord>,
    /// Command records.
    pub commands: Vec<StableWorkItemCommandRecord>,
    /// Support/export packet.
    pub support_export: StableWorkItemSupportExportPacket,
    /// Inspection row.
    pub inspection: StableWorkItemInspectionRecord,
}

impl StableWorkItemStatusTransitionPacket {
    /// Builds a stable work-item status-transition packet from a beta
    /// review-workspace packet and stabilization input.
    ///
    /// # Errors
    ///
    /// Returns [`StableWorkItemValidationError`] when the input violates a
    /// stabilization invariant.
    pub fn from_workspace_packet(
        input: StableWorkItemStatusTransitionInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
    ) -> Result<Self, StableWorkItemValidationError> {
        validate_input(&input, workspace_packet)?;

        let status_transition = status_transition_record(&input, workspace_packet);
        let work_item_details = input
            .work_item_details
            .iter()
            .map(|d| work_item_detail_record(d))
            .collect::<Vec<_>>();
        let transition_sheets = input
            .transition_sheets
            .iter()
            .map(|s| status_transition_sheet_record(s))
            .collect::<Vec<_>>();
        let offline_handoffs = input
            .offline_handoffs
            .iter()
            .map(|h| offline_handoff_record(h))
            .collect::<Vec<_>>();
        let publish_later_continuities = input
            .publish_later_continuities
            .iter()
            .map(|c| publish_later_continuity_record(c))
            .collect::<Vec<_>>();
        let commands = input
            .commands
            .iter()
            .map(|c| command_record(c, &status_transition))
            .collect::<Vec<_>>();
        let support_export = support_export_packet(
            &input.support_export,
            &status_transition,
            workspace_packet,
            &commands,
        );
        let inspection = inspection_record(
            &status_transition,
            &work_item_details,
            &transition_sheets,
            &offline_handoffs,
            &publish_later_continuities,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: STABLE_WORK_ITEM_STATUS_TRANSITION_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
                .to_string(),
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            status_transition,
            work_item_details,
            transition_sheets,
            offline_handoffs,
            publish_later_continuities,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable work-item invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableWorkItemValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), StableWorkItemValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_WORK_ITEM_STATUS_TRANSITION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_status_transition_record(&self.status_transition)?;
        for detail in &self.work_item_details {
            validate_work_item_detail_record(detail)?;
        }
        for sheet in &self.transition_sheets {
            validate_status_transition_sheet_record(sheet)?;
        }
        for handoff in &self.offline_handoffs {
            validate_offline_handoff_record(handoff)?;
        }
        for continuity in &self.publish_later_continuities {
            validate_publish_later_continuity_record(continuity)?;
        }
        for command in &self.commands {
            validate_command_record(command)?;
        }
        validate_support_export(&self.support_export)?;
        validate_inspection(&self.inspection, self)?;

        // Cross-record invariants
        let detail_ids: BTreeSet<&str> = self
            .work_item_details
            .iter()
            .map(|d| d.detail_id.as_str())
            .collect();

        for sheet in &self.transition_sheets {
            if !detail_ids.contains(sheet.work_item_detail_id_ref.as_str()) {
                return Err(stable_work_item_validation_error(format!(
                    "transition_sheet {} cites unknown work_item_detail_id_ref",
                    sheet.transition_sheet_id
                )));
            }
        }

        for _continuity in &self.publish_later_continuities {
            // Continuity state validation can be expanded here
        }

        // Offline handoff durability invariant
        for handoff in &self.offline_handoffs {
            if !handoff.survives_restart {
                return Err(stable_work_item_validation_error(format!(
                    "offline_handoff {} must survive_restart",
                    handoff.offline_handoff_id
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

    /// Returns true when the packet can be reopened from support export.
    pub fn restartable_from_support_export(&self) -> bool {
        self.inspection.offline_handoff_survives_restart
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable work-item operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableWorkItemError {
    /// Validation failed.
    Validation(StableWorkItemValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableWorkItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableWorkItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable work-item status-transition review.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableWorkItemValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableWorkItemValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableWorkItemValidationError {}

fn stable_work_item_validation_error(message: impl Into<String>) -> StableWorkItemValidationError {
    StableWorkItemValidationError {
        message: message.into(),
    }
}

// ---------------------------------------------------------------------------
// Builder / validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StableWorkItemStatusTransitionInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), StableWorkItemValidationError> {
    ensure_nonempty(&input.status_transition_id, "status_transition_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    for reason in &input.invalidation_reasons {
        ensure_token(
            STABLE_WORK_ITEM_INVALIDATION_REASONS,
            reason,
            "invalidation_reason",
        )?;
    }

    if input.work_item_details.is_empty() {
        return Err(stable_work_item_validation_error(
            "input must contain at least one work_item_detail".to_string(),
        ));
    }

    for detail in &input.work_item_details {
        ensure_nonempty(&detail.detail_id, "work_item_detail.detail_id")?;
        ensure_nonempty(
            &detail.canonical_work_item_id,
            "work_item_detail.canonical_work_item_id",
        )?;
        ensure_nonempty(
            &detail.provider_owned_id,
            "work_item_detail.provider_owned_id",
        )?;
    }

    for sheet in &input.transition_sheets {
        ensure_nonempty(
            &sheet.transition_sheet_id,
            "transition_sheet.transition_sheet_id",
        )?;
        ensure_nonempty(
            &sheet.work_item_detail_id_ref,
            "transition_sheet.work_item_detail_id_ref",
        )?;
    }

    for handoff in &input.offline_handoffs {
        ensure_nonempty(
            &handoff.offline_handoff_id,
            "offline_handoff.offline_handoff_id",
        )?;
        ensure_nonempty(&handoff.publish_target, "offline_handoff.publish_target")?;
        ensure_nonempty(&handoff.expires_at, "offline_handoff.expires_at")?;
    }

    for command in &input.commands {
        ensure_nonempty(&command.command_id, "command.command_id")?;
        ensure_nonempty(&command.command_class, "command.command_class")?;
    }

    let _ = workspace_packet;

    Ok(())
}

fn validate_status_transition_record(
    record: &StableWorkItemStatusTransitionRecord,
) -> Result<(), StableWorkItemValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_WORK_ITEM_STATUS_TRANSITION_RECORD_KIND,
        "record.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        "record.schema_version",
    )?;
    ensure_nonempty(
        &record.status_transition_id,
        "status_transition.status_transition_id",
    )?;
    ensure_nonempty(&record.generated_at, "status_transition.generated_at")?;
    ensure_nonempty(&record.summary_label, "status_transition.summary_label")?;

    for reason in &record.invalidation_reasons {
        ensure_token(
            STABLE_WORK_ITEM_INVALIDATION_REASONS,
            reason,
            "status_transition.invalidation_reason",
        )?;
    }

    Ok(())
}

fn validate_work_item_detail_record(
    record: &StableWorkItemDetailRecord,
) -> Result<(), StableWorkItemValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_WORK_ITEM_DETAIL_RECORD_KIND,
        "detail.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        "detail.schema_version",
    )?;
    ensure_nonempty(&record.detail_id, "detail.detail_id")?;
    ensure_nonempty(
        &record.canonical_work_item_id,
        "detail.canonical_work_item_id",
    )?;
    ensure_nonempty(&record.provider_owned_id, "detail.provider_owned_id")?;
    ensure_nonempty(&record.summary_label, "detail.summary_label")?;

    Ok(())
}

fn validate_status_transition_sheet_record(
    record: &StableStatusTransitionSheetRecord,
) -> Result<(), StableWorkItemValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_STATUS_TRANSITION_SHEET_RECORD_KIND,
        "sheet.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        "sheet.schema_version",
    )?;
    ensure_nonempty(&record.transition_sheet_id, "sheet.transition_sheet_id")?;
    ensure_nonempty(
        &record.work_item_detail_id_ref,
        "sheet.work_item_detail_id_ref",
    )?;
    ensure_nonempty(&record.summary_label, "sheet.summary_label")?;

    for effect in &record.previewed_side_effects {
        ensure_eq(
            effect.record_kind.as_str(),
            "review_stable_previewed_side_effect_record",
            "effect.record_kind",
        )?;
        ensure_nonempty(&effect.side_effect_id, "effect.side_effect_id")?;
        ensure_nonempty(&effect.side_effect_class, "effect.side_effect_class")?;
    }

    Ok(())
}

fn validate_offline_handoff_record(
    record: &StableOfflineHandoffRecord,
) -> Result<(), StableWorkItemValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_OFFLINE_HANDOFF_RECORD_KIND,
        "handoff.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        "handoff.schema_version",
    )?;
    ensure_nonempty(&record.offline_handoff_id, "handoff.offline_handoff_id")?;
    ensure_nonempty(&record.publish_target, "handoff.publish_target")?;
    ensure_nonempty(&record.expires_at, "handoff.expires_at")?;
    ensure_nonempty(&record.summary_label, "handoff.summary_label")?;

    Ok(())
}

fn validate_publish_later_continuity_record(
    record: &StablePublishLaterContinuityRecord,
) -> Result<(), StableWorkItemValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_PUBLISH_LATER_CONTINUITY_RECORD_KIND,
        "continuity.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        "continuity.schema_version",
    )?;
    ensure_nonempty(&record.continuity_id, "continuity.continuity_id")?;
    ensure_nonempty(&record.expires_at, "continuity.expires_at")?;
    ensure_nonempty(&record.summary_label, "continuity.summary_label")?;

    Ok(())
}

fn validate_command_record(
    record: &StableWorkItemCommandRecord,
) -> Result<(), StableWorkItemValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        STABLE_WORK_ITEM_COMMAND_RECORD_KIND,
        "command.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        "command.schema_version",
    )?;
    ensure_nonempty(&record.command_id, "command.command_id")?;
    ensure_nonempty(&record.command_class, "command.command_class")?;
    ensure_nonempty(&record.summary_label, "command.summary_label")?;

    Ok(())
}

fn validate_support_export(
    packet: &StableWorkItemSupportExportPacket,
) -> Result<(), StableWorkItemValidationError> {
    ensure_eq(
        packet.record_kind.as_str(),
        STABLE_WORK_ITEM_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export.record_kind",
    )?;
    ensure_eq_u32(
        packet.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        "support_export.schema_version",
    )?;
    ensure_nonempty(
        &packet.support_export_id,
        "support_export.support_export_id",
    )?;
    ensure_nonempty(
        &packet.reopen_context_ref,
        "support_export.reopen_context_ref",
    )?;
    ensure_nonempty(&packet.redaction_class, "support_export.redaction_class")?;
    ensure_nonempty(&packet.summary_label, "support_export.summary_label")?;

    Ok(())
}

fn validate_inspection(
    inspection: &StableWorkItemInspectionRecord,
    packet: &StableWorkItemStatusTransitionPacket,
) -> Result<(), StableWorkItemValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_WORK_ITEM_INSPECTION_RECORD_KIND,
        "inspection.record_kind",
    )?;
    ensure_eq_u32(
        inspection.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        "inspection.schema_version",
    )?;
    ensure_nonempty(
        &inspection.status_transition_id_ref,
        "inspection.status_transition_id_ref",
    )?;
    ensure_nonempty(&inspection.summary_label, "inspection.summary_label")?;

    if inspection.work_item_detail_count != packet.work_item_details.len() {
        return Err(stable_work_item_validation_error(
            "inspection.work_item_detail_count mismatch".to_string(),
        ));
    }
    if inspection.transition_sheet_count != packet.transition_sheets.len() {
        return Err(stable_work_item_validation_error(
            "inspection.transition_sheet_count mismatch".to_string(),
        ));
    }
    if inspection.offline_handoff_count != packet.offline_handoffs.len() {
        return Err(stable_work_item_validation_error(
            "inspection.offline_handoff_count mismatch".to_string(),
        ));
    }
    if inspection.publish_later_continuity_count != packet.publish_later_continuities.len() {
        return Err(stable_work_item_validation_error(
            "inspection.publish_later_continuity_count mismatch".to_string(),
        ));
    }
    if inspection.command_count != packet.commands.len() {
        return Err(stable_work_item_validation_error(
            "inspection.command_count mismatch".to_string(),
        ));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Record builders
// ---------------------------------------------------------------------------

fn status_transition_record(
    input: &StableWorkItemStatusTransitionInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> StableWorkItemStatusTransitionRecord {
    StableWorkItemStatusTransitionRecord {
        record_kind: STABLE_WORK_ITEM_STATUS_TRANSITION_RECORD_KIND.to_string(),
        schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
            .to_string(),
        status_transition_id: input.status_transition_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        work_item_detail_count: input.work_item_details.len(),
        transition_sheet_count: input.transition_sheets.len(),
        offline_handoff_count: input.offline_handoffs.len(),
        publish_later_continuity_count: input.publish_later_continuities.len(),
        invalidation_reasons: input.invalidation_reasons.clone(),
        actionable: input.invalidation_reasons.is_empty(),
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn work_item_detail_record(input: &StableWorkItemDetailInput) -> StableWorkItemDetailRecord {
    StableWorkItemDetailRecord {
        record_kind: STABLE_WORK_ITEM_DETAIL_RECORD_KIND.to_string(),
        schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
            .to_string(),
        detail_id: input.detail_id.clone(),
        canonical_work_item_id: input.canonical_work_item_id.clone(),
        provider_owned_id: input.provider_owned_id.clone(),
        write_mode_disclosure_class: input.write_mode_disclosure_class.clone(),
        row_posture_class: input.row_posture_class.clone(),
        freshness_class: input.freshness_class.clone(),
        provider_authoritative_state_visible: input.provider_authoritative_state_visible,
        local_draft_state_visible: input.local_draft_state_visible,
        sync_pending_state_visible: input.sync_pending_state_visible,
        offline_captured_state_visible: input.offline_captured_state_visible,
        mutation_mode: input.mutation_mode.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn status_transition_sheet_record(
    input: &StableStatusTransitionSheetInput,
) -> StableStatusTransitionSheetRecord {
    let previewed_side_effects = input
        .previewed_side_effects
        .iter()
        .map(|e| StablePreviewedSideEffectRecord {
            record_kind: "review_stable_previewed_side_effect_record".to_string(),
            schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
            side_effect_id: e.side_effect_id.clone(),
            side_effect_class: e.side_effect_class.clone(),
            target_scope: e.target_scope.clone(),
            summary_label: e.summary_label.clone(),
        })
        .collect();

    StableStatusTransitionSheetRecord {
        record_kind: STABLE_STATUS_TRANSITION_SHEET_RECORD_KIND.to_string(),
        schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
            .to_string(),
        transition_sheet_id: input.transition_sheet_id.clone(),
        work_item_detail_id_ref: input.work_item_detail_id_ref.clone(),
        previewed_side_effects,
        confirm_action_available: input.confirm_action_available,
        export_action_available: input.export_action_available,
        cancel_action_available: input.cancel_action_available,
        local_draft_preserved_on_failure: input.local_draft_preserved_on_failure,
        publish_later_continuity_ref: input.publish_later_continuity_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn offline_handoff_record(input: &StableOfflineHandoffInput) -> StableOfflineHandoffRecord {
    StableOfflineHandoffRecord {
        record_kind: STABLE_OFFLINE_HANDOFF_RECORD_KIND.to_string(),
        schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
            .to_string(),
        offline_handoff_id: input.offline_handoff_id.clone(),
        canonical_work_item_ids: input.canonical_work_item_ids.clone(),
        selected_evidence_refs: input.selected_evidence_refs.clone(),
        redaction_choices: input.redaction_choices.clone(),
        queued_actions: input.queued_actions.clone(),
        publish_target: input.publish_target.clone(),
        expires_at: input.expires_at.clone(),
        retry_semantics: input.retry_semantics.clone(),
        export_semantics: input.export_semantics.clone(),
        discard_semantics: input.discard_semantics.clone(),
        survives_restart: input.survives_restart,
        survives_reconnect: input.survives_reconnect,
        survives_export_import: input.survives_export_import,
        summary_label: input.summary_label.clone(),
    }
}

fn publish_later_continuity_record(
    input: &StablePublishLaterContinuityInput,
) -> StablePublishLaterContinuityRecord {
    StablePublishLaterContinuityRecord {
        record_kind: STABLE_PUBLISH_LATER_CONTINUITY_RECORD_KIND.to_string(),
        schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
            .to_string(),
        continuity_id: input.continuity_id.clone(),
        work_item_detail_id_ref: input.work_item_detail_id_ref.clone(),
        continuity_state: input.continuity_state.clone(),
        queued_action_refs: input.queued_action_refs.clone(),
        expires_at: input.expires_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn command_record(
    input: &StableWorkItemCommandInput,
    status_transition: &StableWorkItemStatusTransitionRecord,
) -> StableWorkItemCommandRecord {
    StableWorkItemCommandRecord {
        record_kind: STABLE_WORK_ITEM_COMMAND_RECORD_KIND.to_string(),
        schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
            .to_string(),
        command_id: input.command_id.clone(),
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

fn support_export_packet(
    input: &StableWorkItemSupportExportInput,
    _status_transition: &StableWorkItemStatusTransitionRecord,
    _workspace_packet: &ReviewWorkspaceBetaPacket,
    _commands: &[StableWorkItemCommandRecord],
) -> StableWorkItemSupportExportPacket {
    StableWorkItemSupportExportPacket {
        record_kind: STABLE_WORK_ITEM_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
            .to_string(),
        support_export_id: input.support_export_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/stable_work_item_status_transition_review.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn inspection_record(
    status_transition: &StableWorkItemStatusTransitionRecord,
    details: &[StableWorkItemDetailRecord],
    sheets: &[StableStatusTransitionSheetRecord],
    handoffs: &[StableOfflineHandoffRecord],
    continuities: &[StablePublishLaterContinuityRecord],
    commands: &[StableWorkItemCommandRecord],
    _support_export: &StableWorkItemSupportExportPacket,
) -> StableWorkItemInspectionRecord {
    let provider_authoritative_state_visible = details
        .iter()
        .any(|d| d.provider_authoritative_state_visible);
    let local_draft_state_visible = details.iter().any(|d| d.local_draft_state_visible);
    let sync_pending_state_visible = details.iter().any(|d| d.sync_pending_state_visible);
    let offline_captured_state_visible = details.iter().any(|d| d.offline_captured_state_visible);
    let confirm_action_available = sheets.iter().any(|s| s.confirm_action_available);
    let offline_handoff_survives_restart = handoffs.iter().any(|h| h.survives_restart);
    let offline_handoff_survives_reconnect = handoffs.iter().any(|h| h.survives_reconnect);
    let publish_later_pending = continuities
        .iter()
        .any(|c| c.continuity_state == "queued_pending");
    let preview_capable = commands.iter().any(|c| c.preview_supported);

    StableWorkItemInspectionRecord {
        record_kind: STABLE_WORK_ITEM_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF
            .to_string(),
        status_transition_id_ref: status_transition.status_transition_id.clone(),
        provider_authoritative_state_visible,
        local_draft_state_visible,
        sync_pending_state_visible,
        offline_captured_state_visible,
        confirm_action_available,
        offline_handoff_survives_restart,
        offline_handoff_survives_reconnect,
        publish_later_pending,
        actionable: status_transition.actionable,
        invalidated: !status_transition.invalidation_reasons.is_empty(),
        work_item_detail_count: details.len(),
        transition_sheet_count: sheets.len(),
        offline_handoff_count: handoffs.len(),
        publish_later_continuity_count: continuities.len(),
        command_count: commands.len(),
        preview_capable,
        summary_label: format!(
            "Work-item inspection: {} details, {} sheets, {} handoffs, {} continuities",
            details.len(),
            sheets.len(),
            handoffs.len(),
            continuities.len()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded data
// ---------------------------------------------------------------------------

/// Builds a seeded stable work-item status-transition packet for testing and
/// fixture generation.
pub fn seeded_stable_work_item_status_transition_packet(
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> StableWorkItemStatusTransitionPacket {
    let work_item_details = vec![
        StableWorkItemDetailInput {
            detail_id: "stable-work-item:detail:001".to_string(),
            canonical_work_item_id: "issue:github:payments:backend:42".to_string(),
            provider_owned_id: "GH_ISSUE_42".to_string(),
            write_mode_disclosure_class: "full_edit".to_string(),
            row_posture_class: "provider_authoritative".to_string(),
            freshness_class: "live_authoritative_fresh".to_string(),
            provider_authoritative_state_visible: true,
            local_draft_state_visible: false,
            sync_pending_state_visible: false,
            offline_captured_state_visible: false,
            mutation_mode: "publish_now".to_string(),
            summary_label: "Provider-authoritative issue with full edit".to_string(),
        },
        StableWorkItemDetailInput {
            detail_id: "stable-work-item:detail:002".to_string(),
            canonical_work_item_id: "issue:github:payments:frontend:77".to_string(),
            provider_owned_id: "GH_ISSUE_77".to_string(),
            write_mode_disclosure_class: "offline_capture_only".to_string(),
            row_posture_class: "offline_captured".to_string(),
            freshness_class: "unverifiable_provider_unreachable".to_string(),
            provider_authoritative_state_visible: false,
            local_draft_state_visible: true,
            sync_pending_state_visible: false,
            offline_captured_state_visible: true,
            mutation_mode: "publish_later".to_string(),
            summary_label: "Offline-captured issue awaiting reconnect".to_string(),
        },
    ];

    let transition_sheets = vec![StableStatusTransitionSheetInput {
        transition_sheet_id: "stable-work-item:sheet:001".to_string(),
        work_item_detail_id_ref: "stable-work-item:detail:001".to_string(),
        previewed_side_effects: vec![
            StablePreviewedSideEffectInput {
                side_effect_id: "side-effect:comment:001".to_string(),
                side_effect_class: "comment_create".to_string(),
                target_scope: "issue:github:payments:backend:42".to_string(),
                summary_label: "Create comment on issue 42".to_string(),
            },
            StablePreviewedSideEffectInput {
                side_effect_id: "side-effect:status:001".to_string(),
                side_effect_class: "status_change".to_string(),
                target_scope: "issue:github:payments:backend:42".to_string(),
                summary_label: "Change status to in-progress".to_string(),
            },
        ],
        confirm_action_available: true,
        export_action_available: true,
        cancel_action_available: true,
        local_draft_preserved_on_failure: true,
        publish_later_continuity_ref: None,
        summary_label: "Status transition sheet for issue 42".to_string(),
    }];

    let offline_handoffs = vec![StableOfflineHandoffInput {
        offline_handoff_id: "stable-work-item:handoff:001".to_string(),
        canonical_work_item_ids: vec!["issue:github:payments:frontend:77".to_string()],
        selected_evidence_refs: vec!["evidence:screenshot:001".to_string()],
        redaction_choices: vec!["redact_author_email".to_string()],
        queued_actions: vec!["status_change".to_string(), "label_add".to_string()],
        publish_target: "project:payments:frontend".to_string(),
        expires_at: "2026-06-10T09:55:00Z".to_string(),
        retry_semantics: "retry_with_backoff".to_string(),
        export_semantics: "export_safe".to_string(),
        discard_semantics: "discard_after_expiry".to_string(),
        survives_restart: true,
        survives_reconnect: true,
        survives_export_import: true,
        summary_label: "Durable offline handoff for issue 77".to_string(),
    }];

    let publish_later_continuities = vec![StablePublishLaterContinuityInput {
        continuity_id: "stable-work-item:continuity:001".to_string(),
        work_item_detail_id_ref: "stable-work-item:detail:002".to_string(),
        continuity_state: "queued_pending".to_string(),
        queued_action_refs: vec!["action:status_change:001".to_string()],
        expires_at: "2026-06-10T09:55:00Z".to_string(),
        summary_label: "Publish-later continuity for issue 77".to_string(),
    }];

    let commands = vec![
        StableWorkItemCommandInput {
            command_id: "stable-work-item:cmd:preview:001".to_string(),
            command_class: "preview_status_transition".to_string(),
            target_object_ref: "issue:github:payments:backend:42".to_string(),
            target_object_kind: "issue".to_string(),
            preview_supported: true,
            emits_audit_event: false,
            blocked_reasons: vec![],
            summary_label: "Preview status transition".to_string(),
        },
        StableWorkItemCommandInput {
            command_id: "stable-work-item:cmd:confirm:001".to_string(),
            command_class: "confirm_publish_now".to_string(),
            target_object_ref: "issue:github:payments:backend:42".to_string(),
            target_object_kind: "issue".to_string(),
            preview_supported: false,
            emits_audit_event: true,
            blocked_reasons: vec![],
            summary_label: "Confirm publish now".to_string(),
        },
        StableWorkItemCommandInput {
            command_id: "stable-work-item:cmd:handoff:001".to_string(),
            command_class: "export_offline_handoff".to_string(),
            target_object_ref: "issue:github:payments:frontend:77".to_string(),
            target_object_kind: "issue".to_string(),
            preview_supported: false,
            emits_audit_event: true,
            blocked_reasons: vec![],
            summary_label: "Export offline handoff".to_string(),
        },
    ];

    let input = StableWorkItemStatusTransitionInput {
        status_transition_id: "stable-work-item:m4:001".to_string(),
        packet_id: "stable-work-item-packet:m4:001".to_string(),
        generated_at: "2026-06-03T09:55:00Z".to_string(),
        work_item_details,
        transition_sheets,
        offline_handoffs,
        publish_later_continuities,
        commands,
        invalidation_reasons: vec![],
        summary_label: "Stable work-item status-transition review M4".to_string(),
        support_export: StableWorkItemSupportExportInput {
            support_export_id: "stable-work-item-support-export:m4:001".to_string(),
            reopen_context_ref: "stable-work-item:m4:001".to_string(),
            consumer_surfaces: STABLE_WORK_ITEM_CONSUMER_SURFACES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            redaction_class: "metadata_only".to_string(),
            summary_label: "Stable work-item support export".to_string(),
        },
    };

    StableWorkItemStatusTransitionPacket::from_workspace_packet(input, workspace_packet)
        .expect("seeded stable work-item packet must be valid")
}

// ---------------------------------------------------------------------------
// Audit helpers
// ---------------------------------------------------------------------------

/// Validates a stable work-item packet and returns typed defects on failure.
pub fn validate_stable_work_item_packet(
    packet: &StableWorkItemStatusTransitionPacket,
) -> Result<(), Vec<StableWorkItemValidationError>> {
    match packet.validate() {
        Ok(()) => Ok(()),
        Err(e) => Err(vec![e]),
    }
}

/// Recomputes defects for a stable work-item packet.
pub fn audit_stable_work_item_packet(
    packet: &StableWorkItemStatusTransitionPacket,
) -> Vec<StableWorkItemValidationError> {
    match packet.validate() {
        Ok(()) => vec![],
        Err(e) => vec![e],
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

fn ensure_eq(
    actual: &str,
    expected: &str,
    field: &str,
) -> Result<(), StableWorkItemValidationError> {
    if actual != expected {
        Err(stable_work_item_validation_error(format!(
            "{field} must be '{expected}', got '{actual}'"
        )))
    } else {
        Ok(())
    }
}

fn ensure_eq_u32(
    actual: u32,
    expected: u32,
    field: &str,
) -> Result<(), StableWorkItemValidationError> {
    if actual != expected {
        Err(stable_work_item_validation_error(format!(
            "{field} must be {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableWorkItemValidationError> {
    if value.trim().is_empty() {
        Err(stable_work_item_validation_error(format!(
            "{field} must be non-empty"
        )))
    } else {
        Ok(())
    }
}

fn ensure_token(
    vocabulary: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableWorkItemValidationError> {
    if !vocabulary.contains(&value) {
        Err(stable_work_item_validation_error(format!(
            "{field} '{value}' is not in the closed vocabulary"
        )))
    } else {
        Ok(())
    }
}
