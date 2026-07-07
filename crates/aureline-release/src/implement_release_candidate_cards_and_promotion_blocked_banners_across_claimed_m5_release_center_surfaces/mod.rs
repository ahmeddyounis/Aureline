//! One reusable M5 release-candidate-card / promotion-blocked-banner primitive:
//! candidate identity, channel family, scoped artifact set, blocker summary,
//! evidence freshness, known issues, and rollback-path truth, projected the same
//! way across every claimed M5 release-center surface.
//!
//! Aureline's frozen release-center component matrix
//! ([`crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix`])
//! names the release candidate card as one governed component family and freezes
//! its controlled vocabulary — the candidate scope classes, the candidate blocker
//! states, the rollback blast radii, the publication surface families, the
//! deployment lines, the accessibility routes, the qualification classes, and the
//! downgrade triggers. This module *implements* that release-candidate-card
//! contract as one reusable primitive so a user can tell — from the card and its
//! blocked-state banner alone — what candidate is under review, what artifacts it
//! covers, what evidence is stale, what is known to be broken, and how rollback
//! would work, before promotion, instead of that truth drifting by pipeline page
//! or admin log.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_release_candidate`] — that takes one candidate's
//!    label, version, channel family, scope class, scoped artifact set, declared
//!    blocker state, evidence freshness, known issues, and rollback target /
//!    blast radius, and produces one [`M5ResolvedReleaseCandidate`] carrying the
//!    derived promotability posture (promotable versus promotable-with-reservations
//!    versus narrowed versus blocked), the rollback-path readiness, and — whenever
//!    the candidate is blocked or narrowed — a self-contained
//!    [`M5PromotionBlockedBanner`] that names the exact stale-or-missing evidence
//!    reason, the blocked scope, and the next action rather than a generic
//!    `cannot promote`. The resolver never infers candidate scope or a rollback
//!    target from the semantic version alone, and never shows stale or missing
//!    evidence as clear.
//! 2. A parity matrix — [`M5ReleaseCandidatePrimitivePacket`] — that binds one row
//!    per claimed M5 release-candidate consumer (the release-center card, the
//!    update-center card, the CLI release inspect, the admin release report, and
//!    the support / evaluation export) to the shared card anatomy, the same
//!    promotability postures, blocker states, evidence-freshness states, known-issue
//!    classes, rollback readinesses, block reasons, and next actions, the same
//!    export fields, and the same non-visual accessibility routes, so the
//!    candidate/blocker vocabulary stays identical across the release center, the
//!    CLI, admin/reporting, and support/evaluation.
//!
//! The candidate scope class ([`M5CandidateScopeClass`]), candidate blocker state
//! ([`M5CandidateBlockerState`]), rollback blast radius ([`M5RollbackBlastRadius`]),
//! publication surface family ([`M5PublicationSurfaceFamily`]), deployment line
//! ([`M5DeploymentLine`]), release-center consumer surface
//! ([`M5ReleaseCenterConsumerSurface`]), accessibility route
//! ([`M5ReleaseCenterAccessibilityRoute`]), qualification class
//! ([`M5ReleaseCenterQualificationClass`]), and downgrade trigger
//! ([`M5ReleaseCenterDowngradeTrigger`]) are reused verbatim from the frozen
//! release-center component matrix. This module mints new vocabulary only for what
//! that matrix left implicit about the candidate card itself: its release-candidate
//! consumer families, its card anatomy parts, its channel families, its
//! evidence-freshness states, its known-issue classes, its promotability postures,
//! its rollback-path readinesses, its promotion-block reasons, its next actions,
//! and its export fields. No M5 release surface invents a second candidate grammar.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user
//! text bodies stay outside the support boundary; every candidate label, version,
//! artifact id, and rollback target is carried only as an opaque, export-safe
//! representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-release-candidate-card.schema.json`](../../../../schemas/ui/m5-release-candidate-card.schema.json)
//! and the contract doc is
//! [`docs/release/m5_release_candidate_card_primitive_contract.md`](../../../../docs/release/m5_release_candidate_card_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-release-candidate-card-primitive/`](../../../../fixtures/ui/m5-release-candidate-card-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_release_candidate_primitive_cli_release_inspect_preview_narrowed,
    seeded_m5_release_candidate_primitive_packet,
    seeded_m5_release_candidate_primitive_update_center_card_beta_narrowed,
    M5_RELEASE_CANDIDATE_PRIMITIVE_PACKET_ID,
};

