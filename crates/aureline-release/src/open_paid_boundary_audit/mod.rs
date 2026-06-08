//! Typed open-versus-paid boundary, licensing, provenance, and contribution-policy
//! audit for the stable launch line.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each
//! launch-blocking *requirement* is proven; the
//! [`stable_version_windows`](crate::stable_version_windows) freezes each public
//! interface surface's version window; and the
//! [`stable_publication_pack`](crate::stable_publication_pack) governs each
//! outward-facing publication the release line ships about itself. None of them answer
//! the question this module answers: **for each governance fact the stable launch rests on
//! — where the open-source core ends and the paid/managed tier begins, what the licensing
//! posture is, what build provenance is attested, and what the contribution policy commits
//! to — is that fact actually attested by a fresh proof packet, with its required audit
//! controls satisfied and an owner sign-off, and is it narrowed below the cutline the
//! moment its backing thins out?** This module is the **open/paid boundary audit**. For
//! every audited governance subject it records one row that binds the subject to the
//! [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose lifecycle label it
//! backs, the attestation packet that grounds it, the required audit controls it must
//! satisfy, the waiver (if any) holding it provisionally, and the owner sign-off.
//!
//! Each [`AuditRow`] is one `(governance subject, public claim)` binding. It:
//!
//! - names the audit domain it governs ([`AuditRow::domain`], [`AuditRow::subject_ref`],
//!   [`AuditRow::subject_summary`]) and whether that subject is part of the
//!   release-blocking audit set ([`AuditRow::release_blocking`]);
//! - pins the attestation packet ([`ProofPacket`]) with its packet-freshness SLO and the
//!   [`AuditControl`] set whose every member must be satisfied for the row to attest;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose public
//!   claim it backs ([`AuditRow::claim_ref`]) and the canonical lifecycle label that entry
//!   publishes ([`AuditRow::claim_label`]). That label is a hard **ceiling**: a row may
//!   carry the claim's label or narrow below it, but it may never assert a public claim
//!   wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-audit labels, so
//!   docs, Help/About, the release center, and support exports ingest one label per row
//!   instead of cloning their own;
//! - records the audit state earned ([`AuditState`]), the active gap reasons
//!   ([`AuditGapReason`]), and the label it *effectively* publishes after narrowing
//!   ([`AuditRow::effective_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary between
//! a row whose backing supports a Stable public claim and one narrowed below it. A row that
//! is not attested — because its attestation packet aged out or is missing, because a
//! required audit control is unsatisfied, because its evidence is incomplete, because its
//! owner sign-off is missing, because its waiver expired, or because the public claim it
//! backs is itself below the cutline — is structurally required to drop below the cutline
//! rather than inherit an adjacent attested row. The [`AuditRule`] set names the closed
//! conditions that gate publication, and [`OpenPaidBoundaryAudit::publication`] records the
//! proceed/hold verdict.
//!
//! The audit is checked in at `artifacts/release/open_paid_boundary_audit.json` and
//! embedded here, so this typed consumer and the CI gate agree on every row without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state, a boolean control flag, or an
//! opaque ref. It carries no raw artifacts, raw logs, signatures, or credential material.
//! Two classes of check live outside this model because they need more than the audit sees:
//! date arithmetic (recomputing the packet-freshness state and waiver expiry against an
//! `as_of` date) and the cross-artifact ceiling check (whether each row's `claim_label`
//! still equals the label the stable claim manifest publishes for the entry named by
//! `claim_ref`). Those live in the CI gate. This model enforces the structural and logical
//! invariants that hold regardless of the clock and the neighbouring artifact — the
//! ceiling/no-widening rule, audit-control completeness, narrowing consistency,
//! packet/state coherence, owner sign-off on attested rows, domain and release-line
//! coverage, audit-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported open/paid-boundary-audit schema version.
pub const OPEN_PAID_BOUNDARY_AUDIT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the audit.
pub const OPEN_PAID_BOUNDARY_AUDIT_RECORD_KIND: &str = "open_paid_boundary_audit";

/// Repo-relative path to the checked-in audit.
pub const OPEN_PAID_BOUNDARY_AUDIT_PATH: &str = "artifacts/release/open_paid_boundary_audit.json";

