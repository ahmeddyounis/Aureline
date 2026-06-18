//! Claim-bearing tooling-profile certification matrix for build/test event
//! interoperability.
//!
//! The canonical build/test event interoperability packet
//! ([`crate::build_test_event_interoperability`]) freezes *one* event envelope,
//! the native-first adapter ladder, the confidence/raw-retention rules, and the
//! replay/export contracts. The conformance suite
//! ([`crate::m5_interop_conformance`]) re-runs that contract across the adapter
//! *families* and *archetypes*. This module closes the loop at the *consumer*
//! level: it turns those frozen contracts into a claim-bearing **tooling profile
//! matrix** so each M5 run/test/debug/pipeline/notebook surface can only *claim*
//! event-interoperability coherence when its own path is current and
//! machine-readable.
//!
//! Each claimed [`ToolingProfile`] carries one [`ToolingProfileCertification`]
//! graded on the eight certification [`CertificationDimension`]s the contract
//! requires:
//!
//! - [`CertificationDimension::EventEnvelopeReuse`] — the profile reads the
//!   canonical event envelope (and cites the upstream evidence packets) instead of
//!   a private session history, rendered-log scraping, unlabeled heuristic
//!   parsing, or a path with no raw lineage.
//! - [`CertificationDimension::AdapterHierarchy`] — a native-first capability
//!   handshake is evidenced.
//! - [`CertificationDimension::FallbackReason`] — a degraded/unsupported path
//!   names an explicit fallback reason; a negotiated path names none.
//! - [`CertificationDimension::ConfidencePreservation`] — the observed confidence
//!   does not overclaim its source.
//! - [`CertificationDimension::RawPayloadRetention`] — the raw payload is retained
//!   behind a reference and digest without leaking private material.
//! - [`CertificationDimension::ReplayStability`] — the profile replays
//!   deterministically from canonical envelopes.
//! - [`CertificationDimension::DegradedStateDisclosure`] — a degraded/unsupported
//!   capability is visibly disclosed.
//! - [`CertificationDimension::ExportParity`] — support/release/AI exports
//!   preserve source, confidence, and refs.
//!
//! A profile that fails any dimension *blocks stable*; a profile whose proof has
//! aged past its freshness window *narrows below stable* (a warning, not a
//! blocker) so a claim cannot coast on aged proof. The derived
//! [`CertificationIndex`] names which profiles are claimable, narrowed, or
//! blocked so release, support, AI, and docs/help surfaces can ingest one
//! canonical execution-truth index instead of re-deriving profile maturity by
//! hand.
//!
//! The packet deliberately reuses the source-kind, confidence, capability-state,
//! retention-class, promotion-state, and finding-severity vocabulary frozen in
//! [`crate::build_test_event_interoperability`]; it adds the profile/claim/index
//! layer and nothing that re-derives event truth.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/event-interop-certification.md`](../../../docs/m5/event-interop-certification.md);
//! the machine-readable boundary lives at
//! [`/schemas/tooling/m5-event-interop-certification.schema.json`](../../../schemas/tooling/m5-event-interop-certification.schema.json).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::build_test_event_interoperability::{
    AdapterCapabilityState, BuildTestEventConfidence, BuildTestEventSourceKind,
    BuildTestInteropFindingSeverity, BuildTestInteropPromotionState, RawPayloadRetentionClass,
};

/// Stable record-kind tag for [`EventInteropCertificationPacket`].
pub const EVENT_INTEROP_CERTIFICATION_RECORD_KIND: &str = "m5_event_interop_certification_packet";

/// Stable record-kind tag for [`EventInteropCertificationSupportExport`].
pub const EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_event_interop_certification_support_export";

/// Stable record-kind tag for [`EventInteropCertificationEvidenceJoinView`].
pub const EVENT_INTEROP_CERTIFICATION_EVIDENCE_JOIN_RECORD_KIND: &str =
    "m5_event_interop_certification_evidence_join";

/// Stable record-kind tag for [`EventInteropCertificationCliHeadlessView`].
pub const EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_event_interop_certification_cli_headless";

/// Integer schema version for the certification packet.
pub const EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the certification boundary schema.
pub const EVENT_INTEROP_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/tooling/m5-event-interop-certification.schema.json";

/// Repo-relative path of the per-event task-event envelope boundary schema.
pub const EVENT_INTEROP_CERTIFICATION_ENVELOPE_SCHEMA_REF: &str =
    "schemas/tooling/task-event-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const EVENT_INTEROP_CERTIFICATION_DOC_REF: &str = "docs/m5/event-interop-certification.md";

/// Repo-relative path of the frozen adapter-policy baseline this lane consumes.
pub const EVENT_INTEROP_CERTIFICATION_POLICY_BASELINE_REF: &str =
    "artifacts/m5/tooling/event-interop-baseline/baseline.json";

/// Repo-relative path of the build/test interop packet whose contract every
/// profile certifies against.
pub const EVENT_INTEROP_CERTIFICATION_INTEROP_PACKET_REF: &str =
    "artifacts/runtime/m4/build_test_event_interoperability_packet.json";

/// Repo-relative path of the conformance suite this lane sits above.
pub const EVENT_INTEROP_CERTIFICATION_CONFORMANCE_PACKET_REF: &str =
    "artifacts/m5/tooling/interop-conformance/packet.json";

/// Repo-relative path of the checked-in packet artifact.
pub const EVENT_INTEROP_CERTIFICATION_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/tooling/event-interop-certification/packet.json";

/// Logical certification-index binding ref minted by the seed.
pub const EVENT_INTEROP_CERTIFICATION_INDEX_REF: &str =
    "release-evidence:tooling:m5:event-interop-certification";

/// Stable packet id minted by the seed.
pub const EVENT_INTEROP_CERTIFICATION_ID: &str = "tooling:m5:event-interop-certification:v1";

/// Stable support-export id minted by the seed inspector.
pub const EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:tooling:m5:event-interop-certification";

/// Stable AI evidence join id minted by the seed inspector.
pub const EVENT_INTEROP_CERTIFICATION_AI_EVIDENCE_ID: &str =
    "ai-evidence:tooling:m5:event-interop-certification";

