//! Automated clean-room rebuild, exact-build symbolication, release-center sync,
//! and advisory/revocation rehearsal truth for M5 depth trains.
//!
//! Where the clean-room rebuild *proof* records the static posture of a marketed
//! channel, this register is the *automation* layer on top of it: for every
//! claimed M5 artifact family it runs four standing rehearsals and records their
//! machine-readable result and expiry state so claim-narrowing, shiproom
//! dashboards, the evidence index, and support exports all read one source of
//! truth instead of tribal memory.
//!
//! Each [`M5RehearsalRow`] binds one [`M5ArtifactFamilyKind`] family to the public
//! claim it backs ([`M5RehearsalRow::claim_ref`], [`M5RehearsalRow::claim_label`])
//! and to exactly one [`RehearsalRecord`] per [`RehearsalKind`]:
//!
//! - [`RehearsalKind::CleanRoomRebuild`] — a from-clean-state rebuild proof. A
//!   warmed-cache-only run ([`RebuildProvenance::WarmCacheOnly`]) never counts as
//!   rebuild proof; it narrows the row via [`RehearsalGapReason::RebuildCacheOnly`].
//! - [`RehearsalKind::ExactBuildSymbolication`] — exact-build symbol/source-map
//!   verification. Its freshness may never run ahead of the release-center sync
//!   rehearsal; if it does, the row narrows via
//!   [`RehearsalGapReason::SymbolicationFreshnessDecoupled`].
//! - [`RehearsalKind::ReleaseCenterSync`] — release-center / mirror / offline
//!   parity check that also grounds the support and export surfaces.
//! - [`RehearsalKind::AdvisoryRevocationDrill`] — the advisory / emergency-disable
//!   / revocation rehearsal.
//!
//! Each rehearsal carries a [`ProofPacket`] whose [`FreshnessSloState`] is the
//! expiry signal. A red, stale, or missing rehearsal — or a tripped guardrail —
//! drops the row's [`RehearsalAutomationState`] below the [`LaunchCutline`], moves
//! its [`M5RehearsalRow::published_label`] beneath the claimed label, and, on a
//! claim at or above the cutline, fires a [`RehearsalStopRule`] that holds
//! promotion ([`M5RehearsalAutomationRegister::computed_promotion_decision`]).
//!
//! The register is checked in at
//! `artifacts/release/m5/ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains.json`
//! and embedded here so this typed consumer, the CI gate, and the headless
//! emitter all agree on every row without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::M5ArtifactFamilyKind;
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const SHIP_M5_REHEARSAL_AUTOMATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const SHIP_M5_REHEARSAL_AUTOMATION_RECORD_KIND: &str =
    "ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains";

/// Repo-relative path to the checked-in register.
pub const SHIP_M5_REHEARSAL_AUTOMATION_PATH: &str =
    "artifacts/release/m5/ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains.json";

/// Embedded checked-in register JSON.
pub const SHIP_M5_REHEARSAL_AUTOMATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains.json"
));

/// The standing rehearsal a family row runs on every promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehearsalKind {
    /// From-clean-state rebuild proof.
    CleanRoomRebuild,
    /// Exact-build symbol / source-map verification.
    ExactBuildSymbolication,
    /// Release-center / mirror / offline parity check.
    ReleaseCenterSync,
    /// Advisory / emergency-disable / revocation drill.
    AdvisoryRevocationDrill,
}

impl RehearsalKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CleanRoomRebuild,
        Self::ExactBuildSymbolication,
        Self::ReleaseCenterSync,
        Self::AdvisoryRevocationDrill,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanRoomRebuild => "clean_room_rebuild",
            Self::ExactBuildSymbolication => "exact_build_symbolication",
            Self::ReleaseCenterSync => "release_center_sync",
            Self::AdvisoryRevocationDrill => "advisory_revocation_drill",
        }
    }
}

/// The outcome of running a rehearsal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehearsalResult {
    /// The rehearsal ran and is green.
    Passed,
    /// The rehearsal ran and is red.
    Failed,
    /// The rehearsal has never been run.
    NotRun,
}

impl RehearsalResult {
    /// Every result, in declaration order.
    pub const ALL: [Self; 3] = [Self::Passed, Self::Failed, Self::NotRun];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::NotRun => "not_run",
        }
    }
}

