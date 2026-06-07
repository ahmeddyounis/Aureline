//! Stable actor/action/object chronology grammar for history rows.
//!
//! Rows in this module are intentionally stricter than individual shell,
//! provider, AI, policy, or recovery surfaces. Each row carries the same
//! actor, action, object, outcome, provenance, absolute time posture, relative
//! age hint, follow-up state, and exact reopen target that exports and
//! companion clients must preserve.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for chronology history-row packets.
pub const CHRONOLOGY_HISTORY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path for the chronology history-row boundary schema.
pub const CHRONOLOGY_HISTORY_SCHEMA_REF: &str = "schemas/ux/chronology-history-row.schema.json";

/// Record kind for [`ChronologyHistoryPacket`].
pub const CHRONOLOGY_HISTORY_PACKET_RECORD_KIND: &str = "chronology_history_packet";

/// Record kind for [`ChronologyHistoryRow`].
pub const CHRONOLOGY_HISTORY_ROW_RECORD_KIND: &str = "chronology_history_row";

/// Record kind for [`ChronologyExportPacket`].
pub const CHRONOLOGY_EXPORT_PACKET_RECORD_KIND: &str = "chronology_export_packet";

/// Record kind for [`ChronologyExportRow`].
pub const CHRONOLOGY_EXPORT_ROW_RECORD_KIND: &str = "chronology_export_row";

/// Record kind for [`AccessibilityChronologyFixture`].
pub const ACCESSIBILITY_CHRONOLOGY_FIXTURE_RECORD_KIND: &str = "accessibility_chronology_fixture";

/// Stable source surface class for a chronology row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologySurfaceClass {
    /// Durable activity center and attention inbox.
    ActivityCenter,
    /// Task, test, or durable job history.
    DurableJob,
    /// Debug or replay chronology.
    DebugHistory,
    /// Imported or mirrored provider event.
    ProviderEvent,
    /// AI run, AI evidence, or AI replay row.
    AiRun,
    /// Policy or administrator-authored notice.
    PolicyAdminNotice,
    /// Recovery, restore, or crash-loop timeline.
    RecoveryTimeline,
    /// Support or incident export.
    SupportExport,
    /// Browser or mobile companion delivery.
    CompanionDelivery,
}

/// Actor that produced or owns the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorKind {
    /// Human operator on this install.
    UserActor,
    /// Product-owned background automation.
    SystemActor,
    /// AI agent, AI review, or AI apply flow.
    AiAgentActor,
    /// Hosted service, provider, mirror, or remote agent.
    RemoteServiceActor,
    /// Policy or administrator authority.
    AdminPolicyActor,
    /// Installed extension.
    ExtensionActor,
    /// Imported event with unresolved actor identity.
    UnknownActor,
}

/// Stable action verb used by chronology grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionVerb {
    Started,
    Progressed,
    Succeeded,
    Failed,
    Cancelled,
    Blocked,
    Unblocked,
    Held,
    Released,
    Granted,
    Narrowed,
    Widened,
    Revoked,
    Presented,
    Superseded,
    Proposed,
    Accepted,
    Rejected,
    Restored,
    Recovered,
    Published,
    Unpublished,
    Exported,
    Imported,
    Acknowledged,
    Resolved,
    Dismissed,
    Snoozed,
    Muted,
}

/// Object kind the chronology row is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyObjectKind {
    /// AI evidence or run-history row.
    AiEvidenceRow,
    /// Task/test/debug durable job row.
    TaskRunRow,
    /// Policy decision or administrator notice row.
    PolicyDecisionRow,
    /// Provider object, event, or grant row.
    ProviderEventRow,
    /// Recovery snapshot, restore, or crash-loop row.
    RecoverySnapshotRow,
    /// Support bundle or incident export row.
    SupportBundleRow,
    /// Workspace-local object row.
    WorkspaceObjectRow,
    /// Extension lifecycle row.
    ExtensionLifecycleRow,
}

/// Stable outcome class for a chronology row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryOutcomeClass {
    Pending,
    InProgress,
    Succeeded,
    Failed,
    Cancelled,
    Denied,
    Held,
    Superseded,
    Recovered,
    ObservedOnly,
    AwaitingApproval,
}

/// Current state of local follow-up for the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpState {
    /// Row remains visible but does not need a follow-up control.
    None,
    /// Row is visible and unread.
    Open,
    /// Badge is cleared, durable row remains.
    Acknowledged,
    /// Local follow-up has been completed.
    Resolved,
    /// Transient surface is hidden while durable history remains.
    Dismissed,
    /// Row is hidden until a declared wake time.
    Snoozed,
    /// Future similar alerts are suppressed by local preference or policy.
    Muted,
}

/// Local follow-up transition offered from the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpTransitionKind {
    Acknowledge,
    Resolve,
    Dismiss,
    Snooze,
    Mute,
    Reopen,
}

/// Effect a local chronology control has outside Aureline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalAuthorityEffectClass {
    /// Local-only state change.
    LocalOnly,
    /// Opens a separately reviewed provider command.
    RequiresReviewedProviderCommand,
}

