//! Two reusable M5 governance-dashboard primitives implemented as one controls packet:
//! the **decision-right card** (which council/forum can actually approve the next move,
//! the reason for review, the satisfied/pending state, and the target milestone) and the
//! **milestone dashboard row** (milestone name, owning team, blocker count, waiver count,
//! gate state, nearest review forum, and next-review continuity), projected the same way
//! across every claimed M5 shiproom and operator surface.
//!
//! Aureline's frozen governance-dashboard component matrix
//! ([`crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`])
//! names the decision-right card and the milestone dashboard row as two governed component
//! families and freezes their shared readiness-state vocabulary, the decision-forum
//! classes, the decision-right states, and the milestone-gate states. This module
//! *implements* those two contracts as one reusable controls packet so a shiproom
//! reviewer, an operator, or a release reviewer can tell — from the card and the row alone
//! — who can actually approve the next move, why a review is open, whether it is satisfied
//! or still pending, and how owned work rolls up into milestone readiness with its current
//! blocker and waiver truth, before a surface silently reads `ready` while a forum or gate
//! can still block it.
//!
//! The packet has two resolver halves:
//!
//! 1. [`resolve_decision_right_card`] takes one decision's required forum, decision-right
//!    state, reason for review, target milestone, satisfied/pending state, whether
//!    governance review is required, and evidence freshness, and produces one
//!    [`M5ResolvedDecisionRightCard`] carrying the *derived* readiness state drawn from the
//!    frozen [`M5GovernanceReadinessState`] vocabulary. A card whose governance review is
//!    required never resolves to a clean pass while a forum or gate can still block it, and
//!    an advisory forum is never rendered as authoritative: it reads `warning`.
//! 2. [`resolve_milestone_dashboard_row`] takes one milestone's name, owning team, owner
//!    coverage, blocker count, waiver count, gate state, nearest review forum, next-review
//!    continuity, and evidence freshness, and produces one [`M5ResolvedMilestoneRow`]
//!    carrying the derived readiness state, always-visible ownership, and always-visible
//!    blocker/waiver truth. Milestone readiness never drifts into a summary-only reading:
//!    an open blocker or waiver, or an unresolved owner, never reads as a met gate.
//!
//! A parity matrix — [`M5DecisionRightMilestoneControlsPacket`] — binds one row per claimed
//! M5 governance consumer (the shiproom board, the operator board, the release center, the
//! support export, and the CLI inspect) to the shared card and row anatomy, the same
//! readiness states, decision-forum classes, decision-right states, milestone-gate states,
//! satisfaction states, evidence-freshness readings, degrade reasons, next actions, and
//! export fields, plus worked resolution cases that must reproduce the resolver output
//! exactly, so the decision-right/milestone vocabulary stays identical — one model — across
//! shiproom, operator, release, and support surfaces rather than cloned prose.
//!
//! The frozen readiness-state vocabulary ([`M5GovernanceReadinessState`]), the
//! decision-forum class ([`M5DecisionForumClass`]), the decision-right state
//! ([`M5DecisionRightState`]), the milestone-gate state ([`M5MilestoneGateState`]), the
//! ownership-coverage state ([`M5OwnershipCoverageState`]), the deployment line
//! ([`M5DeploymentLine`]), the governance surface family ([`M5GovernanceSurfaceFamily`]),
//! the governance consumer surface ([`M5GovernanceConsumerSurface`]), the accessibility
//! route ([`M5GovernanceAccessibilityRoute`]), the required label
//! ([`M5GovernanceRequiredLabel`]), the qualification class
//! ([`M5GovernanceQualificationClass`]), and the downgrade trigger
//! ([`M5GovernanceDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the card and
//! the row themselves: their governance consumer families, their anatomy parts, the review
//! satisfaction states, the evidence-freshness readings, the degrade reasons, the next
//! actions, the card and row actions, and the export fields. No M5 governance surface
//! invents a second decision-right or milestone grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user text bodies stay outside
//! the support boundary; every card id, reason-for-review, milestone identity, owning-team
//! alias, and next-review representation is carried only as an opaque, export-safe
//! representation, and an owning-team or forum alias is a role alias, never a personal
//! contact detail.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_decision_right_milestone_controls_operator_board_preview_narrowed,
    seeded_m5_decision_right_milestone_controls_packet,
    seeded_m5_decision_right_milestone_controls_shiproom_board_beta_narrowed,
    M5_DECISION_RIGHT_MILESTONE_CONTROLS_PACKET_ID,
};

