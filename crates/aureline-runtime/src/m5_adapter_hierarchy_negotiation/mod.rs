//! Per-ecosystem adapter capability negotiation and ordered resolution.
//!
//! Where [`crate::m5_task_event_adapter_policy`] freezes the global adapter
//! ladder (native, BSP, Bazel BEP/BES, structured output, heuristic parser) and
//! the closed downgrade vocabulary, this module makes the *choice* a governed
//! product object. For each claimed build/test ecosystem it walks that ladder in
//! priority order, records which adapter was negotiated for execution truth, and
//! keeps an explicit fallback-reason packet naming why every higher-priority
//! adapter was skipped. Unsupported capabilities on the resolved adapter stay
//! named rather than inferred from missing rows, and capability drift is surfaced
//! before it can quietly degrade trust.
//!
//! The invariant this layer protects is honesty about fallback: a lower-priority
//! adapter never silently displaces a higher-confidence source merely because it
//! arrived later or is easier to render. If a higher-priority adapter was
//! available and could negotiate a usable capability, it must win; otherwise the
//! resolution carries an explicit, closed-vocabulary reason for skipping it.
//! Structured-output and heuristic-parser resolutions are always visibly
//! downgraded so users can distinguish them from native/BSP/BEP truth.
//!
//! It reuses the [`crate::build_test_event_interoperability`] source-kind,
//! confidence, capability-state, severity, and promotion vocabulary and the
//! [`crate::m5_task_event_adapter_policy`] priority rank, confidence ceiling,
//! authority, and downgrade-reason helpers rather than minting parallel tokens.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/adapter-hierarchy-and-negotiation.md`](../../../docs/m5/adapter-hierarchy-and-negotiation.md);
//! the machine-readable boundary lives at
//! [`/schemas/tooling/adapter-negotiation.schema.json`](../../../schemas/tooling/adapter-negotiation.schema.json).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::build_test_event_interoperability::{
    AdapterCapabilityState, BuildTestEventConfidence, BuildTestEventSourceKind,
    BuildTestInteropFindingSeverity, BuildTestInteropPromotionState,
};
use crate::m5_task_event_adapter_policy::{
    canonical_confidence_ceiling, canonical_priority_rank, source_is_authoritative, DowngradeReason,
};

/// Stable record-kind tag for [`AdapterNegotiationBaseline`].
pub const ADAPTER_NEGOTIATION_RECORD_KIND: &str = "m5_adapter_hierarchy_negotiation_baseline";

/// Stable record-kind tag for [`AdapterNegotiationSupportExport`].
pub const ADAPTER_NEGOTIATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_adapter_hierarchy_negotiation_support_export";

/// Integer schema version for the negotiation baseline.
pub const ADAPTER_NEGOTIATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the adapter-negotiation boundary schema.
pub const ADAPTER_NEGOTIATION_SCHEMA_REF: &str = "schemas/tooling/adapter-negotiation.schema.json";

/// Repo-relative path of the per-event task-event envelope boundary schema.
pub const ADAPTER_NEGOTIATION_ENVELOPE_SCHEMA_REF: &str =
    "schemas/tooling/task-event-envelope.schema.json";

/// Repo-relative path of the frozen adapter-policy boundary schema this lane extends.
pub const ADAPTER_NEGOTIATION_POLICY_SCHEMA_REF: &str =
    "schemas/tooling/adapter-capability.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const ADAPTER_NEGOTIATION_DOC_REF: &str = "docs/m5/adapter-hierarchy-and-negotiation.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const ADAPTER_NEGOTIATION_FIXTURE_DIR: &str = "fixtures/tooling/m5/bsp-bep-heuristic-fallbacks";

/// Repo-relative path of the checked-in baseline artifact.
pub const ADAPTER_NEGOTIATION_BASELINE_ARTIFACT_REF: &str =
    "artifacts/m5/tooling/adapter-negotiation/baseline.json";

/// Stable baseline id minted by the seed.
pub const ADAPTER_NEGOTIATION_BASELINE_ID: &str = "tooling:m5:adapter-hierarchy-negotiation:v1";

/// Stable support-export id minted by the seed inspector.
pub const ADAPTER_NEGOTIATION_SUPPORT_EXPORT_ID: &str =
    "support-export:tooling:m5:adapter-hierarchy-negotiation";

/// Claimed build/test ecosystem that negotiates an adapter for execution truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ecosystem {
    /// Cargo / Rust, served by the first-party native adapter.
    Cargo,
    /// JVM build tools (Gradle, sbt, Maven) reached over the Build Server Protocol.
    GradleJvm,
    /// Bazel, reached over the Build Event Protocol / Build Event Service.
    Bazel,
    /// Python / pytest, reached through structured JUnit/JSON output.
    PythonPytest,
    /// Node.js test runners, reached through structured JSON output.
    NodeJs,
    /// Unknown or unstructured tooling, reachable only by a heuristic parser.
    Generic,
}

impl Ecosystem {
    /// Every claimed ecosystem in stable declaration order.
    pub const ALL: [Self; 6] = [
        Self::Cargo,
        Self::GradleJvm,
        Self::Bazel,
        Self::PythonPytest,
        Self::NodeJs,
        Self::Generic,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::GradleJvm => "gradle_jvm",
            Self::Bazel => "bazel",
            Self::PythonPytest => "python_pytest",
            Self::NodeJs => "node_js",
            Self::Generic => "generic",
        }
    }
}

/// Capability a consumer negotiates from an adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NegotiatedCapability {
    /// Target / dependency graph discovery.
    TargetGraph,
    /// Task lifecycle events (queued, started, finished).
    LifecycleEvents,
    /// Diagnostic / problem events.
    Diagnostics,
    /// Test case start and finish events.
    TestEvents,
    /// Published artifact references.
    Artifacts,
    /// Progress updates.
    Progress,
}

impl NegotiatedCapability {
    /// Every negotiated capability in stable declaration order.
    pub const ALL: [Self; 6] = [
        Self::TargetGraph,
        Self::LifecycleEvents,
        Self::Diagnostics,
        Self::TestEvents,
        Self::Artifacts,
        Self::Progress,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetGraph => "target_graph",
            Self::LifecycleEvents => "lifecycle_events",
            Self::Diagnostics => "diagnostics",
            Self::TestEvents => "test_events",
            Self::Artifacts => "artifacts",
            Self::Progress => "progress",
        }
    }
}

