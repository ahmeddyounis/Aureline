//! Typed stable version-window freeze for the CLI, schema, API, and manifest
//! surfaces of the release line, with deprecation packets.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes, and the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each
//! launch-blocking *requirement* is proven. Neither freezes the question this
//! module answers: **which version window does each public interface surface
//! commit to for the release line, and is that window actually frozen — backed by
//! a fresh freeze packet, a complete deprecation packet, and an owner sign-off?**
//! This module is the **version-window freeze**. For every public interface
//! surface — a CLI command surface, a wire/state schema, an API, or a manifest
//! format — it records one row that pins the surface's stable version window
//! (floor, current, ceiling, compatibility posture), the deprecation packet that
//! governs how older versions leave the window, the freeze packet that proves the
//! freeze, the waiver (if any) holding it provisionally, and the public claim (a
//! stable-claim-manifest entry) whose lifecycle label the freeze backs.
//!
//! Each [`WindowRow`] is one `(surface, public claim)` binding. It:
//!
//! - names the interface surface it freezes ([`WindowRow::surface_kind`],
//!   [`WindowRow::surface_ref`], [`WindowRow::surface_summary`]) and whether that
//!   surface is part of the frozen release line ([`WindowRow::release_blocking`]);
//! - pins the stable version window ([`VersionWindow`]) and the deprecation packet
//!   ([`DeprecationPacket`]) that governs how versions exit it;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose
//!   public claim it backs ([`WindowRow::claim_ref`]) and the canonical lifecycle
//!   label that entry publishes ([`WindowRow::claim_label`]). That label is a hard
//!   **ceiling**: a window may be frozen at the claim's label or narrowed below it,
//!   but it may never assert a freeze wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-surface
//!   labels, so docs, Help/About, the release center, and support exports ingest
//!   one label per surface instead of cloning their own;
//! - carries a freeze packet with a packet-freshness SLO ([`ProofPacket`]), the
//!   freeze state earned ([`WindowState`]), the active gap reasons ([`GapReason`]),
//!   and the label it *effectively* freezes after narrowing
//!   ([`WindowRow::frozen_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a surface whose freeze backs a Stable public claim and one narrowed
//! below it. A surface that is not frozen — because its freeze packet aged out or
//! is missing, because its deprecation packet is incomplete or carries a removal
//! that is overdue, because its waiver expired, because its surface evidence is
//! incomplete, or because the public claim it backs is itself below the cutline —
//! is structurally required to drop below the cutline rather than inherit an
//! adjacent green surface. The [`FreezeRule`] set names the closed conditions that
//! gate publication, and [`StableVersionWindows::publication`] records the
//! resulting proceed/hold verdict.
//!
//! The freeze is checked in at `artifacts/release/stable_version_windows.json` and
//! embedded here, so this typed consumer and the CI gate agree on every row without
//! a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Three
//! classes of check live outside this model because they need more than the freeze
//! sees: date arithmetic (recomputing the packet-freshness state, waiver expiry,
//! and deprecation-removal-overdue against an `as_of` date) and the cross-artifact
//! ceiling check (whether each row's `claim_label` still equals the label the
//! stable claim manifest publishes for the entry named by `claim_ref`). Those live
//! in the CI gate. This model enforces the structural and logical invariants that
//! hold regardless of the clock and the neighbouring artifact — the ceiling/no-
//! widening rule, version-window ordering, deprecation-packet completeness,
//! narrowing consistency, packet/state coherence, owner sign-off on frozen rows,
//! surface-kind and release-line coverage, publication-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported freeze schema version.
pub const STABLE_VERSION_WINDOWS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the freeze.
pub const STABLE_VERSION_WINDOWS_RECORD_KIND: &str = "stable_version_windows";

/// Repo-relative path to the checked-in freeze.
pub const STABLE_VERSION_WINDOWS_PATH: &str = "artifacts/release/stable_version_windows.json";

/// Embedded checked-in freeze JSON.
pub const STABLE_VERSION_WINDOWS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stable_version_windows.json"
));

/// The interface surface kind a version window freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// A command-line interface surface (command grammar, flags, exit codes).
    Cli,
    /// A wire or on-disk schema surface.
    Schema,
    /// A programmatic API surface.
    Api,
    /// A manifest-format surface (update, deploy, package manifests).
    Manifest,
}

impl SurfaceKind {
    /// Every surface kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Cli, Self::Schema, Self::Api, Self::Manifest];

    /// Stable token recorded in the freeze.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::Schema => "schema",
            Self::Api => "api",
            Self::Manifest => "manifest",
        }
    }
}

/// The compatibility posture a frozen version window commits to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityPosture {
    /// New versions in the window accept inputs valid for older versions.
    BackwardCompatible,
    /// Only additive (non-breaking) changes are admitted within the window.
    AdditiveOnly,
    /// The surface is pinned: no changes are admitted within the window.
    FrozenNoChange,
    /// Breaking changes are admitted only across a major bump (outside the window).
    BreakingMajorOnly,
}

impl CompatibilityPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::BackwardCompatible,
        Self::AdditiveOnly,
        Self::FrozenNoChange,
        Self::BreakingMajorOnly,
    ];

    /// Stable token recorded in the freeze.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackwardCompatible => "backward_compatible",
            Self::AdditiveOnly => "additive_only",
            Self::FrozenNoChange => "frozen_no_change",
            Self::BreakingMajorOnly => "breaking_major_only",
        }
    }
}

/// The lifecycle status of one deprecation notice in a deprecation packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprecationStatus {
    /// The version is deprecated and announced; removal is scheduled.
    Announced,
    /// A migration path is published and available; removal is scheduled.
    MigrationAvailable,
    /// The deprecated version has been removed from the window.
    Removed,
}

