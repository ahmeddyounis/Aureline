//! Continuity-proof freshness SLO dashboard, drill rerun paths, and shiproom
//! promotion blockers for claimed managed, self-hosted, and sovereign rows.
//!
//! Where the
//! [`m5_locality_tenant_keymode_and_drill_matrix`](crate::m5_locality_tenant_keymode_and_drill_matrix)
//! freezes *what* each continuity-claim row discloses (locality, tenant/key
//! posture, continuity packet family, restore identity, partial loss, drill
//! cadence/owner), this module freezes *how fresh* the evidence behind that
//! claim still is. It turns one-time continuity-packet generation into ongoing
//! evidence freshness with visible ownership and expiry, and emits one freshness
//! signal that shiproom gating, docs/public-truth publication, and support
//! exports all read instead of re-deriving staleness by hand.
//!
//! The model answers, for every claimed continuity row:
//!
//! 1. Which continuity proof packet backs this row, when was it last captured or
//!    drilled, and under what freshness SLO (target max age, warn window) does it
//!    stay claim-bearing?
//! 2. Is that packet [`Current`], [`DueForRefresh`], [`Breached`], or
//!    [`Missing`] against the dashboard evaluation clock?
//! 3. Can the evidence be regenerated or refreshed automatically — and which
//!    rerun path proves it — or does it need manual artifact surgery?
//! 4. When the packet breaches its SLO, is missing, lacks a drill-owner
//!    sign-off, or has no rerun path, does the claim narrow below stable and does
//!    a shiproom stop rule hold promotion — *without* ever blocking the local-core
//!    continuity lane?
//!
//! The freshness-SLO state ([`ContinuityFreshnessSloState`]) and the closed
//! narrowing/stop vocabulary deliberately reuse the same tokens the release
//! claim-manifest and shiproom dashboard already publish, so a continuity row
//! that ages past its SLO narrows through the same automation as a stale
//! qualification, publication, or support row.
//!
//! Two classes of check live outside this typed model because they need more
//! than the dashboard sees: date arithmetic (recomputing each packet's
//! freshness-SLO state from `captured_at` against the dashboard `as_of` date) and
//! the rerun rehearsal itself live in the freshness tooling and the CI gate. This
//! model enforces every structural and logical invariant that holds regardless of
//! the clock — freshness-window consistency, packet capture/state coherence,
//! the rerun-path declaration every release-scope row owes, the no-widening
//! narrowing rule, the local-core guardrail, stop-rule wiring, and the promotion
//! verdict.
//!
//! The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
//! labels, UTC dates, and opaque refs. Raw backup bytes, raw drill logs, raw KMS
//! handles, and secret material never cross this boundary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    ClaimSurfaceVisibility, ContinuityClaimQualificationClass, ContinuityLaneClass,
    ContinuityPacketFamilyClass, ContinuityProfileClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const CONTINUITY_FRESHNESS_SLO_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const CONTINUITY_FRESHNESS_SLO_SHARED_CONTRACT_REF: &str =
    "continuity:m5_continuity_freshness_slo:v1";

/// Record-kind tag for [`ContinuityFreshnessSloDashboard`] payloads.
pub const CONTINUITY_FRESHNESS_SLO_DASHBOARD_RECORD_KIND: &str =
    "continuity_freshness_slo_dashboard_record";

/// Record-kind tag for [`ContinuityFreshnessSloSummary`] payloads.
pub const CONTINUITY_FRESHNESS_SLO_SUMMARY_RECORD_KIND: &str =
    "continuity_freshness_slo_summary_record";

/// Record-kind tag for [`ContinuityFreshnessRowOutcome`] payloads.
pub const CONTINUITY_FRESHNESS_ROW_OUTCOME_RECORD_KIND: &str =
    "continuity_freshness_row_outcome_record";

/// Record-kind tag for [`ContinuityFreshnessDefect`] payloads.
pub const CONTINUITY_FRESHNESS_DEFECT_RECORD_KIND: &str = "continuity_freshness_defect_record";

/// Record-kind tag for [`ContinuityFreshnessSloSupportExport`] payloads.
pub const CONTINUITY_FRESHNESS_SLO_SUPPORT_EXPORT_RECORD_KIND: &str =
    "continuity_freshness_slo_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const CONTINUITY_FRESHNESS_SLO_DOC_REF: &str = "docs/release/m5-continuity-shiproom-gates.md";

/// Repo-relative path of the checked-in dashboard artifact for this lane.
pub const CONTINUITY_FRESHNESS_SLO_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/freshness_slo_dashboard.json";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const CONTINUITY_FRESHNESS_SLO_SCHEMA_REF: &str =
    "schemas/continuity/continuity_freshness_slo_dashboard.schema.json";

/// The freshness-SLO state a continuity proof packet earns against its target age.
///
/// `current` and `due_for_refresh` are both within the SLO; `breached` and
/// `missing` are outside it and force a claim to narrow below the stable cutline.
/// The tokens are identical to the release claim-manifest and shiproom dashboard
/// freshness states so one signal flows across every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityFreshnessSloState {
    /// Captured or drilled well within the freshness SLO.
    Current,
    /// Within the SLO but inside the warn window; a rerun is due soon.
    DueForRefresh,
    /// Age exceeds the SLO target; the packet is stale and the claim narrows.
    Breached,
    /// No continuity proof packet has been captured.
    Missing,
}

impl ContinuityFreshnessSloState {
    /// Every state, freshest to stalest.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::DueForRefresh,
        Self::Breached,
        Self::Missing,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DueForRefresh => "due_for_refresh",
            Self::Breached => "breached",
            Self::Missing => "missing",
        }
    }

    /// Freshness rank; a fresher state ranks higher. The CI gate uses this to
    /// fail a packet whose declared state is fresher than the clock allows.
    pub const fn freshness_rank(self) -> u8 {
        match self {
            Self::Current => 3,
            Self::DueForRefresh => 2,
            Self::Breached => 1,
            Self::Missing => 0,
        }
    }

    /// True when the packet is within its freshness SLO (current or due-soon).
    pub const fn is_within_slo(self) -> bool {
        matches!(self, Self::Current | Self::DueForRefresh)
    }

    /// True when the packet is outside its freshness SLO and forces narrowing.
    pub const fn forces_narrowing(self) -> bool {
        !self.is_within_slo()
    }

    /// True when a captured proof packet must back this state.
    pub const fn requires_capture(self) -> bool {
        !matches!(self, Self::Missing)
    }
}

