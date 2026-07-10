//! Two reusable M5 governance-dashboard primitives implemented as one controls
//! packet: the **fitness dashboard tile** (protected-metric identity, fitness state,
//! threshold state, corpus/profile provenance, evidence freshness, owner, and linked
//! evidence) and the **governance report row** (report type, corpus/profile scope,
//! timestamp, pass/partial/fail outcome, provenance disclosure, and
//! compare-or-open-report continuity), projected the same way across every claimed
//! M5 assurance surface.
//!
//! Aureline's frozen governance-dashboard component matrix
//! ([`crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`])
//! names the fitness dashboard tile and the governance report row as two governed
//! component families and freezes their shared readiness-state vocabulary, the
//! fitness provenance classes, and the governance report scopes. This module
//! *implements* those two contracts as one reusable controls packet so a user can
//! tell — from the tile and the row alone — what protected metric is under review,
//! whether its evidence is fresh, which corpus or profile produced a governance
//! result, and whether that result may be trusted outside its support class, before
//! that truth drifts by dashboard page or shiproom spreadsheet.
//!
//! The packet has two resolver halves:
//!
//! 1. [`resolve_fitness_tile`] takes one protected metric's identity, declared
//!    reading, threshold state, corpus/profile provenance, evidence freshness,
//!    profile-match state, owner alias, and linked evidence, and produces one
//!    [`M5ResolvedFitnessTile`] carrying the *derived* readiness state drawn from the
//!    frozen [`M5GovernanceReadinessState`] vocabulary. A green metric whose evidence
//!    is stale or whose profile is wrong never resolves to `passing`: it degrades
//!    visibly to `evidence_stale` or `warning` with a self-describing degrade reason
//!    and a next action.
//! 2. [`resolve_governance_report`] takes one governance report's identity, report
//!    type, corpus/profile scope, provenance class, timestamp, declared outcome,
//!    evidence freshness, and support-class boundedness, and produces one
//!    [`M5ResolvedGovernanceReport`] carrying the derived readiness state, a
//!    [`M5ProvenanceDisclosure`] that names what kind of corpus or profile produced
//!    the result and whether it may be trusted outside its support class, and the
//!    compare/open-report actions. An undisclosed provenance, or a result read
//!    outside its support class, degrades visibly rather than reading like a clean
//!    pass.
//!
//! A parity matrix — [`M5FitnessGovernanceControlsPacket`] — binds one row per
//! claimed M5 assurance consumer (the assurance dashboard, the operator board, the
//! shiproom packet, the CLI inspect, and the support export) to the shared tile and
//! row anatomy, the same readiness states, provenance classes, report scopes,
//! threshold states, evidence-freshness states, degrade reasons, provenance
//! disclosures, and report actions, plus worked resolution cases that must reproduce
//! the resolver output exactly, so the fitness/governance vocabulary stays identical
//! across the assurance center, the operator board, the shiproom, the CLI, and
//! support/export.
//!
//! The frozen readiness-state vocabulary ([`M5GovernanceReadinessState`]), the
//! fitness provenance class ([`M5FitnessProvenanceClass`]), the governance report
//! scope ([`M5GovernanceReportScope`]), the deployment line ([`M5DeploymentLine`]),
//! the governance surface family ([`M5GovernanceSurfaceFamily`]), the governance
//! consumer surface ([`M5GovernanceConsumerSurface`]), the accessibility route
//! ([`M5GovernanceAccessibilityRoute`]), the required label
//! ([`M5GovernanceRequiredLabel`]), the qualification class
//! ([`M5GovernanceQualificationClass`]), and the downgrade trigger
//! ([`M5GovernanceDowngradeTrigger`]) are reused verbatim from the frozen matrix.
//! This module mints new vocabulary only for what that matrix left implicit about the
//! tile and the row themselves: their assurance consumer families, their anatomy
//! parts, the declared fitness state, the threshold state, the shared
//! evidence-freshness input, the profile-match state, the tile degrade reasons, the
//! report types, the report outcomes, the provenance disclosures, the report degrade
//! reasons, the report actions, the next actions, and the export fields. No M5
//! assurance surface invents a second fitness or governance grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user text bodies stay
//! outside the support boundary; every fitness id, report id, owner alias, timestamp,
//! and evidence ref is carried only as an opaque, export-safe representation, and an
//! owner alias is a role alias, never a personal contact detail.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_fitness_governance_controls_assurance_dashboard_beta_narrowed,
    seeded_m5_fitness_governance_controls_packet,
    seeded_m5_fitness_governance_controls_shiproom_packet_preview_narrowed,
    M5_FITNESS_GOVERNANCE_CONTROLS_PACKET_ID,
};

