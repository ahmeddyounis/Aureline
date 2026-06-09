//! Typed M5 certified-archetype health-bundle matrix and regression guardrails.
//!
//! This module seeds the canonical health-bundle matrix for every M5 certified
//! archetype and the regression guardrails that narrow a certified claim when
//! health indicators regress. Each [`HealthBundleRow`] binds one M5 certified
//! archetype to:
//!
//! - a [`HealthBundle`] that defines the required health indicators
//!   ([`HealthIndicatorKind`]) and their states ([`HealthIndicatorState`]),
//! - the bundle state earned ([`HealthBundleRowState`]), the active gap
//!   reasons ([`HealthBundleGapReason`]), and the effective label after narrowing
//!   ([`HealthBundleRow::published_label`]),
//! - the [`RegressionGuardrailRule`] set that names the closed conditions that
//!   gate publication, and [`M5HealthBundleMatrix::publication`] records the
//!   proceed/hold verdict.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between an archetype whose health bundle may publish as Stable and
//! one that must narrow below it.
//!
//! The matrix is checked in at
//! `artifacts/release/m5/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! It carries no raw artifacts, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported matrix schema version.
pub const SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_RECORD_KIND: &str =
    "seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails";

/// Repo-relative path to the checked-in matrix.
pub const SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_PATH: &str =
    "artifacts/release/m5/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.json";

/// Embedded checked-in matrix JSON.
pub const SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails.json"
    ));

/// Certified archetype kind a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedArchetypeKind {
    /// Notebook and data-rich promoted surfaces.
    Notebook,
    /// Data-heavy surfaces (result grids, variable explorers).
    DataRich,
    /// AI-adjacent surfaces and language intelligence.
    AiAdjacent,
    /// Core framework and platform foundations.
    Framework,
    /// Review and diff surfaces.
    Review,
    /// Browser/mobile companion surfaces.
    Companion,
    /// Managed-depth and infrastructure surfaces.
    ManagedDepth,
}

impl CertifiedArchetypeKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Notebook,
        Self::DataRich,
        Self::AiAdjacent,
        Self::Framework,
        Self::Review,
        Self::Companion,
        Self::ManagedDepth,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataRich => "data_rich",
            Self::AiAdjacent => "ai_adjacent",
            Self::Framework => "framework",
            Self::Review => "review",
            Self::Companion => "companion",
            Self::ManagedDepth => "managed_depth",
        }
    }
}

/// Kind of health bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthBundleKind {
    /// Test coverage health bundle.
    TestCoverage,
    /// Benchmark health bundle.
    BenchmarkHealth,
    /// Compatibility health bundle.
    CompatibilityHealth,
    /// Dependency health bundle.
    DependencyHealth,
    /// Security posture health bundle.
    SecurityPosture,
    /// Accessibility health bundle.
    AccessibilityHealth,
    /// Localization health bundle.
    LocalizationHealth,
}

impl HealthBundleKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TestCoverage,
        Self::BenchmarkHealth,
        Self::CompatibilityHealth,
        Self::DependencyHealth,
        Self::SecurityPosture,
        Self::AccessibilityHealth,
        Self::LocalizationHealth,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestCoverage => "test_coverage",
            Self::BenchmarkHealth => "benchmark_health",
            Self::CompatibilityHealth => "compatibility_health",
            Self::DependencyHealth => "dependency_health",
            Self::SecurityPosture => "security_posture",
            Self::AccessibilityHealth => "accessibility_health",
            Self::LocalizationHealth => "localization_health",
        }
    }
}

/// Kind of health indicator within a bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthIndicatorKind {
    /// Unit test pass rate.
    UnitTestPassRate,
    /// Integration test pass rate.
    IntegrationTestPassRate,
    /// Benchmark regression indicator.
    BenchmarkRegression,
    /// Compatibility surface coverage.
    CompatibilitySurfaceCoverage,
    /// Dependency freshness.
    DependencyFreshness,
    /// Security scan clean.
    SecurityScanClean,
    /// Accessibility signoff coverage.
    AccessibilitySignoffCoverage,
}

impl HealthIndicatorKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::UnitTestPassRate,
        Self::IntegrationTestPassRate,
        Self::BenchmarkRegression,
        Self::CompatibilitySurfaceCoverage,
        Self::DependencyFreshness,
        Self::SecurityScanClean,
        Self::AccessibilitySignoffCoverage,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnitTestPassRate => "unit_test_pass_rate",
            Self::IntegrationTestPassRate => "integration_test_pass_rate",
            Self::BenchmarkRegression => "benchmark_regression",
            Self::CompatibilitySurfaceCoverage => "compatibility_surface_coverage",
            Self::DependencyFreshness => "dependency_freshness",
            Self::SecurityScanClean => "security_scan_clean",
            Self::AccessibilitySignoffCoverage => "accessibility_signoff_coverage",
        }
    }
}

