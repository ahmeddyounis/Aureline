//! Typed M5 browser, companion, and embedded-boundary manifest with handoff
//! eligibility rows.
//!
//! This module generates the canonical manifest that governs handoff eligibility
//! between browser surfaces, companion surfaces, and embedded-boundary surfaces.
//! Each [`M5HandoffEligibilityRow`] binds one source surface family to a target
//! surface family and records:
//!
//! - the handoff state earned ([`HandoffEligibilityState`]),
//! - the active gap reasons ([`HandoffGapReason`]) that explain a narrowed or
//!   blocked handoff,
//! - the proof packet, owner sign-off, and optional waiver reused from the
//!   stable-claim vocabulary,
//! - the effective lifecycle label after narrowing.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a handoff that may publish as Stable and one that must
//! narrow below it. The [`HandoffStopRule`] set names the closed conditions that
//! gate publication, and [`M5BrowserCompanionEmbeddedBoundaryManifest::publication`]
//! records the proceed/hold verdict.
//!
//! The manifest is checked in at
//! `artifacts/release/m5/generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows.json`
//! and embedded here, so the typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported manifest schema version.
pub const GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the manifest.
pub const GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_RECORD_KIND: &str =
    "generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows";

/// Repo-relative path to the checked-in manifest.
pub const GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_PATH: &str =
    "artifacts/release/m5/generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows.json";

/// Embedded checked-in manifest JSON.
pub const GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows.json"
    ));

/// Surface family a handoff row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffSurfaceKind {
    /// Browser surface family.
    Browser,
    /// Companion surface family.
    Companion,
    /// Embedded-boundary surface family.
    EmbeddedBoundary,
}

impl HandoffSurfaceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::Browser, Self::Companion, Self::EmbeddedBoundary];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Browser => "browser",
            Self::Companion => "companion",
            Self::EmbeddedBoundary => "embedded_boundary",
        }
    }
}

/// Handoff eligibility state a row earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffEligibilityState {
    /// Fully eligible for handoff at the effective label.
    Eligible,
    /// Eligible with a known degradation (for example, on a waiver).
    EligibleDegraded,
    /// Blocked from handoff at the effective label.
    Blocked,
    /// Pending evidence; handoff is gated until the corpus is refreshed.
    PendingEvidence,
    /// Explicitly not applicable for this pair.
    NotApplicable,
}

impl HandoffEligibilityState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Eligible,
        Self::EligibleDegraded,
        Self::Blocked,
        Self::PendingEvidence,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eligible => "eligible",
            Self::EligibleDegraded => "eligible_degraded",
            Self::Blocked => "blocked",
            Self::PendingEvidence => "pending_evidence",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the state lets the row carry its claim at the effective label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Eligible | Self::EligibleDegraded)
    }

    /// Whether the state forces the row below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a handoff row narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffGapReason {
    /// Proof packet is missing.
    ProofPacketMissing,
    /// Proof packet is stale.
    ProofPacketStale,
    /// Compatibility report is missing.
    CompatibilityReportMissing,
    /// Compatibility report is stale.
    CompatibilityReportStale,
    /// Desktop handoff truth is incomplete.
    DesktopHandoffTruthIncomplete,
    /// Browser fallback path is unavailable.
    BrowserFallbackUnavailable,
    /// Owner sign-off is missing.
    OwnerSignoffMissing,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// Source surface has narrowed below the cutline.
    SourceSurfaceNarrowed,
    /// Target surface has narrowed below the cutline.
    TargetSurfaceNarrowed,
}

