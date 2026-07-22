//! Build/test interoperability conformance corpora and suite.
//!
//! The canonical build/test event interoperability packet
//! ([`crate::build_test_event_interoperability`]) freezes *one* event envelope
//! that joins native adapters, BSP discovery, Bazel BEP/BES, structured-output
//! importers (JUnit/SARIF), and problem-matcher / heuristic fallbacks. That
//! packet proves the contract holds once. This module turns that one-time claim
//! into a continually verified one: it lands the named *corpora* and the
//! *conformance suite* that re-run the adapter contract across the claimed M5
//! tooling archetypes, so adapter drift is measurable before it destabilizes the
//! test, coverage, notebook, or pipeline surfaces downstream.
//!
//! Four corpora cover the adapter families the interop envelope claims:
//!
//! - [`CorpusFamily::BspDiscovery`] — Build Server Protocol discovery/negotiation.
//! - [`CorpusFamily::BazelBepBes`] — Bazel Build Event Protocol / Build Event
//!   Service.
//! - [`CorpusFamily::StructuredOutputJunitSarif`] — structured-output importers
//!   (JUnit, SARIF, JSON).
//! - [`CorpusFamily::ProblemMatcherHeuristic`] — problem-matcher and heuristic
//!   parser fallbacks.
//!
//! Each corpus runs one [`ConformanceCase`] per claimed [`InteropArchetype`] that
//! depends on the family, and each case is graded on the seven conformance
//! [`ConformanceDimension`]s the docs require: capability negotiation, fallback
//! reason, confidence preservation, raw-payload retention, replay stability,
//! degraded-state behavior, and export parity. A case that overclaims confidence,
//! loses its raw payload, drops a fallback reason, hides a degraded state, breaks
//! replay, or breaks export parity *blocks stable*.
//!
//! Freshness is part of the contract: every corpus carries a proof age and a
//! freshness window, and a corpus whose proof has aged past its window
//! *narrows below stable* (a warning, not a blocker) so an interop claim cannot
//! stay green on aged proof. The derived [`ReleaseEvidenceBinding`] rolls the
//! corpus results up for release packets so they can show *current* interop proof
//! instead of one-off dogfood anecdotes.
//!
//! The packet deliberately reuses the source-kind, confidence, capability-state,
//! retention-class, promotion-state, and finding-severity vocabulary frozen in
//! [`crate::build_test_event_interoperability`]; it adds the corpus/case/freshness
//! layer and nothing that re-derives event truth.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/build-test-interop-corpora.md`](../../../docs/m5/build-test-interop-corpora.md);
//! the machine-readable boundary lives at
//! [`/schemas/tooling/interop-conformance.schema.json`](../../../schemas/tooling/interop-conformance.schema.json).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::build_test_event_interoperability::{
    AdapterCapabilityState, BuildTestEventConfidence, BuildTestEventSourceKind,
    BuildTestInteropFindingSeverity, BuildTestInteropPromotionState, RawPayloadRetentionClass,
};

/// Stable record-kind tag for [`InteropConformancePacket`].
pub const INTEROP_CONFORMANCE_RECORD_KIND: &str = "m5_interop_conformance_packet";

/// Stable record-kind tag for [`InteropConformanceSupportExport`].
pub const INTEROP_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_interop_conformance_support_export";

/// Stable record-kind tag for [`InteropConformanceEvidenceJoinView`].
pub const INTEROP_CONFORMANCE_EVIDENCE_JOIN_RECORD_KIND: &str =
    "m5_interop_conformance_evidence_join";

/// Stable record-kind tag for [`InteropConformanceCliHeadlessView`].
pub const INTEROP_CONFORMANCE_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_interop_conformance_cli_headless";

/// Integer schema version for the interop conformance packet.
pub const INTEROP_CONFORMANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the interop conformance boundary schema.
pub const INTEROP_CONFORMANCE_SCHEMA_REF: &str = "schemas/tooling/interop-conformance.schema.json";

/// Repo-relative path of the per-event task-event envelope boundary schema.
pub const INTEROP_CONFORMANCE_ENVELOPE_SCHEMA_REF: &str =
    "schemas/tooling/task-event-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const INTEROP_CONFORMANCE_DOC_REF: &str = "docs/m5/build-test-interop-corpora.md";

/// Repo-relative path of the frozen adapter-policy baseline this lane consumes.
pub const INTEROP_CONFORMANCE_POLICY_BASELINE_REF: &str =
    "artifacts/m5/tooling/event-interop-baseline/baseline.json";

/// Repo-relative path of the build/test interop packet whose contract this
/// suite continually re-verifies.
pub const INTEROP_CONFORMANCE_INTEROP_PACKET_REF: &str =
    "artifacts/runtime/m4/build_test_event_interoperability_packet.json";

/// Repo-relative path of the checked-in packet artifact.
pub const INTEROP_CONFORMANCE_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/tooling/interop-conformance/packet.json";

/// Logical release-evidence binding ref minted by the seed.
pub const INTEROP_CONFORMANCE_RELEASE_EVIDENCE_REF: &str =
    "release-evidence:tooling:m5:build-test-interop-conformance";

/// Stable packet id minted by the seed.
pub const INTEROP_CONFORMANCE_ID: &str = "tooling:m5:interop-conformance:v1";

/// Stable support-export id minted by the seed inspector.
pub const INTEROP_CONFORMANCE_SUPPORT_EXPORT_ID: &str =
    "support-export:tooling:m5:interop-conformance";

/// Stable AI evidence join id minted by the seed inspector.
pub const INTEROP_CONFORMANCE_AI_EVIDENCE_ID: &str = "ai-evidence:tooling:m5:interop-conformance";

/// Stable incident packet join id minted by the seed inspector.
pub const INTEROP_CONFORMANCE_INCIDENT_PACKET_ID: &str = "incident:tooling:m5:interop-conformance";

/// Stable CLI/headless view id minted by the seed inspector.
pub const INTEROP_CONFORMANCE_CLI_HEADLESS_ID: &str = "cli-headless:tooling:m5:interop-conformance";

/// One named corpus family covering an adapter interoperability area.
///
/// Each family maps to a primary [`BuildTestEventSourceKind`] and a checked-in
/// fixture directory; native truth is the implicit baseline every family
/// normalizes onto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorpusFamily {
    /// Build Server Protocol discovery and capability negotiation.
    BspDiscovery,
    /// Bazel Build Event Protocol / Build Event Service.
    BazelBepBes,
    /// Structured-output importers (JUnit, SARIF, JSON).
    StructuredOutputJunitSarif,
    /// Problem-matcher and heuristic parser fallbacks.
    ProblemMatcherHeuristic,
}

