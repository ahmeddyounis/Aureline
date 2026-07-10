//! Three reusable M5 governance-dashboard primitives implemented as one controls
//! packet: the **waiver-expiry queue item** (owner, waiver lifecycle state, expiry,
//! affected milestone or release, mitigation status, and an always-present open-detail
//! action), the **release-gate banner** (blocker count, waived count, stale-evidence
//! count, ship/no-ship decision, user-facing mitigation, fallback path, and
//! packet/export continuity), and the **mitigation note card** (the user-facing
//! mitigation text and its clarity), projected the same way across every claimed M5
//! governance surface.
//!
//! Aureline's frozen governance-dashboard component matrix
//! ([`crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`])
//! names the waiver-expiry queue item, the release-gate banner, and the mitigation note
//! card as three governed component families and freezes their shared readiness-state
//! vocabulary, the waiver-expiry states, the release-gate decisions, and the mitigation
//! postures. This module *implements* those three contracts as one reusable controls
//! packet so a user, an operator, or a release reviewer can tell — from the queue item,
//! the banner, and the card alone — who owns a temporary exception, when it expires,
//! which milestone or release it holds, how many blockers or waived or stale-evidence
//! items stand between the lane and a ship decision, and what the mitigation actually
//! is in plain language, before that truth hides behind a dashboard or release prose.
//!
//! The packet has two resolver halves:
//!
//! 1. [`resolve_waiver_expiry_item`] takes one waiver's identity, the failure it holds,
//!    its lifecycle state, the affected milestone or release, its mitigation posture,
//!    owner alias, expiry, and evidence freshness, and produces one
//!    [`M5ResolvedWaiverExpiryItem`] carrying the *derived* readiness state drawn from
//!    the frozen [`M5GovernanceReadinessState`] vocabulary, an always-visible expiry, and
//!    an always-present open-detail action. An active or expiring waiver that holds a
//!    failure never resolves to `passing`: it reads `waived` (with an expiry that stays
//!    visible) or, once lapsed, `expired_waiver`.
//! 2. [`resolve_release_gate`] takes one release gate's blocker, waived, and
//!    stale-evidence counts, its declared ship/no-ship decision, its mitigation posture,
//!    the user-facing mitigation text, the fallback path, and evidence freshness, and
//!    produces one [`M5ResolvedReleaseGate`] carrying the derived readiness state, an
//!    honestly-derived gate decision (a `go` with open blockers never stays `go`), a
//!    [`M5MitigationClarity`] reading that names whether the mitigation note is plain
//!    language or collapsed into internal-only jargon, and the fallback-path and
//!    packet/export continuity a user needs.
//!
//! A parity matrix — [`M5WaiverGateControlsPacket`] — binds one row per claimed M5
//! governance consumer (the assurance dashboard, the operator board, the shiproom
//! packet, the CLI inspect, and the support export) to the shared queue-item, banner,
//! and card anatomy, the same readiness states, waiver-expiry states, release-gate
//! decisions, mitigation postures, mitigation-clarity readings, degrade reasons, next
//! actions, and export fields, plus worked resolution cases that must reproduce the
//! resolver output exactly, so the waiver/gate/mitigation vocabulary stays identical
//! across the assurance center, the operator board, the shiproom, the CLI, and
//! support/export.
//!
//! The frozen readiness-state vocabulary ([`M5GovernanceReadinessState`]), the
//! waiver-expiry state ([`M5WaiverExpiryState`]), the release-gate decision
//! ([`M5ReleaseGateDecision`]), the mitigation posture ([`M5MitigationPosture`]), the
//! deployment line ([`M5DeploymentLine`]), the governance surface family
//! ([`M5GovernanceSurfaceFamily`]), the governance consumer surface
//! ([`M5GovernanceConsumerSurface`]), the accessibility route
//! ([`M5GovernanceAccessibilityRoute`]), the required label
//! ([`M5GovernanceRequiredLabel`]), the qualification class
//! ([`M5GovernanceQualificationClass`]), and the downgrade trigger
//! ([`M5GovernanceDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the queue
//! item, the banner, and the card themselves: their governance consumer families, their
//! anatomy parts, the shared evidence-freshness input, the affected-target kinds, the
//! waiver and gate degrade reasons, the mitigation-clarity readings, the next actions,
//! the queue-item and gate actions, and the export fields. No M5 governance surface
//! invents a second waiver, gate, or mitigation grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user text bodies stay
//! outside the support boundary; every waiver id, held-failure ref, affected-target id,
//! owner alias, expiry, and fallback path is carried only as an opaque, export-safe
//! representation, and an owner alias is a role alias, never a personal contact detail.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_waiver_gate_controls_operator_board_preview_narrowed,
    seeded_m5_waiver_gate_controls_packet,
    seeded_m5_waiver_gate_controls_shiproom_packet_beta_narrowed,
    M5_WAIVER_GATE_CONTROLS_PACKET_ID,
};