/// Stable incident packet join id minted by the seed inspector.
pub const EVENT_INTEROP_CERTIFICATION_INCIDENT_PACKET_ID: &str =
    "incident:tooling:m5:event-interop-certification";

/// Stable CLI/headless view id minted by the seed inspector.
pub const EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_ID: &str =
    "cli-headless:tooling:m5:event-interop-certification";

/// Canonical upstream evidence packets every claimed profile draws proof from.
///
/// Each ref points at a checked-in upstream artifact: the event-envelope
/// first-consumer bus, the native-first adapter negotiation baseline, the
/// adapter-confidence audit, the raw-plus-normalized replay bundle, the
/// cross-surface event-reuse proof, and the interop conformance suite.
pub const EVENT_INTEROP_CERTIFICATION_EVIDENCE_REFS: [&str; 6] = [
    "artifacts/m5/tooling/event-envelope-first-consumers/packet.json",
    "artifacts/m5/tooling/adapter-negotiation/baseline.json",
    "artifacts/m5/tooling/adapter-confidence-audit/packet.json",
    "artifacts/m5/tooling/raw-plus-normalized-replay/packet.json",
    "artifacts/m5/tooling/cross-surface-event-reuse/packet.json",
    "artifacts/m5/tooling/interop-conformance/packet.json",
];

/// One claimed M5 tooling profile that depends on the task-event contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolingProfile {
    /// Task center run history and reopen/rerun flows.
    TaskCenterRun,
    /// Test explorer sessions, inline results, and watch trees.
    TestSession,
    /// Debug sessions and chronology views.
    DebugSession,
    /// Pipeline overlays over imported / remote CI runs.
    PipelineOverlay,
    /// Notebook-backed runs and their cell execution history.
    NotebookRun,
    /// Coverage, flaky, and snapshot intelligence overlays.
    CoverageIntelligence,
}

impl ToolingProfile {
    /// Every claimed profile in stable declaration order.
    pub const ALL: [Self; 6] = [
        Self::TaskCenterRun,
        Self::TestSession,
        Self::DebugSession,
        Self::PipelineOverlay,
        Self::NotebookRun,
        Self::CoverageIntelligence,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskCenterRun => "task_center_run",
            Self::TestSession => "test_session",
            Self::DebugSession => "debug_session",
            Self::PipelineOverlay => "pipeline_overlay",
            Self::NotebookRun => "notebook_run",
            Self::CoverageIntelligence => "coverage_intelligence",
        }
    }
}

/// How a profile sources the execution truth it presents.
///
/// Only [`ConsumerTruthSource::CanonicalEventEnvelope`] is conformant; every other
/// variant names a way a profile can drift away from the shared event bus and
/// blocks the profile's interop claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerTruthSource {
    /// The profile reads the canonical task/test/debug event envelope.
    CanonicalEventEnvelope,
    /// The profile reconstructs truth from a forked private session history.
    PrivateSessionHistory,
    /// The profile re-parses truth from rendered logs.
    RenderedLogScraping,
    /// The profile relies on an unlabeled heuristic parser.
    UnlabeledHeuristicParsing,
    /// The profile presents events with no retained raw lineage.
    MissingRawLineage,
}

impl ConsumerTruthSource {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalEventEnvelope => "canonical_event_envelope",
            Self::PrivateSessionHistory => "private_session_history",
            Self::RenderedLogScraping => "rendered_log_scraping",
            Self::UnlabeledHeuristicParsing => "unlabeled_heuristic_parsing",
            Self::MissingRawLineage => "missing_raw_lineage",
        }
    }

    /// True when the profile reads the canonical event envelope.
    pub const fn is_conformant(self) -> bool {
        matches!(self, Self::CanonicalEventEnvelope)
    }
}

/// One certification dimension graded for every claimed profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// The profile reuses the canonical event envelope and cites its evidence.
    EventEnvelopeReuse,
    /// A native-first capability handshake is evidenced.
    AdapterHierarchy,
    /// A degraded/unsupported path names a fallback reason; a negotiated one none.
    FallbackReason,
    /// The observed confidence does not overclaim its source.
    ConfidencePreservation,
    /// The raw payload is retained behind a reference without leaking.
    RawPayloadRetention,
    /// The profile replays deterministically from canonical envelopes.
    ReplayStability,
    /// A degraded/unsupported capability is visibly disclosed.
    DegradedStateDisclosure,
    /// Support / release / AI exports preserve source, confidence, and refs.
    ExportParity,
}

impl CertificationDimension {
    /// Every graded dimension in stable declaration order.
    pub const ALL: [Self; 8] = [
        Self::EventEnvelopeReuse,
        Self::AdapterHierarchy,
        Self::FallbackReason,
        Self::ConfidencePreservation,
        Self::RawPayloadRetention,
        Self::ReplayStability,
        Self::DegradedStateDisclosure,
        Self::ExportParity,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventEnvelopeReuse => "event_envelope_reuse",
            Self::AdapterHierarchy => "adapter_hierarchy",
            Self::FallbackReason => "fallback_reason",
            Self::ConfidencePreservation => "confidence_preservation",
            Self::RawPayloadRetention => "raw_payload_retention",
            Self::ReplayStability => "replay_stability",
            Self::DegradedStateDisclosure => "degraded_state_disclosure",
            Self::ExportParity => "export_parity",
        }
    }
}

/// Derived freshness state for a profile's recorded proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessState {
    /// Proof age is within the freshness window.
    Current,
    /// Proof age has exceeded the freshness window (narrows below stable).
    Stale,
}

impl EvidenceFreshnessState {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
        }
    }
}

/// Derived claim state for one profile in the certification matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileClaimState {
    /// The profile is current and certified across every dimension.
    Claimable,
    /// The profile is certified but its proof has aged out (narrows below stable).
    NarrowedBelowStable,
    /// The profile fails a certification dimension and blocks stable.
    Blocked,
}

impl ProfileClaimState {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Claimable => "claimable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::Blocked => "blocked",
        }
    }
}

/// Evidence-join surface that presents the certification matrix across a boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationEvidenceSurface {
    /// Support bundle / support export.
    SupportBundle,
    /// Incident timeline packet.
    IncidentPacket,
    /// AI evidence packet.
    AiEvidence,
}

