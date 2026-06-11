//! M5 diagnosis-latency corpora, blocked-user recovery ladders, and
//! support-bundle parity for Project Doctor and guided-repair flows.
//!
//! This module owns one canonical, checked-in packet —
//! [`ProjectDoctorM5RecoveryFieldReadiness`] — that turns the M5 field-readiness
//! and supportability story into rerunnable truth. Each
//! [`BlockedUserScenario`] seeds one blocked-user situation in one M5 recovery
//! lane (notebook kernel, request/API, database target, profiler/replay, remote
//! preview, sync/device-registry, companion handoff, or incident packet) and
//! pins, in one record:
//!
//! - the initiating [`BlockedUserScenario::initiating_findings`] (lane-scoped,
//!   `doctor.finding.<lane>.`-prefixed) so support can reconstruct *what* was
//!   diagnosed,
//! - the chosen [`RecoveryRung`] on the blocked-user recovery ladder (safe mode,
//!   quarantine, open-without-restore, cache/index repair, restricted reopen, or
//!   a typed repair) so support can reconstruct *which rung* was taken,
//! - a per-percentile [`DiagnosisLatencyBudget`] for time-to-first-actionable
//!   diagnosis (the p90 target is the headline) and the corpus's
//!   [`ObservedLatency`] drill measurements, so latency regressions are visible,
//! - corpus freshness ([`BlockedUserScenario::corpus_age_days`] vs
//!   [`BlockedUserScenario::freshness_window_days`]) so stale corpora are
//!   visible, and
//! - a [`SupportBundleLinkage`] that preserves finding ids, repair ids, scope
//!   refs, and durable-evidence refs under a metadata-safe redaction posture
//!   without overcapturing raw content.
//!
//! The central guarantee is a **non-inheriting promotion gate**: every
//! scenario's published [`BlockedUserScenario::published_promotion_action`] and
//! [`BlockedUserScenario::published_narrowing_reason`] are validated against the
//! gate decision recomputed from the scenario's own drill outcome, freshness,
//! latency state, and escalation completeness ([`BlockedUserScenario::recompute_gate`]).
//! A stale corpus, a breached p90 latency budget, a missing durable-evidence
//! packet, or an unhanded-off drill therefore narrows that scenario's M5
//! promotion automatically instead of inheriting "ready" from an adjacent
//! scenario.
//!
//! The packet is checked in at
//! `artifacts/doctor/m5/project-doctor-m5-recovery-field-readiness.json` and
//! embedded here via `include_str!`, so this typed consumer and any CI gate
//! agree on every row without a cargo build in CI. The model is metadata-only:
//! every field is a typed state or an opaque ref. It carries no credential
//! bodies, raw provider payloads, or mount/port/tunnel secrets.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported field-readiness packet schema version.
pub const PROJECT_DOCTOR_M5_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PROJECT_DOCTOR_M5_RECOVERY_RECORD_KIND: &str =
    "project_doctor_m5_recovery_field_readiness";

/// Repo-relative path to the checked-in packet.
pub const PROJECT_DOCTOR_M5_RECOVERY_PATH: &str =
    "artifacts/doctor/m5/project-doctor-m5-recovery-field-readiness.json";

/// Repo-relative path to the boundary schema.
pub const PROJECT_DOCTOR_M5_RECOVERY_SCHEMA_REF: &str =
    "schemas/doctor/project-doctor-m5-recovery-field-readiness.schema.json";

/// Repo-relative path to the companion document.
pub const PROJECT_DOCTOR_M5_RECOVERY_DOC_REF: &str =
    "docs/doctor/m5/project-doctor-m5-recovery-field-readiness.md";

/// Stable finding-code prefix every initiating finding must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Stable repair-id prefix every typed-repair scenario must use.
pub const DOCTOR_REPAIR_PREFIX: &str = "repair.";

/// Canonical, locale-invariant machine-meaning keys every scenario must carry,
/// so localized prose can never silently change what a surface means.
pub const REQUIRED_MACHINE_MEANING_KEYS: [&str; 5] = [
    "scenario_id",
    "lane",
    "recovery_rung",
    "drill_outcome",
    "promotion_action",
];

/// Required redaction class for every support-bundle linkage.
pub const METADATA_SAFE_REDACTION_CLASS: &str = "metadata_safe_default";