// The readiness state vocabulary, the waiver-expiry states, the release-gate decisions,
// the mitigation postures, the deployment lines, the surface families, the consumer
// surfaces, the accessibility routes, the required labels, the qualification classes,
// and the downgrade triggers are frozen once, in the governance-dashboard component
// matrix. This controls packet reuses them verbatim so it never invents a parallel
// vocabulary.
pub use crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix::{
    M5DeploymentLine, M5GovernanceAccessibilityRoute, M5GovernanceConsumerSurface,
    M5GovernanceDowngradeTrigger, M5GovernanceQualificationClass, M5GovernanceReadinessState,
    M5GovernanceRequiredLabel, M5GovernanceSurfaceFamily, M5MitigationPosture,
    M5ReleaseGateDecision, M5WaiverExpiryState,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5WaiverGateControlsPacket`].
pub const M5_WAIVER_GATE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_waiver_expiry_queue_items_release_gate_banners_and_mitigation_note_cards_across_claimed_m5_governance_surfaces";

/// Schema version for M5 waiver/gate/mitigation controls records.
pub const M5_WAIVER_GATE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const M5_WAIVER_GATE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-waiver-gate-mitigation-controls.schema.json";

/// Repo-relative path of the controls contract doc.
pub const M5_WAIVER_GATE_CONTROLS_DOC_REF: &str =
    "docs/help/m5_waiver_expiry_release_gate_and_mitigation_note_controls.md";

/// Repo-relative path of the frozen governance-dashboard component matrix schema this
/// controls packet narrows from.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-matrix.schema.json";

/// Repo-relative path of the frozen governance-dashboard component matrix doc.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_components_contract.md";

/// Repo-relative path of the per-component waiver-expiry-queue-item contract schema.
pub const M5_WAIVER_EXPIRY_QUEUE_ITEM_CONTRACT_REF: &str =
    "schemas/ui/m5-waiver-expiry-queue-item.schema.json";

/// Repo-relative path of the per-component release-gate-banner contract schema.
pub const M5_RELEASE_GATE_BANNER_CONTRACT_REF: &str =
    "schemas/ui/m5-release-gate-banner.schema.json";

/// Repo-relative path of the per-component mitigation-note-card contract schema.
pub const M5_MITIGATION_NOTE_CARD_CONTRACT_REF: &str =
    "schemas/ui/m5-mitigation-note-card.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WAIVER_GATE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-waiver-gate-mitigation-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_WAIVER_GATE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-waiver-gate-mitigation-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_WAIVER_GATE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-waiver-gate-mitigation-controls-proof/summary.md";

// ---------------------------------------------------------------------------
// Minted vocabulary
// ---------------------------------------------------------------------------

/// One claimed M5 governance consumer that renders the shared waiver-expiry queue item,
/// release-gate banner, and mitigation note card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WaiverGateConsumerSurface {
    /// The assurance-center dashboard.
    AssuranceDashboard,
    /// The operator overview board.
    OperatorBoard,
    /// The shiproom packet.
    ShiproomPacket,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The support / export packet.
    SupportExport,
}

impl M5WaiverGateConsumerSurface {
    /// Every claimed governance consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AssuranceDashboard,
        Self::OperatorBoard,
        Self::ShiproomPacket,
        Self::CliInspect,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceDashboard => "assurance_dashboard",
            Self::OperatorBoard => "operator_board",
            Self::ShiproomPacket => "shiproom_packet",
            Self::CliInspect => "cli_inspect",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceDashboard => "Assurance Dashboard",
            Self::OperatorBoard => "Operator Board",
            Self::ShiproomPacket => "Shiproom Packet",
            Self::CliInspect => "CLI Inspect",
            Self::SupportExport => "Support / Export",
        }
    }
}

/// One anatomy part the shared queue item / banner / card surfaces. The parts in
/// [`M5WaiverGateAnatomyPart::MANDATORY`] are required on every row so a user can orient
/// before trusting a temporary exception or a ship decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WaiverGateAnatomyPart {
    /// The waiver identity (queue-item identity).
    WaiverIdentity,
    /// The waiver lifecycle state cue.
    WaiverState,
    /// The waiver expiry cue.
    Expiry,
    /// The affected milestone or release cue.
    AffectedTarget,
    /// The mitigation-status cue.
    MitigationStatus,
    /// The owner cue.
    OwnerCue,
    /// The open-detail action.
    OpenDetailAction,
    /// The release-gate decision cue (banner identity).
    GateDecision,
    /// The blocker-count cue.
    BlockerCount,
    /// The waived-count cue.
    WaivedCount,
    /// The stale-evidence-count cue.
    StaleEvidenceCount,
    /// The user-facing mitigation text (card body).
    UserFacingMitigation,
    /// The fallback-path cue.
    FallbackPath,
}

impl M5WaiverGateAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::WaiverIdentity,
        Self::WaiverState,
        Self::Expiry,
        Self::AffectedTarget,
        Self::MitigationStatus,
        Self::OwnerCue,
        Self::OpenDetailAction,
        Self::GateDecision,
        Self::BlockerCount,
        Self::WaivedCount,
        Self::StaleEvidenceCount,
        Self::UserFacingMitigation,
        Self::FallbackPath,
    ];

    /// The anatomy parts every row must render before an exception or gate is trusted.
    pub const MANDATORY: [Self; 4] = [
        Self::WaiverIdentity,
        Self::WaiverState,
        Self::GateDecision,
        Self::UserFacingMitigation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WaiverIdentity => "waiver_identity",
            Self::WaiverState => "waiver_state",
            Self::Expiry => "expiry",
            Self::AffectedTarget => "affected_target",
            Self::MitigationStatus => "mitigation_status",
            Self::OwnerCue => "owner_cue",
            Self::OpenDetailAction => "open_detail_action",
            Self::GateDecision => "gate_decision",
            Self::BlockerCount => "blocker_count",
            Self::WaivedCount => "waived_count",
            Self::StaleEvidenceCount => "stale_evidence_count",
            Self::UserFacingMitigation => "user_facing_mitigation",
            Self::FallbackPath => "fallback_path",
        }
    }
}

/// The evidence-freshness reading shared by both resolvers, so a queue item or a banner
/// never shows stale or missing evidence as clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshness {
    /// Evidence is fresh within its freshness window.
    EvidenceFresh,
    /// Evidence is aging but still within tolerance.
    EvidenceAging,
    /// Evidence is stale relative to the current build.
    EvidenceStale,
    /// Required evidence is missing.
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

/// What a waiver-expiry queue item holds up — the milestone or release it affects, so a
/// queue item never leaves its blast radius implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffectedTargetKind {
    /// A milestone exit gate.
    MilestoneTarget,
    /// A release train.
    ReleaseTrainTarget,
    /// A single service.
    ServiceTarget,
    /// The whole fleet.
    FleetTarget,
    /// The affected target is not yet recorded.
    TargetUnrecorded,
}

impl M5AffectedTargetKind {
    /// Every affected-target kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MilestoneTarget,
        Self::ReleaseTrainTarget,
        Self::ServiceTarget,
        Self::FleetTarget,
        Self::TargetUnrecorded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MilestoneTarget => "milestone_target",
            Self::ReleaseTrainTarget => "release_train_target",
            Self::ServiceTarget => "service_target",
            Self::FleetTarget => "fleet_target",
            Self::TargetUnrecorded => "target_unrecorded",
        }
    }
}

/// The next action named on a degraded queue item, banner, or card, so a non-passing
/// reading is actionable rather than a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WaiverGateNextAction {
    /// Open the underlying detail.
    OpenDetail,
    /// Renew or retire the expiring or lapsed waiver.
    RenewOrRetireWaiver,
    /// Resolve the open blocker.
    ResolveBlocker,
    /// Refresh the stale or missing evidence.
    RefreshEvidence,
    /// Resolve the unresolved owner or decision forum.
    ResolveOwnerOrForum,
    /// Clarify the mitigation so it reads in plain language.
    ClarifyMitigation,
}

impl M5WaiverGateNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDetail,
        Self::RenewOrRetireWaiver,
        Self::ResolveBlocker,
        Self::RefreshEvidence,
        Self::ResolveOwnerOrForum,
        Self::ClarifyMitigation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetail => "open_detail",
            Self::RenewOrRetireWaiver => "renew_or_retire_waiver",
            Self::ResolveBlocker => "resolve_blocker",
            Self::RefreshEvidence => "refresh_evidence",
            Self::ResolveOwnerOrForum => "resolve_owner_or_forum",
            Self::ClarifyMitigation => "clarify_mitigation",
        }
    }
}

/// The exact reason a waiver-expiry queue item degraded below a clean pass, so a waived
/// failure never reads like a clean pass and an expiring or lapsed waiver stays visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WaiverDegradeReason {
    /// The item has not been evaluated on this build.
    NotYetEvaluated,
    /// The item has no resolved owner.
    OwnerUnresolvedForItem,
    /// The waiver has expired or was revoked; its held failure is no longer covered.
    WaiverExpiredOrRevoked,
    /// Required evidence for the item is missing.
    EvidenceMissingForItem,
    /// The item's evidence is stale relative to this build.
    EvidenceStaleForItem,
    /// A failure with no waiver is blocking.
    UnwaivedFailureBlocking,
    /// The failure is held under a disclosed, still-valid waiver.
    WaivedUnderDisclosure,
    /// The waiver is active but expiring soon and stays visible.
    WaiverExpiringSoon,
}

impl M5WaiverDegradeReason {
    /// Every waiver degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotYetEvaluated,
        Self::OwnerUnresolvedForItem,
        Self::WaiverExpiredOrRevoked,
        Self::EvidenceMissingForItem,
        Self::EvidenceStaleForItem,
        Self::UnwaivedFailureBlocking,
        Self::WaivedUnderDisclosure,
        Self::WaiverExpiringSoon,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "not_yet_evaluated",
            Self::OwnerUnresolvedForItem => "owner_unresolved_for_item",
            Self::WaiverExpiredOrRevoked => "waiver_expired_or_revoked",
            Self::EvidenceMissingForItem => "evidence_missing_for_item",
            Self::EvidenceStaleForItem => "evidence_stale_for_item",
            Self::UnwaivedFailureBlocking => "unwaived_failure_blocking",
            Self::WaivedUnderDisclosure => "waived_under_disclosure",
            Self::WaiverExpiringSoon => "waiver_expiring_soon",
        }
    }

    /// The frozen readiness state this degrade reason resolves to.
    pub const fn readiness_state(self) -> M5GovernanceReadinessState {
        match self {
            Self::NotYetEvaluated => M5GovernanceReadinessState::NotEvaluated,
            Self::OwnerUnresolvedForItem => M5GovernanceReadinessState::OwnerUnresolved,
            Self::WaiverExpiredOrRevoked => M5GovernanceReadinessState::ExpiredWaiver,
            Self::EvidenceMissingForItem => M5GovernanceReadinessState::Blocked,
            Self::EvidenceStaleForItem => M5GovernanceReadinessState::EvidenceStale,
            Self::UnwaivedFailureBlocking => M5GovernanceReadinessState::Blocked,
            Self::WaivedUnderDisclosure => M5GovernanceReadinessState::Waived,
            Self::WaiverExpiringSoon => M5GovernanceReadinessState::Waived,
        }
    }

    /// The next action a reviewer should take to clear this degrade.
    pub const fn next_action(self) -> M5WaiverGateNextAction {
        match self {
            Self::NotYetEvaluated => M5WaiverGateNextAction::OpenDetail,
            Self::OwnerUnresolvedForItem => M5WaiverGateNextAction::ResolveOwnerOrForum,
            Self::WaiverExpiredOrRevoked => M5WaiverGateNextAction::RenewOrRetireWaiver,
            Self::EvidenceMissingForItem => M5WaiverGateNextAction::RefreshEvidence,
            Self::EvidenceStaleForItem => M5WaiverGateNextAction::RefreshEvidence,
            Self::UnwaivedFailureBlocking => M5WaiverGateNextAction::ResolveBlocker,
            Self::WaivedUnderDisclosure => M5WaiverGateNextAction::OpenDetail,
            Self::WaiverExpiringSoon => M5WaiverGateNextAction::RenewOrRetireWaiver,
        }
    }

    /// Review-safe reason phrase for the queue item's degrade note.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "the queue item has not been evaluated on this build",
            Self::OwnerUnresolvedForItem => "the queue item has no resolved owner",
            Self::WaiverExpiredOrRevoked => {
                "the waiver has expired or was revoked and no longer covers its failure"
            }
            Self::EvidenceMissingForItem => "required evidence for the item is missing",
            Self::EvidenceStaleForItem => "the item's evidence is stale relative to this build",
            Self::UnwaivedFailureBlocking => "a failure with no waiver is blocking",
            Self::WaivedUnderDisclosure => {
                "the failure is held under a disclosed, still-valid waiver"
            }
            Self::WaiverExpiringSoon => "the waiver is active but expiring soon",
        }
    }
}

/// Whether a mitigation note reads in plain language a user, support, or release
/// reviewer can act on, or whether it has collapsed into internal-only jargon — the
/// acceptance-criteria reading a mitigation note card must carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MitigationClarity {
    /// The mitigation reads in plain language and stays understandable.
    PlainLanguage,
    /// The mitigation collapsed into internal-only jargon.
    JargonDetected,
    /// No user-facing mitigation text was provided.
    MitigationAbsent,
}

impl M5MitigationClarity {
    /// Every mitigation-clarity reading, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PlainLanguage,
        Self::JargonDetected,
        Self::MitigationAbsent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainLanguage => "plain_language",
            Self::JargonDetected => "jargon_detected",
            Self::MitigationAbsent => "mitigation_absent",
        }
    }

    /// `true` only for [`Self::PlainLanguage`]: the sole reading a user, support, or
    /// release reviewer can act on without decoding internal jargon.
    pub const fn is_understandable(self) -> bool {
        matches!(self, Self::PlainLanguage)
    }
}

/// The exact reason a release-gate banner degraded below a clean `go`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GateDegradeReason {
    /// The gate has not been evaluated on this build.
    NotYetEvaluated,
    /// The gate is held by an unresolved owner or decision forum.
    OwnerOrForumUnresolved,
    /// Required gate evidence is missing.
    EvidenceMissingForGate,
    /// The gate's evidence is stale relative to this build.
    EvidenceStaleForGate,
    /// One or more hard blockers are open.
    BlockersOpen,
    /// One or more blockers are held under still-valid waivers.
    WaivedItemsPending,
    /// The mitigation note is missing or collapsed into internal-only jargon.
    MitigationUnclear,
    /// The mitigation is only partial, or the risk is merely accepted.
    MitigationIncomplete,
}

impl M5GateDegradeReason {
    /// Every gate degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotYetEvaluated,
        Self::OwnerOrForumUnresolved,
        Self::EvidenceMissingForGate,
        Self::EvidenceStaleForGate,
        Self::BlockersOpen,
        Self::WaivedItemsPending,
        Self::MitigationUnclear,
        Self::MitigationIncomplete,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "not_yet_evaluated",
            Self::OwnerOrForumUnresolved => "owner_or_forum_unresolved",
            Self::EvidenceMissingForGate => "evidence_missing_for_gate",
            Self::EvidenceStaleForGate => "evidence_stale_for_gate",
            Self::BlockersOpen => "blockers_open",
            Self::WaivedItemsPending => "waived_items_pending",
            Self::MitigationUnclear => "mitigation_unclear",
            Self::MitigationIncomplete => "mitigation_incomplete",
        }
    }

    /// The frozen readiness state this degrade reason resolves to.
    pub const fn readiness_state(self) -> M5GovernanceReadinessState {
        match self {
            Self::NotYetEvaluated => M5GovernanceReadinessState::NotEvaluated,
            Self::OwnerOrForumUnresolved => M5GovernanceReadinessState::ForumUnresolved,
            Self::EvidenceMissingForGate => M5GovernanceReadinessState::Blocked,
            Self::EvidenceStaleForGate => M5GovernanceReadinessState::EvidenceStale,
            Self::BlockersOpen => M5GovernanceReadinessState::Blocked,
            Self::WaivedItemsPending => M5GovernanceReadinessState::Waived,
            Self::MitigationUnclear => M5GovernanceReadinessState::Warning,
            Self::MitigationIncomplete => M5GovernanceReadinessState::Warning,
        }
    }

    /// The honestly-derived ship/no-ship decision this degrade reason implies.
    pub const fn gate_decision(self) -> M5ReleaseGateDecision {
        match self {
            Self::NotYetEvaluated => M5ReleaseGateDecision::HeldPendingEvidence,
            Self::OwnerOrForumUnresolved => M5ReleaseGateDecision::BlockedByOwnerOrForum,
            Self::EvidenceMissingForGate => M5ReleaseGateDecision::HeldPendingEvidence,
            Self::EvidenceStaleForGate => M5ReleaseGateDecision::HeldPendingEvidence,
            Self::BlockersOpen => M5ReleaseGateDecision::NoGo,
            Self::WaivedItemsPending => M5ReleaseGateDecision::ConditionalGo,
            Self::MitigationUnclear => M5ReleaseGateDecision::ConditionalGo,
            Self::MitigationIncomplete => M5ReleaseGateDecision::ConditionalGo,
        }
    }

    /// The next action a reviewer should take to clear this degrade.
    pub const fn next_action(self) -> M5WaiverGateNextAction {
        match self {
            Self::NotYetEvaluated => M5WaiverGateNextAction::RefreshEvidence,
            Self::OwnerOrForumUnresolved => M5WaiverGateNextAction::ResolveOwnerOrForum,
            Self::EvidenceMissingForGate => M5WaiverGateNextAction::RefreshEvidence,
            Self::EvidenceStaleForGate => M5WaiverGateNextAction::RefreshEvidence,
            Self::BlockersOpen => M5WaiverGateNextAction::ResolveBlocker,
            Self::WaivedItemsPending => M5WaiverGateNextAction::OpenDetail,
            Self::MitigationUnclear => M5WaiverGateNextAction::ClarifyMitigation,
            Self::MitigationIncomplete => M5WaiverGateNextAction::ClarifyMitigation,
        }
    }

    /// Review-safe reason phrase for the banner's degrade note.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NotYetEvaluated => "the gate has not been evaluated on this build",
            Self::OwnerOrForumUnresolved => {
                "the gate is held by an unresolved owner or decision forum"
            }
            Self::EvidenceMissingForGate => "required gate evidence is missing",
            Self::EvidenceStaleForGate => "the gate's evidence is stale relative to this build",
            Self::BlockersOpen => "one or more hard blockers are open",
            Self::WaivedItemsPending => "one or more blockers are held under still-valid waivers",
            Self::MitigationUnclear => {
                "the mitigation note is missing or collapsed into internal-only jargon"
            }
            Self::MitigationIncomplete => "the mitigation is only partial or the risk is accepted",
        }
    }
}

/// An action a waiver-expiry queue item offers. The actions in
/// [`M5WaiverItemAction::MANDATORY`] are required on every row so a user can always open
/// the underlying detail of a temporary exception.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WaiverItemAction {
    /// Open the queue item's detail.
    OpenDetail,
    /// Compare the waiver's history.
    CompareWaiverHistory,
    /// Export the waiver ledger.
    ExportWaiverLedger,
    /// Escalate the waiver to its owner or forum.
    EscalateWaiver,
}

impl M5WaiverItemAction {
    /// Every queue-item action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenDetail,
        Self::CompareWaiverHistory,
        Self::ExportWaiverLedger,
        Self::EscalateWaiver,
    ];

    /// The queue-item actions every row must offer.
    pub const MANDATORY: [Self; 1] = [Self::OpenDetail];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetail => "open_detail",
            Self::CompareWaiverHistory => "compare_waiver_history",
            Self::ExportWaiverLedger => "export_waiver_ledger",
            Self::EscalateWaiver => "escalate_waiver",
        }
    }
}

/// An action a release-gate banner offers. The actions in [`M5GateAction::MANDATORY`]
/// are required on every row so a reviewer can always open the release packet and follow
/// the fallback path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GateAction {
    /// Open the underlying release packet.
    OpenReleasePacket,
    /// Follow the stated fallback path.
    FollowFallbackPath,
    /// Compare the gate's history.
    CompareGateHistory,
    /// Export the gate packet.
    ExportGatePacket,
}

impl M5GateAction {
    /// Every gate action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenReleasePacket,
        Self::FollowFallbackPath,
        Self::CompareGateHistory,
        Self::ExportGatePacket,
    ];

    /// The gate actions every row must offer.
    pub const MANDATORY: [Self; 2] = [Self::OpenReleasePacket, Self::FollowFallbackPath];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenReleasePacket => "open_release_packet",
            Self::FollowFallbackPath => "follow_fallback_path",
            Self::CompareGateHistory => "compare_gate_history",
            Self::ExportGatePacket => "export_gate_packet",
        }
    }
}

/// A field the support / export packet carries so queue-item, banner, and card truth is
/// reconstructable from the shared model. The fields in
/// [`M5WaiverGateExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WaiverGateExportField {
    /// The opaque waiver id.
    WaiverId,
    /// The waiver lifecycle state.
    WaiverState,
    /// The opaque expiry.
    Expiry,
    /// The affected-target kind.
    AffectedTarget,
    /// The mitigation posture.
    MitigationPosture,
    /// The owner alias.
    OwnerAlias,
    /// The derived readiness state.
    ReadinessState,
    /// The release-gate decision.
    GateDecision,
    /// The blocker count.
    BlockerCount,
    /// The waived count.
    WaivedCount,
    /// The stale-evidence count.
    StaleEvidenceCount,
}

impl M5WaiverGateExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::WaiverId,
        Self::WaiverState,
        Self::Expiry,
        Self::AffectedTarget,
        Self::MitigationPosture,
        Self::OwnerAlias,
        Self::ReadinessState,
        Self::GateDecision,
        Self::BlockerCount,
        Self::WaivedCount,
        Self::StaleEvidenceCount,
    ];

    /// The export fields every controls export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::WaiverId,
        Self::WaiverState,
        Self::Expiry,
        Self::ReadinessState,
        Self::GateDecision,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WaiverId => "waiver_id",
            Self::WaiverState => "waiver_state",
            Self::Expiry => "expiry",
            Self::AffectedTarget => "affected_target",
            Self::MitigationPosture => "mitigation_posture",
            Self::OwnerAlias => "owner_alias",
            Self::ReadinessState => "readiness_state",
            Self::GateDecision => "gate_decision",
            Self::BlockerCount => "blocker_count",
            Self::WaivedCount => "waived_count",
            Self::StaleEvidenceCount => "stale_evidence_count",
        }
    }
}

