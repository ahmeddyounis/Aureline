//! Seeded chronology + inbox conformance packet.
//!
//! The packet is the cross-tool projection consumed by the headless
//! inspector binary, the fixture corpus, and the integration test.
//! It joins one [`super::ActivityTimelineSnapshot`] with structural
//! coverage assertions (lanes present, verbs present, importance
//! mix, attention-triage verbs present, quiet-hours preservation)
//! and a validator so reviewers can compare on-disk fixtures to a
//! live build with one call.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::notifications::envelope::{QuietHoursMode, SourceSubsystem, SuppressionReason};

use super::inbox::{
    AttentionFreshnessClass, AttentionInboxItem, AttentionInboxSuppressionNote,
    AttentionTriageAction, AttentionTriageVerb, InboxAuthoritySourceClass, InboxAvailabilityClass,
    InboxWhyShownReason, ATTENTION_INBOX_ITEM_RECORD_KIND, ATTENTION_INBOX_SCHEMA_VERSION,
};
use super::row::{
    ActionVerb, ActionabilityClass, ActivityEventRow, ActorKind, ChronologyLane, DetailLink,
    DetailLinkKind, ImportanceClass, NarrativeSummaryCard, OutcomeClass, ScopeObjectKind,
    TimelineGroup, TimelineGroupRule, ACTIVITY_EVENT_ROW_RECORD_KIND,
    NARRATIVE_SUMMARY_CARD_RECORD_KIND, TIMELINE_GROUP_RECORD_KIND,
};
use super::{ActivityTimelineRuntime, ActivityTimelineSnapshot};

/// Schema version stamped on every record in the packet.
pub const ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record.
pub const ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF: &str =
    "shell:activity_timeline_and_inbox:v1";

/// Stable record kind for [`ActivityTimelineAndInboxPacket`] payloads.
pub const ACTIVITY_TIMELINE_AND_INBOX_PACKET_RECORD_KIND: &str =
    "shell_activity_timeline_and_inbox_packet_record";

/// Top-level coverage summary carried on the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityTimelineAndInboxSummary {
    /// Distinct chronology lanes present in the row corpus.
    pub lanes_present: Vec<ChronologyLane>,
    /// Distinct action verbs present.
    pub action_verbs_present: Vec<ActionVerb>,
    /// Distinct outcome classes present.
    pub outcome_classes_present: Vec<OutcomeClass>,
    /// Distinct importance classes present.
    pub importance_classes_present: Vec<ImportanceClass>,
    /// Distinct triage verbs exposed across inbox items.
    pub triage_verbs_present: Vec<AttentionTriageVerb>,
    /// Row count.
    pub row_count: usize,
    /// Group count.
    pub group_count: usize,
    /// Summary-card count.
    pub summary_card_count: usize,
    /// Inbox-item count.
    pub inbox_item_count: usize,
    /// True when every row is exact-reopen or a truthful placeholder.
    pub exact_reopen_or_placeholder_only: bool,
    /// True when every consequential or safety-critical row has a
    /// non-truncating durable detail link.
    pub importance_detail_link_rule_satisfied: bool,
    /// True when every inbox item with a held transient surface
    /// preserves durable inbox history.
    pub quiet_hours_durable_history_preserved: bool,
    /// True when every inbox item carries the full
    /// open / snooze / acknowledge / resolve triage verb set.
    pub triage_verb_set_complete: bool,
}

/// Combined chronology + triage packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityTimelineAndInboxPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Packet id used by the inspector binary and the corpus.
    pub packet_id: String,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Coverage summary.
    pub summary: ActivityTimelineAndInboxSummary,
    /// Deterministic snapshot of the chronology and inbox.
    pub snapshot: ActivityTimelineSnapshot,
}

/// Validation error raised by
/// [`validate_activity_timeline_and_inbox_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityTimelineValidationError(pub String);

impl std::fmt::Display for ActivityTimelineValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ActivityTimelineValidationError {}