// The readiness state vocabulary, the fitness provenance classes, the governance
// report scopes, the deployment lines, the surface families, the consumer surfaces,
// the accessibility routes, the required labels, the qualification classes, and the
// downgrade triggers are frozen once, in the governance-dashboard component matrix.
// This controls packet reuses them verbatim so it never invents a parallel vocabulary.
pub use crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix::{
    M5DeploymentLine, M5FitnessProvenanceClass, M5GovernanceAccessibilityRoute,
    M5GovernanceConsumerSurface, M5GovernanceDowngradeTrigger, M5GovernanceQualificationClass,
    M5GovernanceReadinessState, M5GovernanceReportScope, M5GovernanceRequiredLabel,
    M5GovernanceSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5FitnessGovernanceControlsPacket`].
pub const M5_FITNESS_GOVERNANCE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_fitness_dashboard_tiles_and_governance_report_rows_across_claimed_m5_assurance_surfaces";

/// Schema version for M5 fitness / governance-report controls records.
pub const M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-fitness-governance-report-controls.schema.json";

/// Repo-relative path of the controls contract doc.
pub const M5_FITNESS_GOVERNANCE_CONTROLS_DOC_REF: &str =
    "docs/help/m5_fitness_dashboard_tile_and_governance_report_row_controls.md";

/// Repo-relative path of the frozen governance-dashboard component matrix schema this
/// controls packet narrows from.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-matrix.schema.json";

/// Repo-relative path of the frozen governance-dashboard component matrix doc.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_components_contract.md";

/// Repo-relative path of the per-component fitness-dashboard-tile contract schema.
pub const M5_FITNESS_DASHBOARD_TILE_CONTRACT_REF: &str =
    "schemas/ui/m5-fitness-dashboard-tile.schema.json";

/// Repo-relative path of the per-component governance-report-row contract schema.
pub const M5_GOVERNANCE_REPORT_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-governance-report-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_FITNESS_GOVERNANCE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-fitness-governance-report-controls";

/// Repo-relative path of the checked support-export artifact.
pub const M5_FITNESS_GOVERNANCE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-fitness-governance-report-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_FITNESS_GOVERNANCE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-fitness-governance-report-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_FITNESS_GOVERNANCE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-fitness-governance-report-controls-proof/summary.md";

// ---------------------------------------------------------------------------
// Minted vocabulary
// ---------------------------------------------------------------------------

/// One claimed M5 assurance consumer that renders the shared fitness tile and
/// governance report row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FitnessGovernanceConsumerSurface {
    /// The assurance-center dashboard.
    AssuranceDashboard,
    /// The operator overview board.
    OperatorBoard,
    /// The shiproom packet.
    ShiproomPacket,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The support / export packet.
    SupportExport,
}

impl M5FitnessGovernanceConsumerSurface {
    /// Every claimed assurance consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AssuranceDashboard,
        Self::OperatorBoard,
        Self::ShiproomPacket,
        Self::CliInspect,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceDashboard => "assurance_dashboard",
            Self::OperatorBoard => "operator_board",
            Self::ShiproomPacket => "shiproom_packet",
            Self::CliInspect => "cli_inspect",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceDashboard => "Assurance Dashboard",
            Self::OperatorBoard => "Operator Board",
            Self::ShiproomPacket => "Shiproom Packet",
            Self::CliInspect => "CLI Inspect",
            Self::SupportExport => "Support / Export",
        }
    }
}

/// One anatomy part the shared fitness tile / governance report row surfaces. The
/// parts in [`M5FitnessGovernanceAnatomyPart::MANDATORY`] are required on every row so
/// a user can orient before trusting a reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FitnessGovernanceAnatomyPart {
    /// The protected fitness id and family (tile identity).
    FitnessIdentity,
    /// The fitness state cue.
    FitnessState,
    /// The threshold-state cue.
    ThresholdState,
    /// The corpus/profile provenance badge.
    ProvenanceBadge,
    /// The evidence-freshness cue.
    EvidenceFreshnessCue,
    /// The owner cue.
    OwnerCue,
    /// The linked-evidence list.
    LinkedEvidence,
    /// The report type (row identity).
    ReportType,
    /// The report corpus/profile scope.
    ReportScope,
    /// The report timestamp.
    ReportTimestamp,
    /// The report outcome cue.
    ReportOutcome,
    /// The compare / open-report actions.
    CompareOrOpenReportActions,
}

impl M5FitnessGovernanceAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::FitnessIdentity,
        Self::FitnessState,
        Self::ThresholdState,
        Self::ProvenanceBadge,
        Self::EvidenceFreshnessCue,
        Self::OwnerCue,
        Self::LinkedEvidence,
        Self::ReportType,
        Self::ReportScope,
        Self::ReportTimestamp,
        Self::ReportOutcome,
        Self::CompareOrOpenReportActions,
    ];

    /// The anatomy parts every row must render before a reading is trusted.
    pub const MANDATORY: [Self; 4] = [
        Self::FitnessIdentity,
        Self::FitnessState,
        Self::ReportType,
        Self::ReportOutcome,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FitnessIdentity => "fitness_identity",
            Self::FitnessState => "fitness_state",
            Self::ThresholdState => "threshold_state",
            Self::ProvenanceBadge => "provenance_badge",
            Self::EvidenceFreshnessCue => "evidence_freshness_cue",
            Self::OwnerCue => "owner_cue",
            Self::LinkedEvidence => "linked_evidence",
            Self::ReportType => "report_type",
            Self::ReportScope => "report_scope",
            Self::ReportTimestamp => "report_timestamp",
            Self::ReportOutcome => "report_outcome",
            Self::CompareOrOpenReportActions => "compare_or_open_report_actions",
        }
    }
}

/// The declared reading a fitness dashboard tile carries before the resolver derives
/// its readiness — the raw metric verdict, never shown as the final state on its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FitnessDeclaredState {
    /// The metric is declared passing.
    MetricPass,
    /// The metric is declared at warning.
    MetricWarn,
    /// The metric is declared failing.
    MetricFail,
    /// The metric's failure is held under a disclosed waiver.
    MetricWaived,
    /// The metric has not been run on this build.
    MetricNotRun,
}

impl M5FitnessDeclaredState {
    /// Every declared fitness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MetricPass,
        Self::MetricWarn,
        Self::MetricFail,
        Self::MetricWaived,
        Self::MetricNotRun,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetricPass => "metric_pass",
            Self::MetricWarn => "metric_warn",
            Self::MetricFail => "metric_fail",
            Self::MetricWaived => "metric_waived",
            Self::MetricNotRun => "metric_not_run",
        }
    }
}

/// The threshold state of a protected metric relative to its budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ThresholdState {
    /// The reading is within threshold.
    WithinThreshold,
    /// The reading is at the threshold boundary.
    AtThreshold,
    /// The reading has breached the threshold.
    BreachedThreshold,
    /// No threshold is defined for this metric.
    NoThresholdDefined,
    /// The threshold reading is unknown / not yet evaluated.
    ThresholdUnknown,
}

impl M5ThresholdState {
    /// Every threshold state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::WithinThreshold,
        Self::AtThreshold,
        Self::BreachedThreshold,
        Self::NoThresholdDefined,
        Self::ThresholdUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinThreshold => "within_threshold",
            Self::AtThreshold => "at_threshold",
            Self::BreachedThreshold => "breached_threshold",
            Self::NoThresholdDefined => "no_threshold_defined",
            Self::ThresholdUnknown => "threshold_unknown",
        }
    }
}

/// The evidence-freshness reading shared by both resolvers, so a tile or a row never
/// shows stale or missing evidence as clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshness {
    /// Evidence is fresh within its freshness window.
    EvidenceFresh,
    /// Evidence is aging but still within tolerance.
    EvidenceAging,
    /// Evidence is stale relative to the current build.
    EvidenceStale,
    /// Required evidence is missing.
    EvidenceMissing,
    /// The evidence-freshness reading is unknown / not yet evaluated.
    EvidenceUnknown,
}

impl M5EvidenceFreshness {
    /// Every evidence-freshness reading, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EvidenceFresh,
        Self::EvidenceAging,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::EvidenceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceFresh => "evidence_fresh",
            Self::EvidenceAging => "evidence_aging",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::EvidenceUnknown => "evidence_unknown",
        }
    }
}

/// Whether a fitness reading's evidence came from the profile it claims, so a tile
/// never presents a wrong-profile reading as equivalent to a matched one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProfileMatchState {
    /// The evidence came from the claimed profile.
    ProfileMatched,
    /// The evidence came from a different profile than the one claimed.
    WrongProfile,
    /// No profile was pinned for this reading.
    ProfileUnpinned,
    /// The profile-match reading is unknown / not yet evaluated.
    ProfileMatchUnknown,
}

impl M5ProfileMatchState {
    /// Every profile-match state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProfileMatched,
        Self::WrongProfile,
        Self::ProfileUnpinned,
        Self::ProfileMatchUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfileMatched => "profile_matched",
            Self::WrongProfile => "wrong_profile",
            Self::ProfileUnpinned => "profile_unpinned",
            Self::ProfileMatchUnknown => "profile_match_unknown",
        }
    }
}

/// The next action named on a degraded tile or row, so a non-passing reading is
/// actionable rather than a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceNextAction {
    /// Run the evaluation that has not been run.
    RunEvaluation,
    /// Refresh the stale or aging evidence.
    RefreshEvidence,
    /// Provide the missing evidence.
    ProvideEvidence,
    /// Resolve the breach or failure.
    ResolveBreachOrFailure,
    /// Disclose or pin the corpus/profile provenance.
    DiscloseOrPinProvenance,
    /// Resolve the unresolved owner.
    ResolveOwner,
}

impl M5GovernanceNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunEvaluation,
        Self::RefreshEvidence,
        Self::ProvideEvidence,
        Self::ResolveBreachOrFailure,
        Self::DiscloseOrPinProvenance,
        Self::ResolveOwner,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunEvaluation => "run_evaluation",
            Self::RefreshEvidence => "refresh_evidence",
            Self::ProvideEvidence => "provide_evidence",
            Self::ResolveBreachOrFailure => "resolve_breach_or_failure",
            Self::DiscloseOrPinProvenance => "disclose_or_pin_provenance",
            Self::ResolveOwner => "resolve_owner",
        }
    }
}

/// The exact reason a fitness tile degraded below a clean pass, so a green metric with
/// stale or wrong-profile evidence never reads like a fresh pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FitnessDegradeReason {
    /// The reading has not been evaluated on this build.
    NotYetEvaluated,
    /// The tile has no resolved owner.
    OwnerUnresolvedForTile,
    /// Required fitness evidence is missing.
    EvidenceMissingReading,
    /// The fitness evidence is stale relative to this build.
    EvidenceStaleReading,
    /// The metric failed or breached its threshold.
    MetricBreachedThreshold,
    /// The failure is held under a disclosed waiver.
    WaivedUnderDisclosure,
    /// The fitness evidence came from a wrong or unpinned profile.
    WrongOrUnpinnedProfile,
    /// The metric is at warning, or its evidence is aging.
    MetricAtWarning,
}

impl M5FitnessDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotYetEvaluated,
        Self::OwnerUnresolvedForTile,
        Self::EvidenceMissingReading,
        Self::EvidenceStaleReading,
        Self::MetricBreachedThreshold,
        Self::WaivedUnderDisclosure,
        Self::WrongOrUnpinnedProfile,
        Self::MetricAtWarning,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "not_yet_evaluated",
            Self::OwnerUnresolvedForTile => "owner_unresolved_for_tile",
            Self::EvidenceMissingReading => "evidence_missing_reading",
            Self::EvidenceStaleReading => "evidence_stale_reading",
            Self::MetricBreachedThreshold => "metric_breached_threshold",
            Self::WaivedUnderDisclosure => "waived_under_disclosure",
            Self::WrongOrUnpinnedProfile => "wrong_or_unpinned_profile",
            Self::MetricAtWarning => "metric_at_warning",
        }
    }

    /// The frozen readiness state this degrade reason resolves to.
    pub const fn readiness_state(self) -> M5GovernanceReadinessState {
        match self {
            Self::NotYetEvaluated => M5GovernanceReadinessState::NotEvaluated,
            Self::OwnerUnresolvedForTile => M5GovernanceReadinessState::OwnerUnresolved,
            Self::EvidenceMissingReading => M5GovernanceReadinessState::Blocked,
            Self::EvidenceStaleReading => M5GovernanceReadinessState::EvidenceStale,
            Self::MetricBreachedThreshold => M5GovernanceReadinessState::Blocked,
            Self::WaivedUnderDisclosure => M5GovernanceReadinessState::Waived,
            Self::WrongOrUnpinnedProfile => M5GovernanceReadinessState::Warning,
            Self::MetricAtWarning => M5GovernanceReadinessState::Warning,
        }
    }

    /// The next action a reviewer should take to clear this degrade.
    pub const fn next_action(self) -> M5GovernanceNextAction {
        match self {
            Self::NotYetEvaluated => M5GovernanceNextAction::RunEvaluation,
            Self::OwnerUnresolvedForTile => M5GovernanceNextAction::ResolveOwner,
            Self::EvidenceMissingReading => M5GovernanceNextAction::ProvideEvidence,
            Self::EvidenceStaleReading => M5GovernanceNextAction::RefreshEvidence,
            Self::MetricBreachedThreshold => M5GovernanceNextAction::ResolveBreachOrFailure,
            Self::WaivedUnderDisclosure => M5GovernanceNextAction::ResolveBreachOrFailure,
            Self::WrongOrUnpinnedProfile => M5GovernanceNextAction::DiscloseOrPinProvenance,
            Self::MetricAtWarning => M5GovernanceNextAction::RefreshEvidence,
        }
    }

    /// Review-safe reason phrase for the tile's degrade note.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "the fitness reading has not been evaluated on this build",
            Self::OwnerUnresolvedForTile => "the tile has no resolved owner",
            Self::EvidenceMissingReading => "required fitness evidence is missing",
            Self::EvidenceStaleReading => "the fitness evidence is stale relative to this build",
            Self::MetricBreachedThreshold => "the metric failed or breached its threshold",
            Self::WaivedUnderDisclosure => "the failure is held under a disclosed waiver",
            Self::WrongOrUnpinnedProfile => {
                "the fitness evidence came from a wrong or unpinned profile"
            }
            Self::MetricAtWarning => "the metric is at warning or its evidence is aging",
        }
    }
}

/// The kind of governance report a row carries, so a report row never leaves its
/// report type implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceReportType {
    /// A fitness-function rollup report.
    FitnessRollupReport,
    /// A waiver-ledger report.
    WaiverLedgerReport,
    /// An ownership-coverage report.
    OwnershipCoverageReport,
    /// A release-readiness report.
    ReleaseReadinessReport,
    /// A milestone-exit report.
    MilestoneExitReport,
}

impl M5GovernanceReportType {
    /// Every report type, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FitnessRollupReport,
        Self::WaiverLedgerReport,
        Self::OwnershipCoverageReport,
        Self::ReleaseReadinessReport,
        Self::MilestoneExitReport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FitnessRollupReport => "fitness_rollup_report",
            Self::WaiverLedgerReport => "waiver_ledger_report",
            Self::OwnershipCoverageReport => "ownership_coverage_report",
            Self::ReleaseReadinessReport => "release_readiness_report",
            Self::MilestoneExitReport => "milestone_exit_report",
        }
    }
}

/// The declared outcome a governance report row carries before the resolver derives
/// its readiness — the raw pass/partial/fail verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReportOutcome {
    /// The report passed.
    ReportPass,
    /// The report partially passed.
    ReportPartial,
    /// The report failed.
    ReportFail,
    /// The report has not been run on this build.
    ReportNotRun,
}

impl M5ReportOutcome {
    /// Every report outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReportPass,
        Self::ReportPartial,
        Self::ReportFail,
        Self::ReportNotRun,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReportPass => "report_pass",
            Self::ReportPartial => "report_partial",
            Self::ReportFail => "report_fail",
            Self::ReportNotRun => "report_not_run",
        }
    }
}

/// What kind of corpus/profile produced a governance result and whether it may be
/// trusted outside its support class — the acceptance-criteria disclosure a report
/// row must carry before a user trusts a result out of its support class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProvenanceDisclosure {
    /// A canonical, pinned corpus consumed within its support class.
    CanonicalWithinSupportClass,
    /// A sampled corpus; the sampling caveat is disclosed.
    SampledDiscloseCaveat,
    /// A synthetic corpus; the synthetic caveat is disclosed.
    SyntheticDiscloseCaveat,
    /// A pinned profile; its scope is disclosed.
    ProfilePinnedDiscloseScope,
    /// The provenance is undisclosed and the result must not be trusted.
    ProvenanceUndisclosed,
}

impl M5ProvenanceDisclosure {
    /// Every provenance disclosure, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CanonicalWithinSupportClass,
        Self::SampledDiscloseCaveat,
        Self::SyntheticDiscloseCaveat,
        Self::ProfilePinnedDiscloseScope,
        Self::ProvenanceUndisclosed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalWithinSupportClass => "canonical_within_support_class",
            Self::SampledDiscloseCaveat => "sampled_disclose_caveat",
            Self::SyntheticDiscloseCaveat => "synthetic_disclose_caveat",
            Self::ProfilePinnedDiscloseScope => "profile_pinned_disclose_scope",
            Self::ProvenanceUndisclosed => "provenance_undisclosed",
        }
    }

    /// `true` only for [`Self::CanonicalWithinSupportClass`]: the sole disclosure that
    /// may be trusted outside its support class without a stated caveat.
    pub const fn is_trustable_outside_support_class(self) -> bool {
        matches!(self, Self::CanonicalWithinSupportClass)
    }
}

/// The exact reason a governance report row degraded below a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReportDegradeReason {
    /// The report has not been run on this build.
    NotYetEvaluated,
    /// Required report evidence is missing.
    EvidenceMissingReading,
    /// The report evidence is stale relative to this build.
    EvidenceStaleReading,
    /// The report outcome failed.
    OutcomeFailed,
    /// The report's corpus/profile provenance is undisclosed.
    ProvenanceUndisclosedReason,
    /// The report was read outside its support class.
    ProvenanceOutOfSupportClass,
    /// The report partially passed, or its evidence is aging.
    OutcomePartial,
}

impl M5ReportDegradeReason {
    /// Every report degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotYetEvaluated,
        Self::EvidenceMissingReading,
        Self::EvidenceStaleReading,
        Self::OutcomeFailed,
        Self::ProvenanceUndisclosedReason,
        Self::ProvenanceOutOfSupportClass,
        Self::OutcomePartial,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "not_yet_evaluated",
            Self::EvidenceMissingReading => "evidence_missing_reading",
            Self::EvidenceStaleReading => "evidence_stale_reading",
            Self::OutcomeFailed => "outcome_failed",
            Self::ProvenanceUndisclosedReason => "provenance_undisclosed_reason",
            Self::ProvenanceOutOfSupportClass => "provenance_out_of_support_class",
            Self::OutcomePartial => "outcome_partial",
        }
    }

    /// The frozen readiness state this degrade reason resolves to.
    pub const fn readiness_state(self) -> M5GovernanceReadinessState {
        match self {
            Self::NotYetEvaluated => M5GovernanceReadinessState::NotEvaluated,
            Self::EvidenceMissingReading => M5GovernanceReadinessState::Blocked,
            Self::EvidenceStaleReading => M5GovernanceReadinessState::EvidenceStale,
            Self::OutcomeFailed => M5GovernanceReadinessState::Blocked,
            Self::ProvenanceUndisclosedReason => M5GovernanceReadinessState::Warning,
            Self::ProvenanceOutOfSupportClass => M5GovernanceReadinessState::Warning,
            Self::OutcomePartial => M5GovernanceReadinessState::Warning,
        }
    }

    /// The next action a reviewer should take to clear this degrade.
    pub const fn next_action(self) -> M5GovernanceNextAction {
        match self {
            Self::NotYetEvaluated => M5GovernanceNextAction::RunEvaluation,
            Self::EvidenceMissingReading => M5GovernanceNextAction::ProvideEvidence,
            Self::EvidenceStaleReading => M5GovernanceNextAction::RefreshEvidence,
            Self::OutcomeFailed => M5GovernanceNextAction::ResolveBreachOrFailure,
            Self::ProvenanceUndisclosedReason => M5GovernanceNextAction::DiscloseOrPinProvenance,
            Self::ProvenanceOutOfSupportClass => M5GovernanceNextAction::DiscloseOrPinProvenance,
            Self::OutcomePartial => M5GovernanceNextAction::RefreshEvidence,
        }
    }

    /// Review-safe reason phrase for the report row's degrade note.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "the report has not been run on this build",
            Self::EvidenceMissingReading => "required report evidence is missing",
            Self::EvidenceStaleReading => "the report evidence is stale relative to this build",
            Self::OutcomeFailed => "the report outcome failed",
            Self::ProvenanceUndisclosedReason => {
                "the report's corpus/profile provenance is undisclosed"
            }
            Self::ProvenanceOutOfSupportClass => "the report was read outside its support class",
            Self::OutcomePartial => "the report partially passed or its evidence is aging",
        }
    }
}

/// An action a governance report row offers. The actions in
/// [`M5ReportAction::MANDATORY`] are required on every row so a reviewer can always
/// compare or open the underlying report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReportAction {
    /// Compare this report against a prior run.
    CompareReport,
    /// Open the underlying report.
    OpenReport,
    /// Export the report packet.
    ExportReport,
    /// Inspect the corpus/profile provenance.
    InspectProvenance,
}

impl M5ReportAction {
    /// Every report action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CompareReport,
        Self::OpenReport,
        Self::ExportReport,
        Self::InspectProvenance,
    ];

    /// The report actions every row must offer.
    pub const MANDATORY: [Self; 2] = [Self::CompareReport, Self::OpenReport];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompareReport => "compare_report",
            Self::OpenReport => "open_report",
            Self::ExportReport => "export_report",
            Self::InspectProvenance => "inspect_provenance",
        }
    }
}

/// A field the support / export packet carries so tile and row truth is
/// reconstructable from the shared model. The fields in
/// [`M5FitnessGovernanceExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FitnessGovernanceExportField {
    /// The opaque fitness id.
    FitnessId,
    /// The fitness family.
    FitnessFamily,
    /// The threshold state.
    ThresholdState,
    /// The corpus/profile provenance class.
    ProvenanceClass,
    /// The evidence-freshness reading.
    EvidenceFreshness,
    /// The owner alias.
    OwnerAlias,
    /// The derived readiness state.
    ReadinessState,
    /// The report type.
    ReportType,
    /// The report scope.
    ReportScope,
    /// The report outcome.
    ReportOutcome,
    /// The provenance disclosure.
    ProvenanceDisclosure,
}

impl M5FitnessGovernanceExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::FitnessId,
        Self::FitnessFamily,
        Self::ThresholdState,
        Self::ProvenanceClass,
        Self::EvidenceFreshness,
        Self::OwnerAlias,
        Self::ReadinessState,
        Self::ReportType,
        Self::ReportScope,
        Self::ReportOutcome,
        Self::ProvenanceDisclosure,
    ];

    /// The export fields every controls export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::FitnessId,
        Self::ProvenanceClass,
        Self::EvidenceFreshness,
        Self::ReadinessState,
        Self::ProvenanceDisclosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FitnessId => "fitness_id",
            Self::FitnessFamily => "fitness_family",
            Self::ThresholdState => "threshold_state",
            Self::ProvenanceClass => "provenance_class",
            Self::EvidenceFreshness => "evidence_freshness",
            Self::OwnerAlias => "owner_alias",
            Self::ReadinessState => "readiness_state",
            Self::ReportType => "report_type",
            Self::ReportScope => "report_scope",
            Self::ReportOutcome => "report_outcome",
            Self::ProvenanceDisclosure => "provenance_disclosure",
        }
    }
}

