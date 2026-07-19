//! One reusable M5 promotion-timeline-step / rollback-or-revocation-row primitive:
//! event identity, event kind, source and destination stage, immutable-digest joins,
//! evidence refs, approving actors, effective time, reversible window, affected node
//! set, blast radius, node targeting, last-known-good target, continuity note,
//! revocation scope, and break-glass attribution, projected the same way across every
//! claimed M5 release-history surface.
//!
//! Aureline's frozen release-center component matrix
//! ([`crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix`])
//! names the promotion-timeline step and the rollback/revocation row as two governed
//! component families and freezes the controlled vocabulary they depend on — the
//! promotion stage states, the rollout rings, the rollback blast radii, the revocation
//! scopes, the publication surface families, the deployment lines, the accessibility
//! routes, the qualification classes, and the downgrade triggers. This module
//! *implements* that promotion/rollback history contract as one reusable
//! step-plus-row primitive so a user or a support team can reconstruct — from the
//! timeline itself — exactly what changed and why: which stage moved to which stage,
//! under which digest and evidence joins, approved by whom, inside which reversible
//! window, and — for a rollback or a revocation — which nodes it affected, how far the
//! blast radius reached, which last-known-good it restored, and whether trust material
//! was revoked. Emergency (break-glass) operations stay visible in the same history
//! model rather than disappearing into CI-only metadata, and a rollback or revocation
//! never reads like a generic status change when its blast radius or its unaffected
//! nodes matter.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_release_history_event`] — that takes one release-history
//!    event (a promotion step or a rollback/revocation row) and produces one
//!    [`M5ResolvedReleaseHistoryEvent`] carrying the derived history posture (recorded
//!    versus blocked), the promotion-step view or the rollback/revocation view, the
//!    reconstruction readiness, the break-glass attribution, and — whenever the event
//!    cannot be honestly recorded — a self-contained [`M5ReleaseHistoryBanner`] that
//!    names the exact reason, the bound event, its digest join, its actors, and the
//!    next action rather than a generic `history unavailable`. The resolver keeps
//!    break-glass attribution and partial-scope node targeting explicit and never lets
//!    an emergency operation disappear from the history model.
//! 2. A parity matrix — [`M5ReleaseHistoryPrimitivePacket`] — that binds one row per
//!    claimed M5 release-history consumer (the release-center timeline, the
//!    update-center history, the CLI history inspect, the admin history report, and the
//!    support history export) to the shared step/row anatomy, the same promotion-stage
//!    / rollout-ring / blast-radius / revocation-scope vocabulary, the same reversible
//!    windows, node targetings, break-glass postures, history postures, block reasons,
//!    next actions, export fields, and non-visual accessibility routes, so promotion
//!    and rollback truth stays identical across the release center, the update center,
//!    the CLI, admin/reporting, and support.
//!
//! The promotion stage state ([`M5PromotionStageState`]), rollout ring
//! ([`M5RolloutRing`]), rollback blast radius ([`M5RollbackBlastRadius`]), revocation
//! scope ([`M5RevocationScope`]), publication surface family
//! ([`M5PublicationSurfaceFamily`]), deployment line ([`M5DeploymentLine`]),
//! release-center consumer surface ([`M5ReleaseCenterConsumerSurface`]), accessibility
//! route ([`M5ReleaseCenterAccessibilityRoute`]), qualification class
//! ([`M5ReleaseCenterQualificationClass`]), and downgrade trigger
//! ([`M5ReleaseCenterDowngradeTrigger`]) are reused verbatim from the frozen
//! release-center component matrix. This module mints new vocabulary only for what that
//! matrix left implicit about the timeline step and the rollback/revocation row
//! themselves: its history consumer families, its event kinds, its anatomy parts, its
//! reversible-window states, its break-glass postures, its node targetings, its history
//! postures, its block reasons, its next actions, and its export fields. No M5
//! release-history surface invents a second promotion or rollback grammar.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user
//! text bodies stay outside the support boundary; every event id, digest join, actor,
//! node ref, and last-known-good target is carried only as an opaque, export-safe
//! representation.
//!
//! The boundary schemas are
//! [`schemas/ui/m5-promotion-timeline-step.schema.json`](../../../../schemas/ui/m5-promotion-timeline-step.schema.json)
//! and
//! [`schemas/ui/m5-rollback-revocation-row.schema.json`](../../../../schemas/ui/m5-rollback-revocation-row.schema.json)
//! and the contract doc is
//! [`docs/release/m5_promotion_timeline_and_rollback_revocation_primitive_contract.md`](../../../../docs/release/m5_promotion_timeline_and_rollback_revocation_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-promotion-timeline-and-rollback-revocation-primitive/`](../../../../fixtures/ui/m5-promotion-timeline-and-rollback-revocation-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_release_history_primitive_cli_history_inspect_preview_narrowed,
    seeded_m5_release_history_primitive_packet,
    seeded_m5_release_history_primitive_update_center_history_beta_narrowed,
    M5_RELEASE_HISTORY_PRIMITIVE_PACKET_ID,
};

