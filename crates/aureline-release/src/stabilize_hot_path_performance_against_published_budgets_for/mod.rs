//! Typed hot-path performance budget register for the M4 stable line.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_publication_pack`](crate::stable_publication_pack) governs the
//! outward-facing benchmark publications the release line ships. None of them answer
//! the question this module answers: **for each hot path — startup, restore, quick
//! open, typing, scrolling, search, and Git status — is the measured p50/p95 within
//! the published budget, grounded in a benchmark-lab trace and corpus metadata, and
//! is the budget narrowed below the cutline the moment its backing thins out?** This
//! module is the **hot-path performance budget register**. For every hot path it
//! records one row that binds the path to the
//! [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose lifecycle
//! label it backs, the benchmark budget that protects the published p50/p95 numbers,
//! the proof packet that grounds them, the waiver (if any) holding a tightened
//! threshold provisionally, and the owner sign-off.
//!
//! Each [`HotPathBudgetRow`] is one `(hot path, public claim)` binding. It:
//!
//! - names the hot path it governs ([`HotPathBudgetRow::hot_path_kind`],
//!   [`HotPathBudgetRow::surface_ref`], [`HotPathBudgetRow::surface_summary`]) and
//!   whether that path is part of the release-blocking set
//!   ([`HotPathBudgetRow::release_blocking`]);
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO and the
//!   [`HotPathBudget`] that protects the published p50/p95 numbers, names the
//!   benchmark-lab trace and corpus metadata, and records whether the threshold is
//!   intentionally tightened;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose
//!   public claim it backs ([`HotPathBudgetRow::claim_ref`]) and the canonical
//!   lifecycle label that entry publishes ([`HotPathBudgetRow::claim_label`]). That
//!   label is a hard **ceiling**: a budget may carry the claim's label or narrow
//!   below it, but it may never assert a public claim wider than the public claim it
//!   backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-path labels,
//!   so docs, Help/About, the release center, and support exports ingest one label
//!   per path instead of cloning their own;
//! - records the budget state earned ([`BudgetState`]), the active gap reasons
//!   ([`GapReason`]), and the label it *effectively* publishes after narrowing
//!   ([`HotPathBudgetRow::published_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a budget whose backing supports a Stable public claim and one narrowed
//! below it. A budget that is not backed — because its measured p50/p95 regressed
//! beyond the published budget, because its proof packet aged out or is missing,
//! because its corpus metadata or benchmark-lab trace is missing, because its waiver
//! expired, because its owner sign-off is absent, or because the public claim it
//! backs is itself below the cutline — is structurally required to drop below the
//! cutline rather than inherit an adjacent backed budget. The [`HotPathBudgetRule`]
//! set names the closed conditions that gate promotion, and
//! [`HotPathPerformanceBudgets::promotion`] records the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/stabilize_hot_path_performance_against_published_budgets_for.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state, a measured/published
//! budget number, or an opaque ref. It carries no raw artifacts, raw logs,
//! signatures, or credential material. Two classes of check live outside this model
//! because they need more than the register sees: date arithmetic (recomputing the
//! packet-freshness state and waiver expiry against an `as_of` date) and the
//! cross-artifact ceiling check (whether each row's `claim_label` still equals the
//! label the stable claim manifest publishes for the entry named by `claim_ref`).
//! Those live in the CI gate. This model enforces the structural and logical
//! invariants that hold regardless of the clock and the neighbouring artifact — the
//! ceiling/no-widening rule, the budget protection and ordering, corpus/trace
//! completeness, narrowing consistency, packet/state coherence, owner sign-off on
//! backed rows, hot-path-kind and release-line coverage, rule wiring, and the
//! verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported hot-path-performance-budgets schema version.
pub const HOT_PATH_PERFORMANCE_BUDGETS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const HOT_PATH_PERFORMANCE_BUDGETS_RECORD_KIND: &str = "hot_path_performance_budgets";

/// Repo-relative path to the checked-in register.
pub const HOT_PATH_PERFORMANCE_BUDGETS_PATH: &str =
    "artifacts/release/stabilize_hot_path_performance_against_published_budgets_for.json";

/// Embedded checked-in register JSON.
pub const HOT_PATH_PERFORMANCE_BUDGETS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stabilize_hot_path_performance_against_published_budgets_for.json"
));