/// Whether a clean-room rebuild rehearsal rebuilt from a clean state or only from
/// a warmed cache. A warm-cache-only run never counts as rebuild proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildProvenance {
    /// Rebuilt from a clean state; counts as rebuild proof.
    FromCleanState,
    /// Rebuilt from a warmed cache only; does not count as rebuild proof.
    WarmCacheOnly,
    /// Rebuild provenance does not apply to this rehearsal kind.
    NotApplicable,
}

impl RebuildProvenance {
    /// Every value, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::FromCleanState,
        Self::WarmCacheOnly,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FromCleanState => "from_clean_state",
            Self::WarmCacheOnly => "warm_cache_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Automation state a family row earned across its four rehearsals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehearsalAutomationState {
    /// Every rehearsal is green, within its SLO, and guardrail-clean.
    Current,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// A rehearsal packet breached its freshness SLO.
    NarrowedStale,
    /// A required rehearsal has never been run / its packet is missing.
    NarrowedMissing,
    /// A rehearsal ran red.
    NarrowedFailed,
    /// A guardrail tripped or required evidence is incomplete.
    NarrowedUnbacked,
    /// A waiver the row relied on expired.
    NarrowedWaiverExpired,
}

impl RehearsalAutomationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Current,
        Self::OnWaiver,
        Self::NarrowedStale,
        Self::NarrowedMissing,
        Self::NarrowedFailed,
        Self::NarrowedUnbacked,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::OnWaiver => "on_waiver",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedMissing => "narrowed_missing",
            Self::NarrowedFailed => "narrowed_failed",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets the row carry its claimed label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Current | Self::OnWaiver)
    }

    /// Whether the state forces the row below the claimed label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a row narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehearsalGapReason {
    /// The backing public claim's canonical label was already narrowed.
    ClaimLabelNarrowed,
    /// A required rehearsal has never run / its packet is missing.
    RehearsalPacketMissing,
    /// A rehearsal packet breached its freshness SLO.
    RehearsalPacketStale,
    /// A rehearsal ran red.
    RehearsalFailed,
    /// The clean-room rebuild rehearsal was a warm-cache-only run.
    RebuildCacheOnly,
    /// Symbolication freshness ran ahead of the release-center sync rehearsal.
    SymbolicationFreshnessDecoupled,
    /// Required rehearsal evidence is incomplete.
    EvidenceIncomplete,
    /// A waiver the row relied on expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl RehearsalGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ClaimLabelNarrowed,
        Self::RehearsalPacketMissing,
        Self::RehearsalPacketStale,
        Self::RehearsalFailed,
        Self::RebuildCacheOnly,
        Self::SymbolicationFreshnessDecoupled,
        Self::EvidenceIncomplete,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::RehearsalPacketMissing => "rehearsal_packet_missing",
            Self::RehearsalPacketStale => "rehearsal_packet_stale",
            Self::RehearsalFailed => "rehearsal_failed",
            Self::RebuildCacheOnly => "rebuild_cache_only",
            Self::SymbolicationFreshnessDecoupled => "symbolication_freshness_decoupled",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehearsalAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Re-run the failing or stale rehearsal.
    RerunRehearsal,
    /// Refresh the supporting evidence.
    RefreshEvidence,
    /// Re-couple symbolication freshness to the release-center sync rehearsal.
    RecoupleFreshness,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl RehearsalAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::RerunRehearsal,
        Self::RefreshEvidence,
        Self::RecoupleFreshness,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::RerunRehearsal => "rerun_rehearsal",
            Self::RefreshEvidence => "refresh_evidence",
            Self::RecoupleFreshness => "recouple_freshness",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One rehearsal record on a family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RehearsalRecord {
    /// The rehearsal this record reports.
    pub kind: RehearsalKind,
    /// The result of the most recent run.
    pub result: RehearsalResult,
    /// The proof packet and its freshness SLO, carrying the expiry signal.
    pub proof_packet: ProofPacket,
    /// Whether the rebuild ran from a clean state. `not_applicable` for every
    /// kind other than [`RehearsalKind::CleanRoomRebuild`].
    pub rebuild_provenance: RebuildProvenance,
    /// Reviewable one-line statement of the rehearsal posture.
    pub rationale: String,
}

impl RehearsalRecord {
    /// True when the rehearsal packet is within its freshness SLO.
    pub fn is_within_slo(&self) -> bool {
        self.proof_packet.slo_state.is_within_slo()
    }

    /// True when this is a clean-room rebuild that ran warm-cache-only.
    pub fn is_cache_only(&self) -> bool {
        self.kind == RehearsalKind::CleanRoomRebuild
            && self.rebuild_provenance == RebuildProvenance::WarmCacheOnly
    }