/// Why a resolution landed where it did on the adapter ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackClass {
    /// First-party native adapter truth; no fallback.
    NativeAuthoritative,
    /// Negotiated-protocol truth (BSP or Bazel BEP/BES); native unavailable.
    NegotiatedProtocol,
    /// Imported structured output; no authoritative adapter available.
    StructuredImport,
    /// Heuristic parser stood in as the last resort.
    HeuristicLastResort,
}

impl FallbackClass {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeAuthoritative => "native_authoritative",
            Self::NegotiatedProtocol => "negotiated_protocol",
            Self::StructuredImport => "structured_import",
            Self::HeuristicLastResort => "heuristic_last_resort",
        }
    }
}

/// Canonical fallback class for a resolved source kind.
pub const fn fallback_class_for(source_kind: BuildTestEventSourceKind) -> FallbackClass {
    match source_kind {
        BuildTestEventSourceKind::Native => FallbackClass::NativeAuthoritative,
        BuildTestEventSourceKind::Bsp | BuildTestEventSourceKind::BazelBep => {
            FallbackClass::NegotiatedProtocol
        }
        BuildTestEventSourceKind::StructuredOutput => FallbackClass::StructuredImport,
        BuildTestEventSourceKind::HeuristicParser => FallbackClass::HeuristicLastResort,
    }
}

/// Downgrade reason a non-authoritative fallback class must carry, if any.
pub const fn downgrade_reason_for_fallback(class: FallbackClass) -> Option<DowngradeReason> {
    match class {
        FallbackClass::NativeAuthoritative | FallbackClass::NegotiatedProtocol => None,
        FallbackClass::StructuredImport => Some(DowngradeReason::PartialSupport),
        FallbackClass::HeuristicLastResort => Some(DowngradeReason::HeuristicFallback),
    }
}

/// Why a higher-priority adapter was not selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkipReason {
    /// The adapter is not installed or reachable.
    AdapterUnavailable,
    /// The source kind does not apply to this ecosystem.
    EcosystemUnsupported,
    /// The adapter is reachable but could not negotiate any usable capability.
    CapabilityUnsupported,
    /// The adapter is reachable but the capability handshake failed.
    NegotiationFailed,
}

impl SkipReason {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterUnavailable => "adapter_unavailable",
            Self::EcosystemUnsupported => "ecosystem_unsupported",
            Self::CapabilityUnsupported => "capability_unsupported",
            Self::NegotiationFailed => "negotiation_failed",
        }
    }
}

/// Class of adapter-capability drift surfaced before it degrades trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftClass {
    /// A previously supported capability is now unsupported.
    CapabilityLost,
    /// A previously fully negotiated capability is now degraded.
    CapabilityDegraded,
    /// The resolved confidence regressed against the prior baseline.
    ConfidenceRegressed,
    /// The resolution moved to a lower rung than the prior baseline.
    FallbackDeepened,
    /// A previously available adapter is now unavailable.
    AdapterUnavailable,
}

impl DriftClass {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityLost => "capability_lost",
            Self::CapabilityDegraded => "capability_degraded",
            Self::ConfidenceRegressed => "confidence_regressed",
            Self::FallbackDeepened => "fallback_deepened",
            Self::AdapterUnavailable => "adapter_unavailable",
        }
    }
}

/// Consumer surface that must disclose the negotiation outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureSurface {
    /// Run / test / debug / pipeline UI badges.
    Ui,
    /// CLI / headless stable JSON surface.
    CliHeadless,
    /// AI explanations and evidence callouts.
    AiEvidence,
    /// Support and release export packets.
    SupportExport,
}

impl DisclosureSurface {
    /// Every surface that must disclose the negotiation outcome.
    pub const REQUIRED: [Self; 4] = [
        Self::Ui,
        Self::CliHeadless,
        Self::AiEvidence,
        Self::SupportExport,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ui => "ui",
            Self::CliHeadless => "cli_headless",
            Self::AiEvidence => "ai_evidence",
            Self::SupportExport => "support_export",
        }
    }
}

/// Closed validation finding vocabulary for the negotiation baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NegotiationFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// The resolutions do not cover every ecosystem exactly once.
    EcosystemCoverageIncomplete,
    /// A resolution's candidate ladder does not cover every source kind in order.
    CandidateLadderIncomplete,
    /// A candidate's rank disagrees with its canonical adapter rank.
    CandidateRankMismatch,
    /// A resolution does not name exactly one selected, consistent candidate.
    SelectionInvalid,
    /// The selected candidate is not eligible to serve execution truth.
    SelectedCandidateIneligible,
    /// A higher-priority eligible adapter was displaced by a lower one.
    LowerPriorityDisplacedHigher,
    /// A higher-than-selected candidate lacks a skip reason.
    SkipReasonMissing,
    /// A skip reason disagrees with the candidate's availability and capabilities.
    SkipReasonInconsistent,
    /// The fallback class disagrees with the selected source kind.
    FallbackClassMismatch,
    /// The resolution overclaims confidence for its selected source.
    ConfidenceOverclaim,
    /// The downgrade flag/reason disagrees with the fallback class.
    DowngradeInconsistent,
    /// The selected candidate does not disclose every negotiated capability.
    CapabilityCoverageIncomplete,
    /// The named unsupported capabilities disagree with the selected adapter.
    UnsupportedCapabilityUnnamed,
    /// The explicit fallback-reason packet disagrees with the candidate ladder.
    FallbackReasonPacketMismatch,
    /// A capability-drift signal is not visibly surfaced.
    DriftNotVisible,
    /// A required disclosure surface is absent.
    DisclosureSurfaceMissing,
    /// A disclosure surface drops negotiation truth.
    DisclosureSurfaceDropsTruth,
    /// Stored promotion state disagrees with the derived state.
    PromotionStateMismatch,
}

