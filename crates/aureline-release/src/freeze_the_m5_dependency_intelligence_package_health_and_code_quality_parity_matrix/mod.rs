//! Typed M5 dependency-intelligence, package-health, and code-quality parity
//! matrix.
//!
//! This module freezes the canonical M5 dependency/package and quality/scanner
//! control surface. Each [`DqLaneRow`] binds one dependency-intelligence,
//! package-health, or code-quality lane to:
//!
//! - the stable claim it backs ([`DqLaneRow::claim_ref`],
//!   [`DqLaneRow::claim_label`]) plus the explicit
//!   [`DqClaimClass`](DqLaneRow::claim_class) and
//!   [`DeploymentProfile`](DqLaneRow::deployment_profile) posture and the
//!   reviewable [`compatibility/downgrade rule`](DqLaneRow::compatibility_downgrade_rule)
//!   the acceptance criteria require before feature work fans out,
//! - a [`DqScorecard`] that records proof, compatibility, admin/policy, and
//!   rollback posture,
//! - the scorecard state earned ([`DqScorecardState`]), the active gap reasons
//!   ([`DqGapReason`]), and the effective label after narrowing
//!   ([`DqLaneRow::published_label`]),
//! - upstream and downstream lane refs that encode the hard/soft dependency
//!   graph ([`DqDependencyEdge`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a lane that may ship as Stable and one that must narrow
//! below it. The [`DqStopRule`] set names the closed conditions that gate
//! promotion, and [`DqParityMatrix::promotion`] records the proceed/hold
//! verdict so release/public-truth tooling narrows stale or policy-blocked
//! rows automatically instead of depending on manual doc edits.
//!
//! The matrix is checked in at
//! `artifacts/release/m5/freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! It carries no manifests, lockfiles, raw scanner payloads, credentials, or
//! registry bodies.

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
pub const FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_dependency_quality_parity_matrix";

/// Repo-relative path to the checked-in matrix.
pub const FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_PATH: &str =
    "artifacts/release/m5/freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix.json";

/// Embedded checked-in matrix JSON.
pub const FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix.json"
));

/// Dependency/package or code-quality lane a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DqLaneKind {
    /// Advisory, license, suppression, and SBOM intelligence.
    DependencyIntelligence,
    /// Manifest-scope, registry/mirror auth, and grouped-update mutation.
    PackageMutation,
    /// Lockfile authority, validation packs, and package-health review.
    PackageHealth,
    /// Quality-profile precedence and gate configuration.
    QualityProfile,
    /// SARIF/scanner imported-finding visibility.
    ScannerImport,
    /// Live in-repo code-quality findings.
    LiveQuality,
    /// CLI and headless parity for dependency/package/quality flows.
    CliHeadlessParity,
}

impl DqLaneKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DependencyIntelligence,
        Self::PackageMutation,
        Self::PackageHealth,
        Self::QualityProfile,
        Self::ScannerImport,
        Self::LiveQuality,
        Self::CliHeadlessParity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DependencyIntelligence => "dependency_intelligence",
            Self::PackageMutation => "package_mutation",
            Self::PackageHealth => "package_health",
            Self::QualityProfile => "quality_profile",
            Self::ScannerImport => "scanner_import",
            Self::LiveQuality => "live_quality",
            Self::CliHeadlessParity => "cli_headless_parity",
        }
    }
}

/// Claim class a lane is marketed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DqClaimClass {
    /// A marketed M5 depth claim with its own scorecard and proof corpus.
    MarketedDepth,
    /// A foundation/parity claim intaken from the stable line and extended.
    FoundationParity,
    /// Imported-finding visibility (scanner/SARIF) distinct from live truth.
    ImportedVisibility,
    /// Advisory-only visibility with no stable mutation authority.
    AdvisoryOnly,
}

impl DqClaimClass {
    /// Every claim class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MarketedDepth,
        Self::FoundationParity,
        Self::ImportedVisibility,
        Self::AdvisoryOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarketedDepth => "marketed_depth",
            Self::FoundationParity => "foundation_parity",
            Self::ImportedVisibility => "imported_visibility",
            Self::AdvisoryOnly => "advisory_only",
        }
    }
}