/// Source/provenance class rendered as an export-safe badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologySourceClass {
    FirstPartyDirectObservation,
    FirstPartySynthesizedSummary,
    ProviderImported,
    ProviderCached,
    ProviderStaleCached,
    AiAssisted,
    AiGeneratedSummary,
    PolicyAuthored,
    RecoveryReconstructed,
    CompanionDelivered,
    ImportedExternalAuditTrail,
}

impl ChronologySourceClass {
    /// Returns true when the source marker is required in exports.
    pub const fn requires_export_marker(self) -> bool {
        matches!(
            self,
            Self::ProviderImported
                | Self::ProviderCached
                | Self::ProviderStaleCached
                | Self::AiAssisted
                | Self::AiGeneratedSummary
                | Self::PolicyAuthored
                | Self::RecoveryReconstructed
                | Self::CompanionDelivered
                | Self::ImportedExternalAuditTrail
        )
    }
}

/// Freshness posture rendered with relative age hints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyFreshnessClass {
    Current,
    Fresh,
    Cached,
    Stale,
    Expired,
    Unknown,
}

/// Live/imported class for the source event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyImportedClass {
    Live,
    Imported,
    Cached,
    StaleCached,
    Reconstructed,
}

/// Provenance badge preserved by UI, export, copy, and companion paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceBadge {
    /// Source class.
    pub source_class: ChronologySourceClass,
    /// Short visible badge label.
    pub badge_label: String,
    /// Export-safe marker that mirrors the badge.
    pub export_marker_label: String,
}

/// Relative-age hint rendered from an absolute timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelativeAgeHint {
    /// Reference time used to compute the label.
    pub rendered_at: String,
    /// Deterministic relative label such as `7 min ago`.
    pub relative_label: String,
    /// Freshness class for the relative label.
    pub freshness_class: ChronologyFreshnessClass,
    /// Why the row remains visible.
    pub visible_reason_label: String,
    /// Optional stale reason when freshness is stale or expired.
    pub stale_reason_label: Option<String>,
}

/// Time posture that preserves absolute and local context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimePosture {
    /// Absolute timestamp for export and ordering.
    pub absolute_timestamp: String,
    /// IANA time zone used by the source or display context.
    pub timezone_iana: String,
    /// UTC offset displayed with the row.
    pub utc_offset: String,
    /// Local display label for the event.
    pub local_time_label: String,
    /// Live/imported/cache posture for the timestamp.
    pub imported_class: ChronologyImportedClass,
    /// Deterministic relative-age hint.
    pub relative_age: RelativeAgeHint,
}

/// Exact target used to reopen the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenTarget {
    /// Stable target ref.
    pub target_ref: String,
    /// Target kind.
    pub target_kind: String,
    /// Command ref that opens the exact row or object.
    pub command_ref: String,
    /// Human-readable label for the target.
    pub label: String,
}

/// Local follow-up transition and authority boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpTransition {
    /// Transition kind.
    pub transition: FollowUpTransitionKind,
    /// Label shown in UI and export.
    pub label: String,
    /// Authority effect of invoking this control.
    pub local_authority_effect: LocalAuthorityEffectClass,
    /// Reviewed provider command ref required for provider mutation.
    pub reviewed_provider_command_ref: Option<String>,
}

/// Canonical chronology/history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyHistoryRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub item_id: String,
    /// Stable event id shared across surfaces.
    pub canonical_event_id: String,
    /// Surface class that produced the row.
    pub surface_class: ChronologySurfaceClass,
    /// Actor class.
    pub actor_kind: ActorKind,
    /// Stable actor identity ref when known.
    pub actor_ref: Option<String>,
    /// Privacy-safe actor label.
    pub actor_label: String,
    /// Stable action verb.
    pub action: ActionVerb,
    /// Privacy-safe object label.
    pub object_label: String,
    /// Object kind.
    pub object_kind: ChronologyObjectKind,
    /// Outcome class.
    pub outcome: HistoryOutcomeClass,
    /// Privacy-safe outcome label.
    pub outcome_label: String,
    /// Deterministic actor/action/object/outcome sentence.
    pub grammar_sentence: String,
    /// Source/provenance badges.
    pub provenance_badges: Vec<ProvenanceBadge>,
    /// Absolute and relative time posture.
    pub time_posture: TimePosture,
    /// Current follow-up state.
    pub follow_up_state: FollowUpState,
    /// Allowed local follow-up transitions.
    pub allowed_transitions: Vec<FollowUpTransition>,
    /// Exact reopen target.
    pub reopen_target: ReopenTarget,
    /// True when the backing object is provider-owned.
    pub provider_owned_object: bool,
}

impl ChronologyHistoryRow {
    /// Returns the deterministic actor/action/object/outcome sentence.
    pub fn canonical_sentence(&self) -> String {
        format!(
            "{} {} {}: {}",
            self.actor_label,
            action_label(self.action),
            self.object_label,
            self.outcome_label
        )
    }
}

/// Packet of canonical chronology rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyHistoryPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Schema ref.
    pub schema_ref: String,
    /// Rows in deterministic display order.
    pub rows: Vec<ChronologyHistoryRow>,
}

