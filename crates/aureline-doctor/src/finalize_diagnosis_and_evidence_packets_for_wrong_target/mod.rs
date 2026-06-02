//! Finalized diagnosis and evidence packets for wrong-target writes, stale truth,
//! policy denial, route drift, and install failures.
//!
//! This module owns typed diagnosis and evidence contracts for five specific
//! failure families that block users during normal IDE operations. Each family
//! gets a closed vocabulary, a diagnosis packet with finding codes and repair
//! hooks, and an evidence packet that is safe for support export by default.
//!
//! The five failure scenarios are:
//!
//! - [`FailureScenarioClass::WrongTargetWrite`] — a write operation lands on an
//!   unexpected file, route, or durable store because the active target was
//!   misresolved or stale.
//! - [`FailureScenarioClass::StaleTruth`] — a cached or mirrored truth is treated
//!   as current when it has diverged from the ground source.
//! - [`FailureScenarioClass::PolicyDenial`] — a security or policy gate blocks an
//!   operation that the user expected to succeed.
//! - [`FailureScenarioClass::RouteDrift`] — a command, event, or notification is
//!   routed to an unexpected handler or surface because the route manifest has
//!   drifted from the declared contract.
//! - [`FailureScenarioClass::InstallFailure`] — a toolchain, extension, or
//!   dependency install fails in a way that leaves the workspace partially
//!   configured or blocked.
//!
//! The boundary schema is at
//! `/schemas/support/finalize_diagnosis_and_evidence_packets_for_wrong_target.schema.json`.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_project_doctor_probes_finding_codes_explainability_and::{
    ExplainabilityFactorClass, StableFindingSeverityClass, StableProbePackClass,
    DOCTOR_FINDING_PREFIX,
};

/// Frozen schema version for finalized diagnosis and evidence packets.
pub const DIAGNOSIS_EVIDENCE_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for a diagnosis packet.
pub const DIAGNOSIS_PACKET_RECORD_KIND: &str = "diagnosis_packet_record";

/// Record-kind tag for an evidence packet.
pub const EVIDENCE_PACKET_RECORD_KIND: &str = "evidence_packet_record";

/// Record-kind tag for the combined finalized support packet.
pub const FINALIZED_DIAGNOSIS_SUPPORT_PACKET_RECORD_KIND: &str =
    "finalized_diagnosis_support_packet_record";

/// Repo-relative path of the boundary schema mirrored by this module.
pub const DIAGNOSIS_EVIDENCE_SCHEMA_REF: &str =
    "schemas/support/finalize_diagnosis_and_evidence_packets_for_wrong_target.schema.json";

/// Reviewer doc ref quoted by every emitted packet.
pub const DIAGNOSIS_EVIDENCE_DOC_REF: &str =
    "docs/support/m4/finalize_diagnosis_and_evidence_packets_for_wrong_target.md";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed failure-scenario vocabulary. Each variant names one of the five
/// supported blocked-user failure families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureScenarioClass {
    /// A write operation lands on an unexpected target.
    WrongTargetWrite,
    /// Cached or mirrored truth is treated as current when it has diverged.
    StaleTruth,
    /// A security or policy gate blocks an expected operation.
    PolicyDenial,
    /// A command or event is routed to an unexpected handler.
    RouteDrift,
    /// A toolchain, extension, or dependency install fails.
    InstallFailure,
}

impl FailureScenarioClass {
    /// Returns every scenario in catalog order.
    pub const fn all() -> [Self; 5] {
        [
            Self::WrongTargetWrite,
            Self::StaleTruth,
            Self::PolicyDenial,
            Self::RouteDrift,
            Self::InstallFailure,
        ]
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongTargetWrite => "wrong_target_write",
            Self::StaleTruth => "stale_truth",
            Self::PolicyDenial => "policy_denial",
            Self::RouteDrift => "route_drift",
            Self::InstallFailure => "install_failure",
        }
    }
}

