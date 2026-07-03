//! One reusable M5 chronology-grouping primitive: phase-grouped timeline groups,
//! one-sentence narrative summary cards, timezone-safe export previews, and
//! no-lost-causality ordering across every M5 history that leaves the live surface.
//!
//! Aureline's frozen component matrix
//! ([`crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix`])
//! names the timeline group, the narrative summary card, and the chronology export
//! preview as three governed component families and freezes their controlled verb
//! vocabulary, provenance badges, chronology detail states, and export fields. The
//! evidence / activity row primitive
//! ([`crate::implement_the_m5_evidence_and_activity_timeline_row_primitive`]) turned
//! one event into a stable, copyable row. This module takes the next step: it turns
//! a *sequence* of events into a usable chronology *surface* — grouped phases with
//! retained ordering, a one-sentence state summary, absolute / relative time parity,
//! and an export preview that keeps causality visible when history leaves the
//! product.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_chronology`] — that takes one history lane's ordered
//!    events (each carrying a phase, a monotonic causal sequence, an absolute
//!    timestamp, a relative scanning label, a stable verb, a provenance badge, a
//!    controlled outcome, an object / scope, a consequential flag, and an optional
//!    reopen anchor) plus an export request, and produces one
//!    [`M5ResolvedChronology`] carrying (a) the phase-grouped timeline groups with
//!    range labels, retained ordering, event counts, primary outcomes, and
//!    collapse / expand state; (b) the narrative summary card explaining current
//!    state, the most recent consequential event, the next action, and the
//!    export / open-details path; and (c) the export preview declaring the selected
//!    range, included fields, time zone, redaction class, and output format without
//!    ever flattening the causal order. The resolver never reorders events, never
//!    drops the absolute timestamp behind the relative label, and never emits an
//!    export preview that loses causality.
//! 2. A parity matrix — [`M5ChronologyGroupPrimitivePacket`] — that binds one row
//!    per claimed M5 history lane (AI, policy, task, remote, update, and support) to
//!    the shared grouping / narrative / export anatomy, the same stable verb
//!    vocabulary and provenance badges, the same chronology detail states, the same
//!    export fields, and worked resolution cases, so the support / export packet
//!    reconstructs the grouped chronology from one shared model on every lane.
//!
//! The stable verbs ([`M5ChronologyVerb`]), the provenance badges
//! ([`M5ProvenanceBadge`]), the chronology detail states
//! ([`M5ChronologyDetailState`]), the chronology export fields
//! ([`M5ChronologyExportField`]), the non-visual accessibility routes
//! ([`M5TrustAccessibilityRoute`]), the qualification classes
//! ([`M5TrustQualificationClass`]), and the downgrade triggers
//! ([`M5TrustComponentDowngradeTrigger`]) are reused verbatim from the frozen
//! component matrix; the shell topology — zones, responsive classes, window
//! classes, and consumer surfaces — is reused from the frozen shell-zone matrix.
//! This module mints new vocabulary only for what the frozen matrix left implicit
//! about grouping a chronology: its history lanes, its phases, its resolver-side
//! outcomes, its next actions, its redaction classes, its export output formats,
//! its surface anatomy, and its focus behaviors. No M5 surface invents a second
//! chronology grammar.
//!
//! Raw URLs, raw local paths, raw usernames, raw hostnames, tokens, credentials,
//! and user text bodies stay outside the support boundary; opaque, export-safe
//! reprs are the only material carried.
//!
//! The boundary schema is
//! [`schemas/ui/m5-chronology-export-preview.schema.json`](../../../../schemas/ui/m5-chronology-export-preview.schema.json)
//! and the contract doc is
//! [`docs/components/m5_chronology_groups_primitive_contract.md`](../../../../docs/components/m5_chronology_groups_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-chronology-groups-primitive/`](../../../../fixtures/ui/m5-chronology-groups-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_chronology_group_primitive_packet,
    seeded_m5_chronology_group_primitive_support_exports_preview_narrowed,
    seeded_m5_chronology_group_primitive_update_history_beta_narrowed,
    M5_CHRONOLOGY_GROUP_PRIMITIVE_PACKET_ID,
};