/// Embedded checked-in packet JSON.
pub const PROJECT_DOCTOR_M5_RECOVERY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/doctor/m5/project-doctor-m5-recovery-field-readiness.json"
));

/// Generic, non-actionable explanation tokens that may never stand in for a
/// specific first-actionable diagnosis explanation.
const GENERIC_EXPLANATION_TOKENS: [&str; 7] = [
    "unavailable",
    "error",
    "failed",
    "failure",
    "unknown",
    "generic_failure",
    "n_a",
];

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// The M5 recovery lane a blocked-user scenario belongs to.
///
/// Tokens match the Project Doctor feature-lane probe families so the same lane
/// identity is carried across diagnosis, repair, and field-readiness surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLane {
    /// Notebook kernel runtime and attach readiness.
    NotebookKernel,
    /// Request/API auth authority and environment binding.
    RequestApi,
    /// Database target identity and schema alignment.
    DatabaseTarget,
    /// Profiler/replay instrumentation and capture coverage.
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

impl RecoveryLane {
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

    /// The stable finding-code prefix every initiating finding in this lane must
    /// start with.
    pub fn finding_code_prefix(self) -> String {
        format!("{DOCTOR_FINDING_PREFIX}{}.", self.as_str())
    }
}

impl std::fmt::Display for RecoveryLane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One rung of the blocked-user recovery ladder.
///
/// The ladder runs from least to most invasive: observe-and-restart safe mode,
/// component quarantine, opening without restoring prior state, repairing a
/// cache/index, a restricted reopen, then a typed (transactional) repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryRung {
    /// Reopen in safe mode with reduced capability.
    SafeMode,
    /// Quarantine the suspect component, then continue.
    Quarantine,
    /// Open the workspace without restoring prior session state.
    OpenWithoutRestore,
    /// Rebuild or repair a derived cache/index.
    CacheIndexRepair,
    /// Reopen with a restricted, explicitly narrowed capability set.
    RestrictedReopen,
    /// Run a typed, checkpoint-backed repair transaction.
    TypedRepair,
}

impl RecoveryRung {
    /// Every rung, in ladder order.
    pub const ALL: [Self; 6] = [
        Self::SafeMode,
        Self::Quarantine,
        Self::OpenWithoutRestore,
        Self::CacheIndexRepair,
        Self::RestrictedReopen,
        Self::TypedRepair,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::Quarantine => "quarantine",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::CacheIndexRepair => "cache_index_repair",
            Self::RestrictedReopen => "restricted_reopen",
            Self::TypedRepair => "typed_repair",
        }
    }

    /// True when this rung runs a typed repair transaction and therefore must
    /// carry a `repair.`-prefixed repair id.
    pub const fn requires_repair_id(self) -> bool {
        matches!(self, Self::TypedRepair)
    }
}

impl std::fmt::Display for RecoveryRung {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Latency percentile a diagnosis-latency budget or observation is measured at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyPercentile {
    /// 50th percentile.
    P50,
    /// 90th percentile — the headline first-actionable-diagnosis target.
    P90,
    /// 95th percentile.
    P95,
}

impl LatencyPercentile {
    /// Every percentile, in declaration order.
    pub const ALL: [Self; 3] = [Self::P50, Self::P90, Self::P95];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::P50 => "p50",
            Self::P90 => "p90",
            Self::P95 => "p95",
        }
    }
}

/// The state of an observed latency relative to its budget thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyState {
    /// At or under the green target.
    WithinBudget,
    /// Over target but at or under the red threshold.
    Yellow,
    /// Over the red threshold — a regression that narrows promotion.
    Breached,
}

impl LatencyState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::Yellow => "yellow",
            Self::Breached => "breached",
        }
    }
}

/// The outcome of running the blocked-user field-readiness drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillOutcome {
    /// The blocked user was diagnosed and handed off with an export-safe packet.
    DiagnosedAndHandedOff,
    /// The blocked user was diagnosed but no export-safe handoff completed.
    DiagnosedNotHandedOff,
    /// The blocked user could not be diagnosed with the current corpus.
    NotDiagnosed,
}

impl DrillOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DiagnosedAndHandedOff,
        Self::DiagnosedNotHandedOff,
        Self::NotDiagnosed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosedAndHandedOff => "diagnosed_and_handed_off",
            Self::DiagnosedNotHandedOff => "diagnosed_not_handed_off",
            Self::NotDiagnosed => "not_diagnosed",
        }
    }
}

