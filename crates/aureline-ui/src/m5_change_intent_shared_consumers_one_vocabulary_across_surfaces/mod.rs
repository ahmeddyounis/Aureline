//! Shared work-item-detail, start-work-sheet, linked-change-panel, review-detail, ready-for-review-handoff,
//! resolve-close-sheet, blocked-escalate-card, and support / export consumers that keep the B153 change-intent
//! lifecycle objects — the change-intent record, the start-work sheet, the linked-change panel, the
//! ready-for-review handoff sheet, the resolve-or-close sheet, and the blocked-or-escalate card — at **one
//! canonical vocabulary** across every claimed M5 work-item, review, Git / worktree, AI-evidence, and
//! provider-backed team-workflow surface.
//!
//! This module is the consumer-adoption capstone for the six governed change-intent object classes frozen in
//! [`crate::m5_change_intent_and_engineering_lifecycle_matrix`] and implemented by the change-intent-record /
//! start-work-sheet lane ([`crate::m5_change_intent_record_and_start_work_registries`]) and the
//! lifecycle-state / reconcile-flow lane ([`crate::m5_change_intent_lifecycle_registries`]).
//!
//! It binds each shared change-intent object to the concrete work-item, review, Git / worktree, AI-evidence,
//! support / export, and browser / provider-handoff consumers — projected here through the work-item-detail,
//! start-work-sheet, linked-change-panel, review-detail, ready-for-review-handoff, resolve-close-sheet,
//! blocked-escalate-card, support-export, and help / docs surfaces — that render it, and proves — by fixtures,
//! not screenshots — that the same seeded tracked-item change-intent subject presents the same
//! change-intent-role, object, registry-reference, commit-state, surface-context, and relation-source
//! vocabulary wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the six shared change-intent objects must be adopted by at least two distinct
//!    consumers, so an object is proven to be shared change-intent infrastructure rather than a one-surface,
//!    feature-local fork of the change-intent record, start-work sheet, linked-change panel, ready-for-review
//!    handoff, resolve-or-close sheet, or blocked-or-escalate card contract.
//! 2. **One vocabulary / no drift.** For a given seeded tracked-item subject every consumer surface must
//!    present identical [`ChangeIntentSharedStateFacetValues`] — the same change-intent-role word, the same
//!    object word, the same registry-reference word, the same commit-state word, the same surface-context word,
//!    and the same relation-source word. The change-intent-role word must be a token from the frozen
//!    [`M5ChangeIntentRole`] vocabulary, so no surface rewrites `provider_ownership_disclosure`,
//!    `local_versus_provider_state_disclosure`, `linked_engineering_identity_disclosure`,
//!    `side_effect_disclosure`, `validation_evidence_disclosure`, `publish_later_fallback_disclosure`, or
//!    `final_resolution_authority_disclosure` in its own words. A surface may narrow *how much* it shows across
//!    desktop, compact, remote, and exported representations, but it may never reword the underlying vocabulary
//!    per surface, and a role that carries provider-ownership, local-versus-provider-state,
//!    linked-engineering-identity, or side-effect meaning may never let a local handoff packet or queued publish
//!    masquerade as a provider-committed update, silently create a branch / worktree / review draft / provider
//!    link without disclosure, flatten linked-by-provider, linked-locally, suggested-by-Aureline, and
//!    stale-or-broken into one relation badge, auto-resolve tracked work while engineering blockers remain, or
//!    drop local notes / handoff packets / linked evidence when a provider write fails.
//! 3. **Map back to one object.** Support / export consumers must point at the canonical per-domain schema and
//!    the frozen matrix by id, so an exported packet — and every copy / export / open-in-provider action — can
//!    always map a work-item / review / support surface back to one shared contract object rather than diverging
//!    into a surface-local payload or collapsing stable relation / source labels to generic prose.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an explicit
//! [`ChangeIntentSharedNarrowNote`] naming the reason, the preserved vocabulary, and the next action, and an
//! exported representation additionally names its export-safe detail boundary rather than collapsing the
//! subject out of view.
//!
//! The packet references upstream change-intent contracts by id rather than embedding their content. Raw secret
//! values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/teamwork/m5-change-intent-shared-consumers.schema.json`](../../../../schemas/teamwork/m5-change-intent-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/team-workflows/m5_change_intent_shared_consumers_one_vocabulary.md`](../../../../docs/team-workflows/m5_change_intent_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/teamwork/m5-change-intent-shared-consumers/`](../../../../fixtures/teamwork/m5-change-intent-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_change_intent_shared_consumers,
    seeded_m5_change_intent_shared_consumers_compact_remote_narrowed,
    seeded_m5_change_intent_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_change_intent_and_engineering_lifecycle_matrix::{
    M5ChangeIntentConsumerSurface, M5ChangeIntentObject, M5ChangeIntentRole,
    M5_CHANGE_INTENT_MATRIX_DOC_REF, M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ChangeIntentSharedConsumersPacket`].
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_change_intent_shared_consumer_registry_parity";

/// Schema version for change-intent shared-consumer parity records.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-change-intent-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/teamwork/m5-change-intent-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/team-workflows/m5_change_intent_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-change-intent-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-change-intent-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-change-intent-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/teamwork/m5-change-intent-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_CHANGE_INTENT_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Relation-source sentinel words a provider-ownership / local-versus-provider-state /
/// linked-engineering-identity / side-effect gate role may never fall back to; a gate-carrying role that
/// changes surface presentation must always keep a real relation-source-disclosed-and-commit-state-bound
/// continuity, never showing a stale or broken relation as provider-linked, a suggested relation as
/// provider-linked, a local-only draft as provider-committed, or a queued publish as provider-committed.
const RELATION_SOURCE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "stale_relation_shown_as_provider_linked",
    "suggested_relation_shown_as_provider_linked",
    "local_draft_shown_as_provider_committed",
    "queued_publish_shown_as_provider_committed",
];

/// Whether a consumer surface is an export / support path that must map an object back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(consumer: M5ChangeIntentConsumerSurface) -> bool {
    matches!(consumer, M5ChangeIntentConsumerSurface::SupportExportPacket)
}

/// Whether `token` is a member of the frozen [`M5ChangeIntentRole`] vocabulary.
///
/// This is the "one vocabulary" gate: a seeded subject's change-intent-role word must be a controlled role token
/// rather than a per-surface synonym.
pub fn is_known_change_intent_role_token(token: &str) -> bool {
    change_intent_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5ChangeIntentRole`], if it is one.