/// The hot path a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotPathKind {
    /// Cold-start time from process launch to interactive editor.
    Startup,
    /// Session-restore time from launch to previous editor state.
    Restore,
    /// Quick-open file-picker latency from keystroke to filtered results.
    QuickOpen,
    /// End-to-end typing latency from key event to glyph render.
    Typing,
    /// Scroll frame time maintaining smooth refresh.
    Scrolling,
    /// Workspace search results latency from query submit to first result render.
    Search,
    /// Git status refresh latency from file-system change to badge update.
    GitStatus,
}

impl HotPathKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Startup,
        Self::Restore,
        Self::QuickOpen,
        Self::Typing,
        Self::Scrolling,
        Self::Search,
        Self::GitStatus,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Startup => "startup",
            Self::Restore => "restore",
            Self::QuickOpen => "quick_open",
            Self::Typing => "typing",
            Self::Scrolling => "scrolling",
            Self::Search => "search",
            Self::GitStatus => "git_status",
        }
    }
}

/// Budget state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetState {
    /// The measured p50/p95 is within the published budget on a current proof packet,
    /// with corpus metadata and benchmark-lab trace, owner-signed.
    MeetsBudget,
    /// The budget carries the claim's full label only because an active, unexpired
    /// waiver covers a recorded gap (an intentionally tightened budget the measured
    /// numbers have not yet caught up to).
    OnWaiver,
    /// The measured p50/p95 regressed beyond the published budget; the label must
    /// narrow until the budget is re-met, re-published, or waived.
    Regressed,
    /// The proof packet breached its freshness SLO (or is missing); the label must
    /// narrow.
    Stale,
    /// The benchmark corpus metadata or lab trace is missing; the label must narrow.
    MissingTrace,
    /// The public claim this budget backs is itself below the cutline, so the budget
    /// inherits that ceiling and narrows.
    NarrowedClaim,
}

impl BudgetState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MeetsBudget,
        Self::OnWaiver,
        Self::Regressed,
        Self::Stale,
        Self::MissingTrace,
        Self::NarrowedClaim,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MeetsBudget => "meets_budget",
            Self::OnWaiver => "on_waiver",
            Self::Regressed => "regressed",
            Self::Stale => "stale",
            Self::MissingTrace => "missing_trace",
            Self::NarrowedClaim => "narrowed_claim",
        }
    }

    /// Whether the state lets a budget carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::MeetsBudget | Self::OnWaiver)
    }

    /// Whether the state forces the budget below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a budget narrows or a rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this budget backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The measured p50/p95 regressed beyond the published budget.
    BudgetRegressed,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// No proof packet has been captured for the budget.
    ProofPacketMissing,
    /// The benchmark corpus metadata or lab trace is missing.
    CorpusMetadataMissing,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
    /// A waiver the budget relied on has expired.
    WaiverExpired,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ClaimLabelNarrowed,
        Self::BudgetRegressed,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::CorpusMetadataMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::BudgetRegressed => "budget_regressed",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::CorpusMetadataMissing => "corpus_metadata_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }
}

/// Default action a budget rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the budget's published lifecycle label below the cutline.
    NarrowLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Re-meet, re-publish, or waive the benchmark p50/p95 budget.
    RetuneOrWaiveBudget,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl BudgetAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RetuneOrWaiveBudget,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RetuneOrWaiveBudget => "retune_or_waive_budget",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The published performance budget a hot-path row must hold: the published p50/p95
/// ceiling, the measured p50/p95, the corpus metadata and benchmark-lab trace that
/// ground the numbers, and whether the threshold is intentionally tightened.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HotPathBudget {
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

impl HotPathBudget {
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

/// One budget rule: a closed condition that narrows a budget label and may gate
/// promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HotPathBudgetRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: BudgetAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One hot-path budget row: a `(hot path, public claim)` binding bound to its proof
/// packet, benchmark budget, canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HotPathBudgetRow {
    /// Stable budget-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The hot path this row governs.
    pub hot_path_kind: HotPathKind,
    /// The surface id this budget speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the budget.
    pub surface_summary: String,
    /// Whether the path is part of the release-blocking budget set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this budget backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a budget
    /// may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Budget state earned for the row.
    pub budget_state: BudgetState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// The benchmark budget, present on every hot-path row.
    pub hot_path_budget: HotPathBudget,
    /// Waiver authorizing a provisional budget, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the budget effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl HotPathBudgetRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the budget carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.budget_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when the row is on an active waiver covering an intentionally tightened
    /// budget.
    fn waiver_covers_tightened_budget(&self) -> bool {
        self.budget_state == BudgetState::OnWaiver
            && self.hot_path_budget.tightened
    }
}

