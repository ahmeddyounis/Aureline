//! Typed accessibility surface signoff register for the M4 stable line.
//!
//! Where the [`stable_qualification_matrix`](crate::stable_qualification_matrix)
//! carries one cross-cutting accessibility lane, this register answers the
//! per-surface question: **for each touched surface — shell, tree, palette,
//! diff, terminal, debugger, settings, auth, and recovery — is the keyboard,
//! screen-reader, IME/grapheme/bidi, zoom, high-contrast, and reduced-motion
//! behavior signed off, and does any surface that loses qualification narrow
//! automatically instead of inheriting adjacent green rows?**
//!
//! Each [`AccessibilitySurfaceSignoffRow`] is one `(surface, public claim)`
//! binding. It:
//!
//! - names the surface it governs ([`AccessibilitySurfaceSignoffRow::surface_kind`],
//!   [`AccessibilitySurfaceSignoffRow::surface_ref`]) and whether that surface is
//!   part of the release-blocking set
//!   ([`AccessibilitySurfaceSignoffRow::release_blocking`]);
//! - pins the per-dimension checks ([`DimensionCheck`]) that validate keyboard,
//!   screen-reader, IME/grapheme/bidi, zoom, high-contrast, and reduced-motion
//!   behavior, each with its own evidence ref and state;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry
//!   whose public claim it backs and the canonical lifecycle label that entry
//!   publishes, so the surface may never assert a public claim wider than the
//!   claim it backs;
//! - records the signoff state earned ([`SignoffState`]), the active gap reasons
//!   ([`GapReason`]), and the label it *effectively* publishes after narrowing;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-surface
//!   labels, so docs, Help/About, the release center, and support exports ingest
//!   one label per surface instead of cloning their own.
//!
//! The register is checked in at
//! `artifacts/release/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported accessibility-surface-signoffs schema version.
pub const ACCESSIBILITY_SURFACE_SIGNOFFS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const ACCESSIBILITY_SURFACE_SIGNOFFS_RECORD_KIND: &str = "accessibility_surface_signoffs";

/// Repo-relative path to the checked-in register.
pub const ACCESSIBILITY_SURFACE_SIGNOFFS_PATH: &str =
    "artifacts/release/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery.json";

/// Embedded checked-in register JSON.
pub const ACCESSIBILITY_SURFACE_SIGNOFFS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery.json"
));

/// The IDE surface a signoff row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// The command shell / terminal input surface.
    Shell,
    /// The file explorer and outline tree views.
    Tree,
    /// The command palette and quick-pick surfaces.
    Palette,
    /// The diff / compare view.
    Diff,
    /// The integrated terminal pane.
    Terminal,
    /// The debugger panes and controls.
    Debugger,
    /// The settings editor.
    Settings,
    /// The authentication and account flows.
    Auth,
    /// The recovery and restore surfaces.
    Recovery,
}

impl SurfaceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Shell,
        Self::Tree,
        Self::Palette,
        Self::Diff,
        Self::Terminal,
        Self::Debugger,
        Self::Settings,
        Self::Auth,
        Self::Recovery,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Tree => "tree",
            Self::Palette => "palette",
            Self::Diff => "diff",
            Self::Terminal => "terminal",
            Self::Debugger => "debugger",
            Self::Settings => "settings",
            Self::Auth => "auth",
            Self::Recovery => "recovery",
        }
    }
}

/// The accessibility dimension a check validates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionKind {
    /// Full keyboard traversal, shortcuts, and focus behavior.
    Keyboard,
    /// Screen-reader announcements, roles, names, and live regions.
    ScreenReader,
    /// IME, grapheme cluster, bidirectional text, and complex-script behavior.
    ImeGraphemeBidi,
    /// Zoom resilience from 50 % to 400 %.
    Zoom,
    /// High-contrast mode support and non-color state cues.
    HighContrast,
    /// Reduced-motion respect and alternative transitions.
    ReducedMotion,
}

impl DimensionKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Keyboard,
        Self::ScreenReader,
        Self::ImeGraphemeBidi,
        Self::Zoom,
        Self::HighContrast,
        Self::ReducedMotion,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::ImeGraphemeBidi => "ime_grapheme_bidi",
            Self::Zoom => "zoom",
            Self::HighContrast => "high_contrast",
            Self::ReducedMotion => "reduced_motion",
        }
    }
}