/// State of an individual health indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthIndicatorState {
    /// Indicator is healthy.
    Green,
    /// Indicator is degraded but not blocking.
    Yellow,
    /// Indicator is regressed and blocking.
    Red,
    /// Indicator is missing.
    Missing,
}

impl HealthIndicatorState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Green,
        Self::Yellow,
        Self::Red,
        Self::Missing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
            Self::Missing => "missing",
        }
    }
}

/// State of a health bundle row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthBundleRowState {
    /// All required health indicators are green and the owner is signed off.
    Healthy,
    /// One or more required health indicators are yellow or red.
    Degraded,
    /// A benchmark regression or compatibility surface regression is detected.
    Regressed,
    /// The health bundle is missing required indicators.
    Missing,
    /// Holds the claimed label only because an active, unexpired waiver covers
    /// a recorded gap.
    OnWaiver,
}

impl HealthBundleRowState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Healthy,
        Self::Degraded,
        Self::Regressed,
        Self::Missing,
        Self::OnWaiver,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Regressed => "regressed",
            Self::Missing => "missing",
            Self::OnWaiver => "on_waiver",
        }
    }

    /// Whether the state lets a row carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Healthy | Self::OnWaiver)
    }

    /// Whether the state forces the row below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a health bundle row narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthBundleGapReason {
    /// A health indicator is red.
    HealthIndicatorRed,
    /// A health indicator is missing.
    HealthIndicatorMissing,
    /// The health bundle has gone stale.
    HealthBundleStale,
    /// The health bundle is missing.
    HealthBundleMissing,
    /// A benchmark regression is detected.
    BenchmarkRegressionDetected,
    /// The compatibility surface has regressed.
    CompatibilitySurfaceRegressed,
    /// A security scan has failed.
    SecurityScanFailed,
    /// Accessibility signoff is incomplete.
    AccessibilitySignoffIncomplete,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl HealthBundleGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::HealthIndicatorRed,
        Self::HealthIndicatorMissing,
        Self::HealthBundleStale,
        Self::HealthBundleMissing,
        Self::BenchmarkRegressionDetected,
        Self::CompatibilitySurfaceRegressed,
        Self::SecurityScanFailed,
        Self::AccessibilitySignoffIncomplete,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HealthIndicatorRed => "health_indicator_red",
            Self::HealthIndicatorMissing => "health_indicator_missing",
            Self::HealthBundleStale => "health_bundle_stale",
            Self::HealthBundleMissing => "health_bundle_missing",
            Self::BenchmarkRegressionDetected => "benchmark_regression_detected",
            Self::CompatibilitySurfaceRegressed => "compatibility_surface_regressed",
            Self::SecurityScanFailed => "security_scan_failed",
            Self::AccessibilitySignoffIncomplete => "accessibility_signoff_incomplete",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthBundleAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Refresh the health bundle.
    RefreshHealthBundle,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Recapture benchmarks.
    RecaptureBenchmarks,
    /// Re-run the security scan.
    ReRunSecurityScan,
}

impl HealthBundleAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::RefreshHealthBundle,
        Self::RequestOwnerSignoff,
        Self::RecaptureBenchmarks,
        Self::ReRunSecurityScan,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshHealthBundle => "refresh_health_bundle",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RecaptureBenchmarks => "recapture_benchmarks",
            Self::ReRunSecurityScan => "re_run_security_scan",
        }
    }
}

/// One health indicator in a bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthIndicator {
    /// Stable indicator id.
    pub indicator_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of health indicator.
    pub indicator_kind: HealthIndicatorKind,
    /// Whether this indicator is required for the bundle to be healthy.
    pub required: bool,
    /// Current state of the indicator.
    pub state: HealthIndicatorState,
    /// Reviewable reason this indicator carries this state.
    pub rationale: String,
}

/// The health bundle for a certified archetype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthBundle {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Bundle version.
    pub bundle_version: u32,
    /// The kind of health bundle.
    pub bundle_kind: HealthBundleKind,
    /// Health indicators.
    pub indicators: Vec<HealthIndicator>,
}