impl CorpusFamily {
    /// Every required corpus family in stable declaration order.
    pub const ALL: [Self; 4] = [
        Self::BspDiscovery,
        Self::BazelBepBes,
        Self::StructuredOutputJunitSarif,
        Self::ProblemMatcherHeuristic,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BspDiscovery => "bsp_discovery",
            Self::BazelBepBes => "bazel_bep_bes",
            Self::StructuredOutputJunitSarif => "structured_output_junit_sarif",
            Self::ProblemMatcherHeuristic => "problem_matcher_heuristic",
        }
    }

    /// The primary normalized source kind this family exercises.
    pub const fn source_kind(self) -> BuildTestEventSourceKind {
        match self {
            Self::BspDiscovery => BuildTestEventSourceKind::Bsp,
            Self::BazelBepBes => BuildTestEventSourceKind::BazelBep,
            Self::StructuredOutputJunitSarif => BuildTestEventSourceKind::StructuredOutput,
            Self::ProblemMatcherHeuristic => BuildTestEventSourceKind::HeuristicParser,
        }
    }

    /// Repo-relative fixture directory for this family's corpus.
    pub const fn fixture_dir(self) -> &'static str {
        match self {
            Self::BspDiscovery => "fixtures/tooling/m5/bsp-discovery",
            Self::BazelBepBes => "fixtures/tooling/m5/bazel-bep-bes",
            Self::StructuredOutputJunitSarif => "fixtures/tooling/m5/structured-output-junit-sarif",
            Self::ProblemMatcherHeuristic => "fixtures/tooling/m5/problem-matcher-heuristic",
        }
    }

    /// True when this family normalizes a fallback/heuristic source kind.
    pub const fn is_heuristic(self) -> bool {
        matches!(self, Self::ProblemMatcherHeuristic)
    }
}

/// A claimed M5 tooling archetype that depends on adapter interoperability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteropArchetype {
    /// Rust / Cargo workspace (structured JSON output + rustc problem matcher).
    RustCargo,
    /// Node workspace (Jest/JUnit output + tsc/eslint problem matcher).
    NodeWorkspace,
    /// Python project driven by pytest (JUnit XML output + problem matcher).
    PythonPytest,
    /// JVM project served over BSP (Bloop/sbt) with JUnit results.
    JvmBuildServer,
    /// Bazel monorepo (BSP server + BEP/BES + structured test output).
    BazelMonorepo,
    /// Polyglot CI import (JUnit/SARIF artifacts + heuristic fallback).
    PolyglotCi,
}

impl InteropArchetype {
    /// Every claimed archetype in stable declaration order.
    pub const ALL: [Self; 6] = [
        Self::RustCargo,
        Self::NodeWorkspace,
        Self::PythonPytest,
        Self::JvmBuildServer,
        Self::BazelMonorepo,
        Self::PolyglotCi,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RustCargo => "rust_cargo",
            Self::NodeWorkspace => "node_workspace",
            Self::PythonPytest => "python_pytest",
            Self::JvmBuildServer => "jvm_build_server",
            Self::BazelMonorepo => "bazel_monorepo",
            Self::PolyglotCi => "polyglot_ci",
        }
    }

    /// The corpus families this archetype depends on for interoperability.
    pub const fn dependent_families(self) -> &'static [CorpusFamily] {
        match self {
            Self::RustCargo | Self::NodeWorkspace | Self::PythonPytest | Self::PolyglotCi => &[
                CorpusFamily::StructuredOutputJunitSarif,
                CorpusFamily::ProblemMatcherHeuristic,
            ],
            Self::JvmBuildServer => &[
                CorpusFamily::BspDiscovery,
                CorpusFamily::StructuredOutputJunitSarif,
            ],
            Self::BazelMonorepo => &[
                CorpusFamily::BspDiscovery,
                CorpusFamily::BazelBepBes,
                CorpusFamily::StructuredOutputJunitSarif,
            ],
        }
    }

    /// True when this archetype depends on the given corpus family.
    pub fn depends_on(self, family: CorpusFamily) -> bool {
        self.dependent_families().contains(&family)
    }
}

/// Returns every archetype that depends on the given corpus family, in order.
pub fn archetypes_for_family(family: CorpusFamily) -> Vec<InteropArchetype> {
    InteropArchetype::ALL
        .into_iter()
        .filter(|archetype| archetype.depends_on(family))
        .collect()
}

/// One conformance dimension graded for every corpus case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceDimension {
    /// The adapter ran a capability handshake (BSP/BEP/importer negotiation).
    CapabilityNegotiation,
    /// A degraded or unsupported capability names an explicit fallback reason.
    FallbackReason,
    /// The normalized confidence does not overclaim its source.
    ConfidencePreservation,
    /// The raw adapter payload is retained behind a reference without leaking.
    RawPayloadRetention,
    /// The case replays deterministically from canonical envelopes.
    ReplayStability,
    /// A degraded/unsupported capability is visibly disclosed.
    DegradedStateBehavior,
    /// Support / release / AI exports preserve source, confidence, and refs.
    ExportParity,
}

impl ConformanceDimension {
    /// Every graded dimension in stable declaration order.
    pub const ALL: [Self; 7] = [
        Self::CapabilityNegotiation,
        Self::FallbackReason,
        Self::ConfidencePreservation,
        Self::RawPayloadRetention,
        Self::ReplayStability,
        Self::DegradedStateBehavior,
        Self::ExportParity,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityNegotiation => "capability_negotiation",
            Self::FallbackReason => "fallback_reason",
            Self::ConfidencePreservation => "confidence_preservation",
            Self::RawPayloadRetention => "raw_payload_retention",
            Self::ReplayStability => "replay_stability",
            Self::DegradedStateBehavior => "degraded_state_behavior",
            Self::ExportParity => "export_parity",
        }
    }
}

/// Derived freshness state for a corpus's recorded proof.
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

/// Evidence-join surface that presents the conformance suite across a boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceEvidenceSurface {
    /// Support bundle / support export.
    SupportBundle,
    /// Incident timeline packet.
    IncidentPacket,
    /// AI evidence packet.
    AiEvidence,
}

