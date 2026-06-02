//! Typed stable shiproom dashboard binding each shiproom panel to its canonical
//! upstream source, a packet-freshness SLO, measurable fitness functions, and the
//! qualification-row stop rules that hold promotion.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_qualification_matrix`](crate::stable_qualification_matrix) decides whether
//! each per-lane *qualification row* holds its claimed level; the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each launch-blocking
//! *requirement* is proven; and the
//! [`maintenance_control_packet`](crate::maintenance_control_packet) decides whether each
//! post-release maintenance *lane* is governed. None of them answer the question this
//! module answers: **for each panel the shiproom dashboard renders — claim truth,
//! qualification, public proof, or maintenance — is that panel green, backed by a fresh
//! packet, by fitness functions that all clear their thresholds, by the qualification
//! rows it watches still holding the cutline, and by an owner sign-off?** This module is
//! the **shiproom dashboard**. For every panel it records one row that binds the panel to
//! the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose lifecycle label
//! it backs, the upstream source it ingests, the qualification rows it watches, the
//! freshness packet that proves it is current, the fitness functions it must clear, the
//! waiver (if any) holding it provisionally, and the owner sign-off.
//!
//! Each [`DashboardPanel`] is one `(panel, public claim)` binding. It:
//!
//! - names the shiproom panel it renders ([`DashboardPanel::panel_kind`],
//!   [`DashboardPanel::panel_ref`], [`DashboardPanel::panel_summary`]) and whether that
//!   panel is part of the release-blocking dashboard ([`DashboardPanel::release_blocking`]);
//! - pins the upstream source it ingests ([`DashboardPanel::source_ref`]), the
//!   qualification rows it watches ([`DashboardPanel::qualification_row_refs`]), the
//!   freshness packet ([`ProofPacket`]) with its packet-freshness SLO, and the fitness
//!   functions ([`FitnessFunction`]) it must clear;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose public
//!   claim it backs ([`DashboardPanel::claim_ref`]) and the canonical lifecycle label that
//!   entry publishes ([`DashboardPanel::claim_label`]). That label is a hard **ceiling**: a
//!   panel may render at the claim's label or narrow below it, but it may never display a
//!   maturity wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-panel labels, so
//!   docs, Help/About, the release center, and support exports ingest one label per panel
//!   instead of cloning their own;
//! - records the panel state earned ([`PanelState`]), the active stop reasons
//!   ([`StopReason`]), and the label it *effectively* displays after narrowing
//!   ([`DashboardPanel::displayed_label`]).
//!
//! A [`FitnessFunction`] is a measurable architectural-fitness check: a metric, a
//! [`Comparator`], a threshold, an optional warn band, and a measured value, which together
//! compute a [`FitnessStatus`] of pass, warn, fail, or unmeasured. A panel may render green
//! only when every fitness function it carries is measured and clears its threshold (pass
//! or warn); a failing or unmeasured fitness function narrows the panel before the
//! dashboard shows green.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary between a
//! panel that renders green for a Stable public claim and one narrowed below it. A panel
//! that is not green — because its freshness packet aged out or is missing, because a
//! fitness function failed or is unmeasured, because a qualification row it watches
//! regressed below the cutline, because its waiver expired, because its panel evidence is
//! incomplete, or because the public claim it backs is itself below the cutline — is
//! structurally required to drop below the cutline rather than inherit an adjacent green
//! panel. The [`QualificationStopRule`] set names the closed conditions that gate
//! promotion, and [`ShiproomDashboard::publication`] records the proceed/hold verdict.
//!
//! The dashboard is checked in at `artifacts/release/shiproom_dashboard.json` and embedded
//! here, so this typed consumer and the CI gate agree on every panel without a cargo build
//! in CI.
//!
//! The model is metadata-only: every field is a typed state, an integer measurement, or an
//! opaque ref. It carries no raw artifacts, raw logs, signatures, or credential material.
//! Three classes of check live outside this model because they need more than the dashboard
//! sees: date arithmetic (recomputing the packet-freshness state and waiver expiry against
//! an `as_of` date), the claim-ceiling cross-check (whether each panel's `claim_label` still
//! equals the label the stable claim manifest publishes for the entry named by `claim_ref`),
//! and the qualification cross-check (whether each watched qualification row still holds the
//! cutline in the stable qualification matrix). Those live in the CI gate. This model
//! enforces the structural and logical invariants that hold regardless of the clock and the
//! neighbouring artifact — the ceiling/no-widening rule, fitness-status coherence,
//! narrowing consistency, packet/state coherence, owner sign-off on green panels, panel-kind
//! and release-line coverage, publication-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported shiproom-dashboard schema version.
pub const SHIPROOM_DASHBOARD_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the dashboard.
pub const SHIPROOM_DASHBOARD_RECORD_KIND: &str = "shiproom_dashboard";

/// Repo-relative path to the checked-in dashboard.
pub const SHIPROOM_DASHBOARD_PATH: &str = "artifacts/release/shiproom_dashboard.json";

/// Embedded checked-in dashboard JSON.
pub const SHIPROOM_DASHBOARD_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/shiproom_dashboard.json"
));

/// The shiproom-dashboard panel kind a row renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelKind {
    /// A claim-truth panel ingesting the stable claim manifest/matrix.
    ClaimTruth,
    /// A qualification panel ingesting the stable qualification matrix rows.
    Qualification,
    /// A public-proof panel ingesting the stable proof index.
    PublicProof,
    /// A maintenance panel ingesting the maintenance-control packet.
    Maintenance,
}

impl PanelKind {
    /// Every panel kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ClaimTruth,
        Self::Qualification,
        Self::PublicProof,
        Self::Maintenance,
    ];

    /// Stable token recorded in the dashboard.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimTruth => "claim_truth",
            Self::Qualification => "qualification",
            Self::PublicProof => "public_proof",
            Self::Maintenance => "maintenance",
        }
    }
}

/// How a fitness function compares its measured value to its threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Comparator {
    /// The measurement passes when it is at least the threshold (higher is better).
    AtLeast,
    /// The measurement passes when it is at most the threshold (lower is better).
    AtMost,
    /// The measurement passes only when it equals the threshold exactly.
    Equals,
}

impl Comparator {
    /// Every comparator, in declaration order.
    pub const ALL: [Self; 3] = [Self::AtLeast, Self::AtMost, Self::Equals];

    /// Stable token recorded in the dashboard.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtLeast => "at_least",
            Self::AtMost => "at_most",
            Self::Equals => "equals",
        }
    }

    /// True when `measured` satisfies `threshold` under this comparator.
    pub const fn satisfies(self, measured: i64, threshold: i64) -> bool {
        match self {
            Self::AtLeast => measured >= threshold,
            Self::AtMost => measured <= threshold,
            Self::Equals => measured == threshold,
        }
    }

    /// True when `measured` is within the comfort band defined by `warn_threshold`.
    ///
    /// For [`AtLeast`](Self::AtLeast) the comfort band is values at or above the warn
    /// threshold; for [`AtMost`](Self::AtMost) it is values at or below it; for
    /// [`Equals`](Self::Equals) the comfort band is the exact match itself.
    pub const fn within_comfort(self, measured: i64, warn_threshold: i64) -> bool {
        match self {
            Self::AtLeast => measured >= warn_threshold,
            Self::AtMost => measured <= warn_threshold,
            Self::Equals => true,
        }
    }
}