impl fmt::Display for FailureScenarioClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed evidence-class vocabulary. Each variant names one kind of evidence
/// that Doctor can collect for the five failure families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// Log or trace snippet showing the failure path.
    LogTrace,
    /// Snapshot of relevant state at the time of failure.
    StateSnapshot,
    /// Diff between expected and actual configuration.
    ConfigDiff,
    /// Route manifest or routing table at time of failure.
    RouteManifest,
    /// Policy audit trail showing the denial decision.
    PolicyAudit,
    /// Install artifact, manifest, or receipt.
    InstallArtifact,
    /// Symbolication trace or crash envelope.
    SymbolTrace,
}

impl EvidenceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogTrace => "log_trace",
            Self::StateSnapshot => "state_snapshot",
            Self::ConfigDiff => "config_diff",
            Self::RouteManifest => "route_manifest",
            Self::PolicyAudit => "policy_audit",
            Self::InstallArtifact => "install_artifact",
            Self::SymbolTrace => "symbol_trace",
        }
    }
}

/// Closed diagnosis-confidence vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosisConfidenceClass {
    /// The diagnosis is directly observed and authoritative.
    ObservedAuthoritative,
    /// The diagnosis is inferred from collected evidence.
    InferredFromEvidence,
    /// The diagnosis is unknown and requires additional probing.
    UnknownRequiresProbe,
}

impl DiagnosisConfidenceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObservedAuthoritative => "observed_authoritative",
            Self::InferredFromEvidence => "inferred_from_evidence",
            Self::UnknownRequiresProbe => "unknown_requires_probe",
        }
    }
}

/// Closed blast-radius vocabulary. Describes how wide the impact of a repair
/// or diagnosis may be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlastRadiusClass {
    /// Impact is limited to a single disposable state class.
    SingleDisposableState,
    /// Impact is limited to state classes within the same family.
    SameFamilyStateClasses,
    /// Impact crosses family boundaries.
    CrossFamilyStateClasses,
    /// No local repair is safe; escalate only.
    EscalationOnly,
}

impl BlastRadiusClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleDisposableState => "single_disposable_state",
            Self::SameFamilyStateClasses => "same_family_state_classes",
            Self::CrossFamilyStateClasses => "cross_family_state_classes",
            Self::EscalationOnly => "escalation_only",
        }
    }
}

/// Closed repair-class vocabulary for the five failure families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairClass {
    /// No repair; observation and escalation only.
    ObserveOnlyNoRepair,
    /// Reapprove the affected target or route.
    ReapproveTargetOrRoute,
    /// Reset the ephemeral cache that may contain stale truth.
    ResetEphemeralCache,
    /// Reacquire trust or policy approval.
    ReacquireTrustApproval,
    /// Reinstall or repair the affected toolchain or extension.
    ReinstallToolchain,
    /// Refresh the route manifest from the canonical source.
    RefreshRouteManifest,
    /// Reset the targeted durable state while preserving user files.
    ResetTargetedDurableState,
    /// Defer to escalation packet.
    DeferToEscalationPacket,
}

impl RepairClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObserveOnlyNoRepair => "observe_only_no_repair",
            Self::ReapproveTargetOrRoute => "reapprove_target_or_route",
            Self::ResetEphemeralCache => "reset_ephemeral_cache",
            Self::ReacquireTrustApproval => "reacquire_trust_approval",
            Self::ReinstallToolchain => "reinstall_toolchain",
            Self::RefreshRouteManifest => "refresh_route_manifest",
            Self::ResetTargetedDurableState => "reset_targeted_durable_state",
            Self::DeferToEscalationPacket => "defer_to_escalation_packet",
        }
    }
}

/// Closed redaction-class vocabulary for support-export safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Safe for metadata-only export by default.
    MetadataSafeDefault,
    /// Included only when the user explicitly opts in.
    OptInOnly,
    /// Never exported.
    Prohibited,
}

impl RedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OptInOnly => "opt_in_only",
            Self::Prohibited => "prohibited",
        }
    }
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// One explainability-factor assertion expected in a diagnosis packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosisExplainabilityAssertion {
    /// The explainability factor class required.
    pub factor_class: ExplainabilityFactorClass,
    /// Reviewable sentence describing the expected assertion content.
    pub expected_assertion_summary: String,
}

