//! Extended Project Doctor probe families, stable finding codes, and explicit
//! unsupported-state reporting across the M5 feature lanes.
//!
//! This module owns the canonical packet that extends Project Doctor to the new
//! M5 feature families — notebook kernels, request/API auth and environments,
//! database targets, profiler/replay instrumentation, remote preview routes,
//! sync/offboarding/device registry, companion handoff, and incident packets.
//! For every [`DoctorLane`] it pins one read-only [`ProbeFamily`] that names the
//! lane's stable finding-code prefix, its supported finding codes, the diagnosis
//! states it may report, and the affected-scope identities it diagnoses. Each
//! [`LaneFinding`] then carries a stable [`LaneFinding::finding_code`], a
//! [`DiagnosisState`], a [`FindingConfidence`], an [`AffectedScope`], evidence
//! refs, an optional set of repair-candidate ids, and a first actionable
//! explanation.
//!
//! The model exists to keep **unsupported, partial, stale, policy-blocked, and
//! target-mismatch** states reported explicitly rather than collapsed into a
//! generic "unavailable" string. Every non-healthy finding must carry a specific,
//! non-generic [`LaneFinding::state_detail_code`]; a healthy finding must carry
//! none. Because the finding code, lane, scope kind, render surfaces, and state
//! detail are all validated against the lane's pinned probe family, support
//! tooling, automation, and users reason about the same finding ids across
//! desktop, CLI/headless, and support-bundle contexts.
//!
//! Diagnosis stays **read-only by construction**: a probe family carries only a
//! [`ReadOnlyPosture`], so it can never declare a mutating posture, run
//! repo-owned hooks, mutate external services, or silently re-enable a
//! blocked/quarantined component. A lane that does not emit repair candidates may
//! not attach any, so no speculative remediation leaks into a read-only lane.
//!
//! The packet is checked in at
//! `artifacts/doctor/m5/project-doctor-feature-lane-probes.json` and embedded
//! here, so this typed consumer and any CI gate agree on every row without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, or mount/port/tunnel
//! secrets.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported feature-lane probe packet schema version.
pub const PROJECT_DOCTOR_FEATURE_LANE_PROBES_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PROJECT_DOCTOR_FEATURE_LANE_PROBES_RECORD_KIND: &str =
    "project_doctor_feature_lane_probes";

/// Repo-relative path to the checked-in packet.
pub const PROJECT_DOCTOR_FEATURE_LANE_PROBES_PATH: &str =
    "artifacts/doctor/m5/project-doctor-feature-lane-probes.json";

/// Repo-relative path to the boundary schema.
pub const PROJECT_DOCTOR_FEATURE_LANE_PROBES_SCHEMA_REF: &str =
    "schemas/doctor/project-doctor-feature-lane-probes.schema.json";

/// Repo-relative path to the companion document.
pub const PROJECT_DOCTOR_FEATURE_LANE_PROBES_DOC_REF: &str =
    "docs/doctor/m5/project-doctor-feature-lane-probes.md";

/// Stable finding-code prefix every finding and family code must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Stable repair-candidate id prefix every repair candidate must use.
pub const DOCTOR_REPAIR_CANDIDATE_PREFIX: &str = "repair.";

/// Embedded checked-in packet JSON.
pub const PROJECT_DOCTOR_FEATURE_LANE_PROBES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/doctor/m5/project-doctor-feature-lane-probes.json"
));

/// Generic, non-actionable detail tokens that may never stand in for an explicit
/// unsupported/partial/stale/policy-blocked/target-mismatch reason.
const GENERIC_DETAIL_TOKENS: [&str; 9] = [
    "unavailable",
    "error",
    "failed",
    "failure",
    "unknown",
    "unknown_error",
    "generic_failure",
    "n_a",
    "na",
];

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// An M5 feature lane Project Doctor was extended to diagnose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorLane {
    /// Notebook kernel runtime, engine identity, and attach readiness.
    NotebookKernel,
    /// Request/API auth authority and environment binding.
    RequestApi,
    /// Database target identity and schema alignment.
    DatabaseTarget,
    /// Profiler/replay instrumentation attachment and capture coverage.
    ProfilerReplay,
    /// Remote preview route, port, and tunnel scope.
    PreviewRoute,
    /// Sync, offboarding, and device-registry consistency.
    SyncDeviceRegistry,
    /// Companion handoff packet completeness and continuity.
    CompanionHandoff,
    /// Incident packet integrity and chain-of-custody readiness.
    IncidentPacket,
}

impl DoctorLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotebookKernel,
        Self::RequestApi,
        Self::DatabaseTarget,
        Self::ProfilerReplay,
        Self::PreviewRoute,
        Self::SyncDeviceRegistry,
        Self::CompanionHandoff,
        Self::IncidentPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookKernel => "notebook_kernel",
            Self::RequestApi => "request_api",
            Self::DatabaseTarget => "database_target",
            Self::ProfilerReplay => "profiler_replay",
            Self::PreviewRoute => "preview_route",
            Self::SyncDeviceRegistry => "sync_device_registry",
            Self::CompanionHandoff => "companion_handoff",
            Self::IncidentPacket => "incident_packet",
        }
    }

    /// The canonical affected-scope identity this lane diagnoses.
    pub const fn canonical_scope_kind(self) -> ScopeKind {
        match self {
            Self::NotebookKernel => ScopeKind::KernelEngine,
            Self::RequestApi => ScopeKind::ApiRoute,
            Self::DatabaseTarget => ScopeKind::DatabaseTarget,
            Self::ProfilerReplay => ScopeKind::ProfilerSession,
            Self::PreviewRoute => ScopeKind::PreviewRoute,
            Self::SyncDeviceRegistry => ScopeKind::DeviceRegistry,
            Self::CompanionHandoff => ScopeKind::CompanionSession,
            Self::IncidentPacket => ScopeKind::IncidentPacket,
        }
    }

    /// The stable finding-code prefix every code in this lane must start with.
    pub fn finding_code_prefix(self) -> String {
        format!("{DOCTOR_FINDING_PREFIX}{}.", self.as_str())
    }
}