// The promotion stage state, rollout ring, rollback blast radius, revocation scope,
// publication surface family, deployment line, release-center consumer surface,
// accessibility routes, qualification classes, and downgrade triggers are frozen once,
// in the release-center component matrix. This primitive reuses them verbatim so it
// never invents a parallel promotion or rollback vocabulary.
pub use crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix::{
    M5DeploymentLine, M5PromotionStageState, M5PublicationSurfaceFamily,
    M5ReleaseCenterAccessibilityRoute, M5ReleaseCenterConsumerSurface,
    M5ReleaseCenterDowngradeTrigger, M5ReleaseCenterQualificationClass, M5RollbackBlastRadius,
    M5RevocationScope, M5RolloutRing,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ReleaseHistoryPrimitivePacket`].
pub const M5_RELEASE_HISTORY_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories";

/// Schema version for M5 release-history-primitive records.
pub const M5_RELEASE_HISTORY_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the promotion-timeline-step boundary schema.
pub const M5_RELEASE_HISTORY_STEP_SCHEMA_REF: &str =
    "schemas/ui/m5-promotion-timeline-step.schema.json";

/// Repo-relative path of the rollback-revocation-row boundary schema.
pub const M5_RELEASE_HISTORY_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-rollback-revocation-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RELEASE_HISTORY_DOC_REF: &str =
    "docs/release/m5_promotion_timeline_and_rollback_revocation_primitive_contract.md";

/// Repo-relative path of the frozen release-center component matrix this primitive
/// narrows from.
pub const M5_RELEASE_HISTORY_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-release-center-components.schema.json";

/// Repo-relative path of the release-center object-model contract this primitive binds
/// against.
pub const M5_RELEASE_HISTORY_OBJECT_MODEL_REF: &str =
    "docs/release/release_center_object_model_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RELEASE_HISTORY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-promotion-timeline-and-rollback-revocation-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RELEASE_HISTORY_ARTIFACT_REF: &str =
    "artifacts/release/m5-promotion-timeline-and-rollback-revocation-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_RELEASE_HISTORY_CSV_REF: &str =
    "artifacts/release/m5-promotion-timeline-and-rollback-revocation-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_RELEASE_HISTORY_REPORT_REF: &str =
    "artifacts/components/m5-promotion-timeline-step-and-rollback-revocation-row-primitive.md";

/// One claimed M5 release-history consumer that renders the shared promotion-timeline
/// step and rollback/revocation row. These are the consumers the acceptance criteria
/// name — the release center, the update center, the CLI, admin/reporting, and support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseHistoryConsumerSurface {
    /// The release-center / shiproom promotion timeline.
    ReleaseCenterTimeline,
    /// The update-center release history.
    UpdateCenterHistory,
    /// The CLI history-inspect / headless surface.
    CliHistoryInspect,
    /// The admin history report.
    AdminHistoryReport,
    /// The support history export.
    SupportHistoryExport,
}

impl M5ReleaseHistoryConsumerSurface {
    /// Every claimed history consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReleaseCenterTimeline,
        Self::UpdateCenterHistory,
        Self::CliHistoryInspect,
        Self::AdminHistoryReport,
        Self::SupportHistoryExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterTimeline => "release_center_timeline",
            Self::UpdateCenterHistory => "update_center_history",
            Self::CliHistoryInspect => "cli_history_inspect",
            Self::AdminHistoryReport => "admin_history_report",
            Self::SupportHistoryExport => "support_history_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenterTimeline => "Release-Center Promotion Timeline",
            Self::UpdateCenterHistory => "Update-Center Release History",
            Self::CliHistoryInspect => "CLI History Inspect",
            Self::AdminHistoryReport => "Admin History Report",
            Self::SupportHistoryExport => "Support History Export",
        }
    }
}

/// Which kind of release-history event a row describes, so a promotion step and a
/// rollback/revocation row each keep their own grammar rather than collapsing into one
/// generic status change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseHistoryEventKind {
    /// A promotion timeline step moving a candidate from one stage to another.
    PromotionStep,
    /// A rollback / revocation row reversing or revoking a prior promotion.
    RollbackRevocationRow,
}

impl M5ReleaseHistoryEventKind {
    /// Every event kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::PromotionStep, Self::RollbackRevocationRow];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromotionStep => "promotion_step",
            Self::RollbackRevocationRow => "rollback_revocation_row",
        }
    }

    /// True when this event is a promotion step.
    pub const fn is_promotion(self) -> bool {
        matches!(self, Self::PromotionStep)
    }

    /// True when this event is a rollback / revocation row.
    pub const fn is_rollback(self) -> bool {
        matches!(self, Self::RollbackRevocationRow)
    }
}

/// One anatomy part the shared timeline step / rollback row surfaces. The parts in
/// [`M5ReleaseHistoryAnatomyPart::MANDATORY`] are required on every event so a reviewer
/// can reconstruct what changed and why from the timeline alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseHistoryAnatomyPart {
    /// The event identity.
    EventIdentity,
    /// The event kind (promotion step versus rollback/revocation row).
    EventKind,
    /// The source stage (promotion).
    SourceStage,
    /// The destination stage (promotion).
    DestinationStage,
    /// The immutable-digest joins.
    DigestRefs,
    /// The evidence refs.
    EvidenceRefs,
    /// The approving actors.
    ApprovingActors,
    /// The effective time.
    EffectiveTime,
    /// The reversible window (promotion).
    ReversibleWindow,
    /// The affected node set (rollback).
    AffectedNodeSet,
    /// The blast radius (rollback).
    BlastRadius,
    /// The last-known-good target (rollback).
    LastKnownGoodTarget,
    /// The continuity note (rollback).
    ContinuityNote,
    /// The revocation scope (rollback).
    RevocationScope,
    /// The break-glass attribution.
    BreakGlassAttribution,
    /// The derived history verdict.
    HistoryVerdict,
    /// The history-blocked banner (shown when the event cannot be honestly recorded).
    HistoryBanner,
}

impl M5ReleaseHistoryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 17] = [
        Self::EventIdentity,
        Self::EventKind,
        Self::SourceStage,
        Self::DestinationStage,
        Self::DigestRefs,
        Self::EvidenceRefs,
        Self::ApprovingActors,
        Self::EffectiveTime,
        Self::ReversibleWindow,
        Self::AffectedNodeSet,
        Self::BlastRadius,
        Self::LastKnownGoodTarget,
        Self::ContinuityNote,
        Self::RevocationScope,
        Self::BreakGlassAttribution,
        Self::HistoryVerdict,
        Self::HistoryBanner,
    ];

    /// The anatomy parts every history event must render.
    pub const MANDATORY: [Self; 5] = [
        Self::EventIdentity,
        Self::EventKind,
        Self::ApprovingActors,
        Self::EffectiveTime,
        Self::HistoryVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventIdentity => "event_identity",
            Self::EventKind => "event_kind",
            Self::SourceStage => "source_stage",
            Self::DestinationStage => "destination_stage",
            Self::DigestRefs => "digest_refs",
            Self::EvidenceRefs => "evidence_refs",
            Self::ApprovingActors => "approving_actors",
            Self::EffectiveTime => "effective_time",
            Self::ReversibleWindow => "reversible_window",
            Self::AffectedNodeSet => "affected_node_set",
            Self::BlastRadius => "blast_radius",
            Self::LastKnownGoodTarget => "last_known_good_target",
            Self::ContinuityNote => "continuity_note",
            Self::RevocationScope => "revocation_scope",
            Self::BreakGlassAttribution => "break_glass_attribution",
            Self::HistoryVerdict => "history_verdict",
            Self::HistoryBanner => "history_banner",
        }
    }
}

/// Controlled reversible-window state — how much of a promotion step's reversal window
/// remains, so a timeline step never leaves its reversibility implicit and never shows
/// an expired window as still open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReversibleWindowState {
    /// The reversal window is open and comfortable.
    ReversibleWithinWindow,
    /// The reversal window is closing soon.
    ReversibleWindowClosing,
    /// The reversal window has expired.
    ReversibleWindowExpired,
    /// The step is irreversible by design.
    IrreversibleByDesign,
    /// No reversible window applies (a rollback / revocation row).
    NotApplicableWindow,
}

impl M5ReversibleWindowState {
    /// Every reversible-window state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReversibleWithinWindow,
        Self::ReversibleWindowClosing,
        Self::ReversibleWindowExpired,
        Self::IrreversibleByDesign,
        Self::NotApplicableWindow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleWithinWindow => "reversible_within_window",
            Self::ReversibleWindowClosing => "reversible_window_closing",
            Self::ReversibleWindowExpired => "reversible_window_expired",
            Self::IrreversibleByDesign => "irreversible_by_design",
            Self::NotApplicableWindow => "not_applicable_window",
        }
    }

    /// True when the reversal window is still open (comfortable or closing).
    pub const fn is_reversible(self) -> bool {
        matches!(
            self,
            Self::ReversibleWithinWindow | Self::ReversibleWindowClosing
        )
    }
}

/// Controlled break-glass posture — whether an event was a standard change-controlled
/// action or an emergency break-glass action, and whether that emergency is attributed,
/// so an emergency operation stays visible and attributable in the same history model
/// rather than disappearing into CI-only metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BreakGlassPosture {
    /// A standard change-controlled action.
    StandardChangeControl,
    /// An emergency break-glass action attributed to a named actor.
    BreakGlassAttributed,
    /// An emergency break-glass action executed with a named actor, review pending.
    BreakGlassPendingReview,
    /// An emergency break-glass action with no attributed actor (must be blocked).
    BreakGlassUnattributed,
}

impl M5BreakGlassPosture {
    /// Every break-glass posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StandardChangeControl,
        Self::BreakGlassAttributed,
        Self::BreakGlassPendingReview,
        Self::BreakGlassUnattributed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StandardChangeControl => "standard_change_control",
            Self::BreakGlassAttributed => "break_glass_attributed",
            Self::BreakGlassPendingReview => "break_glass_pending_review",
            Self::BreakGlassUnattributed => "break_glass_unattributed",
        }
    }

    /// True when this posture is an emergency break-glass action.
    pub const fn is_emergency(self) -> bool {
        matches!(
            self,
            Self::BreakGlassAttributed
                | Self::BreakGlassPendingReview
                | Self::BreakGlassUnattributed
        )
    }

    /// True when this posture is an emergency action carrying no attribution.
    pub const fn is_unattributed(self) -> bool {
        matches!(self, Self::BreakGlassUnattributed)
    }
}

/// Controlled node targeting — how a rollback / revocation targets the artifact-graph
/// node set, so a partial-scope targeting is preserved and never silently widened to
/// the whole fleet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NodeTargeting {
    /// Every node in the blast radius is targeted.
    AllNodes,
    /// An explicitly enumerated partial node set is targeted.
    PartialNodeSetExplicit,
    /// A single node is targeted.
    SingleNodeTargeted,
    /// No node targeting applies (a promotion step).
    NotApplicableTargeting,
}

impl M5NodeTargeting {
    /// Every node targeting, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AllNodes,
        Self::PartialNodeSetExplicit,
        Self::SingleNodeTargeted,
        Self::NotApplicableTargeting,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllNodes => "all_nodes",
            Self::PartialNodeSetExplicit => "partial_node_set_explicit",
            Self::SingleNodeTargeted => "single_node_targeted",
            Self::NotApplicableTargeting => "not_applicable_targeting",
        }
    }

    /// True when the targeting is an explicit partial scope.
    pub const fn is_partial(self) -> bool {
        matches!(
            self,
            Self::PartialNodeSetExplicit | Self::SingleNodeTargeted
        )
    }
}

/// The derived headline history posture of a release-history event — the resolver's
/// verdict about whether the event can be honestly recorded. Promotion and rollback
/// each keep their own recorded postures; an unattributed emergency, a missing
/// last-known-good, and a missing immutable-digest join block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseHistoryPosture {
    /// A promotion recorded inside an open reversible window.
    PromotionRecordedReversible,
    /// A promotion recorded whose reversible window has expired or is irreversible by
    /// design — honest that it can no longer be reversed.
    PromotionRecordedIrreversible,
    /// A promotion still pending or in progress.
    PromotionInProgress,
    /// A promotion whose stage is blocked.
    PromotionBlocked,
    /// A rollback recorded with an explicit blast radius and last-known-good target.
    RollbackRecordedBounded,
    /// A revocation recorded with an explicit revocation scope (trust material rotated).
    RevocationRecorded,
    /// An emergency break-glass action recorded, attributed, and visible in history.
    EmergencyBreakGlassRecorded,
    /// Blocked: an emergency action carries no attributed actor.
    HistoryBlockedUnattributed,
    /// Blocked: a rollback / revocation names no last-known-good target.
    HistoryBlockedMissingLastKnownGood,
    /// Blocked: the event carries no immutable-digest join.
    HistoryBlockedMissingDigestJoin,
}

impl M5ReleaseHistoryPosture {
    /// Every history posture, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::PromotionRecordedReversible,
        Self::PromotionRecordedIrreversible,
        Self::PromotionInProgress,
        Self::PromotionBlocked,
        Self::RollbackRecordedBounded,
        Self::RevocationRecorded,
        Self::EmergencyBreakGlassRecorded,
        Self::HistoryBlockedUnattributed,
        Self::HistoryBlockedMissingLastKnownGood,
        Self::HistoryBlockedMissingDigestJoin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromotionRecordedReversible => "promotion_recorded_reversible",
            Self::PromotionRecordedIrreversible => "promotion_recorded_irreversible",
            Self::PromotionInProgress => "promotion_in_progress",
            Self::PromotionBlocked => "promotion_blocked",
            Self::RollbackRecordedBounded => "rollback_recorded_bounded",
            Self::RevocationRecorded => "revocation_recorded",
            Self::EmergencyBreakGlassRecorded => "emergency_break_glass_recorded",
            Self::HistoryBlockedUnattributed => "history_blocked_unattributed",
            Self::HistoryBlockedMissingLastKnownGood => "history_blocked_missing_last_known_good",
            Self::HistoryBlockedMissingDigestJoin => "history_blocked_missing_digest_join",
        }
    }

    /// True when the event can be honestly recorded into the timeline.
    pub const fn is_recorded(self) -> bool {
        matches!(
            self,
            Self::PromotionRecordedReversible
                | Self::PromotionRecordedIrreversible
                | Self::PromotionInProgress
                | Self::RollbackRecordedBounded
                | Self::RevocationRecorded
                | Self::EmergencyBreakGlassRecorded
        )
    }

    /// True when the event cannot be honestly recorded and shows a banner.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::PromotionBlocked
                | Self::HistoryBlockedUnattributed
                | Self::HistoryBlockedMissingLastKnownGood
                | Self::HistoryBlockedMissingDigestJoin
        )
    }

    /// The specific block reason for a blocked posture, if any. Returns `None` for a
    /// recorded posture.
    pub const fn block_reason(self) -> Option<M5ReleaseHistoryBlockReason> {
        Some(match self {
            Self::PromotionBlocked => M5ReleaseHistoryBlockReason::StagePromotionBlocked,
            Self::HistoryBlockedUnattributed => {
                M5ReleaseHistoryBlockReason::EmergencyActionUnattributed
            }
            Self::HistoryBlockedMissingLastKnownGood => {
                M5ReleaseHistoryBlockReason::MissingLastKnownGoodTarget
            }
            Self::HistoryBlockedMissingDigestJoin => {
                M5ReleaseHistoryBlockReason::MissingImmutableDigestJoin
            }
            _ => return None,
        })
    }
}

/// The exact reason a release-history event is blocked, so a history-blocked banner
/// never reads like a generic `history unavailable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseHistoryBlockReason {
    /// A promotion stage is blocked.
    StagePromotionBlocked,
    /// An emergency action carries no attributed actor.
    EmergencyActionUnattributed,
    /// A rollback / revocation names no last-known-good target.
    MissingLastKnownGoodTarget,
    /// The event carries no immutable-digest join.
    MissingImmutableDigestJoin,
}

impl M5ReleaseHistoryBlockReason {
    /// Every block reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StagePromotionBlocked,
        Self::EmergencyActionUnattributed,
        Self::MissingLastKnownGoodTarget,
        Self::MissingImmutableDigestJoin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StagePromotionBlocked => "stage_promotion_blocked",
            Self::EmergencyActionUnattributed => "emergency_action_unattributed",
            Self::MissingLastKnownGoodTarget => "missing_last_known_good_target",
            Self::MissingImmutableDigestJoin => "missing_immutable_digest_join",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::StagePromotionBlocked => "the promotion stage is blocked",
            Self::EmergencyActionUnattributed => {
                "the emergency action has no attributed actor (break-glass must stay attributable)"
            }
            Self::MissingLastKnownGoodTarget => "the rollback names no last-known-good target",
            Self::MissingImmutableDigestJoin => {
                "the event carries no immutable-digest join (artifact-graph consistency requires one)"
            }
        }
    }

    /// The next action a reviewer should take to clear this reason.
    pub const fn next_action(self) -> M5ReleaseHistoryNextAction {
        match self {
            Self::StagePromotionBlocked => M5ReleaseHistoryNextAction::ResolveStageBlocker,
            Self::EmergencyActionUnattributed => {
                M5ReleaseHistoryNextAction::AttributeEmergencyActor
            }
            Self::MissingLastKnownGoodTarget => {
                M5ReleaseHistoryNextAction::RecordLastKnownGoodTarget
            }
            Self::MissingImmutableDigestJoin => {
                M5ReleaseHistoryNextAction::RecordImmutableDigestJoin
            }
        }
    }
}