// The readiness state vocabulary, the decision-forum classes, the decision-right states,
// the milestone-gate states, the ownership-coverage states, the deployment lines, the
// surface families, the consumer surfaces, the accessibility routes, the required labels,
// the qualification classes, and the downgrade triggers are frozen once, in the
// governance-dashboard component matrix. This controls packet reuses them verbatim so it
// never invents a parallel vocabulary.
pub use crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix::{
    M5DecisionForumClass, M5DecisionRightState, M5DeploymentLine,
    M5GovernanceAccessibilityRoute, M5GovernanceConsumerSurface, M5GovernanceDowngradeTrigger,
    M5GovernanceQualificationClass, M5GovernanceReadinessState, M5GovernanceRequiredLabel,
    M5GovernanceSurfaceFamily, M5MilestoneGateState, M5OwnershipCoverageState,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DecisionRightMilestoneControlsPacket`].
pub const M5_DECISION_RIGHT_MILESTONE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_decision_right_cards_and_milestone_dashboard_rows_across_claimed_m5_shiproom_and_operator_surfaces";

/// Schema version for M5 decision-right / milestone controls records.
pub const M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-decision-right-milestone-controls.schema.json";

/// Repo-relative path of the controls contract doc.
pub const M5_DECISION_RIGHT_MILESTONE_CONTROLS_DOC_REF: &str =
    "docs/help/m5_decision_right_card_and_milestone_dashboard_row_controls.md";

/// Repo-relative path of the frozen governance-dashboard component matrix schema this
/// controls packet narrows from.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-matrix.schema.json";

/// Repo-relative path of the frozen governance-dashboard component matrix doc.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_components_contract.md";

/// Repo-relative path of the per-component decision-right-card contract schema.
pub const M5_DECISION_RIGHT_CARD_CONTRACT_REF: &str =
    "schemas/ui/m5-decision-right-card.schema.json";

/// Repo-relative path of the per-component milestone-dashboard-row contract schema.
pub const M5_MILESTONE_DASHBOARD_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-milestone-dashboard-row.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DECISION_RIGHT_MILESTONE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-decision-right-milestone-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DECISION_RIGHT_MILESTONE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-decision-right-milestone-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DECISION_RIGHT_MILESTONE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-decision-right-milestone-controls-proof/summary.md";

// ---------------------------------------------------------------------------
// Minted vocabulary
// ---------------------------------------------------------------------------

/// One claimed M5 governance consumer that renders the shared decision-right card and
/// milestone dashboard row. The shiproom, operator, and support surfaces are all named so
/// they can be proven to reuse one decision-right/milestone model rather than cloning
/// prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionMilestoneConsumerSurface {
    /// The shiproom board / packet.
    ShiproomBoard,
    /// The operator overview board.
    OperatorBoard,
    /// The release-center surface.
    ReleaseCenter,
    /// The support / export packet.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
}

impl M5DecisionMilestoneConsumerSurface {
    /// Every claimed governance consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ShiproomBoard,
        Self::OperatorBoard,
        Self::ReleaseCenter,
        Self::SupportExport,
        Self::CliInspect,
    ];

    /// The three surfaces that must share one decision-right/milestone model.
    pub const SHARED_MODEL_REQUIRED: [Self; 3] = [
        Self::ShiproomBoard,
        Self::OperatorBoard,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShiproomBoard => "shiproom_board",
            Self::OperatorBoard => "operator_board",
            Self::ReleaseCenter => "release_center",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ShiproomBoard => "Shiproom Board",
            Self::OperatorBoard => "Operator Board",
            Self::ReleaseCenter => "Release Center",
            Self::SupportExport => "Support / Export",
            Self::CliInspect => "CLI Inspect",
        }
    }
}

/// One anatomy part the shared card / row surfaces. The parts in
/// [`M5DecisionMilestoneAnatomyPart::MANDATORY`] are required on every row so a reviewer
/// can orient before trusting a decision-right or milestone claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionMilestoneAnatomyPart {
    /// The required council/forum cue (card).
    RequiredForum,
    /// The reason-for-review cue (card).
    ReasonForReview,
    /// The satisfied/pending state cue (card).
    SatisfactionState,
    /// The target-milestone cue (card).
    TargetMilestone,
    /// The decision-right state cue (card).
    DecisionRightState,
    /// The decision evidence-freshness cue (card).
    DecisionEvidenceFreshness,
    /// The open-decision-forum action (card).
    OpenForumAction,
    /// The milestone name cue (row).
    MilestoneName,
    /// The owning-team cue (row).
    OwningTeam,
    /// The blocker-count cue (row).
    BlockerCount,
    /// The waiver-count cue (row).
    WaiverCount,
    /// The gate-state cue (row).
    GateState,
    /// The nearest-review-forum cue (row).
    NearestReviewForum,
    /// The next-review-continuity cue (row).
    NextReviewContinuity,
    /// The open-milestone-board action (row).
    OpenMilestoneAction,
}

impl M5DecisionMilestoneAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 15] = [
        Self::RequiredForum,
        Self::ReasonForReview,
        Self::SatisfactionState,
        Self::TargetMilestone,
        Self::DecisionRightState,
        Self::DecisionEvidenceFreshness,
        Self::OpenForumAction,
        Self::MilestoneName,
        Self::OwningTeam,
        Self::BlockerCount,
        Self::WaiverCount,
        Self::GateState,
        Self::NearestReviewForum,
        Self::NextReviewContinuity,
        Self::OpenMilestoneAction,
    ];

    /// The anatomy parts every row must render before a decision or milestone is trusted.
    pub const MANDATORY: [Self; 4] = [
        Self::RequiredForum,
        Self::SatisfactionState,
        Self::MilestoneName,
        Self::GateState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredForum => "required_forum",
            Self::ReasonForReview => "reason_for_review",
            Self::SatisfactionState => "satisfaction_state",
            Self::TargetMilestone => "target_milestone",
            Self::DecisionRightState => "decision_right_state",
            Self::DecisionEvidenceFreshness => "decision_evidence_freshness",
            Self::OpenForumAction => "open_forum_action",
            Self::MilestoneName => "milestone_name",
            Self::OwningTeam => "owning_team",
            Self::BlockerCount => "blocker_count",
            Self::WaiverCount => "waiver_count",
            Self::GateState => "gate_state",
            Self::NearestReviewForum => "nearest_review_forum",
            Self::NextReviewContinuity => "next_review_continuity",
            Self::OpenMilestoneAction => "open_milestone_action",
        }
    }
}

/// Whether the required review on a decision-right card is satisfied, so a card never
/// leaves its satisfied/pending state implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewSatisfactionState {
    /// The required review is satisfied.
    ReviewSatisfied,
    /// The required review is still pending.
    ReviewPending,
    /// The required review is held under a disclosed waiver.
    ReviewWaived,
    /// No review is required for this decision.
    ReviewNotRequired,
}

impl M5ReviewSatisfactionState {
    /// Every satisfaction state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewSatisfied,
        Self::ReviewPending,
        Self::ReviewWaived,
        Self::ReviewNotRequired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewSatisfied => "review_satisfied",
            Self::ReviewPending => "review_pending",
            Self::ReviewWaived => "review_waived",
            Self::ReviewNotRequired => "review_not_required",
        }
    }
}

/// The evidence-freshness reading shared by both resolvers, so a card or a row never shows
/// stale or missing evidence as fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshness {
    /// The evidence is fresh within its freshness window.
    EvidenceFresh,
    /// The evidence is aging but still within tolerance.
    EvidenceAging,
    /// The evidence is stale relative to the current build.
    EvidenceStale,
    /// The evidence is missing.
    EvidenceMissing,
    /// The evidence-freshness reading is unknown / not yet evaluated.
    EvidenceUnknown,
}

impl M5EvidenceFreshness {
    /// Every evidence-freshness reading, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EvidenceFresh,
        Self::EvidenceAging,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::EvidenceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceFresh => "evidence_fresh",
            Self::EvidenceAging => "evidence_aging",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::EvidenceUnknown => "evidence_unknown",
        }
    }
}

/// The next action named on a degraded card or row, so a non-passing reading is actionable
/// rather than a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionMilestoneNextAction {
    /// Open the decision forum.
    OpenDecisionForum,
    /// Route the decision to an authorized forum.
    RouteToAuthorizedForum,
    /// Resolve the unresolved owner or forum.
    ResolveOwnerOrForum,
    /// Clear the open milestone blockers.
    ClearMilestoneBlockers,
    /// Schedule the next review.
    ScheduleNextReview,
    /// Refresh the stale or missing evidence.
    RefreshEvidence,
}

impl M5DecisionMilestoneNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDecisionForum,
        Self::RouteToAuthorizedForum,
        Self::ResolveOwnerOrForum,
        Self::ClearMilestoneBlockers,
        Self::ScheduleNextReview,
        Self::RefreshEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDecisionForum => "open_decision_forum",
            Self::RouteToAuthorizedForum => "route_to_authorized_forum",
            Self::ResolveOwnerOrForum => "resolve_owner_or_forum",
            Self::ClearMilestoneBlockers => "clear_milestone_blockers",
            Self::ScheduleNextReview => "schedule_next_review",
            Self::RefreshEvidence => "refresh_evidence",
        }
    }
}

/// The exact reason a decision-right card degraded below a clean pass, so a card whose
/// governance review is required never reads `ready` while a forum can still block it and
/// an advisory forum never reads as authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionRightDegradeReason {
    /// The card has not been evaluated on this build.
    NotYetEvaluated,
    /// Governance review is required but no authorized forum is resolved to approve or
    /// block it.
    NoAuthorizedForum,
    /// The named forum is advisory only and cannot approve this decision.
    AdvisoryForumNotAuthoritative,
    /// The decision is delegated to another forum than the one named.
    DecisionDelegatedElsewhere,
    /// The decision evidence is missing.
    DecisionEvidenceMissing,
    /// The decision evidence is stale relative to the current build.
    DecisionEvidenceStale,
    /// The required review is held under a disclosed waiver.
    DecisionReviewWaived,
    /// Governance review is required and the review is still pending.
    ReviewPending,
}

impl M5DecisionRightDegradeReason {
    /// Every decision-right degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotYetEvaluated,
        Self::NoAuthorizedForum,
        Self::AdvisoryForumNotAuthoritative,
        Self::DecisionDelegatedElsewhere,
        Self::DecisionEvidenceMissing,
        Self::DecisionEvidenceStale,
        Self::DecisionReviewWaived,
        Self::ReviewPending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "not_yet_evaluated",
            Self::NoAuthorizedForum => "no_authorized_forum",
            Self::AdvisoryForumNotAuthoritative => "advisory_forum_not_authoritative",
            Self::DecisionDelegatedElsewhere => "decision_delegated_elsewhere",
            Self::DecisionEvidenceMissing => "decision_evidence_missing",
            Self::DecisionEvidenceStale => "decision_evidence_stale",
            Self::DecisionReviewWaived => "decision_review_waived",
            Self::ReviewPending => "review_pending",
        }
    }

    /// The frozen readiness state this degrade reason resolves to.
    pub const fn readiness_state(self) -> M5GovernanceReadinessState {
        match self {
            Self::NotYetEvaluated => M5GovernanceReadinessState::NotEvaluated,
            Self::NoAuthorizedForum => M5GovernanceReadinessState::ForumUnresolved,
            Self::AdvisoryForumNotAuthoritative => M5GovernanceReadinessState::Warning,
            Self::DecisionDelegatedElsewhere => M5GovernanceReadinessState::Warning,
            Self::DecisionEvidenceMissing => M5GovernanceReadinessState::Blocked,
            Self::DecisionEvidenceStale => M5GovernanceReadinessState::EvidenceStale,
            Self::DecisionReviewWaived => M5GovernanceReadinessState::Waived,
            Self::ReviewPending => M5GovernanceReadinessState::Warning,
        }
    }

    /// The next action a reviewer should take to clear this degrade.
    pub const fn next_action(self) -> M5DecisionMilestoneNextAction {
        match self {
            Self::NotYetEvaluated => M5DecisionMilestoneNextAction::OpenDecisionForum,
            Self::NoAuthorizedForum => M5DecisionMilestoneNextAction::ResolveOwnerOrForum,
            Self::AdvisoryForumNotAuthoritative => {
                M5DecisionMilestoneNextAction::RouteToAuthorizedForum
            }
            Self::DecisionDelegatedElsewhere => {
                M5DecisionMilestoneNextAction::RouteToAuthorizedForum
            }
            Self::DecisionEvidenceMissing => M5DecisionMilestoneNextAction::RefreshEvidence,
            Self::DecisionEvidenceStale => M5DecisionMilestoneNextAction::RefreshEvidence,
            Self::DecisionReviewWaived => M5DecisionMilestoneNextAction::ScheduleNextReview,
            Self::ReviewPending => M5DecisionMilestoneNextAction::ScheduleNextReview,
        }
    }

    /// Review-safe reason phrase for the card's degrade note.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "the decision-right card has not been evaluated on this build",
            Self::NoAuthorizedForum => {
                "governance review is required but no authorized forum is resolved to approve it"
            }
            Self::AdvisoryForumNotAuthoritative => {
                "the named forum is advisory only and cannot approve this decision"
            }
            Self::DecisionDelegatedElsewhere => {
                "the decision is delegated to a different forum than the one named"
            }
            Self::DecisionEvidenceMissing => "the decision evidence is missing",
            Self::DecisionEvidenceStale => "the decision evidence is stale relative to this build",
            Self::DecisionReviewWaived => "the required review is held under a disclosed waiver",
            Self::ReviewPending => "governance review is required and the review is still pending",
        }
    }
}

/// The exact reason a milestone dashboard row degraded below a clean, met gate, so an open
/// blocker or waiver, or an unresolved owner, never reads as a met milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MilestoneDegradeReason {
    /// The row has not been evaluated on this build.
    NotYetEvaluated,
    /// The milestone has no resolved owner.
    MilestoneOwnerUnresolved,
    /// The milestone has no nearest review forum.
    NoNearestReviewForum,
    /// The exit gate is blocked or open blockers remain.
    MilestoneGateBlocked,
    /// The exit gate is held under a waiver, or open waivers remain.
    MilestoneGateWaived,
    /// The exit-gate evidence is stale or missing.
    MilestoneEvidenceStale,
    /// The exit gate is pending.
    MilestoneGatePending,
    /// The exit-gate evidence is aging and should be refreshed.
    MilestoneEvidenceAging,
}

impl M5MilestoneDegradeReason {
    /// Every milestone degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotYetEvaluated,
        Self::MilestoneOwnerUnresolved,
        Self::NoNearestReviewForum,
        Self::MilestoneGateBlocked,
        Self::MilestoneGateWaived,
        Self::MilestoneEvidenceStale,
        Self::MilestoneGatePending,
        Self::MilestoneEvidenceAging,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "not_yet_evaluated",
            Self::MilestoneOwnerUnresolved => "milestone_owner_unresolved",
            Self::NoNearestReviewForum => "no_nearest_review_forum",
            Self::MilestoneGateBlocked => "milestone_gate_blocked",
            Self::MilestoneGateWaived => "milestone_gate_waived",
            Self::MilestoneEvidenceStale => "milestone_evidence_stale",
            Self::MilestoneGatePending => "milestone_gate_pending",
            Self::MilestoneEvidenceAging => "milestone_evidence_aging",
        }
    }

    /// The frozen readiness state this degrade reason resolves to.
    pub const fn readiness_state(self) -> M5GovernanceReadinessState {
        match self {
            Self::NotYetEvaluated => M5GovernanceReadinessState::NotEvaluated,
            Self::MilestoneOwnerUnresolved => M5GovernanceReadinessState::OwnerUnresolved,
            Self::NoNearestReviewForum => M5GovernanceReadinessState::ForumUnresolved,
            Self::MilestoneGateBlocked => M5GovernanceReadinessState::Blocked,
            Self::MilestoneGateWaived => M5GovernanceReadinessState::Waived,
            Self::MilestoneEvidenceStale => M5GovernanceReadinessState::EvidenceStale,
            Self::MilestoneGatePending => M5GovernanceReadinessState::Warning,
            Self::MilestoneEvidenceAging => M5GovernanceReadinessState::Warning,
        }
    }

    /// The next action a reviewer should take to clear this degrade.
    pub const fn next_action(self) -> M5DecisionMilestoneNextAction {
        match self {
            Self::NotYetEvaluated => M5DecisionMilestoneNextAction::OpenDecisionForum,
            Self::MilestoneOwnerUnresolved => M5DecisionMilestoneNextAction::ResolveOwnerOrForum,
            Self::NoNearestReviewForum => M5DecisionMilestoneNextAction::ResolveOwnerOrForum,
            Self::MilestoneGateBlocked => M5DecisionMilestoneNextAction::ClearMilestoneBlockers,
            Self::MilestoneGateWaived => M5DecisionMilestoneNextAction::ScheduleNextReview,
            Self::MilestoneEvidenceStale => M5DecisionMilestoneNextAction::RefreshEvidence,
            Self::MilestoneGatePending => M5DecisionMilestoneNextAction::ScheduleNextReview,
            Self::MilestoneEvidenceAging => M5DecisionMilestoneNextAction::RefreshEvidence,
        }
    }

    /// Review-safe reason phrase for the row's degrade note.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "the milestone row has not been evaluated on this build",
            Self::MilestoneOwnerUnresolved => "the milestone has no resolved owning team",
            Self::NoNearestReviewForum => "the milestone has no nearest review forum",
            Self::MilestoneGateBlocked => "the exit gate is blocked with open blockers remaining",
            Self::MilestoneGateWaived => "the exit gate is held under a waiver with open waivers",
            Self::MilestoneEvidenceStale => {
                "the exit-gate evidence is stale relative to this build"
            }
            Self::MilestoneGatePending => "the exit gate is still pending",
            Self::MilestoneEvidenceAging => {
                "the exit-gate evidence is aging and should be refreshed"
            }
        }
    }
}

/// An action a decision-right card offers. The actions in
/// [`M5DecisionCardAction::MANDATORY`] are required on every row so a reviewer can always
/// open the decision forum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionCardAction {
    /// Open the decision forum.
    OpenDecisionForum,
    /// View the reason for review.
    ViewReviewReason,
    /// Route the decision to an authorized forum.
    RouteToForum,
    /// Export the decision-right ledger.
    ExportDecisionLedger,
}

impl M5DecisionCardAction {
    /// Every card action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenDecisionForum,
        Self::ViewReviewReason,
        Self::RouteToForum,
        Self::ExportDecisionLedger,
    ];

    /// The card actions every row must offer.
    pub const MANDATORY: [Self; 1] = [Self::OpenDecisionForum];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDecisionForum => "open_decision_forum",
            Self::ViewReviewReason => "view_review_reason",
            Self::RouteToForum => "route_to_forum",
            Self::ExportDecisionLedger => "export_decision_ledger",
        }
    }
}

/// An action a milestone dashboard row offers. The actions in
/// [`M5MilestoneRowAction::MANDATORY`] are required on every row so a reviewer can always
/// open the milestone board and the nearest review forum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MilestoneRowAction {
    /// Open the milestone board.
    OpenMilestoneBoard,
    /// Open the nearest review forum.
    OpenNearestReviewForum,
    /// View the open blockers and waivers.
    ViewBlockersAndWaivers,
    /// Export the milestone ledger.
    ExportMilestoneLedger,
}

impl M5MilestoneRowAction {
    /// Every row action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenMilestoneBoard,
        Self::OpenNearestReviewForum,
        Self::ViewBlockersAndWaivers,
        Self::ExportMilestoneLedger,
    ];

    /// The row actions every row must offer.
    pub const MANDATORY: [Self; 2] = [Self::OpenMilestoneBoard, Self::OpenNearestReviewForum];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenMilestoneBoard => "open_milestone_board",
            Self::OpenNearestReviewForum => "open_nearest_review_forum",
            Self::ViewBlockersAndWaivers => "view_blockers_and_waivers",
            Self::ExportMilestoneLedger => "export_milestone_ledger",
        }
    }
}

/// A field the support / export packet carries so card and row truth is reconstructable
/// from the shared model. The fields in [`M5DecisionMilestoneExportField::MANDATORY`] are
/// required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionMilestoneExportField {
    /// The opaque card id.
    CardId,
    /// The required forum class.
    RequiredForum,
    /// The decision-right state.
    DecisionRightState,
    /// The review satisfaction state.
    SatisfactionState,
    /// The target milestone.
    TargetMilestone,
    /// The milestone name.
    MilestoneName,
    /// The owning team alias.
    OwningTeam,
    /// The blocker count.
    BlockerCount,
    /// The waiver count.
    WaiverCount,
    /// The milestone gate state.
    GateState,
    /// The derived readiness state.
    ReadinessState,
}

impl M5DecisionMilestoneExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::CardId,
        Self::RequiredForum,
        Self::DecisionRightState,
        Self::SatisfactionState,
        Self::TargetMilestone,
        Self::MilestoneName,
        Self::OwningTeam,
        Self::BlockerCount,
        Self::WaiverCount,
        Self::GateState,
        Self::ReadinessState,
    ];

    /// The export fields every controls export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::RequiredForum,
        Self::MilestoneName,
        Self::BlockerCount,
        Self::WaiverCount,
        Self::ReadinessState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CardId => "card_id",
            Self::RequiredForum => "required_forum",
            Self::DecisionRightState => "decision_right_state",
            Self::SatisfactionState => "satisfaction_state",
            Self::TargetMilestone => "target_milestone",
            Self::MilestoneName => "milestone_name",
            Self::OwningTeam => "owning_team",
            Self::BlockerCount => "blocker_count",
            Self::WaiverCount => "waiver_count",
            Self::GateState => "gate_state",
            Self::ReadinessState => "readiness_state",
        }
    }
}

// ---------------------------------------------------------------------------
// Decision-right-card resolver
// ---------------------------------------------------------------------------

/// The full input to the decision-right-card resolver for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionRightResolutionInput {
    /// The opaque, export-safe card id.
    pub card_id_repr: String,
    /// The required council/forum for this decision.
    pub required_forum: M5DecisionForumClass,
    /// The decision-right state of the named forum.
    pub decision_state: M5DecisionRightState,
    /// The opaque, plain-language reason for review.
    pub reason_for_review_repr: String,
    /// The opaque target-milestone identity.
    pub target_milestone_repr: String,
    /// The satisfied/pending state of the required review.
    pub satisfaction_state: M5ReviewSatisfactionState,
    /// Whether governance review is required for this decision.
    pub governance_review_required: bool,
    /// The decision evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
}

/// The resolved decision-right-card truth for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDecisionRightCard {
    /// The opaque card id.
    pub card_id_repr: String,
    /// The required council/forum.
    pub required_forum: M5DecisionForumClass,
    /// The decision-right state.
    pub decision_state: M5DecisionRightState,
    /// The opaque reason for review.
    pub reason_for_review_repr: String,
    /// The opaque target milestone.
    pub target_milestone_repr: String,
    /// The satisfied/pending state.
    pub satisfaction_state: M5ReviewSatisfactionState,
    /// Whether governance review is required.
    pub governance_review_required: bool,
    /// `true` only when the named forum is authoritative and resolved.
    pub decision_authoritative: bool,
    /// `true` always: the required forum stays visible wherever the card is summarized.
    pub forum_visible: bool,
    /// `true` when the card is not a clean pass and therefore names a forum or gate that
    /// can still block it. `false` for a clean pass.
    pub blocking_forum_or_gate_shown: bool,
    /// The decision evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// The derived readiness state drawn from the frozen vocabulary.
    pub readiness_state: M5GovernanceReadinessState,
    /// `true` only when the derived readiness is a clean pass.
    pub is_clean_pass: bool,
    /// The card actions this row always offers (always includes open-forum).
    pub card_actions: Vec<M5DecisionCardAction>,
    /// The degrade reason, present when the card is not a clean pass.
    pub degrade_reason: Option<M5DecisionRightDegradeReason>,
    /// The next action, present when the card is degraded.
    pub next_action: Option<M5DecisionMilestoneNextAction>,
    /// A self-contained forum note naming the required forum, present always.
    pub forum_note: String,
    /// A self-contained degrade note naming the reason and next action, present when the
    /// card is degraded.
    pub degrade_note: Option<String>,
}

/// Errors returned by [`resolve_decision_right_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DecisionRightResolutionError {
    /// The card id was empty.
    EmptyCardId,
    /// The reason for review was empty.
    EmptyReasonForReview,
    /// The target milestone was empty.
    EmptyTargetMilestone,
    /// A card id, reason, or milestone repr carried forbidden material.
    ForbiddenDecisionMaterial,
}

impl M5DecisionRightResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyCardId => "empty_card_id",
            Self::EmptyReasonForReview => "empty_reason_for_review",
            Self::EmptyTargetMilestone => "empty_target_milestone",
            Self::ForbiddenDecisionMaterial => "forbidden_decision_material",
        }
    }
}

