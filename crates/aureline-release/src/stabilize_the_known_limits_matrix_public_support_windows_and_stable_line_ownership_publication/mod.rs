//! Typed register stabilizing the known-limits matrix, public support windows,
//! and stable-line ownership publication for the M4 stable line.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_publication_pack`](crate::stable_publication_pack) governs the
//! outward-facing benchmark publications the release line ships; and the
//! [`maintenance_control_packet`](crate::maintenance_control_packet) governs each
//! post-release maintenance lane. None of them answer the question this module
//! answers: **for each known-limits entry, public support window, and stable-line
//! ownership record — is that entry actually backed by a fresh proof packet, an
//! owner sign-off, and a complete support window or ownership publication, and is
//! it narrowed below the cutline the moment its backing thins out?** This module
//! is the **known-limits, support-window, and ownership publication register**. For
//! every such entry it records one row that binds the entry to the
//! [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose lifecycle
//! label it backs, the proof packet that grounds it, the waiver (if any) holding it
//! provisionally, and the owner sign-off.
//!
//! Each [`StabilizeRow`] is one `(entry, public claim)` binding. It:
//!
//! - names the entry kind it governs ([`StabilizeRow::kind`],
//!   [`StabilizeRow::surface_ref`], [`StabilizeRow::surface_summary`]) and whether
//!   that entry is part of the release-blocking set
//!   ([`StabilizeRow::release_blocking`]);
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose
//!   public claim it backs ([`StabilizeRow::claim_ref`]) and the canonical lifecycle
//!   label that entry publishes ([`StabilizeRow::claim_label`]). That label is a hard
//!   **ceiling**: an entry may carry the claim's label or narrow below it, but it may
//!   never assert a public claim wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-entry labels,
//!   so docs, Help/About, the release center, and support exports ingest one label
//!   per entry instead of cloning their own;
//! - records the entry state earned ([`StabilizeState`]), the active gap reasons
//!   ([`StabilizeGapReason`]), and the label it *effectively* publishes after narrowing
//!   ([`StabilizeRow::published_label`]);
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between an entry whose backing supports a Stable public claim and one narrowed
//! below it. An entry that is not backed — because its proof packet aged out or is
//! missing, because its support window is incomplete or has passed its end-of-support
//! date, because its ownership record is missing, because its waiver expired, because
//! its evidence is incomplete, or because the public claim it backs is itself below
//! the cutline — is structurally required to drop below the cutline rather than
//! inherit an adjacent backed entry. The [`StabilizeRule`] set names the closed
//! conditions that gate publication, and [`StabilizeTheKnownLimitsMatrixPublicSupportWindowsAndStableLineOwnershipPublication::publication`] records the
//! proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Two
//! classes of check live outside this model because they need more than the register
//! sees: date arithmetic (recomputing the packet-freshness state and waiver expiry
//! against an `as_of` date) and the cross-artifact ceiling check (whether each row's
//! `claim_label` still equals the label the stable claim manifest publishes for the
//! entry named by `claim_ref`). Those live in the CI gate. This model enforces the
//! structural and logical invariants that hold regardless of the clock and the
//! neighbouring artifact — the ceiling/no-widening rule, narrowing consistency,
//! packet/state coherence, owner sign-off on backed rows, kind and release-line
//! coverage, publication-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported schema version.
pub const STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_RECORD_KIND: &str = "stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication";

/// Repo-relative path to the checked-in register.
pub const STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_PATH: &str =
    "artifacts/release/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication.json";

/// Embedded checked-in register JSON.
pub const STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication.json"
));

/// The entry kind a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizeKind {
    /// A known-limits entry documenting caveats and unsupported states.
    KnownLimit,
    /// A public support window commitment.
    PublicSupportWindow,
    /// A stable-line ownership publication.
    StableLineOwnership,
}

impl StabilizeKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::KnownLimit,
        Self::PublicSupportWindow,
        Self::StableLineOwnership,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KnownLimit => "known_limit",
            Self::PublicSupportWindow => "public_support_window",
            Self::StableLineOwnership => "stable_line_ownership",
        }
    }
}

