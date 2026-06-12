//! Deferred-publish queue recovery packet for provider-backed work-item and
//! adjacent follow-up flows.
//!
//! This module composes the existing provider queue, work-item mutation review,
//! and provider scope-review contracts into one export-safe recovery packet for
//! publish-later continuity. The packet keeps one authoritative object per
//! deferred mutation so queue rows, durable local packets, activity-center
//! projections, and support exports all reuse the same object identity and
//! state vocabulary.
//!
//! The packet keeps four truths explicit:
//!
//! - deferred queue rows record queue identity, dependency order, freshness
//!   requirements, retry posture, conflict policy, and audit lineage;
//! - durable local packets survive restart, export, and support handoff when
//!   auth, provider health, validation, or redaction policy block a publish;
//! - the activity projection keeps `draft_only`, `queued_for_publish`,
//!   `blocked`, `stale_target`, `conflict_review_required`, and `published`
//!   states visibly distinct instead of collapsing them into one pending badge;
//! - replay never reuses stale target identity or stale effective scope, and
//!   high-impact mutations never auto-replay across changed boundaries.
//!
//! The boundary schema is
//! [`schemas/review/ship-deferred-publish-queue-recovery-packets.schema.json`](../../../../schemas/review/ship-deferred-publish-queue-recovery-packets.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_deferred_publish_queue_recovery_packets.md`](../../../../docs/review/m5/ship_deferred_publish_queue_recovery_packets.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/ship_deferred_publish_queue_recovery_packets/`](../../../../fixtures/review/m5/ship_deferred_publish_queue_recovery_packets/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fs;

use aureline_provider::{seeded_provider_scope_review_page, ProviderFamily};
use serde::{Deserialize, Serialize};

use crate::ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews::canonical_work_item_mutation_review_packet;

/// Stable record-kind tag carried by [`DeferredPublishQueueRecoveryPacket`].
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_RECORD_KIND: &str =
    "ship_deferred_publish_queue_recovery_packets";

/// Schema version for deferred-publish recovery packet records.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_SCHEMA_REF: &str =
    "schemas/review/ship-deferred-publish-queue-recovery-packets.schema.json";

/// Repo-relative path of the contract doc.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_DOC_REF: &str =
    "docs/review/m5/ship_deferred_publish_queue_recovery_packets.md";

/// Repo-relative path of the upstream work-item mutation review contract.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_MUTATION_REVIEW_CONTRACT_REF: &str =
    "schemas/review/ship-work-item-detail-headers-status-transition-sheets-comment-publish-review-and-offline-handoff-packets-with-side-effect-previews.schema.json";

/// Repo-relative path of the provider scope review contract.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_SCOPE_REVIEW_CONTRACT_REF: &str =
    "schemas/providers/provider_scope_review.schema.json";

/// Repo-relative path of the provider work-item transition contract.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_TRANSITION_CONTRACT_REF: &str =
    "schemas/work_items/transition_review.schema.json";

/// Repo-relative path of the provider work-item sync contract.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_SYNC_CONTRACT_REF: &str =
    "schemas/work_items/work_item_sync.schema.json";

/// Repo-relative path of the provider queue contract.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_QUEUE_CONTRACT_REF: &str =
    "schemas/providers/deferred_publish_queue_item.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_FIXTURE_DIR: &str =
    "fixtures/review/m5/ship_deferred_publish_queue_recovery_packets";

/// Repo-relative path of the checked support-export artifact.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_ARTIFACT_REF: &str =
    "artifacts/review/m5/ship_deferred_publish_queue_recovery_packets/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DEFERRED_PUBLISH_QUEUE_RECOVERY_SUMMARY_REF: &str =
    "artifacts/review/m5/ship_deferred_publish_queue_recovery_packets.md";

/// Shared deferred-publish lifecycle state that queue, local-packet, activity,
/// and support-export surfaces must preserve verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredPublishLifecycleState {
    /// Work exists as a durable local draft only.
    DraftOnly,
    /// Work is queued and ready for a later publish.
    QueuedForPublish,
    /// Work is blocked on an authority, policy, or provider gate.
    Blocked,
    /// Work cannot replay until target freshness is re-established.
    StaleTarget,
    /// Work conflicts with fresher provider truth and needs reconcile review.
    ConflictReviewRequired,
    /// Work drained and the provider committed it.
    Published,
}

impl DeferredPublishLifecycleState {
    /// Returns the stable token recorded across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DraftOnly => "draft_only",
            Self::QueuedForPublish => "queued_for_publish",
            Self::Blocked => "blocked",
            Self::StaleTarget => "stale_target",
            Self::ConflictReviewRequired => "conflict_review_required",
            Self::Published => "published",
        }
    }

    /// Returns true when replay preconditions matter for this state.
    pub const fn requires_replay_review(self) -> bool {
        matches!(
            self,
            Self::QueuedForPublish
                | Self::Blocked
                | Self::StaleTarget
                | Self::ConflictReviewRequired
        )
    }
}

/// Typed reason why a publish-later object is not currently provider committed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredPublishBlockReasonClass {
    /// Current credentials or grants deny the write.
    AuthDenied,
    /// Provider or route is unavailable.
    ProviderOutage,
    /// Validation or provider-side conflict rejected the attempted publish.
    ValidationConflict,
    /// Redaction policy blocked the outbound publish.
    RedactionPolicyBlocked,
    /// Target identity or freshness floor must be repaired before replay.
    FreshTargetRequired,
    /// Effective scope or reviewed boundary changed.
    ChangedBoundaryNeedsReview,
}

impl DeferredPublishBlockReasonClass {
    /// Returns the stable token recorded across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthDenied => "auth_denied",
            Self::ProviderOutage => "provider_outage",
            Self::ValidationConflict => "validation_conflict",
            Self::RedactionPolicyBlocked => "redaction_policy_blocked",
            Self::FreshTargetRequired => "fresh_target_required",
            Self::ChangedBoundaryNeedsReview => "changed_boundary_needs_review",
        }
    }
}

/// Freshness requirement bound to a deferred queue row before replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredPublishFreshnessRequirementClass {
    /// A purely local draft has no live-provider freshness requirement yet.
    NotApplicableLocalDraft,
    /// Replay requires the last reviewed target identity to still match.
    CurrentTargetIdentityRequired,
    /// Published rows still name the last verified grace posture.
    FreshWithinGraceWindow,
    /// Replay requires a new provider refresh before any mutation.
    ProviderRefreshRequiredBeforeReplay,
}

impl DeferredPublishFreshnessRequirementClass {
    /// Returns the stable token recorded across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableLocalDraft => "not_applicable_local_draft",
            Self::CurrentTargetIdentityRequired => "current_target_identity_required",
            Self::FreshWithinGraceWindow => "fresh_within_grace_window",
            Self::ProviderRefreshRequiredBeforeReplay => "provider_refresh_required_before_replay",
        }
    }
}

/// Retry posture preserved on deferred queue rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredPublishRetryPostureClass {
    /// Retry is not meaningful for a draft-only or already-published row.
    NotApplicableDraftOrPublished,
    /// Retry is possible after provider health or connectivity returns.
    ManualRetryAfterProviderRecovery,
    /// Retry is possible only after fresh auth and scope review.
    RetryAfterReauthAndScopeReview,
    /// Retry is possible only after target freshness is re-established.
    RetryAfterFreshnessRefresh,
    /// Retry is possible only after explicit conflict reconcile review.
    RetryAfterConflictReconcile,
    /// Retry is possible only after redaction review clears the packet.
    RetryAfterRedactionReview,
    /// Replay must not auto-run across a changed boundary.
    NoAutomaticReplayAcrossChangedBoundaries,
    /// Safe continuation exists only by opening the provider directly.
    OpenExternalOnly,
}