impl CertificationEvidenceSurface {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundle => "support_bundle",
            Self::IncidentPacket => "incident_packet",
            Self::AiEvidence => "ai_evidence",
        }
    }
}

/// Closed validation finding vocabulary for the certification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// A required tooling profile is absent.
    MissingProfile,
    /// Two certifications declare the same profile.
    DuplicateProfile,
    /// A profile sources truth from outside the canonical event envelope.
    EventEnvelopeNotReused,
    /// A profile cites no upstream evidence packet.
    MissingEvidenceRef,
    /// A profile evidences no native-first capability handshake.
    AdapterHierarchyMissing,
    /// A degraded/unsupported profile names no fallback reason.
    FallbackReasonMissing,
    /// A negotiated profile names a spurious fallback reason.
    FallbackReasonUnexpected,
    /// A profile overclaims confidence for its source or capability state.
    ConfidenceOverclaim,
    /// A profile does not retain its raw payload behind a safe reference.
    RawPayloadNotRetained,
    /// A profile does not replay deterministically.
    ReplayUnstable,
    /// A degraded/unsupported profile is not visibly disclosed.
    DegradedStateNotDisclosed,
    /// A profile breaks support/release/AI export parity.
    ExportParityBroken,
    /// Stored per-dimension outcomes disagree with the derivation.
    DimensionOutcomeDrift,
    /// Stored profile certified flag disagrees with the derivation.
    ProfileCertificationDrift,
    /// Stored profile claim state disagrees with the derivation.
    ProfileClaimStateDrift,
    /// Stored profile freshness state disagrees with the derivation.
    ProfileFreshnessDrift,
    /// A profile's recorded proof has aged past its freshness window.
    ProfileEvidenceStale,
    /// The certification-index binding ref is missing.
    CertificationIndexMissing,
    /// Stored certification index disagrees with the derivation.
    CertificationIndexDrift,
    /// Stored profile digest disagrees with the derivation.
    ProfileDigestDrift,
    /// Stored promotion state disagrees with the derivation.
    PromotionStateMismatch,
}

impl CertificationFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingProfile => "missing_profile",
            Self::DuplicateProfile => "duplicate_profile",
            Self::EventEnvelopeNotReused => "event_envelope_not_reused",
            Self::MissingEvidenceRef => "missing_evidence_ref",
            Self::AdapterHierarchyMissing => "adapter_hierarchy_missing",
            Self::FallbackReasonMissing => "fallback_reason_missing",
            Self::FallbackReasonUnexpected => "fallback_reason_unexpected",
            Self::ConfidenceOverclaim => "confidence_overclaim",
            Self::RawPayloadNotRetained => "raw_payload_not_retained",
            Self::ReplayUnstable => "replay_unstable",
            Self::DegradedStateNotDisclosed => "degraded_state_not_disclosed",
            Self::ExportParityBroken => "export_parity_broken",
            Self::DimensionOutcomeDrift => "dimension_outcome_drift",
            Self::ProfileCertificationDrift => "profile_certification_drift",
            Self::ProfileClaimStateDrift => "profile_claim_state_drift",
            Self::ProfileFreshnessDrift => "profile_freshness_drift",
            Self::ProfileEvidenceStale => "profile_evidence_stale",
            Self::CertificationIndexMissing => "certification_index_missing",
            Self::CertificationIndexDrift => "certification_index_drift",
            Self::ProfileDigestDrift => "profile_digest_drift",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the certification validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationValidationFinding {
    /// Closed finding kind.
    pub finding_kind: CertificationFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl CertificationValidationFinding {
    fn blocker(finding_kind: CertificationFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }

    fn warning(finding_kind: CertificationFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Warning,
            summary: summary.into(),
        }
    }
}

/// One graded certification dimension outcome (derived at materialization).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionOutcome {
    /// Certification dimension.
    pub dimension: CertificationDimension,
    /// True when the profile satisfies the dimension.
    pub passed: bool,
    /// Support-safe note describing the result.
    pub detail: String,
}

/// One profile certification: a claimed M5 tooling profile graded across every
/// certification dimension, with its freshness window and cited evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolingProfileCertification {
    /// Claimed tooling profile.
    pub profile: ToolingProfile,
    /// Support-safe summary of the interop claim under certification.
    pub claim_summary: String,
    /// How the profile sources the execution truth it presents.
    pub consumer_truth_source: ConsumerTruthSource,
    /// Normalized source kind of the representative certified event path.
    pub primary_source_kind: BuildTestEventSourceKind,
    /// Negotiated adapter capability state for the certified path.
    pub negotiated_capability: AdapterCapabilityState,
    /// Stable capability handshake / packet ref.
    pub capability_packet_ref: String,
    /// Named fallback reason, required when degraded or unsupported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<String>,
    /// Normalized confidence observed for the certified path.
    pub observed_confidence: BuildTestEventConfidence,
    /// Retained raw payload reference.
    pub raw_payload_ref: String,
    /// Digest of the retained raw payload.
    pub payload_digest: String,
    /// Retention class for the raw payload reference.
    pub raw_payload_retention: RawPayloadRetentionClass,
    /// True when raw private material is excluded from the retained reference.
    pub raw_private_material_excluded: bool,
    /// True when the profile replays deterministically from canonical envelopes.
    pub replay_stable: bool,
    /// True when support/release/AI exports preserve source, confidence, refs.
    pub export_parity_preserved: bool,
    /// True when a degraded/unsupported capability is visibly disclosed.
    pub degraded_state_disclosed: bool,
    /// Upstream evidence packets the profile draws its proof from.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Timestamp of the last recorded certification run.
    pub last_certified_at: String,
    /// Age in days of the recorded proof at the packet's capture time.
    pub proof_age_days: u32,
    /// Freshness window in days before the proof narrows below stable.
    pub freshness_window_days: u32,
    /// Derived freshness state.
    pub freshness_state: EvidenceFreshnessState,
    /// Per-dimension outcomes (derived at materialization).
    #[serde(default)]
    pub dimension_outcomes: Vec<DimensionOutcome>,
    /// True when every dimension passes (derived at materialization).
    pub certified: bool,
    /// Derived claim state for the matrix (derived at materialization).
    pub claim_state: ProfileClaimState,
}