impl fmt::Display for M5DecisionRightResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "decision-right-card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DecisionRightResolutionError {}

/// Resolves one decision-right card from its declared state.
///
/// The derived readiness state is computed in a fixed degrade-first order: an unknown
/// freshness reading or a not-evaluated decision state is `not_evaluated`; a required
/// governance review with no authorized forum is `forum_unresolved` (the forum that can
/// still block it is named); an advisory-only forum is `warning` (never rendered as
/// authoritative); a delegated decision is `warning`; missing evidence blocks; stale
/// evidence is `evidence_stale`; a waived review is `waived`; and a required review that is
/// still pending is `warning`. Only a card with an authoritative, resolved forum, a
/// satisfied-or-not-required review, and fresh-or-aging evidence is a clean pass — so a
/// surface can never appear `ready` while a forum or gate can still block it.
pub fn resolve_decision_right_card(
    input: &M5DecisionRightResolutionInput,
) -> Result<M5ResolvedDecisionRightCard, M5DecisionRightResolutionError> {
    if input.card_id_repr.trim().is_empty() {
        return Err(M5DecisionRightResolutionError::EmptyCardId);
    }
    if input.reason_for_review_repr.trim().is_empty() {
        return Err(M5DecisionRightResolutionError::EmptyReasonForReview);
    }
    if input.target_milestone_repr.trim().is_empty() {
        return Err(M5DecisionRightResolutionError::EmptyTargetMilestone);
    }
    if value_repr_is_forbidden(&input.card_id_repr)
        || value_repr_is_forbidden(&input.reason_for_review_repr)
        || value_repr_is_forbidden(&input.target_milestone_repr)
    {
        return Err(M5DecisionRightResolutionError::ForbiddenDecisionMaterial);
    }

    let decision_authoritative = input.decision_state.is_authoritative()
        && !matches!(
            input.required_forum,
            M5DecisionForumClass::NoAuthorizedForum
        );
    let degrade_reason = derive_decision_right_degrade(
        input.required_forum,
        input.decision_state,
        input.satisfaction_state,
        input.governance_review_required,
        input.evidence_freshness,
    );
    let readiness_state = match degrade_reason {
        Some(reason) => reason.readiness_state(),
        None => M5GovernanceReadinessState::Passing,
    };
    let is_clean_pass = readiness_state.is_clean_pass();
    let next_action = degrade_reason.map(M5DecisionRightDegradeReason::next_action);
    let forum_note = format!(
        "Decision right: forum `{}` ({})",
        input.required_forum.as_str(),
        if decision_authoritative {
            "authoritative for this decision"
        } else {
            "not authoritative — advisory, delegated, or unresolved"
        }
    );
    let degrade_note = degrade_reason.map(|reason| {
        format!(
            "Decision-right card degraded: {} — state `{}`; next: {}",
            reason.phrase(),
            readiness_state.as_str(),
            reason.next_action().as_str()
        )
    });

    Ok(M5ResolvedDecisionRightCard {
        card_id_repr: input.card_id_repr.clone(),
        required_forum: input.required_forum,
        decision_state: input.decision_state,
        reason_for_review_repr: input.reason_for_review_repr.clone(),
        target_milestone_repr: input.target_milestone_repr.clone(),
        satisfaction_state: input.satisfaction_state,
        governance_review_required: input.governance_review_required,
        decision_authoritative,
        forum_visible: true,
        blocking_forum_or_gate_shown: !is_clean_pass,
        evidence_freshness: input.evidence_freshness,
        readiness_state,
        is_clean_pass,
        card_actions: vec![
            M5DecisionCardAction::OpenDecisionForum,
            M5DecisionCardAction::ViewReviewReason,
            M5DecisionCardAction::ExportDecisionLedger,
        ],
        degrade_reason,
        next_action,
        forum_note,
        degrade_note,
    })
}