// ---------------------------------------------------------------------------
// Waiver-expiry-item resolver
// ---------------------------------------------------------------------------

/// The full input to the waiver-expiry-item resolver for one queue item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WaiverExpiryItemResolutionInput {
    /// The opaque, export-safe waiver id.
    pub waiver_id_repr: String,
    /// The opaque, export-safe reference to the failure the waiver holds.
    pub held_failure_repr: String,
    /// The waiver lifecycle state.
    pub waiver_state: M5WaiverExpiryState,
    /// The kind of milestone or release the item affects.
    pub affected_target: M5AffectedTargetKind,
    /// The opaque, export-safe id of the affected milestone or release.
    pub affected_target_repr: String,
    /// The mitigation posture of the held failure.
    pub mitigation_posture: M5MitigationPosture,
    /// The opaque owner role alias (never a personal contact detail).
    pub owner_alias: String,
    /// The opaque, export-safe waiver expiry representation.
    pub expiry_repr: String,
    /// The evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
}

/// The resolved waiver-expiry-item truth for one queue item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWaiverExpiryItem {
    /// The opaque waiver id.
    pub waiver_id_repr: String,
    /// The opaque held-failure ref.
    pub held_failure_repr: String,
    /// The waiver lifecycle state.
    pub waiver_state: M5WaiverExpiryState,
    /// The affected-target kind.
    pub affected_target: M5AffectedTargetKind,
    /// The opaque affected-target id.
    pub affected_target_repr: String,
    /// The mitigation posture.
    pub mitigation_posture: M5MitigationPosture,
    /// The opaque owner alias.
    pub owner_alias: String,
    /// `true` when the item has a resolved owner.
    pub owner_resolved: bool,
    /// The opaque waiver expiry.
    pub expiry_repr: String,
    /// The evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// The derived readiness state drawn from the frozen vocabulary.
    pub readiness_state: M5GovernanceReadinessState,
    /// `true` only when the derived readiness is a clean pass.
    pub is_clean_pass: bool,
    /// `true` always: the waiver expiry stays visible wherever the item is summarized.
    pub expiry_visible: bool,
    /// The queue-item actions this row always offers (always includes open-detail).
    pub item_actions: Vec<M5WaiverItemAction>,
    /// The degrade reason, present when the item is not a clean pass.
    pub degrade_reason: Option<M5WaiverDegradeReason>,
    /// The next action, present when the item is degraded.
    pub next_action: Option<M5WaiverGateNextAction>,
    /// A self-contained degrade note naming the reason and next action, present when the
    /// item is degraded.
    pub degrade_note: Option<String>,
}