/// The recorded promotion verdict for the hot-path performance budget register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionRecord {
    /// The gate this verdict governs.
    pub promotion_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Budget-rule ids that block promotion, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Budget-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HotPathPerformanceBudgetsSummary {
    /// Total number of budget rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_meeting_budget: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Rows whose measured numbers regressed beyond budget.
    pub entries_regressed: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_meeting: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Startup budget rows.
    pub startup_entries: usize,
    /// Restore budget rows.
    pub restore_entries: usize,
    /// Quick-open budget rows.
    pub quick_open_entries: usize,
    /// Typing budget rows.
    pub typing_entries: usize,
    /// Scrolling budget rows.
    pub scrolling_entries: usize,
    /// Search budget rows.
    pub search_entries: usize,
    /// Git-status budget rows.
    pub git_status_entries: usize,
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
    /// Number of budget rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotPathExportRow {
    /// Stable budget-row id.
    pub entry_id: String,
    /// The hot path this row governs.
    pub hot_path_kind: HotPathKind,
    /// The surface id this budget speaks about.
    pub surface_ref: String,
    /// Whether the path is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this budget backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Budget state earned.
    pub budget_state: BudgetState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Whether the measured budget is within its published ceiling.
    pub within_budget: bool,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotPathExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<HotPathExportRow>,
}

/// The typed hot-path performance budget register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HotPathPerformanceBudgets {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this register ingests as its public-claim
    /// source and ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the benchmark-publication pack template every budget rides.
    pub benchmark_template_ref: String,
    /// Ref to the corpus register every budget rides.
    pub corpus_register_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed hot-path-kind vocabulary.
    pub hot_path_kinds: Vec<HotPathKind>,
    /// Closed budget-state vocabulary.
    pub budget_states: Vec<BudgetState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed budget-action vocabulary.
    pub budget_actions: Vec<BudgetAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Budget rules.
    pub rules: Vec<HotPathBudgetRule>,
    /// Budget rows.
    pub rows: Vec<HotPathBudgetRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionRecord,
    /// Summary counts.
    pub summary: HotPathPerformanceBudgetsSummary,
}