impl DeferredPublishRetryPostureClass {
    /// Returns the stable token recorded across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableDraftOrPublished => "not_applicable_draft_or_published",
            Self::ManualRetryAfterProviderRecovery => "manual_retry_after_provider_recovery",
            Self::RetryAfterReauthAndScopeReview => "retry_after_reauth_and_scope_review",
            Self::RetryAfterFreshnessRefresh => "retry_after_freshness_refresh",
            Self::RetryAfterConflictReconcile => "retry_after_conflict_reconcile",
            Self::RetryAfterRedactionReview => "retry_after_redaction_review",
            Self::NoAutomaticReplayAcrossChangedBoundaries => {
                "no_automatic_replay_across_changed_boundaries"
            }
            Self::OpenExternalOnly => "open_external_only",
        }
    }
}

/// Conflict policy that guards later replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredPublishConflictPolicyClass {
    /// No compare/reconcile policy applies yet.
    NotApplicable,
    /// Provider and local truth must be compared before replay.
    CompareAndReconcileBeforeReplay,
    /// Provider version must be reviewed and explicitly re-accepted.
    ProviderVersionReviewRequired,
    /// Local packet must be rebased on the refreshed target.
    RebaseLocalPacketBeforeReplay,
    /// Policy leaves only export or discard as safe continuations.
    ExportOrDiscardOnly,
}

impl DeferredPublishConflictPolicyClass {
    /// Returns the stable token recorded across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::CompareAndReconcileBeforeReplay => "compare_and_reconcile_before_replay",
            Self::ProviderVersionReviewRequired => "provider_version_review_required",
            Self::RebaseLocalPacketBeforeReplay => "rebase_local_packet_before_replay",
            Self::ExportOrDiscardOnly => "export_or_discard_only",
        }
    }
}

/// Durable local packet kind preserved for deferred publish recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredPublishLocalPacketKind {
    /// Work-item status or field update.
    WorkItemStatusUpdate,
    /// Work-item or review-adjacent comment publish.
    CommentPublish,
    /// Incident or outage annotation.
    IncidentAnnotation,
    /// Provider-backed change flow or release follow-up.
    ProviderBackedChangeFlow,
}

impl DeferredPublishLocalPacketKind {
    /// Returns the stable token recorded across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkItemStatusUpdate => "work_item_status_update",
            Self::CommentPublish => "comment_publish",
            Self::IncidentAnnotation => "incident_annotation",
            Self::ProviderBackedChangeFlow => "provider_backed_change_flow",
        }
    }
}

/// Stable action class exposed by the authoritative deferred-publish object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredPublishActionClass {
    /// Retry the deferred publish through reviewed replay.
    Retry,
    /// Discard the durable local packet or queue item.
    Discard,
    /// Export a redaction-safe packet for support or handoff.
    ExportPacket,
    /// Open the external provider or browser handoff route.
    OpenExternal,
}

impl DeferredPublishActionClass {
    /// Returns the stable token recorded across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::Discard => "discard",
            Self::ExportPacket => "export_packet",
            Self::OpenExternal => "open_external",
        }
    }
}

/// One dependency preserved on a deferred-publish queue row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishDependencySummary {
    /// Zero-based order index.
    pub dependency_order_index: usize,
    /// Stable dependency class token.
    pub dependency_class_token: String,
    /// Stable dependency state token.
    pub dependency_state_token: String,
    /// Redaction-safe dependency rationale.
    pub rationale_summary: String,
    /// Optional linked record reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_record_ref: Option<String>,
}

/// One stable action route on the authoritative deferred-publish object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishActionRoute {
    /// Stable action id.
    pub action_id: String,
    /// Action class.
    pub action_class: DeferredPublishActionClass,
    /// Human-readable label.
    pub label: String,
    /// Opaque route or command reference.
    pub route_ref: String,
    /// True when the action is currently enabled.
    pub enabled: bool,
    /// Optional reason when a disabled action remains visible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_label: Option<String>,
}

/// Queue row preserved for deferred publish recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishQueueRow {
    /// Stable queue row id.
    pub row_id: String,
    /// Stable queue id.
    pub queue_id: String,
    /// Canonical object identity shared across surfaces.
    pub canonical_object_ref: String,
    /// Reviewable object label.
    pub canonical_object_label: String,
    /// Provider family owning the target object.
    pub provider_family: ProviderFamily,
    /// Optional work-item detail header ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_item_detail_record_id_ref: Option<String>,
    /// Optional work-item mutation review row ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_review_id_ref: Option<String>,
    /// Scope-review resolution ref required for replay.
    pub scope_review_resolution_id_ref: String,
    /// Durable local packet bound to this authoritative object.
    pub local_packet_id_ref: String,
    /// Optional provider queue record ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_ref: Option<String>,
    /// Optional browser handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Optional offline handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_offline_handoff_packet_ref: Option<String>,
    /// Shared lifecycle state.
    pub lifecycle_state: DeferredPublishLifecycleState,
    /// Optional block reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_class: Option<DeferredPublishBlockReasonClass>,
    /// Required target freshness posture before replay.
    pub target_freshness_requirement_class: DeferredPublishFreshnessRequirementClass,
    /// Stable retry posture.
    pub retry_posture_class: DeferredPublishRetryPostureClass,
    /// Stable conflict policy.
    pub conflict_policy_class: DeferredPublishConflictPolicyClass,
    /// Ordered dependencies that must clear before replay.
    pub dependency_chain: Vec<DeferredPublishDependencySummary>,
    /// Audit refs preserved for admission and replay review.
    pub audit_event_refs: Vec<String>,
    /// Replay requires fresh target identity.
    pub replay_requires_fresh_target_identity: bool,
    /// Replay requires current effective scope.
    pub replay_requires_current_effective_scope: bool,
    /// Replay must be re-reviewed because the boundary changed.
    pub changed_boundary_review_required: bool,
    /// High-impact mutations never auto-replay.
    pub high_impact_provider_mutation: bool,
    /// Whether replay may ever auto-run without user review.
    pub auto_replay_allowed: bool,
    /// Stable action routes preserved on the authoritative object.
    pub action_routes: Vec<DeferredPublishActionRoute>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Durable local packet preserved alongside the queue row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishLocalPacketRow {
    /// Stable packet id.
    pub packet_id: String,
    /// Packet kind.
    pub packet_kind: DeferredPublishLocalPacketKind,
    /// Canonical object identity shared across surfaces.
    pub canonical_object_ref: String,
    /// Reviewable object label.
    pub canonical_object_label: String,
    /// Shared lifecycle state.
    pub lifecycle_state: DeferredPublishLifecycleState,
    /// Optional block reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_class: Option<DeferredPublishBlockReasonClass>,
    /// Optional queue id when the packet is queued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_queue_id_ref: Option<String>,
    /// Optional browser handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Optional offline handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_offline_handoff_packet_ref: Option<String>,
    /// Redaction class token.
    pub redaction_class_token: String,
    /// Packet survives restart.
    pub persisted_across_restart: bool,
    /// Packet is safe for support handoff by default.
    pub export_safe_support_handoff: bool,
    /// Audit refs preserved for packet creation and replay review.
    pub audit_event_refs: Vec<String>,
    /// Stable action routes preserved on the authoritative object.
    pub action_routes: Vec<DeferredPublishActionRoute>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Activity projection row consumed by shell activity surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishActivityProjectionRow {
    /// Stable activity projection row id.
    pub row_id: String,
    /// Canonical object identity shared across surfaces.
    pub canonical_object_ref: String,
    /// Queue row ref.
    pub queue_row_id_ref: String,
    /// Local packet ref.
    pub local_packet_id_ref: String,
    /// Shared lifecycle state.
    pub lifecycle_state: DeferredPublishLifecycleState,
    /// Optional block reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_class: Option<DeferredPublishBlockReasonClass>,
    /// Retry posture shown by the activity row.
    pub retry_posture_class: DeferredPublishRetryPostureClass,
    /// Conflict policy shown by the activity row.
    pub conflict_policy_class: DeferredPublishConflictPolicyClass,
    /// Exact reopen target for the authoritative object.
    pub exact_reopen_ref: String,
    /// Support export row ref for the same object.
    pub support_export_row_ref: String,
    /// Stable phase label that keeps activity rows distinct.
    pub phase_label: String,
    /// Reviewable summary label.
    pub summary_label: String,
    /// Reviewable detail label.
    pub detail_label: String,
}

/// Support/export summary row for the authoritative deferred-publish object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishSupportExportRow {
    /// Stable support row id.
    pub row_id: String,
    /// Canonical object identity shared across surfaces.
    pub canonical_object_ref: String,
    /// Queue row ref.
    pub queue_row_id_ref: String,
    /// Local packet ref.
    pub local_packet_id_ref: String,
    /// Shared lifecycle state.
    pub lifecycle_state: DeferredPublishLifecycleState,
    /// Optional block reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_class: Option<DeferredPublishBlockReasonClass>,
    /// Target freshness requirement.
    pub target_freshness_requirement_class: DeferredPublishFreshnessRequirementClass,
    /// Retry posture.
    pub retry_posture_class: DeferredPublishRetryPostureClass,
    /// Conflict policy.
    pub conflict_policy_class: DeferredPublishConflictPolicyClass,
    /// Stable action ids exported for this object.
    pub action_route_ids: Vec<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Redaction-safe support export for deferred publish recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishRecoverySupportExport {
    /// Stable export id.
    pub export_id: String,
    /// Support rows.
    pub rows: Vec<DeferredPublishSupportExportRow>,
    /// Consumer surfaces expected to read the export.
    pub consumer_surfaces: Vec<String>,
    /// Guardrail: raw provider material never leaves by default.
    pub raw_provider_material_excluded: bool,
}

/// Trust-review assertions for the deferred-publish recovery lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishRecoveryTrustReview {
    /// Queue rows preserve queue id, dependencies, freshness, retry, and conflict truth.
    pub queue_rows_preserve_replay_contract: bool,
    /// Durable local packets survive restart and export.
    pub durable_local_packets_survive_restart_and_export: bool,
    /// Auth, outage, validation, and redaction blocks preserve intent.
    pub blocked_publish_preserves_intent_and_actions: bool,
    /// Replay requires fresh target identity and current scope.
    pub replay_requires_fresh_target_and_scope: bool,
    /// High-impact actions never auto-replay across changed boundaries.
    pub high_impact_actions_never_auto_replay: bool,
}