/// Errors returned by [`resolve_waiver_expiry_item`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WaiverExpiryItemResolutionError {
    /// The waiver id was empty.
    EmptyWaiverId,
    /// The waiver expiry was empty.
    EmptyExpiry,
    /// The owner alias carried a personal contact detail (an `@`), not a role alias.
    PersonContactDetailInAlias,
    /// A waiver id, held-failure ref, target id, owner alias, or expiry carried
    /// forbidden material.
    ForbiddenItemMaterial,
}

impl M5WaiverExpiryItemResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyWaiverId => "empty_waiver_id",
            Self::EmptyExpiry => "empty_expiry",
            Self::PersonContactDetailInAlias => "person_contact_detail_in_alias",
            Self::ForbiddenItemMaterial => "forbidden_item_material",
        }
    }
}

impl fmt::Display for M5WaiverExpiryItemResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "waiver-expiry-item resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WaiverExpiryItemResolutionError {}

/// Resolves one waiver-expiry queue item from its declared state.
///
/// The derived readiness state is computed in a fixed degrade-first order: an unknown
/// evidence reading is `not_evaluated`, an unresolved owner is `owner_unresolved`, an
/// expired or revoked waiver is `expired_waiver`, missing evidence blocks, stale
/// evidence is `evidence_stale`, an unwaived failure blocks, and a still-valid waiver —
/// whether active or expiring soon — is `waived`, never `passing`. Only a queue item
/// with no active waiver, a fully-mitigated held failure, fresh evidence, and a resolved
/// owner is a clean pass (the exception has been retired). A waived or expiring failure
/// therefore never reads as a clean pass, and the expiry stays visible on every item.
pub fn resolve_waiver_expiry_item(
    input: &M5WaiverExpiryItemResolutionInput,
) -> Result<M5ResolvedWaiverExpiryItem, M5WaiverExpiryItemResolutionError> {
    if input.waiver_id_repr.trim().is_empty() {
        return Err(M5WaiverExpiryItemResolutionError::EmptyWaiverId);
    }
    if input.expiry_repr.trim().is_empty() {
        return Err(M5WaiverExpiryItemResolutionError::EmptyExpiry);
    }
    if input.owner_alias.contains('@') {
        return Err(M5WaiverExpiryItemResolutionError::PersonContactDetailInAlias);
    }
    if value_repr_is_forbidden(&input.waiver_id_repr)
        || value_repr_is_forbidden(&input.held_failure_repr)
        || value_repr_is_forbidden(&input.affected_target_repr)
        || value_repr_is_forbidden(&input.owner_alias)
        || value_repr_is_forbidden(&input.expiry_repr)
    {
        return Err(M5WaiverExpiryItemResolutionError::ForbiddenItemMaterial);
    }

    let owner_resolved = !input.owner_alias.trim().is_empty();
    let degrade_reason = derive_waiver_degrade(
        input.waiver_state,
        input.evidence_freshness,
        input.mitigation_posture,
        owner_resolved,
    );
    let readiness_state = match degrade_reason {
        Some(reason) => reason.readiness_state(),
        None => M5GovernanceReadinessState::Passing,
    };
    let next_action = degrade_reason.map(M5WaiverDegradeReason::next_action);
    let degrade_note = degrade_reason.map(|reason| {
        format!(
            "Waiver-expiry item degraded: {} — state `{}`; next: {}",
            reason.phrase(),
            readiness_state.as_str(),
            reason.next_action().as_str()
        )
    });

    Ok(M5ResolvedWaiverExpiryItem {
        waiver_id_repr: input.waiver_id_repr.clone(),
        held_failure_repr: input.held_failure_repr.clone(),
        waiver_state: input.waiver_state,
        affected_target: input.affected_target,
        affected_target_repr: input.affected_target_repr.clone(),
        mitigation_posture: input.mitigation_posture,
        owner_alias: input.owner_alias.clone(),
        owner_resolved,
        expiry_repr: input.expiry_repr.clone(),
        evidence_freshness: input.evidence_freshness,
        readiness_state,
        is_clean_pass: readiness_state.is_clean_pass(),
        expiry_visible: true,
        item_actions: vec![
            M5WaiverItemAction::OpenDetail,
            M5WaiverItemAction::CompareWaiverHistory,
            M5WaiverItemAction::ExportWaiverLedger,
        ],
        degrade_reason,
        next_action,
        degrade_note,
    })
}