// ---------------------------------------------------------------------------
// Fitness-tile resolver
// ---------------------------------------------------------------------------

/// The full input to the fitness-tile resolver for one protected metric.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FitnessTileResolutionInput {
    /// The opaque, export-safe protected-metric id.
    pub fitness_id_repr: String,
    /// The opaque, export-safe fitness family.
    pub fitness_family_repr: String,
    /// The declared fitness reading (never shown as the final state alone).
    pub declared_state: M5FitnessDeclaredState,
    /// The threshold state relative to budget.
    pub threshold_state: M5ThresholdState,
    /// The corpus/profile provenance class.
    pub provenance_class: M5FitnessProvenanceClass,
    /// The evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// Whether the evidence came from the claimed profile.
    pub profile_match: M5ProfileMatchState,
    /// The opaque owner role alias (never a personal contact detail).
    pub owner_alias: String,
    /// The opaque linked-evidence refs, if any.
    pub linked_evidence_refs: Vec<String>,
}

/// The resolved fitness-tile truth for one protected metric.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedFitnessTile {
    /// The opaque protected-metric id.
    pub fitness_id_repr: String,
    /// The opaque fitness family.
    pub fitness_family_repr: String,
    /// The declared fitness reading.
    pub declared_state: M5FitnessDeclaredState,
    /// The threshold state.
    pub threshold_state: M5ThresholdState,
    /// The corpus/profile provenance class.
    pub provenance_class: M5FitnessProvenanceClass,
    /// The evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// The profile-match state.
    pub profile_match: M5ProfileMatchState,
    /// The opaque owner alias.
    pub owner_alias: String,
    /// `true` when the tile has a resolved owner.
    pub owner_resolved: bool,
    /// The opaque linked-evidence refs.
    pub linked_evidence_refs: Vec<String>,
    /// `true` when the tile carries at least one linked-evidence ref.
    pub has_linked_evidence: bool,
    /// The derived readiness state drawn from the frozen vocabulary.
    pub readiness_state: M5GovernanceReadinessState,
    /// `true` only when the derived readiness is a clean pass.
    pub is_clean_pass: bool,
    /// The degrade reason, present when the tile is not a clean pass.
    pub degrade_reason: Option<M5FitnessDegradeReason>,
    /// The next action, present when the tile is degraded.
    pub next_action: Option<M5GovernanceNextAction>,
    /// A self-contained degrade note naming the reason and next action, present when
    /// the tile is degraded.
    pub degrade_note: Option<String>,
}

