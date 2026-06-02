//! Typed benchmark-lab automation, corpus governance, and public benchmark
//! publication pack register for the M4 stable line.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_publication_pack`](crate::stable_publication_pack) governs the
//! outward-facing benchmark publications the release line ships; the
//! [`stabilize_hot_path_performance_against_published_budgets_for`](crate::stabilize_hot_path_performance_against_published_budgets_for)
//! register protects the published p50/p95 numbers for each hot path. None of them
//! answer the question this module answers: **for each benchmark-lab automation lane,
//! corpus governance asset, and public benchmark publication pack — is there a fresh,
//! complete, owner-signed qualification packet behind it, grounded in benchmark-lab
//! traces and corpus metadata where it makes a performance claim, and is the asset
//! narrowed below the cutline the moment its backing thins out?** This module is the
//! **benchmark-lab governance register**. For every asset it records one row that binds
//! the asset to the [`stable_claim_manifest`](crate::stable_claim_manifest) entry
//! whose lifecycle label it backs, the proof packet that grounds it, the waiver (if any)
//! holding a tightened threshold or incomplete evidence provisionally, and the owner
//! sign-off.
//!
//! Each [`GovernanceAssetRow`] is one `(governance asset, public claim)` binding. It:
//!
//! - names the asset it governs ([`GovernanceAssetRow::asset_kind`],
//!   [`GovernanceAssetRow::surface_ref`], [`GovernanceAssetRow::surface_summary`]) and
//!   whether that asset is part of the release-blocking set
//!   ([`GovernanceAssetRow::release_blocking`]);
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO and, for a
//!   benchmark publication pack, records whether the threshold is intentionally tightened;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose
//!   public claim it backs ([`GovernanceAssetRow::claim_ref`]) and the canonical
//!   lifecycle label that entry publishes ([`GovernanceAssetRow::claim_label`]). That
//!   label is a hard **ceiling**: an asset may carry the claim's label or narrow below
//!   it, but it may never assert a public claim wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-asset labels,
//!   so docs, Help/About, the release center, and support exports ingest one label per
//!   asset instead of cloning their own;
//! - records the asset state earned ([`AssetState`]), the active gap reasons
//!   ([`GapReason`]), and the label it *effectively* publishes after narrowing
//!   ([`GovernanceAssetRow::published_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between an asset whose backing supports a Stable public claim and one narrowed below
//! it. An asset that is not backed — because its proof packet aged out or is missing,
//! because its corpus metadata or benchmark-lab trace is missing, because its evidence
//! is incomplete, because its waiver expired, because its owner sign-off is absent, or
//! because the public claim it backs is itself below the cutline — is structurally
//! required to drop below the cutline rather than inherit an adjacent backed asset.
//! The [`GovernanceRule`] set names the closed conditions that gate qualification, and
//! [`BenchmarkLabGovernance::qualification`] records the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row without
//! a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It carries
//! no raw artifacts, raw logs, signatures, or credential material. Two classes of check
//! live outside this model because they need more than the register sees: date arithmetic
//! (recomputing the packet-freshness state and waiver expiry against an `as_of` date) and
//! the cross-artifact ceiling check (whether each row's `claim_label` still equals the
//! label the stable claim manifest publishes for the entry named by `claim_ref`). Those
//! live in the CI gate. This model enforces the structural and logical invariants that
//! hold regardless of the clock and the neighbouring artifact — the ceiling/no-widening
//! rule, packet/state coherence, owner sign-off on backed rows, asset-kind and
//! release-line coverage, rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported benchmark-lab-governance schema version.
pub const BENCHMARK_LAB_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const BENCHMARK_LAB_GOVERNANCE_RECORD_KIND: &str = "benchmark_lab_governance";

/// Repo-relative path to the checked-in register.
pub const BENCHMARK_LAB_GOVERNANCE_PATH: &str =
    "artifacts/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack.json";

/// Embedded checked-in register JSON.
pub const BENCHMARK_LAB_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack.json"
));