/// The status a fitness function earned for its measured value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FitnessStatus {
    /// The measurement clears the threshold and stays inside the comfort band.
    Pass,
    /// The measurement clears the threshold but sits inside the warn band.
    Warn,
    /// The measurement does not clear the threshold.
    Fail,
    /// No measurement has been captured.
    Unmeasured,
}

impl FitnessStatus {
    /// Every status, healthiest to unhealthiest.
    pub const ALL: [Self; 4] = [Self::Pass, Self::Warn, Self::Fail, Self::Unmeasured];

    /// Stable token recorded in the dashboard.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Warn => "warn",
            Self::Fail => "fail",
            Self::Unmeasured => "unmeasured",
        }
    }

    /// Whether the status lets a panel render green: the threshold is cleared (pass or
    /// warn). A failing or unmeasured fitness function narrows the panel.
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Pass | Self::Warn)
    }
}

/// Panel state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelState {
    /// The panel is green: a captured, within-SLO freshness packet, fitness functions
    /// that all clear, qualification rows that all hold the cutline, and an owner
    /// sign-off back the public claim at its full canonical lifecycle label.
    Green,
    /// The panel renders the claim's full label only because an active, unexpired waiver
    /// covers a recorded gap.
    GreenOnWaiver,
    /// The panel evidence is incomplete, a panel capability is absent, or owner sign-off
    /// is absent; the panel is not green and the label must narrow.
    NarrowedUnbacked,
    /// A fitness function failed/is unmeasured or a watched qualification row regressed
    /// below the cutline; the label must narrow.
    NarrowedRegressed,
    /// The public claim this panel backs is itself below the cutline, so the panel
    /// inherits that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The freshness packet breached its SLO (or is missing); the label must narrow.
    NarrowedStale,
    /// The panel relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
}

impl PanelState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Green,
        Self::GreenOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedRegressed,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the dashboard.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::GreenOnWaiver => "green_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedRegressed => "narrowed_regressed",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets a panel render the public claim at its label.
    pub const fn renders_green(self) -> bool {
        matches!(self, Self::Green | Self::GreenOnWaiver)
    }

    /// Whether the state forces the panel below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.renders_green()
    }
}

/// Closed reason a panel narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// The public claim this panel backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The panel names a dashboard capability the build does not yet implement.
    PanelCapabilityAbsent,
    /// A qualification row the panel watches has regressed below the cutline.
    QualificationRowRegressed,
    /// A fitness function the panel carries failed or is unmeasured.
    FitnessFunctionFailing,
    /// The panel's source/fitness evidence is incomplete.
    PanelEvidenceIncomplete,
    /// The freshness packet breached its SLO.
    FreshnessPacketBreached,
    /// No freshness packet has been captured for the panel.
    FreshnessPacketMissing,
    /// A waiver the panel relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl StopReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ClaimLabelNarrowed,
        Self::PanelCapabilityAbsent,
        Self::QualificationRowRegressed,
        Self::FitnessFunctionFailing,
        Self::PanelEvidenceIncomplete,
        Self::FreshnessPacketBreached,
        Self::FreshnessPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the dashboard.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::PanelCapabilityAbsent => "panel_capability_absent",
            Self::QualificationRowRegressed => "qualification_row_regressed",
            Self::FitnessFunctionFailing => "fitness_function_failing",
            Self::PanelEvidenceIncomplete => "panel_evidence_incomplete",
            Self::FreshnessPacketBreached => "freshness_packet_breached",
            Self::FreshnessPacketMissing => "freshness_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold stable promotion until the condition clears.
    HoldPromotion,
    /// Narrow the panel's displayed lifecycle label below the cutline.
    NarrowPanelLabel,
    /// Refresh the freshness packet so it re-enters its SLO.
    RefreshFreshnessPacket,
    /// Remediate the failing fitness function so it clears its threshold.
    RemediateFitnessFunction,
    /// Recapture the panel-level source/fitness evidence.
    RecapturePanelEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPromotion,
        Self::NarrowPanelLabel,
        Self::RefreshFreshnessPacket,
        Self::RemediateFitnessFunction,
        Self::RecapturePanelEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the dashboard.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowPanelLabel => "narrow_panel_label",
            Self::RefreshFreshnessPacket => "refresh_freshness_packet",
            Self::RemediateFitnessFunction => "remediate_fitness_function",
            Self::RecapturePanelEvidence => "recapture_panel_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One measurable fitness function: a metric, a comparator, a threshold, an optional warn
/// band, and a measured value, which together earn a [`FitnessStatus`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FitnessFunction {
    /// Stable fitness-function id.
    pub function_id: String,
    /// Human-readable title.
    pub title: String,
    /// Reviewable name of the metric being measured.
    pub metric: String,
    /// The unit the threshold and measurement are expressed in (e.g. `permille`,
    /// `count`, `hours`).
    pub unit: String,
    /// How the measurement is compared to the threshold.
    pub comparator: Comparator,
    /// The threshold the measurement must clear.
    pub threshold: i64,
    /// The warn-band boundary. When present, a measurement that clears the threshold but
    /// not this comfort boundary earns [`FitnessStatus::Warn`].
    #[serde(default)]
    pub warn_threshold: Option<i64>,
    /// The measured value, or null when no measurement has been captured.
    #[serde(default)]
    pub measured: Option<i64>,
    /// The fitness status earned.
    pub status: FitnessStatus,
    /// Ref to the measurement source that produced this value.
    pub measurement_ref: String,
}

impl FitnessFunction {
    /// Recomputes the fitness status from the comparator, threshold, warn band, and
    /// measured value.
    pub fn computed_status(&self) -> FitnessStatus {
        let Some(measured) = self.measured else {
            return FitnessStatus::Unmeasured;
        };
        if !self.comparator.satisfies(measured, self.threshold) {
            return FitnessStatus::Fail;
        }
        match self.warn_threshold {
            Some(warn) if !self.comparator.within_comfort(measured, warn) => FitnessStatus::Warn,
            _ => FitnessStatus::Pass,
        }
    }

    /// True when the warn band is consistent with the comparator: an `at_least` warn
    /// boundary may not sit below the threshold, an `at_most` warn boundary may not sit
    /// above it, and an `equals` function carries no warn boundary.
    pub fn warn_band_is_consistent(&self) -> bool {
        match (self.comparator, self.warn_threshold) {
            (Comparator::Equals, Some(_)) => false,
            (Comparator::AtLeast, Some(warn)) => warn >= self.threshold,
            (Comparator::AtMost, Some(warn)) => warn <= self.threshold,
            _ => true,
        }
    }

    /// True when the fitness function clears its threshold (pass or warn).
    pub fn is_satisfied(&self) -> bool {
        self.status.is_satisfied()
    }
}

/// One qualification-row stop rule: a closed condition that narrows a panel label and may
/// gate stable promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The stop reason whose presence on a watched panel fires this rule.
    pub trigger_reason: StopReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StopAction,
    /// Whether firing this rule blocks stable promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One shiproom-dashboard panel: a `(panel, public claim)` binding bound to its upstream
