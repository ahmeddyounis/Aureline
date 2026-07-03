//! One reusable M5 evidence / activity timeline row primitive: stable verbs,
//! provenance badges, disclosure-ready detail links, and text / JSON / Markdown
//! copy parity across every M5 history that explains what happened.
//!
//! Aureline's frozen component matrix
//! ([`crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix`])
//! names the event / history row (and its timeline group and chronology export) as
//! one governed component family and freezes its controlled verb vocabulary,
//! provenance badges, chronology detail states, and export fields. This module
//! *implements* that evidence-row contract as one reusable primitive so timestamp,
//! actor, action, object / scope, outcome, expandable detail, and provenance stay
//! consistent — and portable as copyable text, JSON, and Markdown — instead of
//! drifting into per-feature prose rows that only a screenshot can preserve.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_evidence_row`] — that takes one history lane's raw
//!    events (each carrying a timestamp, actor, stable verb, object / scope,
//!    outcome, provenance badge, and optional detail link) and produces one
//!    [`M5ResolvedEvidenceLog`] carrying the resolved detail state per event and,
//!    where the lane already claims portable evidence, the three copy renderings
//!    (text, JSON, and Markdown) of every row. The resolver never invents a local
//!    prose verb, never drops the provenance badge, and never lets a detail link
//!    point at nothing.
//! 2. A parity matrix — [`M5EvidenceRowPrimitivePacket`] — that binds one row per
//!    claimed M5 history lane (AI evidence, task events, policy changes, provider
//!    mutations, remote reconnects, update history, support exports, and repair
//!    flows) to the shared row anatomy, the same stable verb vocabulary and
//!    provenance badges, the same chronology detail states, the same copy formats,
//!    and the same export fields, so the support / export packet reconstructs what
//!    happened from one shared model on every lane.
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
//! about the row itself: its history-lane families, its anatomy parts, its copy
//! formats, and its focus behaviors. No M5 surface invents a second row grammar or
//! a second verb vocabulary.
//!
//! Raw URLs, raw local paths, raw usernames, raw hostnames, tokens, credentials,
//! and user text bodies stay outside the support boundary; opaque, export-safe
//! reprs are the only material carried.
//!
//! The boundary schema is
//! [`schemas/ui/m5-evidence-row.schema.json`](../../../../schemas/ui/m5-evidence-row.schema.json)
//! and the contract doc is
//! [`docs/components/m5_evidence_row_primitive_contract.md`](../../../../docs/components/m5_evidence_row_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-evidence-row-primitive/`](../../../../fixtures/ui/m5-evidence-row-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_evidence_row_primitive_packet,
    seeded_m5_evidence_row_primitive_repair_flows_preview_narrowed,
    seeded_m5_evidence_row_primitive_update_history_beta_narrowed,
    M5_EVIDENCE_ROW_PRIMITIVE_PACKET_ID,
};

// The stable chronology verbs, provenance badges, chronology detail states,
// chronology export fields, accessibility routes, qualification classes, and
// downgrade triggers are frozen once, in the trust-chronology component matrix.
// This primitive reuses them verbatim so it never invents a parallel verb
// vocabulary or a second row grammar.
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

/// Stable record-kind tag carried by [`M5EvidenceRowPrimitivePacket`].
pub const M5_EVIDENCE_ROW_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_evidence_and_activity_timeline_row_stable_verbs_provenance_and_copy_parity_primitive";

/// Schema version for M5 evidence-row-primitive records.
pub const M5_EVIDENCE_ROW_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the evidence-row-primitive boundary schema.
pub const M5_EVIDENCE_ROW_SCHEMA_REF: &str = "schemas/ui/m5-evidence-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EVIDENCE_ROW_DOC_REF: &str = "docs/components/m5_evidence_row_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds
/// against.
pub const M5_EVIDENCE_ROW_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen component matrix this primitive narrows from.
pub const M5_EVIDENCE_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-trust-chronology-components.schema.json";

/// Repo-relative path of the activity-row contract this primitive projects from.
pub const M5_EVIDENCE_ROW_ACTIVITY_ROW_REF: &str = "schemas/events/activity_row.schema.json";

/// Repo-relative path of the task-event contract this primitive consumes.
pub const M5_EVIDENCE_ROW_TASK_EVENT_REF: &str = "schemas/execution/task_event.schema.json";

