//! Typed finalization register for compatibility reports, deprecation packets,
//! schema/version windows, and migration publications for the M4 stable line.
//!
//! The [`stable_version_windows`](crate::stable_version_windows) freezes each public
//! interface surface's version window and deprecation packet. The
//! [`stable_publication_pack`](crate::stable_publication_pack) governs the outward-facing
//! known-limits, benchmark, compatibility, and migration publications. None of them answer
//! the question this module answers: **for each compatibility report, deprecation packet,
//! schema/version window, and migration publication — is there a fresh, complete, owner-signed
//! qualification packet behind it, with machine-readable scorecards, replacement paths,
//! rollback checkpoints, and diagnostics where applicable, and is the row narrowed below the
//! cutline the moment its backing thins out?** This module is the **compatibility,
//! deprecation, schema/version-window, and migration finalization register**. For every such
//! row it records one binding to the [`stable_claim_manifest`](crate::stable_claim_manifest)
//! entry whose lifecycle label it backs, the proof packet that grounds it, the waiver (if any)
//! holding it provisionally, and the owner sign-off.
//!
//! Each [`FinalizeRow`] is one `(finalize subject, public claim)` binding. It:
//!
//! - names the subject it governs ([`FinalizeRow::kind`], [`FinalizeRow::surface_ref`],
//!   [`FinalizeRow::surface_summary`]) and whether that subject is part of the release-blocking
//!   set ([`FinalizeRow::release_blocking`]);
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO and, for a
//!   compatibility report, the [`CompatibilityReportPacket`] that carries the scorecards;
//!   for a deprecation packet, the [`DeprecationDetail`] that carries the replacement path
//!   and removal checkpoint; for a migration publication, the [`MigrationDetail`] that carries
//!   rollback checkpoints and diagnostics;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose public
//!   claim it backs ([`FinalizeRow::claim_ref`]) and the canonical lifecycle label that entry
//!   publishes ([`FinalizeRow::claim_label`]). That label is a hard **ceiling**: a row may
//!   carry the claim's label or narrow below it, but it may never assert a public claim wider
//!   than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-subject labels, so
//!   docs, Help/About, the release center, and support exports ingest one label per subject
//!   instead of cloning their own;
//! - records the finalize state earned ([`FinalizeState`]), the active gap reasons
//!   ([`GapReason`]), and the label it *effectively* publishes after narrowing
//!   ([`FinalizeRow::effective_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary between a
//! row whose backing supports a Stable public claim and one narrowed below it. A row that is
//! not backed — because its proof packet aged out or is missing, because its evidence is
//! incomplete, because its scorecard degraded, because its deprecation removal is overdue,
//! because its waiver expired, because its owner sign-off is absent, or because the public
//! claim it backs is itself below the cutline — is structurally required to drop below the
//! cutline rather than inherit an adjacent backed row. The [`FinalizeRule`] set names the
//! closed conditions that gate publication, and
//! [`FinalizeCompatibilityReportsDeprecationPacketsSchemaVersionWindows::publication`] records
//! the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/finalize_compatibility_reports_deprecation_packets_schema_version_windows.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It carries no
//! raw artifacts, raw logs, signatures, or credential material. Two classes of check live
//! outside this model because they need more than the register sees: date arithmetic
//! (recomputing the packet-freshness state and waiver expiry against an `as_of` date) and the
//! cross-artifact ceiling check (whether each row's `claim_label` still equals the label the
//! stable claim manifest publishes for the entry named by `claim_ref`). Those live in the CI
//! gate. This model enforces the structural and logical invariants that hold regardless of the
//! clock and the neighbouring artifact — the ceiling/no-widening rule, packet/state coherence,
//! owner sign-off on backed rows, kind and release-line coverage, rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported finalize-register schema version.
pub const FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_RECORD_KIND:
    &str = "compatibility_deprecation_schema_migration_finalize";