/// The state earned for one dimension on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionState {
    /// The dimension passes all criteria on current evidence.
    Passed,
    /// The dimension passes core criteria but has known minor degradations.
    Degraded,
    /// The dimension partially passes; some paths or platforms are not yet covered.
    Partial,
    /// The dimension is blocked by a missing dependency or upstream gap.
    Blocked,
    /// Evidence has not yet been captured for this dimension.
    PendingEvidence,
}

impl DimensionState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Passed,
        Self::Degraded,
        Self::Partial,
        Self::Blocked,
        Self::PendingEvidence,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Degraded => "degraded",
            Self::Partial => "partial",
            Self::Blocked => "blocked",
            Self::PendingEvidence => "pending_evidence",
        }
    }

    /// Whether the dimension is qualified enough to support a Stable claim.
    pub const fn supports_stable(self) -> bool {
        matches!(self, Self::Passed | Self::Degraded)
    }
}

/// The overall signoff state a surface row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffState {
    /// All dimensions pass or degrade gracefully on current, owner-signed evidence.
    Qualified,
    /// The surface carries the claim's full label only because an active, unexpired
    /// waiver covers a recorded gap.
    ProvisionalOnWaiver,
    /// At least one dimension is blocked or missing evidence; the label must narrow.
    NotQualified,
    /// The proof packet breached its freshness SLO; the label must narrow.
    EvidenceStale,
    /// The surface relied on a waiver that has expired; the label must narrow.
    WaiverExpired,
}

impl SignoffState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Qualified,
        Self::ProvisionalOnWaiver,
        Self::NotQualified,
        Self::EvidenceStale,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ProvisionalOnWaiver => "provisional_on_waiver",
            Self::NotQualified => "not_qualified",
            Self::EvidenceStale => "evidence_stale",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether the state lets a surface carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Qualified | Self::ProvisionalOnWaiver)
    }

    /// Whether the state forces the surface below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a surface signoff narrows or a rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this surface backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// At least one dimension is blocked, preventing full qualification.
    DimensionBlocked,
    /// The proof packet breached its freshness SLO.
    EvidenceStale,
    /// No proof packet has been captured for the surface.
    EvidenceMissing,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
    /// A waiver the surface relied on has expired.
    WaiverExpired,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ClaimLabelNarrowed,
        Self::DimensionBlocked,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::DimensionBlocked => "dimension_blocked",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }
}

/// Default action a signoff rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the surface's published lifecycle label below the cutline.
    NarrowClaim,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshEvidencePacket,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl SignoffAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HoldPromotion,
        Self::NarrowClaim,
        Self::RefreshEvidencePacket,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowClaim => "narrow_claim",
            Self::RefreshEvidencePacket => "refresh_evidence_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One dimension check for a surface signoff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionCheck {
    /// The accessibility dimension this check validates.
    pub dimension: DimensionKind,
    /// The state earned for this dimension.
    pub dimension_state: DimensionState,
    /// Ref to the evidence backing this dimension, or null when pending.
    #[serde(default)]
    pub evidence_ref: Option<String>,
    /// Optional reviewable note.
    #[serde(default)]
    pub notes: Option<String>,
}

/// One signoff rule: a closed condition that narrows a surface label and may gate
/// promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessibilitySurfaceSignoffRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: SignoffAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One accessibility surface signoff row: a `(surface, public claim)` binding bound
/// to its per-dimension checks, proof packet, canonical ceiling label, and
/// packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessibilitySurfaceSignoffRow {
    /// Stable signoff-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The surface this row governs.
    pub surface_kind: SurfaceKind,
    /// The surface id this signoff speaks about.
    pub surface_ref: String,
    /// Whether the surface is part of the release-blocking signoff set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this signoff backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a
    /// signoff may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Signoff state earned for the row.
    pub signoff_state: SignoffState,
    /// Per-dimension accessibility checks.
    pub dimension_checks: Vec<DimensionCheck>,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional signoff, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the signoff effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl AccessibilitySurfaceSignoffRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the signoff carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.signoff_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when every dimension check supports a Stable claim.
    pub fn dimensions_support_stable(&self) -> bool {
        self.dimension_checks
            .iter()
            .all(|check| check.dimension_state.supports_stable())
    }

    /// True when at least one dimension is blocked or pending evidence.
    pub fn has_blocked_or_pending_dimension(&self) -> bool {
        self.dimension_checks.iter().any(|check| {
            matches!(
                check.dimension_state,
                DimensionState::Blocked | DimensionState::PendingEvidence
            )
        })
    }
}