/// The class of governance asset a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAssetKind {
    /// The nightly benchmark CI automation lane.
    NightlyBenchmarkCiLane,
    /// The self-capture parity verification check.
    SelfCaptureParityCheck,
    /// The microbenchmark corpus governance asset.
    MicrobenchmarkCorpus,
    /// The workflow / archetype corpus governance asset.
    WorkflowArchetypeCorpus,
    /// The remote / collaboration corpus governance asset.
    RemoteCollaborationCorpus,
    /// The accessibility corpus governance asset.
    AccessibilityCorpus,
    /// The protected metrics file governance asset.
    ProtectedMetricsFile,
    /// The reference hardware manifest governance asset.
    ReferenceHardwareManifest,
    /// The lab image / environment manifest governance asset.
    LabImageManifest,
    /// The protected path / budget / evidence ledger governance asset.
    ProtectedPathLedger,
    /// The public benchmark publication pack governance asset.
    PublicBenchmarkPublicationPack,
}

impl GovernanceAssetKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::NightlyBenchmarkCiLane,
        Self::SelfCaptureParityCheck,
        Self::MicrobenchmarkCorpus,
        Self::WorkflowArchetypeCorpus,
        Self::RemoteCollaborationCorpus,
        Self::AccessibilityCorpus,
        Self::ProtectedMetricsFile,
        Self::ReferenceHardwareManifest,
        Self::LabImageManifest,
        Self::ProtectedPathLedger,
        Self::PublicBenchmarkPublicationPack,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NightlyBenchmarkCiLane => "nightly_benchmark_ci_lane",
            Self::SelfCaptureParityCheck => "self_capture_parity_check",
            Self::MicrobenchmarkCorpus => "microbenchmark_corpus",
            Self::WorkflowArchetypeCorpus => "workflow_archetype_corpus",
            Self::RemoteCollaborationCorpus => "remote_collaboration_corpus",
            Self::AccessibilityCorpus => "accessibility_corpus",
            Self::ProtectedMetricsFile => "protected_metrics_file",
            Self::ReferenceHardwareManifest => "reference_hardware_manifest",
            Self::LabImageManifest => "lab_image_manifest",
            Self::ProtectedPathLedger => "protected_path_ledger",
            Self::PublicBenchmarkPublicationPack => "public_benchmark_publication_pack",
        }
    }
}

/// Asset state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetState {
    /// The asset is qualified: a captured, within-SLO proof packet, complete evidence,
    /// and an owner sign-off back the public claim at its full canonical lifecycle label.
    QualifiedStable,
    /// The asset carries the claim's full label only because an active, unexpired waiver
    /// covers a recorded gap.
    QualifiedOnWaiver,
    /// The proof packet or row evidence is incomplete, a required corpus metadata or lab
    /// trace is missing, or a surface capability is absent; the asset is not backed and
    /// the label must narrow.
    NarrowedUnbacked,
    /// The public claim this asset backs is itself below the cutline, so the asset
    /// inherits that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The proof packet breached its freshness SLO (or is missing); the asset is not
    /// backed and the label must narrow.
    NarrowedStale,
    /// The asset relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
    /// The benchmark corpus metadata or lab trace is missing; the label must narrow.
    NarrowedMissingTrace,
}