/// The fixed degrade-first waiver ladder. Returns `None` for a clean pass.
fn derive_waiver_degrade(
    waiver_state: M5WaiverExpiryState,
    evidence: M5EvidenceFreshness,
    mitigation: M5MitigationPosture,
    owner_resolved: bool,
) -> Option<M5WaiverDegradeReason> {
    if matches!(evidence, M5EvidenceFreshness::EvidenceUnknown) {
        Some(M5WaiverDegradeReason::NotYetEvaluated)
    } else if !owner_resolved {
        Some(M5WaiverDegradeReason::OwnerUnresolvedForItem)
    } else if matches!(
        waiver_state,
        M5WaiverExpiryState::ExpiredWaiver | M5WaiverExpiryState::RevokedWaiver
    ) {
        Some(M5WaiverDegradeReason::WaiverExpiredOrRevoked)
    } else if matches!(evidence, M5EvidenceFreshness::EvidenceMissing) {
        Some(M5WaiverDegradeReason::EvidenceMissingForItem)
    } else if matches!(evidence, M5EvidenceFreshness::EvidenceStale) {
        Some(M5WaiverDegradeReason::EvidenceStaleForItem)
    } else if matches!(waiver_state, M5WaiverExpiryState::ExpiringSoon) {
        Some(M5WaiverDegradeReason::WaiverExpiringSoon)
    } else if matches!(waiver_state, M5WaiverExpiryState::ActiveWaiver) {
        Some(M5WaiverDegradeReason::WaivedUnderDisclosure)
    } else if !matches!(mitigation, M5MitigationPosture::Mitigated)
        || matches!(evidence, M5EvidenceFreshness::EvidenceAging)
    {
        // No active waiver (NoWaiver): the held failure is only a clean pass once it is
        // fully mitigated with fresh evidence; otherwise it is an unwaived blocker.
        Some(M5WaiverDegradeReason::UnwaivedFailureBlocking)
    } else {
        None
    }
}

/// One worked waiver-expiry-item resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WaiverExpiryItemCase {
    /// The resolver input.
    pub input: M5WaiverExpiryItemResolutionInput,
    /// The resolved truth. Must equal `resolve_waiver_expiry_item(&input)`.
    pub resolved: M5ResolvedWaiverExpiryItem,
}

impl M5WaiverExpiryItemCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5WaiverExpiryItemResolutionInput) -> Self {
        let resolved =
            resolve_waiver_expiry_item(&input).expect("seed waiver-expiry-item case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_waiver_expiry_item(&self.input).as_ref() == Ok(&self.resolved)
    }
}

// ---------------------------------------------------------------------------
// Release-gate resolver (release-gate banner + mitigation note card)
// ---------------------------------------------------------------------------

/// The full input to the release-gate resolver for one gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseGateResolutionInput {
    /// The opaque, export-safe gate id.
    pub gate_id_repr: String,
    /// The count of open hard blockers.
    pub blocker_count: u32,
    /// The count of blockers held under still-valid waivers.
    pub waived_count: u32,
    /// The count of stale-evidence items.
    pub stale_evidence_count: u32,
    /// The declared ship/no-ship decision (never shown as the final decision alone).
    pub declared_decision: M5ReleaseGateDecision,
    /// The mitigation posture of the gate.
    pub mitigation_posture: M5MitigationPosture,
    /// The user-facing mitigation text (the mitigation note card body).
    pub user_facing_mitigation: String,
    /// The opaque, export-safe fallback-path representation.
    pub fallback_path_repr: String,
    /// The evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// `true` when the gate's owner and decision forum are resolved.
    pub owner_or_forum_resolved: bool,
}

/// The resolved release-gate truth for one gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedReleaseGate {
    /// The opaque gate id.
    pub gate_id_repr: String,
    /// The count of open hard blockers.
    pub blocker_count: u32,
    /// The count of blockers held under still-valid waivers.
    pub waived_count: u32,
    /// The count of stale-evidence items.
    pub stale_evidence_count: u32,
    /// The declared ship/no-ship decision.
    pub declared_decision: M5ReleaseGateDecision,
    /// The honestly-derived ship/no-ship decision.
    pub resolved_decision: M5ReleaseGateDecision,
    /// The mitigation posture.
    pub mitigation_posture: M5MitigationPosture,
    /// The user-facing mitigation text.
    pub user_facing_mitigation: String,
    /// The derived mitigation-clarity reading.
    pub mitigation_clarity: M5MitigationClarity,
    /// `true` when the mitigation reads in plain language.
    pub mitigation_understandable: bool,
    /// The opaque fallback path.
    pub fallback_path_repr: String,
    /// The evidence-freshness reading.
    pub evidence_freshness: M5EvidenceFreshness,
    /// Whether the gate's owner and decision forum are resolved.
    pub owner_or_forum_resolved: bool,
    /// The derived readiness state drawn from the frozen vocabulary.
    pub readiness_state: M5GovernanceReadinessState,
    /// `true` only when the derived readiness is a clean pass.
    pub is_clean_pass: bool,
    /// The gate actions this row always offers (always includes open + fallback).
    pub gate_actions: Vec<M5GateAction>,
    /// `true` always: gate truth is reconstructable from the packet/export.
    pub packet_export_continuity: bool,
    /// The degrade reason, present when the gate is not a clean go.
    pub degrade_reason: Option<M5GateDegradeReason>,
    /// The next action, present when the gate is degraded.
    pub next_action: Option<M5WaiverGateNextAction>,
    /// A self-contained mitigation note naming the clarity reading, present always.
    pub mitigation_note: String,
    /// A self-contained degrade note, present when the gate is degraded.
    pub degrade_note: Option<String>,
}

/// Errors returned by [`resolve_release_gate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ReleaseGateResolutionError {
    /// The gate id was empty.
    EmptyGateId,
    /// The fallback path was empty.
    EmptyFallbackPath,
    /// A gate id, mitigation text, or fallback path carried forbidden material.
    ForbiddenGateMaterial,
}

impl M5ReleaseGateResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyGateId => "empty_gate_id",
            Self::EmptyFallbackPath => "empty_fallback_path",
            Self::ForbiddenGateMaterial => "forbidden_gate_material",
        }
    }
}

impl fmt::Display for M5ReleaseGateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "release-gate resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ReleaseGateResolutionError {}

/// Resolves the mitigation-clarity reading for one mitigation note.
///
/// A mitigation with no user-facing text is `mitigation_absent`. A mitigation whose text
/// collapses into internal-only jargon — a single opaque token, no plain sentence, or a
/// known internal-only marker — is `jargon_detected`. Only a mitigation that reads as a
/// plain-language sentence a user, support, or release reviewer can act on is
/// `plain_language`.
pub fn resolve_mitigation_clarity(user_facing_mitigation: &str) -> M5MitigationClarity {
    let trimmed = user_facing_mitigation.trim();
    if trimmed.is_empty() {
        M5MitigationClarity::MitigationAbsent
    } else if mitigation_is_plain_language(trimmed) {
        M5MitigationClarity::PlainLanguage
    } else {
        M5MitigationClarity::JargonDetected
    }
}

/// True when a mitigation note reads as a plain-language sentence rather than internal
/// jargon. A plain-language note is a multi-word sentence (at least four words and a
/// terminal period) that carries no known internal-only jargon marker.
fn mitigation_is_plain_language(text: &str) -> bool {
    let word_count = text.split_whitespace().count();
    let reads_like_sentence = word_count >= 4 && text.contains('.');
    reads_like_sentence && !contains_internal_jargon(text)
}

/// Known internal-only jargon markers a user-facing mitigation must never collapse into.
const INTERNAL_JARGON_MARKERS: [&str; 8] = [
    "wontfix",
    "icebox",
    "p0-only",
    "yolo",
    "tbd",
    "see internal",
    "ask the team",
    "n/a",
];

/// True when a mitigation note carries a known internal-only jargon marker.
fn contains_internal_jargon(text: &str) -> bool {
    let lower = text.to_lowercase();
    INTERNAL_JARGON_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}