impl ConformanceEvidenceSurface {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundle => "support_bundle",
            Self::IncidentPacket => "incident_packet",
            Self::AiEvidence => "ai_evidence",
        }
    }
}

/// Closed validation finding vocabulary for the conformance packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteropConformanceFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// A required corpus family is absent.
    MissingCorpusFamily,
    /// Two corpora declare the same family.
    DuplicateCorpusFamily,
    /// A corpus carries no cases.
    CorpusEmpty,
    /// A corpus lacks a case for an archetype that depends on its family.
    MissingArchetypeCoverage,
    /// A case's source kind disagrees with its corpus family.
    CaseSourceKindMismatch,
    /// A case did not run a capability handshake.
    CapabilityNegotiationMissing,
    /// A degraded/unsupported capability names no fallback reason.
    FallbackReasonMissing,
    /// A negotiated capability names a spurious fallback reason.
    FallbackReasonUnexpected,
    /// A case overclaims confidence for its source kind or capability state.
    ConfidenceOverclaim,
    /// A case does not retain its raw payload behind a safe reference.
    RawPayloadNotRetained,
    /// A case does not replay deterministically.
    ReplayUnstable,
    /// A degraded/unsupported capability is not visibly disclosed.
    DegradedStateNotDisclosed,
    /// A case breaks support/release/AI export parity.
    ExportParityBroken,
    /// Stored per-dimension outcomes disagree with the derivation.
    DimensionOutcomeDrift,
    /// Stored case conformance disagrees with the derivation.
    CaseConformanceDrift,
    /// Stored corpus freshness state disagrees with the derivation.
    CorpusFreshnessDrift,
    /// Stored corpus conformance roll-up disagrees with the derivation.
    CorpusConformanceDrift,
    /// A corpus's recorded proof has aged past its freshness window.
    CorpusEvidenceStale,
    /// The release-evidence binding ref is missing.
    ReleaseEvidenceMissing,
    /// Stored release-evidence binding disagrees with the derivation.
    ReleaseEvidenceDrift,
    /// Stored corpus digest disagrees with the derivation.
    CorpusDigestDrift,
    /// Stored promotion state disagrees with the derivation.
    PromotionStateMismatch,
}

impl InteropConformanceFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingCorpusFamily => "missing_corpus_family",
            Self::DuplicateCorpusFamily => "duplicate_corpus_family",
            Self::CorpusEmpty => "corpus_empty",
            Self::MissingArchetypeCoverage => "missing_archetype_coverage",
            Self::CaseSourceKindMismatch => "case_source_kind_mismatch",
            Self::CapabilityNegotiationMissing => "capability_negotiation_missing",
            Self::FallbackReasonMissing => "fallback_reason_missing",
            Self::FallbackReasonUnexpected => "fallback_reason_unexpected",
            Self::ConfidenceOverclaim => "confidence_overclaim",
            Self::RawPayloadNotRetained => "raw_payload_not_retained",
            Self::ReplayUnstable => "replay_unstable",
            Self::DegradedStateNotDisclosed => "degraded_state_not_disclosed",
            Self::ExportParityBroken => "export_parity_broken",
            Self::DimensionOutcomeDrift => "dimension_outcome_drift",
            Self::CaseConformanceDrift => "case_conformance_drift",
            Self::CorpusFreshnessDrift => "corpus_freshness_drift",
            Self::CorpusConformanceDrift => "corpus_conformance_drift",
            Self::CorpusEvidenceStale => "corpus_evidence_stale",
            Self::ReleaseEvidenceMissing => "release_evidence_missing",
            Self::ReleaseEvidenceDrift => "release_evidence_drift",
            Self::CorpusDigestDrift => "corpus_digest_drift",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the conformance validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteropConformanceValidationFinding {
    /// Closed finding kind.
    pub finding_kind: InteropConformanceFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl InteropConformanceValidationFinding {
    fn blocker(finding_kind: InteropConformanceFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }

    fn warning(finding_kind: InteropConformanceFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Warning,
            summary: summary.into(),
        }
    }
}

/// One graded conformance dimension outcome (derived at materialization).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionOutcome {
    /// Conformance dimension.
    pub dimension: ConformanceDimension,
    /// True when the case satisfies the dimension.
    pub passed: bool,
    /// Support-safe note describing the result.
    pub detail: String,
}

/// One conformance case: a single adapter family exercised against one
/// archetype, graded across every conformance dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceCase {
    /// Stable case id.
    pub case_id: String,
    /// Corpus family the case belongs to.
    pub family: CorpusFamily,
    /// Archetype the case exercises.
    pub archetype: InteropArchetype,
    /// Normalized source kind for the case (must match the family).
    pub source_kind: BuildTestEventSourceKind,
    /// Support-safe scenario description.
    pub scenario: String,
    /// Negotiated adapter capability state.
    pub negotiated_capability: AdapterCapabilityState,
    /// Stable capability handshake / packet ref.
    pub capability_packet_ref: String,
    /// Named fallback reason, required when degraded or unsupported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<String>,
    /// Normalized confidence observed for the case.
    pub observed_confidence: BuildTestEventConfidence,
    /// Retained raw payload reference.
    pub raw_payload_ref: String,
    /// Digest of the retained raw payload.
    pub payload_digest: String,
    /// Retention class for the raw payload reference.
    pub raw_payload_retention: RawPayloadRetentionClass,
    /// True when raw private material is excluded from the retained reference.
    pub raw_private_material_excluded: bool,
    /// True when the case replays deterministically from canonical envelopes.
    pub replay_stable: bool,
    /// True when support/release/AI exports preserve source, confidence, refs.
    pub export_parity_preserved: bool,
    /// True when a degraded/unsupported capability is visibly disclosed.
    pub degraded_state_disclosed: bool,
    /// Per-dimension outcomes (derived at materialization).
    #[serde(default)]
    pub dimension_outcomes: Vec<DimensionOutcome>,
    /// True when every dimension passes (derived at materialization).
    pub conforms: bool,
}

impl ConformanceCase {
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

