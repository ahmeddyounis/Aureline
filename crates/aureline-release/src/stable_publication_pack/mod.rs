//! Typed stable publication pack for the release line's known-limits, public
//! benchmark, compatibility, and migration publications.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each
//! launch-blocking *requirement* is proven; the
//! [`stable_version_windows`](crate::stable_version_windows) freezes each public
//! interface surface's version window; and the
//! [`maintenance_control_packet`](crate::maintenance_control_packet) governs each
//! post-release maintenance lane. None of them answer the question this module
//! answers: **for each outward-facing publication the release line ships about its own
//! limits and behavior — a known-limits publication, a public benchmark publication, a
//! compatibility publication, or a migration publication — is that publication actually
//! backed by a fresh proof packet, within its published p50/p95 budget where it makes a
//! performance claim, and an owner sign-off, and is it narrowed below the cutline the
//! moment its backing thins out?** This module is the **stable publication pack**. For
//! every publication it records one row that binds the publication to the
//! [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose lifecycle label it
//! backs, the proof packet that grounds it (a known-limits register, a benchmark-lab
//! trace, a compatibility report, or a migration guide), the benchmark budget it must
//! hold (for benchmark publications), the waiver (if any) holding it provisionally, and
//! the owner sign-off.
//!
//! Each [`PublicationRow`] is one `(publication, public claim)` binding. It:
//!
//! - names the publication kind it governs ([`PublicationRow::publication_kind`],
//!   [`PublicationRow::surface_ref`], [`PublicationRow::surface_summary`]) and whether
//!   that publication is part of the release-blocking publication set
//!   ([`PublicationRow::release_blocking`]);
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO and, for a
//!   benchmark publication, the [`BenchmarkBudget`] that protects the published p50/p95
//!   numbers, names the benchmark-lab trace and corpus metadata, and records whether the
//!   threshold is intentionally tightened;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose public
//!   claim it backs ([`PublicationRow::claim_ref`]) and the canonical lifecycle label
//!   that entry publishes ([`PublicationRow::claim_label`]). That label is a hard
//!   **ceiling**: a publication may carry the claim's label or narrow below it, but it
//!   may never assert a public claim wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-publication
//!   labels, so docs, Help/About, the release center, and support exports ingest one
//!   label per publication instead of cloning their own;
//! - records the publication state earned ([`PublicationState`]), the active gap reasons
//!   ([`GapReason`]), and the label it *effectively* publishes after narrowing
//!   ([`PublicationRow::published_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a publication whose backing supports a Stable public claim and one narrowed
//! below it. A publication that is not backed — because its proof packet aged out or is
//! missing, because its measured p50/p95 regressed beyond the published budget, because
//! its corpus metadata or benchmark-lab trace is missing, because its waiver expired,
//! because its evidence is incomplete, or because the public claim it backs is itself
//! below the cutline — is structurally required to drop below the cutline rather than
//! inherit an adjacent backed publication. The [`PublicationRule`] set names the closed
//! conditions that gate publication, and [`StablePublicationPack::publication`] records
//! the proceed/hold verdict.
//!
//! The pack is checked in at `artifacts/release/stable_publication_pack.json` and
//! embedded here, so this typed consumer and the CI gate agree on every row without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state, a measured/published budget
//! number, or an opaque ref. It carries no raw artifacts, raw logs, signatures, or
//! credential material. Two classes of check live outside this model because they need
//! more than the pack sees: date arithmetic (recomputing the packet-freshness state and
//! waiver expiry against an `as_of` date) and the cross-artifact ceiling check (whether
//! each row's `claim_label` still equals the label the stable claim manifest publishes
//! for the entry named by `claim_ref`). Those live in the CI gate. This model enforces
//! the structural and logical invariants that hold regardless of the clock and the
//! neighbouring artifact — the ceiling/no-widening rule, the benchmark-budget protection
//! and ordering, corpus/trace completeness, narrowing consistency, packet/state
//! coherence, owner sign-off on backed rows, publication-kind and release-line coverage,
//! publication-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported stable-publication-pack schema version.
pub const STABLE_PUBLICATION_PACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the pack.
pub const STABLE_PUBLICATION_PACK_RECORD_KIND: &str = "stable_publication_pack";

/// Repo-relative path to the checked-in pack.
pub const STABLE_PUBLICATION_PACK_PATH: &str = "artifacts/release/stable_publication_pack.json";

/// Embedded checked-in pack JSON.
pub const STABLE_PUBLICATION_PACK_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stable_publication_pack.json"
));

/// The publication kind a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationKind {
    /// A known-limits publication: the caveats and unsupported-state notes shipped about
    /// a surface.
    KnownLimit,
    /// A public benchmark publication: published p50/p95 numbers grounded in a
    /// benchmark-lab trace and corpus metadata.
    Benchmark,
    /// A compatibility publication: the compatibility report / matrix shipped about a
    /// surface.
    Compatibility,
    /// A migration publication: the migration guide / playbook shipped about a surface.
    Migration,
}