/// Cross-surface consumer assertions for the deferred-publish recovery lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishRecoveryConsumerProjection {
    /// Queue rows preserve shared lifecycle vocabulary.
    pub queue_rows_use_shared_lifecycle_vocabulary: bool,
    /// Activity rows preserve shared lifecycle vocabulary.
    pub activity_rows_use_shared_lifecycle_vocabulary: bool,
    /// Support export preserves shared lifecycle vocabulary.
    pub support_export_uses_shared_lifecycle_vocabulary: bool,
    /// Queue, local packet, activity, and support rows share object identity.
    pub authoritative_object_identity_is_shared: bool,
    /// One authoritative object exposes retry, discard, export, and open-external actions.
    pub authoritative_object_keeps_stable_actions: bool,
}

/// Proof-freshness metadata for this packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishRecoveryProofFreshness {
    /// Allowed proof staleness window in hours.
    pub proof_freshness_slo_hours: u32,
    /// Last proof refresh timestamp.
    pub last_proof_refresh: String,
    /// Whether stale proof auto-narrows this lane.
    pub auto_narrow_on_stale: bool,
}

/// Stable deferred-publish recovery packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredPublishQueueRecoveryPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewable surface label.
    pub surface_label: String,
    /// Queue rows carried by the packet.
    pub queue_rows: Vec<DeferredPublishQueueRow>,
    /// Durable local packets carried by the packet.
    pub local_packets: Vec<DeferredPublishLocalPacketRow>,
    /// Activity projection rows.
    pub activity_rows: Vec<DeferredPublishActivityProjectionRow>,
    /// Support export projection.
    pub support_export: DeferredPublishRecoverySupportExport,
    /// Trust-review assertions.
    pub trust_review: DeferredPublishRecoveryTrustReview,
    /// Consumer assertions.
    pub consumer_projection: DeferredPublishRecoveryConsumerProjection,
    /// Proof freshness metadata.
    pub proof_freshness: DeferredPublishRecoveryProofFreshness,
    /// Source contracts quoted by this packet.
    pub source_contract_refs: Vec<String>,
    /// Redaction class token for the packet.
    pub redaction_class_token: String,
    /// Minted timestamp.
    pub minted_at: String,
}