impl fmt::Display for DoctorLane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The explicit diagnosis state Doctor reports for a finding.
///
/// Every state but [`DiagnosisState::Healthy`] is a distinct, named condition
/// that must be reported with its own detail rather than folded into a generic
/// "unavailable" string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosisState {
    /// The lane is current and healthy; no problem detail is attached.
    Healthy,
    /// The lane is usable but the diagnosis covers only part of the scope.
    Partial,
    /// The lane's evidence is past its freshness window.
    Stale,
    /// The lane is not supported in the current context.
    Unsupported,
    /// A managed policy blocks the lane.
    PolicyBlocked,
    /// The bound target does not match the cached/expected identity.
    TargetMismatch,
}

impl DiagnosisState {
    /// Every diagnosis state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Healthy,
        Self::Partial,
        Self::Stale,
        Self::Unsupported,
        Self::PolicyBlocked,
        Self::TargetMismatch,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unsupported => "unsupported",
            Self::PolicyBlocked => "policy_blocked",
            Self::TargetMismatch => "target_mismatch",
        }
    }

    /// Whether the state must carry an explicit, non-generic detail code.
    ///
    /// Every non-healthy state must name *why* it fired, so it is never collapsed
    /// into a generic "unavailable" string.
    pub const fn requires_explicit_detail(self) -> bool {
        !matches!(self, Self::Healthy)
    }
}

/// Machine severity class for a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Degraded but still usable.
    Degraded,
    /// Blocks the requested workflow until handled.
    Blocking,
    /// Unsupported in the current context.
    Unsupported,
}

impl FindingSeverity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Info,
        Self::Degraded,
        Self::Blocking,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Degraded => "degraded",
            Self::Blocking => "blocking",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Confidence class describing how strongly the evidence backs the finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingConfidence {
    /// Evidence directly proves the finding.
    ObservedAuthoritative,
    /// Evidence proves the finding but leaves a typed gap.
    ObservedWithGap,
    /// Evidence is sufficient for a bounded inference.
    InferredFromEvidence,
    /// More evidence is required before Doctor can prove the state.
    UnknownRequiresProbe,
}

impl FindingConfidence {
    /// Every confidence class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ObservedAuthoritative,
        Self::ObservedWithGap,
        Self::InferredFromEvidence,
        Self::UnknownRequiresProbe,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObservedAuthoritative => "observed_authoritative",
            Self::ObservedWithGap => "observed_with_gap",
            Self::InferredFromEvidence => "inferred_from_evidence",
            Self::UnknownRequiresProbe => "unknown_requires_probe",
        }
    }
}

/// The affected-scope identity a finding is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    /// Notebook kernel engine identity.
    KernelEngine,
    /// Request/API route identity.
    ApiRoute,
    /// Database target identity.
    DatabaseTarget,
    /// Profiler/replay session identity.
    ProfilerSession,
    /// Remote preview route/tunnel identity.
    PreviewRoute,
    /// Sync device-registry identity.
    DeviceRegistry,
    /// Companion handoff session identity.
    CompanionSession,
    /// Incident packet identity.
    IncidentPacket,
}

impl ScopeKind {
    /// Every scope kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::KernelEngine,
        Self::ApiRoute,
        Self::DatabaseTarget,
        Self::ProfilerSession,
        Self::PreviewRoute,
        Self::DeviceRegistry,
        Self::CompanionSession,
        Self::IncidentPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KernelEngine => "kernel_engine",
            Self::ApiRoute => "api_route",
            Self::DatabaseTarget => "database_target",
            Self::ProfilerSession => "profiler_session",
            Self::PreviewRoute => "preview_route",
            Self::DeviceRegistry => "device_registry",
            Self::CompanionSession => "companion_session",
            Self::IncidentPacket => "incident_packet",
        }
    }
}

/// Read-only posture of a probe family.
///
/// There is no mutating posture: diagnosis is read-only by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadOnlyPosture {
    /// The family performs no mutation at all.
    ReadOnlyNoMutation,
    /// The family may write only local evidence/preview rows.
    MetadataLocalEvidenceOnly,
}

impl ReadOnlyPosture {
    /// Every read-only posture, in declaration order.
    pub const ALL: [Self; 2] = [Self::ReadOnlyNoMutation, Self::MetadataLocalEvidenceOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::MetadataLocalEvidenceOnly => "metadata_local_evidence_only",
        }
    }
}