/// Deployment-profile posture a lane is qualified under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfile {
    /// Local-only: no registry or network side effects beyond the workspace.
    LocalOnly,
    /// Mirrored: reaches a configured registry mirror.
    Mirrored,
    /// Managed: admin/policy-managed registry and quality gates.
    Managed,
    /// Browser-handoff: inspection handed off to the system browser.
    BrowserHandoff,
}

impl DeploymentProfile {
    /// Every profile, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::Mirrored,
        Self::Managed,
        Self::BrowserHandoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Mirrored => "mirrored",
            Self::Managed => "managed",
            Self::BrowserHandoff => "browser_handoff",
        }
    }
}

/// Scorecard state a lane earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DqScorecardState {
    /// All scorecard items are present and current.
    Complete,
    /// One or more required scorecard items are missing.
    Incomplete,
    /// A scorecard item has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers
    /// a recorded gap.
    OnWaiver,
    /// Blocked by a missing admin/policy story.
    Blocked,
    /// Missing a defined rollback or downgrade path.
    RollbackMissing,
}

impl DqScorecardState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Complete,
        Self::Incomplete,
        Self::Stale,
        Self::OnWaiver,
        Self::Blocked,
        Self::RollbackMissing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::Blocked => "blocked",
            Self::RollbackMissing => "rollback_missing",
        }
    }

    /// Whether the state lets a lane carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Complete | Self::OnWaiver)
    }

    /// Whether the state forces the lane below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a lane narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DqGapReason {
    /// Proof packet is missing.
    ProofPacketMissing,
    /// Proof packet is stale.
    ProofPacketStale,
    /// Compatibility report is missing.
    CompatibilityReportMissing,
    /// Compatibility report is stale.
    CompatibilityReportStale,
    /// Admin/policy story is missing.
    AdminPolicyMissing,
    /// Rollback/downgrade path is missing.
    RollbackPathMissing,
    /// An upstream hard-dependency lane is narrowed below the cutline.
    UpstreamLaneNarrowed,
    /// A waiver the lane relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl DqGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::CompatibilityReportMissing,
        Self::CompatibilityReportStale,
        Self::AdminPolicyMissing,
        Self::RollbackPathMissing,
        Self::UpstreamLaneNarrowed,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::CompatibilityReportMissing => "compatibility_report_missing",
            Self::CompatibilityReportStale => "compatibility_report_stale",
            Self::AdminPolicyMissing => "admin_policy_missing",
            Self::RollbackPathMissing => "rollback_path_missing",
            Self::UpstreamLaneNarrowed => "upstream_lane_narrowed",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DqAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Refresh the compatibility report.
    RefreshCompatibilityReport,
    /// Staff the admin/policy story.
    StaffAdminPolicy,
    /// Define the rollback/downgrade path.
    DefineRollbackPath,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl DqAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RefreshCompatibilityReport,
        Self::StaffAdminPolicy,
        Self::DefineRollbackPath,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RefreshCompatibilityReport => "refresh_compatibility_report",
            Self::StaffAdminPolicy => "staff_admin_policy",
            Self::DefineRollbackPath => "define_rollback_path",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Dependency kind between two lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DqDependencyKind {
    /// Hard dependency: downstream lane must narrow if upstream narrows.
    Hard,
    /// Soft dependency: downstream lane may hold independently.
    Soft,
}

impl DqDependencyKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::Hard, Self::Soft];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hard => "hard",
            Self::Soft => "soft",
        }
    }
}

/// The scorecard that grounds a dependency/quality lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DqScorecard {
    /// Ref to the proof packet backing the lane.
    pub proof_packet_ref: String,
    /// Ref to the compatibility report.
    pub compatibility_report_ref: String,
    /// Ref to the admin/policy story.
    pub admin_policy_ref: String,
    /// Ref to the rollback/downgrade path definition.
    pub rollback_path_ref: String,
}

