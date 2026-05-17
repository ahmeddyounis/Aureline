//! Evidence-timeline packet for deletion, hold, and retained-evidence chronology.
//!
//! This module mints the typed [`EvidenceTimelinePacket`] consumed by support
//! bundle previews and headless exports. The packet composes existing
//! records-governance and destruction-receipt truth into a per-event chronology
//! that preserves source timezone, canonical ordering, actor ordering, and the
//! controlled delete/hold vocabulary.

use std::collections::BTreeSet;

use aureline_records::{validate_typed, RecordClassId};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use super::manifest::SizeEstimate;
use super::preview::{PreviewItemSeed, SupportBundlePreviewBuilder};
use super::records::HoldClass;
use super::vocabulary::{ActionabilityImpactClass, DiagnosticDataClass, HighRiskContentClass};

/// Stable record-kind tag carried on every evidence-timeline packet.
pub const EVIDENCE_TIMELINE_PACKET_RECORD_KIND: &str = "evidence_timeline_packet_record";

/// Stable record-kind tag carried on every evidence-timeline event row.
pub const EVIDENCE_TIMELINE_EVENT_RECORD_KIND: &str = "evidence_timeline_event_record";

/// Frozen schema version for the evidence-timeline support packet.
pub const EVIDENCE_TIMELINE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative support boundary schema path.
pub const EVIDENCE_TIMELINE_SCHEMA_REF: &str = "schemas/support/evidence_timeline.schema.json";

/// Reviewer doc ref for chronology and delete-honesty support truth.
pub const EVIDENCE_TIMELINE_DOC_REF: &str = "docs/support/m3/chronology_and_delete_honesty_beta.md";

/// Checked-in review packet that demonstrates the beta lane.
pub const EVIDENCE_TIMELINE_ARTIFACT_REF: &str = "artifacts/support/m3/evidence_timeline_packet.md";

/// Controlled-vocabulary section shared by UI copy, docs, and exports.
pub const EVIDENCE_TIMELINE_VOCABULARY_REF: &str =
    "docs/support/m3/chronology_and_delete_honesty_beta.md#controlled-vocabulary";

/// Required redaction class for evidence-timeline support packets.
pub const EVIDENCE_TIMELINE_REDACTION_CLASS: &str = "metadata_safe_default";

/// Support-pack item id used when an evidence timeline is queued onto a preview.
pub const SUPPORT_ITEM_EVIDENCE_TIMELINE_PACKET: &str = "support.item.evidence_timeline_packet";

/// Closed delete/hold state vocabulary for evidence-timeline rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTimelineStateClass {
    /// A delete request was received but not yet accepted by the execution queue.
    RequestedDeletion,
    /// A delete request was accepted and is waiting on execution, replica purge, or policy gates.
    QueuedDeletion,
    /// One or more object classes remain held and cannot be destroyed.
    HeldData,
    /// Payload deletion, narrowing, or blocking left metadata or evidence behind.
    RetainedEvidence,
    /// Delete completed and no in-scope copy remains.
    CompletedDeletion,
}

impl EvidenceTimelineStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedDeletion => "requested_deletion",
            Self::QueuedDeletion => "queued_deletion",
            Self::HeldData => "held_data",
            Self::RetainedEvidence => "retained_evidence",
            Self::CompletedDeletion => "completed_deletion",
        }
    }

    /// Returns the stable operator-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RequestedDeletion => "Delete requested",
            Self::QueuedDeletion => "Delete queued",
            Self::HeldData => "Legal hold",
            Self::RetainedEvidence => "Policy retention",
            Self::CompletedDeletion => "Delete completed",
        }
    }
}

/// Closed actor vocabulary for evidence-timeline chronology rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTimelineActorClass {
    /// A local product user initiated or confirmed the event.
    LocalUser,
    /// An administrator or operator acting under managed policy.
    AdminOperator,
    /// A support agent handling an exported support case.
    SupportAgent,
    /// Product-owned retention, purge, or export automation.
    AutomatedRetentionJob,
    /// An AI tool or review-assist path produced evidence.
    AiTool,
    /// A review workspace, hosted review, or diff surface produced evidence.
    ReviewSurface,
    /// A search or retrieval indexer produced evidence.
    SearchIndexer,
    /// A content-safety scanner produced evidence.
    ContentSafetyScanner,
    /// Product-owned system automation.
    System,
}