/// Repo-relative path to the checked-in register.
pub const FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_PATH: &str =
    "artifacts/release/finalize_compatibility_reports_deprecation_packets_schema_version_windows.json";

/// Embedded checked-in register JSON.
pub const FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/finalize_compatibility_reports_deprecation_packets_schema_version_windows.json"
));

/// The class of finalize subject a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeKind {
    /// A compatibility report with scorecards for extension, bundle, tooling, bridge, and
    /// migration surfaces.
    CompatibilityReport,
    /// A deprecation packet with replacement path, last supported version, alias/fallback,
    /// removal checkpoint, and migration hints.
    DeprecationPacket,
    /// A schema or version window freeze for a public interface surface.
    SchemaVersionWindow,
    /// A migration publication with rollback checkpoints and diagnostics preservation.
    MigrationPublication,
}

impl FinalizeKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CompatibilityReport,
        Self::DeprecationPacket,
        Self::SchemaVersionWindow,
        Self::MigrationPublication,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompatibilityReport => "compatibility_report",
            Self::DeprecationPacket => "deprecation_packet",
            Self::SchemaVersionWindow => "schema_version_window",
            Self::MigrationPublication => "migration_publication",
        }
    }
}

/// Finalize state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeState {
    /// The row is finalized stable: a captured, within-SLO proof packet, complete evidence,
    /// and an owner sign-off back the public claim at its full canonical lifecycle label.
    FinalizedStable,
    /// The row carries the claim's full label only because an active, unexpired waiver covers
    /// a recorded gap.
    FinalizedOnWaiver,
    /// The proof packet or row evidence is incomplete, a required scorecard is degraded, or
    /// a surface capability is absent; the row is not backed and the label must narrow.
    NarrowedUnbacked,
    /// The public claim this row backs is itself below the cutline, so the row inherits that
    /// ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The proof packet breached its freshness SLO (or is missing); the row is not backed and
    /// the label must narrow.
    NarrowedStale,
    /// The row relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
    /// A deprecation in the packet passed its removal target without removal; the row cannot
    /// finalize stable until the removal lands and the label must narrow.
    NarrowedDeprecationOverdue,
    /// The row evidence is incomplete (scorecard degraded, missing migration hint, etc.); the
    /// label must narrow.
    NarrowedEvidenceIncomplete,
}

impl FinalizeState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::FinalizedStable,
        Self::FinalizedOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
        Self::NarrowedDeprecationOverdue,
        Self::NarrowedEvidenceIncomplete,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FinalizedStable => "finalized_stable",
            Self::FinalizedOnWaiver => "finalized_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::NarrowedDeprecationOverdue => "narrowed_deprecation_overdue",
            Self::NarrowedEvidenceIncomplete => "narrowed_evidence_incomplete",
        }
    }

    /// Whether the state lets a row carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::FinalizedStable | Self::FinalizedOnWaiver)
    }

    /// Whether the state forces the row below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a row narrows or a finalize rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this row backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The row names a surface capability the build does not yet implement.
    SurfaceCapabilityAbsent,
    /// The proof packet's row-level evidence is incomplete.
    EvidenceIncomplete,
    /// A compatibility report is missing its machine-readable report ref.
    ReportRefMissing,
    /// A compatibility report's validity window has expired.
    ValidityWindowExpired,
    /// A compatibility scorecard has degraded below the exact outcome.
    ScorecardDegraded,
    /// A deprecation packet is missing a required field.
    DeprecationPacketIncomplete,
    /// A deprecation's removal target date passed without the removal landing.
    DeprecationRemovalOverdue,
    /// The proof packet (or freeze packet) breached its freshness SLO.
    FreezePacketFreshnessBreached,
    /// No proof packet (or freeze packet) has been captured for the row.
    FreezePacketMissing,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ClaimLabelNarrowed,
        Self::SurfaceCapabilityAbsent,
        Self::EvidenceIncomplete,
        Self::ReportRefMissing,
        Self::ValidityWindowExpired,
        Self::ScorecardDegraded,
        Self::DeprecationPacketIncomplete,
        Self::DeprecationRemovalOverdue,
        Self::FreezePacketFreshnessBreached,
        Self::FreezePacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::SurfaceCapabilityAbsent => "surface_capability_absent",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::ReportRefMissing => "report_ref_missing",
            Self::ValidityWindowExpired => "validity_window_expired",
            Self::ScorecardDegraded => "scorecard_degraded",
            Self::DeprecationPacketIncomplete => "deprecation_packet_incomplete",
            Self::DeprecationRemovalOverdue => "deprecation_removal_overdue",
            Self::FreezePacketFreshnessBreached => "freeze_packet_freshness_breached",
            Self::FreezePacketMissing => "freeze_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a finalize rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the row's published lifecycle label below the cutline.
    NarrowLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Complete the deprecation packet (replacement path, removal target, removal).
    CompleteDeprecationPacket,
    /// Recapture the row-level evidence the proof packet depends on.
    RecaptureEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl FinalizeAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::CompleteDeprecationPacket,
        Self::RecaptureEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::CompleteDeprecationPacket => "complete_deprecation_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The outcome of a compatibility scorecard mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityOutcome {
    /// The mapping is exact: no translation, shim, or partial coverage.
    Exact,
    /// The mapping is translated: semantics are preserved with adaptation.
    Translated,
    /// The mapping is partial: some features are not covered.
    Partial,
    /// The mapping is shimmed: a compatibility layer is required.
    Shimmed,
    /// The mapping is unsupported: the surface does not work.
    Unsupported,
}