/// The freshness state a continuity-claim row earns once owner and rerun posture
/// are folded in with the packet's SLO state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityFreshnessRowState {
    /// Packet current, owned, and rerun-capable; holds the claimed label.
    Fresh,
    /// Within SLO but inside the warn window; holds the label, flagged for rerun.
    DueForRefresh,
    /// The proof packet breached its freshness SLO; the claim narrows.
    NarrowedStale,
    /// No proof packet has been captured; the claim narrows to preview.
    NarrowedMissing,
    /// The drill-owner sign-off is missing; the claim narrows.
    NarrowedUnowned,
}

impl ContinuityFreshnessRowState {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::DueForRefresh => "due_for_refresh",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedMissing => "narrowed_missing",
            Self::NarrowedUnowned => "narrowed_unowned",
        }
    }

    /// Whether the state lets a row publish its claimed lifecycle label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Fresh | Self::DueForRefresh)
    }

    /// Whether the state forces the row below its claimed label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }

    /// The qualification floor a narrowed state imposes, if any.
    fn narrowed_floor(self) -> Option<ContinuityClaimQualificationClass> {
        match self {
            Self::Fresh | Self::DueForRefresh => None,
            Self::NarrowedStale | Self::NarrowedUnowned => {
                Some(ContinuityClaimQualificationClass::Beta)
            }
            Self::NarrowedMissing => Some(ContinuityClaimQualificationClass::Preview),
        }
    }
}

/// Closed reason a continuity claim narrows or a shiproom stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityStopReason {
    /// The continuity proof packet breached its freshness SLO.
    ContinuityPacketFreshnessBreached,
    /// No continuity proof packet has been captured.
    ContinuityPacketMissing,
    /// The required drill-owner sign-off is missing.
    DrillOwnerSignoffMissing,
    /// The row has no rerun path; evidence cannot be regenerated without surgery.
    RerunPathUnavailable,
    /// The backing continuity evidence is unqualified or profile-mismatched.
    ContinuityEvidenceUnqualified,
}

impl ContinuityStopReason {
    /// Every reason, in declaration order. Each must be watched by a stop rule.
    pub const ALL: [Self; 5] = [
        Self::ContinuityPacketFreshnessBreached,
        Self::ContinuityPacketMissing,
        Self::DrillOwnerSignoffMissing,
        Self::RerunPathUnavailable,
        Self::ContinuityEvidenceUnqualified,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuityPacketFreshnessBreached => "continuity_packet_freshness_breached",
            Self::ContinuityPacketMissing => "continuity_packet_missing",
            Self::DrillOwnerSignoffMissing => "drill_owner_signoff_missing",
            Self::RerunPathUnavailable => "rerun_path_unavailable",
            Self::ContinuityEvidenceUnqualified => "continuity_evidence_unqualified",
        }
    }
}

/// Default action a shiproom stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityStopAction {
    /// Hold promotion until the continuity evidence clears.
    HoldPromotion,
    /// Narrow the published continuity claim below the cutline.
    NarrowContinuityClaim,
    /// Rerun the continuity drill so fresh evidence is captured.
    RerunContinuityDrill,
    /// Refresh the continuity proof packet so it re-enters its freshness SLO.
    RefreshContinuityPacket,
    /// Obtain the required drill-owner sign-off.
    RequestDrillOwnerSignoff,
}

impl ContinuityStopAction {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowContinuityClaim => "narrow_continuity_claim",
            Self::RerunContinuityDrill => "rerun_continuity_drill",
            Self::RefreshContinuityPacket => "refresh_continuity_packet",
            Self::RequestDrillOwnerSignoff => "request_drill_owner_signoff",
        }
    }
}

/// How a continuity proof packet can be regenerated or refreshed.
///
/// A release-scope row needs an automatable rerun path so freshness can be
/// restored without manual artifact surgery; a row with no rerun path narrows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunAutomationClass {
    /// Regenerated end-to-end by automated tooling.
    AutomatedRerun,
    /// Refreshed by a scripted, repeatable path (no manual artifact surgery).
    ScriptedRefresh,
    /// Requires a manual runbook; insufficient on its own to keep a claim fresh.
    ManualRunbookOnly,
    /// No rerun path exists; the claim cannot be refreshed without surgery.
    NoRerunPath,
}

impl RerunAutomationClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedRerun => "automated_rerun",
            Self::ScriptedRefresh => "scripted_refresh",
            Self::ManualRunbookOnly => "manual_runbook_only",
            Self::NoRerunPath => "no_rerun_path",
        }
    }

    /// True when the path can refresh evidence without manual artifact surgery.
    pub const fn is_automatable(self) -> bool {
        matches!(self, Self::AutomatedRerun | Self::ScriptedRefresh)
    }
}

/// The freshness SLO for a continuity proof packet: how long it stays
/// claim-bearing before it must be rerun.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFreshnessSlo {
    /// The SLO target: the packet may be at most this many days old.
    pub target_max_age_days: u32,
    /// Days-remaining threshold at or below which the packet is `due_for_refresh`.
    pub warn_within_days: u32,
    /// Opaque ref into the freshness-SLO register that defines this target.
    pub slo_register_ref: String,
}

impl ContinuityFreshnessSlo {
    /// True when the warn window does not exceed the target age.
    pub const fn window_is_consistent(&self) -> bool {
        self.warn_within_days <= self.target_max_age_days
    }
}