// The candidate scope class, candidate blocker state, rollback blast radius,
// publication surface family, deployment line, release-center consumer surface,
// accessibility routes, qualification classes, and downgrade triggers are frozen
// once, in the release-center component matrix. This primitive reuses them
// verbatim so it never invents a parallel candidate vocabulary.
pub use crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix::{
    M5CandidateBlockerState, M5CandidateScopeClass, M5DeploymentLine, M5PublicationSurfaceFamily,
    M5ReleaseCenterAccessibilityRoute, M5ReleaseCenterConsumerSurface,
    M5ReleaseCenterDowngradeTrigger, M5ReleaseCenterQualificationClass, M5RollbackBlastRadius,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ReleaseCandidatePrimitivePacket`].
pub const M5_RELEASE_CANDIDATE_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces";

/// Schema version for M5 release-candidate-primitive records.
pub const M5_RELEASE_CANDIDATE_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the release-candidate-card boundary schema.
pub const M5_RELEASE_CANDIDATE_SCHEMA_REF: &str =
    "schemas/ui/m5-release-candidate-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RELEASE_CANDIDATE_DOC_REF: &str =
    "docs/release/m5_release_candidate_card_primitive_contract.md";

/// Repo-relative path of the frozen release-center component matrix this primitive
/// narrows from.
pub const M5_RELEASE_CANDIDATE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-release-center-components.schema.json";

/// Repo-relative path of the release-center object-model contract this primitive
/// binds against.
pub const M5_RELEASE_CANDIDATE_OBJECT_MODEL_REF: &str =
    "docs/release/release_center_object_model_contract.md";

/// Repo-relative path of the update-and-rollback contract this primitive projects
/// rollback-path truth from.
pub const M5_RELEASE_CANDIDATE_ROLLBACK_CONTRACT_REF: &str =
    "docs/release/update_and_rollback_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RELEASE_CANDIDATE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-release-candidate-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RELEASE_CANDIDATE_ARTIFACT_REF: &str =
    "artifacts/release/m5-release-candidate-card-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_RELEASE_CANDIDATE_CSV_REF: &str =
    "artifacts/release/m5-release-candidate-card-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_RELEASE_CANDIDATE_REPORT_REF: &str =
    "artifacts/components/m5-release-candidate-card-primitive.md";

/// One claimed M5 release-candidate consumer that renders the shared candidate
/// card and its blocked-state banner. These are the consumers the acceptance
/// criteria name — the release center, the CLI, admin/reporting, and
/// support/evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseCandidateConsumerSurface {
    /// The release-center / shiproom candidate card.
    ReleaseCenterCard,
    /// The update-center candidate card.
    UpdateCenterCard,
    /// The CLI release-inspect / headless surface.
    CliReleaseInspect,
    /// The admin release report.
    AdminReleaseReport,
    /// The support / evaluation export.
    SupportEvaluationExport,
}

impl M5ReleaseCandidateConsumerSurface {
    /// Every claimed release-candidate consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReleaseCenterCard,
        Self::UpdateCenterCard,
        Self::CliReleaseInspect,
        Self::AdminReleaseReport,
        Self::SupportEvaluationExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterCard => "release_center_card",
            Self::UpdateCenterCard => "update_center_card",
            Self::CliReleaseInspect => "cli_release_inspect",
            Self::AdminReleaseReport => "admin_release_report",
            Self::SupportEvaluationExport => "support_evaluation_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenterCard => "Release-Center Card",
            Self::UpdateCenterCard => "Update-Center Card",
            Self::CliReleaseInspect => "CLI Release Inspect",
            Self::AdminReleaseReport => "Admin Release Report",
            Self::SupportEvaluationExport => "Support / Evaluation Export",
        }
    }
}

/// One anatomy part the shared candidate card / blocked-state banner surfaces. The
/// parts in [`M5CandidateCardAnatomyPart::MANDATORY`] are required on every card so
/// a user can orient before promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CandidateCardAnatomyPart {
    /// The candidate identity: version and label.
    CandidateIdentity,
    /// The channel-family badge.
    ChannelFamilyBadge,
    /// The scoped artifact set.
    ScopedArtifactSet,
    /// The blocker summary.
    BlockerSummary,
    /// The evidence-freshness cue.
    EvidenceFreshnessCue,
    /// The known-issues list.
    KnownIssuesList,
    /// The rollback-target cue.
    RollbackTargetCue,
    /// The derived promotability verdict.
    PromotabilityVerdict,
    /// The promotion-blocked banner (shown when blocked or narrowed).
    PromotionBlockedBanner,
}

impl M5CandidateCardAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::CandidateIdentity,
        Self::ChannelFamilyBadge,
        Self::ScopedArtifactSet,
        Self::BlockerSummary,
        Self::EvidenceFreshnessCue,
        Self::KnownIssuesList,
        Self::RollbackTargetCue,
        Self::PromotabilityVerdict,
        Self::PromotionBlockedBanner,
    ];

    /// The anatomy parts every candidate card must render before promotion.
    pub const MANDATORY: [Self; 4] = [
        Self::CandidateIdentity,
        Self::ChannelFamilyBadge,
        Self::ScopedArtifactSet,
        Self::PromotabilityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CandidateIdentity => "candidate_identity",
            Self::ChannelFamilyBadge => "channel_family_badge",
            Self::ScopedArtifactSet => "scoped_artifact_set",
            Self::BlockerSummary => "blocker_summary",
            Self::EvidenceFreshnessCue => "evidence_freshness_cue",
            Self::KnownIssuesList => "known_issues_list",
            Self::RollbackTargetCue => "rollback_target_cue",
            Self::PromotabilityVerdict => "promotability_verdict",
            Self::PromotionBlockedBanner => "promotion_blocked_banner",
        }
    }
}

/// Controlled channel family a candidate targets, so a candidate card never leaves
/// its channel implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CandidateChannelFamily {
    /// The stable channel.
    StableChannel,
    /// The beta channel.
    BetaChannel,
    /// The preview / experimental channel.
    PreviewChannel,
    /// The long-term-support maintenance channel.
    LtsMaintenanceChannel,
    /// The nightly channel.
    NightlyChannel,
}

impl M5CandidateChannelFamily {
    /// Every channel family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StableChannel,
        Self::BetaChannel,
        Self::PreviewChannel,
        Self::LtsMaintenanceChannel,
        Self::NightlyChannel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableChannel => "stable_channel",
            Self::BetaChannel => "beta_channel",
            Self::PreviewChannel => "preview_channel",
            Self::LtsMaintenanceChannel => "lts_maintenance_channel",
            Self::NightlyChannel => "nightly_channel",
        }
    }

    /// True when this channel is a first-emit channel with no prior stable release
    /// to roll back to.
    const fn has_no_rollback_predecessor(self) -> bool {
        matches!(self, Self::PreviewChannel | Self::NightlyChannel)
    }
}

/// Controlled evidence-freshness state behind a candidate's promotion claim, so a
/// candidate card never shows stale or missing evidence as clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshnessState {
    /// Evidence is fresh within its freshness window.
    EvidenceFresh,
    /// Evidence is aging but still within tolerance.
    EvidenceAging,
    /// Evidence is stale relative to the candidate build.
    EvidenceStale,
    /// Required evidence is missing.
    EvidenceMissing,
    /// The evidence-freshness reading is unknown / not yet evaluated.
    EvidenceFreshnessUnknown,
}

impl M5EvidenceFreshnessState {
    /// Every evidence-freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EvidenceFresh,
        Self::EvidenceAging,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::EvidenceFreshnessUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceFresh => "evidence_fresh",
            Self::EvidenceAging => "evidence_aging",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::EvidenceFreshnessUnknown => "evidence_freshness_unknown",
        }
    }
}

/// Controlled known-issue class carried on a candidate card, so a candidate card
/// never hides a functional, performance, security, or data-affecting known issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KnownIssueClass {
    /// A cosmetic-only known issue.
    CosmeticKnownIssue,
    /// A functional known issue.
    FunctionalKnownIssue,
    /// A performance known issue.
    PerformanceKnownIssue,
    /// A security known issue.
    SecurityKnownIssue,
    /// A data-affecting known issue.
    DataAffectingKnownIssue,
}

impl M5KnownIssueClass {
    /// Every known-issue class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CosmeticKnownIssue,
        Self::FunctionalKnownIssue,
        Self::PerformanceKnownIssue,
        Self::SecurityKnownIssue,
        Self::DataAffectingKnownIssue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CosmeticKnownIssue => "cosmetic_known_issue",
            Self::FunctionalKnownIssue => "functional_known_issue",
            Self::PerformanceKnownIssue => "performance_known_issue",
            Self::SecurityKnownIssue => "security_known_issue",
            Self::DataAffectingKnownIssue => "data_affecting_known_issue",
        }
    }
}

/// The derived rollback-path readiness of a candidate, so a candidate card never
/// leaves its rollback target to be inferred from the semantic version alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackPathReadiness {
    /// A rollback target is explicitly pinned.
    RollbackTargetPinned,
    /// A first-emit channel with no prior release to roll back to.
    NoPriorToRollBackTo,
    /// The rollback target is undefined and must be defined before promotion.
    RollbackTargetUndefined,
}

impl M5RollbackPathReadiness {
    /// Every rollback-path readiness, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RollbackTargetPinned,
        Self::NoPriorToRollBackTo,
        Self::RollbackTargetUndefined,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollbackTargetPinned => "rollback_target_pinned",
            Self::NoPriorToRollBackTo => "no_prior_to_roll_back_to",
            Self::RollbackTargetUndefined => "rollback_target_undefined",
        }
    }
}

/// The derived headline promotability posture of a candidate — the resolver's
/// verdict about whether the candidate is promotable, promotable with disclosed
/// reservations, narrowed, or blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CandidatePromotability {
    /// Promotable: no blockers, fresh evidence, rollback path ready.
    Promotable,
    /// Promotable with disclosed reservations (soft blockers or aging evidence).
    PromotableWithReservations,
    /// Promotable under a disclosed waiver.
    PromotableUnderWaiver,
    /// Narrowed: a blocker was resolved but is pending re-verification.
    NarrowedPendingReverify,
    /// Narrowed: the rollback target is undefined.
    NarrowedRollbackUndefined,
    /// Blocked: a hard, promotion-gating blocker is open.
    BlockedHardBlocker,
    /// Blocked: required evidence is stale.
    BlockedStaleEvidence,
    /// Blocked: required evidence is missing.
    BlockedMissingEvidence,
    /// Blocked: the candidate state is unknown / not yet evaluated.
    BlockedUnknownState,
}

impl M5CandidatePromotability {
    /// Every promotability posture, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Promotable,
        Self::PromotableWithReservations,
        Self::PromotableUnderWaiver,
        Self::NarrowedPendingReverify,
        Self::NarrowedRollbackUndefined,
        Self::BlockedHardBlocker,
        Self::BlockedStaleEvidence,
        Self::BlockedMissingEvidence,
        Self::BlockedUnknownState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promotable => "promotable",
            Self::PromotableWithReservations => "promotable_with_reservations",
            Self::PromotableUnderWaiver => "promotable_under_waiver",
            Self::NarrowedPendingReverify => "narrowed_pending_reverify",
            Self::NarrowedRollbackUndefined => "narrowed_rollback_undefined",
            Self::BlockedHardBlocker => "blocked_hard_blocker",
            Self::BlockedStaleEvidence => "blocked_stale_evidence",
            Self::BlockedMissingEvidence => "blocked_missing_evidence",
            Self::BlockedUnknownState => "blocked_unknown_state",
        }
    }

    /// True when the candidate may be promoted (possibly with disclosed
    /// reservations or a waiver).
    pub const fn is_promotable(self) -> bool {
        matches!(
            self,
            Self::Promotable | Self::PromotableWithReservations | Self::PromotableUnderWaiver
        )
    }

    /// True when the candidate is hard-blocked from promotion.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedHardBlocker
                | Self::BlockedStaleEvidence
                | Self::BlockedMissingEvidence
                | Self::BlockedUnknownState
        )
    }

    /// True when the candidate is narrowed below a clean promotable claim.
    pub const fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::NarrowedPendingReverify | Self::NarrowedRollbackUndefined
        )
    }

    /// The specific promotion-block reason for a blocked or narrowed posture, if
    /// any. Returns `None` for a promotable posture.
    pub const fn block_reason(self) -> Option<M5PromotionBlockReason> {
        Some(match self {
            Self::BlockedHardBlocker => M5PromotionBlockReason::HardBlockerOpen,
            Self::BlockedStaleEvidence => M5PromotionBlockReason::EvidenceStale,
            Self::BlockedMissingEvidence => M5PromotionBlockReason::EvidenceMissing,
            Self::BlockedUnknownState => M5PromotionBlockReason::CandidateStateUnknown,
            Self::NarrowedPendingReverify => M5PromotionBlockReason::BlockerPendingReverify,
            Self::NarrowedRollbackUndefined => M5PromotionBlockReason::RollbackTargetUndefined,
            Self::Promotable | Self::PromotableWithReservations | Self::PromotableUnderWaiver => {
                return None
            }
        })
    }
}

/// The exact reason a candidate's promotion is blocked or narrowed, so a
/// promotion-blocked banner never reads like a generic `cannot promote`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromotionBlockReason {
    /// A hard, promotion-gating blocker is open.
    HardBlockerOpen,
    /// Required evidence is stale relative to the candidate build.
    EvidenceStale,
    /// Required evidence is missing.
    EvidenceMissing,
    /// The candidate state is unknown / not yet evaluated.
    CandidateStateUnknown,
    /// A blocker was resolved but is pending re-verification.
    BlockerPendingReverify,
    /// The rollback target is undefined.
    RollbackTargetUndefined,
}

impl M5PromotionBlockReason {
    /// Every block reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HardBlockerOpen,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::CandidateStateUnknown,
        Self::BlockerPendingReverify,
        Self::RollbackTargetUndefined,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HardBlockerOpen => "hard_blocker_open",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::CandidateStateUnknown => "candidate_state_unknown",
            Self::BlockerPendingReverify => "blocker_pending_reverify",
            Self::RollbackTargetUndefined => "rollback_target_undefined",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::HardBlockerOpen => "a hard promotion-gating blocker is open",
            Self::EvidenceStale => "required evidence is stale",
            Self::EvidenceMissing => "required evidence is missing",
            Self::CandidateStateUnknown => "the candidate state is not yet evaluated",
            Self::BlockerPendingReverify => "a resolved blocker is pending re-verification",
            Self::RollbackTargetUndefined => "the rollback target is undefined",
        }
    }

    /// The next action a reviewer should take to clear this reason.
    pub const fn next_action(self) -> M5PromotionNextAction {
        match self {
            Self::HardBlockerOpen => M5PromotionNextAction::ResolveHardBlocker,
            Self::EvidenceStale => M5PromotionNextAction::RefreshEvidence,
            Self::EvidenceMissing => M5PromotionNextAction::ProvideEvidence,
            Self::CandidateStateUnknown => M5PromotionNextAction::RunEvaluation,
            Self::BlockerPendingReverify => M5PromotionNextAction::ReverifyBlocker,
            Self::RollbackTargetUndefined => M5PromotionNextAction::DefineRollbackTarget,
        }
    }
}

/// The next action named on a promotion-blocked banner, so a blocked state is
/// actionable from the banner itself rather than from a secondary pipeline page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromotionNextAction {
    /// Resolve the open hard blocker.
    ResolveHardBlocker,
    /// Refresh the stale evidence.
    RefreshEvidence,
    /// Provide the missing evidence.
    ProvideEvidence,
    /// Run the candidate evaluation.
    RunEvaluation,
    /// Re-verify the resolved blocker.
    ReverifyBlocker,
    /// Define the rollback target.
    DefineRollbackTarget,
}

impl M5PromotionNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResolveHardBlocker,
        Self::RefreshEvidence,
        Self::ProvideEvidence,
        Self::RunEvaluation,
        Self::ReverifyBlocker,
        Self::DefineRollbackTarget,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveHardBlocker => "resolve_hard_blocker",
            Self::RefreshEvidence => "refresh_evidence",
            Self::ProvideEvidence => "provide_evidence",
            Self::RunEvaluation => "run_evaluation",
            Self::ReverifyBlocker => "reverify_blocker",
            Self::DefineRollbackTarget => "define_rollback_target",
        }
    }
}

/// A field the support / export packet carries so candidate and blocker truth is
/// reconstructable from the shared card model. The fields in
/// [`M5CandidateCardExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CandidateCardExportField {
    /// The opaque candidate version representation.
    CandidateVersion,
    /// The channel family.
    ChannelFamily,
    /// The candidate scope class.
    ScopeClass,
    /// The scoped artifact set.
    ArtifactSet,
    /// The candidate blocker state.
    BlockerState,
    /// The evidence-freshness state.
    EvidenceFreshness,
    /// The known-issue classes.
    KnownIssues,
    /// The rollback target.
    RollbackTarget,
    /// The derived promotability posture.
    Promotability,
    /// The promotion-block reason (when blocked or narrowed).
    BlockReason,
}

impl M5CandidateCardExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::CandidateVersion,
        Self::ChannelFamily,
        Self::ScopeClass,
        Self::ArtifactSet,
        Self::BlockerState,
        Self::EvidenceFreshness,
        Self::KnownIssues,
        Self::RollbackTarget,
        Self::Promotability,
        Self::BlockReason,
    ];

    /// The export fields every candidate-card export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::CandidateVersion,
        Self::ScopeClass,
        Self::EvidenceFreshness,
        Self::RollbackTarget,
        Self::Promotability,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CandidateVersion => "candidate_version",
            Self::ChannelFamily => "channel_family",
            Self::ScopeClass => "scope_class",
            Self::ArtifactSet => "artifact_set",
            Self::BlockerState => "blocker_state",
            Self::EvidenceFreshness => "evidence_freshness",
            Self::KnownIssues => "known_issues",
            Self::RollbackTarget => "rollback_target",
            Self::Promotability => "promotability",
            Self::BlockReason => "block_reason",
        }
    }
}