impl ToolingProfileCertification {
    fn reason_present(&self) -> bool {
        self.fallback_reason
            .as_deref()
            .map(|reason| !reason.trim().is_empty())
            .unwrap_or(false)
    }

    fn requires_fallback_reason(&self) -> bool {
        matches!(
            self.negotiated_capability,
            AdapterCapabilityState::Degraded | AdapterCapabilityState::Unsupported
        )
    }

    fn overclaims_confidence(&self) -> bool {
        // A heuristic source, or an explicitly unsupported capability, cannot
        // claim more than low confidence.
        let must_be_low = self.primary_source_kind.is_heuristic()
            || matches!(
                self.negotiated_capability,
                AdapterCapabilityState::Unsupported
            );
        must_be_low && !matches!(self.observed_confidence, BuildTestEventConfidence::Low)
    }

    fn reuses_event_envelope(&self) -> bool {
        self.consumer_truth_source.is_conformant() && !self.evidence_refs_empty()
    }

    fn evidence_refs_empty(&self) -> bool {
        self.evidence_refs
            .iter()
            .all(|reference| reference.trim().is_empty())
    }

    fn fallback_detail(&self) -> String {
        match (&self.fallback_reason, self.requires_fallback_reason()) {
            (Some(reason), true) => format!("degraded/unsupported names reason: {reason}"),
            (None, true) => "degraded/unsupported names no fallback reason".to_owned(),
            (Some(reason), false) => format!("negotiated names spurious reason: {reason}"),
            (None, false) => "negotiated capability needs no fallback reason".to_owned(),
        }
    }

    /// Evaluates every certification dimension from the profile's explicit fields.
    fn evaluate_dimensions(&self) -> Vec<DimensionOutcome> {
        let envelope_ok = self.reuses_event_envelope();
        let adapter_ok = !self.capability_packet_ref.trim().is_empty();
        let fallback_ok = if self.requires_fallback_reason() {
            self.reason_present()
        } else {
            !self.reason_present()
        };
        let confidence_ok = !self.overclaims_confidence();
        let retention_ok = !self.raw_payload_ref.trim().is_empty()
            && !self.payload_digest.trim().is_empty()
            && self.raw_private_material_excluded;
        let degraded_ok = !self.requires_fallback_reason() || self.degraded_state_disclosed;

        vec![
            DimensionOutcome {
                dimension: CertificationDimension::EventEnvelopeReuse,
                passed: envelope_ok,
                detail: format!(
                    "truth source {} with {} evidence ref(s)",
                    self.consumer_truth_source.as_str(),
                    self.evidence_refs.len(),
                ),
            },
            DimensionOutcome {
                dimension: CertificationDimension::AdapterHierarchy,
                passed: adapter_ok,
                detail: format!(
                    "capability {} via {}",
                    self.negotiated_capability.as_str(),
                    display_ref(&self.capability_packet_ref),
                ),
            },
            DimensionOutcome {
                dimension: CertificationDimension::FallbackReason,
                passed: fallback_ok,
                detail: self.fallback_detail(),
            },
            DimensionOutcome {
                dimension: CertificationDimension::ConfidencePreservation,
                passed: confidence_ok,
                detail: format!(
                    "{} confidence for {} source",
                    self.observed_confidence.as_str(),
                    self.primary_source_kind.as_str(),
                ),
            },
            DimensionOutcome {
                dimension: CertificationDimension::RawPayloadRetention,
                passed: retention_ok,
                detail: format!(
                    "{} retention, private material excluded={}",
                    self.raw_payload_retention.as_str(),
                    self.raw_private_material_excluded,
                ),
            },
            DimensionOutcome {
                dimension: CertificationDimension::ReplayStability,
                passed: self.replay_stable,
                detail: format!("replay_stable={}", self.replay_stable),
            },
            DimensionOutcome {
                dimension: CertificationDimension::DegradedStateDisclosure,
                passed: degraded_ok,
                detail: format!(
                    "degraded_state_disclosed={} (required={})",
                    self.degraded_state_disclosed,
                    self.requires_fallback_reason(),
                ),
            },
            DimensionOutcome {
                dimension: CertificationDimension::ExportParity,
                passed: self.export_parity_preserved,
                detail: format!("export_parity_preserved={}", self.export_parity_preserved),
            },
        ]
    }

    /// Returns true when the profile is current and certified.
    pub fn is_claimable(&self) -> bool {
        self.freshness_state == EvidenceFreshnessState::Current && self.certified
    }
}

/// Certification index rolled up from the profile certifications (derived).
///
/// This is the one canonical execution-truth index release, support, AI, and
/// docs/help surfaces ingest: it names which profiles are claimable, which have
/// narrowed below stable on aged proof, and which are blocked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationIndex {
    /// Logical certification-index binding ref.
    pub certification_ref: String,
    /// True when every profile's proof is current.
    pub all_profiles_current: bool,
    /// True when every profile certifies across all dimensions.
    pub all_profiles_certified: bool,
    /// Profile tokens that are current and certified.
    #[serde(default)]
    pub claimable_profiles: Vec<String>,
    /// Profile tokens that are certified but narrowed on aged proof.
    #[serde(default)]
    pub narrowed_profiles: Vec<String>,
    /// Profile tokens that fail a dimension and block stable.
    #[serde(default)]
    pub blocked_profiles: Vec<String>,
    /// Support-safe roll-up summary.
    pub certification_summary: String,
}

/// Constructor input for [`EventInteropCertificationPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventInteropCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Logical certification-index binding ref.
    pub certification_ref: String,
    /// Profile certifications (outcomes/roll-ups derived at materialization).
    #[serde(default)]
    pub profiles: Vec<ToolingProfileCertification>,
}