/// The continuity proof packet backing a row, with its freshness SLO and state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityProofPacket {
    /// Stable packet id.
    pub packet_id: String,
    /// Opaque ref to the reviewer-facing continuity proof packet.
    pub packet_ref: String,
    /// Continuity packet family this proof packet belongs to.
    pub packet_family: ContinuityPacketFamilyClass,
    /// Stable token for [`Self::packet_family`].
    pub packet_family_token: String,
    /// UTC date the packet was last captured or drilled; `None` when never run.
    #[serde(default)]
    pub captured_at: Option<String>,
    /// The freshness SLO this packet ages under.
    pub freshness_slo: ContinuityFreshnessSlo,
    /// The freshness-SLO state earned against the dashboard clock.
    pub slo_state: ContinuityFreshnessSloState,
    /// Stable token for [`Self::slo_state`].
    pub slo_state_token: String,
    /// Opaque evidence refs carried by the packet. Empty only on uncaptured packets.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ContinuityProofPacket {
    /// True when the packet has a capture date and at least one evidence ref.
    pub fn has_capture(&self) -> bool {
        self.captured_at.is_some() && !self.evidence_refs.is_empty()
    }
}

/// The rerun path that proves a row's continuity evidence can be regenerated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityRerunPath {
    /// How the evidence is regenerated or refreshed.
    pub rerun_class: RerunAutomationClass,
    /// Stable token for [`Self::rerun_class`].
    pub rerun_class_token: String,
    /// Opaque ref to the rerun tool or command (empty only when none exists).
    pub rerun_command_ref: String,
    /// UTC date the rerun path last refreshed this row, empty when never.
    pub last_rerun_at: String,
}

impl ContinuityRerunPath {
    /// True when the rerun path can refresh evidence without manual surgery.
    pub fn is_automatable(&self) -> bool {
        self.rerun_class.is_automatable()
    }

    /// True when the path names a concrete rerun tool or command.
    pub fn is_declared(&self) -> bool {
        !self.rerun_command_ref.is_empty()
    }
}

/// One continuity-claim row tracked for freshness on the dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFreshnessRow {
    /// Opaque row identifier; matches the continuity-claim matrix row id.
    pub row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Claimed deployment profile.
    pub profile_class: ContinuityProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_class_token: String,
    /// Continuity lane this row belongs to.
    pub continuity_lane: ContinuityLaneClass,
    /// Stable token for [`Self::continuity_lane`].
    pub continuity_lane_token: String,
    /// The lifecycle label the row is put forward as.
    pub claimed_qualification: ContinuityClaimQualificationClass,
    /// Stable token for [`Self::claimed_qualification`].
    pub claimed_qualification_token: String,
    /// The continuity proof packet backing the row.
    pub proof_packet: ContinuityProofPacket,
    /// The rerun path that refreshes the row's evidence.
    pub rerun: ContinuityRerunPath,
    /// Export-safe label naming the current drill owner.
    pub drill_owner_label: String,
    /// Export-safe label naming the future or backup drill owner.
    pub future_drill_owner_label: String,
    /// True when the current drill owner has signed off on the latest evidence.
    pub owner_signoff_present: bool,
    /// True when the backing continuity evidence is unqualified or profile-mismatched.
    pub evidence_unqualified: bool,
    /// Surfaces that reuse this row's one freshness signal.
    pub surface_visibility: ClaimSurfaceVisibility,
}

impl ContinuityFreshnessRow {
    /// True when this row rides the local-core continuity lane.
    ///
    /// A local-core row keeps continuity without any managed lane, so its claim
    /// never narrows or blocks promotion on managed freshness — the guardrail
    /// against conflating a stale managed row with the local core.
    pub fn is_local_core(&self) -> bool {
        self.continuity_lane == ContinuityLaneClass::LocalCore
            && self.profile_class == ContinuityProfileClass::LocalOnly
    }

    /// True when this row is held to managed-lane freshness and may block promotion.
    pub fn in_release_scope(&self) -> bool {
        !self.is_local_core()
    }

    /// True when the row's claimed label is at or above the stable/beta cutline.
    fn claim_above_cutline(&self) -> bool {
        matches!(
            self.claimed_qualification,
            ContinuityClaimQualificationClass::Stable | ContinuityClaimQualificationClass::Beta
        )
    }

    /// The freshness state this row earns from its packet, owner, and rerun posture.
    fn row_state(&self) -> ContinuityFreshnessRowState {
        if !self.in_release_scope() {
            // Local-core rows ride their own lane and never narrow on managed
            // freshness; they report the packet's own SLO state directly.
            return match self.proof_packet.slo_state {
                ContinuityFreshnessSloState::DueForRefresh => {
                    ContinuityFreshnessRowState::DueForRefresh
                }
                _ => ContinuityFreshnessRowState::Fresh,
            };
        }
        if self.proof_packet.slo_state == ContinuityFreshnessSloState::Missing {
            return ContinuityFreshnessRowState::NarrowedMissing;
        }
        if self.proof_packet.slo_state == ContinuityFreshnessSloState::Breached {
            return ContinuityFreshnessRowState::NarrowedStale;
        }
        if !self.owner_signoff_present {
            return ContinuityFreshnessRowState::NarrowedUnowned;
        }
        match self.proof_packet.slo_state {
            ContinuityFreshnessSloState::DueForRefresh => {
                ContinuityFreshnessRowState::DueForRefresh
            }
            _ => ContinuityFreshnessRowState::Fresh,
        }
    }

    /// The stop reasons this row carries, narrowest concern first.
    fn stop_reasons(&self) -> Vec<ContinuityStopReason> {
        let mut reasons = Vec::new();
        if !self.in_release_scope() {
            return reasons;
        }
        match self.proof_packet.slo_state {
            ContinuityFreshnessSloState::Missing => {
                reasons.push(ContinuityStopReason::ContinuityPacketMissing)
            }
            ContinuityFreshnessSloState::Breached => {
                reasons.push(ContinuityStopReason::ContinuityPacketFreshnessBreached)
            }
            _ => {}
        }
        if !self.owner_signoff_present {
            reasons.push(ContinuityStopReason::DrillOwnerSignoffMissing);
        }
        if self.rerun.rerun_class == RerunAutomationClass::NoRerunPath {
            reasons.push(ContinuityStopReason::RerunPathUnavailable);
        }
        if self.evidence_unqualified {
            reasons.push(ContinuityStopReason::ContinuityEvidenceUnqualified);
        }
        reasons
    }

