//! Two reusable M5 protected-path governance components — the DRI-registry row and the
//! merge-readiness strip — so a user can tell *who* is accountable for a governed service or path (the
//! primary and backup DRI role aliases, the escalation alias, the support forum, and the benchmark or
//! compatibility owner where relevant), *how fresh* that registry entry is, and — for the merge gate —
//! whether a change is a local estimate or provider-authoritative, which queue or branch it targets,
//! how many blockers remain, what the required next action is, and how to export the readiness packet,
//! before they hand off, escalate, or merge a governed change.
//!
//! Aureline's frozen protected-path governance component matrix
//! ([`crate::freeze_the_m5_protected_path_governance_component_matrix`]) names the DRI-registry row and
//! the merge-readiness strip as two governed component families and freezes their controlled
//! vocabulary — the one governance-state lexicon ([`M5GovernanceStateVocab`]): `advisory`,
//! `authoritative`, `covered`, `backup_missing`, `waived`, `expired`, `stale`,
//! `provider_authoritative`, and `local_estimate`. This module *implements* that contract as two
//! co-equal component vectors — a full DRI-registry row and a merge-readiness strip — that reuse the one
//! frozen lexicon and share one authority-locus resolver so a local estimate or a CI-only signal can
//! never masquerade as the provider's authoritative state, an advisory owner hint (guessed from the
//! last interacting team) can never read as an authoritative owner, and a change can never appear
//! `mergeable here` when it is only locally reviewable or blocked by provider-authoritative controls.
//!
//! The module has two derived resolvers:
//!
//! * [`resolve_authority_locus`] — takes an authority-locus source and derives the exact locus posture
//!   (provider-authoritative, a local estimate, CI-only, not evaluated here, or stale relative to
//!   base/head), whether the posture is the provider's authoritative state, whether it was evaluated
//!   here at all, whether it is stale, and which frozen governance-state token it maps to — so a local
//!   estimate or a CI-only signal can never read as provider-authoritative, and a not-evaluated-here
//!   signal can never read as evaluated. Both the DRI-registry row and the merge-readiness strip use it,
//!   so their local-versus-provider parity stays one truth. This is the AC pinning the merge-readiness
//!   honesty: a change never widens from a local estimate to provider mergeability without provider
//!   confirmation.
//! * [`resolve_owner_source`] — takes an owner-source signal and derives the exact owner-source posture
//!   (codeowners-authoritative, registry-declared, an advisory heuristic, or unresolved), whether the
//!   owner is authoritatively known, whether the source is only an advisory heuristic, and the notes it
//!   must carry — so an owner guessed from the last interacting team can never read as an authoritative
//!   owner, and owner and escalation truth stay aligned wherever a governed change is listed.
//!
//! A single controls packet — [`DriRegistryMergeReadinessControlsPacket`] — binds one vector of
//! DRI-registry rows and one vector of merge-readiness strips to the same authority-locus,
//! owner-source, and merge-target vocabulary, so primary/backup role aliases, support-or-escalation
//! path, queue-or-branch target truth, blocker counts, export-packet actions, and no-silent-mergeability
//! widening stay explicit across the review-workspace, release-candidate, governance, shiproom, CLI, and
//! support-export consumers.
//!
//! The governance component ([`M5GovernanceComponent`]), governance-state vocabulary
//! ([`M5GovernanceStateVocab`]), downgrade trigger
//! ([`M5GovernanceComponentDowngradeTrigger`]), rollback posture
//! ([`M5GovernanceComponentRollbackPosture`]), and consumer surface
//! ([`M5GovernanceComponentConsumerSurface`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: the authority-locus source, the derived locus posture, the owner-source signal, the
//! derived owner-source posture, the support-forum kind, the escalation-continuity state, the
//! registry-freshness state, the merge-target kind, the required-next-action kind, and the bounded row
//! and strip actions. No M5 governed surface invents a second DRI-registry or merge-readiness grammar.
//!
//! Raw CODEOWNERS bodies, raw provider payloads, raw ruleset definitions, personal contact details,
//! credentials, and secrets stay outside the export boundary; every owner, escalation, forum, and gate
//! reference is carried only as an opaque, export-safe role alias or reference.

#[cfg(test)]
mod tests;

