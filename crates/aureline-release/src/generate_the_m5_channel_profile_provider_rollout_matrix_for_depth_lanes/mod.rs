//! Typed M5 channel/profile/provider rollout matrix for depth lanes.
//!
//! This module generates the canonical rollout matrix that binds every M5 depth
//! lane to the channel, deployment profile, and provider family on which it may
//! ship. Each [`M5RolloutRow`] records:
//!
//! - the lane it governs ([`M5LaneKind`], reused from the M5 feature-train
//!   matrix),
//! - the rollout channel ([`RolloutChannel`]), deployment profile
//!   ([`DeploymentProfile`]), and provider family ([`ProviderFamily`]),
//! - the rollout state earned ([`RolloutState`]), the effective lifecycle label
//!   after narrowing ([`M5RolloutRow::effective_label`]),
//! - the active gap reasons ([`RolloutGapReason`]) that explain a narrowed or
//!   blocked row,
//! - the proof packet, owner sign-off, and optional waiver reused from the
//!   stable-claim vocabulary.
//!
//! The matrix is checked in at
//! `artifacts/release/m5/generate_the_m5_channel_profile_provider_rollout_matrix_for_depth_lanes.json`
//! and embedded here, so the typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph::M5LaneKind;
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported matrix schema version.
pub const GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_SCHEMA_VERSION:
    u32 = 1;

/// Stable record-kind tag for the matrix.
pub const GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_RECORD_KIND:
    &str = "generate_the_m5_channel_profile_provider_rollout_matrix_for_depth_lanes";

/// Repo-relative path to the checked-in matrix.
pub const GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_PATH: &str =
    "artifacts/release/m5/generate_the_m5_channel_profile_provider_rollout_matrix_for_depth_lanes.json";

/// Embedded checked-in matrix JSON.
pub const GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/generate_the_m5_channel_profile_provider_rollout_matrix_for_depth_lanes.json"
    ));

/// Rollout channel a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutChannel {
    /// Broad stable channel.
    Stable,
    /// Beta channel.
    Beta,
    /// Preview channel.
    Preview,
    /// Nightly channel.
    Nightly,
    /// Labs/experimental channel.
    Labs,
}

impl RolloutChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Nightly,
        Self::Labs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Nightly => "nightly",
            Self::Labs => "labs",
        }
    }
}

/// Deployment profile a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfile {
    /// Desktop profile.
    Desktop,
    /// Browser profile.
    Browser,
    /// Mobile profile.
    Mobile,
    /// Remote profile.
    Remote,
    /// Headless profile.
    Headless,
}

impl DeploymentProfile {
    /// Every profile, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Desktop,
        Self::Browser,
        Self::Mobile,
        Self::Remote,
        Self::Headless,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Browser => "browser",
            Self::Mobile => "mobile",
            Self::Remote => "remote",
            Self::Headless => "headless",
        }
    }
}

/// Provider family a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFamily {
    /// Aureline-hosted provider.
    AurelineHosted,
    /// Local-only provider.
    Local,
    /// Managed control plane.
    ManagedControlPlane,
    /// Third-party provider.
    ThirdParty,
}

impl ProviderFamily {
    /// Every provider family, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AurelineHosted,
        Self::Local,
        Self::ManagedControlPlane,
        Self::ThirdParty,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AurelineHosted => "aureline_hosted",
            Self::Local => "local",
            Self::ManagedControlPlane => "managed_control_plane",
            Self::ThirdParty => "third_party",
        }
    }
}

/// Rollout state a lane/channel/profile/provider tuple earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutState {
    /// Fully eligible for rollout at the effective label.
    Eligible,
    /// Eligible with a known degradation (for example, on a waiver).
    EligibleDegraded,
    /// Blocked from rollout at the effective label.
    Blocked,
    /// Pending evidence; rollout is gated until the corpus is refreshed.
    PendingEvidence,
    /// Explicitly not applicable for this tuple.
    NotApplicable,
}