    /// The lifecycle label this row effectively publishes after narrowing.
    fn effective_qualification(&self) -> ContinuityClaimQualificationClass {
        let mut effective = self.claimed_qualification;
        if let Some(floor) = self.row_state().narrowed_floor() {
            effective = effective.max(floor);
        }
        if self.in_release_scope() {
            if self.rerun.rerun_class == RerunAutomationClass::NoRerunPath {
                effective = effective.max(ContinuityClaimQualificationClass::Beta);
            }
            if self.evidence_unqualified {
                effective = effective.max(ContinuityClaimQualificationClass::Preview);
            }
        }
        effective
    }

    /// True when this row holds promotion: a release-scope row at or above the
    /// cutline that carries at least one stop reason.
    fn blocks_promotion(&self) -> bool {
        self.in_release_scope() && self.claim_above_cutline() && !self.stop_reasons().is_empty()
    }
}

/// One shiproom stop rule: a closed condition that narrows a continuity claim and
/// may hold promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The stop reason whose presence on a claimed row fires this rule.
    pub trigger_reason: ContinuityStopReason,
    /// Stable token for [`Self::trigger_reason`].
    pub trigger_reason_token: String,
    /// Claimed lifecycle labels this rule watches.
    pub applies_to_qualification_tokens: Vec<String>,
    /// Default action prescribed when the rule fires.
    pub default_action: ContinuityStopAction,
    /// Stable token for [`Self::default_action`].
    pub default_action_token: String,
    /// Whether firing this rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// Per-row freshness verdict joining a row to its computed state and narrowing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFreshnessRowOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque row identifier this outcome describes.
    pub row_id: String,
    /// Stable token for the row's claimed profile.
    pub profile_class_token: String,
    /// True when the row is held to managed-lane freshness and may block promotion.
    pub in_release_scope: bool,
    /// True when the backing packet is within its freshness SLO.
    pub within_slo: bool,
    /// Stable token for the packet's freshness-SLO state.
    pub slo_state_token: String,
    /// Stable token for the freshness state the row earned.
    pub row_state_token: String,
    /// Stable token for the label the row is put forward as.
    pub claimed_qualification_token: String,
    /// Stable token for the label the row effectively publishes after narrowing.
    pub effective_qualification_token: String,
    /// True when the row narrowed below its claimed label.
    pub narrowed: bool,
    /// True when the row holds promotion.
    pub blocks_promotion: bool,
    /// True when the row's evidence can be refreshed without manual surgery.
    pub rerun_automatable: bool,
    /// Stable stop-reason tokens active on the row.
    pub active_stop_reason_tokens: Vec<String>,
}

/// Closed defect kind emitted by the freshness audit for malformed input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityFreshnessDefectKind {
    /// A packet's warn window exceeds its target max age.
    FreshnessWindowInconsistent,
    /// A packet's declared SLO state disagrees with whether it has a capture.
    PacketStateCaptureIncoherent,
    /// A release-scope row names no rerun path tool or command.
    RerunPathUndeclared,
    /// A local-core row was marked as holding promotion, violating the guardrail.
    LocalCoreMarkedBlocking,
    /// A stop reason has no stop rule watching for it.
    StopReasonUncovered,
    /// The promotion verdict disagrees with the rows that hold promotion.
    PromotionVerdictIncoherent,
}

impl ContinuityFreshnessDefectKind {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshnessWindowInconsistent => "freshness_window_inconsistent",
            Self::PacketStateCaptureIncoherent => "packet_state_capture_incoherent",
            Self::RerunPathUndeclared => "rerun_path_undeclared",
            Self::LocalCoreMarkedBlocking => "local_core_marked_blocking",
            Self::StopReasonUncovered => "stop_reason_uncovered",
            Self::PromotionVerdictIncoherent => "promotion_verdict_incoherent",
        }
    }
}

/// Typed defect emitted by the freshness audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFreshnessDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed defect kind.
    pub defect_kind: ContinuityFreshnessDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Opaque source row id or dashboard concern that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl ContinuityFreshnessDefect {
    fn new(
        defect_kind: ContinuityFreshnessDefectKind,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: CONTINUITY_FRESHNESS_DEFECT_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_FRESHNESS_SLO_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_FRESHNESS_SLO_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:freshness-slo:{}:{}",
                defect_kind.as_str(),
                source
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// The proceed/hold promotion verdict computed from the dashboard rows and rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityPromotionVerdict {
    /// `proceed` when no row holds promotion, `hold` otherwise.
    pub decision: String,
    /// Stop-rule ids that fired against a held row.
    pub firing_rule_ids: Vec<String>,
    /// Row ids holding promotion.
    pub blocked_row_ids: Vec<String>,
}

impl ContinuityPromotionVerdict {
    /// True when promotion may proceed.
    pub fn proceeds(&self) -> bool {
        self.decision == "proceed"
    }
}

/// Aggregate summary for a continuity freshness-SLO dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFreshnessSloSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Promotion decision token (`proceed` or `hold`).
    pub overall_decision_token: String,
    /// Number of continuity rows tracked.
    pub row_count: usize,
    /// Number of rows held to managed-lane freshness.
    pub release_scope_row_count: usize,
    /// Number of rows on the local-core continuity lane.
    pub local_core_row_count: usize,
    /// Number of rows whose packet is within its freshness SLO.
    pub within_slo_row_count: usize,
    /// Number of rows whose packet is due for refresh.
    pub due_for_refresh_row_count: usize,
    /// Number of rows whose packet breached its freshness SLO.
    pub breached_row_count: usize,
    /// Number of rows with no captured continuity proof packet.
    pub missing_row_count: usize,
    /// Number of rows that narrowed below their claimed label.
    pub narrowed_row_count: usize,
    /// Number of rows holding promotion.
    pub blocked_row_count: usize,
    /// Number of stop rules firing against a held row.
    pub stop_rules_firing_count: usize,
    /// Number of release-scope rows whose evidence reruns without manual surgery.
    pub automatable_rerun_row_count: usize,
    /// Number of defects recorded for the dashboard.
    pub defect_count: usize,
}