/// Export projection for a chronology row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyExportRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Source row id.
    pub item_id_ref: String,
    /// Canonical event id.
    pub canonical_event_id: String,
    /// Preserved actor/action/object/outcome sentence.
    pub grammar_sentence: String,
    /// Exported provenance markers.
    pub provenance_marker_labels: Vec<String>,
    /// Absolute timestamp.
    pub absolute_timestamp: String,
    /// Relative-age label.
    pub relative_age_label: String,
    /// Visible reason label.
    pub visible_reason_label: String,
    /// Current follow-up state.
    pub follow_up_state: FollowUpState,
    /// Exact reopen target.
    pub reopen_target: ReopenTarget,
}

/// Export packet that must preserve live-row chronology truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyExportPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export destination class.
    pub destination_class: String,
    /// Source row ids included in the export.
    pub source_item_ids: Vec<String>,
    /// Export rows.
    pub rows: Vec<ChronologyExportRow>,
}

/// Accessibility projection for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityHistoryRowProjection {
    /// Source row id.
    pub item_id_ref: String,
    /// Screen-reader label.
    pub screen_reader_label: String,
    /// Keyboard command refs that can reach row actions.
    pub keyboard_command_refs: Vec<String>,
    /// Reduced-motion behavior.
    pub reduced_motion_behavior: String,
    /// True when state can be understood without color.
    pub distinguishes_without_color: bool,
    /// True when state can be understood without hover.
    pub distinguishes_without_hover: bool,
}

/// Accessibility fixture for chronology rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityChronologyFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Accessibility projections.
    pub rows: Vec<AccessibilityHistoryRowProjection>,
}

/// Validation report for chronology packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyRowAuditReport {
    /// True when no findings were emitted.
    pub passed: bool,
    /// Validation findings.
    pub findings: Vec<ChronologyRowFinding>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyRowFinding {
    /// Row or packet id.
    pub item_id: String,
    /// Stable finding code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
}

/// Validates a chronology packet.
pub fn validate_chronology_packet(packet: &ChronologyHistoryPacket) -> ChronologyRowAuditReport {
    let mut findings = Vec::new();
    if packet.record_kind != CHRONOLOGY_HISTORY_PACKET_RECORD_KIND {
        findings.push(finding(
            "packet",
            "wrong_packet_record_kind",
            "chronology packet record kind must be stable",
        ));
    }
    if packet.schema_version != CHRONOLOGY_HISTORY_SCHEMA_VERSION {
        findings.push(finding(
            "packet",
            "wrong_schema_version",
            "chronology packet schema version must match the canonical version",
        ));
    }

    let mut seen_ids = BTreeSet::new();
    let mut seen_surfaces = BTreeSet::new();
    for row in &packet.rows {
        seen_surfaces.insert(row.surface_class);
        validate_row(row, &mut findings);
        if !seen_ids.insert(row.item_id.clone()) {
            findings.push(finding(
                &row.item_id,
                "duplicate_item_id",
                "row id must be unique",
            ));
        }
    }

    for required in [
        ChronologySurfaceClass::ActivityCenter,
        ChronologySurfaceClass::DurableJob,
        ChronologySurfaceClass::DebugHistory,
        ChronologySurfaceClass::ProviderEvent,
        ChronologySurfaceClass::AiRun,
        ChronologySurfaceClass::PolicyAdminNotice,
        ChronologySurfaceClass::RecoveryTimeline,
    ] {
        if !seen_surfaces.contains(&required) {
            findings.push(finding(
                "packet",
                "missing_required_surface_class",
                "stable chronology packet must cover activity, task/test, debug, provider, AI, policy, and recovery rows",
            ));
        }
    }

    report(findings)
}

/// Validates an export packet against the live chronology packet.
pub fn validate_chronology_export_packet(
    packet: &ChronologyHistoryPacket,
    export: &ChronologyExportPacket,
) -> ChronologyRowAuditReport {
    let mut findings = Vec::new();
    if export.record_kind != CHRONOLOGY_EXPORT_PACKET_RECORD_KIND {
        findings.push(finding(
            "export",
            "wrong_export_record_kind",
            "chronology export packet record kind must be stable",
        ));
    }

    for row in &packet.rows {
        let Some(export_row) = export
            .rows
            .iter()
            .find(|candidate| candidate.item_id_ref == row.item_id)
        else {
            findings.push(finding(
                &row.item_id,
                "missing_export_row",
                "export packet must preserve every source chronology row",
            ));
            continue;
        };

        if export_row.record_kind != CHRONOLOGY_EXPORT_ROW_RECORD_KIND {
            findings.push(finding(
                &row.item_id,
                "wrong_export_row_kind",
                "export row kind is not canonical",
            ));
        }
        if export_row.canonical_event_id != row.canonical_event_id {
            findings.push(finding(
                &row.item_id,
                "event_id_drift",
                "export event id drifted",
            ));
        }
        if export_row.grammar_sentence != row.grammar_sentence {
            findings.push(finding(
                &row.item_id,
                "grammar_sentence_drift",
                "export grammar sentence must match live row",
            ));
        }
        let live_markers: Vec<&str> = row
            .provenance_badges
            .iter()
            .map(|badge| badge.export_marker_label.as_str())
            .collect();
        for marker in live_markers {
            if !export_row
                .provenance_marker_labels
                .iter()
                .any(|label| label == marker)
            {
                findings.push(finding(
                    &row.item_id,
                    "provenance_marker_missing_from_export",
                    "export row must preserve provenance markers",
                ));
            }
        }
        if export_row.absolute_timestamp != row.time_posture.absolute_timestamp {
            findings.push(finding(
                &row.item_id,
                "absolute_time_drift",
                "export absolute timestamp drifted",
            ));
        }
        if export_row.relative_age_label != row.time_posture.relative_age.relative_label {
            findings.push(finding(
                &row.item_id,
                "relative_age_drift",
                "export relative-age hint drifted",
            ));
        }
        if export_row.visible_reason_label != row.time_posture.relative_age.visible_reason_label {
            findings.push(finding(
                &row.item_id,
                "visible_reason_drift",
                "export visible reason drifted",
            ));
        }
        if export_row.follow_up_state != row.follow_up_state {
            findings.push(finding(
                &row.item_id,
                "follow_up_state_drift",
                "export follow-up state drifted",
            ));
        }
        if export_row.reopen_target != row.reopen_target {
            findings.push(finding(
                &row.item_id,
                "reopen_target_drift",
                "export exact reopen target drifted",
            ));
        }
    }

    report(findings)
}