    /// Evaluates every conformance dimension from the case's explicit fields.
    fn evaluate_dimensions(&self) -> Vec<DimensionOutcome> {
        let capability_ok = !self.capability_packet_ref.trim().is_empty();
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
                dimension: ConformanceDimension::CapabilityNegotiation,
                passed: capability_ok,
                detail: format!(
                    "capability {} via {}",
                    self.negotiated_capability.as_str(),
                    display_ref(&self.capability_packet_ref),
                ),
            },
            DimensionOutcome {
                dimension: ConformanceDimension::FallbackReason,
                passed: fallback_ok,
                detail: self.fallback_detail(),
            },
            DimensionOutcome {
                dimension: ConformanceDimension::ConfidencePreservation,
                passed: confidence_ok,
                detail: format!(
                    "{} confidence for {} source",
                    self.observed_confidence.as_str(),
                    self.source_kind.as_str(),
                ),
            },
            DimensionOutcome {
                dimension: ConformanceDimension::RawPayloadRetention,
                passed: retention_ok,
                detail: format!(
                    "{} retention, private material excluded={}",
                    self.raw_payload_retention.as_str(),
                    self.raw_private_material_excluded,
                ),
            },
            DimensionOutcome {
                dimension: ConformanceDimension::ReplayStability,
                passed: self.replay_stable,
                detail: format!("replay_stable={}", self.replay_stable),
            },
            DimensionOutcome {
                dimension: ConformanceDimension::DegradedStateBehavior,
                passed: degraded_ok,
                detail: format!(
                    "degraded_state_disclosed={} (required={})",
                    self.degraded_state_disclosed,
                    self.requires_fallback_reason(),
                ),
            },
            DimensionOutcome {
                dimension: ConformanceDimension::ExportParity,
                passed: self.export_parity_preserved,
                detail: format!("export_parity_preserved={}", self.export_parity_preserved),
            },
        ]
    }

    fn overclaims_confidence(&self) -> bool {
        // A heuristic source, or an explicitly unsupported capability, cannot
        // claim more than low confidence.
        let must_be_low = self.source_kind.is_heuristic()
            || matches!(
                self.negotiated_capability,
                AdapterCapabilityState::Unsupported
            );
        must_be_low && !matches!(self.observed_confidence, BuildTestEventConfidence::Low)
    }

    fn fallback_detail(&self) -> String {
        match (&self.fallback_reason, self.requires_fallback_reason()) {
            (Some(reason), true) => format!("degraded/unsupported names reason: {reason}"),
            (None, true) => "degraded/unsupported names no fallback reason".to_owned(),
            (Some(reason), false) => format!("negotiated names spurious reason: {reason}"),
            (None, false) => "negotiated capability needs no fallback reason".to_owned(),
        }
    }

    fn source_kind_matches_family(&self) -> bool {
        self.source_kind == self.family.source_kind()
    }
}

/// One corpus: every conformance case for an adapter family, plus its freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteropCorpus {
    /// Corpus family.
    pub family: CorpusFamily,
    /// Primary normalized source kind for the family.
    pub source_kind: BuildTestEventSourceKind,
    /// Repo-relative fixture directory for this corpus.
    pub fixture_dir: String,
    /// Timestamp of the last recorded run of this corpus.
    pub last_run_at: String,
    /// Age in days of the recorded proof at the packet's capture time.
    pub proof_age_days: u32,
    /// Freshness window in days before the proof narrows below stable.
    pub freshness_window_days: u32,
    /// Derived freshness state.
    pub freshness_state: EvidenceFreshnessState,
    /// Conformance cases, one per dependent archetype.
    #[serde(default)]
    pub cases: Vec<ConformanceCase>,
    /// True when every case conforms (derived).
    pub all_cases_conform: bool,
}

impl InteropCorpus {
    /// Returns true when the corpus proof is current and every case conforms.
    pub fn is_release_ready(&self) -> bool {
        self.freshness_state == EvidenceFreshnessState::Current && self.all_cases_conform
    }
}

/// Release-evidence binding rolled up from the corpus results (derived).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseEvidenceBinding {
    /// Logical release-evidence binding ref.
    pub release_evidence_ref: String,
    /// True when every corpus's proof is current.
    pub all_corpora_current: bool,
    /// True when every corpus's cases conform.
    pub all_cases_conform: bool,
    /// Corpus family tokens that are stale or non-conforming (narrowing source).
    #[serde(default)]
    pub narrowed_families: Vec<String>,
    /// Support-safe roll-up summary.
    pub conformance_summary: String,
}

/// Constructor input for [`InteropConformancePacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteropConformancePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Logical release-evidence binding ref.
    pub release_evidence_ref: String,
    /// Corpora (case outcomes and corpus roll-ups derived at materialization).
    #[serde(default)]
    pub corpora: Vec<InteropCorpus>,
}

/// Canonical interop conformance packet: the named corpora, the per-case
/// dimension grades, the freshness/stale-narrowing roll-up, and the
/// release-evidence binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteropConformancePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Conformance boundary schema ref.
    pub conformance_schema_ref: String,
    /// Per-event envelope boundary schema ref.
    pub envelope_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Frozen adapter-policy baseline this lane consumes.
    pub policy_baseline_ref: String,
    /// Build/test interop packet whose contract this suite re-verifies.
    pub interop_packet_ref: String,
    /// Named corpora.
    #[serde(default)]
    pub corpora: Vec<InteropCorpus>,
    /// Order-invariant digest of every case id.
    pub corpus_digest: String,
    /// Release-evidence binding rolled up from the corpora.
    pub release_evidence: ReleaseEvidenceBinding,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<InteropConformanceValidationFinding>,
}