impl DeferredPublishQueueRecoveryPacket {
    /// Validates the deferred-publish recovery invariants.
    pub fn validate(&self) -> Vec<DeferredPublishQueueRecoveryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DEFERRED_PUBLISH_QUEUE_RECOVERY_RECORD_KIND {
            violations.push(DeferredPublishQueueRecoveryViolation::PacketContractInvalid(
                "record_kind".to_owned(),
            ));
        }
        if self.schema_version != DEFERRED_PUBLISH_QUEUE_RECOVERY_SCHEMA_VERSION {
            violations.push(DeferredPublishQueueRecoveryViolation::PacketContractInvalid(
                "schema_version".to_owned(),
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DeferredPublishQueueRecoveryViolation::PacketContractInvalid(
                "required_packet_field".to_owned(),
            ));
        }
        if self.queue_rows.is_empty() || self.local_packets.is_empty() || self.activity_rows.is_empty()
        {
            violations.push(DeferredPublishQueueRecoveryViolation::CoverageMissing(
                "rows_missing".to_owned(),
            ));
        }
        if self.source_contract_refs != canonical_source_contract_refs() {
            violations.push(DeferredPublishQueueRecoveryViolation::PacketContractInvalid(
                "source_contract_refs".to_owned(),
            ));
        }
        if !self.support_export.raw_provider_material_excluded {
            violations.push(DeferredPublishQueueRecoveryViolation::SupportExportUnsafe(
                "raw_provider_material_excluded".to_owned(),
            ));
        }

        let mut object_refs = BTreeSet::new();
        let mut queue_row_ids = BTreeSet::new();
        let mut packet_ids = BTreeSet::new();
        let mut activity_ids = BTreeSet::new();
        let mut action_coverage = BTreeSet::new();
        let mut state_coverage = BTreeSet::new();
        let mut block_reason_coverage = BTreeSet::new();
        let packet_by_object = self
            .local_packets
            .iter()
            .map(|packet| (packet.canonical_object_ref.as_str(), packet))
            .collect::<BTreeMap<_, _>>();
        let activity_by_queue = self
            .activity_rows
            .iter()
            .map(|row| (row.queue_row_id_ref.as_str(), row))
            .collect::<BTreeMap<_, _>>();
        let export_by_queue = self
            .support_export
            .rows
            .iter()
            .map(|row| (row.queue_row_id_ref.as_str(), row))
            .collect::<BTreeMap<_, _>>();

        for queue_row in &self.queue_rows {
            if !queue_row_ids.insert(queue_row.row_id.as_str()) {
                violations.push(DeferredPublishQueueRecoveryViolation::DuplicateId(
                    queue_row.row_id.clone(),
                ));
            }
            if !object_refs.insert(queue_row.canonical_object_ref.as_str()) {
                violations.push(DeferredPublishQueueRecoveryViolation::DuplicateCanonicalObject(
                    queue_row.canonical_object_ref.clone(),
                ));
            }
            if queue_row.queue_id.trim().is_empty()
                || queue_row.canonical_object_ref.trim().is_empty()
                || queue_row.canonical_object_label.trim().is_empty()
                || queue_row.scope_review_resolution_id_ref.trim().is_empty()
                || queue_row.local_packet_id_ref.trim().is_empty()
                || queue_row.summary_label.trim().is_empty()
                || queue_row.audit_event_refs.is_empty()
            {
                violations.push(DeferredPublishQueueRecoveryViolation::QueueRowIncomplete(
                    queue_row.row_id.clone(),
                ));
            }
            if !packet_by_object.contains_key(queue_row.canonical_object_ref.as_str()) {
                violations.push(DeferredPublishQueueRecoveryViolation::MissingLocalPacket(
                    queue_row.canonical_object_ref.clone(),
                ));
            }
            if !activity_by_queue.contains_key(queue_row.row_id.as_str()) {
                violations.push(DeferredPublishQueueRecoveryViolation::MissingActivityProjection(
                    queue_row.row_id.clone(),
                ));
            }
            if !export_by_queue.contains_key(queue_row.row_id.as_str()) {
                violations.push(DeferredPublishQueueRecoveryViolation::MissingSupportExportRow(
                    queue_row.row_id.clone(),
                ));
            }
            if queue_row.lifecycle_state == DeferredPublishLifecycleState::QueuedForPublish
                && queue_row.linked_publish_later_queue_item_ref.is_none()
            {
                violations.push(DeferredPublishQueueRecoveryViolation::QueueRowMissingQueueRef(
                    queue_row.row_id.clone(),
                ));
            }
            if queue_row.lifecycle_state.requires_replay_review()
                && (!queue_row.replay_requires_fresh_target_identity
                    || !queue_row.replay_requires_current_effective_scope)
            {
                violations.push(
                    DeferredPublishQueueRecoveryViolation::ReplayPrerequisitesMissing(
                        queue_row.row_id.clone(),
                    ),
                );
            }
            if (queue_row.high_impact_provider_mutation || queue_row.changed_boundary_review_required)
                && queue_row.auto_replay_allowed
            {
                violations.push(
                    DeferredPublishQueueRecoveryViolation::UnsafeAutomaticReplay(
                        queue_row.row_id.clone(),
                    ),
                );
            }
            if !dependency_order_is_strict(&queue_row.dependency_chain) {
                violations.push(DeferredPublishQueueRecoveryViolation::DependencyOrderInvalid(
                    queue_row.row_id.clone(),
                ));
            }
            if !has_all_action_classes(&queue_row.action_routes) {
                violations.push(DeferredPublishQueueRecoveryViolation::StableActionSetMissing(
                    queue_row.row_id.clone(),
                ));
            }
            for route in &queue_row.action_routes {
                action_coverage.insert(route.action_class);
            }
            state_coverage.insert(queue_row.lifecycle_state);
            if let Some(reason) = queue_row.block_reason_class {
                block_reason_coverage.insert(reason);
            }
        }

        for packet in &self.local_packets {
            if !packet_ids.insert(packet.packet_id.as_str()) {
                violations.push(DeferredPublishQueueRecoveryViolation::DuplicateId(
                    packet.packet_id.clone(),
                ));
            }
            if packet.canonical_object_ref.trim().is_empty()
                || packet.canonical_object_label.trim().is_empty()
                || packet.redaction_class_token.trim().is_empty()
                || packet.summary_label.trim().is_empty()
                || packet.audit_event_refs.is_empty()
                || !packet.persisted_across_restart
                || !packet.export_safe_support_handoff
            {
                violations.push(DeferredPublishQueueRecoveryViolation::LocalPacketIncomplete(
                    packet.packet_id.clone(),
                ));
            }
            if !has_all_action_classes(&packet.action_routes) {
                violations.push(DeferredPublishQueueRecoveryViolation::StableActionSetMissing(
                    packet.packet_id.clone(),
                ));
            }
        }

        for activity in &self.activity_rows {
            if !activity_ids.insert(activity.row_id.as_str()) {
                violations.push(DeferredPublishQueueRecoveryViolation::DuplicateId(
                    activity.row_id.clone(),
                ));
            }
            if activity.canonical_object_ref.trim().is_empty()
                || activity.queue_row_id_ref.trim().is_empty()
                || activity.local_packet_id_ref.trim().is_empty()
                || activity.exact_reopen_ref.trim().is_empty()
                || activity.support_export_row_ref.trim().is_empty()
                || activity.phase_label.trim().is_empty()
                || activity.summary_label.trim().is_empty()
                || activity.detail_label.trim().is_empty()
            {
                violations.push(DeferredPublishQueueRecoveryViolation::ActivityProjectionIncomplete(
                    activity.row_id.clone(),
                ));
            }
            if activity.phase_label != activity.lifecycle_state.as_str() {
                violations.push(DeferredPublishQueueRecoveryViolation::ActivityPhaseDrift(
                    activity.row_id.clone(),
                ));
            }
        }

        for row in &self.support_export.rows {
            if row.row_id.trim().is_empty()
                || row.canonical_object_ref.trim().is_empty()
                || row.queue_row_id_ref.trim().is_empty()
                || row.local_packet_id_ref.trim().is_empty()
                || row.summary_label.trim().is_empty()
                || row.action_route_ids.len() != 4
            {
                violations.push(DeferredPublishQueueRecoveryViolation::SupportExportUnsafe(
                    row.row_id.clone(),
                ));
            }
        }

        if action_coverage.len() != 4 {
            violations.push(DeferredPublishQueueRecoveryViolation::CoverageMissing(
                "stable_action_coverage".to_owned(),
            ));
        }
        if state_coverage
            != BTreeSet::from([
                DeferredPublishLifecycleState::DraftOnly,
                DeferredPublishLifecycleState::QueuedForPublish,
                DeferredPublishLifecycleState::Blocked,
                DeferredPublishLifecycleState::StaleTarget,
                DeferredPublishLifecycleState::ConflictReviewRequired,
                DeferredPublishLifecycleState::Published,
            ])
        {
            violations.push(DeferredPublishQueueRecoveryViolation::CoverageMissing(
                "lifecycle_state_coverage".to_owned(),
            ));
        }
        for required in [
            DeferredPublishBlockReasonClass::AuthDenied,
            DeferredPublishBlockReasonClass::ProviderOutage,
            DeferredPublishBlockReasonClass::ValidationConflict,
            DeferredPublishBlockReasonClass::RedactionPolicyBlocked,
        ] {
            if !block_reason_coverage.contains(&required) {
                violations.push(DeferredPublishQueueRecoveryViolation::CoverageMissing(
                    required.as_str().to_owned(),
                ));
            }
        }

        if !self.trust_review.queue_rows_preserve_replay_contract
            || !self
                .trust_review
                .durable_local_packets_survive_restart_and_export
            || !self
                .trust_review
                .blocked_publish_preserves_intent_and_actions
            || !self.trust_review.replay_requires_fresh_target_and_scope
            || !self.trust_review.high_impact_actions_never_auto_replay
        {
            violations.push(DeferredPublishQueueRecoveryViolation::TrustReviewInvalid);
        }
        if !self
            .consumer_projection
            .queue_rows_use_shared_lifecycle_vocabulary
            || !self
                .consumer_projection
                .activity_rows_use_shared_lifecycle_vocabulary
            || !self
                .consumer_projection
                .support_export_uses_shared_lifecycle_vocabulary
            || !self
                .consumer_projection
                .authoritative_object_identity_is_shared
            || !self
                .consumer_projection
                .authoritative_object_keeps_stable_actions
        {
            violations.push(DeferredPublishQueueRecoveryViolation::ConsumerProjectionInvalid);
        }

        violations
    }

    /// Renders a Markdown summary suitable for checked artifacts.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Deferred publish queue recovery packet\n\n");
        out.push_str(&format!("Packet id: `{}`\n\n", self.packet_id));
        out.push_str(&format!("Surface: {}\n\n", self.surface_label));
        out.push_str("## Queue rows\n\n");
        for row in &self.queue_rows {
            out.push_str(&format!(
                "- `{}`: `{}` / `{}` / `{}` / `{}`\n",
                row.canonical_object_ref,
                row.lifecycle_state.as_str(),
                row.target_freshness_requirement_class.as_str(),
                row.retry_posture_class.as_str(),
                row.conflict_policy_class.as_str(),
            ));
        }
        out.push_str("\n## Support export\n\n");
        out.push_str(&format!(
            "- rows: {}\n- consumers: {}\n- raw provider material excluded: {}\n",
            self.support_export.rows.len(),
            self.support_export.consumer_surfaces.join(", "),
            self.support_export.raw_provider_material_excluded,
        ));
        out
    }
}