// The stable chronology verbs, provenance badges, chronology detail states,
// chronology export fields, accessibility routes, qualification classes, and
// downgrade triggers are frozen once, in the trust-chronology component matrix.
// This primitive reuses them verbatim so it never invents a parallel chronology
// grammar.
pub use crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix::{
    M5ChronologyDetailState, M5ChronologyExportField, M5ChronologyVerb, M5ProvenanceBadge,
    M5TrustAccessibilityRoute, M5TrustComponentDowngradeTrigger, M5TrustQualificationClass,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ChronologyGroupPrimitivePacket`].
pub const M5_CHRONOLOGY_GROUP_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_chronology_group_narrative_card_timezone_safe_export_preview_and_no_lost_causality_primitive";

/// Schema version for M5 chronology-group-primitive records.
pub const M5_CHRONOLOGY_GROUP_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the chronology-group-primitive boundary schema.
pub const M5_CHRONOLOGY_GROUP_SCHEMA_REF: &str =
    "schemas/ui/m5-chronology-export-preview.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CHRONOLOGY_GROUP_DOC_REF: &str =
    "docs/components/m5_chronology_groups_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_CHRONOLOGY_GROUP_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen component matrix this primitive narrows from.
pub const M5_CHRONOLOGY_GROUP_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-trust-chronology-components.schema.json";

/// Repo-relative path of the evidence-timeline contract this primitive groups from.
pub const M5_CHRONOLOGY_GROUP_EVIDENCE_TIMELINE_REF: &str =
    "schemas/support/evidence_timeline.schema.json";

/// Repo-relative path of the export-redaction-profile contract this primitive
/// declares against.
pub const M5_CHRONOLOGY_GROUP_REDACTION_PROFILE_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the chronology-lineage contract this primitive preserves.
pub const M5_CHRONOLOGY_GROUP_LINEAGE_REF: &str =
    "schemas/governance/m5_evidence_chronology_lineage.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CHRONOLOGY_GROUP_FIXTURE_DIR: &str = "fixtures/ui/m5-chronology-groups-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CHRONOLOGY_GROUP_ARTIFACT_REF: &str =
    "artifacts/release/m5-chronology-groups-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CHRONOLOGY_GROUP_CSV_REF: &str =
    "artifacts/release/m5-chronology-groups-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_CHRONOLOGY_GROUP_REPORT_REF: &str =
    "artifacts/components/m5-chronology-groups-primitive.md";

/// The chronology export fields every export preview must carry so support / export
/// never drops a truth-bearing column and can reconstruct the grouped chronology
/// without a screenshot. `RedactionClass` is carried on the preview as its own field
/// but is not part of this mandatory-truth core.
pub const MANDATORY_EXPORT_FIELDS: [M5ChronologyExportField; 6] = [
    M5ChronologyExportField::EventVerb,
    M5ChronologyExportField::Provenance,
    M5ChronologyExportField::Timestamp,
    M5ChronologyExportField::ObjectRef,
    M5ChronologyExportField::ActorRole,
    M5ChronologyExportField::OutcomeCode,
];

/// One claimed M5 history lane that renders the shared grouped chronology and
/// export preview. These are the histories the goal names — AI, policy, task,
/// remote, update, and support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyHistoryLane {
    /// AI evidence: what an AI run read, ran, and produced.
    AiEvidence,
    /// Policy changes: admin / policy approvals and denials.
    PolicyChanges,
    /// Task events: the task / job lifecycle.
    TaskEvents,
    /// Remote reconnects: remote-host connection recovery history.
    RemoteReconnects,
    /// Update history: application update / channel history.
    UpdateHistory,
    /// Support exports: what a support / export flow captured.
    SupportExports,
}

impl M5ChronologyHistoryLane {
    /// Every claimed history lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AiEvidence,
        Self::PolicyChanges,
        Self::TaskEvents,
        Self::RemoteReconnects,
        Self::UpdateHistory,
        Self::SupportExports,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiEvidence => "ai_evidence",
            Self::PolicyChanges => "policy_changes",
            Self::TaskEvents => "task_events",
            Self::RemoteReconnects => "remote_reconnects",
            Self::UpdateHistory => "update_history",
            Self::SupportExports => "support_exports",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AiEvidence => "AI Evidence",
            Self::PolicyChanges => "Policy Changes",
            Self::TaskEvents => "Task Events",
            Self::RemoteReconnects => "Remote Reconnects",
            Self::UpdateHistory => "Update History",
            Self::SupportExports => "Support Exports",
        }
    }
}

/// The controlled phase a chronology event belongs to. A phase groups a contiguous
/// run of events on the timeline so a long history reads as labeled bands rather
/// than an undifferentiated event list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyPhase {
    /// The activity was set up / initiated.
    Initiation,
    /// The activity ran / progressed.
    Execution,
    /// The activity was reviewed / decided.
    Review,
    /// A degraded / failed state was recovered.
    Recovery,
    /// The activity reached a terminal / resolved state.
    Resolution,
}

impl M5ChronologyPhase {
    /// Every phase, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Initiation,
        Self::Execution,
        Self::Review,
        Self::Recovery,
        Self::Resolution,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initiation => "initiation",
            Self::Execution => "execution",
            Self::Review => "review",
            Self::Recovery => "recovery",
            Self::Resolution => "resolution",
        }
    }

    /// Review-safe label used in phase / range headers.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Initiation => "Initiation",
            Self::Execution => "Execution",
            Self::Review => "Review",
            Self::Recovery => "Recovery",
            Self::Resolution => "Resolution",
        }
    }
}

/// The controlled outcome a chronology event resolves to. Kept orthogonal to the
/// stable verb (the verb is *what happened*, the outcome is *how it ended*). This is
/// a resolver-side vocabulary and is not part of the frozen component-matrix set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyOutcome {
    /// The action completed successfully.
    Succeeded,
    /// The action failed.
    Failed,
    /// The action is pending / in progress.
    Pending,
    /// The action was denied.
    Denied,
    /// A prior change was reverted.
    Reverted,
}

impl M5ChronologyOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Succeeded,
        Self::Failed,
        Self::Pending,
        Self::Denied,
        Self::Reverted,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Pending => "pending",
            Self::Denied => "denied",
            Self::Reverted => "reverted",
        }
    }
}

/// The controlled next action the narrative summary card proposes. Derived from the
/// most recent consequential event so the card always ends with what to do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NextAction {
    /// Wait for the in-progress activity to complete.
    AwaitCompletion,
    /// Review the completed / denied result.
    ReviewResult,
    /// Retry or recover the failed activity.
    RetryOrRecover,
    /// Acknowledge the reverted / recovered resolution.
    AcknowledgeResolution,
    /// No further action is needed.
    NoActionNeeded,
}

impl M5NextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AwaitCompletion,
        Self::ReviewResult,
        Self::RetryOrRecover,
        Self::AcknowledgeResolution,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitCompletion => "await_completion",
            Self::ReviewResult => "review_result",
            Self::RetryOrRecover => "retry_or_recover",
            Self::AcknowledgeResolution => "acknowledge_resolution",
            Self::NoActionNeeded => "no_action_needed",
        }
    }

    /// One-sentence hint carried in the narrative card.
    pub const fn sentence(self) -> &'static str {
        match self {
            Self::AwaitCompletion => "Await completion of the in-progress activity.",
            Self::ReviewResult => "Review the result and confirm it matches intent.",
            Self::RetryOrRecover => "Retry or recover the failed activity.",
            Self::AcknowledgeResolution => {
                "Acknowledge the resolution and confirm the restored state."
            }
            Self::NoActionNeeded => "No further action is needed.",
        }
    }
}

/// The controlled redaction class an export preview declares, so an export never
/// silently changes what it discloses. Aligns with the export-redaction-profile
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyRedactionClass {
    /// Only export-safe metadata reprs cross the boundary.
    MetadataOnly,
    /// Actor identities are pseudonymized before export.
    PseudonymizedActors,
    /// Only aggregate counts, not per-event rows, are exported.
    AggregateCountsOnly,
}

impl M5ChronologyRedactionClass {
    /// Every redaction class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::MetadataOnly,
        Self::PseudonymizedActors,
        Self::AggregateCountsOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::PseudonymizedActors => "pseudonymized_actors",
            Self::AggregateCountsOnly => "aggregate_counts_only",
        }
    }
}

/// The controlled output format an export preview declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyExportFormat {
    /// A single JSON document.
    Json,
    /// Comma-separated values.
    Csv,
    /// A Markdown chronology.
    Markdown,
    /// Newline-delimited JSON stream (one event per line).
    NdjsonStream,
}

impl M5ChronologyExportFormat {
    /// Every output format, in declaration order.
    pub const ALL: [Self; 4] = [Self::Json, Self::Csv, Self::Markdown, Self::NdjsonStream];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Csv => "csv",
            Self::Markdown => "markdown",
            Self::NdjsonStream => "ndjson_stream",
        }
    }
}

/// One anatomy part the shared chronology surface renders. Every lane renders the
/// full anatomy — the timeline groups, the narrative card, the export preview, and
/// the absolute / relative time parity are the guarantee, not local polish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologySurfaceAnatomyPart {
    /// A group's phase / time-range label.
    PhaseRangeLabel,
    /// The retained causal ordering across and within groups.
    RetainedGroupOrdering,
    /// A group's event count.
    GroupEventCount,
    /// A group's primary outcome.
    GroupPrimaryOutcome,
    /// The collapse / expand control on a group.
    CollapseExpandControl,
    /// The narrative card's one-sentence current-state summary.
    NarrativeCurrentState,
    /// The narrative card's most-recent-consequential event.
    NarrativeRecentConsequentialEvent,
    /// The narrative card's next-action hint.
    NarrativeNextAction,
    /// The narrative card's export / open-details path.
    NarrativeExportOrDetailsPath,
    /// The export preview's selected range.
    ExportSelectedRange,
    /// The export preview's included fields.
    ExportIncludedFields,
    /// The export preview's declared time zone.
    ExportTimeZone,
    /// The export preview's redaction class.
    ExportRedactionClass,
    /// The export preview's output format.
    ExportOutputFormat,
    /// The relative scanning label on every event.
    RelativeTimeLabel,
    /// The absolute timestamp on every event, kept in detail and export.
    AbsoluteTimestamp,
}

impl M5ChronologySurfaceAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::PhaseRangeLabel,
        Self::RetainedGroupOrdering,
        Self::GroupEventCount,
        Self::GroupPrimaryOutcome,
        Self::CollapseExpandControl,
        Self::NarrativeCurrentState,
        Self::NarrativeRecentConsequentialEvent,
        Self::NarrativeNextAction,
        Self::NarrativeExportOrDetailsPath,
        Self::ExportSelectedRange,
        Self::ExportIncludedFields,
        Self::ExportTimeZone,
        Self::ExportRedactionClass,
        Self::ExportOutputFormat,
        Self::RelativeTimeLabel,
        Self::AbsoluteTimestamp,
    ];

    /// Every anatomy part is mandatory: a lane that renders grouped chronology must
    /// render groups, the narrative card, the export preview, and time parity.
    pub const MANDATORY: [Self; 16] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhaseRangeLabel => "phase_range_label",
            Self::RetainedGroupOrdering => "retained_group_ordering",
            Self::GroupEventCount => "group_event_count",
            Self::GroupPrimaryOutcome => "group_primary_outcome",
            Self::CollapseExpandControl => "collapse_expand_control",
            Self::NarrativeCurrentState => "narrative_current_state",
            Self::NarrativeRecentConsequentialEvent => "narrative_recent_consequential_event",
            Self::NarrativeNextAction => "narrative_next_action",
            Self::NarrativeExportOrDetailsPath => "narrative_export_or_details_path",
            Self::ExportSelectedRange => "export_selected_range",
            Self::ExportIncludedFields => "export_included_fields",
            Self::ExportTimeZone => "export_time_zone",
            Self::ExportRedactionClass => "export_redaction_class",
            Self::ExportOutputFormat => "export_output_format",
            Self::RelativeTimeLabel => "relative_time_label",
            Self::AbsoluteTimestamp => "absolute_timestamp",
        }
    }
}

/// A focus / navigation behavior the chronology surface supports so grouping,
/// narrative, and export stay keyboard-reachable and never hover-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyFocusBehavior {
    /// A group header is reachable and operable by keyboard focus.
    GroupHeaderFocusable,
    /// The collapse / expand control is keyboard-operable.
    CollapseExpandKeyboardOperable,
    /// The narrative card is reachable by keyboard focus.
    NarrativeCardFocusable,
    /// The next-action hint is keyboard-reachable.
    NextActionReachable,
    /// The export preview is keyboard-reachable.
    ExportPreviewReachable,
    /// The open-details / reopen path is keyboard-reachable.
    OpenDetailsReachable,
    /// Keyboard navigation moves per group.
    PerGroupNavigation,
}

impl M5ChronologyFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::GroupHeaderFocusable,
        Self::CollapseExpandKeyboardOperable,
        Self::NarrativeCardFocusable,
        Self::NextActionReachable,
        Self::ExportPreviewReachable,
        Self::OpenDetailsReachable,
        Self::PerGroupNavigation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GroupHeaderFocusable => "group_header_focusable",
            Self::CollapseExpandKeyboardOperable => "collapse_expand_keyboard_operable",
            Self::NarrativeCardFocusable => "narrative_card_focusable",
            Self::NextActionReachable => "next_action_reachable",
            Self::ExportPreviewReachable => "export_preview_reachable",
            Self::OpenDetailsReachable => "open_details_reachable",
            Self::PerGroupNavigation => "per_group_navigation",
        }
    }
}

/// One raw chronology event, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyEventItem {
    /// The phase this event belongs to.
    pub phase: M5ChronologyPhase,
    /// The monotonic causal sequence index. Must strictly increase across the input
    /// so causality is never ambiguous.
    pub sequence: u32,
    /// Opaque, export-safe absolute timestamp (RFC 3339, never local wall text).
    pub absolute_timestamp: String,
    /// Opaque, export-safe relative scanning label (e.g. `2h ago`).
    pub relative_label: String,
    /// The stable verb describing what happened.
    pub verb: M5ChronologyVerb,
    /// The provenance badge attributing initiation.
    pub provenance: M5ProvenanceBadge,
    /// The controlled outcome of the event.
    pub outcome: M5ChronologyOutcome,
    /// Opaque, export-safe object / scope representation the action touched.
    pub object_repr: String,
    /// True when this event is a consequential / state-changing event.
    pub consequential: bool,
    /// Opaque, export-safe reopen anchor into durable detail, when present.
    pub detail_ref: Option<String>,
}

impl M5ChronologyEventItem {
    /// True when any representation carries forbidden material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.absolute_timestamp)
            || repr_is_forbidden(&self.relative_label)
            || repr_is_forbidden(&self.object_repr)
            || self.detail_ref.as_deref().is_some_and(repr_is_forbidden)
    }
}

/// The export request declared for one history lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyExportRequest {
    /// Opaque, export-safe start of the selected absolute range.
    pub selected_range_start: String,
    /// Opaque, export-safe end of the selected absolute range.
    pub selected_range_end: String,
    /// The declared time zone the exported timestamps are expressed in.
    pub time_zone_repr: String,
    /// The declared redaction class.
    pub redaction_class: M5ChronologyRedactionClass,
    /// The declared output format.
    pub output_format: M5ChronologyExportFormat,
    /// The fields the export promises to carry (must include the mandatory truth
    /// fields).
    pub included_fields: Vec<M5ChronologyExportField>,
}

/// The full input to the chronology resolver for one history lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyResolutionInput {
    /// The history lane these events render on.
    pub history_lane: M5ChronologyHistoryLane,
    /// The raw events, in causal (sequence) order. Must be non-empty.
    pub events: Vec<M5ChronologyEventItem>,
    /// The export request declared for this lane.
    pub export_request: M5ChronologyExportRequest,
}

/// The resolved posture of one chronology event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedChronologyEvent {
    /// The causal sequence index.
    pub sequence: u32,
    /// The phase this event belongs to.
    pub phase: M5ChronologyPhase,
    /// The absolute timestamp, kept in detail and export.
    pub absolute_timestamp: String,
    /// The relative scanning label.
    pub relative_label: String,
    /// The stable verb.
    pub verb: M5ChronologyVerb,
    /// The provenance badge.
    pub provenance: M5ProvenanceBadge,
    /// The controlled outcome.
    pub outcome: M5ChronologyOutcome,
    /// The opaque object / scope representation.
    pub object_repr: String,
    /// Whether this event is consequential / state-changing.
    pub consequential: bool,
    /// The resolved chronology detail state.
    pub detail_state: M5ChronologyDetailState,
    /// The opaque reopen anchor, when present.
    pub detail_ref: Option<String>,
}

/// One resolved timeline group: a contiguous run of same-phase events with a phase /
/// range label, retained ordering, count, primary outcome, and collapse state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTimelineGroup {
    /// The phase this group covers.
    pub phase: M5ChronologyPhase,
    /// The phase / range header label.
    pub phase_range_label: String,
    /// The absolute start of the group's range.
    pub range_start_absolute: String,
    /// The absolute end of the group's range.
    pub range_end_absolute: String,
    /// The relative scanning label for the group's range.
    pub range_relative_label: String,
    /// The number of events in the group.
    pub event_count: usize,
    /// The group's primary outcome (the terminal event's outcome).
    pub primary_outcome: M5ChronologyOutcome,
    /// The first (lowest) sequence in the group — retained ordering.
    pub first_sequence: u32,
    /// The last (highest) sequence in the group — retained ordering.
    pub last_sequence: u32,
    /// The collapse / expand state the group defaults to.
    pub collapse_state: M5ChronologyDetailState,
    /// The events in the group, in causal order.
    pub events: Vec<M5ResolvedChronologyEvent>,
}

/// The resolved narrative summary card for one history lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedNarrativeCard {
    /// The one-sentence current-state summary.
    pub current_state_sentence: String,
    /// The most recent consequential event.
    pub most_recent_consequential: M5ResolvedChronologyEvent,
    /// The controlled next action.
    pub next_action: M5NextAction,
    /// The one-sentence next-action hint.
    pub next_action_sentence: String,
    /// The reopen anchor to open details, when present.
    pub open_details_ref: Option<String>,
    /// True when an export path is available (an export preview was produced).
    pub export_path_available: bool,
}

/// The resolved export preview for one history lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedExportPreview {
    /// The declared start of the selected absolute range.
    pub selected_range_start: String,
    /// The declared end of the selected absolute range.
    pub selected_range_end: String,
    /// The fields the export carries.
    pub included_fields: Vec<M5ChronologyExportField>,
    /// The declared time zone.
    pub time_zone_repr: String,
    /// The declared redaction class.
    pub redaction_class: M5ChronologyRedactionClass,
    /// The declared output format.
    pub output_format: M5ChronologyExportFormat,
    /// The causal order (event sequences) carried into the export.
    pub event_order: Vec<u32>,
    /// True when the export carries the events in strictly increasing causal order.
    pub preserves_causal_order: bool,
}

/// The resolved chronology for one history lane — groups, narrative, and export
/// preview from one shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedChronology {
    /// The history lane this chronology renders on.
    pub history_lane: M5ChronologyHistoryLane,
    /// The phase-grouped timeline groups, in causal order.
    pub groups: Vec<M5ResolvedTimelineGroup>,
    /// The narrative summary card.
    pub narrative: M5ResolvedNarrativeCard,
    /// The export preview.
    pub export_preview: M5ResolvedExportPreview,
    /// The total number of events across all groups.
    pub total_event_count: usize,
    /// True when groups and events retain strictly increasing causal order with no
    /// event lost.
    pub preserves_causal_order: bool,
}

/// Errors returned by [`resolve_chronology`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ChronologyResolutionError {
    /// The input carried no events.
    NoEvents,
    /// An event had an empty absolute timestamp.
    EmptyTimestamp,
    /// An event had an empty relative label.
    EmptyRelativeLabel,
    /// An event had an empty object / scope.
    EmptyObject,
    /// The event sequences did not strictly increase — causality would be
    /// ambiguous.
    NonMonotonicSequence,
    /// The export request had an empty selected range.
    EmptyExportRange,
    /// The export request had an empty time zone.
    EmptyTimeZone,
    /// The export request omitted a mandatory truth field.
    MissingMandatoryExportField,
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5ChronologyResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoEvents => "no_events",
            Self::EmptyTimestamp => "empty_timestamp",
            Self::EmptyRelativeLabel => "empty_relative_label",
            Self::EmptyObject => "empty_object",
            Self::NonMonotonicSequence => "non_monotonic_sequence",
            Self::EmptyExportRange => "empty_export_range",
            Self::EmptyTimeZone => "empty_time_zone",
            Self::MissingMandatoryExportField => "missing_mandatory_export_field",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ChronologyResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "chronology resolution error: {}", self.as_str())
    }
}

impl Error for M5ChronologyResolutionError {}

/// Resolves one history lane's events into one grouped chronology.
///
/// Events are grouped into contiguous phase runs so ordering is retained exactly:
/// a new group starts whenever the phase differs from the previous event's. Each
/// event resolves to a [`M5ChronologyDetailState`] (reopenable when it carries a
/// detail anchor, else collapsed). The narrative card explains the current state,
/// the most recent consequential event, and the next action. The export preview
/// declares the selected range, fields, time zone, redaction class, and output
/// format while carrying the events in strictly increasing causal order, so history
/// never flattens into ambiguous text when it leaves the live surface.
pub fn resolve_chronology(
    input: &M5ChronologyResolutionInput,
) -> Result<M5ResolvedChronology, M5ChronologyResolutionError> {
    if input.events.is_empty() {
        return Err(M5ChronologyResolutionError::NoEvents);
    }

    let mut previous_sequence: Option<u32> = None;
    for event in &input.events {
        if event.absolute_timestamp.trim().is_empty() {
            return Err(M5ChronologyResolutionError::EmptyTimestamp);
        }
        if event.relative_label.trim().is_empty() {
            return Err(M5ChronologyResolutionError::EmptyRelativeLabel);
        }
        if event.object_repr.trim().is_empty() {
            return Err(M5ChronologyResolutionError::EmptyObject);
        }
        if event.carries_forbidden_material() {
            return Err(M5ChronologyResolutionError::ForbiddenMaterial);
        }
        if let Some(prev) = previous_sequence {
            if event.sequence <= prev {
                return Err(M5ChronologyResolutionError::NonMonotonicSequence);
            }
        }
        previous_sequence = Some(event.sequence);
    }

    validate_export_request(&input.export_request)?;

    let resolved_events: Vec<M5ResolvedChronologyEvent> =
        input.events.iter().map(resolve_event).collect();

    let groups = group_by_phase(&resolved_events);
    let narrative = resolve_narrative(&resolved_events);
    let export_preview = resolve_export_preview(&input.export_request, &resolved_events);

    let event_order: Vec<u32> = resolved_events.iter().map(|event| event.sequence).collect();
    let preserves_causal_order = is_strictly_increasing(&event_order)
        && groups_retain_order(&groups)
        && groups.iter().map(|group| group.event_count).sum::<usize>() == resolved_events.len();

    Ok(M5ResolvedChronology {
        history_lane: input.history_lane,
        groups,
        narrative,
        export_preview,
        total_event_count: resolved_events.len(),
        preserves_causal_order,
    })
}

/// Validates one export request's non-representation invariants.
fn validate_export_request(
    request: &M5ChronologyExportRequest,
) -> Result<(), M5ChronologyResolutionError> {
    if request.selected_range_start.trim().is_empty()
        || request.selected_range_end.trim().is_empty()
    {
        return Err(M5ChronologyResolutionError::EmptyExportRange);
    }
    if request.time_zone_repr.trim().is_empty() {
        return Err(M5ChronologyResolutionError::EmptyTimeZone);
    }
    if repr_is_forbidden(&request.selected_range_start)
        || repr_is_forbidden(&request.selected_range_end)
        || repr_is_forbidden(&request.time_zone_repr)
    {
        return Err(M5ChronologyResolutionError::ForbiddenMaterial);
    }
    let present: BTreeSet<M5ChronologyExportField> =
        request.included_fields.iter().copied().collect();
    if !MANDATORY_EXPORT_FIELDS
        .iter()
        .all(|field| present.contains(field))
    {
        return Err(M5ChronologyResolutionError::MissingMandatoryExportField);
    }
    Ok(())
}

/// Resolves one event's detail state.
fn resolve_event(event: &M5ChronologyEventItem) -> M5ResolvedChronologyEvent {
    let detail_state = if event.detail_ref.is_some() {
        M5ChronologyDetailState::ReopenableDetail
    } else {
        M5ChronologyDetailState::Collapsed
    };

    M5ResolvedChronologyEvent {
        sequence: event.sequence,
        phase: event.phase,
        absolute_timestamp: event.absolute_timestamp.clone(),
        relative_label: event.relative_label.clone(),
        verb: event.verb,
        provenance: event.provenance,
        outcome: event.outcome,
        object_repr: event.object_repr.clone(),
        consequential: event.consequential,
        detail_state,
        detail_ref: event.detail_ref.clone(),
    }
}

/// Groups resolved events into contiguous phase runs, retaining causal order.
fn group_by_phase(events: &[M5ResolvedChronologyEvent]) -> Vec<M5ResolvedTimelineGroup> {
    let mut groups: Vec<Vec<M5ResolvedChronologyEvent>> = Vec::new();
    for event in events {
        match groups.last_mut() {
            Some(current) if current[0].phase == event.phase => current.push(event.clone()),
            _ => groups.push(vec![event.clone()]),
        }
    }

    let group_count = groups.len();
    groups
        .into_iter()
        .enumerate()
        .map(|(index, run)| build_group(run, index == group_count - 1))
        .collect()
}

/// Builds one resolved timeline group from a contiguous phase run.
fn build_group(
    run: Vec<M5ResolvedChronologyEvent>,
    is_last_group: bool,
) -> M5ResolvedTimelineGroup {
    let phase = run[0].phase;
    let first = &run[0];
    let last = &run[run.len() - 1];
    let phase_range_label = format!(
        "{} · {} – {}",
        phase.label(),
        first.relative_label,
        last.relative_label
    );
    // A group defaults to expanded when it holds a failure or is the most recent
    // group; older, clean groups collapse to a summary. Either way the underlying
    // ordering and detail are retained.
    let contains_failure = run.iter().any(|event| {
        matches!(
            event.outcome,
            M5ChronologyOutcome::Failed | M5ChronologyOutcome::Denied
        )
    });
    let collapse_state = if contains_failure || is_last_group {
        M5ChronologyDetailState::Expanded
    } else {
        M5ChronologyDetailState::Collapsed
    };

    M5ResolvedTimelineGroup {
        phase,
        phase_range_label,
        range_start_absolute: first.absolute_timestamp.clone(),
        range_end_absolute: last.absolute_timestamp.clone(),
        range_relative_label: format!("{} – {}", first.relative_label, last.relative_label),
        event_count: run.len(),
        primary_outcome: last.outcome,
        first_sequence: first.sequence,
        last_sequence: last.sequence,
        collapse_state,
        events: run,
    }
}

/// Resolves the narrative summary card from the resolved events.
fn resolve_narrative(events: &[M5ResolvedChronologyEvent]) -> M5ResolvedNarrativeCard {
    // The most recent consequential event, or the last event when none is flagged.
    let most_recent = events
        .iter()
        .rev()
        .find(|event| event.consequential)
        .unwrap_or_else(|| &events[events.len() - 1])
        .clone();

    let current_state_sentence = format!(
        "Current state: {} {} with outcome {} (as of {}, {}), initiated by {}.",
        most_recent.object_repr,
        most_recent.verb.as_str(),
        most_recent.outcome.as_str(),
        most_recent.absolute_timestamp,
        most_recent.relative_label,
        most_recent.provenance.as_str(),
    );

    let next_action = next_action_for(&most_recent);

    M5ResolvedNarrativeCard {
        current_state_sentence,
        open_details_ref: most_recent.detail_ref.clone(),
        next_action,
        next_action_sentence: next_action.sentence().to_owned(),
        export_path_available: true,
        most_recent_consequential: most_recent,
    }
}

/// Derives the next action from the most recent consequential event.
fn next_action_for(event: &M5ResolvedChronologyEvent) -> M5NextAction {
    if event.verb == M5ChronologyVerb::Exported && event.outcome == M5ChronologyOutcome::Succeeded {
        return M5NextAction::NoActionNeeded;
    }
    match event.outcome {
        M5ChronologyOutcome::Succeeded => M5NextAction::ReviewResult,
        M5ChronologyOutcome::Failed => M5NextAction::RetryOrRecover,
        M5ChronologyOutcome::Pending => M5NextAction::AwaitCompletion,
        M5ChronologyOutcome::Denied => M5NextAction::ReviewResult,
        M5ChronologyOutcome::Reverted => M5NextAction::AcknowledgeResolution,
    }
}

/// Resolves the export preview, carrying the events in strictly increasing causal
/// order.
fn resolve_export_preview(
    request: &M5ChronologyExportRequest,
    events: &[M5ResolvedChronologyEvent],
) -> M5ResolvedExportPreview {
    let event_order: Vec<u32> = events.iter().map(|event| event.sequence).collect();
    let preserves_causal_order = is_strictly_increasing(&event_order);
    M5ResolvedExportPreview {
        selected_range_start: request.selected_range_start.clone(),
        selected_range_end: request.selected_range_end.clone(),
        included_fields: request.included_fields.clone(),
        time_zone_repr: request.time_zone_repr.clone(),
        redaction_class: request.redaction_class,
        output_format: request.output_format,
        event_order,
        preserves_causal_order,
    }
}

/// True when a sequence of indices strictly increases.
fn is_strictly_increasing(order: &[u32]) -> bool {
    order.windows(2).all(|pair| pair[0] < pair[1])
}

/// True when the groups themselves retain causal order: each group's first sequence
/// follows the previous group's last sequence.
fn groups_retain_order(groups: &[M5ResolvedTimelineGroup]) -> bool {
    groups.windows(2).all(|pair| {
        pair[0].last_sequence < pair[1].first_sequence
            && pair[0].first_sequence <= pair[0].last_sequence
    })
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs the grouped chronology from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyResolutionCase {
    /// The resolver input.
    pub input: M5ChronologyResolutionInput,
    /// The resolved chronology. Must equal `resolve_chronology(&input)`.
    pub resolved: M5ResolvedChronology,
}

impl M5ChronologyResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ChronologyResolutionInput) -> Self {
        let resolved = resolve_chronology(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_chronology(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one history lane bound to the shared grouping /
/// narrative / export anatomy, stable verbs, provenance badges, chronology detail
/// states, export fields, and worked resolution cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologySurfaceRow {
    /// History lane.
    pub history_lane: M5ChronologyHistoryLane,
    /// Qualification class earned by this lane.
    pub qualification: M5TrustQualificationClass,
    /// Owner role accountable for keeping this lane governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this chronology attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this surface must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this surface keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Anatomy parts this surface renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ChronologySurfaceAnatomyPart>,
    /// Phases this lane can group into.
    pub phases: Vec<M5ChronologyPhase>,
    /// Stable verbs this lane can show.
    pub chronology_verbs: Vec<M5ChronologyVerb>,
    /// Provenance badges this lane can attribute.
    pub provenance_badges: Vec<M5ProvenanceBadge>,
    /// Outcomes this lane can resolve to.
    pub outcomes: Vec<M5ChronologyOutcome>,
    /// Chronology detail states this lane projects.
    pub detail_states: Vec<M5ChronologyDetailState>,
    /// Next actions the narrative card can propose.
    pub next_actions: Vec<M5NextAction>,
    /// Redaction classes this lane's export preview can declare.
    pub redaction_classes: Vec<M5ChronologyRedactionClass>,
    /// Output formats this lane's export preview can declare.
    pub export_formats: Vec<M5ChronologyExportFormat>,
    /// Export fields this lane carries (must include the mandatory truth fields).
    pub export_fields: Vec<M5ChronologyExportField>,
    /// Focus behaviors this lane supports.
    pub focus_behaviors: Vec<M5ChronologyFocusBehavior>,
    /// Non-visual accessibility routes this lane offers.
    pub accessibility_routes: Vec<M5TrustAccessibilityRoute>,
    /// Shell subsystems that consume this lane's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5TrustComponentDowngradeTrigger>,
    /// Proof packet refs that keep this lane current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this lane.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this lane.
    pub example_chronologies: Vec<M5ChronologyResolutionCase>,
    /// Hard invariant: this lane never flattens causal ordering. MUST be `false`.
    pub flattens_causal_ordering: bool,
    /// Hard invariant: this lane never drops the absolute timestamp behind the
    /// relative label. MUST be `false`.
    pub drops_absolute_timestamp: bool,
    /// Hard invariant: this lane never drops the export redaction intent. MUST be
    /// `false`.
    pub drops_redaction_intent: bool,
    /// Hard invariant: this lane never drops export / audit truth. MUST be `false`.
    pub drops_export_or_audit_truth: bool,
}

impl M5ChronologySurfaceRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ChronologySurfaceAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ChronologySurfaceAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ChronologyExportField> =
            self.export_fields.iter().copied().collect();
        MANDATORY_EXPORT_FIELDS
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when every worked case resolves on this lane.
    fn examples_match_lane(&self) -> bool {
        self.example_chronologies
            .iter()
            .all(|case| case.input.history_lane == self.history_lane)
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.flattens_causal_ordering
            && !self.drops_absolute_timestamp
            && !self.drops_redaction_intent
            && !self.drops_export_or_audit_truth
    }
}

/// Self-describing controlled-vocabulary set minted by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyGroupVocabularySet {
    /// History-lane tokens.
    pub history_lanes: Vec<String>,
    /// Phase tokens.
    pub phases: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Stable verb tokens (reused from the frozen matrix).
    pub chronology_verbs: Vec<String>,
    /// Provenance-badge tokens (reused from the frozen matrix).
    pub provenance_badges: Vec<String>,
    /// Outcome tokens.
    pub outcomes: Vec<String>,
    /// Chronology-detail-state tokens (reused from the frozen matrix).
    pub detail_states: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Redaction-class tokens.
    pub redaction_classes: Vec<String>,
    /// Output-format tokens.
    pub export_formats: Vec<String>,
    /// Chronology-export-field tokens (reused from the frozen matrix).
    pub export_fields: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ChronologyGroupVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            history_lanes: tokens(&M5ChronologyHistoryLane::ALL, |v| v.as_str()),
            phases: tokens(&M5ChronologyPhase::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ChronologySurfaceAnatomyPart::ALL, |v| v.as_str()),
            chronology_verbs: tokens(&M5ChronologyVerb::ALL, |v| v.as_str()),
            provenance_badges: tokens(&M5ProvenanceBadge::ALL, |v| v.as_str()),
            outcomes: tokens(&M5ChronologyOutcome::ALL, |v| v.as_str()),
            detail_states: tokens(&M5ChronologyDetailState::ALL, |v| v.as_str()),
            next_actions: tokens(&M5NextAction::ALL, |v| v.as_str()),
            redaction_classes: tokens(&M5ChronologyRedactionClass::ALL, |v| v.as_str()),
            export_formats: tokens(&M5ChronologyExportFormat::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ChronologyExportField::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5ChronologyFocusBehavior::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TrustAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyGroupGovernanceReview {
    /// One chronology model is reused across every history lane.
    pub one_chronology_model_across_lanes: bool,
    /// Timeline groups retain causal ordering.
    pub timeline_groups_retain_ordering: bool,
    /// Every group declares a phase / range label, count, and primary outcome.
    pub groups_declare_phase_count_and_outcome: bool,
    /// The narrative card explains current state, recent event, and next action.
    pub narrative_explains_state_and_next_action: bool,
    /// Relative time stays available while absolute time survives in detail and
    /// export.
    pub relative_and_absolute_time_parity: bool,
    /// Export previews declare range, fields, time zone, redaction, and format.
    pub export_preview_declares_full_disclosure: bool,
    /// Export previews preserve causality instead of flattening history.
    pub export_preserves_causality: bool,
    /// The support / export packet keeps the same chronology vocabulary.
    pub support_export_keeps_chronology_vocabulary: bool,
    /// Every surface is bound to a canonical shell zone.
    pub every_surface_bound_to_shell_zone: bool,
    /// Later M5 lanes cannot invent parallel chronology vocabulary.
    pub later_lanes_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyGroupConsumerProjection {
    /// AI / policy / task / remote / update / support lanes all consume the shared
    /// chronology.
    pub history_lanes_consume_shared_chronology: bool,
    /// Grouping reads a single canonical phase vocabulary.
    pub grouping_reads_single_phase_vocabulary: bool,
    /// The narrative reads a single canonical event source.
    pub narrative_reads_single_source: bool,
    /// The export preview reads a single canonical field / redaction source.
    pub export_reads_single_source: bool,
    /// Support / export reads a single canonical chronology source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyGroupProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the chronology-group primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyGroupReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting chronology audit.
    pub chronology_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ChronologyGroupPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChronologyGroupPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5ChronologySurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ChronologyGroupVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ChronologyGroupGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChronologyGroupConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChronologyGroupProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ChronologyGroupReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 chronology-group-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyGroupPrimitivePacket {
    /// Record kind; must equal [`M5_CHRONOLOGY_GROUP_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CHRONOLOGY_GROUP_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5ChronologySurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ChronologyGroupVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ChronologyGroupGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChronologyGroupConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChronologyGroupProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ChronologyGroupReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ChronologyGroupPrimitivePacket {
    /// Builds an M5 chronology-group-primitive packet from stable-lane input.
    pub fn new(input: M5ChronologyGroupPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_CHRONOLOGY_GROUP_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_CHRONOLOGY_GROUP_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 chronology-group-primitive invariants.
    pub fn validate(&self) -> Vec<M5ChronologyGroupPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CHRONOLOGY_GROUP_PRIMITIVE_RECORD_KIND {
            violations.push(M5ChronologyGroupPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CHRONOLOGY_GROUP_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5ChronologyGroupPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ChronologyGroupPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_grouped_chronology_covered(self, &mut violations);
        validate_phase_vocabulary_covered(self, &mut violations);
        validate_time_parity_covered(self, &mut violations);
        validate_causality_preservation_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 chronology-group primitive packet serializes"),
        ) {
            violations.push(M5ChronologyGroupPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 chronology-group primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per history lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "history_lane,qualification,owner,shell_zone_slot,phases,verbs,outcomes,redaction_classes,export_formats,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.history_lane.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.phases, |v| v.as_str()),
                join_tokens(&row.chronology_verbs, |v| v.as_str()),
                join_tokens(&row.outcomes, |v| v.as_str()),
                join_tokens(&row.redaction_classes, |v| v.as_str()),
                join_tokens(&row.export_formats, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_chronologies.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Chronology Group Primitive: Grouped Phases, Narrative Cards, and Export Previews\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- History lanes: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Phases: {}\n",
            self.vocabulary_set.phases.join(", ")
        ));
        out.push_str(&format!(
            "- Anatomy parts: {}\n",
            self.vocabulary_set.anatomy_parts.join(", ")
        ));
        out.push_str(&format!(
            "- Redaction classes: {}\n",
            self.vocabulary_set.redaction_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Export formats: {}\n",
            self.vocabulary_set.export_formats.join(", ")
        ));
        out.push_str(&format!(
            "- Export fields: {}\n",
            self.vocabulary_set.export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## History lanes\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.history_lane.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked chronologies: {}\n",
                row.example_chronologies.len()
            ));
            for case in &row.example_chronologies {
                out.push_str(&format!(
                    "    - {} event(s) in {} group(s); next action: {}\n",
                    case.resolved.total_event_count,
                    case.resolved.groups.len(),
                    case.resolved.narrative.next_action.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 chronology-group-primitive export.
#[derive(Debug)]
pub enum M5ChronologyGroupPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ChronologyGroupPrimitiveViolation>),
}

impl fmt::Display for M5ChronologyGroupPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 chronology-group primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 chronology-group primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ChronologyGroupPrimitiveArtifactError {}

/// Validation failures emitted by [`M5ChronologyGroupPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ChronologyGroupPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required history lane is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A surface row declares no phases.
    PhaseMissing,
    /// A surface row declares no stable verbs.
    ChronologyVerbMissing,
    /// A surface row declares no provenance badges.
    ProvenanceBadgeMissing,
    /// A surface row declares no chronology detail states.
    DetailStateMissing,
    /// A surface row declares no next actions.
    NextActionMissing,
    /// A surface row declares no redaction classes.
    RedactionClassMissing,
    /// A surface row declares no export formats.
    ExportFormatMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no focus behaviors.
    FocusBehaviorMissing,
    /// A surface row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no worked resolution cases.
    ExampleChronologyMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleChronologyDrift,
    /// A worked resolution case's lane disagrees with its row.
    ExampleLaneMismatch,
    /// A lane claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// Not every history lane is proven to render grouped chronology and an export
    /// preview.
    GroupedChronologyUnproven,
    /// No worked resolution across the matrix exercises the full phase vocabulary.
    PhaseVocabularyUnproven,
    /// No worked resolution proves relative + absolute time parity into the export.
    TimeParityUnproven,
    /// No worked resolution proves a multi-event export preserves causal order with
    /// redaction intent.
    CausalityPreservationUnproven,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ChronologyGroupPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::PhaseMissing => "phase_missing",
            Self::ChronologyVerbMissing => "chronology_verb_missing",
            Self::ProvenanceBadgeMissing => "provenance_badge_missing",
            Self::DetailStateMissing => "detail_state_missing",
            Self::NextActionMissing => "next_action_missing",
            Self::RedactionClassMissing => "redaction_class_missing",
            Self::ExportFormatMissing => "export_format_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleChronologyMissing => "example_chronology_missing",
            Self::ExampleChronologyDrift => "example_chronology_drift",
            Self::ExampleLaneMismatch => "example_lane_mismatch",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::GroupedChronologyUnproven => "grouped_chronology_unproven",
            Self::PhaseVocabularyUnproven => "phase_vocabulary_unproven",
            Self::TimeParityUnproven => "time_parity_unproven",
            Self::CausalityPreservationUnproven => "causality_preservation_unproven",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 chronology-group-primitive export.
pub fn current_stable_m5_chronology_group_primitive_export(
) -> Result<M5ChronologyGroupPrimitivePacket, M5ChronologyGroupPrimitiveArtifactError> {
    let packet: M5ChronologyGroupPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-chronology-groups-proof/support_export.json"
    )))
    .map_err(M5ChronologyGroupPrimitiveArtifactError::SupportExport)?;

    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ChronologyGroupPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CHRONOLOGY_GROUP_SCHEMA_REF,
        M5_CHRONOLOGY_GROUP_DOC_REF,
        M5_CHRONOLOGY_GROUP_SHELL_ZONE_REF,
        M5_CHRONOLOGY_GROUP_COMPONENT_MATRIX_REF,
        M5_CHRONOLOGY_GROUP_EVIDENCE_TIMELINE_REF,
        M5_CHRONOLOGY_GROUP_REDACTION_PROFILE_REF,
        M5_CHRONOLOGY_GROUP_LINEAGE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ChronologyGroupPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ChronologyGroupPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let present: BTreeSet<M5ChronologyHistoryLane> = packet
        .surface_rows
        .iter()
        .map(|row| row.history_lane)
        .collect();
    for required in M5ChronologyHistoryLane::ALL {
        if !present.contains(&required) {
            violations.push(M5ChronologyGroupPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5ChronologyGroupPrimitiveViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ChronologyGroupPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.phases.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::PhaseMissing);
        }
        if row.chronology_verbs.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::ChronologyVerbMissing);
        }
        if row.provenance_badges.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::ProvenanceBadgeMissing);
        }
        if row.detail_states.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::DetailStateMissing);
        }
        if row.next_actions.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::NextActionMissing);
        }
        if row.redaction_classes.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::RedactionClassMissing);
        }
        if row.export_formats.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::ExportFormatMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ChronologyGroupPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::FocusBehaviorMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TrustAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ChronologyGroupPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_chronologies.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::ExampleChronologyMissing);
        }
        if row
            .example_chronologies
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ChronologyGroupPrimitiveViolation::ExampleChronologyDrift);
        }
        if !row.examples_match_lane() {
            violations.push(M5ChronologyGroupPrimitiveViolation::ExampleLaneMismatch);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ChronologyGroupPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ChronologyGroupPrimitiveViolation::SurfaceInvariantViolated);
        }
    }
}