impl InteropConformancePacket {
    /// Materializes a packet, deriving per-case dimension outcomes, corpus
    /// freshness/conformance roll-ups, the corpus digest, and the
    /// release-evidence binding, then records findings and the promotion state.
    pub fn materialize(input: InteropConformancePacketInput) -> Self {
        let corpora: Vec<InteropCorpus> = input.corpora.into_iter().map(derive_corpus).collect();
        let corpus_digest = corpus_digest(&corpora);
        let release_evidence = derive_release_evidence(&input.release_evidence_ref, &corpora);

        let mut packet = Self {
            record_kind: INTEROP_CONFORMANCE_RECORD_KIND.to_owned(),
            schema_version: INTEROP_CONFORMANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            conformance_schema_ref: INTEROP_CONFORMANCE_SCHEMA_REF.to_owned(),
            envelope_schema_ref: INTEROP_CONFORMANCE_ENVELOPE_SCHEMA_REF.to_owned(),
            doc_ref: INTEROP_CONFORMANCE_DOC_REF.to_owned(),
            policy_baseline_ref: INTEROP_CONFORMANCE_POLICY_BASELINE_REF.to_owned(),
            interop_packet_ref: INTEROP_CONFORMANCE_INTEROP_PACKET_REF.to_owned(),
            corpora,
            corpus_digest,
            release_evidence,
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against the frozen invariants.
    pub fn validate(&self) -> Vec<InteropConformanceValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    /// Returns the corpus for the given family, if present.
    pub fn corpus_for(&self, family: CorpusFamily) -> Option<&InteropCorpus> {
        self.corpora.iter().find(|corpus| corpus.family == family)
    }

    /// Returns every case across every corpus.
    pub fn cases(&self) -> impl Iterator<Item = &ConformanceCase> {
        self.corpora.iter().flat_map(|corpus| corpus.cases.iter())
    }

    /// Builds an evidence join for one export/evidence surface.
    pub fn evidence_join(
        &self,
        surface: ConformanceEvidenceSurface,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> InteropConformanceEvidenceJoinView {
        InteropConformanceEvidenceJoinView {
            record_kind: INTEROP_CONFORMANCE_EVIDENCE_JOIN_RECORD_KIND.to_owned(),
            schema_version: INTEROP_CONFORMANCE_SCHEMA_VERSION,
            view_id: view_id.into(),
            surface,
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            corpus_digest: self.corpus_digest.clone(),
            release_evidence: self.release_evidence.clone(),
            corpus_rows: self
                .corpora
                .iter()
                .map(InteropCorpusRow::from_corpus)
                .collect(),
            case_rows: self.cases().map(ConformanceCaseRow::from_case).collect(),
        }
    }

    /// Builds the CLI/headless stable view of the conformance suite.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> InteropConformanceCliHeadlessView {
        InteropConformanceCliHeadlessView {
            record_kind: INTEROP_CONFORMANCE_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: INTEROP_CONFORMANCE_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            corpus_digest: self.corpus_digest.clone(),
            promotion_state: self.promotion_state,
            release_evidence: self.release_evidence.clone(),
            corpus_rows: self
                .corpora
                .iter()
                .map(InteropCorpusRow::from_corpus)
                .collect(),
            case_rows: self.cases().map(ConformanceCaseRow::from_case).collect(),
        }
    }

    /// Builds an export-safe support bundle carrying the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> InteropConformanceSupportExport {
        InteropConformanceSupportExport {
            record_kind: INTEROP_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: INTEROP_CONFORMANCE_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id_ref: self.packet_id.clone(),
            packet: self.clone(),
        }
    }

    /// Returns the corpus-family tokens present in the packet.
    pub fn corpus_family_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for corpus in &self.corpora {
            set.insert(corpus.family);
        }
        set.into_iter().map(CorpusFamily::as_str).collect()
    }