/// Errors returned by [`resolve_fitness_tile`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5FitnessTileResolutionError {
    /// The fitness id was empty.
    EmptyFitnessId,
    /// The fitness family was empty.
    EmptyFitnessFamily,
    /// The owner alias carried a personal contact detail (an `@`), not a role alias.
    PersonContactDetailInAlias,
    /// A fitness id, family, owner alias, or evidence ref carried forbidden material.
    ForbiddenTileMaterial,
}

impl M5FitnessTileResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyFitnessId => "empty_fitness_id",
            Self::EmptyFitnessFamily => "empty_fitness_family",
            Self::PersonContactDetailInAlias => "person_contact_detail_in_alias",
            Self::ForbiddenTileMaterial => "forbidden_tile_material",
        }
    }
}

impl fmt::Display for M5FitnessTileResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "fitness-tile resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5FitnessTileResolutionError {}

/// Resolves one fitness dashboard tile from its declared reading.
///
/// The derived readiness state is computed in a fixed degrade-first order: a not-run
/// metric or any unknown reading is `not_evaluated`, an unresolved owner is
/// `owner_unresolved`, missing evidence blocks, stale evidence is `evidence_stale`, a
/// failed or breached metric blocks, a waived failure is `waived`, a wrong or unpinned
/// profile (or unknown provenance) degrades to `warning`, a metric at warning or aging
/// evidence degrades to `warning`, and only a passing metric within threshold, with
/// fresh evidence from the claimed profile and a resolved owner, is a clean pass. A
/// green metric with stale or wrong-profile evidence therefore never reads as passing.
pub fn resolve_fitness_tile(
    input: &M5FitnessTileResolutionInput,
) -> Result<M5ResolvedFitnessTile, M5FitnessTileResolutionError> {
    if input.fitness_id_repr.trim().is_empty() {
        return Err(M5FitnessTileResolutionError::EmptyFitnessId);
    }
    if input.fitness_family_repr.trim().is_empty() {
        return Err(M5FitnessTileResolutionError::EmptyFitnessFamily);
    }
    if input.owner_alias.contains('@') {
        return Err(M5FitnessTileResolutionError::PersonContactDetailInAlias);
    }
    if value_repr_is_forbidden(&input.fitness_id_repr)
        || value_repr_is_forbidden(&input.fitness_family_repr)
        || value_repr_is_forbidden(&input.owner_alias)
    {
        return Err(M5FitnessTileResolutionError::ForbiddenTileMaterial);
    }
    for evidence in &input.linked_evidence_refs {
        if value_repr_is_forbidden(evidence) {
            return Err(M5FitnessTileResolutionError::ForbiddenTileMaterial);
        }
    }

    let owner_resolved = !input.owner_alias.trim().is_empty();
    let degrade_reason = derive_fitness_degrade(
        input.declared_state,
        input.threshold_state,
        input.evidence_freshness,
        input.profile_match,
        input.provenance_class,
        owner_resolved,
    );
    let readiness_state = match degrade_reason {
        Some(reason) => reason.readiness_state(),
        None => M5GovernanceReadinessState::Passing,
    };
    let next_action = degrade_reason.map(M5FitnessDegradeReason::next_action);
    let degrade_note = degrade_reason.map(|reason| {
        format!(
            "Fitness tile degraded: {} — state `{}`; next: {}",
            reason.phrase(),
            readiness_state.as_str(),
            reason.next_action().as_str()
        )
    });

    Ok(M5ResolvedFitnessTile {
        fitness_id_repr: input.fitness_id_repr.clone(),
        fitness_family_repr: input.fitness_family_repr.clone(),
        declared_state: input.declared_state,
        threshold_state: input.threshold_state,
        provenance_class: input.provenance_class,
        evidence_freshness: input.evidence_freshness,
        profile_match: input.profile_match,
        owner_alias: input.owner_alias.clone(),
        owner_resolved,
        linked_evidence_refs: input.linked_evidence_refs.clone(),
        has_linked_evidence: !input.linked_evidence_refs.is_empty(),
        readiness_state,
        is_clean_pass: readiness_state.is_clean_pass(),
        degrade_reason,
        next_action,
        degrade_note,
    })
}