impl PublicationKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::KnownLimit,
        Self::Benchmark,
        Self::Compatibility,
        Self::Migration,
    ];

    /// Stable token recorded in the pack.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KnownLimit => "known_limit",
            Self::Benchmark => "benchmark",
            Self::Compatibility => "compatibility",
            Self::Migration => "migration",
        }
    }
}

/// Publication state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationState {
    /// The publication is backed: a captured, within-SLO proof packet (and, for a
    /// benchmark, a complete budget within its published p50/p95 numbers) backs the
    /// public claim at its full canonical lifecycle label, owner-signed.
    Published,
    /// The publication carries the claim's full label only because an active, unexpired
    /// waiver covers a recorded gap (for a benchmark, an intentionally tightened budget
    /// the measured numbers have not yet caught up to).
    PublishedOnWaiver,
    /// The proof packet or row evidence is incomplete, a benchmark's corpus metadata or
    /// trace is missing, the surface capability is absent, or owner sign-off is absent;
    /// the publication is not backed and the label must narrow.
    NarrowedUnbacked,
    /// The public claim this publication backs is itself below the cutline, so the
    /// publication inherits that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The proof packet breached its freshness SLO (or is missing); the publication is
    /// not backed and the label must narrow.
    NarrowedStale,
    /// The publication relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
    /// A benchmark publication's measured p50/p95 regressed beyond its published budget;
    /// the label must narrow until the budget is re-met, re-published, or waived.
    NarrowedBudgetRegressed,
}

impl PublicationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Published,
        Self::PublishedOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
        Self::NarrowedBudgetRegressed,
    ];

    /// Stable token recorded in the pack.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::PublishedOnWaiver => "published_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::NarrowedBudgetRegressed => "narrowed_budget_regressed",
        }
    }

    /// Whether the state lets a publication carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Published | Self::PublishedOnWaiver)
    }

    /// Whether the state forces the publication below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a publication narrows or a publication rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this publication backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The publication names a surface capability the build does not yet implement.
    SurfaceCapabilityAbsent,
    /// The proof packet's row-level evidence (caveat coverage, report scope, guide
    /// coverage) is incomplete.
    EvidenceIncomplete,
    /// A benchmark publication is missing its corpus metadata or its benchmark-lab trace.
    CorpusMetadataMissing,
    /// A benchmark publication's measured p50/p95 regressed beyond its published budget.
    BudgetRegressed,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// No proof packet has been captured for the publication.
    ProofPacketMissing,
    /// A waiver the publication relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ClaimLabelNarrowed,
        Self::SurfaceCapabilityAbsent,
        Self::EvidenceIncomplete,
        Self::CorpusMetadataMissing,
        Self::BudgetRegressed,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the pack.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::SurfaceCapabilityAbsent => "surface_capability_absent",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::CorpusMetadataMissing => "corpus_metadata_missing",
            Self::BudgetRegressed => "budget_regressed",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a publication rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the publication's published lifecycle label below the cutline.
    NarrowPublicationLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Recapture the row-level evidence the proof packet depends on.
    RecaptureEvidence,
    /// Re-meet, re-publish, or waive the benchmark p50/p95 budget.
    RetuneOrWaiveBudget,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl PublicationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowPublicationLabel,
        Self::RefreshProofPacket,
        Self::RecaptureEvidence,
        Self::RetuneOrWaiveBudget,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the pack.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowPublicationLabel => "narrow_publication_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RetuneOrWaiveBudget => "retune_or_waive_budget",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The published performance budget a benchmark publication must hold: the published
/// p50/p95 ceiling, the measured p50/p95, the corpus metadata and benchmark-lab trace
/// that ground the numbers, and whether the threshold is intentionally tightened.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkBudget {
    /// The metric / scenario the budget governs.
    pub metric_ref: String,
    /// Published p50 ceiling, milliseconds. The public promise.
    pub published_p50_ms: u32,
    /// Published p95 ceiling, milliseconds. The public promise.
    pub published_p95_ms: u32,
    /// Measured p50, milliseconds, from the benchmark-lab trace.
    pub measured_p50_ms: u32,
    /// Measured p95, milliseconds, from the benchmark-lab trace.
    pub measured_p95_ms: u32,
    /// Ref to the corpus metadata the run used.
    pub corpus_ref: String,
    /// Ref to the benchmark-lab trace backing the measured numbers.
    pub trace_ref: String,
    /// Whether the published budget is intentionally tightened below a prior baseline.
    pub tightened: bool,
}

impl BenchmarkBudget {
    /// True when the measured p50 and p95 are both at or under the published ceilings.
    pub const fn within_budget(&self) -> bool {
        self.measured_p50_ms <= self.published_p50_ms
            && self.measured_p95_ms <= self.published_p95_ms
    }

    /// True when the corpus metadata and benchmark-lab trace are both present, so the
    /// published numbers can be traced back to a run.
    pub fn is_complete(&self) -> bool {
        !self.corpus_ref.trim().is_empty() && !self.trace_ref.trim().is_empty()
    }