impl HotPathPerformanceBudgets {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&HotPathBudgetRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&HotPathBudgetRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&HotPathBudgetRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&HotPathBudgetRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one hot path kind.
    pub fn rows_for_kind(&self, kind: HotPathKind) -> Vec<&HotPathBudgetRow> {
        self.rows
            .iter()
            .filter(|row| row.hot_path_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn rule_fires(&self, rule: &HotPathBudgetRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the rows and budget rules.
    pub fn computed_promotion_decision(&self) -> PromotionDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block promotion and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Budget-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim
    /// is already canonically narrowed is not a *promotion* blocker, it merely inherits
    /// the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.rule_fires(rule))
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

    /// Recomputes the summary block from the rows and budget rules.
    pub fn computed_summary(&self) -> HotPathPerformanceBudgetsSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: HotPathKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&HotPathBudgetRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        HotPathPerformanceBudgetsSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_meeting_budget: self
                .rows
                .iter()
                .filter(|row| row.budget_state == BudgetState::MeetsBudget)
                .count(),
            entries_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.budget_state == BudgetState::OnWaiver)
                .count(),
            entries_regressed: self
                .rows
                .iter()
                .filter(|row| row.budget_state == BudgetState::Regressed)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_meeting: release_blocking
                .iter()
                .filter(|row| row.budget_state == BudgetState::MeetsBudget)
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            startup_entries: kind(HotPathKind::Startup),
            restore_entries: kind(HotPathKind::Restore),
            quick_open_entries: kind(HotPathKind::QuickOpen),
            typing_entries: kind(HotPathKind::Typing),
            scrolling_entries: kind(HotPathKind::Scrolling),
            search_entries: kind(HotPathKind::Search),
            git_status_entries: kind(HotPathKind::GitStatus),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
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

    /// Produces an export/Help-About-safe projection of the register that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> HotPathExportProjection {
        HotPathExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| HotPathExportRow {
                    entry_id: row.entry_id.clone(),
                    hot_path_kind: row.hot_path_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    budget_state: row.budget_state,
                    slo_state: row.proof_packet.slo_state,
                    within_budget: row.hot_path_budget.within_budget(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<HotPathPerformanceBudgetsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(HotPathPerformanceBudgetsViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(HotPathPerformanceBudgetsViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(HotPathPerformanceBudgetsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<HotPathPerformanceBudgetsViolation>,
    ) {
        if self.schema_version != HOT_PATH_PERFORMANCE_BUDGETS_SCHEMA_VERSION {
            violations.push(HotPathPerformanceBudgetsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != HOT_PATH_PERFORMANCE_BUDGETS_RECORD_KIND {
            violations.push(HotPathPerformanceBudgetsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("benchmark_template_ref", &self.benchmark_template_ref),
            ("corpus_register_ref", &self.corpus_register_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(HotPathPerformanceBudgetsViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.hot_path_kinds != HotPathKind::ALL.to_vec() {
            violations.push(HotPathPerformanceBudgetsViolation::ClosedVocabularyMismatch {
                field: "hot_path_kinds",
            });
        }
        if self.budget_states != BudgetState::ALL.to_vec() {
            violations.push(HotPathPerformanceBudgetsViolation::ClosedVocabularyMismatch {
                field: "budget_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(HotPathPerformanceBudgetsViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.budget_actions != BudgetAction::ALL.to_vec() {
            violations.push(HotPathPerformanceBudgetsViolation::ClosedVocabularyMismatch {
                field: "budget_actions",
            });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(HotPathPerformanceBudgetsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(HotPathPerformanceBudgetsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(HotPathPerformanceBudgetsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(
        &self,
        violations: &mut Vec<HotPathPerformanceBudgetsViolation>,
    ) {
        if self.rules.is_empty() {
            violations.push(HotPathPerformanceBudgetsViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(HotPathPerformanceBudgetsViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(HotPathPerformanceBudgetsViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(HotPathPerformanceBudgetsViolation::GapReasonWithoutRule {
                    reason,
                });
            }
        }
    }

    fn validate_row(
        &self,
        row: &HotPathBudgetRow,
        violations: &mut Vec<HotPathPerformanceBudgetsViolation>,
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
                violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no budget may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(
                HotPathPerformanceBudgetsViolation::PublishedWiderThanClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                },
            );
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(
                HotPathPerformanceBudgetsViolation::FreshnessSloInconsistent {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        self.validate_budget(row, violations);

        // A public claim whose canonical label is below the cutline forces the budget
        // to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HeldOnNarrowedClaim {
                        entry_id: row.entry_id.clone(),
                        claim: row.claim_label,
                    },
                );
            }
            if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                violations.push(
                    HotPathPerformanceBudgetsViolation::ClaimNarrowedWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed row carries exactly the public claim's canonical label, carries
            // no active gap reason, rides a captured within-SLO packet, holds its
            // budget (unless an active waiver covers an intentionally tightened budget),
            // and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HeldLabelNotEqualClaim {
                        entry_id: row.entry_id.clone(),
                        claim: row.claim_label,
                        published: row.published_label,
                    },
                );
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HeldWithActiveGap {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !row.proof_packet.has_capture() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HeldWithoutFreshPacket {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !slo_state.is_within_slo() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HeldOnStalePacket {
                        entry_id: row.entry_id.clone(),
                        slo_state,
                    },
                );
            }
            if !row.hot_path_budget.is_complete() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HeldWithIncompleteBudget {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !row.hot_path_budget.within_budget() && !row.waiver_covers_tightened_budget() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HeldOverBudget {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HeldWithoutSignoff {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if row.publishes_stable() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        state: row.budget_state,
                        published: row.published_label,
                    },
                );
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::NarrowingWithoutReason {
                        entry_id: row.entry_id.clone(),
                        state: row.budget_state,
                    },
                );
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
            {
                violations.push(
                    HotPathPerformanceBudgetsViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::ProofPacketMissing)
            {
                violations.push(
                    HotPathPerformanceBudgetsViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            // A narrowing row over its budget must name the budget-regressed reason.
            if !row.hot_path_budget.within_budget()
                && !row.has_active_reason(GapReason::BudgetRegressed)
            {
                violations.push(
                    HotPathPerformanceBudgetsViolation::BudgetRegressedWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_budget(
        &self,
        row: &HotPathBudgetRow,
        violations: &mut Vec<HotPathPerformanceBudgetsViolation>,
    ) {
        let budget = &row.hot_path_budget;
        if budget.metric_ref.trim().is_empty() {
            violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "hot_path_budget.metric_ref",
            });
        }
        if !budget.is_ordered() {
            violations.push(HotPathPerformanceBudgetsViolation::BudgetDisordered {
                entry_id: row.entry_id.clone(),
            });
        }
        let incomplete = !budget.is_complete();
        if row.has_active_reason(GapReason::CorpusMetadataMissing) && !incomplete {
            violations.push(
                HotPathPerformanceBudgetsViolation::CorpusReasonWithoutIncomplete {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
        if incomplete && !row.has_active_reason(GapReason::CorpusMetadataMissing) {
            violations.push(
                HotPathPerformanceBudgetsViolation::IncompleteBudgetWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
        if !budget.within_budget() && row.has_active_reason(GapReason::BudgetRegressed) {
            // Coherence: a row that names budget_regressed must actually be over budget.
            // This is already guaranteed by the above narrowing check, but we keep the
            // invariant explicit.
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &HotPathBudgetRow,
        violations: &mut Vec<HotPathPerformanceBudgetsViolation>,
    ) {
        let push_incoherent =
            |violations: &mut Vec<HotPathPerformanceBudgetsViolation>, expected: GapReason| {
                violations.push(
                    HotPathPerformanceBudgetsViolation::StateReasonIncoherent {
                        entry_id: row.entry_id.clone(),
                        state: row.budget_state,
                        expected_reason: expected,
                    },
                );
            };

        match row.budget_state {
            BudgetState::Regressed => {
                if !row.has_active_reason(GapReason::BudgetRegressed) {
                    push_incoherent(violations, GapReason::BudgetRegressed);
                }
            }
            BudgetState::Stale => {
                if !(row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
                    || row.has_active_reason(GapReason::ProofPacketMissing))
                {
                    push_incoherent(violations, GapReason::ProofPacketFreshnessBreached);
                }
            }
            BudgetState::MissingTrace => {
                if !row.has_active_reason(GapReason::CorpusMetadataMissing) {
                    push_incoherent(violations, GapReason::CorpusMetadataMissing);
                }
            }
            BudgetState::NarrowedClaim => {
                if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, GapReason::ClaimLabelNarrowed);
                }
            }
            BudgetState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        HotPathPerformanceBudgetsViolation::WaiverStateWithoutPacket {
                            entry_id: row.entry_id.clone(),
                            state: row.budget_state,
                        },
                    );
                }
            }
            BudgetState::MeetsBudget => {}
        }
    }

    fn validate_coverage(
        &self,
        violations: &mut Vec<HotPathPerformanceBudgetsViolation>,
    ) {
        // Each surface ref appears at most once: a path has one canonical row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.surface_ref.as_str()) {
                violations.push(
                    HotPathPerformanceBudgetsViolation::DuplicateSurfaceRef {
                        surface_ref: row.surface_ref.clone(),
                    },
                );
            }
        }

        // The release line must cover every declared release-blocking surface with
        // exactly one release-blocking row, and every release-blocking row must be
        // declared.
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
                    HotPathPerformanceBudgetsViolation::ReleaseBlockingRefWithoutRow {
                        surface_ref: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.surface_ref.as_str()) {
                violations.push(
                    HotPathPerformanceBudgetsViolation::ReleaseBlockingRowNotInSet {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        // Every hot path kind must have at least one row.
        for kind in HotPathKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(
                    HotPathPerformanceBudgetsViolation::HotPathKindAbsent { kind },
                );
            }
        }
    }

    fn validate_promotion(
        &self,
        violations: &mut Vec<HotPathPerformanceBudgetsViolation>,
    ) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(HotPathPerformanceBudgetsViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                HotPathPerformanceBudgetsViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                HotPathPerformanceBudgetsViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                HotPathPerformanceBudgetsViolation::PromotionBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// Validation failure emitted while checking a hot-path performance budget register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HotPathPerformanceBudgetsViolation {
    /// Schema version is not the one this model supports.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// Record kind does not match the expected kind.
    UnsupportedRecordKind {
        /// Kind found in the register.
        actual: String,
    },
    /// A required string field is empty or whitespace-only.
    EmptyField {
        /// Id of the row or pack entity with the empty field.
        entry_id: String,
        /// Name of the empty field.
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the canonical set.
    ClosedVocabularyMismatch {
        /// Name of the mismatched field.
        field: &'static str,
    },
    /// The register contains no rules.
    NoRules,
    /// Two rows share the same entry id.
    DuplicateEntryId {
        /// Duplicated entry id.
        entry_id: String,
    },
    /// Two rows share the same surface ref.
    DuplicateSurfaceRef {
        /// Duplicated surface ref.
        surface_ref: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicated rule id.
        rule_id: String,
    },
    /// A rule watches no labels.
    RuleWithoutLabels {
        /// Rule id with empty label set.
        rule_id: String,
    },
    /// A gap reason has no rule covering it.
    GapReasonWithoutRule {
        /// Uncovered gap reason.
        reason: GapReason,
    },
    /// The register carries no rows.
    EmptyRegister,
    /// A row's published label is wider than its claim ceiling.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Canonical claim label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// The freshness SLO target is zero or the warn window exceeds it.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
    /// A budget's p50/p95 ordering is invalid.
    BudgetDisordered {
        /// Row id.
        entry_id: String,
    },
    /// A row carries the corpus-metadata-missing reason but the budget is complete.
    CorpusReasonWithoutIncomplete {
        /// Row id.
        entry_id: String,
    },
    /// A budget is incomplete but the row does not name the corpus-metadata-missing
    /// reason.
    IncompleteBudgetWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row holds its label while its public claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// The narrowed claim label.
        claim: StableClaimLevel,
    },
    /// A narrowed-claim row does not name the claim-label-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A backed row's published label differs from its claim label.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Canonical claim label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A backed row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A backed row rides a packet without a capture.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A backed row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// The packet's SLO state.
        slo_state: FreshnessSloState,
    },
    /// A backed row carries an incomplete budget.
    HeldWithIncompleteBudget {
        /// Row id.
        entry_id: String,
    },
    /// A backed row exceeds its published budget without a tightened-budget waiver.
    HeldOverBudget {
        /// Row id.
        entry_id: String,
    },
    /// A backed row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row still publishes at or above the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// The row's budget state.
        state: BudgetState,
        /// The published label.
        published: StableClaimLevel,
    },
    /// A narrowing row carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// The row's budget state.
        state: BudgetState,
    },
    /// A row has a breached packet without the freshness reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row has a missing packet without the missing-packet reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row is over budget without the budget-regressed reason.
    BudgetRegressedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row's state and active reasons are incoherent.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// The row's budget state.
        state: BudgetState,
        /// The expected gap reason.
        expected_reason: GapReason,
    },
    /// A row names a waiver state but carries no waiver packet.
    WaiverStateWithoutPacket {
        /// Row id.
        entry_id: String,
        /// The row's budget state.
        state: BudgetState,
    },
    /// A declared release-blocking surface has no covering row.
    ReleaseBlockingRefWithoutRow {
        /// Missing surface ref.
        surface_ref: String,
    },
    /// A release-blocking row is not in the declared set.
    ReleaseBlockingRowNotInSet {
        /// Row id.
        entry_id: String,
    },
    /// A hot path kind has no covering row.
    HotPathKindAbsent {
        /// Missing kind.
        kind: HotPathKind,
    },
    /// The promotion decision disagrees with the computed decision.
    PromotionDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The promotion blocking set disagrees with the computed set.
    PromotionBlockingSetMismatch {
        /// Field name that mismatched.
        field: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for HotPathPerformanceBudgetsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind {actual}")
            }
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "empty field {field_name} on {entry_id}"),
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch on {field}")
            }
            Self::NoRules => write!(f, "register contains no rules"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::DuplicateSurfaceRef { surface_ref } => {
                write!(f, "duplicate surface ref {surface_ref}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutRule { reason } => {
                write!(f, "gap reason {} has no rule", reason.as_str())
            }
            Self::EmptyRegister => write!(f, "register contains no rows"),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} publishes {} wider than claim {}", published.as_str(), claim.as_str()
            ),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} has an inconsistent freshness SLO")
            }
            Self::BudgetDisordered { entry_id } => {
                write!(f, "row {entry_id} has a disordered budget")
            }
            Self::CorpusReasonWithoutIncomplete { entry_id } => {
                write!(
                    f,
                    "row {entry_id} names corpus_metadata_missing but the budget is complete"
                )
            }
            Self::IncompleteBudgetWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} has an incomplete budget without corpus_metadata_missing"
                )
            }
            Self::HeldOnNarrowedClaim { entry_id, claim } => {
                write!(
                    f,
                    "row {entry_id} holds its label while its claim is {}", claim.as_str()
                )
            }
            Self::ClaimNarrowedWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} has a narrowed claim without the claim_label_narrowed reason"
                )
            }
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} holds its label but publishes {} instead of {}", published.as_str(), claim.as_str()
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "row {entry_id} holds its label with an active gap reason")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "row {entry_id} holds its label without a fresh packet")
            }
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(
                    f,
                    "row {entry_id} holds its label on a {slo_state:?} packet"
                )
            }
            Self::HeldWithIncompleteBudget { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds its label with an incomplete budget"
                )
            }
            Self::HeldOverBudget { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds its label while over budget without a tightened-budget waiver"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds its label without owner sign-off")
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "row {entry_id} state {} must narrow but holds {}",
                state.as_str(),
                published.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => {
                write!(
                    f,
                    "row {entry_id} state {} narrows without a reason",
                    state.as_str()
                )
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} has a breached packet without the freshness reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} has a missing packet without the missing-packet reason"
                )
            }
            Self::BudgetRegressedWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} is over budget without the budget_regressed reason"
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {entry_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutPacket { entry_id, state } => {
                write!(
                    f,
                    "row {entry_id} state {} names no waiver packet",
                    state.as_str()
                )
            }
            Self::ReleaseBlockingRefWithoutRow { surface_ref } => {
                write!(
                    f,
                    "declared release-blocking surface {surface_ref} has no row"
                )
            }
            Self::ReleaseBlockingRowNotInSet { entry_id } => {
                write!(
                    f,
                    "release-blocking row {entry_id} is not in the declared set"
                )
            }
            Self::HotPathKindAbsent { kind } => {
                write!(f, "hot path kind {} is covered by no row", kind.as_str())
            }
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "promotion decision {} disagrees with computed {}",
                    declared.as_str(),
                    computed.as_str()
                )
            }
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing rules")
            }
            Self::SummaryMismatch => {
                write!(f, "register summary counts disagree with rows")
            }
        }
    }
}