/// AC1: every history lane must be proven — with a worked resolution — to render at
/// least one timeline group and an export preview from the shared model.
fn validate_grouped_chronology_covered(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let proven: BTreeSet<M5ChronologyHistoryLane> = packet
        .surface_rows
        .iter()
        .filter(|row| {
            row.example_chronologies.iter().any(|case| {
                !case.resolved.groups.is_empty()
                    && !case.resolved.export_preview.included_fields.is_empty()
            })
        })
        .map(|row| row.history_lane)
        .collect();
    if !M5ChronologyHistoryLane::ALL
        .iter()
        .all(|lane| proven.contains(lane))
    {
        violations.push(M5ChronologyGroupPrimitiveViolation::GroupedChronologyUnproven);
    }
}

/// Every phase must be exercised by some worked resolution — the proof that grouped
/// phases work across lanes.
fn validate_phase_vocabulary_covered(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let present: BTreeSet<M5ChronologyPhase> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_chronologies.iter())
        .flat_map(|case| case.resolved.groups.iter())
        .map(|group| group.phase)
        .collect();
    if !M5ChronologyPhase::ALL
        .iter()
        .all(|phase| present.contains(phase))
    {
        violations.push(M5ChronologyGroupPrimitiveViolation::PhaseVocabularyUnproven);
    }
}