/// Validates accessibility projections for chronology rows.
pub fn validate_accessibility_fixture(
    packet: &ChronologyHistoryPacket,
    fixture: &AccessibilityChronologyFixture,
) -> ChronologyRowAuditReport {
    let mut findings = Vec::new();
    if fixture.record_kind != ACCESSIBILITY_CHRONOLOGY_FIXTURE_RECORD_KIND {
        findings.push(finding(
            "accessibility",
            "wrong_accessibility_record_kind",
            "accessibility fixture record kind must be stable",
        ));
    }

    for row in &packet.rows {
        let Some(projection) = fixture
            .rows
            .iter()
            .find(|candidate| candidate.item_id_ref == row.item_id)
        else {
            findings.push(finding(
                &row.item_id,
                "missing_accessibility_projection",
                "every chronology row must have an accessibility projection",
            ));
            continue;
        };
        for required in [
            row.actor_label.as_str(),
            action_label(row.action),
            row.object_label.as_str(),
            row.outcome_label.as_str(),
            row.time_posture.absolute_timestamp.as_str(),
            row.time_posture.relative_age.relative_label.as_str(),
            row.provenance_badges[0].badge_label.as_str(),
        ] {
            if !projection.screen_reader_label.contains(required) {
                findings.push(finding(
                    &row.item_id,
                    "screen_reader_label_missing_chronology_field",
                    "screen-reader label must include identity, provenance, time posture, and follow-up state",
                ));
            }
        }
        if projection.keyboard_command_refs.is_empty() {
            findings.push(finding(
                &row.item_id,
                "keyboard_path_missing",
                "row must expose a keyboard path",
            ));
        }
        if !projection.distinguishes_without_color {
            findings.push(finding(
                &row.item_id,
                "color_only_state",
                "row state must not rely on color",
            ));
        }
        if !projection.distinguishes_without_hover {
            findings.push(finding(
                &row.item_id,
                "hover_only_state",
                "row state must not rely on hover",
            ));
        }
        if projection.reduced_motion_behavior.trim().is_empty() {
            findings.push(finding(
                &row.item_id,
                "reduced_motion_missing",
                "reduced-motion behavior must be explicit",
            ));
        }
    }

    report(findings)
}