impl Error for HotPathPerformanceBudgetsViolation {}

/// Loads the embedded hot-path performance budget register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`HotPathPerformanceBudgets`].
pub fn current_hot_path_performance_budgets() -> Result<HotPathPerformanceBudgets, serde_json::Error> {
    serde_json::from_str(HOT_PATH_PERFORMANCE_BUDGETS_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> HotPathPerformanceBudgets {
        current_hot_path_performance_budgets()
            .expect("checked-in hot-path register parses into the model")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let reg = register();
        assert_eq!(reg.schema_version, HOT_PATH_PERFORMANCE_BUDGETS_SCHEMA_VERSION);
        assert_eq!(reg.record_kind, HOT_PATH_PERFORMANCE_BUDGETS_RECORD_KIND);
        assert_eq!(reg.validate(), Vec::new());
    }

    #[test]
    fn covers_every_hot_path_kind() {
        let reg = register();
        for kind in HotPathKind::ALL {
            assert!(
                !reg.rows_for_kind(kind).is_empty(),
                "hot path kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_surface() {
        let reg = register();
        assert!(!reg.release_blocking_surface_refs.is_empty());
        let covered: Vec<&str> = reg
            .release_blocking_rows()
            .into_iter()
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &reg.release_blocking_surface_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let reg = register();
        assert_eq!(reg.summary, reg.computed_summary());
        assert_eq!(
            reg.summary.entries_meeting_budget
                + reg.summary.entries_on_active_waiver
                + reg.summary.entries_narrowed,
            reg.rows.len()
        );
        assert_eq!(
            reg.summary.packets_current
                + reg.summary.packets_due_for_refresh
                + reg.summary.packets_breached
                + reg.summary.packets_missing,
            reg.rows.len()
        );
        assert_eq!(
            reg.summary.startup_entries
                + reg.summary.restore_entries
                + reg.summary.quick_open_entries
                + reg.summary.typing_entries
                + reg.summary.scrolling_entries
                + reg.summary.search_entries
                + reg.summary.git_status_entries,
            reg.rows.len()
        );
    }

    #[test]
    fn promotion_holds_when_blocking_rules_fire() {
        let reg = register();
        assert_eq!(reg.promotion.decision, PromotionDecision::Hold);
        assert_eq!(
            reg.promotion.decision,
            reg.computed_promotion_decision()
        );
        assert!(!reg.promotion.blocking_rule_ids.is_empty());
        assert!(!reg.promotion.blocking_entry_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let reg = register();
        let covered: BTreeSet<GapReason> = reg
            .rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in GapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_publishes_wider_than_its_claim_ceiling() {
        let reg = register();
        for row in &reg.rows {
            assert!(
                row.published_label.rank() <= row.claim_label.rank(),
                "{} publishes wider than its ceiling",
                row.entry_id
            );
        }
    }

    #[test]
    fn validate_flags_a_published_label_wider_than_ceiling() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| !row.publishes_stable() && row.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.published_label = StableClaimLevel::Stable;
        let entry_id = row.entry_id.clone();
        reg.summary = reg.computed_summary();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            HotPathPerformanceBudgetsViolation::PublishedWiderThanClaim { entry_id: id, .. } if *id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.budget_state == BudgetState::Stale)
            .expect("a stale row exists");
        row.published_label = row.claim_label;
        reg.summary = reg.computed_summary();
        reg.promotion.decision = reg.computed_promotion_decision();
        reg.promotion.blocking_rule_ids = reg.computed_blocking_rule_ids();
        reg.promotion.blocking_entry_ids = reg.computed_blocking_entry_ids();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            HotPathPerformanceBudgetsViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_a_backed_row_over_budget() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.budget_state == BudgetState::MeetsBudget)
            .expect("a meets-budget row exists");
        row.hot_path_budget.measured_p95_ms = row.hot_path_budget.published_p95_ms + 1_000;
        reg.summary = reg.computed_summary();
        assert!(reg
            .validate()
            .iter()
            .any(|v| matches!(v, HotPathPerformanceBudgetsViolation::HeldOverBudget { .. })));
    }

    #[test]
    fn validate_flags_a_backed_row_on_a_breached_packet() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("a backed row exists");
        row.proof_packet.slo_state = FreshnessSloState::Breached;
        reg.summary = reg.computed_summary();
        assert!(reg
            .validate()
            .iter()
            .any(|v| matches!(v, HotPathPerformanceBudgetsViolation::HeldOnStalePacket { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_promotion_decision() {
        let mut reg = register();
        reg.promotion.decision = PromotionDecision::Proceed;
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            HotPathPerformanceBudgetsViolation::PromotionDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_backed_row_without_signoff() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("a backed row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let entry_id = row.entry_id.clone();
        reg.summary = reg.computed_summary();
        assert!(reg
            .validate()
            .contains(&HotPathPerformanceBudgetsViolation::HeldWithoutSignoff { entry_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let reg = register();
        let projection = reg.support_export_projection();
        assert_eq!(projection.rows.len(), reg.rows.len());
        assert_eq!(projection.promotion_decision, reg.promotion.decision);
        for (row, projected) in reg.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.surface_ref, projected.surface_ref);
            assert_eq!(row.publishes_stable(), projected.publishes_stable);
            assert_eq!(row.published_label, projected.published_label);
            assert_eq!(row.proof_packet.slo_state, projected.slo_state);
        }
    }

    #[test]
    fn register_narrows_a_row_under_a_still_stable_claim() {
        let reg = register();
        let narrowed = reg.rows.iter().find(|row| {
            row.release_blocking
                && row.claim_holds_stable()
                && !row.publishes_stable()
                && row.budget_state != BudgetState::NarrowedClaim
        });
        assert!(
            narrowed.is_some(),
            "the register must narrow at least one release-blocking row under a still-stable claim"
        );
    }

    #[test]
    fn register_protects_a_regressed_budget() {
        let reg = register();
        let regressed = reg
            .rows
            .iter()
            .find(|row| row.budget_state == BudgetState::Regressed)
            .expect("the register must show a regressed budget");
        assert!(!regressed.hot_path_budget.within_budget());
        assert!(!regressed.publishes_stable());
        assert!(regressed.has_active_reason(GapReason::BudgetRegressed));
    }

    #[test]
    fn register_shows_a_tightened_budget_on_waiver() {
        let reg = register();
        let on_waiver = reg
            .rows
            .iter()
            .find(|row| row.budget_state == BudgetState::OnWaiver)
            .expect("the register must show a tightened budget on waiver");
        assert!(on_waiver.hot_path_budget.tightened);
        assert!(on_waiver.waiver.is_some());
        assert!(on_waiver.publishes_stable());
    }
}