/// Validate a chronology + inbox packet end-to-end.
///
/// # Errors
///
/// Returns a list of validation errors when the packet does not
/// satisfy the chronology / inbox contract.
pub fn validate_activity_timeline_and_inbox_packet(
    packet: &ActivityTimelineAndInboxPacket,
) -> Result<(), Vec<ActivityTimelineValidationError>> {
    let mut errors = Vec::new();
    if packet.shared_contract_ref != ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF {
        errors.push(ActivityTimelineValidationError(
            "shared_contract_ref must equal the canonical packet contract ref".into(),
        ));
    }
    if packet.schema_version != ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION {
        errors.push(ActivityTimelineValidationError(
            "schema_version must match the canonical packet schema version".into(),
        ));
    }
    if packet.snapshot.rows.is_empty() {
        errors.push(ActivityTimelineValidationError(
            "snapshot.rows must not be empty".into(),
        ));
    }
    if packet.snapshot.inbox.items.is_empty() {
        errors.push(ActivityTimelineValidationError(
            "snapshot.inbox.items must not be empty".into(),
        ));
    }
    for row in &packet.snapshot.rows {
        if row.record_kind != ACTIVITY_EVENT_ROW_RECORD_KIND {
            errors.push(ActivityTimelineValidationError(format!(
                "row {} has invalid record_kind",
                row.event_row_id
            )));
        }
        if !row.actor_identity_rule_satisfied() {
            errors.push(ActivityTimelineValidationError(format!(
                "row {} actor_kind requires a non-empty actor_identity_ref",
                row.event_row_id
            )));
        }
        if !row.detail_link_rule_satisfied() {
            errors.push(ActivityTimelineValidationError(format!(
                "row {} detail-link kind requires a non-empty unavailability_reason_label",
                row.event_row_id
            )));
        }
        if !row.supersedes_rule_satisfied() {
            errors.push(ActivityTimelineValidationError(format!(
                "row {} action_verb=superseded must carry supersedes_event_row_id_ref",
                row.event_row_id
            )));
        }
        if !row.importance_rule_satisfied() {
            errors.push(ActivityTimelineValidationError(format!(
                "row {} consequential/safety-critical importance requires a non-truncating durable detail link",
                row.event_row_id
            )));
        }
    }
    for group in &packet.snapshot.groups {
        if group.record_kind != TIMELINE_GROUP_RECORD_KIND {
            errors.push(ActivityTimelineValidationError(format!(
                "group {} has invalid record_kind",
                group.timeline_group_id
            )));
        }
        if !group.members_resolved_in(&packet.snapshot.rows) {
            errors.push(ActivityTimelineValidationError(format!(
                "group {} declares unknown member event_row_ids",
                group.timeline_group_id
            )));
        }
        if group.allow_routine_row_truncation
            && !matches!(group.importance_class, ImportanceClass::Routine)
        {
            errors.push(ActivityTimelineValidationError(format!(
                "group {} truncation only permitted on routine-importance groups",
                group.timeline_group_id
            )));
        }
        if group.member_event_row_ids.is_empty() {
            errors.push(ActivityTimelineValidationError(format!(
                "group {} must cite at least one member event_row_id",
                group.timeline_group_id
            )));
        }
    }
    for card in &packet.snapshot.summary_cards {
        if card.record_kind != NARRATIVE_SUMMARY_CARD_RECORD_KIND {
            errors.push(ActivityTimelineValidationError(format!(
                "summary card {} has invalid record_kind",
                card.narrative_summary_card_id
            )));
        }
        if !card.cites_any_row() {
            errors.push(ActivityTimelineValidationError(format!(
                "summary card {} must cite at least one event_row_id",
                card.narrative_summary_card_id
            )));
        }
        match card.importance_class {
            ImportanceClass::Consequential | ImportanceClass::SafetyCritical => {
                if !card.detail_link_durable() {
                    errors.push(ActivityTimelineValidationError(format!(
                        "summary card {} importance requires a non-truncating durable detail link",
                        card.narrative_summary_card_id
                    )));
                }
            }
            ImportanceClass::Routine => {}
        }
    }
    for item in &packet.snapshot.inbox.items {
        if item.record_kind != ATTENTION_INBOX_ITEM_RECORD_KIND {
            errors.push(ActivityTimelineValidationError(format!(
                "inbox item {} has invalid record_kind",
                item.inbox_item_id
            )));
        }
        if !item.quiet_hours_preserves_history() {
            errors.push(ActivityTimelineValidationError(format!(
                "inbox item {} suppression note must preserve durable history",
                item.inbox_item_id
            )));
        }
        if !item.exposes_triage_verb_set() {
            errors.push(ActivityTimelineValidationError(format!(
                "inbox item {} must expose open/snooze/acknowledge/resolve verbs",
                item.inbox_item_id
            )));
        }
    }
    if !packet.summary.exact_reopen_or_placeholder_only {
        errors.push(ActivityTimelineValidationError(
            "summary.exact_reopen_or_placeholder_only must be true".into(),
        ));
    }
    if !packet.summary.importance_detail_link_rule_satisfied {
        errors.push(ActivityTimelineValidationError(
            "summary.importance_detail_link_rule_satisfied must be true".into(),
        ));
    }
    if !packet.summary.quiet_hours_durable_history_preserved {
        errors.push(ActivityTimelineValidationError(
            "summary.quiet_hours_durable_history_preserved must be true".into(),
        ));
    }
    if !packet.summary.triage_verb_set_complete {
        errors.push(ActivityTimelineValidationError(
            "summary.triage_verb_set_complete must be true".into(),
        ));
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Build the seeded chronology + inbox packet consumed by the
/// inspector binary, fixtures, and the integration test.
pub fn seeded_activity_timeline_and_inbox_packet() -> ActivityTimelineAndInboxPacket {
    let mut runtime = ActivityTimelineRuntime::new();
    seed_activity_center_lane(&mut runtime);
    seed_approval_lane(&mut runtime);
    seed_provider_sync_lane(&mut runtime);
    seed_policy_change_lane(&mut runtime);
    seed_update_history_lane(&mut runtime);
    seed_reconnect_lane(&mut runtime);
    seed_recovery_lane(&mut runtime);
    seed_attention_inbox(&mut runtime);
    let snapshot = runtime.snapshot();
    let summary = compute_summary(&snapshot);
    ActivityTimelineAndInboxPacket {
        record_kind: ACTIVITY_TIMELINE_AND_INBOX_PACKET_RECORD_KIND.to_owned(),
        schema_version: ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        packet_id: "shell:activity-timeline-and-inbox:packet:001".into(),
        generated_at: "2026-05-18T19:48:13Z".into(),
        summary,
        snapshot,
    }
}

fn compute_summary(snapshot: &ActivityTimelineSnapshot) -> ActivityTimelineAndInboxSummary {
    let mut lanes: BTreeSet<ChronologyLane> = BTreeSet::new();
    let mut verbs: BTreeSet<ActionVerb> = BTreeSet::new();
    let mut outcomes: BTreeSet<OutcomeClass> = BTreeSet::new();
    let mut importances: BTreeSet<ImportanceClass> = BTreeSet::new();
    let mut exact_only = true;
    let mut importance_rule = true;
    for row in &snapshot.rows {
        lanes.insert(row.chronology_lane);
        verbs.insert(row.action_verb);
        outcomes.insert(row.outcome_class);
        importances.insert(row.importance_class);
        if !(row.reopens_exact_target()
            || matches!(
                row.detail_link.kind,
                DetailLinkKind::PlaceholderAnnounced
                    | DetailLinkKind::DeniedRequiresRevalidation
                    | DetailLinkKind::AuditTrailOnly
                    | DetailLinkKind::NotAvailableLinkbackLost
            ))
        {
            exact_only = false;
        }
        if !row.importance_rule_satisfied() {
            importance_rule = false;
        }
    }
    let mut triage_verbs: BTreeSet<AttentionTriageVerb> = BTreeSet::new();
    let mut quiet_hours_ok = true;
    let mut triage_complete = true;
    for item in &snapshot.inbox.items {
        for action in &item.actions {
            triage_verbs.insert(action.verb);
        }
        if !item.quiet_hours_preserves_history() {
            quiet_hours_ok = false;
        }
        if !item.exposes_triage_verb_set() {
            triage_complete = false;
        }
    }
    // Insertion order for closed vocabularies must be stable across builds.
    let mut lane_vec: Vec<ChronologyLane> = lanes.into_iter().collect();
    let mut verb_vec: Vec<ActionVerb> = verbs.into_iter().collect();
    let mut outcome_vec: Vec<OutcomeClass> = outcomes.into_iter().collect();
    let mut importance_vec: Vec<ImportanceClass> = importances.into_iter().collect();
    let mut triage_vec: Vec<AttentionTriageVerb> = triage_verbs.into_iter().collect();
    lane_vec.sort_by_key(|lane| lane.as_str());
    verb_vec.sort_by_key(|verb| verb.as_str());
    outcome_vec.sort_by_key(|outcome| outcome.as_str());
    importance_vec.sort_by_key(|importance| importance.as_str());
    triage_vec.sort_by_key(|verb| verb.as_str());
    ActivityTimelineAndInboxSummary {
        lanes_present: lane_vec,
        action_verbs_present: verb_vec,
        outcome_classes_present: outcome_vec,
        importance_classes_present: importance_vec,
        triage_verbs_present: triage_vec,
        row_count: snapshot.rows.len(),
        group_count: snapshot.groups.len(),
        summary_card_count: snapshot.summary_cards.len(),
        inbox_item_count: snapshot.inbox.items.len(),
        exact_reopen_or_placeholder_only: exact_only,
        importance_detail_link_rule_satisfied: importance_rule,
        quiet_hours_durable_history_preserved: quiet_hours_ok,
        triage_verb_set_complete: triage_complete,
    }
}

fn row_builder() -> ActivityEventRow {
    ActivityEventRow {
        record_kind: ACTIVITY_EVENT_ROW_RECORD_KIND.to_owned(),
        schema_version: ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        event_row_id: String::new(),
        canonical_event_id: String::new(),
        canonical_object_target_ref: String::new(),
        chronology_lane: ChronologyLane::ActivityCenter,
        source_subsystem: SourceSubsystem::Shell,
        actor_kind: ActorKind::SystemActor,
        actor_identity_ref: None,
        actor_or_subsystem_label: "Shell".into(),
        scope_object_kind: ScopeObjectKind::WorkspaceObjectRow,
        action_verb: ActionVerb::Progressed,
        outcome_class: OutcomeClass::InProgress,
        importance_class: ImportanceClass::Routine,
        actionability_class: ActionabilityClass::None,
        summary_label: String::new(),
        scope_label: String::new(),
        monotonic_timestamp: String::new(),
        minted_at: String::new(),
        last_observed_at: String::new(),
        detail_link: DetailLink {
            kind: DetailLinkKind::DurableActivityRow,
            target_identity_ref: None,
            is_durable: true,
            unavailability_reason_label: None,
            announcement_label: None,
        },
        linked_canonical_event_id_ref: None,
        grouped_burst_id_ref: None,
        supersedes_event_row_id_ref: None,
        quiet_hours_held: false,
        occurrence_count: 1,
    }
}

fn exact_detail_link(kind: DetailLinkKind, target: &str, announce: &str) -> DetailLink {
    DetailLink {
        kind,
        target_identity_ref: Some(target.into()),
        is_durable: true,
        unavailability_reason_label: None,
        announcement_label: Some(announce.into()),
    }
}

fn placeholder_detail_link(kind: DetailLinkKind, label: &str, announce: &str) -> DetailLink {
    DetailLink {
        kind,
        target_identity_ref: None,
        is_durable: true,
        unavailability_reason_label: Some(label.into()),
        announcement_label: Some(announce.into()),
    }
}

fn seed_activity_center_lane(runtime: &mut ActivityTimelineRuntime) {
    let target = "ux:object:job:test-runner:ws-1";
    let start = ActivityEventRow {
        event_row_id: "ux:event-row:job:test-runner:ws-1:start".into(),
        canonical_event_id: "ux:event:job:test-runner:ws-1".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::ActivityCenter,
        source_subsystem: SourceSubsystem::TestRunner,
        actor_kind: ActorKind::SystemActor,
        actor_or_subsystem_label: "Test runner".into(),
        scope_object_kind: ScopeObjectKind::DurableJobRow,
        action_verb: ActionVerb::Started,
        outcome_class: OutcomeClass::InProgress,
        importance_class: ImportanceClass::Consequential,
        actionability_class: ActionabilityClass::OpenDetailsOnly,
        summary_label: "Test run started.".into(),
        scope_label: "ws-1 / unit".into(),
        monotonic_timestamp: "2026-05-18T10:00:00Z".into(),
        minted_at: "2026-05-18T10:00:00Z".into(),
        last_observed_at: "2026-05-18T10:00:00Z".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::DurableActivityRow,
            target,
            "Open test run details",
        ),
        ..row_builder()
    };
    let progress = ActivityEventRow {
        event_row_id: "ux:event-row:job:test-runner:ws-1:progress".into(),
        action_verb: ActionVerb::Progressed,
        outcome_class: OutcomeClass::InProgress,
        summary_label: "42 of 240 cases complete.".into(),
        monotonic_timestamp: "2026-05-18T10:01:30Z".into(),
        minted_at: "2026-05-18T10:01:30Z".into(),
        last_observed_at: "2026-05-18T10:01:30Z".into(),
        ..start.clone()
    };
    let fail = ActivityEventRow {
        event_row_id: "ux:event-row:job:test-runner:ws-1:fail".into(),
        action_verb: ActionVerb::Failed,
        outcome_class: OutcomeClass::Failed,
        importance_class: ImportanceClass::Consequential,
        actionability_class: ActionabilityClass::RequiresUserAction,
        summary_label: "Test run failed; 3 cases blocking.".into(),
        monotonic_timestamp: "2026-05-18T10:04:12Z".into(),
        minted_at: "2026-05-18T10:04:12Z".into(),
        last_observed_at: "2026-05-18T10:04:12Z".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::ReviewSheet,
            target,
            "Open failure review sheet",
        ),
        ..progress.clone()
    };
    runtime.record_row(start.clone());
    runtime.record_row(progress.clone());
    runtime.record_row(fail.clone());

    runtime.record_group(TimelineGroup {
        record_kind: TIMELINE_GROUP_RECORD_KIND.to_owned(),
        schema_version: ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        timeline_group_id: "ux:timeline-group:job:test-runner:ws-1".into(),
        group_rule: TimelineGroupRule::SameCanonicalObject,
        chronology_lane: ChronologyLane::ActivityCenter,
        canonical_object_target_ref: Some(target.into()),
        grouped_burst_id_ref: None,
        linked_canonical_event_id_ref: Some("ux:event:job:test-runner:ws-1".into()),
        phase_boundary_labels: vec!["Preparing".into(), "Running".into(), "Failed".into()],
        member_event_row_ids: vec![
            start.event_row_id.clone(),
            progress.event_row_id.clone(),
            fail.event_row_id.clone(),
        ],
        importance_class: ImportanceClass::Consequential,
        allow_routine_row_truncation: false,
        summary_label: "Test run: ws-1 / unit".into(),
        opened_at: start.minted_at.clone(),
        last_updated_at: fail.minted_at.clone(),
        collapsed_by_default: true,
    });

    runtime.record_summary_card(NarrativeSummaryCard {
        record_kind: NARRATIVE_SUMMARY_CARD_RECORD_KIND.to_owned(),
        schema_version: ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        narrative_summary_card_id: "ux:narrative:job:test-runner:ws-1".into(),
        subject_canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::ActivityCenter,
        cited_event_row_ids: vec![start.event_row_id, progress.event_row_id, fail.event_row_id],
        cited_timeline_group_ids: vec!["ux:timeline-group:job:test-runner:ws-1".into()],
        importance_class: ImportanceClass::Consequential,
        summary_title: "Test run for ws-1 failed.".into(),
        summary_body_label:
            "Three cases blocked the run. Review the failure sheet to retry or accept the failure."
                .into(),
        detail_link: exact_detail_link(
            DetailLinkKind::ReviewSheet,
            target,
            "Open failure review sheet",
        ),
        opened_at: "2026-05-18T10:00:00Z".into(),
        last_updated_at: "2026-05-18T10:04:12Z".into(),
    });
}