/// Validation issue emitted while checking the deferred-publish recovery packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeferredPublishQueueRecoveryViolation {
    /// Packet-level contract data is invalid.
    PacketContractInvalid(String),
    /// Required coverage is missing.
    CoverageMissing(String),
    /// A duplicate id was found.
    DuplicateId(String),
    /// Two queue rows reused the same canonical object identity.
    DuplicateCanonicalObject(String),
    /// A queue row is incomplete.
    QueueRowIncomplete(String),
    /// A local packet is incomplete.
    LocalPacketIncomplete(String),
    /// An activity projection row is incomplete.
    ActivityProjectionIncomplete(String),
    /// The queue row does not cite its queue ref.
    QueueRowMissingQueueRef(String),
    /// Replay prerequisites are missing.
    ReplayPrerequisitesMissing(String),
    /// High-impact replay could auto-run unsafely.
    UnsafeAutomaticReplay(String),
    /// Dependency order drifted.
    DependencyOrderInvalid(String),
    /// One of the four stable action routes is missing.
    StableActionSetMissing(String),
    /// A queue row has no matching local packet.
    MissingLocalPacket(String),
    /// A queue row has no matching activity row.
    MissingActivityProjection(String),
    /// A queue row has no matching support-export row.
    MissingSupportExportRow(String),
    /// Activity row phase drifted from the lifecycle state.
    ActivityPhaseDrift(String),
    /// Support export is unsafe or incomplete.
    SupportExportUnsafe(String),
    /// Trust review assertions drifted.
    TrustReviewInvalid,
    /// Consumer projection assertions drifted.
    ConsumerProjectionInvalid,
}

impl fmt::Display for DeferredPublishQueueRecoveryViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketContractInvalid(field) => {
                write!(f, "deferred-publish recovery packet contract invalid: {field}")
            }
            Self::CoverageMissing(field) => {
                write!(f, "deferred-publish recovery coverage missing: {field}")
            }
            Self::DuplicateId(id) => write!(f, "duplicate deferred-publish recovery id: {id}"),
            Self::DuplicateCanonicalObject(id) => {
                write!(f, "duplicate deferred-publish canonical object: {id}")
            }
            Self::QueueRowIncomplete(id) => write!(f, "queue row incomplete: {id}"),
            Self::LocalPacketIncomplete(id) => write!(f, "local packet incomplete: {id}"),
            Self::ActivityProjectionIncomplete(id) => {
                write!(f, "activity projection incomplete: {id}")
            }
            Self::QueueRowMissingQueueRef(id) => {
                write!(f, "queued row missing queue ref: {id}")
            }
            Self::ReplayPrerequisitesMissing(id) => {
                write!(f, "replay prerequisites missing: {id}")
            }
            Self::UnsafeAutomaticReplay(id) => write!(f, "unsafe automatic replay: {id}"),
            Self::DependencyOrderInvalid(id) => write!(f, "dependency order invalid: {id}"),
            Self::StableActionSetMissing(id) => write!(f, "stable action set missing: {id}"),
            Self::MissingLocalPacket(id) => write!(f, "missing local packet for {id}"),
            Self::MissingActivityProjection(id) => {
                write!(f, "missing activity projection for {id}")
            }
            Self::MissingSupportExportRow(id) => {
                write!(f, "missing support export row for {id}")
            }
            Self::ActivityPhaseDrift(id) => write!(f, "activity phase drift for {id}"),
            Self::SupportExportUnsafe(id) => write!(f, "support export unsafe: {id}"),
            Self::TrustReviewInvalid => write!(f, "trust review invalid"),
            Self::ConsumerProjectionInvalid => write!(f, "consumer projection invalid"),
        }
    }
}

impl Error for DeferredPublishQueueRecoveryViolation {}

/// Validation error emitted while loading the checked artifact.
#[derive(Debug)]
pub enum DeferredPublishQueueRecoveryArtifactError {
    /// The JSON artifact could not be read.
    Io(std::io::Error),
    /// The JSON artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The checked artifact failed validation.
    Validation(Vec<DeferredPublishQueueRecoveryViolation>),
}

impl fmt::Display for DeferredPublishQueueRecoveryArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "deferred-publish recovery export read failed: {error}"),
            Self::SupportExport(error) => write!(
                f,
                "deferred-publish recovery export parse failed: {error}"
            ),
            Self::Validation(violations) => write!(
                f,
                "deferred-publish recovery export failed validation: {}",
                violations
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        }
    }
}

impl Error for DeferredPublishQueueRecoveryArtifactError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::SupportExport(error) => Some(error),
            Self::Validation(_) => None,
        }
    }
}

/// Returns the canonical source-contract refs for this packet.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        DEFERRED_PUBLISH_QUEUE_RECOVERY_SCHEMA_REF.to_owned(),
        DEFERRED_PUBLISH_QUEUE_RECOVERY_DOC_REF.to_owned(),
        DEFERRED_PUBLISH_QUEUE_RECOVERY_MUTATION_REVIEW_CONTRACT_REF.to_owned(),
        DEFERRED_PUBLISH_QUEUE_RECOVERY_SCOPE_REVIEW_CONTRACT_REF.to_owned(),
        DEFERRED_PUBLISH_QUEUE_RECOVERY_TRANSITION_CONTRACT_REF.to_owned(),
        DEFERRED_PUBLISH_QUEUE_RECOVERY_SYNC_CONTRACT_REF.to_owned(),
        DEFERRED_PUBLISH_QUEUE_RECOVERY_QUEUE_CONTRACT_REF.to_owned(),
    ]
}