/// A self-contained promotion-blocked banner: the exact reason, the blocked scope,
/// and the next action, so a blocked promotion state is understood from the banner
/// alone rather than from secondary logs or internal pipeline pages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromotionBlockedBanner {
    /// The exact block reason.
    pub reason: M5PromotionBlockReason,
    /// The next action a reviewer should take.
    pub next_action: M5PromotionNextAction,
    /// The candidate scope class the block applies to.
    pub blocked_scope_class: M5CandidateScopeClass,
    /// The scoped artifact set the block applies to.
    pub blocked_artifact_set: Vec<String>,
    /// The rollback blast radius that a rollback would reach.
    pub blast_radius: M5RollbackBlastRadius,
    /// A deterministic, self-contained headline naming the reason, the scope, and
    /// the next action — never a generic `cannot promote`.
    pub headline: String,
}

/// The full input to the release-candidate resolver for one candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCandidateResolutionInput {
    /// The opaque, export-safe candidate label.
    pub candidate_label: String,
    /// The opaque, export-safe candidate version.
    pub version_repr: String,
    /// The channel family the candidate targets.
    pub channel_family: M5CandidateChannelFamily,
    /// The candidate scope class (never inferred from the version).
    pub scope_class: M5CandidateScopeClass,
    /// The scoped artifact set. Must be non-empty so scope is explicit.
    pub artifact_set: Vec<String>,
    /// The declared blocker state.
    pub blocker_state: M5CandidateBlockerState,
    /// The evidence-freshness state.
    pub evidence_freshness: M5EvidenceFreshnessState,
    /// The known-issue classes, if any.
    pub known_issue_classes: Vec<M5KnownIssueClass>,
    /// The opaque rollback-target representation, when one is pinned.
    pub rollback_target_repr: Option<String>,
    /// The rollback blast radius a rollback would reach.
    pub rollback_blast_radius: M5RollbackBlastRadius,
}