// The governance component family, the frozen governance-state lexicon, and the downgrade /
// rollback / consumer vocabularies are frozen once, in the protected-path governance component
// matrix. This lane reuses them verbatim so it never invents a parallel DRI-registry or
// merge-readiness vocabulary.
pub use crate::freeze_the_m5_protected_path_governance_component_matrix::{
    M5GovernanceComponent, M5GovernanceComponentConsumerSurface,
    M5GovernanceComponentDowngradeTrigger, M5GovernanceComponentRollbackPosture,
    M5GovernanceStateVocab, M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF,
    M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DriRegistryMergeReadinessControlsPacket`].
pub const DRI_REGISTRY_MERGE_READINESS_CONTROLS_RECORD_KIND: &str =
    "implement_dri_registry_rows_and_merge_readiness_strips_with_primary_backup_role_aliases_support_or_escalation_path_queue_or_branch_target_truth_blocker_counts_export_packet_actions_and_no_silent_mergeability_widening";

/// Schema version for M5 DRI-registry-row / merge-readiness-strip control records.
pub const DRI_REGISTRY_MERGE_READINESS_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const DRI_REGISTRY_MERGE_READINESS_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-dri-registry-merge-readiness-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const DRI_REGISTRY_MERGE_READINESS_CONTROLS_DOC_REF: &str =
    "docs/review/m5/implement_dri_registry_rows_and_merge_readiness_strips.md";

/// Repo-relative path of the protected fixture directory.
pub const DRI_REGISTRY_MERGE_READINESS_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-dri-registry-merge-readiness-controls";

/// Repo-relative path of the checked support-export artifact.
pub const DRI_REGISTRY_MERGE_READINESS_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-dri-registry-merge-readiness-controls-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DRI_REGISTRY_MERGE_READINESS_CONTROLS_SUMMARY_REF: &str =
    "artifacts/release/m5-dri-registry-merge-readiness-controls-proof/summary.md";

// ---- shared authority-locus vocabulary -----------------------------------

/// The source an authority-locus signal comes from, before it is resolved into a posture.
///
/// This is the honest input to [`resolve_authority_locus`]: it names whether a governed signal (a DRI
/// registry entry's freshness, or a merge-readiness state) is provider-authoritative, provider-reported,
/// only a local heuristic estimate, only CI-reported, was not evaluated here at all, or is stale against
/// the current base/head — so a local estimate or a CI-only signal can never be asserted as the
/// provider's authoritative state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityLocusSource {
    /// The provider authoritatively enforces or reports the state.
    ProviderAuthoritativeState,
    /// The provider reported the state authoritatively.
    ProviderReportedState,
    /// The signal is only a local heuristic estimate.
    LocalHeuristicEstimate,
    /// The signal was reported only by CI, not by the provider gate.
    CiReportedOnly,
    /// The signal was not evaluated on this build.
    NotEvaluatedHere,
    /// The evaluation is stale relative to the current base/head.
    StaleAgainstBaseHead,
}

impl AuthorityLocusSource {
    /// Every authority-locus source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderAuthoritativeState,
        Self::ProviderReportedState,
        Self::LocalHeuristicEstimate,
        Self::CiReportedOnly,
        Self::NotEvaluatedHere,
        Self::StaleAgainstBaseHead,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritativeState => "provider_authoritative_state",
            Self::ProviderReportedState => "provider_reported_state",
            Self::LocalHeuristicEstimate => "local_heuristic_estimate",
            Self::CiReportedOnly => "ci_reported_only",
            Self::NotEvaluatedHere => "not_evaluated_here",
            Self::StaleAgainstBaseHead => "stale_against_base_head",
        }
    }
}

/// Derived authority-locus posture a DRI row or merge-readiness strip may present.
///
/// This is the AC-pinned local-versus-provider parity axis: the posture is derived from the frozen
/// locus source, never asserted, so a user can tell whether a signal is a local estimate, CI-only,
/// provider-authoritative, not evaluated here, or stale relative to base/head — without opening raw
/// payloads. A change never widens from a local estimate to provider mergeability without provider
/// confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityLocusPosture {
    /// The provider authoritatively enforces or reports the signal.
    ProviderAuthoritative,
    /// The signal is only a local estimate.
    LocalEstimate,
    /// The signal was reported only by CI.
    CiOnly,
    /// The signal was not evaluated on this build.
    NotEvaluatedHere,
    /// The signal is stale relative to the current base/head.
    StaleRelativeToHead,
}

impl AuthorityLocusPosture {
    /// Every authority-locus posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderAuthoritative,
        Self::LocalEstimate,
        Self::CiOnly,
        Self::NotEvaluatedHere,
        Self::StaleRelativeToHead,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::LocalEstimate => "local_estimate",
            Self::CiOnly => "ci_only",
            Self::NotEvaluatedHere => "not_evaluated_here",
            Self::StaleRelativeToHead => "stale_relative_to_head",
        }
    }

    /// True only when the provider authoritatively backs the signal.
    pub const fn is_provider_authoritative(self) -> bool {
        matches!(self, Self::ProviderAuthoritative)
    }

    /// True when the signal is only a local estimate.
    pub const fn is_local_estimate(self) -> bool {
        matches!(self, Self::LocalEstimate)
    }

    /// True when the signal was reported only by CI.
    pub const fn is_ci_only(self) -> bool {
        matches!(self, Self::CiOnly)
    }

    /// True when the signal was evaluated on this build at all (stale counts as evaluated-but-old).
    pub const fn is_evaluated_here(self) -> bool {
        !matches!(self, Self::NotEvaluatedHere)
    }

    /// True when the signal is stale relative to the current base/head.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StaleRelativeToHead)
    }

    /// The frozen governance-state token this posture must render under, if the frozen lexicon names
    /// one. `ci_only` and `not_evaluated_here` are honest states the frozen lexicon does not name, so
    /// they carry no governance token and must never borrow another state's label.
    pub const fn governance_vocab(self) -> Option<M5GovernanceStateVocab> {
        match self {
            Self::ProviderAuthoritative => Some(M5GovernanceStateVocab::ProviderAuthoritative),
            Self::LocalEstimate => Some(M5GovernanceStateVocab::LocalEstimate),
            Self::StaleRelativeToHead => Some(M5GovernanceStateVocab::Stale),
            Self::CiOnly | Self::NotEvaluatedHere => None,
        }
    }
}

/// Locus disclosures a component must carry, derived from the authority-locus source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorityLocusDisclosure {
    /// The derived locus posture this component may present.
    pub posture: AuthorityLocusPosture,
    /// Whether the provider authoritatively backs the signal.
    pub is_provider_authoritative: bool,
    /// Whether the signal is only a local estimate.
    pub is_local_estimate: bool,
    /// Whether the signal was reported only by CI.
    pub is_ci_only: bool,
    /// Whether the signal was evaluated on this build at all.
    pub is_evaluated_here: bool,
    /// Whether the signal is stale relative to the current base/head.
    pub is_stale: bool,
    /// Whether the component must carry an explicit local-estimate note.
    pub needs_local_estimate_note: bool,
    /// Whether the component must carry an explicit CI-only note.
    pub needs_ci_only_note: bool,
    /// Whether the component must carry an explicit not-evaluated-here note.
    pub needs_not_evaluated_note: bool,
    /// Whether the component must carry an explicit stale note.
    pub needs_stale_note: bool,
    /// The frozen governance-state token this posture must render under, if any.
    pub governance_vocab: Option<M5GovernanceStateVocab>,
}

/// Resolves the authority-locus posture a DRI row or merge-readiness strip may present.
///
/// A `provider_authoritative_state` or `provider_reported_state` source is provider-authoritative; a
/// `local_heuristic_estimate` source is a local estimate; a `ci_reported_only` source is CI-only; a
/// `not_evaluated_here` source is not-evaluated-here; and a `stale_against_base_head` source is stale
/// relative to base/head — so a local estimate or a CI-only signal can never read as the provider's
/// authoritative state.
pub fn resolve_authority_locus(source: AuthorityLocusSource) -> AuthorityLocusDisclosure {
    use AuthorityLocusPosture as Posture;
    use AuthorityLocusSource as Src;

    let posture = match source {
        Src::ProviderAuthoritativeState | Src::ProviderReportedState => {
            Posture::ProviderAuthoritative
        }
        Src::LocalHeuristicEstimate => Posture::LocalEstimate,
        Src::CiReportedOnly => Posture::CiOnly,
        Src::NotEvaluatedHere => Posture::NotEvaluatedHere,
        Src::StaleAgainstBaseHead => Posture::StaleRelativeToHead,
    };

    AuthorityLocusDisclosure {
        posture,
        is_provider_authoritative: posture.is_provider_authoritative(),
        is_local_estimate: posture.is_local_estimate(),
        is_ci_only: posture.is_ci_only(),
        is_evaluated_here: posture.is_evaluated_here(),
        is_stale: posture.is_stale(),
        needs_local_estimate_note: posture.is_local_estimate(),
        needs_ci_only_note: posture.is_ci_only(),
        needs_not_evaluated_note: !posture.is_evaluated_here(),
        needs_stale_note: posture.is_stale(),
        governance_vocab: posture.governance_vocab(),
    }
}

// ---- owner-source vocabulary ---------------------------------------------

/// The signal an owner-source assertion comes from, before it is resolved into a posture.
///
/// This is the honest input to [`resolve_owner_source`]: it names whether the DRI owner is known from a
/// forge CODEOWNERS rule, a provider team assignment, a repository manifest, the DRI registry itself, a
/// last-interacting-team heuristic, or is unresolved — so an owner guessed from the last interacting
/// team can never be asserted as an authoritative owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerSourceSignal {
    /// The owner is known from a forge CODEOWNERS rule.
    ForgeCodeownersRule,
    /// The owner is known from a provider team assignment.
    ProviderTeamAssignment,
    /// The owner is declared in the repository manifest.
    RepositoryManifestDeclared,
    /// The owner is declared in the DRI registry.
    RegistryDeclared,
    /// The owner is guessed from the last interacting team.
    LastInteractingTeamHeuristic,
    /// The owner is unresolved.
    OwnerUnresolved,
}

impl OwnerSourceSignal {
    /// Every owner-source signal, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ForgeCodeownersRule,
        Self::ProviderTeamAssignment,
        Self::RepositoryManifestDeclared,
        Self::RegistryDeclared,
        Self::LastInteractingTeamHeuristic,
        Self::OwnerUnresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForgeCodeownersRule => "forge_codeowners_rule",
            Self::ProviderTeamAssignment => "provider_team_assignment",
            Self::RepositoryManifestDeclared => "repository_manifest_declared",
            Self::RegistryDeclared => "registry_declared",
            Self::LastInteractingTeamHeuristic => "last_interacting_team_heuristic",
            Self::OwnerUnresolved => "owner_unresolved",
        }
    }
}

/// Derived owner-source posture a DRI-registry row may present.
///
/// This is the AC-pinned owner-honesty axis: only [`OwnerSourcePosture::CodeownersAuthoritative`] is an
/// authoritative owner; an advisory heuristic (a guess from the last interacting team) can never read as
/// an authoritative owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerSourcePosture {
    /// The owner is authoritative, from CODEOWNERS or a provider team assignment.
    CodeownersAuthoritative,
    /// The owner is declared in the repository manifest or DRI registry.
    RegistryDeclared,
    /// The owner is an advisory heuristic (guessed from the last interacting team).
    AdvisoryHeuristic,
    /// The owner is unresolved.
    Unresolved,
}

impl OwnerSourcePosture {
    /// Every owner-source posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CodeownersAuthoritative,
        Self::RegistryDeclared,
        Self::AdvisoryHeuristic,
        Self::Unresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeownersAuthoritative => "codeowners_authoritative",
            Self::RegistryDeclared => "registry_declared",
            Self::AdvisoryHeuristic => "advisory_heuristic",
            Self::Unresolved => "unresolved",
        }
    }

    /// True only when the owner is authoritative (CODEOWNERS or provider team assignment).
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::CodeownersAuthoritative)
    }

    /// True when the owner is declared in a manifest or the DRI registry.
    pub const fn is_declared(self) -> bool {
        matches!(self, Self::RegistryDeclared)
    }

    /// True when the owner is an advisory heuristic (guessed from the last interacting team).
    pub const fn is_advisory(self) -> bool {
        matches!(self, Self::AdvisoryHeuristic)
    }

    /// True when the owner is resolved at all.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::Unresolved)
    }
}

/// Owner-source disclosures a DRI-registry row must carry, derived from the owner-source signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnerSourceDisclosure {
    /// The derived owner-source posture this row may present.
    pub posture: OwnerSourcePosture,
    /// Whether the owner is authoritative.
    pub is_authoritative: bool,
    /// Whether the owner is declared in a manifest or the DRI registry.
    pub is_declared: bool,
    /// Whether the owner is an advisory heuristic.
    pub is_advisory: bool,
    /// Whether the owner is resolved at all.
    pub is_resolved: bool,
    /// Whether the row must carry an explicit advisory-owner note.
    pub needs_advisory_note: bool,
    /// Whether the row must carry an explicit unresolved-owner note.
    pub needs_unresolved_note: bool,
}

/// Resolves the owner-source posture a DRI-registry row may present.
///
/// A `forge_codeowners_rule` or `provider_team_assignment` signal is authoritative; a
/// `repository_manifest_declared` or `registry_declared` signal is registry-declared; a
/// `last_interacting_team_heuristic` signal is an advisory heuristic; and an `owner_unresolved` signal
/// is unresolved — so an owner guessed from the last interacting team can never read as an authoritative
/// owner.
pub fn resolve_owner_source(signal: OwnerSourceSignal) -> OwnerSourceDisclosure {
    use OwnerSourcePosture as Posture;
    use OwnerSourceSignal as Src;

    let posture = match signal {
        Src::ForgeCodeownersRule | Src::ProviderTeamAssignment => Posture::CodeownersAuthoritative,
        Src::RepositoryManifestDeclared | Src::RegistryDeclared => Posture::RegistryDeclared,
        Src::LastInteractingTeamHeuristic => Posture::AdvisoryHeuristic,
        Src::OwnerUnresolved => Posture::Unresolved,
    };

    OwnerSourceDisclosure {
        posture,
        is_authoritative: posture.is_authoritative(),
        is_declared: posture.is_declared(),
        is_advisory: posture.is_advisory(),
        is_resolved: posture.is_resolved(),
        needs_advisory_note: posture.is_advisory(),
        needs_unresolved_note: !posture.is_resolved(),
    }
}

// ---- dri-registry-row-specific vocabulary --------------------------------

/// The kind of support forum a DRI-registry row names, so a governed change always names where to reach
/// the accountable owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportForumKind {
    /// A chat / Slack channel.
    SlackChannel,
    /// A discussion thread.
    DiscussionThread,
    /// A mailing list.
    MailingList,
    /// A ticket queue.
    TicketQueue,
    /// No support forum is bound (the row names that it routes nowhere).
    NoForum,
}

impl SupportForumKind {
    /// Every support-forum kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SlackChannel,
        Self::DiscussionThread,
        Self::MailingList,
        Self::TicketQueue,
        Self::NoForum,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SlackChannel => "slack_channel",
            Self::DiscussionThread => "discussion_thread",
            Self::MailingList => "mailing_list",
            Self::TicketQueue => "ticket_queue",
            Self::NoForum => "no_forum",
        }
    }

    /// True when this kind names a resolvable support-forum reference.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoForum)
    }
}

/// The escalation-path continuity a DRI-registry row names, so a governed change never hides a broken
/// escalation handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationContinuityState {
    /// The escalation path is continuous to an accountable owner.
    ContinuousToOwner,
    /// The escalation path is degraded but a fallback exists.
    DegradedFallback,
    /// The escalation path is broken with no fallback.
    BrokenNoFallback,
    /// No escalation path is configured.
    NotConfigured,
}

impl EscalationContinuityState {
    /// Every escalation-continuity state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ContinuousToOwner,
        Self::DegradedFallback,
        Self::BrokenNoFallback,
        Self::NotConfigured,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuousToOwner => "continuous_to_owner",
            Self::DegradedFallback => "degraded_fallback",
            Self::BrokenNoFallback => "broken_no_fallback",
            Self::NotConfigured => "not_configured",
        }
    }

    /// True only when the escalation path is continuous to an accountable owner.
    pub const fn is_continuous(self) -> bool {
        matches!(self, Self::ContinuousToOwner)
    }
}

/// The freshness of a DRI-registry row's entry, so a stale ownership hint never reads as current truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryFreshnessState {
    /// The entry is currently verified.
    CurrentlyVerified,
    /// The entry is due for a refresh.
    RefreshDue,
    /// The entry is stale and superseded.
    StaleSuperseded,
    /// The entry has never been verified.
    NeverVerified,
    /// The freshness is unknown.
    UnknownFreshness,
}

impl RegistryFreshnessState {
    /// Every registry-freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CurrentlyVerified,
        Self::RefreshDue,
        Self::StaleSuperseded,
        Self::NeverVerified,
        Self::UnknownFreshness,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentlyVerified => "currently_verified",
            Self::RefreshDue => "refresh_due",
            Self::StaleSuperseded => "stale_superseded",
            Self::NeverVerified => "never_verified",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }
}

/// One keyboard-complete default action a DRI-registry row offers.
///
/// `OpenSupportForum`, `InspectOwnerSource`, and `ReviewEscalationPath` are always offered so the
/// support forum, the owner source, and the escalation path stay inspectable before a user hands off a
/// governed change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriRegistryRowAction {
    /// Open the support forum (always available).
    OpenSupportForum,
    /// Inspect the owner source (always available).
    InspectOwnerSource,
    /// Review the escalation path (always available).
    ReviewEscalationPath,
    /// Inspect the registry freshness.
    InspectFreshness,
    /// Compare the owner history.
    CompareOwnerHistory,
    /// Copy the export-safe registry digest.
    CopyRegistryDigest,
}

impl DriRegistryRowAction {
    /// Every DRI-registry-row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenSupportForum,
        Self::InspectOwnerSource,
        Self::ReviewEscalationPath,
        Self::InspectFreshness,
        Self::CompareOwnerHistory,
        Self::CopyRegistryDigest,
    ];

    /// The default actions every keyboard-complete DRI-registry row must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenSupportForum,
        Self::InspectOwnerSource,
        Self::ReviewEscalationPath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenSupportForum => "open_support_forum",
            Self::InspectOwnerSource => "inspect_owner_source",
            Self::ReviewEscalationPath => "review_escalation_path",
            Self::InspectFreshness => "inspect_freshness",
            Self::CompareOwnerHistory => "compare_owner_history",
            Self::CopyRegistryDigest => "copy_registry_digest",
        }
    }
}

// ---- merge-readiness-strip-specific vocabulary ---------------------------

/// The target a merge-readiness strip names, so a change never hides whether it merges through a queue
/// or directly into a branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeTargetKind {
    /// A merge queue.
    MergeQueue,
    /// A target branch (direct merge).
    TargetBranch,
    /// A stacked branch.
    StackedBranch,
    /// A protected branch.
    ProtectedBranch,
    /// No target is bound.
    NoTarget,
}

impl MergeTargetKind {
    /// Every merge-target kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MergeQueue,
        Self::TargetBranch,
        Self::StackedBranch,
        Self::ProtectedBranch,
        Self::NoTarget,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MergeQueue => "merge_queue",
            Self::TargetBranch => "target_branch",
            Self::StackedBranch => "stacked_branch",
            Self::ProtectedBranch => "protected_branch",
            Self::NoTarget => "no_target",
        }
    }

    /// True when the target is a merge queue.
    pub const fn is_queue(self) -> bool {
        matches!(self, Self::MergeQueue)
    }

    /// True when the target is a branch (direct, stacked, or protected).
    pub const fn is_branch(self) -> bool {
        matches!(
            self,
            Self::TargetBranch | Self::StackedBranch | Self::ProtectedBranch
        )
    }
}

/// The required next action a merge-readiness strip names, so a blocked change never hides what unblocks
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredNextActionKind {
    /// Resolve the outstanding blockers.
    ResolveBlockers,
    /// Request a provider-authoritative evaluation.
    RequestProviderEvaluation,
    /// Refresh a stale base before re-evaluating.
    RefreshStaleBase,
    /// Await the queue position.
    AwaitQueuePosition,
    /// The change is ready to merge.
    ReadyToMerge,
    /// Escalate to the accountable owner.
    EscalateToOwner,
}

impl RequiredNextActionKind {
    /// Every required-next-action kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResolveBlockers,
        Self::RequestProviderEvaluation,
        Self::RefreshStaleBase,
        Self::AwaitQueuePosition,
        Self::ReadyToMerge,
        Self::EscalateToOwner,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveBlockers => "resolve_blockers",
            Self::RequestProviderEvaluation => "request_provider_evaluation",
            Self::RefreshStaleBase => "refresh_stale_base",
            Self::AwaitQueuePosition => "await_queue_position",
            Self::ReadyToMerge => "ready_to_merge",
            Self::EscalateToOwner => "escalate_to_owner",
        }
    }
}

/// One keyboard-complete default action a merge-readiness strip offers.
///
/// `OpenBlockerList`, `InspectMergeTarget`, and `ExportReadinessPacket` are always offered so the
/// blocker list, the merge target, and the export packet stay reachable before a user trusts the merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeReadinessStripAction {
    /// Open the blocker list (always available).
    OpenBlockerList,
    /// Inspect the merge target (always available).
    InspectMergeTarget,
    /// Export the readiness packet (always available).
    ExportReadinessPacket,
    /// Review the provider-authoritative state.
    ReviewProviderState,
    /// Compare the local-versus-provider mergeability.
    CompareLocalProvider,
    /// Copy the export-safe readiness summary.
    CopyReadinessSummary,
}

impl MergeReadinessStripAction {
    /// Every merge-readiness-strip action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenBlockerList,
        Self::InspectMergeTarget,
        Self::ExportReadinessPacket,
        Self::ReviewProviderState,
        Self::CompareLocalProvider,
        Self::CopyReadinessSummary,
    ];

    /// The default actions every keyboard-complete merge-readiness strip must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenBlockerList,
        Self::InspectMergeTarget,
        Self::ExportReadinessPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenBlockerList => "open_blocker_list",
            Self::InspectMergeTarget => "inspect_merge_target",
            Self::ExportReadinessPacket => "export_readiness_packet",
            Self::ReviewProviderState => "review_provider_state",
            Self::CompareLocalProvider => "compare_local_provider",
            Self::CopyReadinessSummary => "copy_readiness_summary",
        }
    }
}

// ---- component structs ---------------------------------------------------

/// A DRI-registry row naming its service/path identity, primary and backup DRI role aliases, escalation
/// alias, support forum, benchmark or compatibility owner where relevant, escalation-path continuity,
/// owner source, and registry freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriRegistryRow {
    /// Frozen component this control implements; must be `dri_registry_row`.
    pub component: M5GovernanceComponent,
    /// Stable row id.
    pub row_id: String,
    /// Service or path identity label; required and non-empty.
    pub service_path_identity_label: String,
    /// Primary DRI role alias; required, non-empty, and never a personal contact detail.
    pub primary_dri_alias: String,
    /// Backup DRI role alias; never a personal contact detail (may be empty when no backup is named).
    pub backup_dri_alias: String,
    /// Escalation role alias; required, non-empty, and never a personal contact detail.
    pub escalation_alias: String,
    /// Support forum kind.
    pub support_forum_kind: SupportForumKind,
    /// Support forum label; required and non-empty.
    pub support_forum_label: String,
    /// Opaque support-forum reference; required when the forum kind resolves.
    pub support_forum_ref: String,
    /// Benchmark owner role alias; optional (where relevant), never a personal contact detail.
    pub benchmark_owner_alias: String,
    /// Compatibility owner role alias; optional (where relevant), never a personal contact detail.
    pub compatibility_owner_alias: String,
    /// Escalation-path continuity state.
    pub escalation_continuity_state: EscalationContinuityState,
    /// Escalation-path label; required and non-empty so the support-or-escalation path stays explicit.
    pub escalation_path_label: String,
    /// Owner-source signal, resolved into the owner-source posture.
    pub owner_source_signal: OwnerSourceSignal,
    /// Derived owner-source posture (must equal the resolved posture).
    pub derived_owner_source: OwnerSourcePosture,
    /// Whether the row claims an authoritative owner (must equal derived truth).
    pub claims_authoritative_owner: bool,
    /// Registry-freshness state.
    pub registry_freshness_state: RegistryFreshnessState,
    /// Freshness label; required and non-empty.
    pub freshness_label: String,
    /// Authority-locus source, resolved into the locus posture.
    pub authority_locus_source: AuthorityLocusSource,
    /// Derived authority-locus posture (must equal the resolved posture).
    pub derived_authority_locus: AuthorityLocusPosture,
    /// Whether the row claims provider-authoritative freshness (must equal derived truth).
    pub claims_provider_authoritative: bool,
    /// Frozen governance-state vocabulary this row renders (must include the derived locus token).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Local-estimate note; required when the freshness is a local estimate.
    pub local_estimate_note: String,
    /// CI-only note; required when the freshness was only CI-reported.
    pub ci_only_note: String,
    /// Not-evaluated-here note; required when the freshness was not evaluated on this build.
    pub not_evaluated_note: String,
    /// Stale note; required when the freshness is stale relative to base/head.
    pub stale_note: String,
    /// Advisory-owner note; required when the owner is only an advisory heuristic.
    pub advisory_owner_note: String,
    /// Unresolved-owner note; required when the owner is unresolved.
    pub unresolved_owner_note: String,
    /// Context note; always required so the row names what to check before handing off.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub row_actions: Vec<DriRegistryRowAction>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its owner or escalation identity. MUST be `false`.
    pub hides_owner_or_escalation_identity: bool,
    /// Hard invariant: never lets an advisory owner read as authoritative. MUST be `false`.
    pub lets_advisory_owner_read_as_authoritative: bool,
    /// Hard invariant: never guesses the owner from the last interacting team. MUST be `false`.
    pub guesses_owner_from_last_interacting_team: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl DriRegistryRow {
    /// Locus disclosures this row must carry, derived from the frozen source.
    pub fn locus_disclosure(&self) -> AuthorityLocusDisclosure {
        resolve_authority_locus(self.authority_locus_source)
    }

    /// Owner-source disclosures this row must carry, derived from the frozen signal.
    pub fn owner_disclosure(&self) -> OwnerSourceDisclosure {
        resolve_owner_source(self.owner_source_signal)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<DriRegistryRowAction> = self.row_actions.iter().copied().collect();
        DriRegistryRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

/// A merge-readiness strip naming its local-estimate-versus-provider-authoritative state, queue/branch
/// target, blocker count, required next action, export-packet action, and mergeability parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeReadinessStrip {
    /// Frozen component this control implements; must be `merge_readiness_strip`.
    pub component: M5GovernanceComponent,
    /// Stable strip id.
    pub strip_id: String,
    /// Change title label; required and non-empty.
    pub change_title_label: String,
    /// Merge-target kind.
    pub merge_target_kind: MergeTargetKind,
    /// Merge-target label; required and non-empty so the queue/branch target stays explicit.
    pub merge_target_label: String,
    /// Blocker count; the number of outstanding blockers.
    pub blocker_count: u32,
    /// Blocker summary label; required and non-empty when the blocker count is non-zero.
    pub blocker_summary_label: String,
    /// Required-next-action kind.
    pub required_next_action_kind: RequiredNextActionKind,
    /// Required-next-action label; always required so the next action stays explicit.
    pub required_next_action_label: String,
    /// Export-packet action label; always required so the export packet action stays explicit.
    pub export_packet_action_label: String,
    /// Authority-locus source, resolved into the locus posture.
    pub authority_locus_source: AuthorityLocusSource,
    /// Derived authority-locus posture (must equal the resolved posture).
    pub derived_authority_locus: AuthorityLocusPosture,
    /// Whether the strip claims provider-authoritative state (must equal derived truth).
    pub claims_provider_authoritative: bool,
    /// Whether the strip claims it was evaluated here (must equal derived truth).
    pub claims_evaluated_here: bool,
    /// Whether the strip claims the change is mergeable here; must never widen from a local estimate or
    /// past an outstanding blocker without provider-authoritative clearance.
    pub claims_mergeable_here: bool,
    /// Mergeability label; always required so the local-versus-provider mergeability stays explicit.
    pub mergeability_label: String,
    /// Export-parity label; always required so export-packet parity stays explicit.
    pub export_parity_label: String,
    /// Frozen governance-state vocabulary this strip renders (must include the derived locus token).
    pub governance_state_vocab: Vec<M5GovernanceStateVocab>,
    /// Local-estimate note; required when the state is a local estimate.
    pub local_estimate_note: String,
    /// CI-only note; required when the state was only CI-reported.
    pub ci_only_note: String,
    /// Not-evaluated-here note; required when the state was not evaluated on this build.
    pub not_evaluated_note: String,
    /// Stale note; required when the state is stale relative to base/head.
    pub stale_note: String,
    /// Context note; always required so the strip names what to check before trusting the merge.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub strip_actions: Vec<MergeReadinessStripAction>,
    /// Downgrade triggers this strip can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must project this strip.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// Rollback posture.
    pub rollback_posture: M5GovernanceComponentRollbackPosture,
    /// Source contract refs consumed by this strip.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its target or blocker count. MUST be `false`.
    pub hides_target_or_blocker_count: bool,
    /// Hard invariant: never lets a local estimate read as provider-authoritative mergeability. MUST be
    /// `false`.
    pub lets_local_estimate_read_as_provider_mergeable: bool,
    /// Hard invariant: never widens a local estimate to provider mergeability without confirmation. MUST
    /// be `false`.
    pub widens_local_estimate_to_provider_mergeability: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl MergeReadinessStrip {
    /// Locus disclosures this strip must carry, derived from the frozen source.
    pub fn locus_disclosure(&self) -> AuthorityLocusDisclosure {
        resolve_authority_locus(self.authority_locus_source)
    }

    /// Whether the strip is provider-cleared to merge here (provider-authoritative and unblocked).
    pub fn is_provider_cleared_to_merge(&self) -> bool {
        self.locus_disclosure().is_provider_authoritative && self.blocker_count == 0
    }

    /// Whether the strip offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<MergeReadinessStripAction> =
            self.strip_actions.iter().copied().collect();
        MergeReadinessStripAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }
}

// ---- review blocks -------------------------------------------------------

/// First-glance DRI-registry / merge-readiness review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriRegistryMergeReadinessReview {
    /// The DRI row names its service and owner identity.
    pub dri_row_shows_service_and_owner_identity: bool,
    /// The DRI row names its escalation and support path.
    pub dri_row_shows_escalation_and_support_path: bool,
    /// The DRI row offers an inspect-owner-source action.
    pub dri_row_offers_inspect_owner_source: bool,
    /// The merge strip names its target and blocker count.
    pub merge_strip_shows_target_and_blocker_count: bool,
    /// The merge strip names its required next action.
    pub merge_strip_shows_required_next_action: bool,
    /// The merge strip offers an export-packet action.
    pub merge_strip_offers_export_packet_action: bool,
    /// Authority-locus parity is derived from state, never asserted.
    pub authority_locus_derived_never_asserted: bool,
    /// A local estimate or CI-only signal is never shown as provider-authoritative.
    pub local_or_ci_never_shown_as_provider_authoritative: bool,
    /// A not-evaluated-here signal is never shown as evaluated.
    pub not_evaluated_here_never_shown_as_evaluated: bool,
    /// An advisory owner (guessed from the last interacting team) is never shown as authoritative.
    pub advisory_owner_never_shown_as_authoritative: bool,
    /// A change never appears mergeable here without provider-authoritative, unblocked clearance.
    pub mergeable_here_never_widens_from_local_estimate: bool,
    /// A required next action is always present when the change is blocked.
    pub required_next_action_present_when_blocked: bool,
    /// Staleness relative to base/head stays explicit.
    pub stale_relative_to_base_head_always_explicit: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl DriRegistryMergeReadinessReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.dri_row_shows_service_and_owner_identity
            && self.dri_row_shows_escalation_and_support_path
            && self.dri_row_offers_inspect_owner_source
            && self.merge_strip_shows_target_and_blocker_count
            && self.merge_strip_shows_required_next_action
            && self.merge_strip_offers_export_packet_action
            && self.authority_locus_derived_never_asserted
            && self.local_or_ci_never_shown_as_provider_authoritative
            && self.not_evaluated_here_never_shown_as_evaluated
            && self.advisory_owner_never_shown_as_authoritative
            && self.mergeable_here_never_widens_from_local_estimate
            && self.required_next_action_present_when_blocked
            && self.stale_relative_to_base_head_always_explicit
            && self.no_surface_invents_alternate_state_label
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriRegistryMergeReadinessConsumerProjection {
    /// The review-workspace surface reads a single canonical source.
    pub review_workspace_reads_single_source: bool,
    /// The release-candidate surface reads a single canonical source.
    pub release_candidate_reads_single_source: bool,
    /// The governance and shiproom surfaces read a single canonical source.
    pub governance_and_shiproom_read_single_source: bool,
    /// Owner and escalation truth are visible before a handoff feels safe.
    pub owner_and_escalation_visible_before_handoff: bool,
    /// Target and blocker count are visible before a merge feels safe.
    pub target_and_blocker_visible_before_merge: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl DriRegistryMergeReadinessConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_reads_single_source
            && self.release_candidate_reads_single_source
            && self.governance_and_shiproom_read_single_source
            && self.owner_and_escalation_visible_before_handoff
            && self.target_and_blocker_visible_before_merge
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriRegistryMergeReadinessProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`DriRegistryMergeReadinessControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriRegistryMergeReadinessControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// DRI-registry rows.
    pub dri_registry_rows: Vec<DriRegistryRow>,
    /// Merge-readiness strips.
    pub merge_readiness_strips: Vec<MergeReadinessStrip>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// DRI-registry / merge-readiness review block.
    pub review: DriRegistryMergeReadinessReview,
    /// Consumer projection block.
    pub consumer_projection: DriRegistryMergeReadinessConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DriRegistryMergeReadinessProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe DRI-registry-row / merge-readiness-strip controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriRegistryMergeReadinessControlsPacket {
    /// Record kind; must equal [`DRI_REGISTRY_MERGE_READINESS_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DRI_REGISTRY_MERGE_READINESS_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// DRI-registry rows.
    pub dri_registry_rows: Vec<DriRegistryRow>,
    /// Merge-readiness strips.
    pub merge_readiness_strips: Vec<MergeReadinessStrip>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5GovernanceComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5GovernanceComponentConsumerSurface>,
    /// DRI-registry / merge-readiness review block.
    pub review: DriRegistryMergeReadinessReview,
    /// Consumer projection block.
    pub consumer_projection: DriRegistryMergeReadinessConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DriRegistryMergeReadinessProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DriRegistryMergeReadinessControlsPacket {
    /// Builds a DRI-registry-row / merge-readiness-strip controls packet from stable-lane input.
    pub fn new(input: DriRegistryMergeReadinessControlsPacketInput) -> Self {
        Self {
            record_kind: DRI_REGISTRY_MERGE_READINESS_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: DRI_REGISTRY_MERGE_READINESS_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            dri_registry_rows: input.dri_registry_rows,
            merge_readiness_strips: input.merge_readiness_strips,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            review: input.review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the DRI-registry-row / merge-readiness-strip control invariants.
    pub fn validate(&self) -> Vec<DriRegistryMergeReadinessControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DRI_REGISTRY_MERGE_READINESS_CONTROLS_RECORD_KIND {
            violations.push(DriRegistryMergeReadinessControlsViolation::WrongRecordKind);
        }
        if self.schema_version != DRI_REGISTRY_MERGE_READINESS_CONTROLS_SCHEMA_VERSION {
            violations.push(DriRegistryMergeReadinessControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DriRegistryMergeReadinessControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(DriRegistryMergeReadinessControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(DriRegistryMergeReadinessControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_dri_registry_rows(self, &mut violations);
        validate_merge_readiness_strips(self, &mut violations);
        validate_shared_coverage(self, &mut violations);

        if !self.review.all_hold() {
            violations.push(DriRegistryMergeReadinessControlsViolation::ReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(DriRegistryMergeReadinessControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("dri-registry merge-readiness controls packet serializes"),
        ) {
            violations.push(DriRegistryMergeReadinessControlsViolation::RawMaterialInExport);
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
            .expect("dri-registry merge-readiness controls packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let advisory_owners = self
            .dri_registry_rows
            .iter()
            .filter(|row| row.owner_disclosure().is_advisory)
            .count();
        let blocked_strips = self
            .merge_readiness_strips
            .iter()
            .filter(|strip| strip.blocker_count > 0)
            .count();

        let mut out = String::new();
        out.push_str("# DRI-registry rows and merge-readiness strips\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- DRI-registry rows: {} ({} with an advisory owner)\n",
            self.dri_registry_rows.len(),
            advisory_owners
        ));
        out.push_str(&format!(
            "- Merge-readiness strips: {} ({} with an outstanding blocker)\n",
            self.merge_readiness_strips.len(),
            blocked_strips
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## DRI-registry rows\n\n");
        for row in &self.dri_registry_rows {
            let locus = row.locus_disclosure();
            let owner = row.owner_disclosure();
            out.push_str(&format!(
                "- **{}** — owner `{}`, escalation `{}`, freshness `{}`, parity `{}`\n",
                row.service_path_identity_label,
                owner.posture.as_str(),
                row.escalation_continuity_state.as_str(),
                row.registry_freshness_state.as_str(),
                locus.posture.as_str(),
            ));
        }

        out.push_str("\n## Merge-readiness strips\n\n");
        for strip in &self.merge_readiness_strips {
            let locus = strip.locus_disclosure();
            out.push_str(&format!(
                "- **{}** — target `{}`, blockers {}, next `{}`, parity `{}`\n",
                strip.change_title_label,
                strip.merge_target_kind.as_str(),
                strip.blocker_count,
                strip.required_next_action_kind.as_str(),
                locus.posture.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in DRI-registry merge-readiness controls export.
#[derive(Debug)]
pub enum DriRegistryMergeReadinessControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DriRegistryMergeReadinessControlsViolation>),
}

impl fmt::Display for DriRegistryMergeReadinessControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "dri-registry merge-readiness controls export parse failed: {error}"
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
                    "dri-registry merge-readiness controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DriRegistryMergeReadinessControlsArtifactError {}

/// Validation failures emitted by [`DriRegistryMergeReadinessControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriRegistryMergeReadinessControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No DRI-registry rows are present.
    DriRegistryRowsMissing,
    /// A DRI-registry row is incomplete.
    DriRegistryRowIncomplete,
    /// A DRI-registry row carries the wrong frozen component class.
    DriRegistryRowWrongComponentClass,
    /// No merge-readiness strips are present.
    MergeReadinessStripsMissing,
    /// A merge-readiness strip is incomplete.
    MergeReadinessStripIncomplete,
    /// A merge-readiness strip carries the wrong frozen component class.
    MergeReadinessStripWrongComponentClass,
    /// A component misrepresents its derived authority-locus posture.
    AuthorityLocusMisrepresented,
    /// A local-estimate or CI-only component claims provider-authoritative state.
    LocalOrCiClaimsProviderAuthoritative,
    /// A not-evaluated-here component claims it was evaluated.
    NotEvaluatedClaimsEvaluated,
    /// A local-estimate component does not name its local estimate.
    LocalEstimateNoteMissing,
    /// A CI-only component does not name that it was only CI-reported.
    CiOnlyNoteMissing,
    /// A not-evaluated-here component does not name that it was not evaluated here.
    NotEvaluatedNoteMissing,
    /// A stale component does not name its staleness.
    StaleNoteMissing,
    /// A component's governance vocabulary omits its derived locus token.
    GovernanceVocabMissingLocusToken,
    /// A DRI-registry row misrepresents its derived owner-source posture.
    OwnerSourceMisrepresented,
    /// An advisory-owner row claims an authoritative owner.
    AdvisoryOwnerClaimsAuthoritative,
    /// An advisory-owner row does not name that its owner is advisory.
    AdvisoryOwnerNoteMissing,
    /// An unresolved-owner row does not name that its owner is unresolved.
    UnresolvedOwnerNoteMissing,
    /// A DRI-registry row carries a personal contact detail in a role alias.
    PersonContactDetailInAlias,
    /// A DRI-registry row does not offer an inspect-owner-source action.
    InspectOwnerSourceActionMissing,
    /// A DRI-registry row names a support forum but not its stable reference.
    SupportForumRefMissing,
    /// A merge-readiness strip claims mergeable-here without provider-authoritative, unblocked clearance.
    MergeableHereWithoutProviderClearance,
    /// A blocked merge-readiness strip does not name its required next action or blocker summary.
    BlockedStripMissingNextAction,
    /// A merge-readiness strip does not offer an export-readiness-packet action.
    ExportReadinessActionMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component omits a mandatory action.
    ComponentActionsIncomplete,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare any consumer surface.
    ConsumerSurfacesMissing,
    /// The components do not cover every authority-locus source.
    LocusSourceCoverageMissing,
    /// The components do not cover every derived authority-locus posture.
    LocusPostureCoverageMissing,
    /// The DRI rows do not cover every owner-source signal.
    OwnerSourceSignalCoverageMissing,
    /// The DRI rows do not cover every derived owner-source posture.
    OwnerSourcePostureCoverageMissing,
    /// The DRI rows do not cover every support-forum kind.
    SupportForumKindCoverageMissing,
    /// The DRI rows do not cover every escalation-continuity state.
    EscalationContinuityCoverageMissing,
    /// The DRI rows do not cover every registry-freshness state.
    RegistryFreshnessCoverageMissing,
    /// The merge-readiness strips do not cover every merge-target kind.
    MergeTargetKindCoverageMissing,
    /// The merge-readiness strips do not cover every required-next-action kind.
    RequiredNextActionCoverageMissing,
    /// A component hides its owner, escalation, target, or blocker count.
    OwnerOrTargetHidden,
    /// A component lets a local estimate or advisory owner masquerade as authoritative.
    LocalOrAdvisoryMasqueradesAsAuthoritative,
    /// A component guesses the owner from the last interacting team, or widens mergeability.
    OwnerGuessedOrMergeabilityWidened,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// Review does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl DriRegistryMergeReadinessControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::DriRegistryRowsMissing => "dri_registry_rows_missing",
            Self::DriRegistryRowIncomplete => "dri_registry_row_incomplete",
            Self::DriRegistryRowWrongComponentClass => "dri_registry_row_wrong_component_class",
            Self::MergeReadinessStripsMissing => "merge_readiness_strips_missing",
            Self::MergeReadinessStripIncomplete => "merge_readiness_strip_incomplete",
            Self::MergeReadinessStripWrongComponentClass => {
                "merge_readiness_strip_wrong_component_class"
            }
            Self::AuthorityLocusMisrepresented => "authority_locus_misrepresented",
            Self::LocalOrCiClaimsProviderAuthoritative => {
                "local_or_ci_claims_provider_authoritative"
            }
            Self::NotEvaluatedClaimsEvaluated => "not_evaluated_claims_evaluated",
            Self::LocalEstimateNoteMissing => "local_estimate_note_missing",
            Self::CiOnlyNoteMissing => "ci_only_note_missing",
            Self::NotEvaluatedNoteMissing => "not_evaluated_note_missing",
            Self::StaleNoteMissing => "stale_note_missing",
            Self::GovernanceVocabMissingLocusToken => "governance_vocab_missing_locus_token",
            Self::OwnerSourceMisrepresented => "owner_source_misrepresented",
            Self::AdvisoryOwnerClaimsAuthoritative => "advisory_owner_claims_authoritative",
            Self::AdvisoryOwnerNoteMissing => "advisory_owner_note_missing",
            Self::UnresolvedOwnerNoteMissing => "unresolved_owner_note_missing",
            Self::PersonContactDetailInAlias => "person_contact_detail_in_alias",
            Self::InspectOwnerSourceActionMissing => "inspect_owner_source_action_missing",
            Self::SupportForumRefMissing => "support_forum_ref_missing",
            Self::MergeableHereWithoutProviderClearance => {
                "mergeable_here_without_provider_clearance"
            }
            Self::BlockedStripMissingNextAction => "blocked_strip_missing_next_action",
            Self::ExportReadinessActionMissing => "export_readiness_action_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::ComponentActionsIncomplete => "component_actions_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::LocusSourceCoverageMissing => "locus_source_coverage_missing",
            Self::LocusPostureCoverageMissing => "locus_posture_coverage_missing",
            Self::OwnerSourceSignalCoverageMissing => "owner_source_signal_coverage_missing",
            Self::OwnerSourcePostureCoverageMissing => "owner_source_posture_coverage_missing",
            Self::SupportForumKindCoverageMissing => "support_forum_kind_coverage_missing",
            Self::EscalationContinuityCoverageMissing => "escalation_continuity_coverage_missing",
            Self::RegistryFreshnessCoverageMissing => "registry_freshness_coverage_missing",
            Self::MergeTargetKindCoverageMissing => "merge_target_kind_coverage_missing",
            Self::RequiredNextActionCoverageMissing => "required_next_action_coverage_missing",
            Self::OwnerOrTargetHidden => "owner_or_target_hidden",
            Self::LocalOrAdvisoryMasqueradesAsAuthoritative => {
                "local_or_advisory_masquerades_as_authoritative"
            }
            Self::OwnerGuessedOrMergeabilityWidened => "owner_guessed_or_mergeability_widened",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable DRI-registry merge-readiness controls export.
///
/// This is the first real consumer of the DRI-registry / merge-readiness component lane: a
/// review-workspace, release-candidate, governance, shiproom, or support-export surface calls it to
/// ingest the canonical components rather than cloning governance text.
///
/// # Errors
///
/// Returns [`DriRegistryMergeReadinessControlsArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_dri_registry_merge_readiness_controls_export(
) -> Result<DriRegistryMergeReadinessControlsPacket, DriRegistryMergeReadinessControlsArtifactError>
{
    let packet: DriRegistryMergeReadinessControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-dri-registry-merge-readiness-controls-proof/support_export.json"
    )))
    .map_err(DriRegistryMergeReadinessControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DriRegistryMergeReadinessControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &DriRegistryMergeReadinessControlsPacket,
    violations: &mut Vec<DriRegistryMergeReadinessControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        DRI_REGISTRY_MERGE_READINESS_CONTROLS_SCHEMA_REF,
        DRI_REGISTRY_MERGE_READINESS_CONTROLS_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DriRegistryMergeReadinessControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

/// The four hard-invariant bools every component maps into the shared check.
struct ControlInvariants {
    owner_or_target_hidden: bool,
    local_or_advisory_masquerades_as_authoritative: bool,
    owner_guessed_or_mergeability_widened: bool,
    invents_alternate_state_label: bool,
}

/// Validates the authority-locus posture, notes, and cross-checks shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_shared_locus(
    disclosure: &AuthorityLocusDisclosure,
    derived_authority_locus: AuthorityLocusPosture,
    claims_provider_authoritative: bool,
    governance_state_vocab: &[M5GovernanceStateVocab],
    local_estimate_note: &str,
    ci_only_note: &str,
    not_evaluated_note: &str,
    stale_note: &str,
    violations: &mut Vec<DriRegistryMergeReadinessControlsViolation>,
) {
    if derived_authority_locus != disclosure.posture
        || claims_provider_authoritative != disclosure.is_provider_authoritative
    {
        violations.push(DriRegistryMergeReadinessControlsViolation::AuthorityLocusMisrepresented);
    }
    if (disclosure.is_local_estimate || disclosure.is_ci_only) && claims_provider_authoritative {
        violations
            .push(DriRegistryMergeReadinessControlsViolation::LocalOrCiClaimsProviderAuthoritative);
    }
    if disclosure.needs_local_estimate_note && local_estimate_note.trim().is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::LocalEstimateNoteMissing);
    }
    if disclosure.needs_ci_only_note && ci_only_note.trim().is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::CiOnlyNoteMissing);
    }
    if disclosure.needs_not_evaluated_note && not_evaluated_note.trim().is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::NotEvaluatedNoteMissing);
    }
    if disclosure.needs_stale_note && stale_note.trim().is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::StaleNoteMissing);
    }
    if let Some(token) = disclosure.governance_vocab {
        if !governance_state_vocab.contains(&token) {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::GovernanceVocabMissingLocusToken);
        }
    }
}

/// Validates the axes shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_common_control(
    forum_kind: SupportForumKind,
    forum_ref: &str,
    context_note: &str,
    declares_mandatory_actions: bool,
    downgrade_triggers: &[M5GovernanceComponentDowngradeTrigger],
    consumer_surfaces: &[M5GovernanceComponentConsumerSurface],
    invariants: ControlInvariants,
    violations: &mut Vec<DriRegistryMergeReadinessControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::ContextNoteMissing);
    }
    if forum_kind.is_resolvable() && forum_ref.trim().is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::SupportForumRefMissing);
    }
    if !declares_mandatory_actions {
        violations.push(DriRegistryMergeReadinessControlsViolation::ComponentActionsIncomplete);
    }
    if downgrade_triggers.is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::DowngradeTriggersMissing);
    }
    if consumer_surfaces.is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::ConsumerSurfacesMissing);
    }
    if invariants.owner_or_target_hidden {
        violations.push(DriRegistryMergeReadinessControlsViolation::OwnerOrTargetHidden);
    }
    if invariants.local_or_advisory_masquerades_as_authoritative {
        violations.push(
            DriRegistryMergeReadinessControlsViolation::LocalOrAdvisoryMasqueradesAsAuthoritative,
        );
    }
    if invariants.owner_guessed_or_mergeability_widened {
        violations
            .push(DriRegistryMergeReadinessControlsViolation::OwnerGuessedOrMergeabilityWidened);
    }
    if invariants.invents_alternate_state_label {
        violations.push(DriRegistryMergeReadinessControlsViolation::AlternateStateLabelInvented);
    }
}