impl HandoffGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::CompatibilityReportMissing,
        Self::CompatibilityReportStale,
        Self::DesktopHandoffTruthIncomplete,
        Self::BrowserFallbackUnavailable,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
        Self::SourceSurfaceNarrowed,
        Self::TargetSurfaceNarrowed,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::CompatibilityReportMissing => "compatibility_report_missing",
            Self::CompatibilityReportStale => "compatibility_report_stale",
            Self::DesktopHandoffTruthIncomplete => "desktop_handoff_truth_incomplete",
            Self::BrowserFallbackUnavailable => "browser_fallback_unavailable",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::SourceSurfaceNarrowed => "source_surface_narrowed",
            Self::TargetSurfaceNarrowed => "target_surface_narrowed",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Refresh the compatibility report.
    RefreshCompatibilityReport,
    /// Complete the desktop handoff truth.
    CompleteDesktopHandoffTruth,
    /// Restore the browser fallback path.
    RestoreBrowserFallback,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl HandoffAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RefreshCompatibilityReport,
        Self::CompleteDesktopHandoffTruth,
        Self::RestoreBrowserFallback,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RefreshCompatibilityReport => "refresh_compatibility_report",
            Self::CompleteDesktopHandoffTruth => "complete_desktop_handoff_truth",
            Self::RestoreBrowserFallback => "restore_browser_fallback",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One handoff stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: HandoffGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: HandoffAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 handoff eligibility row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5HandoffEligibilityRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The source surface family.
    pub source_kind: HandoffSurfaceKind,
    /// The target surface family.
    pub target_kind: HandoffSurfaceKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the row.
    pub surface_summary: String,
    /// Whether the row is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// Handoff eligibility state earned for the row.
    pub handoff_state: HandoffEligibilityState,
    /// The lifecycle label the row effectively carries after narrowing.
    pub effective_label: StableClaimLevel,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<HandoffGapReason>,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5HandoffEligibilityRow {
    /// True when the effective label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.handoff_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: HandoffGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// Summary counts carried by the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5HandoffEligibilityManifestSummary {
    /// Total number of handoff rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Rows blocked by a missing owner sign-off.
    pub entries_owner_blocked: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_holding: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Browser source rows.
    pub browser_entries: usize,
    /// Companion source rows.
    pub companion_entries: usize,
    /// Embedded-boundary source rows.
    pub embedded_boundary_entries: usize,
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
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffEligibilityManifestExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The source surface family.
    pub source_kind: HandoffSurfaceKind,
    /// The target surface family.
    pub target_kind: HandoffSurfaceKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Whether the row is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Handoff eligibility state earned.
    pub handoff_state: HandoffEligibilityState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<HandoffGapReason>,
    /// Owner ref.
    pub owner_ref: String,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffEligibilityManifestExportProjection {
    /// Manifest identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5HandoffEligibilityManifestExportRow>,
}

/// The typed M5 browser, companion, and embedded-boundary manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5BrowserCompanionEmbeddedBoundaryManifest {
    /// Manifest schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable manifest identifier.
    pub manifest_id: String,
    /// Lifecycle status of this manifest artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this manifest ingests.
    pub claim_manifest_ref: String,
    /// Ref to the M5 feature-train matrix this manifest builds on.
    pub feature_train_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<HandoffSurfaceKind>,
    /// Closed handoff-state vocabulary.
    pub handoff_states: Vec<HandoffEligibilityState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<HandoffGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<HandoffAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this manifest must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<HandoffStopRule>,
    /// Handoff eligibility rows.
    pub rows: Vec<M5HandoffEligibilityRow>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5HandoffEligibilityManifestSummary,
}