impl EvidenceTimelineActorClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::AdminOperator => "admin_operator",
            Self::SupportAgent => "support_agent",
            Self::AutomatedRetentionJob => "automated_retention_job",
            Self::AiTool => "ai_tool",
            Self::ReviewSurface => "review_surface",
            Self::SearchIndexer => "search_indexer",
            Self::ContentSafetyScanner => "content_safety_scanner",
            Self::System => "system",
        }
    }
}

/// Closed source vocabulary for imported-vs-live event rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTimelineSourceClass {
    /// Current product state captured by the observing runtime.
    LiveSystemTruth,
    /// Current product state with a bounded skew label.
    LiveWithBoundedSkew,
    /// Managed surface mirror that may lag its authoritative origin.
    MirroredManagedSurface,
    /// Imported remote-agent evidence.
    ImportedRemoteAgent,
    /// Imported extension-host evidence.
    ImportedExtension,
    /// Imported external audit trail evidence.
    ImportedExternalAuditTrail,
    /// Offline local evidence packet.
    OfflineLocalEvidencePacket,
    /// Support-bundle replay.
    SupportBundleReplay,
    /// Recovery-snapshot replay.
    RecoverySnapshotReplay,
    /// Synthetic fixture evidence.
    SyntheticFixtureOnly,
}

impl EvidenceTimelineSourceClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveSystemTruth => "live_system_truth",
            Self::LiveWithBoundedSkew => "live_with_bounded_skew",
            Self::MirroredManagedSurface => "mirrored_managed_surface",
            Self::ImportedRemoteAgent => "imported_remote_agent",
            Self::ImportedExtension => "imported_extension",
            Self::ImportedExternalAuditTrail => "imported_external_audit_trail",
            Self::OfflineLocalEvidencePacket => "offline_local_evidence_packet",
            Self::SupportBundleReplay => "support_bundle_replay",
            Self::RecoverySnapshotReplay => "recovery_snapshot_replay",
            Self::SyntheticFixtureOnly => "synthetic_fixture_only",
        }
    }
}

/// Closed current-state vocabulary for evidence-timeline rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTimelineCurrentStateClass {
    /// The row reflects live product state.
    LiveCurrentSystemState,
    /// The row is a retained managed artifact.
    RetainedArtifactManagedAuthoritative,
    /// The row is retained at an imported origin.
    RetainedArtifactImportedOrigin,
    /// The row is retained as an offline local packet.
    RetainedArtifactOfflineLocal,
    /// The row is retained as a support replay.
    RetainedArtifactSupportReplay,
    /// The row is retained as a recovery replay.
    RetainedArtifactRecoveryReplay,
    /// The row is retained as a synthetic fixture.
    RetainedArtifactSyntheticFixture,
}

impl EvidenceTimelineCurrentStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveCurrentSystemState => "live_current_system_state",
            Self::RetainedArtifactManagedAuthoritative => "retained_artifact_managed_authoritative",
            Self::RetainedArtifactImportedOrigin => "retained_artifact_imported_origin",
            Self::RetainedArtifactOfflineLocal => "retained_artifact_offline_local",
            Self::RetainedArtifactSupportReplay => "retained_artifact_support_replay",
            Self::RetainedArtifactRecoveryReplay => "retained_artifact_recovery_replay",
            Self::RetainedArtifactSyntheticFixture => "retained_artifact_synthetic_fixture",
        }
    }
}

/// Closed timezone-basis vocabulary for evidence-timeline rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTimelineTimezoneBasisClass {
    /// Exported chronology is pinned to UTC.
    CanonicalUtc,
    /// Display time names the observer device's IANA zone.
    DeviceLocalIana,
    /// Display time names an admin-pinned IANA zone.
    DeploymentPinnedIana,
    /// Display time carries an observer-skew label.
    ObserverSkewLabeled,
    /// Display time names the import origin's IANA zone.
    ImportedOriginZoneLabeled,
    /// Only canonical UTC is rendered.
    NotRenderedCanonicalUtcOnly,
}

impl EvidenceTimelineTimezoneBasisClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalUtc => "canonical_utc",
            Self::DeviceLocalIana => "device_local_iana",
            Self::DeploymentPinnedIana => "deployment_pinned_iana",
            Self::ObserverSkewLabeled => "observer_skew_labeled",
            Self::ImportedOriginZoneLabeled => "imported_origin_zone_labeled",
            Self::NotRenderedCanonicalUtcOnly => "not_rendered_canonical_utc_only",
        }
    }

    fn requires_iana_zone(self) -> bool {
        matches!(
            self,
            Self::DeviceLocalIana | Self::DeploymentPinnedIana | Self::ImportedOriginZoneLabeled
        )
    }
}

/// Closed remaining-location vocabulary for deletion and hold events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTimelineLocationClass {
    /// Only the originating device holds the object.
    LocalDeviceOnly,
    /// A downloaded export copy remains under user control.
    LocalExportCopy,
    /// The managed archive is active.
    ManagedArchiveActive,
    /// The managed archive is held.
    ManagedArchiveHeld,
    /// The managed archive is policy retained.
    ManagedArchivePolicyRetained,
    /// A managed archive is propagating purge to replicas.
    ManagedArchiveReplicatedPendingPurge,
    /// Only metadata-only destruction receipt evidence remains.
    DestructionReceiptOnly,
    /// The remaining copy lives at an import origin.
    ImportSourceOrigin,
    /// The remaining copy is outside platform authority.
    OutsidePlatformScope,
    /// No in-scope location remains.
    NoRemainingLocation,
}

impl EvidenceTimelineLocationClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "local_device_only",
            Self::LocalExportCopy => "local_export_copy",
            Self::ManagedArchiveActive => "managed_archive_active",
            Self::ManagedArchiveHeld => "managed_archive_held",
            Self::ManagedArchivePolicyRetained => "managed_archive_policy_retained",
            Self::ManagedArchiveReplicatedPendingPurge => {
                "managed_archive_replicated_pending_purge"
            }
            Self::DestructionReceiptOnly => "destruction_receipt_only",
            Self::ImportSourceOrigin => "import_source_origin",
            Self::OutsidePlatformScope => "outside_platform_scope",
            Self::NoRemainingLocation => "no_remaining_location",
        }
    }
}

/// Closed retained-evidence reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTimelineRetainedReasonClass {
    /// No retained evidence reason applies.
    None,
    /// A destruction receipt is retained after payload deletion.
    DestructionReceipt,
    /// A policy retention floor keeps evidence visible.
    PolicyRetention,
    /// A support investigation keeps evidence visible.
    SupportInvestigation,
    /// A redaction or omission record remains visible.
    RedactionOmission,
    /// The remaining object is outside platform authority.
    OutsidePlatformScope,
}

impl EvidenceTimelineRetainedReasonClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DestructionReceipt => "destruction_receipt",
            Self::PolicyRetention => "policy_retention",
            Self::SupportInvestigation => "support_investigation",
            Self::RedactionOmission => "redaction_omission",
            Self::OutsidePlatformScope => "outside_platform_scope",
        }
    }
}

/// Timezone and display context preserved on one timeline event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceTimelineTimeContext {
    /// Timezone basis that governed the rendered row.
    pub timezone_basis_class: EvidenceTimelineTimezoneBasisClass,
    /// IANA zone used by the display row when the basis names a real zone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_time_zone_iana: Option<String>,
    /// UTC offset rendered by the source row.
    pub utc_offset: String,
    /// Reviewer-safe local time label.
    pub local_time_label: String,
}