/// Resolves one release-gate banner (and its mitigation note card) from its declared
/// state.
///
/// The derived readiness state is computed in a fixed degrade-first order: an unknown
/// evidence reading is `not_evaluated`, an unresolved owner or forum is
/// `forum_unresolved`, missing evidence blocks, stale evidence (or a non-zero
/// stale-evidence count) is `evidence_stale`, an open blocker blocks, a still-valid
/// waiver holds the gate at `waived`, an unclear or missing mitigation degrades to
/// `warning`, and an incomplete mitigation degrades to `warning`. Only a gate with no
/// open blockers, no waived items, no stale evidence, fresh evidence, a resolved owner
/// and forum, and a fully-mitigated, plain-language mitigation is a clean `go`. A `go`
/// declared over open blockers therefore never stays `go`.
pub fn resolve_release_gate(
    input: &M5ReleaseGateResolutionInput,
) -> Result<M5ResolvedReleaseGate, M5ReleaseGateResolutionError> {
    if input.gate_id_repr.trim().is_empty() {
        return Err(M5ReleaseGateResolutionError::EmptyGateId);
    }
    if input.fallback_path_repr.trim().is_empty() {
        return Err(M5ReleaseGateResolutionError::EmptyFallbackPath);
    }
    if value_repr_is_forbidden(&input.gate_id_repr)
        || value_repr_is_forbidden(&input.user_facing_mitigation)
        || value_repr_is_forbidden(&input.fallback_path_repr)
    {
        return Err(M5ReleaseGateResolutionError::ForbiddenGateMaterial);
    }

    let mitigation_clarity = resolve_mitigation_clarity(&input.user_facing_mitigation);
    let mitigation_understandable = mitigation_clarity.is_understandable();

    let degrade_reason = derive_gate_degrade(
        input.blocker_count,
        input.waived_count,
        input.stale_evidence_count,
        input.mitigation_posture,
        mitigation_clarity,
        input.evidence_freshness,
        input.owner_or_forum_resolved,
    );
    let readiness_state = match degrade_reason {
        Some(reason) => reason.readiness_state(),
        None => M5GovernanceReadinessState::Passing,
    };
    let resolved_decision = match degrade_reason {
        Some(reason) => reason.gate_decision(),
        None => M5ReleaseGateDecision::Go,
    };
    let next_action = degrade_reason.map(M5GateDegradeReason::next_action);
    let mitigation_note = format!(
        "Mitigation ({}): {}",
        mitigation_clarity.as_str(),
        match mitigation_clarity {
            M5MitigationClarity::PlainLanguage => input.user_facing_mitigation.trim().to_owned(),
            M5MitigationClarity::JargonDetected => {
                "the mitigation note collapsed into internal-only jargon and must be reworded so users, support, and release reviewers can act on it".to_owned()
            }
            M5MitigationClarity::MitigationAbsent =>
                "no user-facing mitigation was provided".to_owned(),
        }
    );
    let degrade_note = degrade_reason.map(|reason| {
        format!(
            "Release gate degraded: {} — state `{}`; decision `{}`; next: {}",
            reason.phrase(),
            readiness_state.as_str(),
            resolved_decision.as_str(),
            reason.next_action().as_str()
        )
    });

    Ok(M5ResolvedReleaseGate {
        gate_id_repr: input.gate_id_repr.clone(),
        blocker_count: input.blocker_count,
        waived_count: input.waived_count,
        stale_evidence_count: input.stale_evidence_count,
        declared_decision: input.declared_decision,
        resolved_decision,
        mitigation_posture: input.mitigation_posture,
        user_facing_mitigation: input.user_facing_mitigation.clone(),
        mitigation_clarity,
        mitigation_understandable,
        fallback_path_repr: input.fallback_path_repr.clone(),
        evidence_freshness: input.evidence_freshness,
        owner_or_forum_resolved: input.owner_or_forum_resolved,
        readiness_state,
        is_clean_pass: readiness_state.is_clean_pass(),
        gate_actions: vec![
            M5GateAction::OpenReleasePacket,
            M5GateAction::FollowFallbackPath,
            M5GateAction::ExportGatePacket,
        ],
        packet_export_continuity: true,
        degrade_reason,
        next_action,
        mitigation_note,
        degrade_note,
    })
}

/// The fixed degrade-first gate ladder. Returns `None` for a clean go.
#[allow(clippy::too_many_arguments)]
fn derive_gate_degrade(
    blocker_count: u32,
    waived_count: u32,
    stale_evidence_count: u32,
    mitigation: M5MitigationPosture,
    mitigation_clarity: M5MitigationClarity,
    evidence: M5EvidenceFreshness,
    owner_or_forum_resolved: bool,
) -> Option<M5GateDegradeReason> {
    if matches!(evidence, M5EvidenceFreshness::EvidenceUnknown) {
        Some(M5GateDegradeReason::NotYetEvaluated)
    } else if !owner_or_forum_resolved {
        Some(M5GateDegradeReason::OwnerOrForumUnresolved)
    } else if matches!(evidence, M5EvidenceFreshness::EvidenceMissing) {
        Some(M5GateDegradeReason::EvidenceMissingForGate)
    } else if matches!(evidence, M5EvidenceFreshness::EvidenceStale) || stale_evidence_count > 0 {
        Some(M5GateDegradeReason::EvidenceStaleForGate)
    } else if blocker_count > 0 {
        Some(M5GateDegradeReason::BlockersOpen)
    } else if waived_count > 0 {
        Some(M5GateDegradeReason::WaivedItemsPending)
    } else if !matches!(mitigation_clarity, M5MitigationClarity::PlainLanguage) {
        Some(M5GateDegradeReason::MitigationUnclear)
    } else if !matches!(mitigation, M5MitigationPosture::Mitigated)
        || matches!(evidence, M5EvidenceFreshness::EvidenceAging)
    {
        Some(M5GateDegradeReason::MitigationIncomplete)
    } else {
        None
    }
}

/// One worked release-gate resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseGateCase {
    /// The resolver input.
    pub input: M5ReleaseGateResolutionInput,
    /// The resolved truth. Must equal `resolve_release_gate(&input)`.
    pub resolved: M5ResolvedReleaseGate,
}

impl M5ReleaseGateCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ReleaseGateResolutionInput) -> Self {
        let resolved = resolve_release_gate(&input).expect("seed release-gate case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_release_gate(&self.input).as_ref() == Ok(&self.resolved)
    }
}

// ---------------------------------------------------------------------------
// Parity matrix
// ---------------------------------------------------------------------------

/// One row in the controls matrix: one governance consumer bound to the shared
/// queue-item, banner, and card anatomy, readiness states, waiver-expiry states,
/// release-gate decisions, mitigation postures, mitigation-clarity readings, degrade
/// reasons, actions, export fields, and accessibility routes, plus worked resolution
/// cases for both resolver halves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WaiverGateRow {
    /// Governance consumer family.
    pub consumer_surface: M5WaiverGateConsumerSurface,
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
    pub anatomy_parts: Vec<M5WaiverGateAnatomyPart>,
    /// Required labels these components can show (must include the mandatory labels).
    pub required_labels: Vec<M5GovernanceRequiredLabel>,
    /// Readiness states these components distinguish.
    pub readiness_states: Vec<M5GovernanceReadinessState>,
    /// Waiver-expiry states these queue items distinguish.
    pub waiver_expiry_states: Vec<M5WaiverExpiryState>,
    /// Affected-target kinds these queue items name.
    pub affected_target_kinds: Vec<M5AffectedTargetKind>,
    /// Mitigation postures these components distinguish.
    pub mitigation_postures: Vec<M5MitigationPosture>,
    /// Mitigation-clarity readings these cards distinguish.
    pub mitigation_clarities: Vec<M5MitigationClarity>,
    /// Evidence-freshness readings these components distinguish.
    pub evidence_freshness_states: Vec<M5EvidenceFreshness>,
    /// Waiver degrade reasons these queue items name.
    pub waiver_degrade_reasons: Vec<M5WaiverDegradeReason>,
    /// Release-gate decisions these banners distinguish.
    pub gate_decisions: Vec<M5ReleaseGateDecision>,
    /// Gate degrade reasons these banners name.
    pub gate_degrade_reasons: Vec<M5GateDegradeReason>,
    /// Queue-item actions these rows offer (must include the mandatory actions).
    pub item_actions: Vec<M5WaiverItemAction>,
    /// Gate actions these rows offer (must include the mandatory actions).
    pub gate_actions: Vec<M5GateAction>,
    /// Next actions these components name.
    pub next_actions: Vec<M5WaiverGateNextAction>,
    /// Export fields these components carry (must include the mandatory fields).
    pub export_fields: Vec<M5WaiverGateExportField>,
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
    /// Worked waiver-expiry-item cases proving the item resolver on this consumer.
    pub waiver_expiry_examples: Vec<M5WaiverExpiryItemCase>,
    /// Worked release-gate cases proving the gate resolver on this consumer.
    pub release_gate_examples: Vec<M5ReleaseGateCase>,
    /// Hard invariant: this row never renders a waived or expired exception as a clean
    /// pass. MUST be `false`.
    pub renders_waived_or_expired_as_clean_pass: bool,
    /// Hard invariant: this row never hides the waiver expiry or the owner. MUST be
    /// `false`.
    pub hides_waiver_expiry_or_owner: bool,
    /// Hard invariant: this row never hides mitigation behind internal jargon. MUST be
    /// `false`.
    pub hides_mitigation_behind_internal_jargon: bool,
    /// Hard invariant: this row never invents a gate-local status word. MUST be `false`.
    pub invents_gate_local_status_grammar: bool,
}