impl DeprecationStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 3] = [Self::Announced, Self::MigrationAvailable, Self::Removed];

    /// Stable token recorded in the freeze.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Announced => "announced",
            Self::MigrationAvailable => "migration_available",
            Self::Removed => "removed",
        }
    }

    /// Whether the deprecated version is still present in the window (not removed).
    pub const fn still_present(self) -> bool {
        !matches!(self, Self::Removed)
    }
}

/// Freeze state a surface earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowState {
    /// The surface is frozen: a captured, within-SLO freeze packet and a complete
    /// deprecation packet back the public claim at its full canonical lifecycle
    /// label, owner-signed.
    Frozen,
    /// The surface freezes the claim's full label only because an active, unexpired
    /// waiver covers a recorded freeze gap.
    FrozenOnWaiver,
    /// The freeze packet or surface evidence is incomplete, the deprecation packet
    /// is incomplete, or owner sign-off is absent; the surface is not frozen and
    /// the label must narrow.
    UnfrozenUnbacked,
    /// The public claim this surface backs is itself below the cutline, so the
    /// freeze inherits that ceiling and narrows.
    UnfrozenClaimNarrowed,
    /// The freeze packet breached its freshness SLO (or is missing); the surface is
    /// not frozen and the label must narrow.
    UnfrozenStale,
    /// The surface relied on a waiver that has expired; the label must narrow.
    UnfrozenWaiverExpired,
    /// A deprecation in the packet passed its removal target without removal; the
    /// window cannot freeze until the removal lands and the label must narrow.
    UnfrozenDeprecationOverdue,
}

impl WindowState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Frozen,
        Self::FrozenOnWaiver,
        Self::UnfrozenUnbacked,
        Self::UnfrozenClaimNarrowed,
        Self::UnfrozenStale,
        Self::UnfrozenWaiverExpired,
        Self::UnfrozenDeprecationOverdue,
    ];

    /// Stable token recorded in the freeze.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Frozen => "frozen",
            Self::FrozenOnWaiver => "frozen_on_waiver",
            Self::UnfrozenUnbacked => "unfrozen_unbacked",
            Self::UnfrozenClaimNarrowed => "unfrozen_claim_narrowed",
            Self::UnfrozenStale => "unfrozen_stale",
            Self::UnfrozenWaiverExpired => "unfrozen_waiver_expired",
            Self::UnfrozenDeprecationOverdue => "unfrozen_deprecation_overdue",
        }
    }

    /// Whether the state lets a surface freeze the public claim at its label.
    pub const fn holds_freeze(self) -> bool {
        matches!(self, Self::Frozen | Self::FrozenOnWaiver)
    }

    /// Whether the state forces the surface below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_freeze()
    }
}

/// Closed reason a freeze narrows or a freeze rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this surface backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The surface names a version or capability the build does not yet implement.
    SurfaceCapabilityAbsent,
    /// The freeze packet's surface-level evidence (version inventory) is incomplete.
    FreezeEvidenceIncomplete,
    /// The deprecation packet is missing a required field for a deprecated version.
    DeprecationPacketIncomplete,
    /// A deprecation's removal target date passed without the removal landing.
    DeprecationRemovalOverdue,
    /// The freeze packet breached its freshness SLO.
    FreezePacketFreshnessBreached,
    /// No freeze packet has been captured for the surface.
    FreezePacketMissing,
    /// A waiver the freeze relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ClaimLabelNarrowed,
        Self::SurfaceCapabilityAbsent,
        Self::FreezeEvidenceIncomplete,
        Self::DeprecationPacketIncomplete,
        Self::DeprecationRemovalOverdue,
        Self::FreezePacketFreshnessBreached,
        Self::FreezePacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the freeze.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::SurfaceCapabilityAbsent => "surface_capability_absent",
            Self::FreezeEvidenceIncomplete => "freeze_evidence_incomplete",
            Self::DeprecationPacketIncomplete => "deprecation_packet_incomplete",
            Self::DeprecationRemovalOverdue => "deprecation_removal_overdue",
            Self::FreezePacketFreshnessBreached => "freeze_packet_freshness_breached",
            Self::FreezePacketMissing => "freeze_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a freeze rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the surface's frozen lifecycle label below the cutline.
    NarrowWindowLabel,
    /// Refresh the freeze packet so it re-enters its freshness SLO.
    RefreshFreezePacket,
    /// Complete the deprecation packet (migration path, removal target, removal).
    CompleteDeprecationPacket,
    /// Recapture the surface-level version inventory the freeze packet depends on.
    RecaptureSurfaceEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl WindowAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowWindowLabel,
        Self::RefreshFreezePacket,
        Self::CompleteDeprecationPacket,
        Self::RecaptureSurfaceEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the freeze.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowWindowLabel => "narrow_window_label",
            Self::RefreshFreezePacket => "refresh_freeze_packet",
            Self::CompleteDeprecationPacket => "complete_deprecation_packet",
            Self::RecaptureSurfaceEvidence => "recapture_surface_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The frozen stable version window for a surface: floor, current, and ceiling
/// versions plus the compatibility posture the window commits to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionWindow {
    /// Oldest version still supported within the stable window.
    pub floor_version: String,
    /// The current frozen version for the release line.
    pub current_version: String,
    /// Newest version the window admits before a window-widening bump.
    pub ceiling_version: String,
    /// The compatibility posture the window commits to.
    pub compatibility_posture: CompatibilityPosture,
}

impl VersionWindow {
    /// True when `floor <= current <= ceiling` under dotted-numeric ordering.
    ///
    /// Versions that are not dotted-numeric cannot be ordered structurally, so the
    /// check passes for them and the gate is responsible for any registry-specific
    /// ordering rule.
    pub fn is_ordered(&self) -> bool {
        match (
            parse_version(&self.floor_version),
            parse_version(&self.current_version),
            parse_version(&self.ceiling_version),
        ) {
            (Some(floor), Some(current), Some(ceiling)) => floor <= current && current <= ceiling,
            _ => true,
        }
    }
}