/// One evidence item collected for a diagnosis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceItem {
    /// Unique item id.
    pub item_id: String,
    /// Evidence class.
    pub evidence_class: EvidenceClass,
    /// Redaction class for this item.
    pub redaction_class: RedactionClass,
    /// Repo-relative or URI reference to the evidence artifact.
    pub evidence_ref: String,
    /// Human-readable description of what this evidence proves.
    pub description: String,
    /// Exact-build identity that produced this evidence.
    pub exact_build_id: String,
}

/// One no-touch boundary declared by a diagnosis packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoTouchBoundary {
    /// State class or path that must not be mutated.
    pub boundary: String,
    /// Why this boundary is protected.
    pub rationale: String,
}

/// One recovery-ladder rung referenced by a diagnosis packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderRung {
    /// Repair class for this rung.
    pub repair_class: RepairClass,
    /// Blast radius of applying this rung.
    pub blast_radius_class: BlastRadiusClass,
    /// Whether user consent is required before applying.
    pub requires_user_consent: bool,
    /// Whether the repair is previewable before application.
    pub is_previewable: bool,
    /// Human-readable guidance for the user or support agent.
    pub guidance: String,
}

/// Typed diagnosis packet for one failure scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosisPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// Failure scenario this packet diagnoses.
    pub scenario_class: FailureScenarioClass,
    /// Probe pack class responsible for this scenario.
    pub probe_pack_class: StableProbePackClass,
    /// Stable finding code (must start with `doctor.finding.`).
    pub finding_code: String,
    /// Finding severity.
    pub finding_severity: StableFindingSeverityClass,
    /// Diagnosis confidence.
    pub diagnosis_confidence: DiagnosisConfidenceClass,
    /// Expected explainability factors.
    pub expected_explainability: Vec<DiagnosisExplainabilityAssertion>,
    /// No-touch boundaries.
    pub no_touch_boundaries: Vec<NoTouchBoundary>,
    /// Recovery-ladder rungs ordered from narrowest to widest blast radius.
    pub recovery_ladder_rungs: Vec<RecoveryLadderRung>,
    /// Whether the scenario includes at least one observe-only outcome.
    pub has_observe_only_outcome: bool,
    /// Reviewable notes.
    pub notes: String,
}

/// Typed evidence packet paired with a diagnosis packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidencePacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// Diagnosis packet this evidence supports.
    pub diagnosis_packet_id: String,
    /// Failure scenario.
    pub scenario_class: FailureScenarioClass,
    /// Evidence items.
    pub evidence_items: Vec<EvidenceItem>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Exact-build identity that produced this packet.
    pub exact_build_id: String,
    /// Reviewable notes.
    pub notes: String,
}

/// Combined finalized support packet for export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedDiagnosisSupportPacket {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id.
    pub packet_id: String,
    /// Captured-at timestamp.
    pub captured_at: String,
    /// Doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Diagnosis packet refs included in this export.
    pub diagnosis_packet_refs: Vec<String>,
    /// Evidence packet refs included in this export.
    pub evidence_packet_refs: Vec<String>,
    /// Scenarios covered.
    pub scenarios_covered: Vec<FailureScenarioClass>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Exact-build identity.
    pub exact_build_id: String,
}

/// One validation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosisEvidenceViolation {
    /// Check id.
    pub check_id: String,
    /// Subject ref.
    pub subject_ref: String,
    /// Human-readable message.
    pub message: String,
}

/// Validation report emitted when a record fails validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosisEvidenceValidationReport {
    /// Violations.
    pub violations: Vec<DiagnosisEvidenceViolation>,
}