impl NegotiationFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::EcosystemCoverageIncomplete => "ecosystem_coverage_incomplete",
            Self::CandidateLadderIncomplete => "candidate_ladder_incomplete",
            Self::CandidateRankMismatch => "candidate_rank_mismatch",
            Self::SelectionInvalid => "selection_invalid",
            Self::SelectedCandidateIneligible => "selected_candidate_ineligible",
            Self::LowerPriorityDisplacedHigher => "lower_priority_displaced_higher",
            Self::SkipReasonMissing => "skip_reason_missing",
            Self::SkipReasonInconsistent => "skip_reason_inconsistent",
            Self::FallbackClassMismatch => "fallback_class_mismatch",
            Self::ConfidenceOverclaim => "confidence_overclaim",
            Self::DowngradeInconsistent => "downgrade_inconsistent",
            Self::CapabilityCoverageIncomplete => "capability_coverage_incomplete",
            Self::UnsupportedCapabilityUnnamed => "unsupported_capability_unnamed",
            Self::FallbackReasonPacketMismatch => "fallback_reason_packet_mismatch",
            Self::DriftNotVisible => "drift_not_visible",
            Self::DisclosureSurfaceMissing => "disclosure_surface_missing",
            Self::DisclosureSurfaceDropsTruth => "disclosure_surface_drops_truth",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Numeric weight used to compare confidence levels (higher is stronger).
const fn confidence_weight(confidence: BuildTestEventConfidence) -> u8 {
    match confidence {
        BuildTestEventConfidence::High => 4,
        BuildTestEventConfidence::MediumHigh => 3,
        BuildTestEventConfidence::Medium => 2,
        BuildTestEventConfidence::Low => 1,
    }
}

/// True when `confidence` is above the ceiling allowed for `source_kind`.
fn confidence_overclaims(
    confidence: BuildTestEventConfidence,
    source_kind: BuildTestEventSourceKind,
) -> bool {
    confidence_weight(confidence) > confidence_weight(canonical_confidence_ceiling(source_kind))
}

/// One negotiated capability state on a candidate adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityNegotiation {
    /// Capability negotiated.
    pub capability: NegotiatedCapability,
    /// Negotiated state (negotiated, degraded, or unsupported).
    pub state: AdapterCapabilityState,
    /// Raw handshake / capability-packet ref.
    pub capability_packet_ref: String,
    /// Short reviewer-facing note.
    pub note: String,
}

/// One adapter considered for an ecosystem, in priority order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterCandidate {
    /// Source kind this candidate covers.
    pub source_kind: BuildTestEventSourceKind,
    /// Priority rank (must match the source's canonical rank).
    pub priority_rank: u8,
    /// Adapter id.
    pub adapter_id: String,
    /// True when the adapter is installed and reachable.
    pub available: bool,
    /// True when the adapter is reachable but the capability handshake failed.
    pub negotiation_failed: bool,
    /// True when this candidate produced the resolution's execution truth.
    pub selected: bool,
    /// Reason this candidate was skipped, present iff it was a higher rung skipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_reason: Option<SkipReason>,
    /// Per-capability negotiated states on this candidate.
    #[serde(default)]
    pub capabilities: Vec<CapabilityNegotiation>,
}

impl AdapterCandidate {
    /// True when the candidate can serve execution truth.
    fn eligible(&self) -> bool {
        self.available
            && !self.negotiation_failed
            && self
                .capabilities
                .iter()
                .any(|cap| cap.state != AdapterCapabilityState::Unsupported)
    }

    /// True when `skip_reason` agrees with this candidate's posture.
    fn skip_reason_consistent(&self, reason: SkipReason) -> bool {
        match reason {
            SkipReason::AdapterUnavailable | SkipReason::EcosystemUnsupported => !self.available,
            SkipReason::NegotiationFailed => self.available && self.negotiation_failed,
            SkipReason::CapabilityUnsupported => {
                self.available
                    && !self.negotiation_failed
                    && self
                        .capabilities
                        .iter()
                        .all(|cap| cap.state == AdapterCapabilityState::Unsupported)
            }
        }
    }

    /// Capabilities the candidate explicitly reports as unsupported, in canonical order.
    fn unsupported_capabilities(&self) -> Vec<NegotiatedCapability> {
        NegotiatedCapability::ALL
            .into_iter()
            .filter(|capability| {
                self.capabilities.iter().any(|cap| {
                    cap.capability == *capability
                        && cap.state == AdapterCapabilityState::Unsupported
                })
            })
            .collect()
    }
}

/// One entry in the explicit fallback-reason packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkippedAdapterReason {
    /// Source kind that was skipped.
    pub source_kind: BuildTestEventSourceKind,
    /// Skipped adapter id.
    pub adapter_id: String,
    /// Why this higher-priority adapter was skipped.
    pub skip_reason: SkipReason,
    /// Short reviewer-facing summary.
    pub summary: String,
}

/// One per-ecosystem adapter resolution and its fallback-reason packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemAdapterResolution {
    /// Stable resolution id.
    pub resolution_id: String,
    /// Ecosystem this resolution covers.
    pub ecosystem: Ecosystem,
    /// Workspace or workset identity.
    pub workspace_id: String,
    /// Build target, task, or test suite identity.
    pub target_id: String,
    /// Ordered candidate ladder considered for this ecosystem.
    #[serde(default)]
    pub candidate_ladder: Vec<AdapterCandidate>,
    /// Source kind that produced execution truth.
    pub selected_source_kind: BuildTestEventSourceKind,
    /// Selected adapter id.
    pub selected_adapter_id: String,
    /// Fallback class for the selected source.
    pub fallback_class: FallbackClass,
    /// Confidence asserted (at or below the source's ceiling).
    pub confidence: BuildTestEventConfidence,
    /// True when the resolution is visibly downgraded on every surface.
    pub downgraded: bool,
    /// Downgrade reason, present iff the resolution is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_reason: Option<DowngradeReason>,
    /// Explicit fallback-reason packet: every higher rung skipped, and why.
    #[serde(default)]
    pub fallback_reasons: Vec<SkippedAdapterReason>,
    /// Capabilities the selected adapter explicitly does not support.
    #[serde(default)]
    pub unsupported_capabilities: Vec<NegotiatedCapability>,
}

impl EcosystemAdapterResolution {
    fn selected_candidate(&self) -> Option<&AdapterCandidate> {
        let selected: Vec<&AdapterCandidate> = self
            .candidate_ladder
            .iter()
            .filter(|c| c.selected)
            .collect();
        if selected.len() == 1 {
            Some(selected[0])
        } else {
            None
        }
    }

    fn derived_fallback_reasons(
        &self,
        selected_rank: u8,
    ) -> Vec<(BuildTestEventSourceKind, SkipReason, &str)> {
        self.candidate_ladder
            .iter()
            .filter(|c| c.priority_rank < selected_rank)
            .filter_map(|c| {
                c.skip_reason
                    .map(|reason| (c.source_kind, reason, c.adapter_id.as_str()))
            })
            .collect()
    }
}