/// Whether a role alias carries a personal contact detail rather than a role-scoped alias.
fn alias_carries_contact_detail(alias: &str) -> bool {
    alias.contains('@')
}

fn validate_dri_registry_rows(
    packet: &DriRegistryMergeReadinessControlsPacket,
    violations: &mut Vec<DriRegistryMergeReadinessControlsViolation>,
) {
    if packet.dri_registry_rows.is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::DriRegistryRowsMissing);
        return;
    }

    for row in &packet.dri_registry_rows {
        let locus = row.locus_disclosure();
        let owner = row.owner_disclosure();

        if row.row_id.trim().is_empty()
            || row.service_path_identity_label.trim().is_empty()
            || row.primary_dri_alias.trim().is_empty()
            || row.escalation_alias.trim().is_empty()
            || row.support_forum_label.trim().is_empty()
            || row.escalation_path_label.trim().is_empty()
            || row.freshness_label.trim().is_empty()
            || row.context_note.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(DriRegistryMergeReadinessControlsViolation::DriRegistryRowIncomplete);
        }
        if row.component != M5GovernanceComponent::DriRegistryRow {
            violations.push(
                DriRegistryMergeReadinessControlsViolation::DriRegistryRowWrongComponentClass,
            );
        }
        // Role aliases stay role-scoped: no personal contact detail (an `@` handle or email) ever
        // leaks into an export-safe alias.
        if alias_carries_contact_detail(&row.primary_dri_alias)
            || alias_carries_contact_detail(&row.backup_dri_alias)
            || alias_carries_contact_detail(&row.escalation_alias)
            || alias_carries_contact_detail(&row.benchmark_owner_alias)
            || alias_carries_contact_detail(&row.compatibility_owner_alias)
        {
            violations.push(DriRegistryMergeReadinessControlsViolation::PersonContactDetailInAlias);
        }
        validate_shared_locus(
            &locus,
            row.derived_authority_locus,
            row.claims_provider_authoritative,
            &row.governance_state_vocab,
            &row.local_estimate_note,
            &row.ci_only_note,
            &row.not_evaluated_note,
            &row.stale_note,
            violations,
        );
        if row.derived_owner_source != owner.posture
            || row.claims_authoritative_owner != owner.is_authoritative
        {
            violations.push(DriRegistryMergeReadinessControlsViolation::OwnerSourceMisrepresented);
        }
        // AC-1: an advisory owner (guessed from the last interacting team) can never claim to be an
        // authoritative owner.
        if owner.is_advisory && row.claims_authoritative_owner {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::AdvisoryOwnerClaimsAuthoritative);
        }
        if owner.needs_advisory_note && row.advisory_owner_note.trim().is_empty() {
            violations.push(DriRegistryMergeReadinessControlsViolation::AdvisoryOwnerNoteMissing);
        }
        if owner.needs_unresolved_note && row.unresolved_owner_note.trim().is_empty() {
            violations.push(DriRegistryMergeReadinessControlsViolation::UnresolvedOwnerNoteMissing);
        }
        if !row
            .row_actions
            .contains(&DriRegistryRowAction::InspectOwnerSource)
        {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::InspectOwnerSourceActionMissing);
        }
        validate_common_control(
            row.support_forum_kind,
            &row.support_forum_ref,
            &row.context_note,
            row.declares_mandatory_actions(),
            &row.downgrade_triggers,
            &row.consumer_surfaces,
            ControlInvariants {
                owner_or_target_hidden: row.hides_owner_or_escalation_identity,
                local_or_advisory_masquerades_as_authoritative: row
                    .lets_advisory_owner_read_as_authoritative,
                owner_guessed_or_mergeability_widened: row.guesses_owner_from_last_interacting_team,
                invents_alternate_state_label: row.invents_alternate_state_label,
            },
            violations,
        );
    }

    let mut owner_signals: BTreeSet<OwnerSourceSignal> = BTreeSet::new();
    let mut owner_postures: BTreeSet<OwnerSourcePosture> = BTreeSet::new();
    let mut forum_kinds: BTreeSet<SupportForumKind> = BTreeSet::new();
    let mut escalation_states: BTreeSet<EscalationContinuityState> = BTreeSet::new();
    let mut freshness_states: BTreeSet<RegistryFreshnessState> = BTreeSet::new();
    for row in &packet.dri_registry_rows {
        owner_signals.insert(row.owner_source_signal);
        owner_postures.insert(row.owner_disclosure().posture);
        forum_kinds.insert(row.support_forum_kind);
        escalation_states.insert(row.escalation_continuity_state);
        freshness_states.insert(row.registry_freshness_state);
    }
    if OwnerSourceSignal::ALL
        .iter()
        .any(|signal| !owner_signals.contains(signal))
    {
        violations
            .push(DriRegistryMergeReadinessControlsViolation::OwnerSourceSignalCoverageMissing);
    }
    // AC-1: the DRI rows alone distinguish every owner-source posture, so an advisory owner is always
    // separable from an authoritative one.
    if OwnerSourcePosture::ALL
        .iter()
        .any(|posture| !owner_postures.contains(posture))
    {
        violations
            .push(DriRegistryMergeReadinessControlsViolation::OwnerSourcePostureCoverageMissing);
    }
    if SupportForumKind::ALL
        .iter()
        .any(|kind| !forum_kinds.contains(kind))
    {
        violations
            .push(DriRegistryMergeReadinessControlsViolation::SupportForumKindCoverageMissing);
    }
    if EscalationContinuityState::ALL
        .iter()
        .any(|state| !escalation_states.contains(state))
    {
        violations
            .push(DriRegistryMergeReadinessControlsViolation::EscalationContinuityCoverageMissing);
    }
    if RegistryFreshnessState::ALL
        .iter()
        .any(|state| !freshness_states.contains(state))
    {
        violations
            .push(DriRegistryMergeReadinessControlsViolation::RegistryFreshnessCoverageMissing);
    }
}

