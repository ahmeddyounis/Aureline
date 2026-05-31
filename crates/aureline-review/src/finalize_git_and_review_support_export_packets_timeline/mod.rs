//! Finalized Git and review support-export packets, timeline/chronology truth,
//! and operator playbooks.
//!
//! This module owns the bounded contract that makes a review workspace's Git
//! and review history exportable as one canonical, redaction-safe packet. It
//! binds three concerns into a single previewable, attributable, and reversible
//! artifact:
//!
//! - **Timeline / chronology truth.** Every history event is modeled with an
//!   explicit, strictly increasing `sequence_index`, an explicit clock source
//!   (`local_monotonic`, `local_wall_clock`, `provider_reported`,
//!   `imported_bundle`, `reconstructed_from_lineage`), an acting identity, an
//!   event source, and a lineage parent. Chronology is canonical truth — never
//!   approximated and never silently re-ordered. Events that originate from
//!   hosted or provider authority must disclose it.
//! - **Operator playbooks.** Operator runbooks are modeled as ordered steps.
//!   Every mutating step is previewable and either reversible or
//!   checkpoint-backed, every step carries an explicit authority class, and a
//!   step that would broaden the operator's authority is blocked rather than
//!   silently executable.
//! - **Finalized support-export packet.** The export packet cites the source
//!   schemas, carries reopen context, and keeps every `raw_*_export_allowed`
//!   flag false so raw URLs and raw provider payloads never cross the support
//!   boundary.
//!
//! The record family includes:
//!
//! - [`GitReviewTimelineTruthRecord`] — stable identity binding the review
//!   workspace, chronology state, and derived blocking/invalidation reasons.
//! - [`TimelineEventRecord`] — one chronology event with explicit ordering,
//!   clock source, actor, source, lineage, freshness, and hosted-authority
//!   disclosure.
//! - [`OperatorPlaybookRecord`] — an operator runbook with an ordered step set.
//! - [`OperatorPlaybookStepRecord`] — one runbook step with explicit command
//!   class, authority class, preview/reversibility/checkpoint posture, and
//!   hosted-authority disclosure.
//! - [`GitReviewSupportExportPacket`] — redaction-safe support export that
//!   preserves timeline and playbook lineage and citation.
//! - [`GitReviewTimelineInspectionRecord`] — compact boolean projection for CLI
//!   and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/git_review_support_export_timeline.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/finalize-git-and-review-support-export-packets-timeline/`.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every Git/review timeline record.
pub const GIT_REVIEW_TIMELINE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`GitReviewSupportExportTimelinePacket`].
pub const GIT_REVIEW_TIMELINE_PACKET_RECORD_KIND: &str = "git_review_timeline_packet";

/// Stable record-kind tag for [`GitReviewTimelineTruthRecord`].
pub const GIT_REVIEW_TIMELINE_TRUTH_RECORD_KIND: &str = "git_review_timeline_truth_record";

/// Stable record-kind tag for [`TimelineEventRecord`].
pub const TIMELINE_EVENT_RECORD_KIND: &str = "timeline_event_record";

/// Stable record-kind tag for [`OperatorPlaybookRecord`].
pub const OPERATOR_PLAYBOOK_RECORD_KIND: &str = "operator_playbook_record";

/// Stable record-kind tag for [`OperatorPlaybookStepRecord`].
pub const OPERATOR_PLAYBOOK_STEP_RECORD_KIND: &str = "operator_playbook_step_record";

/// Stable record-kind tag for [`GitReviewSupportExportPacket`].
pub const GIT_REVIEW_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "git_review_support_export_packet";

/// Stable record-kind tag for [`GitReviewTimelineInspectionRecord`].
pub const GIT_REVIEW_TIMELINE_INSPECTION_RECORD_KIND: &str =
    "git_review_timeline_inspection_record";

/// Closed set of chronology states.
pub const CHRONOLOGY_STATES: &[&str] = &[
    "chronology_current",
    "chronology_stale",
    "chronology_reconstructed",
    "chronology_gap_detected",
];

/// Closed set of clock source classes for timeline events.
pub const TIMELINE_CLOCK_SOURCE_CLASSES: &[&str] = &[
    "local_monotonic",
    "local_wall_clock",
    "provider_reported",
    "imported_bundle",
    "reconstructed_from_lineage",
];

/// Closed set of event source classes for timeline events.
pub const TIMELINE_EVENT_SOURCE_CLASSES: &[&str] = &[
    "local_git",
    "review_workspace",
    "provider_linked",
    "browser_handoff",
    "migration_import",
    "ai_evidence",
];

/// Closed set of event kinds for timeline events.
pub const TIMELINE_EVENT_KINDS: &[&str] = &[
    "commit_recorded",
    "review_state_transition",
    "merge_landed",
    "rebase_applied",
    "checkpoint_created",
    "provider_publish",
    "import_applied",
    "comment_posted",
    "approval_recorded",
    "approval_invalidated",
];

/// Closed set of freshness classes for timeline events.
pub const TIMELINE_FRESHNESS_CLASSES: &[&str] =
    &["current", "stale", "superseded", "unverified"];

/// Closed set of operator playbook states.
pub const OPERATOR_PLAYBOOK_STATES: &[&str] = &[
    "draft",
    "ready",
    "in_progress",
    "completed",
    "blocked",
];

/// Closed set of operator playbook step command classes.
pub const PLAYBOOK_STEP_COMMAND_CLASSES: &[&str] = &[
    "inspect",
    "preview",
    "apply_with_checkpoint",
    "revert",
    "export",
    "handoff",
    "escalate",
];

/// Closed set of operator playbook step authority classes.
pub const PLAYBOOK_STEP_AUTHORITY_CLASSES: &[&str] = &[
    "advisory_only",
    "previewable_local_apply",
    "checkpointed_reversible",
    "requires_human_approval",
    "hosted_provider_mutation",
];

/// Closed set of consumer surfaces for timeline packets.
pub const GIT_REVIEW_TIMELINE_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
    "operator_playbook_view",
];