    /// Returns the archetype tokens covered across every corpus.
    pub fn archetype_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for case in self.cases() {
            set.insert(case.archetype);
        }
        set.into_iter().map(InteropArchetype::as_str).collect()
    }

    /// Returns the source-kind tokens present across every corpus.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for corpus in &self.corpora {
            set.insert(corpus.source_kind);
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Returns the graded conformance-dimension tokens.
    pub fn dimension_tokens(&self) -> Vec<&'static str> {
        ConformanceDimension::ALL
            .into_iter()
            .map(ConformanceDimension::as_str)
            .collect()
    }

    /// Compact, support-safe one-line-per-row rendering for the inspector.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "packet {} schema_version={} promotion={} corpora={} cases={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.corpora.len(),
            self.cases().count(),
            self.corpus_digest,
        ));
        lines.push(format!(
            "release_evidence ref={} current={} conforming={} narrowed=[{}]",
            self.release_evidence.release_evidence_ref,
            self.release_evidence.all_corpora_current,
            self.release_evidence.all_cases_conform,
            self.release_evidence.narrowed_families.join(","),
        ));
        for corpus in &self.corpora {
            lines.push(format!(
                "corpus {} source={} freshness={} age={}/{}d cases={} conform={}",
                corpus.family.as_str(),
                corpus.source_kind.as_str(),
                corpus.freshness_state.as_str(),
                corpus.proof_age_days,
                corpus.freshness_window_days,
                corpus.cases.len(),
                corpus.all_cases_conform,
            ));
            for case in &corpus.cases {
                lines.push(format!(
                    "  case {} archetype={} capability={} confidence={} conforms={}",
                    case.case_id,
                    case.archetype.as_str(),
                    case.negotiated_capability.as_str(),
                    case.observed_confidence.as_str(),
                    case.conforms,
                ));
            }
        }
        lines
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<InteropConformanceValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != INTEROP_CONFORMANCE_RECORD_KIND {
            findings.push(InteropConformanceValidationFinding::blocker(
                InteropConformanceFindingKind::WrongRecordKind,
                "packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != INTEROP_CONFORMANCE_SCHEMA_VERSION {
            findings.push(InteropConformanceValidationFinding::blocker(
                InteropConformanceFindingKind::WrongSchemaVersion,
                "packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(InteropConformanceValidationFinding::blocker(
                InteropConformanceFindingKind::MissingIdentity,
                "packet id and timestamp are required",
            ));
        }
        for (label, value) in [
            ("conformance schema", self.conformance_schema_ref.as_str()),
            ("envelope schema", self.envelope_schema_ref.as_str()),
            ("doc", self.doc_ref.as_str()),
            ("policy baseline", self.policy_baseline_ref.as_str()),
            ("interop packet", self.interop_packet_ref.as_str()),
        ] {
            if value.trim().is_empty() {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::MissingIdentity,
                    format!("{label} ref is required"),
                ));
            }
        }

        self.check_corpora(&mut findings, include_record_fields);
        self.check_release_evidence(&mut findings, include_record_fields);

        if include_record_fields {
            let expected_digest = corpus_digest(&self.corpora);
            if self.corpus_digest != expected_digest {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::CorpusDigestDrift,
                    "stored corpus digest does not match the corpora",
                ));
            }
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::PromotionStateMismatch,
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

    fn check_corpora(
        &self,
        findings: &mut Vec<InteropConformanceValidationFinding>,
        include_record_fields: bool,
    ) {
        let mut seen: BTreeMap<CorpusFamily, usize> = BTreeMap::new();
        for corpus in &self.corpora {
            *seen.entry(corpus.family).or_insert(0) += 1;
        }
        for family in CorpusFamily::ALL {
            match seen.get(&family) {
                None => findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::MissingCorpusFamily,
                    format!("corpus family {} is missing", family.as_str()),
                )),
                Some(count) if *count > 1 => {
                    findings.push(InteropConformanceValidationFinding::blocker(
                        InteropConformanceFindingKind::DuplicateCorpusFamily,
                        format!(
                            "corpus family {} is declared more than once",
                            family.as_str()
                        ),
                    ));
                }
                Some(_) => {}
            }
        }

        for corpus in &self.corpora {
            self.check_corpus(corpus, findings, include_record_fields);
        }
    }

    fn check_corpus(
        &self,
        corpus: &InteropCorpus,
        findings: &mut Vec<InteropConformanceValidationFinding>,
        include_record_fields: bool,
    ) {
        let family = corpus.family;
        if corpus.source_kind != family.source_kind() {
            findings.push(InteropConformanceValidationFinding::blocker(
                InteropConformanceFindingKind::CaseSourceKindMismatch,
                format!("corpus {} declares the wrong source kind", family.as_str()),
            ));
        }
        if corpus.cases.is_empty() {
            findings.push(InteropConformanceValidationFinding::blocker(
                InteropConformanceFindingKind::CorpusEmpty,
                format!("corpus {} carries no cases", family.as_str()),
            ));
        }

        let covered: BTreeSet<InteropArchetype> =
            corpus.cases.iter().map(|case| case.archetype).collect();
        for archetype in archetypes_for_family(family) {
            if !covered.contains(&archetype) {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::MissingArchetypeCoverage,
                    format!(
                        "corpus {} does not cover archetype {}",
                        family.as_str(),
                        archetype.as_str()
                    ),
                ));
            }
        }

        // Stale evidence narrows below stable but does not block.
        let expected_freshness = freshness_for(corpus.proof_age_days, corpus.freshness_window_days);
        if expected_freshness == EvidenceFreshnessState::Stale {
            findings.push(InteropConformanceValidationFinding::warning(
                InteropConformanceFindingKind::CorpusEvidenceStale,
                format!(
                    "corpus {} proof aged {} days past its {}-day window",
                    family.as_str(),
                    corpus.proof_age_days,
                    corpus.freshness_window_days,
                ),
            ));
        }

        for case in &corpus.cases {
            self.check_case(case, findings, include_record_fields);
        }

        if include_record_fields {
            if corpus.freshness_state != expected_freshness {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::CorpusFreshnessDrift,
                    format!(
                        "corpus {} freshness state disagrees with proof age",
                        family.as_str()
                    ),
                ));
            }
            let expected_conform = !corpus.cases.is_empty()
                && corpus
                    .cases
                    .iter()
                    .all(|case| case_conforms(&case.evaluate_dimensions()));
            if corpus.all_cases_conform != expected_conform {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::CorpusConformanceDrift,
                    format!(
                        "corpus {} conformance roll-up disagrees with its cases",
                        family.as_str()
                    ),
                ));
            }
        }
    }

    fn check_case(
        &self,
        case: &ConformanceCase,
        findings: &mut Vec<InteropConformanceValidationFinding>,
        include_record_fields: bool,
    ) {
        let label = &case.case_id;
        if !case.source_kind_matches_family() {
            findings.push(InteropConformanceValidationFinding::blocker(
                InteropConformanceFindingKind::CaseSourceKindMismatch,
                format!("case {label} source kind disagrees with its family"),
            ));
        }

        let outcomes = case.evaluate_dimensions();
        for outcome in &outcomes {
            if outcome.passed {
                continue;
            }
            match outcome.dimension {
                ConformanceDimension::CapabilityNegotiation => {
                    findings.push(InteropConformanceValidationFinding::blocker(
                        InteropConformanceFindingKind::CapabilityNegotiationMissing,
                        format!("case {label} ran no capability handshake"),
                    ));
                }
                ConformanceDimension::FallbackReason => {
                    if case.requires_fallback_reason() {
                        findings.push(InteropConformanceValidationFinding::blocker(
                            InteropConformanceFindingKind::FallbackReasonMissing,
                            format!("case {label} degraded/unsupported without a fallback reason"),
                        ));
                    } else {
                        findings.push(InteropConformanceValidationFinding::blocker(
                            InteropConformanceFindingKind::FallbackReasonUnexpected,
                            format!("case {label} negotiated yet names a spurious fallback reason"),
                        ));
                    }
                }
                ConformanceDimension::ConfidencePreservation => {
                    findings.push(InteropConformanceValidationFinding::blocker(
                        InteropConformanceFindingKind::ConfidenceOverclaim,
                        format!("case {label} overclaims confidence for its source/capability"),
                    ));
                }
                ConformanceDimension::RawPayloadRetention => {
                    findings.push(InteropConformanceValidationFinding::blocker(
                        InteropConformanceFindingKind::RawPayloadNotRetained,
                        format!("case {label} does not retain its raw payload safely"),
                    ));
                }
                ConformanceDimension::ReplayStability => {
                    findings.push(InteropConformanceValidationFinding::blocker(
                        InteropConformanceFindingKind::ReplayUnstable,
                        format!("case {label} does not replay deterministically"),
                    ));
                }
                ConformanceDimension::DegradedStateBehavior => {
                    findings.push(InteropConformanceValidationFinding::blocker(
                        InteropConformanceFindingKind::DegradedStateNotDisclosed,
                        format!("case {label} hides its degraded/unsupported capability state"),
                    ));
                }
                ConformanceDimension::ExportParity => {
                    findings.push(InteropConformanceValidationFinding::blocker(
                        InteropConformanceFindingKind::ExportParityBroken,
                        format!("case {label} breaks support/release/AI export parity"),
                    ));
                }
            }
        }

        if include_record_fields {
            if case.dimension_outcomes != outcomes {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::DimensionOutcomeDrift,
                    format!("case {label} stored dimension outcomes disagree with the derivation"),
                ));
            }
            if case.conforms != case_conforms(&outcomes) {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::CaseConformanceDrift,
                    format!("case {label} stored conformance disagrees with the derivation"),
                ));
            }
        }
    }

    fn check_release_evidence(
        &self,
        findings: &mut Vec<InteropConformanceValidationFinding>,
        include_record_fields: bool,
    ) {
        if self.release_evidence.release_evidence_ref.trim().is_empty() {
            findings.push(InteropConformanceValidationFinding::blocker(
                InteropConformanceFindingKind::ReleaseEvidenceMissing,
                "release-evidence binding ref is required",
            ));
        }
        if include_record_fields {
            let expected =
                derive_release_evidence(&self.release_evidence.release_evidence_ref, &self.corpora);
            if self.release_evidence != expected {
                findings.push(InteropConformanceValidationFinding::blocker(
                    InteropConformanceFindingKind::ReleaseEvidenceDrift,
                    "stored release-evidence binding disagrees with the corpora",
                ));
            }
        }
    }
}