impl M5BrowserCompanionEmbeddedBoundaryManifest {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5HandoffEligibilityRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5HandoffEligibilityRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5HandoffEligibilityRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5HandoffEligibilityRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one source surface kind.
    pub fn rows_for_kind(&self, kind: HandoffSurfaceKind) -> Vec<&M5HandoffEligibilityRow> {
        self.rows
            .iter()
            .filter(|row| row.source_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the manifest covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &HandoffStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and stop rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<HandoffGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
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
    pub fn computed_summary(&self) -> M5HandoffEligibilityManifestSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: HandoffSurfaceKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&M5HandoffEligibilityRow> = self.release_blocking_rows();
        M5HandoffEligibilityManifestSummary {
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
                .filter(|row| row.handoff_state == HandoffEligibilityState::EligibleDegraded)
                .count(),
            entries_owner_blocked: self
                .rows
                .iter()
                .filter(|row| {
                    row.active_gap_reasons
                        .contains(&HandoffGapReason::OwnerSignoffMissing)
                })
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
            browser_entries: kind(HandoffSurfaceKind::Browser),
            companion_entries: kind(HandoffSurfaceKind::Companion),
            embedded_boundary_entries: kind(HandoffSurfaceKind::EmbeddedBoundary),
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
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the manifest that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5HandoffEligibilityManifestExportProjection {
        M5HandoffEligibilityManifestExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5HandoffEligibilityManifestExportRow {
                    entry_id: row.entry_id.clone(),
                    source_kind: row.source_kind,
                    target_kind: row.target_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    publishes_stable: row.publishes_stable(),
                    handoff_state: row.handoff_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    owner_ref: row.owner_signoff.owner_ref.clone(),
                })
                .collect(),
        }
    }

    /// Validates the manifest, returning every violation found.
    pub fn validate(&self) -> Vec<M5HandoffEligibilityManifestViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5HandoffEligibilityManifestViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5HandoffEligibilityManifestViolation::EmptyManifest);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5HandoffEligibilityManifestViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5HandoffEligibilityManifestViolation>) {
        if self.schema_version
            != GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_SCHEMA_VERSION
        {
            violations.push(M5HandoffEligibilityManifestViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_RECORD_KIND
        {
            violations.push(M5HandoffEligibilityManifestViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("feature_train_matrix_ref", &self.feature_train_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5HandoffEligibilityManifestViolation::EmptyField {
                    entry_id: "<manifest>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(
                M5HandoffEligibilityManifestViolation::ClosedVocabularyMismatch {
                    field: "lifecycle_labels",
                },
            );
        }
        if self.surface_kinds != HandoffSurfaceKind::ALL.to_vec() {
            violations.push(
                M5HandoffEligibilityManifestViolation::ClosedVocabularyMismatch {
                    field: "surface_kinds",
                },
            );
        }
        if self.handoff_states != HandoffEligibilityState::ALL.to_vec() {
            violations.push(
                M5HandoffEligibilityManifestViolation::ClosedVocabularyMismatch {
                    field: "handoff_states",
                },
            );
        }
        if self.gap_reasons != HandoffGapReason::ALL.to_vec() {
            violations.push(
                M5HandoffEligibilityManifestViolation::ClosedVocabularyMismatch {
                    field: "gap_reasons",
                },
            );
        }
        if self.stop_rule_actions != HandoffAction::ALL.to_vec() {
            violations.push(
                M5HandoffEligibilityManifestViolation::ClosedVocabularyMismatch {
                    field: "stop_rule_actions",
                },
            );
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(
                M5HandoffEligibilityManifestViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.cutline_level",
                },
            );
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(
                M5HandoffEligibilityManifestViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.above_cutline_levels",
                },
            );
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(
                M5HandoffEligibilityManifestViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.below_cutline_levels",
                },
            );
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5HandoffEligibilityManifestViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5HandoffEligibilityManifestViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5HandoffEligibilityManifestViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5HandoffEligibilityManifestViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5HandoffEligibilityManifestViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(
                    M5HandoffEligibilityManifestViolation::StopRuleWithoutLabels {
                        rule_id: rule.rule_id.clone(),
                    },
                );
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in HandoffGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    M5HandoffEligibilityManifestViolation::GapReasonWithoutStopRule { reason },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5HandoffEligibilityRow,
        violations: &mut Vec<M5HandoffEligibilityManifestViolation>,
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
                violations.push(M5HandoffEligibilityManifestViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.holds_label() && !row.active_gap_reasons.is_empty() {
            violations.push(M5HandoffEligibilityManifestViolation::HeldWithActiveGap {
                entry_id: row.entry_id.clone(),
                gap_reasons: row.active_gap_reasons.clone(),
            });
        }

        if row.publishes_stable() {
            if matches!(
                row.proof_packet.slo_state,
                FreshnessSloState::Breached | FreshnessSloState::Missing
            ) {
                violations.push(M5HandoffEligibilityManifestViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state: row.proof_packet.slo_state,
                });
            }
        }

        if row.claim_holds_stable() && row.handoff_state.forces_narrowing() {
            if row.effective_label.is_at_or_above_cutline() {
                violations.push(
                    M5HandoffEligibilityManifestViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        claim_label: row.claim_label,
                        published_label: row.effective_label,
                    },
                );
            }
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5HandoffEligibilityManifestViolation>) {
        let covered: BTreeSet<String> = self
            .release_blocking_rows()
            .into_iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_surface_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5HandoffEligibilityManifestViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: declared.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5HandoffEligibilityManifestViolation>) {
        if self.publication.decision != self.computed_publication_decision() {
            violations.push(
                M5HandoffEligibilityManifestViolation::PublicationDecisionInconsistent {
                    recorded: self.publication.decision,
                    computed: self.computed_publication_decision(),
                    blocking_rule_ids: self.computed_blocking_rule_ids(),
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                M5HandoffEligibilityManifestViolation::PublicationBlockingRuleIdsMismatch {
                    recorded: self.publication.blocking_rule_ids.clone(),
                    computed: self.computed_blocking_rule_ids(),
                },
            );
        }
        if self.publication.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                M5HandoffEligibilityManifestViolation::PublicationBlockingClaimIdsMismatch {
                    recorded: self.publication.blocking_claim_ids.clone(),
                    computed: self.computed_blocking_entry_ids(),
                },
            );
        }
    }
}

