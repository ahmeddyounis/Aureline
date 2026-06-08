//! Typed stable cohort scoreboards for design-partner, certified-archetype, and
//! stable-cohort release control.
//!
//! This module is the release-control lane that keeps the design-partner,
//! certified-archetype, and stable-cohort scoreboards in one canonical packet. The
//! stable claim matrix decides which public claims may publish at the cutline; the
//! support-class ledger decides which subjects are Certified or Supported; the
//! stable proof index binds launch-blocking requirements to proof packets. This
//! packet binds the human signoff loops and scoreboard metrics that decide whether
//! those rows are allowed to widen in public surfaces.
//!
//! Each [`CohortScoreboardRow`] records one release-blocking scoreboard row. It
//! names the scoreboard lane, the public claim whose lifecycle label is a hard
//! ceiling, the proof packet and freshness state, required signoff loop, metric
//! thresholds, active narrowing reasons, and the lifecycle label the row actually
//! displays. A row may hold the claim label or narrow below it; it may never widen
//! beyond the public claim it backs.
//!
//! The packet is checked in at `artifacts/release/cohort_scoreboards.json` and
//! embedded here, so UI, Help/About, support export, release tooling, and tests
//! read one source instead of cloning spreadsheet status text.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported scoreboards schema version.
pub const COHORT_SCOREBOARDS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the scoreboards packet.
pub const COHORT_SCOREBOARDS_RECORD_KIND: &str = "cohort_scoreboards";

/// Repo-relative path to the checked-in scoreboards packet.
pub const COHORT_SCOREBOARDS_PATH: &str = "artifacts/release/cohort_scoreboards.json";

/// Embedded checked-in scoreboards JSON.
pub const COHORT_SCOREBOARDS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/cohort_scoreboards.json"
));

/// Scoreboard lane governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreboardLane {
    /// Design-partner and managed-pilot evidence rows.
    DesignPartner,
    /// Certified-archetype report rows.
    CertifiedArchetype,
    /// Stable-cohort admission and cohort-health rows.
    StableCohort,
}

impl ScoreboardLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DesignPartner,
        Self::CertifiedArchetype,
        Self::StableCohort,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesignPartner => "design_partner",
            Self::CertifiedArchetype => "certified_archetype",
            Self::StableCohort => "stable_cohort",
        }
    }
}

/// Scoreboard state earned by a row for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreboardState {
    /// The row has current proof, passing metrics, and every required signoff.
    SignedOff,
    /// The row holds its claim only because an active waiver covers a recorded gap.
    SignedOffOnWaiver,
    /// Required evidence, capability, metric, or signoff is incomplete.
    NarrowedUnbacked,
    /// The backing public claim is itself below the stable cutline.
    NarrowedClaimNarrowed,
    /// The row's proof packet breached its freshness SLO or is missing.
    NarrowedStale,
    /// The row relied on a waiver that has expired.
    NarrowedWaiverExpired,
}

impl ScoreboardState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SignedOff,
        Self::SignedOffOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedOff => "signed_off",
            Self::SignedOffOnWaiver => "signed_off_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets the row hold the public claim at its label.
    pub const fn holds_scoreboard(self) -> bool {
        matches!(self, Self::SignedOff | Self::SignedOffOnWaiver)
    }

    /// Whether the state forces the row below the public claim label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_scoreboard()
    }
}

/// Closed reason a scoreboard row narrows or a publication rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreboardGapReason {
    /// The backing public claim narrowed below the cutline.
    ClaimLabelNarrowed,
    /// Required scoreboarding evidence is incomplete.
    ScoreboardEvidenceIncomplete,
    /// The scoreboard proof packet breached its freshness SLO.
    ScoreboardPacketFreshnessBreached,
    /// No scoreboard proof packet has been captured.
    ScoreboardPacketMissing,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// The required row owner signoff is missing.
    OwnerSignoffMissing,
    /// One or more required signoff roles have not signed.
    RequiredSignoffMissing,
    /// A scoreboard metric failed its threshold.
    ScoreBelowThreshold,
}

impl ScoreboardGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ClaimLabelNarrowed,
        Self::ScoreboardEvidenceIncomplete,
        Self::ScoreboardPacketFreshnessBreached,
        Self::ScoreboardPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::RequiredSignoffMissing,
        Self::ScoreBelowThreshold,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::ScoreboardEvidenceIncomplete => "scoreboard_evidence_incomplete",
            Self::ScoreboardPacketFreshnessBreached => "scoreboard_packet_freshness_breached",
            Self::ScoreboardPacketMissing => "scoreboard_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::RequiredSignoffMissing => "required_signoff_missing",
            Self::ScoreBelowThreshold => "score_below_threshold",
        }
    }
}

/// Default action a scoreboard rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreboardAction {
    /// Hold stable publication until the condition clears.
    HoldPublication,
    /// Narrow the scoreboard's displayed lifecycle label below the cutline.
    NarrowScoreboardLabel,
    /// Refresh the scoreboard packet so it re-enters its freshness SLO.
    RefreshScoreboardPacket,
    /// Complete the required signoff loop.
    CompleteSignoffLoop,
    /// Recapture the scoreboard evidence.
    RecaptureScoreboardEvidence,
}