impl RolloutState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Eligible,
        Self::EligibleDegraded,
        Self::Blocked,
        Self::PendingEvidence,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eligible => "eligible",
            Self::EligibleDegraded => "eligible_degraded",
            Self::Blocked => "blocked",
            Self::PendingEvidence => "pending_evidence",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the state lets the tuple carry its claim at the effective label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Eligible | Self::EligibleDegraded)
    }

    /// Whether the state forces the tuple below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a rollout tuple narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutGapReason {
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
    /// Channel policy blocks the rollout.
    ChannelBlocked,
    /// Profile is unsupported for the lane.
    ProfileUnsupported,
    /// Provider is not ready for the lane.
    ProviderNotReady,
    /// Upstream lane has narrowed below the cutline.
    LaneNarrowed,
    /// A waiver the tuple relied on has expired.
    WaiverExpired,
}

impl RolloutGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::CompatibilityReportMissing,
        Self::CompatibilityReportStale,
        Self::AdminPolicyMissing,
        Self::RollbackPathMissing,
        Self::ChannelBlocked,
        Self::ProfileUnsupported,
        Self::ProviderNotReady,
        Self::LaneNarrowed,
        Self::WaiverExpired,
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
            Self::ChannelBlocked => "channel_blocked",
            Self::ProfileUnsupported => "profile_unsupported",
            Self::ProviderNotReady => "provider_not_ready",
            Self::LaneNarrowed => "lane_narrowed",
            Self::WaiverExpired => "waiver_expired",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutAction {
    /// Hold rollout until the condition clears.
    HoldRollout,
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
    /// Widen to a less strict channel.
    WidenChannel,
    /// Restrict to a supported profile.
    RestrictProfile,
    /// Restrict to a ready provider.
    RestrictProvider,
}

impl RolloutAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::HoldRollout,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RefreshCompatibilityReport,
        Self::StaffAdminPolicy,
        Self::DefineRollbackPath,
        Self::WidenChannel,
        Self::RestrictProfile,
        Self::RestrictProvider,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldRollout => "hold_rollout",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RefreshCompatibilityReport => "refresh_compatibility_report",
            Self::StaffAdminPolicy => "staff_admin_policy",
            Self::DefineRollbackPath => "define_rollback_path",
            Self::WidenChannel => "widen_channel",
            Self::RestrictProfile => "restrict_profile",
            Self::RestrictProvider => "restrict_provider",
        }
    }
}

/// One rollout stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloutStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: RolloutGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: RolloutAction,
    /// Whether firing this rule blocks rollout.
    pub blocks_rollout: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 rollout row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RolloutRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The lane kind this row governs.
    pub lane_kind: M5LaneKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Rollout channel.
    pub channel: RolloutChannel,
    /// Deployment profile.
    pub profile: DeploymentProfile,
    /// Provider family.
    pub provider: ProviderFamily,
    /// Reviewable one-line statement of the row.
    pub surface_summary: String,
    /// Whether the tuple is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// Rollout state earned for the tuple.
    pub rollout_state: RolloutState,
    /// The lifecycle label the tuple effectively carries after narrowing.
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
    pub active_gap_reasons: Vec<RolloutGapReason>,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5RolloutRow {
    /// True when the effective label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the tuple carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.rollout_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: RolloutGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RolloutMatrixSummary {
    /// Total number of rollout rows.
    pub total_entries: usize,
    /// Rows publishing an effective label at or above the cutline.
    pub entries_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows in the eligible state.
    pub entries_eligible: usize,
    /// Rows in the eligible-degraded state.
    pub entries_eligible_degraded: usize,
    /// Rows in the blocked state.
    pub entries_blocked: usize,
    /// Rows in the pending-evidence state.
    pub entries_pending_evidence: usize,
    /// Rows in the not-applicable state.
    pub entries_not_applicable: usize,
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
    /// Stable-channel rows.
    pub stable_channel_entries: usize,
    /// Beta-channel rows.
    pub beta_channel_entries: usize,
    /// Preview-channel rows.
    pub preview_channel_entries: usize,
    /// Nightly-channel rows.
    pub nightly_channel_entries: usize,
    /// Labs-channel rows.
    pub labs_channel_entries: usize,
    /// Desktop-profile rows.
    pub desktop_profile_entries: usize,
    /// Browser-profile rows.
    pub browser_profile_entries: usize,
    /// Mobile-profile rows.
    pub mobile_profile_entries: usize,
    /// Remote-profile rows.
    pub remote_profile_entries: usize,
    /// Headless-profile rows.
    pub headless_profile_entries: usize,
    /// Aureline-hosted provider rows.
    pub aureline_hosted_entries: usize,
    /// Local provider rows.
    pub local_entries: usize,
    /// Managed-control-plane provider rows.
    pub managed_control_plane_entries: usize,
    /// Third-party provider rows.
    pub third_party_entries: usize,
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
pub struct M5RolloutExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The lane kind this row governs.
    pub lane_kind: M5LaneKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Rollout channel.
    pub channel: RolloutChannel,
    /// Deployment profile.
    pub profile: DeploymentProfile,
    /// Provider family.
    pub provider: ProviderFamily,
    /// Whether the tuple is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Rollout state earned.
    pub rollout_state: RolloutState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<RolloutGapReason>,
    /// Owner ref.
    pub owner_ref: String,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutExportProjection {
    /// Matrix identifier.
    pub matrix_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Rollout decision.
    pub rollout_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5RolloutExportRow>,
}

/// The typed M5 channel/profile/provider rollout matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ChannelProfileProviderRolloutMatrix {
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
    /// Ref to the M5 feature-train matrix this matrix builds on.
    pub feature_train_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed channel vocabulary.
    pub channels: Vec<RolloutChannel>,
    /// Closed profile vocabulary.
    pub profiles: Vec<DeploymentProfile>,
    /// Closed provider-family vocabulary.
    pub providers: Vec<ProviderFamily>,
    /// Closed rollout-state vocabulary.
    pub rollout_states: Vec<RolloutState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<RolloutGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<RolloutAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this matrix must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<RolloutStopRule>,
    /// Rollout rows.
    pub rows: Vec<M5RolloutRow>,
    /// Recorded rollout verdict.
    pub rollout: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5RolloutMatrixSummary,
}