/// The promotion action the non-inheriting gate takes for a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionAction {
    /// Publish the scenario as field-ready at full strength.
    PublishFull,
    /// Narrow the scenario to advisory-only readiness.
    NarrowToAdvisory,
    /// Block the scenario from M5 promotion entirely.
    BlockPromotion,
}

impl PromotionAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PublishFull,
        Self::NarrowToAdvisory,
        Self::BlockPromotion,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishFull => "publish_full",
            Self::NarrowToAdvisory => "narrow_to_advisory",
            Self::BlockPromotion => "block_promotion",
        }
    }
}

/// Why the gate narrowed or blocked a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// No narrowing applied; the scenario published at full strength.
    None,
    /// The diagnosis-latency corpus is past its freshness window.
    StaleCorpus,
    /// The observed p90 first-actionable latency breached its red threshold.
    LatencyBreached,
    /// The escalation packet lacks durable evidence (or a typed-repair id).
    EvidenceMissing,
    /// The drill diagnosed the user but no export-safe handoff completed.
    DrillNotHandedOff,
    /// The drill could not diagnose the user with the current corpus.
    DrillNotDiagnosed,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::None,
        Self::StaleCorpus,
        Self::LatencyBreached,
        Self::EvidenceMissing,
        Self::DrillNotHandedOff,
        Self::DrillNotDiagnosed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::StaleCorpus => "stale_corpus",
            Self::LatencyBreached => "latency_breached",
            Self::EvidenceMissing => "evidence_missing",
            Self::DrillNotHandedOff => "drill_not_handed_off",
            Self::DrillNotDiagnosed => "drill_not_diagnosed",
        }
    }
}

/// A surface that must render the same scenario identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParitySurface {
    /// Desktop field-readiness pane.
    DesktopPane,
    /// CLI summary row.
    CliRow,
    /// Headless machine-readable JSON.
    HeadlessJson,
    /// Support-bundle export.
    SupportExport,
    /// Incident-packet view.
    IncidentPacket,
    /// Public-truth/release surface.
    PublicTruth,
}

impl ParitySurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopPane,
        Self::CliRow,
        Self::HeadlessJson,
        Self::SupportExport,
        Self::IncidentPacket,
        Self::PublicTruth,
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
// Record types
// ---------------------------------------------------------------------------

/// One percentile latency budget for time-to-first-actionable diagnosis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosisLatencyBudget {
    /// Percentile this budget applies to.
    pub percentile: LatencyPercentile,
    /// Green target in milliseconds.
    pub target_ms: u64,
    /// Yellow threshold in milliseconds.
    pub yellow_ms: u64,
    /// Red threshold in milliseconds.
    pub red_ms: u64,
}

/// One observed first-actionable diagnosis latency from the corpus drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedLatency {
    /// Percentile this observation applies to.
    pub percentile: LatencyPercentile,
    /// Observed milliseconds.
    pub observed_ms: u64,
}

/// The support-bundle and escalation-packet linkage that preserves Doctor and
/// repair identity for a scenario without overcapturing raw content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportBundleLinkage {
    /// Opaque ref to the support-bundle manifest.
    pub bundle_manifest_ref: String,
    /// Opaque ref to the escalation packet.
    pub escalation_packet_ref: String,
    /// Finding ids preserved in the packet (each `doctor.finding.`-prefixed).
    pub preserved_finding_ids: Vec<String>,
    /// Repair ids preserved in the packet (each `repair.`-prefixed).
    pub preserved_repair_ids: Vec<String>,
    /// Opaque scope refs preserved (mount/port/tunnel/target identities).
    pub preserved_scope_refs: Vec<String>,
    /// Opaque refs to durable evidence carried for reconstruction.
    pub durable_evidence_refs: Vec<String>,
    /// Redaction posture; must be the metadata-safe default.
    pub redaction_class: String,
    /// True when raw private material is excluded from the packet.
    pub raw_private_material_excluded: bool,
    /// True when content overcapture is excluded (identity-only, no bodies).
    pub overcapture_excluded: bool,
}

impl SupportBundleLinkage {
    /// True when the escalation packet carries enough durable evidence (and a
    /// typed-repair id when one is required) to reconstruct and hand off the
    /// failure.
    pub fn is_escalation_complete(&self, requires_repair_id: bool) -> bool {
        if self.durable_evidence_refs.is_empty() {
            return false;
        }
        if requires_repair_id && self.preserved_repair_ids.is_empty() {
            return false;
        }
        true
    }