/// Caller-supplied timeline event before chronology ordering is assigned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceTimelineEventInput {
    /// Stable event id.
    pub event_id: String,
    /// Display order from the source surface before export sorting.
    pub source_display_order: u32,
    /// Actor order within the same timestamp.
    pub actor_order: u32,
    /// Timeline state class.
    pub state_class: EvidenceTimelineStateClass,
    /// Actor class.
    pub actor_class: EvidenceTimelineActorClass,
    /// Opaque actor ref.
    pub actor_ref: String,
    /// Object, record, or packet the event is about.
    pub subject_ref: String,
    /// Source evidence ref.
    pub source_ref: String,
    /// RFC 3339 timestamp with the source offset preserved.
    pub occurred_at: String,
    /// Timezone and local-display context.
    pub time_context: EvidenceTimelineTimeContext,
    /// Imported-vs-live source class.
    pub evidence_source_class: EvidenceTimelineSourceClass,
    /// Current-state class.
    pub current_state_class: EvidenceTimelineCurrentStateClass,
    /// Optional chronology-context ref from the governance contract.
    pub chronology_context_ref: Option<String>,
    /// Optional delete-request ref.
    pub delete_request_ref: Option<String>,
    /// Optional records-governance packet ref.
    pub records_governance_packet_ref: Option<String>,
    /// Optional destruction-receipt ref.
    pub destruction_receipt_ref: Option<String>,
    /// Hold classes active on this event.
    pub hold_classes: Vec<HoldClass>,
    /// Remaining locations visible after this event.
    pub remaining_location_classes: Vec<EvidenceTimelineLocationClass>,
    /// Reason evidence remains visible.
    pub retained_reason_class: EvidenceTimelineRetainedReasonClass,
    /// Reviewer-safe note.
    pub note: String,
}

/// One chronology-ordered evidence-timeline event row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceTimelineEvent {
    pub record_kind: String,
    pub event_id: String,
    pub source_display_order: u32,
    pub chronology_order: u32,
    pub actor_order: u32,
    pub state_class: EvidenceTimelineStateClass,
    pub label: String,
    pub actor_class: EvidenceTimelineActorClass,
    pub actor_ref: String,
    pub subject_ref: String,
    pub source_ref: String,
    pub occurred_at: String,
    pub time_context: EvidenceTimelineTimeContext,
    pub evidence_source_class: EvidenceTimelineSourceClass,
    pub current_state_class: EvidenceTimelineCurrentStateClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chronology_context_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_request_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub records_governance_packet_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destruction_receipt_ref: Option<String>,
    pub hold_classes: Vec<HoldClass>,
    pub remaining_location_classes: Vec<EvidenceTimelineLocationClass>,
    pub retained_reason_class: EvidenceTimelineRetainedReasonClass,
    pub note: String,
}

/// Count summary for state classes inside a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceTimelineStateCounts {
    pub requested_deletion: u32,
    pub queued_deletion: u32,
    pub held_data: u32,
    pub retained_evidence: u32,
    pub completed_deletion: u32,
}

/// Caller-supplied packet inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceTimelinePacketInput {
    /// Stable packet id. Must start with `evidence_timeline_packet:`.
    pub packet_id: String,
    /// Reviewer-visible title.
    pub title: String,
    /// RFC 3339 timestamp for packet generation.
    pub generated_at: String,
    /// Support/export refs that carry this packet.
    pub support_export_refs: Vec<String>,
    /// Timeline events before chronology ordering is assigned.
    pub events: Vec<EvidenceTimelineEventInput>,
}

/// Operator-facing evidence timeline packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceTimelinePacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub title: String,
    pub generated_at: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub artifact_ref: String,
    pub controlled_vocabulary_ref: String,
    pub support_export_refs: Vec<String>,
    pub state_counts: EvidenceTimelineStateCounts,
    pub chronology_order_basis: String,
    pub events: Vec<EvidenceTimelineEvent>,
    pub redaction_class: String,
    pub raw_content_exported: bool,
    pub reviewer_summary: String,
}

impl EvidenceTimelinePacket {
    /// Returns true when all five delete/hold states appear in the packet.
    pub fn covers_delete_hold_state_vocabulary(&self) -> bool {
        self.state_counts.requested_deletion > 0
            && self.state_counts.queued_deletion > 0
            && self.state_counts.held_data > 0
            && self.state_counts.retained_evidence > 0
            && self.state_counts.completed_deletion > 0
    }

    /// Returns true when export sorting differs from at least one source display order.
    pub fn chronology_order_differs_from_display_order(&self) -> bool {
        self.events
            .iter()
            .any(|event| event.chronology_order != event.source_display_order)
    }
}