/// The next action named on a history-blocked banner, so a blocked event is actionable
/// from the banner itself rather than from CI-only metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseHistoryNextAction {
    /// Resolve the stage blocker.
    ResolveStageBlocker,
    /// Attribute the emergency action to a named actor.
    AttributeEmergencyActor,
    /// Record the last-known-good target.
    RecordLastKnownGoodTarget,
    /// Record the immutable-digest join.
    RecordImmutableDigestJoin,
}

impl M5ReleaseHistoryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ResolveStageBlocker,
        Self::AttributeEmergencyActor,
        Self::RecordLastKnownGoodTarget,
        Self::RecordImmutableDigestJoin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveStageBlocker => "resolve_stage_blocker",
            Self::AttributeEmergencyActor => "attribute_emergency_actor",
            Self::RecordLastKnownGoodTarget => "record_last_known_good_target",
            Self::RecordImmutableDigestJoin => "record_immutable_digest_join",
        }
    }
}

/// A field the support / export packet carries so promotion and rollback truth is
/// reconstructable from the shared model. The fields in
/// [`M5ReleaseHistoryExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseHistoryExportField {
    /// The opaque event identity.
    EventIdentity,
    /// The event kind.
    EventKind,
    /// The source stage.
    SourceStage,
    /// The destination stage.
    DestinationStage,
    /// The immutable-digest joins.
    DigestRefs,
    /// The evidence refs.
    EvidenceRefs,
    /// The approving actors.
    ApprovingActors,
    /// The effective time.
    EffectiveTime,
    /// The reversible-window state.
    ReversibleWindow,
    /// The affected node set.
    AffectedNodeSet,
    /// The blast radius.
    BlastRadius,
    /// The node targeting.
    NodeTargeting,
    /// The last-known-good target.
    LastKnownGoodTarget,
    /// The continuity note.
    ContinuityNote,
    /// The revocation scope.
    RevocationScope,
    /// The break-glass posture.
    BreakGlassPosture,
    /// The derived history posture.
    HistoryPosture,
    /// The block reason (when blocked).
    BlockReason,
}