impl HealthBundle {
    /// True when every required indicator is present and green.
    pub fn is_healthy(&self) -> bool {
        self.indicators
            .iter()
            .all(|i| !i.required || i.state == HealthIndicatorState::Green)
    }

    /// True when every required indicator kind is present.
    pub fn has_all_required_indicators(&self) -> bool {
        let present: BTreeSet<HealthIndicatorKind> = self
            .indicators
            .iter()
            .filter(|i| i.required)
            .map(|i| i.indicator_kind)
            .collect();
        present.len() == HealthIndicatorKind::ALL.len()
    }

    /// Returns indicators whose state is red.
    pub fn red_indicators(&self) -> Vec<&HealthIndicator> {
        self.indicators
            .iter()
            .filter(|i| i.state == HealthIndicatorState::Red)
            .collect()
    }

    /// Returns indicators whose state is missing.
    pub fn missing_indicators(&self) -> Vec<&HealthIndicator> {
        self.indicators
            .iter()
            .filter(|i| i.state == HealthIndicatorState::Missing)
            .collect()
    }

    /// Returns required indicators whose state is not green.
    pub fn failing_required_indicators(&self) -> Vec<&HealthIndicator> {
        self.indicators
            .iter()
            .filter(|i| i.required && i.state != HealthIndicatorState::Green)
            .collect()
    }
}

/// One regression guardrail stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionGuardrailRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: HealthBundleGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: HealthBundleAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One health bundle row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthBundleRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The archetype kind this row governs.
    pub archetype_kind: CertifiedArchetypeKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the row.
    pub surface_summary: String,
    /// Whether the archetype is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The health bundle for this archetype.
    pub health_bundle: HealthBundle,
    /// Bundle state earned for the row.
    pub bundle_state: HealthBundleRowState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<HealthBundleGapReason>,
    /// The lifecycle label the archetype effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl HealthBundleRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the archetype carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.bundle_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: HealthBundleGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when the health bundle is healthy and complete.
    pub fn bundle_healthy(&self) -> bool {
        self.health_bundle.is_healthy() && self.health_bundle.has_all_required_indicators()
    }
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5HealthBundleMatrixSummary {
    /// Total number of health bundle rows.
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
    /// Notebook rows.
    pub notebook_entries: usize,
    /// Data-rich rows.
    pub data_rich_entries: usize,
    /// AI-adjacent rows.
    pub ai_adjacent_entries: usize,
    /// Framework rows.
    pub framework_entries: usize,
    /// Review rows.
    pub review_entries: usize,
    /// Companion rows.
    pub companion_entries: usize,
    /// Managed-depth rows.
    pub managed_depth_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Indicators whose state is `green`.
    pub indicators_green: usize,
    /// Indicators whose state is `yellow`.
    pub indicators_yellow: usize,
    /// Indicators whose state is `red`.
    pub indicators_red: usize,
    /// Indicators whose state is `missing`.
    pub indicators_missing: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HealthBundleMatrixExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The archetype kind this row governs.
    pub archetype_kind: CertifiedArchetypeKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Whether the archetype is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Bundle state earned.
    pub bundle_state: HealthBundleRowState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<HealthBundleGapReason>,
    /// Owner ref.
    pub owner_ref: String,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HealthBundleMatrixExportProjection {
    /// Matrix identifier.
    pub matrix_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5HealthBundleMatrixExportRow>,
}

/// The typed M5 certified-archetype health-bundle matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5HealthBundleMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix identifier.
    pub matrix_id: String,
    /// Lifecycle status of this matrix artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this matrix ingests.
    pub claim_manifest_ref: String,
    /// Ref to the certified reference workspaces.
    pub certified_reference_workspaces_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed archetype-kind vocabulary.
    pub archetype_kinds: Vec<CertifiedArchetypeKind>,
    /// Closed bundle-kind vocabulary.
    pub bundle_kinds: Vec<HealthBundleKind>,
    /// Closed indicator-state vocabulary.
    pub indicator_states: Vec<HealthIndicatorState>,
    /// Closed bundle-state vocabulary.
    pub bundle_states: Vec<HealthBundleRowState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<HealthBundleGapReason>,
    /// Closed guardrail-action vocabulary.
    pub guardrail_actions: Vec<HealthBundleAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this matrix must cover.
    pub release_blocking_archetype_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<RegressionGuardrailRule>,
    /// Health bundle rows.
    pub rows: Vec<HealthBundleRow>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5HealthBundleMatrixSummary,
}