fn validate_row(row: &ChronologyHistoryRow, findings: &mut Vec<ChronologyRowFinding>) {
    if row.record_kind != CHRONOLOGY_HISTORY_ROW_RECORD_KIND {
        findings.push(finding(
            &row.item_id,
            "wrong_row_record_kind",
            "row record kind must be stable",
        ));
    }
    if row.schema_version != CHRONOLOGY_HISTORY_SCHEMA_VERSION {
        findings.push(finding(
            &row.item_id,
            "wrong_row_schema_version",
            "row schema version must be stable",
        ));
    }
    for (field, value) in [
        ("item_id", row.item_id.as_str()),
        ("canonical_event_id", row.canonical_event_id.as_str()),
        ("actor_label", row.actor_label.as_str()),
        ("object_label", row.object_label.as_str()),
        ("outcome_label", row.outcome_label.as_str()),
        (
            "absolute_timestamp",
            row.time_posture.absolute_timestamp.as_str(),
        ),
        ("timezone_iana", row.time_posture.timezone_iana.as_str()),
        ("utc_offset", row.time_posture.utc_offset.as_str()),
        (
            "relative_label",
            row.time_posture.relative_age.relative_label.as_str(),
        ),
        (
            "visible_reason_label",
            row.time_posture.relative_age.visible_reason_label.as_str(),
        ),
        ("reopen_target_ref", row.reopen_target.target_ref.as_str()),
        ("reopen_command_ref", row.reopen_target.command_ref.as_str()),
    ] {
        if value.trim().is_empty() {
            findings.push(finding(
                &row.item_id,
                "required_field_empty",
                format!("{field} must not be empty"),
            ));
        }
    }
    if row.grammar_sentence != row.canonical_sentence() {
        findings.push(finding(
            &row.item_id,
            "grammar_sentence_drift",
            "row grammar sentence must be generated from actor/action/object/outcome",
        ));
    }
    if row.provenance_badges.is_empty() {
        findings.push(finding(
            &row.item_id,
            "missing_provenance_badge",
            "row must carry a provenance badge",
        ));
    }
    for badge in &row.provenance_badges {
        if badge.badge_label.trim().is_empty() || badge.export_marker_label.trim().is_empty() {
            findings.push(finding(
                &row.item_id,
                "empty_provenance_label",
                "badge label and export marker must be non-empty",
            ));
        }
        if badge.source_class.requires_export_marker()
            && !badge
                .export_marker_label
                .contains(badge.badge_label.as_str())
        {
            findings.push(finding(
                &row.item_id,
                "provenance_export_marker_drift",
                "export marker must preserve imported, cached, stale, AI, policy, recovery, or companion badge text",
            ));
        }
    }
    if matches!(
        row.time_posture.relative_age.freshness_class,
        ChronologyFreshnessClass::Stale | ChronologyFreshnessClass::Expired
    ) && row.time_posture.relative_age.stale_reason_label.is_none()
    {
        findings.push(finding(
            &row.item_id,
            "stale_reason_missing",
            "stale or expired rows must explain why the row is still visible",
        ));
    }
    if row.reopen_target.command_ref == "home"
        || row.reopen_target.command_ref == "search"
        || row.reopen_target.target_kind == "generic"
    {
        findings.push(finding(
            &row.item_id,
            "generic_reopen_target",
            "reopen target must be exact, not a generic home or search fallback",
        ));
    }
    for transition in &row.allowed_transitions {
        if row.provider_owned_object
            && transition.local_authority_effect
                == LocalAuthorityEffectClass::RequiresReviewedProviderCommand
            && transition.reviewed_provider_command_ref.is_none()
        {
            findings.push(finding(
                &row.item_id,
                "provider_mutation_without_reviewed_command",
                "local chronology controls may not imply provider mutation without a reviewed command",
            ));
        }
        if row.provider_owned_object
            && matches!(
                transition.transition,
                FollowUpTransitionKind::Acknowledge
                    | FollowUpTransitionKind::Resolve
                    | FollowUpTransitionKind::Dismiss
                    | FollowUpTransitionKind::Snooze
                    | FollowUpTransitionKind::Mute
            )
            && transition.local_authority_effect != LocalAuthorityEffectClass::LocalOnly
        {
            findings.push(finding(
                &row.item_id,
                "local_follow_up_claims_provider_authority",
                "acknowledge, resolve, dismiss, snooze, and mute are local follow-up transitions",
            ));
        }
    }
}