impl DqScorecard {
    /// True when all four scorecard refs are non-empty.
    pub fn is_complete(&self) -> bool {
        !self.proof_packet_ref.trim().is_empty()
            && !self.compatibility_report_ref.trim().is_empty()
            && !self.admin_policy_ref.trim().is_empty()
            && !self.rollback_path_ref.trim().is_empty()
    }
}

/// One dependency edge in the parity graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DqDependencyEdge {
    /// Stable edge id.
    pub edge_id: String,
    /// Upstream lane entry id.
    pub from_lane: String,
    /// Downstream lane entry id.
    pub to_lane: String,
    /// Hard or soft dependency.
    pub dependency_kind: DqDependencyKind,
    /// Reviewable reason for the edge.
    pub rationale: String,
}

/// One stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DqStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: DqGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: DqAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One dependency/quality lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DqLaneRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The lane kind this row governs.
    pub lane_kind: DqLaneKind,
    /// The surface id this lane speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the lane.
    pub surface_summary: String,
    /// Whether the lane is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this lane backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The explicit claim class the lane is marketed under.
    pub claim_class: DqClaimClass,
    /// The deployment-profile posture the lane is qualified under.
    pub deployment_profile: DeploymentProfile,
    /// Ref to the compatibility/downgrade rule that governs this lane.
    pub compatibility_downgrade_rule: String,
    /// Scorecard state earned for the row.
    pub scorecard_state: DqScorecardState,
    /// The scorecard refs.
    pub scorecard: DqScorecard,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<DqGapReason>,
    /// The lifecycle label the lane effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Upstream lane refs.
    #[serde(default)]
    pub upstream_lane_refs: Vec<String>,
    /// Downstream lane refs.
    #[serde(default)]
    pub downstream_lane_refs: Vec<String>,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl DqLaneRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the lane carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.scorecard_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: DqGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DqParityMatrixSummary {
    /// Total number of lane rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Rows blocked by a missing admin/policy story.
    pub entries_blocked: usize,
    /// Rows missing a rollback path.
    pub entries_rollback_missing: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_holding: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Dependency-intelligence rows.
    pub dependency_intelligence_entries: usize,
    /// Package-mutation rows.
    pub package_mutation_entries: usize,
    /// Package-health rows.
    pub package_health_entries: usize,
    /// Quality-profile rows.
    pub quality_profile_entries: usize,
    /// Scanner-import rows.
    pub scanner_import_entries: usize,
    /// Live-quality rows.
    pub live_quality_entries: usize,
    /// CLI/headless-parity rows.
    pub cli_headless_parity_entries: usize,
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
    /// Hard dependency edges.
    pub hard_dependency_edges: usize,
    /// Soft dependency edges.
    pub soft_dependency_edges: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DqParityExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The lane kind this row governs.
    pub lane_kind: DqLaneKind,
    /// The surface id this lane speaks about.
    pub surface_ref: String,
    /// Whether the lane is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this lane backs.
    pub claim_ref: String,
    /// The explicit claim class.
    pub claim_class: DqClaimClass,
    /// The deployment-profile posture.
    pub deployment_profile: DeploymentProfile,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Scorecard state earned.
    pub scorecard_state: DqScorecardState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<DqGapReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DqParityExportProjection {
    /// Matrix identifier.
    pub matrix_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<DqParityExportRow>,
}

/// The typed M5 dependency/quality parity matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DqParityMatrix {
    /// Register schema version.
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
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed lane-kind vocabulary.
    pub lane_kinds: Vec<DqLaneKind>,
    /// Closed claim-class vocabulary.
    pub claim_classes: Vec<DqClaimClass>,
    /// Closed deployment-profile vocabulary.
    pub deployment_profiles: Vec<DeploymentProfile>,
    /// Closed scorecard-state vocabulary.
    pub scorecard_states: Vec<DqScorecardState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<DqGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<DqAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<DqStopRule>,
    /// Lane rows.
    pub rows: Vec<DqLaneRow>,
    /// Dependency edges.
    pub dependencies: Vec<DqDependencyEdge>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: DqParityMatrixSummary,
}