fn seed_approval_lane(runtime: &mut ActivityTimelineRuntime) {
    let target = "ux:object:approval:ai-apply:diff-77";
    let proposed = ActivityEventRow {
        event_row_id: "ux:event-row:approval:ai-apply:diff-77:proposed".into(),
        canonical_event_id: "ux:event:approval:ai-apply:diff-77".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::Approvals,
        source_subsystem: SourceSubsystem::AiApply,
        actor_kind: ActorKind::AiAgentActor,
        actor_identity_ref: Some("id:actor:ai:apply-agent".into()),
        actor_or_subsystem_label: "AI apply agent".into(),
        scope_object_kind: ScopeObjectKind::ApprovalRequestRow,
        action_verb: ActionVerb::Proposed,
        outcome_class: OutcomeClass::AwaitingApproval,
        importance_class: ImportanceClass::Consequential,
        actionability_class: ActionabilityClass::RequiresUserAction,
        summary_label: "AI apply proposed 4 file edits.".into(),
        scope_label: "ws-1 / changes".into(),
        monotonic_timestamp: "2026-05-18T11:00:00Z".into(),
        minted_at: "2026-05-18T11:00:00Z".into(),
        last_observed_at: "2026-05-18T11:00:00Z".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::ReviewSheet,
            target,
            "Open AI apply review sheet",
        ),
        ..row_builder()
    };
    let accepted = ActivityEventRow {
        event_row_id: "ux:event-row:approval:ai-apply:diff-77:accepted".into(),
        action_verb: ActionVerb::Accepted,
        outcome_class: OutcomeClass::Succeeded,
        actionability_class: ActionabilityClass::OpenDetailsOnly,
        actor_kind: ActorKind::UserActor,
        actor_identity_ref: Some("id:actor:user:current".into()),
        actor_or_subsystem_label: "Reviewer".into(),
        summary_label: "Reviewer accepted the AI apply.".into(),
        monotonic_timestamp: "2026-05-18T11:02:48Z".into(),
        minted_at: "2026-05-18T11:02:48Z".into(),
        last_observed_at: "2026-05-18T11:02:48Z".into(),
        ..proposed.clone()
    };
    runtime.record_row(proposed);
    runtime.record_row(accepted);
}