/// A surface a finding may be rendered on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderSurface {
    /// Shell finding card / desktop UI surface.
    UiFindingCard,
    /// Interactive CLI finding row.
    CliFindingRow,
    /// Support export row.
    SupportExportRow,
    /// Headless JSON row.
    HeadlessJsonRow,
}

impl RenderSurface {
    /// Every render surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UiFindingCard,
        Self::CliFindingRow,
        Self::SupportExportRow,
        Self::HeadlessJsonRow,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiFindingCard => "ui_finding_card",
            Self::CliFindingRow => "cli_finding_row",
            Self::SupportExportRow => "support_export_row",
            Self::HeadlessJsonRow => "headless_json_row",
        }
    }
}

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

/// One read-only probe family pinned to a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeFamily {
    /// Stable family id.
    pub family_id: String,
    /// Lane this family diagnoses.
    pub lane: DoctorLane,
    /// Finding-code prefix every code the family emits must use.
    pub finding_code_prefix: String,
    /// Read-only posture; there is no mutating posture.
    pub read_only_posture: ReadOnlyPosture,
    /// Whether the family produces stable findings under headless/CLI runs.
    pub headless_supported: bool,
    /// Whether the family may attach repair-candidate ids to its findings.
    pub emits_repair_candidates: bool,
    /// Stable finding codes the family may emit.
    pub supported_finding_codes: Vec<String>,
    /// Diagnosis states the family may report.
    pub supported_states: Vec<DiagnosisState>,
    /// Affected-scope kinds the family may diagnose.
    pub supported_scope_kinds: Vec<ScopeKind>,
    /// Reviewer-safe summary.
    pub summary: String,
}

impl ProbeFamily {
    /// Whether the family's declared prefix matches its lane.
    pub fn prefix_matches_lane(&self) -> bool {
        self.finding_code_prefix == self.lane.finding_code_prefix()
    }
}

/// The affected-scope identity a finding is about.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AffectedScope {
    /// Scope-kind identity.
    pub scope_kind: ScopeKind,
    /// Opaque, redaction-safe scope reference.
    pub scope_ref: String,
}

/// One typed Project Doctor finding for an M5 feature lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneFinding {
    /// Stable finding id.
    pub finding_id: String,
    /// Stable finding code (must start with the lane's finding-code prefix).
    pub finding_code: String,
    /// Lane that emitted the finding.
    pub lane: DoctorLane,
    /// Family id that emitted the finding.
    pub family_ref: String,
    /// Explicit diagnosis state.
    pub diagnosis_state: DiagnosisState,
    /// Machine severity class.
    pub severity_class: FindingSeverity,
    /// Confidence class.
    pub confidence_class: FindingConfidence,
    /// Explicit, non-generic detail code (required for non-healthy states; empty
    /// for healthy findings).
    pub state_detail_code: String,
    /// Affected-scope identity.
    pub affected_scope: AffectedScope,
    /// Evidence refs backing the finding.
    pub evidence_refs: Vec<String>,
    /// Optional repair-candidate ids; empty when the lane emits no repairs.
    #[serde(default)]
    pub repair_candidate_ids: Vec<String>,
    /// First actionable explanation.
    pub first_action: String,
    /// Render surfaces the finding may be carried on.
    pub render_surfaces: Vec<RenderSurface>,
    /// Redaction class (must be metadata-safe).
    pub redaction_class: String,
    /// Whether raw private material is excluded (must be true).
    pub raw_private_material_excluded: bool,
    /// Reviewer-safe summary.
    pub summary: String,
}

impl LaneFinding {
    /// Whether the finding renders stably across the desktop, headless, and
    /// support-bundle contexts, so support and automation reason about the same
    /// id everywhere.
    pub fn is_cross_context_stable(&self) -> bool {
        let has = |surface| self.render_surfaces.contains(&surface);
        has(RenderSurface::UiFindingCard)
            && has(RenderSurface::HeadlessJsonRow)
            && has(RenderSurface::SupportExportRow)
    }