/// One capability-drift signal surfaced before it degrades trust.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDriftSignal {
    /// Stable drift id.
    pub drift_id: String,
    /// Ecosystem whose adapter drifted.
    pub ecosystem: Ecosystem,
    /// Source kind whose adapter drifted.
    pub source_kind: BuildTestEventSourceKind,
    /// Capability that drifted, absent for adapter-level drift.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<NegotiatedCapability>,
    /// Drift class.
    pub drift_class: DriftClass,
    /// Prior state token.
    pub previous_state: String,
    /// Current state token.
    pub current_state: String,
    /// True when the drift is surfaced before it degrades trust.
    pub visible_before_trust_loss: bool,
    /// Short reviewer-facing summary.
    pub summary: String,
}

impl CapabilityDriftSignal {
    fn is_visible(&self) -> bool {
        self.visible_before_trust_loss
            && !self.drift_id.trim().is_empty()
            && !self.previous_state.trim().is_empty()
            && !self.current_state.trim().is_empty()
            && !self.summary.trim().is_empty()
    }
}

/// Binding proving a consumer surface discloses the negotiation outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegotiationDisclosureBinding {
    /// Consumer surface.
    pub surface: DisclosureSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// True when the surface discloses the selected source kind.
    pub discloses_selected_source_kind: bool,
    /// True when the surface discloses the fallback reason packet.
    pub discloses_fallback_reason: bool,
    /// True when the surface names unsupported capabilities.
    pub discloses_unsupported_capabilities: bool,
    /// True when the surface surfaces capability drift.
    pub discloses_capability_drift: bool,
    /// True when the surface discloses confidence.
    pub discloses_confidence: bool,
}

impl NegotiationDisclosureBinding {
    fn preserves_truth(&self) -> bool {
        !self.binding_ref.trim().is_empty()
            && self.discloses_selected_source_kind
            && self.discloses_fallback_reason
            && self.discloses_unsupported_capabilities
            && self.discloses_capability_drift
            && self.discloses_confidence
    }
}