    /// True when the linkage preserves the stable identity (manifest,
    /// escalation packet, findings, scope) needed to reconstruct the failure,
    /// regardless of whether durable evidence is complete.
    pub fn preserves_identity(&self) -> bool {
        !self.bundle_manifest_ref.trim().is_empty()
            && !self.escalation_packet_ref.trim().is_empty()
            && !self.preserved_finding_ids.is_empty()
            && !self.preserved_scope_refs.is_empty()
    }

    /// True when the linkage is metadata-safe.
    pub fn is_metadata_safe(&self) -> bool {
        self.redaction_class == METADATA_SAFE_REDACTION_CLASS
            && self.raw_private_material_excluded
            && self.overcapture_excluded
    }
}

/// One seeded blocked-user scenario in one M5 recovery lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedUserScenario {
    /// Unique scenario id.
    pub scenario_id: String,
    /// The M5 recovery lane this scenario belongs to.
    pub lane: RecoveryLane,
    /// Reviewer-facing one-line summary.
    pub title: String,
    /// Initiating findings (lane-scoped, `doctor.finding.<lane>.`-prefixed).
    pub initiating_findings: Vec<String>,
    /// The chosen recovery-ladder rung.
    pub recovery_rung: RecoveryRung,
    /// Repair id, present iff the rung runs a typed repair.
    pub repair_id: Option<String>,
    /// Per-percentile first-actionable diagnosis latency budgets (must include
    /// p90).
    pub first_actionable_latency_budgets: Vec<DiagnosisLatencyBudget>,
    /// Observed first-actionable diagnosis latencies (must include p90).
    pub observed_latencies: Vec<ObservedLatency>,
    /// Human-readable date the corpus run was captured.
    pub corpus_run_at: String,
    /// Age of the corpus measurement, in whole days.
    pub corpus_age_days: u32,
    /// Freshness window in whole days; older corpora are stale.
    pub freshness_window_days: u32,
    /// The field-readiness drill outcome.
    pub drill_outcome: DrillOutcome,
    /// Support-bundle/escalation linkage.
    pub support_linkage: SupportBundleLinkage,
    /// Surfaces that render this scenario's identity.
    pub parity_surfaces: Vec<ParitySurface>,
    /// Locale-invariant machine-meaning keys.
    pub machine_meaning_keys: Vec<String>,
    /// First-actionable diagnosis explanation.
    pub explanation: String,
    /// The published promotion action (validated against the recomputed gate).
    pub published_promotion_action: PromotionAction,
    /// The published narrowing reason (validated against the recomputed gate).
    pub published_narrowing_reason: NarrowingReason,
    /// Reviewer notes.
    pub notes: String,
}

impl BlockedUserScenario {
    /// True when the corpus measurement is past its freshness window.
    pub fn is_stale(&self) -> bool {
        self.corpus_age_days > self.freshness_window_days
    }

    /// The p90 latency budget, if present.
    pub fn p90_budget(&self) -> Option<&DiagnosisLatencyBudget> {
        self.first_actionable_latency_budgets
            .iter()
            .find(|b| b.percentile == LatencyPercentile::P90)
    }

    /// The observed p90 latency, if present.
    pub fn observed_p90_ms(&self) -> Option<u64> {
        self.observed_latencies
            .iter()
            .find(|o| o.percentile == LatencyPercentile::P90)
            .map(|o| o.observed_ms)
    }

    /// The latency state of the p90 first-actionable observation relative to its
    /// budget. Returns [`LatencyState::Breached`] when either the budget or the
    /// observation is missing, so an incomplete corpus narrows rather than
    /// silently passing.
    pub fn p90_latency_state(&self) -> LatencyState {
        match (self.p90_budget(), self.observed_p90_ms()) {
            (Some(budget), Some(observed)) => {
                if observed > budget.red_ms {
                    LatencyState::Breached
                } else if observed > budget.target_ms {
                    LatencyState::Yellow
                } else {
                    LatencyState::WithinBudget
                }
            }
            _ => LatencyState::Breached,
        }
    }

    /// True when the escalation packet is complete enough for handoff.
    pub fn is_escalation_complete(&self) -> bool {
        self.support_linkage
            .is_escalation_complete(self.recovery_rung.requires_repair_id())
    }