    /// Whether the finding carries at least one repair candidate.
    pub fn has_repair_candidate(&self) -> bool {
        !self.repair_candidate_ids.is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDoctorFeatureLaneProbesSummary {
    /// Number of lanes.
    pub lane_count: usize,
    /// Number of probe families.
    pub family_count: usize,
    /// Number of findings.
    pub finding_count: usize,
    /// Findings in the healthy state.
    pub healthy_findings: usize,
    /// Findings in the partial state.
    pub partial_findings: usize,
    /// Findings in the stale state.
    pub stale_findings: usize,
    /// Findings in the unsupported state.
    pub unsupported_findings: usize,
    /// Findings in the policy-blocked state.
    pub policy_blocked_findings: usize,
    /// Findings in the target-mismatch state.
    pub target_mismatch_findings: usize,
    /// Findings that carry at least one repair candidate.
    pub findings_with_repair_candidates: usize,
    /// Findings that render stably across desktop, headless, and support.
    pub cross_context_stable_findings: usize,
}

/// A redaction-safe export row projected from a finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorFeatureLaneProbesExportRow {
    /// Finding id.
    pub finding_id: String,
    /// Finding code.
    pub finding_code: String,
    /// Lane token.
    pub lane: String,
    /// Diagnosis-state token.
    pub diagnosis_state: String,
    /// Severity token.
    pub severity_class: String,
    /// Confidence token.
    pub confidence_class: String,
    /// Explicit state-detail code (empty for healthy findings).
    pub state_detail_code: String,
    /// Scope-kind token.
    pub scope_kind: String,
    /// Opaque scope ref.
    pub scope_ref: String,
    /// Repair-candidate ids.
    pub repair_candidate_ids: Vec<String>,
    /// First actionable explanation.
    pub first_action: String,
    /// Whether the finding renders stably across desktop, headless, and support.
    pub cross_context_stable: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorFeatureLaneProbesExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<ProjectDoctorFeatureLaneProbesExportRow>,
    /// Whether every lane is covered by exactly one family.
    pub all_lanes_covered: bool,
    /// Findings that carry at least one repair candidate.
    pub repair_candidate_count: usize,
    /// Findings that render stably across desktop, headless, and support.
    pub cross_context_stable_count: usize,
}

/// The typed feature-lane probe packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDoctorFeatureLaneProbes {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed lane vocabulary; one family per lane.
    pub lanes: Vec<DoctorLane>,
    /// Closed diagnosis-state vocabulary.
    pub diagnosis_states: Vec<DiagnosisState>,
    /// Closed severity vocabulary.
    pub severity_classes: Vec<FindingSeverity>,
    /// Closed confidence vocabulary.
    pub confidence_classes: Vec<FindingConfidence>,
    /// Closed scope-kind vocabulary.
    pub scope_kinds: Vec<ScopeKind>,
    /// Closed read-only-posture vocabulary.
    pub read_only_postures: Vec<ReadOnlyPosture>,
    /// Closed render-surface vocabulary.
    pub render_surfaces: Vec<RenderSurface>,
    /// Probe families, one per lane.
    #[serde(default)]
    pub families: Vec<ProbeFamily>,
    /// Lane findings.
    #[serde(default)]
    pub findings: Vec<LaneFinding>,
    /// Summary counts.
    pub summary: ProjectDoctorFeatureLaneProbesSummary,
}

impl ProjectDoctorFeatureLaneProbes {
    /// Returns the family for a lane.
    pub fn family(&self, lane: DoctorLane) -> Option<&ProbeFamily> {
        self.families.iter().find(|f| f.lane == lane)
    }

    /// Findings for a lane.
    pub fn findings_for(&self, lane: DoctorLane) -> impl Iterator<Item = &LaneFinding> {
        self.findings.iter().filter(move |f| f.lane == lane)
    }

    /// Whether every claimed lane is covered by exactly one family.
    pub fn all_lanes_covered(&self) -> bool {
        self.lanes.iter().all(|&lane| self.family(lane).is_some())
            && self.families.len() == self.lanes.len()
    }