impl ScoreboardAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPublication,
        Self::NarrowScoreboardLabel,
        Self::RefreshScoreboardPacket,
        Self::CompleteSignoffLoop,
        Self::RecaptureScoreboardEvidence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowScoreboardLabel => "narrow_scoreboard_label",
            Self::RefreshScoreboardPacket => "refresh_scoreboard_packet",
            Self::CompleteSignoffLoop => "complete_signoff_loop",
            Self::RecaptureScoreboardEvidence => "recapture_scoreboard_evidence",
        }
    }
}

/// One required signoff role in a row's signoff loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequiredSignoff {
    /// Required role or owner group.
    pub role_ref: String,
    /// Whether the role has signed the row.
    pub signed_off: bool,
    /// Signing person or group, when signed.
    #[serde(default)]
    pub signer_ref: Option<String>,
    /// UTC date of signoff, or null when unsigned.
    #[serde(default)]
    pub signed_at: Option<String>,
}

impl RequiredSignoff {
    /// Whether this required signoff is complete.
    pub fn is_complete(&self) -> bool {
        self.signed_off
            && self
                .signed_at
                .as_ref()
                .is_some_and(|value| !value.trim().is_empty())
            && self
                .signer_ref
                .as_ref()
                .is_some_and(|value| !value.trim().is_empty())
    }
}

/// Required human signoff loop for one scoreboard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignoffLoop {
    /// Stable loop id.
    pub loop_id: String,
    /// Review cadence for this loop.
    pub cadence: String,
    /// Packet or meeting record that carries the signoff loop.
    pub packet_ref: String,
    /// Required signoffs for the row.
    #[serde(default)]
    pub required_signoffs: Vec<RequiredSignoff>,
}

impl SignoffLoop {
    /// Whether every required role has signed off.
    pub fn is_complete(&self) -> bool {
        !self.required_signoffs.is_empty()
            && self
                .required_signoffs
                .iter()
                .all(RequiredSignoff::is_complete)
    }

    /// Required role refs whose signoff is missing or incomplete.
    pub fn missing_roles(&self) -> Vec<String> {
        self.required_signoffs
            .iter()
            .filter(|signoff| !signoff.is_complete())
            .map(|signoff| signoff.role_ref.clone())
            .collect()
    }
}

/// One measured score on a scoreboard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoreboardMetric {
    /// Stable metric id.
    pub metric_id: String,
    /// Human-readable title.
    pub title: String,
    /// Unit for the threshold and measured value.
    pub unit: String,
    /// Minimum measured value required for the row to hold.
    pub threshold: i64,
    /// Measured value, or null when no measurement has been captured.
    #[serde(default)]
    pub measured: Option<i64>,
    /// Measurement source.
    pub measurement_ref: String,
}

impl ScoreboardMetric {
    /// Whether the metric has a measurement that clears the threshold.
    pub fn passes_threshold(&self) -> bool {
        self.measured
            .is_some_and(|measured| measured >= self.threshold)
    }
}

/// One scoreboard stop rule that narrows rows and can hold publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoreboardRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// Gap reason whose presence fires this rule.
    pub trigger_reason: ScoreboardGapReason,
    /// Public-claim labels watched by this rule.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: ScoreboardAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One governed row in the design-partner, certified-archetype, or stable-cohort
/// scoreboard family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CohortScoreboardRow {
    /// Stable row id.
    pub scoreboard_id: String,
    /// Human-readable title.
    pub title: String,
    /// Scoreboard lane this row belongs to.
    pub lane: ScoreboardLane,
    /// Subject family the row governs.
    pub subject_family: String,
    /// Whether this row is release-blocking.
    pub release_blocking: bool,
    /// Stable-claim-manifest entry whose lifecycle label is this row's ceiling.
    pub claim_ref: String,
    /// Public claim's canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// State this row earned.
    pub scoreboard_state: ScoreboardState,
    /// Proof packet and freshness SLO for this row.
    pub scoreboard_packet: ProofPacket,
    /// Waiver authorizing a provisional row, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Row owner signoff.
    pub owner_signoff: OwnerSignoff,
    /// Required signoff loop for this row.
    pub signoff_loop: SignoffLoop,
    /// Scoreboard metrics this row must clear.
    #[serde(default)]
    pub metrics: Vec<ScoreboardMetric>,
    /// Active reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<ScoreboardGapReason>,
    /// Lifecycle label this scoreboard row displays after narrowing.
    pub effective_label: StableClaimLevel,
    /// Destinations that render this row.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason this row carries this posture.
    pub rationale: String,
}

impl CohortScoreboardRow {
    /// Whether the row displays a label at or above the stable cutline.
    pub fn holds_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// Whether the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// Whether the state lets the row hold its public claim label.
    pub fn holds_scoreboard(&self) -> bool {
        self.scoreboard_state.holds_scoreboard()
    }