/// The fixed degrade-first fitness ladder. Returns `None` for a clean pass.
fn derive_fitness_degrade(
    declared: M5FitnessDeclaredState,
    threshold: M5ThresholdState,
    evidence: M5EvidenceFreshness,
    profile: M5ProfileMatchState,
    provenance: M5FitnessProvenanceClass,
    owner_resolved: bool,
) -> Option<M5FitnessDegradeReason> {
    let unknown = matches!(declared, M5FitnessDeclaredState::MetricNotRun)
        || matches!(evidence, M5EvidenceFreshness::EvidenceUnknown)
        || matches!(threshold, M5ThresholdState::ThresholdUnknown)
        || matches!(profile, M5ProfileMatchState::ProfileMatchUnknown);
    if unknown {
        Some(M5FitnessDegradeReason::NotYetEvaluated)
    } else if !owner_resolved {
        Some(M5FitnessDegradeReason::OwnerUnresolvedForTile)
    } else if matches!(evidence, M5EvidenceFreshness::EvidenceMissing) {
        Some(M5FitnessDegradeReason::EvidenceMissingReading)
    } else if matches!(evidence, M5EvidenceFreshness::EvidenceStale) {
        Some(M5FitnessDegradeReason::EvidenceStaleReading)
    } else if matches!(declared, M5FitnessDeclaredState::MetricFail)
        || matches!(threshold, M5ThresholdState::BreachedThreshold)
    {
        Some(M5FitnessDegradeReason::MetricBreachedThreshold)
    } else if matches!(declared, M5FitnessDeclaredState::MetricWaived) {
        Some(M5FitnessDegradeReason::WaivedUnderDisclosure)
    } else if matches!(
        profile,
        M5ProfileMatchState::WrongProfile | M5ProfileMatchState::ProfileUnpinned
    ) || matches!(provenance, M5FitnessProvenanceClass::ProvenanceUnknown)
    {
        Some(M5FitnessDegradeReason::WrongOrUnpinnedProfile)
    } else if matches!(declared, M5FitnessDeclaredState::MetricWarn)
        || matches!(threshold, M5ThresholdState::AtThreshold)
        || matches!(evidence, M5EvidenceFreshness::EvidenceAging)
    {
        Some(M5FitnessDegradeReason::MetricAtWarning)
    } else {
        None
    }
}

/// One worked fitness-tile resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FitnessTileCase {
    /// The resolver input.
    pub input: M5FitnessTileResolutionInput,
    /// The resolved truth. Must equal `resolve_fitness_tile(&input)`.
    pub resolved: M5ResolvedFitnessTile,
}

impl M5FitnessTileCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5FitnessTileResolutionInput) -> Self {
        let resolved = resolve_fitness_tile(&input).expect("seed fitness-tile case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_fitness_tile(&self.input).as_ref() == Ok(&self.resolved)
    }
}

// ---------------------------------------------------------------------------
// Governance-report resolver
// ---------------------------------------------------------------------------

/// The full input to the governance-report resolver for one report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceReportResolutionInput {
    /// The opaque, export-safe report id.
    pub report_id_repr: String,
    /// The report type.
    pub report_type: M5GovernanceReportType,
    /// The report corpus/profile scope.
    pub report_scope: M5GovernanceReportScope,
    /// The corpus/profile provenance class.
    pub provenance_class: M5FitnessProvenanceClass,
    /// The opaque, export-safe report timestamp.
    pub timestamp_repr: String,
    /// The declared report outcome.
    pub declared_outcome: M5ReportOutcome,
    /// The evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// `true` when the result is being read within its support class.
    pub support_class_bounded: bool,
}

/// The resolved governance-report truth for one report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedGovernanceReport {
    /// The opaque report id.
    pub report_id_repr: String,
    /// The report type.
    pub report_type: M5GovernanceReportType,
    /// The report scope.
    pub report_scope: M5GovernanceReportScope,
    /// The corpus/profile provenance class.
    pub provenance_class: M5FitnessProvenanceClass,
    /// The opaque report timestamp.
    pub timestamp_repr: String,
    /// The declared report outcome.
    pub declared_outcome: M5ReportOutcome,
    /// The evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// Whether the result is being read within its support class.
    pub support_class_bounded: bool,
    /// The derived provenance disclosure.
    pub provenance_disclosure: M5ProvenanceDisclosure,
    /// `true` when the provenance may be trusted outside its support class.
    pub provenance_trustable_outside_support_class: bool,
    /// The derived readiness state drawn from the frozen vocabulary.
    pub readiness_state: M5GovernanceReadinessState,
    /// `true` only when the derived readiness is a clean pass.
    pub is_clean_pass: bool,
    /// The degrade reason, present when the report is not a clean pass.
    pub degrade_reason: Option<M5ReportDegradeReason>,
    /// The next action, present when the report is degraded.
    pub next_action: Option<M5GovernanceNextAction>,
    /// The report actions this row always offers (always includes compare + open).
    pub report_actions: Vec<M5ReportAction>,
    /// A self-contained provenance note naming the disclosure, present always.
    pub provenance_note: String,
    /// A self-contained degrade note, present when the report is degraded.
    pub degrade_note: Option<String>,
}

/// Errors returned by [`resolve_governance_report`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5GovernanceReportResolutionError {
    /// The report id was empty.
    EmptyReportId,
    /// The report timestamp was empty.
    EmptyTimestamp,
    /// A report id or timestamp carried forbidden material.
    ForbiddenReportMaterial,
}

impl M5GovernanceReportResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyReportId => "empty_report_id",
            Self::EmptyTimestamp => "empty_timestamp",
            Self::ForbiddenReportMaterial => "forbidden_report_material",
        }
    }
}

impl fmt::Display for M5GovernanceReportResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "governance-report resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5GovernanceReportResolutionError {}

/// Resolves the provenance disclosure for one governance result.
///
/// An unknown provenance is always undisclosed. A canonical corpus consumed within
/// its support class is the only disclosure that may be trusted outside its support
/// class; a canonical corpus read out of its support class, a pinned profile, a
/// sampled corpus, or a synthetic corpus each carry a stated caveat so a user can tell
/// what kind of corpus/profile produced the result before trusting it further.
pub fn resolve_provenance_disclosure(
    provenance_class: M5FitnessProvenanceClass,
    support_class_bounded: bool,
) -> M5ProvenanceDisclosure {
    match provenance_class {
        M5FitnessProvenanceClass::ProvenanceUnknown => {
            M5ProvenanceDisclosure::ProvenanceUndisclosed
        }
        M5FitnessProvenanceClass::CanonicalCorpus => {
            if support_class_bounded {
                M5ProvenanceDisclosure::CanonicalWithinSupportClass
            } else {
                M5ProvenanceDisclosure::ProfilePinnedDiscloseScope
            }
        }
        M5FitnessProvenanceClass::ProfilePinned => {
            M5ProvenanceDisclosure::ProfilePinnedDiscloseScope
        }
        M5FitnessProvenanceClass::SampledCorpus => M5ProvenanceDisclosure::SampledDiscloseCaveat,
        M5FitnessProvenanceClass::SyntheticCorpus => {
            M5ProvenanceDisclosure::SyntheticDiscloseCaveat
        }
    }
}