/// Canonical event-interop certification packet: the claimed profile matrix, the
/// per-profile dimension grades, the freshness/stale-narrowing roll-up, and the
/// certification index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventInteropCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Certification boundary schema ref.
    pub certification_schema_ref: String,
    /// Per-event envelope boundary schema ref.
    pub envelope_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Frozen adapter-policy baseline this lane consumes.
    pub policy_baseline_ref: String,
    /// Build/test interop packet whose contract every profile certifies against.
    pub interop_packet_ref: String,
    /// Conformance suite this lane sits above.
    pub conformance_packet_ref: String,
    /// Profile certifications.
    #[serde(default)]
    pub profiles: Vec<ToolingProfileCertification>,
    /// Order-invariant digest of every profile token.
    pub profile_digest: String,
    /// Certification index rolled up from the profiles.
    pub certification_index: CertificationIndex,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<CertificationValidationFinding>,
}

impl EventInteropCertificationPacket {
    /// Materializes a packet, deriving per-profile dimension outcomes, freshness,
    /// claim states, the profile digest, and the certification index, then
    /// records findings and the promotion state.
    pub fn materialize(input: EventInteropCertificationPacketInput) -> Self {
        let profiles: Vec<ToolingProfileCertification> =
            input.profiles.into_iter().map(derive_profile).collect();
        let profile_digest = profile_digest(&profiles);
        let certification_index = derive_certification_index(&input.certification_ref, &profiles);

        let mut packet = Self {
            record_kind: EVENT_INTEROP_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            certification_schema_ref: EVENT_INTEROP_CERTIFICATION_SCHEMA_REF.to_owned(),
            envelope_schema_ref: EVENT_INTEROP_CERTIFICATION_ENVELOPE_SCHEMA_REF.to_owned(),
            doc_ref: EVENT_INTEROP_CERTIFICATION_DOC_REF.to_owned(),
            policy_baseline_ref: EVENT_INTEROP_CERTIFICATION_POLICY_BASELINE_REF.to_owned(),
            interop_packet_ref: EVENT_INTEROP_CERTIFICATION_INTEROP_PACKET_REF.to_owned(),
            conformance_packet_ref: EVENT_INTEROP_CERTIFICATION_CONFORMANCE_PACKET_REF.to_owned(),
            profiles,
            profile_digest,
            certification_index,
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against the frozen invariants.
    pub fn validate(&self) -> Vec<CertificationValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    /// Returns the certification for the given profile, if present.
    pub fn profile_for(&self, profile: ToolingProfile) -> Option<&ToolingProfileCertification> {
        self.profiles.iter().find(|row| row.profile == profile)
    }

    /// Builds an evidence join for one export/evidence surface.
    pub fn evidence_join(
        &self,
        surface: CertificationEvidenceSurface,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> EventInteropCertificationEvidenceJoinView {
        EventInteropCertificationEvidenceJoinView {
            record_kind: EVENT_INTEROP_CERTIFICATION_EVIDENCE_JOIN_RECORD_KIND.to_owned(),
            schema_version: EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION,
            view_id: view_id.into(),
            surface,
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            profile_digest: self.profile_digest.clone(),
            certification_index: self.certification_index.clone(),
            profile_rows: self
                .profiles
                .iter()
                .map(ProfileCertificationRow::from_profile)
                .collect(),
        }
    }

    /// Builds the CLI/headless stable view of the certification matrix.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> EventInteropCertificationCliHeadlessView {
        EventInteropCertificationCliHeadlessView {
            record_kind: EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            profile_digest: self.profile_digest.clone(),
            promotion_state: self.promotion_state,
            certification_index: self.certification_index.clone(),
            profile_rows: self
                .profiles
                .iter()
                .map(ProfileCertificationRow::from_profile)
                .collect(),
        }
    }

    /// Builds an export-safe support bundle carrying the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> EventInteropCertificationSupportExport {
        EventInteropCertificationSupportExport {
            record_kind: EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id_ref: self.packet_id.clone(),
            packet: self.clone(),
        }
    }

    /// Returns the profile tokens present in the packet.
    pub fn profile_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.profiles {
            set.insert(row.profile);
        }
        set.into_iter().map(ToolingProfile::as_str).collect()
    }