/// The resolved candidate / blocker / rollback truth for one release candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedReleaseCandidate {
    /// The opaque candidate label.
    pub candidate_label: String,
    /// The opaque candidate version.
    pub version_repr: String,
    /// The channel family.
    pub channel_family: M5CandidateChannelFamily,
    /// The candidate scope class.
    pub scope_class: M5CandidateScopeClass,
    /// The scoped artifact set.
    pub artifact_set: Vec<String>,
    /// The count of artifacts in scope.
    pub artifact_count: usize,
    /// The declared blocker state.
    pub blocker_state: M5CandidateBlockerState,
    /// The evidence-freshness state.
    pub evidence_freshness: M5EvidenceFreshnessState,
    /// The known-issue classes.
    pub known_issue_classes: Vec<M5KnownIssueClass>,
    /// True when the candidate carries at least one known issue.
    pub has_known_issues: bool,
    /// The opaque rollback-target representation, when pinned.
    pub rollback_target_repr: Option<String>,
    /// The rollback blast radius.
    pub rollback_blast_radius: M5RollbackBlastRadius,
    /// The derived rollback-path readiness.
    pub rollback_path_readiness: M5RollbackPathReadiness,
    /// The derived promotability posture.
    pub promotability: M5CandidatePromotability,
    /// True when the candidate is promotable.
    pub is_promotable: bool,
    /// True when the candidate is hard-blocked.
    pub is_blocked: bool,
    /// True when the candidate is narrowed.
    pub is_narrowed: bool,
    /// The promotion-blocked banner, present when blocked or narrowed.
    pub promotion_banner: Option<M5PromotionBlockedBanner>,
}

