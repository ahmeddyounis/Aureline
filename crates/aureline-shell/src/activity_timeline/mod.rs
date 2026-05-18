//! Shared event-row / chronology / attention-inbox primitive.
//!
//! This module is the live shell-owned home for one reusable
//! event/history row primitive that the activity center, AI evidence
//! lane, policy-change lane, provider-sync lane, update history,
//! reconnect flows, and recovery flows all project through. It also
//! pins the triage primitive — the attention inbox — for items that
//! require human action rather than background completion.
//!
//! ## Why one primitive
//!
//! Before this primitive landed, each lane minted its own row shape
//! and its own dismissal vocabulary. Toasts vanished, banners cleared,
//! and the only surviving truth lived in the durable activity-center
//! row. That made it impossible to answer "what happened while I was
//! focused elsewhere" without re-running the work.
//!
//! The [`ActivityEventRow`] is the answer: one stable row with a
//! timestamp, an actor or subsystem, an action verb, a scope/object
//! reference, an outcome, current actionability, and a non-truncating
//! detail link. Every lane can build a row from its own typed payload
//! and the chronology surface, attention inbox, narrative summary
//! cards, and support exports read the same shape.
//!
//! ## What this module owns
//!
//! - The shared [`ActivityEventRow`] event/history primitive, plus the
//!   closed vocabularies it carries ([`ActorKind`], [`ActionVerb`],
//!   [`OutcomeClass`], [`ImportanceClass`], [`ActionabilityClass`],
//!   [`ScopeObjectKind`], [`DetailLinkKind`]).
//! - [`TimelineGroup`] grouped views and the
//!   [`NarrativeSummaryCard`] history-heavy summary card, both of
//!   which cite member rows by id rather than replacing them.
//! - [`AttentionInboxItem`] for items needing user action, with
//!   why-shown reasons, freshness, authority source,
//!   snooze/acknowledge/open/resolve actions, and quiet-hours
//!   suppression notes.
//! - [`AttentionTriageVerb`], the closed verb set distinguishing
//!   snooze / acknowledge / clear / mute / resolve so each is
//!   independently attributable in exported history.
//! - [`ActivityTimelineRuntime`], the durable home that keeps event
//!   rows, groups, summary cards, and inbox items together and
//!   produces deterministic snapshots and support exports.
//! - The [`seeded_activity_timeline_and_inbox_packet`] builder used by
//!   the shell, tests, fixtures, and the headless inspector binary.
//!
//! ## What this module does NOT own
//!
//! - The durable activity-center row (the lifecycle truth for
//!   long-running jobs is still owned by
//!   [`crate::activity_center::alpha`]).
//! - The notification envelope vocabulary (severity, privacy,
//!   redaction, dedupe — those live with
//!   [`crate::notifications::envelope`]).
//! - Toast or banner copy. Every projection on this module is
//!   structural; copy is rendered by the chrome from the structured
//!   labels carried on each row.

pub mod inbox;
pub mod packet;
pub mod row;

pub use inbox::{
    AttentionFreshnessClass, AttentionInboxItem, AttentionInboxSnapshot,
    AttentionInboxSuppressionNote, AttentionTriageAction, AttentionTriageVerb,
    InboxAuthoritySourceClass, InboxAvailabilityClass, InboxWhyShownReason,
    ATTENTION_INBOX_ITEM_RECORD_KIND, ATTENTION_INBOX_SCHEMA_VERSION,
    ATTENTION_INBOX_SNAPSHOT_RECORD_KIND,
};
pub use packet::{
    seeded_activity_timeline_and_inbox_packet, validate_activity_timeline_and_inbox_packet,
    ActivityTimelineAndInboxPacket, ActivityTimelineAndInboxSummary,
    ACTIVITY_TIMELINE_AND_INBOX_PACKET_RECORD_KIND, ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION,
    ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF,
};
pub use row::{
    ActionVerb, ActionabilityClass, ActivityEventRow, ActorKind, ChronologyLane, DetailLink,
    DetailLinkKind, ImportanceClass, NarrativeSummaryCard, OutcomeClass, ScopeObjectKind,
    TimelineGroup, TimelineGroupRule, ACTIVITY_EVENT_ROW_RECORD_KIND,
    ACTIVITY_EVENT_ROW_SCHEMA_VERSION, NARRATIVE_SUMMARY_CARD_RECORD_KIND,
    TIMELINE_GROUP_RECORD_KIND,
};

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Stable record kind for a chronology snapshot.
pub const ACTIVITY_TIMELINE_SNAPSHOT_RECORD_KIND: &str = "activity_timeline_snapshot_record";

