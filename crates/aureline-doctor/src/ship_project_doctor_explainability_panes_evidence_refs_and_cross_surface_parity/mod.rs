//! Project Doctor explainability panes, evidence refs, repair-candidate ids,
//! and desktop/CLI/headless/support parity for the M5 feature lanes.
//!
//! This module owns the canonical packet that turns each Project Doctor finding
//! into an inspectable **explainability pane**: a single record that exposes the
//! stable [`ExplainabilityPane::finding_code`], the probe id and
//! [`ExplainabilityPane::probe_version`] that produced it, the
//! [`ExplainabilityPane::evidence_refs`] that back it, the affected scope, and —
//! the central addition over the raw feature-lane probe packet — **why a
//! candidate repair is or is not available**. Every pane carries a
//! [`RepairAvailability`] that either names an available
//! [`ExplainabilityPane::repair_candidate_id`] with its
//! [`ReversalClass`], or names a specific, non-generic
//! [`ExplainabilityPane::repair_unavailable_reason_code`] explaining the block.
//!
//! The packet also pins **cross-surface parity**. Each pane records the
//! [`ParitySurface`]s it renders on and a canonical [`CliExitClass`] derived from
//! its [`DiagnosisState`], so the desktop pane, the interactive CLI row, the
//! headless JSON row, the support export, the incident packet, and the
//! public-truth surface all carry the same machine meaning and the same exit
//! semantics. The [`ExplainabilityPane::machine_meaning_keys`] name the
//! locale-invariant JSON keys: localized prose is additive and may never change
//! the finding code, diagnosis state, repair availability, or exit class.
//!
//! Diagnosis stays **read-only by construction**: a pane is metadata about a
//! finding, names an opaque scope ref, and excludes raw private material. A
//! healthy pane carries no repair candidate and no unavailable reason, so no
//! speculative remediation leaks into a healthy lane.
//!
//! The packet is checked in at
//! `artifacts/doctor/m5/project-doctor-explainability-parity.json` and embedded
//! here, so this typed consumer and any CI gate agree on every row without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, or mount/port/tunnel
//! secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported explainability/parity packet schema version.
pub const PROJECT_DOCTOR_EXPLAINABILITY_PARITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PROJECT_DOCTOR_EXPLAINABILITY_PARITY_RECORD_KIND: &str =
    "project_doctor_explainability_parity";

/// Repo-relative path to the checked-in packet.
pub const PROJECT_DOCTOR_EXPLAINABILITY_PARITY_PATH: &str =
    "artifacts/doctor/m5/project-doctor-explainability-parity.json";

/// Repo-relative path to the boundary schema.
pub const PROJECT_DOCTOR_EXPLAINABILITY_PARITY_SCHEMA_REF: &str =
    "schemas/doctor/project-doctor-explainability-parity.schema.json";

/// Repo-relative path to the companion document.
pub const PROJECT_DOCTOR_EXPLAINABILITY_PARITY_DOC_REF: &str =
    "docs/doctor/m5/project-doctor-explainability-parity.md";

/// Stable finding-code prefix every pane's finding code must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Stable repair-candidate id prefix every repair candidate must use.
pub const DOCTOR_REPAIR_CANDIDATE_PREFIX: &str = "repair.";

/// Canonical, locale-invariant machine-meaning keys every pane must carry, so
/// localized prose can never silently change what a surface means.
pub const REQUIRED_MACHINE_MEANING_KEYS: [&str; 4] = [
    "finding_code",
    "diagnosis_state",
    "repair_availability",
    "cli_exit_class",
];

/// Embedded checked-in packet JSON.
pub const PROJECT_DOCTOR_EXPLAINABILITY_PARITY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/doctor/m5/project-doctor-explainability-parity.json"
));

/// Generic, non-actionable detail tokens that may never stand in for an explicit
/// repair-unavailable reason.
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