/// Builds the canonical deferred-publish queue recovery packet.
pub fn canonical_deferred_publish_queue_recovery_packet() -> DeferredPublishQueueRecoveryPacket {
    let mutation_packet = canonical_work_item_mutation_review_packet();
    let scope_review_page = seeded_provider_scope_review_page();

    let detail_by_id = mutation_packet
        .detail_headers
        .iter()
        .map(|row| (row.work_item_detail_record_id_ref.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let scope_by_id = scope_review_page
        .resolutions
        .iter()
        .map(|row| (row.resolution_id.as_str(), row))
        .collect::<BTreeMap<_, _>>();

    let detail_local = detail_by_id
        .get("work_items:detail:local-draft")
        .expect("local-draft detail exists");
    let detail_cached = detail_by_id
        .get("work_items:detail:cached-stale")
        .expect("cached-stale detail exists");
    let detail_policy = detail_by_id
        .get("work_items:detail:policy-blocked")
        .expect("policy-blocked detail exists");
    let detail_provider = detail_by_id
        .get("work_items:detail:provider-authoritative")
        .expect("provider-authoritative detail exists");

    let local_scope = scope_by_id
        .get("scope_review:resolution:issue:delegated:local_draft")
        .expect("local-draft scope review exists");
    let release_scope = scope_by_id
        .get("scope_review:resolution:release:host_mismatch:denied")
        .expect("release denied scope review exists");
    let browser_scope = scope_by_id
        .get("scope_review:resolution:code_host:browser_only:merge")
        .expect("browser-only scope review exists");
    let comment_scope = scope_by_id
        .get("scope_review:resolution:code_host:human:comment")
        .expect("comment scope review exists");

    let queue_rows = vec![
        DeferredPublishQueueRow {
            row_id: "deferred_publish:row:draft_only".to_owned(),
            queue_id: "queue:draft_only:eng-90".to_owned(),
            canonical_object_ref: "provider_object:work_item:eng-90:link-review".to_owned(),
            canonical_object_label: detail_local.canonical_id.clone(),
            provider_family: detail_local.provider_family,
            work_item_detail_record_id_ref: Some(detail_local.work_item_detail_record_id_ref.clone()),
            publish_review_id_ref: Some("work_item_sync:publish_review:link-review".to_owned()),
            scope_review_resolution_id_ref: local_scope.resolution_id.clone(),
            local_packet_id_ref: "deferred_publish:packet:draft_only".to_owned(),
            linked_publish_later_queue_item_ref: None,
            linked_browser_handoff_packet_ref: None,
            linked_offline_handoff_packet_ref: None,
            lifecycle_state: DeferredPublishLifecycleState::DraftOnly,
            block_reason_class: None,
            target_freshness_requirement_class:
                DeferredPublishFreshnessRequirementClass::NotApplicableLocalDraft,
            retry_posture_class: DeferredPublishRetryPostureClass::NotApplicableDraftOrPublished,
            conflict_policy_class: DeferredPublishConflictPolicyClass::NotApplicable,
            dependency_chain: vec![],
            audit_event_refs: local_scope.audit_event_refs.clone(),
            replay_requires_fresh_target_identity: false,
            replay_requires_current_effective_scope: false,
            changed_boundary_review_required: false,
            high_impact_provider_mutation: false,
            auto_replay_allowed: false,
            action_routes: stable_action_routes(
                "draft_only",
                true,
                true,
                true,
                true,
                None,
            ),
            summary_label:
                "Local draft remains authoritative until the user chooses a later provider path."
                    .to_owned(),
        },
        DeferredPublishQueueRow {
            row_id: "deferred_publish:row:queued_outage".to_owned(),
            queue_id: "publish_later:item:eng-84".to_owned(),
            canonical_object_ref: "provider_object:work_item:eng-84:update".to_owned(),
            canonical_object_label: detail_cached.canonical_id.clone(),
            provider_family: detail_cached.provider_family,
            work_item_detail_record_id_ref: Some(detail_cached.work_item_detail_record_id_ref.clone()),
            publish_review_id_ref: Some("work_item_sync:publish_review:unlink-branch".to_owned()),
            scope_review_resolution_id_ref: local_scope.resolution_id.clone(),
            local_packet_id_ref: "deferred_publish:packet:queued_outage".to_owned(),
            linked_publish_later_queue_item_ref: Some("publish_later:item:eng-84".to_owned()),
            linked_browser_handoff_packet_ref: None,
            linked_offline_handoff_packet_ref: Some(
                "work_items:offline_handoff:provider-unreachable".to_owned(),
            ),
            lifecycle_state: DeferredPublishLifecycleState::QueuedForPublish,
            block_reason_class: Some(DeferredPublishBlockReasonClass::ProviderOutage),
            target_freshness_requirement_class:
                DeferredPublishFreshnessRequirementClass::ProviderRefreshRequiredBeforeReplay,
            retry_posture_class:
                DeferredPublishRetryPostureClass::ManualRetryAfterProviderRecovery,
            conflict_policy_class:
                DeferredPublishConflictPolicyClass::ProviderVersionReviewRequired,
            dependency_chain: vec![
                dependency(
                    0,
                    "connectivity_restored",
                    "unmet",
                    "Provider route must recover before replay.",
                    Some("route_resolution:provider_outage"),
                ),
                dependency(
                    1,
                    "freshness_floor_satisfied",
                    "unmet",
                    "Target must be refreshed again before replay.",
                    Some("freshness_floor:provider:issues:item"),
                ),
            ],
            audit_event_refs: local_scope.audit_event_refs.clone(),
            replay_requires_fresh_target_identity: true,
            replay_requires_current_effective_scope: true,
            changed_boundary_review_required: false,
            high_impact_provider_mutation: true,
            auto_replay_allowed: false,
            action_routes: stable_action_routes(
                "queued_outage",
                true,
                true,
                true,
                true,
                None,
            ),
            summary_label:
                "Provider outage preserves the queued work-item update and its durable local packet."
                    .to_owned(),
        },
        DeferredPublishQueueRow {
            row_id: "deferred_publish:row:auth_denied".to_owned(),
            queue_id: "queue:release_host_mismatch:fleet-0001".to_owned(),
            canonical_object_ref: "provider_object:release:fleet-0001:v1.2.3".to_owned(),
            canonical_object_label: release_scope.target_object_identity.target_label.clone(),
            provider_family: release_scope.provider_family,
            work_item_detail_record_id_ref: None,
            publish_review_id_ref: None,
            scope_review_resolution_id_ref: release_scope.resolution_id.clone(),
            local_packet_id_ref: "deferred_publish:packet:auth_denied".to_owned(),
            linked_publish_later_queue_item_ref: None,
            linked_browser_handoff_packet_ref: None,
            linked_offline_handoff_packet_ref: None,
            lifecycle_state: DeferredPublishLifecycleState::Blocked,
            block_reason_class: Some(DeferredPublishBlockReasonClass::AuthDenied),
            target_freshness_requirement_class:
                DeferredPublishFreshnessRequirementClass::CurrentTargetIdentityRequired,
            retry_posture_class:
                DeferredPublishRetryPostureClass::NoAutomaticReplayAcrossChangedBoundaries,
            conflict_policy_class: DeferredPublishConflictPolicyClass::ExportOrDiscardOnly,
            dependency_chain: vec![dependency(
                0,
                "rescope_completed",
                "unmet",
                "Host and tenant scope must be re-reviewed before replay.",
                Some("scope_review:resolution:release:host_mismatch:denied"),
            )],
            audit_event_refs: release_scope.audit_event_refs.clone(),
            replay_requires_fresh_target_identity: true,
            replay_requires_current_effective_scope: true,
            changed_boundary_review_required: true,
            high_impact_provider_mutation: true,
            auto_replay_allowed: false,
            action_routes: stable_action_routes(
                "auth_denied",
                true,
                true,
                true,
                true,
                Some("Retry reopens scope review because the reviewed boundary changed."),
            ),
            summary_label:
                "Write denial preserves the provider-backed change-flow packet for review, export, or external completion."
                    .to_owned(),
        },
        DeferredPublishQueueRow {
            row_id: "deferred_publish:row:redaction_blocked".to_owned(),
            queue_id: "queue:incident_annotation:redaction_blocked".to_owned(),
            canonical_object_ref: "provider_object:incident:sev-245:annotation".to_owned(),
            canonical_object_label: detail_policy.canonical_id.clone(),
            provider_family: detail_policy.provider_family,
            work_item_detail_record_id_ref: Some(detail_policy.work_item_detail_record_id_ref.clone()),
            publish_review_id_ref: Some(
                "work_item_sync:publish_review:open-in-provider-comment".to_owned(),
            ),
            scope_review_resolution_id_ref: browser_scope.resolution_id.clone(),
            local_packet_id_ref: "deferred_publish:packet:redaction_blocked".to_owned(),
            linked_publish_later_queue_item_ref: None,
            linked_browser_handoff_packet_ref: Some("providers:browser_handoff:issue:242".to_owned()),
            linked_offline_handoff_packet_ref: Some("work_items:offline_handoff:browser-blocked".to_owned()),
            lifecycle_state: DeferredPublishLifecycleState::Blocked,
            block_reason_class: Some(DeferredPublishBlockReasonClass::RedactionPolicyBlocked),
            target_freshness_requirement_class:
                DeferredPublishFreshnessRequirementClass::CurrentTargetIdentityRequired,
            retry_posture_class: DeferredPublishRetryPostureClass::RetryAfterRedactionReview,
            conflict_policy_class: DeferredPublishConflictPolicyClass::ExportOrDiscardOnly,
            dependency_chain: vec![dependency(
                0,
                "approval_ticket_admitted",
                "unmet",
                "Redaction review must clear the packet before a publish path is reopened.",
                Some("redaction:manifest:work-item-offline"),
            )],
            audit_event_refs: browser_scope.audit_event_refs.clone(),
            replay_requires_fresh_target_identity: true,
            replay_requires_current_effective_scope: true,
            changed_boundary_review_required: false,
            high_impact_provider_mutation: true,
            auto_replay_allowed: false,
            action_routes: stable_action_routes(
                "redaction_blocked",
                true,
                true,
                true,
                true,
                None,
            ),
            summary_label:
                "Redaction policy blocks the outbound incident annotation without discarding the local packet."
                    .to_owned(),
        },
        DeferredPublishQueueRow {
            row_id: "deferred_publish:row:stale_target".to_owned(),
            queue_id: "providers:queue_item:issue:242".to_owned(),
            canonical_object_ref: "provider_object:work_item:eng-242:transition".to_owned(),
            canonical_object_label: detail_cached.canonical_id.clone(),
            provider_family: detail_cached.provider_family,
            work_item_detail_record_id_ref: Some(detail_cached.work_item_detail_record_id_ref.clone()),
            publish_review_id_ref: Some(
                "work_item_sync:publish_review:transition-plus-comment".to_owned(),
            ),
            scope_review_resolution_id_ref: local_scope.resolution_id.clone(),
            local_packet_id_ref: "deferred_publish:packet:stale_target".to_owned(),
            linked_publish_later_queue_item_ref: Some("providers:queue_item:issue:242".to_owned()),
            linked_browser_handoff_packet_ref: None,
            linked_offline_handoff_packet_ref: Some(
                "work_items:offline_handoff:provider-unreachable".to_owned(),
            ),
            lifecycle_state: DeferredPublishLifecycleState::StaleTarget,
            block_reason_class: Some(DeferredPublishBlockReasonClass::FreshTargetRequired),
            target_freshness_requirement_class:
                DeferredPublishFreshnessRequirementClass::ProviderRefreshRequiredBeforeReplay,
            retry_posture_class:
                DeferredPublishRetryPostureClass::RetryAfterFreshnessRefresh,
            conflict_policy_class:
                DeferredPublishConflictPolicyClass::ProviderVersionReviewRequired,
            dependency_chain: vec![
                dependency(
                    0,
                    "freshness_floor_satisfied",
                    "unmet",
                    "Target freshness must be refreshed before replay.",
                    Some("freshness_floor:provider:issues:item"),
                ),
                dependency(
                    1,
                    "predecessor_queue_item",
                    "met",
                    "The prior local packet has already been durably captured.",
                    Some("deferred_publish:packet:queued_outage"),
                ),
            ],
            audit_event_refs: local_scope.audit_event_refs.clone(),
            replay_requires_fresh_target_identity: true,
            replay_requires_current_effective_scope: true,
            changed_boundary_review_required: false,
            high_impact_provider_mutation: true,
            auto_replay_allowed: false,
            action_routes: stable_action_routes(
                "stale_target",
                true,
                true,
                true,
                true,
                None,
            ),
            summary_label:
                "Replay is held until the stale work-item target is refreshed again."
                    .to_owned(),
        },
        DeferredPublishQueueRow {
            row_id: "deferred_publish:row:conflict".to_owned(),
            queue_id: "providers:queue_item:comment:retry".to_owned(),
            canonical_object_ref: "provider_object:work_item:eng-242:comment-retry".to_owned(),
            canonical_object_label: detail_cached.canonical_id.clone(),
            provider_family: detail_cached.provider_family,
            work_item_detail_record_id_ref: Some(detail_cached.work_item_detail_record_id_ref.clone()),
            publish_review_id_ref: Some(
                "work_item_sync:publish_review:retry-after-conflict".to_owned(),
            ),
            scope_review_resolution_id_ref: comment_scope.resolution_id.clone(),
            local_packet_id_ref: "deferred_publish:packet:conflict".to_owned(),
            linked_publish_later_queue_item_ref: Some("providers:queue_item:comment:retry".to_owned()),
            linked_browser_handoff_packet_ref: None,
            linked_offline_handoff_packet_ref: None,
            lifecycle_state: DeferredPublishLifecycleState::ConflictReviewRequired,
            block_reason_class: Some(DeferredPublishBlockReasonClass::ValidationConflict),
            target_freshness_requirement_class:
                DeferredPublishFreshnessRequirementClass::CurrentTargetIdentityRequired,
            retry_posture_class:
                DeferredPublishRetryPostureClass::RetryAfterConflictReconcile,
            conflict_policy_class:
                DeferredPublishConflictPolicyClass::CompareAndReconcileBeforeReplay,
            dependency_chain: vec![dependency(
                0,
                "conflict_resolved",
                "unmet",
                "Provider and local comment bodies must be reconciled first.",
                Some("conflict-row:reconciliation_result:issue_comment"),
            )],
            audit_event_refs: comment_scope.audit_event_refs.clone(),
            replay_requires_fresh_target_identity: true,
            replay_requires_current_effective_scope: true,
            changed_boundary_review_required: false,
            high_impact_provider_mutation: true,
            auto_replay_allowed: false,
            action_routes: stable_action_routes(
                "conflict",
                true,
                true,
                true,
                true,
                None,
            ),
            summary_label:
                "Validation conflict preserves the local packet until compare-and-reconcile review completes."
                    .to_owned(),
        },
        DeferredPublishQueueRow {
            row_id: "deferred_publish:row:published".to_owned(),
            queue_id: "providers:queue_item:issue:242:drained".to_owned(),
            canonical_object_ref: "provider_object:work_item:eng-242:published".to_owned(),
            canonical_object_label: detail_provider.canonical_id.clone(),
            provider_family: detail_provider.provider_family,
            work_item_detail_record_id_ref: Some(detail_provider.work_item_detail_record_id_ref.clone()),
            publish_review_id_ref: Some("work_item_sync:publish_review:create-comment".to_owned()),
            scope_review_resolution_id_ref: comment_scope.resolution_id.clone(),
            local_packet_id_ref: "deferred_publish:packet:published".to_owned(),
            linked_publish_later_queue_item_ref: Some("providers:queue_item:issue:242".to_owned()),
            linked_browser_handoff_packet_ref: None,
            linked_offline_handoff_packet_ref: Some("work_items:offline_handoff:drained-accepted".to_owned()),
            lifecycle_state: DeferredPublishLifecycleState::Published,
            block_reason_class: None,
            target_freshness_requirement_class:
                DeferredPublishFreshnessRequirementClass::FreshWithinGraceWindow,
            retry_posture_class: DeferredPublishRetryPostureClass::NotApplicableDraftOrPublished,
            conflict_policy_class: DeferredPublishConflictPolicyClass::NotApplicable,
            dependency_chain: vec![],
            audit_event_refs: comment_scope.audit_event_refs.clone(),
            replay_requires_fresh_target_identity: false,
            replay_requires_current_effective_scope: false,
            changed_boundary_review_required: false,
            high_impact_provider_mutation: true,
            auto_replay_allowed: false,
            action_routes: stable_action_routes(
                "published",
                false,
                false,
                true,
                true,
                Some("Published rows stay reopenable, exportable, and externally inspectable."),
            ),
            summary_label:
                "Provider commit confirmed; the durable packet remains exportable and reopenable."
                    .to_owned(),
        },
    ];

    let local_packets = queue_rows
        .iter()
        .map(|row| local_packet_from_queue_row(row))
        .collect::<Vec<_>>();

    let activity_rows = queue_rows
        .iter()
        .map(activity_projection_from_queue_row)
        .collect::<Vec<_>>();

    let support_rows = queue_rows
        .iter()
        .map(support_export_from_queue_row)
        .collect::<Vec<_>>();

    DeferredPublishQueueRecoveryPacket {
        record_kind: DEFERRED_PUBLISH_QUEUE_RECOVERY_RECORD_KIND.to_owned(),
        schema_version: DEFERRED_PUBLISH_QUEUE_RECOVERY_SCHEMA_VERSION,
        packet_id: "deferred-publish-recovery:stable:0001".to_owned(),
        surface_label: "Deferred publish queue recovery packets".to_owned(),
        queue_rows,
        local_packets,
        activity_rows,
        support_export: DeferredPublishRecoverySupportExport {
            export_id: "support_export:deferred_publish_recovery:stable:0001".to_owned(),
            rows: support_rows,
            consumer_surfaces: vec![
                "queue_panel".to_owned(),
                "activity_center".to_owned(),
                "support_export".to_owned(),
                "incident_workspace".to_owned(),
                "browser_companion".to_owned(),
            ],
            raw_provider_material_excluded: true,
        },
        trust_review: DeferredPublishRecoveryTrustReview {
            queue_rows_preserve_replay_contract: true,
            durable_local_packets_survive_restart_and_export: true,
            blocked_publish_preserves_intent_and_actions: true,
            replay_requires_fresh_target_and_scope: true,
            high_impact_actions_never_auto_replay: true,
        },
        consumer_projection: DeferredPublishRecoveryConsumerProjection {
            queue_rows_use_shared_lifecycle_vocabulary: true,
            activity_rows_use_shared_lifecycle_vocabulary: true,
            support_export_uses_shared_lifecycle_vocabulary: true,
            authoritative_object_identity_is_shared: true,
            authoritative_object_keeps_stable_actions: true,
        },
        proof_freshness: DeferredPublishRecoveryProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-12T23:55:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T23:55:00Z".to_owned(),
    }
}

/// Reads and validates the checked-in packet export.
pub fn current_deferred_publish_queue_recovery_export(
) -> Result<DeferredPublishQueueRecoveryPacket, DeferredPublishQueueRecoveryArtifactError> {
    let export_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/ship_deferred_publish_queue_recovery_packets/support_export.json"
    );
    let payload = fs::read_to_string(export_path)
        .map_err(DeferredPublishQueueRecoveryArtifactError::Io)?;
    let packet: DeferredPublishQueueRecoveryPacket = serde_json::from_str(&payload)
        .map_err(DeferredPublishQueueRecoveryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DeferredPublishQueueRecoveryArtifactError::Validation(
            violations,
        ))
    }
}

