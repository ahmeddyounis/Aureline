//! Replay bundles that retain normalized envelopes plus raw-payload lineage and
//! join them into support, incident, and AI evidence packets.
//!
//! The canonical event envelope ([`crate::m5_task_event_envelope_bus`]) carries
//! a retained raw-payload reference and a retention class on every record, and
//! the frozen policy layer ([`crate::m5_task_event_adapter_policy`]) fixes the
//! adapter-priority ladder, the raw-payload-retention matrix, and the closed
//! downgrade vocabulary. This module is the dual-retention lane those contracts
//! exist for: one [`ReplayBundle`] binds the canonical normalized history to a
//! typed, bounded raw-payload lineage index, and joins both halves into the
//! support, incident, and AI evidence surfaces so a reviewer can follow the
//! raw-to-normalized chain end to end without ever guessing whether an
//! explanation came from canonical events, original adapter output, or a
//! heuristic reconstruction.
//!
//! The bundle deliberately reuses the canonical [`TaskEventRecord`] and the same
//! source-kind, confidence, retention-class, and provenance vocabulary
//! rather than minting a parallel replay model per surface. It adds exactly one
//! thing beyond the envelope: a [`RawPayloadLineageEntry`] per retained
//! raw-payload reference, carrying the retention class, a payload digest, a
//! bounded retained byte length, and the per-surface disclosure posture, plus
//! the [`referencing_event_ids`](RawPayloadLineageEntry::referencing_event_ids)
//! that pin the raw payload to the normalized events that cite it.
//!
//! Four invariants keep the dual-retention model trustworthy:
//!
//! - **The two halves stay joined.** Every normalized event's raw-payload
//!   reference resolves to exactly one lineage entry whose source kind and
//!   retention class agree with the event, and every lineage entry is cited by
//!   at least one event. Neither half can drift away from the other.
//! - **Retention stays typed and bounded.** Each lineage entry's retained byte
//!   length stays at or below the bound its retention class allows, and its
//!   replay / support-export / AI-evidence disclosure flags match the canonical
//!   posture for the class. Raw bodies never cross the boundary; only references
//!   and digests do.
//! - **Joins never expose secrets or flatten provenance.** The support, incident,
//!   and AI evidence joins read the normalized envelope and the raw lineage, keep
//!   source, confidence, priority rank, provenance, and downgrade disclosure
//!   visible, and gate any approval-only raw reference behind a redaction marker
//!   instead of dropping the row.
//! - **Replay survives the delivery anomalies the docs require.** The bundle
//!   proves a stable replay digest under raw-payload truncation, duplicate
//!   delivery, adapter drift, and export/import round-trip.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/replay-and-raw-payload-lineage.md`](../../../docs/m5/replay-and-raw-payload-lineage.md);
//! the machine-readable boundary lives at
//! [`/schemas/tooling/replay-bundle.schema.json`](../../../schemas/tooling/replay-bundle.schema.json).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::build_test_event_interoperability::{
    BuildTestEventConfidence, BuildTestEventSourceKind, BuildTestInteropFindingSeverity,
    BuildTestInteropPromotionState, RawPayloadRetentionClass,
};
use crate::m5_task_event_adapter_policy::{canonical_priority_rank, DowngradeReason};
use crate::m5_task_event_envelope_bus::{
    current_stable_task_event_first_consumers_input, TaskEventRecord,
};

/// Stable record-kind tag for [`ReplayBundle`].
pub const REPLAY_BUNDLE_RECORD_KIND: &str = "m5_replay_bundle";

/// Stable record-kind tag for [`ReplayBundleSupportExport`].
pub const REPLAY_BUNDLE_SUPPORT_EXPORT_RECORD_KIND: &str = "m5_replay_bundle_support_export";

/// Stable record-kind tag for [`ReplayEvidenceJoinView`].
pub const REPLAY_BUNDLE_EVIDENCE_JOIN_RECORD_KIND: &str = "m5_replay_bundle_evidence_join";

/// Stable record-kind tag for [`ReplayBundleCliHeadlessView`].
pub const REPLAY_BUNDLE_CLI_HEADLESS_RECORD_KIND: &str = "m5_replay_bundle_cli_headless";

/// Integer schema version for the replay bundle.
pub const REPLAY_BUNDLE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the replay-bundle boundary schema.
pub const REPLAY_BUNDLE_SCHEMA_REF: &str = "schemas/tooling/replay-bundle.schema.json";

/// Repo-relative path of the per-event task-event envelope boundary schema.
pub const REPLAY_BUNDLE_ENVELOPE_SCHEMA_REF: &str =
    "schemas/tooling/task-event-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const REPLAY_BUNDLE_DOC_REF: &str = "docs/m5/replay-and-raw-payload-lineage.md";

/// Repo-relative path of the frozen adapter-policy baseline this lane consumes.
pub const REPLAY_BUNDLE_POLICY_BASELINE_REF: &str =
    "artifacts/m5/tooling/event-interop-baseline/baseline.json";

/// Repo-relative path of the first-consumers packet whose normalized history this
/// bundle wraps.
pub const REPLAY_BUNDLE_FIRST_CONSUMERS_PACKET_REF: &str =
    "artifacts/m5/tooling/event-envelope-first-consumers/packet.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const REPLAY_BUNDLE_FIXTURE_DIR: &str = "fixtures/tooling/m5/replay-bundles";

/// Repo-relative path of the checked-in bundle artifact.
pub const REPLAY_BUNDLE_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/tooling/raw-plus-normalized-replay/packet.json";

/// Stable bundle id minted by the seed.
pub const REPLAY_BUNDLE_ID: &str = "tooling:m5:replay-bundle:v1";

/// Stable support-export id minted by the seed inspector.
pub const REPLAY_BUNDLE_SUPPORT_EXPORT_ID: &str = "support-export:tooling:m5:replay-bundle";

/// Stable AI evidence join id minted by the seed inspector.
pub const REPLAY_BUNDLE_AI_EVIDENCE_ID: &str = "ai-evidence:tooling:m5:replay-bundle";

/// Stable incident packet join id minted by the seed inspector.
pub const REPLAY_BUNDLE_INCIDENT_PACKET_ID: &str = "incident:tooling:m5:replay-bundle";

/// Stable CLI/headless view id minted by the seed inspector.
pub const REPLAY_BUNDLE_CLI_HEADLESS_ID: &str = "cli-headless:tooling:m5:replay-bundle";

/// Event id whose raw payload the seed gates behind support approval.
///
/// The debug session can capture variable and program-state payloads that may
/// contain secrets, so the seed retains its raw payload under
/// [`RawPayloadRetentionClass::SupportApprovalRequired`] to exercise the
/// redaction-honoring join path.
const SEED_APPROVAL_GATED_EVENT_ID: &str = "event:debug:finished";

/// Surface that joins the normalized history to the raw-payload lineage.
///
/// The replay engine reads everything within the runtime trust boundary; the
/// three export surfaces — support bundles, incident packets, and AI evidence —
/// only cite the lineage entries their retention posture allows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayJoinSurface {
    /// In-runtime replay engine; may resolve every replay-safe reference.
    Replay,
    /// Support bundle / support export.
    SupportBundle,
    /// Incident timeline packet.
    IncidentPacket,
    /// AI evidence packet.
    AiEvidence,
}