/// The fixed degrade-first decision-right ladder. Returns `None` for a clean pass.
fn derive_decision_right_degrade(
    forum: M5DecisionForumClass,
    state: M5DecisionRightState,
    satisfaction: M5ReviewSatisfactionState,
    review_required: bool,
    freshness: M5EvidenceFreshness,
) -> Option<M5DecisionRightDegradeReason> {
    if matches!(freshness, M5EvidenceFreshness::EvidenceUnknown)
        || matches!(state, M5DecisionRightState::NotEvaluatedHere)
    {
        Some(M5DecisionRightDegradeReason::NotYetEvaluated)
    } else if review_required
        && (matches!(forum, M5DecisionForumClass::NoAuthorizedForum)
            || matches!(state, M5DecisionRightState::ForumUnresolved))
    {
        // A required governance review with no authorized forum can still block: it never
        // reads ready.
        Some(M5DecisionRightDegradeReason::NoAuthorizedForum)
    } else if matches!(state, M5DecisionRightState::AdvisoryOnly) {
        // An advisory forum is never rendered as authoritative.
        Some(M5DecisionRightDegradeReason::AdvisoryForumNotAuthoritative)
    } else if matches!(state, M5DecisionRightState::DelegatedDecision) {
        Some(M5DecisionRightDegradeReason::DecisionDelegatedElsewhere)
    } else if matches!(freshness, M5EvidenceFreshness::EvidenceMissing) {
        Some(M5DecisionRightDegradeReason::DecisionEvidenceMissing)
    } else if matches!(freshness, M5EvidenceFreshness::EvidenceStale) {
        Some(M5DecisionRightDegradeReason::DecisionEvidenceStale)
    } else if matches!(satisfaction, M5ReviewSatisfactionState::ReviewWaived) {
        Some(M5DecisionRightDegradeReason::DecisionReviewWaived)
    } else if review_required && matches!(satisfaction, M5ReviewSatisfactionState::ReviewPending) {
        // A required, still-pending review means the named forum can still block it.
        Some(M5DecisionRightDegradeReason::ReviewPending)
    } else {
        // Authoritative resolved forum, satisfied or not-required review, fresh or aging
        // evidence.
        None
    }
}