/// The explicit diagnosis state the underlying finding reports.
///
/// Every state but [`DiagnosisState::Healthy`] is a distinct, named condition
/// reported with its own detail rather than folded into a generic "unavailable"
/// string.
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

    /// The canonical CLI exit class this state maps to.
    ///
    /// The mapping is fixed so the interactive CLI, headless runs, and the
    /// desktop product agree on exit semantics for every diagnosis state.
    pub const fn canonical_exit_class(self) -> CliExitClass {
        match self {
            Self::Healthy => CliExitClass::OkHealthy,
            Self::Partial | Self::Stale => CliExitClass::AdvisoryFindings,
            Self::TargetMismatch => CliExitClass::Blocked,
            Self::Unsupported => CliExitClass::Unsupported,
            Self::PolicyBlocked => CliExitClass::PolicyDenied,
        }
    }
}

impl fmt::Display for DiagnosisState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether a reviewed repair candidate is available for a pane, and if not, the
/// shape of the block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairAvailability {
    /// A reviewed repair candidate is available and named.
    Available,
    /// No repair applies: the lane is healthy.
    NotApplicableHealthy,
    /// A repair exists in principle but is unsupported in this context.
    BlockedUnsupportedContext,
    /// A managed policy blocks the repair.
    BlockedManagedPolicy,
    /// The repair cannot run until partial diagnosis evidence is completed.
    BlockedPartialEvidence,
    /// The repair's reversal path is unproven, so it is withheld.
    BlockedReversalUnproven,
}

impl RepairAvailability {
    /// Every repair-availability class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Available,
        Self::NotApplicableHealthy,
        Self::BlockedUnsupportedContext,
        Self::BlockedManagedPolicy,
        Self::BlockedPartialEvidence,
        Self::BlockedReversalUnproven,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::NotApplicableHealthy => "not_applicable_healthy",
            Self::BlockedUnsupportedContext => "blocked_unsupported_context",
            Self::BlockedManagedPolicy => "blocked_managed_policy",
            Self::BlockedPartialEvidence => "blocked_partial_evidence",
            Self::BlockedReversalUnproven => "blocked_reversal_unproven",
        }
    }

    /// Whether this class names an available, attachable repair candidate.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Whether this class is a `blocked_*` reason requiring an explicit,
    /// non-generic reason code.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedUnsupportedContext
                | Self::BlockedManagedPolicy
                | Self::BlockedPartialEvidence
                | Self::BlockedReversalUnproven
        )
    }
}

/// The reversal class of an available repair candidate.
///
/// A pane without an available repair must be [`ReversalClass::NotApplicable`];
/// an available repair must name a real reversal class so the user can reason
/// about how the change can be undone before applying it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversalClass {
    /// The repair runs inside a transaction and reverses cleanly.
    ReversibleTransactional,
    /// The repair reverses by restoring a captured snapshot.
    ReversibleWithSnapshot,
    /// The repair is irreversible and is gated behind an explicit guard.
    IrreversibleGuarded,
    /// No repair applies, so there is no reversal class.
    NotApplicable,
}

impl ReversalClass {
    /// Every reversal class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReversibleTransactional,
        Self::ReversibleWithSnapshot,
        Self::IrreversibleGuarded,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleTransactional => "reversible_transactional",
            Self::ReversibleWithSnapshot => "reversible_with_snapshot",
            Self::IrreversibleGuarded => "irreversible_guarded",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this class names a real, applicable repair reversal.
    pub const fn is_applicable(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// A canonical CLI/headless exit class.
///
/// Each class maps to a stable exit code so `aureline doctor` and headless runs
/// return the same code for the same diagnosis state across surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliExitClass {
    /// Everything healthy; exit 0.
    OkHealthy,
    /// Non-blocking findings present (partial/stale); exit 10.
    AdvisoryFindings,
    /// A finding blocks the requested workflow; exit 20.
    Blocked,
    /// The requested capability is unsupported here; exit 30.
    Unsupported,
    /// A managed policy denies the capability; exit 40.
    PolicyDenied,
}

impl CliExitClass {
    /// Every exit class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OkHealthy,
        Self::AdvisoryFindings,
        Self::Blocked,
        Self::Unsupported,
        Self::PolicyDenied,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OkHealthy => "ok_healthy",
            Self::AdvisoryFindings => "advisory_findings",
            Self::Blocked => "blocked",
            Self::Unsupported => "unsupported",
            Self::PolicyDenied => "policy_denied",
        }
    }

    /// The stable process exit code for this class.
    pub const fn exit_code(self) -> i32 {
        match self {
            Self::OkHealthy => 0,
            Self::AdvisoryFindings => 10,
            Self::Blocked => 20,
            Self::Unsupported => 30,
            Self::PolicyDenied => 40,
        }
    }
}