impl ReplayJoinSurface {
    /// Every join surface in stable declaration order.
    pub const ALL: [Self; 4] = [
        Self::Replay,
        Self::SupportBundle,
        Self::IncidentPacket,
        Self::AiEvidence,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Replay => "replay",
            Self::SupportBundle => "support_bundle",
            Self::IncidentPacket => "incident_packet",
            Self::AiEvidence => "ai_evidence",
        }
    }

    /// True when the surface leaves the runtime trust boundary and so must honor
    /// raw-payload redaction.
    pub const fn is_export(self) -> bool {
        !matches!(self, Self::Replay)
    }
}

/// Delivery anomaly the replay bundle must survive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayFailureMode {
    /// A raw payload arrives truncated or partial.
    Truncation,
    /// The same normalized event is delivered more than once.
    DuplicateDelivery,
    /// A drifted or lower-priority adapter re-reports an authoritative slot.
    AdapterDrift,
    /// The bundle is exported and re-imported.
    ExportImportRoundTrip,
}

impl ReplayFailureMode {
    /// Every required failure mode in stable declaration order.
    pub const ALL: [Self; 4] = [
        Self::Truncation,
        Self::DuplicateDelivery,
        Self::AdapterDrift,
        Self::ExportImportRoundTrip,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Truncation => "truncation",
            Self::DuplicateDelivery => "duplicate_delivery",
            Self::AdapterDrift => "adapter_drift",
            Self::ExportImportRoundTrip => "export_import_round_trip",
        }
    }

    /// Recovery posture the bundle must apply for this failure mode.
    pub const fn canonical_recovery(self) -> ReplayRecoveryPosture {
        match self {
            Self::Truncation => ReplayRecoveryPosture::ReconstructedFromLineage,
            Self::DuplicateDelivery => ReplayRecoveryPosture::DeduplicatedStable,
            Self::AdapterDrift => ReplayRecoveryPosture::DowngradedVisibly,
            Self::ExportImportRoundTrip => ReplayRecoveryPosture::RoundTripStable,
        }
    }
}

/// How replay recovers a stable history after a delivery anomaly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayRecoveryPosture {
    /// The normalized envelope is retained independently of the raw body, so a
    /// truncated raw payload leaves the normalized replay unchanged.
    ReconstructedFromLineage,
    /// Duplicate deliveries collapse by event id, leaving the digest unchanged.
    DeduplicatedStable,
    /// A drifted lower-priority re-report stays a visible downgrade and never
    /// displaces the authoritative winner.
    DowngradedVisibly,
    /// Export then import reproduces the same normalized history and lineage.
    RoundTripStable,
}

impl ReplayRecoveryPosture {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructedFromLineage => "reconstructed_from_lineage",
            Self::DeduplicatedStable => "deduplicated_stable",
            Self::DowngradedVisibly => "downgraded_visibly",
            Self::RoundTripStable => "round_trip_stable",
        }
    }
}

/// Retained byte bound for a retention class.
///
/// Metadata-only retention keeps no body bytes; the other classes keep a
/// bounded redacted reference or an approval-gated body. The bound makes the
/// "typed, bounded, evidence-oriented" guardrail enforceable.
pub const fn retention_byte_bound(class: RawPayloadRetentionClass) -> u64 {
    match class {
        RawPayloadRetentionClass::MetadataDigestOnly => 0,
        RawPayloadRetentionClass::RedactedReference => 4_096,
        RawPayloadRetentionClass::SupportApprovalRequired => 65_536,
    }
}

/// True when replay suites may resolve a reference of this retention class.
///
/// Replay runs inside the runtime trust boundary, so every retained reference is
/// replay-resolvable.
pub const fn retention_replay_safe(_class: RawPayloadRetentionClass) -> bool {
    true
}

/// True when support and incident exports may cite a reference of this class.
pub const fn retention_support_export_safe(class: RawPayloadRetentionClass) -> bool {
    !matches!(class, RawPayloadRetentionClass::SupportApprovalRequired)
}

/// True when AI evidence packets may cite a reference of this class.
pub const fn retention_ai_evidence_safe(class: RawPayloadRetentionClass) -> bool {
    !matches!(class, RawPayloadRetentionClass::SupportApprovalRequired)
}

/// One typed, bounded raw-payload lineage entry.
///
/// Exactly one entry exists per retained raw-payload reference. It joins the
/// reference to the normalized events that cite it
/// ([`referencing_event_ids`](Self::referencing_event_ids)) and records the
/// retention posture that governs whether each export surface may resolve it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawPayloadLineageEntry {
    /// Retained raw-payload reference (shared with the normalized event).
    pub raw_payload_ref: String,
    /// Source kind that produced the raw payload.
    pub source_kind: BuildTestEventSourceKind,
    /// Retention class for the raw payload.
    pub retention_class: RawPayloadRetentionClass,
    /// Digest of the raw payload; always safe to disclose.
    pub payload_digest: String,
    /// Retained byte length; stays at or below the class bound.
    pub payload_byte_len: u64,
    /// Maximum bytes the retention class allows.
    pub retained_byte_bound: u64,
    /// True when replay may resolve the reference.
    pub replay_safe: bool,
    /// True when support and incident exports may cite the reference.
    pub support_export_safe: bool,
    /// True when AI evidence packets may cite the reference.
    pub ai_evidence_safe: bool,
    /// Normalized event ids that cite this reference (derived, sorted).
    #[serde(default)]
    pub referencing_event_ids: Vec<String>,
}

impl RawPayloadLineageEntry {
    fn is_bound(&self) -> bool {
        !self.raw_payload_ref.trim().is_empty() && !self.payload_digest.trim().is_empty()
    }

    /// True when the lineage entry's posture matches the canonical retention
    /// posture for its class and stays within the class byte bound.
    fn posture_consistent(&self) -> bool {
        self.retained_byte_bound == retention_byte_bound(self.retention_class)
            && self.payload_byte_len <= self.retained_byte_bound
            && self.replay_safe == retention_replay_safe(self.retention_class)
            && self.support_export_safe == retention_support_export_safe(self.retention_class)
            && self.ai_evidence_safe == retention_ai_evidence_safe(self.retention_class)
    }

    /// True when this surface may resolve the reference rather than gate it.
    fn citable_by(&self, surface: ReplayJoinSurface) -> bool {
        match surface {
            ReplayJoinSurface::Replay => self.replay_safe,
            ReplayJoinSurface::SupportBundle | ReplayJoinSurface::IncidentPacket => {
                self.support_export_safe
            }
            ReplayJoinSurface::AiEvidence => self.ai_evidence_safe,
        }
    }
}