/// Resolves one governance report row from its declared state.
pub fn resolve_governance_report(
    input: &M5GovernanceReportResolutionInput,
) -> Result<M5ResolvedGovernanceReport, M5GovernanceReportResolutionError> {
    if input.report_id_repr.trim().is_empty() {
        return Err(M5GovernanceReportResolutionError::EmptyReportId);
    }
    if input.timestamp_repr.trim().is_empty() {
        return Err(M5GovernanceReportResolutionError::EmptyTimestamp);
    }
    if value_repr_is_forbidden(&input.report_id_repr)
        || value_repr_is_forbidden(&input.timestamp_repr)
    {
        return Err(M5GovernanceReportResolutionError::ForbiddenReportMaterial);
    }

    let provenance_disclosure =
        resolve_provenance_disclosure(input.provenance_class, input.support_class_bounded);
    let provenance_trustable_outside_support_class =
        provenance_disclosure.is_trustable_outside_support_class();

    let degrade_reason = derive_report_degrade(
        input.declared_outcome,
        input.evidence_freshness,
        provenance_disclosure,
        input.support_class_bounded,
    );
    let readiness_state = match degrade_reason {
        Some(reason) => reason.readiness_state(),
        None => M5GovernanceReadinessState::Passing,
    };
    let next_action = degrade_reason.map(M5ReportDegradeReason::next_action);
    let provenance_note = format!(
        "Provenance: {} produced this {} result; {}",
        input.provenance_class.as_str(),
        input.report_type.as_str(),
        if provenance_trustable_outside_support_class {
            "trustable outside its support class".to_owned()
        } else {
            format!(
                "disclosed as `{}`, not trustable outside its support class",
                provenance_disclosure.as_str()
            )
        }
    );
    let degrade_note = degrade_reason.map(|reason| {
        format!(
            "Report degraded: {} — state `{}`; next: {}",
            reason.phrase(),
            readiness_state.as_str(),
            reason.next_action().as_str()
        )
    });

    Ok(M5ResolvedGovernanceReport {
        report_id_repr: input.report_id_repr.clone(),
        report_type: input.report_type,
        report_scope: input.report_scope,
        provenance_class: input.provenance_class,
        timestamp_repr: input.timestamp_repr.clone(),
        declared_outcome: input.declared_outcome,
        evidence_freshness: input.evidence_freshness,
        support_class_bounded: input.support_class_bounded,
        provenance_disclosure,
        provenance_trustable_outside_support_class,
        readiness_state,
        is_clean_pass: readiness_state.is_clean_pass(),
        degrade_reason,
        next_action,
        report_actions: vec![
            M5ReportAction::CompareReport,
            M5ReportAction::OpenReport,
            M5ReportAction::InspectProvenance,
        ],
        provenance_note,
        degrade_note,
    })
}

/// The fixed degrade-first report ladder. Returns `None` for a clean pass.
fn derive_report_degrade(
    outcome: M5ReportOutcome,
    evidence: M5EvidenceFreshness,
    provenance_disclosure: M5ProvenanceDisclosure,
    support_class_bounded: bool,
) -> Option<M5ReportDegradeReason> {
    if matches!(outcome, M5ReportOutcome::ReportNotRun)
        || matches!(evidence, M5EvidenceFreshness::EvidenceUnknown)
    {
        Some(M5ReportDegradeReason::NotYetEvaluated)
    } else if matches!(evidence, M5EvidenceFreshness::EvidenceMissing) {
        Some(M5ReportDegradeReason::EvidenceMissingReading)
    } else if matches!(evidence, M5EvidenceFreshness::EvidenceStale) {
        Some(M5ReportDegradeReason::EvidenceStaleReading)
    } else if matches!(outcome, M5ReportOutcome::ReportFail) {
        Some(M5ReportDegradeReason::OutcomeFailed)
    } else if matches!(
        provenance_disclosure,
        M5ProvenanceDisclosure::ProvenanceUndisclosed
    ) {
        Some(M5ReportDegradeReason::ProvenanceUndisclosedReason)
    } else if !support_class_bounded {
        Some(M5ReportDegradeReason::ProvenanceOutOfSupportClass)
    } else if matches!(outcome, M5ReportOutcome::ReportPartial)
        || matches!(evidence, M5EvidenceFreshness::EvidenceAging)
    {
        Some(M5ReportDegradeReason::OutcomePartial)
    } else {
        None
    }
}

/// One worked governance-report resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceReportCase {
    /// The resolver input.
    pub input: M5GovernanceReportResolutionInput,
    /// The resolved truth. Must equal `resolve_governance_report(&input)`.
    pub resolved: M5ResolvedGovernanceReport,
}

impl M5GovernanceReportCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5GovernanceReportResolutionInput) -> Self {
        let resolved =
            resolve_governance_report(&input).expect("seed governance-report case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_governance_report(&self.input).as_ref() == Ok(&self.resolved)
    }
}

// ---------------------------------------------------------------------------
// Parity matrix
// ---------------------------------------------------------------------------

/// One row in the controls matrix: one assurance consumer bound to the shared tile and
/// row anatomy, readiness states, provenance classes, report scopes, threshold states,
/// evidence-freshness states, degrade reasons, provenance disclosures, report actions,
/// export fields, and accessibility routes, plus worked resolution cases for both
/// families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FitnessGovernanceRow {
    /// Assurance consumer family.
    pub consumer_surface: M5FitnessGovernanceConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5GovernanceQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 governance surface families that render / consume these components.
    pub surface_families: Vec<M5GovernanceSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts these components render (must include the mandatory parts).
    pub anatomy_parts: Vec<M5FitnessGovernanceAnatomyPart>,
    /// Required labels these components can show (must include the mandatory labels).
    pub required_labels: Vec<M5GovernanceRequiredLabel>,
    /// Readiness states these components distinguish.
    pub readiness_states: Vec<M5GovernanceReadinessState>,
    /// Declared fitness states these tiles distinguish.
    pub fitness_declared_states: Vec<M5FitnessDeclaredState>,
    /// Threshold states these tiles distinguish.
    pub threshold_states: Vec<M5ThresholdState>,
    /// Fitness provenance classes these components name.
    pub provenance_classes: Vec<M5FitnessProvenanceClass>,
    /// Evidence-freshness readings these components distinguish.
    pub evidence_freshness_states: Vec<M5EvidenceFreshness>,
    /// Profile-match states these tiles distinguish.
    pub profile_match_states: Vec<M5ProfileMatchState>,
    /// Fitness degrade reasons these tiles name.
    pub fitness_degrade_reasons: Vec<M5FitnessDegradeReason>,
    /// Report types these rows name.
    pub report_types: Vec<M5GovernanceReportType>,
    /// Report scopes these rows distinguish.
    pub report_scopes: Vec<M5GovernanceReportScope>,
    /// Report outcomes these rows distinguish.
    pub report_outcomes: Vec<M5ReportOutcome>,
    /// Provenance disclosures these rows name.
    pub provenance_disclosures: Vec<M5ProvenanceDisclosure>,
    /// Report degrade reasons these rows name.
    pub report_degrade_reasons: Vec<M5ReportDegradeReason>,
    /// Report actions these rows offer (must include the mandatory actions).
    pub report_actions: Vec<M5ReportAction>,
    /// Next actions these components name.
    pub next_actions: Vec<M5GovernanceNextAction>,
    /// Export fields these components carry (must include the mandatory fields).
    pub export_fields: Vec<M5FitnessGovernanceExportField>,
    /// Non-visual accessibility routes these components offer.
    pub accessibility_routes: Vec<M5GovernanceAccessibilityRoute>,
    /// Governance subsystems that consume these components' projection.
    pub consumer_surfaces: Vec<M5GovernanceConsumerSurface>,
    /// Downgrade triggers that apply to these components.
    pub downgrade_triggers: Vec<M5GovernanceDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked fitness-tile cases proving the tile resolver on this consumer.
    pub fitness_tile_examples: Vec<M5FitnessTileCase>,
    /// Worked governance-report cases proving the report resolver on this consumer.
    pub report_row_examples: Vec<M5GovernanceReportCase>,
    /// Hard invariant: this row never renders stale or wrong-profile evidence as a
    /// clean pass. MUST be `false`.
    pub renders_stale_or_wrong_profile_as_clean_pass: bool,
    /// Hard invariant: this row never hides the corpus/profile provenance. MUST be
    /// `false`.
    pub hides_corpus_or_profile_provenance: bool,
    /// Hard invariant: this row never hides the owner or evidence freshness. MUST be
    /// `false`.
    pub hides_owner_or_evidence_freshness: bool,
    /// Hard invariant: this row never invents a dashboard-local status word. MUST be
    /// `false`.
    pub invents_dashboard_local_status_grammar: bool,
}