/// Full auditable input for the continuity freshness-SLO dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFreshnessSloInput {
    /// Reviewable label for the dashboard.
    pub dashboard_label: String,
    /// UTC date the dashboard recomputes freshness against (the evaluation clock).
    pub as_of: String,
    /// Opaque ref to the freshness-SLO register defining the targets.
    pub freshness_slo_register_ref: String,
    /// Opaque ref to the continuity-claim matrix the rows mirror.
    pub claim_matrix_ref: String,
    /// Tracked continuity rows.
    pub rows: Vec<ContinuityFreshnessRow>,
    /// Closed set of shiproom stop rules.
    pub stop_rules: Vec<ContinuityStopRule>,
}

/// Canonical proof packet for the continuity freshness-SLO dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFreshnessSloDashboard {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable dashboard identifier.
    pub dashboard_id: String,
    /// Reviewable dashboard label.
    pub dashboard_label: String,
    /// UTC timestamp when the dashboard packet was generated.
    pub generated_at: String,
    /// UTC date the dashboard recomputes freshness against.
    pub as_of: String,
    /// Opaque ref to the freshness-SLO register defining the targets.
    pub freshness_slo_register_ref: String,
    /// Opaque ref to the continuity-claim matrix the rows mirror.
    pub claim_matrix_ref: String,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: ContinuityFreshnessSloSummary,
    /// The proceed/hold promotion verdict.
    pub promotion: ContinuityPromotionVerdict,
    /// Typed defects for the packet.
    pub defects: Vec<ContinuityFreshnessDefect>,
    /// Per-row freshness verdicts.
    pub row_outcomes: Vec<ContinuityFreshnessRowOutcome>,
    /// The audited input embedded as evidence.
    pub input: ContinuityFreshnessSloInput,
}

impl ContinuityFreshnessSloDashboard {
    /// Builds a freshness-SLO dashboard from the supplied input.
    pub fn new(
        dashboard_id: impl Into<String>,
        dashboard_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: ContinuityFreshnessSloInput,
    ) -> Self {
        let row_outcomes = build_row_outcomes(&input);
        let promotion = build_promotion_verdict(&input, &row_outcomes);
        let defects = audit_freshness_input(&input, &row_outcomes, &promotion);
        let summary = build_summary(&input, &row_outcomes, &promotion, &defects);
        Self {
            record_kind: CONTINUITY_FRESHNESS_SLO_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_FRESHNESS_SLO_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_FRESHNESS_SLO_SHARED_CONTRACT_REF.to_owned(),
            dashboard_id: dashboard_id.into(),
            dashboard_label: dashboard_label.into(),
            generated_at: generated_at.into(),
            as_of: input.as_of.clone(),
            freshness_slo_register_ref: input.freshness_slo_register_ref.clone(),
            claim_matrix_ref: input.claim_matrix_ref.clone(),
            summary,
            promotion,
            defects,
            row_outcomes,
            input,
        }
    }

    /// True when no defect was recorded and promotion may proceed.
    pub fn is_clean_and_proceeds(&self) -> bool {
        self.defects.is_empty() && self.promotion.proceeds()
    }

    /// True when no defect was recorded for the dashboard.
    pub fn is_structurally_clean(&self) -> bool {
        self.defects.is_empty()
    }

    /// Returns the computed outcome for a row id, if present.
    pub fn row_outcome(&self, row_id: &str) -> Option<&ContinuityFreshnessRowOutcome> {
        self.row_outcomes
            .iter()
            .find(|outcome| outcome.row_id == row_id)
    }
}