/// AC2: at least one worked resolution must carry both a relative scanning label
/// and an absolute timestamp on an event, and at least one export preview must
/// carry an absolute range — the proof that relative time stays scannable while
/// absolute time survives in detail and export.
fn validate_time_parity_covered(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let event_parity = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_chronologies.iter())
        .flat_map(|case| case.resolved.groups.iter())
        .flat_map(|group| group.events.iter())
        .any(|event| {
            !event.relative_label.trim().is_empty() && !event.absolute_timestamp.trim().is_empty()
        });
    let export_absolute = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_chronologies.iter())
        .any(|case| {
            !case
                .resolved
                .export_preview
                .selected_range_start
                .trim()
                .is_empty()
                && !case
                    .resolved
                    .export_preview
                    .selected_range_end
                    .trim()
                    .is_empty()
        });
    if !(event_parity && export_absolute) {
        violations.push(M5ChronologyGroupPrimitiveViolation::TimeParityUnproven);
    }
}

/// AC3: at least one worked resolution with two or more events must produce an
/// export preview that preserves causal order and declares a redaction class — the
/// proof that export preserves causality and redaction intent rather than
/// flattening history.
fn validate_causality_preservation_covered(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let proven = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_chronologies.iter())
        .any(|case| {
            case.resolved.total_event_count >= 2
                && case.resolved.preserves_causal_order
                && case.resolved.export_preview.preserves_causal_order
                && case.resolved.export_preview.event_order.len() == case.resolved.total_event_count
        });
    if !proven {
        violations.push(M5ChronologyGroupPrimitiveViolation::CausalityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_chronology_model_across_lanes,
        review.timeline_groups_retain_ordering,
        review.groups_declare_phase_count_and_outcome,
        review.narrative_explains_state_and_next_action,
        review.relative_and_absolute_time_parity,
        review.export_preview_declares_full_disclosure,
        review.export_preserves_causality,
        review.support_export_keeps_chronology_vocabulary,
        review.every_surface_bound_to_shell_zone,
        review.later_lanes_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ChronologyGroupPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.history_lanes_consume_shared_chronology,
        projection.grouping_reads_single_phase_vocabulary,
        projection.narrative_reads_single_source,
        projection.export_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ChronologyGroupPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ChronologyGroupPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ChronologyGroupPrimitivePacket,
    violations: &mut Vec<M5ChronologyGroupPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.chronology_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ChronologyGroupPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