impl AssetState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::QualifiedStable,
        Self::QualifiedOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
        Self::NarrowedMissingTrace,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualifiedStable => "qualified_stable",
            Self::QualifiedOnWaiver => "qualified_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::NarrowedMissingTrace => "narrowed_missing_trace",
        }
    }

    /// Whether the state lets an asset carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::QualifiedStable | Self::QualifiedOnWaiver)
    }

    /// Whether the state forces the asset below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason an asset narrows or a governance rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this asset backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The asset names a surface capability the build does not yet implement.
    SurfaceCapabilityAbsent,
    /// The proof packet's row-level evidence is incomplete.
    EvidenceIncomplete,
    /// A benchmark-related asset is missing its corpus metadata or its benchmark-lab trace.
    CorpusMetadataMissing,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// No proof packet has been captured for the asset.
    ProofPacketMissing,
    /// A waiver the asset relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ClaimLabelNarrowed,
        Self::SurfaceCapabilityAbsent,
        Self::EvidenceIncomplete,
        Self::CorpusMetadataMissing,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::SurfaceCapabilityAbsent => "surface_capability_absent",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::CorpusMetadataMissing => "corpus_metadata_missing",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a governance rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetAction {
    /// Hold qualification until the condition clears.
    HoldQualification,
    /// Narrow the asset's published lifecycle label below the cutline.
    NarrowLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Recapture the row-level evidence the proof packet depends on.
    RecaptureEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl AssetAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldQualification,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RecaptureEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldQualification => "hold_qualification",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One governance rule: a closed condition that narrows an asset label and may gate
/// qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: AssetAction,
    /// Whether firing this rule blocks qualification.
    pub blocks_qualification: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One governance asset row: a `(governance asset, public claim)` binding bound to its
/// proof packet, canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceAssetRow {
    /// Stable asset-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The class of governance asset this row governs.
    pub asset_kind: GovernanceAssetKind,
    /// The surface id this asset speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the asset.
    pub surface_summary: String,
    /// Whether the asset is part of the release-blocking governance set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this asset backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: an asset
    /// may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Asset state earned for the row.
    pub asset_state: AssetState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional qualification, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the asset effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl GovernanceAssetRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the asset carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.asset_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// The recorded qualification verdict for the benchmark-lab governance register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationRecord {
    /// The gate this verdict governs.
    pub qualification_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Governance-rule ids that block qualification, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Asset-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkLabGovernanceSummary {
    /// Total number of asset rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_qualified_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed_below_cutline: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_qualified: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Nightly CI lane rows.
    pub nightly_ci_lane_entries: usize,
    /// Self-capture parity check rows.
    pub self_capture_parity_entries: usize,
    /// Microbenchmark corpus rows.
    pub microbenchmark_corpus_entries: usize,
    /// Workflow / archetype corpus rows.
    pub workflow_archetype_corpus_entries: usize,
    /// Remote / collaboration corpus rows.
    pub remote_collaboration_corpus_entries: usize,
    /// Accessibility corpus rows.
    pub accessibility_corpus_entries: usize,
    /// Protected metrics file rows.
    pub protected_metrics_file_entries: usize,
    /// Reference hardware manifest rows.
    pub reference_hardware_manifest_entries: usize,
    /// Lab image manifest rows.
    pub lab_image_manifest_entries: usize,
    /// Protected path ledger rows.
    pub protected_path_ledger_entries: usize,
    /// Public benchmark publication pack rows.
    pub public_benchmark_publication_pack_entries: usize,
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
    /// Number of governance rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkLabGovernanceExportRow {
    /// Stable asset-row id.
    pub entry_id: String,
    /// The class of governance asset this row governs.
    pub asset_kind: GovernanceAssetKind,
    /// The surface id this asset speaks about.
    pub surface_ref: String,
    /// Whether the asset is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this asset backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Asset state earned.
    pub asset_state: AssetState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkLabGovernanceExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Qualification decision.
    pub qualification_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<BenchmarkLabGovernanceExportRow>,
}

/// The typed benchmark-lab governance register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkLabGovernance {
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
    /// Ref to the corpus governance document every corpus row rides.
    pub corpus_governance_ref: String,
    /// Ref to the benchmark publication pack template every publication pack row rides.
    pub benchmark_pack_template_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed governance-asset-kind vocabulary.
    pub asset_kinds: Vec<GovernanceAssetKind>,
    /// Closed asset-state vocabulary.
    pub asset_states: Vec<AssetState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed asset-action vocabulary.
    pub asset_actions: Vec<AssetAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Governance rules.
    pub rules: Vec<GovernanceRule>,
    /// Asset rows.
    pub rows: Vec<GovernanceAssetRow>,
    /// Recorded qualification verdict.
    pub qualification: QualificationRecord,
    /// Summary counts.
    pub summary: BenchmarkLabGovernanceSummary,
}