/// Closed set of invalidation reasons that narrow chronology truth.
pub const GIT_REVIEW_TIMELINE_INVALIDATION_REASONS: &[&str] = &[
    "chronology_gap",
    "clock_source_unverified",
    "event_unattributed",
    "non_monotonic_ordering",
    "playbook_step_authority_exceeded",
    "support_export_stale",
    "lineage_break",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a Git/review timeline packet to materialize on top of a
/// beta review-workspace packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewTimelineInput {
    /// Stable timeline identity.
    pub timeline_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Chronology state from the closed vocabulary.
    pub chronology_state: String,
    /// Ordered timeline event inputs.
    pub timeline_events: Vec<TimelineEventInput>,
    /// Operator playbook inputs.
    pub playbooks: Vec<OperatorPlaybookInput>,
    /// Operator playbook step inputs.
    pub playbook_steps: Vec<OperatorPlaybookStepInput>,
    /// Support/export envelope input.
    pub support_export: GitReviewSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing one timeline event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineEventInput {
    /// Stable event identity.
    pub event_id: String,
    /// Monotonic sequence index; must strictly increase across the timeline.
    pub sequence_index: u64,
    /// Event kind from the closed vocabulary.
    pub event_kind: String,
    /// Event source class from the closed vocabulary.
    pub event_source_class: String,
    /// Clock source class from the closed vocabulary.
    pub clock_source_class: String,
    /// Timestamp recorded for this event.
    pub recorded_at: String,
    /// Opaque actor ref responsible for this event.
    pub actor_ref: String,
    /// Target object ref this event concerns.
    pub target_object_ref: String,
    /// Optional lineage parent event id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_parent_ref: Option<String>,
    /// Freshness class for this event.
    pub freshness_class: String,
    /// True when this event can be reversed or undone.
    pub reversible: bool,
    /// True when this event discloses that it originates from hosted/provider
    /// authority. Required for provider-linked and browser-handoff sources.
    pub discloses_hosted_authority: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one operator playbook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorPlaybookInput {
    /// Stable playbook identity.
    pub playbook_id: String,
    /// Playbook state from the closed vocabulary.
    pub playbook_state: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one operator playbook step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorPlaybookStepInput {
    /// Stable step identity.
    pub step_id: String,
    /// Playbook this step belongs to.
    pub playbook_ref: String,
    /// Monotonic step index within the playbook; must strictly increase.
    pub step_index: u64,
    /// Command class from the closed vocabulary.
    pub command_class: String,
    /// Authority class from the closed vocabulary.
    pub authority_class: String,
    /// True when the step supports preview/dry-run.
    pub preview_supported: bool,
    /// True when the step is reversible after execution.
    pub reversible: bool,
    /// True when the step creates a checkpoint before execution.
    pub checkpoint_required: bool,
    /// True when the step discloses hosted/provider authority.
    pub discloses_hosted_authority: bool,
    /// True when executing the step would broaden the operator's authority.
    pub would_broaden_authority: bool,
    /// Target object ref the step would act on.
    pub target_object_ref: String,
    /// Active blocked reasons preventing execution; empty when actionable.
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input row for the Git/review support-export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the timeline packet.
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

/// Git/review timeline truth record materialized from input plus workspace
/// truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewTimelineTruthRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable timeline identity.
    pub timeline_id: String,
    /// Review workspace this timeline belongs to.
    pub review_workspace_id_ref: String,
    /// Chronology state.
    pub chronology_state: String,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing operator action.
    pub blocked_reasons: Vec<String>,
    /// True when the timeline packet is actionable from the current state.
    pub actionable: bool,
    /// Timestamp the timeline packet was frozen.
    pub generated_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Timeline event record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineEventRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Timeline this event belongs to.
    pub timeline_id_ref: String,
    /// Stable event identity.
    pub event_id: String,
    /// Monotonic sequence index.
    pub sequence_index: u64,
    /// Event kind.
    pub event_kind: String,
    /// Event source class.
    pub event_source_class: String,
    /// Clock source class.
    pub clock_source_class: String,
    /// Timestamp recorded for this event.
    pub recorded_at: String,
    /// Opaque actor ref responsible for this event.
    pub actor_ref: String,
    /// Target object ref this event concerns.
    pub target_object_ref: String,
    /// Optional lineage parent event id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_parent_ref: Option<String>,
    /// Freshness class for this event.
    pub freshness_class: String,
    /// True when this event can be reversed or undone.
    pub reversible: bool,
    /// True when this event discloses hosted/provider authority.
    pub discloses_hosted_authority: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Operator playbook record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorPlaybookRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Timeline this playbook belongs to.
    pub timeline_id_ref: String,
    /// Stable playbook identity.
    pub playbook_id: String,
    /// Playbook state.
    pub playbook_state: String,
    /// Number of steps attached to this playbook.
    pub step_count: usize,
    /// Active blocking reasons preventing playbook progress.
    pub blocked_reasons: Vec<String>,
    /// True when the playbook is actionable from the current state.
    pub actionable: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Operator playbook step record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorPlaybookStepRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Timeline this step belongs to.
    pub timeline_id_ref: String,
    /// Playbook this step belongs to.
    pub playbook_id_ref: String,
    /// Stable step identity.
    pub step_id: String,
    /// Monotonic step index within the playbook.
    pub step_index: u64,
    /// Command class.
    pub command_class: String,
    /// Authority class.
    pub authority_class: String,
    /// True when the step supports preview/dry-run.
    pub preview_supported: bool,
    /// True when the step is reversible after execution.
    pub reversible: bool,
    /// True when the step creates a checkpoint before execution.
    pub checkpoint_required: bool,
    /// True when the step discloses hosted/provider authority.
    pub discloses_hosted_authority: bool,
    /// True when executing the step would broaden the operator's authority.
    pub would_broaden_authority: bool,
    /// Target object ref the step would act on.
    pub target_object_ref: String,
    /// Active blocked reasons preventing execution.
    pub blocked_reasons: Vec<String>,
    /// True when the step is actionable from the current state.
    pub actionable: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Support/export packet for the Git/review timeline lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Timeline this packet exports.
    pub timeline_id_ref: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Stable context ref used to reopen the timeline packet.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Timeline event ids exported in this packet.
    pub timeline_event_id_refs: Vec<String>,
    /// Playbook ids exported in this packet.
    pub playbook_id_refs: Vec<String>,
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
pub struct GitReviewTimelineInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Timeline inspected by this row.
    pub timeline_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the chronology state is chronology_current.
    pub chronology_current: bool,
    /// True when the chronology state is chronology_stale.
    pub chronology_stale: bool,
    /// True when the chronology state is chronology_reconstructed.
    pub chronology_reconstructed: bool,
    /// True when the chronology state is chronology_gap_detected.
    pub chronology_gap_detected: bool,
    /// True when timeline events strictly increase by sequence index.
    pub monotonic_ordering_preserved: bool,
    /// True when every event carries a non-empty actor ref.
    pub all_events_attributed: bool,
    /// True when every event clock source is from the closed vocabulary.
    pub all_events_have_clock_source: bool,
    /// True when every hosted/provider event discloses hosted authority.
    pub all_hosted_events_disclosed: bool,
    /// True when every event lineage parent resolves to a known event.
    pub lineage_resolves: bool,
    /// True when every mutating playbook step is reversible or checkpointed.
    pub all_mutating_steps_reversible_or_checkpointed: bool,
    /// True when every mutating playbook step supports preview.
    pub all_mutating_steps_previewable: bool,
    /// True when every hosted-authority playbook step discloses it.
    pub all_hosted_steps_disclosed: bool,
    /// True when no playbook step would broaden the operator's authority.
    pub no_authority_broadening: bool,
    /// True when the timeline packet is actionable.
    pub actionable: bool,
    /// True when the timeline packet is invalidated by any reason.
    pub invalidated: bool,
    /// Number of timeline event records.
    pub timeline_event_count: usize,
    /// Number of operator playbook records.
    pub playbook_count: usize,
    /// Number of operator playbook step records.
    pub playbook_step_count: usize,
    /// True when support/export can reopen the timeline context.
    pub support_export_reopenable: bool,
    /// True when no raw escape hatch crosses the support boundary.
    pub raw_escape_hatches_absent: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Git/review support-export timeline packet consumed by review surfaces and
/// support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewSupportExportTimelinePacket {
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
    /// Git/review timeline truth record.
    pub timeline_truth: GitReviewTimelineTruthRecord,
    /// Timeline event records.
    pub timeline_events: Vec<TimelineEventRecord>,
    /// Operator playbook records.
    pub playbooks: Vec<OperatorPlaybookRecord>,
    /// Operator playbook step records.
    pub playbook_steps: Vec<OperatorPlaybookStepRecord>,
    /// Support/export packet.
    pub support_export: GitReviewSupportExportPacket,
    /// Inspection row.
    pub inspection: GitReviewTimelineInspectionRecord,
}

impl GitReviewSupportExportTimelinePacket {
    /// Builds a Git/review timeline packet from a beta review-workspace packet
    /// and timeline input.
    ///
    /// # Errors
    ///
    /// Returns [`GitReviewTimelineValidationError`] when the input violates a
    /// timeline, playbook, or support-export invariant.
    pub fn from_workspace_packet(
        input: GitReviewTimelineInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
    ) -> Result<Self, GitReviewTimelineValidationError> {
        validate_input(&input, workspace_packet)?;

        let timeline_truth = timeline_truth_record(&input, workspace_packet);
        let timeline_events = input
            .timeline_events
            .iter()
            .map(|e| event_record(e, &timeline_truth))
            .collect::<Vec<_>>();
        let playbook_steps = input
            .playbook_steps
            .iter()
            .map(|s| step_record(s, &timeline_truth))
            .collect::<Vec<_>>();
        let playbooks = input
            .playbooks
            .iter()
            .map(|p| playbook_record(p, &timeline_truth, &playbook_steps))
            .collect::<Vec<_>>();
        let support_export = support_export_packet(
            &input.support_export,
            &timeline_truth,
            workspace_packet,
            &timeline_events,
            &playbooks,
        );
        let inspection = inspection_record(
            &timeline_truth,
            &timeline_events,
            &playbooks,
            &playbook_steps,
            &support_export,
        );

        let packet = Self {
            record_kind: GIT_REVIEW_TIMELINE_PACKET_RECORD_KIND.to_string(),
            schema_version: GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            timeline_truth,
            timeline_events,
            playbooks,
            playbook_steps,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the Git/review timeline invariants.
    ///
    /// # Errors
    ///
    /// Returns [`GitReviewTimelineValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), GitReviewTimelineValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            GIT_REVIEW_TIMELINE_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_truth_record(&self.timeline_truth, &self.review_workspace.review_workspace_id)?;
        for event in &self.timeline_events {
            validate_event_record(event, &self.timeline_truth.timeline_id)?;
        }
        for playbook in &self.playbooks {
            validate_playbook_record(playbook, &self.timeline_truth.timeline_id)?;
        }
        for step in &self.playbook_steps {
            validate_step_record(step, &self.timeline_truth.timeline_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.timeline_truth,
            &self.timeline_events,
            &self.playbooks,
        )?;
        validate_inspection(&self.inspection, self)?;

        // Chronology truth: events must strictly increase by sequence index.
        let mut previous: Option<u64> = None;
        for event in &self.timeline_events {
            if let Some(prev) = previous {
                if event.sequence_index <= prev {
                    return Err(git_review_timeline_validation_error(format!(
                        "timeline event {} breaks monotonic ordering (sequence_index {} <= {prev})",
                        event.event_id, event.sequence_index
                    )));
                }
            }
            previous = Some(event.sequence_index);
        }

        // Attribution truth: every event must carry an actor and a hosted
        // disclosure when it originates from hosted/provider authority.
        for event in &self.timeline_events {
            ensure_nonempty(&event.actor_ref, "timeline event actor_ref")?;
            if event_is_hosted(event) && !event.discloses_hosted_authority {
                return Err(git_review_timeline_validation_error(format!(
                    "timeline event {} originates from hosted authority but does not disclose it",
                    event.event_id
                )));
            }
        }

        // Lineage truth: every lineage parent must resolve to a known event.
        let event_ids: std::collections::BTreeSet<&str> = self
            .timeline_events
            .iter()
            .map(|e| e.event_id.as_str())
            .collect();
        for event in &self.timeline_events {
            if let Some(ref parent) = event.lineage_parent_ref {
                if !event_ids.contains(parent.as_str()) {
                    return Err(git_review_timeline_validation_error(format!(
                        "timeline event {} cites unknown lineage_parent_ref {parent}",
                        event.event_id
                    )));
                }
            }
        }

        // Playbook step coherence: every step must reference a known playbook.
        let playbook_ids: std::collections::BTreeSet<&str> = self
            .playbooks
            .iter()
            .map(|p| p.playbook_id.as_str())
            .collect();
        for step in &self.playbook_steps {
            if !playbook_ids.contains(step.playbook_id_ref.as_str()) {
                return Err(git_review_timeline_validation_error(format!(
                    "playbook step {} cites unknown playbook_id_ref {}",
                    step.step_id, step.playbook_id_ref
                )));
            }
        }

        // Step indices must strictly increase within each playbook.
        for playbook in &self.playbooks {
            let mut prev: Option<u64> = None;
            for step in self
                .playbook_steps
                .iter()
                .filter(|s| s.playbook_id_ref == playbook.playbook_id)
            {
                if let Some(p) = prev {
                    if step.step_index <= p {
                        return Err(git_review_timeline_validation_error(format!(
                            "playbook {} step {} breaks monotonic ordering",
                            playbook.playbook_id, step.step_id
                        )));
                    }
                }
                prev = Some(step.step_index);
            }
        }

        // Authority truth: a mutating step must be previewable and either
        // reversible or checkpoint-backed; a hosted-authority step must
        // disclose it; an authority-broadening step must never be actionable.
        for step in &self.playbook_steps {
            if step_is_mutating(step) {
                if !step.preview_supported {
                    return Err(git_review_timeline_validation_error(format!(
                        "playbook step {} mutates but does not support preview",
                        step.step_id
                    )));
                }
                if !step.reversible && !step.checkpoint_required {
                    return Err(git_review_timeline_validation_error(format!(
                        "playbook step {} mutates but is neither reversible nor checkpointed",
                        step.step_id
                    )));
                }
            }
            if step.authority_class == "hosted_provider_mutation" && !step.discloses_hosted_authority
            {
                return Err(git_review_timeline_validation_error(format!(
                    "playbook step {} claims hosted authority but does not disclose it",
                    step.step_id
                )));
            }
            if step.would_broaden_authority && step.actionable {
                return Err(git_review_timeline_validation_error(format!(
                    "playbook step {} would broaden authority but is actionable",
                    step.step_id
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

    /// Returns true when timeline events strictly increase by sequence index.
    pub fn monotonic_ordering_preserved(&self) -> bool {
        let mut previous: Option<u64> = None;
        for event in &self.timeline_events {
            if let Some(prev) = previous {
                if event.sequence_index <= prev {
                    return false;
                }
            }
            previous = Some(event.sequence_index);
        }
        true
    }

    /// Returns true when every event carries a non-empty actor ref.
    pub fn all_events_attributed(&self) -> bool {
        self.timeline_events
            .iter()
            .all(|e| !e.actor_ref.trim().is_empty())
    }

    /// Returns true when every hosted/provider event discloses hosted authority.
    pub fn hosted_events_disclosed(&self) -> bool {
        self.timeline_events
            .iter()
            .all(|e| !event_is_hosted(e) || e.discloses_hosted_authority)
    }

    /// Returns true when no playbook step would broaden the operator's authority.
    pub fn no_authority_broadening(&self) -> bool {
        !self
            .playbook_steps
            .iter()
            .any(|s| s.would_broaden_authority)
    }
}

// ---------------------------------------------------------------------------
// Projection type
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitReviewTimelineProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timeline identity.
    pub timeline_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Chronology state.
    pub chronology_state: String,
    /// True when the packet is actionable.
    pub actionable: bool,
    /// True when timeline events strictly increase by sequence index.
    pub monotonic_ordering_preserved: bool,
    /// True when every event carries a non-empty actor ref.
    pub all_events_attributed: bool,
    /// True when every hosted/provider event discloses hosted authority.
    pub all_hosted_events_disclosed: bool,
    /// True when no playbook step would broaden authority.
    pub no_authority_broadening: bool,
    /// Number of timeline events.
    pub timeline_event_count: usize,
    /// Number of playbooks.
    pub playbook_count: usize,
    /// Number of playbook steps.
    pub playbook_step_count: usize,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for Git/review timeline operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitReviewTimelineError {
    /// Validation failed.
    Validation(GitReviewTimelineValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for GitReviewTimelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for GitReviewTimelineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for Git/review timeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitReviewTimelineValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for GitReviewTimelineValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GitReviewTimelineValidationError {}

impl GitReviewTimelineValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Parses and validates a materialized Git/review timeline packet.
///
/// # Errors
///
/// Returns [`GitReviewTimelineError`] when the payload fails to parse or
/// violates the Git/review timeline invariants.
pub fn project_git_review_timeline_packet(
    payload: &str,
) -> Result<GitReviewTimelineProjection, GitReviewTimelineError> {
    let packet: GitReviewSupportExportTimelinePacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(GitReviewTimelineProjection::from(packet))
}

impl From<GitReviewSupportExportTimelinePacket> for GitReviewTimelineProjection {
    fn from(packet: GitReviewSupportExportTimelinePacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            timeline_id: packet.timeline_truth.timeline_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            chronology_state: packet.timeline_truth.chronology_state,
            actionable: packet.timeline_truth.actionable,
            monotonic_ordering_preserved: packet.inspection.monotonic_ordering_preserved,
            all_events_attributed: packet.inspection.all_events_attributed,
            all_hosted_events_disclosed: packet.inspection.all_hosted_events_disclosed,
            no_authority_broadening: packet.inspection.no_authority_broadening,
            timeline_event_count: packet.timeline_events.len(),
            playbook_count: packet.playbooks.len(),
            playbook_step_count: packet.playbook_steps.len(),
            invalidation_reasons: packet.timeline_truth.invalidation_reasons.clone(),
            blocked_reasons: packet.timeline_truth.blocked_reasons.clone(),
        }
    }
}

impl From<serde_json::Error> for GitReviewTimelineError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(GitReviewTimelineValidationError {
            message: err.to_string(),
        })
    }
}

impl From<GitReviewTimelineValidationError> for GitReviewTimelineError {
    fn from(err: GitReviewTimelineValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn event_is_hosted(event: &TimelineEventRecord) -> bool {
    event.event_source_class == "provider_linked"
        || event.event_source_class == "browser_handoff"
        || event.event_kind == "provider_publish"
}

fn step_is_mutating(step: &OperatorPlaybookStepRecord) -> bool {
    step.command_class == "apply_with_checkpoint" || step.command_class == "revert"
}

fn timeline_truth_record(
    input: &GitReviewTimelineInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> GitReviewTimelineTruthRecord {
    let mut invalidation_reasons = input.invalidation_reasons.clone();
    for event in &input.timeline_events {
        match event.freshness_class.as_str() {
            "unverified" => push_unique(&mut invalidation_reasons, "clock_source_unverified"),
            "stale" | "superseded" => push_unique(&mut invalidation_reasons, "chronology_gap"),
            _ => {}
        }
        if event.actor_ref.trim().is_empty() {
            push_unique(&mut invalidation_reasons, "event_unattributed");
        }
    }
    for step in &input.playbook_steps {
        if step.would_broaden_authority {
            push_unique(&mut invalidation_reasons, "playbook_step_authority_exceeded");
        }
    }
    if input.chronology_state == "chronology_gap_detected" {
        push_unique(&mut invalidation_reasons, "chronology_gap");
    }
    invalidation_reasons.sort();
    invalidation_reasons.dedup();

    let mut blocked_reasons = Vec::new();
    if input.chronology_state == "chronology_gap_detected" {
        blocked_reasons.push("chronology_unreliable".to_string());
    }
    for step in &input.playbook_steps {
        if step.would_broaden_authority {
            blocked_reasons.push("authority_exceeded".to_string());
        }
    }
    if invalidation_reasons
        .iter()
        .any(|r| r == "clock_source_unverified")
    {
        blocked_reasons.push("clock_source_unverified".to_string());
    }
    blocked_reasons.sort();
    blocked_reasons.dedup();

    GitReviewTimelineTruthRecord {
        record_kind: GIT_REVIEW_TIMELINE_TRUTH_RECORD_KIND.to_string(),
        schema_version: GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
        timeline_id: input.timeline_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        chronology_state: input.chronology_state.clone(),
        invalidation_reasons,
        actionable: blocked_reasons.is_empty(),
        blocked_reasons,
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn event_record(
    input: &TimelineEventInput,
    truth: &GitReviewTimelineTruthRecord,
) -> TimelineEventRecord {
    TimelineEventRecord {
        record_kind: TIMELINE_EVENT_RECORD_KIND.to_string(),
        schema_version: GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
        timeline_id_ref: truth.timeline_id.clone(),
        event_id: input.event_id.clone(),
        sequence_index: input.sequence_index,
        event_kind: input.event_kind.clone(),
        event_source_class: input.event_source_class.clone(),
        clock_source_class: input.clock_source_class.clone(),
        recorded_at: input.recorded_at.clone(),
        actor_ref: input.actor_ref.clone(),
        target_object_ref: input.target_object_ref.clone(),
        lineage_parent_ref: input.lineage_parent_ref.clone(),
        freshness_class: input.freshness_class.clone(),
        reversible: input.reversible,
        discloses_hosted_authority: input.discloses_hosted_authority,
        summary_label: input.summary_label.clone(),
    }
}

fn step_record(
    input: &OperatorPlaybookStepInput,
    truth: &GitReviewTimelineTruthRecord,
) -> OperatorPlaybookStepRecord {
    let actionable = input.blocked_reasons.is_empty() && !input.would_broaden_authority;
    OperatorPlaybookStepRecord {
        record_kind: OPERATOR_PLAYBOOK_STEP_RECORD_KIND.to_string(),
        schema_version: GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
        timeline_id_ref: truth.timeline_id.clone(),
        playbook_id_ref: input.playbook_ref.clone(),
        step_id: input.step_id.clone(),
        step_index: input.step_index,
        command_class: input.command_class.clone(),
        authority_class: input.authority_class.clone(),
        preview_supported: input.preview_supported,
        reversible: input.reversible,
        checkpoint_required: input.checkpoint_required,
        discloses_hosted_authority: input.discloses_hosted_authority,
        would_broaden_authority: input.would_broaden_authority,
        target_object_ref: input.target_object_ref.clone(),
        blocked_reasons: input.blocked_reasons.clone(),
        actionable,
        summary_label: input.summary_label.clone(),
    }
}

fn playbook_record(
    input: &OperatorPlaybookInput,
    truth: &GitReviewTimelineTruthRecord,
    steps: &[OperatorPlaybookStepRecord],
) -> OperatorPlaybookRecord {
    let owned_steps: Vec<&OperatorPlaybookStepRecord> = steps
        .iter()
        .filter(|s| s.playbook_id_ref == input.playbook_id)
        .collect();
    let mut blocked_reasons = Vec::new();
    if owned_steps.iter().any(|s| s.would_broaden_authority) {
        blocked_reasons.push("authority_exceeded".to_string());
    }
    if owned_steps.iter().any(|s| !s.blocked_reasons.is_empty()) {
        blocked_reasons.push("step_blocked".to_string());
    }
    if input.playbook_state == "blocked" {
        blocked_reasons.push("playbook_blocked".to_string());
    }
    blocked_reasons.sort();
    blocked_reasons.dedup();

    OperatorPlaybookRecord {
        record_kind: OPERATOR_PLAYBOOK_RECORD_KIND.to_string(),
        schema_version: GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
        timeline_id_ref: truth.timeline_id.clone(),
        playbook_id: input.playbook_id.clone(),
        playbook_state: input.playbook_state.clone(),
        step_count: owned_steps.len(),
        actionable: blocked_reasons.is_empty(),
        blocked_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn support_export_packet(
    input: &GitReviewSupportExportInput,
    truth: &GitReviewTimelineTruthRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    events: &[TimelineEventRecord],
    playbooks: &[OperatorPlaybookRecord],
) -> GitReviewSupportExportPacket {
    GitReviewSupportExportPacket {
        record_kind: GIT_REVIEW_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        timeline_id_ref: truth.timeline_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        timeline_event_id_refs: events.iter().map(|e| e.event_id.clone()).collect(),
        playbook_id_refs: playbooks.iter().map(|p| p.playbook_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/git_review_support_export_timeline.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn inspection_record(
    truth: &GitReviewTimelineTruthRecord,
    events: &[TimelineEventRecord],
    playbooks: &[OperatorPlaybookRecord],
    steps: &[OperatorPlaybookStepRecord],
    support_export: &GitReviewSupportExportPacket,
) -> GitReviewTimelineInspectionRecord {
    let mut monotonic = true;
    let mut previous: Option<u64> = None;
    for event in events {
        if let Some(prev) = previous {
            if event.sequence_index <= prev {
                monotonic = false;
            }
        }
        previous = Some(event.sequence_index);
    }

    let event_ids: std::collections::BTreeSet<&str> =
        events.iter().map(|e| e.event_id.as_str()).collect();
    let lineage_resolves = events.iter().all(|e| {
        e.lineage_parent_ref
            .as_ref()
            .map(|p| event_ids.contains(p.as_str()))
            .unwrap_or(true)
    });

    let mutating_steps: Vec<&OperatorPlaybookStepRecord> =
        steps.iter().filter(|s| step_is_mutating(s)).collect();

    GitReviewTimelineInspectionRecord {
        record_kind: GIT_REVIEW_TIMELINE_INSPECTION_RECORD_KIND.to_string(),
        schema_version: GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
        timeline_id_ref: truth.timeline_id.clone(),
        review_workspace_id_ref: truth.review_workspace_id_ref.clone(),
        chronology_current: truth.chronology_state == "chronology_current",
        chronology_stale: truth.chronology_state == "chronology_stale",
        chronology_reconstructed: truth.chronology_state == "chronology_reconstructed",
        chronology_gap_detected: truth.chronology_state == "chronology_gap_detected",
        monotonic_ordering_preserved: monotonic,
        all_events_attributed: events.iter().all(|e| !e.actor_ref.trim().is_empty()),
        all_events_have_clock_source: events
            .iter()
            .all(|e| contains_token(TIMELINE_CLOCK_SOURCE_CLASSES, &e.clock_source_class)),
        all_hosted_events_disclosed: events
            .iter()
            .all(|e| !event_is_hosted(e) || e.discloses_hosted_authority),
        lineage_resolves,
        all_mutating_steps_reversible_or_checkpointed: mutating_steps
            .iter()
            .all(|s| s.reversible || s.checkpoint_required),
        all_mutating_steps_previewable: mutating_steps.iter().all(|s| s.preview_supported),
        all_hosted_steps_disclosed: steps
            .iter()
            .all(|s| s.authority_class != "hosted_provider_mutation" || s.discloses_hosted_authority),
        no_authority_broadening: !steps.iter().any(|s| s.would_broaden_authority),
        actionable: truth.actionable,
        invalidated: !truth.invalidation_reasons.is_empty(),
        timeline_event_count: events.len(),
        playbook_count: playbooks.len(),
        playbook_step_count: steps.len(),
        support_export_reopenable: !support_export.reopen_context_ref.is_empty(),
        raw_escape_hatches_absent: !support_export.raw_url_export_allowed
            && !support_export.raw_provider_payload_export_allowed,
        summary_label: truth.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &GitReviewTimelineInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), GitReviewTimelineValidationError> {
    ensure_nonempty(&input.timeline_id, "timeline_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.chronology_state, "chronology_state")?;
    ensure_token(CHRONOLOGY_STATES, &input.chronology_state, "chronology_state")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    for reason in &input.invalidation_reasons {
        ensure_token(
            GIT_REVIEW_TIMELINE_INVALIDATION_REASONS,
            reason,
            "invalidation_reason",
        )?;
    }

    if input.timeline_events.is_empty() {
        return Err(git_review_timeline_validation_error(
            "input must contain at least one timeline_event".to_string(),
        ));
    }

    let mut event_ids = std::collections::BTreeSet::new();
    let mut sequence_indices = std::collections::BTreeSet::new();
    for event in &input.timeline_events {
        ensure_nonempty(&event.event_id, "timeline_event.event_id")?;
        if !event_ids.insert(&event.event_id) {
            return Err(git_review_timeline_validation_error(format!(
                "duplicate event_id: {}",
                event.event_id
            )));
        }
        if !sequence_indices.insert(event.sequence_index) {
            return Err(git_review_timeline_validation_error(format!(
                "duplicate sequence_index: {}",
                event.sequence_index
            )));
        }
        ensure_token(TIMELINE_EVENT_KINDS, &event.event_kind, "timeline_event.event_kind")?;
        ensure_token(
            TIMELINE_EVENT_SOURCE_CLASSES,
            &event.event_source_class,
            "timeline_event.event_source_class",
        )?;
        ensure_token(
            TIMELINE_CLOCK_SOURCE_CLASSES,
            &event.clock_source_class,
            "timeline_event.clock_source_class",
        )?;
        ensure_token(
            TIMELINE_FRESHNESS_CLASSES,
            &event.freshness_class,
            "timeline_event.freshness_class",
        )?;
        ensure_nonempty(&event.actor_ref, "timeline_event.actor_ref")?;
    }

    let mut playbook_ids = std::collections::BTreeSet::new();
    for playbook in &input.playbooks {
        ensure_nonempty(&playbook.playbook_id, "playbook.playbook_id")?;
        if !playbook_ids.insert(&playbook.playbook_id) {
            return Err(git_review_timeline_validation_error(format!(
                "duplicate playbook_id: {}",
                playbook.playbook_id
            )));
        }
        ensure_token(
            OPERATOR_PLAYBOOK_STATES,
            &playbook.playbook_state,
            "playbook.playbook_state",
        )?;
    }

    let mut step_ids = std::collections::BTreeSet::new();
    for step in &input.playbook_steps {
        ensure_nonempty(&step.step_id, "playbook_step.step_id")?;
        if !step_ids.insert(&step.step_id) {
            return Err(git_review_timeline_validation_error(format!(
                "duplicate step_id: {}",
                step.step_id
            )));
        }
        if !playbook_ids.contains(&step.playbook_ref) {
            return Err(git_review_timeline_validation_error(format!(
                "playbook_step {} references unknown playbook_ref {}",
                step.step_id, step.playbook_ref
            )));
        }
        ensure_token(
            PLAYBOOK_STEP_COMMAND_CLASSES,
            &step.command_class,
            "playbook_step.command_class",
        )?;
        ensure_token(
            PLAYBOOK_STEP_AUTHORITY_CLASSES,
            &step.authority_class,
            "playbook_step.authority_class",
        )?;
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
            GIT_REVIEW_TIMELINE_CONSUMER_SURFACES,
            surface,
            "support_export.consumer_surfaces",
        )?;
    }

    ensure_nonempty(
        &workspace_packet.review_workspace.review_workspace_id,
        "workspace_packet.review_workspace.review_workspace_id",
    )?;

    Ok(())
}

fn validate_truth_record(
    record: &GitReviewTimelineTruthRecord,
    workspace_id: &str,
) -> Result<(), GitReviewTimelineValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        GIT_REVIEW_TIMELINE_TRUTH_RECORD_KIND,
        "timeline truth record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        GIT_REVIEW_TIMELINE_SCHEMA_VERSION,
        "timeline truth schema_version",
    )?;
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        workspace_id,
        "timeline truth review_workspace_id_ref",
    )?;
    ensure_token(CHRONOLOGY_STATES, &record.chronology_state, "timeline truth chronology_state")?;
    for reason in &record.invalidation_reasons {
        ensure_token(
            GIT_REVIEW_TIMELINE_INVALIDATION_REASONS,
            reason,
            "timeline truth invalidation_reason",
        )?;
    }
    Ok(())
}

fn validate_event_record(
    record: &TimelineEventRecord,
    timeline_id: &str,
) -> Result<(), GitReviewTimelineValidationError> {
    ensure_eq(record.record_kind.as_str(), TIMELINE_EVENT_RECORD_KIND, "event record_kind")?;
    ensure_eq(record.timeline_id_ref.as_str(), timeline_id, "event timeline_id_ref")?;
    ensure_token(TIMELINE_EVENT_KINDS, &record.event_kind, "event event_kind")?;
    ensure_token(
        TIMELINE_EVENT_SOURCE_CLASSES,
        &record.event_source_class,
        "event event_source_class",
    )?;
    ensure_token(
        TIMELINE_CLOCK_SOURCE_CLASSES,
        &record.clock_source_class,
        "event clock_source_class",
    )?;
    ensure_token(TIMELINE_FRESHNESS_CLASSES, &record.freshness_class, "event freshness_class")?;
    Ok(())
}

fn validate_playbook_record(
    record: &OperatorPlaybookRecord,
    timeline_id: &str,
) -> Result<(), GitReviewTimelineValidationError> {
    ensure_eq(record.record_kind.as_str(), OPERATOR_PLAYBOOK_RECORD_KIND, "playbook record_kind")?;
    ensure_eq(record.timeline_id_ref.as_str(), timeline_id, "playbook timeline_id_ref")?;
    ensure_token(OPERATOR_PLAYBOOK_STATES, &record.playbook_state, "playbook playbook_state")?;
    Ok(())
}

fn validate_step_record(
    record: &OperatorPlaybookStepRecord,
    timeline_id: &str,
) -> Result<(), GitReviewTimelineValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        OPERATOR_PLAYBOOK_STEP_RECORD_KIND,
        "playbook step record_kind",
    )?;
    ensure_eq(record.timeline_id_ref.as_str(), timeline_id, "playbook step timeline_id_ref")?;
    ensure_token(
        PLAYBOOK_STEP_COMMAND_CLASSES,
        &record.command_class,
        "playbook step command_class",
    )?;
    ensure_token(
        PLAYBOOK_STEP_AUTHORITY_CLASSES,
        &record.authority_class,
        "playbook step authority_class",
    )?;
    Ok(())
}

fn validate_support_export(
    export: &GitReviewSupportExportPacket,
    truth: &GitReviewTimelineTruthRecord,
    events: &[TimelineEventRecord],
    playbooks: &[OperatorPlaybookRecord],
) -> Result<(), GitReviewTimelineValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        GIT_REVIEW_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export record_kind",
    )?;
    ensure_eq(
        export.timeline_id_ref.as_str(),
        truth.timeline_id.as_str(),
        "support_export timeline_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        truth.review_workspace_id_ref.as_str(),
        "support_export review_workspace_id_ref",
    )?;
    if export.raw_url_export_allowed {
        return Err(git_review_timeline_validation_error(
            "support_export raw_url_export_allowed must be false",
        ));
    }
    if export.raw_provider_payload_export_allowed {
        return Err(git_review_timeline_validation_error(
            "support_export raw_provider_payload_export_allowed must be false",
        ));
    }
    if export.timeline_event_id_refs.len() != events.len() {
        return Err(git_review_timeline_validation_error(
            "support_export timeline_event_id_refs length must match events length",
        ));
    }
    if export.playbook_id_refs.len() != playbooks.len() {
        return Err(git_review_timeline_validation_error(
            "support_export playbook_id_refs length must match playbooks length",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &GitReviewTimelineInspectionRecord,
    packet: &GitReviewSupportExportTimelinePacket,
) -> Result<(), GitReviewTimelineValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        GIT_REVIEW_TIMELINE_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.timeline_id_ref.as_str(),
        packet.timeline_truth.timeline_id.as_str(),
        "inspection timeline_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.review_workspace.review_workspace_id.as_str(),
        "inspection review_workspace_id_ref",
    )?;
    if inspection.timeline_event_count != packet.timeline_events.len() {
        return Err(git_review_timeline_validation_error(
            "inspection timeline_event_count must match timeline_events length",
        ));
    }
    if inspection.playbook_count != packet.playbooks.len() {
        return Err(git_review_timeline_validation_error(
            "inspection playbook_count must match playbooks length",
        ));
    }
    if inspection.playbook_step_count != packet.playbook_steps.len() {
        return Err(git_review_timeline_validation_error(
            "inspection playbook_step_count must match playbook_steps length",
        ));
    }
    let expected_no_broadening = !packet
        .playbook_steps
        .iter()
        .any(|s| s.would_broaden_authority);
    if inspection.no_authority_broadening != expected_no_broadening {
        return Err(git_review_timeline_validation_error(
            "inspection no_authority_broadening must match playbook_steps",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn git_review_timeline_validation_error(
    message: impl Into<String>,
) -> GitReviewTimelineValidationError {
    GitReviewTimelineValidationError {
        message: message.into(),
    }
}

fn push_unique(reasons: &mut Vec<String>, reason: &str) {
    if !reasons.iter().any(|r| r == reason) {
        reasons.push(reason.to_string());
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), GitReviewTimelineValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(git_review_timeline_validation_error(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_eq_u32(
    left: u32,
    right: u32,
    field: &str,
) -> Result<(), GitReviewTimelineValidationError> {
    if left != right {
        return Err(git_review_timeline_validation_error(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), GitReviewTimelineValidationError> {
    if value.trim().is_empty() {
        return Err(git_review_timeline_validation_error(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), GitReviewTimelineValidationError> {
    if !tokens.contains(&value) {
        return Err(git_review_timeline_validation_error(format!(
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

    fn workspace_packet() -> ReviewWorkspaceBetaPacket {
        ReviewWorkspaceBetaPacket {
            record_kind: crate::workspace::REVIEW_WORKSPACE_BETA_PACKET_RECORD_KIND.to_string(),
            schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
            packet_id: "wp1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            review_workspace: crate::workspace::ReviewWorkspaceRecord {
                record_kind: crate::workspace::REVIEW_WORKSPACE_RECORD_KIND.to_string(),
                review_workspace_schema_version:
                    crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                review_workspace_id: "ws1".to_string(),
                review_workspace_source_class: "local_git".to_string(),
                provider_authority_class: "local_only".to_string(),
                review_workspace_lifecycle_state: "active".to_string(),
                local_locator: Some(crate::workspace::ReviewLocalLocator {
                    workspace_id_ref: "ws1".to_string(),
                    branch_or_worktree_ref: "main".to_string(),
                    base_revision_ref: None,
                    head_revision_ref: None,
                }),
                provider_overlay: None,
                imported_bundle_envelope: None,
                browser_handoff_envelope: None,
                policy_context: crate::workspace::ReviewPolicyContext {
                    policy_epoch: "1".to_string(),
                    trust_state: "trusted".to_string(),
                    execution_context_id: None,
                    workspace_trust_state_class: "trusted_local".to_string(),
                },
                client_scopes: vec![],
                redaction_class: "metadata_safe_default".to_string(),
                freshness_class: "current".to_string(),
                summary_label: "Workspace".to_string(),
                created_at: "2026-05-27T10:00:00Z".to_string(),
                updated_at: "2026-05-27T10:00:00Z".to_string(),
                archived_at: None,
                hosted_review_inbox_record_id_ref: None,
                merge_policy_record_id_ref: None,
            },
            diff_entries: vec![],
            durable_comment_anchors: vec![],
            object_lineage: vec![],
            check_freshness: vec![],
            browser_handoff: None,
            inspection: crate::workspace::ReviewWorkspaceBetaInspectionRecord {
                record_kind: crate::workspace::REVIEW_WORKSPACE_BETA_INSPECTION_RECORD_KIND
                    .to_string(),
                schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                review_workspace_id_ref: "ws1".to_string(),
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
            support_export: crate::workspace::ReviewWorkspaceSupportExportPacket {
                record_kind: crate::workspace::REVIEW_WORKSPACE_SUPPORT_EXPORT_PACKET_RECORD_KIND
                    .to_string(),
                schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                support_export_id: "wse1".to_string(),
                review_workspace_id_ref: "ws1".to_string(),
                reopen_context_ref: "wrc1".to_string(),
                reopen_command_id_ref: "wcmd1".to_string(),
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
                summary_label: "Export".to_string(),
            },
        }
    }

    fn base_event(id: &str, seq: u64) -> TimelineEventInput {
        TimelineEventInput {
            event_id: id.to_string(),
            sequence_index: seq,
            event_kind: "commit_recorded".to_string(),
            event_source_class: "local_git".to_string(),
            clock_source_class: "local_monotonic".to_string(),
            recorded_at: "2026-05-27T10:00:00Z".to_string(),
            actor_ref: "actor1".to_string(),
            target_object_ref: "obj1".to_string(),
            lineage_parent_ref: None,
            freshness_class: "current".to_string(),
            reversible: true,
            discloses_hosted_authority: false,
            summary_label: "Event".to_string(),
        }
    }

    fn support_export_input() -> GitReviewSupportExportInput {
        GitReviewSupportExportInput {
            support_export_id: "se1".to_string(),
            reopen_context_ref: "rc1".to_string(),
            reopen_command_id_ref: "cmd1".to_string(),
            consumer_surfaces: vec!["support_export".to_string()],
            redaction_class: "metadata_safe_default".to_string(),
            summary_label: "Export".to_string(),
        }
    }

    #[test]
    fn constants_are_nonempty() {
        assert!(!CHRONOLOGY_STATES.is_empty());
        assert!(!TIMELINE_CLOCK_SOURCE_CLASSES.is_empty());
        assert!(!TIMELINE_EVENT_SOURCE_CLASSES.is_empty());
        assert!(!TIMELINE_EVENT_KINDS.is_empty());
        assert!(!TIMELINE_FRESHNESS_CLASSES.is_empty());
        assert!(!OPERATOR_PLAYBOOK_STATES.is_empty());
        assert!(!PLAYBOOK_STEP_COMMAND_CLASSES.is_empty());
        assert!(!PLAYBOOK_STEP_AUTHORITY_CLASSES.is_empty());
        assert!(!GIT_REVIEW_TIMELINE_CONSUMER_SURFACES.is_empty());
        assert!(!GIT_REVIEW_TIMELINE_INVALIDATION_REASONS.is_empty());
    }

    #[test]
    fn happy_path_projects() {
        let input = GitReviewTimelineInput {
            timeline_id: "t1".to_string(),
            packet_id: "p1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            chronology_state: "chronology_current".to_string(),
            timeline_events: vec![base_event("e1", 0), base_event("e2", 1)],
            playbooks: vec![],
            playbook_steps: vec![],
            support_export: support_export_input(),
            invalidation_reasons: vec![],
            summary_label: "Timeline".to_string(),
        };
        let packet =
            GitReviewSupportExportTimelinePacket::from_workspace_packet(input, &workspace_packet())
                .expect("must project");
        assert!(packet.monotonic_ordering_preserved());
        assert!(packet.all_events_attributed());
        assert!(packet.raw_escape_hatches_absent());
        assert!(packet.no_authority_broadening());
    }

    #[test]
    fn non_monotonic_ordering_is_rejected() {
        let input = GitReviewTimelineInput {
            timeline_id: "t1".to_string(),
            packet_id: "p1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            chronology_state: "chronology_current".to_string(),
            timeline_events: vec![base_event("e1", 5), base_event("e2", 2)],
            playbooks: vec![],
            playbook_steps: vec![],
            support_export: support_export_input(),
            invalidation_reasons: vec![],
            summary_label: "Timeline".to_string(),
        };
        let result =
            GitReviewSupportExportTimelinePacket::from_workspace_packet(input, &workspace_packet());
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("monotonic"));
    }

    #[test]
    fn hosted_event_without_disclosure_is_rejected() {
        let mut event = base_event("e1", 0);
        event.event_source_class = "provider_linked".to_string();
        event.discloses_hosted_authority = false;
        let input = GitReviewTimelineInput {
            timeline_id: "t1".to_string(),
            packet_id: "p1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            chronology_state: "chronology_current".to_string(),
            timeline_events: vec![event],
            playbooks: vec![],
            playbook_steps: vec![],
            support_export: support_export_input(),
            invalidation_reasons: vec![],
            summary_label: "Timeline".to_string(),
        };
        let result =
            GitReviewSupportExportTimelinePacket::from_workspace_packet(input, &workspace_packet());
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("hosted authority"));
    }

    #[test]
    fn mutating_step_without_recovery_is_rejected() {
        let input = GitReviewTimelineInput {
            timeline_id: "t1".to_string(),
            packet_id: "p1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            chronology_state: "chronology_current".to_string(),
            timeline_events: vec![base_event("e1", 0)],
            playbooks: vec![OperatorPlaybookInput {
                playbook_id: "pb1".to_string(),
                playbook_state: "ready".to_string(),
                summary_label: "Playbook".to_string(),
            }],
            playbook_steps: vec![OperatorPlaybookStepInput {
                step_id: "st1".to_string(),
                playbook_ref: "pb1".to_string(),
                step_index: 0,
                command_class: "apply_with_checkpoint".to_string(),
                authority_class: "previewable_local_apply".to_string(),
                preview_supported: true,
                reversible: false,
                checkpoint_required: false,
                discloses_hosted_authority: false,
                would_broaden_authority: false,
                target_object_ref: "obj1".to_string(),
                blocked_reasons: vec![],
                summary_label: "Step".to_string(),
            }],
            support_export: support_export_input(),
            invalidation_reasons: vec![],
            summary_label: "Timeline".to_string(),
        };
        let result =
            GitReviewSupportExportTimelinePacket::from_workspace_packet(input, &workspace_packet());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("neither reversible nor checkpointed"));
    }

    #[test]
    fn authority_broadening_step_must_be_blocked() {
        let input = GitReviewTimelineInput {
            timeline_id: "t1".to_string(),
            packet_id: "p1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            chronology_state: "chronology_current".to_string(),
            timeline_events: vec![base_event("e1", 0)],
            playbooks: vec![OperatorPlaybookInput {
                playbook_id: "pb1".to_string(),
                playbook_state: "ready".to_string(),
                summary_label: "Playbook".to_string(),
            }],
            playbook_steps: vec![OperatorPlaybookStepInput {
                step_id: "st1".to_string(),
                playbook_ref: "pb1".to_string(),
                step_index: 0,
                command_class: "escalate".to_string(),
                authority_class: "requires_human_approval".to_string(),
                preview_supported: true,
                reversible: true,
                checkpoint_required: false,
                discloses_hosted_authority: false,
                would_broaden_authority: true,
                target_object_ref: "obj1".to_string(),
                blocked_reasons: vec![],
                summary_label: "Step".to_string(),
            }],
            support_export: support_export_input(),
            invalidation_reasons: vec![],
            summary_label: "Timeline".to_string(),
        };
        // Authority-broadening steps are forced non-actionable, so the packet
        // is valid but the step is blocked and never executable.
        let packet =
            GitReviewSupportExportTimelinePacket::from_workspace_packet(input, &workspace_packet())
                .expect("must project");
        assert!(!packet.no_authority_broadening());
        assert!(!packet.playbook_steps[0].actionable);
        assert!(!packet.timeline_truth.actionable);
    }
}