/// Repo-relative path of the event-provenance-row contract this primitive
/// consumes.
pub const M5_EVIDENCE_ROW_PROVENANCE_REF: &str = "schemas/ops/event_provenance_row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_EVIDENCE_ROW_FIXTURE_DIR: &str = "fixtures/ui/m5-evidence-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EVIDENCE_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-evidence-row-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_EVIDENCE_ROW_CSV_REF: &str = "artifacts/release/m5-evidence-row-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_EVIDENCE_ROW_REPORT_REF: &str = "artifacts/components/m5-evidence-row-primitive.md";

/// The chronology export fields every evidence-row export must carry so support /
/// export never drops a truth-bearing column and can reconstruct what happened
/// without a screenshot. `RedactionClass` is carried too but is not part of this
/// mandatory-truth core.
pub const MANDATORY_EXPORT_FIELDS: [M5ChronologyExportField; 6] = [
    M5ChronologyExportField::EventVerb,
    M5ChronologyExportField::Provenance,
    M5ChronologyExportField::Timestamp,
    M5ChronologyExportField::ObjectRef,
    M5ChronologyExportField::ActorRole,
    M5ChronologyExportField::OutcomeCode,
];

/// One claimed M5 history lane that renders the shared evidence / activity row.
/// These are the histories the goal names — anywhere Aureline explains what
/// happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistorySurfaceFamily {
    /// AI evidence: what an AI run read, ran, and produced.
    AiEvidence,
    /// Task events: the task / job lifecycle.
    TaskEvents,
    /// Policy changes: admin / policy approvals and denials.
    PolicyChanges,
    /// Provider mutations: connected-provider state changes.
    ProviderMutations,
    /// Remote reconnects: remote-host connection recovery history.
    RemoteReconnects,
    /// Update history: application update / channel history.
    UpdateHistory,
    /// Support exports: what a support / export flow captured.
    SupportExports,
    /// Repair flows: recovery / repair drill history.
    RepairFlows,
}

impl M5HistorySurfaceFamily {
    /// Every claimed history lane, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AiEvidence,
        Self::TaskEvents,
        Self::PolicyChanges,
        Self::ProviderMutations,
        Self::RemoteReconnects,
        Self::UpdateHistory,
        Self::SupportExports,
        Self::RepairFlows,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiEvidence => "ai_evidence",
            Self::TaskEvents => "task_events",
            Self::PolicyChanges => "policy_changes",
            Self::ProviderMutations => "provider_mutations",
            Self::RemoteReconnects => "remote_reconnects",
            Self::UpdateHistory => "update_history",
            Self::SupportExports => "support_exports",
            Self::RepairFlows => "repair_flows",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AiEvidence => "AI Evidence",
            Self::TaskEvents => "Task Events",
            Self::PolicyChanges => "Policy Changes",
            Self::ProviderMutations => "Provider Mutations",
            Self::RemoteReconnects => "Remote Reconnects",
            Self::UpdateHistory => "Update History",
            Self::SupportExports => "Support Exports",
            Self::RepairFlows => "Repair Flows",
        }
    }
}

/// One anatomy part the shared evidence / activity row surfaces. The first six in
/// [`M5EvidenceRowAnatomyPart::MANDATORY`] are required on every row; the last is
/// the conditional expandable-detail link that appears whenever an event has
/// disclosure-ready detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceRowAnatomyPart {
    /// When the event happened.
    Timestamp,
    /// Who / what the event attributes the action to.
    Actor,
    /// The stable verb describing what happened.
    Action,
    /// The object or scope the action touched.
    ObjectOrScope,
    /// The controlled outcome of the action.
    Outcome,
    /// The provenance badge attributing initiation.
    ProvenanceBadge,
    /// The expandable detail link into the full disclosure / durable history.
    DetailLink,
}

impl M5EvidenceRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Timestamp,
        Self::Actor,
        Self::Action,
        Self::ObjectOrScope,
        Self::Outcome,
        Self::ProvenanceBadge,
        Self::DetailLink,
    ];

    /// The anatomy parts every evidence row must render.
    pub const MANDATORY: [Self; 6] = [
        Self::Timestamp,
        Self::Actor,
        Self::Action,
        Self::ObjectOrScope,
        Self::Outcome,
        Self::ProvenanceBadge,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Timestamp => "timestamp",
            Self::Actor => "actor",
            Self::Action => "action",
            Self::ObjectOrScope => "object_or_scope",
            Self::Outcome => "outcome",
            Self::ProvenanceBadge => "provenance_badge",
            Self::DetailLink => "detail_link",
        }
    }
}