/// One worked decision-right-card resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionRightCardCase {
    /// The resolver input.
    pub input: M5DecisionRightResolutionInput,
    /// The resolved truth. Must equal `resolve_decision_right_card(&input)`.
    pub resolved: M5ResolvedDecisionRightCard,
}

impl M5DecisionRightCardCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DecisionRightResolutionInput) -> Self {
        let resolved =
            resolve_decision_right_card(&input).expect("seed decision-right-card case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_decision_right_card(&self.input).as_ref() == Ok(&self.resolved)
    }
}

// ---------------------------------------------------------------------------
// Milestone-dashboard-row resolver
// ---------------------------------------------------------------------------

/// The full input to the milestone-dashboard-row resolver for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MilestoneRowResolutionInput {
    /// The opaque, export-safe milestone id.
    pub milestone_id_repr: String,
    /// The opaque milestone name.
    pub milestone_name_repr: String,
    /// The opaque owning-team alias (never a personal contact detail).
    pub owning_team_alias: String,
    /// The owner-coverage state of the milestone owner.
    pub owner_coverage: M5OwnershipCoverageState,
    /// The number of open blockers on this milestone.
    pub blocker_count: u32,
    /// The number of open waivers on this milestone.
    pub waiver_count: u32,
    /// The exit-gate state of the milestone.
    pub gate_state: M5MilestoneGateState,
    /// The nearest review forum for this milestone.
    pub nearest_review_forum: M5DecisionForumClass,
    /// The opaque, export-safe next-review continuity representation.
    pub next_review_repr: String,
    /// The exit-gate evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
}

/// The resolved milestone-dashboard-row truth for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMilestoneRow {
    /// The opaque milestone id.
    pub milestone_id_repr: String,
    /// The opaque milestone name.
    pub milestone_name_repr: String,
    /// The opaque owning-team alias.
    pub owning_team_alias: String,
    /// The owner-coverage state.
    pub owner_coverage: M5OwnershipCoverageState,
    /// The number of open blockers.
    pub blocker_count: u32,
    /// The number of open waivers.
    pub waiver_count: u32,
    /// The exit-gate state.
    pub gate_state: M5MilestoneGateState,
    /// The nearest review forum.
    pub nearest_review_forum: M5DecisionForumClass,
    /// The opaque next-review continuity representation.
    pub next_review_repr: String,
    /// The exit-gate evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// `true` when the milestone has a resolved, accountable owning team.
    pub owner_accountable: bool,
    /// `true` always: the owning team stays visible wherever the row is summarized.
    pub ownership_visible: bool,
    /// `true` always: the blocker and waiver counts stay visible wherever the row is
    /// summarized.
    pub blocker_waiver_truth_visible: bool,
    /// `true` always: the nearest review forum stays visible.
    pub nearest_forum_visible: bool,
    /// `true` always: the next-review continuity is reconstructable from the row/export.
    pub next_review_continuity: bool,
    /// The derived readiness state drawn from the frozen vocabulary.
    pub readiness_state: M5GovernanceReadinessState,
    /// `true` only when the derived readiness is a clean, met gate.
    pub is_clean_pass: bool,
    /// The row actions this row always offers (always includes open + nearest-forum).
    pub row_actions: Vec<M5MilestoneRowAction>,
    /// The degrade reason, present when the row is not a clean pass.
    pub degrade_reason: Option<M5MilestoneDegradeReason>,
    /// The next action, present when the row is degraded.
    pub next_action: Option<M5DecisionMilestoneNextAction>,
    /// A self-contained readiness note naming owner, blocker/waiver counts, gate, and
    /// forum, present always.
    pub readiness_note: String,
    /// A self-contained degrade note, present when the row is degraded.
    pub degrade_note: Option<String>,
}

/// Errors returned by [`resolve_milestone_dashboard_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MilestoneRowResolutionError {
    /// The milestone id was empty.
    EmptyMilestoneId,
    /// The milestone name was empty.
    EmptyMilestoneName,
    /// The owning team alias was empty.
    EmptyOwningTeam,
    /// The next-review representation was empty.
    EmptyNextReview,
    /// The owning-team alias carried a personal contact detail (an `@`), not a role alias.
    PersonContactDetailInAlias,
    /// A milestone id, name, owning-team alias, or next-review repr carried forbidden
    /// material.
    ForbiddenMilestoneMaterial,
}

impl M5MilestoneRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyMilestoneId => "empty_milestone_id",
            Self::EmptyMilestoneName => "empty_milestone_name",
            Self::EmptyOwningTeam => "empty_owning_team",
            Self::EmptyNextReview => "empty_next_review",
            Self::PersonContactDetailInAlias => "person_contact_detail_in_alias",
            Self::ForbiddenMilestoneMaterial => "forbidden_milestone_material",
        }
    }
}

impl fmt::Display for M5MilestoneRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "milestone-dashboard-row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MilestoneRowResolutionError {}

/// Resolves one milestone dashboard row from its declared state.
///
/// The derived readiness state is computed in a fixed degrade-first order: an unknown
/// freshness reading is `not_evaluated`; an unresolved owner is `owner_unresolved` (so
/// readiness stays paired with accountable ownership); a missing nearest review forum is
/// `forum_unresolved`; a blocked gate or any open blocker blocks; a waived gate or any open
/// waiver is `waived` (never a clean, met gate); stale-or-missing gate evidence is
/// `evidence_stale`; a pending gate is `warning`; and aging evidence is `warning`. Only a
/// milestone with a met exit gate, zero open blockers, zero open waivers, a resolved owner,
/// a resolved nearest forum, and fresh evidence is a clean pass — so milestone readiness
/// never drifts into a summary-only reading.
pub fn resolve_milestone_dashboard_row(
    input: &M5MilestoneRowResolutionInput,
) -> Result<M5ResolvedMilestoneRow, M5MilestoneRowResolutionError> {
    if input.milestone_id_repr.trim().is_empty() {
        return Err(M5MilestoneRowResolutionError::EmptyMilestoneId);
    }
    if input.milestone_name_repr.trim().is_empty() {
        return Err(M5MilestoneRowResolutionError::EmptyMilestoneName);
    }
    if input.owning_team_alias.trim().is_empty() {
        return Err(M5MilestoneRowResolutionError::EmptyOwningTeam);
    }
    if input.next_review_repr.trim().is_empty() {
        return Err(M5MilestoneRowResolutionError::EmptyNextReview);
    }
    if input.owning_team_alias.contains('@') {
        return Err(M5MilestoneRowResolutionError::PersonContactDetailInAlias);
    }
    if value_repr_is_forbidden(&input.milestone_id_repr)
        || value_repr_is_forbidden(&input.milestone_name_repr)
        || value_repr_is_forbidden(&input.owning_team_alias)
        || value_repr_is_forbidden(&input.next_review_repr)
    {
        return Err(M5MilestoneRowResolutionError::ForbiddenMilestoneMaterial);
    }

    let owner_accountable = !matches!(
        input.owner_coverage,
        M5OwnershipCoverageState::OwnerUnresolved
    );
    let degrade_reason = derive_milestone_degrade(
        input.owner_coverage,
        input.blocker_count,
        input.waiver_count,
        input.gate_state,
        input.nearest_review_forum,
        input.evidence_freshness,
    );
    let readiness_state = match degrade_reason {
        Some(reason) => reason.readiness_state(),
        None => M5GovernanceReadinessState::Passing,
    };
    let next_action = degrade_reason.map(M5MilestoneDegradeReason::next_action);
    let readiness_note = format!(
        "Milestone readiness: owner `{}` (accountable `{}`), blockers {}, waivers {}, gate `{}`, nearest forum `{}`",
        input.owning_team_alias,
        owner_accountable,
        input.blocker_count,
        input.waiver_count,
        input.gate_state.as_str(),
        input.nearest_review_forum.as_str(),
    );
    let degrade_note = degrade_reason.map(|reason| {
        format!(
            "Milestone row degraded: {} — state `{}`; next: {}",
            reason.phrase(),
            readiness_state.as_str(),
            reason.next_action().as_str()
        )
    });

    Ok(M5ResolvedMilestoneRow {
        milestone_id_repr: input.milestone_id_repr.clone(),
        milestone_name_repr: input.milestone_name_repr.clone(),
        owning_team_alias: input.owning_team_alias.clone(),
        owner_coverage: input.owner_coverage,
        blocker_count: input.blocker_count,
        waiver_count: input.waiver_count,
        gate_state: input.gate_state,
        nearest_review_forum: input.nearest_review_forum,
        next_review_repr: input.next_review_repr.clone(),
        evidence_freshness: input.evidence_freshness,
        owner_accountable,
        ownership_visible: true,
        blocker_waiver_truth_visible: true,
        nearest_forum_visible: true,
        next_review_continuity: true,
        readiness_state,
        is_clean_pass: readiness_state.is_clean_pass(),
        row_actions: vec![
            M5MilestoneRowAction::OpenMilestoneBoard,
            M5MilestoneRowAction::OpenNearestReviewForum,
            M5MilestoneRowAction::ExportMilestoneLedger,
        ],
        degrade_reason,
        next_action,
        readiness_note,
        degrade_note,
    })
}