impl M5ReleaseHistoryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 18] = [
        Self::EventIdentity,
        Self::EventKind,
        Self::SourceStage,
        Self::DestinationStage,
        Self::DigestRefs,
        Self::EvidenceRefs,
        Self::ApprovingActors,
        Self::EffectiveTime,
        Self::ReversibleWindow,
        Self::AffectedNodeSet,
        Self::BlastRadius,
        Self::NodeTargeting,
        Self::LastKnownGoodTarget,
        Self::ContinuityNote,
        Self::RevocationScope,
        Self::BreakGlassPosture,
        Self::HistoryPosture,
        Self::BlockReason,
    ];

    /// The export fields every history export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::EventIdentity,
        Self::EventKind,
        Self::DigestRefs,
        Self::ApprovingActors,
        Self::HistoryPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventIdentity => "event_identity",
            Self::EventKind => "event_kind",
            Self::SourceStage => "source_stage",
            Self::DestinationStage => "destination_stage",
            Self::DigestRefs => "digest_refs",
            Self::EvidenceRefs => "evidence_refs",
            Self::ApprovingActors => "approving_actors",
            Self::EffectiveTime => "effective_time",
            Self::ReversibleWindow => "reversible_window",
            Self::AffectedNodeSet => "affected_node_set",
            Self::BlastRadius => "blast_radius",
            Self::NodeTargeting => "node_targeting",
            Self::LastKnownGoodTarget => "last_known_good_target",
            Self::ContinuityNote => "continuity_note",
            Self::RevocationScope => "revocation_scope",
            Self::BreakGlassPosture => "break_glass_posture",
            Self::HistoryPosture => "history_posture",
            Self::BlockReason => "block_reason",
        }
    }
}

/// The derived promotion-step view for a promotion event, so a timeline step keeps its
/// source stage, destination stage, stage state, rollout ring, and reversible window
/// explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromotionStepView {
    /// The opaque source stage.
    pub source_stage_repr: String,
    /// The opaque destination stage.
    pub destination_stage_repr: String,
    /// The promotion stage state.
    pub stage_state: M5PromotionStageState,
    /// The reversible-window state.
    pub reversible_window: M5ReversibleWindowState,
    /// True when the reversal window is still open.
    pub reversible: bool,
}

/// The derived rollback/revocation view for a rollback event, so a row keeps its
/// affected node set, blast radius, node targeting, last-known-good target, continuity
/// note, and revocation scope explicit — never a generic status change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RollbackRevocationView {
    /// The affected node set.
    pub affected_node_set: Vec<String>,
    /// The count of affected nodes.
    pub affected_node_count: usize,
    /// The rollback blast radius.
    pub blast_radius: M5RollbackBlastRadius,
    /// The node targeting.
    pub node_targeting: M5NodeTargeting,
    /// The opaque last-known-good target.
    pub last_known_good_target_repr: String,
    /// The opaque continuity note.
    pub continuity_note_repr: String,
    /// The revocation scope.
    pub revocation_scope: M5RevocationScope,
    /// True when trust material (artifact, signing key, or trust root) is revoked.
    pub revokes_trust_material: bool,
}

/// The derived reconstruction readiness of a history event, so a reviewer can tell
/// whether the timeline alone carries enough to reconstruct what changed and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryReconstruction {
    /// True when the event names its approving actors.
    pub has_actors: bool,
    /// True when the event carries at least one immutable-digest join.
    pub has_digest_refs: bool,
    /// True when the event carries at least one evidence ref.
    pub has_evidence_refs: bool,
    /// True when the event carries an effective time.
    pub has_effective_time: bool,
    /// True when the timeline alone carries enough to reconstruct what changed and why.
    pub is_reconstructable: bool,
}

/// A self-contained history-blocked banner: the exact reason, the bound event, its
/// digest join, its actors, and the next action, so a blocked event is understood from
/// the banner alone rather than from CI-only metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryBanner {
    /// The exact block reason.
    pub reason: M5ReleaseHistoryBlockReason,
    /// The next action a reviewer should take.
    pub next_action: M5ReleaseHistoryNextAction,
    /// The bound event identity.
    pub event_identity_repr: String,
    /// The primary immutable-digest join the banner binds to (empty when none present).
    pub bound_digest_repr: String,
    /// The approving actors the banner preserves.
    pub approving_actors: Vec<String>,
    /// The history posture the banner reports.
    pub history_posture: M5ReleaseHistoryPosture,
    /// A deterministic, self-contained headline naming the reason, the event, the
    /// digest, and the next action — never a generic `history unavailable`.
    pub headline: String,
}

/// The full input to the release-history resolver for one event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryEventInput {
    /// The opaque, export-safe event identity.
    pub event_identity_repr: String,
    /// The event kind.
    pub event_kind: M5ReleaseHistoryEventKind,
    /// The opaque source stage (empty for a rollback / revocation row).
    pub source_stage_repr: String,
    /// The opaque destination stage (empty for a rollback / revocation row).
    pub destination_stage_repr: String,
    /// The promotion stage state.
    pub stage_state: M5PromotionStageState,
    /// The rollout ring the event moved within.
    pub rollout_ring: M5RolloutRing,
    /// The reversible-window state (promotion).
    pub reversible_window: M5ReversibleWindowState,
    /// The immutable-digest joins. Must be non-empty for the event to be honestly
    /// recorded.
    pub digest_refs: Vec<String>,
    /// The evidence refs.
    pub evidence_refs: Vec<String>,
    /// The approving actors.
    pub approving_actors: Vec<String>,
    /// The opaque effective time (must be non-empty).
    pub effective_time_repr: String,
    /// The break-glass posture.
    pub break_glass_posture: M5BreakGlassPosture,
    /// The affected node set (rollback).
    pub affected_node_set: Vec<String>,
    /// The rollback blast radius.
    pub blast_radius: M5RollbackBlastRadius,
    /// The node targeting.
    pub node_targeting: M5NodeTargeting,
    /// The revocation scope.
    pub revocation_scope: M5RevocationScope,
    /// The opaque last-known-good target (rollback).
    pub last_known_good_target_repr: String,
    /// The opaque continuity note (rollback).
    pub continuity_note_repr: String,
}