impl CompatibilityOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Exact,
        Self::Translated,
        Self::Partial,
        Self::Shimmed,
        Self::Unsupported,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Translated => "translated",
            Self::Partial => "partial",
            Self::Shimmed => "shimmed",
            Self::Unsupported => "unsupported",
        }
    }
}

/// The validity window for a compatibility report or deprecation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidityWindow {
    /// UTC date the window was captured.
    pub captured_at: String,
    /// UTC date the window expires.
    pub expires_at: String,
    /// Days the window stays claim-bearing after capture.
    pub window_days: u32,
}

/// One scorecard in a compatibility report packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scorecard {
    /// The outcome of the scorecard mapping.
    pub outcome: CompatibilityOutcome,
    /// Ref to the machine-readable report backing this scorecard.
    pub report_ref: String,
}

/// The compatibility report packet carried by a compatibility-report row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityReportPacket {
    /// Scorecard for the extension surface.
    pub extension_scorecard: Scorecard,
    /// Scorecard for the bundle surface.
    pub bundle_scorecard: Scorecard,
    /// Scorecard for the tooling surface.
    pub tooling_scorecard: Scorecard,
    /// Scorecard for the bridge surface.
    pub bridge_scorecard: Scorecard,
    /// Scorecard for the migration surface.
    pub migration_scorecard: Scorecard,
    /// Machine-readable report ref for the whole packet.
    pub report_ref: String,
    /// Validity window for the report.
    pub validity_window: ValidityWindow,
    /// Support-class outcome derived from the scorecards.
    pub support_class_outcome: StableClaimLevel,
    /// Known caveats that narrow the report's promise.
    #[serde(default)]
    pub known_caveats: Vec<String>,
}

/// The deprecation detail carried by a deprecation-packet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeprecationDetail {
    /// The replacement path for consumers of the deprecated surface.
    pub replacement_path: String,
    /// The last supported version string.
    pub last_supported_version: String,
    /// The channel window during which the deprecated surface is still supported.
    pub last_supported_channel_window: String,
    /// The alias or fallback behavior when the deprecated surface is invoked.
    pub alias_or_fallback_behavior: String,
    /// The removal checkpoint (UTC date or version) by which the deprecated surface must be
    /// removed.
    pub removal_checkpoint: String,
    /// Ref to the exported migration hints.
    pub exported_migration_hints_ref: String,
}