/// The fixed degrade-first milestone ladder. Returns `None` for a clean, met gate.
fn derive_milestone_degrade(
    owner_coverage: M5OwnershipCoverageState,
    blocker_count: u32,
    waiver_count: u32,
    gate: M5MilestoneGateState,
    nearest_forum: M5DecisionForumClass,
    freshness: M5EvidenceFreshness,
) -> Option<M5MilestoneDegradeReason> {
    if matches!(freshness, M5EvidenceFreshness::EvidenceUnknown) {
        Some(M5MilestoneDegradeReason::NotYetEvaluated)
    } else if matches!(owner_coverage, M5OwnershipCoverageState::OwnerUnresolved) {
        Some(M5MilestoneDegradeReason::MilestoneOwnerUnresolved)
    } else if matches!(nearest_forum, M5DecisionForumClass::NoAuthorizedForum) {
        Some(M5MilestoneDegradeReason::NoNearestReviewForum)
    } else if matches!(gate, M5MilestoneGateState::ExitGateBlocked) || blocker_count > 0 {
        Some(M5MilestoneDegradeReason::MilestoneGateBlocked)
    } else if matches!(gate, M5MilestoneGateState::ExitGateWaived) || waiver_count > 0 {
        Some(M5MilestoneDegradeReason::MilestoneGateWaived)
    } else if matches!(gate, M5MilestoneGateState::ExitGateStale)
        || matches!(freshness, M5EvidenceFreshness::EvidenceStale)
        || matches!(freshness, M5EvidenceFreshness::EvidenceMissing)
    {
        Some(M5MilestoneDegradeReason::MilestoneEvidenceStale)
    } else if matches!(gate, M5MilestoneGateState::ExitGatePending) {
        Some(M5MilestoneDegradeReason::MilestoneGatePending)
    } else if matches!(freshness, M5EvidenceFreshness::EvidenceAging) {
        Some(M5MilestoneDegradeReason::MilestoneEvidenceAging)
    } else {
        // ExitGateMet, zero blockers, zero waivers, resolved owner, resolved nearest forum,
        // fresh evidence.
        None
    }
}

/// One worked milestone-dashboard-row resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MilestoneRowCase {
    /// The resolver input.
    pub input: M5MilestoneRowResolutionInput,
    /// The resolved truth. Must equal `resolve_milestone_dashboard_row(&input)`.
    pub resolved: M5ResolvedMilestoneRow,
}