    /// Returns the source-kind tokens present across every profile.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.profiles {
            set.insert(row.primary_source_kind);
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Returns the consumer-truth-source tokens present across every profile.
    pub fn consumer_truth_source_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.profiles {
            set.insert(row.consumer_truth_source);
        }
        set.into_iter().map(ConsumerTruthSource::as_str).collect()
    }

    /// Returns the graded certification-dimension tokens.
    pub fn dimension_tokens(&self) -> Vec<&'static str> {
        CertificationDimension::ALL
            .into_iter()
            .map(CertificationDimension::as_str)
            .collect()
    }

    /// Compact, support-safe one-line-per-row rendering for the inspector.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "packet {} schema_version={} promotion={} profiles={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.profiles.len(),
            self.profile_digest,
        ));
        lines.push(format!(
            "index ref={} current={} certified={} claimable=[{}] narrowed=[{}] blocked=[{}]",
            self.certification_index.certification_ref,
            self.certification_index.all_profiles_current,
            self.certification_index.all_profiles_certified,
            self.certification_index.claimable_profiles.join(","),
            self.certification_index.narrowed_profiles.join(","),
            self.certification_index.blocked_profiles.join(","),
        ));
        for row in &self.profiles {
            lines.push(format!(
                "profile {} source={} truth={} capability={} confidence={} claim={} age={}/{}d",
                row.profile.as_str(),
                row.primary_source_kind.as_str(),
                row.consumer_truth_source.as_str(),
                row.negotiated_capability.as_str(),
                row.observed_confidence.as_str(),
                row.claim_state.as_str(),
                row.proof_age_days,
                row.freshness_window_days,
            ));
        }
        lines
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<CertificationValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != EVENT_INTEROP_CERTIFICATION_RECORD_KIND {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::WrongRecordKind,
                "packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION
        {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::WrongSchemaVersion,
                "packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::MissingIdentity,
                "packet id and timestamp are required",
            ));
        }
        for (label, value) in [
            (
                "certification schema",
                self.certification_schema_ref.as_str(),
            ),
            ("envelope schema", self.envelope_schema_ref.as_str()),
            ("doc", self.doc_ref.as_str()),
            ("policy baseline", self.policy_baseline_ref.as_str()),
            ("interop packet", self.interop_packet_ref.as_str()),
            ("conformance packet", self.conformance_packet_ref.as_str()),
        ] {
            if value.trim().is_empty() {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::MissingIdentity,
                    format!("{label} ref is required"),
                ));
            }
        }

        self.check_profiles(&mut findings, include_record_fields);
        self.check_certification_index(&mut findings, include_record_fields);

        if include_record_fields {
            let expected_digest = profile_digest(&self.profiles);
            if self.profile_digest != expected_digest {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::ProfileDigestDrift,
                    "stored profile digest does not match the profiles",
                ));
            }
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::PromotionStateMismatch,
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

    fn check_profiles(
        &self,
        findings: &mut Vec<CertificationValidationFinding>,
        include_record_fields: bool,
    ) {
        let mut seen: BTreeMap<ToolingProfile, usize> = BTreeMap::new();
        for row in &self.profiles {
            *seen.entry(row.profile).or_insert(0) += 1;
        }
        for profile in ToolingProfile::ALL {
            match seen.get(&profile) {
                None => findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::MissingProfile,
                    format!("tooling profile {} is missing", profile.as_str()),
                )),
                Some(count) if *count > 1 => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::DuplicateProfile,
                        format!(
                            "tooling profile {} is declared more than once",
                            profile.as_str()
                        ),
                    ));
                }
                Some(_) => {}
            }
        }

        for row in &self.profiles {
            self.check_profile(row, findings, include_record_fields);
        }
    }

    fn check_profile(
        &self,
        row: &ToolingProfileCertification,
        findings: &mut Vec<CertificationValidationFinding>,
        include_record_fields: bool,
    ) {
        let label = row.profile.as_str();

        // Stale evidence narrows below stable but does not block.
        let expected_freshness = freshness_for(row.proof_age_days, row.freshness_window_days);
        if expected_freshness == EvidenceFreshnessState::Stale {
            findings.push(CertificationValidationFinding::warning(
                CertificationFindingKind::ProfileEvidenceStale,
                format!(
                    "profile {} proof aged {} days past its {}-day window",
                    label, row.proof_age_days, row.freshness_window_days,
                ),
            ));
        }

        let outcomes = row.evaluate_dimensions();
        for outcome in &outcomes {
            if outcome.passed {
                continue;
            }
            match outcome.dimension {
                CertificationDimension::EventEnvelopeReuse => {
                    if !row.consumer_truth_source.is_conformant() {
                        findings.push(CertificationValidationFinding::blocker(
                            CertificationFindingKind::EventEnvelopeNotReused,
                            format!(
                                "profile {label} sources truth from {} instead of the canonical event envelope",
                                row.consumer_truth_source.as_str()
                            ),
                        ));
                    } else {
                        findings.push(CertificationValidationFinding::blocker(
                            CertificationFindingKind::MissingEvidenceRef,
                            format!("profile {label} cites no upstream evidence packet"),
                        ));
                    }
                }
                CertificationDimension::AdapterHierarchy => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::AdapterHierarchyMissing,
                        format!("profile {label} evidences no native-first capability handshake"),
                    ));
                }
                CertificationDimension::FallbackReason => {
                    if row.requires_fallback_reason() {
                        findings.push(CertificationValidationFinding::blocker(
                            CertificationFindingKind::FallbackReasonMissing,
                            format!(
                                "profile {label} degraded/unsupported without a fallback reason"
                            ),
                        ));
                    } else {
                        findings.push(CertificationValidationFinding::blocker(
                            CertificationFindingKind::FallbackReasonUnexpected,
                            format!(
                                "profile {label} negotiated yet names a spurious fallback reason"
                            ),
                        ));
                    }
                }
                CertificationDimension::ConfidencePreservation => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::ConfidenceOverclaim,
                        format!("profile {label} overclaims confidence for its source/capability"),
                    ));
                }
                CertificationDimension::RawPayloadRetention => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::RawPayloadNotRetained,
                        format!("profile {label} does not retain its raw payload safely"),
                    ));
                }
                CertificationDimension::ReplayStability => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::ReplayUnstable,
                        format!("profile {label} does not replay deterministically"),
                    ));
                }
                CertificationDimension::DegradedStateDisclosure => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::DegradedStateNotDisclosed,
                        format!("profile {label} hides its degraded/unsupported capability state"),
                    ));
                }
                CertificationDimension::ExportParity => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::ExportParityBroken,
                        format!("profile {label} breaks support/release/AI export parity"),
                    ));
                }
            }
        }

        if include_record_fields {
            if row.dimension_outcomes != outcomes {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::DimensionOutcomeDrift,
                    format!(
                        "profile {label} stored dimension outcomes disagree with the derivation"
                    ),
                ));
            }
            let expected_certified = profile_certified(&outcomes);
            if row.certified != expected_certified {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::ProfileCertificationDrift,
                    format!("profile {label} stored certified flag disagrees with the derivation"),
                ));
            }
            if row.freshness_state != expected_freshness {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::ProfileFreshnessDrift,
                    format!("profile {label} freshness state disagrees with proof age"),
                ));
            }
            let expected_claim = claim_state_for(expected_certified, expected_freshness);
            if row.claim_state != expected_claim {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::ProfileClaimStateDrift,
                    format!("profile {label} stored claim state disagrees with the derivation"),
                ));
            }
        }
    }

    fn check_certification_index(
        &self,
        findings: &mut Vec<CertificationValidationFinding>,
        include_record_fields: bool,
    ) {
        if self.certification_index.certification_ref.trim().is_empty() {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::CertificationIndexMissing,
                "certification-index binding ref is required",
            ));
        }
        if include_record_fields {
            let expected = derive_certification_index(
                &self.certification_index.certification_ref,
                &self.profiles,
            );
            if self.certification_index != expected {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::CertificationIndexDrift,
                    "stored certification index disagrees with the profiles",
                ));
            }
        }
    }
}

/// Support-export wrapper carrying the exact certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventInteropCertificationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Exact packet exported.
    pub packet: EventInteropCertificationPacket,
}

impl EventInteropCertificationSupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.packet_id_ref == self.packet.packet_id
            && self.packet.is_stable()
    }
}