/// One deprecation notice in a surface's deprecation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeprecationNotice {
    /// The version being deprecated.
    pub deprecated_version: String,
    /// The version (or surface) that supersedes the deprecated one.
    pub superseded_by: String,
    /// UTC date the deprecation was announced.
    pub announced_at: String,
    /// The version at which the deprecated version is removed.
    pub removal_target_version: String,
    /// UTC date by which the removal is targeted.
    pub removal_target_date: String,
    /// Ref to the migration guidance for moving off the deprecated version.
    pub migration_ref: String,
    /// The deprecation's lifecycle status.
    pub status: DeprecationStatus,
}

impl DeprecationNotice {
    /// True when every required field is present, so the notice gives a complete
    /// migration story (replacement, announcement, removal target, migration ref).
    pub fn is_complete(&self) -> bool {
        !self.deprecated_version.trim().is_empty()
            && !self.superseded_by.trim().is_empty()
            && !self.announced_at.trim().is_empty()
            && !self.removal_target_version.trim().is_empty()
            && !self.removal_target_date.trim().is_empty()
            && !self.migration_ref.trim().is_empty()
    }
}

/// The deprecation packet governing how older versions leave a surface's window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeprecationPacket {
    /// Stable deprecation-packet id.
    pub packet_id: String,
    /// Deprecation notices, one per deprecated version. May be empty.
    #[serde(default)]
    pub deprecations: Vec<DeprecationNotice>,
}

impl DeprecationPacket {
    /// True when every notice in the packet is complete.
    pub fn is_complete(&self) -> bool {
        self.deprecations.iter().all(DeprecationNotice::is_complete)
    }

    /// True when at least one notice is incomplete.
    pub fn has_incomplete_notice(&self) -> bool {
        !self.is_complete()
    }

    /// True when at least one notice is still present in the window (not removed).
    pub fn has_active_notice(&self) -> bool {
        self.deprecations.iter().any(|n| n.status.still_present())
    }
}