/// Deterministic projection emitted by [`ActivityTimelineRuntime::snapshot`].
///
/// The snapshot orders event rows by `(monotonic_timestamp,
/// event_row_id)` and groups by `(timeline_group_id)` so two
/// processes that observed the same chronology produce byte-identical
/// outputs. The inbox is emitted alongside so the chrome can render
/// the chronology lane and the triage inbox from one read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityTimelineSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version mirrored from
    /// [`ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Shared contract ref reviewers can compare across exports.
    pub shared_contract_ref: String,
    /// Event rows in monotonic order.
    pub rows: Vec<ActivityEventRow>,
    /// Timeline groups keyed by group id.
    pub groups: Vec<TimelineGroup>,
    /// Narrative summary cards. Each cites member rows by id.
    pub summary_cards: Vec<NarrativeSummaryCard>,
    /// Inbox triage items minted from the same row corpus.
    pub inbox: AttentionInboxSnapshot,
}

/// Durable runtime that aggregates rows, groups, summary cards, and
/// inbox items.
#[derive(Debug, Clone, Default)]
pub struct ActivityTimelineRuntime {
    rows: BTreeMap<String, ActivityEventRow>,
    groups: BTreeMap<String, TimelineGroup>,
    summary_cards: BTreeMap<String, NarrativeSummaryCard>,
    inbox: BTreeMap<String, AttentionInboxItem>,
}

impl ActivityTimelineRuntime {
    /// Builds an empty runtime.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records or updates one chronology row. The minted timestamp is
    /// preserved if the row id is already present so later
    /// observations cannot lie about when the work first appeared.
    pub fn record_row(&mut self, mut row: ActivityEventRow) {
        if let Some(existing) = self.rows.get(&row.event_row_id) {
            row.minted_at = existing.minted_at.clone();
            row.occurrence_count = existing.occurrence_count.saturating_add(1);
        } else {
            row.occurrence_count = row.occurrence_count.max(1);
        }
        self.rows.insert(row.event_row_id.clone(), row);
    }

    /// Records or replaces a timeline group keyed by `timeline_group_id`.
    pub fn record_group(&mut self, group: TimelineGroup) {
        self.groups.insert(group.timeline_group_id.clone(), group);
    }

    /// Records or replaces a narrative summary card keyed by id.
    pub fn record_summary_card(&mut self, card: NarrativeSummaryCard) {
        self.summary_cards
            .insert(card.narrative_summary_card_id.clone(), card);
    }

    /// Records or updates one attention inbox item.
    pub fn record_inbox_item(&mut self, item: AttentionInboxItem) {
        self.inbox.insert(item.inbox_item_id.clone(), item);
    }

    /// Finds an event row by id.
    pub fn find_row(&self, event_row_id: &str) -> Option<&ActivityEventRow> {
        self.rows.get(event_row_id)
    }

    /// Finds a group by id.
    pub fn find_group(&self, timeline_group_id: &str) -> Option<&TimelineGroup> {
        self.groups.get(timeline_group_id)
    }

    /// Finds an inbox item by id.
    pub fn find_inbox_item(&self, inbox_item_id: &str) -> Option<&AttentionInboxItem> {
        self.inbox.get(inbox_item_id)
    }