/// Support-export wrapper for the freshness-SLO dashboard packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFreshnessSloSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export identifier.
    pub export_id: String,
    /// UTC timestamp when the export was produced.
    pub generated_at: String,
    /// The dashboard packet embedded as evidence.
    pub dashboard: ContinuityFreshnessSloDashboard,
    /// Stop-reason tokens active across the embedded dashboard.
    pub stop_reasons_present: Vec<String>,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl ContinuityFreshnessSloSupportExport {
    /// Wraps a freshness-SLO dashboard inside a support-export envelope.
    pub fn from_dashboard(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        dashboard: ContinuityFreshnessSloDashboard,
    ) -> Self {
        let mut reasons: BTreeSet<String> = BTreeSet::new();
        for outcome in &dashboard.row_outcomes {
            for token in &outcome.active_stop_reason_tokens {
                reasons.insert(token.clone());
            }
        }
        let mut counts = BTreeMap::new();
        for defect in &dashboard.defects {
            *counts.entry(defect.defect_kind_token.clone()).or_insert(0) += 1;
        }
        Self {
            record_kind: CONTINUITY_FRESHNESS_SLO_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_FRESHNESS_SLO_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_FRESHNESS_SLO_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            dashboard,
            stop_reasons_present: reasons.into_iter().collect(),
            defect_counts_by_kind: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Re-runs the freshness audit over the dashboard's embedded input.
pub fn audit_continuity_freshness_slo_dashboard(
    dashboard: &ContinuityFreshnessSloDashboard,
) -> Vec<ContinuityFreshnessDefect> {
    let row_outcomes = build_row_outcomes(&dashboard.input);
    let promotion = build_promotion_verdict(&dashboard.input, &row_outcomes);
    audit_freshness_input(&dashboard.input, &row_outcomes, &promotion)
}

/// Validates a dashboard and returns `Ok(())` when the audit is clean.
pub fn validate_continuity_freshness_slo_dashboard(
    dashboard: &ContinuityFreshnessSloDashboard,
) -> Result<(), Vec<ContinuityFreshnessDefect>> {
    if dashboard.defects.is_empty() {
        Ok(())
    } else {
        Err(dashboard.defects.clone())
    }
}

/// Returns the seeded clean freshness-SLO dashboard.
pub fn seeded_continuity_freshness_slo_dashboard() -> ContinuityFreshnessSloDashboard {
    ContinuityFreshnessSloDashboard::new(
        "continuity:freshness-slo:seeded",
        "Continuity-proof freshness SLO dashboard and shiproom gates",
        "2026-06-19T00:00:00Z",
        seeded_continuity_freshness_slo_input(),
    )
}

/// Returns the seeded input used by the canonical dashboard.
pub fn seeded_continuity_freshness_slo_input() -> ContinuityFreshnessSloInput {
    ContinuityFreshnessSloInput {
        dashboard_label: "Claimed managed, self-hosted, and sovereign continuity freshness"
            .to_owned(),
        as_of: "2026-06-19".to_owned(),
        freshness_slo_register_ref: "artifacts/m5/continuity/freshness_slo_register.md".to_owned(),
        claim_matrix_ref: "artifacts/m5/continuity/claim_rows_and_drill_schedule.md".to_owned(),
        rows: seeded_rows(),
        stop_rules: seeded_stop_rules(),
    }
}

fn build_row_outcomes(input: &ContinuityFreshnessSloInput) -> Vec<ContinuityFreshnessRowOutcome> {
    input
        .rows
        .iter()
        .map(|row| {
            let row_state = row.row_state();
            let effective = row.effective_qualification();
            let mut reason_tokens: Vec<String> = row
                .stop_reasons()
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            ContinuityFreshnessRowOutcome {
                record_kind: CONTINUITY_FRESHNESS_ROW_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: CONTINUITY_FRESHNESS_SLO_SCHEMA_VERSION,
                shared_contract_ref: CONTINUITY_FRESHNESS_SLO_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                profile_class_token: row.profile_class.as_str().to_owned(),
                in_release_scope: row.in_release_scope(),
                within_slo: row.proof_packet.slo_state.is_within_slo(),
                slo_state_token: row.proof_packet.slo_state.as_str().to_owned(),
                row_state_token: row_state.as_str().to_owned(),
                claimed_qualification_token: row.claimed_qualification.as_str().to_owned(),
                effective_qualification_token: effective.as_str().to_owned(),
                narrowed: effective != row.claimed_qualification,
                blocks_promotion: row.blocks_promotion(),
                rerun_automatable: row.rerun.is_automatable(),
                active_stop_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_promotion_verdict(
    input: &ContinuityFreshnessSloInput,
    row_outcomes: &[ContinuityFreshnessRowOutcome],
) -> ContinuityPromotionVerdict {
    let blocked_row_ids: Vec<String> = row_outcomes
        .iter()
        .filter(|outcome| outcome.blocks_promotion)
        .map(|outcome| outcome.row_id.clone())
        .collect();

    // Gather the stop reasons active on held rows so we can name the rules firing.
    let mut blocked_reasons: BTreeSet<String> = BTreeSet::new();
    for outcome in row_outcomes.iter().filter(|o| o.blocks_promotion) {
        for token in &outcome.active_stop_reason_tokens {
            blocked_reasons.insert(token.clone());
        }
    }
    let firing_rule_ids: Vec<String> = input
        .stop_rules
        .iter()
        .filter(|rule| {
            rule.blocks_promotion && blocked_reasons.contains(&rule.trigger_reason_token)
        })
        .map(|rule| rule.rule_id.clone())
        .collect();

    ContinuityPromotionVerdict {
        decision: if blocked_row_ids.is_empty() {
            "proceed".to_owned()
        } else {
            "hold".to_owned()
        },
        firing_rule_ids,
        blocked_row_ids,
    }
}

fn audit_freshness_input(
    input: &ContinuityFreshnessSloInput,
    row_outcomes: &[ContinuityFreshnessRowOutcome],
    promotion: &ContinuityPromotionVerdict,
) -> Vec<ContinuityFreshnessDefect> {
    let mut defects = Vec::new();

    for row in &input.rows {
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            defects.push(ContinuityFreshnessDefect::new(
                ContinuityFreshnessDefectKind::FreshnessWindowInconsistent,
                row.row_id.clone(),
                "a freshness SLO warn window may not exceed its target max age",
            ));
        }

        let has_capture = row.proof_packet.has_capture();
        if row.proof_packet.slo_state.requires_capture() && !has_capture {
            defects.push(ContinuityFreshnessDefect::new(
                ContinuityFreshnessDefectKind::PacketStateCaptureIncoherent,
                row.row_id.clone(),
                "a packet whose SLO state is not missing must carry a capture date and evidence ref",
            ));
        }
        if !row.proof_packet.slo_state.requires_capture() && has_capture {
            defects.push(ContinuityFreshnessDefect::new(
                ContinuityFreshnessDefectKind::PacketStateCaptureIncoherent,
                row.row_id.clone(),
                "a packet marked missing may not carry a capture date or evidence ref",
            ));
        }

        if row.in_release_scope() && !row.rerun.is_declared() {
            defects.push(ContinuityFreshnessDefect::new(
                ContinuityFreshnessDefectKind::RerunPathUndeclared,
                row.row_id.clone(),
                "every release-scope row must name a rerun tool or command so evidence can be refreshed without manual surgery",
            ));
        }
    }

    // Guardrail: a local-core row may never be reported as holding promotion.
    for outcome in row_outcomes {
        if !outcome.in_release_scope && outcome.blocks_promotion {
            defects.push(ContinuityFreshnessDefect::new(
                ContinuityFreshnessDefectKind::LocalCoreMarkedBlocking,
                outcome.row_id.clone(),
                "a local-core continuity row may not hold promotion when a managed row goes stale",
            ));
        }
    }

    // Every stop reason must be watched by at least one stop rule.
    let covered: BTreeSet<String> = input
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason_token.clone())
        .collect();
    for reason in ContinuityStopReason::ALL {
        if !covered.contains(reason.as_str()) {
            defects.push(ContinuityFreshnessDefect::new(
                ContinuityFreshnessDefectKind::StopReasonUncovered,
                reason.as_str(),
                "every continuity stop reason must be watched by a shiproom stop rule",
            ));
        }
    }

    // The recorded promotion verdict must agree with the held rows.
    let expected_hold = row_outcomes.iter().any(|outcome| outcome.blocks_promotion);
    let recorded_hold = promotion.decision == "hold";
    if expected_hold != recorded_hold {
        defects.push(ContinuityFreshnessDefect::new(
            ContinuityFreshnessDefectKind::PromotionVerdictIncoherent,
            "dashboard:promotion",
            "the promotion decision must be hold when any row holds promotion and proceed otherwise",
        ));
    }

    defects
}

fn build_summary(
    input: &ContinuityFreshnessSloInput,
    row_outcomes: &[ContinuityFreshnessRowOutcome],
    promotion: &ContinuityPromotionVerdict,
    defects: &[ContinuityFreshnessDefect],
) -> ContinuityFreshnessSloSummary {
    let count_slo = |state: ContinuityFreshnessSloState| {
        input
            .rows
            .iter()
            .filter(|row| row.proof_packet.slo_state == state)
            .count()
    };

    ContinuityFreshnessSloSummary {
        record_kind: CONTINUITY_FRESHNESS_SLO_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: CONTINUITY_FRESHNESS_SLO_SCHEMA_VERSION,
        shared_contract_ref: CONTINUITY_FRESHNESS_SLO_SHARED_CONTRACT_REF.to_owned(),
        overall_decision_token: promotion.decision.clone(),
        row_count: input.rows.len(),
        release_scope_row_count: input
            .rows
            .iter()
            .filter(|row| row.in_release_scope())
            .count(),
        local_core_row_count: input.rows.iter().filter(|row| row.is_local_core()).count(),
        within_slo_row_count: row_outcomes.iter().filter(|o| o.within_slo).count(),
        due_for_refresh_row_count: count_slo(ContinuityFreshnessSloState::DueForRefresh),
        breached_row_count: count_slo(ContinuityFreshnessSloState::Breached),
        missing_row_count: count_slo(ContinuityFreshnessSloState::Missing),
        narrowed_row_count: row_outcomes.iter().filter(|o| o.narrowed).count(),
        blocked_row_count: row_outcomes.iter().filter(|o| o.blocks_promotion).count(),
        stop_rules_firing_count: promotion.firing_rule_ids.len(),
        automatable_rerun_row_count: input
            .rows
            .iter()
            .filter(|row| row.in_release_scope() && row.rerun.is_automatable())
            .count(),
        defect_count: defects.len(),
    }
}

fn seeded_rows() -> Vec<ContinuityFreshnessRow> {
    vec![
        freshness_row(
            "continuity-row:managed-cloud-sync",
            "Managed cloud workspace sync and backup",
            ContinuityProfileClass::Managed,
            ContinuityLaneClass::ManagedLane,
            ContinuityClaimQualificationClass::Stable,
            proof_packet(
                "continuity-packet:managed-cloud:backup",
                "artifacts/m5/continuity/drill_packets/backup_restore_failover_page.json",
                ContinuityPacketFamilyClass::Backup,
                Some("2026-06-01"),
                90,
                14,
                ContinuityFreshnessSloState::Current,
                &["drill:managed-cloud:backup:2026-06-01"],
            ),
            rerun(
                RerunAutomationClass::AutomatedRerun,
                "tools/continuity/run_drill_packets.py",
                "2026-06-01",
            ),
            "Managed platform on-call",
            "Reliability guild",
            true,
            false,
        ),
        freshness_row(
            "continuity-row:managed-relay-failover",
            "Managed relay and collaboration failover",
            ContinuityProfileClass::Managed,
            ContinuityLaneClass::ManagedLane,
            ContinuityClaimQualificationClass::Stable,
            proof_packet(
                "continuity-packet:managed-relay:failover",
                "artifacts/m5/continuity/drill_packets/drill_packet_registry.json",
                ContinuityPacketFamilyClass::Failover,
                Some("2026-03-23"),
                90,
                14,
                ContinuityFreshnessSloState::DueForRefresh,
                &["drill:managed-relay:failover:2026-03-23"],
            ),
            rerun(
                RerunAutomationClass::ScriptedRefresh,
                "tools/continuity/run_drill_packets.py",
                "2026-03-23",
            ),
            "Managed platform on-call",
            "Reliability guild",
            true,
            false,
        ),
        freshness_row(
            "continuity-row:self-hosted-restore",
            "Customer self-hosted restore and rebuild",
            ContinuityProfileClass::SelfHosted,
            ContinuityLaneClass::ManagedLane,
            ContinuityClaimQualificationClass::Stable,
            proof_packet(
                "continuity-packet:self-hosted:restore",
                "artifacts/m5/continuity/restore_reviews/restore_review_page.json",
                ContinuityPacketFamilyClass::Restore,
                Some("2026-05-01"),
                180,
                30,
                ContinuityFreshnessSloState::Current,
                &["drill:self-hosted:restore:2026-05-01"],
            ),
            rerun(
                RerunAutomationClass::ScriptedRefresh,
                "tools/continuity/run_drill_packets.py",
                "2026-05-01",
            ),
            "Customer success SRE",
            "Field reliability owner",
            true,
            false,
        ),
        freshness_row(
            "continuity-row:sovereign-airgap-snapshot",
            "Sovereign air-gapped snapshot and replication",
            ContinuityProfileClass::Sovereign,
            ContinuityLaneClass::ManagedLane,
            ContinuityClaimQualificationClass::Stable,
            proof_packet(
                "continuity-packet:sovereign:snapshot",
                "artifacts/m5/continuity/mirror_airgap/offline_continuity_registry.json",
                ContinuityPacketFamilyClass::SnapshotReplication,
                Some("2026-04-15"),
                365,
                45,
                ContinuityFreshnessSloState::Current,
                &["drill:sovereign:snapshot:2026-04-15"],
            ),
            rerun(
                RerunAutomationClass::ScriptedRefresh,
                "tools/continuity/run_drill_packets.py",
                "2026-04-15",
            ),
            "Sovereign operations lead",
            "Customer compliance owner",
            true,
            false,
        ),
        freshness_row(
            "continuity-row:local-desktop-core",
            "Local desktop core continuity",
            ContinuityProfileClass::LocalOnly,
            ContinuityLaneClass::LocalCore,
            ContinuityClaimQualificationClass::Stable,
            proof_packet(
                "continuity-packet:local-core:autosave",
                "artifacts/continuity/m4/connectivity-state-and-deferred-intent.md",
                ContinuityPacketFamilyClass::LocalCoreContinuity,
                Some("2026-06-10"),
                365,
                30,
                ContinuityFreshnessSloState::Current,
                &["drill:local-core:autosave:2026-06-10"],
            ),
            rerun(
                RerunAutomationClass::AutomatedRerun,
                "tools/continuity/run_drill_packets.py",
                "2026-06-10",
            ),
            "Local user",
            "Local user",
            true,
            false,
        ),
    ]
}

fn seeded_stop_rules() -> Vec<ContinuityStopRule> {
    vec![
        stop_rule(
            "continuity-stop:freshness-breached",
            "Hold promotion when a continuity packet breaches its freshness SLO",
            ContinuityStopReason::ContinuityPacketFreshnessBreached,
            ContinuityStopAction::RefreshContinuityPacket,
            "A claimed continuity row whose backing packet ages past its SLO narrows and holds promotion until rerun.",
        ),
        stop_rule(
            "continuity-stop:packet-missing",
            "Hold promotion when no continuity packet has been captured",
            ContinuityStopReason::ContinuityPacketMissing,
            ContinuityStopAction::RerunContinuityDrill,
            "A claimed continuity row with no captured proof packet cannot publish managed continuity.",
        ),
        stop_rule(
            "continuity-stop:owner-signoff-missing",
            "Hold promotion when a drill owner has not signed off the evidence",
            ContinuityStopReason::DrillOwnerSignoffMissing,
            ContinuityStopAction::RequestDrillOwnerSignoff,
            "Fresh evidence without a current drill-owner sign-off is unattested and narrows the claim.",
        ),
        stop_rule(
            "continuity-stop:rerun-path-unavailable",
            "Hold promotion when a row cannot be refreshed without manual surgery",
            ContinuityStopReason::RerunPathUnavailable,
            ContinuityStopAction::NarrowContinuityClaim,
            "A continuity claim with no rerun path cannot keep evidence fresh and narrows below the cutline.",
        ),
        stop_rule(
            "continuity-stop:evidence-unqualified",
            "Hold promotion when the backing continuity evidence is unqualified",
            ContinuityStopReason::ContinuityEvidenceUnqualified,
            ContinuityStopAction::NarrowContinuityClaim,
            "A profile-mismatched or unqualified continuity row narrows rather than inheriting green managed language.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn freshness_row(
    row_id: &str,
    surface_label: &str,
    profile_class: ContinuityProfileClass,
    continuity_lane: ContinuityLaneClass,
    claimed_qualification: ContinuityClaimQualificationClass,
    proof_packet: ContinuityProofPacket,
    rerun: ContinuityRerunPath,
    drill_owner_label: &str,
    future_drill_owner_label: &str,
    owner_signoff_present: bool,
    evidence_unqualified: bool,
) -> ContinuityFreshnessRow {
    ContinuityFreshnessRow {
        row_id: row_id.to_owned(),
        surface_label: surface_label.to_owned(),
        profile_class,
        profile_class_token: profile_class.as_str().to_owned(),
        continuity_lane,
        continuity_lane_token: continuity_lane.as_str().to_owned(),
        claimed_qualification,
        claimed_qualification_token: claimed_qualification.as_str().to_owned(),
        proof_packet,
        rerun,
        drill_owner_label: drill_owner_label.to_owned(),
        future_drill_owner_label: future_drill_owner_label.to_owned(),
        owner_signoff_present,
        evidence_unqualified,
        surface_visibility: if continuity_lane == ContinuityLaneClass::LocalCore {
            ClaimSurfaceVisibility::local_core_required()
        } else {
            ClaimSurfaceVisibility::all_required()
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn proof_packet(
    packet_id: &str,
    packet_ref: &str,
    packet_family: ContinuityPacketFamilyClass,
    captured_at: Option<&str>,
    target_max_age_days: u32,
    warn_within_days: u32,
    slo_state: ContinuityFreshnessSloState,
    evidence_refs: &[&str],
) -> ContinuityProofPacket {
    ContinuityProofPacket {
        packet_id: packet_id.to_owned(),
        packet_ref: packet_ref.to_owned(),
        packet_family,
        packet_family_token: packet_family.as_str().to_owned(),
        captured_at: captured_at.map(str::to_owned),
        freshness_slo: ContinuityFreshnessSlo {
            target_max_age_days,
            warn_within_days,
            slo_register_ref: "artifacts/m5/continuity/freshness_slo_register.md".to_owned(),
        },
        slo_state,
        slo_state_token: slo_state.as_str().to_owned(),
        evidence_refs: evidence_refs.iter().map(|r| (*r).to_owned()).collect(),
    }
}

fn rerun(
    rerun_class: RerunAutomationClass,
    rerun_command_ref: &str,
    last_rerun_at: &str,
) -> ContinuityRerunPath {
    ContinuityRerunPath {
        rerun_class,
        rerun_class_token: rerun_class.as_str().to_owned(),
        rerun_command_ref: rerun_command_ref.to_owned(),
        last_rerun_at: last_rerun_at.to_owned(),
    }
}

fn stop_rule(
    rule_id: &str,
    title: &str,
    trigger_reason: ContinuityStopReason,
    default_action: ContinuityStopAction,
    rationale: &str,
) -> ContinuityStopRule {
    ContinuityStopRule {
        rule_id: rule_id.to_owned(),
        title: title.to_owned(),
        trigger_reason,
        trigger_reason_token: trigger_reason.as_str().to_owned(),
        applies_to_qualification_tokens: vec![
            ContinuityClaimQualificationClass::Stable
                .as_str()
                .to_owned(),
            ContinuityClaimQualificationClass::Beta.as_str().to_owned(),
        ],
        default_action,
        default_action_token: default_action.as_str().to_owned(),
        blocks_promotion: true,
        rationale: rationale.to_owned(),
    }
}