/// Errors returned by the evidence-timeline evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceTimelineError {
    /// A required id, ref, label, or summary field was empty.
    FieldEmpty { field: &'static str },
    /// The packet id did not start with `evidence_timeline_packet:`.
    PacketIdPrefixMissing { packet_id: String },
    /// The event list was empty.
    EventsEmpty,
    /// An event id appeared more than once.
    DuplicateEventId { event_id: String },
    /// A timestamp did not parse as RFC 3339.
    InvalidTimestamp { field: &'static str, value: String },
    /// A timezone basis that names a real zone omitted the IANA zone.
    TimezoneIanaMissing { event_id: String },
    /// Source and current state classes disagree.
    SourceCurrentStateMismatch { event_id: String, reason: String },
    /// State-specific required fields were missing or contradictory.
    StateInvariantViolation { event_id: String, reason: String },
    /// The record-class registry rejected the producer binding.
    RecordRegistry(aureline_records::RecordRegistryError),
}

impl std::fmt::Display for EvidenceTimelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldEmpty { field } => write!(f, "evidence timeline field {field} is required"),
            Self::PacketIdPrefixMissing { packet_id } => write!(
                f,
                "evidence timeline packet_id must start with 'evidence_timeline_packet:' (got {packet_id})"
            ),
            Self::EventsEmpty => write!(f, "evidence timeline must contain at least one event"),
            Self::DuplicateEventId { event_id } => {
                write!(f, "evidence timeline event_id {event_id} appears more than once")
            }
            Self::InvalidTimestamp { field, value } => {
                write!(f, "evidence timeline {field} must be RFC 3339, got {value}")
            }
            Self::TimezoneIanaMissing { event_id } => write!(
                f,
                "evidence timeline event {event_id} names an IANA timezone basis but omitted display_time_zone_iana"
            ),
            Self::SourceCurrentStateMismatch { event_id, reason } => write!(
                f,
                "evidence timeline event {event_id} has inconsistent source/current state: {reason}"
            ),
            Self::StateInvariantViolation { event_id, reason } => {
                write!(f, "evidence timeline event {event_id} violates state invariant: {reason}")
            }
            Self::RecordRegistry(err) => write!(f, "evidence timeline record registry error: {err}"),
        }
    }
}

impl std::error::Error for EvidenceTimelineError {}

impl From<aureline_records::RecordRegistryError> for EvidenceTimelineError {
    fn from(err: aureline_records::RecordRegistryError) -> Self {
        Self::RecordRegistry(err)
    }
}