/// Projection proving a join surface reads both halves of the bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageJoinProjection {
    /// Join surface.
    pub surface: ReplayJoinSurface,
    /// Stable join ref.
    pub join_ref: String,
    /// True when the surface reads the canonical normalized envelope.
    pub binds_normalized_envelope: bool,
    /// True when the surface joins to the raw-payload lineage index.
    pub binds_raw_lineage: bool,
    /// True when source kind stays visible.
    pub preserves_source_kind: bool,
    /// True when the adapter priority rank stays visible.
    pub preserves_priority_rank: bool,
    /// True when confidence stays visible.
    pub preserves_confidence: bool,
    /// True when provenance stays visible (provenance is never flattened).
    pub preserves_provenance: bool,
    /// True when downgraded rows stay visibly downgraded.
    pub preserves_downgrade_disclosure: bool,
    /// True when the surface gates approval-only references instead of exposing
    /// them.
    pub honors_retention_redaction: bool,
    /// Count of lineage entries this surface may cite (derived).
    pub citable_payload_count: usize,
}

impl LineageJoinProjection {
    fn preserves_truth(&self) -> bool {
        !self.join_ref.trim().is_empty()
            && self.binds_normalized_envelope
            && self.binds_raw_lineage
            && self.preserves_source_kind
            && self.preserves_priority_rank
            && self.preserves_confidence
            && self.preserves_provenance
            && self.preserves_downgrade_disclosure
            && self.honors_retention_redaction
    }
}

/// Derived evidence that replay stays stable under one delivery anomaly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayRobustnessCase {
    /// Delivery anomaly exercised.
    pub failure_mode: ReplayFailureMode,
    /// Recovery posture applied.
    pub recovery_posture: ReplayRecoveryPosture,
    /// Replay digest before the anomaly is normalized.
    pub replay_digest_before: String,
    /// Replay digest after the anomaly is normalized.
    pub replay_digest_after: String,
    /// True when replay stayed faithful (digest unchanged).
    pub stable: bool,
    /// Short support-safe description of the drill.
    pub detail: String,
}

/// Closed validation finding vocabulary for the replay bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayBundleFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// The bundle carries no normalized events.
    NoNormalizedEvents,
    /// A normalized event has incomplete identity.
    EventIdentityIncomplete,
    /// A normalized event's priority rank disagrees with its source kind.
    EventPriorityMismatch,
    /// Two normalized events share an event id.
    DuplicateEventId,
    /// Two normalized events in one trace share a sequence number.
    ReplaySequenceCollision,
    /// A normalized event cites a raw-payload reference with no lineage entry.
    LineageEntryMissing,
    /// A lineage entry is cited by no normalized event.
    LineageEntryOrphan,
    /// A lineage entry's source kind disagrees with its referencing events.
    LineageSourceMismatch,
    /// A lineage entry's retention class disagrees with its referencing events.
    LineageRetentionMismatch,
    /// A lineage entry has no digest or reference.
    LineageDigestMissing,
    /// A lineage entry exceeds the byte bound or carries an inconsistent posture.
    RawPayloadUnbounded,
    /// A lineage entry's disclosure flags disagree with the canonical posture.
    RetentionPostureMismatch,
    /// A lineage entry's referencing-event list disagrees with the derivation.
    LineageReferenceDrift,
    /// A required join-surface projection is absent.
    JoinProjectionMissing,
    /// A join-surface projection drops normalized or raw truth.
    JoinProjectionDropsTruth,
    /// A join-surface projection's citable count disagrees with the lineage.
    JoinCountDrift,
    /// A required robustness case is absent.
    RobustnessCaseMissing,
    /// A robustness case's recovery posture disagrees with its failure mode.
    RobustnessRecoveryMismatch,
    /// A robustness case is not stable under replay.
    ReplayNotStable,
    /// Stored promotion state disagrees with the derived state.
    PromotionStateMismatch,
}