/// The resolved promotion / rollback truth for one release-history event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedReleaseHistoryEvent {
    /// The opaque event identity.
    pub event_identity_repr: String,
    /// The event kind.
    pub event_kind: M5ReleaseHistoryEventKind,
    /// The immutable-digest joins.
    pub digest_refs: Vec<String>,
    /// The count of immutable-digest joins.
    pub digest_ref_count: usize,
    /// The evidence refs.
    pub evidence_refs: Vec<String>,
    /// The approving actors.
    pub approving_actors: Vec<String>,
    /// The opaque effective time.
    pub effective_time_repr: String,
    /// The rollout ring the event moved within.
    pub rollout_ring: M5RolloutRing,
    /// The node targeting.
    pub node_targeting: M5NodeTargeting,
    /// The break-glass posture.
    pub break_glass_posture: M5BreakGlassPosture,
    /// True when the event is an emergency action visible in the history model.
    pub emergency_visible_in_history: bool,
    /// The promotion-step view (present for a promotion step).
    pub promotion_view: Option<M5PromotionStepView>,
    /// The rollback/revocation view (present for a rollback / revocation row).
    pub rollback_view: Option<M5RollbackRevocationView>,
    /// The reconstruction readiness.
    pub reconstruction: M5ReleaseHistoryReconstruction,
    /// The derived history posture.
    pub history_posture: M5ReleaseHistoryPosture,
    /// True when the event can be honestly recorded.
    pub is_recorded: bool,
    /// True when the event is blocked.
    pub is_blocked: bool,
    /// True when the event is an emergency action.
    pub is_emergency: bool,
    /// The history-blocked banner, present when the event is blocked.
    pub history_banner: Option<M5ReleaseHistoryBanner>,
}

/// Errors returned by [`resolve_release_history_event`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ReleaseHistoryEventError {
    /// The event identity was empty.
    EmptyEventIdentity,
    /// The effective time was empty.
    EmptyEffectiveTime,
    /// An event id, stage, digest, actor, node ref, target, note, or evidence ref
    /// carried forbidden material.
    ForbiddenHistoryMaterial,
}

impl M5ReleaseHistoryEventError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyEventIdentity => "empty_event_identity",
            Self::EmptyEffectiveTime => "empty_effective_time",
            Self::ForbiddenHistoryMaterial => "forbidden_history_material",
        }
    }
}

impl fmt::Display for M5ReleaseHistoryEventError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "release-history resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ReleaseHistoryEventError {}

/// Resolves one release-history event from its declared kind, stage, digest joins,
/// actors, reversible window, blast radius, revocation scope, and break-glass posture.
///
/// The derived history posture is the headline verdict, computed in a fixed
/// blocking-first order: a missing immutable-digest join blocks first (artifact-graph
/// consistency requires one), then an unattributed emergency action blocks, then a
/// rollback / revocation with no last-known-good target blocks; an attributed emergency
/// action is recorded and kept visible in the same history model; a promotion resolves
/// by its stage state and reversible window; and a rollback / revocation resolves to a
/// bounded rollback or a trust-material revocation. A blocked event always produces a
/// self-contained banner.
pub fn resolve_release_history_event(
    input: &M5ReleaseHistoryEventInput,
) -> Result<M5ResolvedReleaseHistoryEvent, M5ReleaseHistoryEventError> {
    if input.event_identity_repr.trim().is_empty() {
        return Err(M5ReleaseHistoryEventError::EmptyEventIdentity);
    }
    if input.effective_time_repr.trim().is_empty() {
        return Err(M5ReleaseHistoryEventError::EmptyEffectiveTime);
    }
    for value in [
        &input.event_identity_repr,
        &input.source_stage_repr,
        &input.destination_stage_repr,
        &input.effective_time_repr,
        &input.last_known_good_target_repr,
        &input.continuity_note_repr,
    ] {
        if value_repr_is_forbidden(value) {
            return Err(M5ReleaseHistoryEventError::ForbiddenHistoryMaterial);
        }
    }
    for list in [
        &input.digest_refs,
        &input.evidence_refs,
        &input.approving_actors,
        &input.affected_node_set,
    ] {
        if list.iter().any(|v| value_repr_is_forbidden(v)) {
            return Err(M5ReleaseHistoryEventError::ForbiddenHistoryMaterial);
        }
    }

    let non_empty_digests: Vec<String> = input
        .digest_refs
        .iter()
        .filter(|d| !d.trim().is_empty())
        .cloned()
        .collect();
    let non_empty_actors: Vec<String> = input
        .approving_actors
        .iter()
        .filter(|a| !a.trim().is_empty())
        .cloned()
        .collect();

    let history_posture = derive_history_posture(input, &non_empty_digests, &non_empty_actors);

    let promotion_view = input
        .event_kind
        .is_promotion()
        .then(|| M5PromotionStepView {
            source_stage_repr: input.source_stage_repr.clone(),
            destination_stage_repr: input.destination_stage_repr.clone(),
            stage_state: input.stage_state,
            reversible_window: input.reversible_window,
            reversible: input.reversible_window.is_reversible(),
        });

    let rollback_view = input
        .event_kind
        .is_rollback()
        .then(|| M5RollbackRevocationView {
            affected_node_set: input.affected_node_set.clone(),
            affected_node_count: input.affected_node_set.len(),
            blast_radius: input.blast_radius,
            node_targeting: input.node_targeting,
            last_known_good_target_repr: input.last_known_good_target_repr.clone(),
            continuity_note_repr: input.continuity_note_repr.clone(),
            revocation_scope: input.revocation_scope,
            revokes_trust_material: revokes_trust_material(input.revocation_scope),
        });

    let has_actors = !non_empty_actors.is_empty();
    let has_digest_refs = !non_empty_digests.is_empty();
    let has_evidence_refs = input.evidence_refs.iter().any(|e| !e.trim().is_empty());
    let has_effective_time = !input.effective_time_repr.trim().is_empty();
    let reconstruction = M5ReleaseHistoryReconstruction {
        has_actors,
        has_digest_refs,
        has_evidence_refs,
        has_effective_time,
        is_reconstructable: has_actors
            && has_digest_refs
            && has_evidence_refs
            && has_effective_time,
    };

    let is_recorded = history_posture.is_recorded();
    let is_blocked = history_posture.is_blocked();
    let is_emergency = input.break_glass_posture.is_emergency();

    let bound_digest_repr = non_empty_digests.first().cloned().unwrap_or_default();

    let history_banner = history_posture.block_reason().map(|reason| {
        let next_action = reason.next_action();
        let headline = format!(
            "History held: {} — event {} (digest {}, {} approving actor(s)); posture {}, next: {}",
            reason.phrase(),
            input.event_identity_repr,
            if bound_digest_repr.is_empty() {
                "none"
            } else {
                bound_digest_repr.as_str()
            },
            non_empty_actors.len(),
            history_posture.as_str(),
            next_action.as_str()
        );
        M5ReleaseHistoryBanner {
            reason,
            next_action,
            event_identity_repr: input.event_identity_repr.clone(),
            bound_digest_repr: bound_digest_repr.clone(),
            approving_actors: non_empty_actors.clone(),
            history_posture,
            headline,
        }
    });

    Ok(M5ResolvedReleaseHistoryEvent {
        event_identity_repr: input.event_identity_repr.clone(),
        event_kind: input.event_kind,
        digest_refs: non_empty_digests.clone(),
        digest_ref_count: non_empty_digests.len(),
        evidence_refs: input.evidence_refs.clone(),
        approving_actors: non_empty_actors,
        effective_time_repr: input.effective_time_repr.clone(),
        rollout_ring: input.rollout_ring,
        node_targeting: input.node_targeting,
        break_glass_posture: input.break_glass_posture,
        emergency_visible_in_history: is_emergency,
        promotion_view,
        rollback_view,
        reconstruction,
        history_posture,
        is_recorded,
        is_blocked,
        is_emergency,
        history_banner,
    })
}