impl M5FitnessGovernanceRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5FitnessGovernanceAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5FitnessGovernanceAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory required label.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5GovernanceRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5GovernanceRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// True when the row declares every mandatory report action.
    fn declares_mandatory_report_actions(&self) -> bool {
        let present: BTreeSet<M5ReportAction> = self.report_actions.iter().copied().collect();
        M5ReportAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5FitnessGovernanceExportField> =
            self.export_fields.iter().copied().collect();
        M5FitnessGovernanceExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.renders_stale_or_wrong_profile_as_clean_pass
            && !self.hides_corpus_or_profile_provenance
            && !self.hides_owner_or_evidence_freshness
            && !self.invents_dashboard_local_status_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FitnessGovernanceVocabularySet {
    /// Assurance consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Readiness-state tokens (reused from the frozen matrix).
    pub readiness_states: Vec<String>,
    /// Declared-fitness-state tokens.
    pub fitness_declared_states: Vec<String>,
    /// Threshold-state tokens.
    pub threshold_states: Vec<String>,
    /// Fitness-provenance-class tokens (reused from the frozen matrix).
    pub provenance_classes: Vec<String>,
    /// Evidence-freshness tokens.
    pub evidence_freshness_states: Vec<String>,
    /// Profile-match-state tokens.
    pub profile_match_states: Vec<String>,
    /// Fitness-degrade-reason tokens.
    pub fitness_degrade_reasons: Vec<String>,
    /// Report-type tokens.
    pub report_types: Vec<String>,
    /// Report-scope tokens (reused from the frozen matrix).
    pub report_scopes: Vec<String>,
    /// Report-outcome tokens.
    pub report_outcomes: Vec<String>,
    /// Provenance-disclosure tokens.
    pub provenance_disclosures: Vec<String>,
    /// Report-degrade-reason tokens.
    pub report_degrade_reasons: Vec<String>,
    /// Report-action tokens.
    pub report_actions: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5FitnessGovernanceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5FitnessGovernanceConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5FitnessGovernanceAnatomyPart::ALL, |v| v.as_str()),
            readiness_states: tokens(&M5GovernanceReadinessState::ALL, |v| v.as_str()),
            fitness_declared_states: tokens(&M5FitnessDeclaredState::ALL, |v| v.as_str()),
            threshold_states: tokens(&M5ThresholdState::ALL, |v| v.as_str()),
            provenance_classes: tokens(&M5FitnessProvenanceClass::ALL, |v| v.as_str()),
            evidence_freshness_states: tokens(&M5EvidenceFreshness::ALL, |v| v.as_str()),
            profile_match_states: tokens(&M5ProfileMatchState::ALL, |v| v.as_str()),
            fitness_degrade_reasons: tokens(&M5FitnessDegradeReason::ALL, |v| v.as_str()),
            report_types: tokens(&M5GovernanceReportType::ALL, |v| v.as_str()),
            report_scopes: tokens(&M5GovernanceReportScope::ALL, |v| v.as_str()),
            report_outcomes: tokens(&M5ReportOutcome::ALL, |v| v.as_str()),
            provenance_disclosures: tokens(&M5ProvenanceDisclosure::ALL, |v| v.as_str()),
            report_degrade_reasons: tokens(&M5ReportDegradeReason::ALL, |v| v.as_str()),
            report_actions: tokens(&M5ReportAction::ALL, |v| v.as_str()),
            next_actions: tokens(&M5GovernanceNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5FitnessGovernanceExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5GovernanceAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5FitnessGovernanceReview {
    /// One controls packet carries fitness and governance truth on every consumer.
    pub one_packet_carries_fitness_and_governance_truth: bool,
    /// The fitness identity and report type are shown before a reading is trusted.
    pub identity_and_report_type_always_shown: bool,
    /// A green metric with stale or wrong-profile evidence never reads as a clean pass.
    pub stale_or_wrong_profile_never_reads_clean_pass: bool,
    /// The corpus/profile provenance is always disclosed on the row.
    pub corpus_or_profile_provenance_always_disclosed: bool,
    /// A result outside its support class is never presented as trustable.
    pub out_of_support_class_never_presented_trustable: bool,
    /// The owner and evidence freshness are always shown on the tile.
    pub owner_and_evidence_freshness_always_shown: bool,
    /// Every report row offers compare and open-report actions.
    pub compare_and_open_report_always_offered: bool,
    /// The readiness state is drawn only from the frozen vocabulary.
    pub readiness_state_drawn_from_frozen_vocabulary: bool,
    /// The support / export packet reconstructs tile and row truth.
    pub support_export_reconstructs_truth: bool,
    /// No consumer invents a second fitness or governance grammar.
    pub no_surface_invents_second_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// An owner alias is a role alias, never a personal contact detail.
    pub owner_alias_is_role_not_person: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FitnessGovernanceConsumerProjection {
    /// Assurance, operator, shiproom, CLI, and support consumers all consume the
    /// shared controls packet.
    pub surfaces_consume_shared_packet: bool,
    /// The readiness resolver reads a single canonical source.
    pub readiness_resolver_reads_single_source: bool,
    /// The provenance disclosure reads a single canonical source.
    pub provenance_disclosure_reads_single_source: bool,
    /// The evidence-freshness cue reads a single canonical source.
    pub evidence_freshness_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FitnessGovernanceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the controls packet.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FitnessGovernanceReleasePosture {
    /// Ref of the supporting governance packet.
    pub governance_packet_ref: String,
    /// Ref of the supporting assurance audit.
    pub assurance_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5FitnessGovernanceControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FitnessGovernanceControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5FitnessGovernanceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FitnessGovernanceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FitnessGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FitnessGovernanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FitnessGovernanceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FitnessGovernanceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 fitness / governance-report controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FitnessGovernanceControlsPacket {
    /// Record kind; must equal [`M5_FITNESS_GOVERNANCE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5FitnessGovernanceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FitnessGovernanceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FitnessGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FitnessGovernanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FitnessGovernanceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FitnessGovernanceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5FitnessGovernanceControlsPacket {
    /// Builds an M5 fitness / governance-report controls packet from stable-lane input.
    pub fn new(input: M5FitnessGovernanceControlsPacketInput) -> Self {
        Self {
            record_kind: M5_FITNESS_GOVERNANCE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            controls_rows: input.controls_rows,
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

    /// Validates the M5 fitness / governance-report controls invariants.
    pub fn validate(&self) -> Vec<M5FitnessGovernanceControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FITNESS_GOVERNANCE_CONTROLS_RECORD_KIND {
            violations.push(M5FitnessGovernanceControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5FitnessGovernanceControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FitnessGovernanceControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_controls_rows(self, &mut violations);
        validate_fitness_degrade_proven(self, &mut violations);
        validate_provenance_disclosure_proven(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 fitness/governance controls packet serializes"),
        ) {
            violations.push(M5FitnessGovernanceControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 fitness/governance controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per assurance consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,readiness_states,provenance_classes,provenance_disclosures,report_actions,export_fields,fitness_example_count,report_example_count\n",
        );
        for row in &self.controls_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.readiness_states, |v| v.as_str()),
                join_tokens(&row.provenance_classes, |v| v.as_str()),
                join_tokens(&row.provenance_disclosures, |v| v.as_str()),
                join_tokens(&row.report_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.fitness_tile_examples.len(),
                row.report_row_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .controls_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Fitness Dashboard Tile and Governance Report Row Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Assurance consumers: {} ({} stable)\n",
            self.controls_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Readiness states: {}\n",
            self.vocabulary_set.readiness_states.join(", ")
        ));
        out.push_str(&format!(
            "- Provenance disclosures: {}\n",
            self.vocabulary_set.provenance_disclosures.join(", ")
        ));
        out.push_str(&format!(
            "- Evidence-freshness states: {}\n",
            self.vocabulary_set.evidence_freshness_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Assurance consumers\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked fitness tiles: {}\n",
                row.fitness_tile_examples.len()
            ));
            for case in &row.fitness_tile_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (declared `{}`, evidence `{}`, profile `{}`)\n",
                    case.resolved.fitness_id_repr,
                    case.resolved.readiness_state.as_str(),
                    case.resolved.declared_state.as_str(),
                    case.resolved.evidence_freshness.as_str(),
                    case.resolved.profile_match.as_str(),
                ));
            }
            out.push_str(&format!(
                "  - Worked governance reports: {}\n",
                row.report_row_examples.len()
            ));
            for case in &row.report_row_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (provenance `{}`, disclosure `{}`, trustable-outside `{}`)\n",
                    case.resolved.report_id_repr,
                    case.resolved.readiness_state.as_str(),
                    case.resolved.provenance_class.as_str(),
                    case.resolved.provenance_disclosure.as_str(),
                    case.resolved.provenance_trustable_outside_support_class,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 fitness/governance controls export.
#[derive(Debug)]
pub enum M5FitnessGovernanceControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5FitnessGovernanceControlsViolation>),
}

impl fmt::Display for M5FitnessGovernanceControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 fitness/governance controls export parse failed: {error}"
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
                    "m5 fitness/governance controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5FitnessGovernanceControlsArtifactError {}

/// Validation failures emitted by [`M5FitnessGovernanceControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FitnessGovernanceControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required assurance consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory required labels.
    MandatoryLabelMissing,
    /// A controls row omits one of the mandatory report actions.
    MandatoryReportActionMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A controls row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A controls row declares no fitness-tile worked cases.
    FitnessExampleMissing,
    /// A controls row declares no governance-report worked cases.
    ReportExampleMissing,
    /// A worked fitness-tile case does not match a fresh resolve of its input.
    FitnessExampleDrift,
    /// A worked governance-report case does not match a fresh resolve of its input.
    ReportExampleDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked fitness case proves a green metric degrading on stale/wrong-profile
    /// evidence (the AC-1 example).
    FitnessDegradeUnproven,
    /// No worked report case proves a non-canonical provenance disclosed as not
    /// trustable outside its support class (the AC-2 example).
    ProvenanceDisclosureUnproven,
    /// A controls row violates a hard invariant.
    ControlsInvariantViolated,
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

impl M5FitnessGovernanceControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::MandatoryReportActionMissing => "mandatory_report_action_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::FitnessExampleMissing => "fitness_example_missing",
            Self::ReportExampleMissing => "report_example_missing",
            Self::FitnessExampleDrift => "fitness_example_drift",
            Self::ReportExampleDrift => "report_example_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::FitnessDegradeUnproven => "fitness_degrade_unproven",
            Self::ProvenanceDisclosureUnproven => "provenance_disclosure_unproven",
            Self::ControlsInvariantViolated => "controls_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 fitness/governance controls export.
pub fn current_stable_m5_fitness_governance_controls_export(
) -> Result<M5FitnessGovernanceControlsPacket, M5FitnessGovernanceControlsArtifactError> {
    let packet: M5FitnessGovernanceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-fitness-governance-report-controls-proof/support_export.json"
    )))
    .map_err(M5FitnessGovernanceControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FitnessGovernanceControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF,
        M5_FITNESS_GOVERNANCE_CONTROLS_DOC_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF,
        M5_FITNESS_DASHBOARD_TILE_CONTRACT_REF,
        M5_GOVERNANCE_REPORT_ROW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5FitnessGovernanceControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5FitnessGovernanceControlsViolation::VocabularySetDrift);
    }
}