    /// True when the published and measured ceilings are ordered p50 <= p95 and the
    /// published p50 ceiling is a positive number of milliseconds.
    pub const fn is_ordered(&self) -> bool {
        self.published_p50_ms >= 1
            && self.published_p95_ms >= self.published_p50_ms
            && self.measured_p95_ms >= self.measured_p50_ms
    }
}

/// One publication rule: a closed condition that narrows a publication label and may gate
/// publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: PublicationAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One publication-pack row: a `(publication, public claim)` binding bound to its proof
/// packet, benchmark budget (for benchmark publications), canonical ceiling label, and
/// packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationRow {
    /// Stable publication-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The publication kind this row governs.
    pub publication_kind: PublicationKind,
    /// The surface id this publication speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the publication.
    pub surface_summary: String,
    /// Whether the publication is part of the release-blocking publication set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this publication backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a
    /// publication may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Publication state earned for the publication.
    pub publication_state: PublicationState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// The benchmark budget, present only on benchmark publications.
    #[serde(default)]
    pub benchmark_budget: Option<BenchmarkBudget>,
    /// Waiver authorizing a provisional publication, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the publication effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl PublicationRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the publication carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.publication_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when the row is a benchmark publication whose budget is intentionally
    /// tightened and held on an active waiver — the waiver hook for a tightened budget
    /// the measured numbers have not yet caught up to.
    fn waiver_covers_tightened_budget(&self) -> bool {
        self.publication_state == PublicationState::PublishedOnWaiver
            && self
                .benchmark_budget
                .as_ref()
                .map(|b| b.tightened)
                .unwrap_or(false)
    }
}

/// The recorded publication verdict for the stable publication pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Publication-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Publication-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StablePublicationPackSummary {
    /// Total number of publication rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_published_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed_below_cutline: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_published_stable: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Known-limit publications.
    pub known_limit_entries: usize,
    /// Benchmark publications.
    pub benchmark_entries: usize,
    /// Compatibility publications.
    pub compatibility_entries: usize,
    /// Migration publications.
    pub migration_entries: usize,
    /// Benchmark publications whose measured numbers are within their published budget.
    pub benchmark_budgets_within: usize,
    /// Benchmark publications whose measured numbers regressed beyond their budget.
    pub benchmark_budgets_regressed: usize,
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
    /// Number of publication rules currently firing.
    pub publication_rules_firing: usize,
}

/// The typed stable publication pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StablePublicationPack {
    /// Pack schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable pack identifier.
    pub pack_id: String,
    /// Lifecycle status of this pack artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this pack ingests as its public-claim source and
    /// ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the known-limits register every known-limit publication rides.
    pub known_limits_register_ref: String,
    /// Ref to the benchmark-publication pack template every benchmark publication rides.
    pub benchmark_pack_template_ref: String,
    /// Ref to the compatibility-report template every compatibility publication rides.
    pub compatibility_report_template_ref: String,
    /// Ref to the migration contract every migration publication rides.
    pub migration_contract_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed publication-kind vocabulary.
    pub publication_kinds: Vec<PublicationKind>,
    /// Closed publication-state vocabulary.
    pub publication_states: Vec<PublicationState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed publication-action vocabulary.
    pub publication_actions: Vec<PublicationAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this pack must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Publication rules.
    pub publication_rules: Vec<PublicationRule>,
    /// Publication rows.
    pub rows: Vec<PublicationRow>,
    /// Recorded publication verdict.
    pub publication: PackPublicationRecord,
    /// Summary counts.
    pub summary: StablePublicationPackSummary,
}

