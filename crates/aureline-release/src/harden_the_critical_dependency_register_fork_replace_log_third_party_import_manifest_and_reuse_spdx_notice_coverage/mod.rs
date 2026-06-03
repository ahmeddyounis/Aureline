//! Harden the critical dependency register, fork/replace log, third-party import
//! manifest, and REUSE/SPDX/notice coverage.
//!
//! Where the [`stable_claim_manifest`](crate::stable_claim_manifest) decides the
//! single canonical lifecycle label each *subject* publishes, this module answers
//! the question: **for each governance lane — critical dependency register,
//! fork/replace log, third-party import manifest, and REUSE/SPDX/notice coverage —
//! is the lane current, grounded in a signed or governed packet, and narrowed below
//! the cutline the moment its backing thins out?** This module is the **dependency
//! and licensing governance register**. For every lane it records one row that binds
//! the lane to the [`stable_claim_manifest`](crate::stable_claim_manifest) entry
//! whose lifecycle label it backs, the proof packet that grounds it, and the waiver
//! (if any) holding it provisionally.
//!
//! Each [`LaneRow`] is one `(governance lane, public claim)` binding. It:
//!
//! - names the lane it governs ([`LaneRow::lane_kind`],
//!   [`LaneRow::subject_ref`], [`LaneRow::subject_summary`])
//!   and whether that lane is part of the release-blocking set
//!   ([`LaneRow::release_blocking`]);
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO and
//!   the evidence refs that ground the lane's claim;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry
//!   whose public claim it backs ([`LaneRow::claim_ref`]) and the canonical
//!   lifecycle label that entry publishes ([`LaneRow::claim_label`]);
//! - records the lane state earned ([`LaneState`]), the active gap reasons
//!   ([`LaneGapReason`]), and the effective label after narrowing
//!   ([`LaneRow::effective_label`]);
//! - carries an owner sign-off and optional waiver so a provisionally held
//!   lane can still block promotion if the waiver expires or sign-off is missing.
//!
//! The [`LaneRule`] set names the closed conditions that gate publication, and
//! [`HardenCriticalDependencyRegister::publication`] records the resulting
//! proceed/hold verdict.
//!
//! The artifact is checked in at
//! `artifacts/release/harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! Date arithmetic lives in the CI gate; this model enforces the structural and
//! logical invariants that hold regardless of the clock.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::ProofPacket;
use crate::stable_claim_matrix::{OwnerSignoff, QualificationWaiver};

/// Supported artifact schema version.
pub const HARDEN_CRITICAL_DEPENDENCY_REGISTER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the artifact.
pub const HARDEN_CRITICAL_DEPENDENCY_REGISTER_RECORD_KIND: &str =
    "harden_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage";

/// Repo-relative path to the checked-in artifact.
pub const HARDEN_CRITICAL_DEPENDENCY_REGISTER_PATH: &str =
    "artifacts/release/harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage.json";

/// Embedded checked-in artifact JSON.
pub const HARDEN_CRITICAL_DEPENDENCY_REGISTER_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage.json"
));

/// Governance lane kind governed by this register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneKind {
    /// Critical upstream dependency register (libraries, tools, services).
    CriticalDependencyRegister,
    /// Fork/replace log tracking diverged forks and upstream replacements.
    ForkReplaceLog,
    /// Third-party import manifest (direct and transitive imports).
    ThirdPartyImportManifest,
    /// REUSE/SPDX/notice coverage register.
    ReuseSpdxNoticeCoverage,
}

impl LaneKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CriticalDependencyRegister,
        Self::ForkReplaceLog,
        Self::ThirdPartyImportManifest,
        Self::ReuseSpdxNoticeCoverage,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CriticalDependencyRegister => "critical_dependency_register",
            Self::ForkReplaceLog => "fork_replace_log",
            Self::ThirdPartyImportManifest => "third_party_import_manifest",
            Self::ReuseSpdxNoticeCoverage => "reuse_spdx_notice_coverage",
        }
    }
}