/// Errors returned by [`resolve_release_candidate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ReleaseCandidateResolutionError {
    /// The candidate label was empty.
    EmptyCandidateLabel,
    /// The candidate version was empty.
    EmptyVersion,
    /// The scoped artifact set was empty (scope must be explicit).
    EmptyArtifactSet,
    /// The rollback target equals the candidate version (cannot roll back to self).
    RollbackTargetEqualsCandidate,
    /// A candidate label, version, artifact id, or rollback target carried
    /// forbidden material.
    ForbiddenCandidateMaterial,
}

impl M5ReleaseCandidateResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyCandidateLabel => "empty_candidate_label",
            Self::EmptyVersion => "empty_version",
            Self::EmptyArtifactSet => "empty_artifact_set",
            Self::RollbackTargetEqualsCandidate => "rollback_target_equals_candidate",
            Self::ForbiddenCandidateMaterial => "forbidden_candidate_material",
        }
    }
}

impl fmt::Display for M5ReleaseCandidateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "release-candidate resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ReleaseCandidateResolutionError {}

/// Resolves one release candidate from its declared state.
///
/// The derived promotability posture is the headline verdict, computed in a fixed
/// blocking-first order: an unknown blocker state or unknown evidence reading
/// blocks first, then missing evidence, then stale evidence, then an open hard
/// blocker, then a resolved-pending-reverify blocker narrows, then an undefined
/// rollback target narrows, then a disclosed waiver, then soft blockers or aging
/// evidence carry disclosed reservations, and only a candidate with no blockers,
/// fresh evidence, and a ready rollback path is cleanly promotable. Candidate scope
/// and rollback target are carried explicitly — never inferred from the version —
/// and a blocked or narrowed candidate always produces a self-contained banner.
pub fn resolve_release_candidate(
    input: &M5ReleaseCandidateResolutionInput,
) -> Result<M5ResolvedReleaseCandidate, M5ReleaseCandidateResolutionError> {
    if input.candidate_label.trim().is_empty() {
        return Err(M5ReleaseCandidateResolutionError::EmptyCandidateLabel);
    }
    if input.version_repr.trim().is_empty() {
        return Err(M5ReleaseCandidateResolutionError::EmptyVersion);
    }
    if input.artifact_set.is_empty() {
        return Err(M5ReleaseCandidateResolutionError::EmptyArtifactSet);
    }
    if value_repr_is_forbidden(&input.candidate_label)
        || value_repr_is_forbidden(&input.version_repr)
    {
        return Err(M5ReleaseCandidateResolutionError::ForbiddenCandidateMaterial);
    }
    for artifact in &input.artifact_set {
        if value_repr_is_forbidden(artifact) {
            return Err(M5ReleaseCandidateResolutionError::ForbiddenCandidateMaterial);
        }
    }
    if let Some(target) = &input.rollback_target_repr {
        if value_repr_is_forbidden(target) {
            return Err(M5ReleaseCandidateResolutionError::ForbiddenCandidateMaterial);
        }
        if target == &input.version_repr {
            return Err(M5ReleaseCandidateResolutionError::RollbackTargetEqualsCandidate);
        }
    }

    let rollback_path_readiness = if input.rollback_target_repr.is_some() {
        M5RollbackPathReadiness::RollbackTargetPinned
    } else if input.channel_family.has_no_rollback_predecessor() {
        M5RollbackPathReadiness::NoPriorToRollBackTo
    } else {
        M5RollbackPathReadiness::RollbackTargetUndefined
    };

    let promotability = derive_promotability(
        input.blocker_state,
        input.evidence_freshness,
        rollback_path_readiness,
    );

    let is_promotable = promotability.is_promotable();
    let is_blocked = promotability.is_blocked();
    let is_narrowed = promotability.is_narrowed();

    let promotion_banner = promotability.block_reason().map(|reason| {
        let next_action = reason.next_action();
        let headline = format!(
            "Promotion held: {} — {} artifact(s) in {} scope; next: {}",
            reason.phrase(),
            input.artifact_set.len(),
            input.scope_class.as_str(),
            next_action.as_str()
        );
        M5PromotionBlockedBanner {
            reason,
            next_action,
            blocked_scope_class: input.scope_class,
            blocked_artifact_set: input.artifact_set.clone(),
            blast_radius: input.rollback_blast_radius,
            headline,
        }
    });

    Ok(M5ResolvedReleaseCandidate {
        candidate_label: input.candidate_label.clone(),
        version_repr: input.version_repr.clone(),
        channel_family: input.channel_family,
        scope_class: input.scope_class,
        artifact_set: input.artifact_set.clone(),
        artifact_count: input.artifact_set.len(),
        blocker_state: input.blocker_state,
        evidence_freshness: input.evidence_freshness,
        known_issue_classes: input.known_issue_classes.clone(),
        has_known_issues: !input.known_issue_classes.is_empty(),
        rollback_target_repr: input.rollback_target_repr.clone(),
        rollback_blast_radius: input.rollback_blast_radius,
        rollback_path_readiness,
        promotability,
        is_promotable,
        is_blocked,
        is_narrowed,
        promotion_banner,
    })
}