/// One profile row for an evidence join or CLI/headless view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCertificationRow {
    /// Profile token.
    pub profile: String,
    /// Support-safe claim summary.
    pub claim_summary: String,
    /// Source kind token.
    pub source_kind: String,
    /// Consumer truth source token.
    pub consumer_truth_source: String,
    /// Capability state token.
    pub capability_state: String,
    /// Confidence token.
    pub confidence: String,
    /// Named fallback reason, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<String>,
    /// Claim state token.
    pub claim_state: String,
    /// Freshness state token.
    pub freshness_state: String,
    /// Proof age in days.
    pub proof_age_days: u32,
    /// Freshness window in days.
    pub freshness_window_days: u32,
    /// True when every dimension passes.
    pub certified: bool,
    /// True when the profile is current and certified.
    pub claimable: bool,
    /// Number of upstream evidence refs cited.
    pub evidence_ref_count: usize,
    /// Dimension tokens that failed (empty when certified).
    #[serde(default)]
    pub failed_dimensions: Vec<String>,
    /// Support-safe explanation of the profile certification.
    pub explanation: String,
}

impl ProfileCertificationRow {
    fn from_profile(row: &ToolingProfileCertification) -> Self {
        let failed_dimensions: Vec<String> = row
            .dimension_outcomes
            .iter()
            .filter(|outcome| !outcome.passed)
            .map(|outcome| outcome.dimension.as_str().to_owned())
            .collect();
        let explanation = format!(
            "{} via {} truth ({} source, {} capability, {} confidence); claim={}{}",
            row.profile.as_str(),
            row.consumer_truth_source.as_str(),
            row.primary_source_kind.as_str(),
            row.negotiated_capability.as_str(),
            row.observed_confidence.as_str(),
            row.claim_state.as_str(),
            if failed_dimensions.is_empty() {
                String::new()
            } else {
                format!(", failed=[{}]", failed_dimensions.join(","))
            },
        );
        Self {
            profile: row.profile.as_str().to_owned(),
            claim_summary: row.claim_summary.clone(),
            source_kind: row.primary_source_kind.as_str().to_owned(),
            consumer_truth_source: row.consumer_truth_source.as_str().to_owned(),
            capability_state: row.negotiated_capability.as_str().to_owned(),
            confidence: row.observed_confidence.as_str().to_owned(),
            fallback_reason: row.fallback_reason.clone(),
            claim_state: row.claim_state.as_str().to_owned(),
            freshness_state: row.freshness_state.as_str().to_owned(),
            proof_age_days: row.proof_age_days,
            freshness_window_days: row.freshness_window_days,
            certified: row.certified,
            claimable: row.is_claimable(),
            evidence_ref_count: row.evidence_refs.len(),
            failed_dimensions,
            explanation,
        }
    }
}

/// Evidence-join view for one export/evidence surface (support, incident, AI).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventInteropCertificationEvidenceJoinView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Evidence surface this view serves.
    pub surface: CertificationEvidenceSurface,
    /// View timestamp.
    pub generated_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Order-invariant digest of the profiles.
    pub profile_digest: String,
    /// Certification index.
    pub certification_index: CertificationIndex,
    /// Profile rows.
    #[serde(default)]
    pub profile_rows: Vec<ProfileCertificationRow>,
}

impl EventInteropCertificationEvidenceJoinView {
    /// Returns true when every row keeps its explanation and provenance fields.
    pub fn explains_consistently(&self) -> bool {
        self.profile_rows.iter().all(|row| {
            !row.profile.trim().is_empty()
                && !row.source_kind.trim().is_empty()
                && !row.claim_summary.trim().is_empty()
                && !row.explanation.trim().is_empty()
        })
    }
}

/// CLI/headless stable view of the certification matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventInteropCertificationCliHeadlessView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// View timestamp.
    pub generated_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Order-invariant digest of the profiles.
    pub profile_digest: String,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Certification index.
    pub certification_index: CertificationIndex,
    /// Profile rows.
    #[serde(default)]
    pub profile_rows: Vec<ProfileCertificationRow>,
}

impl EventInteropCertificationCliHeadlessView {
    /// Returns true when every profile row is explained and cites evidence.
    pub fn every_profile_explained(&self) -> bool {
        self.profile_rows
            .iter()
            .all(|row| row.evidence_ref_count > 0 && !row.explanation.trim().is_empty())
    }
}

fn display_ref(value: &str) -> &str {
    if value.trim().is_empty() {
        "<missing>"
    } else {
        value
    }
}

fn profile_certified(outcomes: &[DimensionOutcome]) -> bool {
    outcomes.iter().all(|outcome| outcome.passed)
}

fn freshness_for(proof_age_days: u32, freshness_window_days: u32) -> EvidenceFreshnessState {
    if proof_age_days > freshness_window_days {
        EvidenceFreshnessState::Stale
    } else {
        EvidenceFreshnessState::Current
    }
}

fn claim_state_for(certified: bool, freshness: EvidenceFreshnessState) -> ProfileClaimState {
    if !certified {
        ProfileClaimState::Blocked
    } else if freshness == EvidenceFreshnessState::Stale {
        ProfileClaimState::NarrowedBelowStable
    } else {
        ProfileClaimState::Claimable
    }
}

fn derive_profile(mut row: ToolingProfileCertification) -> ToolingProfileCertification {
    row.dimension_outcomes = row.evaluate_dimensions();
    row.certified = profile_certified(&row.dimension_outcomes);
    row.freshness_state = freshness_for(row.proof_age_days, row.freshness_window_days);
    row.claim_state = claim_state_for(row.certified, row.freshness_state);
    row
}

fn derive_certification_index(
    certification_ref: &str,
    profiles: &[ToolingProfileCertification],
) -> CertificationIndex {
    let all_profiles_current = profiles
        .iter()
        .all(|row| row.freshness_state == EvidenceFreshnessState::Current);
    let all_profiles_certified = !profiles.is_empty() && profiles.iter().all(|row| row.certified);
    let claimable_profiles = profiles_with_state(profiles, ProfileClaimState::Claimable);
    let narrowed_profiles = profiles_with_state(profiles, ProfileClaimState::NarrowedBelowStable);
    let blocked_profiles = profiles_with_state(profiles, ProfileClaimState::Blocked);
    let certification_summary = format!(
        "{} profiles; claimable={}, narrowed={}, blocked={}",
        profiles.len(),
        claimable_profiles.len(),
        narrowed_profiles.len(),
        blocked_profiles.len(),
    );
    CertificationIndex {
        certification_ref: certification_ref.to_owned(),
        all_profiles_current,
        all_profiles_certified,
        claimable_profiles,
        narrowed_profiles,
        blocked_profiles,
        certification_summary,
    }
}

