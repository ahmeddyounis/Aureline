//! Typed stable claim matrix, launch cutline, qualification rows, and shiproom
//! stop rules.
//!
//! Stable launch-facing wording regresses when the decision to call a surface
//! "Stable" lives in prose, side spreadsheets, or optimistic badges. This
//! module makes that decision a single typed, gated record. Each
//! [`StableClaimRow`] binds one claim subject to:
//!
//! - the stable level it is being put forward as ([`StableClaimRow::claimed_level`]),
//! - a qualification row carrying its proof refs, freshness window, optional
//!   waiver, and owner sign-off ([`QualificationEvidence`], [`QualificationWaiver`],
//!   [`OwnerSignoff`]),
//! - the qualification state earned ([`QualificationState`]), the active
//!   downgrade reasons ([`DowngradeReason`]), and
//! - the level it *effectively* holds after narrowing ([`StableClaimRow::effective_level`]).
//!
//! The [`LaunchCutline`] fixes the boundary between "claimed Stable" and
//! "narrowed below Stable": a row that has not earned its claim is structurally
//! required to drop below the cutline rather than inherit an adjacent green row.
//! The [`ShiproomStopRule`] set names the closed conditions that block stable
//! promotion, and [`StableClaimMatrix::promotion`] records the resulting
//! proceed/hold verdict.
//!
//! The matrix is checked in at
//! `artifacts/release/stable_claim_matrix.json` and embedded here, so this typed
//! consumer and the CI gate agree on every row without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Date
//! arithmetic (waiver expiry and evidence staleness against an `as_of` date)
//! lives in the CI gate; this model enforces the structural and logical
//! invariants that hold regardless of the clock — narrowing consistency, the
//! no-widening rule, owner sign-off on held claims, stop-rule wiring, and the
//! promotion verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported matrix schema version.
pub const STABLE_CLAIM_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const STABLE_CLAIM_MATRIX_RECORD_KIND: &str = "stable_claim_matrix";

/// Repo-relative path to the checked-in matrix.
pub const STABLE_CLAIM_MATRIX_PATH: &str = "artifacts/release/stable_claim_matrix.json";

/// Embedded checked-in matrix JSON.
pub const STABLE_CLAIM_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stable_claim_matrix.json"
));

/// Launch level a row asserts or effectively holds, strongest to weakest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimLevel {
    /// Long-term-support stable.
    Lts,
    /// Broad stable.
    Stable,
    /// Narrowed to beta (below the stable cutline).
    Beta,
    /// Narrowed to preview (below the stable cutline).
    Preview,
    /// Claim withdrawn; no stable assertion is made.
    Withdrawn,
}

impl StableClaimLevel {
    /// Every level, strongest to weakest.
    pub const ALL: [Self; 5] = [
        Self::Lts,
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Withdrawn,
    ];

    /// Levels at or above the stable cutline, strongest first.
    pub const ABOVE_CUTLINE: [Self; 2] = [Self::Lts, Self::Stable];

    /// Levels below the stable cutline, strongest first.
    pub const BELOW_CUTLINE: [Self; 3] = [Self::Beta, Self::Preview, Self::Withdrawn];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lts => "lts",
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Strength rank; higher is a stronger claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Lts => 4,
            Self::Stable => 3,
            Self::Beta => 2,
            Self::Preview => 1,
            Self::Withdrawn => 0,
        }
    }

    /// True when this level is at or above the stable cutline.
    pub const fn is_at_or_above_cutline(self) -> bool {
        self.rank() >= Self::Stable.rank()
    }
}

/// Qualification state a row earned for its claimed level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationState {
    /// Full, current proof present with owner sign-off; holds the claimed level.
    Qualified,
    /// Holds the claimed level only because an active, unexpired waiver covers a
    /// recorded gap.
    ProvisionalOnWaiver,
    /// Required qualification proof is absent; the row must narrow.
    NotQualified,
    /// Proof exists but its freshness window expired; the row must narrow.
    EvidenceStale,
    /// The row relied on a waiver that has expired; the row must narrow.
    WaiverExpired,
}