/// A copy / export format the shared row renders so support / export preserves what
/// happened without a screenshot. Every lane that claims portable evidence must
/// offer all three.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceCopyFormat {
    /// Plain-text single-line row.
    Text,
    /// Machine-readable JSON object.
    Json,
    /// Markdown list item.
    Markdown,
}

impl M5EvidenceCopyFormat {
    /// Every copy format, in declaration order.
    pub const ALL: [Self; 3] = [Self::Text, Self::Json, Self::Markdown];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
            Self::Markdown => "markdown",
        }
    }
}

/// A focus / navigation behavior the evidence row supports so terse rows stay
/// scannable while detail and copy stay keyboard-reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceRowFocusBehavior {
    /// The row is reachable and operable by keyboard focus.
    RowKeyboardFocusable,
    /// The detail link is reachable and expands without pointer hover.
    DetailExpandReachable,
    /// Focus returns to the row after an expanded detail collapses.
    ReturnFocusOnCollapse,
    /// The copy action is keyboard-reachable.
    CopyActionReachable,
    /// Keyboard navigation moves per row.
    PerRowNavigation,
    /// A stable deep-link anchor jumps to the durable history.
    DeepLinkToDurableHistory,
}

impl M5EvidenceRowFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RowKeyboardFocusable,
        Self::DetailExpandReachable,
        Self::ReturnFocusOnCollapse,
        Self::CopyActionReachable,
        Self::PerRowNavigation,
        Self::DeepLinkToDurableHistory,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RowKeyboardFocusable => "row_keyboard_focusable",
            Self::DetailExpandReachable => "detail_expand_reachable",
            Self::ReturnFocusOnCollapse => "return_focus_on_collapse",
            Self::CopyActionReachable => "copy_action_reachable",
            Self::PerRowNavigation => "per_row_navigation",
            Self::DeepLinkToDurableHistory => "deep_link_to_durable_history",
        }
    }
}

/// The controlled outcome an event resolves to. Kept orthogonal to the stable verb
/// (the verb is *what happened*, the outcome is *how it ended*). This is a
/// resolver-side vocabulary and is not part of the frozen component-matrix set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceOutcome {
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

impl M5EvidenceOutcome {
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

/// One raw history event, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceEventItem {
    /// Opaque, export-safe timestamp representation (RFC 3339, never local wall
    /// text).
    pub timestamp_repr: String,
    /// Opaque, export-safe actor representation (never a raw username or path).
    pub actor_repr: String,
    /// The stable verb describing what happened.
    pub verb: M5ChronologyVerb,
    /// Opaque, export-safe object / scope representation the action touched.
    pub object_repr: String,
    /// The controlled outcome of the event.
    pub outcome: M5EvidenceOutcome,
    /// The provenance badge attributing initiation.
    pub provenance: M5ProvenanceBadge,
    /// True when this event has disclosure-ready expandable detail.
    pub has_detail: bool,
    /// Opaque, export-safe detail anchor. Required when `has_detail` is true and
    /// must be absent otherwise.
    pub detail_ref: Option<String>,
}

impl M5EvidenceEventItem {
    /// True when any representation carries forbidden material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.timestamp_repr)
            || repr_is_forbidden(&self.actor_repr)
            || repr_is_forbidden(&self.object_repr)
            || self.detail_ref.as_deref().is_some_and(repr_is_forbidden)
    }
}

/// The full input to the evidence-row resolver for one history lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceRowResolutionInput {
    /// The history lane these events render on.
    pub surface_family: M5HistorySurfaceFamily,
    /// True when this lane already claims portable evidence and therefore produces
    /// copyable text / JSON / Markdown renderings.
    pub portable_evidence: bool,
    /// The raw events, in chronological order. Must be non-empty.
    pub events: Vec<M5EvidenceEventItem>,
}

/// The three portable copy renderings of one resolved row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceRowCopy {
    /// Plain-text single-line rendering.
    pub text: String,
    /// Machine-readable JSON-object rendering.
    pub json: String,
    /// Markdown list-item rendering.
    pub markdown: String,
}