pub fn change_intent_role_from_token(token: &str) -> Option<M5ChangeIntentRole> {
    M5ChangeIntentRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared change-intent object a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed representation still
/// carries the same change-intent-role, object, registry-reference, commit-state, surface-context, and
/// relation-source words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentSharedRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl ChangeIntentSharedRepresentation {
    /// Every representation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopFull,
        Self::CompactNarrowed,
        Self::RemoteProjected,
        Self::ExportedRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompactNarrowed => "compact_narrowed",
            Self::RemoteProjected => "remote_projected",
            Self::ExportedRedacted => "exported_redacted",
        }
    }

    /// Whether this representation narrows below full desktop disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }
}

/// A vocabulary axis whose word must stay identical across surfaces for one subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentSharedStateFacet {
    /// The frozen change-intent-role word.
    ChangeIntentRoleWord,
    /// The change-intent-object word.
    ObjectWord,
    /// The canonical registry-reference word the object points at.
    RegistryReferenceWord,
    /// The commit-state word (provider-committed / local-only-draft / queued-for-publish /
    /// publish-failed-retained / provider-unavailable / offline-handoff-packet / stale-relative-to-provider)
    /// the subject ships.
    CommitStateWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The relation-source word paired with a provider-ownership / local-versus-provider-state /
    /// linked-engineering-identity / side-effect gate role.
    RelationSourceWord,
}