/// Lane state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneState {
    /// The lane is current: a captured, within-SLO proof packet and complete
    /// evidence back the public claim at its full canonical lifecycle label.
    Current,
    /// The lane carries the claim's full label only because an active,
    /// unexpired waiver covers a recorded residual gap.
    OnWaiver,
    /// The proof packet breached its freshness SLO (or is missing); the label
    /// must narrow.
    NarrowedStale,
    /// No proof packet has been captured; the label must narrow.
    NarrowedMissing,
    /// Required evidence is incomplete or the artifact is unverified; the label
    /// must narrow.
    NarrowedUnbacked,
    /// The row relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
}

impl LaneState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Current,
        Self::OnWaiver,
        Self::NarrowedStale,
        Self::NarrowedMissing,
        Self::NarrowedUnbacked,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::OnWaiver => "on_waiver",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedMissing => "narrowed_missing",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets the lane carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Current | Self::OnWaiver)
    }

    /// Whether the state forces the lane below the public claim label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a lane narrows or a rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneGapReason {
    /// The backing public claim narrowed below the cutline.
    ClaimLabelNarrowed,
    /// No proof packet has been captured.
    PacketMissing,
    /// The proof packet breached its freshness SLO.
    PacketFreshnessBreached,
    /// Required evidence is incomplete.
    EvidenceIncomplete,
    /// The artifact is unverified or incomplete.
    ArtifactUnverified,
    /// A dependency audit gap is open (out-of-support, unaudited, or vulnerable).
    DependencyAuditGap,
    /// A fork has diverged from its upstream baseline or a replacement is incomplete.
    ForkDivergence,
    /// A third-party import is unmapped or carries an incompatible license.
    ImportMappingFailed,
    /// REUSE/SPDX or human-readable notice coverage is incomplete or nonconformant.
    LicenseCoverageGap,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// The required row owner sign-off is missing.
    OwnerSignoffMissing,
}

impl LaneGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ClaimLabelNarrowed,
        Self::PacketMissing,
        Self::PacketFreshnessBreached,
        Self::EvidenceIncomplete,
        Self::ArtifactUnverified,
        Self::DependencyAuditGap,
        Self::ForkDivergence,
        Self::ImportMappingFailed,
        Self::LicenseCoverageGap,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::PacketMissing => "packet_missing",
            Self::PacketFreshnessBreached => "packet_freshness_breached",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::ArtifactUnverified => "artifact_unverified",
            Self::DependencyAuditGap => "dependency_audit_gap",
            Self::ForkDivergence => "fork_divergence",
            Self::ImportMappingFailed => "import_mapping_failed",
            Self::LicenseCoverageGap => "license_coverage_gap",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the stable cutline.
    NarrowClaim,
    /// Refresh the proof packet.
    RefreshPacket,
    /// Recapture the evidence the proof packet depends on.
    RecaptureEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl LaneAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPublication,
        Self::NarrowClaim,
        Self::RefreshPacket,
        Self::RecaptureEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowClaim => "narrow_claim",
            Self::RefreshPacket => "refresh_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication verdict for the dependency and licensing governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// The lane may publish.
    Proceed,
    /// Publication is blocked by one or more firing rules.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// One governance lane row in the dependency and licensing register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The governance lane this row governs.
    pub lane_kind: LaneKind,
    /// Ref to the subject (document, manifest, or seed) this row speaks for.
    pub subject_ref: String,
    /// Human-readable summary of the subject.
    pub subject_summary: String,
    /// Whether this lane is part of the release-blocking set.
    pub release_blocking: bool,
    /// Ref to the stable claim manifest entry this row backs.
    pub claim_ref: String,
    /// Canonical lifecycle label the claim entry publishes.
    pub claim_label: String,
    /// Lane state earned.
    pub lane_state: LaneState,
    /// Proof packet grounding this row.
    pub proof_packet: ProofPacket,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Waiver authorizing a provisional hold, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<LaneGapReason>,
    /// The label the row effectively publishes after narrowing.
    pub effective_label: String,
    /// Publication destinations that render this row.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl LaneRow {
    /// True when the row's effective label is not a narrowing label.
    ///
    /// A row holds its claim when its state is `Current` or `OnWaiver` and its
    /// effective label matches its claim label.
    pub fn holds_claim(&self) -> bool {
        self.lane_state.holds_label() && self.effective_label == self.claim_label
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: LaneGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// One rule: a closed condition that narrows a claim and may gate publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a claimed row fires this rule.
    pub trigger_reason: LaneGapReason,
    /// Lane states this rule watches.
    pub applies_to_states: Vec<LaneState>,
    /// Default action prescribed when the rule fires.
    pub default_action: LaneAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// The recorded publication verdict for the dependency and licensing governance lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationDecisionRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PublicationDecision,
    /// Rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_row_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HardenCriticalDependencyRegisterSummary {
    /// Total number of lane rows.
    pub total_rows: usize,
    /// Rows effectively holding their claim.
    pub rows_holding_claim: usize,
    /// Rows narrowed below their claim.
    pub rows_narrowed: usize,
    /// Rows holding claim via an active waiver.
    pub rows_on_active_waiver: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of rules currently firing.
    pub rules_firing: usize,
}