impl StablePublicationPack {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&PublicationRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&PublicationRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&PublicationRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&PublicationRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one publication kind.
    pub fn rows_for_kind(&self, kind: PublicationKind) -> Vec<&PublicationRow> {
        self.rows
            .iter()
            .filter(|row| row.publication_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the pack covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn publication_rule_fires(&self, rule: &PublicationRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and publication rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .publication_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.publication_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .publication_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.publication_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Publication-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim
    /// is already canonically narrowed is not a *publication* blocker, it merely inherits
    /// the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .publication_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.publication_rule_fires(rule))
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

    /// Recomputes the summary block from the rows and publication rules.
    pub fn computed_summary(&self) -> StablePublicationPackSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: PublicationKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&PublicationRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        let benchmark_within = self
            .rows
            .iter()
            .filter(|row| {
                row.benchmark_budget
                    .as_ref()
                    .map(|b| b.within_budget())
                    .unwrap_or(false)
            })
            .count();
        let benchmark_regressed = self
            .rows
            .iter()
            .filter(|row| {
                row.benchmark_budget
                    .as_ref()
                    .map(|b| !b.within_budget())
                    .unwrap_or(false)
            })
            .count();
        StablePublicationPackSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_published_stable: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.publication_state == PublicationState::PublishedOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_published_stable: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            known_limit_entries: kind(PublicationKind::KnownLimit),
            benchmark_entries: kind(PublicationKind::Benchmark),
            compatibility_entries: kind(PublicationKind::Compatibility),
            migration_entries: kind(PublicationKind::Migration),
            benchmark_budgets_within: benchmark_within,
            benchmark_budgets_regressed: benchmark_regressed,
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            publication_rules_firing: self
                .publication_rules
                .iter()
                .filter(|rule| self.publication_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the pack that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> PublicationPackExportProjection {
        PublicationPackExportProjection {
            pack_id: self.pack_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| PublicationPackExportRow {
                    entry_id: row.entry_id.clone(),
                    publication_kind: row.publication_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    publication_state: row.publication_state,
                    slo_state: row.proof_packet.slo_state,
                    benchmark_within_budget: row
                        .benchmark_budget
                        .as_ref()
                        .map(|b| b.within_budget()),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the pack, returning every violation found.
    pub fn validate(&self) -> Vec<StablePublicationPackViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(StablePublicationPackViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(StablePublicationPackViolation::EmptyPack);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(StablePublicationPackViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<StablePublicationPackViolation>) {
        if self.schema_version != STABLE_PUBLICATION_PACK_SCHEMA_VERSION {
            violations.push(StablePublicationPackViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != STABLE_PUBLICATION_PACK_RECORD_KIND {
            violations.push(StablePublicationPackViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("pack_id", &self.pack_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("known_limits_register_ref", &self.known_limits_register_ref),
            (
                "benchmark_pack_template_ref",
                &self.benchmark_pack_template_ref,
            ),
            (
                "compatibility_report_template_ref",
                &self.compatibility_report_template_ref,
            ),
            ("migration_contract_ref", &self.migration_contract_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StablePublicationPackViolation::EmptyField {
                    entry_id: "<pack>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(StablePublicationPackViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.publication_kinds != PublicationKind::ALL.to_vec() {
            violations.push(StablePublicationPackViolation::ClosedVocabularyMismatch {
                field: "publication_kinds",
            });
        }
        if self.publication_states != PublicationState::ALL.to_vec() {
            violations.push(StablePublicationPackViolation::ClosedVocabularyMismatch {
                field: "publication_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(StablePublicationPackViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.publication_actions != PublicationAction::ALL.to_vec() {
            violations.push(StablePublicationPackViolation::ClosedVocabularyMismatch {
                field: "publication_actions",
            });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(StablePublicationPackViolation::EmptyField {
                entry_id: "<pack>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(StablePublicationPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(StablePublicationPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(StablePublicationPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(StablePublicationPackViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<StablePublicationPackViolation>) {
        if self.publication_rules.is_empty() {
            violations.push(StablePublicationPackViolation::NoPublicationRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.publication_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(StablePublicationPackViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(StablePublicationPackViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(StablePublicationPackViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every gap reason must have a rule, so a gap reason cannot fire without a
        // corresponding publication gate.
        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(StablePublicationPackViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &PublicationRow,
        violations: &mut Vec<StablePublicationPackViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
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
                violations.push(StablePublicationPackViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no publication may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(StablePublicationPackViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn window
        // may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(StablePublicationPackViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(StablePublicationPackViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        self.validate_benchmark(row, violations);

        // A public claim whose canonical label is below the cutline forces the
        // publication to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(StablePublicationPackViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                violations.push(StablePublicationPackViolation::ClaimNarrowedWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed row carries exactly the public claim's canonical label, carries no
            // active gap reason, rides a captured within-SLO packet, holds its benchmark
            // budget (unless an active waiver covers an intentionally tightened budget),
            // and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(StablePublicationPackViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(StablePublicationPackViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(StablePublicationPackViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(StablePublicationPackViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if let Some(budget) = &row.benchmark_budget {
                if !budget.is_complete() {
                    violations.push(StablePublicationPackViolation::HeldWithIncompleteBudget {
                        entry_id: row.entry_id.clone(),
                    });
                }
                if !budget.within_budget() && !row.waiver_covers_tightened_budget() {
                    violations.push(StablePublicationPackViolation::HeldOverBudget {
                        entry_id: row.entry_id.clone(),
                    });
                }
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(StablePublicationPackViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline and name
            // at least one active reason.
            if row.publishes_stable() {
                violations.push(StablePublicationPackViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.publication_state,
                    published: row.published_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(StablePublicationPackViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.publication_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the matching
            // freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
            {
                violations.push(
                    StablePublicationPackViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::ProofPacketMissing)
            {
                violations.push(StablePublicationPackViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            // A narrowing benchmark row over its budget must name the budget-regressed
            // reason, so the budget-protection automation stays honest.
            if let Some(budget) = &row.benchmark_budget {
                if !budget.within_budget() && !row.has_active_reason(GapReason::BudgetRegressed) {
                    violations.push(
                        StablePublicationPackViolation::BudgetRegressedWithoutReason {
                            entry_id: row.entry_id.clone(),
                        },
                    );
                }
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_benchmark(
        &self,
        row: &PublicationRow,
        violations: &mut Vec<StablePublicationPackViolation>,
    ) {
        match (row.publication_kind, &row.benchmark_budget) {
            (PublicationKind::Benchmark, None) => {
                violations.push(StablePublicationPackViolation::BenchmarkRowWithoutBudget {
                    entry_id: row.entry_id.clone(),
                });
            }
            (kind, Some(_)) if kind != PublicationKind::Benchmark => {
                violations.push(StablePublicationPackViolation::NonBenchmarkRowWithBudget {
                    entry_id: row.entry_id.clone(),
                });
            }
            (PublicationKind::Benchmark, Some(budget)) => {
                if budget.metric_ref.trim().is_empty() {
                    violations.push(StablePublicationPackViolation::EmptyField {
                        entry_id: row.entry_id.clone(),
                        field_name: "benchmark_budget.metric_ref",
                    });
                }
                if !budget.is_ordered() {
                    violations.push(StablePublicationPackViolation::BudgetDisordered {
                        entry_id: row.entry_id.clone(),
                    });
                }
                // Corpus / trace completeness coherence: a row carrying the
                // corpus-metadata-missing reason must actually be missing one, and an
                // incomplete budget must name the reason.
                let incomplete = !budget.is_complete();
                if row.has_active_reason(GapReason::CorpusMetadataMissing) && !incomplete {
                    violations.push(
                        StablePublicationPackViolation::CorpusReasonWithoutIncomplete {
                            entry_id: row.entry_id.clone(),
                        },
                    );
                }
                if incomplete && !row.has_active_reason(GapReason::CorpusMetadataMissing) {
                    violations.push(
                        StablePublicationPackViolation::IncompleteBudgetWithoutReason {
                            entry_id: row.entry_id.clone(),
                        },
                    );
                }
                // Budget-regression honesty: a row that names the budget-regressed reason
                // must actually be over its published budget.
                if row.has_active_reason(GapReason::BudgetRegressed) && budget.within_budget() {
                    violations.push(
                        StablePublicationPackViolation::BudgetRegressedReasonWithoutRegression {
                            entry_id: row.entry_id.clone(),
                        },
                    );
                }
            }
            _ => {}
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &PublicationRow,
        violations: &mut Vec<StablePublicationPackViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<StablePublicationPackViolation>,
                               expected: GapReason| {
            violations.push(StablePublicationPackViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.publication_state,
                expected_reason: expected,
            });
        };

        match row.publication_state {
            PublicationState::NarrowedUnbacked => {
                const ALLOWED: [GapReason; 4] = [
                    GapReason::SurfaceCapabilityAbsent,
                    GapReason::EvidenceIncomplete,
                    GapReason::CorpusMetadataMissing,
                    GapReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, GapReason::EvidenceIncomplete);
                }
            }
            PublicationState::NarrowedClaimNarrowed => {
                if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, GapReason::ClaimLabelNarrowed);
                }
            }
            PublicationState::NarrowedStale => {
                if !(row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
                    || row.has_active_reason(GapReason::ProofPacketMissing))
                {
                    push_incoherent(violations, GapReason::ProofPacketFreshnessBreached);
                }
            }
            PublicationState::NarrowedWaiverExpired => {
                if !row.has_active_reason(GapReason::WaiverExpired) {
                    push_incoherent(violations, GapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(StablePublicationPackViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.publication_state,
                    });
                }
            }
            PublicationState::NarrowedBudgetRegressed => {
                if !row.has_active_reason(GapReason::BudgetRegressed) {
                    push_incoherent(violations, GapReason::BudgetRegressed);
                }
                // Only a benchmark publication can regress a budget.
                if row.publication_kind != PublicationKind::Benchmark {
                    violations.push(StablePublicationPackViolation::BudgetStateOnNonBenchmark {
                        entry_id: row.entry_id.clone(),
                    });
                }
            }
            PublicationState::PublishedOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(StablePublicationPackViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.publication_state,
                    });
                }
            }
            PublicationState::Published => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<StablePublicationPackViolation>) {
        // Each surface ref appears at most once: a publication has one canonical row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.surface_ref.as_str()) {
                violations.push(StablePublicationPackViolation::DuplicateSurfaceRef {
                    surface_ref: row.surface_ref.clone(),
                });
            }
        }

        // The release line must cover every declared release-blocking surface with
        // exactly one release-blocking row, and every release-blocking row must be
        // declared, so a publication cannot quietly drop out of the pack.
        let declared: BTreeSet<&str> = self
            .release_blocking_surface_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(
                    StablePublicationPackViolation::ReleaseBlockingRefWithoutRow {
                        surface_ref: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.surface_ref.as_str()) {
                violations.push(StablePublicationPackViolation::ReleaseBlockingRowNotInSet {
                    entry_id: row.entry_id.clone(),
                    surface_ref: row.surface_ref.clone(),
                });
            }
        }

        // The pack must cover all four publication kinds — known-limit, benchmark,
        // compatibility, and migration — so the release line cannot publish some kinds
        // and silently leave a whole publication kind ungoverned.
        for kind in PublicationKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(StablePublicationPackViolation::PublicationKindAbsent { kind });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<StablePublicationPackViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(StablePublicationPackViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(StablePublicationPackViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                StablePublicationPackViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                StablePublicationPackViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                StablePublicationPackViolation::PublicationBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationPackExportRow {
    /// Stable publication-row id.
    pub entry_id: String,
    /// Publication kind.
    pub publication_kind: PublicationKind,
    /// Publication surface ref.
    pub surface_ref: String,
    /// Whether the publication is part of the release-blocking set.
    pub release_blocking: bool,
    /// The public-claim entry ref the publication backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the publication carries.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes a label at or above the cutline.
    pub publishes_stable: bool,
    /// Publication state.
    pub publication_state: PublicationState,
    /// Proof-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Whether the benchmark budget is within its published numbers, or `None` for a
    /// non-benchmark publication.
    pub benchmark_within_budget: Option<bool>,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// A redaction-safe export projection of the pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationPackExportProjection {
    /// Pack id this projection was produced from.
    pub pack_id: String,
    /// Pack as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<PublicationPackExportRow>,
}

/// A validation violation for the stable publication pack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StablePublicationPackViolation {
    /// The pack carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the pack.
        actual: u32,
    },
    /// The pack carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the pack.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The pack has no rows.
    EmptyPack,
    /// The pack has no publication rules.
    NoPublicationRules,
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
    /// A publication rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: GapReason,
    },
    /// A published label is wider than the public claim's canonical label.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
    /// A benchmark publication carries no benchmark budget.
    BenchmarkRowWithoutBudget {
        /// Row id.
        entry_id: String,
    },
    /// A non-benchmark publication carries a benchmark budget.
    NonBenchmarkRowWithBudget {
        /// Row id.
        entry_id: String,
    },
    /// A benchmark budget is not ordered p50 <= p95 with a positive published floor.
    BudgetDisordered {
        /// Row id.
        entry_id: String,
    },
    /// A row carries the corpus-metadata-missing reason but the budget is complete.
    CorpusReasonWithoutIncomplete {
        /// Row id.
        entry_id: String,
    },
    /// A benchmark budget is missing corpus metadata or a trace but does not name the
    /// reason.
    IncompleteBudgetWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row names the budget-regressed reason but is within its published budget.
    BudgetRegressedReasonWithoutRegression {
        /// Row id.
        entry_id: String,
    },
    /// A row holds its label while the public claim's canonical label is narrowed.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
    },
    /// A row whose claim is narrowed does not carry the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Publication state.
        state: PublicationState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Publication state.
        state: PublicationState,
    },
    /// A backed row's published label is not equal to its claim ceiling label.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A backed row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A backed row rides a proof packet with no capture or evidence.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A backed row rides a proof packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// The packet's freshness-SLO state.
        slo_state: FreshnessSloState,
    },
    /// A backed benchmark row carries an incomplete budget (missing corpus/trace).
    HeldWithIncompleteBudget {
        /// Row id.
        entry_id: String,
    },
    /// A backed benchmark row is over its published budget without a tightened-budget
    /// waiver.
    HeldOverBudget {
        /// Row id.
        entry_id: String,
    },
    /// A backed row has no owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a breached packet does not name the breach reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing benchmark row over budget does not name the budget-regressed reason.
    BudgetRegressedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A publication state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Publication state.
        state: PublicationState,
        /// Reason the state requires.
        expected_reason: GapReason,
    },
    /// A budget-regressed state names a non-benchmark publication.
    BudgetStateOnNonBenchmark {
        /// Row id.
        entry_id: String,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Publication state.
        state: PublicationState,
    },
    /// A surface ref appears on more than one row.
    DuplicateSurfaceRef {
        /// Duplicate surface ref.
        surface_ref: String,
    },
    /// A declared release-blocking surface ref has no covering row.
    ReleaseBlockingRefWithoutRow {
        /// Uncovered surface ref.
        surface_ref: String,
    },
    /// A release-blocking row's surface ref is not in the declared set.
    ReleaseBlockingRowNotInSet {
        /// Row id.
        entry_id: String,
        /// The row's surface ref.
        surface_ref: String,
    },
    /// A publication kind is not covered by any row.
    PublicationKindAbsent {
        /// The uncovered publication kind.
        kind: PublicationKind,
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

impl fmt::Display for StablePublicationPackViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported pack schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported pack record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "pack {field} is not the canonical value")
            }
            Self::EmptyPack => write!(f, "pack has no rows"),
            Self::NoPublicationRules => write!(f, "pack has no publication rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate publication row id {entry_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate publication rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "publication rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "publication {entry_id} published label {} is wider than the claim ceiling {}",
                published.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { entry_id } => write!(
                f,
                "publication {entry_id} freshness SLO warn window exceeds its target age"
            ),
            Self::BenchmarkRowWithoutBudget { entry_id } => write!(
                f,
                "benchmark publication {entry_id} carries no benchmark_budget"
            ),
            Self::NonBenchmarkRowWithBudget { entry_id } => write!(
                f,
                "non-benchmark publication {entry_id} carries a benchmark_budget"
            ),
            Self::BudgetDisordered { entry_id } => write!(
                f,
                "publication {entry_id} benchmark budget is not ordered p50 <= p95 with a positive published floor"
            ),
            Self::CorpusReasonWithoutIncomplete { entry_id } => write!(
                f,
                "publication {entry_id} names corpus_metadata_missing but the budget carries corpus and trace refs"
            ),
            Self::IncompleteBudgetWithoutReason { entry_id } => write!(
                f,
                "publication {entry_id} benchmark budget is missing corpus or trace refs but does not name corpus_metadata_missing"
            ),
            Self::BudgetRegressedReasonWithoutRegression { entry_id } => write!(
                f,
                "publication {entry_id} names budget_regressed but the measured numbers are within the published budget"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "publication {entry_id} holds its label while the public claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { entry_id } => write!(
                f,
                "publication {entry_id} backs a claim that is narrowed but does not name claim_label_narrowed"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "publication {entry_id} state {} must narrow below the cutline but publishes {}",
                state.as_str(),
                published.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "publication {entry_id} state {} narrows without naming an active gap reason",
                state.as_str()
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "publication {entry_id} publishes {} but its public claim label is {}",
                published.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { entry_id } => write!(
                f,
                "publication {entry_id} holds its label while a gap reason is active"
            ),
            Self::HeldWithoutFreshPacket { entry_id } => write!(
                f,
                "publication {entry_id} holds its label with no captured, evidence-backed proof packet"
            ),
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "publication {entry_id} holds its label while its packet is {} (outside its freshness SLO)",
                slo_state.as_str()
            ),
            Self::HeldWithIncompleteBudget { entry_id } => write!(
                f,
                "publication {entry_id} holds its label with a benchmark budget missing corpus or trace refs"
            ),
            Self::HeldOverBudget { entry_id } => write!(
                f,
                "publication {entry_id} holds its label while its measured numbers exceed the published budget without a tightened-budget waiver"
            ),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "publication {entry_id} holds its label without owner sign-off")
            }
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "publication {entry_id} has a breached packet but does not name proof_packet_freshness_breached"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "publication {entry_id} has a missing packet but does not name proof_packet_missing"
            ),
            Self::BudgetRegressedWithoutReason { entry_id } => write!(
                f,
                "publication {entry_id} is over its benchmark budget but does not name budget_regressed"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "publication {entry_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::BudgetStateOnNonBenchmark { entry_id } => write!(
                f,
                "publication {entry_id} is narrowed_budget_regressed but is not a benchmark publication"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => write!(
                f,
                "publication {entry_id} state {} names no waiver",
                state.as_str()
            ),
            Self::DuplicateSurfaceRef { surface_ref } => {
                write!(f, "duplicate surface ref {surface_ref}")
            }
            Self::ReleaseBlockingRefWithoutRow { surface_ref } => write!(
                f,
                "declared release-blocking surface {surface_ref} has no covering row"
            ),
            Self::ReleaseBlockingRowNotInSet {
                entry_id,
                surface_ref,
            } => write!(
                f,
                "publication {entry_id} is release-blocking but its surface {surface_ref} is not in release_blocking_surface_refs"
            ),
            Self::PublicationKindAbsent { kind } => write!(
                f,
                "publication kind {} is not covered by any row",
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
            Self::SummaryMismatch => write!(f, "pack summary counts disagree with the rows"),
        }
    }
}

impl Error for StablePublicationPackViolation {}

/// Loads the embedded stable publication pack.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in pack no longer matches
/// [`StablePublicationPack`] — including when a row carries a lifecycle label, publication
/// kind, publication state, freshness-SLO state, gap reason, or publication action
/// outside the closed vocabularies.
pub fn current_stable_publication_pack() -> Result<StablePublicationPack, serde_json::Error> {
    serde_json::from_str(STABLE_PUBLICATION_PACK_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack() -> StablePublicationPack {
        current_stable_publication_pack().expect("pack parses")
    }

    #[test]
    fn embedded_pack_parses_and_validates() {
        let pack = pack();
        assert_eq!(pack.schema_version, STABLE_PUBLICATION_PACK_SCHEMA_VERSION);
        assert_eq!(pack.record_kind, STABLE_PUBLICATION_PACK_RECORD_KIND);
        assert_eq!(pack.validate(), Vec::new());
        assert!(!pack.rows.is_empty());
    }

    #[test]
    fn every_publication_kind_is_covered() {
        let pack = pack();
        for kind in PublicationKind::ALL {
            assert!(
                !pack.rows_for_kind(kind).is_empty(),
                "publication kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_release_blocking_surface_is_covered() {
        let pack = pack();
        let covered: BTreeSet<&str> = pack
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.as_str())
            .collect();
        assert!(!pack.release_blocking_surface_refs.is_empty());
        for declared in &pack.release_blocking_surface_refs {
            assert!(
                covered.contains(declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn pack_exercises_published_and_narrowed_rows() {
        let pack = pack();
        assert!(
            !pack.rows_published_stable().is_empty(),
            "pack must show at least one published-stable row"
        );
        assert!(
            !pack.rows_narrowed().is_empty(),
            "pack must show at least one narrowed row"
        );
    }

    #[test]
    fn pack_protects_a_benchmark_budget() {
        let pack = pack();
        let regressed = pack.rows.iter().find(|row| {
            row.publication_kind == PublicationKind::Benchmark
                && row.publication_state == PublicationState::NarrowedBudgetRegressed
        });
        assert!(
            regressed.is_some(),
            "pack must narrow at least one benchmark publication for a budget regression"
        );
        let regressed = regressed.unwrap();
        let budget = regressed
            .benchmark_budget
            .as_ref()
            .expect("a benchmark row carries a budget");
        assert!(
            !budget.within_budget(),
            "a budget-regressed row must actually be over its published budget"
        );
        assert!(regressed.has_active_reason(GapReason::BudgetRegressed));
        assert!(!regressed.publishes_stable());
    }

    #[test]
    fn summary_counts_match_rows() {
        let pack = pack();
        assert_eq!(pack.summary, pack.computed_summary());
        assert_eq!(
            pack.summary.entries_published_stable + pack.summary.entries_narrowed_below_cutline,
            pack.rows.len()
        );
        assert_eq!(
            pack.summary.packets_current
                + pack.summary.packets_due_for_refresh
                + pack.summary.packets_breached
                + pack.summary.packets_missing,
            pack.rows.len()
        );
        assert_eq!(
            pack.summary.known_limit_entries
                + pack.summary.benchmark_entries
                + pack.summary.compatibility_entries
                + pack.summary.migration_entries,
            pack.rows.len()
        );
        assert_eq!(
            pack.summary.benchmark_budgets_within + pack.summary.benchmark_budgets_regressed,
            pack.summary.benchmark_entries
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let pack = pack();
        assert_eq!(pack.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            pack.publication.decision,
            pack.computed_publication_decision()
        );
        assert!(!pack.publication.blocking_rule_ids.is_empty());
        assert!(!pack.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let pack = pack();
        let covered: BTreeSet<GapReason> = pack
            .publication_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in GapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_publishes_wider_than_its_claim_ceiling() {
        let pack = pack();
        for row in &pack.rows {
            assert!(
                row.published_label.rank() <= row.claim_label.rank(),
                "{} publishes wider than its ceiling",
                row.entry_id
            );
        }
    }

    #[test]
    fn validate_flags_a_publication_wider_than_ceiling() {
        let mut pack = pack();
        let row = pack
            .rows
            .iter_mut()
            .find(|row| !row.publishes_stable() && row.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.published_label = StableClaimLevel::Stable;
        let entry_id = row.entry_id.clone();
        pack.summary = pack.computed_summary();
        assert!(pack.validate().iter().any(|v| matches!(
            v,
            StablePublicationPackViolation::PublishedWiderThanClaim { entry_id: id, .. } if *id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut pack = pack();
        let row = pack
            .rows
            .iter_mut()
            .find(|row| row.publication_state == PublicationState::NarrowedStale)
            .expect("a narrowed-stale row exists");
        row.published_label = row.claim_label;
        pack.summary = pack.computed_summary();
        pack.publication.decision = pack.computed_publication_decision();
        pack.publication.blocking_rule_ids = pack.computed_blocking_rule_ids();
        pack.publication.blocking_entry_ids = pack.computed_blocking_entry_ids();
        assert!(pack.validate().iter().any(|v| matches!(
            v,
            StablePublicationPackViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_a_benchmark_held_over_budget() {
        let mut pack = pack();
        let row = pack
            .rows
            .iter_mut()
            .find(|row| {
                row.publication_state == PublicationState::Published
                    && row.benchmark_budget.is_some()
            })
            .expect("a published benchmark row exists");
        let budget = row.benchmark_budget.as_mut().unwrap();
        budget.measured_p95_ms = budget.published_p95_ms + 1_000;
        pack.summary = pack.computed_summary();
        assert!(pack
            .validate()
            .iter()
            .any(|v| matches!(v, StablePublicationPackViolation::HeldOverBudget { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut pack = pack();
        pack.publication.decision = PromotionDecision::Proceed;
        assert!(pack.validate().iter().any(|v| matches!(
            v,
            StablePublicationPackViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_backed_row_without_signoff() {
        let mut pack = pack();
        let row = pack
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("a backed row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let entry_id = row.entry_id.clone();
        pack.summary = pack.computed_summary();
        assert!(pack
            .validate()
            .contains(&StablePublicationPackViolation::HeldWithoutSignoff { entry_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let pack = pack();
        let projection = pack.support_export_projection();
        assert_eq!(projection.rows.len(), pack.rows.len());
        assert_eq!(projection.publication_decision, pack.publication.decision);
        for (row, projected) in pack.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.surface_ref, projected.surface_ref);
            assert_eq!(row.publishes_stable(), projected.publishes_stable);
            assert_eq!(row.published_label, projected.published_label);
            assert_eq!(row.proof_packet.slo_state, projected.slo_state);
        }
    }
}