/// The recorded promotion verdict for the accessibility surface signoff register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionRecord {
    /// The gate this verdict governs.
    pub promotion_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Signoff-rule ids that block promotion, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Signoff-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessibilitySurfaceSignoffsSummary {
    /// Total number of signoff rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_qualified: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Rows with at least one blocked dimension.
    pub entries_blocked: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_qualified: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Shell signoff rows.
    pub shell_entries: usize,
    /// Tree signoff rows.
    pub tree_entries: usize,
    /// Palette signoff rows.
    pub palette_entries: usize,
    /// Diff signoff rows.
    pub diff_entries: usize,
    /// Terminal signoff rows.
    pub terminal_entries: usize,
    /// Debugger signoff rows.
    pub debugger_entries: usize,
    /// Settings signoff rows.
    pub settings_entries: usize,
    /// Auth signoff rows.
    pub auth_entries: usize,
    /// Recovery signoff rows.
    pub recovery_entries: usize,
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
    /// Number of signoff rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilitySurfaceSignoffExportRow {
    /// Stable signoff-row id.
    pub entry_id: String,
    /// The surface this row governs.
    pub surface_kind: SurfaceKind,
    /// The surface id this signoff speaks about.
    pub surface_ref: String,
    /// Whether the surface is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this signoff backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Signoff state earned.
    pub signoff_state: SignoffState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilitySurfaceSignoffExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<AccessibilitySurfaceSignoffExportRow>,
}

/// The typed accessibility surface signoff register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessibilitySurfaceSignoffs {
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
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<SurfaceKind>,
    /// Closed dimension-kind vocabulary.
    pub dimension_kinds: Vec<DimensionKind>,
    /// Closed signoff-state vocabulary.
    pub signoff_states: Vec<SignoffState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed signoff-action vocabulary.
    pub signoff_actions: Vec<SignoffAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Signoff rules.
    pub rules: Vec<AccessibilitySurfaceSignoffRule>,
    /// Signoff rows.
    pub rows: Vec<AccessibilitySurfaceSignoffRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionRecord,
    /// Summary counts.
    pub summary: AccessibilitySurfaceSignoffsSummary,
}