impl QualificationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Qualified,
        Self::ProvisionalOnWaiver,
        Self::NotQualified,
        Self::EvidenceStale,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ProvisionalOnWaiver => "provisional_on_waiver",
            Self::NotQualified => "not_qualified",
            Self::EvidenceStale => "evidence_stale",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether the state lets a row hold a claim at or above the cutline.
    pub const fn holds_claim(self) -> bool {
        matches!(self, Self::Qualified | Self::ProvisionalOnWaiver)
    }

    /// Whether the state forces the row below the cutline.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_claim()
    }
}

/// Closed reason a stable claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// Required qualification proof is absent.
    QualificationEvidenceMissing,
    /// Qualification proof exists but is no longer current.
    QualificationEvidenceStale,
    /// A waiver the claim relied on has expired.
    WaiverExpired,
    /// The evidence freshness window has been exceeded.
    FreshnessWindowExceeded,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
    /// A compatibility row the claim depends on degraded.
    CompatibilityRowDegraded,
    /// A blocking defect is open against the claimed surface.
    BlockingDefectOpen,
}

impl DowngradeReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::QualificationEvidenceMissing,
        Self::QualificationEvidenceStale,
        Self::WaiverExpired,
        Self::FreshnessWindowExceeded,
        Self::OwnerSignoffMissing,
        Self::CompatibilityRowDegraded,
        Self::BlockingDefectOpen,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationEvidenceMissing => "qualification_evidence_missing",
            Self::QualificationEvidenceStale => "qualification_evidence_stale",
            Self::WaiverExpired => "waiver_expired",
            Self::FreshnessWindowExceeded => "freshness_window_exceeded",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::CompatibilityRowDegraded => "compatibility_row_degraded",
            Self::BlockingDefectOpen => "blocking_defect_open",
        }
    }
}

/// Default action a shiproom stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold stable promotion until the condition clears.
    HoldPromotion,
    /// Narrow the public claim below the cutline.
    NarrowClaim,
    /// Refresh the qualification evidence packet.
    RefreshEvidencePacket,
    /// Staff a correction lane.
    StaffCorrectionLane,
    /// Block the milestone close.
    BlockMilestoneClose,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPromotion,
        Self::NarrowClaim,
        Self::RefreshEvidencePacket,
        Self::StaffCorrectionLane,
        Self::BlockMilestoneClose,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowClaim => "narrow_claim",
            Self::RefreshEvidencePacket => "refresh_evidence_packet",
            Self::StaffCorrectionLane => "staff_correction_lane",
            Self::BlockMilestoneClose => "block_milestone_close",
        }
    }
}

/// Promotion verdict for the stable train.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionDecision {
    /// The stable train may promote.
    Proceed,
    /// Promotion is blocked by one or more firing stop rules.
    Hold,
}

impl PromotionDecision {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// The boundary between claimed-Stable and narrowed-below-Stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchCutline {
    /// The minimum level considered stable-qualified. Always `stable`.
    pub cutline_level: StableClaimLevel,
    /// Levels at or above the cutline.
    pub above_cutline_levels: Vec<StableClaimLevel>,
    /// Levels below the cutline that a narrowed row drops to.
    pub below_cutline_levels: Vec<StableClaimLevel>,
    /// Reviewable description of the cutline.
    pub description: String,
}

/// One shiproom stop rule: a closed condition that gates stable promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShiproomStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The downgrade reason whose presence on a claimed row fires this rule.
    pub trigger_reason: DowngradeReason,
    /// Claimed levels this rule watches.
    pub applies_to_levels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StopAction,
    /// Whether firing this rule blocks stable promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// Qualification proof refs and freshness window for a claim row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationEvidence {
    /// Proof packet refs backing the claim. Empty only on narrowed rows.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// The stable proof-index row this claim is registered under.
    pub proof_index_ref: String,
    /// UTC date the evidence was captured, or null when none exists yet.
    #[serde(default)]
    pub captured_at: Option<String>,
    /// Days the evidence stays claim-bearing after capture.
    pub freshness_window_days: u32,
}