fn seed_provider_sync_lane(runtime: &mut ActivityTimelineRuntime) {
    let target = "ux:object:provider:sync:identity-store";
    let started = ActivityEventRow {
        event_row_id: "ux:event-row:provider:sync:identity-store:start".into(),
        canonical_event_id: "ux:event:provider:sync:identity-store".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::ProviderSync,
        source_subsystem: SourceSubsystem::ProviderBearing,
        actor_kind: ActorKind::RemoteServiceActor,
        actor_identity_ref: Some("id:actor:provider:identity-store".into()),
        actor_or_subsystem_label: "Identity provider".into(),
        scope_object_kind: ScopeObjectKind::ProviderSyncRow,
        action_verb: ActionVerb::Started,
        outcome_class: OutcomeClass::InProgress,
        importance_class: ImportanceClass::Consequential,
        actionability_class: ActionabilityClass::OpenDetailsOnly,
        summary_label: "Provider sync started.".into(),
        scope_label: "deployment / identity".into(),
        monotonic_timestamp: "2026-05-18T09:30:00Z".into(),
        minted_at: "2026-05-18T09:30:00Z".into(),
        last_observed_at: "2026-05-18T09:30:00Z".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::CanonicalObjectExact,
            target,
            "Open provider sync details",
        ),
        ..row_builder()
    };
    let succeeded = ActivityEventRow {
        event_row_id: "ux:event-row:provider:sync:identity-store:succeeded".into(),
        action_verb: ActionVerb::Succeeded,
        outcome_class: OutcomeClass::Succeeded,
        actionability_class: ActionabilityClass::OpenDetailsOnly,
        summary_label: "Provider sync imported 3 new groups, narrowed 1.".into(),
        monotonic_timestamp: "2026-05-18T09:31:14Z".into(),
        minted_at: "2026-05-18T09:31:14Z".into(),
        last_observed_at: "2026-05-18T09:31:14Z".into(),
        ..started.clone()
    };
    runtime.record_row(started);
    runtime.record_row(succeeded);
}