/// A surface a pane must render on with identical machine meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParitySurface {
    /// Desktop Project Doctor explainability pane.
    DesktopPane,
    /// Interactive CLI finding row.
    CliRow,
    /// Headless JSON row.
    HeadlessJson,
    /// Support export row.
    SupportExport,
    /// Incident packet row.
    IncidentPacket,
    /// Public-truth/release surface row.
    PublicTruth,
}

impl ParitySurface {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopPane,
        Self::CliRow,
        Self::HeadlessJson,
        Self::SupportExport,
        Self::IncidentPacket,
        Self::PublicTruth,
    ];

    /// The core surfaces every pane must render on to be cross-surface stable.
    pub const CORE: [Self; 4] = [
        Self::DesktopPane,
        Self::CliRow,
        Self::HeadlessJson,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopPane => "desktop_pane",
            Self::CliRow => "cli_row",
            Self::HeadlessJson => "headless_json",
            Self::SupportExport => "support_export",
            Self::IncidentPacket => "incident_packet",
            Self::PublicTruth => "public_truth",
        }
    }
}

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

/// The affected-scope identity a pane is about.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AffectedScope {
    /// Stable scope-kind token (lane engine/route/target/session/packet kind).
    pub scope_kind: String,
    /// Opaque, redaction-safe scope reference.
    pub scope_ref: String,
}

/// One Project Doctor explainability pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainabilityPane {
    /// Stable pane id.
    pub pane_id: String,
    /// Stable finding code (must start with [`DOCTOR_FINDING_PREFIX`]).
    pub finding_code: String,
    /// Probe id that produced the finding.
    pub probe_id: String,
    /// Probe version that produced the finding (referenced by support/release).
    pub probe_version: String,
    /// Explicit diagnosis state.
    pub diagnosis_state: DiagnosisState,
    /// Affected-scope identity.
    pub affected_scope: AffectedScope,
    /// Evidence refs backing the finding (at least one).
    pub evidence_refs: Vec<String>,
    /// Whether and why a repair candidate is available.
    pub repair_availability: RepairAvailability,
    /// Repair-candidate id (present iff [`RepairAvailability::Available`]).
    #[serde(default)]
    pub repair_candidate_id: String,
    /// Explicit, non-generic reason a repair is withheld (present iff a
    /// `blocked_*` availability; empty otherwise).
    #[serde(default)]
    pub repair_unavailable_reason_code: String,
    /// Reversal class of the available repair (`not_applicable` when none).
    pub reversal_class: ReversalClass,
    /// Canonical CLI/headless exit class (must equal the state's canonical map).
    pub cli_exit_class: CliExitClass,
    /// Surfaces this pane renders on with identical machine meaning.
    pub parity_surfaces: Vec<ParitySurface>,
    /// Locale-invariant JSON keys whose values may never change with locale.
    pub machine_meaning_keys: Vec<String>,
    /// Human-readable explanation (additive, localizable prose).
    pub explanation: String,
    /// Redaction class (must be metadata-safe).
    pub redaction_class: String,
    /// Whether raw private material is excluded (must be true).
    pub raw_private_material_excluded: bool,
    /// Reviewer-safe summary.
    pub summary: String,
}

impl ExplainabilityPane {
    /// Whether the pane renders on every core surface, so desktop, CLI,
    /// headless, and support reason about the same machine meaning.
    pub fn is_cross_surface_stable(&self) -> bool {
        ParitySurface::CORE
            .iter()
            .all(|surface| self.parity_surfaces.contains(surface))
    }

    /// Whether the pane names an available, attachable repair candidate.
    pub fn has_available_repair(&self) -> bool {
        self.repair_availability.is_available()
    }