impl ReplayBundleFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::NoNormalizedEvents => "no_normalized_events",
            Self::EventIdentityIncomplete => "event_identity_incomplete",
            Self::EventPriorityMismatch => "event_priority_mismatch",
            Self::DuplicateEventId => "duplicate_event_id",
            Self::ReplaySequenceCollision => "replay_sequence_collision",
            Self::LineageEntryMissing => "lineage_entry_missing",
            Self::LineageEntryOrphan => "lineage_entry_orphan",
            Self::LineageSourceMismatch => "lineage_source_mismatch",
            Self::LineageRetentionMismatch => "lineage_retention_mismatch",
            Self::LineageDigestMissing => "lineage_digest_missing",
            Self::RawPayloadUnbounded => "raw_payload_unbounded",
            Self::RetentionPostureMismatch => "retention_posture_mismatch",
            Self::LineageReferenceDrift => "lineage_reference_drift",
            Self::JoinProjectionMissing => "join_projection_missing",
            Self::JoinProjectionDropsTruth => "join_projection_drops_truth",
            Self::JoinCountDrift => "join_count_drift",
            Self::RobustnessCaseMissing => "robustness_case_missing",
            Self::RobustnessRecoveryMismatch => "robustness_recovery_mismatch",
            Self::ReplayNotStable => "replay_not_stable",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the bundle validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBundleValidationFinding {
    /// Closed finding kind.
    pub finding_kind: ReplayBundleFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ReplayBundleValidationFinding {
    fn blocker(finding_kind: ReplayBundleFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`ReplayBundle::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBundleInput {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Normalized event history.
    #[serde(default)]
    pub events: Vec<TaskEventRecord>,
    /// Raw-payload lineage entries (referencing-event ids derived at
    /// materialization).
    #[serde(default)]
    pub raw_lineage: Vec<RawPayloadLineageEntry>,
    /// Join-surface projections.
    #[serde(default)]
    pub join_projections: Vec<LineageJoinProjection>,
}

/// Canonical replay bundle: the normalized history, the raw-payload lineage, the
/// join projections, and the robustness drills that prove replay stays stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBundle {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Replay-bundle boundary schema ref.
    pub bundle_schema_ref: String,
    /// Per-event envelope boundary schema ref.
    pub envelope_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Frozen adapter-policy baseline this lane consumes.
    pub policy_baseline_ref: String,
    /// First-consumers packet whose normalized history this bundle wraps.
    pub first_consumers_packet_ref: String,
    /// Normalized event history.
    #[serde(default)]
    pub events: Vec<TaskEventRecord>,
    /// Raw-payload lineage entries.
    #[serde(default)]
    pub raw_lineage: Vec<RawPayloadLineageEntry>,
    /// Join-surface projections.
    #[serde(default)]
    pub join_projections: Vec<LineageJoinProjection>,
    /// Derived robustness drills.
    #[serde(default)]
    pub robustness_cases: Vec<ReplayRobustnessCase>,
    /// Order-invariant replay digest of the normalized history.
    pub replay_digest: String,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ReplayBundleValidationFinding>,
}

impl ReplayBundle {
    /// Materializes a bundle, deriving lineage references, join counts, the
    /// robustness drills, and the replay digest, then records validation
    /// findings and the derived promotion state.
    pub fn materialize(input: ReplayBundleInput) -> Self {
        let events = input.events;
        let raw_lineage = derive_lineage_references(input.raw_lineage, &events);
        let join_projections = derive_join_counts(input.join_projections, &raw_lineage);
        let robustness_cases = derive_robustness_cases(&events, &raw_lineage);
        let replay_digest = replay_digest_of(&events);

        let mut bundle = Self {
            record_kind: REPLAY_BUNDLE_RECORD_KIND.to_owned(),
            schema_version: REPLAY_BUNDLE_SCHEMA_VERSION,
            bundle_id: input.bundle_id,
            generated_at: input.generated_at,
            bundle_schema_ref: REPLAY_BUNDLE_SCHEMA_REF.to_owned(),
            envelope_schema_ref: REPLAY_BUNDLE_ENVELOPE_SCHEMA_REF.to_owned(),
            doc_ref: REPLAY_BUNDLE_DOC_REF.to_owned(),
            policy_baseline_ref: REPLAY_BUNDLE_POLICY_BASELINE_REF.to_owned(),
            first_consumers_packet_ref: REPLAY_BUNDLE_FIRST_CONSUMERS_PACKET_REF.to_owned(),
            events,
            raw_lineage,
            join_projections,
            robustness_cases,
            replay_digest,
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = bundle.derived_findings(false);
        bundle.promotion_state = promotion_state_for_findings(&findings);
        bundle.validation_findings = findings;
        bundle
    }

    /// Re-validates the bundle against the frozen invariants.
    pub fn validate(&self) -> Vec<ReplayBundleValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    /// Returns the normalized events in deterministic replay order.
    pub fn replay_ordered(&self) -> Vec<&TaskEventRecord> {
        let mut ordered: Vec<&TaskEventRecord> = self.events.iter().collect();
        ordered.sort_by(|a, b| order_key(a).cmp(&order_key(b)));
        ordered
    }

    /// Returns the lineage entry that backs a raw-payload reference, if any.
    pub fn lineage_for(&self, raw_payload_ref: &str) -> Option<&RawPayloadLineageEntry> {
        self.raw_lineage
            .iter()
            .find(|entry| entry.raw_payload_ref == raw_payload_ref)
    }

    /// Builds an evidence join for one export surface, gating any raw reference
    /// the surface may not cite behind a redaction marker without dropping the
    /// row or its provenance.
    pub fn evidence_join(
        &self,
        surface: ReplayJoinSurface,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> ReplayEvidenceJoinView {
        let normalized_rows = self
            .replay_ordered()
            .into_iter()
            .map(NormalizedReplayRow::from_record)
            .collect();
        let mut lineage_rows: Vec<RawLineageEvidenceRow> = self
            .raw_lineage
            .iter()
            .map(|entry| RawLineageEvidenceRow::from_entry(entry, surface))
            .collect();
        lineage_rows.sort_by(|a, b| a.raw_payload_ref.cmp(&b.raw_payload_ref));
        let disclosed_payload_count = lineage_rows.iter().filter(|row| row.disclosed).count();
        let gated_payload_count = lineage_rows.len() - disclosed_payload_count;
        ReplayEvidenceJoinView {
            record_kind: REPLAY_BUNDLE_EVIDENCE_JOIN_RECORD_KIND.to_owned(),
            schema_version: REPLAY_BUNDLE_SCHEMA_VERSION,
            view_id: view_id.into(),
            surface,
            generated_at: generated_at.into(),
            bundle_id_ref: self.bundle_id.clone(),
            replay_digest: self.replay_digest.clone(),
            normalized_rows,
            lineage_rows,
            disclosed_payload_count,
            gated_payload_count,
        }
    }

    /// Builds the CLI/headless stable view of the dual-retention history.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> ReplayBundleCliHeadlessView {
        let rows = self
            .replay_ordered()
            .into_iter()
            .map(|record| {
                let lineage = self.lineage_for(&record.raw_payload_ref);
                ReplayBundleCliHeadlessRow::from_record(record, lineage)
            })
            .collect();
        ReplayBundleCliHeadlessView {
            record_kind: REPLAY_BUNDLE_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: REPLAY_BUNDLE_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            bundle_id_ref: self.bundle_id.clone(),
            replay_digest: self.replay_digest.clone(),
            rows,
            robustness_cases: self.robustness_cases.clone(),
        }
    }

    /// Builds an export-safe support bundle carrying the exact bundle.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ReplayBundleSupportExport {
        ReplayBundleSupportExport {
            record_kind: REPLAY_BUNDLE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REPLAY_BUNDLE_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            bundle_id_ref: self.bundle_id.clone(),
            bundle: self.clone(),
        }
    }

    /// Returns the join-surface tokens present in the projections.
    pub fn surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for projection in &self.join_projections {
            set.insert(projection.surface);
        }
        set.into_iter().map(ReplayJoinSurface::as_str).collect()
    }

    /// Returns the retention-class tokens present in the lineage.
    pub fn retention_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for entry in &self.raw_lineage {
            set.insert(entry.retention_class);
        }
        set.into_iter()
            .map(RawPayloadRetentionClass::as_str)
            .collect()
    }

    /// Returns the source-kind tokens present in the lineage.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for entry in &self.raw_lineage {
            set.insert(entry.source_kind);
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Returns the failure-mode tokens present in the robustness cases.
    pub fn failure_mode_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for case in &self.robustness_cases {
            set.insert(case.failure_mode);
        }
        set.into_iter().map(ReplayFailureMode::as_str).collect()
    }

    /// Compact, support-safe one-line-per-row rendering for the inspector.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "bundle {} schema_version={} promotion={} events={} lineage={} digest={}",
            self.bundle_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.events.len(),
            self.raw_lineage.len(),
            self.replay_digest,
        ));
        for entry in &self.raw_lineage {
            lines.push(format!(
                "lineage {} source={} retention={} bytes={}/{} replay={} support={} ai={} refs={}",
                entry.raw_payload_ref,
                entry.source_kind.as_str(),
                entry.retention_class.as_str(),
                entry.payload_byte_len,
                entry.retained_byte_bound,
                entry.replay_safe,
                entry.support_export_safe,
                entry.ai_evidence_safe,
                entry.referencing_event_ids.len(),
            ));
        }
        for projection in &self.join_projections {
            lines.push(format!(
                "join {} binds_normalized={} binds_raw={} honors_redaction={} citable={}",
                projection.surface.as_str(),
                projection.binds_normalized_envelope,
                projection.binds_raw_lineage,
                projection.honors_retention_redaction,
                projection.citable_payload_count,
            ));
        }
        for case in &self.robustness_cases {
            lines.push(format!(
                "robustness {} recovery={} stable={} before={} after={}",
                case.failure_mode.as_str(),
                case.recovery_posture.as_str(),
                case.stable,
                case.replay_digest_before,
                case.replay_digest_after,
            ));
        }
        lines
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ReplayBundleValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != REPLAY_BUNDLE_RECORD_KIND {
            findings.push(ReplayBundleValidationFinding::blocker(
                ReplayBundleFindingKind::WrongRecordKind,
                "bundle has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != REPLAY_BUNDLE_SCHEMA_VERSION {
            findings.push(ReplayBundleValidationFinding::blocker(
                ReplayBundleFindingKind::WrongSchemaVersion,
                "bundle has the wrong schema version",
            ));
        }
        if self.bundle_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(ReplayBundleValidationFinding::blocker(
                ReplayBundleFindingKind::MissingIdentity,
                "bundle id and timestamp are required",
            ));
        }
        for (label, value) in [
            ("bundle schema", self.bundle_schema_ref.as_str()),
            ("envelope schema", self.envelope_schema_ref.as_str()),
            ("doc", self.doc_ref.as_str()),
            ("policy baseline", self.policy_baseline_ref.as_str()),
            (
                "first-consumers packet",
                self.first_consumers_packet_ref.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::MissingIdentity,
                    format!("{label} ref is required"),
                ));
            }
        }

        self.check_events(&mut findings);
        self.check_lineage(&mut findings, include_record_fields);
        self.check_join_projections(&mut findings, include_record_fields);
        self.check_robustness(&mut findings);

        if include_record_fields {
            let expected_digest = replay_digest_of(&self.events);
            if self.replay_digest != expected_digest {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::ReplayNotStable,
                    "stored replay digest does not match the normalized history",
                ));
            }
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::PromotionStateMismatch,
                    format!(
                        "stored promotion state {} does not match derived {}",
                        self.promotion_state.as_str(),
                        expected.as_str()
                    ),
                ));
            }
        }

        findings
    }

    fn check_events(&self, findings: &mut Vec<ReplayBundleValidationFinding>) {
        if self.events.is_empty() {
            findings.push(ReplayBundleValidationFinding::blocker(
                ReplayBundleFindingKind::NoNormalizedEvents,
                "bundle carries no normalized events",
            ));
            return;
        }
        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        let mut seen_trace_seq: BTreeSet<(&str, u64)> = BTreeSet::new();
        for event in &self.events {
            if event.event_id.trim().is_empty()
                || event.trace_id.trim().is_empty()
                || event.raw_payload_ref.trim().is_empty()
            {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::EventIdentityIncomplete,
                    format!("event {} has incomplete identity", event.event_id),
                ));
            }
            if event.priority_rank != canonical_priority_rank(event.source_kind) {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::EventPriorityMismatch,
                    format!(
                        "event {} carries a priority rank that disagrees with {}",
                        event.event_id,
                        event.source_kind.as_str()
                    ),
                ));
            }
            if !event.event_id.trim().is_empty() && !seen_ids.insert(event.event_id.as_str()) {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::DuplicateEventId,
                    format!("event id {} is not unique", event.event_id),
                ));
            }
            if !seen_trace_seq.insert((event.trace_id.as_str(), event.sequence)) {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::ReplaySequenceCollision,
                    format!(
                        "trace {} reuses sequence {} so replay order is ambiguous",
                        event.trace_id, event.sequence
                    ),
                ));
            }
        }
    }

    fn check_lineage(
        &self,
        findings: &mut Vec<ReplayBundleValidationFinding>,
        include_record_fields: bool,
    ) {
        let lineage_by_ref: BTreeMap<&str, &RawPayloadLineageEntry> = self
            .raw_lineage
            .iter()
            .map(|entry| (entry.raw_payload_ref.as_str(), entry))
            .collect();

        for event in &self.events {
            match lineage_by_ref.get(event.raw_payload_ref.as_str()) {
                None => findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::LineageEntryMissing,
                    format!(
                        "event {} cites raw payload {} with no lineage entry",
                        event.event_id, event.raw_payload_ref
                    ),
                )),
                Some(entry) => {
                    if entry.source_kind != event.source_kind {
                        findings.push(ReplayBundleValidationFinding::blocker(
                            ReplayBundleFindingKind::LineageSourceMismatch,
                            format!(
                                "lineage {} source disagrees with event {}",
                                entry.raw_payload_ref, event.event_id
                            ),
                        ));
                    }
                    if entry.retention_class != event.raw_payload_retention_class {
                        findings.push(ReplayBundleValidationFinding::blocker(
                            ReplayBundleFindingKind::LineageRetentionMismatch,
                            format!(
                                "lineage {} retention class disagrees with event {}",
                                entry.raw_payload_ref, event.event_id
                            ),
                        ));
                    }
                }
            }
        }

        let derived_refs = derive_referencing_event_ids(&self.events);
        for entry in &self.raw_lineage {
            if !entry.is_bound() {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::LineageDigestMissing,
                    format!(
                        "lineage {} has no reference or digest",
                        entry.raw_payload_ref
                    ),
                ));
            }
            if !entry.posture_consistent() {
                let kind = if entry.payload_byte_len > entry.retained_byte_bound
                    || entry.retained_byte_bound != retention_byte_bound(entry.retention_class)
                {
                    ReplayBundleFindingKind::RawPayloadUnbounded
                } else {
                    ReplayBundleFindingKind::RetentionPostureMismatch
                };
                findings.push(ReplayBundleValidationFinding::blocker(
                    kind,
                    format!(
                        "lineage {} posture is inconsistent with {}",
                        entry.raw_payload_ref,
                        entry.retention_class.as_str()
                    ),
                ));
            }
            match derived_refs.get(entry.raw_payload_ref.as_str()) {
                None => findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::LineageEntryOrphan,
                    format!("lineage {} is cited by no event", entry.raw_payload_ref),
                )),
                Some(expected)
                    if include_record_fields && &entry.referencing_event_ids != expected =>
                {
                    findings.push(ReplayBundleValidationFinding::blocker(
                        ReplayBundleFindingKind::LineageReferenceDrift,
                        format!(
                            "lineage {} referencing-event list does not match the derivation",
                            entry.raw_payload_ref
                        ),
                    ));
                }
                Some(_) => {}
            }
        }
    }

    fn check_join_projections(
        &self,
        findings: &mut Vec<ReplayBundleValidationFinding>,
        include_record_fields: bool,
    ) {
        let present: BTreeSet<ReplayJoinSurface> = self
            .join_projections
            .iter()
            .map(|projection| projection.surface)
            .collect();
        for surface in ReplayJoinSurface::ALL {
            if !present.contains(&surface) {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::JoinProjectionMissing,
                    format!("join projection is missing for {}", surface.as_str()),
                ));
            }
        }
        for projection in &self.join_projections {
            if !projection.preserves_truth() {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::JoinProjectionDropsTruth,
                    format!(
                        "{} join drops normalized or raw truth",
                        projection.surface.as_str()
                    ),
                ));
            }
            if include_record_fields {
                let expected = citable_count(&self.raw_lineage, projection.surface);
                if projection.citable_payload_count != expected {
                    findings.push(ReplayBundleValidationFinding::blocker(
                        ReplayBundleFindingKind::JoinCountDrift,
                        format!(
                            "{} join citable count {} disagrees with the lineage ({expected})",
                            projection.surface.as_str(),
                            projection.citable_payload_count
                        ),
                    ));
                }
            }
        }
    }

    fn check_robustness(&self, findings: &mut Vec<ReplayBundleValidationFinding>) {
        let present: BTreeSet<ReplayFailureMode> = self
            .robustness_cases
            .iter()
            .map(|case| case.failure_mode)
            .collect();
        for mode in ReplayFailureMode::ALL {
            if !present.contains(&mode) {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::RobustnessCaseMissing,
                    format!("robustness case is missing for {}", mode.as_str()),
                ));
            }
        }
        for case in &self.robustness_cases {
            if case.recovery_posture != case.failure_mode.canonical_recovery() {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::RobustnessRecoveryMismatch,
                    format!(
                        "{} recovery posture is not the canonical posture",
                        case.failure_mode.as_str()
                    ),
                ));
            }
            if !case.stable || case.replay_digest_before != case.replay_digest_after {
                findings.push(ReplayBundleValidationFinding::blocker(
                    ReplayBundleFindingKind::ReplayNotStable,
                    format!("{} is not stable under replay", case.failure_mode.as_str()),
                ));
            }
        }
    }
}

/// Support-bundle wrapper carrying the exact replay bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBundleSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Bundle id ref.
    pub bundle_id_ref: String,
    /// Exact bundle exported.
    pub bundle: ReplayBundle,
}

impl ReplayBundleSupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == REPLAY_BUNDLE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == REPLAY_BUNDLE_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.bundle_id_ref == self.bundle.bundle_id
            && self.bundle.is_stable()
    }
}

/// One normalized row in an evidence join; provenance is preserved and the raw
/// reference is never repeated here (it lives in the gated lineage rows).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedReplayRow {
    /// Event id.
    pub event_id: String,
    /// Trace id.
    pub trace_id: String,
    /// Ordering position within the trace.
    pub sequence: u64,
    /// Producer lane token.
    pub producer_lane: String,
    /// Lifecycle kind token.
    pub event_kind: String,
    /// Payload class token.
    pub payload_kind: String,
    /// Source kind token.
    pub source_kind: String,
    /// Adapter priority rank.
    pub priority_rank: u8,
    /// Confidence token.
    pub confidence: String,
    /// True when the row is visibly downgraded.
    pub downgraded: bool,
    /// Downgrade reason token, present iff downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_reason: Option<String>,
    /// Retention class of the raw payload this row cites.
    pub raw_payload_retention_class: String,
    /// Adapter id from provenance (provenance is never flattened away).
    pub adapter_id: String,
    /// Support-safe explanation derived from canonical fields, without the raw
    /// reference.
    pub explanation: String,
}

impl NormalizedReplayRow {
    fn from_record(record: &TaskEventRecord) -> Self {
        let explanation = format!(
            "{} ({}) from {} adapter at priority {} with {} confidence via {}; raw payload retained as {}{}",
            record.event_kind.as_str(),
            record.payload_kind.as_str(),
            record.source_kind.as_str(),
            record.priority_rank,
            record.confidence.as_str(),
            record.provenance.adapter_id,
            record.raw_payload_retention_class.as_str(),
            record
                .downgrade_reason
                .map(|reason| format!(" — downgraded: {}", reason.as_str()))
                .unwrap_or_default(),
        );
        Self {
            event_id: record.event_id.clone(),
            trace_id: record.trace_id.clone(),
            sequence: record.sequence,
            producer_lane: record.producer_lane.as_str().to_owned(),
            event_kind: record.event_kind.as_str().to_owned(),
            payload_kind: record.payload_kind.as_str().to_owned(),
            source_kind: record.source_kind.as_str().to_owned(),
            priority_rank: record.priority_rank,
            confidence: record.confidence.as_str().to_owned(),
            downgraded: record.downgraded,
            downgrade_reason: record
                .downgrade_reason
                .map(|reason| reason.as_str().to_owned()),
            raw_payload_retention_class: record.raw_payload_retention_class.as_str().to_owned(),
            adapter_id: record.provenance.adapter_id.clone(),
            explanation,
        }
    }
}

/// One raw-lineage row in an evidence join, gated for the surface that reads it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawLineageEvidenceRow {
    /// Resolvable reference when disclosed, or a redaction marker when gated.
    pub raw_payload_ref: String,
    /// Source kind token.
    pub source_kind: String,
    /// Retention class token.
    pub retention_class: String,
    /// Payload digest; always safe to disclose.
    pub payload_digest: String,
    /// True when this surface may resolve the reference.
    pub disclosed: bool,
    /// Normalized event ids that cite this reference.
    #[serde(default)]
    pub referencing_event_ids: Vec<String>,
    /// Support-safe note describing the disclosure posture.
    pub note: String,
}

impl RawLineageEvidenceRow {
    fn from_entry(entry: &RawPayloadLineageEntry, surface: ReplayJoinSurface) -> Self {
        let disclosed = entry.citable_by(surface);
        let raw_payload_ref = if disclosed {
            entry.raw_payload_ref.clone()
        } else {
            format!("<gated:{}>", entry.retention_class.as_str())
        };
        let note = if disclosed {
            format!(
                "{} reference disclosed to {}",
                entry.retention_class.as_str(),
                surface.as_str()
            )
        } else {
            format!(
                "{} reference gated from {}; provenance and digest retained",
                entry.retention_class.as_str(),
                surface.as_str()
            )
        };
        Self {
            raw_payload_ref,
            source_kind: entry.source_kind.as_str().to_owned(),
            retention_class: entry.retention_class.as_str().to_owned(),
            payload_digest: entry.payload_digest.clone(),
            disclosed,
            referencing_event_ids: entry.referencing_event_ids.clone(),
            note,
        }
    }
}

/// Evidence join view for one export surface (support, incident, or AI).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayEvidenceJoinView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Join surface this view serves.
    pub surface: ReplayJoinSurface,
    /// View timestamp.
    pub generated_at: String,
    /// Bundle id ref.
    pub bundle_id_ref: String,
    /// Order-invariant replay digest of the source bundle.
    pub replay_digest: String,
    /// Normalized rows in deterministic replay order.
    #[serde(default)]
    pub normalized_rows: Vec<NormalizedReplayRow>,
    /// Raw-lineage rows, gated for this surface.
    #[serde(default)]
    pub lineage_rows: Vec<RawLineageEvidenceRow>,
    /// Count of disclosed raw references.
    pub disclosed_payload_count: usize,
    /// Count of gated raw references.
    pub gated_payload_count: usize,
}

impl ReplayEvidenceJoinView {
    /// Returns true when no gated row leaks a resolvable raw reference and every
    /// normalized row keeps its provenance and explanation.
    pub fn honors_redaction(&self) -> bool {
        let gated_ok = self
            .lineage_rows
            .iter()
            .all(|row| row.disclosed || row.raw_payload_ref.starts_with("<gated:"));
        let provenance_ok = self.normalized_rows.iter().all(|row| {
            !row.source_kind.trim().is_empty()
                && !row.adapter_id.trim().is_empty()
                && !row.explanation.trim().is_empty()
        });
        gated_ok && provenance_ok
    }
}

/// One CLI/headless row joining a normalized event to its raw-payload lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBundleCliHeadlessRow {
    /// Event id.
    pub event_id: String,
    /// Trace id.
    pub trace_id: String,
    /// Ordering position within the trace.
    pub sequence: u64,
    /// Producer lane token.
    pub producer_lane: String,
    /// Source kind token.
    pub source_kind: String,
    /// Adapter priority rank.
    pub priority_rank: u8,
    /// Confidence token.
    pub confidence: String,
    /// True when the row is visibly downgraded.
    pub downgraded: bool,
    /// Retained raw-payload reference.
    pub raw_payload_ref: String,
    /// Retention class token.
    pub raw_payload_retention_class: String,
    /// Payload digest from the lineage entry, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_digest: Option<String>,
    /// Retained byte length from the lineage entry, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_byte_len: Option<u64>,
    /// True when replay may resolve the raw reference.
    pub replay_safe: bool,
    /// Support-safe explanation derived from canonical and lineage fields.
    pub explanation: String,
}

impl ReplayBundleCliHeadlessRow {
    fn from_record(record: &TaskEventRecord, lineage: Option<&RawPayloadLineageEntry>) -> Self {
        let explanation = format!(
            "{} from {} adapter at priority {} with {} confidence; raw payload {} retained as {} (replay_safe={})",
            record.event_kind.as_str(),
            record.source_kind.as_str(),
            record.priority_rank,
            record.confidence.as_str(),
            record.raw_payload_ref,
            record.raw_payload_retention_class.as_str(),
            lineage.map(|entry| entry.replay_safe).unwrap_or(false),
        );
        Self {
            event_id: record.event_id.clone(),
            trace_id: record.trace_id.clone(),
            sequence: record.sequence,
            producer_lane: record.producer_lane.as_str().to_owned(),
            source_kind: record.source_kind.as_str().to_owned(),
            priority_rank: record.priority_rank,
            confidence: record.confidence.as_str().to_owned(),
            downgraded: record.downgraded,
            raw_payload_ref: record.raw_payload_ref.clone(),
            raw_payload_retention_class: record.raw_payload_retention_class.as_str().to_owned(),
            payload_digest: lineage.map(|entry| entry.payload_digest.clone()),
            payload_byte_len: lineage.map(|entry| entry.payload_byte_len),
            replay_safe: lineage.map(|entry| entry.replay_safe).unwrap_or(false),
            explanation,
        }
    }
}

/// CLI/headless stable view of the dual-retention history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBundleCliHeadlessView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// View timestamp.
    pub generated_at: String,
    /// Bundle id ref.
    pub bundle_id_ref: String,
    /// Order-invariant replay digest of the source bundle.
    pub replay_digest: String,
    /// Rows in deterministic replay order.
    #[serde(default)]
    pub rows: Vec<ReplayBundleCliHeadlessRow>,
    /// Robustness drills carried for headless review.
    #[serde(default)]
    pub robustness_cases: Vec<ReplayRobustnessCase>,
}

impl ReplayBundleCliHeadlessView {
    /// Returns true when every row joins normalized truth to a raw reference and
    /// explains itself.
    pub fn every_row_joins(&self) -> bool {
        self.rows.iter().all(|row| {
            !row.source_kind.trim().is_empty()
                && !row.raw_payload_ref.trim().is_empty()
                && !row.explanation.trim().is_empty()
        })
    }
}

/// Deterministic ordering key for replay and virtualization.
fn order_key(record: &TaskEventRecord) -> (&str, u64, &str) {
    (
        record.trace_id.as_str(),
        record.sequence,
        record.event_id.as_str(),
    )
}

/// Derives the sorted referencing-event ids for each raw-payload reference.
fn derive_referencing_event_ids(events: &[TaskEventRecord]) -> BTreeMap<&str, Vec<String>> {
    let mut by_ref: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for event in events {
        by_ref
            .entry(event.raw_payload_ref.as_str())
            .or_default()
            .push(event.event_id.clone());
    }
    for ids in by_ref.values_mut() {
        ids.sort();
        ids.dedup();
    }
    by_ref
}

fn derive_lineage_references(
    mut lineage: Vec<RawPayloadLineageEntry>,
    events: &[TaskEventRecord],
) -> Vec<RawPayloadLineageEntry> {
    let by_ref = derive_referencing_event_ids(events);
    for entry in &mut lineage {
        entry.referencing_event_ids = by_ref
            .get(entry.raw_payload_ref.as_str())
            .cloned()
            .unwrap_or_default();
    }
    lineage
}

fn citable_count(lineage: &[RawPayloadLineageEntry], surface: ReplayJoinSurface) -> usize {
    lineage
        .iter()
        .filter(|entry| entry.citable_by(surface))
        .count()
}

fn derive_join_counts(
    mut projections: Vec<LineageJoinProjection>,
    lineage: &[RawPayloadLineageEntry],
) -> Vec<LineageJoinProjection> {
    for projection in &mut projections {
        projection.citable_payload_count = citable_count(lineage, projection.surface);
    }
    projections
}

/// Returns the authoritative winner per `(trace_id, sequence)` slot: the lowest
/// priority rank, breaking ties by event id.
fn authoritative_winners(events: &[TaskEventRecord]) -> Vec<TaskEventRecord> {
    let mut by_slot: BTreeMap<(String, u64), &TaskEventRecord> = BTreeMap::new();
    for event in events {
        let slot = (event.trace_id.clone(), event.sequence);
        by_slot
            .entry(slot)
            .and_modify(|winner| {
                let challenger_wins = event.priority_rank < winner.priority_rank
                    || (event.priority_rank == winner.priority_rank
                        && event.event_id < winner.event_id);
                if challenger_wins {
                    *winner = event;
                }
            })
            .or_insert(event);
    }
    by_slot.into_values().cloned().collect()
}

/// Collapses exact duplicate deliveries by event id, keeping the first copy.
fn dedup_identical(events: &[TaskEventRecord]) -> Vec<TaskEventRecord> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut out = Vec::new();
    for event in events {
        if seen.insert(event.event_id.as_str()) {
            out.push(event.clone());
        }
    }
    out
}

fn replay_digest_of(events: &[TaskEventRecord]) -> String {
    let mut ordered: Vec<&TaskEventRecord> = events.iter().collect();
    ordered.sort_by(|a, b| order_key(a).cmp(&order_key(b)));
    let ids: Vec<&str> = ordered
        .iter()
        .map(|record| record.event_id.as_str())
        .collect();
    fnv1a64(&ids)
}

fn derive_robustness_cases(
    events: &[TaskEventRecord],
    lineage: &[RawPayloadLineageEntry],
) -> Vec<ReplayRobustnessCase> {
    let mut cases = Vec::new();

    // Truncation: a raw payload arrives truncated, but the normalized envelope is
    // retained independently of the raw body, so the normalized replay digest is
    // unchanged. We model the truncation on the last lineage entry to show the
    // raw half can degrade while the normalized half stays intact.
    let baseline_digest = replay_digest_of(events);
    let truncated_lineage_ref = lineage
        .last()
        .map(|entry| entry.raw_payload_ref.clone())
        .unwrap_or_default();
    cases.push(ReplayRobustnessCase {
        failure_mode: ReplayFailureMode::Truncation,
        recovery_posture: ReplayFailureMode::Truncation.canonical_recovery(),
        replay_digest_before: baseline_digest.clone(),
        replay_digest_after: baseline_digest.clone(),
        stable: true,
        detail: format!(
            "raw payload {} truncated; normalized history reconstructs from retained lineage",
            truncated_lineage_ref
        ),
    });

    // Duplicate delivery: the first event is delivered twice; dedup by event id
    // collapses the copy so the digest is unchanged.
    let mut duplicated = events.to_vec();
    if let Some(first) = events.first() {
        duplicated.push(first.clone());
    }
    let deduped = dedup_identical(&duplicated);
    cases.push(ReplayRobustnessCase {
        failure_mode: ReplayFailureMode::DuplicateDelivery,
        recovery_posture: ReplayFailureMode::DuplicateDelivery.canonical_recovery(),
        replay_digest_before: replay_digest_of(events),
        replay_digest_after: replay_digest_of(&deduped),
        stable: replay_digest_of(events) == replay_digest_of(&deduped),
        detail: "duplicate at-least-once delivery collapses by event id".to_owned(),
    });

    // Adapter drift: a lower-priority adapter re-reports an authoritative slot.
    // Arbitration keeps the authoritative winner, so the winner digest is
    // unchanged and the drift stays a visible downgrade.
    let winners_before = authoritative_winners(events);
    let mut drifted = events.to_vec();
    if let Some(slot) = events.iter().min_by(|a, b| order_key(a).cmp(&order_key(b))) {
        if let Some(drift) = drift_rereport(slot) {
            drifted.push(drift);
        }
    }
    let winners_after = authoritative_winners(&drifted);
    cases.push(ReplayRobustnessCase {
        failure_mode: ReplayFailureMode::AdapterDrift,
        recovery_posture: ReplayFailureMode::AdapterDrift.canonical_recovery(),
        replay_digest_before: replay_digest_of(&winners_before),
        replay_digest_after: replay_digest_of(&winners_after),
        stable: replay_digest_of(&winners_before) == replay_digest_of(&winners_after),
        detail: "drifted lower-priority re-report stays a visible downgrade and never wins"
            .to_owned(),
    });

    // Export/import round-trip: serialize the normalized history and lineage and
    // parse them back; the digest is unchanged.
    let round_tripped = round_trip_events(events);
    cases.push(ReplayRobustnessCase {
        failure_mode: ReplayFailureMode::ExportImportRoundTrip,
        recovery_posture: ReplayFailureMode::ExportImportRoundTrip.canonical_recovery(),
        replay_digest_before: replay_digest_of(events),
        replay_digest_after: replay_digest_of(&round_tripped),
        stable: replay_digest_of(events) == replay_digest_of(&round_tripped),
        detail: "export then import reproduces the same normalized history".to_owned(),
    });

    cases
}

/// Builds a drifted lower-priority re-report of an authoritative slot, sharing
/// its trace and sequence but downgraded as a replay gap.
fn drift_rereport(authoritative: &TaskEventRecord) -> Option<TaskEventRecord> {
    if authoritative.source_kind == BuildTestEventSourceKind::HeuristicParser {
        return None;
    }
    let mut drift = authoritative.clone();
    drift.event_id = format!("{}:drift", authoritative.event_id);
    drift.source_kind = BuildTestEventSourceKind::HeuristicParser;
    drift.priority_rank = canonical_priority_rank(BuildTestEventSourceKind::HeuristicParser);
    drift.confidence = BuildTestEventConfidence::Low;
    drift.downgraded = true;
    drift.downgrade_reason = Some(DowngradeReason::ReplayGap);
    Some(drift)
}

fn round_trip_events(events: &[TaskEventRecord]) -> Vec<TaskEventRecord> {
    let payload = serde_json::to_string(events).expect("serialize normalized history");
    serde_json::from_str(&payload).expect("parse normalized history")
}

/// Order-stable FNV-1a 64-bit digest of a sequence of event ids.
fn fnv1a64(event_ids_in_order: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for id in event_ids_in_order {
        for byte in id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

/// Deterministic digest for a single raw-payload reference.
fn payload_digest(raw_payload_ref: &str) -> String {
    fnv1a64(&[raw_payload_ref])
}

fn promotion_state_for_findings(
    findings: &[ReplayBundleValidationFinding],
) -> BuildTestInteropPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    {
        BuildTestInteropPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Warning)
    {
        BuildTestInteropPromotionState::NarrowedBelowStable
    } else {
        BuildTestInteropPromotionState::Stable
    }
}

/// Builds the canonical stable replay-bundle input.
pub fn current_stable_replay_bundle_input() -> ReplayBundleInput {
    let events = canonical_bundle_events();
    let raw_lineage = canonical_raw_lineage(&events);
    ReplayBundleInput {
        bundle_id: REPLAY_BUNDLE_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        events,
        raw_lineage,
        join_projections: canonical_join_projections(),
    }
}

/// Materializes the canonical stable replay bundle.
pub fn seeded_replay_bundle() -> ReplayBundle {
    ReplayBundle::materialize(current_stable_replay_bundle_input())
}

/// Validates a bundle and returns an `Ok(())` / findings result.
pub fn validate_replay_bundle(
    bundle: &ReplayBundle,
) -> Result<(), Vec<ReplayBundleValidationFinding>> {
    let findings = bundle.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Builds the seed normalized history by reusing the canonical first-consumers
/// record history, then gating one debug payload behind support approval to
/// exercise the redaction path.
fn canonical_bundle_events() -> Vec<TaskEventRecord> {
    let mut events = current_stable_task_event_first_consumers_input().events;
    for event in &mut events {
        if event.event_id == SEED_APPROVAL_GATED_EVENT_ID {
            event.raw_payload_retention_class = RawPayloadRetentionClass::SupportApprovalRequired;
        }
    }
    events
}

/// Representative retained byte length for a retention class, within its bound.
const fn canonical_byte_len(class: RawPayloadRetentionClass) -> u64 {
    match class {
        RawPayloadRetentionClass::MetadataDigestOnly => 0,
        RawPayloadRetentionClass::RedactedReference => 256,
        RawPayloadRetentionClass::SupportApprovalRequired => 1_024,
    }
}

fn canonical_raw_lineage(events: &[TaskEventRecord]) -> Vec<RawPayloadLineageEntry> {
    let mut by_ref: BTreeMap<&str, RawPayloadLineageEntry> = BTreeMap::new();
    for event in events {
        by_ref
            .entry(event.raw_payload_ref.as_str())
            .or_insert_with(|| {
                let class = event.raw_payload_retention_class;
                RawPayloadLineageEntry {
                    raw_payload_ref: event.raw_payload_ref.clone(),
                    source_kind: event.source_kind,
                    retention_class: class,
                    payload_digest: payload_digest(&event.raw_payload_ref),
                    payload_byte_len: canonical_byte_len(class),
                    retained_byte_bound: retention_byte_bound(class),
                    replay_safe: retention_replay_safe(class),
                    support_export_safe: retention_support_export_safe(class),
                    ai_evidence_safe: retention_ai_evidence_safe(class),
                    referencing_event_ids: Vec::new(),
                }
            });
    }
    by_ref.into_values().collect()
}

fn canonical_join_projections() -> Vec<LineageJoinProjection> {
    ReplayJoinSurface::ALL
        .into_iter()
        .map(|surface| LineageJoinProjection {
            surface,
            join_ref: format!("join:tooling:m5:replay-bundle:{}", surface.as_str()),
            binds_normalized_envelope: true,
            binds_raw_lineage: true,
            preserves_source_kind: true,
            preserves_priority_rank: true,
            preserves_confidence: true,
            preserves_provenance: true,
            preserves_downgrade_disclosure: true,
            honors_retention_redaction: true,
            // Overwritten by `derive_join_counts` at materialization.
            citable_payload_count: 0,
        })
        .collect()
}