/// An active or expired waiver that authorized a provisional claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationWaiver {
    /// Stable waiver ref.
    pub waiver_ref: String,
    /// UTC date the waiver expires.
    pub expires_at: String,
    /// Reviewable reason the waiver was granted.
    pub reason: String,
}

/// Owner sign-off on a claim row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerSignoff {
    /// Owning team or role.
    pub owner_ref: String,
    /// Whether the owner has signed off the stable claim.
    pub signed_off: bool,
    /// UTC date of sign-off, or null when not signed off.
    #[serde(default)]
    pub signed_at: Option<String>,
}

/// One stable claim row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableClaimRow {
    /// Stable claim id.
    pub claim_id: String,
    /// Human-readable title.
    pub title: String,
    /// Claim subject family the row speaks for.
    pub subject_family: String,
    /// The stable level the row is put forward as. Always at or above cutline.
    pub claimed_level: StableClaimLevel,
    /// Qualification state earned for the claimed level.
    pub qualification_state: QualificationState,
    /// Qualification proof and freshness window.
    pub evidence: QualificationEvidence,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active downgrade reasons narrowing the row.
    #[serde(default)]
    pub active_downgrade_reasons: Vec<DowngradeReason>,
    /// The level the row effectively holds after narrowing.
    pub effective_level: StableClaimLevel,
    /// Publication destinations that render this row.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Upstream cutline rows this stable claim inherits from.
    #[serde(default)]
    pub cutline_row_refs: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl StableClaimRow {
    /// True when the row's effective level is at or above the cutline.
    pub fn holds_stable(&self) -> bool {
        self.effective_level.is_at_or_above_cutline()
    }

    /// True when a downgrade reason is active on the row.
    pub fn has_active_reason(&self, reason: DowngradeReason) -> bool {
        self.active_downgrade_reasons.contains(&reason)
    }
}

/// The recorded promotion verdict for the stable train.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionDecisionRecord {
    /// The gate this verdict governs.
    pub promotion_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Stop-rule ids that block promotion, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Claim ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_claim_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableClaimMatrixSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Rows whose effective level is at or above the cutline.
    pub rows_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub rows_narrowed_below_cutline: usize,
    /// Rows holding a claim via an active waiver.
    pub rows_on_active_waiver: usize,
    /// Total active downgrade reasons across all rows.
    pub total_active_downgrade_reasons: usize,
    /// Number of stop rules currently firing.
    pub stop_rules_firing: usize,
}

/// The typed stable claim matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableClaimMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix identifier.
    pub matrix_id: String,
    /// Lifecycle status of this matrix artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed level vocabulary.
    pub claim_levels: Vec<StableClaimLevel>,
    /// Closed qualification-state vocabulary.
    pub qualification_states: Vec<QualificationState>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed stop-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// Shiproom stop rules.
    pub stop_rules: Vec<ShiproomStopRule>,
    /// Stable claim rows.
    pub rows: Vec<StableClaimRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: StableClaimMatrixSummary,
}

impl StableClaimMatrix {
    /// Returns the row registered for `claim_id`.
    pub fn row(&self, claim_id: &str) -> Option<&StableClaimRow> {
        self.rows.iter().find(|row| row.claim_id == claim_id)
    }