fn seed_policy_change_lane(runtime: &mut ActivityTimelineRuntime) {
    let target = "ux:object:policy:decision:apply-allowlist";
    let narrowed = ActivityEventRow {
        event_row_id: "ux:event-row:policy:apply-allowlist:narrowed".into(),
        canonical_event_id: "ux:event:policy:apply-allowlist".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::PolicyChanges,
        source_subsystem: SourceSubsystem::AdminPolicy,
        actor_kind: ActorKind::AdminPolicyActor,
        actor_identity_ref: Some("id:actor:admin:policy-owner".into()),
        actor_or_subsystem_label: "Policy admin".into(),
        scope_object_kind: ScopeObjectKind::PolicyDecisionRow,
        action_verb: ActionVerb::Narrowed,
        outcome_class: OutcomeClass::ObservedOnly,
        importance_class: ImportanceClass::SafetyCritical,
        actionability_class: ActionabilityClass::OpenDetailsOnly,
        summary_label: "Apply allowlist narrowed to repo-scoped paths.".into(),
        scope_label: "deployment / policy".into(),
        monotonic_timestamp: "2026-05-18T08:45:00Z".into(),
        minted_at: "2026-05-18T08:45:00Z".into(),
        last_observed_at: "2026-05-18T08:45:00Z".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::CanonicalObjectExact,
            target,
            "Open policy change",
        ),
        ..row_builder()
    };
    runtime.record_row(narrowed);
}

fn seed_update_history_lane(runtime: &mut ActivityTimelineRuntime) {
    let target = "ux:object:update:release-channel:beta";
    let proposed = ActivityEventRow {
        event_row_id: "ux:event-row:update:release-channel:beta:proposed".into(),
        canonical_event_id: "ux:event:update:release-channel:beta".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::UpdateHistory,
        source_subsystem: SourceSubsystem::InstallUpdateAttach,
        actor_kind: ActorKind::SystemActor,
        actor_or_subsystem_label: "Update service".into(),
        scope_object_kind: ScopeObjectKind::UpdateEventRow,
        action_verb: ActionVerb::Proposed,
        outcome_class: OutcomeClass::Pending,
        importance_class: ImportanceClass::Consequential,
        actionability_class: ActionabilityClass::RequiresUserAction,
        summary_label: "New beta build available.".into(),
        scope_label: "channel / beta".into(),
        monotonic_timestamp: "2026-05-17T22:00:00Z".into(),
        minted_at: "2026-05-17T22:00:00Z".into(),
        last_observed_at: "2026-05-17T22:00:00Z".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::CanonicalObjectExact,
            target,
            "Open update details",
        ),
        ..row_builder()
    };
    let snoozed = ActivityEventRow {
        event_row_id: "ux:event-row:update:release-channel:beta:snoozed".into(),
        actor_kind: ActorKind::UserActor,
        actor_identity_ref: Some("id:actor:user:current".into()),
        actor_or_subsystem_label: "User".into(),
        action_verb: ActionVerb::Snoozed,
        outcome_class: OutcomeClass::Held,
        actionability_class: ActionabilityClass::Reviewable,
        summary_label: "User snoozed the update until tomorrow.".into(),
        monotonic_timestamp: "2026-05-17T22:05:00Z".into(),
        minted_at: "2026-05-17T22:05:00Z".into(),
        last_observed_at: "2026-05-17T22:05:00Z".into(),
        ..proposed.clone()
    };
    runtime.record_row(proposed);
    runtime.record_row(snoozed);
}

fn seed_reconnect_lane(runtime: &mut ActivityTimelineRuntime) {
    let target = "ux:object:reconnect:remote-agent:east-1";
    let disconnected = ActivityEventRow {
        event_row_id: "ux:event-row:reconnect:remote-agent:east-1:disconnected".into(),
        canonical_event_id: "ux:event:reconnect:remote-agent:east-1".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::ReconnectFlow,
        source_subsystem: SourceSubsystem::RemoteAgent,
        actor_kind: ActorKind::SystemActor,
        actor_or_subsystem_label: "Remote agent transport".into(),
        scope_object_kind: ScopeObjectKind::ReconnectEventRow,
        action_verb: ActionVerb::Disconnected,
        outcome_class: OutcomeClass::Failed,
        importance_class: ImportanceClass::Consequential,
        actionability_class: ActionabilityClass::OpenDetailsOnly,
        summary_label: "Remote agent disconnected.".into(),
        scope_label: "east-1 / agent".into(),
        monotonic_timestamp: "2026-05-18T07:15:00Z".into(),
        minted_at: "2026-05-18T07:15:00Z".into(),
        last_observed_at: "2026-05-18T07:15:00Z".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::DurableActivityRow,
            target,
            "Open reconnect details",
        ),
        quiet_hours_held: true,
        ..row_builder()
    };
    let reconnected = ActivityEventRow {
        event_row_id: "ux:event-row:reconnect:remote-agent:east-1:reconnected".into(),
        action_verb: ActionVerb::Reconnected,
        outcome_class: OutcomeClass::Recovered,
        actionability_class: ActionabilityClass::OpenDetailsOnly,
        summary_label: "Remote agent reconnected with no state loss.".into(),
        monotonic_timestamp: "2026-05-18T07:15:42Z".into(),
        minted_at: "2026-05-18T07:15:42Z".into(),
        last_observed_at: "2026-05-18T07:15:42Z".into(),
        quiet_hours_held: false,
        ..disconnected.clone()
    };
    runtime.record_row(disconnected);
    runtime.record_row(reconnected);
}