    /// Whether the row has an active gap reason.
    pub fn has_active_reason(&self, reason: ScoreboardGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// Whether every metric has a measurement that clears its threshold.
    pub fn all_metrics_pass(&self) -> bool {
        !self.metrics.is_empty() && self.metrics.iter().all(ScoreboardMetric::passes_threshold)
    }

    /// Count of metrics that fail or are unmeasured.
    pub fn failing_metric_count(&self) -> usize {
        self.metrics
            .iter()
            .filter(|metric| !metric.passes_threshold())
            .count()
    }
}

/// Recorded publication verdict for the scoreboards packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoreboardPublicationRecord {
    /// Gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Blocking rule ids, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Scoreboard row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_scoreboard_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the scoreboards packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CohortScoreboardsSummary {
    /// Total rows.
    pub total_rows: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows displaying a label at or above the cutline.
    pub rows_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub rows_narrowed_below_cutline: usize,
    /// Release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows displaying a label at or above the cutline.
    pub release_blocking_holding_stable: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Rows holding via an active waiver.
    pub rows_on_active_waiver: usize,
    /// Current proof packets.
    pub packets_current: usize,
    /// Proof packets due for refresh.
    pub packets_due_for_refresh: usize,
    /// Breached proof packets.
    pub packets_breached: usize,
    /// Missing proof packets.
    pub packets_missing: usize,
    /// Complete signoff loops.
    pub complete_signoff_loops: usize,
    /// Incomplete signoff loops.
    pub incomplete_signoff_loops: usize,
    /// Total metrics.
    pub total_metrics: usize,
    /// Metrics that pass their threshold.
    pub metrics_passing: usize,
    /// Metrics that fail or are unmeasured.
    pub metrics_failing: usize,
    /// Total active gap reasons.
    pub total_active_gap_reasons: usize,
    /// Number of rules currently firing.
    pub rules_firing: usize,
}

/// Typed stable cohort scoreboards packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CohortScoreboards {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Stable claim manifest ingested as the public-claim source and ceiling.
    pub claim_manifest_ref: String,
    /// Stable proof-index row proving this scoreboards packet.
    pub stable_proof_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed lane vocabulary.
    pub scoreboard_lanes: Vec<ScoreboardLane>,
    /// Closed row-state vocabulary.
    pub scoreboard_states: Vec<ScoreboardState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<ScoreboardGapReason>,
    /// Closed action vocabulary.
    pub scoreboard_actions: Vec<ScoreboardAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// Release-blocking scoreboard refs this packet must cover.
    pub release_blocking_scoreboard_refs: Vec<String>,
    /// Publication rules.
    pub rules: Vec<ScoreboardRule>,
    /// Scoreboard rows.
    pub rows: Vec<CohortScoreboardRow>,
    /// Recorded publication verdict.
    pub publication: ScoreboardPublicationRecord,
    /// Summary counts.
    pub summary: CohortScoreboardsSummary,
}

impl CohortScoreboards {
    /// Returns the row registered for `scoreboard_id`.
    pub fn row(&self, scoreboard_id: &str) -> Option<&CohortScoreboardRow> {
        self.rows
            .iter()
            .find(|row| row.scoreboard_id == scoreboard_id)
    }

    /// Returns the rows displaying a label at or above the stable cutline.
    pub fn rows_holding_stable(&self) -> Vec<&CohortScoreboardRow> {
        self.rows.iter().filter(|row| row.holds_stable()).collect()
    }