/// Returns a seeded packet covering stable attention surfaces.
pub fn seeded_chronology_packet() -> ChronologyHistoryPacket {
    let rows = vec![
        row(RowSeed {
            id: "chrono:activity:index-refresh",
            event_id: "event:shell:index-refresh:2026-06-07T02:10:00Z",
            surface: ChronologySurfaceClass::ActivityCenter,
            actor_kind: ActorKind::SystemActor,
            actor_ref: None,
            actor: "Indexer",
            action: ActionVerb::Succeeded,
            object: "workspace symbol refresh",
            object_kind: ChronologyObjectKind::WorkspaceObjectRow,
            outcome: HistoryOutcomeClass::Succeeded,
            outcome_label: "current results available",
            source: ChronologySourceClass::FirstPartyDirectObservation,
            badge: "Direct",
            absolute: "2026-06-07T02:10:00Z",
            local: "2026-06-06 19:10 America/Los_Angeles",
            relative: "18 min ago",
            freshness: ChronologyFreshnessClass::Fresh,
            visible_reason: "completed rows remain visible in activity history",
            stale_reason: None,
            imported: ChronologyImportedClass::Live,
            follow_up: FollowUpState::None,
            provider_owned: false,
        }),
        row(RowSeed {
            id: "chrono:job:test-run",
            event_id: "event:task:test-run:2026-06-07T02:12:00Z",
            surface: ChronologySurfaceClass::DurableJob,
            actor_kind: ActorKind::UserActor,
            actor_ref: Some("actor:local-user"),
            actor: "You",
            action: ActionVerb::Failed,
            object: "unit test run",
            object_kind: ChronologyObjectKind::TaskRunRow,
            outcome: HistoryOutcomeClass::Failed,
            outcome_label: "2 failing tests need review",
            source: ChronologySourceClass::FirstPartyDirectObservation,
            badge: "Direct",
            absolute: "2026-06-07T02:12:00Z",
            local: "2026-06-06 19:12 America/Los_Angeles",
            relative: "16 min ago",
            freshness: ChronologyFreshnessClass::Fresh,
            visible_reason: "failed task remains visible until reviewed or superseded",
            stale_reason: None,
            imported: ChronologyImportedClass::Live,
            follow_up: FollowUpState::Open,
            provider_owned: false,
        }),
        row(RowSeed {
            id: "chrono:debug:partial-capture",
            event_id: "event:debug:partial-capture:2026-06-07T02:13:00Z",
            surface: ChronologySurfaceClass::DebugHistory,
            actor_kind: ActorKind::SystemActor,
            actor_ref: None,
            actor: "Debugger",
            action: ActionVerb::Presented,
            object: "partial replay capture",
            object_kind: ChronologyObjectKind::TaskRunRow,
            outcome: HistoryOutcomeClass::ObservedOnly,
            outcome_label: "recording gap labeled",
            source: ChronologySourceClass::FirstPartySynthesizedSummary,
            badge: "Synthesized",
            absolute: "2026-06-07T02:13:00Z",
            local: "2026-06-06 19:13 America/Los_Angeles",
            relative: "15 min ago",
            freshness: ChronologyFreshnessClass::Current,
            visible_reason: "partial chronology remains visible for replay honesty",
            stale_reason: None,
            imported: ChronologyImportedClass::Live,
            follow_up: FollowUpState::Open,
            provider_owned: false,
        }),
        row(RowSeed {
            id: "chrono:provider:review-import",
            event_id: "event:provider:review-import:2026-06-07T02:15:00Z",
            surface: ChronologySurfaceClass::ProviderEvent,
            actor_kind: ActorKind::RemoteServiceActor,
            actor_ref: Some("provider:github-enterprise"),
            actor: "GitHub Enterprise",
            action: ActionVerb::Imported,
            object: "PR review status",
            object_kind: ChronologyObjectKind::ProviderEventRow,
            outcome: HistoryOutcomeClass::ObservedOnly,
            outcome_label: "provider state mirrored locally",
            source: ChronologySourceClass::ProviderImported,
            badge: "Imported",
            absolute: "2026-06-07T02:15:00Z",
            local: "2026-06-06 19:15 America/Los_Angeles",
            relative: "13 min ago",
            freshness: ChronologyFreshnessClass::Fresh,
            visible_reason: "provider-linked update remains visible with local follow-up state",
            stale_reason: None,
            imported: ChronologyImportedClass::Imported,
            follow_up: FollowUpState::Acknowledged,
            provider_owned: true,
        }),
        row(RowSeed {
            id: "chrono:ai:run-history",
            event_id: "event:ai:run-history:2026-06-07T02:18:00Z",
            surface: ChronologySurfaceClass::AiRun,
            actor_kind: ActorKind::AiAgentActor,
            actor_ref: Some("ai-run:2048"),
            actor: "AI run",
            action: ActionVerb::Proposed,
            object: "patch review evidence",
            object_kind: ChronologyObjectKind::AiEvidenceRow,
            outcome: HistoryOutcomeClass::AwaitingApproval,
            outcome_label: "review required before apply",
            source: ChronologySourceClass::AiAssisted,
            badge: "AI assisted",
            absolute: "2026-06-07T02:18:00Z",
            local: "2026-06-06 19:18 America/Los_Angeles",
            relative: "10 min ago",
            freshness: ChronologyFreshnessClass::Current,
            visible_reason: "reviewable AI evidence remains visible until accepted or dismissed",
            stale_reason: None,
            imported: ChronologyImportedClass::Live,
            follow_up: FollowUpState::Open,
            provider_owned: false,
        }),
        row(RowSeed {
            id: "chrono:policy:connector-denial",
            event_id: "event:policy:connector-denial:2026-06-07T02:20:00Z",
            surface: ChronologySurfaceClass::PolicyAdminNotice,
            actor_kind: ActorKind::AdminPolicyActor,
            actor_ref: Some("policy:org-policy-43"),
            actor: "Org policy",
            action: ActionVerb::Blocked,
            object: "remote connector write",
            object_kind: ChronologyObjectKind::PolicyDecisionRow,
            outcome: HistoryOutcomeClass::Denied,
            outcome_label: "local-only fallback offered",
            source: ChronologySourceClass::PolicyAuthored,
            badge: "Policy",
            absolute: "2026-06-07T02:20:00Z",
            local: "2026-06-06 19:20 America/Los_Angeles",
            relative: "8 min ago",
            freshness: ChronologyFreshnessClass::Current,
            visible_reason: "policy notices stay visible while the denied action is still relevant",
            stale_reason: None,
            imported: ChronologyImportedClass::Live,
            follow_up: FollowUpState::Open,
            provider_owned: false,
        }),
        row(RowSeed {
            id: "chrono:recovery:crash-restore",
            event_id: "event:recovery:crash-restore:2026-06-07T01:22:00Z",
            surface: ChronologySurfaceClass::RecoveryTimeline,
            actor_kind: ActorKind::SystemActor,
            actor_ref: None,
            actor: "Recovery",
            action: ActionVerb::Recovered,
            object: "autosave snapshot",
            object_kind: ChronologyObjectKind::RecoverySnapshotRow,
            outcome: HistoryOutcomeClass::Recovered,
            outcome_label: "draft restored from local snapshot",
            source: ChronologySourceClass::RecoveryReconstructed,
            badge: "Reconstructed",
            absolute: "2026-06-07T01:22:00Z",
            local: "2026-06-06 18:22 America/Los_Angeles",
            relative: "1 h 6 min ago",
            freshness: ChronologyFreshnessClass::Stale,
            visible_reason: "recovery-created snapshots remain visible until discarded",
            stale_reason: Some("snapshot is older than current editor state"),
            imported: ChronologyImportedClass::Reconstructed,
            follow_up: FollowUpState::Open,
            provider_owned: false,
        }),
    ];

    ChronologyHistoryPacket {
        record_kind: CHRONOLOGY_HISTORY_PACKET_RECORD_KIND.to_owned(),
        schema_version: CHRONOLOGY_HISTORY_SCHEMA_VERSION,
        schema_ref: CHRONOLOGY_HISTORY_SCHEMA_REF.to_owned(),
        rows,
    }
}