impl M5ChannelProfileProviderRolloutMatrix {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5RolloutRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5RolloutRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5RolloutRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5RolloutRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one lane kind.
    pub fn rows_for_kind(&self, kind: M5LaneKind) -> Vec<&M5RolloutRow> {
        self.rows
            .iter()
            .filter(|row| row.lane_kind == kind)
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
    pub fn stop_rule_fires(&self, rule: &RolloutStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the rollout verdict from the rows and stop rules.
    pub fn computed_rollout_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_rollout && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block rollout and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_rollout && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Rollout-row ids that trigger a blocking, firing rule, sorted and unique.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<RolloutGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_rollout && self.stop_rule_fires(rule))
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
    pub fn computed_summary(&self) -> M5RolloutMatrixSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5LaneKind| self.rows_for_kind(kind).len();
        let channel = |channel: RolloutChannel| {
            self.rows
                .iter()
                .filter(|row| row.channel == channel)
                .count()
        };
        let profile = |profile: DeploymentProfile| {
            self.rows
                .iter()
                .filter(|row| row.profile == profile)
                .count()
        };
        let provider = |provider: ProviderFamily| {
            self.rows
                .iter()
                .filter(|row| row.provider == provider)
                .count()
        };
        let release_blocking: Vec<&M5RolloutRow> = self.release_blocking_rows();
        M5RolloutMatrixSummary {
            total_entries: self.rows.len(),
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
            entries_eligible: self
                .rows
                .iter()
                .filter(|row| row.rollout_state == RolloutState::Eligible)
                .count(),
            entries_eligible_degraded: self
                .rows
                .iter()
                .filter(|row| row.rollout_state == RolloutState::EligibleDegraded)
                .count(),
            entries_blocked: self
                .rows
                .iter()
                .filter(|row| row.rollout_state == RolloutState::Blocked)
                .count(),
            entries_pending_evidence: self
                .rows
                .iter()
                .filter(|row| row.rollout_state == RolloutState::PendingEvidence)
                .count(),
            entries_not_applicable: self
                .rows
                .iter()
                .filter(|row| row.rollout_state == RolloutState::NotApplicable)
                .count(),
            entries_on_active_waiver: self.rows.iter().filter(|row| row.waiver.is_some()).count(),
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
            notebook_entries: kind(M5LaneKind::Notebook),
            data_rich_entries: kind(M5LaneKind::DataRich),
            ai_adjacent_entries: kind(M5LaneKind::AiAdjacent),
            framework_entries: kind(M5LaneKind::Framework),
            review_entries: kind(M5LaneKind::Review),
            companion_entries: kind(M5LaneKind::Companion),
            managed_depth_entries: kind(M5LaneKind::ManagedDepth),
            stable_channel_entries: channel(RolloutChannel::Stable),
            beta_channel_entries: channel(RolloutChannel::Beta),
            preview_channel_entries: channel(RolloutChannel::Preview),
            nightly_channel_entries: channel(RolloutChannel::Nightly),
            labs_channel_entries: channel(RolloutChannel::Labs),
            desktop_profile_entries: profile(DeploymentProfile::Desktop),
            browser_profile_entries: profile(DeploymentProfile::Browser),
            mobile_profile_entries: profile(DeploymentProfile::Mobile),
            remote_profile_entries: profile(DeploymentProfile::Remote),
            headless_profile_entries: profile(DeploymentProfile::Headless),
            aureline_hosted_entries: provider(ProviderFamily::AurelineHosted),
            local_entries: provider(ProviderFamily::Local),
            managed_control_plane_entries: provider(ProviderFamily::ManagedControlPlane),
            third_party_entries: provider(ProviderFamily::ThirdParty),
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

    /// Produces an export/Help-About-safe projection of the matrix that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5RolloutExportProjection {
        M5RolloutExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            rollout_decision: self.rollout.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5RolloutExportRow {
                    entry_id: row.entry_id.clone(),
                    lane_kind: row.lane_kind,
                    surface_ref: row.surface_ref.clone(),
                    channel: row.channel,
                    profile: row.profile,
                    provider: row.provider,
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    publishes_stable: row.publishes_stable(),
                    rollout_state: row.rollout_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    owner_ref: row.owner_signoff.owner_ref.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<M5RolloutMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5RolloutMatrixViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5RolloutMatrixViolation::EmptyMatrix);
        }

        self.validate_coverage(&mut violations);
        self.validate_rollout(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5RolloutMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5RolloutMatrixViolation>) {
        if self.schema_version
            != GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_SCHEMA_VERSION
        {
            violations.push(M5RolloutMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_RECORD_KIND
        {
            violations.push(M5RolloutMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("feature_train_matrix_ref", &self.feature_train_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RolloutMatrixViolation::EmptyField {
                    entry_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5RolloutMatrixViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.channels != RolloutChannel::ALL.to_vec() {
            violations
                .push(M5RolloutMatrixViolation::ClosedVocabularyMismatch { field: "channels" });
        }
        if self.profiles != DeploymentProfile::ALL.to_vec() {
            violations
                .push(M5RolloutMatrixViolation::ClosedVocabularyMismatch { field: "profiles" });
        }
        if self.providers != ProviderFamily::ALL.to_vec() {
            violations
                .push(M5RolloutMatrixViolation::ClosedVocabularyMismatch { field: "providers" });
        }
        if self.rollout_states != RolloutState::ALL.to_vec() {
            violations.push(M5RolloutMatrixViolation::ClosedVocabularyMismatch {
                field: "rollout_states",
            });
        }
        if self.gap_reasons != RolloutGapReason::ALL.to_vec() {
            violations.push(M5RolloutMatrixViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != RolloutAction::ALL.to_vec() {
            violations.push(M5RolloutMatrixViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5RolloutMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5RolloutMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5RolloutMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5RolloutMatrixViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5RolloutMatrixViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5RolloutMatrixViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5RolloutMatrixViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5RolloutMatrixViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5RolloutMatrixViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in RolloutGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(M5RolloutMatrixViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &M5RolloutRow, violations: &mut Vec<M5RolloutMatrixViolation>) {
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
                violations.push(M5RolloutMatrixViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(M5RolloutMatrixViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.effective_label,
            });
        }

        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5RolloutMatrixViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5RolloutMatrixViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5RolloutMatrixViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5RolloutMatrixViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.rollout_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            if row.effective_label != row.claim_label {
                violations.push(M5RolloutMatrixViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.effective_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(M5RolloutMatrixViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5RolloutMatrixViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5RolloutMatrixViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5RolloutMatrixViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            if row.publishes_stable() {
                violations.push(M5RolloutMatrixViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.rollout_state,
                    published: row.effective_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5RolloutMatrixViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.rollout_state,
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(RolloutGapReason::ProofPacketStale)
            {
                violations.push(M5RolloutMatrixViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(RolloutGapReason::ProofPacketMissing)
            {
                violations.push(M5RolloutMatrixViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &M5RolloutRow,
        violations: &mut Vec<M5RolloutMatrixViolation>,
    ) {
        match row.rollout_state {
            RolloutState::Eligible => {
                if !row.active_gap_reasons.is_empty() {
                    violations.push(M5RolloutMatrixViolation::StateReasonIncoherent {
                        entry_id: row.entry_id.clone(),
                        state: row.rollout_state,
                        expected_reason: RolloutGapReason::ProofPacketMissing,
                    });
                }
            }
            RolloutState::EligibleDegraded => {
                if !row.active_gap_reasons.is_empty() {
                    violations.push(M5RolloutMatrixViolation::StateReasonIncoherent {
                        entry_id: row.entry_id.clone(),
                        state: row.rollout_state,
                        expected_reason: RolloutGapReason::ProofPacketMissing,
                    });
                }
            }
            RolloutState::Blocked => {
                if row.active_gap_reasons.is_empty() {
                    violations.push(M5RolloutMatrixViolation::StateReasonIncoherent {
                        entry_id: row.entry_id.clone(),
                        state: row.rollout_state,
                        expected_reason: RolloutGapReason::ProofPacketMissing,
                    });
                }
            }
            RolloutState::PendingEvidence => {
                let has_pending_reason = row
                    .has_active_reason(RolloutGapReason::ProofPacketMissing)
                    || row.has_active_reason(RolloutGapReason::CompatibilityReportMissing)
                    || row.has_active_reason(RolloutGapReason::AdminPolicyMissing)
                    || row.has_active_reason(RolloutGapReason::RollbackPathMissing);
                if !has_pending_reason {
                    violations.push(M5RolloutMatrixViolation::StateReasonIncoherent {
                        entry_id: row.entry_id.clone(),
                        state: row.rollout_state,
                        expected_reason: RolloutGapReason::ProofPacketMissing,
                    });
                }
            }
            RolloutState::NotApplicable => {
                if row.effective_label != StableClaimLevel::Withdrawn {
                    violations.push(M5RolloutMatrixViolation::StateReasonIncoherent {
                        entry_id: row.entry_id.clone(),
                        state: row.rollout_state,
                        expected_reason: RolloutGapReason::ProofPacketMissing,
                    });
                }
            }
        }

        if row.waiver.is_some()
            && row
                .waiver
                .as_ref()
                .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                .unwrap_or(true)
        {
            violations.push(M5RolloutMatrixViolation::WaiverStateWithoutWaiver {
                entry_id: row.entry_id.clone(),
                state: row.rollout_state,
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5RolloutMatrixViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_lane_refs {
            if !covered.contains(declared) {
                violations.push(M5RolloutMatrixViolation::ReleaseBlockingSurfaceUncovered {
                    surface_ref: declared.clone(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_lane_refs.contains(&row.surface_ref) {
                violations.push(M5RolloutMatrixViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_rollout(&self, violations: &mut Vec<M5RolloutMatrixViolation>) {
        if self.rollout.promotion_gate.trim().is_empty() {
            violations.push(M5RolloutMatrixViolation::EmptyField {
                entry_id: "<rollout>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.rollout.rationale.trim().is_empty() {
            violations.push(M5RolloutMatrixViolation::EmptyField {
                entry_id: "<rollout>".to_owned(),
                field_name: "rollout.rationale",
            });
        }
        let computed = self.computed_rollout_decision();
        if self.rollout.decision != computed {
            violations.push(M5RolloutMatrixViolation::RolloutDecisionInconsistent {
                declared: self.rollout.decision,
                computed,
            });
        }
        if self.rollout.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(M5RolloutMatrixViolation::RolloutBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.rollout.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(M5RolloutMatrixViolation::RolloutBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 rollout matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RolloutMatrixViolation {
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
        reason: RolloutGapReason,
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
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Rollout state.
        state: RolloutState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Rollout state.
        state: RolloutState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held row holds label while the public claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
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
    /// A rollout state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Rollout state.
        state: RolloutState,
        /// Reason the state requires.
        expected_reason: RolloutGapReason,
    },
    /// A waiver-bearing state names no valid waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Rollout state.
        state: RolloutState,
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
    /// The declared rollout decision disagrees with the computed one.
    RolloutDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared rollout blocking set disagrees with the computed one.
    RolloutBlockingSetMismatch {
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

impl fmt::Display for M5RolloutMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported rollout matrix schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported rollout matrix record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "rollout matrix {field} is not the canonical value")
            }
            Self::EmptyMatrix => write!(f, "rollout matrix has no rows"),
            Self::NoStopRules => write!(f, "rollout matrix has no stop rules"),
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
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} published level {published:?} is wider than claim {claim:?}"
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
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "row {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds stable without owner signoff")
            }
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
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "row {entry_id} holds label while claim {claim:?} is below cutline"
            ),
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
                write!(f, "row {entry_id} state {state:?} names no valid waiver")
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
                    "release-blocking row {entry_id} is not declared in release_blocking_lane_refs"
                )
            }
            Self::RolloutDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "rollout {declared:?} disagrees with computed {computed:?}"
                )
            }
            Self::RolloutBlockingSetMismatch { field } => {
                write!(f, "rollout {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for M5RolloutMatrixViolation {}

/// Loads the embedded M5 channel/profile/provider rollout matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`M5ChannelProfileProviderRolloutMatrix`].
pub fn current_m5_channel_profile_provider_rollout_matrix(
) -> Result<M5ChannelProfileProviderRolloutMatrix, serde_json::Error> {
    serde_json::from_str(
        GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> M5ChannelProfileProviderRolloutMatrix {
        current_m5_channel_profile_provider_rollout_matrix().expect("matrix parses")
    }

    #[test]
    fn embedded_matrix_parses_and_validates() {
        let m = matrix();
        assert_eq!(
            m.schema_version,
            GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_SCHEMA_VERSION
        );
        assert_eq!(
            m.record_kind,
            GENERATE_THE_M5_CHANNEL_PROFILE_PROVIDER_ROLLOUT_MATRIX_FOR_DEPTH_LANES_RECORD_KIND
        );
        assert_eq!(m.validate(), Vec::new());
        assert!(!m.rows.is_empty());
    }

    #[test]
    fn covers_every_lane_kind() {
        let m = matrix();
        for kind in M5LaneKind::ALL {
            assert!(
                !m.rows_for_kind(kind).is_empty(),
                "lane kind {} must have at least one row",
                kind.as_str()
            );
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
    fn rollout_decision_matches_computed() {
        let m = matrix();
        assert_eq!(m.rollout.decision, m.computed_rollout_decision());
        assert_eq!(m.rollout.blocking_rule_ids, m.computed_blocking_rule_ids());
        assert_eq!(
            m.rollout.blocking_claim_ids,
            m.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let m = matrix();
        let covered: BTreeSet<RolloutGapReason> = m
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in RolloutGapReason::ALL {
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
            .push(RolloutGapReason::ProofPacketMissing);
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, M5RolloutMatrixViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| {
                row.rollout_state == RolloutState::PendingEvidence
                    && row.claim_label == StableClaimLevel::Stable
            })
            .expect("a pending-evidence row under a stable ceiling exists");
        row.effective_label = StableClaimLevel::Stable;
        m.summary = m.computed_summary();
        m.rollout.decision = m.computed_rollout_decision();
        m.rollout.blocking_rule_ids = m.computed_blocking_rule_ids();
        m.rollout.blocking_claim_ids = m.computed_blocking_entry_ids();
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5RolloutMatrixViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_rollout_decision() {
        let mut m = matrix();
        m.rollout.decision = PromotionDecision::Proceed;
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5RolloutMatrixViolation::RolloutDecisionInconsistent { .. }
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
            .any(|v| matches!(v, M5RolloutMatrixViolation::HeldWithoutSignoff { .. })));
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
