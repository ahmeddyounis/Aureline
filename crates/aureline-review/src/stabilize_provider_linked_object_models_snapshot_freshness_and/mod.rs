//! Stabilized provider-linked object models, snapshot freshness, and
//! publish-now/publish-later/handoff continuity for review lanes.
//!
//! This module owns the bounded beta contract that keeps provider-backed
//! mutations previewable, attributable, and reversible on review surfaces.
//! Every claimed provider-backed mutation is modeled in one of four explicit
//! modes: draft-only, publish-later, publish-now, or handoff-only. The chosen
//! mode, acting identity, target object, and required auth source remain
//! inspectable before and after dispatch.
//!
//! Local draft and publish-later packets persist across restart with stable
//! action identity, dependency order, and replay safety. Snapshot freshness
//! and degradation behavior distinguish inspect-only cached state from live
//! provider-backed state, and unsupported mutations downgrade to
//! export/copy/handoff rather than appearing enabled.
//!
//! The record family includes:
//!
//! - [`ProviderLinkedReviewStabilizationRecord`] — stable identity binding
//!   workspace, provider object rows, and explicit mutation modes.
//! - [`ProviderLinkedObjectRowRecord`] — binding of one provider object row
//!   to the review surface with explicit mode disclosure, auth source, and
//!   target identity.
//! - [`ActorTargetIdentityRecord`] — preserved acting identity and target
//!   identity for every claimed provider-backed write path.
//! - [`DeferredIntentRecord`] — persisted deferred intent (publish-later or
//!   handoff-only) with idempotency key, expiry, replay lineage, and reconnect
//!   review requirement.
//! - [`FreshnessSnapshotRecord`] — snapshot freshness observation with
//!   degradation class, provider source, and downgraded action.
//! - [`ProviderLinkedCommandRecord`] — command-graph operations surfaced to
//!   the inspector.
//! - [`ProviderLinkedSupportExportPacket`] — redaction-safe support export
//!   that preserves actor, target, mode, and intent lineage.
//! - [`ProviderLinkedInspectionRecord`] — compact boolean projection for CLI
//!   and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/provider_linked_review_stabilization.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/stabilize-provider-linked-object-models-snapshot-freshness-and/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every provider-linked review stabilization record.
pub const PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ProviderLinkedReviewStabilizationPacket`].
pub const PROVIDER_LINKED_REVIEW_STABILIZATION_PACKET_RECORD_KIND: &str =
    "provider_linked_review_stabilization_packet";

/// Stable record-kind tag for [`ProviderLinkedReviewStabilizationRecord`].
pub const PROVIDER_LINKED_REVIEW_STABILIZATION_RECORD_KIND: &str =
    "provider_linked_review_stabilization_record";

/// Stable record-kind tag for [`ProviderLinkedObjectRowRecord`].
pub const PROVIDER_LINKED_OBJECT_ROW_RECORD_KIND: &str = "provider_linked_object_row_record";

/// Stable record-kind tag for [`ActorTargetIdentityRecord`].
pub const ACTOR_TARGET_IDENTITY_RECORD_KIND: &str = "actor_target_identity_record";

/// Stable record-kind tag for [`DeferredIntentRecord`].
pub const DEFERRED_INTENT_RECORD_KIND: &str = "deferred_intent_record";

/// Stable record-kind tag for [`FreshnessSnapshotRecord`].
pub const FRESHNESS_SNAPSHOT_RECORD_KIND: &str = "freshness_snapshot_record";

/// Stable record-kind tag for [`ProviderLinkedCommandRecord`].
pub const PROVIDER_LINKED_COMMAND_RECORD_KIND: &str = "provider_linked_command_record";

/// Stable record-kind tag for [`ProviderLinkedSupportExportPacket`].
pub const PROVIDER_LINKED_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "provider_linked_support_export_packet";

/// Stable record-kind tag for [`ProviderLinkedInspectionRecord`].
pub const PROVIDER_LINKED_INSPECTION_RECORD_KIND: &str = "provider_linked_inspection_record";

/// Closed set of explicit mutation mode classes.
pub const MUTATION_MODE_CLASSES: &[&str] =
    &["draft_only", "publish_later", "publish_now", "handoff_only"];

/// Closed set of freshness degradation classes.
pub const FRESHNESS_DEGRADATION_CLASSES: &[&str] = &[
    "none",
    "stale_within_window",
    "expired_beyond_window",
    "provider_offline",
    "revoked_or_disconnected",
    "disagrees_with_local",
];

/// Closed set of replay safety classes.
pub const REPLAY_SAFETY_CLASSES: &[&str] =
    &["idempotent", "retry_safe", "destructive_requires_review"];

/// Closed set of provider-linked review states.
pub const PROVIDER_LINKED_REVIEW_STATES: &[&str] = &[
    "active",
    "degraded_freshness",
    "blocked_drift",
    "blocked_auth",
    "blocked_scope",
    "queued_publish_later",
    "pending_handoff",
    "completed",
    "cancelled",
];

/// Closed set of consumer surfaces.
pub const PROVIDER_LINKED_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
];

/// Closed set of invalidation reasons.
pub const PROVIDER_LINKED_INVALIDATION_REASONS: &[&str] = &[
    "freshness_expired",
    "provider_drift",
    "actor_scope_changed",
    "policy_epoch_changed",
    "target_identity_changed",
    "auth_revoked",
    "reconnect_required",
];

/// Closed set of command classes.
pub const PROVIDER_LINKED_COMMAND_CLASSES: &[&str] = &[
    "preview_mutation",
    "approve_publish_now",
    "queue_publish_later",
    "open_handoff",
    "cancel_intent",
    "export_evidence",
    "reconcile_drift",
    "replay_after_reconnect",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a provider-linked review stabilization to materialize on
/// top of a beta review-workspace packet and provider object rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedReviewStabilizationInput {
    /// Stable stabilization identity.
    pub stabilization_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Review state from the closed vocabulary.
    pub review_state: String,
    /// Provider object row inputs.
    pub object_rows: Vec<ProviderLinkedObjectRowInput>,
    /// Actor/target identity inputs.
    pub actor_target_identities: Vec<ActorTargetIdentityInput>,
    /// Deferred intent inputs.
    pub deferred_intents: Vec<DeferredIntentInput>,
    /// Freshness snapshot inputs.
    pub freshness_snapshots: Vec<FreshnessSnapshotInput>,
    /// Command-graph operations.
    pub commands: Vec<ProviderLinkedCommandInput>,
    /// Support/export envelope input.
    pub support_export: ProviderLinkedSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing one provider-linked object row binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedObjectRowInput {
    /// Stable object row identity.
    pub object_row_id: String,
    /// Opaque ref to the upstream provider-object model row.
    pub provider_object_row_ref: String,
    /// Explicit mutation mode class from the closed vocabulary.
    pub mutation_mode_class: String,
    /// Human-readable mode disclosure label.
    pub mode_disclosure_label: String,
    /// Opaque auth source ref.
    pub auth_source_ref: String,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Opaque actor identity ref.
    pub actor_identity_ref: String,
    /// Freshness degradation class from the closed vocabulary.
    pub freshness_degradation_class: String,
    /// Downgraded action class when degraded.
    pub downgraded_action_class: String,
    /// Optional deferred intent ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferred_intent_ref: Option<String>,
    /// Optional freshness snapshot ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_snapshot_ref: Option<String>,
    /// True when local draft editing is preserved.
    pub local_draft_preserved: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one actor/target identity binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorTargetIdentityInput {
    /// Stable identity binding id.
    pub identity_binding_id: String,
    /// Opaque actor identity ref.
    pub actor_identity_ref: String,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Opaque auth source ref.
    pub auth_source_ref: String,
    /// Opaque policy scope epoch ref.
    pub scope_epoch_ref: String,
    /// True when this identity must persist across restart.
    pub preserve_across_restart: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one deferred intent (publish-later or handoff-only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredIntentInput {
    /// Stable intent identity.
    pub intent_id: String,
    /// Stable command id for replay safety.
    pub command_id: String,
    /// Idempotency key for replay deduplication.
    pub idempotency_key: String,
    /// Mutation mode class (`publish_later` or `handoff_only`).
    pub mutation_mode_class: String,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Opaque actor identity ref.
    pub actor_identity_ref: String,
    /// Timestamp when the intent was queued.
    pub queued_at: String,
    /// Timestamp when the intent expires.
    pub expires_at: String,
    /// Opaque replay ledger ref.
    pub replay_ledger_ref: String,
    /// True when reconnect review is required before replay.
    pub reconnect_review_required: bool,
    /// Zero-based dependency order index.
    pub dependency_order_index: usize,
    /// True when the intent is safe to replay without live provider.
    pub safe_to_replay_offline: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one freshness snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessSnapshotInput {
    /// Stable snapshot identity.
    pub snapshot_id: String,
    /// Freshness class (`fresh`, `stale_within_window`, etc.).
    pub freshness_class: String,
    /// Freshness degradation class from the closed vocabulary.
    pub freshness_degradation_class: String,
    /// Provider source class (`live_provider`, `cached_provider_overlay`, etc.).
    pub provider_source_class: String,
    /// Timestamp when the freshness was observed.
    pub observed_at: String,
    /// Optional stale-after duration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_after: Option<String>,
    /// Downgraded action class when degraded.
    pub downgraded_action_class: String,
    /// True when this snapshot represents inspect-only cached state.
    pub inspect_only_cached: bool,
    /// True when this snapshot is backed by a live provider.
    pub live_provider_backed: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one command-graph operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedCommandInput {
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
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input row for the support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedSupportExportInput {
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
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Provider-linked review stabilization record materialized from input plus
/// workspace truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedReviewStabilizationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable stabilization identity.
    pub stabilization_id: String,
    /// Review workspace this stabilization belongs to.
    pub review_workspace_id_ref: String,
    /// Review state.
    pub review_state: String,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing mutation.
    pub blocked_reasons: Vec<String>,
    /// True when the stabilization is actionable from the current state.
    pub actionable: bool,
    /// Timestamp the stabilization was frozen.
    pub generated_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Provider-linked object row record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedObjectRowRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this row belongs to.
    pub stabilization_id_ref: String,
    /// Stable object row identity.
    pub object_row_id: String,
    /// Opaque ref to the upstream provider-object model row.
    pub provider_object_row_ref: String,
    /// Explicit mutation mode class.
    pub mutation_mode_class: String,
    /// Human-readable mode disclosure label.
    pub mode_disclosure_label: String,
    /// Opaque auth source ref.
    pub auth_source_ref: String,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Opaque actor identity ref.
    pub actor_identity_ref: String,
    /// Freshness degradation class.
    pub freshness_degradation_class: String,
    /// Downgraded action class when degraded.
    pub downgraded_action_class: String,
    /// Optional deferred intent ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferred_intent_ref: Option<String>,
    /// Optional freshness snapshot ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_snapshot_ref: Option<String>,
    /// True when local draft editing is preserved.
    pub local_draft_preserved: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Actor/target identity record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorTargetIdentityRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this identity binding belongs to.
    pub stabilization_id_ref: String,
    /// Stable identity binding id.
    pub identity_binding_id: String,
    /// Opaque actor identity ref.
    pub actor_identity_ref: String,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Opaque auth source ref.
    pub auth_source_ref: String,
    /// Opaque policy scope epoch ref.
    pub scope_epoch_ref: String,
    /// True when this identity must persist across restart.
    pub preserve_across_restart: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Deferred intent record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredIntentRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this intent belongs to.
    pub stabilization_id_ref: String,
    /// Stable intent identity.
    pub intent_id: String,
    /// Stable command id for replay safety.
    pub command_id: String,
    /// Idempotency key for replay deduplication.
    pub idempotency_key: String,
    /// Mutation mode class (`publish_later` or `handoff_only`).
    pub mutation_mode_class: String,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Opaque actor identity ref.
    pub actor_identity_ref: String,
    /// Timestamp when the intent was queued.
    pub queued_at: String,
    /// Timestamp when the intent expires.
    pub expires_at: String,
    /// Opaque replay ledger ref.
    pub replay_ledger_ref: String,
    /// True when reconnect review is required before replay.
    pub reconnect_review_required: bool,
    /// Zero-based dependency order index.
    pub dependency_order_index: usize,
    /// True when the intent is safe to replay without live provider.
    pub safe_to_replay_offline: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Freshness snapshot record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessSnapshotRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization this snapshot belongs to.
    pub stabilization_id_ref: String,
    /// Stable snapshot identity.
    pub snapshot_id: String,
    /// Freshness class.
    pub freshness_class: String,
    /// Freshness degradation class.
    pub freshness_degradation_class: String,
    /// Provider source class.
    pub provider_source_class: String,
    /// Timestamp when the freshness was observed.
    pub observed_at: String,
    /// Optional stale-after duration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_after: Option<String>,
    /// Downgraded action class when degraded.
    pub downgraded_action_class: String,
    /// True when this snapshot represents inspect-only cached state.
    pub inspect_only_cached: bool,
    /// True when this snapshot is backed by a live provider.
    pub live_provider_backed: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Command-graph operation record for provider-linked review stabilization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedCommandRecord {
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
    /// Reviewable summary.
    pub summary_label: String,
}

/// Support/export packet for the provider-linked review stabilization lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedSupportExportPacket {
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
    /// Reviewable summary.
    pub summary_label: String,
}

/// Inspection row used by support/export and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stabilization inspected by this row.
    pub stabilization_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the review state is active.
    pub review_state_active: bool,
    /// True when the review state is degraded_freshness.
    pub review_state_degraded_freshness: bool,
    /// True when the review state is blocked_drift.
    pub review_state_blocked_drift: bool,
    /// True when the review state is blocked_auth.
    pub review_state_blocked_auth: bool,
    /// True when the review state is blocked_scope.
    pub review_state_blocked_scope: bool,
    /// True when at least one object row is in publish-now mode.
    pub publish_now_present: bool,
    /// True when at least one object row is in publish-later mode.
    pub publish_later_present: bool,
    /// True when at least one object row is in handoff-only mode.
    pub handoff_only_present: bool,
    /// True when at least one object row is in draft-only mode.
    pub draft_only_present: bool,
    /// True when at least one deferred intent requires reconnect review.
    pub reconnect_review_required: bool,
    /// True when at least one object row has degraded freshness.
    pub freshness_degraded: bool,
    /// True when all provider-backed rows disclose auth source.
    pub all_auth_sources_disclosed: bool,
    /// True when all provider-backed rows disclose target identity.
    pub all_target_identities_disclosed: bool,
    /// True when all provider-backed rows disclose actor identity.
    pub all_actor_identities_disclosed: bool,
    /// True when local drafts are preserved.
    pub local_drafts_preserved: bool,
    /// True when the stabilization is actionable.
    pub actionable: bool,
    /// True when the stabilization is invalidated by any reason.
    pub invalidated: bool,
    /// Number of object row records.
    pub object_row_count: usize,
    /// Number of deferred intent records.
    pub deferred_intent_count: usize,
    /// Number of freshness snapshot records.
    pub freshness_snapshot_count: usize,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the stabilization context.
    pub support_export_reopenable: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Provider-linked review stabilization packet consumed by review surfaces and
/// support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLinkedReviewStabilizationPacket {
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
    /// Provider-linked review stabilization record.
    pub stabilization: ProviderLinkedReviewStabilizationRecord,
    /// Provider-linked object row records.
    pub object_rows: Vec<ProviderLinkedObjectRowRecord>,
    /// Actor/target identity records.
    pub actor_target_identities: Vec<ActorTargetIdentityRecord>,
    /// Deferred intent records.
    pub deferred_intents: Vec<DeferredIntentRecord>,
    /// Freshness snapshot records.
    pub freshness_snapshots: Vec<FreshnessSnapshotRecord>,
    /// Command-graph operation records.
    pub commands: Vec<ProviderLinkedCommandRecord>,
    /// Support/export packet.
    pub support_export: ProviderLinkedSupportExportPacket,
    /// Inspection row.
    pub inspection: ProviderLinkedInspectionRecord,
}

impl ProviderLinkedReviewStabilizationPacket {
    /// Builds a provider-linked review stabilization packet from a beta
    /// review-workspace packet and stabilization input.
    ///
    /// # Errors
    ///
    /// Returns [`ProviderLinkedReviewStabilizationValidationError`] when the
    /// input violates a stabilization invariant.
    pub fn from_workspace_packet(
        input: ProviderLinkedReviewStabilizationInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
    ) -> Result<Self, ProviderLinkedReviewStabilizationValidationError> {
        validate_input(&input, workspace_packet)?;

        let stabilization = stabilization_record(&input, workspace_packet);
        let object_rows = input
            .object_rows
            .iter()
            .map(|r| object_row_record(r, &stabilization))
            .collect::<Vec<_>>();
        let actor_target_identities = input
            .actor_target_identities
            .iter()
            .map(|i| actor_target_identity_record(i, &stabilization))
            .collect::<Vec<_>>();
        let deferred_intents = input
            .deferred_intents
            .iter()
            .map(|i| deferred_intent_record(i, &stabilization))
            .collect::<Vec<_>>();
        let freshness_snapshots = input
            .freshness_snapshots
            .iter()
            .map(|s| freshness_snapshot_record(s, &stabilization))
            .collect::<Vec<_>>();
        let commands = input
            .commands
            .iter()
            .map(|c| command_record(c, &stabilization))
            .collect::<Vec<_>>();
        let support_export = support_export_packet(
            &input.support_export,
            &stabilization,
            workspace_packet,
            &commands,
        );
        let inspection = inspection_record(
            &stabilization,
            &object_rows,
            &deferred_intents,
            &freshness_snapshots,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: PROVIDER_LINKED_REVIEW_STABILIZATION_PACKET_RECORD_KIND.to_string(),
            schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            stabilization,
            object_rows,
            actor_target_identities,
            deferred_intents,
            freshness_snapshots,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the provider-linked review stabilization
    /// invariants.
    ///
    /// # Errors
    ///
    /// Returns [`ProviderLinkedReviewStabilizationValidationError`] when an
    /// invariant is violated.
    pub fn validate(&self) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            PROVIDER_LINKED_REVIEW_STABILIZATION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_stabilization_record(
            &self.stabilization,
            &self.review_workspace.review_workspace_id,
        )?;
        for row in &self.object_rows {
            validate_object_row_record(row, &self.stabilization.stabilization_id)?;
        }
        for identity in &self.actor_target_identities {
            validate_actor_target_identity_record(identity, &self.stabilization.stabilization_id)?;
        }
        for intent in &self.deferred_intents {
            validate_deferred_intent_record(intent, &self.stabilization.stabilization_id)?;
        }
        for snapshot in &self.freshness_snapshots {
            validate_freshness_snapshot_record(snapshot, &self.stabilization.stabilization_id)?;
        }
        for command in &self.commands {
            validate_command_record(command, &self.stabilization.stabilization_id)?;
        }
        validate_support_export(&self.support_export, &self.stabilization, &self.commands)?;
        validate_inspection(&self.inspection, self)?;

        // Cross-record invariants
        let _object_row_ids: BTreeSet<&str> = self
            .object_rows
            .iter()
            .map(|r| r.object_row_id.as_str())
            .collect();
        let intent_ids: BTreeSet<&str> = self
            .deferred_intents
            .iter()
            .map(|i| i.intent_id.as_str())
            .collect();
        let snapshot_ids: BTreeSet<&str> = self
            .freshness_snapshots
            .iter()
            .map(|s| s.snapshot_id.as_str())
            .collect();

        for row in &self.object_rows {
            if let Some(ref intent_ref) = row.deferred_intent_ref {
                if !intent_ids.contains(intent_ref.as_str()) {
                    return Err(provider_linked_validation_error(format!(
                        "object_row {intent_ref} cites unknown deferred_intent_ref"
                    )));
                }
            }
            if let Some(ref snapshot_ref) = row.freshness_snapshot_ref {
                if !snapshot_ids.contains(snapshot_ref.as_str()) {
                    return Err(provider_linked_validation_error(format!(
                        "object_row {snapshot_ref} cites unknown freshness_snapshot_ref"
                    )));
                }
            }
        }

        // Identity coverage: every provider-backed row must have an actor/target identity
        for row in &self.object_rows {
            if row.mutation_mode_class != "draft_only" {
                let has_identity = self.actor_target_identities.iter().any(|i| {
                    i.actor_identity_ref == row.actor_identity_ref
                        && i.target_identity_ref == row.target_identity_ref
                });
                if !has_identity {
                    return Err(provider_linked_validation_error(format!(
                        "provider-backed object_row {} lacks matching actor_target_identity",
                        row.object_row_id
                    )));
                }
            }
        }

        // Deferred intent dependency order must be contiguous
        if !self.deferred_intents.is_empty() {
            let mut indexes: Vec<usize> = self
                .deferred_intents
                .iter()
                .map(|i| i.dependency_order_index)
                .collect();
            indexes.sort_unstable();
            if !indexes
                .iter()
                .enumerate()
                .all(|(expected, actual)| expected == *actual)
            {
                return Err(provider_linked_validation_error(
                    "deferred_intent dependency_order_index values must be zero-based and contiguous".to_string(),
                ));
            }
        }

        // Freshness coherence: inspect_only_cached and live_provider_backed are mutually exclusive
        for snapshot in &self.freshness_snapshots {
            if snapshot.inspect_only_cached && snapshot.live_provider_backed {
                return Err(provider_linked_validation_error(format!(
                    "freshness_snapshot {} cannot be both inspect_only_cached and live_provider_backed",
                    snapshot.snapshot_id
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

    /// Returns true when the stabilization can be reopened after restart from
    /// the support export.
    pub fn restartable_from_support_export(&self) -> bool {
        self.inspection.support_export_reopenable
    }
}

// ---------------------------------------------------------------------------
// Projection type
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderLinkedReviewStabilizationProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Stabilization identity.
    pub stabilization_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Review state.
    pub review_state: String,
    /// True when the stabilization is actionable.
    pub actionable: bool,
    /// True when at least one object row is in publish-now mode.
    pub publish_now_present: bool,
    /// True when at least one object row is in publish-later mode.
    pub publish_later_present: bool,
    /// True when at least one object row is in handoff-only mode.
    pub handoff_only_present: bool,
    /// True when at least one object row is in draft-only mode.
    pub draft_only_present: bool,
    /// True when freshness is degraded.
    pub freshness_degraded: bool,
    /// True when reconnect review is required.
    pub reconnect_review_required: bool,
    /// True when all auth sources are disclosed.
    pub all_auth_sources_disclosed: bool,
    /// True when all target identities are disclosed.
    pub all_target_identities_disclosed: bool,
    /// True when local drafts are preserved.
    pub local_drafts_preserved: bool,
    /// Number of deferred intents.
    pub deferred_intent_count: usize,
    /// Number of object rows.
    pub object_row_count: usize,
    /// Command count.
    pub command_count: usize,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for provider-linked review stabilization operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderLinkedReviewStabilizationError {
    /// Validation failed.
    Validation(ProviderLinkedReviewStabilizationValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for ProviderLinkedReviewStabilizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for ProviderLinkedReviewStabilizationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for provider-linked review stabilization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderLinkedReviewStabilizationValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for ProviderLinkedReviewStabilizationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ProviderLinkedReviewStabilizationValidationError {}

fn provider_linked_validation_error(
    message: impl Into<String>,
) -> ProviderLinkedReviewStabilizationValidationError {
    ProviderLinkedReviewStabilizationValidationError {
        message: message.into(),
    }
}

// ---------------------------------------------------------------------------
// Builder / validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &ProviderLinkedReviewStabilizationInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_nonempty(&input.stabilization_id, "stabilization_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.review_state, "review_state")?;
    ensure_token(
        PROVIDER_LINKED_REVIEW_STATES,
        &input.review_state,
        "review_state",
    )?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    for reason in &input.invalidation_reasons {
        ensure_token(
            PROVIDER_LINKED_INVALIDATION_REASONS,
            reason,
            "invalidation_reason",
        )?;
    }

    if input.object_rows.is_empty() {
        return Err(provider_linked_validation_error(
            "input must contain at least one object_row".to_string(),
        ));
    }

    let mut object_row_ids = BTreeSet::new();
    for row in &input.object_rows {
        ensure_nonempty(&row.object_row_id, "object_row.object_row_id")?;
        if !object_row_ids.insert(&row.object_row_id) {
            return Err(provider_linked_validation_error(format!(
                "duplicate object_row_id: {}",
                row.object_row_id
            )));
        }
        ensure_nonempty(
            &row.provider_object_row_ref,
            "object_row.provider_object_row_ref",
        )?;
        ensure_token(
            MUTATION_MODE_CLASSES,
            &row.mutation_mode_class,
            "mutation_mode_class",
        )?;
        ensure_nonempty(
            &row.mode_disclosure_label,
            "object_row.mode_disclosure_label",
        )?;
        ensure_nonempty(&row.auth_source_ref, "object_row.auth_source_ref")?;
        ensure_nonempty(&row.target_identity_ref, "object_row.target_identity_ref")?;
        ensure_nonempty(&row.actor_identity_ref, "object_row.actor_identity_ref")?;
        ensure_token(
            FRESHNESS_DEGRADATION_CLASSES,
            &row.freshness_degradation_class,
            "freshness_degradation_class",
        )?;
        ensure_nonempty(&row.summary_label, "object_row.summary_label")?;
    }

    let mut identity_binding_ids = BTreeSet::new();
    for identity in &input.actor_target_identities {
        ensure_nonempty(&identity.identity_binding_id, "identity_binding_id")?;
        if !identity_binding_ids.insert(&identity.identity_binding_id) {
            return Err(provider_linked_validation_error(format!(
                "duplicate identity_binding_id: {}",
                identity.identity_binding_id
            )));
        }
        ensure_nonempty(
            &identity.actor_identity_ref,
            "actor_target_identity.actor_identity_ref",
        )?;
        ensure_nonempty(
            &identity.target_identity_ref,
            "actor_target_identity.target_identity_ref",
        )?;
        ensure_nonempty(
            &identity.auth_source_ref,
            "actor_target_identity.auth_source_ref",
        )?;
        ensure_nonempty(
            &identity.scope_epoch_ref,
            "actor_target_identity.scope_epoch_ref",
        )?;
        ensure_nonempty(
            &identity.summary_label,
            "actor_target_identity.summary_label",
        )?;
    }

    let mut intent_ids = BTreeSet::new();
    for intent in &input.deferred_intents {
        ensure_nonempty(&intent.intent_id, "deferred_intent.intent_id")?;
        if !intent_ids.insert(&intent.intent_id) {
            return Err(provider_linked_validation_error(format!(
                "duplicate intent_id: {}",
                intent.intent_id
            )));
        }
        ensure_nonempty(&intent.command_id, "deferred_intent.command_id")?;
        ensure_nonempty(&intent.idempotency_key, "deferred_intent.idempotency_key")?;
        ensure_token(
            &["publish_later", "handoff_only"],
            &intent.mutation_mode_class,
            "deferred_intent.mutation_mode_class",
        )?;
        ensure_nonempty(
            &intent.target_identity_ref,
            "deferred_intent.target_identity_ref",
        )?;
        ensure_nonempty(
            &intent.actor_identity_ref,
            "deferred_intent.actor_identity_ref",
        )?;
        ensure_nonempty(&intent.queued_at, "deferred_intent.queued_at")?;
        ensure_nonempty(&intent.expires_at, "deferred_intent.expires_at")?;
        ensure_nonempty(
            &intent.replay_ledger_ref,
            "deferred_intent.replay_ledger_ref",
        )?;
        ensure_nonempty(&intent.summary_label, "deferred_intent.summary_label")?;
    }

    let mut snapshot_ids = BTreeSet::new();
    for snapshot in &input.freshness_snapshots {
        ensure_nonempty(&snapshot.snapshot_id, "freshness_snapshot.snapshot_id")?;
        if !snapshot_ids.insert(&snapshot.snapshot_id) {
            return Err(provider_linked_validation_error(format!(
                "duplicate snapshot_id: {}",
                snapshot.snapshot_id
            )));
        }
        ensure_nonempty(
            &snapshot.freshness_class,
            "freshness_snapshot.freshness_class",
        )?;
        ensure_token(
            FRESHNESS_DEGRADATION_CLASSES,
            &snapshot.freshness_degradation_class,
            "freshness_snapshot.freshness_degradation_class",
        )?;
        ensure_nonempty(
            &snapshot.provider_source_class,
            "freshness_snapshot.provider_source_class",
        )?;
        ensure_nonempty(&snapshot.observed_at, "freshness_snapshot.observed_at")?;
        ensure_nonempty(&snapshot.summary_label, "freshness_snapshot.summary_label")?;
    }

    let mut command_ids = BTreeSet::new();
    for command in &input.commands {
        ensure_nonempty(&command.command_id, "command.command_id")?;
        if !command_ids.insert(&command.command_id) {
            return Err(provider_linked_validation_error(format!(
                "duplicate command_id: {}",
                command.command_id
            )));
        }
        ensure_token(
            PROVIDER_LINKED_COMMAND_CLASSES,
            &command.command_class,
            "command.command_class",
        )?;
        ensure_nonempty(&command.target_object_ref, "command.target_object_ref")?;
        ensure_nonempty(&command.target_object_kind, "command.target_object_kind")?;
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
            PROVIDER_LINKED_CONSUMER_SURFACES,
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

    ensure_nonempty(
        &workspace_packet.review_workspace.review_workspace_id,
        "review_workspace_id",
    )?;

    Ok(())
}

fn stabilization_record(
    input: &ProviderLinkedReviewStabilizationInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> ProviderLinkedReviewStabilizationRecord {
    ProviderLinkedReviewStabilizationRecord {
        record_kind: PROVIDER_LINKED_REVIEW_STABILIZATION_RECORD_KIND.to_string(),
        schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id: input.stabilization_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        review_state: input.review_state.clone(),
        invalidation_reasons: input.invalidation_reasons.clone(),
        blocked_reasons: input
            .commands
            .iter()
            .flat_map(|c| c.blocked_reasons.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        actionable: input.commands.iter().any(|c| c.blocked_reasons.is_empty()),
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn object_row_record(
    input: &ProviderLinkedObjectRowInput,
    stabilization: &ProviderLinkedReviewStabilizationRecord,
) -> ProviderLinkedObjectRowRecord {
    ProviderLinkedObjectRowRecord {
        record_kind: PROVIDER_LINKED_OBJECT_ROW_RECORD_KIND.to_string(),
        schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        object_row_id: input.object_row_id.clone(),
        provider_object_row_ref: input.provider_object_row_ref.clone(),
        mutation_mode_class: input.mutation_mode_class.clone(),
        mode_disclosure_label: input.mode_disclosure_label.clone(),
        auth_source_ref: input.auth_source_ref.clone(),
        target_identity_ref: input.target_identity_ref.clone(),
        actor_identity_ref: input.actor_identity_ref.clone(),
        freshness_degradation_class: input.freshness_degradation_class.clone(),
        downgraded_action_class: input.downgraded_action_class.clone(),
        deferred_intent_ref: input.deferred_intent_ref.clone(),
        freshness_snapshot_ref: input.freshness_snapshot_ref.clone(),
        local_draft_preserved: input.local_draft_preserved,
        summary_label: input.summary_label.clone(),
    }
}

fn actor_target_identity_record(
    input: &ActorTargetIdentityInput,
    stabilization: &ProviderLinkedReviewStabilizationRecord,
) -> ActorTargetIdentityRecord {
    ActorTargetIdentityRecord {
        record_kind: ACTOR_TARGET_IDENTITY_RECORD_KIND.to_string(),
        schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        identity_binding_id: input.identity_binding_id.clone(),
        actor_identity_ref: input.actor_identity_ref.clone(),
        target_identity_ref: input.target_identity_ref.clone(),
        auth_source_ref: input.auth_source_ref.clone(),
        scope_epoch_ref: input.scope_epoch_ref.clone(),
        preserve_across_restart: input.preserve_across_restart,
        summary_label: input.summary_label.clone(),
    }
}

fn deferred_intent_record(
    input: &DeferredIntentInput,
    stabilization: &ProviderLinkedReviewStabilizationRecord,
) -> DeferredIntentRecord {
    DeferredIntentRecord {
        record_kind: DEFERRED_INTENT_RECORD_KIND.to_string(),
        schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        intent_id: input.intent_id.clone(),
        command_id: input.command_id.clone(),
        idempotency_key: input.idempotency_key.clone(),
        mutation_mode_class: input.mutation_mode_class.clone(),
        target_identity_ref: input.target_identity_ref.clone(),
        actor_identity_ref: input.actor_identity_ref.clone(),
        queued_at: input.queued_at.clone(),
        expires_at: input.expires_at.clone(),
        replay_ledger_ref: input.replay_ledger_ref.clone(),
        reconnect_review_required: input.reconnect_review_required,
        dependency_order_index: input.dependency_order_index,
        safe_to_replay_offline: input.safe_to_replay_offline,
        summary_label: input.summary_label.clone(),
    }
}

fn freshness_snapshot_record(
    input: &FreshnessSnapshotInput,
    stabilization: &ProviderLinkedReviewStabilizationRecord,
) -> FreshnessSnapshotRecord {
    FreshnessSnapshotRecord {
        record_kind: FRESHNESS_SNAPSHOT_RECORD_KIND.to_string(),
        schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        snapshot_id: input.snapshot_id.clone(),
        freshness_class: input.freshness_class.clone(),
        freshness_degradation_class: input.freshness_degradation_class.clone(),
        provider_source_class: input.provider_source_class.clone(),
        observed_at: input.observed_at.clone(),
        stale_after: input.stale_after.clone(),
        downgraded_action_class: input.downgraded_action_class.clone(),
        inspect_only_cached: input.inspect_only_cached,
        live_provider_backed: input.live_provider_backed,
        summary_label: input.summary_label.clone(),
    }
}

fn command_record(
    input: &ProviderLinkedCommandInput,
    stabilization: &ProviderLinkedReviewStabilizationRecord,
) -> ProviderLinkedCommandRecord {
    ProviderLinkedCommandRecord {
        record_kind: PROVIDER_LINKED_COMMAND_RECORD_KIND.to_string(),
        schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
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

fn support_export_packet(
    input: &ProviderLinkedSupportExportInput,
    stabilization: &ProviderLinkedReviewStabilizationRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    commands: &[ProviderLinkedCommandRecord],
) -> ProviderLinkedSupportExportPacket {
    ProviderLinkedSupportExportPacket {
        record_kind: PROVIDER_LINKED_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/provider_linked_review_stabilization.schema.json".to_string(),
            "schemas/providers/provider_object_model_alpha.schema.json".to_string(),
            "schemas/providers/publish_later_queue_alpha.schema.json".to_string(),
            "schemas/providers/provider_browser_handoff_alpha.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn inspection_record(
    stabilization: &ProviderLinkedReviewStabilizationRecord,
    object_rows: &[ProviderLinkedObjectRowRecord],
    deferred_intents: &[DeferredIntentRecord],
    freshness_snapshots: &[FreshnessSnapshotRecord],
    commands: &[ProviderLinkedCommandRecord],
    support_export: &ProviderLinkedSupportExportPacket,
) -> ProviderLinkedInspectionRecord {
    ProviderLinkedInspectionRecord {
        record_kind: PROVIDER_LINKED_INSPECTION_RECORD_KIND.to_string(),
        schema_version: PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        stabilization_id_ref: stabilization.stabilization_id.clone(),
        review_workspace_id_ref: stabilization.review_workspace_id_ref.clone(),
        review_state_active: stabilization.review_state == "active",
        review_state_degraded_freshness: stabilization.review_state == "degraded_freshness",
        review_state_blocked_drift: stabilization.review_state == "blocked_drift",
        review_state_blocked_auth: stabilization.review_state == "blocked_auth",
        review_state_blocked_scope: stabilization.review_state == "blocked_scope",
        publish_now_present: object_rows
            .iter()
            .any(|r| r.mutation_mode_class == "publish_now"),
        publish_later_present: object_rows
            .iter()
            .any(|r| r.mutation_mode_class == "publish_later"),
        handoff_only_present: object_rows
            .iter()
            .any(|r| r.mutation_mode_class == "handoff_only"),
        draft_only_present: object_rows
            .iter()
            .any(|r| r.mutation_mode_class == "draft_only"),
        reconnect_review_required: deferred_intents.iter().any(|i| i.reconnect_review_required),
        freshness_degraded: freshness_snapshots
            .iter()
            .any(|s| s.freshness_degradation_class != "none"),
        all_auth_sources_disclosed: object_rows
            .iter()
            .all(|r| !r.auth_source_ref.trim().is_empty()),
        all_target_identities_disclosed: object_rows
            .iter()
            .all(|r| !r.target_identity_ref.trim().is_empty()),
        all_actor_identities_disclosed: object_rows
            .iter()
            .all(|r| !r.actor_identity_ref.trim().is_empty()),
        local_drafts_preserved: object_rows.iter().all(|r| r.local_draft_preserved),
        actionable: stabilization.actionable,
        invalidated: !stabilization.invalidation_reasons.is_empty(),
        object_row_count: object_rows.len(),
        deferred_intent_count: deferred_intents.len(),
        freshness_snapshot_count: freshness_snapshots.len(),
        command_count: commands.len(),
        preview_capable: commands.iter().any(|c| c.preview_supported),
        support_export_reopenable: !support_export.reopen_context_ref.trim().is_empty()
            && !support_export.reopen_command_id_ref.trim().is_empty(),
        summary_label: stabilization.summary_label.clone(),
    }
}

fn validate_stabilization_record(
    record: &ProviderLinkedReviewStabilizationRecord,
    expected_workspace_id: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PROVIDER_LINKED_REVIEW_STABILIZATION_RECORD_KIND,
        "stabilization.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        "stabilization.schema_version",
    )?;
    ensure_nonempty(&record.stabilization_id, "stabilization.stabilization_id")?;
    if record.review_workspace_id_ref != expected_workspace_id {
        return Err(provider_linked_validation_error(format!(
            "stabilization.review_workspace_id_ref mismatch: expected {expected_workspace_id}, got {}",
            record.review_workspace_id_ref
        )));
    }
    ensure_token(
        PROVIDER_LINKED_REVIEW_STATES,
        &record.review_state,
        "stabilization.review_state",
    )?;
    ensure_nonempty(&record.generated_at, "stabilization.generated_at")?;
    ensure_nonempty(&record.summary_label, "stabilization.summary_label")?;
    Ok(())
}

fn validate_object_row_record(
    record: &ProviderLinkedObjectRowRecord,
    expected_stabilization_id: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PROVIDER_LINKED_OBJECT_ROW_RECORD_KIND,
        "object_row.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        "object_row.schema_version",
    )?;
    if record.stabilization_id_ref != expected_stabilization_id {
        return Err(provider_linked_validation_error(format!(
            "object_row.stabilization_id_ref mismatch: expected {expected_stabilization_id}, got {}",
            record.stabilization_id_ref
        )));
    }
    ensure_nonempty(&record.object_row_id, "object_row.object_row_id")?;
    ensure_nonempty(
        &record.provider_object_row_ref,
        "object_row.provider_object_row_ref",
    )?;
    ensure_token(
        MUTATION_MODE_CLASSES,
        &record.mutation_mode_class,
        "object_row.mutation_mode_class",
    )?;
    ensure_nonempty(
        &record.mode_disclosure_label,
        "object_row.mode_disclosure_label",
    )?;
    ensure_nonempty(&record.auth_source_ref, "object_row.auth_source_ref")?;
    ensure_nonempty(
        &record.target_identity_ref,
        "object_row.target_identity_ref",
    )?;
    ensure_nonempty(&record.actor_identity_ref, "object_row.actor_identity_ref")?;
    ensure_token(
        FRESHNESS_DEGRADATION_CLASSES,
        &record.freshness_degradation_class,
        "object_row.freshness_degradation_class",
    )?;
    ensure_nonempty(&record.summary_label, "object_row.summary_label")?;
    Ok(())
}

fn validate_actor_target_identity_record(
    record: &ActorTargetIdentityRecord,
    expected_stabilization_id: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        ACTOR_TARGET_IDENTITY_RECORD_KIND,
        "actor_target_identity.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        "actor_target_identity.schema_version",
    )?;
    if record.stabilization_id_ref != expected_stabilization_id {
        return Err(provider_linked_validation_error(format!(
            "actor_target_identity.stabilization_id_ref mismatch"
        )));
    }
    ensure_nonempty(
        &record.identity_binding_id,
        "actor_target_identity.identity_binding_id",
    )?;
    ensure_nonempty(
        &record.actor_identity_ref,
        "actor_target_identity.actor_identity_ref",
    )?;
    ensure_nonempty(
        &record.target_identity_ref,
        "actor_target_identity.target_identity_ref",
    )?;
    ensure_nonempty(
        &record.auth_source_ref,
        "actor_target_identity.auth_source_ref",
    )?;
    ensure_nonempty(
        &record.scope_epoch_ref,
        "actor_target_identity.scope_epoch_ref",
    )?;
    ensure_nonempty(&record.summary_label, "actor_target_identity.summary_label")?;
    Ok(())
}

fn validate_deferred_intent_record(
    record: &DeferredIntentRecord,
    expected_stabilization_id: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        DEFERRED_INTENT_RECORD_KIND,
        "deferred_intent.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        "deferred_intent.schema_version",
    )?;
    if record.stabilization_id_ref != expected_stabilization_id {
        return Err(provider_linked_validation_error(format!(
            "deferred_intent.stabilization_id_ref mismatch"
        )));
    }
    ensure_nonempty(&record.intent_id, "deferred_intent.intent_id")?;
    ensure_nonempty(&record.command_id, "deferred_intent.command_id")?;
    ensure_nonempty(&record.idempotency_key, "deferred_intent.idempotency_key")?;
    ensure_token(
        &["publish_later", "handoff_only"],
        &record.mutation_mode_class,
        "deferred_intent.mutation_mode_class",
    )?;
    ensure_nonempty(
        &record.target_identity_ref,
        "deferred_intent.target_identity_ref",
    )?;
    ensure_nonempty(
        &record.actor_identity_ref,
        "deferred_intent.actor_identity_ref",
    )?;
    ensure_nonempty(&record.queued_at, "deferred_intent.queued_at")?;
    ensure_nonempty(&record.expires_at, "deferred_intent.expires_at")?;
    ensure_nonempty(
        &record.replay_ledger_ref,
        "deferred_intent.replay_ledger_ref",
    )?;
    ensure_nonempty(&record.summary_label, "deferred_intent.summary_label")?;
    Ok(())
}

fn validate_freshness_snapshot_record(
    record: &FreshnessSnapshotRecord,
    expected_stabilization_id: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        FRESHNESS_SNAPSHOT_RECORD_KIND,
        "freshness_snapshot.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        "freshness_snapshot.schema_version",
    )?;
    if record.stabilization_id_ref != expected_stabilization_id {
        return Err(provider_linked_validation_error(format!(
            "freshness_snapshot.stabilization_id_ref mismatch"
        )));
    }
    ensure_nonempty(&record.snapshot_id, "freshness_snapshot.snapshot_id")?;
    ensure_nonempty(
        &record.freshness_class,
        "freshness_snapshot.freshness_class",
    )?;
    ensure_token(
        FRESHNESS_DEGRADATION_CLASSES,
        &record.freshness_degradation_class,
        "freshness_snapshot.freshness_degradation_class",
    )?;
    ensure_nonempty(
        &record.provider_source_class,
        "freshness_snapshot.provider_source_class",
    )?;
    ensure_nonempty(&record.observed_at, "freshness_snapshot.observed_at")?;
    ensure_nonempty(&record.summary_label, "freshness_snapshot.summary_label")?;
    if record.inspect_only_cached && record.live_provider_backed {
        return Err(provider_linked_validation_error(
            "freshness_snapshot cannot be both inspect_only_cached and live_provider_backed"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_command_record(
    record: &ProviderLinkedCommandRecord,
    expected_stabilization_id: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PROVIDER_LINKED_COMMAND_RECORD_KIND,
        "command.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        "command.schema_version",
    )?;
    if record.stabilization_id_ref != expected_stabilization_id {
        return Err(provider_linked_validation_error(format!(
            "command.stabilization_id_ref mismatch"
        )));
    }
    ensure_nonempty(&record.command_id, "command.command_id")?;
    ensure_token(
        PROVIDER_LINKED_COMMAND_CLASSES,
        &record.command_class,
        "command.command_class",
    )?;
    ensure_nonempty(&record.target_object_ref, "command.target_object_ref")?;
    ensure_nonempty(&record.target_object_kind, "command.target_object_kind")?;
    ensure_nonempty(&record.summary_label, "command.summary_label")?;
    Ok(())
}

fn validate_support_export(
    packet: &ProviderLinkedSupportExportPacket,
    stabilization: &ProviderLinkedReviewStabilizationRecord,
    commands: &[ProviderLinkedCommandRecord],
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_eq(
        packet.record_kind.as_str(),
        PROVIDER_LINKED_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export.record_kind",
    )?;
    ensure_eq_u32(
        packet.schema_version,
        PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        "support_export.schema_version",
    )?;
    if packet.stabilization_id_ref != stabilization.stabilization_id {
        return Err(provider_linked_validation_error(format!(
            "support_export.stabilization_id_ref mismatch"
        )));
    }
    ensure_nonempty(
        &packet.support_export_id,
        "support_export.support_export_id",
    )?;
    ensure_nonempty(
        &packet.reopen_context_ref,
        "support_export.reopen_context_ref",
    )?;
    ensure_nonempty(
        &packet.reopen_command_id_ref,
        "support_export.reopen_command_id_ref",
    )?;
    if packet.command_id_refs.len() != commands.len() {
        return Err(provider_linked_validation_error(format!(
            "support_export.command_id_refs length ({}) must match commands length ({})",
            packet.command_id_refs.len(),
            commands.len()
        )));
    }
    for surface in &packet.consumer_surfaces {
        ensure_token(
            PROVIDER_LINKED_CONSUMER_SURFACES,
            surface,
            "support_export.consumer_surface",
        )?;
    }
    ensure_nonempty(&packet.redaction_class, "support_export.redaction_class")?;
    ensure_nonempty(&packet.summary_label, "support_export.summary_label")?;
    if packet.raw_url_export_allowed {
        return Err(provider_linked_validation_error(
            "support_export.raw_url_export_allowed must be false".to_string(),
        ));
    }
    if packet.raw_provider_payload_export_allowed {
        return Err(provider_linked_validation_error(
            "support_export.raw_provider_payload_export_allowed must be false".to_string(),
        ));
    }
    Ok(())
}

fn validate_inspection(
    record: &ProviderLinkedInspectionRecord,
    packet: &ProviderLinkedReviewStabilizationPacket,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PROVIDER_LINKED_INSPECTION_RECORD_KIND,
        "inspection.record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION,
        "inspection.schema_version",
    )?;
    if record.stabilization_id_ref != packet.stabilization.stabilization_id {
        return Err(provider_linked_validation_error(format!(
            "inspection.stabilization_id_ref mismatch"
        )));
    }
    if record.review_workspace_id_ref != packet.stabilization.review_workspace_id_ref {
        return Err(provider_linked_validation_error(format!(
            "inspection.review_workspace_id_ref mismatch"
        )));
    }
    if record.object_row_count != packet.object_rows.len() {
        return Err(provider_linked_validation_error(format!(
            "inspection.object_row_count mismatch"
        )));
    }
    if record.deferred_intent_count != packet.deferred_intents.len() {
        return Err(provider_linked_validation_error(format!(
            "inspection.deferred_intent_count mismatch"
        )));
    }
    if record.freshness_snapshot_count != packet.freshness_snapshots.len() {
        return Err(provider_linked_validation_error(format!(
            "inspection.freshness_snapshot_count mismatch"
        )));
    }
    if record.command_count != packet.commands.len() {
        return Err(provider_linked_validation_error(format!(
            "inspection.command_count mismatch"
        )));
    }
    ensure_nonempty(&record.summary_label, "inspection.summary_label")?;
    Ok(())
}

fn ensure_eq(
    actual: &str,
    expected: &str,
    field: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    if actual != expected {
        Err(provider_linked_validation_error(format!(
            "{field} mismatch: expected {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}

fn ensure_eq_u32(
    actual: u32,
    expected: u32,
    field: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    if actual != expected {
        Err(provider_linked_validation_error(format!(
            "{field} mismatch: expected {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}

fn ensure_nonempty(
    value: &str,
    field: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    if value.trim().is_empty() {
        Err(provider_linked_validation_error(format!(
            "{field} must be non-empty"
        )))
    } else {
        Ok(())
    }
}

fn ensure_token(
    allowed: &[&str],
    value: &str,
    field: &str,
) -> Result<(), ProviderLinkedReviewStabilizationValidationError> {
    if !allowed.contains(&value) {
        Err(provider_linked_validation_error(format!(
            "{field} must be one of {allowed:?}, got {value}"
        )))
    } else {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Public builder
// ---------------------------------------------------------------------------

/// Builds a [`ProviderLinkedReviewStabilizationPacket`] from workspace input.
///
/// # Errors
///
/// Returns [`ProviderLinkedReviewStabilizationError`] when input violates
/// invariants.
pub fn project_provider_linked_review_stabilization_packet(
    input: ProviderLinkedReviewStabilizationInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<ProviderLinkedReviewStabilizationPacket, ProviderLinkedReviewStabilizationError> {
    ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, workspace_packet)
        .map_err(ProviderLinkedReviewStabilizationError::Validation)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_packet() -> ReviewWorkspaceBetaPacket {
        ReviewWorkspaceBetaPacket {
            record_kind: crate::workspace::REVIEW_WORKSPACE_BETA_PACKET_RECORD_KIND.to_string(),
            schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
            packet_id: "workspace.packet.test".to_string(),
            generated_at: "2026-05-27T08:00:00Z".to_string(),
            review_workspace: crate::workspace::ReviewWorkspaceRecord {
                record_kind: crate::workspace::REVIEW_WORKSPACE_RECORD_KIND.to_string(),
                review_workspace_schema_version:
                    crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                review_workspace_id: "workspace.test".to_string(),
                review_workspace_source_class: "local_git".to_string(),
                provider_authority_class: "provider_authoritative".to_string(),
                review_workspace_lifecycle_state: "active".to_string(),
                local_locator: None,
                provider_overlay: None,
                imported_bundle_envelope: None,
                browser_handoff_envelope: None,
                policy_context: crate::workspace::ReviewPolicyContext {
                    policy_epoch: "epoch.1".to_string(),
                    trust_state: "trusted".to_string(),
                    execution_context_id: None,
                    workspace_trust_state_class: "trusted_local".to_string(),
                },
                client_scopes: vec![],
                redaction_class: "metadata_safe_default".to_string(),
                freshness_class: "fresh".to_string(),
                summary_label: "Test workspace".to_string(),
                created_at: "2026-05-27T08:00:00Z".to_string(),
                updated_at: "2026-05-27T08:00:00Z".to_string(),
                archived_at: None,
                hosted_review_inbox_record_id_ref: None,
                merge_policy_record_id_ref: None,
            },
            diff_entries: vec![],
            check_freshness: vec![],
            object_lineage: vec![],
            durable_comment_anchors: vec![],
            browser_handoff: None,
            inspection: crate::workspace::ReviewWorkspaceBetaInspectionRecord {
                record_kind: crate::workspace::REVIEW_WORKSPACE_BETA_INSPECTION_RECORD_KIND
                    .to_string(),
                schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                review_workspace_id_ref: "workspace.test".to_string(),
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
                summary_label: "Test workspace".to_string(),
            },
            support_export: crate::workspace::ReviewWorkspaceSupportExportPacket {
                record_kind: crate::workspace::REVIEW_WORKSPACE_SUPPORT_EXPORT_PACKET_RECORD_KIND
                    .to_string(),
                schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                support_export_id: "support.export.test".to_string(),
                review_workspace_id_ref: "workspace.test".to_string(),
                reopen_context_ref: "reopen.test".to_string(),
                reopen_command_id_ref: "reopen.cmd.test".to_string(),
                durable_comment_anchor_refs: vec![],
                check_freshness_refs: vec![],
                object_lineage_refs: vec![],
                browser_handoff_ref: None,
                consumer_surfaces: vec!["support_export".to_string()],
                source_schema_refs: vec![],
                raw_comment_body_export_allowed: false,
                raw_url_export_allowed: false,
                raw_source_body_export_allowed: false,
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Test support export".to_string(),
            },
        }
    }

    fn baseline_input() -> ProviderLinkedReviewStabilizationInput {
        ProviderLinkedReviewStabilizationInput {
            stabilization_id: "stab.test.001".to_string(),
            packet_id: "packet.test.001".to_string(),
            generated_at: "2026-05-27T08:00:00Z".to_string(),
            review_state: "active".to_string(),
            object_rows: vec![
                ProviderLinkedObjectRowInput {
                    object_row_id: "row.pr.4012".to_string(),
                    provider_object_row_ref: "provider.row.pr.4012".to_string(),
                    mutation_mode_class: "publish_now".to_string(),
                    mode_disclosure_label: "This action will mutate the provider immediately."
                        .to_string(),
                    auth_source_ref: "auth.provider.code_host".to_string(),
                    target_identity_ref: "target.pr.4012".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    freshness_degradation_class: "none".to_string(),
                    downgraded_action_class: "none".to_string(),
                    deferred_intent_ref: None,
                    freshness_snapshot_ref: Some("snap.fresh.001".to_string()),
                    local_draft_preserved: false,
                    summary_label: "PR 4012 publish-now row".to_string(),
                },
                ProviderLinkedObjectRowInput {
                    object_row_id: "row.issue.aur.104".to_string(),
                    provider_object_row_ref: "provider.row.issue.aur.104".to_string(),
                    mutation_mode_class: "publish_later".to_string(),
                    mode_disclosure_label: "This action is queued for later publish.".to_string(),
                    auth_source_ref: "auth.provider.issue_tracker".to_string(),
                    target_identity_ref: "target.issue.aur.104".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    freshness_degradation_class: "stale_within_window".to_string(),
                    downgraded_action_class: "hold_for_freshness_repair".to_string(),
                    deferred_intent_ref: Some("intent.001".to_string()),
                    freshness_snapshot_ref: Some("snap.stale.001".to_string()),
                    local_draft_preserved: true,
                    summary_label: "Issue AUR-104 publish-later row".to_string(),
                },
                ProviderLinkedObjectRowInput {
                    object_row_id: "row.pr.4013".to_string(),
                    provider_object_row_ref: "provider.row.pr.4013".to_string(),
                    mutation_mode_class: "handoff_only".to_string(),
                    mode_disclosure_label: "This action opens a browser handoff packet."
                        .to_string(),
                    auth_source_ref: "auth.provider.code_host".to_string(),
                    target_identity_ref: "target.pr.4013".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    freshness_degradation_class: "none".to_string(),
                    downgraded_action_class: "none".to_string(),
                    deferred_intent_ref: Some("intent.002".to_string()),
                    freshness_snapshot_ref: Some("snap.fresh.002".to_string()),
                    local_draft_preserved: false,
                    summary_label: "PR 4013 handoff row".to_string(),
                },
                ProviderLinkedObjectRowInput {
                    object_row_id: "row.pr.4014".to_string(),
                    provider_object_row_ref: "provider.row.pr.4014".to_string(),
                    mutation_mode_class: "draft_only".to_string(),
                    mode_disclosure_label: "This action stays local.".to_string(),
                    auth_source_ref: "auth.local".to_string(),
                    target_identity_ref: "target.pr.4014.draft".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    freshness_degradation_class: "none".to_string(),
                    downgraded_action_class: "continue_local_authoring".to_string(),
                    deferred_intent_ref: None,
                    freshness_snapshot_ref: None,
                    local_draft_preserved: true,
                    summary_label: "PR 4014 draft-only row".to_string(),
                },
            ],
            actor_target_identities: vec![
                ActorTargetIdentityInput {
                    identity_binding_id: "id.bind.001".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    target_identity_ref: "target.pr.4012".to_string(),
                    auth_source_ref: "auth.provider.code_host".to_string(),
                    scope_epoch_ref: "epoch.2026.05".to_string(),
                    preserve_across_restart: true,
                    summary_label: "Alice -> PR 4012".to_string(),
                },
                ActorTargetIdentityInput {
                    identity_binding_id: "id.bind.002".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    target_identity_ref: "target.issue.aur.104".to_string(),
                    auth_source_ref: "auth.provider.issue_tracker".to_string(),
                    scope_epoch_ref: "epoch.2026.05".to_string(),
                    preserve_across_restart: true,
                    summary_label: "Alice -> Issue AUR-104".to_string(),
                },
                ActorTargetIdentityInput {
                    identity_binding_id: "id.bind.003".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    target_identity_ref: "target.pr.4013".to_string(),
                    auth_source_ref: "auth.provider.code_host".to_string(),
                    scope_epoch_ref: "epoch.2026.05".to_string(),
                    preserve_across_restart: true,
                    summary_label: "Alice -> PR 4013".to_string(),
                },
            ],
            deferred_intents: vec![
                DeferredIntentInput {
                    intent_id: "intent.001".to_string(),
                    command_id: "cmd.publish_later.001".to_string(),
                    idempotency_key: "idempotency.key.001".to_string(),
                    mutation_mode_class: "publish_later".to_string(),
                    target_identity_ref: "target.issue.aur.104".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    queued_at: "2026-05-27T08:00:00Z".to_string(),
                    expires_at: "2026-05-28T08:00:00Z".to_string(),
                    replay_ledger_ref: "ledger.001".to_string(),
                    reconnect_review_required: false,
                    dependency_order_index: 0,
                    safe_to_replay_offline: true,
                    summary_label: "Publish-later intent for AUR-104".to_string(),
                },
                DeferredIntentInput {
                    intent_id: "intent.002".to_string(),
                    command_id: "cmd.handoff.001".to_string(),
                    idempotency_key: "idempotency.key.002".to_string(),
                    mutation_mode_class: "handoff_only".to_string(),
                    target_identity_ref: "target.pr.4013".to_string(),
                    actor_identity_ref: "actor.user.alice".to_string(),
                    queued_at: "2026-05-27T08:00:00Z".to_string(),
                    expires_at: "2026-05-27T09:00:00Z".to_string(),
                    replay_ledger_ref: "ledger.002".to_string(),
                    reconnect_review_required: true,
                    dependency_order_index: 1,
                    safe_to_replay_offline: false,
                    summary_label: "Handoff intent for PR 4013".to_string(),
                },
            ],
            freshness_snapshots: vec![
                FreshnessSnapshotInput {
                    snapshot_id: "snap.fresh.001".to_string(),
                    freshness_class: "fresh".to_string(),
                    freshness_degradation_class: "none".to_string(),
                    provider_source_class: "live_provider".to_string(),
                    observed_at: "2026-05-27T08:00:00Z".to_string(),
                    stale_after: Some("PT10M".to_string()),
                    downgraded_action_class: "none".to_string(),
                    inspect_only_cached: false,
                    live_provider_backed: true,
                    summary_label: "PR 4012 is fresh".to_string(),
                },
                FreshnessSnapshotInput {
                    snapshot_id: "snap.stale.001".to_string(),
                    freshness_class: "stale_within_window".to_string(),
                    freshness_degradation_class: "stale_within_window".to_string(),
                    provider_source_class: "cached_provider_overlay".to_string(),
                    observed_at: "2026-05-27T07:30:00Z".to_string(),
                    stale_after: Some("PT45M".to_string()),
                    downgraded_action_class: "hold_for_freshness_repair".to_string(),
                    inspect_only_cached: true,
                    live_provider_backed: false,
                    summary_label: "Issue AUR-104 is stale within window".to_string(),
                },
                FreshnessSnapshotInput {
                    snapshot_id: "snap.fresh.002".to_string(),
                    freshness_class: "fresh".to_string(),
                    freshness_degradation_class: "none".to_string(),
                    provider_source_class: "live_provider".to_string(),
                    observed_at: "2026-05-27T08:00:00Z".to_string(),
                    stale_after: Some("PT10M".to_string()),
                    downgraded_action_class: "none".to_string(),
                    inspect_only_cached: false,
                    live_provider_backed: true,
                    summary_label: "PR 4013 is fresh".to_string(),
                },
            ],
            commands: vec![
                ProviderLinkedCommandInput {
                    command_id: "cmd.preview.001".to_string(),
                    command_class: "preview_mutation".to_string(),
                    target_object_ref: "target.pr.4012".to_string(),
                    target_object_kind: "pull_request".to_string(),
                    preview_supported: true,
                    emits_audit_event: false,
                    blocked_reasons: vec![],
                    summary_label: "Preview PR 4012 mutation".to_string(),
                },
                ProviderLinkedCommandInput {
                    command_id: "cmd.approve.001".to_string(),
                    command_class: "approve_publish_now".to_string(),
                    target_object_ref: "target.pr.4012".to_string(),
                    target_object_kind: "pull_request".to_string(),
                    preview_supported: false,
                    emits_audit_event: true,
                    blocked_reasons: vec![],
                    summary_label: "Approve publish-now for PR 4012".to_string(),
                },
            ],
            support_export: ProviderLinkedSupportExportInput {
                support_export_id: "support.export.001".to_string(),
                reopen_context_ref: "reopen.ctx.001".to_string(),
                reopen_command_id_ref: "reopen.cmd.001".to_string(),
                consumer_surfaces: vec!["support_export".to_string(), "audit_lane".to_string()],
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Support export for provider-linked stabilization".to_string(),
            },
            invalidation_reasons: vec![],
            summary_label: "Provider-linked review stabilization baseline".to_string(),
        }
    }

    #[test]
    fn baseline_packet_constructs_and_validates() {
        let workspace = workspace_packet();
        let input = baseline_input();
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace);
        assert!(
            packet.is_ok(),
            "baseline must construct: {:?}",
            packet.err()
        );
        let packet = packet.unwrap();
        assert!(packet.validate().is_ok());
        assert!(packet.raw_escape_hatches_absent());
        assert!(packet.restartable_from_support_export());
    }

    #[test]
    fn missing_mutation_mode_fails() {
        let workspace = workspace_packet();
        let mut input = baseline_input();
        input.object_rows[0].mutation_mode_class = "invalid_mode".to_string();
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace);
        assert!(packet.is_err());
    }

    #[test]
    fn missing_actor_target_identity_for_publish_now_fails() {
        let workspace = workspace_packet();
        let mut input = baseline_input();
        input.actor_target_identities.clear();
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace);
        assert!(packet.is_err());
    }

    #[test]
    fn draft_only_allows_missing_identity() {
        let workspace = workspace_packet();
        let mut input = baseline_input();
        input
            .object_rows
            .retain(|r| r.mutation_mode_class == "draft_only");
        input.actor_target_identities.clear();
        input.deferred_intents.clear();
        input.freshness_snapshots.clear();
        input.commands.clear();
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace);
        assert!(
            packet.is_ok(),
            "draft-only rows should not require actor_target_identities"
        );
    }

    #[test]
    fn inspect_only_cached_and_live_provider_backed_mutual_exclusion() {
        let workspace = workspace_packet();
        let mut input = baseline_input();
        input.freshness_snapshots[0].inspect_only_cached = true;
        input.freshness_snapshots[0].live_provider_backed = true;
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace);
        assert!(packet.is_err());
    }

    #[test]
    fn deferred_intent_dependency_order_must_be_contiguous() {
        let workspace = workspace_packet();
        let mut input = baseline_input();
        input.deferred_intents[0].dependency_order_index = 5;
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace);
        assert!(packet.is_err());
    }

    #[test]
    fn support_export_allows_no_raw_urls() {
        let workspace = workspace_packet();
        let mut input = baseline_input();
        input.support_export.support_export_id = "support.export.test".to_string();
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace)
                .unwrap();
        assert!(!packet.support_export.raw_url_export_allowed);
        assert!(!packet.support_export.raw_provider_payload_export_allowed);
    }

    #[test]
    fn projection_from_packet() {
        let workspace = workspace_packet();
        let input = baseline_input();
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace)
                .unwrap();
        assert!(packet.inspection.publish_now_present);
        assert!(packet.inspection.publish_later_present);
        assert!(packet.inspection.handoff_only_present);
        assert!(packet.inspection.draft_only_present);
        assert!(packet.inspection.all_auth_sources_disclosed);
        assert!(packet.inspection.all_target_identities_disclosed);
        assert!(packet.inspection.all_actor_identities_disclosed);
        assert!(!packet.inspection.local_drafts_preserved);
        assert_eq!(packet.inspection.object_row_count, 4);
        assert_eq!(packet.inspection.deferred_intent_count, 2);
        assert_eq!(packet.inspection.freshness_snapshot_count, 3);
        assert_eq!(packet.inspection.command_count, 2);
    }

    #[test]
    fn invalid_deferred_intent_mode_fails() {
        let workspace = workspace_packet();
        let mut input = baseline_input();
        input.deferred_intents[0].mutation_mode_class = "publish_now".to_string();
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace);
        assert!(packet.is_err());
    }

    #[test]
    fn empty_object_rows_fails() {
        let workspace = workspace_packet();
        let mut input = baseline_input();
        input.object_rows.clear();
        let packet =
            ProviderLinkedReviewStabilizationPacket::from_workspace_packet(input, &workspace);
        assert!(packet.is_err());
    }
}