    /// True when the rehearsal counts as proof: it ran green, is within its SLO,
    /// and — for a rebuild — rebuilt from a clean state.
    pub fn is_proven(&self) -> bool {
        self.result == RehearsalResult::Passed && self.is_within_slo() && !self.is_cache_only()
    }
}

/// One rehearsal-automation stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RehearsalStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: RehearsalGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: RehearsalAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 rehearsal-automation row: a claimed artifact family and its rehearsals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RehearsalRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The artifact family this row governs.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact-family subject ref this row speaks about.
    pub subject_ref: String,
    /// Reviewable one-line statement of the family.
    pub subject_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim entry id whose public claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// Automation state earned for the row.
    pub automation_state: RehearsalAutomationState,
    /// The four rehearsal records, one per [`RehearsalKind`].
    pub rehearsals: Vec<RehearsalRecord>,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<RehearsalGapReason>,
    /// The lifecycle label the family effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that ingest this row's rehearsal truth.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5RehearsalRow {
    /// Returns the rehearsal record for `kind`, if present.
    pub fn rehearsal(&self, kind: RehearsalKind) -> Option<&RehearsalRecord> {
        self.rehearsals.iter().find(|r| r.kind == kind)
    }

    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the family carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.automation_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: RehearsalGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when every rehearsal counts as proof.
    pub fn all_rehearsals_proven(&self) -> bool {
        RehearsalKind::ALL
            .iter()
            .all(|kind| self.rehearsal(*kind).map(RehearsalRecord::is_proven) == Some(true))
    }

    /// True when symbolication freshness has run ahead of the release-center sync
    /// rehearsal: symbolication is within its SLO while release-center sync has
    /// fallen out of policy. This is the guardrail against decoupling
    /// symbolication freshness from release-center / support / export freshness.
    pub fn symbolication_decoupled(&self) -> bool {
        match (
            self.rehearsal(RehearsalKind::ExactBuildSymbolication),
            self.rehearsal(RehearsalKind::ReleaseCenterSync),
        ) {
            (Some(symbol), Some(release)) => {
                symbol.is_within_slo() && release.proof_packet.slo_state.forces_narrowing()
            }
            _ => false,
        }
    }

    /// True when the clean-room rebuild rehearsal ran warm-cache-only.
    pub fn rebuild_cache_only(&self) -> bool {
        self.rehearsal(RehearsalKind::CleanRoomRebuild)
            .map(RehearsalRecord::is_cache_only)
            .unwrap_or(false)
    }

    /// True when any rehearsal ran red.
    pub fn any_rehearsal_failed(&self) -> bool {
        self.rehearsals
            .iter()
            .any(|r| r.result == RehearsalResult::Failed)
    }

    /// True when any rehearsal packet breached its freshness SLO.
    pub fn any_rehearsal_stale(&self) -> bool {
        self.rehearsals
            .iter()
            .any(|r| r.proof_packet.slo_state == FreshnessSloState::Breached)
    }

    /// True when any rehearsal packet is missing / never run.
    pub fn any_rehearsal_missing(&self) -> bool {
        self.rehearsals
            .iter()
            .any(|r| r.proof_packet.slo_state == FreshnessSloState::Missing)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RehearsalAutomationSummary {
    /// Total number of family rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_holding: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook-pack rows.
    pub notebook_pack_entries: usize,
    /// Request/data-asset rows.
    pub request_data_asset_entries: usize,
    /// Profiler/replay-artifact rows.
    pub profiler_replay_artifact_entries: usize,
    /// Framework/template-pack rows.
    pub framework_template_pack_entries: usize,
    /// Docs-pack rows.
    pub docs_pack_entries: usize,
    /// Model-pack rows.
    pub model_pack_entries: usize,
    /// Companion/offboarding-packet rows.
    pub companion_offboarding_packet_entries: usize,
    /// Managed-output rows.
    pub managed_output_entries: usize,
    /// Rehearsal packets whose SLO state is `current`.
    pub rehearsals_current: usize,
    /// Rehearsal packets whose SLO state is `due_for_refresh`.
    pub rehearsals_due_for_refresh: usize,
    /// Rehearsal packets whose SLO state is `breached`.
    pub rehearsals_breached: usize,
    /// Rehearsal packets whose SLO state is `missing`.
    pub rehearsals_missing: usize,
    /// Rehearsals that passed.
    pub rehearsals_passed: usize,
    /// Rehearsals that failed.
    pub rehearsals_failed: usize,
    /// Rehearsals that have never been run.
    pub rehearsals_not_run: usize,
    /// Rows narrowed by the warm-cache-only rebuild guardrail.
    pub rebuild_cache_only_entries: usize,
    /// Rows narrowed by the symbolication-freshness-decoupled guardrail.
    pub symbolication_decoupled_entries: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RehearsalAutomationExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The artifact family this row governs.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact-family subject ref.
    pub subject_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim entry id whose public claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Automation state earned.
    pub automation_state: RehearsalAutomationState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<RehearsalGapReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RehearsalAutomationExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5RehearsalAutomationExportRow>,
}

/// One per-rehearsal entry in the expiry feed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RehearsalExpiryEntry {
    /// The family-row id this rehearsal belongs to.
    pub entry_id: String,
    /// The artifact family this rehearsal belongs to.
    pub family_kind: M5ArtifactFamilyKind,
    /// The rehearsal kind.
    pub rehearsal_kind: RehearsalKind,
    /// The rehearsal result.
    pub result: RehearsalResult,
    /// The rehearsal packet's freshness/expiry state.
    pub slo_state: FreshnessSloState,
    /// UTC date the rehearsal packet was captured, or null when never run.
    pub captured_at: Option<String>,
    /// Whether the rehearsal counts as proof.
    pub proven: bool,
}