fn seed_recovery_lane(runtime: &mut ActivityTimelineRuntime) {
    let target = "ux:object:recovery:session-restore:proposal-19";
    let proposed = ActivityEventRow {
        event_row_id: "ux:event-row:recovery:session-restore:proposal-19:proposed".into(),
        canonical_event_id: "ux:event:recovery:session-restore:proposal-19".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::Recovery,
        source_subsystem: SourceSubsystem::Shell,
        actor_kind: ActorKind::SystemActor,
        actor_or_subsystem_label: "Recovery".into(),
        scope_object_kind: ScopeObjectKind::RecoverySnapshotRow,
        action_verb: ActionVerb::Proposed,
        outcome_class: OutcomeClass::AwaitingApproval,
        importance_class: ImportanceClass::Consequential,
        actionability_class: ActionabilityClass::RequiresUserAction,
        summary_label: "Recovery proposes restoring 1 window, 1 tab group.".into(),
        scope_label: "previous session".into(),
        monotonic_timestamp: "2026-05-18T05:00:00Z".into(),
        minted_at: "2026-05-18T05:00:00Z".into(),
        last_observed_at: "2026-05-18T05:00:00Z".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::CanonicalObjectExact,
            target,
            "Open recovery proposal",
        ),
        ..row_builder()
    };
    let restored = ActivityEventRow {
        event_row_id: "ux:event-row:recovery:session-restore:proposal-19:restored".into(),
        actor_kind: ActorKind::UserActor,
        actor_identity_ref: Some("id:actor:user:current".into()),
        actor_or_subsystem_label: "User".into(),
        action_verb: ActionVerb::Restored,
        outcome_class: OutcomeClass::Recovered,
        actionability_class: ActionabilityClass::OpenDetailsOnly,
        summary_label: "Recovery applied: 1 window, 1 tab group restored.".into(),
        monotonic_timestamp: "2026-05-18T05:00:11Z".into(),
        minted_at: "2026-05-18T05:00:11Z".into(),
        last_observed_at: "2026-05-18T05:00:11Z".into(),
        ..proposed.clone()
    };
    // Placeholder row: a recovery item whose underlying snapshot has expired.
    let placeholder = ActivityEventRow {
        event_row_id: "ux:event-row:recovery:session-restore:proposal-19:placeholder".into(),
        canonical_event_id: "ux:event:recovery:session-restore:proposal-19:placeholder".into(),
        canonical_object_target_ref: "ux:object:recovery:session-restore:proposal-19:placeholder"
            .into(),
        action_verb: ActionVerb::Held,
        outcome_class: OutcomeClass::Held,
        importance_class: ImportanceClass::Routine,
        actionability_class: ActionabilityClass::RequiresRevalidation,
        summary_label: "Recovery snapshot expired; revalidate before restoring.".into(),
        monotonic_timestamp: "2026-05-18T05:00:30Z".into(),
        minted_at: "2026-05-18T05:00:30Z".into(),
        last_observed_at: "2026-05-18T05:00:30Z".into(),
        detail_link: placeholder_detail_link(
            DetailLinkKind::PlaceholderAnnounced,
            "Recovery snapshot expired",
            "Open placeholder",
        ),
        ..proposed.clone()
    };
    runtime.record_row(proposed);
    runtime.record_row(restored);
    runtime.record_row(placeholder);
}

fn seed_attention_inbox(runtime: &mut ActivityTimelineRuntime) {
    runtime.record_inbox_item(approval_inbox_item());
    runtime.record_inbox_item(update_inbox_item());
    runtime.record_inbox_item(recovery_inbox_item());
    runtime.record_inbox_item(provider_sync_inbox_item());
    runtime.record_inbox_item(reconnect_inbox_item_with_quiet_hours());
    runtime.record_inbox_item(resolved_history_inbox_item());
    runtime.record_inbox_item(muted_class_inbox_item());
}

fn full_triage_action_set(target: &str) -> Vec<AttentionTriageAction> {
    vec![
        AttentionTriageAction {
            verb: AttentionTriageVerb::Open,
            command_id: Some("cmd:attention.open".into()),
            availability: InboxAvailabilityClass::Enabled,
            disabled_reason_label: None,
            target_identity_ref: target.into(),
        },
        AttentionTriageAction {
            verb: AttentionTriageVerb::Snooze,
            command_id: Some("cmd:attention.snooze".into()),
            availability: InboxAvailabilityClass::Enabled,
            disabled_reason_label: None,
            target_identity_ref: target.into(),
        },
        AttentionTriageAction {
            verb: AttentionTriageVerb::Acknowledge,
            command_id: Some("cmd:attention.acknowledge".into()),
            availability: InboxAvailabilityClass::Enabled,
            disabled_reason_label: None,
            target_identity_ref: target.into(),
        },
        AttentionTriageAction {
            verb: AttentionTriageVerb::Clear,
            command_id: Some("cmd:attention.clear".into()),
            availability: InboxAvailabilityClass::Enabled,
            disabled_reason_label: None,
            target_identity_ref: target.into(),
        },
        AttentionTriageAction {
            verb: AttentionTriageVerb::Mute,
            command_id: Some("cmd:attention.mute".into()),
            availability: InboxAvailabilityClass::Enabled,
            disabled_reason_label: None,
            target_identity_ref: target.into(),
        },
        AttentionTriageAction {
            verb: AttentionTriageVerb::Resolve,
            command_id: Some("cmd:attention.resolve".into()),
            availability: InboxAvailabilityClass::Enabled,
            disabled_reason_label: None,
            target_identity_ref: target.into(),
        },
    ]
}

