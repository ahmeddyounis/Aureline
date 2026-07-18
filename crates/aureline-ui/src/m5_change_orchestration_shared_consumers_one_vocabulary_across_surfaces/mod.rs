//! Shared change-object-detail, patch-stack-queue, stack-edit-review-sheet, review-detail,
//! provider-merge-queue, portable-shelf, worktree-cleanup-preview, and support / export consumers that keep the
//! B154 change-orchestration objects — the change object, the patch stack / queue, the stack-edit / review
//! sheet, the landing-candidate sheet, the portable shelf / bundle, and the worktree cleanup preview — at **one
//! canonical vocabulary** across every claimed M5 Git / worktree, review, work-item, AI-branch-agent, help /
//! docs, support, and export surface.
//!
//! This module is the consumer-adoption capstone for the six governed change-orchestration object classes frozen
//! in [`crate::m5_change_object_patch_stack_and_landing_matrix`] and implemented by the change-object /
//! selected-change-binding lane ([`crate::m5_change_object_record_and_selected_change_binding_registries`]) and
//! the five sibling patch-stack, stack-edit, landing-candidate, worktree-manager, and portable-shelf implement
//! lanes that consume the same matrix.
//!
//! It binds each shared change-orchestration object to the concrete Git / worktree, review, work-item,
//! AI-branch-agent, support / export, and portable-handoff consumers — projected here through the
//! change-object-detail, patch-stack-queue, stack-edit-review-sheet, review-detail, provider-merge-queue,
//! portable-shelf, worktree-cleanup-preview, support-export, and help / docs surfaces — that render it, and
//! proves — by fixtures, not screenshots — that the same seeded change subject presents the same
//! change-orchestration-role, object, registry-reference, landing-state, surface-context, and
//! membership-source vocabulary wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the six shared change-orchestration objects must be adopted by at least two distinct
//!    consumers, so an object is proven to be shared change-orchestration infrastructure rather than a one-surface,
//!    feature-local fork of the change object, patch stack / queue, stack-edit / review sheet, landing-candidate
//!    sheet, portable shelf, or worktree cleanup preview contract.
//! 2. **One vocabulary / no drift.** For a given seeded change subject every consumer surface must present
//!    identical [`ChangeOrchestrationSharedStateFacetValues`] — the same change-orchestration-role word, the same
//!    object word, the same registry-reference word, the same landing-state word, the same surface-context word,
//!    and the same membership-source word. The change-orchestration-role word must be a token from the frozen
//!    [`M5ChangeOrchestrationRole`] vocabulary, so no surface rewrites `selected_change_object_disclosure`,
//!    `worktree_binding_disclosure`, `stack_membership_disclosure`, `landing_state_disclosure`,
//!    `validation_freshness_disclosure`, `rollback_export_fallback_disclosure`, or `cleanup_safety_disclosure`
//!    in its own words. A surface may narrow *how much* it shows across desktop, compact, remote, and exported
//!    representations, but it may never reword the underlying vocabulary per surface, and no surface may treat
//!    ambient branch state as a reviewed landing candidate, mutate another worktree without an explicit selected
//!    change object and worktree binding, infer stack membership from branch names alone, silently reorder /
//!    collapse / retarget stack members, or delete orphaned worktrees or stale members without previewing
//!    running tasks, open editors, uncommitted changes, recovery checkpoints, and export-safe evidence.
//! 3. **Map back to one object.** Support / export consumers must point at the canonical per-domain schema and
//!    the frozen matrix by id, so an exported packet — and every copy / export / open-in-provider action — can
//!    always map a Git / review / support surface back to one shared contract object rather than diverging into a
//!    surface-local payload or collapsing stable membership / landing labels to generic prose.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an explicit
//! [`ChangeOrchestrationSharedNarrowNote`] naming the reason, the preserved vocabulary, and the next action, and an
//! exported representation additionally names its export-safe detail boundary rather than collapsing the
//! subject out of view.
//!
//! The packet references upstream change-orchestration contracts by id rather than embedding their content. Raw secret
//! values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/teamwork/m5-change-orchestration-shared-consumers.schema.json`](../../../../schemas/teamwork/m5-change-orchestration-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/team-workflows/m5_change_orchestration_shared_consumers_one_vocabulary.md`](../../../../docs/team-workflows/m5_change_orchestration_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/teamwork/m5-change-orchestration-shared-consumers/`](../../../../fixtures/teamwork/m5-change-orchestration-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_change_orchestration_shared_consumers,
    seeded_m5_change_orchestration_shared_consumers_compact_remote_narrowed,
    seeded_m5_change_orchestration_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_change_object_patch_stack_and_landing_matrix::{
    M5ChangeOrchestrationConsumerSurface, M5ChangeOrchestrationObject, M5ChangeOrchestrationRole,
    M5_CHANGE_ORCHESTRATION_MATRIX_DOC_REF, M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ChangeOrchestrationSharedConsumersPacket`].
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_change_orchestration_shared_consumer_registry_parity";

/// Schema version for change-orchestration shared-consumer parity records.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-change-orchestration-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/teamwork/m5-change-orchestration-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/team-workflows/m5_change_orchestration_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-change-orchestration-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-change-orchestration-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-change-orchestration-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/teamwork/m5-change-orchestration-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Stack-membership-source sentinel words a selected-change-object / worktree-binding /
/// stack-membership / landing-state gate role may never fall back to; a gate-carrying role that changes
/// surface presentation must always keep a real membership-source-disclosed-and-worktree-binding-bound
/// continuity, never inferring stack membership from a branch name alone, showing ambient branch state as a
/// reviewed landing candidate, showing a stale stack member as queue-eligible, or letting a cross-worktree
/// write read as the selected change object.
const MEMBERSHIP_SOURCE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "membership_inferred_from_branch_name_alone",
    "ambient_branch_state_shown_as_reviewed_landing_candidate",
    "stale_member_shown_as_queue_eligible",
    "cross_worktree_write_shown_as_selected_change",
];

/// Whether a consumer surface is an export / support path that must map an object back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5ChangeOrchestrationConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5ChangeOrchestrationConsumerSurface::SupportExportPacket
    )
}

/// Whether `token` is a member of the frozen [`M5ChangeOrchestrationRole`] vocabulary.
///
/// This is the "one vocabulary" gate: a seeded subject's change-orchestration-role word must be a controlled role token
/// rather than a per-surface synonym.
pub fn is_known_change_orchestration_role_token(token: &str) -> bool {
    change_orchestration_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5ChangeOrchestrationRole`], if it is one.