/// One validation finding emitted by the negotiation validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegotiationValidationFinding {
    /// Closed finding kind.
    pub finding_kind: NegotiationFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl NegotiationValidationFinding {
    fn blocker(finding_kind: NegotiationFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`AdapterNegotiationBaseline::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterNegotiationBaselineInput {
    /// Stable baseline id.
    pub baseline_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Per-ecosystem resolutions.
    #[serde(default)]
    pub resolutions: Vec<EcosystemAdapterResolution>,
    /// Capability-drift signals.
    #[serde(default)]
    pub drift_signals: Vec<CapabilityDriftSignal>,
    /// Disclosure-surface bindings.
    #[serde(default)]
    pub disclosure_surfaces: Vec<NegotiationDisclosureBinding>,
}

/// Per-ecosystem adapter capability negotiation and resolution baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterNegotiationBaseline {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable baseline id.
    pub baseline_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Negotiation boundary schema ref.
    pub negotiation_schema_ref: String,
    /// Per-event task-event envelope boundary schema ref.
    pub envelope_schema_ref: String,
    /// Frozen adapter-policy boundary schema ref this lane extends.
    pub policy_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Per-ecosystem resolutions.
    #[serde(default)]
    pub resolutions: Vec<EcosystemAdapterResolution>,
    /// Capability-drift signals.
    #[serde(default)]
    pub drift_signals: Vec<CapabilityDriftSignal>,
    /// Disclosure-surface bindings.
    #[serde(default)]
    pub disclosure_surfaces: Vec<NegotiationDisclosureBinding>,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<NegotiationValidationFinding>,
}

impl AdapterNegotiationBaseline {
    /// Materializes a baseline and records derived validation findings.
    pub fn materialize(input: AdapterNegotiationBaselineInput) -> Self {
        let mut baseline = Self {
            record_kind: ADAPTER_NEGOTIATION_RECORD_KIND.to_owned(),
            schema_version: ADAPTER_NEGOTIATION_SCHEMA_VERSION,
            baseline_id: input.baseline_id,
            generated_at: input.generated_at,
            negotiation_schema_ref: ADAPTER_NEGOTIATION_SCHEMA_REF.to_owned(),
            envelope_schema_ref: ADAPTER_NEGOTIATION_ENVELOPE_SCHEMA_REF.to_owned(),
            policy_schema_ref: ADAPTER_NEGOTIATION_POLICY_SCHEMA_REF.to_owned(),
            doc_ref: ADAPTER_NEGOTIATION_DOC_REF.to_owned(),
            resolutions: input.resolutions,
            drift_signals: input.drift_signals,
            disclosure_surfaces: input.disclosure_surfaces,
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = baseline.derived_findings(false);
        baseline.promotion_state = promotion_state_for_findings(&findings);
        baseline.validation_findings = findings;
        baseline
    }

    /// Re-validates the baseline against the frozen negotiation invariants.
    pub fn validate(&self) -> Vec<NegotiationValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    /// Builds an export-safe support packet carrying the exact baseline.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> AdapterNegotiationSupportExport {
        AdapterNegotiationSupportExport {
            record_kind: ADAPTER_NEGOTIATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ADAPTER_NEGOTIATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            baseline_id_ref: self.baseline_id.clone(),
            baseline: self.clone(),
        }
    }

    /// Returns the ecosystem tokens present in the resolutions.
    pub fn ecosystem_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for resolution in &self.resolutions {
            set.insert(resolution.ecosystem);
        }
        set.into_iter().map(Ecosystem::as_str).collect()
    }

    /// Returns the selected source-kind tokens present in the resolutions.
    pub fn selected_source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for resolution in &self.resolutions {
            set.insert(resolution.selected_source_kind);
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Returns the fallback-class tokens present in the resolutions.
    pub fn fallback_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for resolution in &self.resolutions {
            set.insert(resolution.fallback_class);
        }
        set.into_iter().map(FallbackClass::as_str).collect()
    }

    /// Returns the drift-class tokens present in the drift signals.
    pub fn drift_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for signal in &self.drift_signals {
            set.insert(signal.drift_class);
        }
        set.into_iter().map(DriftClass::as_str).collect()
    }

    /// Returns the disclosure-surface tokens present in the bindings.
    pub fn disclosure_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for binding in &self.disclosure_surfaces {
            set.insert(binding.surface);
        }
        set.into_iter().map(DisclosureSurface::as_str).collect()
    }

    /// Compact, support-safe one-line-per-row rendering for the inspector.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "baseline {} schema_version={} promotion={}",
            self.baseline_id,
            self.schema_version,
            self.promotion_state.as_str()
        ));
        for resolution in &self.resolutions {
            lines.push(format!(
                "resolution {} selected={} class={} confidence={} downgraded={} skipped={} unsupported={}",
                resolution.ecosystem.as_str(),
                resolution.selected_source_kind.as_str(),
                resolution.fallback_class.as_str(),
                resolution.confidence.as_str(),
                resolution.downgraded,
                resolution.fallback_reasons.len(),
                resolution.unsupported_capabilities.len(),
            ));
            for reason in &resolution.fallback_reasons {
                lines.push(format!(
                    "  skip ecosystem={} source={} reason={}",
                    resolution.ecosystem.as_str(),
                    reason.source_kind.as_str(),
                    reason.skip_reason.as_str(),
                ));
            }
        }
        for signal in &self.drift_signals {
            lines.push(format!(
                "drift {} ecosystem={} source={} class={} visible={}",
                signal.drift_id,
                signal.ecosystem.as_str(),
                signal.source_kind.as_str(),
                signal.drift_class.as_str(),
                signal.visible_before_trust_loss,
            ));
        }
        for binding in &self.disclosure_surfaces {
            lines.push(format!(
                "disclosure {} fallback={} unsupported={} drift={}",
                binding.surface.as_str(),
                binding.discloses_fallback_reason,
                binding.discloses_unsupported_capabilities,
                binding.discloses_capability_drift,
            ));
        }
        lines
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<NegotiationValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != ADAPTER_NEGOTIATION_RECORD_KIND {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::WrongRecordKind,
                "baseline has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != ADAPTER_NEGOTIATION_SCHEMA_VERSION {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::WrongSchemaVersion,
                "baseline has the wrong schema version",
            ));
        }
        if self.baseline_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::MissingIdentity,
                "baseline id and timestamp are required",
            ));
        }
        for (label, value) in [
            ("negotiation schema", self.negotiation_schema_ref.as_str()),
            ("envelope schema", self.envelope_schema_ref.as_str()),
            ("policy schema", self.policy_schema_ref.as_str()),
            ("doc", self.doc_ref.as_str()),
        ] {
            if value.trim().is_empty() {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::MissingIdentity,
                    format!("{label} ref is required"),
                ));
            }
        }

        self.check_ecosystem_coverage(&mut findings);
        for resolution in &self.resolutions {
            self.check_resolution(resolution, &mut findings);
        }
        self.check_drift(&mut findings);
        self.check_disclosure(&mut findings);

        if include_record_fields {
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::PromotionStateMismatch,
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

    fn check_ecosystem_coverage(&self, findings: &mut Vec<NegotiationValidationFinding>) {
        let present: BTreeSet<Ecosystem> = self.resolutions.iter().map(|r| r.ecosystem).collect();
        if present.len() != self.resolutions.len() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::EcosystemCoverageIncomplete,
                "resolutions repeat an ecosystem",
            ));
        }
        for ecosystem in Ecosystem::ALL {
            if !present.contains(&ecosystem) {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::EcosystemCoverageIncomplete,
                    format!("resolutions are missing {}", ecosystem.as_str()),
                ));
            }
        }
    }

    fn check_resolution(
        &self,
        resolution: &EcosystemAdapterResolution,
        findings: &mut Vec<NegotiationValidationFinding>,
    ) {
        let label = resolution.ecosystem.as_str();

        if resolution.resolution_id.trim().is_empty()
            || resolution.workspace_id.trim().is_empty()
            || resolution.target_id.trim().is_empty()
            || resolution.selected_adapter_id.trim().is_empty()
        {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::MissingIdentity,
                format!("{label} resolution has incomplete identity"),
            ));
        }

        self.check_candidate_ladder(resolution, findings);

        let Some(selected) = resolution.selected_candidate() else {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::SelectionInvalid,
                format!("{label} resolution must name exactly one selected candidate"),
            ));
            return;
        };

        if selected.source_kind != resolution.selected_source_kind
            || selected.adapter_id != resolution.selected_adapter_id
        {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::SelectionInvalid,
                format!("{label} selected candidate disagrees with the resolution header"),
            ));
        }
        if selected.skip_reason.is_some() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::SkipReasonInconsistent,
                format!("{label} selected candidate must not carry a skip reason"),
            ));
        }
        if !selected.eligible() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::SelectedCandidateIneligible,
                format!("{label} selected candidate is not eligible to serve truth"),
            ));
        }

        let selected_rank = selected.priority_rank;
        for candidate in &resolution.candidate_ladder {
            if candidate.priority_rank >= selected_rank || candidate.selected {
                continue;
            }
            if candidate.eligible() {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::LowerPriorityDisplacedHigher,
                    format!(
                        "{label} skipped eligible higher adapter {}",
                        candidate.source_kind.as_str()
                    ),
                ));
                continue;
            }
            match candidate.skip_reason {
                None => findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::SkipReasonMissing,
                    format!(
                        "{label} higher adapter {} lacks a skip reason",
                        candidate.source_kind.as_str()
                    ),
                )),
                Some(reason) if !candidate.skip_reason_consistent(reason) => {
                    findings.push(NegotiationValidationFinding::blocker(
                        NegotiationFindingKind::SkipReasonInconsistent,
                        format!(
                            "{label} skip reason for {} disagrees with its posture",
                            candidate.source_kind.as_str()
                        ),
                    ));
                }
                Some(_) => {}
            }
        }

        self.check_fallback_class(resolution, findings);
        self.check_capabilities(resolution, selected, findings);
        self.check_fallback_reason_packet(resolution, selected_rank, findings);
    }

    fn check_candidate_ladder(
        &self,
        resolution: &EcosystemAdapterResolution,
        findings: &mut Vec<NegotiationValidationFinding>,
    ) {
        let label = resolution.ecosystem.as_str();
        let mut seen = BTreeSet::new();
        let mut expected_rank = 1u8;
        for candidate in &resolution.candidate_ladder {
            seen.insert(candidate.source_kind);
            if candidate.priority_rank != canonical_priority_rank(candidate.source_kind) {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::CandidateRankMismatch,
                    format!(
                        "{label} candidate {} carries a non-canonical rank",
                        candidate.source_kind.as_str()
                    ),
                ));
            }
            if candidate.priority_rank != expected_rank {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::CandidateLadderIncomplete,
                    format!("{label} candidate ladder is out of priority order"),
                ));
            }
            expected_rank = expected_rank.saturating_add(1);
        }
        if seen.len() != resolution.candidate_ladder.len() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::CandidateLadderIncomplete,
                format!("{label} candidate ladder repeats a source kind"),
            ));
        }
        for source_kind in BuildTestEventSourceKind::ALL {
            if !seen.contains(&source_kind) {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::CandidateLadderIncomplete,
                    format!(
                        "{label} candidate ladder is missing {}",
                        source_kind.as_str()
                    ),
                ));
            }
        }
    }

    fn check_fallback_class(
        &self,
        resolution: &EcosystemAdapterResolution,
        findings: &mut Vec<NegotiationValidationFinding>,
    ) {
        let label = resolution.ecosystem.as_str();
        let expected_class = fallback_class_for(resolution.selected_source_kind);
        if resolution.fallback_class != expected_class {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::FallbackClassMismatch,
                format!("{label} fallback class disagrees with the selected source"),
            ));
        }
        if confidence_overclaims(resolution.confidence, resolution.selected_source_kind) {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::ConfidenceOverclaim,
                format!("{label} resolution overclaims confidence for its source"),
            ));
        }
        let expected_reason = downgrade_reason_for_fallback(expected_class);
        if resolution.downgraded != expected_reason.is_some()
            || resolution.downgrade_reason != expected_reason
        {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::DowngradeInconsistent,
                format!(
                    "{label} downgrade posture disagrees with its fallback class (authoritative={})",
                    source_is_authoritative(resolution.selected_source_kind)
                ),
            ));
        }
    }

    fn check_capabilities(
        &self,
        resolution: &EcosystemAdapterResolution,
        selected: &AdapterCandidate,
        findings: &mut Vec<NegotiationValidationFinding>,
    ) {
        let label = resolution.ecosystem.as_str();
        let present: BTreeSet<NegotiatedCapability> =
            selected.capabilities.iter().map(|c| c.capability).collect();
        if present.len() != selected.capabilities.len() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::CapabilityCoverageIncomplete,
                format!("{label} selected candidate repeats a capability"),
            ));
        }
        for capability in NegotiatedCapability::ALL {
            if !present.contains(&capability) {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::CapabilityCoverageIncomplete,
                    format!("{label} selected candidate omits {}", capability.as_str()),
                ));
            }
        }
        let derived: Vec<NegotiatedCapability> = selected.unsupported_capabilities();
        let stored: Vec<NegotiatedCapability> = NegotiatedCapability::ALL
            .into_iter()
            .filter(|c| resolution.unsupported_capabilities.contains(c))
            .collect();
        let stored_set: BTreeSet<NegotiatedCapability> = resolution
            .unsupported_capabilities
            .iter()
            .copied()
            .collect();
        if derived != stored || stored_set.len() != resolution.unsupported_capabilities.len() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::UnsupportedCapabilityUnnamed,
                format!(
                    "{label} named unsupported capabilities disagree with the selected adapter"
                ),
            ));
        }
    }

    fn check_fallback_reason_packet(
        &self,
        resolution: &EcosystemAdapterResolution,
        selected_rank: u8,
        findings: &mut Vec<NegotiationValidationFinding>,
    ) {
        let label = resolution.ecosystem.as_str();
        let derived = resolution.derived_fallback_reasons(selected_rank);
        let derived_keys: BTreeSet<BuildTestEventSourceKind> =
            derived.iter().map(|(source, _, _)| *source).collect();
        let stored_keys: BTreeSet<BuildTestEventSourceKind> = resolution
            .fallback_reasons
            .iter()
            .map(|r| r.source_kind)
            .collect();
        if derived_keys != stored_keys || stored_keys.len() != resolution.fallback_reasons.len() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::FallbackReasonPacketMismatch,
                format!("{label} fallback-reason packet disagrees with the candidate ladder"),
            ));
            return;
        }
        for reason in &resolution.fallback_reasons {
            let matched = derived
                .iter()
                .find(|(source, _, _)| *source == reason.source_kind);
            let Some((_, skip_reason, adapter_id)) = matched else {
                continue;
            };
            if reason.skip_reason != *skip_reason
                || reason.adapter_id != *adapter_id
                || reason.summary.trim().is_empty()
            {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::FallbackReasonPacketMismatch,
                    format!(
                        "{label} fallback reason for {} disagrees with the ladder",
                        reason.source_kind.as_str()
                    ),
                ));
            }
        }
    }

    fn check_drift(&self, findings: &mut Vec<NegotiationValidationFinding>) {
        if self.drift_signals.is_empty() {
            findings.push(NegotiationValidationFinding::blocker(
                NegotiationFindingKind::DriftNotVisible,
                "baseline must demonstrate at least one surfaced capability-drift signal",
            ));
        }
        for signal in &self.drift_signals {
            if !signal.is_visible() {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::DriftNotVisible,
                    format!("drift signal {} is not visibly surfaced", signal.drift_id),
                ));
            }
        }
    }

    fn check_disclosure(&self, findings: &mut Vec<NegotiationValidationFinding>) {
        let present: BTreeSet<DisclosureSurface> =
            self.disclosure_surfaces.iter().map(|b| b.surface).collect();
        for surface in DisclosureSurface::REQUIRED {
            if !present.contains(&surface) {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::DisclosureSurfaceMissing,
                    format!("disclosure surface {} is missing", surface.as_str()),
                ));
            }
        }
        for binding in &self.disclosure_surfaces {
            if !binding.preserves_truth() {
                findings.push(NegotiationValidationFinding::blocker(
                    NegotiationFindingKind::DisclosureSurfaceDropsTruth,
                    format!(
                        "disclosure surface {} drops negotiation truth",
                        binding.surface.as_str()
                    ),
                ));
            }
        }
    }
}

/// Support-export wrapper carrying the exact negotiation baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterNegotiationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Baseline id ref.
    pub baseline_id_ref: String,
    /// Exact baseline exported.
    pub baseline: AdapterNegotiationBaseline,
}

impl AdapterNegotiationSupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == ADAPTER_NEGOTIATION_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == ADAPTER_NEGOTIATION_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.baseline_id_ref == self.baseline.baseline_id
            && self.baseline.is_stable()
    }
}

fn promotion_state_for_findings(
    findings: &[NegotiationValidationFinding],
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

/// Builds the canonical stable negotiation-baseline input.
pub fn current_stable_adapter_hierarchy_negotiation_input() -> AdapterNegotiationBaselineInput {
    AdapterNegotiationBaselineInput {
        baseline_id: ADAPTER_NEGOTIATION_BASELINE_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        resolutions: canonical_resolutions(),
        drift_signals: canonical_drift_signals(),
        disclosure_surfaces: canonical_disclosure_surfaces(),
    }
}

/// Materializes the canonical stable negotiation baseline.
pub fn seeded_adapter_hierarchy_negotiation_baseline() -> AdapterNegotiationBaseline {
    AdapterNegotiationBaseline::materialize(current_stable_adapter_hierarchy_negotiation_input())
}

/// Validates a baseline and returns an `Ok(())` / findings result.
pub fn validate_adapter_hierarchy_negotiation_baseline(
    baseline: &AdapterNegotiationBaseline,
) -> Result<(), Vec<NegotiationValidationFinding>> {
    let findings = baseline.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Spec for a single candidate when building a resolution.
struct CandidateSpec {
    source_kind: BuildTestEventSourceKind,
    available: bool,
    negotiation_failed: bool,
    skip_reason: Option<SkipReason>,
}

fn canonical_resolutions() -> Vec<EcosystemAdapterResolution> {
    vec![
        // Cargo resolves at the native rung; no fallback.
        build_resolution(Ecosystem::Cargo, BuildTestEventSourceKind::Native, &[]),
        // JVM tooling has no native adapter, so it negotiates BSP truth.
        build_resolution(
            Ecosystem::GradleJvm,
            BuildTestEventSourceKind::Bsp,
            &[CandidateSpec {
                source_kind: BuildTestEventSourceKind::Native,
                available: false,
                negotiation_failed: false,
                skip_reason: Some(SkipReason::EcosystemUnsupported),
            }],
        ),
        // Bazel skips native and BSP, then negotiates Build Event Protocol truth.
        build_resolution(
            Ecosystem::Bazel,
            BuildTestEventSourceKind::BazelBep,
            &[
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::Native,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::Bsp,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
            ],
        ),
        // pytest has no authoritative adapter, so it falls back to structured import.
        build_resolution(
            Ecosystem::PythonPytest,
            BuildTestEventSourceKind::StructuredOutput,
            &[
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::Native,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::Bsp,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::BazelBep,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
            ],
        ),
        // Node.js has a reachable BSP server whose handshake fails, so it deepens to
        // structured import with the negotiation failure named explicitly.
        build_resolution(
            Ecosystem::NodeJs,
            BuildTestEventSourceKind::StructuredOutput,
            &[
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::Native,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::Bsp,
                    available: true,
                    negotiation_failed: true,
                    skip_reason: Some(SkipReason::NegotiationFailed),
                },
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::BazelBep,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
            ],
        ),
        // Generic tooling exposes a structured adapter that negotiates no usable
        // capability, so the heuristic parser stands in as a visible last resort.
        build_resolution(
            Ecosystem::Generic,
            BuildTestEventSourceKind::HeuristicParser,
            &[
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::Native,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::Bsp,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::BazelBep,
                    available: false,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::EcosystemUnsupported),
                },
                CandidateSpec {
                    source_kind: BuildTestEventSourceKind::StructuredOutput,
                    available: true,
                    negotiation_failed: false,
                    skip_reason: Some(SkipReason::CapabilityUnsupported),
                },
            ],
        ),
    ]
}

/// Builds one ecosystem resolution from its selected source and skipped higher rungs.
fn build_resolution(
    ecosystem: Ecosystem,
    selected_source_kind: BuildTestEventSourceKind,
    higher_specs: &[CandidateSpec],
) -> EcosystemAdapterResolution {
    let selected_rank = canonical_priority_rank(selected_source_kind);
    let mut ladder = Vec::new();
    let mut fallback_reasons = Vec::new();

    for source_kind in BuildTestEventSourceKind::ALL {
        let rank = canonical_priority_rank(source_kind);
        if source_kind == selected_source_kind {
            ladder.push(AdapterCandidate {
                source_kind,
                priority_rank: rank,
                adapter_id: adapter_id(ecosystem, source_kind),
                available: true,
                negotiation_failed: false,
                selected: true,
                skip_reason: None,
                capabilities: capabilities_for(ecosystem, source_kind, CapabilityProfile::Native),
            });
        } else if rank < selected_rank {
            let spec = higher_specs
                .iter()
                .find(|s| s.source_kind == source_kind)
                .unwrap_or_else(|| {
                    panic!(
                        "{} resolution must spec the higher rung {}",
                        ecosystem.as_str(),
                        source_kind.as_str()
                    )
                });
            let profile = if spec.skip_reason == Some(SkipReason::CapabilityUnsupported) {
                CapabilityProfile::AllUnsupported
            } else {
                CapabilityProfile::None
            };
            ladder.push(AdapterCandidate {
                source_kind,
                priority_rank: rank,
                adapter_id: adapter_id(ecosystem, source_kind),
                available: spec.available,
                negotiation_failed: spec.negotiation_failed,
                selected: false,
                skip_reason: spec.skip_reason,
                capabilities: capabilities_for(ecosystem, source_kind, profile),
            });
            if let Some(reason) = spec.skip_reason {
                fallback_reasons.push(SkippedAdapterReason {
                    source_kind,
                    adapter_id: adapter_id(ecosystem, source_kind),
                    skip_reason: reason,
                    summary: skip_summary(ecosystem, source_kind, reason),
                });
            }
        } else {
            // Lower rungs were never reached because a higher adapter already won.
            ladder.push(AdapterCandidate {
                source_kind,
                priority_rank: rank,
                adapter_id: adapter_id(ecosystem, source_kind),
                available: source_kind == BuildTestEventSourceKind::HeuristicParser,
                negotiation_failed: false,
                selected: false,
                skip_reason: None,
                capabilities: Vec::new(),
            });
        }
    }

    let fallback_class = fallback_class_for(selected_source_kind);
    let downgrade_reason = downgrade_reason_for_fallback(fallback_class);
    let unsupported_capabilities = ladder
        .iter()
        .find(|c| c.selected)
        .map(AdapterCandidate::unsupported_capabilities)
        .unwrap_or_default();

    EcosystemAdapterResolution {
        resolution_id: format!("resolution:m5:adapter-negotiation:{}", ecosystem.as_str()),
        ecosystem,
        workspace_id: "workspace:checkout".to_owned(),
        target_id: format!("target:checkout:{}", ecosystem.as_str()),
        candidate_ladder: ladder,
        selected_source_kind,
        selected_adapter_id: adapter_id(ecosystem, selected_source_kind),
        fallback_class,
        confidence: canonical_confidence_ceiling(selected_source_kind),
        downgraded: downgrade_reason.is_some(),
        downgrade_reason,
        fallback_reasons,
        unsupported_capabilities,
    }
}

fn adapter_id(ecosystem: Ecosystem, source_kind: BuildTestEventSourceKind) -> String {
    format!("adapter:{}:{}", ecosystem.as_str(), source_kind.as_str())
}

fn skip_summary(
    ecosystem: Ecosystem,
    source_kind: BuildTestEventSourceKind,
    reason: SkipReason,
) -> String {
    format!(
        "{} skipped {} ({})",
        ecosystem.as_str(),
        source_kind.as_str(),
        match reason {
            SkipReason::AdapterUnavailable => "adapter not installed",
            SkipReason::EcosystemUnsupported => "source kind does not apply to this ecosystem",
            SkipReason::CapabilityUnsupported => "no usable capability negotiated",
            SkipReason::NegotiationFailed => "capability handshake failed",
        }
    )
}

/// Which set of capability states a candidate reports.
enum CapabilityProfile {
    /// The candidate's native per-source capability matrix (used for selected adapters).
    Native,
    /// The candidate reports no capabilities (unreachable / unavailable rung).
    None,
    /// The candidate reports every capability as unsupported.
    AllUnsupported,
}

fn capabilities_for(
    ecosystem: Ecosystem,
    source_kind: BuildTestEventSourceKind,
    profile: CapabilityProfile,
) -> Vec<CapabilityNegotiation> {
    match profile {
        CapabilityProfile::None => Vec::new(),
        CapabilityProfile::AllUnsupported => NegotiatedCapability::ALL
            .into_iter()
            .map(|capability| {
                capability_row(
                    ecosystem,
                    source_kind,
                    capability,
                    AdapterCapabilityState::Unsupported,
                )
            })
            .collect(),
        CapabilityProfile::Native => NegotiatedCapability::ALL
            .into_iter()
            .map(|capability| {
                capability_row(
                    ecosystem,
                    source_kind,
                    capability,
                    canonical_capability_state(source_kind, capability),
                )
            })
            .collect(),
    }
}

/// Canonical per-source capability state matrix.
fn canonical_capability_state(
    source_kind: BuildTestEventSourceKind,
    capability: NegotiatedCapability,
) -> AdapterCapabilityState {
    use AdapterCapabilityState::{Degraded, Negotiated, Unsupported};
    use NegotiatedCapability::{
        Artifacts, Diagnostics, LifecycleEvents, Progress, TargetGraph, TestEvents,
    };
    match source_kind {
        BuildTestEventSourceKind::Native => Negotiated,
        BuildTestEventSourceKind::Bsp => match capability {
            Artifacts | Progress => Degraded,
            _ => Negotiated,
        },
        BuildTestEventSourceKind::BazelBep => match capability {
            Diagnostics | Progress => Degraded,
            _ => Negotiated,
        },
        BuildTestEventSourceKind::StructuredOutput => match capability {
            TargetGraph | Progress => Unsupported,
            LifecycleEvents | Artifacts => Degraded,
            Diagnostics | TestEvents => Negotiated,
        },
        BuildTestEventSourceKind::HeuristicParser => match capability {
            Diagnostics => Degraded,
            _ => Unsupported,
        },
    }
}

fn capability_row(
    ecosystem: Ecosystem,
    source_kind: BuildTestEventSourceKind,
    capability: NegotiatedCapability,
    state: AdapterCapabilityState,
) -> CapabilityNegotiation {
    CapabilityNegotiation {
        capability,
        state,
        capability_packet_ref: format!(
            "capability-packet:{}:{}:{}",
            ecosystem.as_str(),
            source_kind.as_str(),
            capability.as_str()
        ),
        note: format!(
            "{} {} reports {} as {}",
            ecosystem.as_str(),
            source_kind.as_str(),
            capability.as_str(),
            state.as_str()
        ),
    }
}

fn canonical_drift_signals() -> Vec<CapabilityDriftSignal> {
    vec![
        CapabilityDriftSignal {
            drift_id: "drift:gradle-jvm:bsp:progress".to_owned(),
            ecosystem: Ecosystem::GradleJvm,
            source_kind: BuildTestEventSourceKind::Bsp,
            capability: Some(NegotiatedCapability::Progress),
            drift_class: DriftClass::CapabilityDegraded,
            previous_state: "negotiated".to_owned(),
            current_state: "degraded".to_owned(),
            visible_before_trust_loss: true,
            summary: "BSP progress events degraded to coarse percentage updates".to_owned(),
        },
        CapabilityDriftSignal {
            drift_id: "drift:bazel:bazel-bep:diagnostics".to_owned(),
            ecosystem: Ecosystem::Bazel,
            source_kind: BuildTestEventSourceKind::BazelBep,
            capability: Some(NegotiatedCapability::Diagnostics),
            drift_class: DriftClass::CapabilityDegraded,
            previous_state: "negotiated".to_owned(),
            current_state: "degraded".to_owned(),
            visible_before_trust_loss: true,
            summary: "Bazel BEP diagnostics degraded to summary-only on the pinned version"
                .to_owned(),
        },
        CapabilityDriftSignal {
            drift_id: "drift:node-js:bsp:fallback-deepened".to_owned(),
            ecosystem: Ecosystem::NodeJs,
            source_kind: BuildTestEventSourceKind::Bsp,
            capability: None,
            drift_class: DriftClass::FallbackDeepened,
            previous_state: "negotiated_protocol".to_owned(),
            current_state: "structured_import".to_owned(),
            visible_before_trust_loss: true,
            summary:
                "Node.js BSP handshake now fails, deepening the resolution to structured import"
                    .to_owned(),
        },
    ]
}

fn canonical_disclosure_surfaces() -> Vec<NegotiationDisclosureBinding> {
    DisclosureSurface::REQUIRED
        .into_iter()
        .map(|surface| NegotiationDisclosureBinding {
            surface,
            binding_ref: format!("binding:m5:adapter-negotiation:{}", surface.as_str()),
            discloses_selected_source_kind: true,
            discloses_fallback_reason: true,
            discloses_unsupported_capabilities: true,
            discloses_capability_drift: true,
            discloses_confidence: true,
        })
        .collect()
}