/// The typed dependency and licensing governance artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HardenCriticalDependencyRegister {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable artifact identifier.
    pub artifact_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this artifact ingests.
    pub claim_manifest_ref: String,
    /// Ref to the stable proof index this artifact ingests.
    pub stable_proof_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<String>,
    /// Closed lane-kind vocabulary.
    pub lane_kinds: Vec<LaneKind>,
    /// Closed lane-state vocabulary.
    pub lane_states: Vec<LaneState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<LaneGapReason>,
    /// Closed lane-action vocabulary.
    pub lane_actions: Vec<LaneAction>,
    /// Release-blocking lane refs.
    pub release_blocking_lane_refs: Vec<String>,
    /// Governance lane rows.
    pub rows: Vec<LaneRow>,
    /// Downgrade rules.
    pub rules: Vec<LaneRule>,
    /// Recorded publication verdict.
    pub publication: PublicationDecisionRecord,
    /// Summary counts.
    pub summary: HardenCriticalDependencyRegisterSummary,
}

impl HardenCriticalDependencyRegister {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&LaneRow> {
        self.rows.iter().find(|r| r.entry_id == entry_id)
    }

    /// Returns the rows effectively holding their claim.
    pub fn rows_holding_claim(&self) -> Vec<&LaneRow> {
        self.rows.iter().filter(|r| r.holds_claim()).collect()
    }

    /// Returns the rows narrowed below their claim.
    pub fn rows_narrowed(&self) -> Vec<&LaneRow> {
        self.rows.iter().filter(|r| !r.holds_claim()).collect()
    }