    /// Whether the pane's exit class matches the canonical map for its state.
    pub fn exit_class_is_canonical(&self) -> bool {
        self.cli_exit_class == self.diagnosis_state.canonical_exit_class()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDoctorExplainabilityParitySummary {
    /// Number of panes.
    pub pane_count: usize,
    /// Panes whose underlying finding is healthy.
    pub healthy_panes: usize,
    /// Panes with an available repair candidate.
    pub panes_with_available_repair: usize,
    /// Panes whose repair is blocked with an explicit reason.
    pub panes_with_blocked_repair: usize,
    /// Panes that render stably across the core surfaces.
    pub cross_surface_stable_panes: usize,
    /// Panes with exit class `ok_healthy`.
    pub exit_ok_panes: usize,
    /// Panes with exit class `advisory_findings`.
    pub exit_advisory_panes: usize,
    /// Panes with exit class `blocked`.
    pub exit_blocked_panes: usize,
    /// Panes with exit class `unsupported`.
    pub exit_unsupported_panes: usize,
    /// Panes with exit class `policy_denied`.
    pub exit_policy_denied_panes: usize,
}

/// A redaction-safe export row projected from a pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorExplainabilityParityExportRow {
    /// Pane id.
    pub pane_id: String,
    /// Finding code.
    pub finding_code: String,
    /// Probe id.
    pub probe_id: String,
    /// Probe version.
    pub probe_version: String,
    /// Diagnosis-state token.
    pub diagnosis_state: String,
    /// Scope-kind token.
    pub scope_kind: String,
    /// Opaque scope ref.
    pub scope_ref: String,
    /// Repair-availability token.
    pub repair_availability: String,
    /// Repair-candidate id (empty when no repair is available).
    pub repair_candidate_id: String,
    /// Explicit repair-unavailable reason code (empty when available/healthy).
    pub repair_unavailable_reason_code: String,
    /// Reversal-class token.
    pub reversal_class: String,
    /// CLI exit-class token.
    pub cli_exit_class: String,
    /// Stable CLI exit code.
    pub cli_exit_code: i32,
    /// Whether the pane renders stably across the core surfaces.
    pub cross_surface_stable: bool,
    /// Human-readable explanation.
    pub explanation: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorExplainabilityParityExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<ProjectDoctorExplainabilityParityExportRow>,
    /// Panes with an available repair candidate.
    pub available_repair_count: usize,
    /// Panes whose repair is blocked with an explicit reason.
    pub blocked_repair_count: usize,
    /// Panes that render stably across the core surfaces.
    pub cross_surface_stable_count: usize,
}

/// The typed explainability/parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDoctorExplainabilityParity {
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
    /// Closed diagnosis-state vocabulary.
    pub diagnosis_states: Vec<DiagnosisState>,
    /// Closed repair-availability vocabulary.
    pub repair_availabilities: Vec<RepairAvailability>,
    /// Closed reversal-class vocabulary.
    pub reversal_classes: Vec<ReversalClass>,
    /// Closed CLI exit-class vocabulary.
    pub cli_exit_classes: Vec<CliExitClass>,
    /// Closed parity-surface vocabulary.
    pub parity_surfaces: Vec<ParitySurface>,
    /// Explainability panes.
    #[serde(default)]
    pub panes: Vec<ExplainabilityPane>,
    /// Summary counts.
    pub summary: ProjectDoctorExplainabilityParitySummary,
}

impl ProjectDoctorExplainabilityParity {
    /// Panes whose underlying finding is in a given diagnosis state.
    pub fn panes_in_state(
        &self,
        state: DiagnosisState,
    ) -> impl Iterator<Item = &ExplainabilityPane> {
        self.panes
            .iter()
            .filter(move |p| p.diagnosis_state == state)
    }

