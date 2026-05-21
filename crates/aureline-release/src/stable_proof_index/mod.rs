//! Typed stable proof index linking launch-blocking requirements, proof packets,
//! waivers, and public claims.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes, and the
//! [`stable_boundary_manifest`](crate::stable_boundary_manifest) decides what each
//! deployment value line can carry. Neither answers the question the launch
//! shiproom actually asks: **is each launch-blocking requirement proven, and which
//! public claim does that proof back?** This module is the **proof index** that
//! closes that loop. For every launch-blocking requirement it records one row that
//! binds the requirement to the proof packet that proves it, the waiver (if any)
//! that holds it provisionally, and the public claim (a stable-claim-manifest
//! entry) whose lifecycle label the proof backs.
//!
//! Each [`ProofRow`] is one `(requirement, public claim)` binding. It:
//!
//! - names the launch-blocking requirement it proves ([`ProofRow::requirement_ref`],
//!   [`ProofRow::requirement_class`], [`ProofRow::requirement_summary`]) and whether
//!   that requirement is launch-blocking ([`ProofRow::launch_blocking`]);
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose
//!   public claim it backs ([`ProofRow::claim_ref`]) and the canonical lifecycle
//!   label that entry publishes ([`ProofRow::claim_label`]). That label is a hard
//!   **ceiling**: a proof row may back the claim at its label or narrow below it,
//!   but it may never assert a proof wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-requirement
//!   labels, so docs, Help/About, the release center, and support exports ingest
//!   one label per row instead of cloning their own;
//! - carries a proof packet with a packet-freshness SLO ([`ProofPacket`]), the
//!   proof state earned ([`ProofState`]), the active gap reasons ([`GapReason`]),
//!   and the label it *effectively* backs after narrowing ([`ProofRow::proven_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a requirement whose proof backs a Stable public claim and one narrowed
//! below it. A requirement that is not proven — because its proof packet aged out
//! or is missing, because its waiver expired, because its requirement evidence is
//! incomplete, or because the public claim it backs is itself below the cutline —
//! is structurally required to drop below the cutline rather than inherit an
//! adjacent green requirement. The [`ProofRule`] set names the closed conditions
//! that gate publication, and [`StableProofIndex::publication`] records the
//! resulting proceed/hold verdict.
//!
//! The index is checked in at `artifacts/release/stable_proof_index.json` and
//! embedded here, so this typed consumer and the CI gate agree on every row without
//! a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Two
//! classes of check live outside this model because they need more than the index
//! sees: date arithmetic (recomputing the packet-freshness state and waiver expiry
//! against an `as_of` date) and the cross-artifact ceiling check (whether each
//! row's `claim_label` still equals the label the stable claim manifest publishes
//! for the entry named by `claim_ref`). Both live in the CI gate. This model
//! enforces the structural and logical invariants that hold regardless of the clock
//! and the neighbouring artifact — the ceiling/no-widening rule, narrowing
//! consistency, packet/state coherence, owner sign-off on proven rows,
//! launch-blocking requirement coverage, publication-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported index schema version.
pub const STABLE_PROOF_INDEX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the index.
pub const STABLE_PROOF_INDEX_RECORD_KIND: &str = "stable_proof_index";

/// Repo-relative path to the checked-in index.
pub const STABLE_PROOF_INDEX_PATH: &str = "artifacts/release/stable_proof_index.json";

/// Embedded checked-in index JSON.
pub const STABLE_PROOF_INDEX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stable_proof_index.json"
));

/// Proof state a requirement earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofState {
    /// The requirement is proven: a captured, within-SLO proof packet backs the
    /// public claim at its full canonical lifecycle label, owner-signed.
    Proven,
    /// The requirement backs the claim at its full label only because an active,
    /// unexpired waiver covers a recorded proof gap.
    ProvenOnWaiver,
    /// The proof packet or requirement evidence is missing or incomplete, or owner
    /// sign-off is absent; the requirement is not proven and the label must narrow.
    UnprovenUnbacked,
    /// The public claim this requirement backs is itself below the cutline, so the
    /// proof inherits that ceiling and narrows.
    UnprovenClaimNarrowed,
    /// The proof packet breached its freshness SLO (or is missing); the requirement
    /// is not proven and the label must narrow.
    UnprovenStale,
    /// The requirement relied on a waiver that has expired; the label must narrow.
    UnprovenWaiverExpired,
}