    /// Returns the rows narrowed below the stable cutline.
    pub fn rows_narrowed(&self) -> Vec<&CohortScoreboardRow> {
        self.rows.iter().filter(|row| !row.holds_stable()).collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&CohortScoreboardRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Distinct public claims covered by the packet.
    pub fn claims(&self) -> Vec<String> {
        let mut claims = BTreeSet::new();
        for row in &self.rows {
            claims.insert(row.claim_ref.clone());
        }
        claims.into_iter().collect()
    }

    /// Rows in one scoreboard lane.
    pub fn rows_for_lane(&self, lane: ScoreboardLane) -> Vec<&CohortScoreboardRow> {
        self.rows.iter().filter(|row| row.lane == lane).collect()
    }

    /// True when `rule` fires for at least one watched row.
    pub fn rule_fires(&self, rule: &ScoreboardRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from rows and rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_publication && self.rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Blocking rule ids that are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Scoreboard ids that trigger a blocking, firing rule, sorted.
    pub fn computed_blocking_scoreboard_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<ScoreboardGapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.scoreboard_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and rules.
    pub fn computed_summary(&self) -> CohortScoreboardsSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.scoreboard_packet.slo_state == state)
                .count()
        };
        let release_blocking = self.release_blocking_rows();
        let total_metrics: usize = self.rows.iter().map(|row| row.metrics.len()).sum();
        let metrics_failing: usize = self
            .rows
            .iter()
            .map(CohortScoreboardRow::failing_metric_count)
            .sum();
        CohortScoreboardsSummary {
            total_rows: self.rows.len(),
            total_claims: self.claims().len(),
            rows_holding_stable: self.rows.iter().filter(|row| row.holds_stable()).count(),
            rows_narrowed_below_cutline: self.rows.iter().filter(|row| !row.holds_stable()).count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_holding_stable: release_blocking
                .iter()
                .filter(|row| row.holds_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.holds_stable())
                .count(),
            rows_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.scoreboard_state == ScoreboardState::SignedOffOnWaiver)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            complete_signoff_loops: self
                .rows
                .iter()
                .filter(|row| row.signoff_loop.is_complete())
                .count(),
            incomplete_signoff_loops: self
                .rows
                .iter()
                .filter(|row| !row.signoff_loop.is_complete())
                .count(),
            total_metrics,
            metrics_passing: total_metrics - metrics_failing,
            metrics_failing,
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .rules
                .iter()
                .filter(|rule| self.rule_fires(rule))
                .count(),
        }
    }

    /// Produces a redaction-safe projection for Help/About, docs, release packet,
    /// shiproom, and support export surfaces.
    pub fn support_export_projection(&self) -> CohortScoreboardsExportProjection {
        CohortScoreboardsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| CohortScoreboardsExportRow {
                    scoreboard_id: row.scoreboard_id.clone(),
                    lane: row.lane,
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    holds_stable: row.holds_stable(),
                    scoreboard_state: row.scoreboard_state,
                    slo_state: row.scoreboard_packet.slo_state,
                    signoff_loop_complete: row.signoff_loop.is_complete(),
                    missing_signoff_roles: row.signoff_loop.missing_roles(),
                    metric_total: row.metrics.len(),
                    metric_failing: row.failing_metric_count(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the scoreboards packet, returning every violation found.
    pub fn validate(&self) -> Vec<CohortScoreboardsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.scoreboard_id.clone()) {
                violations.push(CohortScoreboardsViolation::DuplicateScoreboardId {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(CohortScoreboardsViolation::EmptyPacket);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(CohortScoreboardsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<CohortScoreboardsViolation>) {
        if self.schema_version != COHORT_SCOREBOARDS_SCHEMA_VERSION {
            violations.push(CohortScoreboardsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != COHORT_SCOREBOARDS_RECORD_KIND {
            violations.push(CohortScoreboardsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("stable_proof_index_ref", &self.stable_proof_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CohortScoreboardsViolation::EmptyField {
                    scoreboard_id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(CohortScoreboardsViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.scoreboard_lanes != ScoreboardLane::ALL.to_vec() {
            violations.push(CohortScoreboardsViolation::ClosedVocabularyMismatch {
                field: "scoreboard_lanes",
            });
        }
        if self.scoreboard_states != ScoreboardState::ALL.to_vec() {
            violations.push(CohortScoreboardsViolation::ClosedVocabularyMismatch {
                field: "scoreboard_states",
            });
        }
        if self.gap_reasons != ScoreboardGapReason::ALL.to_vec() {
            violations.push(CohortScoreboardsViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.scoreboard_actions != ScoreboardAction::ALL.to_vec() {
            violations.push(CohortScoreboardsViolation::ClosedVocabularyMismatch {
                field: "scoreboard_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(CohortScoreboardsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(CohortScoreboardsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(CohortScoreboardsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(CohortScoreboardsViolation::EmptyField {
                scoreboard_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<CohortScoreboardsViolation>) {
        if self.rules.is_empty() {
            violations.push(CohortScoreboardsViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(CohortScoreboardsViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(CohortScoreboardsViolation::EmptyField {
                        scoreboard_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(CohortScoreboardsViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }
        for reason in ScoreboardGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(CohortScoreboardsViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &CohortScoreboardRow,
        violations: &mut Vec<CohortScoreboardsViolation>,
    ) {
        for (field, value) in [
            ("scoreboard_id", &row.scoreboard_id),
            ("title", &row.title),
            ("subject_family", &row.subject_family),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            (
                "scoreboard_packet.packet_id",
                &row.scoreboard_packet.packet_id,
            ),
            (
                "scoreboard_packet.packet_ref",
                &row.scoreboard_packet.packet_ref,
            ),
            (
                "scoreboard_packet.proof_index_ref",
                &row.scoreboard_packet.proof_index_ref,
            ),
            (
                "scoreboard_packet.freshness_slo.slo_register_ref",
                &row.scoreboard_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
            ("signoff_loop.loop_id", &row.signoff_loop.loop_id),
            ("signoff_loop.cadence", &row.signoff_loop.cadence),
            ("signoff_loop.packet_ref", &row.signoff_loop.packet_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CohortScoreboardsViolation::EmptyField {
                    scoreboard_id: row.scoreboard_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(CohortScoreboardsViolation::EffectiveWiderThanClaim {
                scoreboard_id: row.scoreboard_id.clone(),
                claim: row.claim_label,
                effective: row.effective_label,
            });
        }
        if row.scoreboard_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(CohortScoreboardsViolation::EmptyField {
                scoreboard_id: row.scoreboard_id.clone(),
                field_name: "scoreboard_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.scoreboard_packet.freshness_slo.window_is_consistent() {
            violations.push(CohortScoreboardsViolation::FreshnessSloInconsistent {
                scoreboard_id: row.scoreboard_id.clone(),
            });
        }

        self.validate_signoffs(row, violations);
        self.validate_metrics(row, violations);
        self.validate_state(row, violations);
        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_signoffs(
        &self,
        row: &CohortScoreboardRow,
        violations: &mut Vec<CohortScoreboardsViolation>,
    ) {
        if row.signoff_loop.required_signoffs.is_empty() {
            violations.push(CohortScoreboardsViolation::EmptyField {
                scoreboard_id: row.scoreboard_id.clone(),
                field_name: "signoff_loop.required_signoffs",
            });
        }
        let mut roles = BTreeSet::new();
        for signoff in &row.signoff_loop.required_signoffs {
            if signoff.role_ref.trim().is_empty() {
                violations.push(CohortScoreboardsViolation::EmptyField {
                    scoreboard_id: row.scoreboard_id.clone(),
                    field_name: "required_signoff.role_ref",
                });
            }
            if !roles.insert(signoff.role_ref.clone()) {
                violations.push(CohortScoreboardsViolation::DuplicateSignoffRole {
                    scoreboard_id: row.scoreboard_id.clone(),
                    role_ref: signoff.role_ref.clone(),
                });
            }
        }
        if !row.signoff_loop.is_complete()
            && !row.has_active_reason(ScoreboardGapReason::RequiredSignoffMissing)
        {
            violations.push(
                CohortScoreboardsViolation::IncompleteSignoffLoopWithoutReason {
                    scoreboard_id: row.scoreboard_id.clone(),
                },
            );
        }
    }

    fn validate_metrics(
        &self,
        row: &CohortScoreboardRow,
        violations: &mut Vec<CohortScoreboardsViolation>,
    ) {
        if row.metrics.is_empty() {
            violations.push(CohortScoreboardsViolation::EmptyField {
                scoreboard_id: row.scoreboard_id.clone(),
                field_name: "metrics",
            });
        }
        let mut metrics = BTreeSet::new();
        for metric in &row.metrics {
            if !metrics.insert(metric.metric_id.clone()) {
                violations.push(CohortScoreboardsViolation::DuplicateMetricId {
                    scoreboard_id: row.scoreboard_id.clone(),
                    metric_id: metric.metric_id.clone(),
                });
            }
            for (field, value) in [
                ("metric_id", &metric.metric_id),
                ("title", &metric.title),
                ("unit", &metric.unit),
                ("measurement_ref", &metric.measurement_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(CohortScoreboardsViolation::EmptyField {
                        scoreboard_id: row.scoreboard_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
        if row.failing_metric_count() > 0
            && !row.has_active_reason(ScoreboardGapReason::ScoreBelowThreshold)
        {
            violations.push(CohortScoreboardsViolation::FailingMetricWithoutReason {
                scoreboard_id: row.scoreboard_id.clone(),
            });
        }
    }

    fn validate_state(
        &self,
        row: &CohortScoreboardRow,
        violations: &mut Vec<CohortScoreboardsViolation>,
    ) {
        if !row.claim_holds_stable() {
            if row.holds_scoreboard() {
                violations.push(CohortScoreboardsViolation::HeldOnNarrowedClaim {
                    scoreboard_id: row.scoreboard_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(ScoreboardGapReason::ClaimLabelNarrowed) {
                violations.push(CohortScoreboardsViolation::ClaimNarrowedWithoutReason {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
        }

        let slo_state = row.scoreboard_packet.slo_state;
        if row.holds_scoreboard() {
            if row.effective_label != row.claim_label {
                violations.push(CohortScoreboardsViolation::HeldLabelNotEqualClaim {
                    scoreboard_id: row.scoreboard_id.clone(),
                    claim: row.claim_label,
                    effective: row.effective_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(CohortScoreboardsViolation::HeldWithActiveGap {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
            if !row.scoreboard_packet.has_capture() {
                violations.push(CohortScoreboardsViolation::HeldWithoutFreshPacket {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(CohortScoreboardsViolation::HeldOnStalePacket {
                    scoreboard_id: row.scoreboard_id.clone(),
                    slo_state,
                });
            }
            if !row.all_metrics_pass() {
                violations.push(CohortScoreboardsViolation::HeldWithFailingMetric {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(CohortScoreboardsViolation::HeldWithoutOwnerSignoff {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
            if !row.signoff_loop.is_complete() {
                violations.push(CohortScoreboardsViolation::HeldWithoutRequiredSignoffs {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
            if row
                .waiver
                .as_ref()
                .is_some_and(|waiver| waiver.expires_at.as_str() <= self.as_of.as_str())
            {
                violations.push(CohortScoreboardsViolation::HeldOnExpiredWaiver {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
        } else {
            if row.holds_stable() {
                violations.push(CohortScoreboardsViolation::EffectiveLabelNotNarrowed {
                    scoreboard_id: row.scoreboard_id.clone(),
                    state: row.scoreboard_state,
                    effective: row.effective_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(CohortScoreboardsViolation::NarrowingWithoutReason {
                    scoreboard_id: row.scoreboard_id.clone(),
                    state: row.scoreboard_state,
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(ScoreboardGapReason::ScoreboardPacketFreshnessBreached)
            {
                violations.push(CohortScoreboardsViolation::BreachedPacketWithoutReason {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(ScoreboardGapReason::ScoreboardPacketMissing)
            {
                violations.push(CohortScoreboardsViolation::MissingPacketWithoutReason {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &CohortScoreboardRow,
        violations: &mut Vec<CohortScoreboardsViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<CohortScoreboardsViolation>,
                               expected: ScoreboardGapReason| {
            violations.push(CohortScoreboardsViolation::StateReasonIncoherent {
                scoreboard_id: row.scoreboard_id.clone(),
                state: row.scoreboard_state,
                expected_reason: expected,
            });
        };

        match row.scoreboard_state {
            ScoreboardState::NarrowedUnbacked => {
                const ALLOWED: [ScoreboardGapReason; 4] = [
                    ScoreboardGapReason::ScoreboardEvidenceIncomplete,
                    ScoreboardGapReason::OwnerSignoffMissing,
                    ScoreboardGapReason::RequiredSignoffMissing,
                    ScoreboardGapReason::ScoreBelowThreshold,
                ];
                if !ALLOWED.iter().any(|reason| row.has_active_reason(*reason)) {
                    push_incoherent(
                        violations,
                        ScoreboardGapReason::ScoreboardEvidenceIncomplete,
                    );
                }
            }
            ScoreboardState::NarrowedClaimNarrowed => {
                if !row.has_active_reason(ScoreboardGapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, ScoreboardGapReason::ClaimLabelNarrowed);
                }
            }
            ScoreboardState::NarrowedStale => {
                if !(row.has_active_reason(ScoreboardGapReason::ScoreboardPacketFreshnessBreached)
                    || row.has_active_reason(ScoreboardGapReason::ScoreboardPacketMissing))
                {
                    push_incoherent(
                        violations,
                        ScoreboardGapReason::ScoreboardPacketFreshnessBreached,
                    );
                }
            }
            ScoreboardState::NarrowedWaiverExpired => {
                if !row.has_active_reason(ScoreboardGapReason::WaiverExpired) {
                    push_incoherent(violations, ScoreboardGapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(CohortScoreboardsViolation::WaiverStateWithoutWaiver {
                        scoreboard_id: row.scoreboard_id.clone(),
                        state: row.scoreboard_state,
                    });
                }
            }
            ScoreboardState::SignedOffOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|waiver| {
                        waiver.waiver_ref.trim().is_empty() || waiver.expires_at.trim().is_empty()
                    })
                    .unwrap_or(true)
                {
                    violations.push(CohortScoreboardsViolation::WaiverStateWithoutWaiver {
                        scoreboard_id: row.scoreboard_id.clone(),
                        state: row.scoreboard_state,
                    });
                }
            }
            ScoreboardState::SignedOff => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<CohortScoreboardsViolation>) {
        let declared: BTreeSet<&str> = self
            .release_blocking_scoreboard_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.scoreboard_id.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(CohortScoreboardsViolation::ReleaseBlockingRefWithoutRow {
                    scoreboard_id: (*declared_ref).to_owned(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.scoreboard_id.as_str()) {
                violations.push(CohortScoreboardsViolation::ReleaseBlockingRowNotInSet {
                    scoreboard_id: row.scoreboard_id.clone(),
                });
            }
        }
        for lane in ScoreboardLane::ALL {
            if self.rows_for_lane(lane).is_empty() {
                violations.push(CohortScoreboardsViolation::LaneAbsent { lane });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<CohortScoreboardsViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(CohortScoreboardsViolation::EmptyField {
                scoreboard_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(CohortScoreboardsViolation::EmptyField {
                scoreboard_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                CohortScoreboardsViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(CohortScoreboardsViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_scoreboard_ids != self.computed_blocking_scoreboard_ids() {
            violations.push(CohortScoreboardsViolation::PublicationBlockingSetMismatch {
                field: "blocking_scoreboard_ids",
            });
        }
    }
}

/// Redaction-safe export row projected from the scoreboards packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CohortScoreboardsExportRow {
    /// Stable row id.
    pub scoreboard_id: String,
    /// Scoreboard lane.
    pub lane: ScoreboardLane,
    /// Whether this row is release-blocking.
    pub release_blocking: bool,
    /// Backing public claim ref.
    pub claim_ref: String,
    /// Public claim's canonical label.
    pub claim_label: StableClaimLevel,
    /// Effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// Whether the row holds at or above the stable cutline.
    pub holds_stable: bool,
    /// Row state.
    pub scoreboard_state: ScoreboardState,
    /// Proof-packet freshness state.
    pub slo_state: FreshnessSloState,
    /// Whether the required signoff loop is complete.
    pub signoff_loop_complete: bool,
    /// Required roles that have not signed.
    pub missing_signoff_roles: Vec<String>,
    /// Total metrics carried by the row.
    pub metric_total: usize,
    /// Metrics that failed or are unmeasured.
    pub metric_failing: usize,
    /// Active narrowing reasons.
    pub active_gap_reasons: Vec<ScoreboardGapReason>,
}

/// Redaction-safe export projection of the scoreboards packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CohortScoreboardsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<CohortScoreboardsExportRow>,
}

/// Validation violation for the scoreboards packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CohortScoreboardsViolation {
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
    /// A closed vocabulary or pinned cutline is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The packet has no rows.
    EmptyPacket,
    /// The packet has no rules.
    NoRules,
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        scoreboard_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A scoreboard id appears more than once.
    DuplicateScoreboardId {
        /// Duplicate row id.
        scoreboard_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A signoff role appears more than once in a row.
    DuplicateSignoffRole {
        /// Row id.
        scoreboard_id: String,
        /// Duplicate role.
        role_ref: String,
    },
    /// A metric id appears more than once in a row.
    DuplicateMetricId {
        /// Row id.
        scoreboard_id: String,
        /// Duplicate metric id.
        metric_id: String,
    },
    /// A rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: ScoreboardGapReason,
    },
    /// A row's effective label is wider than its claim ceiling.
    EffectiveWiderThanClaim {
        /// Row id.
        scoreboard_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Row id.
        scoreboard_id: String,
    },
    /// A held row is backed by a public claim already below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        scoreboard_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
    },
    /// A row whose claim narrowed does not name the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Row id.
        scoreboard_id: String,
    },
    /// A held row displays a label different from its claim ceiling.
    HeldLabelNotEqualClaim {
        /// Row id.
        scoreboard_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A held row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        scoreboard_id: String,
    },
    /// A held row has no captured, evidence-backed packet.
    HeldWithoutFreshPacket {
        /// Row id.
        scoreboard_id: String,
    },
    /// A held row rides a stale or missing packet.
    HeldOnStalePacket {
        /// Row id.
        scoreboard_id: String,
        /// SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held row carries a failing metric.
    HeldWithFailingMetric {
        /// Row id.
        scoreboard_id: String,
    },
    /// A held row lacks owner signoff.
    HeldWithoutOwnerSignoff {
        /// Row id.
        scoreboard_id: String,
    },
    /// A held row lacks required signoffs.
    HeldWithoutRequiredSignoffs {
        /// Row id.
        scoreboard_id: String,
    },
    /// A held row relies on an expired waiver.
    HeldOnExpiredWaiver {
        /// Row id.
        scoreboard_id: String,
    },
    /// A narrowing state did not narrow below the cutline.
    EffectiveLabelNotNarrowed {
        /// Row id.
        scoreboard_id: String,
        /// Row state.
        state: ScoreboardState,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A narrowing state carries no reason.
    NarrowingWithoutReason {
        /// Row id.
        scoreboard_id: String,
        /// Row state.
        state: ScoreboardState,
    },
    /// A breached packet does not name the freshness breach reason.
    BreachedPacketWithoutReason {
        /// Row id.
        scoreboard_id: String,
    },
    /// A missing packet does not name the missing packet reason.
    MissingPacketWithoutReason {
        /// Row id.
        scoreboard_id: String,
    },
    /// An incomplete signoff loop does not name the missing-signoff reason.
    IncompleteSignoffLoopWithoutReason {
        /// Row id.
        scoreboard_id: String,
    },
    /// A failing metric does not name the threshold reason.
    FailingMetricWithoutReason {
        /// Row id.
        scoreboard_id: String,
    },
    /// A row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        scoreboard_id: String,
        /// Row state.
        state: ScoreboardState,
        /// Expected reason.
        expected_reason: ScoreboardGapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        scoreboard_id: String,
        /// Row state.
        state: ScoreboardState,
    },
    /// A declared release-blocking row has no row.
    ReleaseBlockingRefWithoutRow {
        /// Missing row id.
        scoreboard_id: String,
    },
    /// A release-blocking row was not declared.
    ReleaseBlockingRowNotInSet {
        /// Row id.
        scoreboard_id: String,
    },
    /// A lane is absent from the packet.
    LaneAbsent {
        /// Missing lane.
        lane: ScoreboardLane,
    },
    /// The declared publication decision disagrees with the computed decision.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared publication blocking set disagrees with computed rules.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// Summary counts disagree with rows.
    SummaryMismatch,
}

impl fmt::Display for CohortScoreboardsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported scoreboards schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported scoreboards record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "scoreboards {field} is not the canonical value")
            }
            Self::EmptyPacket => write!(f, "scoreboards packet has no rows"),
            Self::NoRules => write!(f, "scoreboards packet has no rules"),
            Self::EmptyField {
                scoreboard_id,
                field_name,
            } => write!(f, "{scoreboard_id} has empty field {field_name}"),
            Self::DuplicateScoreboardId { scoreboard_id } => {
                write!(f, "duplicate scoreboard row id {scoreboard_id}")
            }
            Self::DuplicateRuleId { rule_id } => write!(f, "duplicate rule id {rule_id}"),
            Self::DuplicateSignoffRole {
                scoreboard_id,
                role_ref,
            } => write!(
                f,
                "scoreboard {scoreboard_id} repeats required signoff role {role_ref}"
            ),
            Self::DuplicateMetricId {
                scoreboard_id,
                metric_id,
            } => write!(f, "scoreboard {scoreboard_id} repeats metric {metric_id}"),
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule {rule_id} watches no lifecycle labels")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::EffectiveWiderThanClaim {
                scoreboard_id,
                claim,
                effective,
            } => write!(
                f,
                "scoreboard {scoreboard_id} effective label {} is wider than claim ceiling {}",
                effective.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} freshness SLO warn window exceeds target age"
            ),
            Self::HeldOnNarrowedClaim {
                scoreboard_id,
                claim,
            } => write!(
                f,
                "scoreboard {scoreboard_id} holds while claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} backs a narrowed claim without naming claim_label_narrowed"
            ),
            Self::HeldLabelNotEqualClaim {
                scoreboard_id,
                claim,
                effective,
            } => write!(
                f,
                "scoreboard {scoreboard_id} holds {} but claim ceiling is {}",
                effective.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} holds while a gap reason is active"
            ),
            Self::HeldWithoutFreshPacket { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} holds with no captured, evidence-backed packet"
            ),
            Self::HeldOnStalePacket {
                scoreboard_id,
                slo_state,
            } => write!(
                f,
                "scoreboard {scoreboard_id} holds while packet is {}",
                slo_state.as_str()
            ),
            Self::HeldWithFailingMetric { scoreboard_id } => {
                write!(f, "scoreboard {scoreboard_id} holds with a failing metric")
            }
            Self::HeldWithoutOwnerSignoff { scoreboard_id } => {
                write!(f, "scoreboard {scoreboard_id} holds without owner signoff")
            }
            Self::HeldWithoutRequiredSignoffs { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} holds without all required signoffs"
            ),
            Self::HeldOnExpiredWaiver { scoreboard_id } => {
                write!(f, "scoreboard {scoreboard_id} holds on an expired waiver")
            }
            Self::EffectiveLabelNotNarrowed {
                scoreboard_id,
                state,
                effective,
            } => write!(
                f,
                "scoreboard {scoreboard_id} state {} must narrow but holds {}",
                state.as_str(),
                effective.as_str()
            ),
            Self::NarrowingWithoutReason {
                scoreboard_id,
                state,
            } => write!(
                f,
                "scoreboard {scoreboard_id} state {} narrows without a reason",
                state.as_str()
            ),
            Self::BreachedPacketWithoutReason { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} has a breached packet without the freshness reason"
            ),
            Self::MissingPacketWithoutReason { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} has a missing packet without the missing-packet reason"
            ),
            Self::IncompleteSignoffLoopWithoutReason { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} has an incomplete signoff loop without required_signoff_missing"
            ),
            Self::FailingMetricWithoutReason { scoreboard_id } => write!(
                f,
                "scoreboard {scoreboard_id} has a failing metric without score_below_threshold"
            ),
            Self::StateReasonIncoherent {
                scoreboard_id,
                state,
                expected_reason,
            } => write!(
                f,
                "scoreboard {scoreboard_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver {
                scoreboard_id,
                state,
            } => write!(
                f,
                "scoreboard {scoreboard_id} state {} names no waiver",
                state.as_str()
            ),
            Self::ReleaseBlockingRefWithoutRow { scoreboard_id } => write!(
                f,
                "declared release-blocking scoreboard {scoreboard_id} has no row"
            ),
            Self::ReleaseBlockingRowNotInSet { scoreboard_id } => write!(
                f,
                "release-blocking scoreboard {scoreboard_id} is not declared"
            ),
            Self::LaneAbsent { lane } => write!(
                f,
                "scoreboard lane {} is covered by no row",
                lane.as_str()
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing rules")
            }
            Self::SummaryMismatch => write!(f, "scoreboards summary counts disagree with rows"),
        }
    }
}

impl Error for CohortScoreboardsViolation {}

/// Loads the embedded cohort scoreboards packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`CohortScoreboards`].
pub fn current_cohort_scoreboards() -> Result<CohortScoreboards, serde_json::Error> {
    serde_json::from_str(COHORT_SCOREBOARDS_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scoreboards() -> CohortScoreboards {
        current_cohort_scoreboards().expect("cohort scoreboards parse")
    }

    #[test]
    fn embedded_scoreboards_parse_and_validate() {
        let scoreboards = scoreboards();
        assert_eq!(
            scoreboards.schema_version,
            COHORT_SCOREBOARDS_SCHEMA_VERSION
        );
        assert_eq!(scoreboards.record_kind, COHORT_SCOREBOARDS_RECORD_KIND);
        assert_eq!(scoreboards.validate(), Vec::new());
    }

    #[test]
    fn every_lane_is_covered() {
        let scoreboards = scoreboards();
        for lane in ScoreboardLane::ALL {
            assert!(
                !scoreboards.rows_for_lane(lane).is_empty(),
                "{} must have at least one row",
                lane.as_str()
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let scoreboards = scoreboards();
        assert_eq!(scoreboards.summary, scoreboards.computed_summary());
        assert_eq!(
            scoreboards.summary.rows_holding_stable
                + scoreboards.summary.rows_narrowed_below_cutline,
            scoreboards.rows.len()
        );
    }

    #[test]
    fn publication_proceeds_without_blocking_rules() {
        let scoreboards = scoreboards();
        assert_eq!(scoreboards.publication.decision, PromotionDecision::Proceed);
        assert_eq!(
            scoreboards.publication.decision,
            scoreboards.computed_publication_decision()
        );
        assert!(scoreboards.publication.blocking_rule_ids.is_empty());
        assert!(scoreboards.publication.blocking_scoreboard_ids.is_empty());
    }

    #[test]
    fn held_row_on_breached_packet_fails() {
        let mut scoreboards = scoreboards();
        let row = scoreboards
            .rows
            .iter_mut()
            .find(|row| row.holds_scoreboard())
            .expect("a held row exists");
        row.scoreboard_packet.slo_state = FreshnessSloState::Breached;
        scoreboards.summary = scoreboards.computed_summary();
        assert!(scoreboards.validate().iter().any(|violation| matches!(
            violation,
            CohortScoreboardsViolation::HeldOnStalePacket { .. }
        )));
    }

    #[test]
    fn held_row_with_missing_required_signoff_fails() {
        let mut scoreboards = scoreboards();
        let row = scoreboards
            .rows
            .iter_mut()
            .find(|row| row.holds_scoreboard())
            .expect("a held row exists");
        let signoff = row
            .signoff_loop
            .required_signoffs
            .iter_mut()
            .find(|signoff| signoff.signed_off)
            .expect("a signed role exists");
        signoff.signed_off = false;
        signoff.signed_at = None;
        signoff.signer_ref = None;
        scoreboards.summary = scoreboards.computed_summary();
        assert!(scoreboards.validate().iter().any(|violation| matches!(
            violation,
            CohortScoreboardsViolation::HeldWithoutRequiredSignoffs { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let scoreboards = scoreboards();
        let projection = scoreboards.support_export_projection();
        assert_eq!(projection.rows.len(), scoreboards.rows.len());
        assert_eq!(
            projection.publication_decision,
            scoreboards.publication.decision
        );
        for (row, projected) in scoreboards.rows.iter().zip(&projection.rows) {
            assert_eq!(row.scoreboard_id, projected.scoreboard_id);
            assert_eq!(row.holds_stable(), projected.holds_stable);
            assert_eq!(
                row.signoff_loop.is_complete(),
                projected.signoff_loop_complete
            );
        }
    }
}