fn profiles_with_state(
    profiles: &[ToolingProfileCertification],
    state: ProfileClaimState,
) -> Vec<String> {
    profiles
        .iter()
        .filter(|row| row.claim_state == state)
        .map(|row| row.profile.as_str().to_owned())
        .collect()
}

/// Order-invariant FNV-1a 64-bit digest of every profile token.
fn profile_digest(profiles: &[ToolingProfileCertification]) -> String {
    let mut tokens: Vec<&str> = profiles.iter().map(|row| row.profile.as_str()).collect();
    tokens.sort_unstable();
    fnv1a64(&tokens)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of strings.
fn fnv1a64(items_in_order: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for item in items_in_order {
        for byte in item.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

fn promotion_state_for_findings(
    findings: &[CertificationValidationFinding],
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

/// Builds the canonical stable event-interop certification packet input.
pub fn current_stable_event_interop_certification_input() -> EventInteropCertificationPacketInput {
    EventInteropCertificationPacketInput {
        packet_id: EVENT_INTEROP_CERTIFICATION_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        certification_ref: EVENT_INTEROP_CERTIFICATION_INDEX_REF.to_owned(),
        profiles: ToolingProfile::ALL
            .into_iter()
            .map(canonical_profile)
            .collect(),
    }
}

/// Materializes the canonical stable event-interop certification packet.
pub fn seeded_event_interop_certification_packet() -> EventInteropCertificationPacket {
    EventInteropCertificationPacket::materialize(current_stable_event_interop_certification_input())
}

/// Validates a packet and returns an `Ok(())` / findings result.
pub fn validate_event_interop_certification_packet(
    packet: &EventInteropCertificationPacket,
) -> Result<(), Vec<CertificationValidationFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn canonical_profile(profile: ToolingProfile) -> ToolingProfileCertification {
    // The pipeline overlay certifies the hardest path it actually consumes: an
    // imported-CI heuristic event normalized onto the canonical envelope. It
    // negotiates a degraded capability with a named reason and visibly downgraded,
    // low-confidence truth. The other profiles certify clean native or structured
    // paths. Every profile still reads the canonical event envelope.
    let (
        primary_source_kind,
        negotiated_capability,
        observed_confidence,
        fallback_reason,
        degraded_state_disclosed,
        retention,
        claim_summary,
    ) = match profile {
        ToolingProfile::TaskCenterRun => (
            BuildTestEventSourceKind::Native,
            AdapterCapabilityState::Negotiated,
            BuildTestEventConfidence::High,
            None,
            false,
            RawPayloadRetentionClass::MetadataDigestOnly,
            "task center run history reuses the canonical event objects with native truth",
        ),
        ToolingProfile::TestSession => (
            BuildTestEventSourceKind::StructuredOutput,
            AdapterCapabilityState::Negotiated,
            BuildTestEventConfidence::MediumHigh,
            None,
            false,
            RawPayloadRetentionClass::RedactedReference,
            "test sessions reuse the canonical event objects with structured-output truth",
        ),
        ToolingProfile::DebugSession => (
            BuildTestEventSourceKind::Native,
            AdapterCapabilityState::Negotiated,
            BuildTestEventConfidence::High,
            None,
            false,
            RawPayloadRetentionClass::MetadataDigestOnly,
            "debug sessions reuse the canonical event objects with native truth",
        ),
        ToolingProfile::PipelineOverlay => (
            BuildTestEventSourceKind::HeuristicParser,
            AdapterCapabilityState::Degraded,
            BuildTestEventConfidence::Low,
            Some("imported_ci_heuristic_fallback".to_owned()),
            true,
            RawPayloadRetentionClass::MetadataDigestOnly,
            "pipeline overlays normalize imported CI onto the canonical envelope as visibly degraded heuristic truth",
        ),
        ToolingProfile::NotebookRun => (
            BuildTestEventSourceKind::StructuredOutput,
            AdapterCapabilityState::Negotiated,
            BuildTestEventConfidence::MediumHigh,
            None,
            false,
            RawPayloadRetentionClass::RedactedReference,
            "notebook runs reuse the canonical event objects with structured-output truth",
        ),
        ToolingProfile::CoverageIntelligence => (
            BuildTestEventSourceKind::StructuredOutput,
            AdapterCapabilityState::Negotiated,
            BuildTestEventConfidence::MediumHigh,
            None,
            false,
            RawPayloadRetentionClass::RedactedReference,
            "coverage/flaky/snapshot intelligence reuses the canonical event objects with structured-output truth",
        ),
    };

    let token = profile.as_str();
    let raw_payload_ref = format!("raw:{token}");
    ToolingProfileCertification {
        profile,
        claim_summary: claim_summary.to_owned(),
        consumer_truth_source: ConsumerTruthSource::CanonicalEventEnvelope,
        primary_source_kind,
        negotiated_capability,
        capability_packet_ref: format!("capability-packet:{token}"),
        fallback_reason,
        observed_confidence,
        payload_digest: format!("sha256:{}", raw_payload_ref.replace(':', "-")),
        raw_payload_ref,
        raw_payload_retention: retention,
        raw_private_material_excluded: true,
        replay_stable: true,
        export_parity_preserved: true,
        degraded_state_disclosed,
        evidence_refs: EVENT_INTEROP_CERTIFICATION_EVIDENCE_REFS
            .iter()
            .map(|reference| (*reference).to_owned())
            .collect(),
        last_certified_at: "2026-06-15T00:00:00Z".to_owned(),
        proof_age_days: 3,
        freshness_window_days: 30,
        // Overwritten by `derive_profile` at materialization.
        freshness_state: EvidenceFreshnessState::Current,
        dimension_outcomes: Vec::new(),
        certified: false,
        claim_state: ProfileClaimState::Blocked,
    }
}