/// The fixed blocking-first history ladder.
fn derive_history_posture(
    input: &M5ReleaseHistoryEventInput,
    non_empty_digests: &[String],
    non_empty_actors: &[String],
) -> M5ReleaseHistoryPosture {
    // Consistency guards first — an event with no immutable-digest join cannot be
    // joined into the artifact graph.
    if non_empty_digests.is_empty() {
        return M5ReleaseHistoryPosture::HistoryBlockedMissingDigestJoin;
    }
    // An emergency action must stay attributed to a named actor.
    let emergency = input.break_glass_posture.is_emergency();
    if emergency && (input.break_glass_posture.is_unattributed() || non_empty_actors.is_empty()) {
        return M5ReleaseHistoryPosture::HistoryBlockedUnattributed;
    }
    // A rollback / revocation must name a last-known-good target.
    if input.event_kind.is_rollback() && input.last_known_good_target_repr.trim().is_empty() {
        return M5ReleaseHistoryPosture::HistoryBlockedMissingLastKnownGood;
    }
    // An attributed emergency action is recorded and kept visible in the same history
    // model rather than disappearing into CI-only metadata.
    if emergency {
        return M5ReleaseHistoryPosture::EmergencyBreakGlassRecorded;
    }
    match input.event_kind {
        M5ReleaseHistoryEventKind::PromotionStep => match input.stage_state {
            M5PromotionStageState::StageBlocked => M5ReleaseHistoryPosture::PromotionBlocked,
            M5PromotionStageState::StagePending | M5PromotionStageState::StageInProgress => {
                M5ReleaseHistoryPosture::PromotionInProgress
            }
            M5PromotionStageState::StageRolledBack => {
                M5ReleaseHistoryPosture::PromotionRecordedReversible
            }
            M5PromotionStageState::StagePromoted => {
                if input.reversible_window.is_reversible() {
                    M5ReleaseHistoryPosture::PromotionRecordedReversible
                } else {
                    M5ReleaseHistoryPosture::PromotionRecordedIrreversible
                }
            }
        },
        M5ReleaseHistoryEventKind::RollbackRevocationRow => {
            if revokes_trust_material(input.revocation_scope) {
                M5ReleaseHistoryPosture::RevocationRecorded
            } else {
                M5ReleaseHistoryPosture::RollbackRecordedBounded
            }
        }
    }
}