    /// Recomputes the non-inheriting promotion gate decision for this scenario
    /// from its own drill outcome, freshness, p90 latency state, and escalation
    /// completeness.
    ///
    /// Each input narrows independently; the precedence (block > stale >
    /// latency > evidence > handoff) only decides which reason is reported when
    /// several apply. No decision is inherited from another scenario.
    pub fn recompute_gate(&self) -> (PromotionAction, NarrowingReason) {
        if self.drill_outcome == DrillOutcome::NotDiagnosed {
            return (
                PromotionAction::BlockPromotion,
                NarrowingReason::DrillNotDiagnosed,
            );
        }
        if self.is_stale() {
            return (
                PromotionAction::NarrowToAdvisory,
                NarrowingReason::StaleCorpus,
            );
        }
        if self.p90_latency_state() == LatencyState::Breached {
            return (
                PromotionAction::NarrowToAdvisory,
                NarrowingReason::LatencyBreached,
            );
        }
        if !self.is_escalation_complete() {
            return (
                PromotionAction::NarrowToAdvisory,
                NarrowingReason::EvidenceMissing,
            );
        }
        if self.drill_outcome == DrillOutcome::DiagnosedNotHandedOff {
            return (
                PromotionAction::NarrowToAdvisory,
                NarrowingReason::DrillNotHandedOff,
            );
        }
        (PromotionAction::PublishFull, NarrowingReason::None)
    }

    /// True when the scenario renders on every parity surface.
    pub fn is_cross_surface_stable(&self) -> bool {
        ParitySurface::ALL
            .iter()
            .all(|surface| self.parity_surfaces.contains(surface))
    }
}

/// Roll-up summary over all scenarios.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorM5RecoverySummary {
    /// Total scenarios.
    pub scenario_count: usize,
    /// Distinct lanes covered.
    pub lanes_covered: usize,
    /// Scenarios published at full strength.
    pub published_full_count: usize,
    /// Scenarios narrowed to advisory.
    pub narrowed_count: usize,
    /// Scenarios blocked from promotion.
    pub blocked_count: usize,
    /// Scenarios with a stale corpus.
    pub stale_corpus_count: usize,
    /// Scenarios with a breached p90 latency budget.
    pub latency_breached_count: usize,
    /// Scenarios diagnosed and handed off.
    pub diagnosed_and_handed_off_count: usize,
    /// Scenarios using a typed repair.
    pub typed_repair_count: usize,
}

/// The canonical M5 field-readiness packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorM5RecoveryFieldReadiness {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// Publication status.
    pub status: String,
    /// Overview doc page.
    pub overview_page: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Capture date.
    pub as_of: String,
    /// Enumerated recovery-lane vocabulary.
    pub recovery_lanes: Vec<String>,
    /// Enumerated recovery-rung vocabulary.
    pub recovery_rungs: Vec<String>,
    /// Enumerated latency-percentile vocabulary.
    pub latency_percentiles: Vec<String>,
    /// Enumerated drill-outcome vocabulary.
    pub drill_outcomes: Vec<String>,
    /// Enumerated promotion-action vocabulary.
    pub promotion_actions: Vec<String>,
    /// Enumerated narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<String>,
    /// Enumerated parity-surface vocabulary.
    pub parity_surfaces: Vec<String>,
    /// The seeded blocked-user scenarios.
    pub scenarios: Vec<BlockedUserScenario>,
    /// Roll-up summary.
    pub summary: ProjectDoctorM5RecoverySummary,
}

impl ProjectDoctorM5RecoveryFieldReadiness {
    /// Returns all scenarios in the given lane.
    pub fn scenarios_in_lane(
        &self,
        lane: RecoveryLane,
    ) -> impl Iterator<Item = &BlockedUserScenario> {
        self.scenarios.iter().filter(move |s| s.lane == lane)
    }