impl DiagnosisEvidenceValidationReport {
    /// True when no violations were found.
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Load helpers
// ---------------------------------------------------------------------------

/// Deserialize a diagnosis packet from YAML.
pub fn load_diagnosis_packet(yaml: &str) -> Result<DiagnosisPacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Deserialize an evidence packet from YAML.
pub fn load_evidence_packet(yaml: &str) -> Result<EvidencePacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Validates diagnosis packets, evidence packets, and folds them into a
/// metadata-safe [`FinalizedDiagnosisSupportPacket`].
pub struct DiagnosisEvidenceEvaluator;

impl Default for DiagnosisEvidenceEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosisEvidenceEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a diagnosis packet.
    pub fn validate_diagnosis_packet(
        &self,
        packet: &DiagnosisPacket,
    ) -> DiagnosisEvidenceValidationReport {
        let mut violations = Vec::new();

        if packet.schema_version != DIAGNOSIS_EVIDENCE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "diagnosis_evidence.diagnosis_schema_version",
                &packet.packet_id,
                "diagnosis packet schema_version must be 1",
            );
        }
        if packet.record_kind != DIAGNOSIS_PACKET_RECORD_KIND {
            push_violation(
                &mut violations,
                "diagnosis_evidence.diagnosis_record_kind",
                &packet.packet_id,
                "diagnosis packet record_kind must be diagnosis_packet_record",
            );
        }
        if packet.packet_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "diagnosis_evidence.diagnosis_packet_id_empty",
                &packet.packet_id,
                "diagnosis packet_id must be non-empty",
            );
        }
        if !packet.finding_code.starts_with(DOCTOR_FINDING_PREFIX) {
            push_violation(
                &mut violations,
                "diagnosis_evidence.diagnosis_finding_code_prefix",
                &packet.packet_id,
                format!("finding_code must start with {DOCTOR_FINDING_PREFIX}"),
            );
        }
        if packet.expected_explainability.is_empty() {
            push_violation(
                &mut violations,
                "diagnosis_evidence.diagnosis_explainability_missing",
                &packet.packet_id,
                "diagnosis packet must declare at least one expected_explainability factor",
            );
        }
        if packet.no_touch_boundaries.is_empty() {
            push_violation(
                &mut violations,
                "diagnosis_evidence.diagnosis_no_touch_boundaries_missing",
                &packet.packet_id,
                "diagnosis packet must declare at least one no_touch_boundary",
            );
        }
        if packet.recovery_ladder_rungs.is_empty() {
            push_violation(
                &mut violations,
                "diagnosis_evidence.diagnosis_recovery_ladder_missing",
                &packet.packet_id,
                "diagnosis packet must declare at least one recovery_ladder_rung",
            );
        }

        let mut seen_repairs = BTreeMap::new();
        for rung in &packet.recovery_ladder_rungs {
            if seen_repairs.insert(rung.repair_class, ()).is_some() {
                push_violation(
                    &mut violations,
                    "diagnosis_evidence.diagnosis_recovery_ladder_duplicate_repair",
                    &packet.packet_id,
                    format!(
                        "duplicate recovery_ladder_rung for repair {}",
                        rung.repair_class.as_str()
                    ),
                );
            }
        }

        DiagnosisEvidenceValidationReport { violations }
    }

    /// Validates an evidence packet.
    pub fn validate_evidence_packet(
        &self,
        packet: &EvidencePacket,
        diagnosis_packet: &DiagnosisPacket,
    ) -> DiagnosisEvidenceValidationReport {
        let mut violations = Vec::new();

        if packet.schema_version != DIAGNOSIS_EVIDENCE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "diagnosis_evidence.evidence_schema_version",
                &packet.packet_id,
                "evidence packet schema_version must be 1",
            );
        }
        if packet.record_kind != EVIDENCE_PACKET_RECORD_KIND {
            push_violation(
                &mut violations,
                "diagnosis_evidence.evidence_record_kind",
                &packet.packet_id,
                "evidence packet record_kind must be evidence_packet_record",
            );
        }
        if packet.packet_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "diagnosis_evidence.evidence_packet_id_empty",
                &packet.packet_id,
                "evidence packet_id must be non-empty",
            );
        }
        if packet.diagnosis_packet_id != diagnosis_packet.packet_id {
            push_violation(
                &mut violations,
                "diagnosis_evidence.evidence_diagnosis_mismatch",
                &packet.packet_id,
                "evidence packet diagnosis_packet_id must match the diagnosis packet_id",
            );
        }
        if packet.scenario_class != diagnosis_packet.scenario_class {
            push_violation(
                &mut violations,
                "diagnosis_evidence.evidence_scenario_mismatch",
                &packet.packet_id,
                "evidence packet scenario_class must match the diagnosis packet scenario_class",
            );
        }
        if packet.evidence_items.is_empty() {
            push_violation(
                &mut violations,
                "diagnosis_evidence.evidence_items_empty",
                &packet.packet_id,
                "evidence packet must contain at least one evidence_item",
            );
        }

        let mut seen_item_ids = BTreeMap::new();
        for item in &packet.evidence_items {
            if item.item_id.trim().is_empty() {
                push_violation(
                    &mut violations,
                    "diagnosis_evidence.evidence_item_id_empty",
                    &packet.packet_id,
                    "evidence item_id must be non-empty",
                );
            }
            if seen_item_ids.insert(item.item_id.clone(), ()).is_some() {
                push_violation(
                    &mut violations,
                    "diagnosis_evidence.evidence_item_id_duplicate",
                    &item.item_id,
                    "evidence item_id must be unique within the packet",
                );
            }
            if item.exact_build_id.trim().is_empty() {
                push_violation(
                    &mut violations,
                    "diagnosis_evidence.evidence_exact_build_id_empty",
                    &item.item_id,
                    "evidence item exact_build_id must be non-empty",
                );
            }
        }

        if packet.exact_build_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "diagnosis_evidence.evidence_packet_exact_build_id_empty",
                &packet.packet_id,
                "evidence packet exact_build_id must be non-empty",
            );
        }

        DiagnosisEvidenceValidationReport { violations }
    }

    /// Folds validated diagnosis and evidence packets into a single
    /// metadata-safe support packet.
    pub fn finalize_support_packet(
        &self,
        packet_id: &str,
        captured_at: &str,
        exact_build_id: &str,
        diagnosis_packets: &[DiagnosisPacket],
        evidence_packets: &[EvidencePacket],
    ) -> Result<FinalizedDiagnosisSupportPacket, DiagnosisEvidenceValidationReport> {
        let mut all_violations = Vec::new();

        for dp in diagnosis_packets {
            let report = self.validate_diagnosis_packet(dp);
            all_violations.extend(report.violations);
        }

        for ep in evidence_packets {
            let matching = diagnosis_packets
                .iter()
                .find(|dp| dp.packet_id == ep.diagnosis_packet_id);
            if let Some(dp) = matching {
                let report = self.validate_evidence_packet(ep, dp);
                all_violations.extend(report.violations);
            } else {
                push_violation(
                    &mut all_violations,
                    "diagnosis_evidence.finalize_missing_diagnosis",
                    &ep.packet_id,
                    format!(
                        "evidence packet {} references missing diagnosis packet {}",
                        ep.packet_id, ep.diagnosis_packet_id
                    ),
                );
            }
        }

        if !all_violations.is_empty() {
            return Err(DiagnosisEvidenceValidationReport {
                violations: all_violations,
            });
        }

        let mut scenarios = std::collections::BTreeSet::new();
        for dp in diagnosis_packets {
            scenarios.insert(dp.scenario_class);
        }

        Ok(FinalizedDiagnosisSupportPacket {
            record_kind: FINALIZED_DIAGNOSIS_SUPPORT_PACKET_RECORD_KIND.to_string(),
            schema_version: DIAGNOSIS_EVIDENCE_SCHEMA_VERSION,
            packet_id: packet_id.to_string(),
            captured_at: captured_at.to_string(),
            doc_ref: DIAGNOSIS_EVIDENCE_DOC_REF.to_string(),
            schema_ref: DIAGNOSIS_EVIDENCE_SCHEMA_REF.to_string(),
            diagnosis_packet_refs: diagnosis_packets
                .iter()
                .map(|dp| dp.packet_id.clone())
                .collect(),
            evidence_packet_refs: evidence_packets
                .iter()
                .map(|ep| ep.packet_id.clone())
                .collect(),
            scenarios_covered: scenarios.into_iter().collect(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            exact_build_id: exact_build_id.to_string(),
        })
    }
}

fn push_violation(
    violations: &mut Vec<DiagnosisEvidenceViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(DiagnosisEvidenceViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