/// Returns the seeded export packet for the canonical chronology packet.
pub fn seeded_chronology_export_packet() -> ChronologyExportPacket {
    let packet = seeded_chronology_packet();
    ChronologyExportPacket {
        record_kind: CHRONOLOGY_EXPORT_PACKET_RECORD_KIND.to_owned(),
        schema_version: CHRONOLOGY_HISTORY_SCHEMA_VERSION,
        destination_class: "support_bundle_and_companion_safe".to_owned(),
        source_item_ids: packet.rows.iter().map(|row| row.item_id.clone()).collect(),
        rows: packet
            .rows
            .iter()
            .map(|row| ChronologyExportRow {
                record_kind: CHRONOLOGY_EXPORT_ROW_RECORD_KIND.to_owned(),
                item_id_ref: row.item_id.clone(),
                canonical_event_id: row.canonical_event_id.clone(),
                grammar_sentence: row.grammar_sentence.clone(),
                provenance_marker_labels: row
                    .provenance_badges
                    .iter()
                    .map(|badge| badge.export_marker_label.clone())
                    .collect(),
                absolute_timestamp: row.time_posture.absolute_timestamp.clone(),
                relative_age_label: row.time_posture.relative_age.relative_label.clone(),
                visible_reason_label: row.time_posture.relative_age.visible_reason_label.clone(),
                follow_up_state: row.follow_up_state,
                reopen_target: row.reopen_target.clone(),
            })
            .collect(),
    }
}

/// Returns the seeded accessibility fixture for the canonical chronology packet.
pub fn seeded_accessibility_fixture() -> AccessibilityChronologyFixture {
    let packet = seeded_chronology_packet();
    AccessibilityChronologyFixture {
        record_kind: ACCESSIBILITY_CHRONOLOGY_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: CHRONOLOGY_HISTORY_SCHEMA_VERSION,
        rows: packet
            .rows
            .iter()
            .map(|row| AccessibilityHistoryRowProjection {
                item_id_ref: row.item_id.clone(),
                screen_reader_label: format!(
                    "{}. Provenance {}. Absolute time {}. Relative age {}. Follow-up {:?}.",
                    row.grammar_sentence,
                    row.provenance_badges[0].badge_label,
                    row.time_posture.absolute_timestamp,
                    row.time_posture.relative_age.relative_label,
                    row.follow_up_state
                ),
                keyboard_command_refs: vec![row.reopen_target.command_ref.clone()],
                reduced_motion_behavior:
                    "No animated age countdown; relative label updates on explicit refresh."
                        .to_owned(),
                distinguishes_without_color: true,
                distinguishes_without_hover: true,
            })
            .collect(),
    }
}

fn action_label(action: ActionVerb) -> &'static str {
    match action {
        ActionVerb::Started => "started",
        ActionVerb::Progressed => "progressed",
        ActionVerb::Succeeded => "succeeded",
        ActionVerb::Failed => "failed",
        ActionVerb::Cancelled => "cancelled",
        ActionVerb::Blocked => "blocked",
        ActionVerb::Unblocked => "unblocked",
        ActionVerb::Held => "held",
        ActionVerb::Released => "released",
        ActionVerb::Granted => "granted",
        ActionVerb::Narrowed => "narrowed",
        ActionVerb::Widened => "widened",
        ActionVerb::Revoked => "revoked",
        ActionVerb::Presented => "presented",
        ActionVerb::Superseded => "superseded",
        ActionVerb::Proposed => "proposed",
        ActionVerb::Accepted => "accepted",
        ActionVerb::Rejected => "rejected",
        ActionVerb::Restored => "restored",
        ActionVerb::Recovered => "recovered",
        ActionVerb::Published => "published",
        ActionVerb::Unpublished => "unpublished",
        ActionVerb::Exported => "exported",
        ActionVerb::Imported => "imported",
        ActionVerb::Acknowledged => "acknowledged",
        ActionVerb::Resolved => "resolved",
        ActionVerb::Dismissed => "dismissed",
        ActionVerb::Snoozed => "snoozed",
        ActionVerb::Muted => "muted",
    }
}

fn report(findings: Vec<ChronologyRowFinding>) -> ChronologyRowAuditReport {
    ChronologyRowAuditReport {
        passed: findings.is_empty(),
        findings,
    }
}

fn finding(
    item_id: impl Into<String>,
    code: impl Into<String>,
    message: impl Into<String>,
) -> ChronologyRowFinding {
    ChronologyRowFinding {
        item_id: item_id.into(),
        code: code.into(),
        message: message.into(),
    }
}