    /// Recomputes the roll-up summary from the scenarios.
    pub fn computed_summary(&self) -> ProjectDoctorM5RecoverySummary {
        let lanes: BTreeSet<RecoveryLane> = self.scenarios.iter().map(|s| s.lane).collect();
        ProjectDoctorM5RecoverySummary {
            scenario_count: self.scenarios.len(),
            lanes_covered: lanes.len(),
            published_full_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_promotion_action == PromotionAction::PublishFull)
                .count(),
            narrowed_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_promotion_action == PromotionAction::NarrowToAdvisory)
                .count(),
            blocked_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_promotion_action == PromotionAction::BlockPromotion)
                .count(),
            stale_corpus_count: self.scenarios.iter().filter(|s| s.is_stale()).count(),
            latency_breached_count: self
                .scenarios
                .iter()
                .filter(|s| s.p90_latency_state() == LatencyState::Breached)
                .count(),
            diagnosed_and_handed_off_count: self
                .scenarios
                .iter()
                .filter(|s| s.drill_outcome == DrillOutcome::DiagnosedAndHandedOff)
                .count(),
            typed_repair_count: self
                .scenarios
                .iter()
                .filter(|s| s.recovery_rung == RecoveryRung::TypedRepair)
                .count(),
        }
    }

    /// Builds the metadata-safe support-export projection.
    pub fn export_projection(&self) -> ProjectDoctorM5RecoveryExportProjection {
        ProjectDoctorM5RecoveryExportProjection {
            packet_id: self.packet_id.clone(),
            schema_ref: self.schema_ref.clone(),
            rows: self
                .scenarios
                .iter()
                .map(|s| ProjectDoctorM5RecoveryExportRow {
                    scenario_id: s.scenario_id.clone(),
                    lane: s.lane,
                    recovery_rung: s.recovery_rung,
                    drill_outcome: s.drill_outcome,
                    promotion_action: s.published_promotion_action,
                    narrowing_reason: s.published_narrowing_reason,
                    bundle_manifest_ref: s.support_linkage.bundle_manifest_ref.clone(),
                    escalation_packet_ref: s.support_linkage.escalation_packet_ref.clone(),
                })
                .collect(),
            full_readiness_count: self
                .scenarios
                .iter()
                .filter(|s| s.published_promotion_action == PromotionAction::PublishFull)
                .count(),
            raw_private_material_excluded: true,
        }
    }

    /// Validates the packet and returns every violation found.
    pub fn validate(&self) -> Vec<ProjectDoctorM5RecoveryViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROJECT_DOCTOR_M5_RECOVERY_SCHEMA_VERSION {
            push(
                &mut violations,
                "m5_recovery.schema_version",
                &self.packet_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != PROJECT_DOCTOR_M5_RECOVERY_RECORD_KIND {
            push(
                &mut violations,
                "m5_recovery.record_kind",
                &self.packet_id,
                "record_kind must be project_doctor_m5_recovery_field_readiness",
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
                push(
                    &mut violations,
                    "m5_recovery.empty_field",
                    &self.packet_id,
                    format!("{field} must be non-empty"),
                );
            }
        }
        if self.schema_ref != PROJECT_DOCTOR_M5_RECOVERY_SCHEMA_REF {
            push(
                &mut violations,
                "m5_recovery.schema_ref",
                &self.packet_id,
                format!("schema_ref must equal {PROJECT_DOCTOR_M5_RECOVERY_SCHEMA_REF}"),
            );
        }
        if self.overview_page != PROJECT_DOCTOR_M5_RECOVERY_DOC_REF {
            push(
                &mut violations,
                "m5_recovery.overview_page",
                &self.packet_id,
                format!("overview_page must equal {PROJECT_DOCTOR_M5_RECOVERY_DOC_REF}"),
            );
        }
        if self.scenarios.is_empty() {
            push(
                &mut violations,
                "m5_recovery.no_scenarios",
                &self.packet_id,
                "packet must contain at least one scenario",
            );
        }

        let mut seen_ids = BTreeSet::new();
        for scenario in &self.scenarios {
            self.validate_scenario(scenario, &mut seen_ids, &mut violations);
        }

        if self.summary != self.computed_summary() {
            push(
                &mut violations,
                "m5_recovery.summary_mismatch",
                &self.packet_id,
                "summary does not match the recomputed summary",
            );
        }

        violations
    }

    fn validate_scenario(
        &self,
        scenario: &BlockedUserScenario,
        seen_ids: &mut BTreeSet<String>,
        violations: &mut Vec<ProjectDoctorM5RecoveryViolation>,
    ) {
        let sid = scenario.scenario_id.as_str();
        if scenario.scenario_id.trim().is_empty() {
            push(
                violations,
                "m5_recovery.scenario_id_empty",
                &self.packet_id,
                "scenario_id must be non-empty",
            );
        }
        if !seen_ids.insert(scenario.scenario_id.clone()) {
            push(
                violations,
                "m5_recovery.scenario_id_duplicate",
                sid,
                "scenario_id must be unique",
            );
        }

        // Initiating findings: present and lane-scoped.
        if scenario.initiating_findings.is_empty() {
            push(
                violations,
                "m5_recovery.findings_missing",
                sid,
                "scenario must declare at least one initiating finding",
            );
        }
        let lane_prefix = scenario.lane.finding_code_prefix();
        for finding in &scenario.initiating_findings {
            if !finding.starts_with(DOCTOR_FINDING_PREFIX) {
                push(
                    violations,
                    "m5_recovery.finding_prefix",
                    sid,
                    format!("initiating finding {finding} must start with {DOCTOR_FINDING_PREFIX}"),
                );
            } else if !finding.starts_with(&lane_prefix) {
                push(
                    violations,
                    "m5_recovery.finding_lane_mismatch",
                    sid,
                    format!("initiating finding {finding} must be lane-scoped under {lane_prefix}"),
                );
            }
        }

        // Repair id presence rule.
        match (
            &scenario.repair_id,
            scenario.recovery_rung.requires_repair_id(),
        ) {
            (Some(id), true) => {
                if !id.starts_with(DOCTOR_REPAIR_PREFIX) {
                    push(
                        violations,
                        "m5_recovery.repair_id_prefix",
                        sid,
                        format!("repair_id {id} must start with {DOCTOR_REPAIR_PREFIX}"),
                    );
                }
            }
            (Some(_), false) => push(
                violations,
                "m5_recovery.repair_id_unexpected",
                sid,
                "repair_id must be absent unless the rung is typed_repair",
            ),
            (None, true) => push(
                violations,
                "m5_recovery.repair_id_missing",
                sid,
                "typed_repair rung must carry a repair_id",
            ),
            (None, false) => {}
        }

        // Latency budgets.
        validate_latency_budgets(scenario, violations);

        // Support-bundle linkage.
        let linkage = &scenario.support_linkage;
        if !linkage.preserves_identity() {
            push(
                violations,
                "m5_recovery.identity_not_preserved",
                sid,
                "support linkage must preserve manifest, escalation packet, findings, and scope",
            );
        }
        if !linkage.is_metadata_safe() {
            push(
                violations,
                "m5_recovery.linkage_not_metadata_safe",
                sid,
                "support linkage must be metadata-safe (redaction_class metadata_safe_default, raw + overcapture excluded)",
            );
        }
        for finding in &linkage.preserved_finding_ids {
            if !finding.starts_with(DOCTOR_FINDING_PREFIX) {
                push(
                    violations,
                    "m5_recovery.preserved_finding_prefix",
                    sid,
                    format!("preserved finding {finding} must start with {DOCTOR_FINDING_PREFIX}"),
                );
            }
        }
        for repair in &linkage.preserved_repair_ids {
            if !repair.starts_with(DOCTOR_REPAIR_PREFIX) {
                push(
                    violations,
                    "m5_recovery.preserved_repair_prefix",
                    sid,
                    format!("preserved repair id {repair} must start with {DOCTOR_REPAIR_PREFIX}"),
                );
            }
        }

        // Cross-surface stability.
        if !scenario.is_cross_surface_stable() {
            push(
                violations,
                "m5_recovery.parity_surface_missing",
                sid,
                "scenario must render on every parity surface",
            );
        }

        // Machine-meaning keys.
        for required in REQUIRED_MACHINE_MEANING_KEYS {
            if !scenario.machine_meaning_keys.iter().any(|k| k == required) {
                push(
                    violations,
                    "m5_recovery.machine_meaning_key_missing",
                    sid,
                    format!("scenario must carry machine-meaning key {required}"),
                );
            }
        }

        // Explanation must be specific.
        let explanation = scenario.explanation.trim().to_ascii_lowercase();
        if explanation.is_empty() {
            push(
                violations,
                "m5_recovery.explanation_empty",
                sid,
                "explanation must be non-empty",
            );
        } else if GENERIC_EXPLANATION_TOKENS.contains(&explanation.as_str()) {
            push(
                violations,
                "m5_recovery.explanation_generic",
                sid,
                "explanation must be specific, not a generic failure token",
            );
        }

        // Non-inheriting promotion gate: published == recomputed.
        let (action, reason) = scenario.recompute_gate();
        if scenario.published_promotion_action != action {
            push(
                violations,
                "m5_recovery.gate_action_mismatch",
                sid,
                format!(
                    "published_promotion_action {} does not match recomputed gate decision {}",
                    scenario.published_promotion_action.as_str(),
                    action.as_str()
                ),
            );
        }
        if scenario.published_narrowing_reason != reason {
            push(
                violations,
                "m5_recovery.gate_reason_mismatch",
                sid,
                format!(
                    "published_narrowing_reason {} does not match recomputed reason {}",
                    scenario.published_narrowing_reason.as_str(),
                    reason.as_str()
                ),
            );
        }

        // Guardrail: a full-strength publication must not coexist with a stale
        // corpus, a breached latency budget, an incomplete escalation, or an
        // unhanded-off drill. (The gate equality above already enforces this;
        // this explicit check keeps the guardrail legible and independently
        // tested.)
        if scenario.published_promotion_action == PromotionAction::PublishFull
            && (scenario.is_stale()
                || scenario.p90_latency_state() == LatencyState::Breached
                || !scenario.is_escalation_complete()
                || scenario.drill_outcome != DrillOutcome::DiagnosedAndHandedOff)
        {
            push(
                violations,
                "m5_recovery.full_publication_unsupported",
                sid,
                "publish_full requires a fresh corpus, an unbreached p90 budget, a complete escalation, and a handed-off drill",
            );
        }
    }
}