    /// Recomputes the summary block from the panes.
    pub fn computed_summary(&self) -> ProjectDoctorExplainabilityParitySummary {
        let count_exit = |class: CliExitClass| {
            self.panes
                .iter()
                .filter(|p| p.cli_exit_class == class)
                .count()
        };
        ProjectDoctorExplainabilityParitySummary {
            pane_count: self.panes.len(),
            healthy_panes: self
                .panes
                .iter()
                .filter(|p| p.diagnosis_state == DiagnosisState::Healthy)
                .count(),
            panes_with_available_repair: self
                .panes
                .iter()
                .filter(|p| p.has_available_repair())
                .count(),
            panes_with_blocked_repair: self
                .panes
                .iter()
                .filter(|p| p.repair_availability.is_blocked())
                .count(),
            cross_surface_stable_panes: self
                .panes
                .iter()
                .filter(|p| p.is_cross_surface_stable())
                .count(),
            exit_ok_panes: count_exit(CliExitClass::OkHealthy),
            exit_advisory_panes: count_exit(CliExitClass::AdvisoryFindings),
            exit_blocked_panes: count_exit(CliExitClass::Blocked),
            exit_unsupported_panes: count_exit(CliExitClass::Unsupported),
            exit_policy_denied_panes: count_exit(CliExitClass::PolicyDenied),
        }
    }