/// State a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizeState {
    /// The entry is backed: a captured, within-SLO proof packet backs the public
    /// claim at its full canonical lifecycle label, owner-signed.
    Stabilized,
    /// The entry carries the claim's full label only because an active, unexpired
    /// waiver covers a recorded gap.
    StabilizedOnWaiver,
    /// The proof packet or row evidence is incomplete, or owner sign-off is absent;
    /// the entry is not backed and the label must narrow.
    NarrowedUnbacked,
    /// The public claim this entry backs is itself below the cutline, so the entry
    /// inherits that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The proof packet breached its freshness SLO (or is missing); the entry is
    /// not backed and the label must narrow.
    NarrowedStale,
    /// The entry relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
    /// A support window entry passed its end-of-support date; the label must narrow.
    NarrowedSupportExpired,
    /// An ownership entry is missing its ownership record; the label must narrow.
    NarrowedOwnershipMissing,
}

impl StabilizeState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Stabilized,
        Self::StabilizedOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
        Self::NarrowedSupportExpired,
        Self::NarrowedOwnershipMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stabilized => "stabilized",
            Self::StabilizedOnWaiver => "stabilized_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::NarrowedSupportExpired => "narrowed_support_expired",
            Self::NarrowedOwnershipMissing => "narrowed_ownership_missing",
        }
    }

    /// Whether the state lets an entry carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Stabilized | Self::StabilizedOnWaiver)
    }

    /// Whether the state forces the entry below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason an entry narrows or a rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizeGapReason {
    /// The public claim this entry backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The entry names a surface capability the build does not yet implement.
    SurfaceCapabilityAbsent,
    /// The proof packet's row-level evidence is incomplete.
    EvidenceIncomplete,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// No proof packet has been captured for the entry.
    ProofPacketMissing,
    /// A waiver the entry relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
    /// A support window entry passed its end-of-support date.
    SupportWindowExpired,
    /// An ownership entry is missing its ownership record.
    OwnershipUnpublished,
}

impl StabilizeGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ClaimLabelNarrowed,
        Self::SurfaceCapabilityAbsent,
        Self::EvidenceIncomplete,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::SupportWindowExpired,
        Self::OwnershipUnpublished,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::SurfaceCapabilityAbsent => "surface_capability_absent",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::SupportWindowExpired => "support_window_expired",
            Self::OwnershipUnpublished => "ownership_unpublished",
        }
    }
}

/// Default action a rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizeAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the entry's published lifecycle label below the cutline.
    NarrowClaimLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Recapture the row-level evidence the proof packet depends on.
    RecaptureEvidence,
    /// Renew the support window so it re-enters its valid period.
    RenewSupportWindow,
    /// Publish the missing ownership record.
    PublishOwnershipRecord,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl StabilizeAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::HoldPublication,
        Self::NarrowClaimLabel,
        Self::RefreshProofPacket,
        Self::RecaptureEvidence,
        Self::RenewSupportWindow,
        Self::PublishOwnershipRecord,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowClaimLabel => "narrow_claim_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RenewSupportWindow => "renew_support_window",
            Self::PublishOwnershipRecord => "publish_ownership_record",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One rule: a closed condition that narrows an entry label and may gate publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StabilizeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: StabilizeGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StabilizeAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One register row: an `(entry, public claim)` binding bound to its proof packet,
/// canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StabilizeRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The entry kind this row governs.
    pub kind: StabilizeKind,
    /// The surface id this entry speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the entry.
    pub surface_summary: String,
    /// Whether the entry is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this entry backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: an
    /// entry may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// State earned for the entry.
    pub publication_state: StabilizeState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional entry, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<StabilizeGapReason>,
    /// The lifecycle label the entry effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl StabilizeRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the entry carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.publication_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: StabilizeGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// The recorded publication verdict for the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StabilizePublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StabilizeSummary {
    /// Total number of rows.
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
    /// Known-limit entries.
    pub known_limit_entries: usize,
    /// Support-window entries.
    pub support_window_entries: usize,
    /// Ownership entries.
    pub ownership_entries: usize,
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
    /// Number of rules currently firing.
    pub publication_rules_firing: usize,
}