impl DqParityMatrix {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&DqLaneRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&DqLaneRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&DqLaneRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&DqLaneRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one lane kind.
    pub fn rows_for_kind(&self, kind: DqLaneKind) -> Vec<&DqLaneRow> {
        self.rows
            .iter()
            .filter(|row| row.lane_kind == kind)
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
    pub fn stop_rule_fires(&self, rule: &DqStopRule) -> bool {
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

    /// Lane-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row
    /// whose claim is already canonically narrowed is not a *promotion*
    /// blocker, it merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<DqGapReason> = self
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
    pub fn computed_summary(&self) -> DqParityMatrixSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: DqLaneKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&DqLaneRow> = self.release_blocking_rows();
        let (hard_edges, soft_edges) =
            self.dependencies
                .iter()
                .fold((0, 0), |(h, s), edge| match edge.dependency_kind {
                    DqDependencyKind::Hard => (h + 1, s),
                    DqDependencyKind::Soft => (h, s + 1),
                });
        DqParityMatrixSummary {
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
                .filter(|row| row.scorecard_state == DqScorecardState::OnWaiver)
                .count(),
            entries_blocked: self
                .rows
                .iter()
                .filter(|row| row.scorecard_state == DqScorecardState::Blocked)
                .count(),
            entries_rollback_missing: self
                .rows
                .iter()
                .filter(|row| row.scorecard_state == DqScorecardState::RollbackMissing)
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
            dependency_intelligence_entries: kind(DqLaneKind::DependencyIntelligence),
            package_mutation_entries: kind(DqLaneKind::PackageMutation),
            package_health_entries: kind(DqLaneKind::PackageHealth),
            quality_profile_entries: kind(DqLaneKind::QualityProfile),
            scanner_import_entries: kind(DqLaneKind::ScannerImport),
            live_quality_entries: kind(DqLaneKind::LiveQuality),
            cli_headless_parity_entries: kind(DqLaneKind::CliHeadlessParity),
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
            hard_dependency_edges: hard_edges,
            soft_dependency_edges: soft_edges,
        }
    }

    /// Produces an export/Help-About-safe projection of the matrix that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> DqParityExportProjection {
        DqParityExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| DqParityExportRow {
                    entry_id: row.entry_id.clone(),
                    lane_kind: row.lane_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_class: row.claim_class,
                    deployment_profile: row.deployment_profile,
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    scorecard_state: row.scorecard_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<DqParityMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);
        self.validate_dependencies(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(DqParityMatrixViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(DqParityMatrixViolation::EmptyMatrix);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(DqParityMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<DqParityMatrixViolation>) {
        if self.schema_version != FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_SCHEMA_VERSION {
            violations.push(DqParityMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_RECORD_KIND {
            violations.push(DqParityMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(DqParityMatrixViolation::EmptyField {
                    entry_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.lane_kinds != DqLaneKind::ALL.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "lane_kinds",
            });
        }
        if self.claim_classes != DqClaimClass::ALL.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "claim_classes",
            });
        }
        if self.deployment_profiles != DeploymentProfile::ALL.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "deployment_profiles",
            });
        }
        if self.scorecard_states != DqScorecardState::ALL.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "scorecard_states",
            });
        }
        if self.gap_reasons != DqGapReason::ALL.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != DqAction::ALL.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(DqParityMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(DqParityMatrixViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<DqParityMatrixViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(DqParityMatrixViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(DqParityMatrixViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(DqParityMatrixViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(DqParityMatrixViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in DqGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(DqParityMatrixViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_dependencies(&self, violations: &mut Vec<DqParityMatrixViolation>) {
        let row_ids: BTreeSet<String> = self.rows.iter().map(|row| row.entry_id.clone()).collect();
        let mut seen = BTreeSet::new();
        for edge in &self.dependencies {
            if !seen.insert(edge.edge_id.clone()) {
                violations.push(DqParityMatrixViolation::DuplicateEdgeId {
                    edge_id: edge.edge_id.clone(),
                });
            }
            if edge.from_lane.trim().is_empty() || edge.to_lane.trim().is_empty() {
                violations.push(DqParityMatrixViolation::EmptyField {
                    entry_id: edge.edge_id.clone(),
                    field_name: "dependency_lane_ref",
                });
            }
            if edge.from_lane == edge.to_lane {
                violations.push(DqParityMatrixViolation::SelfLoop {
                    edge_id: edge.edge_id.clone(),
                });
            }
            if !row_ids.contains(&edge.from_lane) {
                violations.push(DqParityMatrixViolation::UnresolvedLaneRef {
                    edge_id: edge.edge_id.clone(),
                    lane_ref: edge.from_lane.clone(),
                });
            }
            if !row_ids.contains(&edge.to_lane) {
                violations.push(DqParityMatrixViolation::UnresolvedLaneRef {
                    edge_id: edge.edge_id.clone(),
                    lane_ref: edge.to_lane.clone(),
                });
            }
            if edge.rationale.trim().is_empty() {
                violations.push(DqParityMatrixViolation::EmptyField {
                    entry_id: edge.edge_id.clone(),
                    field_name: "rationale",
                });
            }
        }
    }

    fn validate_row(&self, row: &DqLaneRow, violations: &mut Vec<DqParityMatrixViolation>) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
            ("claim_ref", &row.claim_ref),
            (
                "compatibility_downgrade_rule",
                &row.compatibility_downgrade_rule,
            ),
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
                violations.push(DqParityMatrixViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no lane may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(DqParityMatrixViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(DqParityMatrixViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(DqParityMatrixViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A held lane must have a complete scorecard.
        if row.holds_label() && !row.scorecard.is_complete() {
            violations.push(DqParityMatrixViolation::HeldWithIncompleteScorecard {
                entry_id: row.entry_id.clone(),
            });
        }

        // A public claim whose canonical label is below the cutline forces the lane
        // to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(DqParityMatrixViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(DqParityMatrixViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.scorecard_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed row carries exactly the public claim's canonical label,
            // carries no active gap reason, rides a captured within-SLO packet,
            // and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(DqParityMatrixViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(DqParityMatrixViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(DqParityMatrixViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(DqParityMatrixViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(DqParityMatrixViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(DqParityMatrixViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.scorecard_state,
                    published: row.published_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(DqParityMatrixViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.scorecard_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(DqGapReason::ProofPacketStale)
            {
                violations.push(DqParityMatrixViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(DqGapReason::ProofPacketMissing)
            {
                violations.push(DqParityMatrixViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);

        // Hard dependency check: if an upstream hard dependency is narrowed,
        // this row must have UpstreamLaneNarrowed or be narrowed itself.
        for edge in &self.dependencies {
            if edge.to_lane == row.entry_id && edge.dependency_kind == DqDependencyKind::Hard {
                if let Some(upstream) = self.row(&edge.from_lane) {
                    if !upstream.publishes_stable()
                        && !row.has_active_reason(DqGapReason::UpstreamLaneNarrowed)
                        && row.publishes_stable()
                    {
                        violations.push(DqParityMatrixViolation::UpstreamHardDependencyNarrowed {
                            entry_id: row.entry_id.clone(),
                            upstream_lane: upstream.entry_id.clone(),
                        });
                    }
                }
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &DqLaneRow,
        violations: &mut Vec<DqParityMatrixViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<DqParityMatrixViolation>,
                               expected: DqGapReason| {
            violations.push(DqParityMatrixViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.scorecard_state,
                expected_reason: expected,
            });
        };

        match row.scorecard_state {
            DqScorecardState::Incomplete => {
                if !row.has_active_reason(DqGapReason::ProofPacketMissing)
                    && !row.has_active_reason(DqGapReason::CompatibilityReportMissing)
                    && !row.has_active_reason(DqGapReason::AdminPolicyMissing)
                    && !row.has_active_reason(DqGapReason::RollbackPathMissing)
                {
                    push_incoherent(violations, DqGapReason::ProofPacketMissing);
                }
            }
            DqScorecardState::Stale => {
                if !(row.has_active_reason(DqGapReason::ProofPacketStale)
                    || row.has_active_reason(DqGapReason::CompatibilityReportStale))
                {
                    push_incoherent(violations, DqGapReason::ProofPacketStale);
                }
            }
            DqScorecardState::Blocked => {
                if !row.has_active_reason(DqGapReason::AdminPolicyMissing) {
                    push_incoherent(violations, DqGapReason::AdminPolicyMissing);
                }
            }
            DqScorecardState::RollbackMissing => {
                if !row.has_active_reason(DqGapReason::RollbackPathMissing) {
                    push_incoherent(violations, DqGapReason::RollbackPathMissing);
                }
            }
            DqScorecardState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(DqParityMatrixViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.scorecard_state,
                    });
                }
            }
            DqScorecardState::Complete => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<DqParityMatrixViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_lane_refs {
            if !covered.contains(declared) {
                violations.push(DqParityMatrixViolation::ReleaseBlockingSurfaceUncovered {
                    surface_ref: declared.clone(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_lane_refs.contains(&row.surface_ref) {
                violations.push(DqParityMatrixViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<DqParityMatrixViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(DqParityMatrixViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(DqParityMatrixViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(DqParityMatrixViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(DqParityMatrixViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(DqParityMatrixViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 dependency/quality parity matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DqParityMatrixViolation {
    /// The matrix carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the matrix.
        actual: u32,
    },
    /// The matrix carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the matrix.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The matrix has no rows.
    EmptyMatrix,
    /// The matrix has no stop rules.
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
        reason: DqGapReason,
    },
    /// An edge id appears more than once.
    DuplicateEdgeId {
        /// Duplicate edge id.
        edge_id: String,
    },
    /// A dependency edge is a self-loop.
    SelfLoop {
        /// Edge id.
        edge_id: String,
    },
    /// A dependency edge references an unknown lane.
    UnresolvedLaneRef {
        /// Edge id.
        edge_id: String,
        /// Unknown lane ref.
        lane_ref: String,
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
        /// Scorecard state.
        state: DqScorecardState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Scorecard state.
        state: DqScorecardState,
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
    /// A held row has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A held row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held row has an incomplete scorecard.
    HeldWithIncompleteScorecard {
        /// Row id.
        entry_id: String,
    },
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a breached packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A scorecard state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Scorecard state.
        state: DqScorecardState,
        /// Reason the state requires.
        expected_reason: DqGapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Scorecard state.
        state: DqScorecardState,
    },
    /// A row holds stable while a hard-dependency upstream lane is narrowed.
    UpstreamHardDependencyNarrowed {
        /// Row id.
        entry_id: String,
        /// Upstream lane id.
        upstream_lane: String,
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
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for DqParityMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported matrix schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported matrix record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "matrix {field} is not the canonical value")
            }
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
            Self::NoStopRules => write!(f, "matrix has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
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
            Self::DuplicateEdgeId { edge_id } => {
                write!(f, "duplicate edge id {edge_id}")
            }
            Self::SelfLoop { edge_id } => {
                write!(f, "dependency edge {edge_id} is a self-loop")
            }
            Self::UnresolvedLaneRef { edge_id, lane_ref } => {
                write!(
                    f,
                    "dependency edge {edge_id} references unknown lane {lane_ref}"
                )
            }
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "row {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "row {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "row {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "row {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "row {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(f, "row {entry_id} holds stable on stale packet {slo_state:?}")
            }
            Self::HeldWithIncompleteScorecard { entry_id } => {
                write!(f, "row {entry_id} holds stable with incomplete scorecard")
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds stable without owner signoff")
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} breached packet without proof_packet_stale reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} missing packet without proof_packet_missing reason"
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "row {entry_id} state {state:?} names no waiver")
            }
            Self::UpstreamHardDependencyNarrowed {
                entry_id,
                upstream_lane,
            } => write!(
                f,
                "row {entry_id} holds stable while hard-dependency upstream {upstream_lane} is narrowed"
            ),
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(f, "release-blocking surface {surface_ref} has no covering row")
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => {
                write!(
                    f,
                    "release-blocking row {entry_id} is not declared in release_blocking_lane_refs"
                )
            }
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(f, "promotion {declared:?} disagrees with computed {computed:?}")
            }
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for DqParityMatrixViolation {}

/// Loads the embedded M5 dependency/quality parity matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`DqParityMatrix`].
pub fn current_m5_dependency_quality_parity_matrix() -> Result<DqParityMatrix, serde_json::Error> {
    serde_json::from_str(FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> DqParityMatrix {
        current_m5_dependency_quality_parity_matrix().expect("matrix parses")
    }

    #[test]
    fn embedded_matrix_parses_and_validates() {
        let m = matrix();
        assert_eq!(
            m.schema_version,
            FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_SCHEMA_VERSION
        );
        assert_eq!(
            m.record_kind,
            FREEZE_M5_DEPENDENCY_QUALITY_PARITY_MATRIX_RECORD_KIND
        );
        assert_eq!(m.validate(), Vec::new());
        assert!(!m.rows.is_empty());
    }

    #[test]
    fn covers_every_lane_kind() {
        let m = matrix();
        for kind in DqLaneKind::ALL {
            assert!(
                !m.rows_for_kind(kind).is_empty(),
                "lane kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_row_names_class_profile_and_downgrade_rule() {
        let m = matrix();
        for row in &m.rows {
            assert!(
                !row.compatibility_downgrade_rule.trim().is_empty(),
                "{} must name a compatibility/downgrade rule",
                row.entry_id
            );
            assert!(DqClaimClass::ALL.contains(&row.claim_class));
            assert!(DeploymentProfile::ALL.contains(&row.deployment_profile));
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_surface() {
        let m = matrix();
        assert!(!m.release_blocking_lane_refs.is_empty());
        let covered: Vec<&str> = m
            .release_blocking_rows()
            .iter()
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &m.release_blocking_lane_refs {
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
    fn promotion_decision_matches_computed() {
        let m = matrix();
        assert_eq!(m.promotion.decision, m.computed_promotion_decision());
        assert_eq!(
            m.promotion.blocking_rule_ids,
            m.computed_blocking_rule_ids()
        );
        assert_eq!(
            m.promotion.blocking_claim_ids,
            m.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let m = matrix();
        let covered: BTreeSet<DqGapReason> = m
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in DqGapReason::ALL {
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
        row.active_gap_reasons.push(DqGapReason::ProofPacketMissing);
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, DqParityMatrixViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.scorecard_state = DqScorecardState::Incomplete;
        row.active_gap_reasons.push(DqGapReason::ProofPacketMissing);
        row.published_label = StableClaimLevel::Stable;
        m.summary = m.computed_summary();
        m.promotion.decision = m.computed_promotion_decision();
        m.promotion.blocking_rule_ids = m.computed_blocking_rule_ids();
        m.promotion.blocking_claim_ids = m.computed_blocking_entry_ids();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, DqParityMatrixViolation::PublishedLabelNotNarrowed { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_promotion_decision() {
        let mut m = matrix();
        m.promotion.decision = PromotionDecision::Proceed;
        assert!(m.validate().iter().any(|v| matches!(
            v,
            DqParityMatrixViolation::PromotionDecisionInconsistent { .. }
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
            .any(|v| matches!(v, DqParityMatrixViolation::HeldWithoutSignoff { .. })));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let m = matrix();
        let projection = m.support_export_projection();
        assert_eq!(projection.rows.len(), m.rows.len());
        for (row, proj) in m.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, proj.entry_id);
            assert_eq!(row.publishes_stable(), proj.publishes_stable);
            assert_eq!(row.claim_class, proj.claim_class);
            assert_eq!(row.deployment_profile, proj.deployment_profile);
        }
    }
}