    /// Returns the rows whose effective level is at or above the cutline.
    pub fn rows_holding_stable(&self) -> Vec<&StableClaimRow> {
        self.rows.iter().filter(|row| row.holds_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&StableClaimRow> {
        self.rows.iter().filter(|row| !row.holds_stable()).collect()
    }

    /// True when `rule` fires: a claimed row in its watch set carries its
    /// trigger reason.
    pub fn stop_rule_fires(&self, rule: &ShiproomStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_levels.contains(&row.claimed_level)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the rows and stop rules.
    pub fn computed_promotion_decision(&self) -> PromotionDecision {
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

    /// Stop-rule ids that block promotion and are currently firing, sorted.
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

    /// Claim ids that trigger a blocking, firing stop rule, sorted and unique.
    pub fn computed_blocking_claim_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<DowngradeReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claimed_level.is_at_or_above_cutline()
                && row
                    .active_downgrade_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.claim_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> StableClaimMatrixSummary {
        StableClaimMatrixSummary {
            total_rows: self.rows.len(),
            rows_holding_stable: self.rows.iter().filter(|row| row.holds_stable()).count(),
            rows_narrowed_below_cutline: self.rows.iter().filter(|row| !row.holds_stable()).count(),
            rows_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.qualification_state == QualificationState::ProvisionalOnWaiver)
                .count(),
            total_active_downgrade_reasons: self
                .rows
                .iter()
                .map(|row| row.active_downgrade_reasons.len())
                .sum(),
            stop_rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the matrix that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> StableClaimExportProjection {
        StableClaimExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| StableClaimExportRow {
                    claim_id: row.claim_id.clone(),
                    subject_family: row.subject_family.clone(),
                    claimed_level: row.claimed_level,
                    effective_level: row.effective_level,
                    holds_stable: row.holds_stable(),
                    qualification_state: row.qualification_state,
                    active_downgrade_reasons: row.active_downgrade_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<StableClaimMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.claim_id.clone()) {
                violations.push(StableClaimMatrixViolation::DuplicateClaimId {
                    claim_id: row.claim_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(StableClaimMatrixViolation::EmptyMatrix);
        }

        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(StableClaimMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<StableClaimMatrixViolation>) {
        if self.schema_version != STABLE_CLAIM_MATRIX_SCHEMA_VERSION {
            violations.push(StableClaimMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != STABLE_CLAIM_MATRIX_RECORD_KIND {
            violations.push(StableClaimMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(StableClaimMatrixViolation::EmptyField {
                    claim_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.claim_levels != StableClaimLevel::ALL.to_vec() {
            violations.push(StableClaimMatrixViolation::ClosedVocabularyMismatch {
                field: "claim_levels",
            });
        }
        if self.qualification_states != QualificationState::ALL.to_vec() {
            violations.push(StableClaimMatrixViolation::ClosedVocabularyMismatch {
                field: "qualification_states",
            });
        }
        if self.downgrade_reasons != DowngradeReason::ALL.to_vec() {
            violations.push(StableClaimMatrixViolation::ClosedVocabularyMismatch {
                field: "downgrade_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(StableClaimMatrixViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(StableClaimMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(StableClaimMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(StableClaimMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(StableClaimMatrixViolation::EmptyField {
                claim_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<StableClaimMatrixViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(StableClaimMatrixViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered_triggers = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(StableClaimMatrixViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(StableClaimMatrixViolation::EmptyField {
                        claim_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_levels.is_empty() {
                violations.push(StableClaimMatrixViolation::StopRuleWithoutLevels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered_triggers.insert(rule.trigger_reason);
        }

        // Every downgrade reason must have a stop rule, so a narrowing reason
        // cannot fire without a corresponding promotion gate.
        for reason in DowngradeReason::ALL {
            if !covered_triggers.contains(&reason) {
                violations
                    .push(StableClaimMatrixViolation::DowngradeReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &StableClaimRow, violations: &mut Vec<StableClaimMatrixViolation>) {
        for (field, value) in [
            ("claim_id", &row.claim_id),
            ("title", &row.title),
            ("subject_family", &row.subject_family),
            ("rationale", &row.rationale),
            ("evidence.proof_index_ref", &row.evidence.proof_index_ref),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableClaimMatrixViolation::EmptyField {
                    claim_id: row.claim_id.clone(),
                    field_name: field,
                });
            }
        }

        // A stable-claim row must assert a level at or above the cutline.
        if !row.claimed_level.is_at_or_above_cutline() {
            violations.push(StableClaimMatrixViolation::ClaimedLevelBelowCutline {
                claim_id: row.claim_id.clone(),
                claimed: row.claimed_level,
            });
        }

        // No widening: the effective level may not be stronger than claimed.
        if row.effective_level.rank() > row.claimed_level.rank() {
            violations.push(StableClaimMatrixViolation::EffectiveWiderThanClaimed {
                claim_id: row.claim_id.clone(),
                claimed: row.claimed_level,
                effective: row.effective_level,
            });
        }

        if row.evidence.freshness_window_days == 0 {
            violations.push(StableClaimMatrixViolation::EmptyField {
                claim_id: row.claim_id.clone(),
                field_name: "evidence.freshness_window_days",
            });
        }

        // Acceptance core: a state that forces narrowing must drop the row
        // below the cutline and name at least one active reason.
        if row.qualification_state.forces_narrowing() {
            if row.holds_stable() {
                violations.push(StableClaimMatrixViolation::EffectiveLevelNotNarrowed {
                    claim_id: row.claim_id.clone(),
                    state: row.qualification_state,
                    effective: row.effective_level,
                });
            }
            if row.active_downgrade_reasons.is_empty() {
                violations.push(StableClaimMatrixViolation::NarrowingWithoutReason {
                    claim_id: row.claim_id.clone(),
                    state: row.qualification_state,
                });
            }
        }

        // Acceptance core: a row holding a stable claim must have current,
        // proof-backed, owner-signed qualification with no active reason.
        if row.holds_stable() {
            if row.qualification_state.forces_narrowing() {
                violations.push(StableClaimMatrixViolation::StableClaimWithNarrowingState {
                    claim_id: row.claim_id.clone(),
                    state: row.qualification_state,
                });
            }
            if !row.active_downgrade_reasons.is_empty() {
                violations.push(StableClaimMatrixViolation::StableClaimWithActiveDowngrade {
                    claim_id: row.claim_id.clone(),
                });
            }
            if row.evidence.evidence_refs.is_empty() {
                violations.push(StableClaimMatrixViolation::StableClaimWithoutEvidence {
                    claim_id: row.claim_id.clone(),
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(StableClaimMatrixViolation::StableClaimWithoutSignoff {
                    claim_id: row.claim_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &StableClaimRow,
        violations: &mut Vec<StableClaimMatrixViolation>,
    ) {
        let require = |violations: &mut Vec<StableClaimMatrixViolation>,
                       reason: DowngradeReason| {
            if !row.has_active_reason(reason) {
                violations.push(StableClaimMatrixViolation::StateReasonIncoherent {
                    claim_id: row.claim_id.clone(),
                    state: row.qualification_state,
                    expected_reason: reason,
                });
            }
        };

        match row.qualification_state {
            QualificationState::NotQualified => {
                require(violations, DowngradeReason::QualificationEvidenceMissing);
            }
            QualificationState::EvidenceStale => {
                if !(row.has_active_reason(DowngradeReason::QualificationEvidenceStale)
                    || row.has_active_reason(DowngradeReason::FreshnessWindowExceeded))
                {
                    violations.push(StableClaimMatrixViolation::StateReasonIncoherent {
                        claim_id: row.claim_id.clone(),
                        state: row.qualification_state,
                        expected_reason: DowngradeReason::QualificationEvidenceStale,
                    });
                }
            }
            QualificationState::WaiverExpired => {
                require(violations, DowngradeReason::WaiverExpired);
                if row.waiver.is_none() {
                    violations.push(StableClaimMatrixViolation::WaiverStateWithoutWaiver {
                        claim_id: row.claim_id.clone(),
                        state: row.qualification_state,
                    });
                }
            }
            QualificationState::ProvisionalOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|waiver| {
                        waiver.waiver_ref.trim().is_empty() || waiver.expires_at.trim().is_empty()
                    })
                    .unwrap_or(true)
                {
                    violations.push(StableClaimMatrixViolation::WaiverStateWithoutWaiver {
                        claim_id: row.claim_id.clone(),
                        state: row.qualification_state,
                    });
                }
            }
            QualificationState::Qualified => {}
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<StableClaimMatrixViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(StableClaimMatrixViolation::EmptyField {
                claim_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(StableClaimMatrixViolation::EmptyField {
                claim_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(StableClaimMatrixViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(StableClaimMatrixViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_claim_ids() {
            violations.push(StableClaimMatrixViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A redaction-safe export row projected from the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableClaimExportRow {
    /// Stable claim id.
    pub claim_id: String,
    /// Claim subject family.
    pub subject_family: String,
    /// Level the row is put forward as.
    pub claimed_level: StableClaimLevel,
    /// Level the row effectively holds.
    pub effective_level: StableClaimLevel,
    /// Whether the row holds a stable claim.
    pub holds_stable: bool,
    /// Qualification state.
    pub qualification_state: QualificationState,
    /// Active downgrade reasons.
    pub active_downgrade_reasons: Vec<DowngradeReason>,
}

/// A redaction-safe export projection of the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableClaimExportProjection {
    /// Matrix id this projection was produced from.
    pub matrix_id: String,
    /// Matrix as-of date.
    pub as_of: String,
    /// Promotion verdict.
    pub promotion_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<StableClaimExportRow>,
}

/// A validation violation for the stable claim matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableClaimMatrixViolation {
    /// The matrix carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the matrix.
        actual: u32,
    },
    /// The matrix carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the matrix.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The matrix has no rows.
    EmptyMatrix,
    /// The matrix has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        claim_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A claim id appears more than once.
    DuplicateClaimId {
        /// Duplicate claim id.
        claim_id: String,
    },
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule names no levels to watch.
    StopRuleWithoutLevels {
        /// Rule id.
        rule_id: String,
    },
    /// A downgrade reason has no stop rule watching for it.
    DowngradeReasonWithoutStopRule {
        /// Uncovered reason.
        reason: DowngradeReason,
    },
    /// A claim row asserts a level below the cutline.
    ClaimedLevelBelowCutline {
        /// Row id.
        claim_id: String,
        /// Claimed level.
        claimed: StableClaimLevel,
    },
    /// An effective level is stronger than the claimed level.
    EffectiveWiderThanClaimed {
        /// Row id.
        claim_id: String,
        /// Claimed level.
        claimed: StableClaimLevel,
        /// Effective level.
        effective: StableClaimLevel,
    },
    /// A narrowing state did not drop the row below the cutline.
    EffectiveLevelNotNarrowed {
        /// Row id.
        claim_id: String,
        /// Qualification state.
        state: QualificationState,
        /// Effective level.
        effective: StableClaimLevel,
    },
    /// A narrowing state carries no active downgrade reason.
    NarrowingWithoutReason {
        /// Row id.
        claim_id: String,
        /// Qualification state.
        state: QualificationState,
    },
    /// A row holds a stable claim while its state forces narrowing.
    StableClaimWithNarrowingState {
        /// Row id.
        claim_id: String,
        /// Qualification state.
        state: QualificationState,
    },
    /// A row holds a stable claim while a downgrade reason is active.
    StableClaimWithActiveDowngrade {
        /// Row id.
        claim_id: String,
    },
    /// A row holds a stable claim with no qualification evidence.
    StableClaimWithoutEvidence {
        /// Row id.
        claim_id: String,
    },
    /// A row holds a stable claim without owner sign-off.
    StableClaimWithoutSignoff {
        /// Row id.
        claim_id: String,
    },
    /// A qualification state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        claim_id: String,
        /// Qualification state.
        state: QualificationState,
        /// Reason the state requires.
        expected_reason: DowngradeReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        claim_id: String,
        /// Qualification state.
        state: QualificationState,
    },
    /// The declared promotion decision disagrees with the computed one.
    PromotionDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared promotion blocking set disagrees with the computed one.
    PromotionBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for StableClaimMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported matrix schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported matrix record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "matrix {field} is not the canonical value")
            }
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
            Self::NoStopRules => write!(f, "matrix has no shiproom stop rules"),
            Self::EmptyField {
                claim_id,
                field_name,
            } => write!(f, "{claim_id} has empty field {field_name}"),
            Self::DuplicateClaimId { claim_id } => {
                write!(f, "duplicate claim row id {claim_id}")
            }
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::StopRuleWithoutLevels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no levels")
            }
            Self::DowngradeReasonWithoutStopRule { reason } => write!(
                f,
                "downgrade reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::ClaimedLevelBelowCutline { claim_id, claimed } => write!(
                f,
                "claim {claim_id} asserts level {} which is below the stable cutline",
                claimed.as_str()
            ),
            Self::EffectiveWiderThanClaimed {
                claim_id,
                claimed,
                effective,
            } => write!(
                f,
                "claim {claim_id} effective level {} is wider than claimed level {}",
                effective.as_str(),
                claimed.as_str()
            ),
            Self::EffectiveLevelNotNarrowed {
                claim_id,
                state,
                effective,
            } => write!(
                f,
                "claim {claim_id} state {} must narrow below the cutline but holds {}",
                state.as_str(),
                effective.as_str()
            ),
            Self::NarrowingWithoutReason { claim_id, state } => write!(
                f,
                "claim {claim_id} state {} narrows without naming an active downgrade reason",
                state.as_str()
            ),
            Self::StableClaimWithNarrowingState { claim_id, state } => write!(
                f,
                "claim {claim_id} holds stable while its state {} forces narrowing",
                state.as_str()
            ),
            Self::StableClaimWithActiveDowngrade { claim_id } => write!(
                f,
                "claim {claim_id} holds stable while a downgrade reason is active"
            ),
            Self::StableClaimWithoutEvidence { claim_id } => {
                write!(
                    f,
                    "claim {claim_id} holds stable with no qualification evidence"
                )
            }
            Self::StableClaimWithoutSignoff { claim_id } => {
                write!(f, "claim {claim_id} holds stable without owner sign-off")
            }
            Self::StateReasonIncoherent {
                claim_id,
                state,
                expected_reason,
            } => write!(
                f,
                "claim {claim_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { claim_id, state } => write!(
                f,
                "claim {claim_id} state {} names no waiver",
                state.as_str()
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => write!(
                f,
                "promotion decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with the firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "matrix summary counts disagree with the rows"),
        }
    }
}

impl Error for StableClaimMatrixViolation {}

/// Loads the embedded stable claim matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`StableClaimMatrix`] — including when a row carries a level, qualification
/// state, downgrade reason, or stop action outside the closed vocabularies.
pub fn current_stable_claim_matrix() -> Result<StableClaimMatrix, serde_json::Error> {
    serde_json::from_str(STABLE_CLAIM_MATRIX_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> StableClaimMatrix {
        current_stable_claim_matrix().expect("matrix parses")
    }

    #[test]
    fn embedded_matrix_parses_and_validates() {
        let matrix = matrix();
        assert_eq!(matrix.schema_version, STABLE_CLAIM_MATRIX_SCHEMA_VERSION);
        assert_eq!(matrix.record_kind, STABLE_CLAIM_MATRIX_RECORD_KIND);
        assert_eq!(matrix.validate(), Vec::new());
        assert!(!matrix.rows.is_empty());
    }

    #[test]
    fn cutline_partitions_levels() {
        for level in StableClaimLevel::ABOVE_CUTLINE {
            assert!(level.is_at_or_above_cutline(), "{}", level.as_str());
        }
        for level in StableClaimLevel::BELOW_CUTLINE {
            assert!(!level.is_at_or_above_cutline(), "{}", level.as_str());
        }
    }

    #[test]
    fn matrix_exercises_holding_and_narrowed_rows() {
        let matrix = matrix();
        assert!(
            !matrix.rows_holding_stable().is_empty(),
            "matrix must show at least one held stable claim"
        );
        assert!(
            !matrix.rows_narrowed().is_empty(),
            "matrix must show at least one narrowed claim"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let matrix = matrix();
        assert_eq!(matrix.summary, matrix.computed_summary());
        assert_eq!(
            matrix.summary.rows_holding_stable + matrix.summary.rows_narrowed_below_cutline,
            matrix.rows.len()
        );
    }

    #[test]
    fn promotion_holds_when_a_blocking_stop_rule_fires() {
        let matrix = matrix();
        // The frozen matrix carries narrowed rows whose reasons fire blocking
        // stop rules, so the stable train is held.
        assert_eq!(matrix.promotion.decision, PromotionDecision::Hold);
        assert_eq!(
            matrix.promotion.decision,
            matrix.computed_promotion_decision()
        );
        assert!(!matrix.promotion.blocking_rule_ids.is_empty());
        assert!(!matrix.promotion.blocking_claim_ids.is_empty());
    }

    #[test]
    fn every_downgrade_reason_has_a_stop_rule() {
        let matrix = matrix();
        let covered: BTreeSet<DowngradeReason> = matrix
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in DowngradeReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_stable_claim_with_active_downgrade() {
        let mut matrix = matrix();
        // Force an active reason onto a held stable row without narrowing it.
        let row = matrix
            .rows
            .iter_mut()
            .find(|row| row.holds_stable())
            .expect("a held stable row exists");
        row.active_downgrade_reasons
            .push(DowngradeReason::CompatibilityRowDegraded);
        let claim_id = row.claim_id.clone();
        matrix.summary = matrix.computed_summary();
        assert!(matrix
            .validate()
            .contains(&StableClaimMatrixViolation::StableClaimWithActiveDowngrade { claim_id }));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut matrix = matrix();
        // Pretend an unqualified row still holds its claimed stable level.
        let row = matrix
            .rows
            .iter_mut()
            .find(|row| row.qualification_state == QualificationState::NotQualified)
            .expect("a not-qualified row exists");
        row.effective_level = row.claimed_level;
        matrix.summary = matrix.computed_summary();
        matrix.promotion.decision = matrix.computed_promotion_decision();
        matrix.promotion.blocking_rule_ids = matrix.computed_blocking_rule_ids();
        matrix.promotion.blocking_claim_ids = matrix.computed_blocking_claim_ids();
        assert!(matrix.validate().iter().any(|violation| matches!(
            violation,
            StableClaimMatrixViolation::EffectiveLevelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_promotion_decision() {
        let mut matrix = matrix();
        matrix.promotion.decision = PromotionDecision::Proceed;
        assert!(matrix.validate().iter().any(|violation| matches!(
            violation,
            StableClaimMatrixViolation::PromotionDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_claim_without_signoff() {
        let mut matrix = matrix();
        let row = matrix
            .rows
            .iter_mut()
            .find(|row| row.holds_stable())
            .expect("a held stable row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let claim_id = row.claim_id.clone();
        matrix.summary = matrix.computed_summary();
        assert!(matrix
            .validate()
            .contains(&StableClaimMatrixViolation::StableClaimWithoutSignoff { claim_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let matrix = matrix();
        let projection = matrix.support_export_projection();
        assert_eq!(projection.rows.len(), matrix.rows.len());
        assert_eq!(projection.promotion_decision, matrix.promotion.decision);
        for (row, projected) in matrix.rows.iter().zip(&projection.rows) {
            assert_eq!(row.claim_id, projected.claim_id);
            assert_eq!(row.holds_stable(), projected.holds_stable);
            assert_eq!(row.effective_level, projected.effective_level);
        }
    }
}