/// The typed register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StabilizeTheKnownLimitsMatrixPublicSupportWindowsAndStableLineOwnershipPublication {
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
    /// Ref to the known-limits register every known-limit entry rides.
    pub known_limits_register_ref: String,
    /// Ref to the support-window template every support-window entry rides.
    pub support_window_template_ref: String,
    /// Ref to the ownership template every ownership entry rides.
    pub ownership_template_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed kind vocabulary.
    pub kinds: Vec<StabilizeKind>,
    /// Closed state vocabulary.
    pub publication_states: Vec<StabilizeState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<StabilizeGapReason>,
    /// Closed action vocabulary.
    pub publication_actions: Vec<StabilizeAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Publication rules.
    pub publication_rules: Vec<StabilizeRule>,
    /// Register rows.
    pub rows: Vec<StabilizeRow>,
    /// Recorded publication verdict.
    pub publication: StabilizePublicationRecord,
    /// Summary counts.
    pub summary: StabilizeSummary,
}

/// Parses the embedded checked-in register JSON.
pub fn current_stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication(
) -> Result<
    StabilizeTheKnownLimitsMatrixPublicSupportWindowsAndStableLineOwnershipPublication,
    serde_json::Error,
> {
    serde_json::from_str(STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_JSON)
}

impl StabilizeTheKnownLimitsMatrixPublicSupportWindowsAndStableLineOwnershipPublication {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&StabilizeRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&StabilizeRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&StabilizeRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&StabilizeRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one kind.
    pub fn rows_for_kind(&self, kind: StabilizeKind) -> Vec<&StabilizeRow> {
        self.rows.iter().filter(|row| row.kind == kind).collect()
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
    pub fn rule_fires(&self, rule: &StabilizeRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .publication_rules
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
            .publication_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim
    /// is already canonically narrowed is not a *register* blocker, it merely inherits
    /// the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<StabilizeGapReason> = self
            .publication_rules
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

    /// Recomputes the summary block from the rows and rules.
    pub fn computed_summary(&self) -> StabilizeSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: StabilizeKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&StabilizeRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        StabilizeSummary {
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
                .filter(|row| row.publication_state == StabilizeState::StabilizedOnWaiver)
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
            known_limit_entries: kind(StabilizeKind::KnownLimit),
            support_window_entries: kind(StabilizeKind::PublicSupportWindow),
            ownership_entries: kind(StabilizeKind::StableLineOwnership),
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
                .filter(|rule| self.rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> StabilizeExportProjection {
        StabilizeExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| StabilizeExportRow {
                    entry_id: row.entry_id.clone(),
                    kind: row.kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    publication_state: row.publication_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<StabilizeViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(StabilizeViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(StabilizeViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(StabilizeViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<StabilizeViolation>) {
        if self.schema_version != STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_SCHEMA_VERSION {
            violations.push(StabilizeViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_RECORD_KIND {
            violations.push(StabilizeViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("known_limits_register_ref", &self.known_limits_register_ref),
            (
                "support_window_template_ref",
                &self.support_window_template_ref,
            ),
            ("ownership_template_ref", &self.ownership_template_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StabilizeViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(StabilizeViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.kinds != StabilizeKind::ALL.to_vec() {
            violations.push(StabilizeViolation::ClosedVocabularyMismatch { field: "kinds" });
        }
        if self.publication_states != StabilizeState::ALL.to_vec() {
            violations.push(StabilizeViolation::ClosedVocabularyMismatch {
                field: "publication_states",
            });
        }
        if self.gap_reasons != StabilizeGapReason::ALL.to_vec() {
            violations.push(StabilizeViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.publication_actions != StabilizeAction::ALL.to_vec() {
            violations.push(StabilizeViolation::ClosedVocabularyMismatch {
                field: "publication_actions",
            });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(StabilizeViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(StabilizeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(StabilizeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(StabilizeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(StabilizeViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<StabilizeViolation>) {
        if self.publication_rules.is_empty() {
            violations.push(StabilizeViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.publication_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(StabilizeViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(StabilizeViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(StabilizeViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in StabilizeGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(StabilizeViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &StabilizeRow, violations: &mut Vec<StabilizeViolation>) {
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
                violations.push(StabilizeViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(StabilizeViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(StabilizeViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(StabilizeViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(StabilizeViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(StabilizeGapReason::ClaimLabelNarrowed) {
                violations.push(StabilizeViolation::ClaimNarrowedWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            if row.published_label != row.claim_label {
                violations.push(StabilizeViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(StabilizeViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(StabilizeViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(StabilizeViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(StabilizeViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            if row.publishes_stable() {
                violations.push(StabilizeViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.publication_state,
                    published: row.published_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(StabilizeViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.publication_state,
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(StabilizeGapReason::ProofPacketFreshnessBreached)
            {
                violations.push(StabilizeViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(StabilizeGapReason::ProofPacketMissing)
            {
                violations.push(StabilizeViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &StabilizeRow,
        violations: &mut Vec<StabilizeViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<StabilizeViolation>,
                               expected: StabilizeGapReason| {
            violations.push(StabilizeViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.publication_state,
                expected_reason: expected,
            });
        };

        match row.publication_state {
            StabilizeState::NarrowedUnbacked => {
                const ALLOWED: [StabilizeGapReason; 3] = [
                    StabilizeGapReason::SurfaceCapabilityAbsent,
                    StabilizeGapReason::EvidenceIncomplete,
                    StabilizeGapReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, StabilizeGapReason::EvidenceIncomplete);
                }
            }
            StabilizeState::NarrowedClaimNarrowed => {
                if !row.has_active_reason(StabilizeGapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, StabilizeGapReason::ClaimLabelNarrowed);
                }
            }
            StabilizeState::NarrowedStale => {
                if !(row.has_active_reason(StabilizeGapReason::ProofPacketFreshnessBreached)
                    || row.has_active_reason(StabilizeGapReason::ProofPacketMissing))
                {
                    push_incoherent(violations, StabilizeGapReason::ProofPacketFreshnessBreached);
                }
            }
            StabilizeState::NarrowedWaiverExpired => {
                if !row.has_active_reason(StabilizeGapReason::WaiverExpired) {
                    push_incoherent(violations, StabilizeGapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(StabilizeViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.publication_state,
                    });
                }
            }
            StabilizeState::NarrowedSupportExpired => {
                if !row.has_active_reason(StabilizeGapReason::SupportWindowExpired) {
                    push_incoherent(violations, StabilizeGapReason::SupportWindowExpired);
                }
            }
            StabilizeState::NarrowedOwnershipMissing => {
                if !row.has_active_reason(StabilizeGapReason::OwnershipUnpublished) {
                    push_incoherent(violations, StabilizeGapReason::OwnershipUnpublished);
                }
            }
            StabilizeState::StabilizedOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(StabilizeViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.publication_state,
                    });
                }
            }
            StabilizeState::Stabilized => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<StabilizeViolation>) {
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.surface_ref.as_str()) {
                // surface refs may repeat across kinds; we do not enforce uniqueness here.
            }
        }

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
                violations.push(StabilizeViolation::ReleaseBlockingRefWithoutRow {
                    surface_ref: (*declared_ref).to_owned(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.surface_ref.as_str()) {
                violations.push(StabilizeViolation::ReleaseBlockingRowNotInSet {
                    entry_id: row.entry_id.clone(),
                    surface_ref: row.surface_ref.clone(),
                });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<StabilizeViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(StabilizeViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(StabilizeViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(StabilizeViolation::PublicationDecisionInconsistent {
                declared: self.publication.decision,
                computed,
            });
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(StabilizeViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(StabilizeViolation::PublicationBlockingSetMismatch {
                field: "blocking_entry_ids",
            });
        }
    }
}

/// A redaction-safe export row projected from the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizeExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The entry kind this row governs.
    pub kind: StabilizeKind,
    /// The surface id this entry speaks about.
    pub surface_ref: String,
    /// Whether the entry is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this entry backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The lifecycle label the entry effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the entry publishes a stable label.
    pub publishes_stable: bool,
    /// State earned for the entry.
    pub publication_state: StabilizeState,
    /// Proof packet freshness SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons narrowing the row.
    pub active_gap_reasons: Vec<StabilizeGapReason>,
}

/// A redaction-safe export projection of the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizeExportProjection {
    /// Register id this projection was produced from.
    pub register_id: String,
    /// Register as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<StabilizeExportRow>,
}

/// A validation violation for the register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabilizeViolation {
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
    /// The register has no rules.
    NoRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate row id.
        entry_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: StabilizeGapReason,
    },
    /// A published label is stronger than the claim's canonical label.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing state did not drop the row below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// State.
        state: StabilizeState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing row carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// State.
        state: StabilizeState,
    },
    /// A held row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A held row does not have a captured within-SLO proof packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A held row rides a stale proof packet.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held row's published label does not match its claim label.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A row holds a label on a narrowed claim.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
    },
    /// A narrowed claim row does not name `claim_label_narrowed`.
    ClaimNarrowedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A breached packet does not name `proof_packet_freshness_breached`.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A missing packet does not name `proof_packet_missing`.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A state and its active gap reasons are incoherent.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// State.
        state: StabilizeState,
        /// Expected reason.
        expected_reason: StabilizeGapReason,
    },
    /// A waiver state lacks a waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// State.
        state: StabilizeState,
    },
    /// A declared release-blocking surface ref has no covering row.
    ReleaseBlockingRefWithoutRow {
        /// Surface ref.
        surface_ref: String,
    },
    /// A release-blocking row is not in the declared set.
    ReleaseBlockingRowNotInSet {
        /// Row id.
        entry_id: String,
        /// Surface ref.
        surface_ref: String,
    },
    /// The publication decision does not match the computed decision.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// A blocking set does not match the computed set.
    PublicationBlockingSetMismatch {
        /// Field name.
        field: &'static str,
    },
    /// The summary does not match the computed summary.
    SummaryMismatch,
    /// The freshness SLO target/warn window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for StabilizeViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version: {}", actual)
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind: {}", actual)
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch on {}", field)
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::NoRules => write!(f, "register has no rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => {
                write!(f, "empty field '{}' in {}", field_name, entry_id)
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id: {}", entry_id)
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id: {}", rule_id)
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule '{}' has no labels", rule_id)
            }
            Self::GapReasonWithoutRule { reason } => {
                write!(f, "gap reason '{}' has no rule", reason.as_str())
            }
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => {
                write!(
                    f,
                    "entry '{}' published '{}' is wider than claim '{}'",
                    entry_id,
                    published.as_str(),
                    claim.as_str()
                )
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => {
                write!(
                    f,
                    "entry '{}' in state '{}' should be narrowed but is '{}'",
                    entry_id,
                    state.as_str(),
                    published.as_str()
                )
            }
            Self::NarrowingWithoutReason { entry_id, state } => {
                write!(
                    f,
                    "entry '{}' in state '{}' narrows without a reason",
                    entry_id,
                    state.as_str()
                )
            }
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "entry '{}' is held but has an active gap", entry_id)
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "entry '{}' is held without a fresh packet", entry_id)
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "entry '{}' is held on a stale packet ({})",
                    entry_id,
                    slo_state.as_str()
                )
            }
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => {
                write!(
                    f,
                    "entry '{}' is held but published '{}' != claim '{}'",
                    entry_id,
                    published.as_str(),
                    claim.as_str()
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "entry '{}' is held without owner sign-off", entry_id)
            }
            Self::HeldOnNarrowedClaim { entry_id, claim } => {
                write!(
                    f,
                    "entry '{}' is held on a narrowed claim ({})",
                    entry_id,
                    claim.as_str()
                )
            }
            Self::ClaimNarrowedWithoutReason { entry_id } => {
                write!(
                    f,
                    "entry '{}' inherits a narrowed claim without recording the reason",
                    entry_id
                )
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "entry '{}' has a breached packet without recording the reason",
                    entry_id
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "entry '{}' has a missing packet without recording the reason",
                    entry_id
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => {
                write!(
                    f,
                    "entry '{}' in state '{}' is incoherent (expected '{}')",
                    entry_id,
                    state.as_str(),
                    expected_reason.as_str()
                )
            }
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(
                    f,
                    "entry '{}' in state '{}' lacks a waiver",
                    entry_id,
                    state.as_str()
                )
            }
            Self::ReleaseBlockingRefWithoutRow { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface '{}' has no covering row",
                    surface_ref
                )
            }
            Self::ReleaseBlockingRowNotInSet {
                entry_id,
                surface_ref,
            } => {
                write!(
                    f,
                    "release-blocking row '{}' (surface '{}') is not in the declared set",
                    entry_id, surface_ref
                )
            }
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication decision '{}' does not match computed '{}'",
                    declared.as_str(),
                    computed.as_str()
                )
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication blocking set mismatch on {}", field)
            }
            Self::SummaryMismatch => write!(f, "summary does not match computed summary"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "entry '{}' has an inconsistent freshness SLO", entry_id)
            }
        }
    }
}

impl Error for StabilizeViolation {}