/// The fixed blocking-first promotability ladder.
fn derive_promotability(
    blocker_state: M5CandidateBlockerState,
    evidence: M5EvidenceFreshnessState,
    rollback: M5RollbackPathReadiness,
) -> M5CandidatePromotability {
    let state_unknown = matches!(blocker_state, M5CandidateBlockerState::BlockerStateUnknown)
        || matches!(evidence, M5EvidenceFreshnessState::EvidenceFreshnessUnknown);
    if state_unknown {
        M5CandidatePromotability::BlockedUnknownState
    } else if matches!(evidence, M5EvidenceFreshnessState::EvidenceMissing) {
        M5CandidatePromotability::BlockedMissingEvidence
    } else if matches!(evidence, M5EvidenceFreshnessState::EvidenceStale) {
        M5CandidatePromotability::BlockedStaleEvidence
    } else if matches!(blocker_state, M5CandidateBlockerState::HardBlockerOpen) {
        M5CandidatePromotability::BlockedHardBlocker
    } else if matches!(
        blocker_state,
        M5CandidateBlockerState::BlockerResolvedPendingReverify
    ) {
        M5CandidatePromotability::NarrowedPendingReverify
    } else if matches!(rollback, M5RollbackPathReadiness::RollbackTargetUndefined) {
        M5CandidatePromotability::NarrowedRollbackUndefined
    } else if matches!(blocker_state, M5CandidateBlockerState::BlockerWaived) {
        M5CandidatePromotability::PromotableUnderWaiver
    } else if matches!(blocker_state, M5CandidateBlockerState::SoftBlockersOnly)
        || matches!(evidence, M5EvidenceFreshnessState::EvidenceAging)
    {
        M5CandidatePromotability::PromotableWithReservations
    } else {
        M5CandidatePromotability::Promotable
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs candidate and blocker truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCandidateResolutionCase {
    /// The resolver input.
    pub input: M5ReleaseCandidateResolutionInput,
    /// The resolved truth. Must equal `resolve_release_candidate(&input)`.
    pub resolved: M5ResolvedReleaseCandidate,
}

impl M5ReleaseCandidateResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ReleaseCandidateResolutionInput) -> Self {
        let resolved = resolve_release_candidate(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_release_candidate(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one release-candidate consumer bound to the
/// shared card anatomy, promotability postures, blocker states, evidence-freshness
/// states, known-issue classes, rollback readinesses, block reasons, next actions,
/// export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCandidateRow {
    /// Release-candidate consumer family.
    pub consumer_surface: M5ReleaseCandidateConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ReleaseCenterQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 publication surface families that render / consume this card.
    pub surface_families: Vec<M5PublicationSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this card renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5CandidateCardAnatomyPart>,
    /// Channel families this card names.
    pub channel_families: Vec<M5CandidateChannelFamily>,
    /// Candidate scope classes this card names.
    pub scope_classes: Vec<M5CandidateScopeClass>,
    /// Candidate blocker states this card distinguishes.
    pub blocker_states: Vec<M5CandidateBlockerState>,
    /// Evidence-freshness states this card distinguishes.
    pub evidence_freshness_states: Vec<M5EvidenceFreshnessState>,
    /// Known-issue classes this card discloses.
    pub known_issue_classes: Vec<M5KnownIssueClass>,
    /// Promotability postures this card distinguishes.
    pub promotability_postures: Vec<M5CandidatePromotability>,
    /// Rollback-path readinesses this card distinguishes.
    pub rollback_path_readinesses: Vec<M5RollbackPathReadiness>,
    /// Promotion-block reasons this card names.
    pub block_reasons: Vec<M5PromotionBlockReason>,
    /// Next actions this card names.
    pub next_actions: Vec<M5PromotionNextAction>,
    /// Rollback blast radii this card discloses.
    pub rollback_blast_radii: Vec<M5RollbackBlastRadius>,
    /// Export fields this card carries (must include the mandatory fields).
    pub export_fields: Vec<M5CandidateCardExportField>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5ReleaseCenterAccessibilityRoute>,
    /// Release-center subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5ReleaseCenterConsumerSurface>,
    /// Downgrade triggers that apply to this card.
    pub downgrade_triggers: Vec<M5ReleaseCenterDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5ReleaseCandidateResolutionCase>,
    /// Hard invariant: this card never infers candidate scope from the semantic
    /// version alone. MUST be `false`.
    pub infers_scope_from_semver_alone: bool,
    /// Hard invariant: this card never shows stale or missing evidence as clear.
    /// MUST be `false`.
    pub shows_stale_or_missing_evidence_as_clear: bool,
    /// Hard invariant: this card never emits a generic `cannot promote` banner.
    /// MUST be `false`.
    pub emits_generic_cannot_promote_banner: bool,
    /// Hard invariant: this card never overstates rollback reversibility. MUST be
    /// `false`.
    pub overstates_rollback_reversibility: bool,
}

impl M5ReleaseCandidateRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CandidateCardAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5CandidateCardAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CandidateCardExportField> =
            self.export_fields.iter().copied().collect();
        M5CandidateCardExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.infers_scope_from_semver_alone
            && !self.shows_stale_or_missing_evidence_as_clear
            && !self.emits_generic_cannot_promote_banner
            && !self.overstates_rollback_reversibility
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCandidateVocabularySet {
    /// Release-candidate consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Channel-family tokens.
    pub channel_families: Vec<String>,
    /// Evidence-freshness-state tokens.
    pub evidence_freshness_states: Vec<String>,
    /// Known-issue-class tokens.
    pub known_issue_classes: Vec<String>,
    /// Promotability-posture tokens.
    pub promotability_postures: Vec<String>,
    /// Rollback-path-readiness tokens.
    pub rollback_path_readinesses: Vec<String>,
    /// Promotion-block-reason tokens.
    pub block_reasons: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Candidate-scope-class tokens (reused from the frozen matrix).
    pub scope_classes: Vec<String>,
    /// Candidate-blocker-state tokens (reused from the frozen matrix).
    pub blocker_states: Vec<String>,
    /// Rollback-blast-radius tokens (reused from the frozen matrix).
    pub rollback_blast_radii: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ReleaseCandidateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5ReleaseCandidateConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5CandidateCardAnatomyPart::ALL, |v| v.as_str()),
            channel_families: tokens(&M5CandidateChannelFamily::ALL, |v| v.as_str()),
            evidence_freshness_states: tokens(&M5EvidenceFreshnessState::ALL, |v| v.as_str()),
            known_issue_classes: tokens(&M5KnownIssueClass::ALL, |v| v.as_str()),
            promotability_postures: tokens(&M5CandidatePromotability::ALL, |v| v.as_str()),
            rollback_path_readinesses: tokens(&M5RollbackPathReadiness::ALL, |v| v.as_str()),
            block_reasons: tokens(&M5PromotionBlockReason::ALL, |v| v.as_str()),
            next_actions: tokens(&M5PromotionNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CandidateCardExportField::ALL, |v| v.as_str()),
            scope_classes: tokens(&M5CandidateScopeClass::ALL, |v| v.as_str()),
            blocker_states: tokens(&M5CandidateBlockerState::ALL, |v| v.as_str()),
            rollback_blast_radii: tokens(&M5RollbackBlastRadius::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ReleaseCenterAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5ReleaseCandidateGovernanceReview {
    /// One candidate primitive carries scope, blocker, evidence, and rollback truth
    /// on every consumer.
    pub one_primitive_carries_candidate_truth: bool,
    /// The candidate identity and channel family are shown before promotion.
    pub identity_and_channel_always_shown: bool,
    /// Candidate scope and rollback target are explicit, never inferred from the
    /// version.
    pub scope_and_rollback_never_inferred_from_version: bool,
    /// Stale or missing evidence is never shown as clear.
    pub stale_or_missing_evidence_never_shown_clear: bool,
    /// Known issues are always disclosed on the card.
    pub known_issues_always_disclosed: bool,
    /// A blocked or narrowed candidate always shows a self-contained banner.
    pub blocked_state_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and next action, never a generic message.
    pub banner_names_exact_reason_and_next_action: bool,
    /// The support / export packet reconstructs candidate and blocker truth.
    pub support_export_reconstructs_candidate_truth: bool,
    /// No consumer invents a second candidate grammar.
    pub no_surface_invents_second_candidate_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel candidate-card vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCandidateConsumerProjection {
    /// Release-center, update-center, CLI, admin, and support/evaluation consumers
    /// all consume the shared primitive.
    pub candidate_surfaces_consume_shared_primitive: bool,
    /// The promotability resolver reads a single canonical source.
    pub promotability_resolver_reads_single_source: bool,
    /// The evidence-freshness cue reads a single canonical source.
    pub evidence_freshness_reads_single_source: bool,
    /// The rollback-path cue reads a single canonical source.
    pub rollback_path_reads_single_source: bool,
    /// Support / export reads a single canonical candidate-card source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCandidateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the candidate primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCandidateReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting candidate audit.
    pub candidate_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ReleaseCandidatePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReleaseCandidatePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Candidate rows.
    pub candidate_rows: Vec<M5ReleaseCandidateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ReleaseCandidateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ReleaseCandidateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReleaseCandidateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReleaseCandidateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ReleaseCandidateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 release-candidate-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseCandidatePrimitivePacket {
    /// Record kind; must equal [`M5_RELEASE_CANDIDATE_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RELEASE_CANDIDATE_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Candidate rows.
    pub candidate_rows: Vec<M5ReleaseCandidateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ReleaseCandidateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ReleaseCandidateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReleaseCandidateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReleaseCandidateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ReleaseCandidateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ReleaseCandidatePrimitivePacket {
    /// Builds an M5 release-candidate-primitive packet from stable-lane input.
    pub fn new(input: M5ReleaseCandidatePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_RELEASE_CANDIDATE_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_RELEASE_CANDIDATE_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            candidate_rows: input.candidate_rows,
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

    /// Validates the M5 release-candidate-primitive invariants.
    pub fn validate(&self) -> Vec<M5ReleaseCandidatePrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RELEASE_CANDIDATE_PRIMITIVE_RECORD_KIND {
            violations.push(M5ReleaseCandidatePrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RELEASE_CANDIDATE_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5ReleaseCandidatePrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReleaseCandidatePrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_candidate_rows(self, &mut violations);
        validate_promotability_coverage(self, &mut violations);
        validate_scope_and_rollback_explicit(self, &mut violations);
        validate_blocked_banner_self_contained(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 release-candidate primitive packet serializes"),
        ) {
            violations.push(M5ReleaseCandidatePrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 release-candidate primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per candidate consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,promotability_postures,evidence_freshness_states,block_reasons,next_actions,export_fields,example_count\n",
        );
        for row in &self.candidate_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.promotability_postures, |v| v.as_str()),
                join_tokens(&row.evidence_freshness_states, |v| v.as_str()),
                join_tokens(&row.block_reasons, |v| v.as_str()),
                join_tokens(&row.next_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .candidate_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Release-Candidate Card and Promotion-Blocked-Banner Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Release-candidate consumers: {} ({} stable)\n",
            self.candidate_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Promotability postures: {}\n",
            self.vocabulary_set.promotability_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Evidence-freshness states: {}\n",
            self.vocabulary_set.evidence_freshness_states.join(", ")
        ));
        out.push_str(&format!(
            "- Block reasons: {}\n",
            self.vocabulary_set.block_reasons.join(", ")
        ));
        out.push_str(&format!(
            "- Rollback-path readinesses: {}\n",
            self.vocabulary_set.rollback_path_readinesses.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Release-candidate consumers\n\n");
        for row in &self.candidate_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let banner = match &case.resolved.promotion_banner {
                    Some(banner) => banner.reason.as_str(),
                    None => "clear",
                };
                out.push_str(&format!(
                    "    - `{}` on `{}` → `{}` (evidence `{}`, rollback `{}`, banner `{}`)\n",
                    case.resolved.version_repr,
                    case.resolved.channel_family.as_str(),
                    case.resolved.promotability.as_str(),
                    case.resolved.evidence_freshness.as_str(),
                    case.resolved.rollback_path_readiness.as_str(),
                    banner
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 release-candidate-primitive export.
#[derive(Debug)]
pub enum M5ReleaseCandidatePrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ReleaseCandidatePrimitiveViolation>),
}

impl fmt::Display for M5ReleaseCandidatePrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 release-candidate primitive export parse failed: {error}"
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
                    "m5 release-candidate primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ReleaseCandidatePrimitiveArtifactError {}

/// Validation failures emitted by [`M5ReleaseCandidatePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReleaseCandidatePrimitiveViolation {
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
    /// A required release-candidate consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A candidate row is incomplete.
    CandidateRowIncomplete,
    /// A candidate row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A candidate row declares no channel families.
    ChannelFamilyMissing,
    /// A candidate row declares no promotability postures.
    PromotabilityPostureMissing,
    /// A candidate row declares no evidence-freshness states.
    EvidenceFreshnessMissing,
    /// A candidate row declares no rollback-path readinesses.
    RollbackReadinessMissing,
    /// A candidate row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A candidate row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A candidate row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A candidate row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A candidate row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A candidate claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves both a promotable and a blocked candidate.
    PromotabilityCoverageUnproven,
    /// No worked resolution proves an explicit scope and pinned rollback target.
    ScopeAndRollbackExplicitUnproven,
    /// No worked resolution proves a blocked candidate with a self-contained banner.
    BlockedBannerSelfContainedUnproven,
    /// A candidate row violates a hard invariant.
    CandidateInvariantViolated,
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

impl M5ReleaseCandidatePrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::CandidateRowIncomplete => "candidate_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::ChannelFamilyMissing => "channel_family_missing",
            Self::PromotabilityPostureMissing => "promotability_posture_missing",
            Self::EvidenceFreshnessMissing => "evidence_freshness_missing",
            Self::RollbackReadinessMissing => "rollback_readiness_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::PromotabilityCoverageUnproven => "promotability_coverage_unproven",
            Self::ScopeAndRollbackExplicitUnproven => "scope_and_rollback_explicit_unproven",
            Self::BlockedBannerSelfContainedUnproven => "blocked_banner_self_contained_unproven",
            Self::CandidateInvariantViolated => "candidate_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 release-candidate-primitive export.
pub fn current_stable_m5_release_candidate_primitive_export(
) -> Result<M5ReleaseCandidatePrimitivePacket, M5ReleaseCandidatePrimitiveArtifactError> {
    let packet: M5ReleaseCandidatePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-release-candidate-card-proof/support_export.json"
    )))
    .map_err(M5ReleaseCandidatePrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ReleaseCandidatePrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RELEASE_CANDIDATE_SCHEMA_REF,
        M5_RELEASE_CANDIDATE_DOC_REF,
        M5_RELEASE_CANDIDATE_COMPONENT_MATRIX_REF,
        M5_RELEASE_CANDIDATE_OBJECT_MODEL_REF,
        M5_RELEASE_CANDIDATE_ROLLBACK_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ReleaseCandidatePrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ReleaseCandidatePrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_candidate_rows(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    let present: BTreeSet<M5ReleaseCandidateConsumerSurface> = packet
        .candidate_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5ReleaseCandidateConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ReleaseCandidatePrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.candidate_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.scope_classes.is_empty()
            || row.blocker_states.is_empty()
            || row.known_issue_classes.is_empty()
            || row.block_reasons.is_empty()
            || row.next_actions.is_empty()
            || row.rollback_blast_radii.is_empty()
        {
            violations.push(M5ReleaseCandidatePrimitiveViolation::CandidateRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.channel_families.is_empty() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::ChannelFamilyMissing);
        }
        if row.promotability_postures.is_empty() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::PromotabilityPostureMissing);
        }
        if row.evidence_freshness_states.is_empty() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::EvidenceFreshnessMissing);
        }
        if row.rollback_path_readinesses.is_empty() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::RollbackReadinessMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ReleaseCandidatePrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ReleaseCandidatePrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ReleaseCandidatePrimitiveViolation::CandidateInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove a promotable
/// candidate and at least one must prove a blocked candidate — the
/// acceptance-criterion example that a user can tell promotable from blocked.
fn validate_promotability_coverage(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    let has_promotable = packet.candidate_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_promotable)
    });
    let has_blocked = packet.candidate_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_blocked)
    });
    if !(has_promotable && has_blocked) {
        violations.push(M5ReleaseCandidatePrimitiveViolation::PromotabilityCoverageUnproven);
    }
}

/// At least one worked resolution across the matrix must carry an explicit,
/// non-empty scoped artifact set and a pinned rollback target — the
/// acceptance-criterion example that scope and rollback are explicit rather than
/// inferred from the version.
fn validate_scope_and_rollback_explicit(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    let proven = packet.candidate_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            !case.resolved.artifact_set.is_empty()
                && case.resolved.rollback_path_readiness
                    == M5RollbackPathReadiness::RollbackTargetPinned
                && case.resolved.rollback_target_repr.is_some()
        })
    });
    if !proven {
        violations.push(M5ReleaseCandidatePrimitiveViolation::ScopeAndRollbackExplicitUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a blocked candidate
/// whose banner carries a specific reason, a next action, the blocked scope, and a
/// non-empty artifact set — the acceptance-criterion example that a blocked state
/// is understood from the banner rather than a secondary log.
fn validate_blocked_banner_self_contained(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    let proven = packet.candidate_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_blocked
                && case
                    .resolved
                    .promotion_banner
                    .as_ref()
                    .is_some_and(|banner| {
                        !banner.headline.trim().is_empty()
                            && !banner.blocked_artifact_set.is_empty()
                    })
        })
    });
    if !proven {
        violations.push(M5ReleaseCandidatePrimitiveViolation::BlockedBannerSelfContainedUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_candidate_truth,
        review.identity_and_channel_always_shown,
        review.scope_and_rollback_never_inferred_from_version,
        review.stale_or_missing_evidence_never_shown_clear,
        review.known_issues_always_disclosed,
        review.blocked_state_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_next_action,
        review.support_export_reconstructs_candidate_truth,
        review.no_surface_invents_second_candidate_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ReleaseCandidatePrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.candidate_surfaces_consume_shared_primitive,
        projection.promotability_resolver_reads_single_source,
        projection.evidence_freshness_reads_single_source,
        projection.rollback_path_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ReleaseCandidatePrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ReleaseCandidatePrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ReleaseCandidatePrimitivePacket,
    violations: &mut Vec<M5ReleaseCandidatePrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.candidate_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ReleaseCandidatePrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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