impl M5WaiverGateRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5WaiverGateAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5WaiverGateAnatomyPart::MANDATORY
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

    /// True when the row declares every mandatory queue-item action.
    fn declares_mandatory_item_actions(&self) -> bool {
        let present: BTreeSet<M5WaiverItemAction> = self.item_actions.iter().copied().collect();
        M5WaiverItemAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// True when the row declares every mandatory gate action.
    fn declares_mandatory_gate_actions(&self) -> bool {
        let present: BTreeSet<M5GateAction> = self.gate_actions.iter().copied().collect();
        M5GateAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5WaiverGateExportField> =
            self.export_fields.iter().copied().collect();
        M5WaiverGateExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.renders_waived_or_expired_as_clean_pass
            && !self.hides_waiver_expiry_or_owner
            && !self.hides_mitigation_behind_internal_jargon
            && !self.invents_gate_local_status_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WaiverGateVocabularySet {
    /// Governance consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Readiness-state tokens (reused from the frozen matrix).
    pub readiness_states: Vec<String>,
    /// Waiver-expiry-state tokens (reused from the frozen matrix).
    pub waiver_expiry_states: Vec<String>,
    /// Affected-target-kind tokens.
    pub affected_target_kinds: Vec<String>,
    /// Mitigation-posture tokens (reused from the frozen matrix).
    pub mitigation_postures: Vec<String>,
    /// Mitigation-clarity tokens.
    pub mitigation_clarities: Vec<String>,
    /// Evidence-freshness tokens.
    pub evidence_freshness_states: Vec<String>,
    /// Waiver-degrade-reason tokens.
    pub waiver_degrade_reasons: Vec<String>,
    /// Release-gate-decision tokens (reused from the frozen matrix).
    pub gate_decisions: Vec<String>,
    /// Gate-degrade-reason tokens.
    pub gate_degrade_reasons: Vec<String>,
    /// Queue-item-action tokens.
    pub item_actions: Vec<String>,
    /// Gate-action tokens.
    pub gate_actions: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5WaiverGateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5WaiverGateConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5WaiverGateAnatomyPart::ALL, |v| v.as_str()),
            readiness_states: tokens(&M5GovernanceReadinessState::ALL, |v| v.as_str()),
            waiver_expiry_states: tokens(&M5WaiverExpiryState::ALL, |v| v.as_str()),
            affected_target_kinds: tokens(&M5AffectedTargetKind::ALL, |v| v.as_str()),
            mitigation_postures: tokens(&M5MitigationPosture::ALL, |v| v.as_str()),
            mitigation_clarities: tokens(&M5MitigationClarity::ALL, |v| v.as_str()),
            evidence_freshness_states: tokens(&M5EvidenceFreshness::ALL, |v| v.as_str()),
            waiver_degrade_reasons: tokens(&M5WaiverDegradeReason::ALL, |v| v.as_str()),
            gate_decisions: tokens(&M5ReleaseGateDecision::ALL, |v| v.as_str()),
            gate_degrade_reasons: tokens(&M5GateDegradeReason::ALL, |v| v.as_str()),
            item_actions: tokens(&M5WaiverItemAction::ALL, |v| v.as_str()),
            gate_actions: tokens(&M5GateAction::ALL, |v| v.as_str()),
            next_actions: tokens(&M5WaiverGateNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5WaiverGateExportField::ALL, |v| v.as_str()),
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
pub struct M5WaiverGateReview {
    /// One controls packet carries waiver, gate, and mitigation truth on every consumer.
    pub one_packet_carries_waiver_gate_and_mitigation_truth: bool,
    /// The waiver identity and gate decision are shown before an exception is trusted.
    pub identity_and_gate_decision_always_shown: bool,
    /// A waived or expiring failure never reads as a clean pass.
    pub waived_or_expiring_never_reads_clean_pass: bool,
    /// The waiver expiry stays visible wherever the item is summarized.
    pub waiver_expiry_always_visible: bool,
    /// A blocker with no owner or forum never reads resolved.
    pub ownerless_or_forumless_blocker_never_resolved: bool,
    /// The blocker, waived, and stale-evidence counts are always shown on the banner.
    pub blocker_waived_stale_counts_always_shown: bool,
    /// The mitigation note stays understandable, never internal-only jargon.
    pub mitigation_stays_understandable: bool,
    /// The readiness state is drawn only from the frozen vocabulary.
    pub readiness_state_drawn_from_frozen_vocabulary: bool,
    /// The support / export packet reconstructs queue-item, banner, and card truth.
    pub support_export_reconstructs_truth: bool,
    /// No consumer invents a second waiver, gate, or mitigation grammar.
    pub no_surface_invents_second_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// An owner alias is a role alias, never a personal contact detail.
    pub owner_alias_is_role_not_person: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WaiverGateConsumerProjection {
    /// Assurance, operator, shiproom, CLI, and support consumers all consume the shared
    /// controls packet.
    pub surfaces_consume_shared_packet: bool,
    /// The readiness resolver reads a single canonical source.
    pub readiness_resolver_reads_single_source: bool,
    /// The mitigation-clarity reading reads a single canonical source.
    pub mitigation_clarity_reads_single_source: bool,
    /// The waiver-expiry visibility reads a single canonical source.
    pub waiver_expiry_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WaiverGateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the controls packet.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WaiverGateReleasePosture {
    /// Ref of the supporting governance packet.
    pub governance_packet_ref: String,
    /// Ref of the supporting assurance audit.
    pub assurance_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WaiverGateControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WaiverGateControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5WaiverGateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WaiverGateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WaiverGateReview,
    /// Consumer projection block.
    pub consumer_projection: M5WaiverGateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WaiverGateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WaiverGateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 waiver/gate/mitigation controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WaiverGateControlsPacket {
    /// Record kind; must equal [`M5_WAIVER_GATE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WAIVER_GATE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5WaiverGateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WaiverGateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WaiverGateReview,
    /// Consumer projection block.
    pub consumer_projection: M5WaiverGateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WaiverGateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WaiverGateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WaiverGateControlsPacket {
    /// Builds an M5 waiver/gate/mitigation controls packet from stable-lane input.
    pub fn new(input: M5WaiverGateControlsPacketInput) -> Self {
        Self {
            record_kind: M5_WAIVER_GATE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_WAIVER_GATE_CONTROLS_SCHEMA_VERSION,
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

    /// Validates the M5 waiver/gate/mitigation controls invariants.
    pub fn validate(&self) -> Vec<M5WaiverGateControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WAIVER_GATE_CONTROLS_RECORD_KIND {
            violations.push(M5WaiverGateControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WAIVER_GATE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5WaiverGateControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WaiverGateControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_controls_rows(self, &mut violations);
        validate_waived_never_clean_pass_proven(self, &mut violations);
        validate_mitigation_understandable_proven(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 waiver/gate controls packet serializes"),
        ) {
            violations.push(M5WaiverGateControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 waiver/gate controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governance consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,readiness_states,waiver_expiry_states,gate_decisions,mitigation_clarities,item_actions,gate_actions,export_fields,waiver_example_count,gate_example_count\n",
        );
        for row in &self.controls_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.readiness_states, |v| v.as_str()),
                join_tokens(&row.waiver_expiry_states, |v| v.as_str()),
                join_tokens(&row.gate_decisions, |v| v.as_str()),
                join_tokens(&row.mitigation_clarities, |v| v.as_str()),
                join_tokens(&row.item_actions, |v| v.as_str()),
                join_tokens(&row.gate_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.waiver_expiry_examples.len(),
                row.release_gate_examples.len(),
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
        out.push_str(
            "# M5 Waiver-Expiry Queue Item, Release-Gate Banner, and Mitigation Note Card Controls\n\n",
        );
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
            "- Waiver-expiry states: {}\n",
            self.vocabulary_set.waiver_expiry_states.join(", ")
        ));
        out.push_str(&format!(
            "- Mitigation clarities: {}\n",
            self.vocabulary_set.mitigation_clarities.join(", ")
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
                "  - Worked waiver-expiry items: {}\n",
                row.waiver_expiry_examples.len()
            ));
            for case in &row.waiver_expiry_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (waiver `{}`, mitigation `{}`, expiry-visible `{}`)\n",
                    case.resolved.waiver_id_repr,
                    case.resolved.readiness_state.as_str(),
                    case.resolved.waiver_state.as_str(),
                    case.resolved.mitigation_posture.as_str(),
                    case.resolved.expiry_visible,
                ));
            }
            out.push_str(&format!(
                "  - Worked release gates: {}\n",
                row.release_gate_examples.len()
            ));
            for case in &row.release_gate_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (decision `{}`, blockers `{}`, waived `{}`, stale `{}`, mitigation `{}`)\n",
                    case.resolved.gate_id_repr,
                    case.resolved.readiness_state.as_str(),
                    case.resolved.resolved_decision.as_str(),
                    case.resolved.blocker_count,
                    case.resolved.waived_count,
                    case.resolved.stale_evidence_count,
                    case.resolved.mitigation_clarity.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 waiver/gate controls export.
#[derive(Debug)]
pub enum M5WaiverGateControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WaiverGateControlsViolation>),
}

impl fmt::Display for M5WaiverGateControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 waiver/gate controls export parse failed: {error}"
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
                    "m5 waiver/gate controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WaiverGateControlsArtifactError {}

/// Validation failures emitted by [`M5WaiverGateControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WaiverGateControlsViolation {
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
    /// A controls row omits one of the mandatory queue-item actions.
    MandatoryItemActionMissing,
    /// A controls row omits one of the mandatory gate actions.
    MandatoryGateActionMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A controls row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A controls row declares no waiver-expiry worked cases.
    WaiverExampleMissing,
    /// A controls row declares no release-gate worked cases.
    GateExampleMissing,
    /// A worked waiver-expiry case does not match a fresh resolve of its input.
    WaiverExampleDrift,
    /// A worked release-gate case does not match a fresh resolve of its input.
    GateExampleDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked waiver case proves a waived or expiring failure that never reads as a
    /// clean pass with its expiry still visible (the AC-1 example).
    WaivedNeverCleanPassUnproven,
    /// No worked gate case proves a mitigation rendered in plain language understandable
    /// to users, support, and release reviewers (the AC-2 example).
    MitigationUnderstandableUnproven,
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

impl M5WaiverGateControlsViolation {
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
            Self::MandatoryItemActionMissing => "mandatory_item_action_missing",
            Self::MandatoryGateActionMissing => "mandatory_gate_action_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::WaiverExampleMissing => "waiver_example_missing",
            Self::GateExampleMissing => "gate_example_missing",
            Self::WaiverExampleDrift => "waiver_example_drift",
            Self::GateExampleDrift => "gate_example_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::WaivedNeverCleanPassUnproven => "waived_never_clean_pass_unproven",
            Self::MitigationUnderstandableUnproven => "mitigation_understandable_unproven",
            Self::ControlsInvariantViolated => "controls_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 waiver/gate controls export.
pub fn current_stable_m5_waiver_gate_controls_export(
) -> Result<M5WaiverGateControlsPacket, M5WaiverGateControlsArtifactError> {
    let packet: M5WaiverGateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-waiver-gate-mitigation-controls-proof/support_export.json"
    )))
    .map_err(M5WaiverGateControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WaiverGateControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WAIVER_GATE_CONTROLS_SCHEMA_REF,
        M5_WAIVER_GATE_CONTROLS_DOC_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF,
        M5_WAIVER_EXPIRY_QUEUE_ITEM_CONTRACT_REF,
        M5_RELEASE_GATE_BANNER_CONTRACT_REF,
        M5_MITIGATION_NOTE_CARD_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5WaiverGateControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5WaiverGateControlsViolation::VocabularySetDrift);
    }
}

fn validate_controls_rows(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    let present: BTreeSet<M5WaiverGateConsumerSurface> = packet
        .controls_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5WaiverGateConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5WaiverGateControlsViolation::RequiredConsumerMissing);
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
            || row.waiver_expiry_states.is_empty()
            || row.affected_target_kinds.is_empty()
            || row.mitigation_postures.is_empty()
            || row.mitigation_clarities.is_empty()
            || row.evidence_freshness_states.is_empty()
            || row.waiver_degrade_reasons.is_empty()
            || row.gate_decisions.is_empty()
            || row.gate_degrade_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5WaiverGateControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5WaiverGateControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5WaiverGateControlsViolation::MandatoryLabelMissing);
        }
        if !row.declares_mandatory_item_actions() {
            violations.push(M5WaiverGateControlsViolation::MandatoryItemActionMissing);
        }
        if !row.declares_mandatory_gate_actions() {
            violations.push(M5WaiverGateControlsViolation::MandatoryGateActionMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5WaiverGateControlsViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5WaiverGateControlsViolation::AccessibilityRouteMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5WaiverGateControlsViolation::DowngradeTriggersMissing);
        }
        if row.waiver_expiry_examples.is_empty() {
            violations.push(M5WaiverGateControlsViolation::WaiverExampleMissing);
        }
        if row.release_gate_examples.is_empty() {
            violations.push(M5WaiverGateControlsViolation::GateExampleMissing);
        }
        if row
            .waiver_expiry_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5WaiverGateControlsViolation::WaiverExampleDrift);
        }
        if row
            .release_gate_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5WaiverGateControlsViolation::GateExampleDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5WaiverGateControlsViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5WaiverGateControlsViolation::ControlsInvariantViolated);
        }
    }
}