/// source, watched qualification rows, freshness packet, fitness functions, canonical
/// ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DashboardPanel {
    /// Stable panel id.
    pub panel_id: String,
    /// Human-readable title.
    pub title: String,
    /// The shiproom panel kind this row renders.
    pub panel_kind: PanelKind,
    /// The panel id/ref this row renders.
    pub panel_ref: String,
    /// Reviewable one-line statement of the panel.
    pub panel_summary: String,
    /// Whether the panel is part of the release-blocking dashboard.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this panel backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a panel may
    /// never render a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Panel state earned for the panel.
    pub panel_state: PanelState,
    /// Ref to the upstream canonical source this panel ingests.
    pub source_ref: String,
    /// The qualification-matrix row ids this panel watches.
    #[serde(default)]
    pub qualification_row_refs: Vec<String>,
    /// The freshness packet and its SLO.
    pub freshness_packet: ProofPacket,
    /// The fitness functions this panel must clear.
    #[serde(default)]
    pub fitness_functions: Vec<FitnessFunction>,
    /// Waiver authorizing a provisional green panel, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active stop reasons narrowing the panel.
    #[serde(default)]
    pub active_stop_reasons: Vec<StopReason>,
    /// The lifecycle label the panel effectively displays after narrowing.
    pub displayed_label: StableClaimLevel,
    /// Publication destinations that render this panel's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the panel carries this posture.
    pub rationale: String,
}

impl DashboardPanel {
    /// True when the displayed label is at or above the cutline.
    pub fn renders_stable(&self) -> bool {
        self.displayed_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the panel's state lets it render its claimed label.
    pub fn renders_green(&self) -> bool {
        self.panel_state.renders_green()
    }

    /// True when a stop reason is active on the panel.
    pub fn has_active_reason(&self, reason: StopReason) -> bool {
        self.active_stop_reasons.contains(&reason)
    }

    /// Whether every fitness function the panel carries clears its threshold.
    pub fn all_fitness_satisfied(&self) -> bool {
        self.fitness_functions
            .iter()
            .all(FitnessFunction::is_satisfied)
    }

    /// Whether any fitness function failed.
    pub fn any_fitness_failed(&self) -> bool {
        self.fitness_functions
            .iter()
            .any(|f| f.status == FitnessStatus::Fail)
    }

    /// Whether any fitness function is unmeasured.
    pub fn any_fitness_unmeasured(&self) -> bool {
        self.fitness_functions
            .iter()
            .any(|f| f.status == FitnessStatus::Unmeasured)
    }
}

/// The recorded publication verdict for the shiproom dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DashboardPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Stop-rule ids that block promotion, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Panel ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_panel_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShiproomDashboardSummary {
    /// Total number of panels.
    pub total_panels: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Panels rendering a label at or above the cutline.
    pub panels_green_stable: usize,
    /// Panels narrowed below the cutline.
    pub panels_narrowed_below_cutline: usize,
    /// Panels rendering green via an active waiver.
    pub panels_on_active_waiver: usize,
    /// Total release-blocking panels.
    pub release_blocking_total: usize,
    /// Release-blocking panels rendering a label at or above the cutline.
    pub release_blocking_green_stable: usize,
    /// Release-blocking panels narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Claim-truth panels.
    pub claim_truth_panels: usize,
    /// Qualification panels.
    pub qualification_panels: usize,
    /// Public-proof panels.
    pub public_proof_panels: usize,
    /// Maintenance panels.
    pub maintenance_panels: usize,
    /// Freshness packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Freshness packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Freshness packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Freshness packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total fitness functions across all panels.
    pub total_fitness_functions: usize,
    /// Fitness functions whose status is `pass`.
    pub fitness_pass: usize,
    /// Fitness functions whose status is `warn`.
    pub fitness_warn: usize,
    /// Fitness functions whose status is `fail`.
    pub fitness_fail: usize,
    /// Fitness functions whose status is `unmeasured`.
    pub fitness_unmeasured: usize,
    /// Total active stop reasons across all panels.
    pub total_active_stop_reasons: usize,
    /// Number of stop rules currently firing.
    pub stop_rules_firing: usize,
}

/// The typed shiproom dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShiproomDashboard {
    /// Dashboard schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable dashboard identifier.
    pub dashboard_id: String,
    /// Lifecycle status of this dashboard artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this dashboard ingests as its public-claim source
    /// and ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the stable qualification matrix the watched qualification rows resolve
    /// against.
    pub qualification_matrix_ref: String,
    /// Ref to the freshness-SLO register every freshness packet rides.
    pub freshness_slo_register_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed panel-kind vocabulary.
    pub panel_kinds: Vec<PanelKind>,
    /// Closed comparator vocabulary.
    pub comparators: Vec<Comparator>,
    /// Closed fitness-status vocabulary.
    pub fitness_statuses: Vec<FitnessStatus>,
    /// Closed panel-state vocabulary.
    pub panel_states: Vec<PanelState>,
    /// Closed stop-reason vocabulary.
    pub stop_reasons: Vec<StopReason>,
    /// Closed stop-action vocabulary.
    pub stop_actions: Vec<StopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking panel refs this dashboard must cover.
    pub release_blocking_panel_refs: Vec<String>,
    /// Qualification-row stop rules.
    pub stop_rules: Vec<QualificationStopRule>,
    /// Dashboard panels.
    pub panels: Vec<DashboardPanel>,
    /// Recorded publication verdict.
    pub publication: DashboardPublicationRecord,
    /// Summary counts.
    pub summary: ShiproomDashboardSummary,
}

impl ShiproomDashboard {
    /// Returns the panel registered for `panel_id`.
    pub fn panel(&self, panel_id: &str) -> Option<&DashboardPanel> {
        self.panels.iter().find(|panel| panel.panel_id == panel_id)
    }

    /// Returns the panels rendering a label at or above the cutline.
    pub fn panels_green_stable(&self) -> Vec<&DashboardPanel> {
        self.panels
            .iter()
            .filter(|panel| panel.renders_stable())
            .collect()
    }

    /// Returns the panels narrowed below the cutline.
    pub fn panels_narrowed(&self) -> Vec<&DashboardPanel> {
        self.panels
            .iter()
            .filter(|panel| !panel.renders_stable())
            .collect()
    }

    /// Returns the release-blocking panels.
    pub fn release_blocking_panels(&self) -> Vec<&DashboardPanel> {
        self.panels
            .iter()
            .filter(|panel| panel.release_blocking)
            .collect()
    }