impl M5HealthBundleMatrix {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&HealthBundleRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&HealthBundleRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&HealthBundleRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&HealthBundleRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one archetype kind.
    pub fn rows_for_kind(&self, kind: CertifiedArchetypeKind) -> Vec<&HealthBundleRow> {
        self.rows
            .iter()
            .filter(|row| row.archetype_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the matrix covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &RegressionGuardrailRule) -> bool {
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
        let blocking_triggers: BTreeSet<HealthBundleGapReason> = self
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
    pub fn computed_summary(&self) -> M5HealthBundleMatrixSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: CertifiedArchetypeKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&HealthBundleRow> = self.release_blocking_rows();
        let indicator_states = |state: HealthIndicatorState| {
            self.rows
                .iter()
                .flat_map(|row| &row.health_bundle.indicators)
                .filter(|i| i.state == state)
                .count()
        };
        M5HealthBundleMatrixSummary {
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
                .filter(|row| row.bundle_state == HealthBundleRowState::OnWaiver)
                .count(),
            entries_owner_blocked: self
                .rows
                .iter()
                .filter(|row| !row.owner_signoff.signed_off)
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
            notebook_entries: kind(CertifiedArchetypeKind::Notebook),
            data_rich_entries: kind(CertifiedArchetypeKind::DataRich),
            ai_adjacent_entries: kind(CertifiedArchetypeKind::AiAdjacent),
            framework_entries: kind(CertifiedArchetypeKind::Framework),
            review_entries: kind(CertifiedArchetypeKind::Review),
            companion_entries: kind(CertifiedArchetypeKind::Companion),
            managed_depth_entries: kind(CertifiedArchetypeKind::ManagedDepth),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            indicators_green: indicator_states(HealthIndicatorState::Green),
            indicators_yellow: indicator_states(HealthIndicatorState::Yellow),
            indicators_red: indicator_states(HealthIndicatorState::Red),
            indicators_missing: indicator_states(HealthIndicatorState::Missing),
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

    /// Produces an export/Help-About-safe projection of the matrix that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5HealthBundleMatrixExportProjection {
        M5HealthBundleMatrixExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5HealthBundleMatrixExportRow {
                    entry_id: row.entry_id.clone(),
                    archetype_kind: row.archetype_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    bundle_state: row.bundle_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    owner_ref: row.owner_signoff.owner_ref.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<M5HealthBundleMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5HealthBundleMatrixViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5HealthBundleMatrixViolation::EmptyMatrix);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5HealthBundleMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5HealthBundleMatrixViolation>) {
        if self.schema_version
            != SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_SCHEMA_VERSION
        {
            violations.push(M5HealthBundleMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_RECORD_KIND
        {
            violations.push(M5HealthBundleMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            (
                "certified_reference_workspaces_ref",
                &self.certified_reference_workspaces_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(M5HealthBundleMatrixViolation::EmptyField {
                    entry_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.archetype_kinds != CertifiedArchetypeKind::ALL.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "archetype_kinds",
            });
        }
        if self.bundle_kinds != HealthBundleKind::ALL.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "bundle_kinds",
            });
        }
        if self.indicator_states != HealthIndicatorState::ALL.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "indicator_states",
            });
        }
        if self.bundle_states != HealthBundleRowState::ALL.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "bundle_states",
            });
        }
        if self.gap_reasons != HealthBundleGapReason::ALL.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.guardrail_actions != HealthBundleAction::ALL.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "guardrail_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5HealthBundleMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5HealthBundleMatrixViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5HealthBundleMatrixViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5HealthBundleMatrixViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5HealthBundleMatrixViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5HealthBundleMatrixViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5HealthBundleMatrixViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in HealthBundleGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(M5HealthBundleMatrixViolation::GapReasonWithoutStopRule {
                    reason,
                });
            }
        }
    }

    fn validate_row(
        &self,
        row: &HealthBundleRow,
        violations: &mut Vec<M5HealthBundleMatrixViolation>,
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
                violations.push(M5HealthBundleMatrixViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no row may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5HealthBundleMatrixViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5HealthBundleMatrixViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5HealthBundleMatrixViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row that holds its label must have owner sign-off.
        if row.holds_label() && !row.owner_signoff.signed_off {
            violations.push(M5HealthBundleMatrixViolation::HeldWithoutSignoff {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row that holds its label must not have active gap reasons.
        if row.holds_label() && !row.active_gap_reasons.is_empty() {
            violations.push(M5HealthBundleMatrixViolation::HeldWithActiveGap {
                entry_id: row.entry_id.clone(),
                reasons: row.active_gap_reasons.clone(),
            });
        }

        // A row whose state forces narrowing must actually narrow.
        if row.bundle_state.forces_narrowing()
            && row.published_label.rank() >= row.claim_label.rank()
        {
            violations.push(M5HealthBundleMatrixViolation::PublishedLabelNotNarrowed {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
                state: row.bundle_state,
            });
        }

        // The health bundle must contain all required indicator kinds.
        if !row.health_bundle.has_all_required_indicators() {
            violations.push(M5HealthBundleMatrixViolation::IncompleteHealthBundle {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5HealthBundleMatrixViolation>) {
        let covered_refs: BTreeSet<String> =
            self.rows.iter().map(|row| row.surface_ref.clone()).collect();
        for surface_ref in &self.release_blocking_archetype_refs {
            if !covered_refs.contains(surface_ref) {
                violations.push(
                    M5HealthBundleMatrixViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: surface_ref.clone(),
                    },
                );
            }
        }
        let rb_set: BTreeSet<String> =
            self.release_blocking_archetype_refs.iter().cloned().collect();
        for row in &self.rows {
            if row.release_blocking && !rb_set.contains(&row.surface_ref) {
                violations.push(
                    M5HealthBundleMatrixViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5HealthBundleMatrixViolation>) {
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                M5HealthBundleMatrixViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        let blocking_rules = self.computed_blocking_rule_ids();
        if self.publication.blocking_rule_ids != blocking_rules {
            violations.push(
                M5HealthBundleMatrixViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        let blocking_entries = self.computed_blocking_entry_ids();
        if self.publication.blocking_claim_ids != blocking_entries {
            violations.push(
                M5HealthBundleMatrixViolation::PublicationBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// Validation error for the M5 health bundle matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5HealthBundleMatrixViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion { actual: u32 },
    /// Record kind does not match the expected kind.
    UnsupportedRecordKind { actual: String },
    /// A required field is empty.
    EmptyField {
        entry_id: String,
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the canonical set.
    ClosedVocabularyMismatch { field: &'static str },
    /// No stop rules are defined.
    NoStopRules,
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId { rule_id: String },
    /// A stop rule watches no labels.
    StopRuleWithoutLabels { rule_id: String },
    /// A gap reason has no covering stop rule.
    GapReasonWithoutStopRule { reason: HealthBundleGapReason },
    /// A row id appears more than once.
    DuplicateEntryId { entry_id: String },
    /// The matrix contains no rows.
    EmptyMatrix,
    /// The published label is wider than the canonical claim label.
    PublishedWiderThanClaim {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
    },
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent { entry_id: String },
    /// A row that holds its label lacks owner sign-off.
    HeldWithoutSignoff { entry_id: String },
    /// A row that holds its label has active gap reasons.
    HeldWithActiveGap {
        entry_id: String,
        reasons: Vec<HealthBundleGapReason>,
    },
    /// A row whose state forces narrowing did not narrow.
    PublishedLabelNotNarrowed {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
        state: HealthBundleRowState,
    },
    /// A row that holds its label rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        entry_id: String,
        slo_state: FreshnessSloState,
    },
    /// A narrowing row with a breached packet does not name the stale reason.
    NarrowingRowMissingStaleReason { entry_id: String },
    /// A narrowing row with a missing packet does not name the missing reason.
    NarrowingRowMissingMissingReason { entry_id: String },
    /// A release-blocking surface has no covering row.
    ReleaseBlockingSurfaceUncovered { surface_ref: String },
    /// A release-blocking row is not declared in release_blocking_archetype_refs.
    ReleaseBlockingRowNotDeclared { entry_id: String },
    /// The declared publication decision disagrees with the computed decision.
    PublicationDecisionInconsistent {
        declared: PromotionDecision,
        computed: PromotionDecision,
    },
    /// A publication blocking set field disagrees with firing stop rules.
    PublicationBlockingSetMismatch { field: &'static str },
    /// Summary counts disagree with row state.
    SummaryMismatch,
    /// The health bundle is missing required indicators.
    IncompleteHealthBundle { entry_id: String },
}

impl fmt::Display for M5HealthBundleMatrixViolation {
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
            } => write!(f, "{entry_id}: empty field {field_name}"),
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch: {field}")
            }
            Self::NoStopRules => write!(f, "no stop rules defined"),
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop-rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => {
                write!(f, "gap reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "{entry_id}: published label {published:?} is wider than claim {claim:?}"
            ),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds its label but lacks owner sign-off"
                )
            }
            Self::HeldWithActiveGap { entry_id, reasons } => {
                write!(
                    f,
                    "row {entry_id} holds its label but has active gaps: {reasons:?}"
                )
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                claim,
                published,
                state,
            } => write!(
                f,
                "row {entry_id} state {state:?} forces narrowing but claim {claim:?} / published {published:?} does not narrow"
            ),
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(f, "row {entry_id} holds stable on stale packet {slo_state:?}")
            }
            Self::NarrowingRowMissingStaleReason { entry_id } => {
                write!(f, "row {entry_id} breached packet without health_bundle_stale reason")
            }
            Self::NarrowingRowMissingMissingReason { entry_id } => {
                write!(f, "row {entry_id} missing packet without health_bundle_missing reason")
            }
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering row"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => {
                write!(
                    f,
                    "release-blocking row {entry_id} is not declared in release_blocking_archetype_refs"
                )
            }
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication {declared:?} disagrees with computed {computed:?}"
                )
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
            Self::IncompleteHealthBundle { entry_id } => {
                write!(f, "row {entry_id} health bundle is incomplete")
            }
        }
    }
}

impl Error for M5HealthBundleMatrixViolation {}

/// Loads the embedded M5 health bundle matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`M5HealthBundleMatrix`].
pub fn current_m5_health_bundle_matrix() -> Result<M5HealthBundleMatrix, serde_json::Error> {
    serde_json::from_str(
        SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> M5HealthBundleMatrix {
        current_m5_health_bundle_matrix().expect("matrix parses")
    }

    #[test]
    fn embedded_matrix_parses_and_validates() {
        let m = matrix();
        assert_eq!(
            m.schema_version,
            SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_SCHEMA_VERSION
        );
        assert_eq!(
            m.record_kind,
            SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_RECORD_KIND
        );
        assert_eq!(m.validate(), Vec::new());
        assert!(!m.rows.is_empty());
    }

    #[test]
    fn covers_every_archetype_kind() {
        let m = matrix();
        for kind in CertifiedArchetypeKind::ALL {
            assert!(
                !m.rows_for_kind(kind).is_empty(),
                "archetype kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_surface() {
        let m = matrix();
        assert!(!m.release_blocking_archetype_refs.is_empty());
        let covered: Vec<&str> = m
            .release_blocking_rows()
            .iter()
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &m.release_blocking_archetype_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let m = matrix();
        assert_eq!(m.summary, m.computed_summary());
        assert_eq!(
            m.summary.entries_holding_stable + m.summary.entries_narrowed,
            m.rows.len()
        );
    }

    #[test]
    fn publication_decision_matches_computed() {
        let m = matrix();
        assert_eq!(m.publication.decision, m.computed_publication_decision());
        assert_eq!(
            m.publication.blocking_rule_ids,
            m.computed_blocking_rule_ids()
        );
        assert_eq!(
            m.publication.blocking_claim_ids,
            m.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let m = matrix();
        let covered: BTreeSet<HealthBundleGapReason> =
            m.stop_rules.iter().map(|rule| rule.trigger_reason).collect();
        for reason in HealthBundleGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_row_with_active_gap() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.active_gap_reasons
            .push(HealthBundleGapReason::HealthIndicatorRed);
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, M5HealthBundleMatrixViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.bundle_state = HealthBundleRowState::Regressed;
        row.active_gap_reasons
            .push(HealthBundleGapReason::HealthIndicatorRed);
        row.published_label = StableClaimLevel::Stable;
        m.summary = m.computed_summary();
        m.publication.decision = m.computed_publication_decision();
        m.publication.blocking_rule_ids = m.computed_blocking_rule_ids();
        m.publication.blocking_claim_ids = m.computed_blocking_entry_ids();
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5HealthBundleMatrixViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut m = matrix();
        m.publication.decision = PromotionDecision::Proceed;
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5HealthBundleMatrixViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_claim_without_signoff() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, M5HealthBundleMatrixViolation::HeldWithoutSignoff { .. })));
    }

    #[test]
    fn validate_flags_an_incomplete_health_bundle() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.health_bundle.indicators.clear();
        m.summary = m.computed_summary();
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5HealthBundleMatrixViolation::IncompleteHealthBundle { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let m = matrix();
        let projection = m.support_export_projection();
        assert_eq!(projection.rows.len(), m.rows.len());
        for (row, proj) in m.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, proj.entry_id);
            assert_eq!(row.publishes_stable(), proj.publishes_stable);
        }
    }
}