    /// Recomputes the summary block from the families and findings.
    pub fn computed_summary(&self) -> ProjectDoctorFeatureLaneProbesSummary {
        let count_state = |state: DiagnosisState| {
            self.findings
                .iter()
                .filter(|f| f.diagnosis_state == state)
                .count()
        };
        ProjectDoctorFeatureLaneProbesSummary {
            lane_count: self.lanes.len(),
            family_count: self.families.len(),
            finding_count: self.findings.len(),
            healthy_findings: count_state(DiagnosisState::Healthy),
            partial_findings: count_state(DiagnosisState::Partial),
            stale_findings: count_state(DiagnosisState::Stale),
            unsupported_findings: count_state(DiagnosisState::Unsupported),
            policy_blocked_findings: count_state(DiagnosisState::PolicyBlocked),
            target_mismatch_findings: count_state(DiagnosisState::TargetMismatch),
            findings_with_repair_candidates: self
                .findings
                .iter()
                .filter(|f| f.has_repair_candidate())
                .count(),
            cross_context_stable_findings: self
                .findings
                .iter()
                .filter(|f| f.is_cross_context_stable())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — Help/About,
    /// docs/help, support exports, and release/public-truth packets — render
    /// instead of restating feature-lane diagnosis text by hand.
    pub fn export_projection(&self) -> ProjectDoctorFeatureLaneProbesExportProjection {
        let rows = self
            .findings
            .iter()
            .map(|finding| ProjectDoctorFeatureLaneProbesExportRow {
                finding_id: finding.finding_id.clone(),
                finding_code: finding.finding_code.clone(),
                lane: finding.lane.as_str().to_owned(),
                diagnosis_state: finding.diagnosis_state.as_str().to_owned(),
                severity_class: finding.severity_class.as_str().to_owned(),
                confidence_class: finding.confidence_class.as_str().to_owned(),
                state_detail_code: finding.state_detail_code.clone(),
                scope_kind: finding.affected_scope.scope_kind.as_str().to_owned(),
                scope_ref: finding.affected_scope.scope_ref.clone(),
                repair_candidate_ids: finding.repair_candidate_ids.clone(),
                first_action: finding.first_action.clone(),
                cross_context_stable: finding.is_cross_context_stable(),
                summary: finding.summary.clone(),
            })
            .collect();
        ProjectDoctorFeatureLaneProbesExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_lanes_covered: self.all_lanes_covered(),
            repair_candidate_count: self
                .findings
                .iter()
                .filter(|f| f.has_repair_candidate())
                .count(),
            cross_context_stable_count: self
                .findings
                .iter()
                .filter(|f| f.is_cross_context_stable())
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ProjectDoctorFeatureLaneProbesViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_families = BTreeSet::new();
        let mut seen_lanes = BTreeSet::new();
        for family in &self.families {
            if !seen_families.insert(family.family_id.clone()) {
                violations.push(ProjectDoctorFeatureLaneProbesViolation::DuplicateFamilyId {
                    family_id: family.family_id.clone(),
                });
            }
            if !seen_lanes.insert(family.lane) {
                violations.push(
                    ProjectDoctorFeatureLaneProbesViolation::DuplicateLaneFamily {
                        lane: family.lane.as_str(),
                    },
                );
            }
            self.validate_family(family, &mut violations);
        }

        // Every claimed lane must carry its own family.
        for &lane in &self.lanes {
            if !seen_lanes.contains(&lane) {
                violations.push(ProjectDoctorFeatureLaneProbesViolation::MissingLaneFamily {
                    lane: lane.as_str(),
                });
            }
        }

        let family_index: BTreeMap<&str, &ProbeFamily> = self
            .families
            .iter()
            .map(|f| (f.family_id.as_str(), f))
            .collect();

        let mut seen_findings = BTreeSet::new();
        for finding in &self.findings {
            if !seen_findings.insert(finding.finding_id.clone()) {
                violations.push(
                    ProjectDoctorFeatureLaneProbesViolation::DuplicateFindingId {
                        finding_id: finding.finding_id.clone(),
                    },
                );
            }
            self.validate_finding(finding, &family_index, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(ProjectDoctorFeatureLaneProbesViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ProjectDoctorFeatureLaneProbesViolation>) {
        if self.schema_version != PROJECT_DOCTOR_FEATURE_LANE_PROBES_SCHEMA_VERSION {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != PROJECT_DOCTOR_FEATURE_LANE_PROBES_RECORD_KIND {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("schema_ref", &self.schema_ref),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(ProjectDoctorFeatureLaneProbesViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("lanes", self.lanes == DoctorLane::ALL.to_vec()),
            (
                "diagnosis_states",
                self.diagnosis_states == DiagnosisState::ALL.to_vec(),
            ),
            (
                "severity_classes",
                self.severity_classes == FindingSeverity::ALL.to_vec(),
            ),
            (
                "confidence_classes",
                self.confidence_classes == FindingConfidence::ALL.to_vec(),
            ),
            ("scope_kinds", self.scope_kinds == ScopeKind::ALL.to_vec()),
            (
                "read_only_postures",
                self.read_only_postures == ReadOnlyPosture::ALL.to_vec(),
            ),
            (
                "render_surfaces",
                self.render_surfaces == RenderSurface::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    ProjectDoctorFeatureLaneProbesViolation::ClosedVocabularyMismatch { field },
                );
            }
        }
    }

    fn validate_family(
        &self,
        family: &ProbeFamily,
        violations: &mut Vec<ProjectDoctorFeatureLaneProbesViolation>,
    ) {
        for (field, value) in [
            ("family_id", &family.family_id),
            ("finding_code_prefix", &family.finding_code_prefix),
            ("summary", &family.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(ProjectDoctorFeatureLaneProbesViolation::EmptyField {
                    id: family.family_id.clone(),
                    field_name: field,
                });
            }
        }

        if !family.prefix_matches_lane() {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FamilyPrefixMismatch {
                    family_id: family.family_id.clone(),
                    lane: family.lane.as_str(),
                },
            );
        }

        if family.supported_finding_codes.is_empty() {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FamilyFindingCodesEmpty {
                    family_id: family.family_id.clone(),
                },
            );
        }
        let mut seen_codes = BTreeSet::new();
        for code in &family.supported_finding_codes {
            if !code.starts_with(&family.finding_code_prefix) {
                violations.push(
                    ProjectDoctorFeatureLaneProbesViolation::FamilyFindingCodePrefix {
                        family_id: family.family_id.clone(),
                        code: code.clone(),
                    },
                );
            }
            if !seen_codes.insert(code.as_str()) {
                violations.push(
                    ProjectDoctorFeatureLaneProbesViolation::FamilyDuplicateFindingCode {
                        family_id: family.family_id.clone(),
                        code: code.clone(),
                    },
                );
            }
        }

        if family.supported_states.is_empty() {
            violations.push(ProjectDoctorFeatureLaneProbesViolation::FamilyStatesEmpty {
                family_id: family.family_id.clone(),
            });
        }
        // The lane's canonical scope kind must be one the family can diagnose.
        if !family
            .supported_scope_kinds
            .contains(&family.lane.canonical_scope_kind())
        {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FamilyMissingCanonicalScope {
                    family_id: family.family_id.clone(),
                    lane: family.lane.as_str(),
                },
            );
        }
    }

    fn validate_finding(
        &self,
        finding: &LaneFinding,
        family_index: &BTreeMap<&str, &ProbeFamily>,
        violations: &mut Vec<ProjectDoctorFeatureLaneProbesViolation>,
    ) {
        for (field, value) in [
            ("finding_id", &finding.finding_id),
            ("finding_code", &finding.finding_code),
            ("family_ref", &finding.family_ref),
            ("first_action", &finding.first_action),
            ("summary", &finding.summary),
            ("scope_ref", &finding.affected_scope.scope_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(ProjectDoctorFeatureLaneProbesViolation::EmptyField {
                    id: finding.finding_id.clone(),
                    field_name: field,
                });
            }
        }

        if !finding.finding_code.starts_with(DOCTOR_FINDING_PREFIX) {
            violations.push(ProjectDoctorFeatureLaneProbesViolation::FindingCodePrefix {
                finding_id: finding.finding_id.clone(),
            });
        }

        if finding.evidence_refs.is_empty() {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FindingEvidenceMissing {
                    finding_id: finding.finding_id.clone(),
                },
            );
        }
        if finding.render_surfaces.is_empty() {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FindingRenderSurfacesEmpty {
                    finding_id: finding.finding_id.clone(),
                },
            );
        }
        // Stable across desktop, headless, and support so the same id can be
        // reasoned about everywhere.
        if !finding.is_cross_context_stable() {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FindingNotCrossContextStable {
                    finding_id: finding.finding_id.clone(),
                },
            );
        }

        if finding.redaction_class != "metadata_safe_default" {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FindingRedactionNotSafe {
                    finding_id: finding.finding_id.clone(),
                },
            );
        }
        if !finding.raw_private_material_excluded {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FindingRawMaterialPresent {
                    finding_id: finding.finding_id.clone(),
                },
            );
        }

        // Explicit, non-generic detail for every non-healthy state.
        let detail = finding.state_detail_code.trim();
        if finding.diagnosis_state.requires_explicit_detail() {
            if detail.is_empty()
                || GENERIC_DETAIL_TOKENS.contains(&detail.to_ascii_lowercase().as_str())
            {
                violations.push(
                    ProjectDoctorFeatureLaneProbesViolation::GenericUnsupportedDetail {
                        finding_id: finding.finding_id.clone(),
                        state: finding.diagnosis_state.as_str(),
                    },
                );
            }
        } else if !detail.is_empty() {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::HealthyFindingHasDetail {
                    finding_id: finding.finding_id.clone(),
                },
            );
        }

        // Unsupported state and unsupported severity must agree, so an
        // unsupported lane is never dressed up as a lesser severity and a
        // lesser state never claims unsupported severity.
        let unsupported_state = finding.diagnosis_state == DiagnosisState::Unsupported;
        let unsupported_severity = finding.severity_class == FindingSeverity::Unsupported;
        if unsupported_state != unsupported_severity {
            violations.push(
                ProjectDoctorFeatureLaneProbesViolation::SeverityStateMismatch {
                    finding_id: finding.finding_id.clone(),
                    state: finding.diagnosis_state.as_str(),
                    severity: finding.severity_class.as_str(),
                },
            );
        }

        for id in &finding.repair_candidate_ids {
            if id.trim().is_empty() {
                violations.push(
                    ProjectDoctorFeatureLaneProbesViolation::EmptyRepairCandidateId {
                        finding_id: finding.finding_id.clone(),
                    },
                );
            } else if !id.starts_with(DOCTOR_REPAIR_CANDIDATE_PREFIX) {
                violations.push(
                    ProjectDoctorFeatureLaneProbesViolation::RepairCandidatePrefix {
                        finding_id: finding.finding_id.clone(),
                        candidate: id.clone(),
                    },
                );
            }
        }

        match family_index.get(finding.family_ref.as_str()) {
            None => violations.push(
                ProjectDoctorFeatureLaneProbesViolation::FindingFamilyRefUnknown {
                    finding_id: finding.finding_id.clone(),
                    family_ref: finding.family_ref.clone(),
                },
            ),
            Some(family) => {
                if family.lane != finding.lane {
                    violations.push(
                        ProjectDoctorFeatureLaneProbesViolation::FindingLaneMismatch {
                            finding_id: finding.finding_id.clone(),
                            finding_lane: finding.lane.as_str(),
                            family_lane: family.lane.as_str(),
                        },
                    );
                }
                if !family
                    .supported_finding_codes
                    .iter()
                    .any(|code| code == &finding.finding_code)
                {
                    violations.push(
                        ProjectDoctorFeatureLaneProbesViolation::FindingCodeNotSupported {
                            finding_id: finding.finding_id.clone(),
                            family_id: family.family_id.clone(),
                        },
                    );
                }
                if !family
                    .supported_states
                    .iter()
                    .any(|state| *state == finding.diagnosis_state)
                {
                    violations.push(
                        ProjectDoctorFeatureLaneProbesViolation::FindingStateNotSupported {
                            finding_id: finding.finding_id.clone(),
                            family_id: family.family_id.clone(),
                            state: finding.diagnosis_state.as_str(),
                        },
                    );
                }
                if !family
                    .supported_scope_kinds
                    .iter()
                    .any(|kind| *kind == finding.affected_scope.scope_kind)
                {
                    violations.push(
                        ProjectDoctorFeatureLaneProbesViolation::FindingScopeNotSupported {
                            finding_id: finding.finding_id.clone(),
                            family_id: family.family_id.clone(),
                            scope_kind: finding.affected_scope.scope_kind.as_str(),
                        },
                    );
                }
                // A read-only lane that does not emit repair candidates may not
                // attach any, so no speculative remediation leaks in.
                if finding.has_repair_candidate() && !family.emits_repair_candidates {
                    violations.push(
                        ProjectDoctorFeatureLaneProbesViolation::RepairCandidateNotPermitted {
                            finding_id: finding.finding_id.clone(),
                            family_id: family.family_id.clone(),
                        },
                    );
                }
                // A headless-only-denied family may not carry a headless surface.
                if !family.headless_supported
                    && finding
                        .render_surfaces
                        .contains(&RenderSurface::HeadlessJsonRow)
                {
                    violations.push(
                        ProjectDoctorFeatureLaneProbesViolation::FindingHeadlessNotSupported {
                            finding_id: finding.finding_id.clone(),
                            family_id: family.family_id.clone(),
                        },
                    );
                }
            }
        }
    }
}