/// Support-export wrapper carrying the exact conformance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteropConformanceSupportExport {
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
    pub packet: InteropConformancePacket,
}

impl InteropConformanceSupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == INTEROP_CONFORMANCE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == INTEROP_CONFORMANCE_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.packet_id_ref == self.packet.packet_id
            && self.packet.is_stable()
    }
}

/// One corpus row for an evidence join or CLI/headless view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteropCorpusRow {
    /// Corpus family token.
    pub family: String,
    /// Source kind token.
    pub source_kind: String,
    /// Repo-relative fixture directory.
    pub fixture_dir: String,
    /// Freshness state token.
    pub freshness_state: String,
    /// Proof age in days.
    pub proof_age_days: u32,
    /// Freshness window in days.
    pub freshness_window_days: u32,
    /// Number of cases in the corpus.
    pub case_count: usize,
    /// True when every case conforms.
    pub all_cases_conform: bool,
    /// True when the corpus is current and conforming.
    pub release_ready: bool,
    /// Support-safe explanation of the corpus.
    pub explanation: String,
}

impl InteropCorpusRow {
    fn from_corpus(corpus: &InteropCorpus) -> Self {
        let explanation = format!(
            "{} corpus ({}) ran {} case(s); freshness={} ({}d/{}d), conforming={}",
            corpus.family.as_str(),
            corpus.source_kind.as_str(),
            corpus.cases.len(),
            corpus.freshness_state.as_str(),
            corpus.proof_age_days,
            corpus.freshness_window_days,
            corpus.all_cases_conform,
        );
        Self {
            family: corpus.family.as_str().to_owned(),
            source_kind: corpus.source_kind.as_str().to_owned(),
            fixture_dir: corpus.fixture_dir.clone(),
            freshness_state: corpus.freshness_state.as_str().to_owned(),
            proof_age_days: corpus.proof_age_days,
            freshness_window_days: corpus.freshness_window_days,
            case_count: corpus.cases.len(),
            all_cases_conform: corpus.all_cases_conform,
            release_ready: corpus.is_release_ready(),
            explanation,
        }
    }
}

/// One case row for an evidence join or CLI/headless view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceCaseRow {
    /// Stable case id.
    pub case_id: String,
    /// Corpus family token.
    pub family: String,
    /// Archetype token.
    pub archetype: String,
    /// Source kind token.
    pub source_kind: String,
    /// Capability state token.
    pub capability_state: String,
    /// Confidence token.
    pub confidence: String,
    /// Named fallback reason, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<String>,
    /// True when every dimension passes.
    pub conforms: bool,
    /// Dimension tokens that failed (empty when conforming).
    #[serde(default)]
    pub failed_dimensions: Vec<String>,
    /// Support-safe explanation of the case.
    pub explanation: String,
}

impl ConformanceCaseRow {
    fn from_case(case: &ConformanceCase) -> Self {
        let failed_dimensions: Vec<String> = case
            .dimension_outcomes
            .iter()
            .filter(|outcome| !outcome.passed)
            .map(|outcome| outcome.dimension.as_str().to_owned())
            .collect();
        let explanation = format!(
            "{} on {} via {} capability ({} confidence); conforms={}{}",
            case.family.as_str(),
            case.archetype.as_str(),
            case.negotiated_capability.as_str(),
            case.observed_confidence.as_str(),
            case.conforms,
            if failed_dimensions.is_empty() {
                String::new()
            } else {
                format!(", failed=[{}]", failed_dimensions.join(","))
            },
        );
        Self {
            case_id: case.case_id.clone(),
            family: case.family.as_str().to_owned(),
            archetype: case.archetype.as_str().to_owned(),
            source_kind: case.source_kind.as_str().to_owned(),
            capability_state: case.negotiated_capability.as_str().to_owned(),
            confidence: case.observed_confidence.as_str().to_owned(),
            fallback_reason: case.fallback_reason.clone(),
            conforms: case.conforms,
            failed_dimensions,
            explanation,
        }
    }
}

/// Evidence-join view for one export/evidence surface (support, incident, AI).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteropConformanceEvidenceJoinView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Evidence surface this view serves.
    pub surface: ConformanceEvidenceSurface,
    /// View timestamp.
    pub generated_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Order-invariant digest of the corpora.
    pub corpus_digest: String,
    /// Release-evidence binding.
    pub release_evidence: ReleaseEvidenceBinding,
    /// Corpus rows.
    #[serde(default)]
    pub corpus_rows: Vec<InteropCorpusRow>,
    /// Case rows.
    #[serde(default)]
    pub case_rows: Vec<ConformanceCaseRow>,
}

impl InteropConformanceEvidenceJoinView {
    /// Returns true when every row keeps its explanation and provenance fields.
    pub fn explains_consistently(&self) -> bool {
        let corpora_ok = self.corpus_rows.iter().all(|row| {
            !row.family.trim().is_empty()
                && !row.fixture_dir.trim().is_empty()
                && !row.explanation.trim().is_empty()
        });
        let cases_ok = self.case_rows.iter().all(|row| {
            !row.case_id.trim().is_empty()
                && !row.source_kind.trim().is_empty()
                && !row.explanation.trim().is_empty()
        });
        corpora_ok && cases_ok
    }
}

/// CLI/headless stable view of the conformance suite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteropConformanceCliHeadlessView {
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
    /// Order-invariant digest of the corpora.
    pub corpus_digest: String,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Release-evidence binding.
    pub release_evidence: ReleaseEvidenceBinding,
    /// Corpus rows.
    #[serde(default)]
    pub corpus_rows: Vec<InteropCorpusRow>,
    /// Case rows.
    #[serde(default)]
    pub case_rows: Vec<ConformanceCaseRow>,
}

impl InteropConformanceCliHeadlessView {
    /// Returns true when every corpus row ran at least one explained case.
    pub fn every_corpus_runs(&self) -> bool {
        self.corpus_rows
            .iter()
            .all(|row| row.case_count > 0 && !row.explanation.trim().is_empty())
    }
}