/// The migration detail carried by a migration-publication row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationDetail {
    /// Ref to the migration guide.
    pub migration_guide_ref: String,
    /// Rollback checkpoints that must be verified during migration.
    #[serde(default)]
    pub rollback_checkpoints: Vec<String>,
    /// Ref to the diagnostics preservation documentation.
    pub diagnostics_preservation_ref: String,
}

/// One finalize rule: a closed condition that narrows a row label and may gate publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: FinalizeAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One finalize row: a `(finalize subject, public claim)` binding bound to its proof packet,
/// kind-specific details, canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of finalize subject this row governs.
    pub kind: FinalizeKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the subject.
    pub surface_summary: String,
    /// Whether the row is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a row may never
    /// carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Finalize state earned for the row.
    pub finalize_state: FinalizeState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// The compatibility report packet, present only on compatibility-report rows.
    #[serde(default)]
    pub compatibility_report: Option<CompatibilityReportPacket>,
    /// The deprecation detail, present only on deprecation-packet rows.
    #[serde(default)]
    pub deprecation_detail: Option<DeprecationDetail>,
    /// The migration detail, present only on migration-publication rows.
    #[serde(default)]
    pub migration_detail: Option<MigrationDetail>,
    /// Waiver authorizing a provisional finalize, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the row effectively carries after narrowing.
    pub effective_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl FinalizeRow {
    /// True when the effective label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the row carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.finalize_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// The recorded publication verdict for the finalize register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizePublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Finalize-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Finalize-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeSummary {
    /// Total number of rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_finalized_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed_below_cutline: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_finalized_stable: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Compatibility-report rows.
    pub compatibility_report_entries: usize,
    /// Deprecation-packet rows.
    pub deprecation_packet_entries: usize,
    /// Schema/version-window rows.
    pub schema_version_window_entries: usize,
    /// Migration-publication rows.
    pub migration_publication_entries: usize,
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
    /// Number of finalize rules currently firing.
    pub rules_firing: usize,
}

/// The typed finalize register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeCompatibilityReportsDeprecationPacketsSchemaVersionWindows {
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
    /// Ref to the stable claim manifest this register ingests as its public-claim source and
    /// ceiling.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed finalize-kind vocabulary.
    pub kinds: Vec<FinalizeKind>,
    /// Closed finalize-state vocabulary.
    pub states: Vec<FinalizeState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed finalize-action vocabulary.
    pub actions: Vec<FinalizeAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Finalize rules.
    pub rules: Vec<FinalizeRule>,
    /// Finalize rows.
    pub rows: Vec<FinalizeRow>,
    /// Recorded publication verdict.
    pub publication: FinalizePublicationRecord,
    /// Summary counts.
    pub summary: FinalizeSummary,
}