/// Evaluates caller inputs into a chronology-ordered evidence timeline packet.
pub fn evaluate_evidence_timeline_packet(
    input: EvidenceTimelinePacketInput,
) -> Result<EvidenceTimelinePacket, EvidenceTimelineError> {
    validate_typed(
        EVIDENCE_TIMELINE_PACKET_RECORD_KIND,
        RecordClassId::SupportBundleArchive,
    )?;
    validate_packet_input(&input)?;

    let mut keyed_events = Vec::with_capacity(input.events.len());
    let mut seen_event_ids = BTreeSet::new();
    for event in input.events {
        if !seen_event_ids.insert(event.event_id.clone()) {
            return Err(EvidenceTimelineError::DuplicateEventId {
                event_id: event.event_id,
            });
        }
        validate_event_input(&event)?;
        let occurred_at = parse_rfc3339("events[].occurred_at", &event.occurred_at)?;
        keyed_events.push((
            occurred_at,
            event.actor_order,
            event.source_display_order,
            event,
        ));
    }

    keyed_events.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
            .then_with(|| left.3.event_id.cmp(&right.3.event_id))
    });

    let mut events = Vec::with_capacity(keyed_events.len());
    for (index, (_, _, _, event)) in keyed_events.into_iter().enumerate() {
        events.push(EvidenceTimelineEvent {
            record_kind: EVIDENCE_TIMELINE_EVENT_RECORD_KIND.to_owned(),
            event_id: event.event_id,
            source_display_order: event.source_display_order,
            chronology_order: index as u32,
            actor_order: event.actor_order,
            state_class: event.state_class,
            label: event.state_class.label().to_owned(),
            actor_class: event.actor_class,
            actor_ref: event.actor_ref,
            subject_ref: event.subject_ref,
            source_ref: event.source_ref,
            occurred_at: event.occurred_at,
            time_context: event.time_context,
            evidence_source_class: event.evidence_source_class,
            current_state_class: event.current_state_class,
            chronology_context_ref: event.chronology_context_ref,
            delete_request_ref: event.delete_request_ref,
            records_governance_packet_ref: event.records_governance_packet_ref,
            destruction_receipt_ref: event.destruction_receipt_ref,
            hold_classes: event.hold_classes,
            remaining_location_classes: event.remaining_location_classes,
            retained_reason_class: event.retained_reason_class,
            note: event.note,
        });
    }

    let state_counts = state_counts(&events);
    let reviewer_summary = format!(
        "Evidence timeline preserves {} event(s): requested {}, queued {}, held {}, retained {}, completed {}; source timestamps and timezone context are exported verbatim and chronology_order is derived from occurred_at then actor_order.",
        events.len(),
        state_counts.requested_deletion,
        state_counts.queued_deletion,
        state_counts.held_data,
        state_counts.retained_evidence,
        state_counts.completed_deletion
    );

    Ok(EvidenceTimelinePacket {
        schema_version: EVIDENCE_TIMELINE_SCHEMA_VERSION,
        record_kind: EVIDENCE_TIMELINE_PACKET_RECORD_KIND.to_owned(),
        packet_id: input.packet_id,
        title: input.title,
        generated_at: input.generated_at,
        schema_ref: EVIDENCE_TIMELINE_SCHEMA_REF.to_owned(),
        doc_ref: EVIDENCE_TIMELINE_DOC_REF.to_owned(),
        artifact_ref: EVIDENCE_TIMELINE_ARTIFACT_REF.to_owned(),
        controlled_vocabulary_ref: EVIDENCE_TIMELINE_VOCABULARY_REF.to_owned(),
        support_export_refs: input.support_export_refs,
        state_counts,
        chronology_order_basis: "occurred_at_rfc3339_then_actor_order_then_source_display_order"
            .to_owned(),
        events,
        redaction_class: EVIDENCE_TIMELINE_REDACTION_CLASS.to_owned(),
        raw_content_exported: false,
        reviewer_summary,
    })
}

fn validate_packet_input(input: &EvidenceTimelinePacketInput) -> Result<(), EvidenceTimelineError> {
    if !input.packet_id.starts_with("evidence_timeline_packet:") {
        return Err(EvidenceTimelineError::PacketIdPrefixMissing {
            packet_id: input.packet_id.clone(),
        });
    }
    require_non_empty(&input.title, "title")?;
    parse_rfc3339("generated_at", &input.generated_at)?;
    require_non_empty_string_list(&input.support_export_refs, "support_export_refs")?;
    if input.events.is_empty() {
        return Err(EvidenceTimelineError::EventsEmpty);
    }
    Ok(())
}

fn validate_event_input(event: &EvidenceTimelineEventInput) -> Result<(), EvidenceTimelineError> {
    require_non_empty(&event.event_id, "events[].event_id")?;
    require_non_empty(&event.actor_ref, "events[].actor_ref")?;
    require_non_empty(&event.subject_ref, "events[].subject_ref")?;
    require_non_empty(&event.source_ref, "events[].source_ref")?;
    require_non_empty(
        &event.time_context.utc_offset,
        "events[].time_context.utc_offset",
    )?;
    require_non_empty(
        &event.time_context.local_time_label,
        "events[].time_context.local_time_label",
    )?;
    require_non_empty(&event.note, "events[].note")?;

    if event.time_context.timezone_basis_class.requires_iana_zone()
        && event
            .time_context
            .display_time_zone_iana
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        return Err(EvidenceTimelineError::TimezoneIanaMissing {
            event_id: event.event_id.clone(),
        });
    }

    validate_source_current_state(event)?;
    validate_state_invariants(event)?;
    Ok(())
}