fn dependency(
    dependency_order_index: usize,
    dependency_class_token: &str,
    dependency_state_token: &str,
    rationale_summary: &str,
    linked_record_ref: Option<&str>,
) -> DeferredPublishDependencySummary {
    DeferredPublishDependencySummary {
        dependency_order_index,
        dependency_class_token: dependency_class_token.to_owned(),
        dependency_state_token: dependency_state_token.to_owned(),
        rationale_summary: rationale_summary.to_owned(),
        linked_record_ref: linked_record_ref.map(str::to_owned),
    }
}

fn stable_action_routes(
    slug: &str,
    retry_enabled: bool,
    discard_enabled: bool,
    export_enabled: bool,
    open_external_enabled: bool,
    disabled_reason_label: Option<&str>,
) -> Vec<DeferredPublishActionRoute> {
    vec![
        action_route(
            slug,
            DeferredPublishActionClass::Retry,
            "Retry",
            format!("cmd:deferred_publish.retry:{slug}"),
            retry_enabled,
            disabled_reason_label,
        ),
        action_route(
            slug,
            DeferredPublishActionClass::Discard,
            "Discard",
            format!("cmd:deferred_publish.discard:{slug}"),
            discard_enabled,
            disabled_reason_label,
        ),
        action_route(
            slug,
            DeferredPublishActionClass::ExportPacket,
            "Export packet",
            format!("cmd:deferred_publish.export:{slug}"),
            export_enabled,
            disabled_reason_label,
        ),
        action_route(
            slug,
            DeferredPublishActionClass::OpenExternal,
            "Open external",
            format!("cmd:deferred_publish.open_external:{slug}"),
            open_external_enabled,
            disabled_reason_label,
        ),
    ]
}

