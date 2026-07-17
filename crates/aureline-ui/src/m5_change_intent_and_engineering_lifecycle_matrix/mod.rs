//! Frozen M5 change-intent, start-work-sheet, linked-change-panel, ready-for-review-handoff, resolve-close-sheet, and blocked-escalate-card matrix.
//!
//! This module locks Aureline's change-intent lifecycle model — the durable change-intent record, the
//! start-work sheet, the linked-change panel, the ready-for-review handoff sheet, the resolve-or-close sheet, and
//! the blocked-or-escalate card that a work-item-driven consumer must treat as first-class, durable,
//! provider-aware engineering objects rather than intent scattered across browser tabs, local notes, review
//! titles, and terminal history — into one export-safe packet. Every covered object class is named once here and
//! constrained by the same shared change-intent role taxonomy (provider_ownership_disclosure,
//! local_versus_provider_state_disclosure, linked_engineering_identity_disclosure, side_effect_disclosure,
//! validation_evidence_disclosure, publish_later_fallback_disclosure, final_resolution_authority_disclosure), the
//! same required visible state (surface label, provider ownership, local-versus-provider state, linked
//! engineering identity, relation source, blocker state, and validation evidence), the same
//! no-start-work-side-effect-created-without-disclosure rule, the same
//! no-local-handoff-packet-masquerading-as-a-provider-committed-update rule, the same
//! no-flattening-the-four-relation-sources-into-one-badge rule, the same
//! no-auto-resolve-while-engineering-blockers-remain rule, and the same
//! no-dropping-local-notes-handoff-packets-or-linked-evidence-when-provider-write-fails rule regardless of the
//! surface that renders it.
//!
//! The matrix makes a provider-committed update mechanically distinct from a local-only draft, a queued publish,
//! or an offline handoff packet (see [`M5ChangeIntentCommitState`]) so the work-item detail, the start-work
//! sheet, the linked-change panel, the ready-for-review handoff, the resolve-or-close sheet, the blocked-or-
//! escalate card, and support / export packets can key off the commit state, relation source, and blocker state
//! rather than guessing from a generic status pill. It does not widen M5 into a full tracker backend, a hosted
//! project-management client, or a workflow engine — it reuses the already-landed provider-linked
//! draft / publish-now / open-in-provider mutation flows, work-item rows and detail headers, offline handoff
//! packets, hosted-review reusable rows, Git worktree identity, and review-pack / local-parity truth — it is the
//! shared reusable change-intent contract those consumers read, and it binds back to the already-landed
//! stable-proof-index and migration-task-row packets so change-intent and handoff truth is not split across
//! scattered internal notes. The controlled vocabularies are frozen in one self-describing
//! [`M5ChangeIntentVocabularySet`] rather than minted per surface. Raw paths, raw glob bodies, raw command lines,
//! raw provider payloads, secret values, and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_change_intent_matrix,
    seeded_m5_change_intent_matrix_blocked_escalate_card_preview_narrowed,
    seeded_m5_change_intent_matrix_start_work_sheet_beta_narrowed,
    M5_CHANGE_INTENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ChangeIntentMatrixPacket`].
pub const M5_CHANGE_INTENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_change_intent_start_work_handoff_resolve_and_blocker_matrix";

/// Schema version for M5 change-intent matrix records.
pub const M5_CHANGE_INTENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined change-intent lifecycle matrix schema.
pub const M5_CHANGE_INTENT_MATRIX_SCHEMA_REF: &str =
    "schemas/teamwork/m5-change-intent-lifecycle-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CHANGE_INTENT_MATRIX_DOC_REF: &str =
    "docs/team-workflows/m5-change-intent-lifecycle-ops.md";

/// Repo-relative path of the canonical change-intent domain schema (the durable change-intent record with its
/// provider ownership, local-versus-provider state, and linked engineering identity).
pub const M5_CHANGE_INTENT_DOMAIN_SCHEMA_REF: &str =
    "schemas/teamwork/m5-change-intent.schema.json";

/// Repo-relative path of the canonical start-work-sheet domain schema (the sheet that discloses each start-work
/// side effect separately).
pub const M5_START_WORK_SHEET_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-start-work-sheet.schema.json";

/// Repo-relative path of the canonical linked-change-panel domain schema (the relation strip keeping the four
/// relation sources distinct).
pub const M5_LINKED_CHANGE_PANEL_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-linked-change-panel.schema.json";

/// Repo-relative path of the canonical ready-for-review-handoff-sheet domain schema (validation evidence plus a
/// publish-later fallback).
pub const M5_READY_FOR_REVIEW_HANDOFF_SHEET_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-ready-for-review-handoff-sheet.schema.json";

/// Repo-relative path of the canonical resolve-close-sheet domain schema (final-resolution authority plus
/// unresolved-blocker state).
pub const M5_RESOLVE_CLOSE_SHEET_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-resolve-close-sheet.schema.json";

/// Repo-relative path of the canonical blocked-escalate-card domain schema (the engineering blocker and its
/// escalation path).
pub const M5_BLOCKED_ESCALATE_CARD_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-blocked-escalate-card.schema.json";

/// Repo-relative path of the work-item handoff-packet schema the matrix references for offline handoff continuity.
pub const M5_WORK_ITEM_HANDOFF_PACKET_DOMAIN_SCHEMA_REF: &str =
    "schemas/teamwork/m5-work-item-handoff-packet.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CHANGE_INTENT_FIXTURE_DIR: &str = "fixtures/teamwork/m5-change-intent";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CHANGE_INTENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-change-intent-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CHANGE_INTENT_CSV_REF: &str = "artifacts/release/m5-change-intent-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CHANGE_INTENT_REPORT_REF: &str =
    "artifacts/design/m5-change-intent-component-matrix.md";

/// Repo-relative path of the checked change-intent-health dashboard.
pub const M5_CHANGE_INTENT_DASHBOARD_REF: &str = "dashboards/m5-change-intent-health.json";

/// One of the six governed change-intent object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentObject {
    /// A durable change-intent record: the tracked work item's intent bound to provider ownership, local-versus-provider state, and the linked branch / worktree / review identity a whole engineering lifecycle hangs from.
    ChangeIntentRecord,
    /// A start-work sheet: the sheet that launches work from a tracked item and separately discloses each side effect it creates — branch, worktree, review draft, and provider link.
    StartWorkSheet,
    /// A linked-change panel: the relation strip that keeps linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken relations distinct instead of one generic badge.
    LinkedChangePanel,
    /// A ready-for-review handoff sheet: the sheet that packages a review handoff with its validation evidence and a publish-later fallback, never letting a local handoff packet read as a provider-committed update.
    ReadyForReviewHandoffSheet,
    /// A resolve-or-close sheet: the sheet that records final-resolution authority and refuses to auto-resolve tracked work while engineering blockers remain.
    ResolveCloseSheet,
    /// A blocked-or-escalate card: the card that surfaces an unresolved engineering blocker and its escalation path without dropping local notes or linked evidence.
    BlockedEscalateCard,
}

impl M5ChangeIntentObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ChangeIntentRecord,
        Self::StartWorkSheet,
        Self::LinkedChangePanel,
        Self::ReadyForReviewHandoffSheet,
        Self::ResolveCloseSheet,
        Self::BlockedEscalateCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeIntentRecord => "change_intent_record",
            Self::StartWorkSheet => "start_work_sheet",
            Self::LinkedChangePanel => "linked_change_panel",
            Self::ReadyForReviewHandoffSheet => "ready_for_review_handoff_sheet",
            Self::ResolveCloseSheet => "resolve_close_sheet",
            Self::BlockedEscalateCard => "blocked_escalate_card",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's change-intent, start-work, linked-change, handoff, resolve, or blocker meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::ChangeIntentRecord => M5_CHANGE_INTENT_DOMAIN_SCHEMA_REF,
            Self::StartWorkSheet => M5_START_WORK_SHEET_DOMAIN_SCHEMA_REF,
            Self::LinkedChangePanel => M5_LINKED_CHANGE_PANEL_DOMAIN_SCHEMA_REF,
            Self::ReadyForReviewHandoffSheet => M5_READY_FOR_REVIEW_HANDOFF_SHEET_DOMAIN_SCHEMA_REF,
            Self::ResolveCloseSheet => M5_RESOLVE_CLOSE_SHEET_DOMAIN_SCHEMA_REF,
            Self::BlockedEscalateCard => M5_BLOCKED_ESCALATE_CARD_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this class must name a controlled change intent record role.
    pub const fn declares_change_intent_record_roles(self) -> bool {
        matches!(self, Self::ChangeIntentRecord)
    }

    /// `true` when this class must name a controlled start work role.
    pub const fn declares_start_work_roles(self) -> bool {
        matches!(self, Self::StartWorkSheet)
    }

    /// `true` when this class must name a controlled linked change role.
    pub const fn declares_linked_change_roles(self) -> bool {
        matches!(self, Self::LinkedChangePanel)
    }

    /// `true` when this class must name a controlled handoff role.
    pub const fn declares_handoff_roles(self) -> bool {
        matches!(self, Self::ReadyForReviewHandoffSheet)
    }

    /// `true` when this class must name a controlled resolve role.
    pub const fn declares_resolve_roles(self) -> bool {
        matches!(self, Self::ResolveCloseSheet)
    }

    /// `true` when this class must name a controlled blocked escalate role.
    pub const fn declares_blocked_escalate_roles(self) -> bool {
        matches!(self, Self::BlockedEscalateCard)
    }
}

/// The single controlled change-intent role vocabulary every work-item, start-work, review, provider handoff, help / docs, or support / export consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentRole {
    /// The provider ownership of the tracked work item disclosed on every claimed surface.
    ProviderOwnershipDisclosure,
    /// The local-versus-provider commit state disclosed so a local draft or queued publish never reads as a provider-committed update.
    LocalVersusProviderStateDisclosure,
    /// The linked branch / worktree / review identity disclosed so intent joins back to concrete engineering artifacts.
    LinkedEngineeringIdentityDisclosure,
    /// Each start-work side effect (branch, worktree, review draft, provider link) disclosed separately, never silently created.
    SideEffectDisclosure,
    /// The validation evidence packaged with a handoff disclosed as an explicit set.
    ValidationEvidenceDisclosure,
    /// The publish-later / queued-publish fallback disclosed so a deferred update is never mistaken for a committed one.
    PublishLaterFallbackDisclosure,
    /// The final-resolution authority and any unresolved blocker disclosed before a tracked item is resolved or closed.
    FinalResolutionAuthorityDisclosure,
}

impl M5ChangeIntentRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProviderOwnershipDisclosure,
        Self::LocalVersusProviderStateDisclosure,
        Self::LinkedEngineeringIdentityDisclosure,
        Self::SideEffectDisclosure,
        Self::ValidationEvidenceDisclosure,
        Self::PublishLaterFallbackDisclosure,
        Self::FinalResolutionAuthorityDisclosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOwnershipDisclosure => "provider_ownership_disclosure",
            Self::LocalVersusProviderStateDisclosure => "local_versus_provider_state_disclosure",
            Self::LinkedEngineeringIdentityDisclosure => "linked_engineering_identity_disclosure",
            Self::SideEffectDisclosure => "side_effect_disclosure",
            Self::ValidationEvidenceDisclosure => "validation_evidence_disclosure",
            Self::PublishLaterFallbackDisclosure => "publish_later_fallback_disclosure",
            Self::FinalResolutionAuthorityDisclosure => "final_resolution_authority_disclosure",
        }
    }
    /// Whether this role is a hard posture requirement that must be present before a class may be
    /// surfaced as a change-intent result (`provider_ownership_disclosure`,
    /// `local_versus_provider_state_disclosure`, `linked_engineering_identity_disclosure`,
    /// `side_effect_disclosure`). The contextual roles (`validation_evidence_disclosure`,
    /// `publish_later_fallback_disclosure`, `final_resolution_authority_disclosure`) apply where the
    /// object class calls for them.
    pub const fn must_be_present_before_surfacing_as_a_change_intent_result(self) -> bool {
        matches!(
            self,
            Self::ProviderOwnershipDisclosure
                | Self::LocalVersusProviderStateDisclosure
                | Self::LinkedEngineeringIdentityDisclosure
                | Self::SideEffectDisclosure
        )
    }
}

/// Commit state that makes a provider-committed update (authoritative tracked-item truth) mechanically distinct from a local-only draft, a queued publish, a publish-failed-retained state, a provider-unavailable state, an offline handoff packet, or a stale-relative-to-provider view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentCommitState {
    /// Provider-committed: the update was written to the connected provider and carries authoritative tracked-item truth.
    ProviderCommitted,
    /// A local-only draft: intent captured on this machine only, never a provider-committed update.
    LocalOnlyDraft,
    /// Queued for publish: a deferred publish-later update waiting to reach the provider, not yet committed.
    QueuedForPublish,
    /// Publish failed and the local notes, handoff packet, and linked evidence were retained for retry, not a committed update.
    PublishFailedRetained,
    /// The provider is unavailable so authoritative tracked-item truth cannot be written or fetched right now.
    ProviderUnavailable,
    /// An offline handoff packet: a portable handoff captured without a provider connection, never a provider-committed update.
    OfflineHandoffPacket,
    /// The local view is stale relative to the provider and must be refreshed before it is trusted.
    StaleRelativeToProvider,
}

impl M5ChangeIntentCommitState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProviderCommitted,
        Self::LocalOnlyDraft,
        Self::QueuedForPublish,
        Self::PublishFailedRetained,
        Self::ProviderUnavailable,
        Self::OfflineHandoffPacket,
        Self::StaleRelativeToProvider,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderCommitted => "provider_committed",
            Self::LocalOnlyDraft => "local_only_draft",
            Self::QueuedForPublish => "queued_for_publish",
            Self::PublishFailedRetained => "publish_failed_retained",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::OfflineHandoffPacket => "offline_handoff_packet",
            Self::StaleRelativeToProvider => "stale_relative_to_provider",
        }
    }
    /// `true` only for the provider-committed state, so downstream work-item detail, the start-work
    /// sheet, the ready-for-review handoff, provider handoff, and support / export packets can key off a
    /// committed provider update rather than confusing it with a local-only draft, a queued publish, or
    /// an offline handoff packet.
    pub const fn is_provider_committed(self) -> bool {
        matches!(self, Self::ProviderCommitted)
    }
}

/// Named relation source (linked by provider, linked locally, suggested by Aureline, stale or broken relation) so the four relation kinds are never flattened into one generic relation badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentRelationSource {
    /// Linked by the provider: an authoritative provider-side link between the tracked item and the change.
    LinkedByProvider,
    /// Linked locally: a link recorded on this machine that has not been confirmed by the provider.
    LinkedLocally,
    /// Suggested by Aureline: an inferred relation offered as a suggestion, not an established link.
    SuggestedByAureline,
    /// A stale or broken relation: a previously linked change whose target moved or no longer resolves.
    StaleOrBrokenRelation,
}

impl M5ChangeIntentRelationSource {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LinkedByProvider,
        Self::LinkedLocally,
        Self::SuggestedByAureline,
        Self::StaleOrBrokenRelation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinkedByProvider => "linked_by_provider",
            Self::LinkedLocally => "linked_locally",
            Self::SuggestedByAureline => "suggested_by_aureline",
            Self::StaleOrBrokenRelation => "stale_or_broken_relation",
        }
    }
    /// `true` only for a provider-side link, so a consumer can mechanically refuse to flatten a
    /// locally linked, suggested, or stale relation into a provider-authoritative link badge.
    pub const fn is_provider_linked(self) -> bool {
        matches!(self, Self::LinkedByProvider)
    }
}

/// Named blocker / resolution state (ready to resolve, blocked by engineering, escalation open, awaiting provider write, resolution authority missing) so no claimed surface lacks a named state for an unresolved engineering blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentBlockerState {
    /// Ready to resolve: no engineering blocker remains and the tracked item may be resolved or closed.
    ReadyToResolve,
    /// Blocked by engineering: an unresolved engineering blocker prevents resolution.
    BlockedByEngineering,
    /// An open escalation: the blocker has been escalated and is awaiting a decision.
    EscalationOpen,
    /// Awaiting provider write: the resolution is captured locally but a provider write is still pending.
    AwaitingProviderWrite,
    /// Resolution authority missing: no actor with final-resolution authority has confirmed the close.
    ResolutionAuthorityMissing,
}

impl M5ChangeIntentBlockerState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReadyToResolve,
        Self::BlockedByEngineering,
        Self::EscalationOpen,
        Self::AwaitingProviderWrite,
        Self::ResolutionAuthorityMissing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToResolve => "ready_to_resolve",
            Self::BlockedByEngineering => "blocked_by_engineering",
            Self::EscalationOpen => "escalation_open",
            Self::AwaitingProviderWrite => "awaiting_provider_write",
            Self::ResolutionAuthorityMissing => "resolution_authority_missing",
        }
    }
    /// `true` for the blocked / escalated / pending / authority-missing states (`blocked_by_engineering`,
    /// `escalation_open`, `awaiting_provider_write`, `resolution_authority_missing`) so a consumer can
    /// mechanically refuse to auto-resolve tracked work while an engineering blocker remains.
    pub const fn is_blocked_or_unresolved(self) -> bool {
        matches!(
            self,
            Self::BlockedByEngineering
                | Self::EscalationOpen
                | Self::AwaitingProviderWrite
                | Self::ResolutionAuthorityMissing
        )
    }
}

/// Controlled change-intent-record role for one tracked work item's change intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentRecordRole {
    /// Provider ownership shown so a tracked item names who owns it upstream.
    ProviderOwnershipShown,
    /// Linked branch / worktree / review identity named so intent binds to concrete artifacts.
    LinkedEngineeringIdentityNamed,
    /// Local-versus-provider commit state shown so a local draft never reads as committed.
    LocalVersusProviderStateShown,
    /// Intent lifecycle stage shown so a record states where it sits from captured to resolved.
    IntentLifecycleStageShown,
    /// A role bound to the single change-intent registry.
    BoundToChangeIntentRegistry,
    /// Silently swapping the provider link or ownership without disclosure, which is disallowed.
    SilentProviderLinkSwapDisallowed,
}

impl M5ChangeIntentRecordRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderOwnershipShown,
        Self::LinkedEngineeringIdentityNamed,
        Self::LocalVersusProviderStateShown,
        Self::IntentLifecycleStageShown,
        Self::BoundToChangeIntentRegistry,
        Self::SilentProviderLinkSwapDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOwnershipShown => "provider_ownership_shown",
            Self::LinkedEngineeringIdentityNamed => "linked_engineering_identity_named",
            Self::LocalVersusProviderStateShown => "local_versus_provider_state_shown",
            Self::IntentLifecycleStageShown => "intent_lifecycle_stage_shown",
            Self::BoundToChangeIntentRegistry => "bound_to_change_intent_registry",
            Self::SilentProviderLinkSwapDisallowed => "silent_provider_link_swap_disallowed",
        }
    }
}

/// Controlled start-work-sheet role for launching work with disclosed side effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentStartWorkRole {
    /// The branch this start-work sheet would create disclosed separately.
    BranchSideEffectDisclosed,
    /// The worktree this start-work sheet would create disclosed separately.
    WorktreeSideEffectDisclosed,
    /// The review draft this start-work sheet would create disclosed separately.
    ReviewDraftSideEffectDisclosed,
    /// The provider link this start-work sheet would create disclosed separately.
    ProviderLinkSideEffectDisclosed,
    /// A role bound to the single change-intent registry.
    BoundToChangeIntentRegistry,
    /// Silently creating a branch, worktree, review draft, or provider link without disclosing each side effect, which is disallowed.
    SilentSideEffectCreationDisallowed,
}

impl M5ChangeIntentStartWorkRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BranchSideEffectDisclosed,
        Self::WorktreeSideEffectDisclosed,
        Self::ReviewDraftSideEffectDisclosed,
        Self::ProviderLinkSideEffectDisclosed,
        Self::BoundToChangeIntentRegistry,
        Self::SilentSideEffectCreationDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BranchSideEffectDisclosed => "branch_side_effect_disclosed",
            Self::WorktreeSideEffectDisclosed => "worktree_side_effect_disclosed",
            Self::ReviewDraftSideEffectDisclosed => "review_draft_side_effect_disclosed",
            Self::ProviderLinkSideEffectDisclosed => "provider_link_side_effect_disclosed",
            Self::BoundToChangeIntentRegistry => "bound_to_change_intent_registry",
            Self::SilentSideEffectCreationDisallowed => "silent_side_effect_creation_disallowed",
        }
    }
}

/// Controlled linked-change-panel role for the relation source of a linked change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentLinkedChangeRole {
    /// Relation source shown so linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken stay distinct.
    RelationSourceShown,
    /// Linked target identity named so a relation points at an exact change.
    LinkedTargetIdentityNamed,
    /// Stale-or-broken relation flagged so a dead link never reads as live.
    StaleOrBrokenRelationFlagged,
    /// A suggested relation labelled as a suggestion so it never reads as an established link.
    SuggestedRelationLabelledAsSuggestion,
    /// A role bound to the single change-intent registry.
    BoundToChangeIntentRegistry,
    /// Flattening linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken into one generic relation badge, which is disallowed.
    RelationSourcesFlattenedDisallowed,
}

impl M5ChangeIntentLinkedChangeRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RelationSourceShown,
        Self::LinkedTargetIdentityNamed,
        Self::StaleOrBrokenRelationFlagged,
        Self::SuggestedRelationLabelledAsSuggestion,
        Self::BoundToChangeIntentRegistry,
        Self::RelationSourcesFlattenedDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelationSourceShown => "relation_source_shown",
            Self::LinkedTargetIdentityNamed => "linked_target_identity_named",
            Self::StaleOrBrokenRelationFlagged => "stale_or_broken_relation_flagged",
            Self::SuggestedRelationLabelledAsSuggestion => {
                "suggested_relation_labelled_as_suggestion"
            }
            Self::BoundToChangeIntentRegistry => "bound_to_change_intent_registry",
            Self::RelationSourcesFlattenedDisallowed => "relation_sources_flattened_disallowed",
        }
    }
}

/// Controlled ready-for-review-handoff role for packaging a review handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentHandoffRole {
    /// Validation evidence shown so a handoff names the checks that back it.
    ValidationEvidenceShown,
    /// Publish-later fallback shown so a deferred handoff states it is not yet committed.
    PublishLaterFallbackShown,
    /// A local handoff packet labelled as local so it never reads as a provider-committed update.
    LocalHandoffPacketLabelledAsLocal,
    /// Handoff destination named so a review handoff states where it is going.
    HandoffDestinationNamed,
    /// A role bound to the single change-intent registry.
    BoundToChangeIntentRegistry,
    /// Letting a local handoff packet or queued publish masquerade as a provider-committed update, which is disallowed.
    LocalHandoffMasqueradingAsProviderCommittedDisallowed,
}

impl M5ChangeIntentHandoffRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ValidationEvidenceShown,
        Self::PublishLaterFallbackShown,
        Self::LocalHandoffPacketLabelledAsLocal,
        Self::HandoffDestinationNamed,
        Self::BoundToChangeIntentRegistry,
        Self::LocalHandoffMasqueradingAsProviderCommittedDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidationEvidenceShown => "validation_evidence_shown",
            Self::PublishLaterFallbackShown => "publish_later_fallback_shown",
            Self::LocalHandoffPacketLabelledAsLocal => "local_handoff_packet_labelled_as_local",
            Self::HandoffDestinationNamed => "handoff_destination_named",
            Self::BoundToChangeIntentRegistry => "bound_to_change_intent_registry",
            Self::LocalHandoffMasqueradingAsProviderCommittedDisallowed => {
                "local_handoff_masquerading_as_provider_committed_disallowed"
            }
        }
    }
}

/// Controlled resolve-or-close-sheet role for recording final resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentResolveRole {
    /// Final-resolution authority shown so a close names who confirmed it.
    FinalResolutionAuthorityShown,
    /// Unresolved engineering blocker shown so a resolution never hides an open blocker.
    UnresolvedBlockerShown,
    /// Resolution outcome named so a close states resolved-versus-closed intent.
    ResolutionOutcomeNamed,
    /// Provider-write-pending state shown so a locally captured resolution never reads as committed.
    ProviderWritePendingShown,
    /// A role bound to the single change-intent registry.
    BoundToChangeIntentRegistry,
    /// Auto-resolving tracked work while engineering blockers remain unresolved, which is disallowed.
    AutoResolveWithOpenBlockerDisallowed,
}

impl M5ChangeIntentResolveRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FinalResolutionAuthorityShown,
        Self::UnresolvedBlockerShown,
        Self::ResolutionOutcomeNamed,
        Self::ProviderWritePendingShown,
        Self::BoundToChangeIntentRegistry,
        Self::AutoResolveWithOpenBlockerDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FinalResolutionAuthorityShown => "final_resolution_authority_shown",
            Self::UnresolvedBlockerShown => "unresolved_blocker_shown",
            Self::ResolutionOutcomeNamed => "resolution_outcome_named",
            Self::ProviderWritePendingShown => "provider_write_pending_shown",
            Self::BoundToChangeIntentRegistry => "bound_to_change_intent_registry",
            Self::AutoResolveWithOpenBlockerDisallowed => {
                "auto_resolve_with_open_blocker_disallowed"
            }
        }
    }
}

/// Controlled blocked-or-escalate-card role for surfacing an engineering blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentBlockedEscalateRole {
    /// Blocker cause named so an escalation states what is blocked.
    BlockerCauseNamed,
    /// Escalation path shown so a blocked card names where the escalation goes.
    EscalationPathShown,
    /// Local notes and linked evidence retained so nothing is dropped when a blocker is raised.
    LocalNotesAndLinkedEvidenceRetained,
    /// Blocker state shown so a card states blocked-versus-escalated-versus-ready.
    BlockerStateShown,
    /// A role bound to the single change-intent registry.
    BoundToChangeIntentRegistry,
    /// Dropping local notes, handoff packets, or linked evidence when provider write fails, which is disallowed.
    DroppingLocalNotesOrLinkedEvidenceDisallowed,
}

impl M5ChangeIntentBlockedEscalateRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BlockerCauseNamed,
        Self::EscalationPathShown,
        Self::LocalNotesAndLinkedEvidenceRetained,
        Self::BlockerStateShown,
        Self::BoundToChangeIntentRegistry,
        Self::DroppingLocalNotesOrLinkedEvidenceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockerCauseNamed => "blocker_cause_named",
            Self::EscalationPathShown => "escalation_path_shown",
            Self::LocalNotesAndLinkedEvidenceRetained => "local_notes_and_linked_evidence_retained",
            Self::BlockerStateShown => "blocker_state_shown",
            Self::BoundToChangeIntentRegistry => "bound_to_change_intent_registry",
            Self::DroppingLocalNotesOrLinkedEvidenceDisallowed => {
                "dropping_local_notes_or_linked_evidence_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a change-intent object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentSurfaceFamily {
    /// The work-item surface (work-item rows and detail headers).
    WorkItem,
    /// The start-work surface (branch / worktree creation and side-effect disclosure).
    StartWork,
    /// The review surface (ready-for-review handoff and review detail).
    Review,
    /// The provider handoff / open-in-provider / publish-later surface.
    ProviderHandoff,
    /// The support / export surface.
    SupportExport,
    /// The help / docs surface.
    HelpDocs,
}

impl M5ChangeIntentSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkItem,
        Self::StartWork,
        Self::Review,
        Self::ProviderHandoff,
        Self::SupportExport,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkItem => "work_item",
            Self::StartWork => "start_work",
            Self::Review => "review",
            Self::ProviderHandoff => "provider_handoff",
            Self::SupportExport => "support_export",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Classification stage a class passes through from intent capture to a work-started, change-linked, handoff-packaged, and resolution-recorded change-intent object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentClassificationStage {
    /// The intent-captured stage: the tracked work item's change intent and provider ownership are captured.
    IntentCaptured,
    /// The work-started stage: the branch / worktree / review draft / provider link side effects are disclosed and created.
    WorkStarted,
    /// The change-linked stage: the branch / review relation is linked with its relation source.
    ChangeLinked,
    /// The handoff-packaged stage: validation evidence and the publish-later fallback are packaged for review.
    HandoffPackaged,
    /// The resolution-recorded stage: the final-resolution authority and any unresolved blocker are recorded.
    ResolutionRecorded,
}

impl M5ChangeIntentClassificationStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::IntentCaptured,
        Self::WorkStarted,
        Self::ChangeLinked,
        Self::HandoffPackaged,
        Self::ResolutionRecorded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IntentCaptured => "intent_captured",
            Self::WorkStarted => "work_started",
            Self::ChangeLinked => "change_linked",
            Self::HandoffPackaged => "handoff_packaged",
            Self::ResolutionRecorded => "resolution_recorded",
        }
    }
}

/// Shared consumer surface that must agree on a class's change-intent truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentConsumerSurface {
    /// The work-item detail surface.
    WorkItemDetail,
    /// The start-work sheet.
    StartWorkSheet,
    /// The linked-change panel.
    LinkedChangePanel,
    /// The review detail surface.
    ReviewDetail,
    /// The ready-for-review handoff surface.
    ReadyForReviewHandoff,
    /// The resolve-or-close sheet.
    ResolveCloseSheet,
    /// The blocked-or-escalate card.
    BlockedEscalateCard,
    /// The support / export packet.
    SupportExportPacket,
    /// The help / docs surface.
    HelpDocs,
}

impl M5ChangeIntentConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::WorkItemDetail,
        Self::StartWorkSheet,
        Self::LinkedChangePanel,
        Self::ReviewDetail,
        Self::ReadyForReviewHandoff,
        Self::ResolveCloseSheet,
        Self::BlockedEscalateCard,
        Self::SupportExportPacket,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkItemDetail => "work_item_detail",
            Self::StartWorkSheet => "start_work_sheet",
            Self::LinkedChangePanel => "linked_change_panel",
            Self::ReviewDetail => "review_detail",
            Self::ReadyForReviewHandoff => "ready_for_review_handoff",
            Self::ResolveCloseSheet => "resolve_close_sheet",
            Self::BlockedEscalateCard => "blocked_escalate_card",
            Self::SupportExportPacket => "support_export_packet",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no change-intent meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5ChangeIntentAccessibilityRoute {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a class has degraded below its qualified change-intent-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentDegradedReason {
    /// The provider ownership of the tracked item is unresolved.
    ProviderOwnershipUnresolved,
    /// The local-versus-provider commit state is unknown.
    LocalVersusProviderStateUnknown,
    /// The linked branch / worktree / review identity is unresolved.
    LinkedEngineeringIdentityUnresolved,
    /// One or more start-work side effects are undisclosed.
    SideEffectDisclosureIncomplete,
    /// The relation source of a linked change is unknown.
    RelationSourceUnknown,
    /// The blocker / resolution-authority state is unknown.
    BlockerStateUnknown,
}

impl M5ChangeIntentDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderOwnershipUnresolved,
        Self::LocalVersusProviderStateUnknown,
        Self::LinkedEngineeringIdentityUnresolved,
        Self::SideEffectDisclosureIncomplete,
        Self::RelationSourceUnknown,
        Self::BlockerStateUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOwnershipUnresolved => "provider_ownership_unresolved",
            Self::LocalVersusProviderStateUnknown => "local_versus_provider_state_unknown",
            Self::LinkedEngineeringIdentityUnresolved => "linked_engineering_identity_unresolved",
            Self::SideEffectDisclosureIncomplete => "side_effect_disclosure_incomplete",
            Self::RelationSourceUnknown => "relation_source_unknown",
            Self::BlockerStateUnknown => "blocker_state_unknown",
        }
    }
}

/// Mandatory label a claimed change-intent class must be able to show. The first three are hard requirements; the remaining three make the local-versus-provider commit state, the relation source, and the blocker state mechanically distinct for every covered class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's change-intent lifecycle role.
    LifecycleRole,
    /// The canonical per-domain descriptor the class points at.
    CanonicalReference,
    /// The local-versus-provider commit state the class must show.
    ProviderCommitState,
    /// The relation source the class must state.
    RelationSource,
    /// The blocker / resolution state the class must state.
    BlockerState,
}

impl M5ChangeIntentRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::LifecycleRole,
        Self::CanonicalReference,
        Self::ProviderCommitState,
        Self::RelationSource,
        Self::BlockerState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::LifecycleRole => "lifecycle_role",
            Self::CanonicalReference => "canonical_reference",
            Self::ProviderCommitState => "provider_commit_state",
            Self::RelationSource => "relation_source",
            Self::BlockerState => "blocker_state",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::LifecycleRole,
        Self::CanonicalReference,
    ];
}

/// Qualification class for an M5 change-intent row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentQualificationClass {
    /// Class change-intent handling qualifies for the Stable claim.
    Stable,
    /// Class change-intent handling is narrowed to Beta.
    Beta,
    /// Class change-intent handling is narrowed to Preview.
    Preview,
    /// Class change-intent handling is experimental and not claimed.
    Experimental,
    /// Class change-intent handling is unavailable on this build.
    Unavailable,
    /// Class change-intent handling is held pending review.
    Held,
}

impl M5ChangeIntentQualificationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Unavailable,
        Self::Held,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }
    /// Whether the class may carry a public Stable change-intent-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a change-intent class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentDowngradeTrigger {
    /// A start-work side effect (branch, worktree, review draft, or provider link) was created without disclosure.
    SilentSideEffectCreated,
    /// A local handoff packet or queued publish was shown as a provider-committed update.
    LocalHandoffShownAsProviderCommitted,
    /// Linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken were flattened into one relation badge.
    RelationSourcesFlattened,
    /// Tracked work was auto-resolved while an engineering blocker remained unresolved.
    AutoResolvedWithOpenBlocker,
    /// Local notes, a handoff packet, or linked evidence were dropped when a provider write failed.
    LocalEvidenceDroppedOnProviderWriteFailure,
    /// A class left its provider ownership unstated.
    ProviderOwnershipUnstated,
    /// A class left its local-versus-provider commit state unstated.
    LocalVersusProviderStateUnstated,
    /// A class left its linked branch / worktree / review identity unstated.
    LinkedEngineeringIdentityUnstated,
    /// A class left its relation source unstated.
    RelationSourceUnstated,
    /// A class left its blocker / resolution state unstated.
    BlockerStateUnstated,
    /// A class left its validation evidence unstated.
    ValidationEvidenceUnstated,
    /// The change-intent matrix packet has gone stale.
    ChangeIntentMatrixStale,
}

impl M5ChangeIntentDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::SilentSideEffectCreated,
        Self::LocalHandoffShownAsProviderCommitted,
        Self::RelationSourcesFlattened,
        Self::AutoResolvedWithOpenBlocker,
        Self::LocalEvidenceDroppedOnProviderWriteFailure,
        Self::ProviderOwnershipUnstated,
        Self::LocalVersusProviderStateUnstated,
        Self::LinkedEngineeringIdentityUnstated,
        Self::RelationSourceUnstated,
        Self::BlockerStateUnstated,
        Self::ValidationEvidenceUnstated,
        Self::ChangeIntentMatrixStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SilentSideEffectCreated => "silent_side_effect_created",
            Self::LocalHandoffShownAsProviderCommitted => {
                "local_handoff_shown_as_provider_committed"
            }
            Self::RelationSourcesFlattened => "relation_sources_flattened",
            Self::AutoResolvedWithOpenBlocker => "auto_resolved_with_open_blocker",
            Self::LocalEvidenceDroppedOnProviderWriteFailure => {
                "local_evidence_dropped_on_provider_write_failure"
            }
            Self::ProviderOwnershipUnstated => "provider_ownership_unstated",
            Self::LocalVersusProviderStateUnstated => "local_versus_provider_state_unstated",
            Self::LinkedEngineeringIdentityUnstated => "linked_engineering_identity_unstated",
            Self::RelationSourceUnstated => "relation_source_unstated",
            Self::BlockerStateUnstated => "blocker_state_unstated",
            Self::ValidationEvidenceUnstated => "validation_evidence_unstated",
            Self::ChangeIntentMatrixStale => "change_intent_matrix_stale",
        }
    }
}

/// Required visible state a class must carry so a change-intent result never reads without its provider
/// ownership, local-versus-provider state, linked engineering identity, or relation source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentVisibleState {
    /// Class / surface label shown on the surface (work-item detail, start-work sheet, linked-change panel, card).
    pub surface_label: String,
    /// Provider ownership of the tracked work item.
    pub provider_ownership: String,
    /// Local-versus-provider commit state disclosed before any committed-update claim.
    pub local_versus_provider_state: String,
    /// Linked branch / worktree / review identity bound to the change intent.
    pub linked_engineering_identity: String,
    /// Relation source (linked by provider, linked locally, suggested by Aureline, stale or broken).
    pub relation_source_state: String,
    /// Blocker / resolution state (ready to resolve, blocked by engineering, escalation open, awaiting provider write, resolution authority missing).
    pub blocker_state: String,
    /// Validation evidence and publish-later fallback packaged with a handoff.
    pub validation_evidence: String,
}

impl M5ChangeIntentVisibleState {
    /// `true` when every required visible-state field is present.
    fn is_complete(&self) -> bool {
        !self.surface_label.trim().is_empty()
            && !self.provider_ownership.trim().is_empty()
            && !self.local_versus_provider_state.trim().is_empty()
            && !self.linked_engineering_identity.trim().is_empty()
            && !self.relation_source_state.trim().is_empty()
            && !self.blocker_state.trim().is_empty()
            && !self.validation_evidence.trim().is_empty()
    }
}

/// One row in the matrix: one governed change-intent object class bound to the surface-specific
/// change-intent truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentRow {
    /// Governed change-intent object class.
    pub object_class: M5ChangeIntentObject,
    /// Qualification class earned by this class's change-intent handling.
    pub qualification: M5ChangeIntentQualificationClass,
    /// Commit state this row governs (distinguishes a provider-committed update from a local-only draft or a queued publish).
    pub commit_state: M5ChangeIntentCommitState,
    /// Owner role accountable for keeping this class's change-intent state governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required visible state that keeps this class's change-intent result visibly owned, provider-attributed, and commit-honest.
    pub required_visible_state: M5ChangeIntentVisibleState,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5ChangeIntentSurfaceFamily>,
    /// Classification stages this class passes through from intent capture to a recorded resolution.
    pub classification_stages: Vec<M5ChangeIntentClassificationStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5ChangeIntentRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ChangeIntentRequiredLabel>,
    /// Change-intent roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5ChangeIntentRole>,
    /// ChangeIntentRecord roles this class names (ChangeIntentRecord only).
    pub change_intent_record_roles: Vec<M5ChangeIntentRecordRole>,
    /// StartWorkSheet roles this class names (StartWorkSheet only).
    pub start_work_roles: Vec<M5ChangeIntentStartWorkRole>,
    /// LinkedChangePanel roles this class names (LinkedChangePanel only).
    pub linked_change_roles: Vec<M5ChangeIntentLinkedChangeRole>,
    /// ReadyForReviewHandoffSheet roles this class names (ReadyForReviewHandoffSheet only).
    pub handoff_roles: Vec<M5ChangeIntentHandoffRole>,
    /// ResolveCloseSheet roles this class names (ResolveCloseSheet only).
    pub resolve_roles: Vec<M5ChangeIntentResolveRole>,
    /// BlockedEscalateCard roles this class names (BlockedEscalateCard only).
    pub blocked_escalate_roles: Vec<M5ChangeIntentBlockedEscalateRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5ChangeIntentDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5ChangeIntentAccessibilityRoute>,
    /// First consumer surfaces that consume this class's change-intent projection.
    pub consumer_surfaces: Vec<M5ChangeIntentConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5ChangeIntentDowngradeTrigger>,
    /// Required closure-artifact refs that keep this class's change-intent state provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets start work silently create a branch, worktree, review draft, or provider link without separately disclosing each side effect. MUST be `false`.
    pub lets_start_work_silently_create_a_side_effect_without_disclosure: bool,
    /// Hard invariant: this class never lets a local handoff packet or queued publish masquerade as a provider-committed update. MUST be `false`.
    pub lets_a_local_handoff_packet_or_queued_publish_masquerade_as_a_provider_committed_update:
        bool,
    /// Hard invariant: this class never flattens linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken relation into one generic relation badge. MUST be `false`.
    pub flattens_linked_by_provider_linked_locally_suggested_and_stale_into_one_relation_badge:
        bool,
    /// Hard invariant: this class never auto-resolves tracked work while engineering blockers remain unresolved. MUST be `false`.
    pub auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved: bool,
    /// Hard invariant: this class never drops local notes, handoff packets, or linked evidence when provider write fails. MUST be `false`.
    pub drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails: bool,
}

impl M5ChangeIntentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ChangeIntentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ChangeIntentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_start_work_silently_create_a_side_effect_without_disclosure
            && !self.lets_a_local_handoff_packet_or_queued_publish_masquerade_as_a_provider_committed_update
            && !self.flattens_linked_by_provider_linked_locally_suggested_and_stale_into_one_relation_badge
            && !self.auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved
            && !self.drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Commit states tokens.
    pub commit_states: Vec<String>,
    /// Relation sources tokens.
    pub relation_sources: Vec<String>,
    /// Blocker states tokens.
    pub blocker_states: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// Change intent record roles tokens.
    pub change_intent_record_roles: Vec<String>,
    /// Start work roles tokens.
    pub start_work_roles: Vec<String>,
    /// Linked change roles tokens.
    pub linked_change_roles: Vec<String>,
    /// Handoff roles tokens.
    pub handoff_roles: Vec<String>,
    /// Resolve roles tokens.
    pub resolve_roles: Vec<String>,
    /// Blocked escalate roles tokens.
    pub blocked_escalate_roles: Vec<String>,
    /// Surface families tokens.
    pub surface_families: Vec<String>,
    /// Classification stages tokens.
    pub classification_stages: Vec<String>,
    /// Consumer surfaces tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility routes tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded reasons tokens.
    pub degraded_reasons: Vec<String>,
    /// Required labels tokens.
    pub required_labels: Vec<String>,
    /// Downgrade triggers tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5ChangeIntentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5ChangeIntentObject::ALL, |v| v.as_str()),
            commit_states: tokens(&M5ChangeIntentCommitState::ALL, |v| v.as_str()),
            relation_sources: tokens(&M5ChangeIntentRelationSource::ALL, |v| v.as_str()),
            blocker_states: tokens(&M5ChangeIntentBlockerState::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5ChangeIntentRole::ALL, |v| v.as_str()),
            change_intent_record_roles: tokens(&M5ChangeIntentRecordRole::ALL, |v| v.as_str()),
            start_work_roles: tokens(&M5ChangeIntentStartWorkRole::ALL, |v| v.as_str()),
            linked_change_roles: tokens(&M5ChangeIntentLinkedChangeRole::ALL, |v| v.as_str()),
            handoff_roles: tokens(&M5ChangeIntentHandoffRole::ALL, |v| v.as_str()),
            resolve_roles: tokens(&M5ChangeIntentResolveRole::ALL, |v| v.as_str()),
            blocked_escalate_roles: tokens(&M5ChangeIntentBlockedEscalateRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ChangeIntentSurfaceFamily::ALL, |v| v.as_str()),
            classification_stages: tokens(&M5ChangeIntentClassificationStage::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ChangeIntentConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ChangeIntentAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5ChangeIntentDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ChangeIntentRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5ChangeIntentDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5ChangeIntentGovernanceReview {
    /// No local handoff packet is shown as a provider committed update.
    pub no_local_handoff_packet_is_shown_as_a_provider_committed_update: bool,
    /// Every covered object class names owner backup owner and first consumer.
    pub every_covered_object_class_names_owner_backup_owner_and_first_consumer: bool,
    /// Provider committed state is mechanically distinct from local only draft.
    pub provider_committed_state_is_mechanically_distinct_from_local_only_draft: bool,
    /// Every change intent names its provider ownership.
    pub every_change_intent_names_its_provider_ownership: bool,
    /// Every start work sheet discloses each side effect separately.
    pub every_start_work_sheet_discloses_each_side_effect_separately: bool,
    /// Every linked change names its relation source.
    pub every_linked_change_names_its_relation_source: bool,
    /// No start work side effect is created without disclosure.
    pub no_start_work_side_effect_is_created_without_disclosure: bool,
    /// Every handoff discloses its publish later fallback.
    pub every_handoff_discloses_its_publish_later_fallback: bool,
    /// No tracked work is auto resolved while engineering blockers remain.
    pub no_tracked_work_is_auto_resolved_while_engineering_blockers_remain: bool,
    /// Every object declares classification stages.
    pub every_object_declares_classification_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single change intent source.
    pub support_export_reads_single_change_intent_source: bool,
    /// Work item start work review provider and support bind to single source.
    pub work_item_start_work_review_provider_and_support_bind_to_single_source: bool,
    /// Later rows cannot invent parallel change intent vocabulary.
    pub later_rows_cannot_invent_parallel_change_intent_vocabulary: bool,
    /// Change intent truth survives zoom and high contrast.
    pub change_intent_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentConsumerProjection {
    /// Work item detail and start work consume shared change intent truth.
    pub work_item_detail_and_start_work_consume_shared_change_intent_truth: bool,
    /// Ready for review handoff and provider handoff consume shared commit state truth.
    pub ready_for_review_handoff_and_provider_handoff_consume_shared_commit_state_truth: bool,
    /// Help and support export consume shared relation and blocker truth.
    pub help_and_support_export_consume_shared_relation_and_blocker_truth: bool,
    /// Docs help and screenshots read single change intent source.
    pub docs_help_and_screenshots_read_single_change_intent_source: bool,
    /// Change intents bind to shared linked change relation.
    pub change_intents_bind_to_shared_linked_change_relation: bool,
    /// Support export reads single change intent source.
    pub support_export_reads_single_change_intent_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the change-intent lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting change-intent audit for the lane.
    pub change_intent_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ChangeIntentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChangeIntentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Change-intent rows.
    pub change_intent_rows: Vec<M5ChangeIntentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ChangeIntentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ChangeIntentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChangeIntentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChangeIntentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ChangeIntentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 change-intent matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentMatrixPacket {
    /// Record kind; must equal [`M5_CHANGE_INTENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CHANGE_INTENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Change-intent rows.
    pub change_intent_rows: Vec<M5ChangeIntentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ChangeIntentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ChangeIntentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChangeIntentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChangeIntentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ChangeIntentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ChangeIntentMatrixPacket {
    /// Builds an M5 change-intent matrix packet from input.
    pub fn new(input: M5ChangeIntentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_CHANGE_INTENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_CHANGE_INTENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            change_intent_rows: input.change_intent_rows,
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

    /// Validates the M5 change-intent matrix invariants.
    pub fn validate(&self) -> Vec<M5ChangeIntentMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_CHANGE_INTENT_MATRIX_RECORD_KIND {
            violations.push(M5ChangeIntentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CHANGE_INTENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5ChangeIntentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ChangeIntentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_change_intent_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 change-intent matrix serializes"),
        ) {
            violations.push(M5ChangeIntentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 change-intent matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed change-intent class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,commit_state,owner,backup_owner,canonical_schema,surface_families,classification_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.change_intent_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.commit_state.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.backup_owner_role),
                row.object_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.classification_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic change-intent-health dashboard JSON that work-item and support surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .change_intent_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "commit_state": row.commit_state.as_str(),
                    "canonical_schema": row.object_class.canonical_domain_schema_ref(),
                    "classification_stages": row
                        .classification_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_change_intent_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_CHANGE_INTENT_ARTIFACT_REF,
            "classification_stages": self.vocabulary_set.classification_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 change-intent-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or work-item handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .change_intent_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Change Intent: Start-Work, Linked-Change, Ready-for-Review Handoff, Resolve/Close, and Blocked/Escalate Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.change_intent_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Change-intent roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Classification stages: {}\n",
            self.vocabulary_set.classification_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.change_intent_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (commit_state: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.commit_state.as_str()
            ));
            out.push_str(&format!(
                "  - Owner: {} (backup: {})\n",
                row.owner_role, row.backup_owner_role
            ));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.object_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Local vs provider state: {}\n",
                row.required_visible_state.local_versus_provider_state
            ));
            out.push_str(&format!(
                "  - Blocker state: {}\n",
                row.required_visible_state.blocker_state
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 change-intent matrix export.
#[derive(Debug)]
pub enum M5ChangeIntentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ChangeIntentMatrixViolation>),
}

impl fmt::Display for M5ChangeIntentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 change-intent matrix export parse failed: {error}"
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
                    "m5 change-intent matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ChangeIntentMatrixArtifactError {}

/// Validation failures emitted by [`M5ChangeIntentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ChangeIntentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed object class is missing from the matrix.
    RequiredObjectMissing,
    /// A change-intent row is incomplete.
    ChangeIntentRowIncomplete,
    /// A change-intent row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A change-intent row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no change-intent roles.
    SemanticRoleMissing,
    /// The ChangeIntentRecord class declares no ChangeIntentRecord roles.
    ChangeIntentRecordRoleMissing,
    /// The StartWorkSheet class declares no StartWorkSheet roles.
    StartWorkRoleMissing,
    /// The LinkedChangePanel class declares no LinkedChangePanel roles.
    LinkedChangeRoleMissing,
    /// The ReadyForReviewHandoffSheet class declares no ReadyForReviewHandoffSheet roles.
    HandoffRoleMissing,
    /// The ResolveCloseSheet class declares no ResolveCloseSheet roles.
    ResolveRoleMissing,
    /// The BlockedEscalateCard class declares no BlockedEscalateCard roles.
    BlockedEscalateRoleMissing,
    /// A class omits required visible-state fields.
    VisibleStateIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no classification stages.
    ClassificationStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard change-intent invariant.
    ChangeIntentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ChangeIntentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ChangeIntentRowIncomplete => "change_intent_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::ChangeIntentRecordRoleMissing => "change_intent_record_role_missing",
            Self::StartWorkRoleMissing => "start_work_role_missing",
            Self::LinkedChangeRoleMissing => "linked_change_role_missing",
            Self::HandoffRoleMissing => "handoff_role_missing",
            Self::ResolveRoleMissing => "resolve_role_missing",
            Self::BlockedEscalateRoleMissing => "blocked_escalate_role_missing",
            Self::VisibleStateIncomplete => "visible_state_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ClassificationStageMissing => "classification_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::ChangeIntentInvariantViolated => "change_intent_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 change-intent matrix export.
pub fn current_stable_m5_change_intent_matrix_export(
) -> Result<M5ChangeIntentMatrixPacket, M5ChangeIntentMatrixArtifactError> {
    let packet: M5ChangeIntentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-change-intent-proof/support_export.json"
    )))
    .map_err(M5ChangeIntentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ChangeIntentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ChangeIntentMatrixPacket,
    violations: &mut Vec<M5ChangeIntentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
        M5_CHANGE_INTENT_MATRIX_DOC_REF,
        M5_CHANGE_INTENT_DOMAIN_SCHEMA_REF,
        M5_START_WORK_SHEET_DOMAIN_SCHEMA_REF,
        M5_LINKED_CHANGE_PANEL_DOMAIN_SCHEMA_REF,
        M5_READY_FOR_REVIEW_HANDOFF_SHEET_DOMAIN_SCHEMA_REF,
        M5_RESOLVE_CLOSE_SHEET_DOMAIN_SCHEMA_REF,
        M5_BLOCKED_ESCALATE_CARD_DOMAIN_SCHEMA_REF,
        M5_WORK_ITEM_HANDOFF_PACKET_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ChangeIntentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ChangeIntentMatrixPacket,
    violations: &mut Vec<M5ChangeIntentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ChangeIntentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_change_intent_rows(
    packet: &M5ChangeIntentMatrixPacket,
    violations: &mut Vec<M5ChangeIntentMatrixViolation>,
) {
    let present: BTreeSet<M5ChangeIntentObject> = packet
        .change_intent_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5ChangeIntentObject::ALL {
        if !present.contains(&required) {
            violations.push(M5ChangeIntentMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.change_intent_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ChangeIntentMatrixViolation::ChangeIntentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ChangeIntentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5ChangeIntentMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_change_intent_record_roles() && row.change_intent_record_roles.is_empty()
        {
            violations.push(M5ChangeIntentMatrixViolation::ChangeIntentRecordRoleMissing);
        }
        if class.declares_start_work_roles() && row.start_work_roles.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::StartWorkRoleMissing);
        }
        if class.declares_linked_change_roles() && row.linked_change_roles.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::LinkedChangeRoleMissing);
        }
        if class.declares_handoff_roles() && row.handoff_roles.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::HandoffRoleMissing);
        }
        if class.declares_resolve_roles() && row.resolve_roles.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::ResolveRoleMissing);
        }
        if class.declares_blocked_escalate_roles() && row.blocked_escalate_roles.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::BlockedEscalateRoleMissing);
        }
        if !row.required_visible_state.is_complete() {
            violations.push(M5ChangeIntentMatrixViolation::VisibleStateIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.classification_stages.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::ClassificationStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations.push(M5ChangeIntentMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations.push(M5ChangeIntentMatrixViolation::ChangeIntentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ChangeIntentMatrixPacket,
    violations: &mut Vec<M5ChangeIntentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_local_handoff_packet_is_shown_as_a_provider_committed_update,
        review.every_covered_object_class_names_owner_backup_owner_and_first_consumer,
        review.provider_committed_state_is_mechanically_distinct_from_local_only_draft,
        review.every_change_intent_names_its_provider_ownership,
        review.every_start_work_sheet_discloses_each_side_effect_separately,
        review.every_linked_change_names_its_relation_source,
        review.no_start_work_side_effect_is_created_without_disclosure,
        review.every_handoff_discloses_its_publish_later_fallback,
        review.no_tracked_work_is_auto_resolved_while_engineering_blockers_remain,
        review.every_object_declares_classification_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_change_intent_source,
        review.work_item_start_work_review_provider_and_support_bind_to_single_source,
        review.later_rows_cannot_invent_parallel_change_intent_vocabulary,
        review.change_intent_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5ChangeIntentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ChangeIntentMatrixPacket,
    violations: &mut Vec<M5ChangeIntentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.work_item_detail_and_start_work_consume_shared_change_intent_truth,
        projection.ready_for_review_handoff_and_provider_handoff_consume_shared_commit_state_truth,
        projection.help_and_support_export_consume_shared_relation_and_blocker_truth,
        projection.docs_help_and_screenshots_read_single_change_intent_source,
        projection.change_intents_bind_to_shared_linked_change_relation,
        projection.support_export_reads_single_change_intent_source,
    ] {
        if !ok {
            violations.push(M5ChangeIntentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ChangeIntentMatrixPacket,
    violations: &mut Vec<M5ChangeIntentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ChangeIntentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ChangeIntentMatrixPacket,
    violations: &mut Vec<M5ChangeIntentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.change_intent_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ChangeIntentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses change / intent / provider / handoff / blocker words; what is rejected is a raw secret
/// *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