/// The resolved posture of one history event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEvidenceEvent {
    /// The opaque timestamp representation.
    pub timestamp_repr: String,
    /// The opaque actor representation.
    pub actor_repr: String,
    /// The stable verb.
    pub verb: M5ChronologyVerb,
    /// The opaque object / scope representation.
    pub object_repr: String,
    /// The controlled outcome.
    pub outcome: M5EvidenceOutcome,
    /// The provenance badge.
    pub provenance: M5ProvenanceBadge,
    /// The resolved chronology detail state.
    pub detail_state: M5ChronologyDetailState,
    /// The opaque detail anchor, when the event has expandable detail.
    pub detail_ref: Option<String>,
    /// The three portable copy renderings, when the lane claims portable evidence.
    pub copy: Option<M5EvidenceRowCopy>,
}

/// The resolved evidence log for one history lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEvidenceLog {
    /// The history lane these events render on.
    pub surface_family: M5HistorySurfaceFamily,
    /// Whether this lane claims portable evidence.
    pub portable_evidence: bool,
    /// The resolved events, in chronological order.
    pub resolved_events: Vec<M5ResolvedEvidenceEvent>,
    /// True when every resolved event carries the three portable copy renderings.
    pub emits_portable_copy: bool,
}

/// Errors returned by [`resolve_evidence_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5EvidenceResolutionError {
    /// The input carried no events.
    NoEvents,
    /// An event had an empty timestamp.
    EmptyTimestamp,
    /// An event had an empty actor.
    EmptyActor,
    /// An event had an empty object / scope.
    EmptyObject,
    /// An event claimed expandable detail but named no detail anchor.
    MissingDetailRef,
    /// An event named a detail anchor but did not claim expandable detail.
    UnexpectedDetailRef,
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5EvidenceResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoEvents => "no_events",
            Self::EmptyTimestamp => "empty_timestamp",
            Self::EmptyActor => "empty_actor",
            Self::EmptyObject => "empty_object",
            Self::MissingDetailRef => "missing_detail_ref",
            Self::UnexpectedDetailRef => "unexpected_detail_ref",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5EvidenceResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "evidence-row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5EvidenceResolutionError {}

/// Compact serializable projection used to render one row's JSON copy. Field order
/// here is the stable JSON key order.
#[derive(Serialize)]
struct EvidenceCopyJson<'a> {
    timestamp: &'a str,
    actor: &'a str,
    verb: &'a str,
    object: &'a str,
    outcome: &'a str,
    provenance: &'a str,
    has_detail: bool,
}

/// Resolves one history lane's events into one evidence log.
///
/// Each event resolves to exactly one [`M5ChronologyDetailState`]: an event with
/// expandable detail reads as reopenable-from-history, an event without reads as
/// collapsed. Where the lane claims portable evidence, every resolved row also
/// carries the three copy renderings — text, JSON, and Markdown — built from the
/// same stable verb vocabulary so support / export never needs a screenshot.
pub fn resolve_evidence_row(
    input: &M5EvidenceRowResolutionInput,
) -> Result<M5ResolvedEvidenceLog, M5EvidenceResolutionError> {
    if input.events.is_empty() {
        return Err(M5EvidenceResolutionError::NoEvents);
    }

    for event in &input.events {
        if event.timestamp_repr.trim().is_empty() {
            return Err(M5EvidenceResolutionError::EmptyTimestamp);
        }
        if event.actor_repr.trim().is_empty() {
            return Err(M5EvidenceResolutionError::EmptyActor);
        }
        if event.object_repr.trim().is_empty() {
            return Err(M5EvidenceResolutionError::EmptyObject);
        }
        if event.carries_forbidden_material() {
            return Err(M5EvidenceResolutionError::ForbiddenMaterial);
        }
        let has_ref = event
            .detail_ref
            .as_deref()
            .map(str::trim)
            .is_some_and(|value| !value.is_empty());
        if event.has_detail && !has_ref {
            return Err(M5EvidenceResolutionError::MissingDetailRef);
        }
        if !event.has_detail && event.detail_ref.is_some() {
            return Err(M5EvidenceResolutionError::UnexpectedDetailRef);
        }
    }

    let resolved_events: Vec<M5ResolvedEvidenceEvent> = input
        .events
        .iter()
        .map(|event| resolve_event(event, input.portable_evidence))
        .collect();

    let emits_portable_copy =
        input.portable_evidence && resolved_events.iter().all(|event| event.copy.is_some());

    Ok(M5ResolvedEvidenceLog {
        surface_family: input.surface_family,
        portable_evidence: input.portable_evidence,
        resolved_events,
        emits_portable_copy,
    })
}