impl ProofState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Proven,
        Self::ProvenOnWaiver,
        Self::UnprovenUnbacked,
        Self::UnprovenClaimNarrowed,
        Self::UnprovenStale,
        Self::UnprovenWaiverExpired,
    ];

    /// Stable token recorded in the index.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::ProvenOnWaiver => "proven_on_waiver",
            Self::UnprovenUnbacked => "unproven_unbacked",
            Self::UnprovenClaimNarrowed => "unproven_claim_narrowed",
            Self::UnprovenStale => "unproven_stale",
            Self::UnprovenWaiverExpired => "unproven_waiver_expired",
        }
    }

    /// Whether the state lets a requirement back the public claim at its label.
    pub const fn holds_proof(self) -> bool {
        matches!(self, Self::Proven | Self::ProvenOnWaiver)
    }

    /// Whether the state forces the requirement below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_proof()
    }
}

/// Closed reason a proof narrows or a proof rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this requirement backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The requirement names a capability the build does not yet implement.
    RequirementCapabilityAbsent,
    /// The proof packet's requirement-level evidence is missing or narrowed.
    RequirementEvidenceIncomplete,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// No proof packet has been captured for the requirement.
    ProofPacketMissing,
    /// A waiver the proof relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ClaimLabelNarrowed,
        Self::RequirementCapabilityAbsent,
        Self::RequirementEvidenceIncomplete,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the index.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::RequirementCapabilityAbsent => "requirement_capability_absent",
            Self::RequirementEvidenceIncomplete => "requirement_evidence_incomplete",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a proof rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the requirement's backed lifecycle label below the cutline.
    NarrowClaimLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Recapture the requirement-level evidence the proof packet depends on.
    RecaptureRequirementEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl IndexAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPublication,
        Self::NarrowClaimLabel,
        Self::RefreshProofPacket,
        Self::RecaptureRequirementEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the index.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowClaimLabel => "narrow_claim_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RecaptureRequirementEvidence => "recapture_requirement_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One proof rule: a closed condition that narrows a backed label and may gate
/// publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: IndexAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One stable proof index row: a `(requirement, public claim)` binding bound to its
/// proof packet, the canonical ceiling label, and its packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofRow {
    /// Stable proof-row id.
    pub proof_id: String,
    /// Human-readable title.
    pub title: String,
    /// The launch-blocking requirement id this row proves.
    pub requirement_ref: String,
    /// The requirement family this requirement belongs to.
    pub requirement_class: String,
    /// Reviewable one-line statement of the requirement.
    pub requirement_summary: String,
    /// Whether the requirement is launch-blocking.
    pub launch_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this proof backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a
    /// proof may never back a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Proof state earned for the requirement.
    pub index_state: ProofState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional proof, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the proof effectively backs after narrowing.
    pub proven_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl ProofRow {
    /// True when the proven label is at or above the cutline.
    pub fn proves_stable(&self) -> bool {
        self.proven_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the requirement back its claimed label.
    pub fn holds_proof(&self) -> bool {
        self.index_state.holds_proof()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// The recorded publication verdict for the stable proof index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Proof-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Proof-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_proof_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableProofIndexSummary {
    /// Total number of proof rows.
    pub total_requirements: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows backing a label at or above the cutline.
    pub requirements_proven_stable: usize,
    /// Rows narrowed below the cutline.
    pub requirements_narrowed_below_cutline: usize,
    /// Rows holding a proof via an active waiver.
    pub requirements_on_active_waiver: usize,
    /// Total launch-blocking rows.
    pub launch_blocking_total: usize,
    /// Launch-blocking rows backing a label at or above the cutline.
    pub launch_blocking_proven_stable: usize,
    /// Launch-blocking rows narrowed below the cutline.
    pub launch_blocking_unproven: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of proof rules currently firing.
    pub proof_rules_firing: usize,
}

/// The typed stable proof index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableProofIndex {
    /// Index schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable index identifier.
    pub index_id: String,
    /// Lifecycle status of this index artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this index ingests as its public-claim
    /// source and ceiling.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed proof-state vocabulary.
    pub proof_states: Vec<ProofState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed index-action vocabulary.
    pub index_actions: Vec<IndexAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of launch-blocking requirement refs this index must cover.
    pub launch_blocking_requirement_refs: Vec<String>,
    /// Proof rules.
    pub proof_rules: Vec<ProofRule>,
    /// Proof rows.
    pub rows: Vec<ProofRow>,
    /// Recorded publication verdict.
    pub publication: ProofPublicationRecord,
    /// Summary counts.
    pub summary: StableProofIndexSummary,
}

impl StableProofIndex {
    /// Returns the row registered for `proof_id`.
    pub fn row(&self, proof_id: &str) -> Option<&ProofRow> {
        self.rows.iter().find(|row| row.proof_id == proof_id)
    }

    /// Returns the rows backing a label at or above the cutline.
    pub fn rows_proven_stable(&self) -> Vec<&ProofRow> {
        self.rows.iter().filter(|row| row.proves_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&ProofRow> {
        self.rows
            .iter()
            .filter(|row| !row.proves_stable())
            .collect()
    }

    /// Returns the launch-blocking rows.
    pub fn launch_blocking_rows(&self) -> Vec<&ProofRow> {
        self.rows.iter().filter(|row| row.launch_blocking).collect()
    }

    /// Distinct public claims (by claim ref) the index covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn proof_rule_fires(&self, rule: &ProofRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and proof rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .proof_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.proof_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .proof_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.proof_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Proof-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose
    /// claim is already canonically narrowed is not a *proof-index* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_proof_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .proof_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.proof_rule_fires(rule))
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
                ids.insert(row.proof_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and proof rules.
    pub fn computed_summary(&self) -> StableProofIndexSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let launch_blocking: Vec<&ProofRow> =
            self.rows.iter().filter(|row| row.launch_blocking).collect();
        StableProofIndexSummary {
            total_requirements: self.rows.len(),
            total_claims: self.claims().len(),
            requirements_proven_stable: self.rows.iter().filter(|row| row.proves_stable()).count(),
            requirements_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.proves_stable())
                .count(),
            requirements_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.index_state == ProofState::ProvenOnWaiver)
                .count(),
            launch_blocking_total: launch_blocking.len(),
            launch_blocking_proven_stable: launch_blocking
                .iter()
                .filter(|row| row.proves_stable())
                .count(),
            launch_blocking_unproven: launch_blocking
                .iter()
                .filter(|row| !row.proves_stable())
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            proof_rules_firing: self
                .proof_rules
                .iter()
                .filter(|rule| self.proof_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the index that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> ProofIndexExportProjection {
        ProofIndexExportProjection {
            index_id: self.index_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| ProofIndexExportRow {
                    proof_id: row.proof_id.clone(),
                    requirement_ref: row.requirement_ref.clone(),
                    requirement_class: row.requirement_class.clone(),
                    launch_blocking: row.launch_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    proven_label: row.proven_label,
                    proves_stable: row.proves_stable(),
                    index_state: row.index_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the index, returning every violation found.
    pub fn validate(&self) -> Vec<StableProofIndexViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.proof_id.clone()) {
                violations.push(StableProofIndexViolation::DuplicateProofId {
                    proof_id: row.proof_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(StableProofIndexViolation::EmptyIndex);
        }

        self.validate_requirement_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(StableProofIndexViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<StableProofIndexViolation>) {
        if self.schema_version != STABLE_PROOF_INDEX_SCHEMA_VERSION {
            violations.push(StableProofIndexViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != STABLE_PROOF_INDEX_RECORD_KIND {
            violations.push(StableProofIndexViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("index_id", &self.index_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableProofIndexViolation::EmptyField {
                    proof_id: "<index>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(StableProofIndexViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.proof_states != ProofState::ALL.to_vec() {
            violations.push(StableProofIndexViolation::ClosedVocabularyMismatch {
                field: "proof_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(StableProofIndexViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.index_actions != IndexAction::ALL.to_vec() {
            violations.push(StableProofIndexViolation::ClosedVocabularyMismatch {
                field: "index_actions",
            });
        }
        if self.launch_blocking_requirement_refs.is_empty() {
            violations.push(StableProofIndexViolation::EmptyField {
                proof_id: "<index>".to_owned(),
                field_name: "launch_blocking_requirement_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(StableProofIndexViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(StableProofIndexViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(StableProofIndexViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(StableProofIndexViolation::EmptyField {
                proof_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<StableProofIndexViolation>) {
        if self.proof_rules.is_empty() {
            violations.push(StableProofIndexViolation::NoProofRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.proof_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(StableProofIndexViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(StableProofIndexViolation::EmptyField {
                        proof_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(StableProofIndexViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every gap reason must have a rule, so a gap reason cannot fire without a
        // corresponding publication gate.
        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(StableProofIndexViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &ProofRow, violations: &mut Vec<StableProofIndexViolation>) {
        for (field, value) in [
            ("proof_id", &row.proof_id),
            ("title", &row.title),
            ("requirement_ref", &row.requirement_ref),
            ("requirement_class", &row.requirement_class),
            ("requirement_summary", &row.requirement_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableProofIndexViolation::EmptyField {
                    proof_id: row.proof_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no proof may back a label wider than the public claim's
        // canonical label.
        if row.proven_label.rank() > row.claim_label.rank() {
            violations.push(StableProofIndexViolation::ProvenWiderThanClaim {
                proof_id: row.proof_id.clone(),
                claim: row.claim_label,
                proven: row.proven_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(StableProofIndexViolation::EmptyField {
                proof_id: row.proof_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(StableProofIndexViolation::FreshnessSloInconsistent {
                proof_id: row.proof_id.clone(),
            });
        }

        // A public claim whose canonical label is below the cutline forces the
        // proof to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_proof() {
                violations.push(StableProofIndexViolation::HeldOnNarrowedClaim {
                    proof_id: row.proof_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                violations.push(StableProofIndexViolation::ClaimNarrowedWithoutReason {
                    proof_id: row.proof_id.clone(),
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_proof() {
            // A proven row backs exactly the public claim's canonical label,
            // carries no active gap reason, rides a captured within-SLO packet, and
            // is owner-signed.
            if row.proven_label != row.claim_label {
                violations.push(StableProofIndexViolation::HeldLabelNotEqualClaim {
                    proof_id: row.proof_id.clone(),
                    claim: row.claim_label,
                    proven: row.proven_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(StableProofIndexViolation::HeldWithActiveGap {
                    proof_id: row.proof_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(StableProofIndexViolation::HeldWithoutFreshPacket {
                    proof_id: row.proof_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(StableProofIndexViolation::HeldOnStalePacket {
                    proof_id: row.proof_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(StableProofIndexViolation::HeldWithoutSignoff {
                    proof_id: row.proof_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the backed label below the cutline and
            // name at least one active reason.
            if row.proves_stable() {
                violations.push(StableProofIndexViolation::ProvenLabelNotNarrowed {
                    proof_id: row.proof_id.clone(),
                    state: row.index_state,
                    proven: row.proven_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(StableProofIndexViolation::NarrowingWithoutReason {
                    proof_id: row.proof_id.clone(),
                    state: row.index_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
            {
                violations.push(StableProofIndexViolation::BreachedPacketWithoutReason {
                    proof_id: row.proof_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::ProofPacketMissing)
            {
                violations.push(StableProofIndexViolation::MissingPacketWithoutReason {
                    proof_id: row.proof_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &ProofRow,
        violations: &mut Vec<StableProofIndexViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<StableProofIndexViolation>,
                               expected: GapReason| {
            violations.push(StableProofIndexViolation::StateReasonIncoherent {
                proof_id: row.proof_id.clone(),
                state: row.index_state,
                expected_reason: expected,
            });
        };

        match row.index_state {
            ProofState::UnprovenUnbacked => {
                const ALLOWED: [GapReason; 3] = [
                    GapReason::RequirementCapabilityAbsent,
                    GapReason::RequirementEvidenceIncomplete,
                    GapReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, GapReason::RequirementEvidenceIncomplete);
                }
            }
            ProofState::UnprovenClaimNarrowed => {
                if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, GapReason::ClaimLabelNarrowed);
                }
            }
            ProofState::UnprovenStale => {
                if !(row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
                    || row.has_active_reason(GapReason::ProofPacketMissing))
                {
                    push_incoherent(violations, GapReason::ProofPacketFreshnessBreached);
                }
            }
            ProofState::UnprovenWaiverExpired => {
                if !row.has_active_reason(GapReason::WaiverExpired) {
                    push_incoherent(violations, GapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(StableProofIndexViolation::WaiverStateWithoutWaiver {
                        proof_id: row.proof_id.clone(),
                        state: row.index_state,
                    });
                }
            }
            ProofState::ProvenOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(StableProofIndexViolation::WaiverStateWithoutWaiver {
                        proof_id: row.proof_id.clone(),
                        state: row.index_state,
                    });
                }
            }
            ProofState::Proven => {}
        }
    }

    fn validate_requirement_coverage(&self, violations: &mut Vec<StableProofIndexViolation>) {
        // Each requirement ref appears at most once: a requirement has one canonical
        // proof row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.requirement_ref.as_str()) {
                violations.push(StableProofIndexViolation::DuplicateRequirementRef {
                    requirement_ref: row.requirement_ref.clone(),
                });
            }
        }

        // Every declared launch-blocking requirement ref must be covered by exactly
        // one launch-blocking row, so a launch-blocking requirement cannot quietly
        // drop out of the index.
        let declared: BTreeSet<&str> = self
            .launch_blocking_requirement_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.launch_blocking)
            .map(|row| row.requirement_ref.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(StableProofIndexViolation::LaunchBlockingRefWithoutRow {
                    requirement_ref: (*declared_ref).to_owned(),
                });
            }
        }
        for row in &self.rows {
            if row.launch_blocking && !declared.contains(row.requirement_ref.as_str()) {
                violations.push(StableProofIndexViolation::LaunchBlockingRowNotInSet {
                    proof_id: row.proof_id.clone(),
                    requirement_ref: row.requirement_ref.clone(),
                });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<StableProofIndexViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(StableProofIndexViolation::EmptyField {
                proof_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(StableProofIndexViolation::EmptyField {
                proof_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(StableProofIndexViolation::PublicationDecisionInconsistent {
                declared: self.publication.decision,
                computed,
            });
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(StableProofIndexViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_proof_ids != self.computed_blocking_proof_ids() {
            violations.push(StableProofIndexViolation::PublicationBlockingSetMismatch {
                field: "blocking_proof_ids",
            });
        }
    }
}

/// A redaction-safe export row projected from the index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofIndexExportRow {
    /// Stable proof-row id.
    pub proof_id: String,
    /// Launch-blocking requirement ref.
    pub requirement_ref: String,
    /// Requirement family.
    pub requirement_class: String,
    /// Whether the requirement is launch-blocking.
    pub launch_blocking: bool,
    /// The public-claim entry ref the proof backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the proof backs.
    pub proven_label: StableClaimLevel,
    /// Whether the row backs a label at or above the cutline.
    pub proves_stable: bool,
    /// Proof state.
    pub index_state: ProofState,
    /// Proof-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// A redaction-safe export projection of the index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofIndexExportProjection {
    /// Index id this projection was produced from.
    pub index_id: String,
    /// Index as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<ProofIndexExportRow>,
}

/// A validation violation for the stable proof index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableProofIndexViolation {
    /// The index carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the index.
        actual: u32,
    },
    /// The index carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the index.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The index has no rows.
    EmptyIndex,
    /// The index has no proof rules.
    NoProofRules,
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        proof_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A proof id appears more than once.
    DuplicateProofId {
        /// Duplicate proof id.
        proof_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A proof rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: GapReason,
    },
    /// A proven label is wider than the public claim's canonical label.
    ProvenWiderThanClaim {
        /// Row id.
        proof_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Proven label.
        proven: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Row id.
        proof_id: String,
    },
    /// A row holds a proof while the public claim's canonical label is narrowed.
    HeldOnNarrowedClaim {
        /// Row id.
        proof_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
    },
    /// A row whose claim is narrowed does not carry the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Row id.
        proof_id: String,
    },
    /// A narrowing state did not drop the proven label below the cutline.
    ProvenLabelNotNarrowed {
        /// Row id.
        proof_id: String,
        /// Proof state.
        state: ProofState,
        /// Proven label.
        proven: StableClaimLevel,
    },
    /// A narrowing state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        proof_id: String,
        /// Proof state.
        state: ProofState,
    },
    /// A proven row's proven label is not equal to its claim ceiling label.
    HeldLabelNotEqualClaim {
        /// Row id.
        proof_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Proven label.
        proven: StableClaimLevel,
    },
    /// A proven row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        proof_id: String,
    },
    /// A proven row rides a proof packet with no capture or evidence.
    HeldWithoutFreshPacket {
        /// Row id.
        proof_id: String,
    },
    /// A proven row rides a proof packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        proof_id: String,
        /// The packet's freshness-SLO state.
        slo_state: FreshnessSloState,
    },
    /// A proven row has no owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        proof_id: String,
    },
    /// A narrowing row with a breached packet does not name the breach reason.
    BreachedPacketWithoutReason {
        /// Row id.
        proof_id: String,
    },
    /// A narrowing row with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        proof_id: String,
    },
    /// A proof state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        proof_id: String,
        /// Proof state.
        state: ProofState,
        /// Reason the state requires.
        expected_reason: GapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        proof_id: String,
        /// Proof state.
        state: ProofState,
    },
    /// A requirement ref appears on more than one row.
    DuplicateRequirementRef {
        /// Duplicate requirement ref.
        requirement_ref: String,
    },
    /// A declared launch-blocking requirement ref has no covering row.
    LaunchBlockingRefWithoutRow {
        /// Uncovered requirement ref.
        requirement_ref: String,
    },
    /// A launch-blocking row's requirement ref is not in the declared set.
    LaunchBlockingRowNotInSet {
        /// Row id.
        proof_id: String,
        /// The row's requirement ref.
        requirement_ref: String,
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
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for StableProofIndexViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported index schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported index record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "index {field} is not the canonical value")
            }
            Self::EmptyIndex => write!(f, "index has no rows"),
            Self::NoProofRules => write!(f, "index has no proof rules"),
            Self::EmptyField {
                proof_id,
                field_name,
            } => write!(f, "{proof_id} has empty field {field_name}"),
            Self::DuplicateProofId { proof_id } => {
                write!(f, "duplicate proof row id {proof_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate proof rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "proof rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::ProvenWiderThanClaim {
                proof_id,
                claim,
                proven,
            } => write!(
                f,
                "proof {proof_id} proven label {} is wider than the claim ceiling {}",
                proven.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { proof_id } => write!(
                f,
                "proof {proof_id} freshness SLO warn window exceeds its target age"
            ),
            Self::HeldOnNarrowedClaim { proof_id, claim } => write!(
                f,
                "proof {proof_id} holds a proof while the public claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { proof_id } => write!(
                f,
                "proof {proof_id} backs a claim that is narrowed but does not name claim_label_narrowed"
            ),
            Self::ProvenLabelNotNarrowed {
                proof_id,
                state,
                proven,
            } => write!(
                f,
                "proof {proof_id} state {} must narrow below the cutline but backs {}",
                state.as_str(),
                proven.as_str()
            ),
            Self::NarrowingWithoutReason { proof_id, state } => write!(
                f,
                "proof {proof_id} state {} narrows without naming an active gap reason",
                state.as_str()
            ),
            Self::HeldLabelNotEqualClaim {
                proof_id,
                claim,
                proven,
            } => write!(
                f,
                "proof {proof_id} backs {} but its public claim label is {}",
                proven.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { proof_id } => write!(
                f,
                "proof {proof_id} backs its label while a gap reason is active"
            ),
            Self::HeldWithoutFreshPacket { proof_id } => write!(
                f,
                "proof {proof_id} backs its label with no captured, evidence-backed proof packet"
            ),
            Self::HeldOnStalePacket { proof_id, slo_state } => write!(
                f,
                "proof {proof_id} backs its label while its packet is {} (outside its freshness SLO)",
                slo_state.as_str()
            ),
            Self::HeldWithoutSignoff { proof_id } => {
                write!(f, "proof {proof_id} backs its label without owner sign-off")
            }
            Self::BreachedPacketWithoutReason { proof_id } => write!(
                f,
                "proof {proof_id} has a breached packet but does not name proof_packet_freshness_breached"
            ),
            Self::MissingPacketWithoutReason { proof_id } => write!(
                f,
                "proof {proof_id} has a missing packet but does not name proof_packet_missing"
            ),
            Self::StateReasonIncoherent {
                proof_id,
                state,
                expected_reason,
            } => write!(
                f,
                "proof {proof_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { proof_id, state } => write!(
                f,
                "proof {proof_id} state {} names no waiver",
                state.as_str()
            ),
            Self::DuplicateRequirementRef { requirement_ref } => {
                write!(f, "duplicate requirement ref {requirement_ref}")
            }
            Self::LaunchBlockingRefWithoutRow { requirement_ref } => write!(
                f,
                "declared launch-blocking requirement {requirement_ref} has no covering row"
            ),
            Self::LaunchBlockingRowNotInSet {
                proof_id,
                requirement_ref,
            } => write!(
                f,
                "proof {proof_id} is launch-blocking but its requirement {requirement_ref} is not in launch_blocking_requirement_refs"
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
            Self::SummaryMismatch => write!(f, "index summary counts disagree with the rows"),
        }
    }
}

impl Error for StableProofIndexViolation {}

/// Loads the embedded stable proof index.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in index no longer matches
/// [`StableProofIndex`] — including when a row carries a lifecycle label, proof
/// state, freshness-SLO state, gap reason, or index action outside the closed
/// vocabularies.
pub fn current_stable_proof_index() -> Result<StableProofIndex, serde_json::Error> {
    serde_json::from_str(STABLE_PROOF_INDEX_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index() -> StableProofIndex {
        current_stable_proof_index().expect("index parses")
    }

    #[test]
    fn embedded_index_parses_and_validates() {
        let index = index();
        assert_eq!(index.schema_version, STABLE_PROOF_INDEX_SCHEMA_VERSION);
        assert_eq!(index.record_kind, STABLE_PROOF_INDEX_RECORD_KIND);
        assert_eq!(index.validate(), Vec::new());
        assert!(!index.rows.is_empty());
    }

    #[test]
    fn every_launch_blocking_requirement_is_covered() {
        let index = index();
        let covered: BTreeSet<&str> = index
            .rows
            .iter()
            .filter(|row| row.launch_blocking)
            .map(|row| row.requirement_ref.as_str())
            .collect();
        assert!(!index.launch_blocking_requirement_refs.is_empty());
        for declared in &index.launch_blocking_requirement_refs {
            assert!(
                covered.contains(declared.as_str()),
                "{declared} has no covering launch-blocking row"
            );
        }
    }

    #[test]
    fn index_exercises_proven_and_narrowed_rows() {
        let index = index();
        assert!(
            !index.rows_proven_stable().is_empty(),
            "index must show at least one proven-stable requirement"
        );
        assert!(
            !index.rows_narrowed().is_empty(),
            "index must show at least one narrowed requirement"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let index = index();
        assert_eq!(index.summary, index.computed_summary());
        assert_eq!(
            index.summary.requirements_proven_stable
                + index.summary.requirements_narrowed_below_cutline,
            index.rows.len()
        );
        assert_eq!(
            index.summary.packets_current
                + index.summary.packets_due_for_refresh
                + index.summary.packets_breached
                + index.summary.packets_missing,
            index.rows.len()
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let index = index();
        assert_eq!(index.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            index.publication.decision,
            index.computed_publication_decision()
        );
        assert!(!index.publication.blocking_rule_ids.is_empty());
        assert!(!index.publication.blocking_proof_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let index = index();
        let covered: BTreeSet<GapReason> = index
            .proof_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in GapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_backs_wider_than_its_claim_ceiling() {
        let index = index();
        for row in &index.rows {
            assert!(
                row.proven_label.rank() <= row.claim_label.rank(),
                "{} backs wider than its ceiling",
                row.proof_id
            );
        }
    }

    #[test]
    fn validate_flags_a_proof_backed_wider_than_ceiling() {
        let mut index = index();
        let row = index
            .rows
            .iter_mut()
            .find(|row| !row.proves_stable() && row.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.proven_label = StableClaimLevel::Stable;
        let proof_id = row.proof_id.clone();
        index.summary = index.computed_summary();
        assert!(index.validate().iter().any(|v| matches!(
            v,
            StableProofIndexViolation::ProvenWiderThanClaim { proof_id: id, .. } if *id == proof_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut index = index();
        let row = index
            .rows
            .iter_mut()
            .find(|row| row.index_state == ProofState::UnprovenStale)
            .expect("an unproven-stale row exists");
        row.proven_label = row.claim_label;
        index.summary = index.computed_summary();
        index.publication.decision = index.computed_publication_decision();
        index.publication.blocking_rule_ids = index.computed_blocking_rule_ids();
        index.publication.blocking_proof_ids = index.computed_blocking_proof_ids();
        assert!(index
            .validate()
            .iter()
            .any(|v| matches!(v, StableProofIndexViolation::ProvenLabelNotNarrowed { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut index = index();
        index.publication.decision = PromotionDecision::Proceed;
        assert!(index.validate().iter().any(|v| matches!(
            v,
            StableProofIndexViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_proven_row_without_signoff() {
        let mut index = index();
        let row = index
            .rows
            .iter_mut()
            .find(|row| row.holds_proof())
            .expect("a proven row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let proof_id = row.proof_id.clone();
        index.summary = index.computed_summary();
        assert!(index
            .validate()
            .contains(&StableProofIndexViolation::HeldWithoutSignoff { proof_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let index = index();
        let projection = index.support_export_projection();
        assert_eq!(projection.rows.len(), index.rows.len());
        assert_eq!(projection.publication_decision, index.publication.decision);
        for (row, projected) in index.rows.iter().zip(&projection.rows) {
            assert_eq!(row.proof_id, projected.proof_id);
            assert_eq!(row.requirement_ref, projected.requirement_ref);
            assert_eq!(row.proves_stable(), projected.proves_stable);
            assert_eq!(row.proven_label, projected.proven_label);
            assert_eq!(row.proof_packet.slo_state, projected.slo_state);
        }
    }
}