impl ChangeIntentSharedStateFacet {
    /// Every state facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ChangeIntentRoleWord,
        Self::ObjectWord,
        Self::RegistryReferenceWord,
        Self::CommitStateWord,
        Self::SurfaceContextWord,
        Self::RelationSourceWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeIntentRoleWord => "change_intent_role_word",
            Self::ObjectWord => "object_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::CommitStateWord => "commit_state_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::RelationSourceWord => "relation_source_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared change-intent object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentSharedNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl ChangeIntentSharedNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactionNarrowed => "compaction_narrowed",
            Self::RemoteProjectionNarrowed => "remote_projection_narrowed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentSharedNarrowNextAction {
    /// Expand the object in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl ChangeIntentSharedNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInDesktop => "expand_in_desktop",
            Self::OpenRemoteSource => "open_remote_source",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentSharedVocabularyState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl ChangeIntentSharedVocabularyState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed change-intent parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Change-intent vocabulary drifted between surfaces for the same subject.
    ChangeIntentVocabularyDriftDetected,
    /// A gate-carrying role dropped its relation-source or commit-state disclosure.
    RelationSourceOrCommitStateDisclosureDropped,
    /// A surface let a local handoff packet or queued publish masquerade as a provider-committed update.
    LetsALocalHandoffOrQueuedPublishMasqueradeAsAProviderCommittedUpdate,
    /// A surface silently created a branch, worktree, review draft, or provider link without disclosure.
    SilentlyCreatesABranchWorktreeReviewDraftOrProviderLinkWithoutDisclosure,
    /// A surface flattened linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken into one relation badge.
    FlattensLinkedByProviderLinkedLocallySuggestedAndStaleOrBrokenIntoOneRelationBadge,
    /// A surface auto-resolved tracked work while engineering blockers remained unresolved.
    AutoResolvesTrackedWorkWhileEngineeringBlockersRemainUnresolved,
    /// A surface dropped local notes, handoff packets, or linked evidence when a provider write failed.
    DropsLocalNotesHandoffPacketsOrLinkedEvidenceWhenProviderWriteFails,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared change-intent object narrowed.
    UpstreamChangeIntentNarrowed,
}

impl M5ChangeIntentSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ChangeIntentVocabularyDriftDetected,
        Self::RelationSourceOrCommitStateDisclosureDropped,
        Self::LetsALocalHandoffOrQueuedPublishMasqueradeAsAProviderCommittedUpdate,
        Self::SilentlyCreatesABranchWorktreeReviewDraftOrProviderLinkWithoutDisclosure,
        Self::FlattensLinkedByProviderLinkedLocallySuggestedAndStaleOrBrokenIntoOneRelationBadge,
        Self::AutoResolvesTrackedWorkWhileEngineeringBlockersRemainUnresolved,
        Self::DropsLocalNotesHandoffPacketsOrLinkedEvidenceWhenProviderWriteFails,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamChangeIntentNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ChangeIntentVocabularyDriftDetected => "change_intent_vocabulary_drift_detected",
            Self::RelationSourceOrCommitStateDisclosureDropped => {
                "relation_source_or_commit_state_disclosure_dropped"
            }
            Self::LetsALocalHandoffOrQueuedPublishMasqueradeAsAProviderCommittedUpdate => {
                "lets_a_local_handoff_or_queued_publish_masquerade_as_a_provider_committed_update"
            }
            Self::SilentlyCreatesABranchWorktreeReviewDraftOrProviderLinkWithoutDisclosure => {
                "silently_creates_a_branch_worktree_review_draft_or_provider_link_without_disclosure"
            }
            Self::FlattensLinkedByProviderLinkedLocallySuggestedAndStaleOrBrokenIntoOneRelationBadge => {
                "flattens_linked_by_provider_linked_locally_suggested_and_stale_or_broken_into_one_relation_badge"
            }
            Self::AutoResolvesTrackedWorkWhileEngineeringBlockersRemainUnresolved => {
                "auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved"
            }
            Self::DropsLocalNotesHandoffPacketsOrLinkedEvidenceWhenProviderWriteFails => {
                "drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamChangeIntentNarrowed => "upstream_change_intent_narrowed",
        }
    }
}