/// Resolves one event's detail state and portable copy renderings.
fn resolve_event(event: &M5EvidenceEventItem, portable: bool) -> M5ResolvedEvidenceEvent {
    // An event with disclosure-ready detail is always reopenable from durable
    // history; an event without detail reads as a collapsed terse summary row.
    let detail_state = if event.has_detail {
        M5ChronologyDetailState::ReopenableDetail
    } else {
        M5ChronologyDetailState::Collapsed
    };

    let copy = if portable {
        Some(render_copy(event))
    } else {
        None
    };

    M5ResolvedEvidenceEvent {
        timestamp_repr: event.timestamp_repr.clone(),
        actor_repr: event.actor_repr.clone(),
        verb: event.verb,
        object_repr: event.object_repr.clone(),
        outcome: event.outcome,
        provenance: event.provenance,
        detail_state,
        detail_ref: event.detail_ref.clone(),
        copy,
    }
}

/// Renders the three portable copy forms of one row from the shared stable
/// vocabulary. All three forms carry the same seven truth columns.
fn render_copy(event: &M5EvidenceEventItem) -> M5EvidenceRowCopy {
    let text = format!(
        "{} · {} · {} · {} · {} · {}",
        event.timestamp_repr,
        event.actor_repr,
        event.verb.as_str(),
        event.object_repr,
        event.outcome.as_str(),
        event.provenance.as_str(),
    );

    let json = serde_json::to_string(&EvidenceCopyJson {
        timestamp: &event.timestamp_repr,
        actor: &event.actor_repr,
        verb: event.verb.as_str(),
        object: &event.object_repr,
        outcome: event.outcome.as_str(),
        provenance: event.provenance.as_str(),
        has_detail: event.has_detail,
    })
    .expect("evidence copy json serializes");

    let markdown = format!(
        "- `{}` **{}** {} — {} (actor: {}, provenance: {})",
        event.timestamp_repr,
        event.verb.as_str(),
        event.object_repr,
        event.outcome.as_str(),
        event.actor_repr,
        event.provenance.as_str(),
    );

    M5EvidenceRowCopy {
        text,
        json,
        markdown,
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs history truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceRowResolutionCase {
    /// The resolver input.
    pub input: M5EvidenceRowResolutionInput,
    /// The resolved evidence log. Must equal `resolve_evidence_row(&input)`.
    pub resolved: M5ResolvedEvidenceLog,
}

impl M5EvidenceRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5EvidenceRowResolutionInput) -> Self {
        let resolved = resolve_evidence_row(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_evidence_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one history lane bound to the shared
/// evidence-row anatomy, stable verbs, provenance badges, chronology detail
/// states, copy formats, and export fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceSurfaceRow {
    /// History-lane family.
    pub surface_family: M5HistorySurfaceFamily,
    /// Qualification class earned by this lane.
    pub qualification: M5TrustQualificationClass,
    /// Owner role accountable for keeping this lane governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this history attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this row must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this row keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Anatomy parts this row renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5EvidenceRowAnatomyPart>,
    /// Stable verbs this lane can show.
    pub chronology_verbs: Vec<M5ChronologyVerb>,
    /// Provenance badges this lane can attribute.
    pub provenance_badges: Vec<M5ProvenanceBadge>,
    /// Chronology detail states this lane projects.
    pub detail_states: Vec<M5ChronologyDetailState>,
    /// True when this lane already claims portable evidence.
    pub portable_evidence: bool,
    /// Copy formats this lane offers (all three when portable, none otherwise).
    pub copy_formats: Vec<M5EvidenceCopyFormat>,
    /// Focus behaviors this lane supports.
    pub focus_behaviors: Vec<M5EvidenceRowFocusBehavior>,
    /// Export fields this lane carries (must include the mandatory truth fields).
    pub export_fields: Vec<M5ChronologyExportField>,
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
    pub example_logs: Vec<M5EvidenceRowResolutionCase>,
    /// Hard invariant: this lane never drifts from the stable verb vocabulary. MUST
    /// be `false`.
    pub drifts_from_verb_vocabulary: bool,
    /// Hard invariant: this lane never drops the provenance badge. MUST be `false`.
    pub drops_provenance_badge: bool,
    /// Hard invariant: this lane never leaves detail un-reopenable from history.
    /// MUST be `false`.
    pub detail_not_reopenable: bool,
    /// Hard invariant: this lane never drops export / audit truth. MUST be `false`.
    pub drops_export_or_audit_truth: bool,
}

impl M5EvidenceSurfaceRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5EvidenceRowAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5EvidenceRowAnatomyPart::MANDATORY
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

    /// True when the row's declared copy formats match its portability: a portable
    /// lane offers all three formats, a non-portable lane offers none.
    fn declares_consistent_copy_formats(&self) -> bool {
        let present: BTreeSet<M5EvidenceCopyFormat> = self.copy_formats.iter().copied().collect();
        if self.portable_evidence {
            M5EvidenceCopyFormat::ALL
                .iter()
                .all(|format| present.contains(format))
        } else {
            present.is_empty()
        }
    }

    /// True when every worked case's declared portability matches the row's.
    fn examples_match_portability(&self) -> bool {
        self.example_logs
            .iter()
            .all(|case| case.input.portable_evidence == self.portable_evidence)
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.drifts_from_verb_vocabulary
            && !self.drops_provenance_badge
            && !self.detail_not_reopenable
            && !self.drops_export_or_audit_truth
    }
}

/// Self-describing controlled-vocabulary set minted by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceRowVocabularySet {
    /// History-lane-family tokens.
    pub history_families: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Stable verb tokens (reused from the frozen matrix).
    pub chronology_verbs: Vec<String>,
    /// Provenance-badge tokens (reused from the frozen matrix).
    pub provenance_badges: Vec<String>,
    /// Chronology-detail-state tokens (reused from the frozen matrix).
    pub detail_states: Vec<String>,
    /// Copy-format tokens.
    pub copy_formats: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Chronology-export-field tokens (reused from the frozen matrix).
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5EvidenceRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            history_families: tokens(&M5HistorySurfaceFamily::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5EvidenceRowAnatomyPart::ALL, |v| v.as_str()),
            chronology_verbs: tokens(&M5ChronologyVerb::ALL, |v| v.as_str()),
            provenance_badges: tokens(&M5ProvenanceBadge::ALL, |v| v.as_str()),
            detail_states: tokens(&M5ChronologyDetailState::ALL, |v| v.as_str()),
            copy_formats: tokens(&M5EvidenceCopyFormat::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5EvidenceRowFocusBehavior::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ChronologyExportField::ALL, |v| v.as_str()),
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
pub struct M5EvidenceRowGovernanceReview {
    /// One row model is reused across every history lane.
    pub one_row_model_across_history_lanes: bool,
    /// The stable verb vocabulary is enforced everywhere.
    pub stable_verb_vocabulary_enforced: bool,
    /// A provenance badge is always attributed on every row.
    pub provenance_badge_always_attributed: bool,
    /// Detail is always reopenable from durable history.
    pub detail_reopenable_from_durable_history: bool,
    /// Copy / export parity holds across text, JSON, and Markdown.
    pub copy_export_parity_text_json_markdown: bool,
    /// The support / export packet keeps the same chronology vocabulary.
    pub support_export_keeps_chronology_vocabulary: bool,
    /// No lane invents local prose verbs.
    pub no_lane_invents_local_prose_verbs: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 lanes cannot invent parallel chronology vocabulary.
    pub later_lanes_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceRowConsumerProjection {
    /// AI / task / policy / provider / remote / update / support / repair lanes all
    /// consume the shared row.
    pub history_lanes_consume_shared_row: bool,
    /// The resolver reads a single canonical verb vocabulary.
    pub resolver_reads_single_verb_vocabulary: bool,
    /// Provenance attribution reads a single canonical source.
    pub provenance_reads_single_source: bool,
    /// Detail reopen reads a single canonical durable-history source.
    pub detail_reopen_reads_single_source: bool,
    /// Support / export reads a single canonical evidence-row source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the evidence-row primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceRowReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting evidence-row audit.
    pub evidence_row_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5EvidenceRowPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EvidenceRowPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5EvidenceSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EvidenceRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EvidenceRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EvidenceRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EvidenceRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EvidenceRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 evidence-row-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceRowPrimitivePacket {
    /// Record kind; must equal [`M5_EVIDENCE_ROW_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EVIDENCE_ROW_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5EvidenceSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EvidenceRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EvidenceRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EvidenceRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EvidenceRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EvidenceRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EvidenceRowPrimitivePacket {
    /// Builds an M5 evidence-row-primitive packet from stable-lane input.
    pub fn new(input: M5EvidenceRowPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_EVIDENCE_ROW_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_EVIDENCE_ROW_PRIMITIVE_SCHEMA_VERSION,
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

    /// Validates the M5 evidence-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5EvidenceRowPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EVIDENCE_ROW_PRIMITIVE_RECORD_KIND {
            violations.push(M5EvidenceRowPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EVIDENCE_ROW_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5EvidenceRowPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EvidenceRowPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_verb_vocabulary_covered(self, &mut violations);
        validate_provenance_coverage(self, &mut violations);
        validate_portable_copy_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 evidence-row primitive packet serializes"),
        ) {
            violations.push(M5EvidenceRowPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 evidence-row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per history lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,qualification,owner,shell_zone_slot,portable,anatomy_parts,verbs,provenance_badges,copy_formats,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                row.portable_evidence,
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.chronology_verbs, |v| v.as_str()),
                join_tokens(&row.provenance_badges, |v| v.as_str()),
                join_tokens(&row.copy_formats, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_logs.len(),
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
            "# M5 Evidence / Activity Row Primitive: Stable Verbs, Provenance, and Copy Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- History lanes: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Anatomy parts: {}\n",
            self.vocabulary_set.anatomy_parts.join(", ")
        ));
        out.push_str(&format!(
            "- Stable verbs: {}\n",
            self.vocabulary_set.chronology_verbs.join(", ")
        ));
        out.push_str(&format!(
            "- Provenance badges: {}\n",
            self.vocabulary_set.provenance_badges.join(", ")
        ));
        out.push_str(&format!(
            "- Copy formats: {}\n",
            self.vocabulary_set.copy_formats.join(", ")
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
                row.surface_family.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Portable evidence: {}\n",
                row.portable_evidence
            ));
            out.push_str(&format!("  - Worked logs: {}\n", row.example_logs.len()));
            for case in &row.example_logs {
                out.push_str(&format!(
                    "    - {} event(s){}\n",
                    case.resolved.resolved_events.len(),
                    if case.resolved.emits_portable_copy {
                        ", copyable as text / JSON / Markdown"
                    } else {
                        ""
                    }
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 evidence-row-primitive export.
#[derive(Debug)]
pub enum M5EvidenceRowPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EvidenceRowPrimitiveViolation>),
}

impl fmt::Display for M5EvidenceRowPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 evidence-row primitive export parse failed: {error}"
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
                    "m5 evidence-row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EvidenceRowPrimitiveArtifactError {}

/// Validation failures emitted by [`M5EvidenceRowPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EvidenceRowPrimitiveViolation {
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
    /// A required history-lane family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A surface row declares no stable verbs.
    ChronologyVerbMissing,
    /// A surface row declares no provenance badges.
    ProvenanceBadgeMissing,
    /// A surface row declares no chronology detail states.
    DetailStateMissing,
    /// A surface row's copy formats do not match its portability.
    CopyFormatParityMismatch,
    /// A surface row declares no focus behaviors.
    FocusBehaviorMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no worked resolution cases.
    ExampleLogMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleLogDrift,
    /// A worked resolution case's portability disagrees with its row.
    ExamplePortabilityMismatch,
    /// A lane claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No worked resolution across the matrix exercises the full stable verb
    /// vocabulary.
    VerbVocabularyUnproven,
    /// No worked resolution across the matrix exercises every provenance badge.
    ProvenanceCoverageUnproven,
    /// No worked resolution across the matrix proves a row copyable as text, JSON,
    /// and Markdown.
    PortableCopyUnproven,
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

impl M5EvidenceRowPrimitiveViolation {
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
            Self::ChronologyVerbMissing => "chronology_verb_missing",
            Self::ProvenanceBadgeMissing => "provenance_badge_missing",
            Self::DetailStateMissing => "detail_state_missing",
            Self::CopyFormatParityMismatch => "copy_format_parity_mismatch",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleLogMissing => "example_log_missing",
            Self::ExampleLogDrift => "example_log_drift",
            Self::ExamplePortabilityMismatch => "example_portability_mismatch",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::VerbVocabularyUnproven => "verb_vocabulary_unproven",
            Self::ProvenanceCoverageUnproven => "provenance_coverage_unproven",
            Self::PortableCopyUnproven => "portable_copy_unproven",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 evidence-row-primitive export.
pub fn current_stable_m5_evidence_row_primitive_export(
) -> Result<M5EvidenceRowPrimitivePacket, M5EvidenceRowPrimitiveArtifactError> {
    let packet: M5EvidenceRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-evidence-row-proof/support_export.json"
    )))
    .map_err(M5EvidenceRowPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EvidenceRowPrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EVIDENCE_ROW_SCHEMA_REF,
        M5_EVIDENCE_ROW_DOC_REF,
        M5_EVIDENCE_ROW_SHELL_ZONE_REF,
        M5_EVIDENCE_ROW_COMPONENT_MATRIX_REF,
        M5_EVIDENCE_ROW_ACTIVITY_ROW_REF,
        M5_EVIDENCE_ROW_TASK_EVENT_REF,
        M5_EVIDENCE_ROW_PROVENANCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EvidenceRowPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5EvidenceRowPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5HistorySurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5HistorySurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5EvidenceRowPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5EvidenceRowPrimitiveViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5EvidenceRowPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.chronology_verbs.is_empty() {
            violations.push(M5EvidenceRowPrimitiveViolation::ChronologyVerbMissing);
        }
        if row.provenance_badges.is_empty() {
            violations.push(M5EvidenceRowPrimitiveViolation::ProvenanceBadgeMissing);
        }
        if row.detail_states.is_empty() {
            violations.push(M5EvidenceRowPrimitiveViolation::DetailStateMissing);
        }
        if !row.declares_consistent_copy_formats() {
            violations.push(M5EvidenceRowPrimitiveViolation::CopyFormatParityMismatch);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5EvidenceRowPrimitiveViolation::FocusBehaviorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5EvidenceRowPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TrustAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5EvidenceRowPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5EvidenceRowPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5EvidenceRowPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_logs.is_empty() {
            violations.push(M5EvidenceRowPrimitiveViolation::ExampleLogMissing);
        }
        if row
            .example_logs
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5EvidenceRowPrimitiveViolation::ExampleLogDrift);
        }
        if !row.examples_match_portability() {
            violations.push(M5EvidenceRowPrimitiveViolation::ExamplePortabilityMismatch);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5EvidenceRowPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5EvidenceRowPrimitiveViolation::SurfaceInvariantViolated);
        }
    }
}

/// Every stable verb must be exercised by some worked resolution — the
/// acceptance-criterion proof that history lanes reuse one stable verb vocabulary.
fn validate_verb_vocabulary_covered(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5ChronologyVerb> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_logs.iter())
        .flat_map(|case| case.resolved.resolved_events.iter())
        .map(|event| event.verb)
        .collect();
    if !M5ChronologyVerb::ALL
        .iter()
        .all(|verb| present.contains(verb))
    {
        violations.push(M5EvidenceRowPrimitiveViolation::VerbVocabularyUnproven);
    }
}

/// Every provenance badge must be attributed by some worked resolution — the
/// acceptance-criterion proof that provenance distinguishes user, extension,
/// policy, local system, remote host, managed service, and provider-owned state
/// where evidence exists.
fn validate_provenance_coverage(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5ProvenanceBadge> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_logs.iter())
        .flat_map(|case| case.resolved.resolved_events.iter())
        .map(|event| event.provenance)
        .collect();
    if !M5ProvenanceBadge::ALL
        .iter()
        .all(|badge| present.contains(badge))
    {
        violations.push(M5EvidenceRowPrimitiveViolation::ProvenanceCoverageUnproven);
    }
}