    /// Produces an export projection that downstream surfaces — Help/About,
    /// docs/help, support exports, incident packets, and release/public-truth
    /// packets — render instead of restating explainability text by hand.
    pub fn export_projection(&self) -> ProjectDoctorExplainabilityParityExportProjection {
        let rows = self
            .panes
            .iter()
            .map(|pane| ProjectDoctorExplainabilityParityExportRow {
                pane_id: pane.pane_id.clone(),
                finding_code: pane.finding_code.clone(),
                probe_id: pane.probe_id.clone(),
                probe_version: pane.probe_version.clone(),
                diagnosis_state: pane.diagnosis_state.as_str().to_owned(),
                scope_kind: pane.affected_scope.scope_kind.clone(),
                scope_ref: pane.affected_scope.scope_ref.clone(),
                repair_availability: pane.repair_availability.as_str().to_owned(),
                repair_candidate_id: pane.repair_candidate_id.clone(),
                repair_unavailable_reason_code: pane.repair_unavailable_reason_code.clone(),
                reversal_class: pane.reversal_class.as_str().to_owned(),
                cli_exit_class: pane.cli_exit_class.as_str().to_owned(),
                cli_exit_code: pane.cli_exit_class.exit_code(),
                cross_surface_stable: pane.is_cross_surface_stable(),
                explanation: pane.explanation.clone(),
            })
            .collect();
        ProjectDoctorExplainabilityParityExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            available_repair_count: self
                .panes
                .iter()
                .filter(|p| p.has_available_repair())
                .count(),
            blocked_repair_count: self
                .panes
                .iter()
                .filter(|p| p.repair_availability.is_blocked())
                .count(),
            cross_surface_stable_count: self
                .panes
                .iter()
                .filter(|p| p.is_cross_surface_stable())
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ProjectDoctorExplainabilityParityViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_panes = BTreeSet::new();
        for pane in &self.panes {
            if !seen_panes.insert(pane.pane_id.clone()) {
                violations.push(
                    ProjectDoctorExplainabilityParityViolation::DuplicatePaneId {
                        pane_id: pane.pane_id.clone(),
                    },
                );
            }
            self.validate_pane(pane, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(ProjectDoctorExplainabilityParityViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ProjectDoctorExplainabilityParityViolation>) {
        if self.schema_version != PROJECT_DOCTOR_EXPLAINABILITY_PARITY_SCHEMA_VERSION {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != PROJECT_DOCTOR_EXPLAINABILITY_PARITY_RECORD_KIND {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::UnsupportedRecordKind {
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
                violations.push(ProjectDoctorExplainabilityParityViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "diagnosis_states",
                self.diagnosis_states == DiagnosisState::ALL.to_vec(),
            ),
            (
                "repair_availabilities",
                self.repair_availabilities == RepairAvailability::ALL.to_vec(),
            ),
            (
                "reversal_classes",
                self.reversal_classes == ReversalClass::ALL.to_vec(),
            ),
            (
                "cli_exit_classes",
                self.cli_exit_classes == CliExitClass::ALL.to_vec(),
            ),
            (
                "parity_surfaces",
                self.parity_surfaces == ParitySurface::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    ProjectDoctorExplainabilityParityViolation::ClosedVocabularyMismatch { field },
                );
            }
        }
    }

    fn validate_pane(
        &self,
        pane: &ExplainabilityPane,
        violations: &mut Vec<ProjectDoctorExplainabilityParityViolation>,
    ) {
        for (field, value) in [
            ("pane_id", &pane.pane_id),
            ("finding_code", &pane.finding_code),
            ("probe_id", &pane.probe_id),
            ("probe_version", &pane.probe_version),
            ("explanation", &pane.explanation),
            ("summary", &pane.summary),
            ("scope_kind", &pane.affected_scope.scope_kind),
            ("scope_ref", &pane.affected_scope.scope_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(ProjectDoctorExplainabilityParityViolation::EmptyField {
                    id: pane.pane_id.clone(),
                    field_name: field,
                });
            }
        }

        if !pane.finding_code.starts_with(DOCTOR_FINDING_PREFIX) {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::FindingCodePrefix {
                    pane_id: pane.pane_id.clone(),
                },
            );
        }

        if pane.evidence_refs.is_empty() {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::EvidenceMissing {
                    pane_id: pane.pane_id.clone(),
                },
            );
        }

        if pane.parity_surfaces.is_empty() {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::ParitySurfacesEmpty {
                    pane_id: pane.pane_id.clone(),
                },
            );
        }
        if !pane.is_cross_surface_stable() {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::PaneNotCrossSurfaceStable {
                    pane_id: pane.pane_id.clone(),
                },
            );
        }

        // The locale-invariant keys must all be present, so localized prose can
        // never silently change machine meaning.
        for required in REQUIRED_MACHINE_MEANING_KEYS {
            if !pane.machine_meaning_keys.iter().any(|k| k == required) {
                violations.push(
                    ProjectDoctorExplainabilityParityViolation::MissingMachineMeaningKey {
                        pane_id: pane.pane_id.clone(),
                        key: required,
                    },
                );
            }
        }

        if pane.redaction_class != "metadata_safe_default" {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::RedactionNotSafe {
                    pane_id: pane.pane_id.clone(),
                },
            );
        }
        if !pane.raw_private_material_excluded {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::RawMaterialPresent {
                    pane_id: pane.pane_id.clone(),
                },
            );
        }

        // Exit semantics must match the canonical state map, so CLI/headless and
        // desktop never disagree on exit class for the same finding.
        if !pane.exit_class_is_canonical() {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::ExitClassNotCanonical {
                    pane_id: pane.pane_id.clone(),
                    state: pane.diagnosis_state.as_str(),
                    exit_class: pane.cli_exit_class.as_str(),
                },
            );
        }

        // Healthy panes must declare no repair and no block; non-applicable
        // availability is reserved for the healthy state.
        let healthy = pane.diagnosis_state == DiagnosisState::Healthy;
        let not_applicable = pane.repair_availability == RepairAvailability::NotApplicableHealthy;
        if healthy != not_applicable {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::HealthyAvailabilityMismatch {
                    pane_id: pane.pane_id.clone(),
                    state: pane.diagnosis_state.as_str(),
                    availability: pane.repair_availability.as_str(),
                },
            );
        }

        // Repair-candidate id and unavailable reason must agree with the
        // availability class — an available repair names a candidate, a blocked
        // repair names a reason, and a not-applicable repair names neither.
        let has_candidate = !pane.repair_candidate_id.trim().is_empty();
        let has_reason = !pane.repair_unavailable_reason_code.trim().is_empty();
        if pane.repair_availability.is_available() {
            if !has_candidate {
                violations.push(
                    ProjectDoctorExplainabilityParityViolation::AvailableRepairMissingCandidate {
                        pane_id: pane.pane_id.clone(),
                    },
                );
            }
            if has_reason {
                violations.push(
                    ProjectDoctorExplainabilityParityViolation::AvailableRepairHasReason {
                        pane_id: pane.pane_id.clone(),
                    },
                );
            }
        } else {
            if has_candidate {
                violations.push(
                    ProjectDoctorExplainabilityParityViolation::UnavailableRepairHasCandidate {
                        pane_id: pane.pane_id.clone(),
                    },
                );
            }
            if pane.repair_availability.is_blocked() {
                let reason = pane.repair_unavailable_reason_code.trim();
                if reason.is_empty()
                    || GENERIC_DETAIL_TOKENS.contains(&reason.to_ascii_lowercase().as_str())
                {
                    violations.push(
                        ProjectDoctorExplainabilityParityViolation::GenericBlockedReason {
                            pane_id: pane.pane_id.clone(),
                            availability: pane.repair_availability.as_str(),
                        },
                    );
                }
            } else if has_reason {
                // not_applicable_healthy must carry no reason.
                violations.push(
                    ProjectDoctorExplainabilityParityViolation::HealthyRepairHasReason {
                        pane_id: pane.pane_id.clone(),
                    },
                );
            }
        }

        // A named repair candidate must use the repair prefix.
        if has_candidate
            && !pane
                .repair_candidate_id
                .starts_with(DOCTOR_REPAIR_CANDIDATE_PREFIX)
        {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::RepairCandidatePrefix {
                    pane_id: pane.pane_id.clone(),
                    candidate: pane.repair_candidate_id.clone(),
                },
            );
        }

        // Reversal class must be applicable iff a repair is available, so an
        // unavailable repair never claims a reversal path and an available one
        // always names how it can be undone.
        if pane.repair_availability.is_available() != pane.reversal_class.is_applicable() {
            violations.push(
                ProjectDoctorExplainabilityParityViolation::ReversalAvailabilityMismatch {
                    pane_id: pane.pane_id.clone(),
                    availability: pane.repair_availability.as_str(),
                    reversal_class: pane.reversal_class.as_str(),
                },
            );
        }
    }
}