impl FinalizeCompatibilityReportsDeprecationPacketsSchemaVersionWindows {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&FinalizeRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&FinalizeRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&FinalizeRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&FinalizeRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one finalize kind.
    pub fn rows_for_kind(&self, kind: FinalizeKind) -> Vec<&FinalizeRow> {
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
    pub fn rule_fires(&self, rule: &FinalizeRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and rules.
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

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim is
    /// already canonically narrowed is not a *finalize* blocker, it merely inherits the
    /// upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
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

    /// Recomputes the summary block from the rows and rules.
    pub fn computed_summary(&self) -> FinalizeSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: FinalizeKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&FinalizeRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        FinalizeSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_finalized_stable: self
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
                .filter(|row| row.finalize_state == FinalizeState::FinalizedOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_finalized_stable: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            compatibility_report_entries: kind(FinalizeKind::CompatibilityReport),
            deprecation_packet_entries: kind(FinalizeKind::DeprecationPacket),
            schema_version_window_entries: kind(FinalizeKind::SchemaVersionWindow),
            migration_publication_entries: kind(FinalizeKind::MigrationPublication),
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

    /// Produces an export/Help-About-safe projection of the register that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> FinalizeExportProjection {
        FinalizeExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| FinalizeExportRow {
                    entry_id: row.entry_id.clone(),
                    kind: row.kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    publishes_stable: row.publishes_stable(),
                    finalize_state: row.finalize_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<FinalizeViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(FinalizeViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(FinalizeViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(FinalizeViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<FinalizeViolation>) {
        if self.schema_version != FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_SCHEMA_VERSION {
            violations.push(FinalizeViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_RECORD_KIND
        {
            violations.push(FinalizeViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(FinalizeViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(FinalizeViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.kinds != FinalizeKind::ALL.to_vec() {
            violations.push(FinalizeViolation::ClosedVocabularyMismatch { field: "kinds" });
        }
        if self.states != FinalizeState::ALL.to_vec() {
            violations.push(FinalizeViolation::ClosedVocabularyMismatch { field: "states" });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(FinalizeViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.actions != FinalizeAction::ALL.to_vec() {
            violations.push(FinalizeViolation::ClosedVocabularyMismatch { field: "actions" });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(FinalizeViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(FinalizeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(FinalizeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(FinalizeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(FinalizeViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<FinalizeViolation>) {
        if self.rules.is_empty() {
            violations.push(FinalizeViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(FinalizeViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(FinalizeViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(FinalizeViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(FinalizeViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &FinalizeRow, violations: &mut Vec<FinalizeViolation>) {
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
                violations.push(FinalizeViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(FinalizeViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.effective_label,
            });
        }

        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(FinalizeViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(FinalizeViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // Kind-specific completeness checks.
        if let Some(packet) = &row.compatibility_report {
            if packet.report_ref.trim().is_empty() {
                violations.push(FinalizeViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "compatibility_report.report_ref",
                });
            }
            if packet.validity_window.window_days == 0 {
                violations.push(FinalizeViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "compatibility_report.validity_window.window_days",
                });
            }
        }
        if let Some(detail) = &row.deprecation_detail {
            if detail.replacement_path.trim().is_empty() {
                violations.push(FinalizeViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "deprecation_detail.replacement_path",
                });
            }
            if detail.removal_checkpoint.trim().is_empty() {
                violations.push(FinalizeViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "deprecation_detail.removal_checkpoint",
                });
            }
        }
        if let Some(detail) = &row.migration_detail {
            if detail.migration_guide_ref.trim().is_empty() {
                violations.push(FinalizeViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "migration_detail.migration_guide_ref",
                });
            }
        }

        // Coherence: a narrowed state must have an active gap reason, and a held label must not.
        if row.holds_label() {
            if row.effective_label != row.claim_label {
                violations.push(FinalizeViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.effective_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(FinalizeViolation::HeldWithActiveReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row.proof_packet.slo_state != FreshnessSloState::Current {
                violations.push(FinalizeViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state: row.proof_packet.slo_state,
                });
            }
            if !row.owner_signoff.signed_off {
                violations.push(FinalizeViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            if row.effective_label.rank() >= row.claim_label.rank() {
                violations.push(FinalizeViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.finalize_state,
                    published: row.effective_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(FinalizeViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.finalize_state,
                });
            }
        }

        if row.finalize_state == FinalizeState::FinalizedOnWaiver && row.waiver.is_none() {
            violations.push(FinalizeViolation::OnWaiverWithoutWaiver {
                entry_id: row.entry_id.clone(),
            });
        }

        if row.finalize_state == FinalizeState::NarrowedStale
            && !row.has_active_reason(GapReason::FreezePacketFreshnessBreached)
        {
            violations.push(FinalizeViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.finalize_state,
                expected_reason: GapReason::FreezePacketFreshnessBreached,
            });
        }

        if row.finalize_state == FinalizeState::NarrowedEvidenceIncomplete
            && !row.has_active_reason(GapReason::EvidenceIncomplete)
        {
            violations.push(FinalizeViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.finalize_state,
                expected_reason: GapReason::EvidenceIncomplete,
            });
        }

        if row.finalize_state == FinalizeState::NarrowedDeprecationOverdue
            && !row.has_active_reason(GapReason::DeprecationRemovalOverdue)
        {
            violations.push(FinalizeViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                state: row.finalize_state,
                expected_reason: GapReason::DeprecationRemovalOverdue,
            });
        }

        if row.release_blocking
            && !self
                .release_blocking_surface_refs
                .contains(&row.surface_ref)
        {
            violations.push(FinalizeViolation::ReleaseBlockingSurfaceNotDeclared {
                entry_id: row.entry_id.clone(),
                surface_ref: row.surface_ref.clone(),
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<FinalizeViolation>) {
        for kind in FinalizeKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(FinalizeViolation::KindUncovered { kind });
            }
        }
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &self.release_blocking_surface_refs {
            if !covered.contains(declared.as_str()) {
                violations.push(FinalizeViolation::ReleaseBlockingSurfaceUncovered {
                    surface_ref: declared.clone(),
                });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<FinalizeViolation>) {
        if self.publication.decision != self.computed_publication_decision() {
            violations.push(FinalizeViolation::PublicationDecisionInconsistent {
                declared: self.publication.decision,
                computed: self.computed_publication_decision(),
            });
        }
        let computed_blocking_rule_ids = self.computed_blocking_rule_ids();
        let computed_blocking_entry_ids = self.computed_blocking_entry_ids();
        if self.publication.blocking_rule_ids != computed_blocking_rule_ids {
            violations.push(FinalizeViolation::BlockingRuleIdsMismatch {
                recorded: self.publication.blocking_rule_ids.clone(),
                computed: computed_blocking_rule_ids,
            });
        }
        if self.publication.blocking_entry_ids != computed_blocking_entry_ids {
            violations.push(FinalizeViolation::BlockingEntryIdsMismatch {
                recorded: self.publication.blocking_entry_ids.clone(),
                computed: computed_blocking_entry_ids,
            });
        }
    }
}

/// Export/Help-About-safe projection of one finalize row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The kind of finalize subject.
    pub kind: FinalizeKind,
    /// The surface id.
    pub surface_ref: String,
    /// Whether the row is release-blocking.
    pub release_blocking: bool,
    /// The claim ref.
    pub claim_ref: String,
    /// The canonical claim label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// Whether the effective label is at or above the cutline.
    pub publishes_stable: bool,
    /// The finalize state.
    pub finalize_state: FinalizeState,
    /// The freshness-SLO state of the proof packet.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
}

/// Export/Help-About-safe projection of the finalize register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeExportProjection {
    /// Stable register id.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// The publication decision.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<FinalizeExportRow>,
}

/// Every structural or logical violation the validation can detect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeViolation {
    /// The schema version does not match the supported version.
    UnsupportedSchemaVersion { actual: u32 },
    /// The record kind does not match the expected kind.
    UnsupportedRecordKind { actual: String },
    /// A required string field is empty or missing.
    EmptyField {
        entry_id: String,
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the canonical set.
    ClosedVocabularyMismatch { field: &'static str },
    /// No rules are defined.
    NoRules,
    /// A rule id appears more than once.
    DuplicateRuleId { rule_id: String },
    /// A rule watches no labels.
    RuleWithoutLabels { rule_id: String },
    /// A gap reason has no rule that covers it.
    GapReasonWithoutRule { reason: GapReason },
    /// A row id appears more than once.
    DuplicateEntryId { entry_id: String },
    /// The register has no rows.
    EmptyRegister,
    /// A kind has no covering row.
    KindUncovered { kind: FinalizeKind },
    /// A declared release-blocking surface ref has no covering row.
    ReleaseBlockingSurfaceUncovered { surface_ref: String },
    /// A row's effective label is wider than its claim's canonical label.
    PublishedWiderThanClaim {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
    },
    /// A row's freshness SLO window is inconsistent.
    FreshnessSloInconsistent { entry_id: String },
    /// A row holds its label but the label does not equal the claim.
    HeldLabelNotEqualClaim {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
    },
    /// A row holds its label but has active gap reasons.
    HeldWithActiveReason { entry_id: String },
    /// A row holds its label but its packet is not fresh.
    HeldOnStalePacket {
        entry_id: String,
        slo_state: FreshnessSloState,
    },
    /// A row holds its label but lacks owner sign-off.
    HeldWithoutSignoff { entry_id: String },
    /// A row is narrowed but its published label is not narrowed.
    PublishedLabelNotNarrowed {
        entry_id: String,
        state: FinalizeState,
        published: StableClaimLevel,
    },
    /// A row is narrowed but has no active gap reason.
    NarrowingWithoutReason {
        entry_id: String,
        state: FinalizeState,
    },
    /// A row is on waiver but has no waiver record.
    OnWaiverWithoutWaiver { entry_id: String },
    /// A row's state does not match its expected active gap reason.
    StateReasonMismatch {
        entry_id: String,
        state: FinalizeState,
        expected_reason: GapReason,
    },
    /// A release-blocking row references a surface not declared in the register.
    ReleaseBlockingSurfaceNotDeclared {
        entry_id: String,
        surface_ref: String,
    },
    /// The stored publication decision does not match the computed decision.
    PublicationDecisionInconsistent {
        declared: PromotionDecision,
        computed: PromotionDecision,
    },
    /// The stored blocking rule ids do not match the computed ids.
    BlockingRuleIdsMismatch {
        recorded: Vec<String>,
        computed: Vec<String>,
    },
    /// The stored blocking entry ids do not match the computed ids.
    BlockingEntryIdsMismatch {
        recorded: Vec<String>,
        computed: Vec<String>,
    },
    /// The summary counts do not match the computed counts.
    SummaryMismatch,
}

impl fmt::Display for FinalizeViolation {
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
            } => {
                write!(f, "field {field_name} is empty on {entry_id}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch for {field}")
            }
            Self::NoRules => write!(f, "register has no rules"),
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutRule { reason } => {
                write!(f, "gap reason {} has no rule", reason.as_str())
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::KindUncovered { kind } => {
                write!(f, "kind {} has no covering row", kind.as_str())
            }
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering row"
                )
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
                write!(
                    f,
                    "row {entry_id} holds its label but has active gap reasons"
                )
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
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
                    "row {entry_id} is in state {state:?} but lacks expected reason {expected_reason:?}"
                )
            }
            Self::ReleaseBlockingSurfaceNotDeclared {
                entry_id,
                surface_ref,
            } => {
                write!(
                    f,
                    "row {entry_id} is release-blocking but its surface {surface_ref} is not in release_blocking_surface_refs"
                )
            }
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication decision {declared:?} does not match computed {computed:?}"
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
            Self::SummaryMismatch => write!(f, "summary does not match computed summary"),
        }
    }
}

impl Error for FinalizeViolation {}

/// Loads the embedded finalize register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`FinalizeCompatibilityReportsDeprecationPacketsSchemaVersionWindows`] — including when a
/// row carries a lifecycle label, finalize kind, finalize state, freshness-SLO state, gap
/// reason, or finalize action outside the closed vocabularies.
pub fn current_finalize_compatibility_reports_deprecation_packets_schema_version_windows(
) -> Result<FinalizeCompatibilityReportsDeprecationPacketsSchemaVersionWindows, serde_json::Error> {
    serde_json::from_str(
        FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> FinalizeCompatibilityReportsDeprecationPacketsSchemaVersionWindows {
        current_finalize_compatibility_reports_deprecation_packets_schema_version_windows()
            .expect("register parses")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let reg = register();
        assert_eq!(
            reg.schema_version,
            FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_SCHEMA_VERSION
        );
        assert_eq!(
            reg.record_kind,
            FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_RECORD_KIND
        );
        assert_eq!(reg.validate(), Vec::new());
        assert!(!reg.rows.is_empty());
    }

    #[test]
    fn every_finalize_kind_is_covered() {
        let reg = register();
        for kind in FinalizeKind::ALL {
            assert!(
                !reg.rows_for_kind(kind).is_empty(),
                "finalize kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_release_blocking_surface_is_covered() {
        let reg = register();
        let covered: BTreeSet<&str> = reg
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.as_str())
            .collect();
        assert!(!reg.release_blocking_surface_refs.is_empty());
        for declared in &reg.release_blocking_surface_refs {
            assert!(
                covered.contains(declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn register_exercises_published_and_narrowed_rows() {
        let reg = register();
        assert!(
            !reg.rows_published_stable().is_empty(),
            "register must show at least one published-stable row"
        );
        assert!(
            !reg.rows_narrowed().is_empty(),
            "register must show at least one narrowed row"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let reg = register();
        assert_eq!(reg.summary, reg.computed_summary());
        assert_eq!(
            reg.summary.entries_finalized_stable + reg.summary.entries_narrowed_below_cutline,
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
            reg.summary.compatibility_report_entries
                + reg.summary.deprecation_packet_entries
                + reg.summary.schema_version_window_entries
                + reg.summary.migration_publication_entries,
            reg.rows.len()
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let reg = register();
        assert_eq!(reg.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            reg.publication.decision,
            reg.computed_publication_decision()
        );
        assert!(!reg.publication.blocking_rule_ids.is_empty());
        assert!(!reg.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let reg = register();
        let covered: BTreeSet<GapReason> =
            reg.rules.iter().map(|rule| rule.trigger_reason).collect();
        for reason in GapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_publishes_wider_than_its_claim_ceiling() {
        let reg = register();
        for row in &reg.rows {
            assert!(
                row.effective_label.rank() <= row.claim_label.rank(),
                "{} publishes wider than its ceiling",
                row.entry_id
            );
        }
    }

    #[test]
    fn validate_flags_a_publication_wider_than_ceiling() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| !row.publishes_stable() && row.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.effective_label = StableClaimLevel::Stable;
        let entry_id = row.entry_id.clone();
        reg.summary = reg.computed_summary();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            FinalizeViolation::PublishedWiderThanClaim { entry_id: id, .. } if *id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.finalize_state == FinalizeState::NarrowedStale)
            .expect("a narrowed-stale row exists");
        row.effective_label = row.claim_label;
        reg.summary = reg.computed_summary();
        reg.publication.decision = reg.computed_publication_decision();
        reg.publication.blocking_rule_ids = reg.computed_blocking_rule_ids();
        reg.publication.blocking_entry_ids = reg.computed_blocking_entry_ids();
        assert!(reg
            .validate()
            .iter()
            .any(|v| matches!(v, FinalizeViolation::PublishedLabelNotNarrowed { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut reg = register();
        reg.publication.decision = PromotionDecision::Proceed;
        assert!(reg
            .validate()
            .iter()
            .any(|v| matches!(v, FinalizeViolation::PublicationDecisionInconsistent { .. })));
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
            .contains(&FinalizeViolation::HeldWithoutSignoff { entry_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let reg = register();
        let projection = reg.support_export_projection();
        assert_eq!(projection.rows.len(), reg.rows.len());
        assert_eq!(projection.publication_decision, reg.publication.decision);
        for (row, projected) in reg.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.surface_ref, projected.surface_ref);
            assert_eq!(row.publishes_stable(), projected.publishes_stable);
            assert_eq!(row.effective_label, projected.effective_label);
            assert_eq!(row.proof_packet.slo_state, projected.slo_state);
        }
    }
}