/// Embedded checked-in audit JSON.
pub const OPEN_PAID_BOUNDARY_AUDIT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/open_paid_boundary_audit.json"
));

/// The audit domain a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditDomain {
    /// Where the open-source core ends and the paid/managed tier begins.
    OpenPaidBoundary,
    /// The licensing posture: SPDX coverage, third-party inventory, redistribution.
    Licensing,
    /// Build provenance, SBOM, and signing attestation.
    Provenance,
    /// The contribution policy: DCO/CLA, contribution terms, maintainer governance.
    ContributionPolicy,
}

impl AuditDomain {
    /// Every domain, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenPaidBoundary,
        Self::Licensing,
        Self::Provenance,
        Self::ContributionPolicy,
    ];

    /// Stable token recorded in the audit.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenPaidBoundary => "open_paid_boundary",
            Self::Licensing => "licensing",
            Self::Provenance => "provenance",
            Self::ContributionPolicy => "contribution_policy",
        }
    }
}

/// Audit state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditState {
    /// The row is attested: a captured, within-SLO attestation packet, every required
    /// audit control satisfied, and an owner sign-off back the public claim at its full
    /// canonical lifecycle label.
    Attested,
    /// The row carries the claim's full label only because an active, unexpired waiver
    /// covers a recorded residual gap.
    AttestedOnWaiver,
    /// A required audit control is unsatisfied, the row evidence is incomplete, or the
    /// owner sign-off is absent; the row is not attested and the label must narrow.
    NarrowedUnbacked,
    /// The public claim this row backs is itself below the cutline, so the row inherits
    /// that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The attestation packet breached its freshness SLO (or is missing); the row is not
    /// attested and the label must narrow.
    NarrowedStale,
    /// The row relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
}

impl AuditState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Attested,
        Self::AttestedOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the audit.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attested => "attested",
            Self::AttestedOnWaiver => "attested_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets the row carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Attested | Self::AttestedOnWaiver)
    }

    /// Whether the state forces the row below the public claim label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a row narrows or an audit rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditGapReason {
    /// The backing public claim narrowed below the cutline.
    ClaimLabelNarrowed,
    /// Required audit evidence is incomplete.
    AuditEvidenceIncomplete,
    /// The attestation packet breached its freshness SLO.
    AttestationPacketFreshnessBreached,
    /// No attestation packet has been captured.
    AttestationPacketMissing,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// The required row owner sign-off is missing.
    OwnerSignoffMissing,
    /// One or more required audit controls are unsatisfied.
    AuditControlUnsatisfied,
}

impl AuditGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ClaimLabelNarrowed,
        Self::AuditEvidenceIncomplete,
        Self::AttestationPacketFreshnessBreached,
        Self::AttestationPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::AuditControlUnsatisfied,
    ];

    /// Stable token recorded in the audit.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::AuditEvidenceIncomplete => "audit_evidence_incomplete",
            Self::AttestationPacketFreshnessBreached => "attestation_packet_freshness_breached",
            Self::AttestationPacketMissing => "attestation_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::AuditControlUnsatisfied => "audit_control_unsatisfied",
        }
    }
}

/// Default action an audit rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the row's published lifecycle label below the cutline.
    NarrowAuditLabel,
    /// Refresh the attestation packet so it re-enters its freshness SLO.
    RefreshAttestationPacket,
    /// Satisfy the unsatisfied required audit control.
    SatisfyAuditControl,
    /// Recapture the audit evidence the attestation packet depends on.
    RecaptureAuditEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl AuditAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowAuditLabel,
        Self::RefreshAttestationPacket,
        Self::SatisfyAuditControl,
        Self::RecaptureAuditEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the audit.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowAuditLabel => "narrow_audit_label",
            Self::RefreshAttestationPacket => "refresh_attestation_packet",
            Self::SatisfyAuditControl => "satisfy_audit_control",
            Self::RecaptureAuditEvidence => "recapture_audit_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One required audit control on a row: a concrete governance check that must be satisfied