/// True when a revocation scope rotates trust material (the artifact, a signing key, or
/// the trust root) rather than merely repointing a tag or performing a soft rollback.
fn revokes_trust_material(scope: M5RevocationScope) -> bool {
    matches!(
        scope,
        M5RevocationScope::ArtifactRevoked
            | M5RevocationScope::SigningKeyRevoked
            | M5RevocationScope::TrustRootRotated
    )
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs promotion and rollback truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryResolutionCase {
    /// The resolver input.
    pub input: M5ReleaseHistoryEventInput,
    /// The resolved truth. Must equal `resolve_release_history_event(&input)`.
    pub resolved: M5ResolvedReleaseHistoryEvent,
}

impl M5ReleaseHistoryResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ReleaseHistoryEventInput) -> Self {
        let resolved =
            resolve_release_history_event(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_release_history_event(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one history consumer bound to the shared step/row
/// anatomy, history postures, promotion-stage / rollout-ring / blast-radius /
/// revocation-scope vocabulary, reversible windows, node targetings, break-glass
/// postures, block reasons, next actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryRow {
    /// History consumer family.
    pub consumer_surface: M5ReleaseHistoryConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ReleaseCenterQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 publication surface families that render / consume this component.
    pub surface_families: Vec<M5PublicationSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this component renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ReleaseHistoryAnatomyPart>,
    /// Event kinds this component distinguishes.
    pub event_kinds: Vec<M5ReleaseHistoryEventKind>,
    /// Promotion stage states this component distinguishes.
    pub promotion_stage_states: Vec<M5PromotionStageState>,
    /// Rollout rings this component distinguishes.
    pub rollout_rings: Vec<M5RolloutRing>,
    /// Reversible-window states this component distinguishes.
    pub reversible_window_states: Vec<M5ReversibleWindowState>,
    /// Rollback blast radii this component distinguishes.
    pub blast_radii: Vec<M5RollbackBlastRadius>,
    /// Node targetings this component distinguishes.
    pub node_targetings: Vec<M5NodeTargeting>,
    /// Revocation scopes this component distinguishes.
    pub revocation_scopes: Vec<M5RevocationScope>,
    /// Break-glass postures this component distinguishes.
    pub break_glass_postures: Vec<M5BreakGlassPosture>,
    /// History postures this component distinguishes.
    pub history_postures: Vec<M5ReleaseHistoryPosture>,
    /// Block reasons this component names.
    pub block_reasons: Vec<M5ReleaseHistoryBlockReason>,
    /// Next actions this component names.
    pub next_actions: Vec<M5ReleaseHistoryNextAction>,
    /// Export fields this component carries (must include the mandatory fields).
    pub export_fields: Vec<M5ReleaseHistoryExportField>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5ReleaseCenterAccessibilityRoute>,
    /// Release-center subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ReleaseCenterConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5ReleaseCenterDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5ReleaseHistoryResolutionCase>,
    /// Hard invariant: this component never reads a rollback / revocation as a generic
    /// status change. MUST be `false`.
    pub reads_rollback_as_generic_status: bool,
    /// Hard invariant: this component never drops break-glass attribution. MUST be
    /// `false`.
    pub drops_break_glass_attribution: bool,
    /// Hard invariant: this component never hides the blast radius or the unaffected
    /// nodes. MUST be `false`.
    pub hides_blast_radius_or_unaffected_nodes: bool,
    /// Hard invariant: this component never lets an emergency action disappear into
    /// CI-only metadata. MUST be `false`.
    pub lets_emergency_disappear_into_ci_only_metadata: bool,
}

impl M5ReleaseHistoryRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ReleaseHistoryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ReleaseHistoryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ReleaseHistoryExportField> =
            self.export_fields.iter().copied().collect();
        M5ReleaseHistoryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.reads_rollback_as_generic_status
            && !self.drops_break_glass_attribution
            && !self.hides_blast_radius_or_unaffected_nodes
            && !self.lets_emergency_disappear_into_ci_only_metadata
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryVocabularySet {
    /// History consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Event-kind tokens.
    pub event_kinds: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Reversible-window-state tokens.
    pub reversible_window_states: Vec<String>,
    /// Node-targeting tokens.
    pub node_targetings: Vec<String>,
    /// Break-glass-posture tokens.
    pub break_glass_postures: Vec<String>,
    /// History-posture tokens.
    pub history_postures: Vec<String>,
    /// Block-reason tokens.
    pub block_reasons: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Promotion-stage-state tokens (reused from the frozen matrix).
    pub promotion_stage_states: Vec<String>,
    /// Rollout-ring tokens (reused from the frozen matrix).
    pub rollout_rings: Vec<String>,
    /// Rollback-blast-radius tokens (reused from the frozen matrix).
    pub blast_radii: Vec<String>,
    /// Revocation-scope tokens (reused from the frozen matrix).
    pub revocation_scopes: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ReleaseHistoryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5ReleaseHistoryConsumerSurface::ALL, |v| v.as_str()),
            event_kinds: tokens(&M5ReleaseHistoryEventKind::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ReleaseHistoryAnatomyPart::ALL, |v| v.as_str()),
            reversible_window_states: tokens(&M5ReversibleWindowState::ALL, |v| v.as_str()),
            node_targetings: tokens(&M5NodeTargeting::ALL, |v| v.as_str()),
            break_glass_postures: tokens(&M5BreakGlassPosture::ALL, |v| v.as_str()),
            history_postures: tokens(&M5ReleaseHistoryPosture::ALL, |v| v.as_str()),
            block_reasons: tokens(&M5ReleaseHistoryBlockReason::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ReleaseHistoryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ReleaseHistoryExportField::ALL, |v| v.as_str()),
            promotion_stage_states: tokens(&M5PromotionStageState::ALL, |v| v.as_str()),
            rollout_rings: tokens(&M5RolloutRing::ALL, |v| v.as_str()),
            blast_radii: tokens(&M5RollbackBlastRadius::ALL, |v| v.as_str()),
            revocation_scopes: tokens(&M5RevocationScope::ALL, |v| v.as_str()),
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
pub struct M5ReleaseHistoryGovernanceReview {
    /// One history primitive carries promotion and rollback truth on every consumer.
    pub one_primitive_carries_history_truth: bool,
    /// A reviewer can reconstruct what changed and why from the timeline itself.
    pub reconstructable_from_timeline: bool,
    /// A rollback / revocation never reads like a generic status change.
    pub rollback_never_reads_as_generic_status: bool,
    /// The blast radius and the unaffected nodes stay explicit.
    pub blast_radius_and_unaffected_nodes_explicit: bool,
    /// Break-glass attribution and partial-scope targeting are preserved.
    pub break_glass_attribution_and_partial_scope_preserved: bool,
    /// Emergency operations stay visible in the same history model.
    pub emergency_stays_visible_in_history_model: bool,
    /// Artifact-graph consistency is preserved (every recorded event carries a digest
    /// join).
    pub artifact_graph_consistency_preserved: bool,
    /// A blocked event always shows a self-contained banner.
    pub blocked_state_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and next action, never a generic message.
    pub banner_names_exact_reason_and_next_action: bool,
    /// The support / export packet reconstructs promotion and rollback truth.
    pub support_export_reconstructs_history_truth: bool,
    /// No consumer invents a second promotion or rollback grammar.
    pub no_surface_invents_second_history_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel promotion / rollback vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryConsumerProjection {
    /// Release-center, update-center, CLI, admin, and support consumers all consume the
    /// shared primitive.
    pub history_surfaces_consume_shared_primitive: bool,
    /// The history-posture resolver reads a single canonical source.
    pub history_resolver_reads_single_source: bool,
    /// The promotion-step view reads a single canonical source.
    pub promotion_view_reads_single_source: bool,
    /// The rollback/revocation view reads a single canonical source.
    pub rollback_view_reads_single_source: bool,
    /// Support / export reads a single canonical history source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the history primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting history audit.
    pub history_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ReleaseHistoryPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReleaseHistoryPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// History rows.
    pub history_rows: Vec<M5ReleaseHistoryRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ReleaseHistoryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ReleaseHistoryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReleaseHistoryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReleaseHistoryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ReleaseHistoryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 release-history-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReleaseHistoryPrimitivePacket {
    /// Record kind; must equal [`M5_RELEASE_HISTORY_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RELEASE_HISTORY_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// History rows.
    pub history_rows: Vec<M5ReleaseHistoryRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ReleaseHistoryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ReleaseHistoryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReleaseHistoryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReleaseHistoryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ReleaseHistoryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ReleaseHistoryPrimitivePacket {
    /// Builds an M5 release-history-primitive packet from stable-lane input.
    pub fn new(input: M5ReleaseHistoryPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_RELEASE_HISTORY_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_RELEASE_HISTORY_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            history_rows: input.history_rows,
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

    /// Validates the M5 release-history-primitive invariants.
    pub fn validate(&self) -> Vec<M5ReleaseHistoryPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RELEASE_HISTORY_PRIMITIVE_RECORD_KIND {
            violations.push(M5ReleaseHistoryPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RELEASE_HISTORY_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5ReleaseHistoryPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReleaseHistoryPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_history_rows(self, &mut violations);
        validate_history_coverage(self, &mut violations);
        validate_rollback_not_generic(self, &mut violations);
        validate_emergency_visible_in_history(self, &mut violations);
        validate_break_glass_attribution_preserved(self, &mut violations);
        validate_reconstructable_from_timeline(self, &mut violations);
        validate_reversible_window_preserved(self, &mut violations);
        validate_history_banner_self_contained(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 release-history primitive packet serializes"),
        ) {
            violations.push(M5ReleaseHistoryPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 release-history primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per history consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,event_kinds,history_postures,promotion_stage_states,blast_radii,revocation_scopes,break_glass_postures,block_reasons,next_actions,export_fields,example_count\n",
        );
        for row in &self.history_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.event_kinds, |v| v.as_str()),
                join_tokens(&row.history_postures, |v| v.as_str()),
                join_tokens(&row.promotion_stage_states, |v| v.as_str()),
                join_tokens(&row.blast_radii, |v| v.as_str()),
                join_tokens(&row.revocation_scopes, |v| v.as_str()),
                join_tokens(&row.break_glass_postures, |v| v.as_str()),
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
            .history_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Promotion-Timeline-Step and Rollback/Revocation-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- History consumers: {} ({} stable)\n",
            self.history_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- History postures: {}\n",
            self.vocabulary_set.history_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Blast radii: {}\n",
            self.vocabulary_set.blast_radii.join(", ")
        ));
        out.push_str(&format!(
            "- Revocation scopes: {}\n",
            self.vocabulary_set.revocation_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Break-glass postures: {}\n",
            self.vocabulary_set.break_glass_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## History consumers\n\n");
        for row in &self.history_rows {
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
                let banner = match &case.resolved.history_banner {
                    Some(banner) => banner.reason.as_str(),
                    None => "recorded",
                };
                out.push_str(&format!(
                    "    - `{}` ({}, digests {}) → `{}` (banner `{}`)\n",
                    case.resolved.event_identity_repr,
                    case.resolved.event_kind.as_str(),
                    case.resolved.digest_ref_count,
                    case.resolved.history_posture.as_str(),
                    banner
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 release-history-primitive export.
#[derive(Debug)]
pub enum M5ReleaseHistoryPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ReleaseHistoryPrimitiveViolation>),
}

impl fmt::Display for M5ReleaseHistoryPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 release-history primitive export parse failed: {error}"
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
                    "m5 release-history primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ReleaseHistoryPrimitiveArtifactError {}

/// Validation failures emitted by [`M5ReleaseHistoryPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReleaseHistoryPrimitiveViolation {
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
    /// A required history consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A history row is incomplete.
    HistoryRowIncomplete,
    /// A history row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A history row declares no event kinds.
    EventKindMissing,
    /// A history row declares no history postures.
    HistoryPostureMissing,
    /// A history row declares no blast radii.
    BlastRadiusMissing,
    /// A history row declares no revocation scopes.
    RevocationScopeMissing,
    /// A history row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A history row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A history row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A history row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A history row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A history claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves both a recorded and a blocked event.
    HistoryCoverageUnproven,
    /// No worked resolution proves a rollback / revocation that is not a generic status
    /// change.
    RollbackNotGenericUnproven,
    /// No worked resolution proves an emergency action recorded and visible in history.
    EmergencyVisibleInHistoryUnproven,
    /// No worked resolution both preserves an attributed break-glass and blocks an
    /// unattributed break-glass.
    BreakGlassAttributionUnproven,
    /// No worked resolution proves an event reconstructable from the timeline alone.
    ReconstructableFromTimelineUnproven,
    /// No worked resolution proves both a reversible and an irreversible promotion.
    ReversibleWindowUnproven,
    /// No worked resolution proves a blocked event with a self-contained banner.
    BlockedBannerSelfContainedUnproven,
    /// A history row violates a hard invariant.
    HistoryInvariantViolated,
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

impl M5ReleaseHistoryPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::HistoryRowIncomplete => "history_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::EventKindMissing => "event_kind_missing",
            Self::HistoryPostureMissing => "history_posture_missing",
            Self::BlastRadiusMissing => "blast_radius_missing",
            Self::RevocationScopeMissing => "revocation_scope_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::HistoryCoverageUnproven => "history_coverage_unproven",
            Self::RollbackNotGenericUnproven => "rollback_not_generic_unproven",
            Self::EmergencyVisibleInHistoryUnproven => "emergency_visible_in_history_unproven",
            Self::BreakGlassAttributionUnproven => "break_glass_attribution_unproven",
            Self::ReconstructableFromTimelineUnproven => "reconstructable_from_timeline_unproven",
            Self::ReversibleWindowUnproven => "reversible_window_unproven",
            Self::BlockedBannerSelfContainedUnproven => "blocked_banner_self_contained_unproven",
            Self::HistoryInvariantViolated => "history_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 release-history-primitive export.
pub fn current_stable_m5_release_history_primitive_export(
) -> Result<M5ReleaseHistoryPrimitivePacket, M5ReleaseHistoryPrimitiveArtifactError> {
    let packet: M5ReleaseHistoryPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-promotion-timeline-and-rollback-revocation-proof/support_export.json"
    )))
    .map_err(M5ReleaseHistoryPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ReleaseHistoryPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RELEASE_HISTORY_STEP_SCHEMA_REF,
        M5_RELEASE_HISTORY_ROW_SCHEMA_REF,
        M5_RELEASE_HISTORY_DOC_REF,
        M5_RELEASE_HISTORY_COMPONENT_MATRIX_REF,
        M5_RELEASE_HISTORY_OBJECT_MODEL_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ReleaseHistoryPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ReleaseHistoryPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_history_rows(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let present: BTreeSet<M5ReleaseHistoryConsumerSurface> = packet
        .history_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5ReleaseHistoryConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ReleaseHistoryPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.history_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.promotion_stage_states.is_empty()
            || row.rollout_rings.is_empty()
            || row.reversible_window_states.is_empty()
            || row.node_targetings.is_empty()
            || row.break_glass_postures.is_empty()
            || row.block_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5ReleaseHistoryPrimitiveViolation::HistoryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.event_kinds.is_empty() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::EventKindMissing);
        }
        if row.history_postures.is_empty() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::HistoryPostureMissing);
        }
        if row.blast_radii.is_empty() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::BlastRadiusMissing);
        }
        if row.revocation_scopes.is_empty() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::RevocationScopeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ReleaseHistoryPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ReleaseHistoryPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ReleaseHistoryPrimitiveViolation::HistoryInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must record a promotion, at least
/// one must record a rollback / revocation, and at least one must be blocked — the
/// acceptance-criterion example that a reviewer can tell a recorded event from a blocked
/// one from the timeline itself.
fn validate_history_coverage(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let cases: Vec<&M5ReleaseHistoryResolutionCase> = all_cases(packet);
    let has_promotion = cases
        .iter()
        .any(|case| case.resolved.event_kind.is_promotion() && case.resolved.is_recorded);
    let has_rollback = cases
        .iter()
        .any(|case| case.resolved.event_kind.is_rollback() && case.resolved.is_recorded);
    let has_blocked = cases.iter().any(|case| case.resolved.is_blocked);
    if !(has_promotion && has_rollback && has_blocked) {
        violations.push(M5ReleaseHistoryPrimitiveViolation::HistoryCoverageUnproven);
    }
}

/// At least one worked resolution must prove a rollback / revocation whose blast radius
/// is wider than a single artifact and whose node targeting is an explicit partial
/// scope over a non-empty affected node set — the acceptance-criterion example that a
/// rollback no longer reads like a generic status change.
fn validate_rollback_not_generic(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let proven = all_cases(packet).iter().any(|case| {
        case.resolved.rollback_view.as_ref().is_some_and(|view| {
            !matches!(view.blast_radius, M5RollbackBlastRadius::SingleArtifact)
                && view.node_targeting.is_partial()
                && !view.affected_node_set.is_empty()
        })
    });
    if !proven {
        violations.push(M5ReleaseHistoryPrimitiveViolation::RollbackNotGenericUnproven);
    }
}

/// At least one worked resolution must record an emergency break-glass action that stays
/// visible in the history model — the acceptance-criterion example that emergency
/// operations do not disappear into CI-only metadata.
fn validate_emergency_visible_in_history(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let proven = all_cases(packet).iter().any(|case| {
        matches!(
            case.resolved.history_posture,
            M5ReleaseHistoryPosture::EmergencyBreakGlassRecorded
        ) && case.resolved.emergency_visible_in_history
            && case.resolved.is_recorded
    });
    if !proven {
        violations.push(M5ReleaseHistoryPrimitiveViolation::EmergencyVisibleInHistoryUnproven);
    }
}

/// At least one worked resolution must preserve an attributed break-glass action and at
/// least one must block an unattributed break-glass action — the implementation
/// requirement that break-glass attribution is preserved.
fn validate_break_glass_attribution_preserved(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let cases = all_cases(packet);
    let has_attributed = cases.iter().any(|case| {
        case.resolved.is_emergency
            && case.resolved.is_recorded
            && !case.resolved.approving_actors.is_empty()
    });
    let has_blocked_unattributed = cases.iter().any(|case| {
        matches!(
            case.resolved.history_posture,
            M5ReleaseHistoryPosture::HistoryBlockedUnattributed
        )
    });
    if !(has_attributed && has_blocked_unattributed) {
        violations.push(M5ReleaseHistoryPrimitiveViolation::BreakGlassAttributionUnproven);
    }
}

/// At least one worked resolution must be reconstructable from the timeline alone
/// (actors, digest joins, evidence refs, and a time) and recorded — the
/// acceptance-criterion example that a reviewer can reconstruct what changed and why.
fn validate_reconstructable_from_timeline(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let proven = all_cases(packet).iter().any(|case| {
        case.resolved.reconstruction.is_reconstructable
            && case.resolved.is_recorded
            && case.resolved.history_banner.is_none()
    });
    if !proven {
        violations.push(M5ReleaseHistoryPrimitiveViolation::ReconstructableFromTimelineUnproven);
    }
}

/// At least one worked resolution must record a reversible promotion and at least one
/// must record an irreversible promotion — the implementation requirement that the
/// reversible window stays explicit.
fn validate_reversible_window_preserved(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let cases = all_cases(packet);
    let has_reversible = cases.iter().any(|case| {
        matches!(
            case.resolved.history_posture,
            M5ReleaseHistoryPosture::PromotionRecordedReversible
        )
    });
    let has_irreversible = cases.iter().any(|case| {
        matches!(
            case.resolved.history_posture,
            M5ReleaseHistoryPosture::PromotionRecordedIrreversible
        )
    });
    if !(has_reversible && has_irreversible) {
        violations.push(M5ReleaseHistoryPrimitiveViolation::ReversibleWindowUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a blocked event whose
/// banner carries a specific reason, a next action, the bound event, and its digest —
/// the acceptance-criterion example that a blocked state is understood from the banner
/// rather than CI-only metadata.
fn validate_history_banner_self_contained(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let proven = all_cases(packet).iter().any(|case| {
        case.resolved.is_blocked
            && case.resolved.history_banner.as_ref().is_some_and(|banner| {
                !banner.headline.trim().is_empty() && !banner.bound_digest_repr.trim().is_empty()
            })
    });
    if !proven {
        violations.push(M5ReleaseHistoryPrimitiveViolation::BlockedBannerSelfContainedUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_history_truth,
        review.reconstructable_from_timeline,
        review.rollback_never_reads_as_generic_status,
        review.blast_radius_and_unaffected_nodes_explicit,
        review.break_glass_attribution_and_partial_scope_preserved,
        review.emergency_stays_visible_in_history_model,
        review.artifact_graph_consistency_preserved,
        review.blocked_state_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_next_action,
        review.support_export_reconstructs_history_truth,
        review.no_surface_invents_second_history_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ReleaseHistoryPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.history_surfaces_consume_shared_primitive,
        projection.history_resolver_reads_single_source,
        projection.promotion_view_reads_single_source,
        projection.rollback_view_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ReleaseHistoryPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ReleaseHistoryPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ReleaseHistoryPrimitivePacket,
    violations: &mut Vec<M5ReleaseHistoryPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.history_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ReleaseHistoryPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Flattens every worked resolution case across every row.
fn all_cases(packet: &M5ReleaseHistoryPrimitivePacket) -> Vec<&M5ReleaseHistoryResolutionCase> {
    packet
        .history_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect()
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