    /// Returns a deterministic chronology + inbox snapshot.
    pub fn snapshot(&self) -> ActivityTimelineSnapshot {
        let mut rows: Vec<ActivityEventRow> = self.rows.values().cloned().collect();
        rows.sort_by(|a, b| {
            a.monotonic_timestamp
                .cmp(&b.monotonic_timestamp)
                .then_with(|| a.event_row_id.cmp(&b.event_row_id))
        });
        let mut groups: Vec<TimelineGroup> = self.groups.values().cloned().collect();
        groups.sort_by(|a, b| {
            a.opened_at
                .cmp(&b.opened_at)
                .then_with(|| a.timeline_group_id.cmp(&b.timeline_group_id))
        });
        let mut summary_cards: Vec<NarrativeSummaryCard> =
            self.summary_cards.values().cloned().collect();
        summary_cards.sort_by(|a, b| {
            a.opened_at.cmp(&b.opened_at).then_with(|| {
                a.narrative_summary_card_id
                    .cmp(&b.narrative_summary_card_id)
            })
        });
        let mut inbox_items: Vec<AttentionInboxItem> = self.inbox.values().cloned().collect();
        inbox_items.sort_by(|a, b| {
            a.minted_at
                .cmp(&b.minted_at)
                .then_with(|| a.inbox_item_id.cmp(&b.inbox_item_id))
        });
        ActivityTimelineSnapshot {
            record_kind: ACTIVITY_TIMELINE_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION,
            shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
            rows,
            groups,
            summary_cards,
            inbox: AttentionInboxSnapshot::from_items(inbox_items),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::envelope::SourceSubsystem;

    fn sample_row(id: &str, ts: &str) -> ActivityEventRow {
        ActivityEventRow {
            record_kind: ACTIVITY_EVENT_ROW_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION,
            shared_contract_ref: ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF.to_owned(),
            event_row_id: id.to_owned(),
            canonical_event_id: format!("ux:event:{id}"),
            canonical_object_target_ref: format!("ux:object:{id}"),
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
            summary_label: "Routine update.".into(),
            scope_label: "Active workspace".into(),
            monotonic_timestamp: ts.to_owned(),
            minted_at: ts.to_owned(),
            last_observed_at: ts.to_owned(),
            detail_link: DetailLink {
                kind: DetailLinkKind::DurableActivityRow,
                target_identity_ref: Some(format!("ux:object:{id}")),
                is_durable: true,
                unavailability_reason_label: None,
                announcement_label: Some("Open details".into()),
            },
            linked_canonical_event_id_ref: None,
            grouped_burst_id_ref: None,
            supersedes_event_row_id_ref: None,
            quiet_hours_held: false,
            occurrence_count: 1,
        }
    }

    #[test]
    fn snapshot_sorts_rows_by_monotonic_timestamp_then_id() {
        let mut rt = ActivityTimelineRuntime::new();
        rt.record_row(sample_row("evt:b", "2026-05-10T10:00:01Z"));
        rt.record_row(sample_row("evt:a", "2026-05-10T10:00:01Z"));
        rt.record_row(sample_row("evt:c", "2026-05-10T10:00:00Z"));
        let snap = rt.snapshot();
        let ids: Vec<&str> = snap.rows.iter().map(|r| r.event_row_id.as_str()).collect();
        assert_eq!(ids, vec!["evt:c", "evt:a", "evt:b"]);
    }

    #[test]
    fn second_observation_preserves_minted_at_and_bumps_occurrence_count() {
        let mut rt = ActivityTimelineRuntime::new();
        rt.record_row(sample_row("evt:a", "2026-05-10T10:00:00Z"));
        let mut later = sample_row("evt:a", "2026-05-10T10:00:05Z");
        later.minted_at = "2026-05-10T10:00:05Z".into();
        rt.record_row(later);
        let row = rt.find_row("evt:a").expect("row");
        assert_eq!(row.minted_at, "2026-05-10T10:00:00Z");
        assert_eq!(row.occurrence_count, 2);
    }
}