fn validate_latency_budgets(
    scenario: &BlockedUserScenario,
    violations: &mut Vec<ProjectDoctorM5RecoveryViolation>,
) {
    let sid = scenario.scenario_id.as_str();
    if scenario.p90_budget().is_none() {
        push(
            violations,
            "m5_recovery.latency_budget_missing_p90",
            sid,
            "scenario must declare a p90 first-actionable latency budget",
        );
    }
    if scenario.observed_p90_ms().is_none() {
        push(
            violations,
            "m5_recovery.observed_missing_p90",
            sid,
            "scenario must record an observed p90 first-actionable latency",
        );
    }
    for budget in &scenario.first_actionable_latency_budgets {
        let ok = budget.target_ms > 0
            && budget.yellow_ms > budget.target_ms
            && budget.red_ms > budget.yellow_ms;
        if !ok {
            push(
                violations,
                "m5_recovery.latency_budget_order",
                sid,
                format!(
                    "latency budget for {} must satisfy 0 < target < yellow < red",
                    budget.percentile.as_str()
                ),
            );
        }
    }
}

/// One validation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorM5RecoveryViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref (packet id or scenario id).
    pub subject_ref: String,
    /// Human-readable message.
    pub message: String,
}

/// One row of the metadata-safe support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorM5RecoveryExportRow {
    /// Scenario id.
    pub scenario_id: String,
    /// Lane.
    pub lane: RecoveryLane,
    /// Chosen recovery rung.
    pub recovery_rung: RecoveryRung,
    /// Drill outcome.
    pub drill_outcome: DrillOutcome,
    /// Promotion action.
    pub promotion_action: PromotionAction,
    /// Narrowing reason.
    pub narrowing_reason: NarrowingReason,
    /// Support-bundle manifest ref.
    pub bundle_manifest_ref: String,
    /// Escalation packet ref.
    pub escalation_packet_ref: String,
}

/// The metadata-safe support-export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorM5RecoveryExportProjection {
    /// Packet id.
    pub packet_id: String,
    /// Schema ref.
    pub schema_ref: String,
    /// One row per scenario.
    pub rows: Vec<ProjectDoctorM5RecoveryExportRow>,
    /// Count of full-readiness scenarios.
    pub full_readiness_count: usize,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

fn push(
    violations: &mut Vec<ProjectDoctorM5RecoveryViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ProjectDoctorM5RecoveryViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Parses the embedded checked-in packet.
///
/// Returns a JSON parse error when the checked-in packet no longer matches the
/// typed model.
pub fn current_project_doctor_m5_recovery_field_readiness(
) -> Result<ProjectDoctorM5RecoveryFieldReadiness, serde_json::Error> {
    serde_json::from_str(PROJECT_DOCTOR_M5_RECOVERY_JSON)
}

#[cfg(test)]
mod tests;