/// A validation violation for the explainability/parity packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectDoctorExplainabilityParityViolation {
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
    /// A pane id appears more than once.
    DuplicatePaneId {
        /// Duplicate pane id.
        pane_id: String,
    },
    /// A finding code does not use the global Doctor finding prefix.
    FindingCodePrefix {
        /// Pane id.
        pane_id: String,
    },
    /// A pane cites no evidence ref.
    EvidenceMissing {
        /// Pane id.
        pane_id: String,
    },
    /// A pane declares no parity surface.
    ParitySurfacesEmpty {
        /// Pane id.
        pane_id: String,
    },
    /// A pane does not render stably across the core surfaces.
    PaneNotCrossSurfaceStable {
        /// Pane id.
        pane_id: String,
    },
    /// A pane omits a required locale-invariant machine-meaning key.
    MissingMachineMeaningKey {
        /// Pane id.
        pane_id: String,
        /// Missing key.
        key: &'static str,
    },
    /// A pane's redaction class is not metadata-safe.
    RedactionNotSafe {
        /// Pane id.
        pane_id: String,
    },
    /// A pane does not exclude raw private material.
    RawMaterialPresent {
        /// Pane id.
        pane_id: String,
    },
    /// A pane's exit class does not match the canonical state map.
    ExitClassNotCanonical {
        /// Pane id.
        pane_id: String,
        /// State token.
        state: &'static str,
        /// Exit-class token.
        exit_class: &'static str,
    },
    /// The healthy state and not-applicable availability disagree.
    HealthyAvailabilityMismatch {
        /// Pane id.
        pane_id: String,
        /// State token.
        state: &'static str,
        /// Availability token.
        availability: &'static str,
    },
    /// An available repair declares no repair-candidate id.
    AvailableRepairMissingCandidate {
        /// Pane id.
        pane_id: String,
    },
    /// An available repair carries an unavailable reason code.
    AvailableRepairHasReason {
        /// Pane id.
        pane_id: String,
    },
    /// An unavailable repair carries a repair-candidate id.
    UnavailableRepairHasCandidate {
        /// Pane id.
        pane_id: String,
    },
    /// A blocked repair hides behind a generic/empty reason code.
    GenericBlockedReason {
        /// Pane id.
        pane_id: String,
        /// Availability token.
        availability: &'static str,
    },
    /// A healthy (not-applicable) pane carries a repair-unavailable reason.
    HealthyRepairHasReason {
        /// Pane id.
        pane_id: String,
    },
    /// A repair-candidate id does not use the repair prefix.
    RepairCandidatePrefix {
        /// Pane id.
        pane_id: String,
        /// Offending candidate id.
        candidate: String,
    },
    /// The repair availability and reversal class disagree on applicability.
    ReversalAvailabilityMismatch {
        /// Pane id.
        pane_id: String,
        /// Availability token.
        availability: &'static str,
        /// Reversal-class token.
        reversal_class: &'static str,
    },
    /// The summary counts disagree with the panes.
    SummaryMismatch,
}