fn validate_source_current_state(
    event: &EvidenceTimelineEventInput,
) -> Result<(), EvidenceTimelineError> {
    let expected = match event.evidence_source_class {
        EvidenceTimelineSourceClass::LiveSystemTruth
        | EvidenceTimelineSourceClass::LiveWithBoundedSkew => {
            EvidenceTimelineCurrentStateClass::LiveCurrentSystemState
        }
        EvidenceTimelineSourceClass::MirroredManagedSurface => {
            EvidenceTimelineCurrentStateClass::RetainedArtifactManagedAuthoritative
        }
        EvidenceTimelineSourceClass::ImportedRemoteAgent
        | EvidenceTimelineSourceClass::ImportedExtension
        | EvidenceTimelineSourceClass::ImportedExternalAuditTrail => {
            EvidenceTimelineCurrentStateClass::RetainedArtifactImportedOrigin
        }
        EvidenceTimelineSourceClass::OfflineLocalEvidencePacket => {
            EvidenceTimelineCurrentStateClass::RetainedArtifactOfflineLocal
        }
        EvidenceTimelineSourceClass::SupportBundleReplay => {
            EvidenceTimelineCurrentStateClass::RetainedArtifactSupportReplay
        }
        EvidenceTimelineSourceClass::RecoverySnapshotReplay => {
            EvidenceTimelineCurrentStateClass::RetainedArtifactRecoveryReplay
        }
        EvidenceTimelineSourceClass::SyntheticFixtureOnly => {
            EvidenceTimelineCurrentStateClass::RetainedArtifactSyntheticFixture
        }
    };
    if event.current_state_class != expected {
        return Err(EvidenceTimelineError::SourceCurrentStateMismatch {
            event_id: event.event_id.clone(),
            reason: format!(
                "source {} requires current state {}, got {}",
                event.evidence_source_class.as_str(),
                expected.as_str(),
                event.current_state_class.as_str()
            ),
        });
    }
    Ok(())
}

fn validate_state_invariants(
    event: &EvidenceTimelineEventInput,
) -> Result<(), EvidenceTimelineError> {
    match event.state_class {
        EvidenceTimelineStateClass::RequestedDeletion
        | EvidenceTimelineStateClass::QueuedDeletion => {
            if event
                .delete_request_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                return Err(EvidenceTimelineError::StateInvariantViolation {
                    event_id: event.event_id.clone(),
                    reason: "delete request states require delete_request_ref".to_owned(),
                });
            }
        }
        EvidenceTimelineStateClass::HeldData => {
            if event.hold_classes.is_empty() {
                return Err(EvidenceTimelineError::StateInvariantViolation {
                    event_id: event.event_id.clone(),
                    reason: "held_data requires at least one hold_class".to_owned(),
                });
            }
            if !event
                .remaining_location_classes
                .contains(&EvidenceTimelineLocationClass::ManagedArchiveHeld)
            {
                return Err(EvidenceTimelineError::StateInvariantViolation {
                    event_id: event.event_id.clone(),
                    reason: "held_data requires managed_archive_held as a remaining location"
                        .to_owned(),
                });
            }
        }
        EvidenceTimelineStateClass::RetainedEvidence => {
            if matches!(
                event.retained_reason_class,
                EvidenceTimelineRetainedReasonClass::None
            ) {
                return Err(EvidenceTimelineError::StateInvariantViolation {
                    event_id: event.event_id.clone(),
                    reason: "retained_evidence requires a retained_reason_class".to_owned(),
                });
            }
            if event
                .remaining_location_classes
                .contains(&EvidenceTimelineLocationClass::NoRemainingLocation)
            {
                return Err(EvidenceTimelineError::StateInvariantViolation {
                    event_id: event.event_id.clone(),
                    reason: "retained_evidence cannot claim no_remaining_location".to_owned(),
                });
            }
        }
        EvidenceTimelineStateClass::CompletedDeletion => {
            if event.remaining_location_classes
                != vec![EvidenceTimelineLocationClass::NoRemainingLocation]
            {
                return Err(EvidenceTimelineError::StateInvariantViolation {
                    event_id: event.event_id.clone(),
                    reason: "completed_deletion requires only no_remaining_location".to_owned(),
                });
            }
            if !matches!(
                event.retained_reason_class,
                EvidenceTimelineRetainedReasonClass::None
            ) {
                return Err(EvidenceTimelineError::StateInvariantViolation {
                    event_id: event.event_id.clone(),
                    reason: "completed_deletion cannot carry a retained evidence reason".to_owned(),
                });
            }
        }
    }
    Ok(())
}