/// A validation violation for the feature-lane probe packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectDoctorFeatureLaneProbesViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Record or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A family id appears more than once.
    DuplicateFamilyId {
        /// Duplicate family id.
        family_id: String,
    },
    /// A lane carries more than one family.
    DuplicateLaneFamily {
        /// Lane token.
        lane: &'static str,
    },
    /// A claimed lane has no family.
    MissingLaneFamily {
        /// Lane token.
        lane: &'static str,
    },
    /// A family's finding-code prefix does not match its lane.
    FamilyPrefixMismatch {
        /// Family id.
        family_id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// A family declares no supported finding codes.
    FamilyFindingCodesEmpty {
        /// Family id.
        family_id: String,
    },
    /// A family supported finding code does not use the family prefix.
    FamilyFindingCodePrefix {
        /// Family id.
        family_id: String,
        /// Offending code.
        code: String,
    },
    /// A family repeats a supported finding code.
    FamilyDuplicateFindingCode {
        /// Family id.
        family_id: String,
        /// Duplicated code.
        code: String,
    },
    /// A family declares no supported diagnosis states.
    FamilyStatesEmpty {
        /// Family id.
        family_id: String,
    },
    /// A family cannot diagnose its lane's canonical scope kind.
    FamilyMissingCanonicalScope {
        /// Family id.
        family_id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// A finding id appears more than once.
    DuplicateFindingId {
        /// Duplicate finding id.
        finding_id: String,
    },
    /// A finding code does not use the global Doctor finding prefix.
    FindingCodePrefix {
        /// Finding id.
        finding_id: String,
    },
    /// A finding cites no evidence ref.
    FindingEvidenceMissing {
        /// Finding id.
        finding_id: String,
    },
    /// A finding declares no render surface.
    FindingRenderSurfacesEmpty {
        /// Finding id.
        finding_id: String,
    },
    /// A finding does not render stably across desktop, headless, and support.
    FindingNotCrossContextStable {
        /// Finding id.
        finding_id: String,
    },
    /// A finding's redaction class is not metadata-safe.
    FindingRedactionNotSafe {
        /// Finding id.
        finding_id: String,
    },
    /// A finding does not exclude raw private material.
    FindingRawMaterialPresent {
        /// Finding id.
        finding_id: String,
    },
    /// A non-healthy state carries an empty or generic detail code.
    GenericUnsupportedDetail {
        /// Finding id.
        finding_id: String,
        /// State token.
        state: &'static str,
    },
    /// A healthy finding carries a non-empty detail code.
    HealthyFindingHasDetail {
        /// Finding id.
        finding_id: String,
    },
    /// The diagnosis state and severity disagree on the unsupported axis.
    SeverityStateMismatch {
        /// Finding id.
        finding_id: String,
        /// State token.
        state: &'static str,
        /// Severity token.
        severity: &'static str,
    },
    /// A repair-candidate id is empty.
    EmptyRepairCandidateId {
        /// Finding id.
        finding_id: String,
    },
    /// A repair-candidate id does not use the repair prefix.
    RepairCandidatePrefix {
        /// Finding id.
        finding_id: String,
        /// Offending candidate id.
        candidate: String,
    },
    /// A finding references a family id not present in the packet.
    FindingFamilyRefUnknown {
        /// Finding id.
        finding_id: String,
        /// Unknown family ref.
        family_ref: String,
    },
    /// A finding's lane disagrees with its family's lane.
    FindingLaneMismatch {
        /// Finding id.
        finding_id: String,
        /// Finding lane token.
        finding_lane: &'static str,
        /// Family lane token.
        family_lane: &'static str,
    },
    /// A finding code is not in its family's supported codes.
    FindingCodeNotSupported {
        /// Finding id.
        finding_id: String,
        /// Family id.
        family_id: String,
    },
    /// A finding's diagnosis state is not supported by its family.
    FindingStateNotSupported {
        /// Finding id.
        finding_id: String,
        /// Family id.
        family_id: String,
        /// State token.
        state: &'static str,
    },
    /// A finding's scope kind is not supported by its family.
    FindingScopeNotSupported {
        /// Finding id.
        finding_id: String,
        /// Family id.
        family_id: String,
        /// Scope-kind token.
        scope_kind: &'static str,
    },
    /// A finding attaches repair candidates in a lane that emits none.
    RepairCandidateNotPermitted {
        /// Finding id.
        finding_id: String,
        /// Family id.
        family_id: String,
    },
    /// A finding renders headless under a family that does not support headless.
    FindingHeadlessNotSupported {
        /// Finding id.
        finding_id: String,
        /// Family id.
        family_id: String,
    },
    /// The summary counts disagree with the families and findings.
    SummaryMismatch,
}

impl fmt::Display for ProjectDoctorFeatureLaneProbesViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateFamilyId { family_id } => {
                write!(f, "duplicate family id {family_id}")
            }
            Self::DuplicateLaneFamily { lane } => {
                write!(f, "duplicate family for lane {lane}")
            }
            Self::MissingLaneFamily { lane } => {
                write!(f, "missing family for lane {lane}")
            }
            Self::FamilyPrefixMismatch { family_id, lane } => {
                write!(f, "family {family_id} prefix does not match lane {lane}")
            }
            Self::FamilyFindingCodesEmpty { family_id } => {
                write!(f, "family {family_id} declares no supported finding codes")
            }
            Self::FamilyFindingCodePrefix { family_id, code } => {
                write!(
                    f,
                    "family {family_id} code {code} does not use the family prefix"
                )
            }
            Self::FamilyDuplicateFindingCode { family_id, code } => {
                write!(f, "family {family_id} repeats finding code {code}")
            }
            Self::FamilyStatesEmpty { family_id } => {
                write!(f, "family {family_id} declares no supported states")
            }
            Self::FamilyMissingCanonicalScope { family_id, lane } => {
                write!(
                    f,
                    "family {family_id} cannot diagnose the canonical scope for lane {lane}"
                )
            }
            Self::DuplicateFindingId { finding_id } => {
                write!(f, "duplicate finding id {finding_id}")
            }
            Self::FindingCodePrefix { finding_id } => {
                write!(
                    f,
                    "finding {finding_id} code does not start with {DOCTOR_FINDING_PREFIX}"
                )
            }
            Self::FindingEvidenceMissing { finding_id } => {
                write!(f, "finding {finding_id} cites no evidence ref")
            }
            Self::FindingRenderSurfacesEmpty { finding_id } => {
                write!(f, "finding {finding_id} declares no render surface")
            }
            Self::FindingNotCrossContextStable { finding_id } => {
                write!(
                    f,
                    "finding {finding_id} is not stable across desktop, headless, and support"
                )
            }
            Self::FindingRedactionNotSafe { finding_id } => {
                write!(
                    f,
                    "finding {finding_id} redaction_class must be metadata_safe_default"
                )
            }
            Self::FindingRawMaterialPresent { finding_id } => {
                write!(
                    f,
                    "finding {finding_id} raw_private_material_excluded must be true"
                )
            }
            Self::GenericUnsupportedDetail { finding_id, state } => {
                write!(
                    f,
                    "finding {finding_id} state {state} hides behind a generic/empty detail code"
                )
            }
            Self::HealthyFindingHasDetail { finding_id } => {
                write!(
                    f,
                    "finding {finding_id} is healthy but carries a state detail code"
                )
            }
            Self::SeverityStateMismatch {
                finding_id,
                state,
                severity,
            } => {
                write!(
                    f,
                    "finding {finding_id} state {state} and severity {severity} disagree on unsupported"
                )
            }
            Self::EmptyRepairCandidateId { finding_id } => {
                write!(
                    f,
                    "finding {finding_id} carries an empty repair-candidate id"
                )
            }
            Self::RepairCandidatePrefix {
                finding_id,
                candidate,
            } => {
                write!(
                    f,
                    "finding {finding_id} repair candidate {candidate} does not use the {DOCTOR_REPAIR_CANDIDATE_PREFIX} prefix"
                )
            }
            Self::FindingFamilyRefUnknown {
                finding_id,
                family_ref,
            } => {
                write!(
                    f,
                    "finding {finding_id} references unknown family {family_ref}"
                )
            }
            Self::FindingLaneMismatch {
                finding_id,
                finding_lane,
                family_lane,
            } => {
                write!(
                    f,
                    "finding {finding_id} lane {finding_lane} does not match family lane {family_lane}"
                )
            }
            Self::FindingCodeNotSupported {
                finding_id,
                family_id,
            } => {
                write!(
                    f,
                    "finding {finding_id} code is not supported by family {family_id}"
                )
            }
            Self::FindingStateNotSupported {
                finding_id,
                family_id,
                state,
            } => {
                write!(
                    f,
                    "finding {finding_id} state {state} is not supported by family {family_id}"
                )
            }
            Self::FindingScopeNotSupported {
                finding_id,
                family_id,
                scope_kind,
            } => {
                write!(
                    f,
                    "finding {finding_id} scope {scope_kind} is not supported by family {family_id}"
                )
            }
            Self::RepairCandidateNotPermitted {
                finding_id,
                family_id,
            } => {
                write!(
                    f,
                    "finding {finding_id} attaches repair candidates but family {family_id} emits none"
                )
            }
            Self::FindingHeadlessNotSupported {
                finding_id,
                family_id,
            } => {
                write!(
                    f,
                    "finding {finding_id} renders headless but family {family_id} denies headless"
                )
            }
            Self::SummaryMismatch => {
                write!(
                    f,
                    "packet summary counts disagree with families and findings"
                )
            }
        }
    }
}

impl Error for ProjectDoctorFeatureLaneProbesViolation {}

/// Loads the embedded feature-lane probe packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`ProjectDoctorFeatureLaneProbes`].
pub fn current_project_doctor_feature_lane_probes(
) -> Result<ProjectDoctorFeatureLaneProbes, serde_json::Error> {
    serde_json::from_str(PROJECT_DOCTOR_FEATURE_LANE_PROBES_JSON)
}

#[cfg(test)]
mod tests;