impl fmt::Display for ProjectDoctorExplainabilityParityViolation {
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
            Self::DuplicatePaneId { pane_id } => {
                write!(f, "duplicate pane id {pane_id}")
            }
            Self::FindingCodePrefix { pane_id } => {
                write!(
                    f,
                    "pane {pane_id} finding code does not start with {DOCTOR_FINDING_PREFIX}"
                )
            }
            Self::EvidenceMissing { pane_id } => {
                write!(f, "pane {pane_id} cites no evidence ref")
            }
            Self::ParitySurfacesEmpty { pane_id } => {
                write!(f, "pane {pane_id} declares no parity surface")
            }
            Self::PaneNotCrossSurfaceStable { pane_id } => {
                write!(
                    f,
                    "pane {pane_id} is not stable across desktop, CLI, headless, and support"
                )
            }
            Self::MissingMachineMeaningKey { pane_id, key } => {
                write!(f, "pane {pane_id} omits machine-meaning key {key}")
            }
            Self::RedactionNotSafe { pane_id } => {
                write!(
                    f,
                    "pane {pane_id} redaction_class must be metadata_safe_default"
                )
            }
            Self::RawMaterialPresent { pane_id } => {
                write!(
                    f,
                    "pane {pane_id} raw_private_material_excluded must be true"
                )
            }
            Self::ExitClassNotCanonical {
                pane_id,
                state,
                exit_class,
            } => {
                write!(
                    f,
                    "pane {pane_id} exit class {exit_class} is not canonical for state {state}"
                )
            }
            Self::HealthyAvailabilityMismatch {
                pane_id,
                state,
                availability,
            } => {
                write!(
                    f,
                    "pane {pane_id} state {state} and availability {availability} disagree on healthy"
                )
            }
            Self::AvailableRepairMissingCandidate { pane_id } => {
                write!(
                    f,
                    "pane {pane_id} declares an available repair but no candidate id"
                )
            }
            Self::AvailableRepairHasReason { pane_id } => {
                write!(
                    f,
                    "pane {pane_id} declares an available repair but carries a block reason"
                )
            }
            Self::UnavailableRepairHasCandidate { pane_id } => {
                write!(
                    f,
                    "pane {pane_id} has no available repair but carries a candidate id"
                )
            }
            Self::GenericBlockedReason {
                pane_id,
                availability,
            } => {
                write!(
                    f,
                    "pane {pane_id} availability {availability} hides behind a generic/empty reason code"
                )
            }
            Self::HealthyRepairHasReason { pane_id } => {
                write!(
                    f,
                    "pane {pane_id} is healthy/not-applicable but carries a block reason"
                )
            }
            Self::RepairCandidatePrefix { pane_id, candidate } => {
                write!(
                    f,
                    "pane {pane_id} repair candidate {candidate} does not use the {DOCTOR_REPAIR_CANDIDATE_PREFIX} prefix"
                )
            }
            Self::ReversalAvailabilityMismatch {
                pane_id,
                availability,
                reversal_class,
            } => {
                write!(
                    f,
                    "pane {pane_id} availability {availability} and reversal class {reversal_class} disagree on applicability"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the panes")
            }
        }
    }
}

impl Error for ProjectDoctorExplainabilityParityViolation {}

/// Loads the embedded explainability/parity packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`ProjectDoctorExplainabilityParity`].
pub fn current_project_doctor_explainability_parity(
) -> Result<ProjectDoctorExplainabilityParity, serde_json::Error> {
    serde_json::from_str(PROJECT_DOCTOR_EXPLAINABILITY_PARITY_JSON)
}

#[cfg(test)]
mod tests;