/// for the row to attest (a published boundary matrix, a clean license inventory, a signed
/// provenance attestation, an enforced contributor sign-off, …).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditControl {
    /// Stable control id.
    pub control_id: String,
    /// Human-readable title.
    pub title: String,
    /// Ref to the artifact or policy the control checks.
    pub control_ref: String,
    /// Whether the control is satisfied.
    pub satisfied: bool,
}

/// One audit rule: a closed condition that narrows a row and may gate publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: AuditGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: AuditAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One audit row: a `(governance subject, public claim)` binding bound to its attestation
/// packet, required audit controls, canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditRow {
    /// Stable audit-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The audit domain this row governs.
    pub domain: AuditDomain,
    /// Ref to the governance subject this row audits.
    pub subject_ref: String,
    /// Reviewable one-line statement of the audited subject.
    pub subject_summary: String,
    /// Whether the subject is part of the release-blocking audit set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a row may
    /// never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Audit state earned for the row.
    pub audit_state: AuditState,
    /// The attestation packet and its freshness SLO.
    pub attestation_packet: ProofPacket,
    /// Waiver authorizing a provisional attestation, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Row owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Required audit controls the row must satisfy.
    #[serde(default)]
    pub audit_controls: Vec<AuditControl>,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<AuditGapReason>,
    /// The lifecycle label the row effectively carries after narrowing.
    pub effective_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl AuditRow {
    /// True when the effective label is at or above the cutline.
    pub fn holds_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.audit_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: AuditGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when every required audit control is satisfied.
    pub fn all_controls_satisfied(&self) -> bool {
        !self.audit_controls.is_empty()
            && self.audit_controls.iter().all(|control| control.satisfied)
    }

    /// Count of required audit controls that are unsatisfied.
    pub fn unsatisfied_control_count(&self) -> usize {
        self.audit_controls
            .iter()
            .filter(|control| !control.satisfied)
            .count()
    }
}

/// The recorded publication verdict for the open/paid boundary audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Audit-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Audit-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenPaidBoundaryAuditSummary {
    /// Total number of audit rows.
    pub total_rows: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows carrying a label at or above the cutline.
    pub rows_attested: usize,
    /// Rows narrowed below the cutline.
    pub rows_narrowed_below_cutline: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows carrying a label at or above the cutline.
    pub release_blocking_attested: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub rows_on_active_waiver: usize,
    /// Attestation packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Attestation packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Attestation packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Attestation packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total required audit controls across all rows.
    pub total_controls: usize,
    /// Required audit controls that are satisfied.
    pub controls_satisfied: usize,
    /// Required audit controls that are unsatisfied.
    pub controls_unsatisfied: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of audit rules currently firing.
    pub rules_firing: usize,
}

/// The typed open/paid boundary audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenPaidBoundaryAudit {
    /// Audit schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable audit identifier.
    pub audit_id: String,
    /// Lifecycle status of this audit artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this audit ingests as its public-claim source and
    /// ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the stable proof-index row proving this audit.
    pub stable_proof_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed audit-domain vocabulary.
    pub audit_domains: Vec<AuditDomain>,
    /// Closed audit-state vocabulary.
    pub audit_states: Vec<AuditState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<AuditGapReason>,
    /// Closed audit-action vocabulary.
    pub audit_actions: Vec<AuditAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking audit refs this audit must cover.
    pub release_blocking_audit_refs: Vec<String>,
    /// Audit rules.
    pub rules: Vec<AuditRule>,
    /// Audit rows.
    pub rows: Vec<AuditRow>,
    /// Recorded publication verdict.
    pub publication: AuditPublicationRecord,
    /// Summary counts.
    pub summary: OpenPaidBoundaryAuditSummary,
}