/// Machine-readable rehearsal-result and expiry feed.
///
/// Shiproom dashboards and the canonical evidence index both ingest this exact
/// feed, so neither has to reconstruct rehearsal freshness from prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RehearsalExpiryFeed {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision the feed rolls up to.
    pub promotion_decision: PromotionDecision,
    /// Per-rehearsal expiry entries, in row-then-kind order.
    pub entries: Vec<RehearsalExpiryEntry>,
}

/// The typed M5 rehearsal-automation register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RehearsalAutomationRegister {
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
    /// Ref to the stable claim manifest this register ingests.
    pub claim_manifest_ref: String,
    /// Ref to the M5 release-candidate/publication matrix this register drives.
    pub publication_matrix_ref: String,
    /// Ref to the clean-room rebuild proof this automation exercises.
    pub clean_room_proof_ref: String,
    /// Ref to the exact-build identity this automation symbolicates against.
    pub exact_build_identity_ref: String,
    /// Ref to the release-center object this automation syncs against.
    pub release_center_ref: String,
    /// Ref to the advisory / revocation register this automation drills.
    pub advisory_register_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<M5ArtifactFamilyKind>,
    /// Closed rehearsal-kind vocabulary.
    pub rehearsal_kinds: Vec<RehearsalKind>,
    /// Closed rehearsal-result vocabulary.
    pub rehearsal_results: Vec<RehearsalResult>,
    /// Closed rebuild-provenance vocabulary.
    pub rebuild_provenances: Vec<RebuildProvenance>,
    /// Closed automation-state vocabulary.
    pub automation_states: Vec<RehearsalAutomationState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<RehearsalGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<RehearsalAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family subject refs this register covers.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<RehearsalStopRule>,
    /// Family rows.
    pub rows: Vec<M5RehearsalRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5RehearsalAutomationSummary,
}