/// At least one worked resolution must produce a row copyable as text, JSON, and
/// Markdown — the acceptance-criterion proof that support / export no longer needs
/// a screenshot to preserve what happened.
fn validate_portable_copy_covered(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    let proven = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_logs.iter())
        .flat_map(|case| case.resolved.resolved_events.iter())
        .any(|event| {
            event.copy.as_ref().is_some_and(|copy| {
                !copy.text.trim().is_empty()
                    && !copy.json.trim().is_empty()
                    && !copy.markdown.trim().is_empty()
            })
        });
    if !proven {
        violations.push(M5EvidenceRowPrimitiveViolation::PortableCopyUnproven);
    }
}

fn validate_governance_review(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_row_model_across_history_lanes,
        review.stable_verb_vocabulary_enforced,
        review.provenance_badge_always_attributed,
        review.detail_reopenable_from_durable_history,
        review.copy_export_parity_text_json_markdown,
        review.support_export_keeps_chronology_vocabulary,
        review.no_lane_invents_local_prose_verbs,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_lanes_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5EvidenceRowPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.history_lanes_consume_shared_row,
        projection.resolver_reads_single_verb_vocabulary,
        projection.provenance_reads_single_source,
        projection.detail_reopen_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5EvidenceRowPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EvidenceRowPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EvidenceRowPrimitivePacket,
    violations: &mut Vec<M5EvidenceRowPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.evidence_row_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5EvidenceRowPrimitiveViolation::ReleasePostureIncomplete);
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