impl OpenPaidBoundaryAudit {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&AuditRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows carrying a label at or above the cutline.
    pub fn rows_attested(&self) -> Vec<&AuditRow> {
        self.rows.iter().filter(|row| row.holds_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&AuditRow> {
        self.rows.iter().filter(|row| !row.holds_stable()).collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&AuditRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one audit domain.
    pub fn rows_for_domain(&self, domain: AuditDomain) -> Vec<&AuditRow> {
        self.rows
            .iter()
            .filter(|row| row.domain == domain)
            .collect()
    }

    /// Distinct public claims (by claim ref) the audit covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn rule_fires(&self, rule: &AuditRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and audit rules.
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

    /// Audit-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim is
    /// already canonically narrowed is not a *publication* blocker, it merely inherits the
    /// upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<AuditGapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and audit rules.
    pub fn computed_summary(&self) -> OpenPaidBoundaryAuditSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.attestation_packet.slo_state == state)
                .count()
        };
        let release_blocking: Vec<&AuditRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        let total_controls: usize = self.rows.iter().map(|row| row.audit_controls.len()).sum();
        let controls_unsatisfied: usize = self
            .rows
            .iter()
            .map(AuditRow::unsatisfied_control_count)
            .sum();
        OpenPaidBoundaryAuditSummary {
            total_rows: self.rows.len(),
            total_claims: self.claims().len(),
            rows_attested: self.rows.iter().filter(|row| row.holds_stable()).count(),
            rows_narrowed_below_cutline: self.rows.iter().filter(|row| !row.holds_stable()).count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_attested: release_blocking
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
                .filter(|row| row.audit_state == AuditState::AttestedOnWaiver)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_controls,
            controls_satisfied: total_controls - controls_unsatisfied,
            controls_unsatisfied,
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

    /// Produces an export/Help-About-safe projection of the audit that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> AuditExportProjection {
        AuditExportProjection {
            audit_id: self.audit_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| AuditExportRow {
                    entry_id: row.entry_id.clone(),
                    domain: row.domain,
                    subject_ref: row.subject_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    holds_stable: row.holds_stable(),
                    audit_state: row.audit_state,
                    slo_state: row.attestation_packet.slo_state,
                    control_total: row.audit_controls.len(),
                    control_unsatisfied: row.unsatisfied_control_count(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the audit, returning every violation found.
    pub fn validate(&self) -> Vec<OpenPaidBoundaryAuditViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(OpenPaidBoundaryAuditViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(OpenPaidBoundaryAuditViolation::EmptyAudit);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(OpenPaidBoundaryAuditViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<OpenPaidBoundaryAuditViolation>) {
        if self.schema_version != OPEN_PAID_BOUNDARY_AUDIT_SCHEMA_VERSION {
            violations.push(OpenPaidBoundaryAuditViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != OPEN_PAID_BOUNDARY_AUDIT_RECORD_KIND {
            violations.push(OpenPaidBoundaryAuditViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("audit_id", &self.audit_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("stable_proof_index_ref", &self.stable_proof_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                    entry_id: "<audit>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(OpenPaidBoundaryAuditViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.audit_domains != AuditDomain::ALL.to_vec() {
            violations.push(OpenPaidBoundaryAuditViolation::ClosedVocabularyMismatch {
                field: "audit_domains",
            });
        }
        if self.audit_states != AuditState::ALL.to_vec() {
            violations.push(OpenPaidBoundaryAuditViolation::ClosedVocabularyMismatch {
                field: "audit_states",
            });
        }
        if self.gap_reasons != AuditGapReason::ALL.to_vec() {
            violations.push(OpenPaidBoundaryAuditViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.audit_actions != AuditAction::ALL.to_vec() {
            violations.push(OpenPaidBoundaryAuditViolation::ClosedVocabularyMismatch {
                field: "audit_actions",
            });
        }
        if self.release_blocking_audit_refs.is_empty() {
            violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                entry_id: "<audit>".to_owned(),
                field_name: "release_blocking_audit_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(OpenPaidBoundaryAuditViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(OpenPaidBoundaryAuditViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(OpenPaidBoundaryAuditViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<OpenPaidBoundaryAuditViolation>) {
        if self.rules.is_empty() {
            violations.push(OpenPaidBoundaryAuditViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(OpenPaidBoundaryAuditViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(OpenPaidBoundaryAuditViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every gap reason must have a rule, so a gap reason cannot fire without a
        // corresponding publication gate.
        for reason in AuditGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(OpenPaidBoundaryAuditViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &AuditRow, violations: &mut Vec<OpenPaidBoundaryAuditViolation>) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("subject_ref", &row.subject_ref),
            ("subject_summary", &row.subject_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            (
                "attestation_packet.packet_id",
                &row.attestation_packet.packet_id,
            ),
            (
                "attestation_packet.packet_ref",
                &row.attestation_packet.packet_ref,
            ),
            (
                "attestation_packet.proof_index_ref",
                &row.attestation_packet.proof_index_ref,
            ),
            (
                "attestation_packet.freshness_slo.slo_register_ref",
                &row.attestation_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no row may carry a label wider than the public claim's canonical
        // label.
        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(OpenPaidBoundaryAuditViolation::EffectiveWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                effective: row.effective_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn window
        // may not exceed it.
        if row.attestation_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "attestation_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.attestation_packet.freshness_slo.window_is_consistent() {
            violations.push(OpenPaidBoundaryAuditViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        self.validate_controls(row, violations);

        // A public claim whose canonical label is below the cutline forces the row to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(OpenPaidBoundaryAuditViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(AuditGapReason::ClaimLabelNarrowed) {
                violations.push(OpenPaidBoundaryAuditViolation::ClaimNarrowedWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        let slo_state = row.attestation_packet.slo_state;

        if row.holds_label() {
            // An attested row carries exactly the public claim's canonical label, carries
            // no active gap reason, rides a captured within-SLO packet, satisfies every
            // audit control, is owner-signed, and (for an on-waiver row) relies on an
            // unexpired waiver.
            if row.effective_label != row.claim_label {
                violations.push(OpenPaidBoundaryAuditViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    effective: row.effective_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(OpenPaidBoundaryAuditViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.attestation_packet.has_capture() {
                violations.push(OpenPaidBoundaryAuditViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(OpenPaidBoundaryAuditViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !row.all_controls_satisfied() {
                violations.push(OpenPaidBoundaryAuditViolation::HeldWithUnsatisfiedControl {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(OpenPaidBoundaryAuditViolation::HeldWithoutOwnerSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row
                .waiver
                .as_ref()
                .is_some_and(|waiver| waiver.expires_at.as_str() <= self.as_of.as_str())
            {
                violations.push(OpenPaidBoundaryAuditViolation::HeldOnExpiredWaiver {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the effective label below the cutline and name at
            // least one active reason.
            if row.holds_stable() {
                violations.push(OpenPaidBoundaryAuditViolation::EffectiveLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.audit_state,
                    effective: row.effective_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(OpenPaidBoundaryAuditViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.audit_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the matching
            // freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(AuditGapReason::AttestationPacketFreshnessBreached)
            {
                violations.push(
                    OpenPaidBoundaryAuditViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(AuditGapReason::AttestationPacketMissing)
            {
                violations.push(OpenPaidBoundaryAuditViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_controls(
        &self,
        row: &AuditRow,
        violations: &mut Vec<OpenPaidBoundaryAuditViolation>,
    ) {
        if row.audit_controls.is_empty() {
            violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "audit_controls",
            });
        }
        let mut seen = BTreeSet::new();
        for control in &row.audit_controls {
            if !seen.insert(control.control_id.clone()) {
                violations.push(OpenPaidBoundaryAuditViolation::DuplicateControlId {
                    entry_id: row.entry_id.clone(),
                    control_id: control.control_id.clone(),
                });
            }
            for (field, value) in [
                ("control_id", &control.control_id),
                ("title", &control.title),
                ("control_ref", &control.control_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                        entry_id: row.entry_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
        if row.unsatisfied_control_count() > 0
            && !row.has_active_reason(AuditGapReason::AuditControlUnsatisfied)
        {
            violations.push(
                OpenPaidBoundaryAuditViolation::UnsatisfiedControlWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &AuditRow,
        violations: &mut Vec<OpenPaidBoundaryAuditViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<OpenPaidBoundaryAuditViolation>,
                               expected: AuditGapReason| {
            violations.push(OpenPaidBoundaryAuditViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.audit_state,
                expected_reason: expected,
            });
        };

        match row.audit_state {
            AuditState::NarrowedUnbacked => {
                const ALLOWED: [AuditGapReason; 3] = [
                    AuditGapReason::AuditEvidenceIncomplete,
                    AuditGapReason::OwnerSignoffMissing,
                    AuditGapReason::AuditControlUnsatisfied,
                ];
                if !ALLOWED.iter().any(|reason| row.has_active_reason(*reason)) {
                    push_incoherent(violations, AuditGapReason::AuditEvidenceIncomplete);
                }
            }
            AuditState::NarrowedClaimNarrowed => {
                if !row.has_active_reason(AuditGapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, AuditGapReason::ClaimLabelNarrowed);
                }
            }
            AuditState::NarrowedStale => {
                if !(row.has_active_reason(AuditGapReason::AttestationPacketFreshnessBreached)
                    || row.has_active_reason(AuditGapReason::AttestationPacketMissing))
                {
                    push_incoherent(
                        violations,
                        AuditGapReason::AttestationPacketFreshnessBreached,
                    );
                }
            }
            AuditState::NarrowedWaiverExpired => {
                if !row.has_active_reason(AuditGapReason::WaiverExpired) {
                    push_incoherent(violations, AuditGapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(OpenPaidBoundaryAuditViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.audit_state,
                    });
                }
            }
            AuditState::AttestedOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|waiver| {
                        waiver.waiver_ref.trim().is_empty() || waiver.expires_at.trim().is_empty()
                    })
                    .unwrap_or(true)
                {
                    violations.push(OpenPaidBoundaryAuditViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.audit_state,
                    });
                }
            }
            AuditState::Attested => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<OpenPaidBoundaryAuditViolation>) {
        // Each subject ref appears at most once: a subject has one canonical row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.subject_ref.as_str()) {
                violations.push(OpenPaidBoundaryAuditViolation::DuplicateSubjectRef {
                    subject_ref: row.subject_ref.clone(),
                });
            }
        }

        // The release line must cover every declared release-blocking subject with exactly
        // one release-blocking row, and every release-blocking row must be declared, so a
        // subject cannot quietly drop out of the audit.
        let declared: BTreeSet<&str> = self
            .release_blocking_audit_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.entry_id.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(
                    OpenPaidBoundaryAuditViolation::ReleaseBlockingRefWithoutRow {
                        entry_id: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.entry_id.as_str()) {
                violations.push(OpenPaidBoundaryAuditViolation::ReleaseBlockingRowNotInSet {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        // The audit must cover all four domains — open/paid boundary, licensing,
        // provenance, and contribution policy — so the release line cannot audit some
        // domains and silently leave a whole domain ungoverned.
        for domain in AuditDomain::ALL {
            if self.rows_for_domain(domain).is_empty() {
                violations.push(OpenPaidBoundaryAuditViolation::DomainAbsent { domain });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<OpenPaidBoundaryAuditViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(OpenPaidBoundaryAuditViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                OpenPaidBoundaryAuditViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                OpenPaidBoundaryAuditViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                OpenPaidBoundaryAuditViolation::PublicationBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditExportRow {
    /// Stable audit-row id.
    pub entry_id: String,
    /// Audit domain.
    pub domain: AuditDomain,
    /// Audited subject ref.
    pub subject_ref: String,
    /// Whether the subject is part of the release-blocking set.
    pub release_blocking: bool,
    /// The public-claim entry ref the row backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the row carries.
    pub effective_label: StableClaimLevel,
    /// Whether the row carries a label at or above the cutline.
    pub holds_stable: bool,
    /// Audit state.
    pub audit_state: AuditState,
    /// Attestation-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Total required audit controls.
    pub control_total: usize,
    /// Required audit controls that are unsatisfied.
    pub control_unsatisfied: usize,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<AuditGapReason>,
}

/// A redaction-safe export projection of the audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditExportProjection {
    /// Audit id this projection was produced from.
    pub audit_id: String,
    /// Audit as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<AuditExportRow>,
}

/// A validation violation for the open/paid boundary audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenPaidBoundaryAuditViolation {
    /// The audit carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the audit.
        actual: u32,
    },
    /// The audit carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the audit.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The audit has no rows.
    EmptyAudit,
    /// The audit has no rules.
    NoRules,
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An entry id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A control id appears more than once in a row.
    DuplicateControlId {
        /// Row id.
        entry_id: String,
        /// Duplicate control id.
        control_id: String,
    },
    /// A subject ref appears on more than one row.
    DuplicateSubjectRef {
        /// Duplicate subject ref.
        subject_ref: String,
    },
    /// An audit rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: AuditGapReason,
    },
    /// A row's effective label is wider than its claim ceiling.
    EffectiveWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
    /// An attested row is backed by a public claim already below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
    },
    /// A row whose claim narrowed does not name the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// An attested row carries a label different from its claim ceiling.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// An attested row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// An attested row has no captured, evidence-backed packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// An attested row rides a stale or missing packet.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// SLO state.
        slo_state: FreshnessSloState,
    },
    /// An attested row carries an unsatisfied audit control.
    HeldWithUnsatisfiedControl {
        /// Row id.
        entry_id: String,
    },
    /// An attested row lacks owner sign-off.
    HeldWithoutOwnerSignoff {
        /// Row id.
        entry_id: String,
    },
    /// An attested row relies on an expired waiver.
    HeldOnExpiredWaiver {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing state did not narrow below the cutline.
    EffectiveLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: AuditState,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A narrowing state carries no reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: AuditState,
    },
    /// A breached packet does not name the freshness breach reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A missing packet does not name the missing packet reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// An unsatisfied audit control does not name the unsatisfied-control reason.
    UnsatisfiedControlWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: AuditState,
        /// Expected reason.
        expected_reason: AuditGapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: AuditState,
    },
    /// A declared release-blocking row has no row.
    ReleaseBlockingRefWithoutRow {
        /// Missing row id.
        entry_id: String,
    },
    /// A release-blocking row was not declared.
    ReleaseBlockingRowNotInSet {
        /// Row id.
        entry_id: String,
    },
    /// A domain is absent from the audit.
    DomainAbsent {
        /// Missing domain.
        domain: AuditDomain,
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

impl fmt::Display for OpenPaidBoundaryAuditViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported audit schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported audit record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "audit {field} is not the canonical value")
            }
            Self::EmptyAudit => write!(f, "audit has no rows"),
            Self::NoRules => write!(f, "audit has no rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate audit row id {entry_id}"),
            Self::DuplicateRuleId { rule_id } => write!(f, "duplicate rule id {rule_id}"),
            Self::DuplicateControlId {
                entry_id,
                control_id,
            } => write!(f, "audit row {entry_id} repeats control {control_id}"),
            Self::DuplicateSubjectRef { subject_ref } => {
                write!(f, "subject ref {subject_ref} appears on more than one row")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule {rule_id} watches no lifecycle labels")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::EffectiveWiderThanClaim {
                entry_id,
                claim,
                effective,
            } => write!(
                f,
                "audit row {entry_id} effective label {} is wider than claim ceiling {}",
                effective.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { entry_id } => write!(
                f,
                "audit row {entry_id} freshness SLO warn window exceeds target age"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "audit row {entry_id} attests while claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { entry_id } => write!(
                f,
                "audit row {entry_id} backs a narrowed claim without naming claim_label_narrowed"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                effective,
            } => write!(
                f,
                "audit row {entry_id} attests {} but claim ceiling is {}",
                effective.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(
                    f,
                    "audit row {entry_id} attests while a gap reason is active"
                )
            }
            Self::HeldWithoutFreshPacket { entry_id } => write!(
                f,
                "audit row {entry_id} attests with no captured, evidence-backed packet"
            ),
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "audit row {entry_id} attests while packet is {}",
                slo_state.as_str()
            ),
            Self::HeldWithUnsatisfiedControl { entry_id } => write!(
                f,
                "audit row {entry_id} attests with an unsatisfied required control"
            ),
            Self::HeldWithoutOwnerSignoff { entry_id } => {
                write!(f, "audit row {entry_id} attests without owner sign-off")
            }
            Self::HeldOnExpiredWaiver { entry_id } => {
                write!(f, "audit row {entry_id} attests on an expired waiver")
            }
            Self::EffectiveLabelNotNarrowed {
                entry_id,
                state,
                effective,
            } => write!(
                f,
                "audit row {entry_id} state {} must narrow but holds {}",
                state.as_str(),
                effective.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "audit row {entry_id} state {} narrows without a reason",
                state.as_str()
            ),
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "audit row {entry_id} has a breached packet without the freshness reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "audit row {entry_id} has a missing packet without the missing-packet reason"
            ),
            Self::UnsatisfiedControlWithoutReason { entry_id } => write!(
                f,
                "audit row {entry_id} has an unsatisfied control without audit_control_unsatisfied"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "audit row {entry_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => write!(
                f,
                "audit row {entry_id} state {} names no waiver",
                state.as_str()
            ),
            Self::ReleaseBlockingRefWithoutRow { entry_id } => {
                write!(f, "declared release-blocking audit {entry_id} has no row")
            }
            Self::ReleaseBlockingRowNotInSet { entry_id } => {
                write!(f, "release-blocking audit {entry_id} is not declared")
            }
            Self::DomainAbsent { domain } => {
                write!(f, "audit domain {} is covered by no row", domain.as_str())
            }
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing rules")
            }
            Self::SummaryMismatch => write!(f, "audit summary counts disagree with rows"),
        }
    }
}

impl Error for OpenPaidBoundaryAuditViolation {}

/// Loads the embedded open/paid boundary audit.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in audit no longer matches
/// [`OpenPaidBoundaryAudit`].
pub fn current_open_paid_boundary_audit() -> Result<OpenPaidBoundaryAudit, serde_json::Error> {
    serde_json::from_str(OPEN_PAID_BOUNDARY_AUDIT_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn audit() -> OpenPaidBoundaryAudit {
        current_open_paid_boundary_audit().expect("open/paid boundary audit parse")
    }

    #[test]
    fn embedded_audit_parses_and_validates() {
        let audit = audit();
        assert_eq!(
            audit.schema_version,
            OPEN_PAID_BOUNDARY_AUDIT_SCHEMA_VERSION
        );
        assert_eq!(audit.record_kind, OPEN_PAID_BOUNDARY_AUDIT_RECORD_KIND);
        assert_eq!(audit.validate(), Vec::new());
    }

    #[test]
    fn every_domain_is_covered() {
        let audit = audit();
        for domain in AuditDomain::ALL {
            assert!(
                !audit.rows_for_domain(domain).is_empty(),
                "{} must have at least one row",
                domain.as_str()
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let audit = audit();
        assert_eq!(audit.summary, audit.computed_summary());
        assert_eq!(
            audit.summary.rows_attested + audit.summary.rows_narrowed_below_cutline,
            audit.rows.len()
        );
    }

    #[test]
    fn publication_proceeds_without_blocking_rules() {
        let audit = audit();
        assert_eq!(audit.publication.decision, PromotionDecision::Proceed);
        assert_eq!(
            audit.publication.decision,
            audit.computed_publication_decision()
        );
        assert!(audit.publication.blocking_rule_ids.is_empty());
        assert!(audit.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn attested_row_on_breached_packet_fails() {
        let mut audit = audit();
        let row = audit
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("an attested row exists");
        row.attestation_packet.slo_state = FreshnessSloState::Breached;
        audit.summary = audit.computed_summary();
        assert!(audit.validate().iter().any(|violation| matches!(
            violation,
            OpenPaidBoundaryAuditViolation::HeldOnStalePacket { .. }
        )));
    }

    #[test]
    fn attested_row_with_unsatisfied_control_fails() {
        let mut audit = audit();
        let row = audit
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("an attested row exists");
        row.audit_controls[0].satisfied = false;
        audit.summary = audit.computed_summary();
        assert!(audit.validate().iter().any(|violation| matches!(
            violation,
            OpenPaidBoundaryAuditViolation::HeldWithUnsatisfiedControl { .. }
        )));
    }

    #[test]
    fn audit_rows_attest_without_narrowing() {
        let audit = audit();
        assert!(
            audit.rows_narrowed().is_empty(),
            "clean audit must not narrow a row"
        );
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let audit = audit();
        let projection = audit.support_export_projection();
        assert_eq!(projection.rows.len(), audit.rows.len());
        assert_eq!(projection.publication_decision, audit.publication.decision);
        for (row, projected) in audit.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.holds_stable(), projected.holds_stable);
            assert_eq!(
                row.unsatisfied_control_count(),
                projected.control_unsatisfied
            );
        }
    }
}