impl M5RehearsalAutomationRegister {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5RehearsalRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5RehearsalRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5RehearsalRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5RehearsalRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one artifact family kind.
    pub fn rows_for_kind(&self, kind: M5ArtifactFamilyKind) -> Vec<&M5RehearsalRow> {
        self.rows
            .iter()
            .filter(|row| row.family_kind == kind)
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
    pub fn stop_rule_fires(&self, rule: &RehearsalStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
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

    /// Family-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose
    /// claim is already canonically narrowed is not a *promotion* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<RehearsalGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
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

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> M5RehearsalAutomationSummary {
        let kind = |kind: M5ArtifactFamilyKind| self.rows_for_kind(kind).len();
        let slo = |state: FreshnessSloState| {
            self.rows
                .iter()
                .flat_map(|row| row.rehearsals.iter())
                .filter(|r| r.proof_packet.slo_state == state)
                .count()
        };
        let result = |outcome: RehearsalResult| {
            self.rows
                .iter()
                .flat_map(|row| row.rehearsals.iter())
                .filter(|r| r.result == outcome)
                .count()
        };
        let release_blocking = self.release_blocking_rows();
        M5RehearsalAutomationSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_holding_stable: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.automation_state == RehearsalAutomationState::OnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_holding: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_pack_entries: kind(M5ArtifactFamilyKind::NotebookPack),
            request_data_asset_entries: kind(M5ArtifactFamilyKind::RequestDataAsset),
            profiler_replay_artifact_entries: kind(M5ArtifactFamilyKind::ProfilerReplayArtifact),
            framework_template_pack_entries: kind(M5ArtifactFamilyKind::FrameworkTemplatePack),
            docs_pack_entries: kind(M5ArtifactFamilyKind::DocsPack),
            model_pack_entries: kind(M5ArtifactFamilyKind::ModelPack),
            companion_offboarding_packet_entries: kind(
                M5ArtifactFamilyKind::CompanionOffboardingPacket,
            ),
            managed_output_entries: kind(M5ArtifactFamilyKind::ManagedOutput),
            rehearsals_current: slo(FreshnessSloState::Current),
            rehearsals_due_for_refresh: slo(FreshnessSloState::DueForRefresh),
            rehearsals_breached: slo(FreshnessSloState::Breached),
            rehearsals_missing: slo(FreshnessSloState::Missing),
            rehearsals_passed: result(RehearsalResult::Passed),
            rehearsals_failed: result(RehearsalResult::Failed),
            rehearsals_not_run: result(RehearsalResult::NotRun),
            rebuild_cache_only_entries: self
                .rows
                .iter()
                .filter(|row| row.has_active_reason(RehearsalGapReason::RebuildCacheOnly))
                .count(),
            symbolication_decoupled_entries: self
                .rows
                .iter()
                .filter(|row| {
                    row.has_active_reason(RehearsalGapReason::SymbolicationFreshnessDecoupled)
                })
                .count(),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5RehearsalAutomationExportProjection {
        M5RehearsalAutomationExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5RehearsalAutomationExportRow {
                    entry_id: row.entry_id.clone(),
                    family_kind: row.family_kind,
                    subject_ref: row.subject_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    automation_state: row.automation_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Produces the machine-readable rehearsal-result and expiry feed that
    /// shiproom dashboards and the evidence index ingest verbatim.
    pub fn rehearsal_expiry_feed(&self) -> RehearsalExpiryFeed {
        let mut entries = Vec::new();
        for row in &self.rows {
            for kind in RehearsalKind::ALL {
                if let Some(record) = row.rehearsal(kind) {
                    entries.push(RehearsalExpiryEntry {
                        entry_id: row.entry_id.clone(),
                        family_kind: row.family_kind,
                        rehearsal_kind: kind,
                        result: record.result,
                        slo_state: record.proof_packet.slo_state,
                        captured_at: record.proof_packet.captured_at.clone(),
                        proven: record.is_proven(),
                    });
                }
            }
        }
        RehearsalExpiryFeed {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            entries,
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5RehearsalAutomationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5RehearsalAutomationViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5RehearsalAutomationViolation::EmptyRegister);
        }

        let present_kinds: BTreeSet<M5ArtifactFamilyKind> =
            self.rows.iter().map(|row| row.family_kind).collect();
        for kind in M5ArtifactFamilyKind::ALL {
            if !present_kinds.contains(&kind) {
                violations.push(M5RehearsalAutomationViolation::FamilyKindMissing { kind });
            }
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5RehearsalAutomationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5RehearsalAutomationViolation>) {
        if self.schema_version != SHIP_M5_REHEARSAL_AUTOMATION_SCHEMA_VERSION {
            violations.push(M5RehearsalAutomationViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != SHIP_M5_REHEARSAL_AUTOMATION_RECORD_KIND {
            violations.push(M5RehearsalAutomationViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("publication_matrix_ref", &self.publication_matrix_ref),
            ("clean_room_proof_ref", &self.clean_room_proof_ref),
            ("exact_build_identity_ref", &self.exact_build_identity_ref),
            ("release_center_ref", &self.release_center_ref),
            ("advisory_register_ref", &self.advisory_register_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RehearsalAutomationViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.family_kinds != M5ArtifactFamilyKind::ALL.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "family_kinds",
            });
        }
        if self.rehearsal_kinds != RehearsalKind::ALL.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "rehearsal_kinds",
            });
        }
        if self.rehearsal_results != RehearsalResult::ALL.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "rehearsal_results",
            });
        }
        if self.rebuild_provenances != RebuildProvenance::ALL.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "rebuild_provenances",
            });
        }
        if self.automation_states != RehearsalAutomationState::ALL.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "automation_states",
            });
        }
        if self.gap_reasons != RehearsalGapReason::ALL.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != RehearsalAction::ALL.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5RehearsalAutomationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5RehearsalAutomationViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5RehearsalAutomationViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5RehearsalAutomationViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5RehearsalAutomationViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5RehearsalAutomationViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5RehearsalAutomationViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in RehearsalGapReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(M5RehearsalAutomationViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5RehearsalRow,
        violations: &mut Vec<M5RehearsalAutomationViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("subject_ref", &row.subject_ref),
            ("subject_summary", &row.subject_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RehearsalAutomationViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_row_rehearsals(row, violations);

        // The ceiling: no family may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5RehearsalAutomationViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // A public claim whose canonical label is below the cutline forces the
        // family to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5RehearsalAutomationViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5RehearsalAutomationViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.automation_state,
                });
            }
        }

        if row.holds_label() {
            self.validate_held_row(row, violations);
        } else {
            self.validate_narrowed_row(row, violations);
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_row_rehearsals(
        &self,
        row: &M5RehearsalRow,
        violations: &mut Vec<M5RehearsalAutomationViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for record in &row.rehearsals {
            if !seen.insert(record.kind) {
                violations.push(M5RehearsalAutomationViolation::DuplicateRehearsalKind {
                    entry_id: row.entry_id.clone(),
                    kind: record.kind,
                });
            }
            for (field, value) in [
                ("proof_packet.packet_id", &record.proof_packet.packet_id),
                ("proof_packet.packet_ref", &record.proof_packet.packet_ref),
                (
                    "proof_packet.proof_index_ref",
                    &record.proof_packet.proof_index_ref,
                ),
                (
                    "proof_packet.freshness_slo.slo_register_ref",
                    &record.proof_packet.freshness_slo.slo_register_ref,
                ),
                ("rationale", &record.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5RehearsalAutomationViolation::EmptyField {
                        entry_id: format!("{}::{}", row.entry_id, record.kind.as_str()),
                        field_name: field,
                    });
                }
            }
            if record.proof_packet.freshness_slo.target_max_age_days == 0
                || !record.proof_packet.freshness_slo.window_is_consistent()
            {
                violations.push(M5RehearsalAutomationViolation::FreshnessSloInconsistent {
                    entry_id: row.entry_id.clone(),
                    kind: record.kind,
                });
            }
            // Rebuild provenance applies only to the clean-room rebuild.
            if record.kind != RehearsalKind::CleanRoomRebuild
                && record.rebuild_provenance != RebuildProvenance::NotApplicable
            {
                violations.push(M5RehearsalAutomationViolation::ProvenanceMisapplied {
                    entry_id: row.entry_id.clone(),
                    kind: record.kind,
                });
            }
        }
        for kind in RehearsalKind::ALL {
            if row.rehearsal(kind).is_none() {
                violations.push(M5RehearsalAutomationViolation::RehearsalKindMissing {
                    entry_id: row.entry_id.clone(),
                    kind,
                });
            }
        }
    }

    fn validate_held_row(
        &self,
        row: &M5RehearsalRow,
        violations: &mut Vec<M5RehearsalAutomationViolation>,
    ) {
        if row.published_label != row.claim_label {
            violations.push(M5RehearsalAutomationViolation::HeldLabelNotEqualClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }
        if !row.active_gap_reasons.is_empty() {
            violations.push(M5RehearsalAutomationViolation::HeldWithActiveGap {
                entry_id: row.entry_id.clone(),
            });
        }
        if !row.all_rehearsals_proven() {
            violations.push(M5RehearsalAutomationViolation::HeldWithUnprovenRehearsal {
                entry_id: row.entry_id.clone(),
            });
        }
        if row.symbolication_decoupled() {
            violations.push(M5RehearsalAutomationViolation::HeldWhileDecoupled {
                entry_id: row.entry_id.clone(),
            });
        }
        if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
            violations.push(M5RehearsalAutomationViolation::HeldWithoutSignoff {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_narrowed_row(
        &self,
        row: &M5RehearsalRow,
        violations: &mut Vec<M5RehearsalAutomationViolation>,
    ) {
        if row.publishes_stable() {
            violations.push(M5RehearsalAutomationViolation::PublishedLabelNotNarrowed {
                entry_id: row.entry_id.clone(),
                state: row.automation_state,
                published: row.published_label,
            });
        }
        if row.active_gap_reasons.is_empty() {
            violations.push(M5RehearsalAutomationViolation::NarrowingWithoutReason {
                entry_id: row.entry_id.clone(),
                state: row.automation_state,
            });
        }
        // Every narrowing data condition must name its matching reason, so the
        // register cannot narrow for one cause while claiming another.
        if row.any_rehearsal_stale()
            && !row.has_active_reason(RehearsalGapReason::RehearsalPacketStale)
        {
            violations.push(M5RehearsalAutomationViolation::DataWithoutReason {
                entry_id: row.entry_id.clone(),
                reason: RehearsalGapReason::RehearsalPacketStale,
            });
        }
        if row.any_rehearsal_missing()
            && !row.has_active_reason(RehearsalGapReason::RehearsalPacketMissing)
        {
            violations.push(M5RehearsalAutomationViolation::DataWithoutReason {
                entry_id: row.entry_id.clone(),
                reason: RehearsalGapReason::RehearsalPacketMissing,
            });
        }
        if row.any_rehearsal_failed() && !row.has_active_reason(RehearsalGapReason::RehearsalFailed)
        {
            violations.push(M5RehearsalAutomationViolation::DataWithoutReason {
                entry_id: row.entry_id.clone(),
                reason: RehearsalGapReason::RehearsalFailed,
            });
        }
        if row.rebuild_cache_only() && !row.has_active_reason(RehearsalGapReason::RebuildCacheOnly)
        {
            violations.push(M5RehearsalAutomationViolation::DataWithoutReason {
                entry_id: row.entry_id.clone(),
                reason: RehearsalGapReason::RebuildCacheOnly,
            });
        }
        if row.symbolication_decoupled()
            && !row.has_active_reason(RehearsalGapReason::SymbolicationFreshnessDecoupled)
        {
            violations.push(M5RehearsalAutomationViolation::DataWithoutReason {
                entry_id: row.entry_id.clone(),
                reason: RehearsalGapReason::SymbolicationFreshnessDecoupled,
            });
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &M5RehearsalRow,
        violations: &mut Vec<M5RehearsalAutomationViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<M5RehearsalAutomationViolation>,
                               expected: RehearsalGapReason| {
            violations.push(M5RehearsalAutomationViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.automation_state,
                expected_reason: expected,
            });
        };

        match row.automation_state {
            RehearsalAutomationState::NarrowedStale => {
                if !row.has_active_reason(RehearsalGapReason::RehearsalPacketStale) {
                    push_incoherent(violations, RehearsalGapReason::RehearsalPacketStale);
                }
            }
            RehearsalAutomationState::NarrowedMissing => {
                if !row.has_active_reason(RehearsalGapReason::RehearsalPacketMissing) {
                    push_incoherent(violations, RehearsalGapReason::RehearsalPacketMissing);
                }
            }
            RehearsalAutomationState::NarrowedFailed => {
                if !row.has_active_reason(RehearsalGapReason::RehearsalFailed) {
                    push_incoherent(violations, RehearsalGapReason::RehearsalFailed);
                }
            }
            RehearsalAutomationState::NarrowedUnbacked => {
                const ALLOWED: [RehearsalGapReason; 4] = [
                    RehearsalGapReason::EvidenceIncomplete,
                    RehearsalGapReason::RebuildCacheOnly,
                    RehearsalGapReason::SymbolicationFreshnessDecoupled,
                    RehearsalGapReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|reason| row.has_active_reason(*reason)) {
                    push_incoherent(violations, RehearsalGapReason::EvidenceIncomplete);
                }
            }
            RehearsalAutomationState::NarrowedWaiverExpired => {
                if !row.has_active_reason(RehearsalGapReason::WaiverExpired) {
                    push_incoherent(violations, RehearsalGapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(M5RehearsalAutomationViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.automation_state,
                    });
                }
            }
            RehearsalAutomationState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(M5RehearsalAutomationViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.automation_state,
                    });
                }
            }
            RehearsalAutomationState::Current => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5RehearsalAutomationViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.subject_ref.clone())
            .collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5RehearsalAutomationViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_family_refs.contains(&row.subject_ref)
            {
                violations.push(
                    M5RehearsalAutomationViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<M5RehearsalAutomationViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(M5RehearsalAutomationViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(M5RehearsalAutomationViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                M5RehearsalAutomationViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                M5RehearsalAutomationViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                M5RehearsalAutomationViolation::PromotionBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the M5 rehearsal-automation register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RehearsalAutomationViolation {
    /// The register carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// The register carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no rows.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule names no labels to watch.
    StopRuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no stop rule watching for it.
    GapReasonWithoutStopRule {
        /// Uncovered reason.
        reason: RehearsalGapReason,
    },
    /// A required artifact family kind has no row.
    FamilyKindMissing {
        /// Missing family kind.
        kind: M5ArtifactFamilyKind,
    },
    /// A row is missing a required rehearsal kind.
    RehearsalKindMissing {
        /// Row id.
        entry_id: String,
        /// Missing rehearsal kind.
        kind: RehearsalKind,
    },
    /// A row carries the same rehearsal kind more than once.
    DuplicateRehearsalKind {
        /// Row id.
        entry_id: String,
        /// Duplicated rehearsal kind.
        kind: RehearsalKind,
    },
    /// A non-rebuild rehearsal declares a rebuild provenance.
    ProvenanceMisapplied {
        /// Row id.
        entry_id: String,
        /// Offending rehearsal kind.
        kind: RehearsalKind,
    },
    /// A rehearsal's freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
        /// Offending rehearsal kind.
        kind: RehearsalKind,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A row holds a label while the public claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Automation state.
        state: RehearsalAutomationState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Automation state.
        state: RehearsalAutomationState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held row carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held row has active gap reasons.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A held row carries a rehearsal that does not count as proof.
    HeldWithUnprovenRehearsal {
        /// Row id.
        entry_id: String,
    },
    /// A held row's symbolication freshness ran ahead of release-center sync.
    HeldWhileDecoupled {
        /// Row id.
        entry_id: String,
    },
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing data condition is present but its matching reason is not named.
    DataWithoutReason {
        /// Row id.
        entry_id: String,
        /// The reason the data implies.
        reason: RehearsalGapReason,
    },
    /// A state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Automation state.
        state: RehearsalAutomationState,
        /// Reason the state requires.
        expected_reason: RehearsalGapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Automation state.
        state: RehearsalAutomationState,
    },
    /// A release-blocking surface ref has no covering row.
    ReleaseBlockingSurfaceUncovered {
        /// Surface ref.
        surface_ref: String,
    },
    /// A release-blocking row is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Row id.
        entry_id: String,
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

impl fmt::Display for M5RehearsalAutomationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::NoStopRules => write!(f, "register has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate entry id {entry_id}"),
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => write!(
                f,
                "gap reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::FamilyKindMissing { kind } => {
                write!(f, "missing row for family kind {}", kind.as_str())
            }
            Self::RehearsalKindMissing { entry_id, kind } => {
                write!(f, "row {entry_id} is missing rehearsal {}", kind.as_str())
            }
            Self::DuplicateRehearsalKind { entry_id, kind } => write!(
                f,
                "row {entry_id} carries rehearsal {} more than once",
                kind.as_str()
            ),
            Self::ProvenanceMisapplied { entry_id, kind } => write!(
                f,
                "row {entry_id} rehearsal {} must not declare a rebuild provenance",
                kind.as_str()
            ),
            Self::FreshnessSloInconsistent { entry_id, kind } => write!(
                f,
                "row {entry_id} rehearsal {} freshness SLO window is inconsistent",
                kind.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} published level {} is wider than claim {}",
                published.as_str(),
                claim.as_str()
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "row {entry_id} holds label while claim {} is below cutline",
                claim.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "row {entry_id} state {} narrows without active reason",
                state.as_str()
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "row {entry_id} state {} must narrow but publishes {}",
                state.as_str(),
                published.as_str()
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} held label {} does not equal claim {}",
                published.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "row {entry_id} holds label with active gap")
            }
            Self::HeldWithUnprovenRehearsal { entry_id } => {
                write!(f, "row {entry_id} holds label with an unproven rehearsal")
            }
            Self::HeldWhileDecoupled { entry_id } => write!(
                f,
                "row {entry_id} holds label while symbolication freshness is decoupled"
            ),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds label without owner signoff")
            }
            Self::DataWithoutReason { entry_id, reason } => write!(
                f,
                "row {entry_id} has the condition for {} but does not name it",
                reason.as_str()
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {entry_id} state {} requires reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "row {entry_id} state {} names no waiver", state.as_str())
            }
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering row"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking row {entry_id} is not declared in release_blocking_family_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => write!(
                f,
                "promotion {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
        }
    }
}

impl Error for M5RehearsalAutomationViolation {}

/// Loads the embedded M5 rehearsal-automation register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5RehearsalAutomationRegister`].
pub fn current_m5_rehearsal_automation_register(
) -> Result<M5RehearsalAutomationRegister, serde_json::Error> {
    serde_json::from_str(SHIP_M5_REHEARSAL_AUTOMATION_JSON)
}

#[cfg(test)]
mod tests;