impl BenchmarkLabGovernance {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&GovernanceAssetRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&GovernanceAssetRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&GovernanceAssetRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&GovernanceAssetRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one asset kind.
    pub fn rows_for_kind(&self, kind: GovernanceAssetKind) -> Vec<&GovernanceAssetRow> {
        self.rows
            .iter()
            .filter(|row| row.asset_kind == kind)
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
    pub fn rule_fires(&self, rule: &GovernanceRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the qualification verdict from the rows and governance rules.
    pub fn computed_qualification_decision(&self) -> PromotionDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_qualification && self.rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block qualification and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_qualification && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Asset-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim
    /// is already canonically narrowed is not a *qualification* blocker, it merely inherits
    /// the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_qualification && self.rule_fires(rule))
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

    /// Recomputes the summary block from the rows and governance rules.
    pub fn computed_summary(&self) -> BenchmarkLabGovernanceSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: GovernanceAssetKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&GovernanceAssetRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        BenchmarkLabGovernanceSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_qualified_stable: self
                .rows
                .iter()
                .filter(|row| row.asset_state == AssetState::QualifiedStable)
                .count(),
            entries_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.asset_state == AssetState::QualifiedOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_qualified: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            nightly_ci_lane_entries: kind(GovernanceAssetKind::NightlyBenchmarkCiLane),
            self_capture_parity_entries: kind(GovernanceAssetKind::SelfCaptureParityCheck),
            microbenchmark_corpus_entries: kind(GovernanceAssetKind::MicrobenchmarkCorpus),
            workflow_archetype_corpus_entries: kind(GovernanceAssetKind::WorkflowArchetypeCorpus),
            remote_collaboration_corpus_entries: kind(GovernanceAssetKind::RemoteCollaborationCorpus),
            accessibility_corpus_entries: kind(GovernanceAssetKind::AccessibilityCorpus),
            protected_metrics_file_entries: kind(GovernanceAssetKind::ProtectedMetricsFile),
            reference_hardware_manifest_entries: kind(GovernanceAssetKind::ReferenceHardwareManifest),
            lab_image_manifest_entries: kind(GovernanceAssetKind::LabImageManifest),
            protected_path_ledger_entries: kind(GovernanceAssetKind::ProtectedPathLedger),
            public_benchmark_publication_pack_entries: kind(GovernanceAssetKind::PublicBenchmarkPublicationPack),
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
    pub fn support_export_projection(&self) -> BenchmarkLabGovernanceExportProjection {
        BenchmarkLabGovernanceExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            qualification_decision: self.qualification.decision,
            rows: self
                .rows
                .iter()
                .map(|row| BenchmarkLabGovernanceExportRow {
                    entry_id: row.entry_id.clone(),
                    asset_kind: row.asset_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    asset_state: row.asset_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<BenchmarkLabGovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(BenchmarkLabGovernanceViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(BenchmarkLabGovernanceViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_qualification(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(BenchmarkLabGovernanceViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<BenchmarkLabGovernanceViolation>) {
        if self.schema_version != BENCHMARK_LAB_GOVERNANCE_SCHEMA_VERSION {
            violations.push(BenchmarkLabGovernanceViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != BENCHMARK_LAB_GOVERNANCE_RECORD_KIND {
            violations.push(BenchmarkLabGovernanceViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("corpus_governance_ref", &self.corpus_governance_ref),
            ("benchmark_pack_template_ref", &self.benchmark_pack_template_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(BenchmarkLabGovernanceViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(BenchmarkLabGovernanceViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.asset_kinds != GovernanceAssetKind::ALL.to_vec() {
            violations.push(BenchmarkLabGovernanceViolation::ClosedVocabularyMismatch {
                field: "asset_kinds",
            });
        }
        if self.asset_states != AssetState::ALL.to_vec() {
            violations.push(BenchmarkLabGovernanceViolation::ClosedVocabularyMismatch {
                field: "asset_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(BenchmarkLabGovernanceViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.asset_actions != AssetAction::ALL.to_vec() {
            violations.push(BenchmarkLabGovernanceViolation::ClosedVocabularyMismatch {
                field: "asset_actions",
            });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(BenchmarkLabGovernanceViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(BenchmarkLabGovernanceViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(BenchmarkLabGovernanceViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(BenchmarkLabGovernanceViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(BenchmarkLabGovernanceViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<BenchmarkLabGovernanceViolation>) {
        if self.rules.is_empty() {
            violations.push(BenchmarkLabGovernanceViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(BenchmarkLabGovernanceViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(BenchmarkLabGovernanceViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(BenchmarkLabGovernanceViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(BenchmarkLabGovernanceViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &GovernanceAssetRow,
        violations: &mut Vec<BenchmarkLabGovernanceViolation>,
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
                violations.push(BenchmarkLabGovernanceViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no asset may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(BenchmarkLabGovernanceViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn window
        // may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(BenchmarkLabGovernanceViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(BenchmarkLabGovernanceViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A public claim whose canonical label is below the cutline forces the row to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(BenchmarkLabGovernanceViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                violations.push(
                    BenchmarkLabGovernanceViolation::ClaimNarrowedWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        if row.holds_label() {
            // A backed row renders exactly the public claim's canonical label, carries
            // no active gap reason, rides a captured within-SLO packet, and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(BenchmarkLabGovernanceViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(BenchmarkLabGovernanceViolation::HeldWithActiveReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(BenchmarkLabGovernanceViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.slo_state.is_within_slo() {
                violations.push(BenchmarkLabGovernanceViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state: row.proof_packet.slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(BenchmarkLabGovernanceViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline and name
            // at least one active reason.
            if row.publishes_stable() {
                violations.push(BenchmarkLabGovernanceViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.asset_state,
                    published: row.published_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(BenchmarkLabGovernanceViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.asset_state,
                });
            }
            // A narrowing row whose packet is breached must name the matching freshness
            // reason, so the freshness automation stays honest.
            if row.proof_packet.slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
            {
                violations.push(
                    BenchmarkLabGovernanceViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            // A narrowing row whose packet is missing must name the matching reason.
            if row.proof_packet.slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::ProofPacketMissing)
            {
                violations.push(
                    BenchmarkLabGovernanceViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &GovernanceAssetRow,
        violations: &mut Vec<BenchmarkLabGovernanceViolation>,
    ) {
        // A row on waiver must have a waiver and the waiver must be active.
        if row.asset_state == AssetState::QualifiedOnWaiver && row.waiver.is_none() {
            violations.push(BenchmarkLabGovernanceViolation::OnWaiverWithoutWaiver {
                entry_id: row.entry_id.clone(),
            });
        }
        // A row whose state is narrowed_stale must have the freshness reason.
        if row.asset_state == AssetState::NarrowedStale
            && !row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
            && !row.has_active_reason(GapReason::ProofPacketMissing)
        {
            violations.push(BenchmarkLabGovernanceViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.asset_state,
                expected_reason: "proof_packet_freshness_breached or proof_packet_missing",
            });
        }
        // A row whose state is narrowed_missing_trace must have the corpus reason.
        if row.asset_state == AssetState::NarrowedMissingTrace
            && !row.has_active_reason(GapReason::CorpusMetadataMissing)
        {
            violations.push(BenchmarkLabGovernanceViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.asset_state,
                expected_reason: "corpus_metadata_missing",
            });
        }
        // A row whose state is narrowed_waiver_expired must have the waiver reason.
        if row.asset_state == AssetState::NarrowedWaiverExpired
            && !row.has_active_reason(GapReason::WaiverExpired)
        {
            violations.push(BenchmarkLabGovernanceViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.asset_state,
                expected_reason: "waiver_expired",
            });
        }
        // A row whose state is narrowed_claim_narrowed must have the claim reason.
        if row.asset_state == AssetState::NarrowedClaimNarrowed
            && !row.has_active_reason(GapReason::ClaimLabelNarrowed)
        {
            violations.push(BenchmarkLabGovernanceViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.asset_state,
                expected_reason: "claim_label_narrowed",
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<BenchmarkLabGovernanceViolation>) {
        // Every declared asset kind must have at least one row.
        for kind in GovernanceAssetKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(BenchmarkLabGovernanceViolation::AssetKindUncovered { kind });
            }
        }
        // Every declared release-blocking surface ref must have at least one row.
        for surface_ref in &self.release_blocking_surface_refs {
            if !self.rows.iter().any(|row| &row.surface_ref == surface_ref) {
                violations.push(BenchmarkLabGovernanceViolation::ReleaseBlockingSurfaceUncovered {
                    surface_ref: surface_ref.clone(),
                });
            }
        }
    }

    fn validate_qualification(
        &self,
        violations: &mut Vec<BenchmarkLabGovernanceViolation>,
    ) {
        if self.qualification.decision != self.computed_qualification_decision() {
            violations.push(BenchmarkLabGovernanceViolation::QualificationDecisionInconsistent {
                recorded: self.qualification.decision,
                computed: self.computed_qualification_decision(),
            });
        }
        let computed_blocking_ids = self.computed_blocking_rule_ids();
        if self.qualification.blocking_rule_ids != computed_blocking_ids {
            violations.push(BenchmarkLabGovernanceViolation::BlockingRuleIdsMismatch {
                recorded: self.qualification.blocking_rule_ids.clone(),
                computed: computed_blocking_ids,
            });
        }
        let computed_blocking_entries = self.computed_blocking_entry_ids();
        if self.qualification.blocking_entry_ids != computed_blocking_entries {
            violations.push(BenchmarkLabGovernanceViolation::BlockingEntryIdsMismatch {
                recorded: self.qualification.blocking_entry_ids.clone(),
                computed: computed_blocking_entries,
            });
        }
    }
}

/// Parses the embedded checked-in register JSON.
pub fn current_benchmark_lab_governance() -> Result<BenchmarkLabGovernance, serde_json::Error> {
    serde_json::from_str(BENCHMARK_LAB_GOVERNANCE_JSON)
}

/// Every violation the register validation can report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchmarkLabGovernanceViolation {
    UnsupportedSchemaVersion { actual: u32 },
    UnsupportedRecordKind { actual: String },
    EmptyField { entry_id: String, field_name: &'static str },
    ClosedVocabularyMismatch { field: &'static str },
    DuplicateEntryId { entry_id: String },
    DuplicateRuleId { rule_id: String },
    RuleWithoutLabels { rule_id: String },
    GapReasonWithoutRule { reason: GapReason },
    PublishedWiderThanClaim {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
    },
    FreshnessSloInconsistent { entry_id: String },
    HeldOnNarrowedClaim {
        entry_id: String,
        claim: StableClaimLevel,
    },
    ClaimNarrowedWithoutReason { entry_id: String },
    HeldLabelNotEqualClaim {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
    },
    HeldWithActiveReason { entry_id: String },
    HeldWithoutFreshPacket { entry_id: String },
    HeldOnStalePacket {
        entry_id: String,
        slo_state: FreshnessSloState,
    },
    HeldWithoutSignoff { entry_id: String },
    PublishedLabelNotNarrowed {
        entry_id: String,
        state: AssetState,
        published: StableClaimLevel,
    },
    NarrowingWithoutReason {
        entry_id: String,
        state: AssetState,
    },
    BreachedPacketWithoutReason { entry_id: String },
    MissingPacketWithoutReason { entry_id: String },
    OnWaiverWithoutWaiver { entry_id: String },
    StateReasonMismatch {
        entry_id: String,
        state: AssetState,
        expected_reason: &'static str,
    },
    AssetKindUncovered { kind: GovernanceAssetKind },
    ReleaseBlockingSurfaceUncovered { surface_ref: String },
    QualificationDecisionInconsistent {
        recorded: PromotionDecision,
        computed: PromotionDecision,
    },
    BlockingRuleIdsMismatch {
        recorded: Vec<String>,
        computed: Vec<String>,
    },
    BlockingEntryIdsMismatch {
        recorded: Vec<String>,
        computed: Vec<String>,
    },
    EmptyRegister,
    NoRules,
    SummaryMismatch,
}

impl fmt::Display for BenchmarkLabGovernanceViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version: {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind: {actual}")
            }
            Self::EmptyField {
                entry_id,
                field_name,
            } => {
                write!(f, "empty field '{field_name}' on {entry_id}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch: {field}")
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry_id: {entry_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule_id: {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule {rule_id} has no labels")
            }
            Self::GapReasonWithoutRule { reason } => {
                write!(f, "gap reason without rule: {}", reason.as_str())
            }
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => {
                write!(
                    f,
                    "published label {published:?} is wider than claim {claim:?} on {entry_id}"
                )
            }
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "freshness SLO inconsistent on {entry_id}")
            }
            Self::HeldOnNarrowedClaim { entry_id, claim } => {
                write!(
                    f,
                    "row {entry_id} holds its label but its claim is narrowed to {claim:?}"
                )
            }
            Self::ClaimNarrowedWithoutReason { entry_id } => {
                write!(f, "row {entry_id} has a narrowed claim but no claim_label_narrowed reason")
            }
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => {
                write!(
                    f,
                    "row {entry_id} holds label {published:?} but claim is {claim:?}"
                )
            }
            Self::HeldWithActiveReason { entry_id } => {
                write!(f, "row {entry_id} holds its label but has active gap reasons")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "row {entry_id} holds its label but its packet is not fresh")
            }
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(
                    f,
                    "row {entry_id} holds its label but its packet SLO state is {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds its label but lacks owner sign-off")
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => {
                write!(
                    f,
                    "row {entry_id} is in state {state:?} but published label {published:?} is not narrowed"
                )
            }
            Self::NarrowingWithoutReason { entry_id, state } => {
                write!(
                    f,
                    "row {entry_id} is in state {state:?} but has no active gap reason"
                )
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} has a breached packet but no proof_packet_freshness_breached reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} has a missing packet but no proof_packet_missing reason"
                )
            }
            Self::OnWaiverWithoutWaiver { entry_id } => {
                write!(f, "row {entry_id} is on waiver but has no waiver record")
            }
            Self::StateReasonMismatch {
                entry_id,
                state,
                expected_reason,
            } => {
                write!(
                    f,
                    "row {entry_id} is in state {state:?} but lacks expected reason {expected_reason}"
                )
            }
            Self::AssetKindUncovered { kind } => {
                write!(f, "asset kind {} has no covering row", kind.as_str())
            }
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering row"
                )
            }
            Self::QualificationDecisionInconsistent { recorded, computed } => {
                write!(
                    f,
                    "qualification decision {recorded:?} does not match computed {computed:?}"
                )
            }
            Self::BlockingRuleIdsMismatch { recorded, computed } => {
                write!(
                    f,
                    "blocking rule ids {recorded:?} do not match computed {computed:?}"
                )
            }
            Self::BlockingEntryIdsMismatch { recorded, computed } => {
                write!(
                    f,
                    "blocking entry ids {recorded:?} do not match computed {computed:?}"
                )
            }
            Self::EmptyRegister => {
                write!(f, "register has no rows")
            }
            Self::NoRules => {
                write!(f, "register has no rules")
            }
            Self::SummaryMismatch => {
                write!(f, "summary does not match computed summary")
            }
        }
    }
}

impl Error for BenchmarkLabGovernanceViolation {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn governance_asset_kind_as_str_round_trips() {
        for kind in GovernanceAssetKind::ALL {
            let s = kind.as_str();
            let parsed: GovernanceAssetKind = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(parsed, kind);
        }
    }

    #[test]
    fn asset_state_as_str_round_trips() {
        for state in AssetState::ALL {
            let s = state.as_str();
            let parsed: AssetState = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(parsed, state);
        }
    }

    #[test]
    fn gap_reason_as_str_round_trips() {
        for reason in GapReason::ALL {
            let s = reason.as_str();
            let parsed: GapReason = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(parsed, reason);
        }
    }

    #[test]
    fn asset_action_as_str_round_trips() {
        for action in AssetAction::ALL {
            let s = action.as_str();
            let parsed: AssetAction = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(parsed, action);
        }
    }

    #[test]
    fn closed_vocabularies_are_complete() {
        assert_eq!(GovernanceAssetKind::ALL.len(), 11);
        assert_eq!(AssetState::ALL.len(), 7);
        assert_eq!(GapReason::ALL.len(), 8);
        assert_eq!(AssetAction::ALL.len(), 5);
    }
}