    /// Returns the panels of one panel kind.
    pub fn panels_for_kind(&self, kind: PanelKind) -> Vec<&DashboardPanel> {
        self.panels
            .iter()
            .filter(|panel| panel.panel_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the dashboard covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for panel in &self.panels {
            set.insert(panel.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched panel carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &QualificationStopRule) -> bool {
        self.panels.iter().any(|panel| {
            rule.applies_to_labels.contains(&panel.claim_label)
                && panel.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the panels and stop rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block promotion and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Panel ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only panels whose public claim is at or above the cutline count: a panel whose
    /// claim is already canonically narrowed is not a *dashboard* blocker, it merely
    /// inherits the upstream ceiling.
    pub fn computed_blocking_panel_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<StopReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for panel in &self.panels {
            if panel.claim_holds_stable()
                && panel
                    .active_stop_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(panel.panel_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the panels and stop rules.
    pub fn computed_summary(&self) -> ShiproomDashboardSummary {
        let packets = |state: FreshnessSloState| {
            self.panels
                .iter()
                .filter(|panel| panel.freshness_packet.slo_state == state)
                .count()
        };
        let kind = |kind: PanelKind| self.panels_for_kind(kind).len();
        let fitness = |status: FitnessStatus| {
            self.panels
                .iter()
                .flat_map(|panel| &panel.fitness_functions)
                .filter(|f| f.status == status)
                .count()
        };
        let release_blocking: Vec<&DashboardPanel> = self
            .panels
            .iter()
            .filter(|panel| panel.release_blocking)
            .collect();
        ShiproomDashboardSummary {
            total_panels: self.panels.len(),
            total_claims: self.claims().len(),
            panels_green_stable: self
                .panels
                .iter()
                .filter(|panel| panel.renders_stable())
                .count(),
            panels_narrowed_below_cutline: self
                .panels
                .iter()
                .filter(|panel| !panel.renders_stable())
                .count(),
            panels_on_active_waiver: self
                .panels
                .iter()
                .filter(|panel| panel.panel_state == PanelState::GreenOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_green_stable: release_blocking
                .iter()
                .filter(|panel| panel.renders_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|panel| !panel.renders_stable())
                .count(),
            claim_truth_panels: kind(PanelKind::ClaimTruth),
            qualification_panels: kind(PanelKind::Qualification),
            public_proof_panels: kind(PanelKind::PublicProof),
            maintenance_panels: kind(PanelKind::Maintenance),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_fitness_functions: self
                .panels
                .iter()
                .map(|panel| panel.fitness_functions.len())
                .sum(),
            fitness_pass: fitness(FitnessStatus::Pass),
            fitness_warn: fitness(FitnessStatus::Warn),
            fitness_fail: fitness(FitnessStatus::Fail),
            fitness_unmeasured: fitness(FitnessStatus::Unmeasured),
            total_active_stop_reasons: self
                .panels
                .iter()
                .map(|panel| panel.active_stop_reasons.len())
                .sum(),
            stop_rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the dashboard that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> DashboardExportProjection {
        DashboardExportProjection {
            dashboard_id: self.dashboard_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            panels: self
                .panels
                .iter()
                .map(|panel| DashboardExportRow {
                    panel_id: panel.panel_id.clone(),
                    panel_kind: panel.panel_kind,
                    panel_ref: panel.panel_ref.clone(),
                    release_blocking: panel.release_blocking,
                    claim_ref: panel.claim_ref.clone(),
                    claim_label: panel.claim_label,
                    displayed_label: panel.displayed_label,
                    renders_stable: panel.renders_stable(),
                    panel_state: panel.panel_state,
                    slo_state: panel.freshness_packet.slo_state,
                    fitness_total: panel.fitness_functions.len(),
                    fitness_failing: panel
                        .fitness_functions
                        .iter()
                        .filter(|f| !f.is_satisfied())
                        .count(),
                    active_stop_reasons: panel.active_stop_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the dashboard, returning every violation found.
    pub fn validate(&self) -> Vec<ShiproomDashboardViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for panel in &self.panels {
            if !seen.insert(panel.panel_id.clone()) {
                violations.push(ShiproomDashboardViolation::DuplicatePanelId {
                    panel_id: panel.panel_id.clone(),
                });
            }
            self.validate_panel(panel, &mut violations);
        }
        if self.panels.is_empty() {
            violations.push(ShiproomDashboardViolation::EmptyDashboard);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(ShiproomDashboardViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ShiproomDashboardViolation>) {
        if self.schema_version != SHIPROOM_DASHBOARD_SCHEMA_VERSION {
            violations.push(ShiproomDashboardViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != SHIPROOM_DASHBOARD_RECORD_KIND {
            violations.push(ShiproomDashboardViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("dashboard_id", &self.dashboard_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("qualification_matrix_ref", &self.qualification_matrix_ref),
            (
                "freshness_slo_register_ref",
                &self.freshness_slo_register_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(ShiproomDashboardViolation::EmptyField {
                    panel_id: "<dashboard>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.panel_kinds != PanelKind::ALL.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "panel_kinds",
            });
        }
        if self.comparators != Comparator::ALL.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "comparators",
            });
        }
        if self.fitness_statuses != FitnessStatus::ALL.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "fitness_statuses",
            });
        }
        if self.panel_states != PanelState::ALL.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "panel_states",
            });
        }
        if self.stop_reasons != StopReason::ALL.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "stop_reasons",
            });
        }
        if self.stop_actions != StopAction::ALL.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "stop_actions",
            });
        }
        if self.release_blocking_panel_refs.is_empty() {
            violations.push(ShiproomDashboardViolation::EmptyField {
                panel_id: "<dashboard>".to_owned(),
                field_name: "release_blocking_panel_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(ShiproomDashboardViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(ShiproomDashboardViolation::EmptyField {
                panel_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<ShiproomDashboardViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(ShiproomDashboardViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(ShiproomDashboardViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(ShiproomDashboardViolation::EmptyField {
                        panel_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(ShiproomDashboardViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every stop reason must have a rule, so a stop reason cannot fire without a
        // corresponding promotion gate.
        for reason in StopReason::ALL {
            if !covered.contains(&reason) {
                violations.push(ShiproomDashboardViolation::StopReasonWithoutRule { reason });
            }
        }
    }

    fn validate_panel(
        &self,
        panel: &DashboardPanel,
        violations: &mut Vec<ShiproomDashboardViolation>,
    ) {
        for (field, value) in [
            ("panel_id", &panel.panel_id),
            ("title", &panel.title),
            ("panel_ref", &panel.panel_ref),
            ("panel_summary", &panel.panel_summary),
            ("claim_ref", &panel.claim_ref),
            ("source_ref", &panel.source_ref),
            ("rationale", &panel.rationale),
            (
                "freshness_packet.packet_id",
                &panel.freshness_packet.packet_id,
            ),
            (
                "freshness_packet.packet_ref",
                &panel.freshness_packet.packet_ref,
            ),
            (
                "freshness_packet.proof_index_ref",
                &panel.freshness_packet.proof_index_ref,
            ),
            (
                "freshness_packet.freshness_slo.slo_register_ref",
                &panel.freshness_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &panel.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(ShiproomDashboardViolation::EmptyField {
                    panel_id: panel.panel_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no panel may render a label wider than the public claim's
        // canonical label.
        if panel.displayed_label.rank() > panel.claim_label.rank() {
            violations.push(ShiproomDashboardViolation::DisplayedWiderThanClaim {
                panel_id: panel.panel_id.clone(),
                claim: panel.claim_label,
                displayed: panel.displayed_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn window
        // may not exceed it.
        if panel.freshness_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(ShiproomDashboardViolation::EmptyField {
                panel_id: panel.panel_id.clone(),
                field_name: "freshness_packet.freshness_slo.target_max_age_days",
            });
        }
        if !panel.freshness_packet.freshness_slo.window_is_consistent() {
            violations.push(ShiproomDashboardViolation::FreshnessSloInconsistent {
                panel_id: panel.panel_id.clone(),
            });
        }

        self.validate_fitness(panel, violations);

        // A public claim whose canonical label is below the cutline forces the panel to
        // inherit that ceiling and narrow.
        if !panel.claim_holds_stable() {
            if panel.renders_green() {
                violations.push(ShiproomDashboardViolation::GreenOnNarrowedClaim {
                    panel_id: panel.panel_id.clone(),
                    claim: panel.claim_label,
                });
            }
            if !panel.has_active_reason(StopReason::ClaimLabelNarrowed) {
                violations.push(ShiproomDashboardViolation::ClaimNarrowedWithoutReason {
                    panel_id: panel.panel_id.clone(),
                });
            }
        }

        let slo_state = panel.freshness_packet.slo_state;

        if panel.renders_green() {
            // A green panel renders exactly the public claim's canonical label, carries no
            // active stop reason, rides a captured within-SLO packet, has every fitness
            // function satisfied, and is owner-signed.
            if panel.displayed_label != panel.claim_label {
                violations.push(ShiproomDashboardViolation::GreenLabelNotEqualClaim {
                    panel_id: panel.panel_id.clone(),
                    claim: panel.claim_label,
                    displayed: panel.displayed_label,
                });
            }
            if !panel.active_stop_reasons.is_empty() {
                violations.push(ShiproomDashboardViolation::GreenWithActiveStop {
                    panel_id: panel.panel_id.clone(),
                });
            }
            if !panel.freshness_packet.has_capture() {
                violations.push(ShiproomDashboardViolation::GreenWithoutFreshPacket {
                    panel_id: panel.panel_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(ShiproomDashboardViolation::GreenOnStalePacket {
                    panel_id: panel.panel_id.clone(),
                    slo_state,
                });
            }
            if !panel.all_fitness_satisfied() {
                violations.push(ShiproomDashboardViolation::GreenWithFailingFitness {
                    panel_id: panel.panel_id.clone(),
                });
            }
            if !(panel.owner_signoff.signed_off && panel.owner_signoff.signed_at.is_some()) {
                violations.push(ShiproomDashboardViolation::GreenWithoutSignoff {
                    panel_id: panel.panel_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the displayed label below the cutline and name
            // at least one active reason.
            if panel.renders_stable() {
                violations.push(ShiproomDashboardViolation::DisplayedLabelNotNarrowed {
                    panel_id: panel.panel_id.clone(),
                    state: panel.panel_state,
                    displayed: panel.displayed_label,
                });
            }
            if panel.active_stop_reasons.is_empty() {
                violations.push(ShiproomDashboardViolation::NarrowingWithoutReason {
                    panel_id: panel.panel_id.clone(),
                    state: panel.panel_state,
                });
            }
            // A narrowing panel whose packet is breached or missing must name the matching
            // freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !panel.has_active_reason(StopReason::FreshnessPacketBreached)
            {
                violations.push(ShiproomDashboardViolation::BreachedPacketWithoutReason {
                    panel_id: panel.panel_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !panel.has_active_reason(StopReason::FreshnessPacketMissing)
            {
                violations.push(ShiproomDashboardViolation::MissingPacketWithoutReason {
                    panel_id: panel.panel_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(panel, violations);
    }

    fn validate_fitness(
        &self,
        panel: &DashboardPanel,
        violations: &mut Vec<ShiproomDashboardViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for function in &panel.fitness_functions {
            if !seen.insert(function.function_id.clone()) {
                violations.push(ShiproomDashboardViolation::DuplicateFunctionId {
                    panel_id: panel.panel_id.clone(),
                    function_id: function.function_id.clone(),
                });
            }
            for (field, value) in [
                ("function_id", &function.function_id),
                ("title", &function.title),
                ("metric", &function.metric),
                ("unit", &function.unit),
                ("measurement_ref", &function.measurement_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(ShiproomDashboardViolation::EmptyField {
                        panel_id: panel.panel_id.clone(),
                        field_name: field,
                    });
                }
            }
            // The recorded fitness status must equal the status the comparator, threshold,
            // warn band, and measurement compute.
            if function.status != function.computed_status() {
                violations.push(ShiproomDashboardViolation::FitnessStatusInconsistent {
                    panel_id: panel.panel_id.clone(),
                    function_id: function.function_id.clone(),
                    declared: function.status,
                    computed: function.computed_status(),
                });
            }
            if !function.warn_band_is_consistent() {
                violations.push(ShiproomDashboardViolation::FitnessWarnBandInconsistent {
                    panel_id: panel.panel_id.clone(),
                    function_id: function.function_id.clone(),
                });
            }
        }

        // A failing fitness function forces a narrowing panel that names the failing
        // reason; an unmeasured one names the evidence-incomplete reason.
        if panel.any_fitness_failed()
            && !panel.has_active_reason(StopReason::FitnessFunctionFailing)
        {
            violations.push(ShiproomDashboardViolation::FailingFitnessWithoutReason {
                panel_id: panel.panel_id.clone(),
            });
        }
        if panel.any_fitness_unmeasured()
            && !panel.has_active_reason(StopReason::PanelEvidenceIncomplete)
        {
            violations.push(ShiproomDashboardViolation::UnmeasuredFitnessWithoutReason {
                panel_id: panel.panel_id.clone(),
            });
        }
    }

    fn validate_state_reason_coherence(
        &self,
        panel: &DashboardPanel,
        violations: &mut Vec<ShiproomDashboardViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<ShiproomDashboardViolation>,
                               expected: StopReason| {
            violations.push(ShiproomDashboardViolation::StateReasonIncoherent {
                panel_id: panel.panel_id.clone(),
                state: panel.panel_state,
                expected_reason: expected,
            });
        };

        match panel.panel_state {
            PanelState::NarrowedUnbacked => {
                const ALLOWED: [StopReason; 3] = [
                    StopReason::PanelCapabilityAbsent,
                    StopReason::PanelEvidenceIncomplete,
                    StopReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| panel.has_active_reason(*r)) {
                    push_incoherent(violations, StopReason::PanelEvidenceIncomplete);
                }
            }
            PanelState::NarrowedRegressed => {
                if !(panel.has_active_reason(StopReason::QualificationRowRegressed)
                    || panel.has_active_reason(StopReason::FitnessFunctionFailing))
                {
                    push_incoherent(violations, StopReason::QualificationRowRegressed);
                }
            }
            PanelState::NarrowedClaimNarrowed => {
                if !panel.has_active_reason(StopReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, StopReason::ClaimLabelNarrowed);
                }
            }
            PanelState::NarrowedStale => {
                if !(panel.has_active_reason(StopReason::FreshnessPacketBreached)
                    || panel.has_active_reason(StopReason::FreshnessPacketMissing))
                {
                    push_incoherent(violations, StopReason::FreshnessPacketBreached);
                }
            }
            PanelState::NarrowedWaiverExpired => {
                if !panel.has_active_reason(StopReason::WaiverExpired) {
                    push_incoherent(violations, StopReason::WaiverExpired);
                }
                if panel.waiver.is_none() {
                    violations.push(ShiproomDashboardViolation::WaiverStateWithoutWaiver {
                        panel_id: panel.panel_id.clone(),
                        state: panel.panel_state,
                    });
                }
            }
            PanelState::GreenOnWaiver => {
                if panel
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(ShiproomDashboardViolation::WaiverStateWithoutWaiver {
                        panel_id: panel.panel_id.clone(),
                        state: panel.panel_state,
                    });
                }
            }
            PanelState::Green => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<ShiproomDashboardViolation>) {
        // Each panel ref appears at most once: a panel has one canonical dashboard row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for panel in &self.panels {
            if !seen.insert(panel.panel_ref.as_str()) {
                violations.push(ShiproomDashboardViolation::DuplicatePanelRef {
                    panel_ref: panel.panel_ref.clone(),
                });
            }
        }

        // The release line must render every declared release-blocking panel with exactly
        // one release-blocking row, and every release-blocking row must be declared, so a
        // panel cannot quietly drop out of the dashboard.
        let declared: BTreeSet<&str> = self
            .release_blocking_panel_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .panels
            .iter()
            .filter(|panel| panel.release_blocking)
            .map(|panel| panel.panel_ref.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(ShiproomDashboardViolation::ReleaseBlockingRefWithoutPanel {
                    panel_ref: (*declared_ref).to_owned(),
                });
            }
        }
        for panel in &self.panels {
            if panel.release_blocking && !declared.contains(panel.panel_ref.as_str()) {
                violations.push(ShiproomDashboardViolation::ReleaseBlockingPanelNotInSet {
                    panel_id: panel.panel_id.clone(),
                    panel_ref: panel.panel_ref.clone(),
                });
            }
        }

        // The dashboard must cover all four panel kinds — claim truth, qualification,
        // public proof, and maintenance — so the release line cannot render some panels
        // and silently leave a whole shiproom lane dark.
        for kind in PanelKind::ALL {
            if self.panels_for_kind(kind).is_empty() {
                violations.push(ShiproomDashboardViolation::PanelKindAbsent { kind });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<ShiproomDashboardViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(ShiproomDashboardViolation::EmptyField {
                panel_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(ShiproomDashboardViolation::EmptyField {
                panel_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                ShiproomDashboardViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(ShiproomDashboardViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_panel_ids != self.computed_blocking_panel_ids() {
            violations.push(ShiproomDashboardViolation::PublicationBlockingSetMismatch {
                field: "blocking_panel_ids",
            });
        }
    }
}

/// A redaction-safe export row projected from the dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardExportRow {
    /// Stable panel id.
    pub panel_id: String,
    /// Panel kind.
    pub panel_kind: PanelKind,
    /// Panel ref.
    pub panel_ref: String,
    /// Whether the panel is part of the release-blocking dashboard.
    pub release_blocking: bool,
    /// The public-claim entry ref the panel backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the panel displays.
    pub displayed_label: StableClaimLevel,
    /// Whether the panel renders a label at or above the cutline.
    pub renders_stable: bool,
    /// Panel state.
    pub panel_state: PanelState,
    /// Freshness-packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Total fitness functions on the panel.
    pub fitness_total: usize,
    /// Fitness functions that did not clear their threshold (fail or unmeasured).
    pub fitness_failing: usize,
    /// Active stop reasons.
    pub active_stop_reasons: Vec<StopReason>,
}

/// A redaction-safe export projection of the dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardExportProjection {
    /// Dashboard id this projection was produced from.
    pub dashboard_id: String,
    /// Dashboard as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected panels.
    pub panels: Vec<DashboardExportRow>,
}

/// A validation violation for the shiproom dashboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiproomDashboardViolation {
    /// The dashboard carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the dashboard.
        actual: u32,
    },
    /// The dashboard carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the dashboard.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The dashboard has no panels.
    EmptyDashboard,
    /// The dashboard has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Panel, rule, or section id.
        panel_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A panel id appears more than once.
    DuplicatePanelId {
        /// Duplicate panel id.
        panel_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A fitness-function id appears more than once on a panel.
    DuplicateFunctionId {
        /// Panel id.
        panel_id: String,
        /// Duplicate function id.
        function_id: String,
    },
    /// A stop rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A stop reason has no rule watching for it.
    StopReasonWithoutRule {
        /// Uncovered reason.
        reason: StopReason,
    },
    /// A displayed label is wider than the public claim's canonical label.
    DisplayedWiderThanClaim {
        /// Panel id.
        panel_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Displayed label.
        displayed: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Panel id.
        panel_id: String,
    },
    /// A fitness function's declared status disagrees with its computed status.
    FitnessStatusInconsistent {
        /// Panel id.
        panel_id: String,
        /// Function id.
        function_id: String,
        /// Declared status.
        declared: FitnessStatus,
        /// Computed status.
        computed: FitnessStatus,
    },
    /// A fitness function's warn band is inconsistent with its comparator.
    FitnessWarnBandInconsistent {
        /// Panel id.
        panel_id: String,
        /// Function id.
        function_id: String,
    },
    /// A panel with a failing fitness function does not name the failing reason.
    FailingFitnessWithoutReason {
        /// Panel id.
        panel_id: String,
    },
    /// A panel with an unmeasured fitness function does not name the evidence reason.
    UnmeasuredFitnessWithoutReason {
        /// Panel id.
        panel_id: String,
    },
    /// A panel renders green while the public claim's canonical label is narrowed.
    GreenOnNarrowedClaim {
        /// Panel id.
        panel_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
    },
    /// A panel whose claim is narrowed does not carry the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Panel id.
        panel_id: String,
    },
    /// A narrowing state did not drop the displayed label below the cutline.
    DisplayedLabelNotNarrowed {
        /// Panel id.
        panel_id: String,
        /// Panel state.
        state: PanelState,
        /// Displayed label.
        displayed: StableClaimLevel,
    },
    /// A narrowing state carries no active stop reason.
    NarrowingWithoutReason {
        /// Panel id.
        panel_id: String,
        /// Panel state.
        state: PanelState,
    },
    /// A green panel's displayed label is not equal to its claim ceiling label.
    GreenLabelNotEqualClaim {
        /// Panel id.
        panel_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Displayed label.
        displayed: StableClaimLevel,
    },
    /// A green panel carries an active stop reason.
    GreenWithActiveStop {
        /// Panel id.
        panel_id: String,
    },
    /// A green panel rides a freshness packet with no capture or evidence.
    GreenWithoutFreshPacket {
        /// Panel id.
        panel_id: String,
    },
    /// A green panel rides a freshness packet outside its SLO.
    GreenOnStalePacket {
        /// Panel id.
        panel_id: String,
        /// The packet's freshness-SLO state.
        slo_state: FreshnessSloState,
    },
    /// A green panel carries a failing or unmeasured fitness function.
    GreenWithFailingFitness {
        /// Panel id.
        panel_id: String,
    },
    /// A green panel has no owner sign-off.
    GreenWithoutSignoff {
        /// Panel id.
        panel_id: String,
    },
    /// A narrowing panel with a breached packet does not name the breach reason.
    BreachedPacketWithoutReason {
        /// Panel id.
        panel_id: String,
    },
    /// A narrowing panel with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Panel id.
        panel_id: String,
    },
    /// A panel state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Panel id.
        panel_id: String,
        /// Panel state.
        state: PanelState,
        /// Reason the state requires.
        expected_reason: StopReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Panel id.
        panel_id: String,
        /// Panel state.
        state: PanelState,
    },
    /// A panel ref appears on more than one row.
    DuplicatePanelRef {
        /// Duplicate panel ref.
        panel_ref: String,
    },
    /// A declared release-blocking panel ref has no covering row.
    ReleaseBlockingRefWithoutPanel {
        /// Uncovered panel ref.
        panel_ref: String,
    },
    /// A release-blocking row's panel ref is not in the declared set.
    ReleaseBlockingPanelNotInSet {
        /// Panel id.
        panel_id: String,
        /// The row's panel ref.
        panel_ref: String,
    },
    /// A panel kind is not covered by any row.
    PanelKindAbsent {
        /// The uncovered panel kind.
        kind: PanelKind,
    },
    /// The declared publication decision disagrees with the computed one.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared publication blocking set disagrees with the computed one.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the panels.
    SummaryMismatch,
}

impl fmt::Display for ShiproomDashboardViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported dashboard schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported dashboard record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "dashboard {field} is not the canonical value")
            }
            Self::EmptyDashboard => write!(f, "dashboard has no panels"),
            Self::NoStopRules => write!(f, "dashboard has no stop rules"),
            Self::EmptyField {
                panel_id,
                field_name,
            } => write!(f, "{panel_id} has empty field {field_name}"),
            Self::DuplicatePanelId { panel_id } => {
                write!(f, "duplicate panel id {panel_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::DuplicateFunctionId {
                panel_id,
                function_id,
            } => write!(f, "panel {panel_id} has duplicate fitness function id {function_id}"),
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::StopReasonWithoutRule { reason } => write!(
                f,
                "stop reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::DisplayedWiderThanClaim {
                panel_id,
                claim,
                displayed,
            } => write!(
                f,
                "panel {panel_id} displayed label {} is wider than the claim ceiling {}",
                displayed.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { panel_id } => write!(
                f,
                "panel {panel_id} freshness SLO warn window exceeds its target age"
            ),
            Self::FitnessStatusInconsistent {
                panel_id,
                function_id,
                declared,
                computed,
            } => write!(
                f,
                "panel {panel_id} fitness function {function_id} status {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::FitnessWarnBandInconsistent {
                panel_id,
                function_id,
            } => write!(
                f,
                "panel {panel_id} fitness function {function_id} has a warn band inconsistent with its comparator"
            ),
            Self::FailingFitnessWithoutReason { panel_id } => write!(
                f,
                "panel {panel_id} has a failing fitness function but does not name fitness_function_failing"
            ),
            Self::UnmeasuredFitnessWithoutReason { panel_id } => write!(
                f,
                "panel {panel_id} has an unmeasured fitness function but does not name panel_evidence_incomplete"
            ),
            Self::GreenOnNarrowedClaim { panel_id, claim } => write!(
                f,
                "panel {panel_id} renders green while the public claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { panel_id } => write!(
                f,
                "panel {panel_id} backs a claim that is narrowed but does not name claim_label_narrowed"
            ),
            Self::DisplayedLabelNotNarrowed {
                panel_id,
                state,
                displayed,
            } => write!(
                f,
                "panel {panel_id} state {} must narrow below the cutline but displays {}",
                state.as_str(),
                displayed.as_str()
            ),
            Self::NarrowingWithoutReason { panel_id, state } => write!(
                f,
                "panel {panel_id} state {} narrows without naming an active stop reason",
                state.as_str()
            ),
            Self::GreenLabelNotEqualClaim {
                panel_id,
                claim,
                displayed,
            } => write!(
                f,
                "panel {panel_id} displays {} but its public claim label is {}",
                displayed.as_str(),
                claim.as_str()
            ),
            Self::GreenWithActiveStop { panel_id } => write!(
                f,
                "panel {panel_id} renders green while a stop reason is active"
            ),
            Self::GreenWithoutFreshPacket { panel_id } => write!(
                f,
                "panel {panel_id} renders green with no captured, evidence-backed freshness packet"
            ),
            Self::GreenOnStalePacket {
                panel_id,
                slo_state,
            } => write!(
                f,
                "panel {panel_id} renders green while its packet is {} (outside its freshness SLO)",
                slo_state.as_str()
            ),
            Self::GreenWithFailingFitness { panel_id } => write!(
                f,
                "panel {panel_id} renders green while a fitness function fails or is unmeasured"
            ),
            Self::GreenWithoutSignoff { panel_id } => {
                write!(f, "panel {panel_id} renders green without owner sign-off")
            }
            Self::BreachedPacketWithoutReason { panel_id } => write!(
                f,
                "panel {panel_id} has a breached packet but does not name freshness_packet_breached"
            ),
            Self::MissingPacketWithoutReason { panel_id } => write!(
                f,
                "panel {panel_id} has a missing packet but does not name freshness_packet_missing"
            ),
            Self::StateReasonIncoherent {
                panel_id,
                state,
                expected_reason,
            } => write!(
                f,
                "panel {panel_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { panel_id, state } => write!(
                f,
                "panel {panel_id} state {} names no waiver",
                state.as_str()
            ),
            Self::DuplicatePanelRef { panel_ref } => {
                write!(f, "duplicate panel ref {panel_ref}")
            }
            Self::ReleaseBlockingRefWithoutPanel { panel_ref } => write!(
                f,
                "declared release-blocking panel {panel_ref} has no covering row"
            ),
            Self::ReleaseBlockingPanelNotInSet {
                panel_id,
                panel_ref,
            } => write!(
                f,
                "panel {panel_id} is release-blocking but its panel {panel_ref} is not in release_blocking_panel_refs"
            ),
            Self::PanelKindAbsent { kind } => write!(
                f,
                "panel kind {} is not covered by any dashboard row",
                kind.as_str()
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with the firing rules")
            }
            Self::SummaryMismatch => write!(f, "dashboard summary counts disagree with the panels"),
        }
    }
}

impl Error for ShiproomDashboardViolation {}

/// Loads the embedded shiproom dashboard.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in dashboard no longer matches
/// [`ShiproomDashboard`] — including when a panel carries a lifecycle label, panel kind,
/// comparator, fitness status, panel state, freshness-SLO state, stop reason, or stop
/// action outside the closed vocabularies.
pub fn current_shiproom_dashboard() -> Result<ShiproomDashboard, serde_json::Error> {
    serde_json::from_str(SHIPROOM_DASHBOARD_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dashboard() -> ShiproomDashboard {
        current_shiproom_dashboard().expect("dashboard parses")
    }

    #[test]
    fn embedded_dashboard_parses_and_validates() {
        let dashboard = dashboard();
        assert_eq!(dashboard.schema_version, SHIPROOM_DASHBOARD_SCHEMA_VERSION);
        assert_eq!(dashboard.record_kind, SHIPROOM_DASHBOARD_RECORD_KIND);
        assert_eq!(dashboard.validate(), Vec::new());
        assert!(!dashboard.panels.is_empty());
    }

    #[test]
    fn every_panel_kind_is_covered() {
        let dashboard = dashboard();
        for kind in PanelKind::ALL {
            assert!(
                !dashboard.panels_for_kind(kind).is_empty(),
                "panel kind {} must have at least one dashboard row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_release_blocking_panel_is_covered() {
        let dashboard = dashboard();
        let covered: BTreeSet<&str> = dashboard
            .panels
            .iter()
            .filter(|panel| panel.release_blocking)
            .map(|panel| panel.panel_ref.as_str())
            .collect();
        assert!(!dashboard.release_blocking_panel_refs.is_empty());
        for declared in &dashboard.release_blocking_panel_refs {
            assert!(
                covered.contains(declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn dashboard_exercises_green_and_narrowed_panels() {
        let dashboard = dashboard();
        assert!(
            !dashboard.panels_green_stable().is_empty(),
            "dashboard must show at least one green-stable panel"
        );
        assert!(
            !dashboard.panels_narrowed().is_empty(),
            "dashboard must show at least one narrowed panel"
        );
    }

    #[test]
    fn summary_counts_match_panels() {
        let dashboard = dashboard();
        assert_eq!(dashboard.summary, dashboard.computed_summary());
        assert_eq!(
            dashboard.summary.panels_green_stable + dashboard.summary.panels_narrowed_below_cutline,
            dashboard.panels.len()
        );
        assert_eq!(
            dashboard.summary.packets_current
                + dashboard.summary.packets_due_for_refresh
                + dashboard.summary.packets_breached
                + dashboard.summary.packets_missing,
            dashboard.panels.len()
        );
        assert_eq!(
            dashboard.summary.claim_truth_panels
                + dashboard.summary.qualification_panels
                + dashboard.summary.public_proof_panels
                + dashboard.summary.maintenance_panels,
            dashboard.panels.len()
        );
        assert_eq!(
            dashboard.summary.fitness_pass
                + dashboard.summary.fitness_warn
                + dashboard.summary.fitness_fail
                + dashboard.summary.fitness_unmeasured,
            dashboard.summary.total_fitness_functions
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let dashboard = dashboard();
        assert_eq!(dashboard.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            dashboard.publication.decision,
            dashboard.computed_publication_decision()
        );
        assert!(!dashboard.publication.blocking_rule_ids.is_empty());
        assert!(!dashboard.publication.blocking_panel_ids.is_empty());
    }

    #[test]
    fn every_stop_reason_has_a_rule() {
        let dashboard = dashboard();
        let covered: BTreeSet<StopReason> = dashboard
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in StopReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_panel_renders_wider_than_its_claim_ceiling() {
        let dashboard = dashboard();
        for panel in &dashboard.panels {
            assert!(
                panel.displayed_label.rank() <= panel.claim_label.rank(),
                "{} renders wider than its ceiling",
                panel.panel_id
            );
        }
    }

    #[test]
    fn fitness_status_matches_computation() {
        let dashboard = dashboard();
        for panel in &dashboard.panels {
            for function in &panel.fitness_functions {
                assert_eq!(
                    function.status,
                    function.computed_status(),
                    "{}/{} status disagrees with computation",
                    panel.panel_id,
                    function.function_id
                );
                assert!(
                    function.warn_band_is_consistent(),
                    "{}/{} warn band is inconsistent",
                    panel.panel_id,
                    function.function_id
                );
            }
        }
    }

    #[test]
    fn dashboard_has_a_fail_and_an_unmeasured_fitness_function() {
        let dashboard = dashboard();
        assert!(
            dashboard
                .panels
                .iter()
                .any(DashboardPanel::any_fitness_failed),
            "dashboard must exercise a failing fitness function"
        );
        assert!(
            dashboard
                .panels
                .iter()
                .any(DashboardPanel::any_fitness_unmeasured),
            "dashboard must exercise an unmeasured fitness function"
        );
    }

    #[test]
    fn validate_flags_a_panel_wider_than_ceiling() {
        let mut dashboard = dashboard();
        let panel = dashboard
            .panels
            .iter_mut()
            .find(|panel| !panel.renders_stable() && panel.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed panel under a beta ceiling exists");
        panel.displayed_label = StableClaimLevel::Stable;
        let panel_id = panel.panel_id.clone();
        dashboard.summary = dashboard.computed_summary();
        assert!(dashboard.validate().iter().any(|v| matches!(
            v,
            ShiproomDashboardViolation::DisplayedWiderThanClaim { panel_id: id, .. } if *id == panel_id
        )));
    }

    #[test]
    fn validate_flags_a_fitness_status_inconsistency() {
        let mut dashboard = dashboard();
        let panel = dashboard
            .panels
            .iter_mut()
            .find(|panel| !panel.fitness_functions.is_empty())
            .expect("a panel with a fitness function exists");
        let function = panel
            .fitness_functions
            .iter_mut()
            .find(|f| f.status == FitnessStatus::Pass)
            .expect("a passing fitness function exists");
        function.status = FitnessStatus::Fail;
        assert!(dashboard.validate().iter().any(|v| matches!(
            v,
            ShiproomDashboardViolation::FitnessStatusInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut dashboard = dashboard();
        let panel = dashboard
            .panels
            .iter_mut()
            .find(|panel| panel.panel_state == PanelState::NarrowedStale)
            .expect("a narrowed-stale panel exists");
        panel.displayed_label = panel.claim_label;
        dashboard.summary = dashboard.computed_summary();
        dashboard.publication.decision = dashboard.computed_publication_decision();
        dashboard.publication.blocking_rule_ids = dashboard.computed_blocking_rule_ids();
        dashboard.publication.blocking_panel_ids = dashboard.computed_blocking_panel_ids();
        assert!(dashboard.validate().iter().any(|v| matches!(
            v,
            ShiproomDashboardViolation::DisplayedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut dashboard = dashboard();
        dashboard.publication.decision = PromotionDecision::Proceed;
        assert!(dashboard.validate().iter().any(|v| matches!(
            v,
            ShiproomDashboardViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_green_panel_without_signoff() {
        let mut dashboard = dashboard();
        let panel = dashboard
            .panels
            .iter_mut()
            .find(|panel| panel.renders_green())
            .expect("a green panel exists");
        panel.owner_signoff.signed_off = false;
        panel.owner_signoff.signed_at = None;
        let panel_id = panel.panel_id.clone();
        dashboard.summary = dashboard.computed_summary();
        assert!(dashboard
            .validate()
            .contains(&ShiproomDashboardViolation::GreenWithoutSignoff { panel_id }));
    }

    #[test]
    fn export_projection_mirrors_panels() {
        let dashboard = dashboard();
        let projection = dashboard.support_export_projection();
        assert_eq!(projection.panels.len(), dashboard.panels.len());
        assert_eq!(
            projection.publication_decision,
            dashboard.publication.decision
        );
        for (panel, projected) in dashboard.panels.iter().zip(&projection.panels) {
            assert_eq!(panel.panel_id, projected.panel_id);
            assert_eq!(panel.panel_ref, projected.panel_ref);
            assert_eq!(panel.renders_stable(), projected.renders_stable);
            assert_eq!(panel.displayed_label, projected.displayed_label);
            assert_eq!(panel.freshness_packet.slo_state, projected.slo_state);
        }
    }
}