fn state_counts(events: &[EvidenceTimelineEvent]) -> EvidenceTimelineStateCounts {
    let mut counts = EvidenceTimelineStateCounts {
        requested_deletion: 0,
        queued_deletion: 0,
        held_data: 0,
        retained_evidence: 0,
        completed_deletion: 0,
    };
    for event in events {
        match event.state_class {
            EvidenceTimelineStateClass::RequestedDeletion => counts.requested_deletion += 1,
            EvidenceTimelineStateClass::QueuedDeletion => counts.queued_deletion += 1,
            EvidenceTimelineStateClass::HeldData => counts.held_data += 1,
            EvidenceTimelineStateClass::RetainedEvidence => counts.retained_evidence += 1,
            EvidenceTimelineStateClass::CompletedDeletion => counts.completed_deletion += 1,
        }
    }
    counts
}

/// Builds a preview-row seed for an evidence timeline packet.
pub fn evidence_timeline_preview_item_seed(packet: &EvidenceTimelinePacket) -> PreviewItemSeed {
    let bytes_estimate = serde_json::to_vec(packet)
        .map(|bytes| bytes.len() as u64)
        .ok();
    let display_label = bytes_estimate
        .map(|bytes| format!("{bytes} B"))
        .unwrap_or_else(|| "unknown until export".to_owned());

    PreviewItemSeed {
        support_pack_item_id: SUPPORT_ITEM_EVIDENCE_TIMELINE_PACKET.to_owned(),
        title: format!("Evidence timeline - {}", packet.title),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "governance_and_export_controls".to_owned(),
        artifact_kind_class: "evidence_timeline_packet".to_owned(),
        manifest_path_ref: "preview_items[].evidence_timeline_packet_record".to_owned(),
        bundle_member_path_ref: Some(format!(
            "manifest/evidence_timelines/{}.json",
            safe_member_name(&packet.packet_id)
        )),
        source_refs: vec![
            packet.schema_ref.clone(),
            packet.doc_ref.clone(),
            packet.artifact_ref.clone(),
            packet.controlled_vocabulary_ref.clone(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: bytes_estimate,
            confidence_class: if bytes_estimate.is_some() {
                "exact".to_owned()
            } else {
                "unknown".to_owned()
            },
            display_label,
            size_source_class: if bytes_estimate.is_some() {
                "precomputed_manifest".to_owned()
            } else {
                "unknown_until_export".to_owned()
            },
        },
        impact_class: if packet.state_counts.held_data > 0
            || packet.state_counts.retained_evidence > 0
        {
            ActionabilityImpactClass::High
        } else {
            ActionabilityImpactClass::Medium
        },
        impact_summary: format!(
            "Evidence timeline with {} chronology-ordered events and {} retained/held events.",
            packet.events.len(),
            packet.state_counts.held_data + packet.state_counts.retained_evidence
        ),
        notes: packet.reviewer_summary.clone(),
    }
}

/// Queues an evidence-timeline row on a support-bundle preview builder.
pub fn add_evidence_timeline_preview_item<'a>(
    builder: &'a mut SupportBundlePreviewBuilder,
    packet: &EvidenceTimelinePacket,
) -> &'a mut SupportBundlePreviewBuilder {
    builder.add_item(evidence_timeline_preview_item_seed(packet))
}

fn parse_rfc3339(
    field: &'static str,
    value: &str,
) -> Result<OffsetDateTime, EvidenceTimelineError> {
    OffsetDateTime::parse(value, &Rfc3339).map_err(|_| EvidenceTimelineError::InvalidTimestamp {
        field,
        value: value.to_owned(),
    })
}

fn require_non_empty(value: &str, field: &'static str) -> Result<(), EvidenceTimelineError> {
    if value.trim().is_empty() {
        Err(EvidenceTimelineError::FieldEmpty { field })
    } else {
        Ok(())
    }
}

fn require_non_empty_string_list(
    items: &[String],
    field: &'static str,
) -> Result<(), EvidenceTimelineError> {
    if items.is_empty() {
        return Err(EvidenceTimelineError::FieldEmpty { field });
    }
    for item in items {
        require_non_empty(item, field)?;
    }
    Ok(())
}

fn safe_member_name(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