pub fn change_orchestration_role_from_token(token: &str) -> Option<M5ChangeOrchestrationRole> {
    M5ChangeOrchestrationRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared change-orchestration object a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed representation still
/// carries the same change-orchestration-role, object, registry-reference, landing-state, surface-context, and
/// membership-source words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOrchestrationSharedRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl ChangeOrchestrationSharedRepresentation {
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
pub enum ChangeOrchestrationSharedStateFacet {
    /// The frozen change-orchestration-role word.
    ChangeOrchestrationRoleWord,
    /// The change-orchestration-object word.
    ObjectWord,
    /// The canonical registry-reference word the object points at.
    RegistryReferenceWord,
    /// The landing-state word (selected-change / stale-validation / restack-required / queue-eligible /
    /// queue-blocked / protected-branch-blocked / orphaned / abandoned / exported / imported-reopened) the
    /// subject ships.
    LandingStateWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The stack-membership-source word paired with a selected-change-object / worktree-binding /
    /// stack-membership / landing-state gate role.
    MembershipSourceWord,
}

impl ChangeOrchestrationSharedStateFacet {
    /// Every state facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ChangeOrchestrationRoleWord,
        Self::ObjectWord,
        Self::RegistryReferenceWord,
        Self::LandingStateWord,
        Self::SurfaceContextWord,
        Self::MembershipSourceWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeOrchestrationRoleWord => "change_orchestration_role_word",
            Self::ObjectWord => "object_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::LandingStateWord => "landing_state_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::MembershipSourceWord => "membership_source_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared change-orchestration object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOrchestrationSharedNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl ChangeOrchestrationSharedNarrowReason {
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
pub enum ChangeOrchestrationSharedNarrowNextAction {
    /// Expand the object in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl ChangeOrchestrationSharedNarrowNextAction {
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
pub enum ChangeOrchestrationSharedVocabularyState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl ChangeOrchestrationSharedVocabularyState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed change-orchestration parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Change-orchestration vocabulary drifted between surfaces for the same subject.
    ChangeOrchestrationVocabularyDriftDetected,
    /// A gate-carrying role dropped its membership-source or landing-state disclosure.
    MembershipSourceOrLandingStateDisclosureDropped,
    /// A surface treated ambient branch state as a reviewed landing candidate.
    TreatsAmbientBranchStateAsAReviewedLandingCandidate,
    /// A surface mutated another worktree without an explicit selected change object and worktree binding.
    MutatesAnotherWorktreeWithoutASelectedChangeObjectAndWorktreeBinding,
    /// A surface inferred stack membership from branch names alone.
    InfersStackMembershipFromBranchNamesAlone,
    /// A surface silently reordered, collapsed, or retargeted stack members.
    SilentlyReordersCollapsesOrRetargetsStackMembers,
    /// A surface deleted orphaned worktrees or stale members without previewing running work and recovery.
    DeletesOrphanedWorktreesOrStaleMembersWithoutPreviewingRunningWorkAndRecovery,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared change-orchestration object narrowed.
    UpstreamChangeOrchestrationNarrowed,
}

impl M5ChangeOrchestrationSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ChangeOrchestrationVocabularyDriftDetected,
        Self::MembershipSourceOrLandingStateDisclosureDropped,
        Self::TreatsAmbientBranchStateAsAReviewedLandingCandidate,
        Self::MutatesAnotherWorktreeWithoutASelectedChangeObjectAndWorktreeBinding,
        Self::InfersStackMembershipFromBranchNamesAlone,
        Self::SilentlyReordersCollapsesOrRetargetsStackMembers,
        Self::DeletesOrphanedWorktreesOrStaleMembersWithoutPreviewingRunningWorkAndRecovery,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamChangeOrchestrationNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ChangeOrchestrationVocabularyDriftDetected => "change_orchestration_vocabulary_drift_detected",
            Self::MembershipSourceOrLandingStateDisclosureDropped => {
                "membership_source_or_landing_state_disclosure_dropped"
            }
            Self::TreatsAmbientBranchStateAsAReviewedLandingCandidate => {
                "treats_ambient_branch_state_as_a_reviewed_landing_candidate"
            }
            Self::MutatesAnotherWorktreeWithoutASelectedChangeObjectAndWorktreeBinding => {
                "mutates_another_worktree_without_a_selected_change_object_and_worktree_binding"
            }
            Self::InfersStackMembershipFromBranchNamesAlone => {
                "infers_stack_membership_from_branch_names_alone"
            }
            Self::SilentlyReordersCollapsesOrRetargetsStackMembers => {
                "silently_reorders_collapses_or_retargets_stack_members"
            }
            Self::DeletesOrphanedWorktreesOrStaleMembersWithoutPreviewingRunningWorkAndRecovery => {
                "deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamChangeOrchestrationNarrowed => "upstream_change_orchestration_narrowed",
        }
    }
}