/// Parses the embedded checked-in manifest JSON.
pub fn current_m5_browser_companion_embedded_boundary_manifest_with_handoff_eligibility_rows(
) -> Result<M5BrowserCompanionEmbeddedBoundaryManifest, Box<dyn Error>> {
    let manifest: M5BrowserCompanionEmbeddedBoundaryManifest =
        serde_json::from_str(GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_JSON)?;
    Ok(manifest)
}

/// Violation found during manifest validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5HandoffEligibilityManifestViolation {
    /// Schema version does not match the expected version.
    UnsupportedSchemaVersion { actual: u32 },
    /// Record kind does not match the expected kind.
    UnsupportedRecordKind { actual: String },
    /// A required string field is empty or whitespace-only.
    EmptyField {
        entry_id: String,
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the canonical ordering.
    ClosedVocabularyMismatch { field: &'static str },
    /// The manifest contains no stop rules.
    NoStopRules,
    /// A stop rule id appears more than once.
    DuplicateStopRuleId { rule_id: String },
    /// A stop rule watches no labels.
    StopRuleWithoutLabels { rule_id: String },
    /// A gap reason has no stop rule covering it.
    GapReasonWithoutStopRule { reason: HandoffGapReason },
    /// A row entry id appears more than once.
    DuplicateEntryId { entry_id: String },
    /// The manifest contains no rows.
    EmptyManifest,
    /// A release-blocking surface ref has no covering row.
    ReleaseBlockingSurfaceUncovered { surface_ref: String },
    /// A backed row carries one or more active gap reasons.
    HeldWithActiveGap {
        entry_id: String,
        gap_reasons: Vec<HandoffGapReason>,
    },
    /// A backed row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        entry_id: String,
        slo_state: FreshnessSloState,
    },
    /// A narrowed row is still published at or above the cutline.
    PublishedLabelNotNarrowed {
        entry_id: String,
        claim_label: StableClaimLevel,
        published_label: StableClaimLevel,
    },
    /// The recorded publication decision does not match the computed decision.
    PublicationDecisionInconsistent {
        recorded: PromotionDecision,
        computed: PromotionDecision,
        blocking_rule_ids: Vec<String>,
    },
    /// The recorded blocking rule ids do not match the computed ids.
    PublicationBlockingRuleIdsMismatch {
        recorded: Vec<String>,
        computed: Vec<String>,
    },
    /// The recorded blocking claim ids do not match the computed ids.
    PublicationBlockingClaimIdsMismatch {
        recorded: Vec<String>,
        computed: Vec<String>,
    },
    /// The summary block does not match the computed summary.
    SummaryMismatch,
}

impl fmt::Display for M5HandoffEligibilityManifestViolation {
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
                write!(f, "empty field `{field_name}` on {entry_id}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch: {field}")
            }
            Self::NoStopRules => write!(f, "manifest has no stop rules"),
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop rule id: {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => {
                write!(f, "gap reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id: {entry_id}")
            }
            Self::EmptyManifest => write!(f, "manifest has no rows"),
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(f, "release-blocking surface {surface_ref} is uncovered")
            }
            Self::HeldWithActiveGap {
                entry_id,
                gap_reasons,
            } => {
                write!(
                    f,
                    "row {entry_id} is held but carries active gaps: {:?}",
                    gap_reasons.iter().map(|r| r.as_str()).collect::<Vec<_>>()
                )
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "row {entry_id} is held on a stale packet ({slo_state:?})"
                )
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                claim_label,
                published_label,
            } => {
                write!(
                    f,
                    "row {entry_id} claims {claim_label:?} but is published at {published_label:?}"
                )
            }
            Self::PublicationDecisionInconsistent {
                recorded,
                computed,
                blocking_rule_ids,
            } => {
                write!(
                    f,
                    "publication decision {recorded:?} does not match computed {computed:?}; blocking rules: {blocking_rule_ids:?}"
                )
            }
            Self::PublicationBlockingRuleIdsMismatch { recorded, computed } => {
                write!(
                    f,
                    "blocking rule ids {recorded:?} do not match computed {computed:?}"
                )
            }
            Self::PublicationBlockingClaimIdsMismatch { recorded, computed } => {
                write!(
                    f,
                    "blocking claim ids {recorded:?} do not match computed {computed:?}"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match computed summary"),
        }
    }
}

impl Error for M5HandoffEligibilityManifestViolation {}