/// At least one worked waiver case across the matrix must prove a waived or expiring
/// failure — an active or expiring waiver holding a failure — that never reads as a
/// clean pass while its expiry stays visible. This is the AC-1 example that a waived
/// failure never renders as a clean pass and an expiring waiver remains visible.
fn validate_waived_never_clean_pass_proven(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    let proven = packet.controls_rows.iter().any(|row| {
        row.waiver_expiry_examples.iter().any(|case| {
            matches!(
                case.resolved.waiver_state,
                M5WaiverExpiryState::ActiveWaiver | M5WaiverExpiryState::ExpiringSoon
            ) && !case.resolved.is_clean_pass
                && case.resolved.readiness_state == M5GovernanceReadinessState::Waived
                && case.resolved.expiry_visible
        })
    });
    if !proven {
        violations.push(M5WaiverGateControlsViolation::WaivedNeverCleanPassUnproven);
    }
}

/// At least one worked gate case across the matrix must prove a mitigation whose posture
/// requires mitigation and whose user-facing note reads in plain language a user,
/// support, or release reviewer can act on. This is the AC-2 example that mitigation
/// notes stay understandable without collapsing into internal-only jargon.
fn validate_mitigation_understandable_proven(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    let proven = packet.controls_rows.iter().any(|row| {
        row.release_gate_examples.iter().any(|case| {
            matches!(
                case.resolved.mitigation_posture,
                M5MitigationPosture::Mitigated
                    | M5MitigationPosture::PartiallyMitigated
                    | M5MitigationPosture::RiskAccepted
            ) && case.resolved.mitigation_clarity == M5MitigationClarity::PlainLanguage
                && case.resolved.mitigation_understandable
        })
    });
    if !proven {
        violations.push(M5WaiverGateControlsViolation::MitigationUnderstandableUnproven);
    }
}

fn validate_governance_review(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_packet_carries_waiver_gate_and_mitigation_truth,
        review.identity_and_gate_decision_always_shown,
        review.waived_or_expiring_never_reads_clean_pass,
        review.waiver_expiry_always_visible,
        review.ownerless_or_forumless_blocker_never_resolved,
        review.blocker_waived_stale_counts_always_shown,
        review.mitigation_stays_understandable,
        review.readiness_state_drawn_from_frozen_vocabulary,
        review.support_export_reconstructs_truth,
        review.no_surface_invents_second_grammar,
        review.every_row_declares_accessibility_route,
        review.owner_alias_is_role_not_person,
    ] {
        if !ok {
            violations.push(M5WaiverGateControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.surfaces_consume_shared_packet,
        projection.readiness_resolver_reads_single_source,
        projection.mitigation_clarity_reads_single_source,
        projection.waiver_expiry_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5WaiverGateControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5WaiverGateControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WaiverGateControlsPacket,
    violations: &mut Vec<M5WaiverGateControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.governance_packet_ref.trim().is_empty()
        || posture.assurance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5WaiverGateControlsViolation::ReleasePostureIncomplete);
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