fn approval_inbox_item() -> AttentionInboxItem {
    let target = "ux:object:approval:ai-apply:diff-77";
    AttentionInboxItem {
        record_kind: ATTENTION_INBOX_ITEM_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        inbox_item_id: "ux:inbox-item:approval:ai-apply:diff-77".into(),
        canonical_event_id: "ux:event:approval:ai-apply:diff-77".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::Approvals,
        source_subsystem: SourceSubsystem::AiApply,
        scope_object_kind: ScopeObjectKind::ApprovalRequestRow,
        importance_class: ImportanceClass::Consequential,
        why_shown_reason: InboxWhyShownReason::AssignedReviewer,
        authority_source_class: InboxAuthoritySourceClass::AiAgent,
        freshness_class: AttentionFreshnessClass::Fresh,
        title_label: "AI apply needs review.".into(),
        body_label: "4 file edits across 2 packages.".into(),
        scope_label: "ws-1 / changes".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::ReviewSheet,
            target,
            "Open AI apply review sheet",
        ),
        event_row_id_ref: Some("ux:event-row:approval:ai-apply:diff-77:proposed".into()),
        minted_at: "2026-05-18T11:00:00Z".into(),
        last_observed_at: "2026-05-18T11:00:00Z".into(),
        snoozed_until: None,
        acknowledged: false,
        resolved: false,
        muted: false,
        actions: full_triage_action_set(target),
        suppression_note: AttentionInboxSuppressionNote::never_suppressed(),
    }
}

fn update_inbox_item() -> AttentionInboxItem {
    let target = "ux:object:update:release-channel:beta";
    AttentionInboxItem {
        record_kind: ATTENTION_INBOX_ITEM_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        inbox_item_id: "ux:inbox-item:update:release-channel:beta".into(),
        canonical_event_id: "ux:event:update:release-channel:beta".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::UpdateHistory,
        source_subsystem: SourceSubsystem::InstallUpdateAttach,
        scope_object_kind: ScopeObjectKind::UpdateEventRow,
        importance_class: ImportanceClass::Consequential,
        why_shown_reason: InboxWhyShownReason::UpdateAddressed,
        authority_source_class: InboxAuthoritySourceClass::FirstParty,
        freshness_class: AttentionFreshnessClass::Recent,
        title_label: "Beta build ready to install.".into(),
        body_label: "Snoozed until tomorrow morning.".into(),
        scope_label: "channel / beta".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::CanonicalObjectExact,
            target,
            "Open update details",
        ),
        event_row_id_ref: Some("ux:event-row:update:release-channel:beta:snoozed".into()),
        minted_at: "2026-05-17T22:05:00Z".into(),
        last_observed_at: "2026-05-17T22:05:00Z".into(),
        snoozed_until: Some("2026-05-18T15:00:00Z".into()),
        acknowledged: false,
        resolved: false,
        muted: false,
        actions: full_triage_action_set(target),
        suppression_note: AttentionInboxSuppressionNote::never_suppressed(),
    }
}

fn recovery_inbox_item() -> AttentionInboxItem {
    let target = "ux:object:recovery:session-restore:proposal-19";
    AttentionInboxItem {
        record_kind: ATTENTION_INBOX_ITEM_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        inbox_item_id: "ux:inbox-item:recovery:session-restore:proposal-19".into(),
        canonical_event_id: "ux:event:recovery:session-restore:proposal-19".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::Recovery,
        source_subsystem: SourceSubsystem::Shell,
        scope_object_kind: ScopeObjectKind::RecoverySnapshotRow,
        importance_class: ImportanceClass::Consequential,
        why_shown_reason: InboxWhyShownReason::RecoveryAddressed,
        authority_source_class: InboxAuthoritySourceClass::RecoverySubsystem,
        freshness_class: AttentionFreshnessClass::Fresh,
        title_label: "Recovery is ready to restore your session.".into(),
        body_label: "1 window, 1 tab group available.".into(),
        scope_label: "previous session".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::CanonicalObjectExact,
            target,
            "Open recovery proposal",
        ),
        event_row_id_ref: Some("ux:event-row:recovery:session-restore:proposal-19:proposed".into()),
        minted_at: "2026-05-18T05:00:00Z".into(),
        last_observed_at: "2026-05-18T05:00:00Z".into(),
        snoozed_until: None,
        acknowledged: false,
        resolved: false,
        muted: false,
        actions: full_triage_action_set(target),
        suppression_note: AttentionInboxSuppressionNote::never_suppressed(),
    }
}

fn provider_sync_inbox_item() -> AttentionInboxItem {
    let target = "ux:object:provider:sync:identity-store";
    AttentionInboxItem {
        record_kind: ATTENTION_INBOX_ITEM_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        inbox_item_id: "ux:inbox-item:provider:sync:identity-store".into(),
        canonical_event_id: "ux:event:provider:sync:identity-store".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::ProviderSync,
        source_subsystem: SourceSubsystem::ProviderBearing,
        scope_object_kind: ScopeObjectKind::ProviderSyncRow,
        importance_class: ImportanceClass::Consequential,
        why_shown_reason: InboxWhyShownReason::ProviderSyncAddressed,
        authority_source_class: InboxAuthoritySourceClass::RemoteService,
        freshness_class: AttentionFreshnessClass::StaleRevalidationRecommended,
        title_label: "Provider sync changed 4 groups.".into(),
        body_label: "Review before re-running policy-bound work.".into(),
        scope_label: "deployment / identity".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::CanonicalObjectExact,
            target,
            "Open provider sync details",
        ),
        event_row_id_ref: Some("ux:event-row:provider:sync:identity-store:succeeded".into()),
        minted_at: "2026-05-18T09:31:14Z".into(),
        last_observed_at: "2026-05-18T09:31:14Z".into(),
        snoozed_until: None,
        acknowledged: false,
        resolved: false,
        muted: false,
        actions: full_triage_action_set(target),
        suppression_note: AttentionInboxSuppressionNote::never_suppressed(),
    }
}