/// The controlled vocabulary a seeded change-orchestration subject presents.
///
/// These six words must be identical across every consumer surface that shows the same seeded subject. The
/// change-orchestration-role word must be a frozen role token; the rest are controlled words the subject's object
/// carries. A surface may narrow how much it renders, but it may never reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationSharedStateFacetValues {
    /// Change-orchestration-role word (must be a frozen [`M5ChangeOrchestrationRole`] token).
    pub change_orchestration_role_word: String,
    /// Change-orchestration-object word.
    pub object_word: String,
    /// Canonical registry-reference word the object points at.
    pub registry_reference_word: String,
    /// Landing-state word (selected-change / stale-validation / restack-required / queue-eligible /
    /// queue-blocked / protected-branch-blocked / orphaned / abandoned / exported / imported-reopened) the
    /// subject ships.
    pub landing_state_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Stack-membership-source word paired with a selected-change-object / worktree-binding /
    /// stack-membership / landing-state gate role.
    pub membership_source_word: String,
}

impl ChangeOrchestrationSharedStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.change_orchestration_role_word.trim().is_empty()
            && !self.object_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.landing_state_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.membership_source_word.trim().is_empty()
    }

    /// Whether the change-orchestration-role word is a member of the frozen role vocabulary.
    pub fn change_orchestration_role_word_in_vocabulary(&self) -> bool {
        is_known_change_orchestration_role_token(self.change_orchestration_role_word.trim())
    }

    /// Whether the subject honours the membership-source rule: a role that carries selected-change-object,
    /// worktree-binding, stack-membership, or landing-state meaning must pair its surface change with a real
    /// membership-source-disclosed-and-worktree-binding-bound continuity and never collapse to a
    /// membership-inferred-from-branch-name-alone, ambient-branch-state-shown-as-reviewed-landing-candidate,
    /// stale-member-shown-as-queue-eligible, or cross-worktree-write-shown-as-selected-change sentinel.
    pub fn membership_source_satisfied(&self) -> bool {
        match change_orchestration_role_from_token(self.change_orchestration_role_word.trim()) {
            Some(role)
                if role.must_be_present_before_surfacing_as_a_change_orchestration_result() =>
            {
                let membership = self.membership_source_word.trim().to_lowercase();
                !membership.is_empty()
                    && !MEMBERSHIP_SOURCE_ABSENT_SENTINELS.contains(&membership.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationSharedNarrowNote {
    /// Why the representation narrowed.
    pub reason: ChangeOrchestrationSharedNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: ChangeOrchestrationSharedNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeOrchestrationSharedRenderDisclosure {
    /// The vocabulary state the representation requires.
    pub vocabulary_state: ChangeOrchestrationSharedVocabularyState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<ChangeOrchestrationSharedNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<ChangeOrchestrationSharedNarrowNextAction>,
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
pub const fn resolve_change_orchestration_shared_render_disclosure(
    representation: ChangeOrchestrationSharedRepresentation,
) -> ChangeOrchestrationSharedRenderDisclosure {
    match representation {
        ChangeOrchestrationSharedRepresentation::DesktopFull => {
            ChangeOrchestrationSharedRenderDisclosure {
                vocabulary_state: ChangeOrchestrationSharedVocabularyState::FacetsPreserved,
                narrow_reason: None,
                narrow_next_action: None,
                needs_narrow_note: false,
                needs_remote_source_note: false,
                needs_export_detail_note: false,
            }
        }
        ChangeOrchestrationSharedRepresentation::CompactNarrowed => {
            ChangeOrchestrationSharedRenderDisclosure {
                vocabulary_state: ChangeOrchestrationSharedVocabularyState::FacetsDisclosedNarrowed,
                narrow_reason: Some(ChangeOrchestrationSharedNarrowReason::CompactionNarrowed),
                narrow_next_action: Some(
                    ChangeOrchestrationSharedNarrowNextAction::ExpandInDesktop,
                ),
                needs_narrow_note: true,
                needs_remote_source_note: false,
                needs_export_detail_note: false,
            }
        }
        ChangeOrchestrationSharedRepresentation::RemoteProjected => {
            ChangeOrchestrationSharedRenderDisclosure {
                vocabulary_state: ChangeOrchestrationSharedVocabularyState::FacetsDisclosedNarrowed,
                narrow_reason: Some(
                    ChangeOrchestrationSharedNarrowReason::RemoteProjectionNarrowed,
                ),
                narrow_next_action: Some(
                    ChangeOrchestrationSharedNarrowNextAction::OpenRemoteSource,
                ),
                needs_narrow_note: true,
                needs_remote_source_note: true,
                needs_export_detail_note: false,
            }
        }
        ChangeOrchestrationSharedRepresentation::ExportedRedacted => {
            ChangeOrchestrationSharedRenderDisclosure {
                vocabulary_state: ChangeOrchestrationSharedVocabularyState::FacetsDisclosedNarrowed,
                narrow_reason: Some(ChangeOrchestrationSharedNarrowReason::ExportRedactionNarrowed),
                narrow_next_action: Some(ChangeOrchestrationSharedNarrowNextAction::OpenFullDetail),
                needs_narrow_note: true,
                needs_remote_source_note: false,
                needs_export_detail_note: true,
            }
        }
    }
}

/// One consumer binding: a shared change-orchestration object rendered on one consumer surface in one representation for
/// one seeded change-orchestration subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationSharedConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable subject id (shared across surfaces that show the same subject).
    pub subject_id: String,
    /// Human-readable subject identity.
    pub subject_label: String,
    /// Which shared change-orchestration object this binding renders.
    pub object: M5ChangeOrchestrationObject,
    /// Which consumer surface renders it.
    pub consumer: M5ChangeOrchestrationConsumerSurface,
    /// Which representation this surface renders.
    pub representation: ChangeOrchestrationSharedRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one subject).
    pub state_facets: ChangeOrchestrationSharedStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub vocabulary_state: ChangeOrchestrationSharedVocabularyState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<ChangeOrchestrationSharedNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface treats ambient branch state as a reviewed landing candidate. MUST be `false`.
    pub treats_ambient_branch_state_as_a_reviewed_landing_candidate: bool,
    /// Guardrail: this surface mutates another worktree without an explicit selected change object and worktree
    /// binding. MUST be `false`.
    pub mutates_another_worktree_without_a_selected_change_object_and_worktree_binding: bool,
    /// Guardrail: this surface infers stack membership from branch names alone. MUST be `false`.
    pub infers_stack_membership_from_branch_names_alone: bool,
    /// Guardrail: this surface silently reorders, collapses, or retargets stack members. MUST be `false`.
    pub silently_reorders_collapses_or_retargets_stack_members: bool,
    /// Guardrail: this surface deletes orphaned worktrees or stale members without previewing running tasks,
    /// open editors, uncommitted changes, recovery checkpoints, and export-safe evidence. MUST be `false`.
    pub deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery:
        bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ChangeOrchestrationSharedConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> ChangeOrchestrationSharedRenderDisclosure {
        resolve_change_orchestration_shared_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.treats_ambient_branch_state_as_a_reviewed_landing_candidate
            && !self.mutates_another_worktree_without_a_selected_change_object_and_worktree_binding
            && !self.infers_stack_membership_from_branch_names_alone
            && !self.silently_reorders_collapses_or_retargets_stack_members
            && !self
                .deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery
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
                .any(|reference| reference == M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationSharedConsumersTrustReview {
    /// Object reuse is proven by fixtures rather than inferred from screenshots.
    pub object_reuse_proven_by_fixtures: bool,
    /// The same seeded subject presents the same vocabulary across surfaces.
    pub same_subject_same_change_orchestration_vocabulary_across_surfaces: bool,
    /// Every change-orchestration-role word is a frozen role token.
    pub change_orchestration_role_words_stay_in_frozen_vocabulary: bool,
    /// Gate-carrying roles never let ambient branch state read as a reviewed landing candidate.
    pub gate_roles_never_let_ambient_branch_read_as_landing_candidate: bool,
    /// A surface never mutates another worktree without an explicit selected change object and worktree binding.
    pub cross_worktree_writes_never_made_without_selected_change_binding: bool,
    /// A surface never infers stack membership from branch names alone.
    pub stack_membership_never_inferred_from_branch_names: bool,
    /// Stack members are never silently reordered, collapsed, or retargeted.
    pub stack_members_never_silently_reordered_collapsed_or_retargeted: bool,
    /// A surface never deletes orphaned worktrees or stale members without previewing running work and recovery.
    pub orphaned_worktrees_never_deleted_without_previewing_running_work_and_recovery: bool,
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

impl M5ChangeOrchestrationSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_reuse_proven_by_fixtures
            && self.same_subject_same_change_orchestration_vocabulary_across_surfaces
            && self.change_orchestration_role_words_stay_in_frozen_vocabulary
            && self.gate_roles_never_let_ambient_branch_read_as_landing_candidate
            && self.cross_worktree_writes_never_made_without_selected_change_binding
            && self.stack_membership_never_inferred_from_branch_names
            && self.stack_members_never_silently_reordered_collapsed_or_retargeted
            && self.orphaned_worktrees_never_deleted_without_previewing_running_work_and_recovery
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.copy_export_open_provider_preserve_one_payload
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationSharedConsumersProjection {
    /// The review-detail surface consumes the shared change-orchestration vocabulary.
    pub review_detail_consumes_shared_change_orchestration_vocabulary: bool,
    /// The start-work sheet consumes the shared change-orchestration vocabulary.
    pub patch_stack_queue_consumes_shared_change_orchestration_vocabulary: bool,
    /// The linked-change panel consumes the shared change-orchestration vocabulary.
    pub stack_edit_review_sheet_consumes_shared_change_orchestration_vocabulary: bool,
    /// The ready-for-review handoff surface consumes the shared change-orchestration vocabulary.
    pub provider_merge_queue_consumes_shared_change_orchestration_vocabulary: bool,
    /// The work-item-detail surface consumes the shared change-orchestration vocabulary.
    pub change_object_detail_consumes_shared_change_orchestration_vocabulary: bool,
    /// The resolve-or-close sheet consumes the shared change-orchestration vocabulary.
    pub portable_shelf_consumes_shared_change_orchestration_vocabulary: bool,
    /// The blocked-or-escalate card consumes the shared change-orchestration vocabulary.
    pub worktree_cleanup_preview_consumes_shared_change_orchestration_vocabulary: bool,
    /// The support / export packet consumes the shared change-orchestration vocabulary.
    pub support_export_packet_consumes_shared_change_orchestration_vocabulary: bool,
    /// The help / docs surface consumes the shared change-orchestration vocabulary.
    pub help_docs_consumes_shared_change_orchestration_vocabulary: bool,
    /// Every object is adopted by two or more consumers.
    pub every_object_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same seeded subject.
    pub change_orchestration_vocabulary_identical_for_same_subject: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps an object back to one shared contract object.
    pub export_maps_back_to_one_change_orchestration_object: bool,
}

impl M5ChangeOrchestrationSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_detail_consumes_shared_change_orchestration_vocabulary
            && self.patch_stack_queue_consumes_shared_change_orchestration_vocabulary
            && self.stack_edit_review_sheet_consumes_shared_change_orchestration_vocabulary
            && self.provider_merge_queue_consumes_shared_change_orchestration_vocabulary
            && self.change_object_detail_consumes_shared_change_orchestration_vocabulary
            && self.portable_shelf_consumes_shared_change_orchestration_vocabulary
            && self.worktree_cleanup_preview_consumes_shared_change_orchestration_vocabulary
            && self.support_export_packet_consumes_shared_change_orchestration_vocabulary
            && self.help_docs_consumes_shared_change_orchestration_vocabulary
            && self.every_object_adopted_by_two_or_more_consumers
            && self.change_orchestration_vocabulary_identical_for_same_subject
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_change_orchestration_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ChangeOrchestrationSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChangeOrchestrationSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ChangeOrchestrationSharedConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ChangeOrchestrationSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ChangeOrchestrationConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5ChangeOrchestrationSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChangeOrchestrationSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChangeOrchestrationSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe change-orchestration shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChangeOrchestrationSharedConsumersPacket {
    /// Record kind; must equal [`M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ChangeOrchestrationSharedConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ChangeOrchestrationSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ChangeOrchestrationConsumerSurface>,
    /// Trust review block.
    pub trust_review: M5ChangeOrchestrationSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ChangeOrchestrationSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ChangeOrchestrationSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ChangeOrchestrationSharedConsumersPacket {
    /// Builds a change-orchestration shared-consumer packet from stable-lane input.
    pub fn new(input: M5ChangeOrchestrationSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the change-orchestration shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5ChangeOrchestrationSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(M5ChangeOrchestrationSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5ChangeOrchestrationSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(M5ChangeOrchestrationSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("change-orchestration shared-consumer packet serializes"),
        ) {
            violations
                .push(M5ChangeOrchestrationSharedConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("change-orchestration shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object,consumer,representation,change_orchestration_role_word,vocabulary_state\n",
        );
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.object.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.change_orchestration_role_word,
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
        out.push_str("# Shared Change-Orchestration Consumers: One Vocabulary Across Surfaces\n\n");
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
                binding.state_facets.change_orchestration_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in change-orchestration shared-consumer export.
#[derive(Debug)]
pub enum M5ChangeOrchestrationSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ChangeOrchestrationSharedConsumersViolation>),
}

impl fmt::Display for M5ChangeOrchestrationSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "change-orchestration shared-consumer export parse failed: {error}"
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
                    "change-orchestration shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ChangeOrchestrationSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5ChangeOrchestrationSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ChangeOrchestrationSharedConsumersViolation {
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
    /// A binding's change-orchestration-role word is not a frozen role token.
    ChangeOrchestrationRoleWordOutsideVocabulary,
    /// A binding's gate-carrying role dropped its stack-membership source.
    MembershipSourceMissingForGateRole,
    /// A binding's vocabulary state does not match its representation.
    VocabularyStateMismatch,
    /// Two surfaces show the same seeded subject with different vocabulary.
    ChangeOrchestrationVocabularyDriftAcrossSurfaces,
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
    /// A binding treats ambient branch state as a reviewed landing candidate.
    TreatsAmbientBranchStateAsAReviewedLandingCandidate,
    /// A binding mutates another worktree without an explicit selected change object and worktree binding.
    MutatesAnotherWorktreeWithoutASelectedChangeObjectAndWorktreeBinding,
    /// A binding infers stack membership from branch names alone.
    InfersStackMembershipFromBranchNamesAlone,
    /// A binding silently reorders, collapses, or retargets stack members.
    SilentlyReordersCollapsesOrRetargetsStackMembers,
    /// A binding deletes orphaned worktrees or stale members without previewing running work and recovery.
    DeletesOrphanedWorktreesOrStaleMembersWithoutPreviewingRunningWorkAndRecovery,
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

impl M5ChangeOrchestrationSharedConsumersViolation {
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
            Self::ChangeOrchestrationRoleWordOutsideVocabulary => "change_orchestration_role_word_outside_vocabulary",
            Self::MembershipSourceMissingForGateRole => "membership_source_missing_for_gate_role",
            Self::VocabularyStateMismatch => "vocabulary_state_mismatch",
            Self::ChangeOrchestrationVocabularyDriftAcrossSurfaces => {
                "change_orchestration_vocabulary_drift_across_surfaces"
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
            Self::TreatsAmbientBranchStateAsAReviewedLandingCandidate => {
                "treats_ambient_branch_state_as_a_reviewed_landing_candidate"
            }
            Self::MutatesAnotherWorktreeWithoutASelectedChangeObjectAndWorktreeBinding => {
                "mutates_another_worktree_without_a_selected_change_object_and_worktree_binding"
            }
            Self::InfersStackMembershipFromBranchNamesAlone => {
                "infers_stack_membership_from_branch_names_alone"
            }
            Self::SilentlyReordersCollapsesOrRetargetsStackMembers => {
                "silently_reorders_collapses_or_retargets_stack_members"
            }
            Self::DeletesOrphanedWorktreesOrStaleMembersWithoutPreviewingRunningWorkAndRecovery => {
                "deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery"
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

/// Reads and validates the checked-in stable change-orchestration shared-consumer export.
pub fn current_stable_m5_change_orchestration_shared_consumers_export() -> Result<
    M5ChangeOrchestrationSharedConsumersPacket,
    M5ChangeOrchestrationSharedConsumersArtifactError,
> {
    let packet: M5ChangeOrchestrationSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-change-orchestration-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5ChangeOrchestrationSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ChangeOrchestrationSharedConsumersArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ChangeOrchestrationSharedConsumersPacket,
    violations: &mut Vec<M5ChangeOrchestrationSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_SCHEMA_REF,
        M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_DOC_REF,
        M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
        M5_CHANGE_ORCHESTRATION_MATRIX_DOC_REF,
    ];
    // The six objects each map to their own canonical domain schema; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5ChangeOrchestrationObject::ALL {
        domains.insert(object.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ChangeOrchestrationSharedConsumersPacket,
    violations: &mut Vec<M5ChangeOrchestrationSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5ChangeOrchestrationSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders the same seeded
    // subject.
    let mut subject_facets: BTreeMap<&str, &ChangeOrchestrationSharedStateFacetValues> =
        BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object must be adopted by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5ChangeOrchestrationObject,
        BTreeSet<M5ChangeOrchestrationConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5ChangeOrchestrationConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5ChangeOrchestrationObject> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.subject_id.trim().is_empty()
            || binding.subject_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations
                .push(M5ChangeOrchestrationSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding
            .state_facets
            .change_orchestration_role_word_in_vocabulary()
        {
            violations.push(
                M5ChangeOrchestrationSharedConsumersViolation::ChangeOrchestrationRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.membership_source_satisfied() {
            violations.push(
                M5ChangeOrchestrationSharedConsumersViolation::MembershipSourceMissingForGateRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.vocabulary_state != disclosure.vocabulary_state {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::VocabularyStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations
                        .push(M5ChangeOrchestrationSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations.push(
                            M5ChangeOrchestrationSharedConsumersViolation::NarrowReasonMismatch,
                        );
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5ChangeOrchestrationSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5ChangeOrchestrationSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5ChangeOrchestrationSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.treats_ambient_branch_state_as_a_reviewed_landing_candidate {
            violations.push(
                M5ChangeOrchestrationSharedConsumersViolation::TreatsAmbientBranchStateAsAReviewedLandingCandidate,
            );
        }
        if binding.mutates_another_worktree_without_a_selected_change_object_and_worktree_binding {
            violations.push(
                M5ChangeOrchestrationSharedConsumersViolation::MutatesAnotherWorktreeWithoutASelectedChangeObjectAndWorktreeBinding,
            );
        }
        if binding.infers_stack_membership_from_branch_names_alone {
            violations.push(
                M5ChangeOrchestrationSharedConsumersViolation::InfersStackMembershipFromBranchNamesAlone,
            );
        }
        if binding.silently_reorders_collapses_or_retargets_stack_members {
            violations.push(
                M5ChangeOrchestrationSharedConsumersViolation::SilentlyReordersCollapsesOrRetargetsStackMembers,
            );
        }
        if binding.deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery {
            violations.push(
                M5ChangeOrchestrationSharedConsumersViolation::DeletesOrphanedWorktreesOrStaleMembersWithoutPreviewingRunningWorkAndRecovery,
            );
        }

        // Support / export consumers must map an object back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5ChangeOrchestrationSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Vocabulary-drift accumulation.
        match subject_facets.get(binding.subject_id.as_str()) {
            None => {
                subject_facets.insert(binding.subject_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5ChangeOrchestrationSharedConsumersViolation::ChangeOrchestrationVocabularyDriftAcrossSurfaces,
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
    for consumer in M5ChangeOrchestrationConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object in M5ChangeOrchestrationObject::ALL {
        if !seen_objects.contains(&object) {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::ObjectCoverageMissing);
            break;
        }
    }

    // Reuse: every present object must be adopted by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5ChangeOrchestrationSharedConsumersViolation::ObjectReuseUnproven);
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