fn display_ref(value: &str) -> &str {
    if value.trim().is_empty() {
        "<missing>"
    } else {
        value
    }
}

fn case_conforms(outcomes: &[DimensionOutcome]) -> bool {
    outcomes.iter().all(|outcome| outcome.passed)
}

fn freshness_for(proof_age_days: u32, freshness_window_days: u32) -> EvidenceFreshnessState {
    if proof_age_days > freshness_window_days {
        EvidenceFreshnessState::Stale
    } else {
        EvidenceFreshnessState::Current
    }
}

fn derive_corpus(mut corpus: InteropCorpus) -> InteropCorpus {
    for case in &mut corpus.cases {
        case.dimension_outcomes = case.evaluate_dimensions();
        case.conforms = case_conforms(&case.dimension_outcomes);
    }
    corpus.freshness_state = freshness_for(corpus.proof_age_days, corpus.freshness_window_days);
    corpus.all_cases_conform =
        !corpus.cases.is_empty() && corpus.cases.iter().all(|case| case.conforms);
    corpus
}

fn derive_release_evidence(
    release_evidence_ref: &str,
    corpora: &[InteropCorpus],
) -> ReleaseEvidenceBinding {
    let all_corpora_current = corpora
        .iter()
        .all(|corpus| corpus.freshness_state == EvidenceFreshnessState::Current);
    let all_cases_conform =
        !corpora.is_empty() && corpora.iter().all(|corpus| corpus.all_cases_conform);
    let narrowed_families: Vec<String> = corpora
        .iter()
        .filter(|corpus| !corpus.is_release_ready())
        .map(|corpus| corpus.family.as_str().to_owned())
        .collect();
    let case_count: usize = corpora.iter().map(|corpus| corpus.cases.len()).sum();
    let conformance_summary = format!(
        "{} corpora, {} cases; current={}, conforming={}, narrowed={}",
        corpora.len(),
        case_count,
        all_corpora_current,
        all_cases_conform,
        narrowed_families.len(),
    );
    ReleaseEvidenceBinding {
        release_evidence_ref: release_evidence_ref.to_owned(),
        all_corpora_current,
        all_cases_conform,
        narrowed_families,
        conformance_summary,
    }
}

/// Order-invariant FNV-1a 64-bit digest of every case id.
fn corpus_digest(corpora: &[InteropCorpus]) -> String {
    let mut ids: Vec<&str> = corpora
        .iter()
        .flat_map(|corpus| corpus.cases.iter())
        .map(|case| case.case_id.as_str())
        .collect();
    ids.sort_unstable();
    fnv1a64(&ids)
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
    findings: &[InteropConformanceValidationFinding],
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

/// Builds the canonical stable interop conformance packet input.
pub fn current_stable_interop_conformance_input() -> InteropConformancePacketInput {
    InteropConformancePacketInput {
        packet_id: INTEROP_CONFORMANCE_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        release_evidence_ref: INTEROP_CONFORMANCE_RELEASE_EVIDENCE_REF.to_owned(),
        corpora: CorpusFamily::ALL
            .into_iter()
            .map(canonical_corpus)
            .collect(),
    }
}

/// Materializes the canonical stable interop conformance packet.
pub fn seeded_interop_conformance_packet() -> InteropConformancePacket {
    InteropConformancePacket::materialize(current_stable_interop_conformance_input())
}

/// Validates a packet and returns an `Ok(())` / findings result.
pub fn validate_interop_conformance_packet(
    packet: &InteropConformancePacket,
) -> Result<(), Vec<InteropConformanceValidationFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn canonical_corpus(family: CorpusFamily) -> InteropCorpus {
    let cases = archetypes_for_family(family)
        .into_iter()
        .map(|archetype| conforming_case(family, archetype))
        .collect();
    InteropCorpus {
        family,
        source_kind: family.source_kind(),
        fixture_dir: family.fixture_dir().to_owned(),
        last_run_at: "2026-06-15T00:00:00Z".to_owned(),
        proof_age_days: 3,
        freshness_window_days: 30,
        // Overwritten by `derive_corpus` at materialization.
        freshness_state: EvidenceFreshnessState::Current,
        cases,
        all_cases_conform: false,
    }
}

fn conforming_case(family: CorpusFamily, archetype: InteropArchetype) -> ConformanceCase {
    let source_kind = family.source_kind();
    // Heuristic fallbacks negotiate a degraded capability with a named reason and
    // visibly downgraded, low-confidence truth; first-party protocol and
    // structured importers negotiate cleanly.
    let (capability, confidence, fallback_reason, degraded_disclosed, retention) =
        if family.is_heuristic() {
            (
                AdapterCapabilityState::Degraded,
                BuildTestEventConfidence::Low,
                Some("heuristic_parser_fallback".to_owned()),
                true,
                RawPayloadRetentionClass::MetadataDigestOnly,
            )
        } else if matches!(family, CorpusFamily::StructuredOutputJunitSarif) {
            (
                AdapterCapabilityState::Negotiated,
                BuildTestEventConfidence::MediumHigh,
                None,
                false,
                RawPayloadRetentionClass::RedactedReference,
            )
        } else {
            (
                AdapterCapabilityState::Negotiated,
                BuildTestEventConfidence::High,
                None,
                false,
                RawPayloadRetentionClass::MetadataDigestOnly,
            )
        };
    let case_id = format!("case:{}:{}", family.as_str(), archetype.as_str());
    let raw_payload_ref = format!("raw:{}:{}", family.as_str(), archetype.as_str());
    ConformanceCase {
        scenario: format!(
            "{} archetype exercises the {} corpus end to end",
            archetype.as_str(),
            family.as_str()
        ),
        payload_digest: crate::digest::sha256_token(raw_payload_ref.as_bytes()),
        case_id,
        family,
        archetype,
        source_kind,
        negotiated_capability: capability,
        capability_packet_ref: format!(
            "capability-packet:{}:{}",
            family.as_str(),
            archetype.as_str()
        ),
        fallback_reason,
        observed_confidence: confidence,
        raw_payload_ref,
        raw_payload_retention: retention,
        raw_private_material_excluded: true,
        replay_stable: true,
        export_parity_preserved: true,
        degraded_state_disclosed: degraded_disclosed,
        // Overwritten by `derive_corpus` at materialization.
        dimension_outcomes: Vec::new(),
        conforms: false,
    }
}