fn action_route(
    slug: &str,
    action_class: DeferredPublishActionClass,
    label: &str,
    route_ref: String,
    enabled: bool,
    disabled_reason_label: Option<&str>,
) -> DeferredPublishActionRoute {
    DeferredPublishActionRoute {
        action_id: format!("deferred_publish:action:{slug}:{}", action_class.as_str()),
        action_class,
        label: label.to_owned(),
        route_ref,
        enabled,
        disabled_reason_label: (!enabled).then(|| {
            disabled_reason_label
                .unwrap_or("This action is not meaningful in the current lifecycle state.")
                .to_owned()
        }),
    }
}

fn local_packet_from_queue_row(row: &DeferredPublishQueueRow) -> DeferredPublishLocalPacketRow {
    DeferredPublishLocalPacketRow {
        packet_id: row.local_packet_id_ref.clone(),
        packet_kind: match row.lifecycle_state {
            DeferredPublishLifecycleState::DraftOnly => {
                DeferredPublishLocalPacketKind::WorkItemStatusUpdate
            }
            DeferredPublishLifecycleState::QueuedForPublish
            | DeferredPublishLifecycleState::StaleTarget => {
                DeferredPublishLocalPacketKind::CommentPublish
            }
            DeferredPublishLifecycleState::Blocked
                if row.block_reason_class
                    == Some(DeferredPublishBlockReasonClass::RedactionPolicyBlocked) =>
            {
                DeferredPublishLocalPacketKind::IncidentAnnotation
            }
            DeferredPublishLifecycleState::Blocked => {
                DeferredPublishLocalPacketKind::ProviderBackedChangeFlow
            }
            DeferredPublishLifecycleState::ConflictReviewRequired => {
                DeferredPublishLocalPacketKind::CommentPublish
            }
            DeferredPublishLifecycleState::Published => {
                DeferredPublishLocalPacketKind::CommentPublish
            }
        },
        canonical_object_ref: row.canonical_object_ref.clone(),
        canonical_object_label: row.canonical_object_label.clone(),
        lifecycle_state: row.lifecycle_state,
        block_reason_class: row.block_reason_class,
        linked_queue_id_ref: Some(row.queue_id.clone()),
        linked_browser_handoff_packet_ref: row.linked_browser_handoff_packet_ref.clone(),
        linked_offline_handoff_packet_ref: row.linked_offline_handoff_packet_ref.clone(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        persisted_across_restart: true,
        export_safe_support_handoff: true,
        audit_event_refs: row.audit_event_refs.clone(),
        action_routes: row.action_routes.clone(),
        summary_label: format!(
            "{} packet preserves the same deferred-publish vocabulary as the queue row.",
            row.canonical_object_label
        ),
    }
}

fn activity_projection_from_queue_row(
    row: &DeferredPublishQueueRow,
) -> DeferredPublishActivityProjectionRow {
    DeferredPublishActivityProjectionRow {
        row_id: format!("activity_projection:{}", row.row_id),
        canonical_object_ref: row.canonical_object_ref.clone(),
        queue_row_id_ref: row.row_id.clone(),
        local_packet_id_ref: row.local_packet_id_ref.clone(),
        lifecycle_state: row.lifecycle_state,
        block_reason_class: row.block_reason_class,
        retry_posture_class: row.retry_posture_class,
        conflict_policy_class: row.conflict_policy_class,
        exact_reopen_ref: row.local_packet_id_ref.clone(),
        support_export_row_ref: format!("support_row:{}", row.row_id),
        phase_label: row.lifecycle_state.as_str().to_owned(),
        summary_label: format!("{}: {}", row.canonical_object_label, row.lifecycle_state.as_str()),
        detail_label: row.summary_label.clone(),
    }
}

fn support_export_from_queue_row(row: &DeferredPublishQueueRow) -> DeferredPublishSupportExportRow {
    DeferredPublishSupportExportRow {
        row_id: format!("support_row:{}", row.row_id),
        canonical_object_ref: row.canonical_object_ref.clone(),
        queue_row_id_ref: row.row_id.clone(),
        local_packet_id_ref: row.local_packet_id_ref.clone(),
        lifecycle_state: row.lifecycle_state,
        block_reason_class: row.block_reason_class,
        target_freshness_requirement_class: row.target_freshness_requirement_class,
        retry_posture_class: row.retry_posture_class,
        conflict_policy_class: row.conflict_policy_class,
        action_route_ids: row
            .action_routes
            .iter()
            .map(|route| route.action_id.clone())
            .collect(),
        summary_label: row.summary_label.clone(),
    }
}

fn dependency_order_is_strict(chain: &[DeferredPublishDependencySummary]) -> bool {
    let mut indexes = chain
        .iter()
        .map(|dependency| dependency.dependency_order_index)
        .collect::<Vec<_>>();
    indexes.sort_unstable();
    indexes
        .iter()
        .enumerate()
        .all(|(expected, actual)| expected == *actual)
}

fn has_all_action_classes(routes: &[DeferredPublishActionRoute]) -> bool {
    let classes = routes
        .iter()
        .map(|route| route.action_class)
        .collect::<BTreeSet<_>>();
    classes
        == BTreeSet::from([
            DeferredPublishActionClass::Retry,
            DeferredPublishActionClass::Discard,
            DeferredPublishActionClass::ExportPacket,
            DeferredPublishActionClass::OpenExternal,
        ])
}