impl AccessibilitySurfaceSignoffs {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&AccessibilitySurfaceSignoffRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&AccessibilitySurfaceSignoffRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&AccessibilitySurfaceSignoffRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&AccessibilitySurfaceSignoffRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one surface kind.
    pub fn rows_for_kind(&self, kind: SurfaceKind) -> Vec<&AccessibilitySurfaceSignoffRow> {
        self.rows
            .iter()
            .filter(|row| row.surface_kind == kind)
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
    pub fn rule_fires(&self, rule: &AccessibilitySurfaceSignoffRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the rows and signoff rules.
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

    /// Signoff-row ids that trigger a blocking, firing rule, sorted and unique.
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

    /// Recomputes the summary block from the rows and signoff rules.
    pub fn computed_summary(&self) -> AccessibilitySurfaceSignoffsSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: SurfaceKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&AccessibilitySurfaceSignoffRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        AccessibilitySurfaceSignoffsSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_qualified: self
                .rows
                .iter()
                .filter(|row| row.signoff_state == SignoffState::Qualified)
                .count(),
            entries_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.signoff_state == SignoffState::ProvisionalOnWaiver)
                .count(),
            entries_blocked: self
                .rows
                .iter()
                .filter(|row| row.has_blocked_or_pending_dimension())
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_qualified: release_blocking
                .iter()
                .filter(|row| row.signoff_state == SignoffState::Qualified)
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            shell_entries: kind(SurfaceKind::Shell),
            tree_entries: kind(SurfaceKind::Tree),
            palette_entries: kind(SurfaceKind::Palette),
            diff_entries: kind(SurfaceKind::Diff),
            terminal_entries: kind(SurfaceKind::Terminal),
            debugger_entries: kind(SurfaceKind::Debugger),
            settings_entries: kind(SurfaceKind::Settings),
            auth_entries: kind(SurfaceKind::Auth),
            recovery_entries: kind(SurfaceKind::Recovery),
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
    pub fn support_export_projection(&self) -> AccessibilitySurfaceSignoffExportProjection {
        AccessibilitySurfaceSignoffExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| AccessibilitySurfaceSignoffExportRow {
                    entry_id: row.entry_id.clone(),
                    surface_kind: row.surface_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    signoff_state: row.signoff_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<AccessibilitySurfaceSignoffsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(AccessibilitySurfaceSignoffsViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(AccessibilitySurfaceSignoffsViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(AccessibilitySurfaceSignoffsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<AccessibilitySurfaceSignoffsViolation>,
    ) {
        if self.schema_version != ACCESSIBILITY_SURFACE_SIGNOFFS_SCHEMA_VERSION {
            violations.push(
                AccessibilitySurfaceSignoffsViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != ACCESSIBILITY_SURFACE_SIGNOFFS_RECORD_KIND {
            violations.push(
                AccessibilitySurfaceSignoffsViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(AccessibilitySurfaceSignoffsViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.surface_kinds != SurfaceKind::ALL.to_vec() {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "surface_kinds",
            });
        }
        if self.dimension_kinds != DimensionKind::ALL.to_vec() {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "dimension_kinds",
            });
        }
        if self.signoff_states != SignoffState::ALL.to_vec() {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "signoff_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.signoff_actions != SignoffAction::ALL.to_vec() {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "signoff_actions",
            });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(AccessibilitySurfaceSignoffsViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(AccessibilitySurfaceSignoffsViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(AccessibilitySurfaceSignoffsViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(
        &self,
        violations: &mut Vec<AccessibilitySurfaceSignoffsViolation>,
    ) {
        if self.rules.is_empty() {
            violations.push(AccessibilitySurfaceSignoffsViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(AccessibilitySurfaceSignoffsViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(AccessibilitySurfaceSignoffsViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(AccessibilitySurfaceSignoffsViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(AccessibilitySurfaceSignoffsViolation::GapReasonWithoutRule {
                    reason,
                });
            }
        }
    }

    fn validate_row(
        &self,
        row: &AccessibilitySurfaceSignoffRow,
        violations: &mut Vec<AccessibilitySurfaceSignoffsViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("surface_ref", &row.surface_ref),
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
                violations.push(AccessibilitySurfaceSignoffsViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no signoff may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(
                AccessibilitySurfaceSignoffsViolation::PublishedWiderThanClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                },
            );
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(AccessibilitySurfaceSignoffsViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(
                AccessibilitySurfaceSignoffsViolation::FreshnessSloInconsistent {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // Every dimension kind must appear exactly once.
        let mut seen_dims: BTreeSet<DimensionKind> = BTreeSet::new();
        for check in &row.dimension_checks {
            if !seen_dims.insert(check.dimension) {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::DuplicateDimension {
                        entry_id: row.entry_id.clone(),
                        dimension: check.dimension,
                    },
                );
            }
        }
        for dim in DimensionKind::ALL {
            if !seen_dims.contains(&dim) {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::DimensionMissing {
                        entry_id: row.entry_id.clone(),
                        dimension: dim,
                    },
                );
            }
        }

        self.validate_dimensions(row, violations);

        // A public claim whose canonical label is below the cutline forces the signoff
        // to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::HeldOnNarrowedClaim {
                        entry_id: row.entry_id.clone(),
                        claim: row.claim_label,
                    },
                );
            }
            if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::ClaimNarrowedWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed row carries exactly the public claim's canonical label, carries
            // no active gap reason, rides a captured within-SLO packet, has all
            // dimensions supporting stable (passed or degraded), and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::HeldLabelNotEqualClaim {
                        entry_id: row.entry_id.clone(),
                        claim: row.claim_label,
                        published: row.published_label,
                    },
                );
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::HeldWithActiveGap {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !row.proof_packet.has_capture() {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::HeldWithoutFreshPacket {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !slo_state.is_within_slo() {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::HeldOnStalePacket {
                        entry_id: row.entry_id.clone(),
                        slo_state,
                    },
                );
            }
            if row.has_blocked_or_pending_dimension() {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::HeldWithBlockedDimension {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::HeldWithoutSignoff {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if row.publishes_stable() {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        state: row.signoff_state,
                        published: row.published_label,
                    },
                );
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::NarrowingWithoutReason {
                        entry_id: row.entry_id.clone(),
                        state: row.signoff_state,
                    },
                );
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::EvidenceStale)
            {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::EvidenceMissing)
            {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            // A narrowing row with a blocked or pending dimension must name the dimension_blocked reason.
            if row.has_blocked_or_pending_dimension()
                && !row.has_active_reason(GapReason::DimensionBlocked)
            {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::BlockedDimensionWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_dimensions(
        &self,
        row: &AccessibilitySurfaceSignoffRow,
        violations: &mut Vec<AccessibilitySurfaceSignoffsViolation>,
    ) {
        for check in &row.dimension_checks {
            // A dimension in pending_evidence or blocked state must not claim an
            // evidence ref, while passed/degraded/partial may optionally carry one.
            match check.dimension_state {
                DimensionState::PendingEvidence | DimensionState::Blocked => {
                    if check.evidence_ref.is_some() {
                        violations.push(
                            AccessibilitySurfaceSignoffsViolation::UnexpectedEvidenceRef {
                                entry_id: row.entry_id.clone(),
                                dimension: check.dimension,
                                dimension_state: check.dimension_state,
                            },
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &AccessibilitySurfaceSignoffRow,
        violations: &mut Vec<AccessibilitySurfaceSignoffsViolation>,
    ) {
        let push_incoherent =
            |violations: &mut Vec<AccessibilitySurfaceSignoffsViolation>, expected: GapReason| {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::StateReasonIncoherent {
                        entry_id: row.entry_id.clone(),
                        state: row.signoff_state,
                        expected_reason: expected,
                    },
                );
            };

        match row.signoff_state {
            SignoffState::NotQualified => {
                if !(row.has_active_reason(GapReason::DimensionBlocked)
                    || row.has_active_reason(GapReason::EvidenceMissing))
                {
                    push_incoherent(violations, GapReason::DimensionBlocked);
                }
            }
            SignoffState::EvidenceStale => {
                if !(row.has_active_reason(GapReason::EvidenceStale)
                    || row.has_active_reason(GapReason::EvidenceMissing))
                {
                    push_incoherent(violations, GapReason::EvidenceStale);
                }
            }
            SignoffState::WaiverExpired => {
                if !row.has_active_reason(GapReason::WaiverExpired) {
                    push_incoherent(violations, GapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(
                        AccessibilitySurfaceSignoffsViolation::WaiverStateWithoutWaiver {
                            entry_id: row.entry_id.clone(),
                            state: row.signoff_state,
                        },
                    );
                }
            }
            SignoffState::ProvisionalOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        AccessibilitySurfaceSignoffsViolation::WaiverStateWithoutWaiver {
                            entry_id: row.entry_id.clone(),
                            state: row.signoff_state,
                        },
                    );
                }
            }
            SignoffState::Qualified => {}
        }
    }

    fn validate_coverage(
        &self,
        violations: &mut Vec<AccessibilitySurfaceSignoffsViolation>,
    ) {
        // Each surface ref appears at most once: a surface has one canonical row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.surface_ref.as_str()) {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::DuplicateSurfaceRef {
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
                    AccessibilitySurfaceSignoffsViolation::ReleaseBlockingRefWithoutRow {
                        surface_ref: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.surface_ref.as_str()) {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::ReleaseBlockingRowNotInSet {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        // Every surface kind must have at least one row.
        for kind in SurfaceKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(
                    AccessibilitySurfaceSignoffsViolation::SurfaceKindAbsent { kind },
                );
            }
        }
    }

    fn validate_promotion(
        &self,
        violations: &mut Vec<AccessibilitySurfaceSignoffsViolation>,
    ) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(AccessibilitySurfaceSignoffsViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(AccessibilitySurfaceSignoffsViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                AccessibilitySurfaceSignoffsViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                AccessibilitySurfaceSignoffsViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                AccessibilitySurfaceSignoffsViolation::PromotionBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// Validation failure emitted while checking an accessibility surface signoff register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessibilitySurfaceSignoffsViolation {
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
    /// A dimension appears more than once on a row.
    DuplicateDimension {
        /// Row id.
        entry_id: String,
        /// Duplicated dimension.
        dimension: DimensionKind,
    },
    /// A required dimension is missing from a row.
    DimensionMissing {
        /// Row id.
        entry_id: String,
        /// Missing dimension.
        dimension: DimensionKind,
    },
    /// A dimension in pending/blocked state carries an evidence ref.
    UnexpectedEvidenceRef {
        /// Row id.
        entry_id: String,
        /// Dimension.
        dimension: DimensionKind,
        /// Dimension state.
        dimension_state: DimensionState,
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
    /// A backed row has a dimension that does not support stable.
    HeldWithBlockedDimension {
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
        /// The row's signoff state.
        state: SignoffState,
        /// The published label.
        published: StableClaimLevel,
    },
    /// A narrowing row carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// The row's signoff state.
        state: SignoffState,
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
    /// A row has a blocked dimension without the dimension-blocked reason.
    BlockedDimensionWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row's state and active reasons are incoherent.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// The row's signoff state.
        state: SignoffState,
        /// The expected gap reason.
        expected_reason: GapReason,
    },
    /// A row names a waiver state but carries no waiver packet.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// The row's signoff state.
        state: SignoffState,
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
    /// A surface kind has no covering row.
    SurfaceKindAbsent {
        /// Missing kind.
        kind: SurfaceKind,
    },
    /// The promotion decision disagrees with the computed decision.
    PromotionDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The promotion blocking set disagrees with the computed one.
    PromotionBlockingSetMismatch {
        /// Field name that mismatched.
        field: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for AccessibilitySurfaceSignoffsViolation {
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
                "row {entry_id} publishes {} wider than claim {}",
                published.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} has an inconsistent freshness SLO")
            }
            Self::DuplicateDimension {
                entry_id,
                dimension,
            } => write!(
                f,
                "row {entry_id} has duplicate dimension {}",
                dimension.as_str()
            ),
            Self::DimensionMissing {
                entry_id,
                dimension,
            } => write!(
                f,
                "row {entry_id} is missing dimension {}",
                dimension.as_str()
            ),
            Self::UnexpectedEvidenceRef {
                entry_id,
                dimension,
                dimension_state,
            } => write!(
                f,
                "row {entry_id} dimension {} in state {} must not carry an evidence ref",
                dimension.as_str(),
                dimension_state.as_str()
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => {
                write!(
                    f,
                    "row {entry_id} holds its label while its claim is {}",
                    claim.as_str()
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
                "row {entry_id} holds its label but publishes {} instead of {}",
                published.as_str(),
                claim.as_str()
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
            Self::HeldWithBlockedDimension { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds its label while a dimension is blocked or pending"
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
                    "row {entry_id} has a breached packet without the evidence_stale reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} has a missing packet without the evidence_missing reason"
                )
            }
            Self::BlockedDimensionWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} has a blocked dimension without the dimension_blocked reason"
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
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
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
            Self::SurfaceKindAbsent { kind } => {
                write!(f, "surface kind {} is covered by no row", kind.as_str())
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
                write!(f, "promotion {field} disagrees with the firing rules")
            }
            Self::SummaryMismatch => {
                write!(f, "register summary counts disagree with the rows")
            }
        }
    }
}

impl Error for AccessibilitySurfaceSignoffsViolation {}

/// Loads the embedded accessibility surface signoff register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`AccessibilitySurfaceSignoffs`].
pub fn current_accessibility_surface_signoffs() -> Result<AccessibilitySurfaceSignoffs, serde_json::Error> {
    serde_json::from_str(ACCESSIBILITY_SURFACE_SIGNOFFS_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> AccessibilitySurfaceSignoffs {
        current_accessibility_surface_signoffs()
            .expect("checked-in accessibility register parses into the model")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let reg = register();
        assert_eq!(reg.schema_version, ACCESSIBILITY_SURFACE_SIGNOFFS_SCHEMA_VERSION);
        assert_eq!(reg.record_kind, ACCESSIBILITY_SURFACE_SIGNOFFS_RECORD_KIND);
        assert_eq!(reg.validate(), Vec::new());
        assert!(!reg.rows.is_empty());
    }

    #[test]
    fn covers_every_surface_kind() {
        let reg = register();
        for kind in SurfaceKind::ALL {
            assert!(
                !reg.rows_for_kind(kind).is_empty(),
                "surface kind {} must have at least one row",
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
            reg.summary.entries_qualified
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
            reg.summary.shell_entries
                + reg.summary.tree_entries
                + reg.summary.palette_entries
                + reg.summary.diff_entries
                + reg.summary.terminal_entries
                + reg.summary.debugger_entries
                + reg.summary.settings_entries
                + reg.summary.auth_entries
                + reg.summary.recovery_entries,
            reg.rows.len()
        );
    }

    #[test]
    fn promotion_holds_when_blocking_rules_fire() {
        let reg = register();
        assert_eq!(reg.promotion.decision, PromotionDecision::Hold);
        assert_eq!(reg.promotion.decision, reg.computed_promotion_decision());
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
            .find(|row| !row.publishes_stable())
            .expect("a narrowed row exists");
        row.claim_label = StableClaimLevel::Beta;
        row.published_label = StableClaimLevel::Stable;
        let entry_id = row.entry_id.clone();
        reg.summary = reg.computed_summary();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            AccessibilitySurfaceSignoffsViolation::PublishedWiderThanClaim { entry_id: id, .. } if *id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.signoff_state == SignoffState::EvidenceStale)
            .expect("a stale row exists");
        row.published_label = row.claim_label;
        reg.summary = reg.computed_summary();
        reg.promotion.decision = reg.computed_promotion_decision();
        reg.promotion.blocking_rule_ids = reg.computed_blocking_rule_ids();
        reg.promotion.blocking_entry_ids = reg.computed_blocking_entry_ids();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            AccessibilitySurfaceSignoffsViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_a_backed_row_with_blocked_dimension() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.signoff_state == SignoffState::Qualified)
            .expect("a qualified row exists");
        for check in &mut row.dimension_checks {
            if check.dimension == DimensionKind::ScreenReader {
                check.dimension_state = DimensionState::Blocked;
                check.evidence_ref = None;
                break;
            }
        }
        reg.summary = reg.computed_summary();
        assert!(reg
            .validate()
            .iter()
            .any(|v| matches!(v, AccessibilitySurfaceSignoffsViolation::HeldWithBlockedDimension { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_promotion_decision() {
        let mut reg = register();
        reg.promotion.decision = PromotionDecision::Proceed;
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            AccessibilitySurfaceSignoffsViolation::PromotionDecisionInconsistent { .. }
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
            .contains(&AccessibilitySurfaceSignoffsViolation::HeldWithoutSignoff { entry_id }));
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
                && row.signoff_state != SignoffState::Qualified
        });
        assert!(
            narrowed.is_some(),
            "the register must narrow at least one release-blocking row under a still-stable claim"
        );
    }

    #[test]
    fn register_shows_a_blocked_dimension() {
        let reg = register();
        let blocked = reg
            .rows
            .iter()
            .find(|row| !row.dimensions_support_stable());
        assert!(
            blocked.is_some(),
            "the register must show at least one row with a blocked or pending dimension"
        );
    }

    #[test]
    fn validate_flags_missing_dimension() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.signoff_state == SignoffState::Qualified)
            .expect("a qualified row exists");
        row.dimension_checks.retain(|c| c.dimension != DimensionKind::ReducedMotion);
        reg.summary = reg.computed_summary();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            AccessibilitySurfaceSignoffsViolation::DimensionMissing { dimension: DimensionKind::ReducedMotion, .. }
        )));
    }
}