/// The controlled vocabulary a seeded change-intent subject presents.
///
/// These six words must be identical across every consumer surface that shows the same seeded subject. The
/// change-intent-role word must be a frozen role token; the rest are controlled words the subject's object
/// carries. A surface may narrow how much it renders, but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentSharedStateFacetValues {
    /// Change-intent-role word (must be a frozen [`M5ChangeIntentRole`] token).
    pub change_intent_role_word: String,
    /// Change-intent-object word.
    pub object_word: String,
    /// Canonical registry-reference word the object points at.
    pub registry_reference_word: String,
    /// Commit-state word (provider-committed / local-only-draft / queued-for-publish /
    /// publish-failed-retained / provider-unavailable / offline-handoff-packet / stale-relative-to-provider)
    /// the subject ships.
    pub commit_state_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Relation-source word paired with a provider-ownership / local-versus-provider-state /
    /// linked-engineering-identity / side-effect gate role.
    pub relation_source_word: String,
}

impl ChangeIntentSharedStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.change_intent_role_word.trim().is_empty()
            && !self.object_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.commit_state_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.relation_source_word.trim().is_empty()
    }

    /// Whether the change-intent-role word is a member of the frozen role vocabulary.
    pub fn change_intent_role_word_in_vocabulary(&self) -> bool {
        is_known_change_intent_role_token(self.change_intent_role_word.trim())
    }

    /// Whether the subject honours the relation-source rule: a role that carries provider-ownership,
    /// local-versus-provider-state, linked-engineering-identity, or side-effect meaning must pair its surface
    /// change with a real relation-source-disclosed-and-commit-state-bound continuity and never collapse to a
    /// stale-relation-shown-as-provider-linked, suggested-relation-shown-as-provider-linked,
    /// local-draft-shown-as-provider-committed, or queued-publish-shown-as-provider-committed sentinel.
    pub fn relation_source_satisfied(&self) -> bool {
        match change_intent_role_from_token(self.change_intent_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_a_change_intent_result() => {
                let relation = self.relation_source_word.trim().to_lowercase();
                !relation.is_empty()
                    && !RELATION_SOURCE_ABSENT_SENTINELS.contains(&relation.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentSharedNarrowNote {
    /// Why the representation narrowed.
    pub reason: ChangeIntentSharedNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: ChangeIntentSharedNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeIntentSharedRenderDisclosure {
    /// The vocabulary state the representation requires.
    pub vocabulary_state: ChangeIntentSharedVocabularyState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<ChangeIntentSharedNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<ChangeIntentSharedNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation narrows disclosure depth, a
/// remote-projected representation names its remote source, and an exported representation names its
/// export-safe-detail boundary — but all three keep every vocabulary word and disclose the narrowing through an
/// explicit note.
pub const fn resolve_change_intent_shared_render_disclosure(
    representation: ChangeIntentSharedRepresentation,
) -> ChangeIntentSharedRenderDisclosure {
    match representation {
        ChangeIntentSharedRepresentation::DesktopFull => ChangeIntentSharedRenderDisclosure {
            vocabulary_state: ChangeIntentSharedVocabularyState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        ChangeIntentSharedRepresentation::CompactNarrowed => ChangeIntentSharedRenderDisclosure {
            vocabulary_state: ChangeIntentSharedVocabularyState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ChangeIntentSharedNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(ChangeIntentSharedNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        ChangeIntentSharedRepresentation::RemoteProjected => ChangeIntentSharedRenderDisclosure {
            vocabulary_state: ChangeIntentSharedVocabularyState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ChangeIntentSharedNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(ChangeIntentSharedNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        ChangeIntentSharedRepresentation::ExportedRedacted => ChangeIntentSharedRenderDisclosure {
            vocabulary_state: ChangeIntentSharedVocabularyState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ChangeIntentSharedNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(ChangeIntentSharedNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared change-intent object rendered on one consumer surface in one representation for
/// one seeded change-intent subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentSharedConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable subject id (shared across surfaces that show the same subject).
    pub subject_id: String,
    /// Human-readable subject identity.
    pub subject_label: String,
    /// Which shared change-intent object this binding renders.
    pub object: M5ChangeIntentObject,
    /// Which consumer surface renders it.
    pub consumer: M5ChangeIntentConsumerSurface,
    /// Which representation this surface renders.
    pub representation: ChangeIntentSharedRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one subject).
    pub state_facets: ChangeIntentSharedStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub vocabulary_state: ChangeIntentSharedVocabularyState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<ChangeIntentSharedNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets a local handoff packet or queued publish masquerade as a provider-committed
    /// update. MUST be `false`.
    pub lets_a_local_handoff_or_queued_publish_masquerade_as_a_provider_committed_update: bool,
    /// Guardrail: this surface silently creates a branch, worktree, review draft, or provider link without
    /// separate disclosure. MUST be `false`.
    pub silently_creates_a_branch_worktree_review_draft_or_provider_link_without_disclosure: bool,
    /// Guardrail: this surface flattens linked-by-provider, linked-locally, suggested-by-Aureline, and
    /// stale-or-broken into one relation badge. MUST be `false`.
    pub flattens_linked_by_provider_linked_locally_suggested_and_stale_or_broken_into_one_relation_badge:
        bool,
    /// Guardrail: this surface auto-resolves tracked work while engineering blockers remain unresolved. MUST be
    /// `false`.
    pub auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved: bool,
    /// Guardrail: this surface drops local notes, handoff packets, or linked evidence when a provider write
    /// fails. MUST be `false`.
    pub drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ChangeIntentSharedConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> ChangeIntentSharedRenderDisclosure {
        resolve_change_intent_shared_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.lets_a_local_handoff_or_queued_publish_masquerade_as_a_provider_committed_update
            && !self.silently_creates_a_branch_worktree_review_draft_or_provider_link_without_disclosure
            && !self.flattens_linked_by_provider_linked_locally_suggested_and_stale_or_broken_into_one_relation_badge
            && !self.auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved
            && !self
                .drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.object.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_CHANGE_INTENT_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentSharedConsumersTrustReview {
    /// Object reuse is proven by fixtures rather than inferred from screenshots.
    pub object_reuse_proven_by_fixtures: bool,
    /// The same seeded subject presents the same vocabulary across surfaces.
    pub same_subject_same_change_intent_vocabulary_across_surfaces: bool,
    /// Every change-intent-role word is a frozen role token.
    pub change_intent_role_words_stay_in_frozen_vocabulary: bool,
    /// Gate-carrying roles never let a local-only draft read as a provider-committed update.
    pub gate_roles_never_let_local_draft_read_as_provider_committed: bool,
    /// A surface never creates a branch, worktree, review draft, or provider link without separate disclosure.
    pub side_effects_never_created_without_separate_disclosure: bool,
    /// A surface never flattens linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken
    /// into one relation badge.
    pub relation_sources_never_flattened_into_one_badge: bool,
    /// Tracked work is never auto-resolved while engineering blockers remain unresolved.
    pub tracked_work_never_auto_resolved_while_blockers_remain: bool,
    /// A surface never drops local notes, handoff packets, or linked evidence when a provider write fails.
    pub local_notes_handoff_and_linked_evidence_never_dropped_on_provider_write_failure: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Copy / export / open-in-provider actions preserve one canonical payload rather than diverging.
    pub copy_export_open_provider_preserve_one_payload: bool,
    /// Downgrade narrows the claim rather than hiding the object.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl M5ChangeIntentSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_reuse_proven_by_fixtures
            && self.same_subject_same_change_intent_vocabulary_across_surfaces
            && self.change_intent_role_words_stay_in_frozen_vocabulary
            && self.gate_roles_never_let_local_draft_read_as_provider_committed
            && self.side_effects_never_created_without_separate_disclosure
            && self.relation_sources_never_flattened_into_one_badge
            && self.tracked_work_never_auto_resolved_while_blockers_remain
            && self.local_notes_handoff_and_linked_evidence_never_dropped_on_provider_write_failure
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.copy_export_open_provider_preserve_one_payload
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentSharedConsumersProjection {
    /// The review-detail surface consumes the shared change-intent vocabulary.
    pub review_detail_consumes_shared_change_intent_vocabulary: bool,
    /// The start-work sheet consumes the shared change-intent vocabulary.
    pub start_work_sheet_consumes_shared_change_intent_vocabulary: bool,
    /// The linked-change panel consumes the shared change-intent vocabulary.
    pub linked_change_panel_consumes_shared_change_intent_vocabulary: bool,
    /// The ready-for-review handoff surface consumes the shared change-intent vocabulary.
    pub ready_for_review_handoff_consumes_shared_change_intent_vocabulary: bool,
    /// The work-item-detail surface consumes the shared change-intent vocabulary.
    pub work_item_detail_consumes_shared_change_intent_vocabulary: bool,
    /// The resolve-or-close sheet consumes the shared change-intent vocabulary.
    pub resolve_close_sheet_consumes_shared_change_intent_vocabulary: bool,
    /// The blocked-or-escalate card consumes the shared change-intent vocabulary.
    pub blocked_escalate_card_consumes_shared_change_intent_vocabulary: bool,
    /// The support / export packet consumes the shared change-intent vocabulary.
    pub support_export_packet_consumes_shared_change_intent_vocabulary: bool,
    /// The help / docs surface consumes the shared change-intent vocabulary.
    pub help_docs_consumes_shared_change_intent_vocabulary: bool,
    /// Every object is adopted by two or more consumers.
    pub every_object_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same seeded subject.
    pub change_intent_vocabulary_identical_for_same_subject: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps an object back to one shared contract object.
    pub export_maps_back_to_one_change_intent_object: bool,
}

impl M5ChangeIntentSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_detail_consumes_shared_change_intent_vocabulary
            && self.start_work_sheet_consumes_shared_change_intent_vocabulary
            && self.linked_change_panel_consumes_shared_change_intent_vocabulary
            && self.ready_for_review_handoff_consumes_shared_change_intent_vocabulary
            && self.work_item_detail_consumes_shared_change_intent_vocabulary
            && self.resolve_close_sheet_consumes_shared_change_intent_vocabulary
            && self.blocked_escalate_card_consumes_shared_change_intent_vocabulary
            && self.support_export_packet_consumes_shared_change_intent_vocabulary
            && self.help_docs_consumes_shared_change_intent_vocabulary
            && self.every_object_adopted_by_two_or_more_consumers
            && self.change_intent_vocabulary_identical_for_same_subject
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_change_intent_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ChangeIntentSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChangeIntentSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ChangeIntentSharedConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ChangeIntentSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ChangeIntentConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5ChangeIntentSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChangeIntentSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChangeIntentSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe change-intent shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeIntentSharedConsumersPacket {
    /// Record kind; must equal [`M5_CHANGE_INTENT_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CHANGE_INTENT_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ChangeIntentSharedConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ChangeIntentSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ChangeIntentConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5ChangeIntentSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChangeIntentSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChangeIntentSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ChangeIntentSharedConsumersPacket {
    /// Builds a change-intent shared-consumer packet from stable-lane input.
    pub fn new(input: M5ChangeIntentSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_CHANGE_INTENT_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_CHANGE_INTENT_SHARED_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the change-intent shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5ChangeIntentSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CHANGE_INTENT_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5ChangeIntentSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CHANGE_INTENT_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5ChangeIntentSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ChangeIntentSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5ChangeIntentSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ChangeIntentSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ChangeIntentSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5ChangeIntentSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5ChangeIntentSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("change-intent shared-consumer packet serializes"),
        ) {
            violations.push(M5ChangeIntentSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("change-intent shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object,consumer,representation,change_intent_role_word,vocabulary_state\n",
        );
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.change_intent_role_word,
                binding.vocabulary_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Shared Change-Intent Consumers: One Vocabulary Across Surfaces\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` on `{}`, representation `{}`, role `{}`\n",
                binding.subject_label,
                binding.binding_id,
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.change_intent_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in change-intent shared-consumer export.
#[derive(Debug)]
pub enum M5ChangeIntentSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ChangeIntentSharedConsumersViolation>),
}

impl fmt::Display for M5ChangeIntentSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "change-intent shared-consumer export parse failed: {error}"
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
                    "change-intent shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ChangeIntentSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5ChangeIntentSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ChangeIntentSharedConsumersViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's vocabulary values are incomplete.
    VocabularyFacetIncomplete,
    /// A binding's change-intent-role word is not a frozen role token.
    ChangeIntentRoleWordOutsideVocabulary,
    /// A binding's gate-carrying role dropped its relation source.
    RelationSourceMissingForGateRole,
    /// A binding's vocabulary state does not match its representation.
    VocabularyStateMismatch,
    /// Two surfaces show the same seeded subject with different vocabulary.
    ChangeIntentVocabularyDriftAcrossSurfaces,
    /// A shared object is not adopted by at least two distinct consumers.
    ObjectReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-vocabulary note.
    NarrowNotePreservedVocabularyMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding lets a local handoff packet or queued publish masquerade as a provider-committed update.
    LetsALocalHandoffOrQueuedPublishMasqueradeAsAProviderCommittedUpdate,
    /// A binding silently creates a branch, worktree, review draft, or provider link without disclosure.
    SilentlyCreatesABranchWorktreeReviewDraftOrProviderLinkWithoutDisclosure,
    /// A binding flattens linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken into one relation badge.
    FlattensLinkedByProviderLinkedLocallySuggestedAndStaleOrBrokenIntoOneRelationBadge,
    /// A binding auto-resolves tracked work while engineering blockers remain unresolved.
    AutoResolvesTrackedWorkWhileEngineeringBlockersRemainUnresolved,
    /// A binding drops local notes, handoff packets, or linked evidence when a provider write fails.
    DropsLocalNotesHandoffPacketsOrLinkedEvidenceWhenProviderWriteFails,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object appears among the bindings.
    ObjectCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ChangeIntentSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::VocabularyFacetIncomplete => "vocabulary_facet_incomplete",
            Self::ChangeIntentRoleWordOutsideVocabulary => "change_intent_role_word_outside_vocabulary",
            Self::RelationSourceMissingForGateRole => "relation_source_missing_for_gate_role",
            Self::VocabularyStateMismatch => "vocabulary_state_mismatch",
            Self::ChangeIntentVocabularyDriftAcrossSurfaces => {
                "change_intent_vocabulary_drift_across_surfaces"
            }
            Self::ObjectReuseUnproven => "object_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedVocabularyMissing => "narrow_note_preserved_vocabulary_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::LetsALocalHandoffOrQueuedPublishMasqueradeAsAProviderCommittedUpdate => {
                "lets_a_local_handoff_or_queued_publish_masquerade_as_a_provider_committed_update"
            }
            Self::SilentlyCreatesABranchWorktreeReviewDraftOrProviderLinkWithoutDisclosure => {
                "silently_creates_a_branch_worktree_review_draft_or_provider_link_without_disclosure"
            }
            Self::FlattensLinkedByProviderLinkedLocallySuggestedAndStaleOrBrokenIntoOneRelationBadge => {
                "flattens_linked_by_provider_linked_locally_suggested_and_stale_or_broken_into_one_relation_badge"
            }
            Self::AutoResolvesTrackedWorkWhileEngineeringBlockersRemainUnresolved => {
                "auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved"
            }
            Self::DropsLocalNotesHandoffPacketsOrLinkedEvidenceWhenProviderWriteFails => {
                "drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectCoverageMissing => "object_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable change-intent shared-consumer export.
pub fn current_stable_m5_change_intent_shared_consumers_export(
) -> Result<M5ChangeIntentSharedConsumersPacket, M5ChangeIntentSharedConsumersArtifactError> {
    let packet: M5ChangeIntentSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-change-intent-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5ChangeIntentSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ChangeIntentSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ChangeIntentSharedConsumersPacket,
    violations: &mut Vec<M5ChangeIntentSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_CHANGE_INTENT_SHARED_CONSUMERS_SCHEMA_REF,
        M5_CHANGE_INTENT_SHARED_CONSUMERS_DOC_REF,
        M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
        M5_CHANGE_INTENT_MATRIX_DOC_REF,
    ];
    // The six objects each map to their own canonical domain schema; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5ChangeIntentObject::ALL {
        domains.insert(object.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5ChangeIntentSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ChangeIntentSharedConsumersPacket,
    violations: &mut Vec<M5ChangeIntentSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5ChangeIntentSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders the same seeded
    // subject.
    let mut subject_facets: BTreeMap<&str, &ChangeIntentSharedStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object must be adopted by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5ChangeIntentObject,
        BTreeSet<M5ChangeIntentConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5ChangeIntentConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5ChangeIntentObject> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.subject_id.trim().is_empty()
            || binding.subject_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ChangeIntentSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5ChangeIntentSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding.state_facets.change_intent_role_word_in_vocabulary() {
            violations.push(
                M5ChangeIntentSharedConsumersViolation::ChangeIntentRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.relation_source_satisfied() {
            violations
                .push(M5ChangeIntentSharedConsumersViolation::RelationSourceMissingForGateRole);
        }

        let disclosure = binding.disclosure();

        if binding.vocabulary_state != disclosure.vocabulary_state {
            violations.push(M5ChangeIntentSharedConsumersViolation::VocabularyStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5ChangeIntentSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5ChangeIntentSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations
                            .push(M5ChangeIntentSharedConsumersViolation::NarrowNextActionMismatch);
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5ChangeIntentSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5ChangeIntentSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5ChangeIntentSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5ChangeIntentSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5ChangeIntentSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.lets_a_local_handoff_or_queued_publish_masquerade_as_a_provider_committed_update
        {
            violations.push(
                M5ChangeIntentSharedConsumersViolation::LetsALocalHandoffOrQueuedPublishMasqueradeAsAProviderCommittedUpdate,
            );
        }
        if binding
            .silently_creates_a_branch_worktree_review_draft_or_provider_link_without_disclosure
        {
            violations.push(
                M5ChangeIntentSharedConsumersViolation::SilentlyCreatesABranchWorktreeReviewDraftOrProviderLinkWithoutDisclosure,
            );
        }
        if binding.flattens_linked_by_provider_linked_locally_suggested_and_stale_or_broken_into_one_relation_badge {
            violations.push(
                M5ChangeIntentSharedConsumersViolation::FlattensLinkedByProviderLinkedLocallySuggestedAndStaleOrBrokenIntoOneRelationBadge,
            );
        }
        if binding.auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved {
            violations.push(
                M5ChangeIntentSharedConsumersViolation::AutoResolvesTrackedWorkWhileEngineeringBlockersRemainUnresolved,
            );
        }
        if binding.drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails {
            violations.push(
                M5ChangeIntentSharedConsumersViolation::DropsLocalNotesHandoffPacketsOrLinkedEvidenceWhenProviderWriteFails,
            );
        }

        // Support / export consumers must map an object back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5ChangeIntentSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Vocabulary-drift accumulation.
        match subject_facets.get(binding.subject_id.as_str()) {
            None => {
                subject_facets.insert(binding.subject_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5ChangeIntentSharedConsumersViolation::ChangeIntentVocabularyDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        object_consumers
            .entry(binding.object)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_objects.insert(binding.object);
    }

    // Coverage: every consumer surface and every object must appear.
    for consumer in M5ChangeIntentConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5ChangeIntentSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object in M5ChangeIntentObject::ALL {
        if !seen_objects.contains(&object) {
            violations.push(M5ChangeIntentSharedConsumersViolation::ObjectCoverageMissing);
            break;
        }
    }

    // Reuse: every present object must be adopted by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5ChangeIntentSharedConsumersViolation::ObjectReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