impl M5MilestoneRowCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5MilestoneRowResolutionInput) -> Self {
        let resolved =
            resolve_milestone_dashboard_row(&input).expect("seed milestone-row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_milestone_dashboard_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

// ---------------------------------------------------------------------------
// Parity matrix
// ---------------------------------------------------------------------------

/// One row in the controls matrix: one governance consumer bound to the shared card and
/// row anatomy, readiness states, decision-forum classes, decision-right states,
/// milestone-gate states, satisfaction states, evidence-freshness readings, degrade
/// reasons, actions, export fields, and accessibility routes, plus worked resolution cases
/// for both resolver halves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionMilestoneRow {
    /// Governance consumer family.
    pub consumer_surface: M5DecisionMilestoneConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5GovernanceQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 governance surface families that render / consume these components.
    pub surface_families: Vec<M5GovernanceSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts these components render (must include the mandatory parts).
    pub anatomy_parts: Vec<M5DecisionMilestoneAnatomyPart>,
    /// Required labels these components can show (must include the mandatory labels).
    pub required_labels: Vec<M5GovernanceRequiredLabel>,
    /// Readiness states these components distinguish.
    pub readiness_states: Vec<M5GovernanceReadinessState>,
    /// Decision-forum classes these cards distinguish.
    pub decision_forum_classes: Vec<M5DecisionForumClass>,
    /// Decision-right states these cards distinguish.
    pub decision_right_states: Vec<M5DecisionRightState>,
    /// Review satisfaction states these cards distinguish.
    pub satisfaction_states: Vec<M5ReviewSatisfactionState>,
    /// Decision-right degrade reasons these cards name.
    pub decision_degrade_reasons: Vec<M5DecisionRightDegradeReason>,
    /// Milestone-gate states these rows distinguish.
    pub milestone_gate_states: Vec<M5MilestoneGateState>,
    /// Owner-coverage states these rows distinguish.
    pub owner_coverage_states: Vec<M5OwnershipCoverageState>,
    /// Milestone degrade reasons these rows name.
    pub milestone_degrade_reasons: Vec<M5MilestoneDegradeReason>,
    /// Evidence-freshness readings these components distinguish.
    pub evidence_freshness_states: Vec<M5EvidenceFreshness>,
    /// Card actions these rows offer (must include the mandatory actions).
    pub card_actions: Vec<M5DecisionCardAction>,
    /// Row actions these rows offer (must include the mandatory actions).
    pub row_actions: Vec<M5MilestoneRowAction>,
    /// Next actions these components name.
    pub next_actions: Vec<M5DecisionMilestoneNextAction>,
    /// Export fields these components carry (must include the mandatory fields).
    pub export_fields: Vec<M5DecisionMilestoneExportField>,
    /// Non-visual accessibility routes these components offer.
    pub accessibility_routes: Vec<M5GovernanceAccessibilityRoute>,
    /// Governance subsystems that consume these components' projection.
    pub consumer_surfaces: Vec<M5GovernanceConsumerSurface>,
    /// Downgrade triggers that apply to these components.
    pub downgrade_triggers: Vec<M5GovernanceDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked decision-right-card cases proving the card resolver on this consumer.
    pub decision_examples: Vec<M5DecisionRightCardCase>,
    /// Worked milestone-dashboard-row cases proving the row resolver on this consumer.
    pub milestone_examples: Vec<M5MilestoneRowCase>,
    /// Hard invariant: this row never lets a surface read `ready` while a forum or gate can
    /// still block it when governance review is required. MUST be `false`.
    pub lets_ready_hide_a_blocking_forum_or_gate: bool,
    /// Hard invariant: this row never lets an advisory forum read as authoritative. MUST be
    /// `false`.
    pub lets_advisory_forum_read_authoritative: bool,
    /// Hard invariant: this row never drifts milestone readiness away from ownership and
    /// blocker/waiver truth. MUST be `false`.
    pub drifts_milestone_readiness_from_ownership_and_counts: bool,
    /// Hard invariant: this row never invents a decision-right-local status word. MUST be
    /// `false`.
    pub invents_decision_local_status_grammar: bool,
}

impl M5DecisionMilestoneRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DecisionMilestoneAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DecisionMilestoneAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory required label.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5GovernanceRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5GovernanceRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// True when the row declares every mandatory card action.
    fn declares_mandatory_card_actions(&self) -> bool {
        let present: BTreeSet<M5DecisionCardAction> = self.card_actions.iter().copied().collect();
        M5DecisionCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// True when the row declares every mandatory row action.
    fn declares_mandatory_row_actions(&self) -> bool {
        let present: BTreeSet<M5MilestoneRowAction> = self.row_actions.iter().copied().collect();
        M5MilestoneRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DecisionMilestoneExportField> =
            self.export_fields.iter().copied().collect();
        M5DecisionMilestoneExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_ready_hide_a_blocking_forum_or_gate
            && !self.lets_advisory_forum_read_authoritative
            && !self.drifts_milestone_readiness_from_ownership_and_counts
            && !self.invents_decision_local_status_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionMilestoneVocabularySet {
    /// Governance consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Readiness-state tokens (reused from the frozen matrix).
    pub readiness_states: Vec<String>,
    /// Decision-forum-class tokens (reused from the frozen matrix).
    pub decision_forum_classes: Vec<String>,
    /// Decision-right-state tokens (reused from the frozen matrix).
    pub decision_right_states: Vec<String>,
    /// Review-satisfaction-state tokens.
    pub satisfaction_states: Vec<String>,
    /// Decision-right-degrade-reason tokens.
    pub decision_degrade_reasons: Vec<String>,
    /// Milestone-gate-state tokens (reused from the frozen matrix).
    pub milestone_gate_states: Vec<String>,
    /// Owner-coverage-state tokens (reused from the frozen matrix).
    pub owner_coverage_states: Vec<String>,
    /// Milestone-degrade-reason tokens.
    pub milestone_degrade_reasons: Vec<String>,
    /// Evidence-freshness tokens.
    pub evidence_freshness_states: Vec<String>,
    /// Card-action tokens.
    pub card_actions: Vec<String>,
    /// Row-action tokens.
    pub row_actions: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5DecisionMilestoneVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5DecisionMilestoneConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DecisionMilestoneAnatomyPart::ALL, |v| v.as_str()),
            readiness_states: tokens(&M5GovernanceReadinessState::ALL, |v| v.as_str()),
            decision_forum_classes: tokens(&M5DecisionForumClass::ALL, |v| v.as_str()),
            decision_right_states: tokens(&M5DecisionRightState::ALL, |v| v.as_str()),
            satisfaction_states: tokens(&M5ReviewSatisfactionState::ALL, |v| v.as_str()),
            decision_degrade_reasons: tokens(&M5DecisionRightDegradeReason::ALL, |v| v.as_str()),
            milestone_gate_states: tokens(&M5MilestoneGateState::ALL, |v| v.as_str()),
            owner_coverage_states: tokens(&M5OwnershipCoverageState::ALL, |v| v.as_str()),
            milestone_degrade_reasons: tokens(&M5MilestoneDegradeReason::ALL, |v| v.as_str()),
            evidence_freshness_states: tokens(&M5EvidenceFreshness::ALL, |v| v.as_str()),
            card_actions: tokens(&M5DecisionCardAction::ALL, |v| v.as_str()),
            row_actions: tokens(&M5MilestoneRowAction::ALL, |v| v.as_str()),
            next_actions: tokens(&M5DecisionMilestoneNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DecisionMilestoneExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5GovernanceAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionMilestoneReview {
    /// One controls packet carries decision-right and milestone truth on every consumer.
    pub one_packet_carries_decision_and_milestone_truth: bool,
    /// The required forum and reason for review are shown before a decision is trusted.
    pub required_forum_and_reason_always_shown: bool,
    /// A surface never reads `ready` while a forum or gate can still block it when
    /// governance review is required.
    pub ready_never_hides_a_blocking_forum_or_gate: bool,
    /// An advisory forum is never rendered as authoritative.
    pub advisory_forum_never_reads_authoritative: bool,
    /// The satisfied/pending state and target milestone are always shown on the card.
    pub satisfaction_state_and_target_always_shown: bool,
    /// Milestone readiness stays paired with accountable ownership.
    pub milestone_readiness_paired_with_ownership: bool,
    /// The blocker and waiver counts stay visible on every milestone row.
    pub blocker_and_waiver_counts_always_shown: bool,
    /// The readiness state is drawn only from the frozen vocabulary.
    pub readiness_state_drawn_from_frozen_vocabulary: bool,
    /// Shiproom, operator, and support surfaces reuse one decision-right/milestone model.
    pub shiproom_operator_support_reuse_one_model: bool,
    /// The support / export packet reconstructs card and row truth.
    pub support_export_reconstructs_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// An owning-team or forum alias is a role alias, never a personal contact detail.
    pub owner_alias_is_role_not_person: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionMilestoneConsumerProjection {
    /// Shiproom, operator, release, support, and CLI consumers all consume the shared
    /// controls packet.
    pub surfaces_consume_shared_packet: bool,
    /// The decision-right resolver reads a single canonical source.
    pub decision_resolver_reads_single_source: bool,
    /// The milestone resolver reads a single canonical source.
    pub milestone_resolver_reads_single_source: bool,
    /// The nearest-review-forum reading reads a single canonical source.
    pub nearest_forum_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionMilestoneProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the controls packet.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionMilestoneReleasePosture {
    /// Ref of the supporting governance packet.
    pub governance_packet_ref: String,
    /// Ref of the supporting assurance audit.
    pub assurance_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DecisionRightMilestoneControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DecisionRightMilestoneControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DecisionMilestoneRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DecisionMilestoneVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DecisionMilestoneReview,
    /// Consumer projection block.
    pub consumer_projection: M5DecisionMilestoneConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DecisionMilestoneProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DecisionMilestoneReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 decision-right / milestone controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DecisionRightMilestoneControlsPacket {
    /// Record kind; must equal [`M5_DECISION_RIGHT_MILESTONE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DecisionMilestoneRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DecisionMilestoneVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DecisionMilestoneReview,
    /// Consumer projection block.
    pub consumer_projection: M5DecisionMilestoneConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DecisionMilestoneProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DecisionMilestoneReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DecisionRightMilestoneControlsPacket {
    /// Builds an M5 decision-right / milestone controls packet from stable-lane input.
    pub fn new(input: M5DecisionRightMilestoneControlsPacketInput) -> Self {
        Self {
            record_kind: M5_DECISION_RIGHT_MILESTONE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            controls_rows: input.controls_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 decision-right / milestone controls invariants.
    pub fn validate(&self) -> Vec<M5DecisionRightMilestoneControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DECISION_RIGHT_MILESTONE_CONTROLS_RECORD_KIND {
            violations.push(M5DecisionRightMilestoneControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5DecisionRightMilestoneControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DecisionRightMilestoneControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_controls_rows(self, &mut violations);
        validate_ready_shows_blocking_forum_proven(self, &mut violations);
        validate_milestone_readiness_paired_proven(self, &mut violations);
        validate_shared_model_proven(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 decision-right/milestone controls serializes"),
        ) {
            violations.push(M5DecisionRightMilestoneControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 decision-right/milestone controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governance consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,readiness_states,decision_forum_classes,decision_right_states,milestone_gate_states,card_actions,row_actions,export_fields,decision_example_count,milestone_example_count\n",
        );
        for row in &self.controls_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.readiness_states, |v| v.as_str()),
                join_tokens(&row.decision_forum_classes, |v| v.as_str()),
                join_tokens(&row.decision_right_states, |v| v.as_str()),
                join_tokens(&row.milestone_gate_states, |v| v.as_str()),
                join_tokens(&row.card_actions, |v| v.as_str()),
                join_tokens(&row.row_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.decision_examples.len(),
                row.milestone_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .controls_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Decision-Right Card and Milestone Dashboard Row Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Governance consumers: {} ({} stable)\n",
            self.controls_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Readiness states: {}\n",
            self.vocabulary_set.readiness_states.join(", ")
        ));
        out.push_str(&format!(
            "- Decision-forum classes: {}\n",
            self.vocabulary_set.decision_forum_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Milestone-gate states: {}\n",
            self.vocabulary_set.milestone_gate_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Governance consumers\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked decision cards: {}\n",
                row.decision_examples.len()
            ));
            for case in &row.decision_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (forum `{}`, state `{}`, satisfaction `{}`, review-required `{}`, blocking-shown `{}`)\n",
                    case.resolved.card_id_repr,
                    case.resolved.readiness_state.as_str(),
                    case.resolved.required_forum.as_str(),
                    case.resolved.decision_state.as_str(),
                    case.resolved.satisfaction_state.as_str(),
                    case.resolved.governance_review_required,
                    case.resolved.blocking_forum_or_gate_shown,
                ));
            }
            out.push_str(&format!(
                "  - Worked milestone rows: {}\n",
                row.milestone_examples.len()
            ));
            for case in &row.milestone_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (owner-accountable `{}`, blockers {}, waivers {}, gate `{}`, nearest forum `{}`)\n",
                    case.resolved.milestone_id_repr,
                    case.resolved.readiness_state.as_str(),
                    case.resolved.owner_accountable,
                    case.resolved.blocker_count,
                    case.resolved.waiver_count,
                    case.resolved.gate_state.as_str(),
                    case.resolved.nearest_review_forum.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 decision-right / milestone controls
/// export.
#[derive(Debug)]
pub enum M5DecisionRightMilestoneControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DecisionRightMilestoneControlsViolation>),
}

impl fmt::Display for M5DecisionRightMilestoneControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 decision-right/milestone controls export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 decision-right/milestone controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DecisionRightMilestoneControlsArtifactError {}

/// Validation failures emitted by [`M5DecisionRightMilestoneControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DecisionRightMilestoneControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governance consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory required labels.
    MandatoryLabelMissing,
    /// A controls row omits one of the mandatory card actions.
    MandatoryCardActionMissing,
    /// A controls row omits one of the mandatory row actions.
    MandatoryRowActionMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A controls row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A controls row declares no decision-right worked cases.
    DecisionExampleMissing,
    /// A controls row declares no milestone worked cases.
    MilestoneExampleMissing,
    /// A worked decision-right case does not match a fresh resolve of its input.
    DecisionExampleDrift,
    /// A worked milestone case does not match a fresh resolve of its input.
    MilestoneExampleDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked case proves that a review-required surface cannot read `ready` while a
    /// forum can still block it, and that an advisory forum never reads authoritative (the
    /// AC-1 example).
    ReadyHidingBlockingForumUnproven,
    /// No worked case proves that milestone readiness stays paired with accountable
    /// ownership and current blocker/waiver truth (the AC-2 example).
    MilestoneReadinessPairingUnproven,
    /// The shiproom, operator, and support consumers do not all reuse the shared
    /// decision-right/milestone model with worked cases.
    SharedModelUnproven,
    /// A controls row violates a hard invariant.
    ControlsInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DecisionRightMilestoneControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::MandatoryCardActionMissing => "mandatory_card_action_missing",
            Self::MandatoryRowActionMissing => "mandatory_row_action_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::DecisionExampleMissing => "decision_example_missing",
            Self::MilestoneExampleMissing => "milestone_example_missing",
            Self::DecisionExampleDrift => "decision_example_drift",
            Self::MilestoneExampleDrift => "milestone_example_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ReadyHidingBlockingForumUnproven => "ready_hiding_blocking_forum_unproven",
            Self::MilestoneReadinessPairingUnproven => "milestone_readiness_pairing_unproven",
            Self::SharedModelUnproven => "shared_model_unproven",
            Self::ControlsInvariantViolated => "controls_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 decision-right / milestone controls export.
pub fn current_stable_m5_decision_right_milestone_controls_export(
) -> Result<M5DecisionRightMilestoneControlsPacket, M5DecisionRightMilestoneControlsArtifactError> {
    let packet: M5DecisionRightMilestoneControlsPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-decision-right-milestone-controls-proof/support_export.json"
    )))
        .map_err(M5DecisionRightMilestoneControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DecisionRightMilestoneControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF,
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_DOC_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF,
        M5_DECISION_RIGHT_CARD_CONTRACT_REF,
        M5_MILESTONE_DASHBOARD_ROW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DecisionRightMilestoneControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DecisionRightMilestoneControlsViolation::VocabularySetDrift);
    }
}

fn validate_controls_rows(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    let present: BTreeSet<M5DecisionMilestoneConsumerSurface> = packet
        .controls_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5DecisionMilestoneConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5DecisionRightMilestoneControlsViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.readiness_states.is_empty()
            || row.decision_forum_classes.is_empty()
            || row.decision_right_states.is_empty()
            || row.satisfaction_states.is_empty()
            || row.decision_degrade_reasons.is_empty()
            || row.milestone_gate_states.is_empty()
            || row.owner_coverage_states.is_empty()
            || row.milestone_degrade_reasons.is_empty()
            || row.evidence_freshness_states.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5DecisionRightMilestoneControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DecisionRightMilestoneControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5DecisionRightMilestoneControlsViolation::MandatoryLabelMissing);
        }
        if !row.declares_mandatory_card_actions() {
            violations.push(M5DecisionRightMilestoneControlsViolation::MandatoryCardActionMissing);
        }
        if !row.declares_mandatory_row_actions() {
            violations.push(M5DecisionRightMilestoneControlsViolation::MandatoryRowActionMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DecisionRightMilestoneControlsViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5DecisionRightMilestoneControlsViolation::AccessibilityRouteMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DecisionRightMilestoneControlsViolation::DowngradeTriggersMissing);
        }
        if row.decision_examples.is_empty() {
            violations.push(M5DecisionRightMilestoneControlsViolation::DecisionExampleMissing);
        }
        if row.milestone_examples.is_empty() {
            violations.push(M5DecisionRightMilestoneControlsViolation::MilestoneExampleMissing);
        }
        if row
            .decision_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DecisionRightMilestoneControlsViolation::DecisionExampleDrift);
        }
        if row
            .milestone_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DecisionRightMilestoneControlsViolation::MilestoneExampleDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DecisionRightMilestoneControlsViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DecisionRightMilestoneControlsViolation::ControlsInvariantViolated);
        }
    }
}