fn reconnect_inbox_item_with_quiet_hours() -> AttentionInboxItem {
    let target = "ux:object:reconnect:remote-agent:east-1";
    AttentionInboxItem {
        record_kind: ATTENTION_INBOX_ITEM_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        inbox_item_id: "ux:inbox-item:reconnect:remote-agent:east-1".into(),
        canonical_event_id: "ux:event:reconnect:remote-agent:east-1".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::ReconnectFlow,
        source_subsystem: SourceSubsystem::RemoteAgent,
        scope_object_kind: ScopeObjectKind::ReconnectEventRow,
        importance_class: ImportanceClass::Consequential,
        why_shown_reason: InboxWhyShownReason::ReconnectAddressed,
        authority_source_class: InboxAuthoritySourceClass::FirstParty,
        freshness_class: AttentionFreshnessClass::Recent,
        title_label: "Remote agent reconnected after a 42 second gap.".into(),
        body_label: "Quiet hours held the toast; durable history is preserved.".into(),
        scope_label: "east-1 / agent".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::DurableActivityRow,
            target,
            "Open reconnect details",
        ),
        event_row_id_ref: Some("ux:event-row:reconnect:remote-agent:east-1:reconnected".into()),
        minted_at: "2026-05-18T07:15:42Z".into(),
        last_observed_at: "2026-05-18T07:15:42Z".into(),
        snoozed_until: None,
        acknowledged: false,
        resolved: false,
        muted: false,
        actions: full_triage_action_set(target),
        suppression_note: AttentionInboxSuppressionNote {
            active_modes: vec![QuietHoursMode::ModeQuietHoursUser],
            suppression_reasons: vec![SuppressionReason::QuietHoursUserPolicy],
            transient_surface_held: true,
            durable_history_preserved: true,
            release_rule_label:
                "Release on next user attention; durable inbox row remains visible during hold."
                    .into(),
        },
    }
}

fn resolved_history_inbox_item() -> AttentionInboxItem {
    let target = "ux:object:approval:ai-apply:diff-66";
    AttentionInboxItem {
        record_kind: ATTENTION_INBOX_ITEM_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        inbox_item_id: "ux:inbox-item:approval:ai-apply:diff-66".into(),
        canonical_event_id: "ux:event:approval:ai-apply:diff-66".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::Approvals,
        source_subsystem: SourceSubsystem::AiApply,
        scope_object_kind: ScopeObjectKind::ApprovalRequestRow,
        importance_class: ImportanceClass::Consequential,
        why_shown_reason: InboxWhyShownReason::AssignedReviewer,
        authority_source_class: InboxAuthoritySourceClass::AiAgent,
        freshness_class: AttentionFreshnessClass::Recent,
        title_label: "AI apply resolved.".into(),
        body_label: "Reviewer accepted; row remains in history.".into(),
        scope_label: "ws-1 / changes".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::ReviewSheet,
            target,
            "Open resolved AI apply",
        ),
        event_row_id_ref: None,
        minted_at: "2026-05-17T18:00:00Z".into(),
        last_observed_at: "2026-05-17T18:02:30Z".into(),
        snoozed_until: None,
        acknowledged: true,
        resolved: true,
        muted: false,
        actions: full_triage_action_set(target),
        suppression_note: AttentionInboxSuppressionNote::never_suppressed(),
    }
}

fn muted_class_inbox_item() -> AttentionInboxItem {
    let target = "ux:object:policy:decision:apply-allowlist";
    AttentionInboxItem {
        record_kind: ATTENTION_INBOX_ITEM_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_INBOX_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
        inbox_item_id: "ux:inbox-item:policy:apply-allowlist".into(),
        canonical_event_id: "ux:event:policy:apply-allowlist".into(),
        canonical_object_target_ref: target.into(),
        chronology_lane: ChronologyLane::PolicyChanges,
        source_subsystem: SourceSubsystem::AdminPolicy,
        scope_object_kind: ScopeObjectKind::PolicyDecisionRow,
        importance_class: ImportanceClass::SafetyCritical,
        why_shown_reason: InboxWhyShownReason::PolicyAuthority,
        authority_source_class: InboxAuthoritySourceClass::AdminPolicy,
        freshness_class: AttentionFreshnessClass::Recent,
        title_label: "Policy class muted by user.".into(),
        body_label: "Durable evidence preserved; mute is reversible.".into(),
        scope_label: "deployment / policy".into(),
        detail_link: exact_detail_link(
            DetailLinkKind::CanonicalObjectExact,
            target,
            "Open policy change",
        ),
        event_row_id_ref: Some("ux:event-row:policy:apply-allowlist:narrowed".into()),
        minted_at: "2026-05-18T08:45:00Z".into(),
        last_observed_at: "2026-05-18T08:46:00Z".into(),
        snoozed_until: None,
        acknowledged: false,
        resolved: false,
        muted: true,
        actions: full_triage_action_set(target),
        suppression_note: AttentionInboxSuppressionNote::never_suppressed(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates_against_contract() {
        let packet = seeded_activity_timeline_and_inbox_packet();
        validate_activity_timeline_and_inbox_packet(&packet).expect("seeded packet must validate");
        assert!(packet.summary.row_count >= 13);
        assert!(packet.summary.inbox_item_count >= 7);
        assert!(packet
            .summary
            .lanes_present
            .contains(&ChronologyLane::ActivityCenter));
        assert!(packet
            .summary
            .lanes_present
            .contains(&ChronologyLane::Recovery));
        assert!(packet
            .summary
            .lanes_present
            .contains(&ChronologyLane::ReconnectFlow));
    }

    #[test]
    fn triage_verb_set_is_complete_for_every_inbox_item() {
        let packet = seeded_activity_timeline_and_inbox_packet();
        for item in &packet.snapshot.inbox.items {
            assert!(
                item.exposes_triage_verb_set(),
                "inbox item {} missing triage verb set",
                item.inbox_item_id
            );
        }
    }

    #[test]
    fn quiet_hours_held_row_preserves_durable_inbox_history() {
        let packet = seeded_activity_timeline_and_inbox_packet();
        let item = packet
            .snapshot
            .inbox
            .items
            .iter()
            .find(|item| item.inbox_item_id == "ux:inbox-item:reconnect:remote-agent:east-1")
            .expect("quiet-hours inbox item present");
        assert!(item.suppression_note.transient_surface_held);
        assert!(item.suppression_note.durable_history_preserved);
        assert!(packet.summary.quiet_hours_durable_history_preserved);
    }

    #[test]
    fn snapshot_is_deterministic_across_two_builds() {
        let a = seeded_activity_timeline_and_inbox_packet();
        let b = seeded_activity_timeline_and_inbox_packet();
        assert_eq!(a, b);
    }
}