fn validate_controls_rows(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    let present: BTreeSet<M5FitnessGovernanceConsumerSurface> = packet
        .controls_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5FitnessGovernanceConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5FitnessGovernanceControlsViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.readiness_states.is_empty()
            || row.fitness_declared_states.is_empty()
            || row.threshold_states.is_empty()
            || row.provenance_classes.is_empty()
            || row.evidence_freshness_states.is_empty()
            || row.profile_match_states.is_empty()
            || row.fitness_degrade_reasons.is_empty()
            || row.report_types.is_empty()
            || row.report_scopes.is_empty()
            || row.report_outcomes.is_empty()
            || row.provenance_disclosures.is_empty()
            || row.report_degrade_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5FitnessGovernanceControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5FitnessGovernanceControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5FitnessGovernanceControlsViolation::MandatoryLabelMissing);
        }
        if !row.declares_mandatory_report_actions() {
            violations.push(M5FitnessGovernanceControlsViolation::MandatoryReportActionMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5FitnessGovernanceControlsViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5FitnessGovernanceControlsViolation::AccessibilityRouteMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5FitnessGovernanceControlsViolation::DowngradeTriggersMissing);
        }
        if row.fitness_tile_examples.is_empty() {
            violations.push(M5FitnessGovernanceControlsViolation::FitnessExampleMissing);
        }
        if row.report_row_examples.is_empty() {
            violations.push(M5FitnessGovernanceControlsViolation::ReportExampleMissing);
        }
        if row
            .fitness_tile_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5FitnessGovernanceControlsViolation::FitnessExampleDrift);
        }
        if row
            .report_row_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5FitnessGovernanceControlsViolation::ReportExampleDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5FitnessGovernanceControlsViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5FitnessGovernanceControlsViolation::ControlsInvariantViolated);
        }
    }
}

/// At least one worked fitness case across the matrix must prove a green metric —
/// declared passing — that degrades below a clean pass because its evidence is stale
/// or its profile is wrong. This is the AC-1 example that a green metric with stale or
/// wrong-profile evidence never looks equivalent to a fresh pass.
fn validate_fitness_degrade_proven(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    let proven = packet.controls_rows.iter().any(|row| {
        row.fitness_tile_examples.iter().any(|case| {
            case.resolved.declared_state == M5FitnessDeclaredState::MetricPass
                && !case.resolved.is_clean_pass
                && (matches!(
                    case.resolved.evidence_freshness,
                    M5EvidenceFreshness::EvidenceStale
                ) || matches!(
                    case.resolved.profile_match,
                    M5ProfileMatchState::WrongProfile | M5ProfileMatchState::ProfileUnpinned
                ))
        })
    });
    if !proven {
        violations.push(M5FitnessGovernanceControlsViolation::FitnessDegradeUnproven);
    }
}

/// At least one worked report case across the matrix must prove a non-canonical
/// corpus/profile whose provenance is disclosed as not trustable outside its support
/// class. This is the AC-2 example that a user can tell what kind of corpus/profile
/// produced a result before trusting it outside its support class.
fn validate_provenance_disclosure_proven(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    let proven = packet.controls_rows.iter().any(|row| {
        row.report_row_examples.iter().any(|case| {
            !case.resolved.provenance_trustable_outside_support_class
                && matches!(
                    case.resolved.provenance_disclosure,
                    M5ProvenanceDisclosure::SampledDiscloseCaveat
                        | M5ProvenanceDisclosure::SyntheticDiscloseCaveat
                        | M5ProvenanceDisclosure::ProfilePinnedDiscloseScope
                )
        })
    });
    if !proven {
        violations.push(M5FitnessGovernanceControlsViolation::ProvenanceDisclosureUnproven);
    }
}

fn validate_governance_review(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_packet_carries_fitness_and_governance_truth,
        review.identity_and_report_type_always_shown,
        review.stale_or_wrong_profile_never_reads_clean_pass,
        review.corpus_or_profile_provenance_always_disclosed,
        review.out_of_support_class_never_presented_trustable,
        review.owner_and_evidence_freshness_always_shown,
        review.compare_and_open_report_always_offered,
        review.readiness_state_drawn_from_frozen_vocabulary,
        review.support_export_reconstructs_truth,
        review.no_surface_invents_second_grammar,
        review.every_row_declares_accessibility_route,
        review.owner_alias_is_role_not_person,
    ] {
        if !ok {
            violations.push(M5FitnessGovernanceControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.surfaces_consume_shared_packet,
        projection.readiness_resolver_reads_single_source,
        projection.provenance_disclosure_reads_single_source,
        projection.evidence_freshness_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5FitnessGovernanceControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5FitnessGovernanceControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5FitnessGovernanceControlsPacket,
    violations: &mut Vec<M5FitnessGovernanceControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.governance_packet_ref.trim().is_empty()
        || posture.assurance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5FitnessGovernanceControlsViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces
/// a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
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
fn value_repr_is_forbidden(value: &str) -> bool {
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
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