/// One freeze rule: a closed condition that narrows a window label and may gate
/// publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreezeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: WindowAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One stable version-window row: a `(surface, public claim)` binding bound to its
/// version window, deprecation packet, freeze packet, canonical ceiling label, and
/// packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WindowRow {
    /// Stable window-row id.
    pub window_id: String,
    /// Human-readable title.
    pub title: String,
    /// The interface surface kind this row freezes.
    pub surface_kind: SurfaceKind,
    /// The interface surface id this row freezes.
    pub surface_ref: String,
    /// Reviewable one-line statement of the surface.
    pub surface_summary: String,
    /// Whether the surface is part of the frozen release line.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this freeze backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a
    /// freeze may never assert a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Freeze state earned for the surface.
    pub window_state: WindowState,
    /// The pinned stable version window.
    pub version_window: VersionWindow,
    /// The freeze packet and its freshness SLO.
    pub freeze_packet: ProofPacket,
    /// The deprecation packet governing the window.
    pub deprecation_packet: DeprecationPacket,
    /// Waiver authorizing a provisional freeze, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the freeze effectively backs after narrowing.
    pub frozen_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl WindowRow {
    /// True when the frozen label is at or above the cutline.
    pub fn freezes_stable(&self) -> bool {
        self.frozen_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the surface freeze its claimed label.
    pub fn holds_freeze(&self) -> bool {
        self.window_state.holds_freeze()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// The recorded publication verdict for the stable version-window freeze.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreezePublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Freeze-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Window-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_window_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the freeze.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableVersionWindowsSummary {
    /// Total number of window rows.
    pub total_surfaces: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows freezing a label at or above the cutline.
    pub surfaces_frozen_stable: usize,
    /// Rows narrowed below the cutline.
    pub surfaces_narrowed_below_cutline: usize,
    /// Rows holding a freeze via an active waiver.
    pub surfaces_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows freezing a label at or above the cutline.
    pub release_blocking_frozen_stable: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_unfrozen: usize,
    /// CLI surfaces.
    pub cli_surfaces: usize,
    /// Schema surfaces.
    pub schema_surfaces: usize,
    /// API surfaces.
    pub api_surfaces: usize,
    /// Manifest surfaces.
    pub manifest_surfaces: usize,
    /// Freeze packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Freeze packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Freeze packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Freeze packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total deprecation notices across all packets.
    pub total_deprecations: usize,
    /// Rows whose state is `unfrozen_deprecation_overdue`.
    pub surfaces_deprecation_overdue: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of freeze rules currently firing.
    pub freeze_rules_firing: usize,
}

/// The typed stable version-window freeze.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableVersionWindows {
    /// Freeze schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable freeze identifier.
    pub freeze_id: String,
    /// Lifecycle status of this freeze artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this freeze ingests as its public-claim
    /// source and ceiling.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<SurfaceKind>,
    /// Closed compatibility-posture vocabulary.
    pub compatibility_postures: Vec<CompatibilityPosture>,
    /// Closed deprecation-status vocabulary.
    pub deprecation_statuses: Vec<DeprecationStatus>,
    /// Closed window-state vocabulary.
    pub window_states: Vec<WindowState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed window-action vocabulary.
    pub window_actions: Vec<WindowAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-line surface refs this freeze must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Freeze rules.
    pub freeze_rules: Vec<FreezeRule>,
    /// Window rows.
    pub rows: Vec<WindowRow>,
    /// Recorded publication verdict.
    pub publication: FreezePublicationRecord,
    /// Summary counts.
    pub summary: StableVersionWindowsSummary,
}

impl StableVersionWindows {
    /// Returns the row registered for `window_id`.
    pub fn row(&self, window_id: &str) -> Option<&WindowRow> {
        self.rows.iter().find(|row| row.window_id == window_id)
    }

    /// Returns the rows freezing a label at or above the cutline.
    pub fn rows_frozen_stable(&self) -> Vec<&WindowRow> {
        self.rows
            .iter()
            .filter(|row| row.freezes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&WindowRow> {
        self.rows
            .iter()
            .filter(|row| !row.freezes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&WindowRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one surface kind.
    pub fn rows_for_kind(&self, kind: SurfaceKind) -> Vec<&WindowRow> {
        self.rows
            .iter()
            .filter(|row| row.surface_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the freeze covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn freeze_rule_fires(&self, rule: &FreezeRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and freeze rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .freeze_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.freeze_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .freeze_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.freeze_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Window-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose
    /// claim is already canonically narrowed is not a *freeze* blocker, it merely
    /// inherits the upstream ceiling.
    pub fn computed_blocking_window_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .freeze_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.freeze_rule_fires(rule))
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
                ids.insert(row.window_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and freeze rules.
    pub fn computed_summary(&self) -> StableVersionWindowsSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.freeze_packet.slo_state == state)
                .count()
        };
        let kind = |kind: SurfaceKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&WindowRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        StableVersionWindowsSummary {
            total_surfaces: self.rows.len(),
            total_claims: self.claims().len(),
            surfaces_frozen_stable: self.rows.iter().filter(|row| row.freezes_stable()).count(),
            surfaces_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.freezes_stable())
                .count(),
            surfaces_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.window_state == WindowState::FrozenOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_frozen_stable: release_blocking
                .iter()
                .filter(|row| row.freezes_stable())
                .count(),
            release_blocking_unfrozen: release_blocking
                .iter()
                .filter(|row| !row.freezes_stable())
                .count(),
            cli_surfaces: kind(SurfaceKind::Cli),
            schema_surfaces: kind(SurfaceKind::Schema),
            api_surfaces: kind(SurfaceKind::Api),
            manifest_surfaces: kind(SurfaceKind::Manifest),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_deprecations: self
                .rows
                .iter()
                .map(|row| row.deprecation_packet.deprecations.len())
                .sum(),
            surfaces_deprecation_overdue: self
                .rows
                .iter()
                .filter(|row| row.window_state == WindowState::UnfrozenDeprecationOverdue)
                .count(),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            freeze_rules_firing: self
                .freeze_rules
                .iter()
                .filter(|rule| self.freeze_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the freeze that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> VersionWindowExportProjection {
        VersionWindowExportProjection {
            freeze_id: self.freeze_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| VersionWindowExportRow {
                    window_id: row.window_id.clone(),
                    surface_kind: row.surface_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    frozen_label: row.frozen_label,
                    freezes_stable: row.freezes_stable(),
                    window_state: row.window_state,
                    floor_version: row.version_window.floor_version.clone(),
                    current_version: row.version_window.current_version.clone(),
                    ceiling_version: row.version_window.ceiling_version.clone(),
                    slo_state: row.freeze_packet.slo_state,
                    deprecation_count: row.deprecation_packet.deprecations.len(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the freeze, returning every violation found.
    pub fn validate(&self) -> Vec<StableVersionWindowsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.window_id.clone()) {
                violations.push(StableVersionWindowsViolation::DuplicateWindowId {
                    window_id: row.window_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(StableVersionWindowsViolation::EmptyFreeze);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(StableVersionWindowsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<StableVersionWindowsViolation>) {
        if self.schema_version != STABLE_VERSION_WINDOWS_SCHEMA_VERSION {
            violations.push(StableVersionWindowsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != STABLE_VERSION_WINDOWS_RECORD_KIND {
            violations.push(StableVersionWindowsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("freeze_id", &self.freeze_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableVersionWindowsViolation::EmptyField {
                    window_id: "<freeze>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.surface_kinds != SurfaceKind::ALL.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "surface_kinds",
            });
        }
        if self.compatibility_postures != CompatibilityPosture::ALL.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "compatibility_postures",
            });
        }
        if self.deprecation_statuses != DeprecationStatus::ALL.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "deprecation_statuses",
            });
        }
        if self.window_states != WindowState::ALL.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "window_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.window_actions != WindowAction::ALL.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "window_actions",
            });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(StableVersionWindowsViolation::EmptyField {
                window_id: "<freeze>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(StableVersionWindowsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(StableVersionWindowsViolation::EmptyField {
                window_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<StableVersionWindowsViolation>) {
        if self.freeze_rules.is_empty() {
            violations.push(StableVersionWindowsViolation::NoFreezeRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.freeze_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(StableVersionWindowsViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(StableVersionWindowsViolation::EmptyField {
                        window_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(StableVersionWindowsViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every gap reason must have a rule, so a gap reason cannot fire without a
        // corresponding publication gate.
        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(StableVersionWindowsViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &WindowRow, violations: &mut Vec<StableVersionWindowsViolation>) {
        for (field, value) in [
            ("window_id", &row.window_id),
            ("title", &row.title),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            (
                "version_window.floor_version",
                &row.version_window.floor_version,
            ),
            (
                "version_window.current_version",
                &row.version_window.current_version,
            ),
            (
                "version_window.ceiling_version",
                &row.version_window.ceiling_version,
            ),
            ("freeze_packet.packet_id", &row.freeze_packet.packet_id),
            ("freeze_packet.packet_ref", &row.freeze_packet.packet_ref),
            (
                "freeze_packet.proof_index_ref",
                &row.freeze_packet.proof_index_ref,
            ),
            (
                "freeze_packet.freshness_slo.slo_register_ref",
                &row.freeze_packet.freshness_slo.slo_register_ref,
            ),
            (
                "deprecation_packet.packet_id",
                &row.deprecation_packet.packet_id,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableVersionWindowsViolation::EmptyField {
                    window_id: row.window_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no freeze may back a label wider than the public claim's
        // canonical label.
        if row.frozen_label.rank() > row.claim_label.rank() {
            violations.push(StableVersionWindowsViolation::FrozenWiderThanClaim {
                window_id: row.window_id.clone(),
                claim: row.claim_label,
                frozen: row.frozen_label,
            });
        }

        // The version window must be ordered floor <= current <= ceiling.
        if !row.version_window.is_ordered() {
            violations.push(StableVersionWindowsViolation::VersionWindowDisordered {
                window_id: row.window_id.clone(),
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.freeze_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(StableVersionWindowsViolation::EmptyField {
                window_id: row.window_id.clone(),
                field_name: "freeze_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.freeze_packet.freshness_slo.window_is_consistent() {
            violations.push(StableVersionWindowsViolation::FreshnessSloInconsistent {
                window_id: row.window_id.clone(),
            });
        }

        self.validate_deprecation_packet(row, violations);

        // A public claim whose canonical label is below the cutline forces the
        // freeze to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_freeze() {
                violations.push(StableVersionWindowsViolation::HeldOnNarrowedClaim {
                    window_id: row.window_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                violations.push(StableVersionWindowsViolation::ClaimNarrowedWithoutReason {
                    window_id: row.window_id.clone(),
                });
            }
        }

        let slo_state = row.freeze_packet.slo_state;

        if row.holds_freeze() {
            // A frozen row backs exactly the public claim's canonical label, carries
            // no active gap reason, rides a captured within-SLO packet, has a complete
            // deprecation packet, and is owner-signed.
            if row.frozen_label != row.claim_label {
                violations.push(StableVersionWindowsViolation::HeldLabelNotEqualClaim {
                    window_id: row.window_id.clone(),
                    claim: row.claim_label,
                    frozen: row.frozen_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(StableVersionWindowsViolation::HeldWithActiveGap {
                    window_id: row.window_id.clone(),
                });
            }
            if !row.freeze_packet.has_capture() {
                violations.push(StableVersionWindowsViolation::HeldWithoutFreshPacket {
                    window_id: row.window_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(StableVersionWindowsViolation::HeldOnStalePacket {
                    window_id: row.window_id.clone(),
                    slo_state,
                });
            }
            if row.deprecation_packet.has_incomplete_notice() {
                violations.push(
                    StableVersionWindowsViolation::HeldWithIncompleteDeprecation {
                        window_id: row.window_id.clone(),
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(StableVersionWindowsViolation::HeldWithoutSignoff {
                    window_id: row.window_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the frozen label below the cutline and name
            // at least one active reason.
            if row.freezes_stable() {
                violations.push(StableVersionWindowsViolation::FrozenLabelNotNarrowed {
                    window_id: row.window_id.clone(),
                    state: row.window_state,
                    frozen: row.frozen_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(StableVersionWindowsViolation::NarrowingWithoutReason {
                    window_id: row.window_id.clone(),
                    state: row.window_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::FreezePacketFreshnessBreached)
            {
                violations.push(StableVersionWindowsViolation::BreachedPacketWithoutReason {
                    window_id: row.window_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::FreezePacketMissing)
            {
                violations.push(StableVersionWindowsViolation::MissingPacketWithoutReason {
                    window_id: row.window_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_deprecation_packet(
        &self,
        row: &WindowRow,
        violations: &mut Vec<StableVersionWindowsViolation>,
    ) {
        for (idx, notice) in row.deprecation_packet.deprecations.iter().enumerate() {
            // Every notice must at least name the deprecated version, so the packet
            // does not carry an anonymous deprecation.
            if notice.deprecated_version.trim().is_empty() {
                violations.push(StableVersionWindowsViolation::DeprecationMissingVersion {
                    window_id: row.window_id.clone(),
                    index: idx,
                });
            }
        }

        let has_incomplete = row.deprecation_packet.has_incomplete_notice();
        // A row carrying the incomplete-deprecation reason must actually have an
        // incomplete notice, and a row with an incomplete notice must name the reason
        // (so the deprecation-completeness automation stays honest).
        if row.has_active_reason(GapReason::DeprecationPacketIncomplete) && !has_incomplete {
            violations.push(
                StableVersionWindowsViolation::DeprecationReasonWithoutIncomplete {
                    window_id: row.window_id.clone(),
                },
            );
        }
        if has_incomplete && !row.has_active_reason(GapReason::DeprecationPacketIncomplete) {
            violations.push(
                StableVersionWindowsViolation::IncompleteDeprecationWithoutReason {
                    window_id: row.window_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &WindowRow,
        violations: &mut Vec<StableVersionWindowsViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<StableVersionWindowsViolation>,
                               expected: GapReason| {
            violations.push(StableVersionWindowsViolation::StateReasonIncoherent {
                window_id: row.window_id.clone(),
                state: row.window_state,
                expected_reason: expected,
            });
        };

        match row.window_state {
            WindowState::UnfrozenUnbacked => {
                const ALLOWED: [GapReason; 4] = [
                    GapReason::SurfaceCapabilityAbsent,
                    GapReason::FreezeEvidenceIncomplete,
                    GapReason::DeprecationPacketIncomplete,
                    GapReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, GapReason::FreezeEvidenceIncomplete);
                }
            }
            WindowState::UnfrozenClaimNarrowed => {
                if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, GapReason::ClaimLabelNarrowed);
                }
            }
            WindowState::UnfrozenStale => {
                if !(row.has_active_reason(GapReason::FreezePacketFreshnessBreached)
                    || row.has_active_reason(GapReason::FreezePacketMissing))
                {
                    push_incoherent(violations, GapReason::FreezePacketFreshnessBreached);
                }
            }
            WindowState::UnfrozenWaiverExpired => {
                if !row.has_active_reason(GapReason::WaiverExpired) {
                    push_incoherent(violations, GapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(StableVersionWindowsViolation::WaiverStateWithoutWaiver {
                        window_id: row.window_id.clone(),
                        state: row.window_state,
                    });
                }
            }
            WindowState::UnfrozenDeprecationOverdue => {
                if !row.has_active_reason(GapReason::DeprecationRemovalOverdue) {
                    push_incoherent(violations, GapReason::DeprecationRemovalOverdue);
                }
                // An overdue removal requires a deprecation notice still present in
                // the window to be overdue about.
                if !row.deprecation_packet.has_active_notice() {
                    violations.push(StableVersionWindowsViolation::OverdueStateWithoutNotice {
                        window_id: row.window_id.clone(),
                    });
                }
            }
            WindowState::FrozenOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(StableVersionWindowsViolation::WaiverStateWithoutWaiver {
                        window_id: row.window_id.clone(),
                        state: row.window_state,
                    });
                }
            }
            WindowState::Frozen => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<StableVersionWindowsViolation>) {
        // Each surface ref appears at most once: a surface has one canonical window
        // row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.surface_ref.as_str()) {
                violations.push(StableVersionWindowsViolation::DuplicateSurfaceRef {
                    surface_ref: row.surface_ref.clone(),
                });
            }
        }

        // The release line must freeze every declared release-blocking surface with
        // exactly one release-blocking row, and every release-blocking row must be
        // declared, so a surface cannot quietly drop out of the freeze.
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
                    StableVersionWindowsViolation::ReleaseBlockingRefWithoutRow {
                        surface_ref: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.surface_ref.as_str()) {
                violations.push(StableVersionWindowsViolation::ReleaseBlockingRowNotInSet {
                    window_id: row.window_id.clone(),
                    surface_ref: row.surface_ref.clone(),
                });
            }
        }

        // The freeze must cover all four surface kinds — CLI, schema, API, and
        // manifest — so the release line cannot freeze some surfaces and silently
        // leave a whole interface kind unfrozen.
        for kind in SurfaceKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(StableVersionWindowsViolation::SurfaceKindAbsent { kind });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<StableVersionWindowsViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(StableVersionWindowsViolation::EmptyField {
                window_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(StableVersionWindowsViolation::EmptyField {
                window_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                StableVersionWindowsViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                StableVersionWindowsViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_window_ids != self.computed_blocking_window_ids() {
            violations.push(
                StableVersionWindowsViolation::PublicationBlockingSetMismatch {
                    field: "blocking_window_ids",
                },
            );
        }
    }
}

/// Parses a dotted-numeric version such as `1.4.0` into comparable components.
///
/// Returns `None` when the version is empty or any component is not a base-10
/// unsigned integer, so callers can fall back to a registry-specific rule.
fn parse_version(value: &str) -> Option<Vec<u64>> {
    if value.trim().is_empty() {
        return None;
    }
    value
        .split('.')
        .map(|part| part.parse::<u64>().ok())
        .collect()
}

/// A redaction-safe export row projected from the freeze.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionWindowExportRow {
    /// Stable window-row id.
    pub window_id: String,
    /// Interface surface kind.
    pub surface_kind: SurfaceKind,
    /// Interface surface ref.
    pub surface_ref: String,
    /// Whether the surface is part of the frozen release line.
    pub release_blocking: bool,
    /// The public-claim entry ref the freeze backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the freeze backs.
    pub frozen_label: StableClaimLevel,
    /// Whether the row freezes a label at or above the cutline.
    pub freezes_stable: bool,
    /// Freeze state.
    pub window_state: WindowState,
    /// Window floor version.
    pub floor_version: String,
    /// Window current version.
    pub current_version: String,
    /// Window ceiling version.
    pub ceiling_version: String,
    /// Freeze-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Number of deprecation notices in the packet.
    pub deprecation_count: usize,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// A redaction-safe export projection of the freeze.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionWindowExportProjection {
    /// Freeze id this projection was produced from.
    pub freeze_id: String,
    /// Freeze as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<VersionWindowExportRow>,
}

/// A validation violation for the stable version-window freeze.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableVersionWindowsViolation {
    /// The freeze carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the freeze.
        actual: u32,
    },
    /// The freeze carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the freeze.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The freeze has no rows.
    EmptyFreeze,
    /// The freeze has no freeze rules.
    NoFreezeRules,
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        window_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A window id appears more than once.
    DuplicateWindowId {
        /// Duplicate window id.
        window_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A freeze rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: GapReason,
    },
    /// A frozen label is wider than the public claim's canonical label.
    FrozenWiderThanClaim {
        /// Row id.
        window_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Frozen label.
        frozen: StableClaimLevel,
    },
    /// A version window is not ordered floor <= current <= ceiling.
    VersionWindowDisordered {
        /// Row id.
        window_id: String,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Row id.
        window_id: String,
    },
    /// A deprecation notice does not name the deprecated version.
    DeprecationMissingVersion {
        /// Row id.
        window_id: String,
        /// Notice index within the packet.
        index: usize,
    },
    /// A row carries the incomplete-deprecation reason but has no incomplete notice.
    DeprecationReasonWithoutIncomplete {
        /// Row id.
        window_id: String,
    },
    /// A row has an incomplete deprecation notice but does not name the reason.
    IncompleteDeprecationWithoutReason {
        /// Row id.
        window_id: String,
    },
    /// A deprecation-overdue state names no still-present deprecation notice.
    OverdueStateWithoutNotice {
        /// Row id.
        window_id: String,
    },
    /// A row holds a freeze while the public claim's canonical label is narrowed.
    HeldOnNarrowedClaim {
        /// Row id.
        window_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
    },
    /// A row whose claim is narrowed does not carry the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Row id.
        window_id: String,
    },
    /// A narrowing state did not drop the frozen label below the cutline.
    FrozenLabelNotNarrowed {
        /// Row id.
        window_id: String,
        /// Freeze state.
        state: WindowState,
        /// Frozen label.
        frozen: StableClaimLevel,
    },
    /// A narrowing state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        window_id: String,
        /// Freeze state.
        state: WindowState,
    },
    /// A frozen row's frozen label is not equal to its claim ceiling label.
    HeldLabelNotEqualClaim {
        /// Row id.
        window_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Frozen label.
        frozen: StableClaimLevel,
    },
    /// A frozen row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        window_id: String,
    },
    /// A frozen row rides a freeze packet with no capture or evidence.
    HeldWithoutFreshPacket {
        /// Row id.
        window_id: String,
    },
    /// A frozen row rides a freeze packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        window_id: String,
        /// The packet's freshness-SLO state.
        slo_state: FreshnessSloState,
    },
    /// A frozen row carries an incomplete deprecation packet.
    HeldWithIncompleteDeprecation {
        /// Row id.
        window_id: String,
    },
    /// A frozen row has no owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        window_id: String,
    },
    /// A narrowing row with a breached packet does not name the breach reason.
    BreachedPacketWithoutReason {
        /// Row id.
        window_id: String,
    },
    /// A narrowing row with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        window_id: String,
    },
    /// A freeze state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        window_id: String,
        /// Freeze state.
        state: WindowState,
        /// Reason the state requires.
        expected_reason: GapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        window_id: String,
        /// Freeze state.
        state: WindowState,
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
        window_id: String,
        /// The row's surface ref.
        surface_ref: String,
    },
    /// A surface kind is not covered by any row.
    SurfaceKindAbsent {
        /// The uncovered surface kind.
        kind: SurfaceKind,
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

impl fmt::Display for StableVersionWindowsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported freeze schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported freeze record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "freeze {field} is not the canonical value")
            }
            Self::EmptyFreeze => write!(f, "freeze has no rows"),
            Self::NoFreezeRules => write!(f, "freeze has no freeze rules"),
            Self::EmptyField {
                window_id,
                field_name,
            } => write!(f, "{window_id} has empty field {field_name}"),
            Self::DuplicateWindowId { window_id } => {
                write!(f, "duplicate window row id {window_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate freeze rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "freeze rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::FrozenWiderThanClaim {
                window_id,
                claim,
                frozen,
            } => write!(
                f,
                "window {window_id} frozen label {} is wider than the claim ceiling {}",
                frozen.as_str(),
                claim.as_str()
            ),
            Self::VersionWindowDisordered { window_id } => write!(
                f,
                "window {window_id} version window is not ordered floor <= current <= ceiling"
            ),
            Self::FreshnessSloInconsistent { window_id } => write!(
                f,
                "window {window_id} freshness SLO warn window exceeds its target age"
            ),
            Self::DeprecationMissingVersion { window_id, index } => write!(
                f,
                "window {window_id} deprecation #{index} does not name the deprecated version"
            ),
            Self::DeprecationReasonWithoutIncomplete { window_id } => write!(
                f,
                "window {window_id} names deprecation_packet_incomplete but every notice is complete"
            ),
            Self::IncompleteDeprecationWithoutReason { window_id } => write!(
                f,
                "window {window_id} has an incomplete deprecation notice but does not name deprecation_packet_incomplete"
            ),
            Self::OverdueStateWithoutNotice { window_id } => write!(
                f,
                "window {window_id} is deprecation-overdue but names no still-present deprecation notice"
            ),
            Self::HeldOnNarrowedClaim { window_id, claim } => write!(
                f,
                "window {window_id} holds a freeze while the public claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { window_id } => write!(
                f,
                "window {window_id} backs a claim that is narrowed but does not name claim_label_narrowed"
            ),
            Self::FrozenLabelNotNarrowed {
                window_id,
                state,
                frozen,
            } => write!(
                f,
                "window {window_id} state {} must narrow below the cutline but freezes {}",
                state.as_str(),
                frozen.as_str()
            ),
            Self::NarrowingWithoutReason { window_id, state } => write!(
                f,
                "window {window_id} state {} narrows without naming an active gap reason",
                state.as_str()
            ),
            Self::HeldLabelNotEqualClaim {
                window_id,
                claim,
                frozen,
            } => write!(
                f,
                "window {window_id} freezes {} but its public claim label is {}",
                frozen.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { window_id } => write!(
                f,
                "window {window_id} freezes its label while a gap reason is active"
            ),
            Self::HeldWithoutFreshPacket { window_id } => write!(
                f,
                "window {window_id} freezes its label with no captured, evidence-backed freeze packet"
            ),
            Self::HeldOnStalePacket {
                window_id,
                slo_state,
            } => write!(
                f,
                "window {window_id} freezes its label while its packet is {} (outside its freshness SLO)",
                slo_state.as_str()
            ),
            Self::HeldWithIncompleteDeprecation { window_id } => write!(
                f,
                "window {window_id} freezes its label with an incomplete deprecation packet"
            ),
            Self::HeldWithoutSignoff { window_id } => {
                write!(f, "window {window_id} freezes its label without owner sign-off")
            }
            Self::BreachedPacketWithoutReason { window_id } => write!(
                f,
                "window {window_id} has a breached packet but does not name freeze_packet_freshness_breached"
            ),
            Self::MissingPacketWithoutReason { window_id } => write!(
                f,
                "window {window_id} has a missing packet but does not name freeze_packet_missing"
            ),
            Self::StateReasonIncoherent {
                window_id,
                state,
                expected_reason,
            } => write!(
                f,
                "window {window_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { window_id, state } => write!(
                f,
                "window {window_id} state {} names no waiver",
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
                window_id,
                surface_ref,
            } => write!(
                f,
                "window {window_id} is release-blocking but its surface {surface_ref} is not in release_blocking_surface_refs"
            ),
            Self::SurfaceKindAbsent { kind } => write!(
                f,
                "surface kind {} is not covered by any window row",
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
            Self::SummaryMismatch => write!(f, "freeze summary counts disagree with the rows"),
        }
    }
}

impl Error for StableVersionWindowsViolation {}

/// Loads the embedded stable version-window freeze.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in freeze no longer matches
/// [`StableVersionWindows`] — including when a row carries a lifecycle label,
/// surface kind, compatibility posture, deprecation status, freeze state, freshness-
/// SLO state, gap reason, or window action outside the closed vocabularies.
pub fn current_stable_version_windows() -> Result<StableVersionWindows, serde_json::Error> {
    serde_json::from_str(STABLE_VERSION_WINDOWS_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn freeze() -> StableVersionWindows {
        current_stable_version_windows().expect("freeze parses")
    }

    #[test]
    fn embedded_freeze_parses_and_validates() {
        let freeze = freeze();
        assert_eq!(freeze.schema_version, STABLE_VERSION_WINDOWS_SCHEMA_VERSION);
        assert_eq!(freeze.record_kind, STABLE_VERSION_WINDOWS_RECORD_KIND);
        assert_eq!(freeze.validate(), Vec::new());
        assert!(!freeze.rows.is_empty());
    }

    #[test]
    fn every_surface_kind_is_covered() {
        let freeze = freeze();
        for kind in SurfaceKind::ALL {
            assert!(
                !freeze.rows_for_kind(kind).is_empty(),
                "surface kind {} must have at least one window row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_release_blocking_surface_is_covered() {
        let freeze = freeze();
        let covered: BTreeSet<&str> = freeze
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.as_str())
            .collect();
        assert!(!freeze.release_blocking_surface_refs.is_empty());
        for declared in &freeze.release_blocking_surface_refs {
            assert!(
                covered.contains(declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn freeze_exercises_frozen_and_narrowed_rows() {
        let freeze = freeze();
        assert!(
            !freeze.rows_frozen_stable().is_empty(),
            "freeze must show at least one frozen-stable surface"
        );
        assert!(
            !freeze.rows_narrowed().is_empty(),
            "freeze must show at least one narrowed surface"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let freeze = freeze();
        assert_eq!(freeze.summary, freeze.computed_summary());
        assert_eq!(
            freeze.summary.surfaces_frozen_stable + freeze.summary.surfaces_narrowed_below_cutline,
            freeze.rows.len()
        );
        assert_eq!(
            freeze.summary.packets_current
                + freeze.summary.packets_due_for_refresh
                + freeze.summary.packets_breached
                + freeze.summary.packets_missing,
            freeze.rows.len()
        );
        assert_eq!(
            freeze.summary.cli_surfaces
                + freeze.summary.schema_surfaces
                + freeze.summary.api_surfaces
                + freeze.summary.manifest_surfaces,
            freeze.rows.len()
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let freeze = freeze();
        assert_eq!(freeze.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            freeze.publication.decision,
            freeze.computed_publication_decision()
        );
        assert!(!freeze.publication.blocking_rule_ids.is_empty());
        assert!(!freeze.publication.blocking_window_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let freeze = freeze();
        let covered: BTreeSet<GapReason> = freeze
            .freeze_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in GapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_freezes_wider_than_its_claim_ceiling() {
        let freeze = freeze();
        for row in &freeze.rows {
            assert!(
                row.frozen_label.rank() <= row.claim_label.rank(),
                "{} freezes wider than its ceiling",
                row.window_id
            );
        }
    }

    #[test]
    fn every_version_window_is_ordered() {
        let freeze = freeze();
        for row in &freeze.rows {
            assert!(
                row.version_window.is_ordered(),
                "{} version window is disordered",
                row.window_id
            );
        }
    }

    #[test]
    fn validate_flags_a_freeze_wider_than_ceiling() {
        let mut freeze = freeze();
        let row = freeze
            .rows
            .iter_mut()
            .find(|row| !row.freezes_stable() && row.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.frozen_label = StableClaimLevel::Stable;
        let window_id = row.window_id.clone();
        freeze.summary = freeze.computed_summary();
        assert!(freeze.validate().iter().any(|v| matches!(
            v,
            StableVersionWindowsViolation::FrozenWiderThanClaim { window_id: id, .. } if *id == window_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut freeze = freeze();
        let row = freeze
            .rows
            .iter_mut()
            .find(|row| row.window_state == WindowState::UnfrozenStale)
            .expect("an unfrozen-stale row exists");
        row.frozen_label = row.claim_label;
        freeze.summary = freeze.computed_summary();
        freeze.publication.decision = freeze.computed_publication_decision();
        freeze.publication.blocking_rule_ids = freeze.computed_blocking_rule_ids();
        freeze.publication.blocking_window_ids = freeze.computed_blocking_window_ids();
        assert!(freeze.validate().iter().any(|v| matches!(
            v,
            StableVersionWindowsViolation::FrozenLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_a_disordered_version_window() {
        let mut freeze = freeze();
        let row = freeze.rows.first_mut().expect("freeze has a row");
        row.version_window.floor_version = "9.9.9".to_owned();
        let window_id = row.window_id.clone();
        assert!(freeze.validate().iter().any(|v| matches!(
            v,
            StableVersionWindowsViolation::VersionWindowDisordered { window_id: id } if *id == window_id
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut freeze = freeze();
        freeze.publication.decision = PromotionDecision::Proceed;
        assert!(freeze.validate().iter().any(|v| matches!(
            v,
            StableVersionWindowsViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_frozen_row_without_signoff() {
        let mut freeze = freeze();
        let row = freeze
            .rows
            .iter_mut()
            .find(|row| row.holds_freeze())
            .expect("a frozen row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let window_id = row.window_id.clone();
        freeze.summary = freeze.computed_summary();
        assert!(freeze
            .validate()
            .contains(&StableVersionWindowsViolation::HeldWithoutSignoff { window_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let freeze = freeze();
        let projection = freeze.support_export_projection();
        assert_eq!(projection.rows.len(), freeze.rows.len());
        assert_eq!(projection.publication_decision, freeze.publication.decision);
        for (row, projected) in freeze.rows.iter().zip(&projection.rows) {
            assert_eq!(row.window_id, projected.window_id);
            assert_eq!(row.surface_ref, projected.surface_ref);
            assert_eq!(row.freezes_stable(), projected.freezes_stable);
            assert_eq!(row.frozen_label, projected.frozen_label);
            assert_eq!(row.freeze_packet.slo_state, projected.slo_state);
        }
    }
}