/// At least one worked decision-right case across the matrix must prove both halves of
/// AC-1: a review-required card that never reads a clean pass while a real, named forum can
/// still block it (the blocking forum is shown), and an advisory-only forum that reads
/// `warning` rather than authoritative.
fn validate_ready_shows_blocking_forum_proven(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    let blocking_forum_shown = packet.controls_rows.iter().any(|row| {
        row.decision_examples.iter().any(|case| {
            case.resolved.governance_review_required
                && !case.resolved.is_clean_pass
                && case.resolved.blocking_forum_or_gate_shown
                && !matches!(
                    case.resolved.required_forum,
                    M5DecisionForumClass::NoAuthorizedForum
                )
        })
    });
    let advisory_not_authoritative = packet.controls_rows.iter().any(|row| {
        row.decision_examples.iter().any(|case| {
            matches!(
                case.resolved.decision_state,
                M5DecisionRightState::AdvisoryOnly
            ) && !case.resolved.decision_authoritative
                && case.resolved.readiness_state == M5GovernanceReadinessState::Warning
        })
    });
    if !blocking_forum_shown || !advisory_not_authoritative {
        violations
            .push(M5DecisionRightMilestoneControlsViolation::ReadyHidingBlockingForumUnproven);
    }
}

/// At least one worked milestone case across the matrix must prove AC-2: milestone
/// readiness stays paired with accountable ownership and current blocker/waiver truth. A
/// milestone with an open blocker or waiver never reads as a met gate (with ownership and
/// counts still visible), and a milestone with an unresolved owner reads `owner_unresolved`
/// rather than drifting into a summary-only pass.
fn validate_milestone_readiness_paired_proven(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    let blocker_case = packet.controls_rows.iter().any(|row| {
        row.milestone_examples.iter().any(|case| {
            case.resolved.blocker_count > 0
                && !case.resolved.is_clean_pass
                && case.resolved.blocker_waiver_truth_visible
                && case.resolved.ownership_visible
                && case.resolved.owner_accountable
        })
    });
    let waiver_case = packet.controls_rows.iter().any(|row| {
        row.milestone_examples.iter().any(|case| {
            case.resolved.waiver_count > 0
                && !case.resolved.is_clean_pass
                && case.resolved.readiness_state == M5GovernanceReadinessState::Waived
                && case.resolved.blocker_waiver_truth_visible
        })
    });
    let owner_unresolved_case = packet.controls_rows.iter().any(|row| {
        row.milestone_examples.iter().any(|case| {
            matches!(
                case.resolved.owner_coverage,
                M5OwnershipCoverageState::OwnerUnresolved
            ) && !case.resolved.owner_accountable
                && case.resolved.readiness_state == M5GovernanceReadinessState::OwnerUnresolved
        })
    });
    if !blocker_case || !waiver_case || !owner_unresolved_case {
        violations
            .push(M5DecisionRightMilestoneControlsViolation::MilestoneReadinessPairingUnproven);
    }
}

/// The shiproom, operator, and support consumers must each be present and reuse the shared
/// decision-right/milestone model, each carrying at least one worked decision card and one
/// worked milestone row.
fn validate_shared_model_proven(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    let shared = M5DecisionMilestoneConsumerSurface::SHARED_MODEL_REQUIRED
        .iter()
        .all(|required| {
            packet.controls_rows.iter().any(|row| {
                row.consumer_surface == *required
                    && !row.decision_examples.is_empty()
                    && !row.milestone_examples.is_empty()
            })
        });
    if !shared {
        violations.push(M5DecisionRightMilestoneControlsViolation::SharedModelUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_packet_carries_decision_and_milestone_truth,
        review.required_forum_and_reason_always_shown,
        review.ready_never_hides_a_blocking_forum_or_gate,
        review.advisory_forum_never_reads_authoritative,
        review.satisfaction_state_and_target_always_shown,
        review.milestone_readiness_paired_with_ownership,
        review.blocker_and_waiver_counts_always_shown,
        review.readiness_state_drawn_from_frozen_vocabulary,
        review.shiproom_operator_support_reuse_one_model,
        review.support_export_reconstructs_truth,
        review.every_row_declares_accessibility_route,
        review.owner_alias_is_role_not_person,
    ] {
        if !ok {
            violations.push(M5DecisionRightMilestoneControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.surfaces_consume_shared_packet,
        projection.decision_resolver_reads_single_source,
        projection.milestone_resolver_reads_single_source,
        projection.nearest_forum_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5DecisionRightMilestoneControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DecisionRightMilestoneControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DecisionRightMilestoneControlsPacket,
    violations: &mut Vec<M5DecisionRightMilestoneControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.governance_packet_ref.trim().is_empty()
        || posture.assurance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DecisionRightMilestoneControlsViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