    /// True when `rule` fires: a claimed row in its watch set carries its
    /// trigger reason.
    pub fn rule_fires(&self, rule: &LaneRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_states.contains(&row.lane_state)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and rules.
    pub fn computed_publication_decision(&self) -> PublicationDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_publication && self.rule_fires(rule))
        {
            PublicationDecision::Hold
        } else {
            PublicationDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
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

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    pub fn computed_blocking_row_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<LaneGapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row
                .active_gap_reasons
                .iter()
                .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and rules.
    pub fn computed_summary(&self) -> HardenCriticalDependencyRegisterSummary {
        HardenCriticalDependencyRegisterSummary {
            total_rows: self.rows.len(),
            rows_holding_claim: self.rows.iter().filter(|r| r.holds_claim()).count(),
            rows_narrowed: self.rows.iter().filter(|r| !r.holds_claim()).count(),
            rows_on_active_waiver: self
                .rows
                .iter()
                .filter(|r| r.lane_state == LaneState::OnWaiver)
                .count(),
            total_active_gap_reasons: self.rows.iter().map(|r| r.active_gap_reasons.len()).sum(),
            rules_firing: self
                .rules
                .iter()
                .filter(|rule| self.rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the artifact that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> HardenCriticalDependencyRegisterExportProjection {
        HardenCriticalDependencyRegisterExportProjection {
            artifact_id: self.artifact_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| LaneExportRow {
                    entry_id: row.entry_id.clone(),
                    lane_kind: row.lane_kind,
                    claim_label: row.claim_label.clone(),
                    effective_label: row.effective_label.clone(),
                    holds_claim: row.holds_claim(),
                    lane_state: row.lane_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the artifact, returning every violation found.
    pub fn validate(&self) -> Vec<HardenCriticalDependencyRegisterViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(HardenCriticalDependencyRegisterViolation::DuplicateRowId {
                    row_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(HardenCriticalDependencyRegisterViolation::EmptyRows);
        }

        // Every lane kind must appear at least once.
        let present_kinds: BTreeSet<LaneKind> = self.rows.iter().map(|r| r.lane_kind).collect();
        for kind in LaneKind::ALL {
            if !present_kinds.contains(&kind) {
                violations
                    .push(HardenCriticalDependencyRegisterViolation::LaneKindMissing { kind });
            }
        }

        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(HardenCriticalDependencyRegisterViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<HardenCriticalDependencyRegisterViolation>) {
        if self.schema_version != HARDEN_CRITICAL_DEPENDENCY_REGISTER_SCHEMA_VERSION {
            violations.push(
                HardenCriticalDependencyRegisterViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != HARDEN_CRITICAL_DEPENDENCY_REGISTER_RECORD_KIND {
            violations.push(
                HardenCriticalDependencyRegisterViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("artifact_id", &self.artifact_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("stable_proof_index_ref", &self.stable_proof_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(HardenCriticalDependencyRegisterViolation::EmptyField {
                    id: "<artifact>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lane_kinds != LaneKind::ALL.to_vec() {
            violations.push(
                HardenCriticalDependencyRegisterViolation::ClosedVocabularyMismatch {
                    field: "lane_kinds",
                },
            );
        }
        if self.lane_states != LaneState::ALL.to_vec() {
            violations.push(
                HardenCriticalDependencyRegisterViolation::ClosedVocabularyMismatch {
                    field: "lane_states",
                },
            );
        }
        if self.gap_reasons != LaneGapReason::ALL.to_vec() {
            violations.push(
                HardenCriticalDependencyRegisterViolation::ClosedVocabularyMismatch {
                    field: "gap_reasons",
                },
            );
        }
        if self.lane_actions != LaneAction::ALL.to_vec() {
            violations.push(
                HardenCriticalDependencyRegisterViolation::ClosedVocabularyMismatch {
                    field: "lane_actions",
                },
            );
        }
    }

    fn validate_rules(&self, violations: &mut Vec<HardenCriticalDependencyRegisterViolation>) {
        if self.rules.is_empty() {
            violations.push(HardenCriticalDependencyRegisterViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(HardenCriticalDependencyRegisterViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(HardenCriticalDependencyRegisterViolation::EmptyField {
                        id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_states.is_empty() {
                violations.push(
                    HardenCriticalDependencyRegisterViolation::RuleWithoutStates {
                        rule_id: rule.rule_id.clone(),
                    },
                );
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in LaneGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    HardenCriticalDependencyRegisterViolation::GapReasonWithoutRule { reason },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &LaneRow,
        violations: &mut Vec<HardenCriticalDependencyRegisterViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("subject_ref", &row.subject_ref),
            ("subject_summary", &row.subject_summary),
            ("rationale", &row.rationale),
            ("claim_ref", &row.claim_ref),
            ("claim_label", &row.claim_label),
            ("effective_label", &row.effective_label),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(HardenCriticalDependencyRegisterViolation::EmptyField {
                    id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // No widening: effective_label may not be wider than claim_label when
        // claim_label is below stable. We approximate this by checking that a
        // narrowing state does not retain the claim label.
        if row.lane_state.forces_narrowing() {
            if row.effective_label == row.claim_label {
                violations.push(
                    HardenCriticalDependencyRegisterViolation::EffectiveLabelNotNarrowed {
                        row_id: row.entry_id.clone(),
                        state: row.lane_state,
                        claim_label: row.claim_label.clone(),
                        effective_label: row.effective_label.clone(),
                    },
                );
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(
                    HardenCriticalDependencyRegisterViolation::NarrowingWithoutReason {
                        row_id: row.entry_id.clone(),
                        state: row.lane_state,
                    },
                );
            }
        }

        // A current or on-waiver row must have owner sign-off and no active
        // gap reasons.
        if row.lane_state.holds_label() {
            if !row.active_gap_reasons.is_empty() {
                violations.push(
                    HardenCriticalDependencyRegisterViolation::HeldRowWithActiveGap {
                        row_id: row.entry_id.clone(),
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(
                    HardenCriticalDependencyRegisterViolation::HeldRowWithoutSignoff {
                        row_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_row_state_reason_coherence(row, violations);
    }

    fn validate_row_state_reason_coherence(
        &self,
        row: &LaneRow,
        violations: &mut Vec<HardenCriticalDependencyRegisterViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<HardenCriticalDependencyRegisterViolation>,
                               expected: LaneGapReason| {
            violations.push(
                HardenCriticalDependencyRegisterViolation::StateReasonIncoherent {
                    row_id: row.entry_id.clone(),
                    state: row.lane_state,
                    expected_reason: expected,
                },
            );
        };

        match row.lane_state {
            LaneState::NarrowedStale => {
                if !row.has_active_reason(LaneGapReason::PacketFreshnessBreached) {
                    push_incoherent(violations, LaneGapReason::PacketFreshnessBreached);
                }
            }
            LaneState::NarrowedMissing => {
                if !row.has_active_reason(LaneGapReason::PacketMissing) {
                    push_incoherent(violations, LaneGapReason::PacketMissing);
                }
            }
            LaneState::NarrowedUnbacked => {
                const ALLOWED: [LaneGapReason; 7] = [
                    LaneGapReason::EvidenceIncomplete,
                    LaneGapReason::ArtifactUnverified,
                    LaneGapReason::OwnerSignoffMissing,
                    LaneGapReason::DependencyAuditGap,
                    LaneGapReason::ForkDivergence,
                    LaneGapReason::ImportMappingFailed,
                    LaneGapReason::LicenseCoverageGap,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, LaneGapReason::EvidenceIncomplete);
                }
            }
            LaneState::NarrowedWaiverExpired => {
                if !row.has_active_reason(LaneGapReason::WaiverExpired) {
                    push_incoherent(violations, LaneGapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(
                        HardenCriticalDependencyRegisterViolation::WaiverStateWithoutWaiver {
                            row_id: row.entry_id.clone(),
                            state: row.lane_state,
                        },
                    );
                }
            }
            LaneState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        HardenCriticalDependencyRegisterViolation::WaiverStateWithoutWaiver {
                            row_id: row.entry_id.clone(),
                            state: row.lane_state,
                        },
                    );
                }
            }
            LaneState::Current => {}
        }
    }

    fn validate_publication(
        &self,
        violations: &mut Vec<HardenCriticalDependencyRegisterViolation>,
    ) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(HardenCriticalDependencyRegisterViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(HardenCriticalDependencyRegisterViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                HardenCriticalDependencyRegisterViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                HardenCriticalDependencyRegisterViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_row_ids != self.computed_blocking_row_ids() {
            violations.push(
                HardenCriticalDependencyRegisterViolation::PublicationBlockingSetMismatch {
                    field: "blocking_row_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from a lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The governance lane kind.
    pub lane_kind: LaneKind,
    /// Canonical lifecycle label the claim entry publishes.
    pub claim_label: String,
    /// Effective label after narrowing.
    pub effective_label: String,
    /// Whether the row holds its claim.
    pub holds_claim: bool,
    /// Lane state.
    pub lane_state: LaneState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<LaneGapReason>,
}

/// A redaction-safe export projection of the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenCriticalDependencyRegisterExportProjection {
    /// Artifact id this projection was produced from.
    pub artifact_id: String,
    /// Artifact as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PublicationDecision,
    /// Projected rows.
    pub rows: Vec<LaneExportRow>,
}

/// A validation violation for the dependency and licensing governance artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardenCriticalDependencyRegisterViolation {
    /// The artifact carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the artifact.
        actual: u32,
    },
    /// The artifact carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the artifact.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The artifact has no rows.
    EmptyRows,
    /// The artifact has no rules.
    NoRules,
    /// A required lane kind is missing from the rows.
    LaneKindMissing {
        /// Missing kind.
        kind: LaneKind,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A rule names no states to watch.
    RuleWithoutStates {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: LaneGapReason,
    },
    /// A narrowing state did not drop the effective label below the claim label.
    EffectiveLabelNotNarrowed {
        /// Row id.
        row_id: String,
        /// Lane state.
        state: LaneState,
        /// Claim label.
        claim_label: String,
        /// Effective label.
        effective_label: String,
    },
    /// A narrowing row state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        row_id: String,
        /// State.
        state: LaneState,
    },
    /// A held row carries an active gap reason.
    HeldRowWithActiveGap {
        /// Row id.
        row_id: String,
    },
    /// A held row has no owner sign-off.
    HeldRowWithoutSignoff {
        /// Row id.
        row_id: String,
    },
    /// A matrix row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        row_id: String,
        /// State.
        state: LaneState,
        /// Reason the state requires.
        expected_reason: LaneGapReason,
    },
    /// A waiver-bearing row state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        row_id: String,
        /// State.
        state: LaneState,
    },
    /// The declared publication decision disagrees with the computed one.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PublicationDecision,
        /// Computed decision.
        computed: PublicationDecision,
    },
    /// The declared publication blocking set disagrees with the computed one.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for HardenCriticalDependencyRegisterViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported artifact schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported artifact record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "artifact {field} is not the canonical value")
            }
            Self::EmptyRows => write!(f, "artifact has no lane rows"),
            Self::NoRules => write!(f, "artifact has no rules"),
            Self::LaneKindMissing { kind } => {
                write!(f, "missing row for lane kind {}", kind.as_str())
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate row id {row_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id {rule_id}")
            }
            Self::RuleWithoutStates { rule_id } => {
                write!(f, "rule {rule_id} watches no states")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::EffectiveLabelNotNarrowed {
                row_id,
                state,
                claim_label,
                effective_label,
            } => write!(
                f,
                "row {row_id} state {} must narrow below claim label {claim_label} but effective is {effective_label}",
                state.as_str()
            ),
            Self::NarrowingWithoutReason { row_id, state } => write!(
                f,
                "row {row_id} state {} narrows without naming an active gap reason",
                state.as_str()
            ),
            Self::HeldRowWithActiveGap { row_id } => write!(
                f,
                "row {row_id} holds claim while a gap reason is active"
            ),
            Self::HeldRowWithoutSignoff { row_id } => {
                write!(f, "row {row_id} holds claim without owner sign-off")
            }
            Self::StateReasonIncoherent {
                row_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {row_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { row_id, state } => write!(
                f,
                "row {row_id} state {} names no waiver",
                state.as_str()
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => write!(
                f,
                "publication {field} disagrees with the firing rules"
            ),
            Self::SummaryMismatch => write!(
                f,
                "artifact summary counts disagree with the rows"
            ),
        }
    }
}

impl Error for HardenCriticalDependencyRegisterViolation {}

/// Loads the embedded dependency and licensing governance artifact.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in artifact no longer matches
/// [`HardenCriticalDependencyRegister`].
pub fn current_harden_critical_dependency_register(
) -> Result<HardenCriticalDependencyRegister, serde_json::Error> {
    serde_json::from_str(HARDEN_CRITICAL_DEPENDENCY_REGISTER_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn artifact() -> HardenCriticalDependencyRegister {
        current_harden_critical_dependency_register().expect("artifact parses")
    }

    #[test]
    fn embedded_artifact_parses_and_validates() {
        let artifact = artifact();
        assert_eq!(
            artifact.schema_version,
            HARDEN_CRITICAL_DEPENDENCY_REGISTER_SCHEMA_VERSION
        );
        assert_eq!(
            artifact.record_kind,
            HARDEN_CRITICAL_DEPENDENCY_REGISTER_RECORD_KIND
        );
        assert_eq!(artifact.validate(), Vec::new());
        assert!(!artifact.rows.is_empty());
    }

    #[test]
    fn artifact_exercises_holding_and_narrowed_rows() {
        let artifact = artifact();
        assert!(
            !artifact.rows_holding_claim().is_empty(),
            "artifact must hold at least one claim"
        );
        assert!(
            !artifact.rows_narrowed().is_empty(),
            "artifact must narrow at least one claim"
        );
    }

    #[test]
    fn every_lane_kind_is_present() {
        let artifact = artifact();
        let present: BTreeSet<LaneKind> = artifact.rows.iter().map(|r| r.lane_kind).collect();
        for kind in LaneKind::ALL {
            assert!(
                present.contains(&kind),
                "missing lane kind {}",
                kind.as_str()
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let artifact = artifact();
        assert_eq!(artifact.summary, artifact.computed_summary());
        assert_eq!(
            artifact.summary.rows_holding_claim + artifact.summary.rows_narrowed,
            artifact.rows.len()
        );
    }

    #[test]
    fn publication_matches_computed() {
        let artifact = artifact();
        assert_eq!(
            artifact.publication.decision,
            artifact.computed_publication_decision()
        );
        assert_eq!(
            artifact.publication.blocking_rule_ids,
            artifact.computed_blocking_rule_ids()
        );
        assert_eq!(
            artifact.publication.blocking_row_ids,
            artifact.computed_blocking_row_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let artifact = artifact();
        let covered: BTreeSet<LaneGapReason> = artifact
            .rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in LaneGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_row_with_active_gap() {
        let mut artifact = artifact();
        let row = artifact
            .rows
            .iter_mut()
            .find(|r| r.lane_state == LaneState::Current)
            .expect("a current row exists");
        row.active_gap_reasons
            .push(LaneGapReason::PacketFreshnessBreached);
        let row_id = row.entry_id.clone();
        artifact.summary = artifact.computed_summary();
        assert!(artifact
            .validate()
            .contains(&HardenCriticalDependencyRegisterViolation::HeldRowWithActiveGap { row_id }));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut artifact = artifact();
        let row = artifact
            .rows
            .iter_mut()
            .find(|r| r.lane_state == LaneState::NarrowedStale)
            .expect("a narrowed-stale row exists");
        row.effective_label = row.claim_label.clone();
        artifact.summary = artifact.computed_summary();
        artifact.publication.decision = artifact.computed_publication_decision();
        artifact.publication.blocking_rule_ids = artifact.computed_blocking_rule_ids();
        artifact.publication.blocking_row_ids = artifact.computed_blocking_row_ids();
        assert!(artifact.validate().iter().any(|v| matches!(
            v,
            HardenCriticalDependencyRegisterViolation::EffectiveLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut artifact = artifact();
        artifact.publication.decision = PublicationDecision::Proceed;
        assert!(artifact.validate().iter().any(|v| matches!(
            v,
            HardenCriticalDependencyRegisterViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let artifact = artifact();
        let projection = artifact.support_export_projection();
        assert_eq!(projection.rows.len(), artifact.rows.len());
        assert_eq!(
            projection.publication_decision,
            artifact.publication.decision
        );
        for (row, projected) in artifact.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.holds_claim(), projected.holds_claim);
            assert_eq!(row.effective_label, projected.effective_label);
        }
    }
}