struct RowSeed {
    id: &'static str,
    event_id: &'static str,
    surface: ChronologySurfaceClass,
    actor_kind: ActorKind,
    actor_ref: Option<&'static str>,
    actor: &'static str,
    action: ActionVerb,
    object: &'static str,
    object_kind: ChronologyObjectKind,
    outcome: HistoryOutcomeClass,
    outcome_label: &'static str,
    source: ChronologySourceClass,
    badge: &'static str,
    absolute: &'static str,
    local: &'static str,
    relative: &'static str,
    freshness: ChronologyFreshnessClass,
    visible_reason: &'static str,
    stale_reason: Option<&'static str>,
    imported: ChronologyImportedClass,
    follow_up: FollowUpState,
    provider_owned: bool,
}

fn row(seed: RowSeed) -> ChronologyHistoryRow {
    let action = FollowUpTransition {
        transition: FollowUpTransitionKind::Acknowledge,
        label: "Acknowledge".to_owned(),
        local_authority_effect: LocalAuthorityEffectClass::LocalOnly,
        reviewed_provider_command_ref: None,
    };
    let mut row = ChronologyHistoryRow {
        record_kind: CHRONOLOGY_HISTORY_ROW_RECORD_KIND.to_owned(),
        schema_version: CHRONOLOGY_HISTORY_SCHEMA_VERSION,
        item_id: seed.id.to_owned(),
        canonical_event_id: seed.event_id.to_owned(),
        surface_class: seed.surface,
        actor_kind: seed.actor_kind,
        actor_ref: seed.actor_ref.map(str::to_owned),
        actor_label: seed.actor.to_owned(),
        action: seed.action,
        object_label: seed.object.to_owned(),
        object_kind: seed.object_kind,
        outcome: seed.outcome,
        outcome_label: seed.outcome_label.to_owned(),
        grammar_sentence: String::new(),
        provenance_badges: vec![ProvenanceBadge {
            source_class: seed.source,
            badge_label: seed.badge.to_owned(),
            export_marker_label: format!("{} source", seed.badge),
        }],
        time_posture: TimePosture {
            absolute_timestamp: seed.absolute.to_owned(),
            timezone_iana: "America/Los_Angeles".to_owned(),
            utc_offset: "-07:00".to_owned(),
            local_time_label: seed.local.to_owned(),
            imported_class: seed.imported,
            relative_age: RelativeAgeHint {
                rendered_at: "2026-06-07T02:28:00Z".to_owned(),
                relative_label: seed.relative.to_owned(),
                freshness_class: seed.freshness,
                visible_reason_label: seed.visible_reason.to_owned(),
                stale_reason_label: seed.stale_reason.map(str::to_owned),
            },
        },
        follow_up_state: seed.follow_up,
        allowed_transitions: vec![
            action,
            FollowUpTransition {
                transition: FollowUpTransitionKind::Resolve,
                label: "Resolve".to_owned(),
                local_authority_effect: LocalAuthorityEffectClass::LocalOnly,
                reviewed_provider_command_ref: None,
            },
            FollowUpTransition {
                transition: FollowUpTransitionKind::Dismiss,
                label: "Dismiss".to_owned(),
                local_authority_effect: LocalAuthorityEffectClass::LocalOnly,
                reviewed_provider_command_ref: None,
            },
            FollowUpTransition {
                transition: FollowUpTransitionKind::Snooze,
                label: "Snooze".to_owned(),
                local_authority_effect: LocalAuthorityEffectClass::LocalOnly,
                reviewed_provider_command_ref: None,
            },
            FollowUpTransition {
                transition: FollowUpTransitionKind::Mute,
                label: "Mute".to_owned(),
                local_authority_effect: LocalAuthorityEffectClass::LocalOnly,
                reviewed_provider_command_ref: None,
            },
        ],
        reopen_target: ReopenTarget {
            target_ref: format!("target:{}", seed.id),
            target_kind: format!("{:?}", seed.surface).to_lowercase(),
            command_ref: format!("cmd:chronology.open:{}", seed.id),
            label: format!("Open {}", seed.object),
        },
        provider_owned_object: seed.provider_owned,
    };
    row.grammar_sentence = row.canonical_sentence();
    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_passes_validation() {
        let report = validate_chronology_packet(&seeded_chronology_packet());
        assert!(report.passed, "{:#?}", report.findings);
    }

    #[test]
    fn export_packet_preserves_live_rows() {
        let packet = seeded_chronology_packet();
        let report = validate_chronology_export_packet(&packet, &seeded_chronology_export_packet());
        assert!(report.passed, "{:#?}", report.findings);
    }

    #[test]
    fn provider_owned_local_follow_up_cannot_claim_provider_mutation() {
        let mut packet = seeded_chronology_packet();
        let row = packet
            .rows
            .iter_mut()
            .find(|row| row.provider_owned_object)
            .expect("provider row");
        row.allowed_transitions[0].local_authority_effect =
            LocalAuthorityEffectClass::RequiresReviewedProviderCommand;
        row.allowed_transitions[0].reviewed_provider_command_ref = None;
        let report = validate_chronology_packet(&packet);
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "provider_mutation_without_reviewed_command"));
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "local_follow_up_claims_provider_authority"));
    }
}