fn validate_merge_readiness_strips(
    packet: &DriRegistryMergeReadinessControlsPacket,
    violations: &mut Vec<DriRegistryMergeReadinessControlsViolation>,
) {
    if packet.merge_readiness_strips.is_empty() {
        violations.push(DriRegistryMergeReadinessControlsViolation::MergeReadinessStripsMissing);
        return;
    }

    let mut target_kinds: BTreeSet<MergeTargetKind> = BTreeSet::new();
    let mut next_actions: BTreeSet<RequiredNextActionKind> = BTreeSet::new();

    for strip in &packet.merge_readiness_strips {
        let locus = strip.locus_disclosure();
        target_kinds.insert(strip.merge_target_kind);
        next_actions.insert(strip.required_next_action_kind);

        if strip.strip_id.trim().is_empty()
            || strip.change_title_label.trim().is_empty()
            || strip.merge_target_label.trim().is_empty()
            || strip.required_next_action_label.trim().is_empty()
            || strip.export_packet_action_label.trim().is_empty()
            || strip.mergeability_label.trim().is_empty()
            || strip.export_parity_label.trim().is_empty()
            || strip.context_note.trim().is_empty()
            || strip.source_contract_refs.is_empty()
        {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::MergeReadinessStripIncomplete);
        }
        if strip.component != M5GovernanceComponent::MergeReadinessStrip {
            violations.push(
                DriRegistryMergeReadinessControlsViolation::MergeReadinessStripWrongComponentClass,
            );
        }
        // A blocked change must always name what unblocks it.
        if strip.blocker_count > 0 && strip.blocker_summary_label.trim().is_empty() {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::BlockedStripMissingNextAction);
        }
        validate_shared_locus(
            &locus,
            strip.derived_authority_locus,
            strip.claims_provider_authoritative,
            &strip.governance_state_vocab,
            &strip.local_estimate_note,
            &strip.ci_only_note,
            &strip.not_evaluated_note,
            &strip.stale_note,
            violations,
        );
        if strip.claims_evaluated_here != locus.is_evaluated_here {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::AuthorityLocusMisrepresented);
        }
        if !locus.is_evaluated_here && strip.claims_evaluated_here {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::NotEvaluatedClaimsEvaluated);
        }
        // AC-2: a change never appears mergeable here unless the provider authoritatively clears it and
        // no blocker remains — it never widens from a local estimate, a CI-only signal, a stale gate, or
        // an outstanding blocker.
        if strip.claims_mergeable_here
            && (!locus.is_provider_authoritative || strip.blocker_count > 0)
        {
            violations.push(
                DriRegistryMergeReadinessControlsViolation::MergeableHereWithoutProviderClearance,
            );
        }
        if !strip
            .strip_actions
            .contains(&MergeReadinessStripAction::ExportReadinessPacket)
        {
            violations
                .push(DriRegistryMergeReadinessControlsViolation::ExportReadinessActionMissing);
        }
        // A merge-readiness strip binds no openable support forum of its own, but it must still name a
        // context note and declare its mandatory actions, triggers, and surfaces.
        validate_common_control(
            SupportForumKind::NoForum,
            "",
            &strip.context_note,
            strip.declares_mandatory_actions(),
            &strip.downgrade_triggers,
            &strip.consumer_surfaces,
            ControlInvariants {
                owner_or_target_hidden: strip.hides_target_or_blocker_count,
                local_or_advisory_masquerades_as_authoritative: strip
                    .lets_local_estimate_read_as_provider_mergeable,
                owner_guessed_or_mergeability_widened: strip
                    .widens_local_estimate_to_provider_mergeability,
                invents_alternate_state_label: strip.invents_alternate_state_label,
            },
            violations,
        );
    }

    if MergeTargetKind::ALL
        .iter()
        .any(|kind| !target_kinds.contains(kind))
    {
        violations.push(DriRegistryMergeReadinessControlsViolation::MergeTargetKindCoverageMissing);
    }
    if RequiredNextActionKind::ALL
        .iter()
        .any(|kind| !next_actions.contains(kind))
    {
        violations
            .push(DriRegistryMergeReadinessControlsViolation::RequiredNextActionCoverageMissing);
    }
}

/// Validates that the union of both component vectors covers every locus source and posture.
fn validate_shared_coverage(
    packet: &DriRegistryMergeReadinessControlsPacket,
    violations: &mut Vec<DriRegistryMergeReadinessControlsViolation>,
) {
    let mut sources: BTreeSet<AuthorityLocusSource> = BTreeSet::new();
    let mut postures: BTreeSet<AuthorityLocusPosture> = BTreeSet::new();

    for row in &packet.dri_registry_rows {
        sources.insert(row.authority_locus_source);
        postures.insert(row.locus_disclosure().posture);
    }
    for strip in &packet.merge_readiness_strips {
        sources.insert(strip.authority_locus_source);
        postures.insert(strip.locus_disclosure().posture);
    }

    if AuthorityLocusSource::ALL
        .iter()
        .any(|source| !sources.contains(source))
    {
        violations.push(DriRegistryMergeReadinessControlsViolation::LocusSourceCoverageMissing);
    }
    if AuthorityLocusPosture::ALL
        .iter()
        .any(|posture| !postures.contains(posture))
    {
        violations.push(DriRegistryMergeReadinessControlsViolation::LocusPostureCoverageMissing);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
