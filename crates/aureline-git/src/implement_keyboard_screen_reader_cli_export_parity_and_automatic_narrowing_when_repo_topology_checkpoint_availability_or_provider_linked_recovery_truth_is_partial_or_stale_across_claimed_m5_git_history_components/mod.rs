//! Keyboard, screen-reader, CLI, and export parity plus automatic claim narrowing
//! for the twelve shared M5 Git-history and risky-mutation components.
//!
//! This module is the accessibility / headless / export capstone over the
//! Git-history and sequence components frozen in
//! [`crate::freeze_the_m5_git_history_sequence_component_matrix`], implemented by the
//! commit-graph / history-graph / branch-comparison / worktree identity lane, the
//! stash-entry / reflog-recovery lane, the rebase-todo / sequence-editor lane, and
//! the cherry-pick / revert / patch-apply / conflict-checkpoint / force-push
//! mutation-review lane, and adopted by the shared consumers in
//! [`crate::add_shared_history_sidebar_review_workspace_command_help_support_and_export_consumers_so_git_history_components_keep_ref_worktree_recovery_language_aligned`].
//! Where the consumer lane proves ref / worktree / recovery / verb parity across
//! desktop surfaces, this lane proves the harder claim: that commit / worktree /
//! stash / sequence / patch / recovery state is exposed just as honestly in
//! assistive, headless, and exported forms as it is on desktop — and that a
//! claim-bearing component automatically narrows the moment repo topology, checkpoint
//! availability, or provider-linked recovery truth stops being trustworthy.
//!
//! The honesty axes are two. First, parity across forms: every claimed component must
//! expose a keyboard label, a screen-reader label, a CLI enum token, an export enum
//! token, and a human-readable explanation field, and must render on the desktop, the
//! headless CLI, and the support export alike. No component may be pointer-only,
//! export-opaque, or semantically stronger on the desktop than it is in CLI or support
//! output.
//!
//! Second, automatic narrowing: each component carries a claim about how safe and
//! recoverable its Git-history truth is, drawn from [`GitHistoryClaimTier`]. When repo
//! topology is shallow / partial / sparse, when no checkpoint exists and only a
//! reflog-only fallback recovery is available, when provider-linked review state is
//! stale and cannot safely claim publish / approval parity, or when the surface is
//! offline / local-only, the claim must narrow to the ceiling permitted by that
//! condition ([`GitHistoryClaimCondition::permitted_ceiling`]), disclose the narrowing
//! through an explicit trigger and next action, keep the incomplete-history and
//! recovery destination spelled out, and preserve local-only continuation. A component
//! may never keep asserting full recoverable-in-product safety while one of those
//! conditions holds.
//!
//! The packet references upstream component and consumer contracts by id rather than
//! embedding their content. Raw paths, raw object bytes, raw branch names, raw
//! patch/reflog/stash bodies, raw provider payloads, and credentials stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-git-history-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-git-history-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/git/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_repo_topology_checkpoint_availability_or_provider_linked_recovery_truth_is_partial_or_stale_across_claimed_m5_git_history_components.md`](../../../../docs/git/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_repo_topology_checkpoint_availability_or_provider_linked_recovery_truth_is_partial_or_stale_across_claimed_m5_git_history_components.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-git-history-component-accessibility-parity/`](../../../../fixtures/ui/m5-git-history-component-accessibility-parity/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_git_history_sequence_component_matrix::M5GitHistoryComponent;

/// Stable record-kind tag carried by [`GitHistoryComponentAccessibilityPacket`].
pub const GIT_HISTORY_ACCESSIBILITY_RECORD_KIND: &str =
    "git_history_component_accessibility_parity_truth";

/// Schema version for Git-history component accessibility parity records.
pub const GIT_HISTORY_ACCESSIBILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GIT_HISTORY_ACCESSIBILITY_SCHEMA_REF: &str =
    "schemas/ui/m5-git-history-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const GIT_HISTORY_ACCESSIBILITY_DOC_REF: &str =
    "docs/git/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_repo_topology_checkpoint_availability_or_provider_linked_recovery_truth_is_partial_or_stale_across_claimed_m5_git_history_components.md";

/// Repo-relative path of the frozen component matrix these claims exercise.
pub const GIT_HISTORY_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-sequence-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this capstone extends.
pub const GIT_HISTORY_ACCESSIBILITY_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-component-consumer.schema.json";

/// Repo-relative path of the commit-graph / history-graph / branch-comparison /
/// worktree identity component contract.
pub const GIT_HISTORY_ACCESSIBILITY_IDENTITY_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-identity-component.schema.json";

/// Repo-relative path of the stash-entry / reflog-recovery component contract.
pub const GIT_HISTORY_ACCESSIBILITY_STASH_RECOVERY_CONTRACT_REF: &str =
    "schemas/ui/m5-stash-reflog-recovery-component.schema.json";

/// Repo-relative path of the rebase-todo / sequence-editor component contract.
pub const GIT_HISTORY_ACCESSIBILITY_SEQUENCE_EDIT_CONTRACT_REF: &str =
    "schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json";

/// Repo-relative path of the cherry-pick / revert / patch-apply / conflict-checkpoint
/// / force-push mutation-review component contract.
pub const GIT_HISTORY_ACCESSIBILITY_MUTATION_REVIEW_CONTRACT_REF: &str =
    "schemas/ui/m5-git-mutation-review-recovery-component.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const GIT_HISTORY_ACCESSIBILITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-git-history-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact.
pub const GIT_HISTORY_ACCESSIBILITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-git-history-component-accessibility-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const GIT_HISTORY_ACCESSIBILITY_SUMMARY_REF: &str =
    "artifacts/release/m5-git-history-component-accessibility-proof/summary.md";

/// Canonical component contract that a row must point at for a given component.
///
/// Each of the twelve shared components resolves to the checked-in schema of the
/// implement lane that owns it: the commit-graph / history-graph / branch-comparison
/// / worktree identity lane, the stash-entry / reflog-recovery lane, the rebase-todo
/// / sequence-editor lane, or the cherry-pick / revert / patch-apply /
/// conflict-checkpoint / force-push mutation-review lane.
pub const fn component_canonical_schema_ref(component: M5GitHistoryComponent) -> &'static str {
    match component {
        M5GitHistoryComponent::CommitGraphHeader
        | M5GitHistoryComponent::HistoryGraphRow
        | M5GitHistoryComponent::BranchComparisonChip
        | M5GitHistoryComponent::WorktreeRow => GIT_HISTORY_ACCESSIBILITY_IDENTITY_CONTRACT_REF,
        M5GitHistoryComponent::StashEntry | M5GitHistoryComponent::ReflogRecoveryBanner => {
            GIT_HISTORY_ACCESSIBILITY_STASH_RECOVERY_CONTRACT_REF
        }
        M5GitHistoryComponent::RebaseTodoRow | M5GitHistoryComponent::SequenceEditorHeader => {
            GIT_HISTORY_ACCESSIBILITY_SEQUENCE_EDIT_CONTRACT_REF
        }
        M5GitHistoryComponent::CherryPickRevertReviewSheet
        | M5GitHistoryComponent::PatchApplyReviewSheet
        | M5GitHistoryComponent::ConflictCheckpointCard
        | M5GitHistoryComponent::ForcePushReviewDialog => {
            GIT_HISTORY_ACCESSIBILITY_MUTATION_REVIEW_CONTRACT_REF
        }
    }
}

/// The condition governing how trustworthy a component's Git-history safety claim is.
///
/// [`LocalTruthAligned`](Self::LocalTruthAligned) is the baseline where the full
/// recoverable-in-product claim is permitted. The other four are the weakening
/// conditions named by the spec: a stale provider-linked review state, partial repo
/// topology, unavailable checkpoint recovery, and an offline / local-only surface.
/// Each weakening condition pins the claim to a ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryClaimCondition {
    /// Local Git truth is aligned; the full recoverable-in-product claim is permitted.
    LocalTruthAligned,
    /// Provider-linked review state is stale; publish / approval parity cannot be claimed.
    ProviderReviewStateStale,
    /// Repo topology is shallow / partial / sparse; the history shown is incomplete.
    RepoTopologyPartial,
    /// No checkpoint exists; only a reflog-only fallback recovery is available.
    CheckpointRecoveryUnavailable,
    /// The surface is offline / local-only; provider handoff is unavailable.
    OfflineLocalOnly,
}

impl GitHistoryClaimCondition {
    /// Every condition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalTruthAligned,
        Self::ProviderReviewStateStale,
        Self::RepoTopologyPartial,
        Self::CheckpointRecoveryUnavailable,
        Self::OfflineLocalOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTruthAligned => "local_truth_aligned",
            Self::ProviderReviewStateStale => "provider_review_state_stale",
            Self::RepoTopologyPartial => "repo_topology_partial",
            Self::CheckpointRecoveryUnavailable => "checkpoint_recovery_unavailable",
            Self::OfflineLocalOnly => "offline_local_only",
        }
    }

    /// Whether this condition weakens the safety claim (everything but aligned truth).
    pub const fn is_weakening(self) -> bool {
        !matches!(self, Self::LocalTruthAligned)
    }

    /// The strongest claim tier this condition still permits.
    pub const fn permitted_ceiling(self) -> GitHistoryClaimTier {
        match self {
            Self::LocalTruthAligned => GitHistoryClaimTier::RecoverableInProduct,
            Self::ProviderReviewStateStale => GitHistoryClaimTier::LocallyRecoverable,
            Self::RepoTopologyPartial => GitHistoryClaimTier::PartialHistoryOnly,
            Self::CheckpointRecoveryUnavailable => GitHistoryClaimTier::ReflogOnlyRecovery,
            Self::OfflineLocalOnly => GitHistoryClaimTier::LocalContinueOnly,
        }
    }

    /// The downgrade trigger a weakening condition must disclose, if any.
    pub const fn default_trigger(self) -> Option<GitHistoryAccessibilityDowngradeTrigger> {
        match self {
            Self::LocalTruthAligned => None,
            Self::ProviderReviewStateStale => {
                Some(GitHistoryAccessibilityDowngradeTrigger::ProviderReviewStateStale)
            }
            Self::RepoTopologyPartial => {
                Some(GitHistoryAccessibilityDowngradeTrigger::RepoTopologyPartial)
            }
            Self::CheckpointRecoveryUnavailable => {
                Some(GitHistoryAccessibilityDowngradeTrigger::CheckpointRecoveryUnavailable)
            }
            Self::OfflineLocalOnly => {
                Some(GitHistoryAccessibilityDowngradeTrigger::OfflineLocalOnly)
            }
        }
    }

    /// The next action a weakening condition's narrow disclosure must offer.
    pub const fn next_action(self) -> GitHistoryClaimNextAction {
        match self {
            Self::LocalTruthAligned | Self::ProviderReviewStateStale => {
                GitHistoryClaimNextAction::ReconcileProviderReviewState
            }
            Self::RepoTopologyPartial => GitHistoryClaimNextAction::CompleteRepoTopology,
            Self::CheckpointRecoveryUnavailable => {
                GitHistoryClaimNextAction::OpenRecoveryCheckpoint
            }
            Self::OfflineLocalOnly => GitHistoryClaimNextAction::ContinueLocalHistory,
        }
    }
}

/// A component's claim about how safe and recoverable its Git-history truth is.
///
/// Ordered strongest to weakest. [`RecoverableInProduct`](Self::RecoverableInProduct)
/// is the only tier that asserts full in-product history surgery with recoverable
/// depth and provider parity; the rest are the honest fallbacks a weakening condition
/// narrows to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryClaimTier {
    /// Safe in-product history surgery with recoverable depth and provider parity.
    RecoverableInProduct,
    /// Recoverable from local checkpoints while provider-linked review state is stale.
    LocallyRecoverable,
    /// Topology is partial; the history shown is incomplete, not the full depth.
    PartialHistoryOnly,
    /// No checkpoint exists; only a reflog-only fallback recovery is offered.
    ReflogOnlyRecovery,
    /// Offline / local-only; publish / approval parity cannot be claimed.
    LocalContinueOnly,
}

impl GitHistoryClaimTier {
    /// Every tier, in declaration order (strongest first).
    pub const ALL: [Self; 5] = [
        Self::RecoverableInProduct,
        Self::LocallyRecoverable,
        Self::PartialHistoryOnly,
        Self::ReflogOnlyRecovery,
        Self::LocalContinueOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecoverableInProduct => "recoverable_in_product",
            Self::LocallyRecoverable => "locally_recoverable",
            Self::PartialHistoryOnly => "partial_history_only",
            Self::ReflogOnlyRecovery => "reflog_only_recovery",
            Self::LocalContinueOnly => "local_continue_only",
        }
    }

    /// Strength rank, higher is stronger. Used for the ceiling comparison.
    pub const fn rank(self) -> u8 {
        match self {
            Self::RecoverableInProduct => 5,
            Self::LocallyRecoverable => 4,
            Self::PartialHistoryOnly => 3,
            Self::ReflogOnlyRecovery => 2,
            Self::LocalContinueOnly => 1,
        }
    }

    /// Whether this tier asserts full recoverable-in-product safety.
    pub const fn asserts_recoverable_in_product(self) -> bool {
        matches!(self, Self::RecoverableInProduct)
    }
}

/// A rendering form the claim must reach with identical semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryRenderingSurface {
    /// The full desktop surface.
    DesktopFull,
    /// The headless CLI.
    CliHeadless,
    /// The support export.
    SupportExport,
}

impl GitHistoryRenderingSurface {
    /// Every rendering surface, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopFull, Self::CliHeadless, Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The next action a narrow disclosure offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryClaimNextAction {
    /// Reconcile the provider-linked review state before claiming parity.
    ReconcileProviderReviewState,
    /// Complete the repo topology (unshallow / fetch) to restore full history depth.
    CompleteRepoTopology,
    /// Open the reflog-only recovery checkpoint / destination.
    OpenRecoveryCheckpoint,
    /// Continue the history work locally while offline.
    ContinueLocalHistory,
}

impl GitHistoryClaimNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconcileProviderReviewState => "reconcile_provider_review_state",
            Self::CompleteRepoTopology => "complete_repo_topology",
            Self::OpenRecoveryCheckpoint => "open_recovery_checkpoint",
            Self::ContinueLocalHistory => "continue_local_history",
        }
    }
}

/// Downgrade trigger that can narrow this accessibility lane below its full claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryAccessibilityDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-linked review state is stale relative to local Git truth.
    ProviderReviewStateStale,
    /// Repo topology is shallow / partial / sparse.
    RepoTopologyPartial,
    /// No checkpoint exists; only reflog-only recovery is available.
    CheckpointRecoveryUnavailable,
    /// The surface is offline / local-only.
    OfflineLocalOnly,
    /// A claim was overstated relative to its permitted ceiling.
    ClaimOverstated,
    /// Parity across desktop, CLI, or export was dropped.
    ParityDropped,
    /// Consumer trust narrowed.
    TrustNarrowing,
}

impl GitHistoryAccessibilityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderReviewStateStale,
        Self::RepoTopologyPartial,
        Self::CheckpointRecoveryUnavailable,
        Self::OfflineLocalOnly,
        Self::ClaimOverstated,
        Self::ParityDropped,
        Self::TrustNarrowing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderReviewStateStale => "provider_review_state_stale",
            Self::RepoTopologyPartial => "repo_topology_partial",
            Self::CheckpointRecoveryUnavailable => "checkpoint_recovery_unavailable",
            Self::OfflineLocalOnly => "offline_local_only",
            Self::ClaimOverstated => "claim_overstated",
            Self::ParityDropped => "parity_dropped",
            Self::TrustNarrowing => "trust_narrowing",
        }
    }
}

/// The disclosures an accessibility row must carry, derived from its condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GitHistoryComponentClaimResolution {
    /// The strongest claim tier the condition permits.
    pub permitted_ceiling: GitHistoryClaimTier,
    /// Whether the condition requires an explicit narrow disclosure.
    pub requires_narrowing: bool,
    /// The downgrade trigger the narrow disclosure must name, if any.
    pub expected_trigger: Option<GitHistoryAccessibilityDowngradeTrigger>,
    /// The next action the narrow disclosure must offer.
    pub expected_next_action: GitHistoryClaimNextAction,
    /// Whether the row must spell out that the history shown is incomplete.
    pub needs_topology_note: bool,
    /// Whether the row must name the reflog-only recovery destination.
    pub needs_recovery_note: bool,
    /// Whether the row must carry an explicit local-continue note.
    pub needs_local_continue_note: bool,
}

/// Resolves the claim narrowing an accessibility row must carry from its condition.
///
/// Aligned local truth keeps the full recoverable-in-product claim. Each weakening
/// condition pins the claim to a ceiling, demands an explicit narrow disclosure naming
/// its trigger and next action, and preserves a local-continue path so the reviewer's
/// history work never vanishes. Partial repo topology additionally spells out that the
/// history shown is incomplete; an unavailable checkpoint additionally keeps the
/// reflog-only recovery destination named rather than forcing raw navigation.
pub const fn resolve_git_history_component_claim_narrowing(
    condition: GitHistoryClaimCondition,
) -> GitHistoryComponentClaimResolution {
    GitHistoryComponentClaimResolution {
        permitted_ceiling: condition.permitted_ceiling(),
        requires_narrowing: condition.is_weakening(),
        expected_trigger: condition.default_trigger(),
        expected_next_action: condition.next_action(),
        needs_topology_note: matches!(condition, GitHistoryClaimCondition::RepoTopologyPartial),
        needs_recovery_note: matches!(
            condition,
            GitHistoryClaimCondition::CheckpointRecoveryUnavailable
        ),
        needs_local_continue_note: condition.is_weakening(),
    }
}

/// The explicit narrow disclosure a claim-narrowed row shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentClaimNarrowing {
    /// The downgrade trigger the narrowing discloses.
    pub trigger: GitHistoryAccessibilityDowngradeTrigger,
    /// The claim tier the narrowing pins the component to.
    pub narrowed_to: GitHistoryClaimTier,
    /// Note naming the truth preserved through the narrowing (never omitted).
    pub preserved_truth_note: String,
    /// The next action offered.
    pub next_action: GitHistoryClaimNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// One accessibility row: a claimed component under one condition, exposed across
/// keyboard, screen-reader, CLI, and export forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentAccessibilityRow {
    /// Stable row id.
    pub row_id: String,
    /// Which shared component this row claims.
    pub component: M5GitHistoryComponent,
    /// The condition governing the claim.
    pub condition: GitHistoryClaimCondition,
    /// The claim tier the component effectively asserts.
    pub effective_claim: GitHistoryClaimTier,
    /// Keyboard reach / operation label (never empty).
    pub keyboard_label: String,
    /// Screen-reader label (never empty).
    pub screen_reader_label: String,
    /// CLI enum token (never empty).
    pub cli_enum_token: String,
    /// Export enum token (never empty).
    pub export_enum_token: String,
    /// Human-readable explanation field (never empty).
    pub explanation_field: String,
    /// The rendering surfaces this row reaches (must cover all three).
    pub rendering_surfaces: Vec<GitHistoryRenderingSurface>,
    /// The explicit narrow disclosure; required and complete when the claim narrows.
    pub narrowing: Option<GitHistoryComponentClaimNarrowing>,
    /// Incomplete-history note; required and non-empty when the disclosure demands it.
    pub topology_note: String,
    /// Recovery-destination note; required and non-empty when the disclosure demands it.
    pub recovery_note: String,
    /// Local-continue note; required and non-empty when the disclosure demands it.
    pub local_continue_note: String,
    /// Guardrail: this component is reachable only by pointer.
    pub is_pointer_only: bool,
    /// Guardrail: this component omits itself from the export.
    pub is_export_opaque: bool,
    /// Guardrail: this component claims more on the desktop than in CLI or export.
    pub desktop_stronger_than_cli: bool,
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl GitHistoryComponentAccessibilityRow {
    /// The disclosures this row must carry, derived from its condition.
    pub const fn resolution(&self) -> GitHistoryComponentClaimResolution {
        resolve_git_history_component_claim_narrowing(self.condition)
    }

    /// Whether this row narrows below the full recoverable-in-product claim.
    pub const fn is_narrowed(&self) -> bool {
        self.condition.is_weakening()
    }

    /// Whether this row reaches all three rendering surfaces.
    pub fn covers_all_rendering_surfaces(&self) -> bool {
        GitHistoryRenderingSurface::ALL
            .iter()
            .all(|surface| self.rendering_surfaces.contains(surface))
    }

    /// Whether every accessibility field is present.
    pub fn accessibility_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.is_pointer_only && !self.is_export_opaque && !self.desktop_stronger_than_cli
    }

    /// Whether this row points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == GIT_HISTORY_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF
            })
    }

    /// Whether the effective claim is honest under the row's condition: it never
    /// exceeds the permitted ceiling, and a weakening condition narrows the claim down
    /// to exactly that ceiling.
    pub fn claim_is_honest(&self) -> bool {
        let resolution = self.resolution();
        let ceiling = resolution.permitted_ceiling;
        if self.effective_claim.rank() > ceiling.rank() {
            return false;
        }
        if resolution.requires_narrowing {
            self.effective_claim == ceiling
                && self
                    .narrowing
                    .as_ref()
                    .is_some_and(|narrowing| narrowing.narrowed_to == ceiling)
        } else {
            self.effective_claim == ceiling && self.narrowing.is_none()
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentAccessibilityTrustReview {
    /// Every claim is keyboard-reachable.
    pub keyboard_reachable_on_every_claim: bool,
    /// Every claim carries a screen-reader label.
    pub screen_reader_labeled_on_every_claim: bool,
    /// Every claim exposes a CLI enum token.
    pub cli_enum_exposed_on_every_claim: bool,
    /// Every claim exposes an export enum token.
    pub export_enum_exposed_on_every_claim: bool,
    /// Every claim carries an explanation field.
    pub explanation_field_present_on_every_claim: bool,
    /// No component is pointer-only.
    pub no_component_pointer_only: bool,
    /// No component is export-opaque.
    pub no_component_export_opaque: bool,
    /// No component claims more on the desktop than in CLI or export.
    pub desktop_never_stronger_than_cli: bool,
    /// The claim narrows when topology, recovery, or provider truth degrades.
    pub claim_narrows_when_topology_or_recovery_or_provider_degrades: bool,
    /// Recovery or mutation safety is never overstated while a weakening condition holds.
    pub recovery_or_mutation_safety_never_overstated_under_weakening: bool,
    /// The recovery destination is kept explicit rather than reduced to a badge.
    pub recovery_destination_kept_explicit: bool,
    /// Local continuation is preserved when Git-history truth is degraded.
    pub local_continue_preserved_under_degraded_truth: bool,
}

impl GitHistoryComponentAccessibilityTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.keyboard_reachable_on_every_claim
            && self.screen_reader_labeled_on_every_claim
            && self.cli_enum_exposed_on_every_claim
            && self.export_enum_exposed_on_every_claim
            && self.explanation_field_present_on_every_claim
            && self.no_component_pointer_only
            && self.no_component_export_opaque
            && self.desktop_never_stronger_than_cli
            && self.claim_narrows_when_topology_or_recovery_or_provider_degrades
            && self.recovery_or_mutation_safety_never_overstated_under_weakening
            && self.recovery_destination_kept_explicit
            && self.local_continue_preserved_under_degraded_truth
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentAccessibilityProjection {
    /// Keyboard and screen-reader labels are exposed.
    pub exposes_keyboard_and_screen_reader_labels: bool,
    /// CLI and export enums are exposed.
    pub exposes_cli_and_export_enums: bool,
    /// Explanation fields are exposed.
    pub exposes_explanation_fields: bool,
    /// The claim auto-narrows when repo topology is partial.
    pub auto_narrows_on_partial_repo_topology: bool,
    /// The claim auto-narrows when checkpoint recovery is unavailable.
    pub auto_narrows_on_unavailable_checkpoint_recovery: bool,
    /// The claim auto-narrows when provider-linked review state is stale.
    pub auto_narrows_on_stale_provider_review_state: bool,
    /// The claim auto-narrows when the surface is offline / local-only.
    pub auto_narrows_on_offline_local_only: bool,
    /// Desktop, CLI, and export semantics are identical.
    pub desktop_cli_export_semantics_identical: bool,
    /// Narrowing prevents overstated recovery or mutation safety.
    pub narrowing_prevents_overstated_recovery_or_mutation_safety: bool,
    /// Every component is reachable non-visually.
    pub every_component_reachable_non_visually: bool,
}

impl GitHistoryComponentAccessibilityProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.exposes_keyboard_and_screen_reader_labels
            && self.exposes_cli_and_export_enums
            && self.exposes_explanation_fields
            && self.auto_narrows_on_partial_repo_topology
            && self.auto_narrows_on_unavailable_checkpoint_recovery
            && self.auto_narrows_on_stale_provider_review_state
            && self.auto_narrows_on_offline_local_only
            && self.desktop_cli_export_semantics_identical
            && self.narrowing_prevents_overstated_recovery_or_mutation_safety
            && self.every_component_reachable_non_visually
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentAccessibilityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GitHistoryComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHistoryComponentAccessibilityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<GitHistoryComponentAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<GitHistoryRenderingSurface>,
    /// Trust review block.
    pub trust_review: GitHistoryComponentAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: GitHistoryComponentAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: GitHistoryComponentAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe Git-history component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryComponentAccessibilityPacket {
    /// Record kind; must equal [`GIT_HISTORY_ACCESSIBILITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GIT_HISTORY_ACCESSIBILITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<GitHistoryComponentAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<GitHistoryRenderingSurface>,
    /// Trust review block.
    pub trust_review: GitHistoryComponentAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: GitHistoryComponentAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: GitHistoryComponentAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GitHistoryComponentAccessibilityPacket {
    /// Builds a Git-history component accessibility packet from stable-lane input.
    pub fn new(input: GitHistoryComponentAccessibilityPacketInput) -> Self {
        Self {
            record_kind: GIT_HISTORY_ACCESSIBILITY_RECORD_KIND.to_owned(),
            schema_version: GIT_HISTORY_ACCESSIBILITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            accessibility_rows: input.accessibility_rows,
            downgrade_triggers: input.downgrade_triggers,
            rendering_surfaces: input.rendering_surfaces,
            trust_review: input.trust_review,
            projection: input.projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the Git-history component accessibility parity invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<GitHistoryComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GIT_HISTORY_ACCESSIBILITY_RECORD_KIND {
            violations.push(GitHistoryComponentAccessibilityViolation::WrongRecordKind);
        }
        if self.schema_version != GIT_HISTORY_ACCESSIBILITY_SCHEMA_VERSION {
            violations.push(GitHistoryComponentAccessibilityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GitHistoryComponentAccessibilityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::DowngradeTriggersMissing);
        }
        if self.rendering_surfaces.is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::RenderingSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(GitHistoryComponentAccessibilityViolation::TrustReviewIncomplete);
        }
        if !self.projection.all_hold() {
            violations.push(GitHistoryComponentAccessibilityViolation::ProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GitHistoryComponentAccessibilityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("git history component accessibility packet serializes"),
        ) {
            violations.push(GitHistoryComponentAccessibilityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("git history component accessibility packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .accessibility_rows
            .iter()
            .filter(|row| row.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Git-History Component Accessibility, Headless, and Export Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Accessibility rows: {} ({} claim-narrowed)\n",
            self.accessibility_rows.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Accessibility rows\n\n");
        for row in &self.accessibility_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: condition `{}`, claim `{}`\n",
                row.component.as_str(),
                row.row_id,
                row.condition.as_str(),
                row.effective_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in Git-history component accessibility export.
#[derive(Debug)]
pub enum GitHistoryComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GitHistoryComponentAccessibilityViolation>),
}

impl fmt::Display for GitHistoryComponentAccessibilityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "git history component accessibility export parse failed: {error}"
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
                    "git history component accessibility export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GitHistoryComponentAccessibilityArtifactError {}

/// Validation failures emitted by [`GitHistoryComponentAccessibilityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitHistoryComponentAccessibilityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No accessibility rows are present.
    AccessibilityRowsMissing,
    /// An accessibility row is incomplete.
    RowIncomplete,
    /// A row is missing its keyboard label.
    KeyboardLabelMissing,
    /// A row is missing its screen-reader label.
    ScreenReaderLabelMissing,
    /// A row is missing its CLI enum token.
    CliEnumTokenMissing,
    /// A row is missing its export enum token.
    ExportEnumTokenMissing,
    /// A row is missing its explanation field.
    ExplanationFieldMissing,
    /// A row does not reach all three rendering surfaces.
    RenderingSurfaceCoverageMissing,
    /// A component is reachable only by pointer.
    PointerOnlyComponent,
    /// A component omits itself from the export.
    ExportOpaqueComponent,
    /// A component claims more on the desktop than in CLI or export.
    DesktopStrongerThanCli,
    /// A row's effective claim exceeds the ceiling its condition permits.
    ClaimCeilingExceeded,
    /// A weakening condition is missing its explicit narrow disclosure.
    ClaimNarrowingMissing,
    /// A baseline condition unexpectedly carries a narrow disclosure.
    ClaimNarrowingUnexpected,
    /// A narrow disclosure pins the claim to the wrong tier.
    NarrowedToMismatch,
    /// A narrow disclosure names the wrong trigger.
    NarrowTriggerMismatch,
    /// A narrow disclosure offers the wrong next action.
    NarrowNextActionMismatch,
    /// A narrow disclosure is missing its preserved-truth note.
    NarrowPreservedTruthMissing,
    /// A narrow disclosure is missing its next-action copy.
    NarrowNextActionMissing,
    /// A row that must spell out incomplete history is missing its topology note.
    TopologyNoteMissing,
    /// A row that must name the recovery destination is missing its note.
    RecoveryNoteMissing,
    /// A row that must preserve a local-continue path is missing its note.
    LocalContinueNoteMissing,
    /// A row does not point at the canonical component and matrix contracts.
    CanonicalContractReferenceMissing,
    /// Not every shared component appears among the rows.
    ComponentCoverageMissing,
    /// Not every claim condition appears among the rows.
    ConditionCoverageMissing,
    /// Not every claim tier appears as an effective claim.
    ClaimTierCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No rendering surfaces are present.
    RenderingSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl GitHistoryComponentAccessibilityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::AccessibilityRowsMissing => "accessibility_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::RenderingSurfaceCoverageMissing => "rendering_surface_coverage_missing",
            Self::PointerOnlyComponent => "pointer_only_component",
            Self::ExportOpaqueComponent => "export_opaque_component",
            Self::DesktopStrongerThanCli => "desktop_stronger_than_cli",
            Self::ClaimCeilingExceeded => "claim_ceiling_exceeded",
            Self::ClaimNarrowingMissing => "claim_narrowing_missing",
            Self::ClaimNarrowingUnexpected => "claim_narrowing_unexpected",
            Self::NarrowedToMismatch => "narrowed_to_mismatch",
            Self::NarrowTriggerMismatch => "narrow_trigger_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowPreservedTruthMissing => "narrow_preserved_truth_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::TopologyNoteMissing => "topology_note_missing",
            Self::RecoveryNoteMissing => "recovery_note_missing",
            Self::LocalContinueNoteMissing => "local_continue_note_missing",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::ConditionCoverageMissing => "condition_coverage_missing",
            Self::ClaimTierCoverageMissing => "claim_tier_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RenderingSurfacesMissing => "rendering_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ProjectionIncomplete => "projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable Git-history component accessibility export.
///
/// # Errors
///
/// Returns [`GitHistoryComponentAccessibilityArtifactError`] when the checked-in
/// export fails to parse or violates the frozen contract.
pub fn current_git_history_component_accessibility_export(
) -> Result<GitHistoryComponentAccessibilityPacket, GitHistoryComponentAccessibilityArtifactError> {
    let packet: GitHistoryComponentAccessibilityPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-git-history-component-accessibility-proof/support_export.json"
    )))
        .map_err(GitHistoryComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GitHistoryComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &GitHistoryComponentAccessibilityPacket,
    violations: &mut Vec<GitHistoryComponentAccessibilityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GIT_HISTORY_ACCESSIBILITY_SCHEMA_REF,
        GIT_HISTORY_ACCESSIBILITY_DOC_REF,
        GIT_HISTORY_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF,
        GIT_HISTORY_ACCESSIBILITY_CONSUMER_CONTRACT_REF,
        GIT_HISTORY_ACCESSIBILITY_IDENTITY_CONTRACT_REF,
        GIT_HISTORY_ACCESSIBILITY_STASH_RECOVERY_CONTRACT_REF,
        GIT_HISTORY_ACCESSIBILITY_SEQUENCE_EDIT_CONTRACT_REF,
        GIT_HISTORY_ACCESSIBILITY_MUTATION_REVIEW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GitHistoryComponentAccessibilityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &GitHistoryComponentAccessibilityPacket,
    violations: &mut Vec<GitHistoryComponentAccessibilityViolation>,
) {
    if packet.accessibility_rows.is_empty() {
        violations.push(GitHistoryComponentAccessibilityViolation::AccessibilityRowsMissing);
        return;
    }

    let mut seen_components: BTreeSet<M5GitHistoryComponent> = BTreeSet::new();
    let mut seen_conditions: BTreeSet<GitHistoryClaimCondition> = BTreeSet::new();
    let mut seen_tiers: BTreeSet<GitHistoryClaimTier> = BTreeSet::new();

    for row in &packet.accessibility_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::RowIncomplete);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_rendering_surfaces() {
            violations
                .push(GitHistoryComponentAccessibilityViolation::RenderingSurfaceCoverageMissing);
        }

        // AC1 guardrails: parity across desktop, CLI, and export.
        if row.is_pointer_only {
            violations.push(GitHistoryComponentAccessibilityViolation::PointerOnlyComponent);
        }
        if row.is_export_opaque {
            violations.push(GitHistoryComponentAccessibilityViolation::ExportOpaqueComponent);
        }
        if row.desktop_stronger_than_cli {
            violations.push(GitHistoryComponentAccessibilityViolation::DesktopStrongerThanCli);
        }

        let resolution = row.resolution();
        let ceiling = resolution.permitted_ceiling;

        // AC2 core: a claim may never exceed the ceiling its condition permits.
        if row.effective_claim.rank() > ceiling.rank() {
            violations.push(GitHistoryComponentAccessibilityViolation::ClaimCeilingExceeded);
        }

        // Narrow-disclosure presence and completeness.
        if resolution.requires_narrowing {
            match &row.narrowing {
                None => {
                    violations
                        .push(GitHistoryComponentAccessibilityViolation::ClaimNarrowingMissing);
                }
                Some(narrowing) => {
                    if narrowing.narrowed_to != ceiling {
                        violations
                            .push(GitHistoryComponentAccessibilityViolation::NarrowedToMismatch);
                    }
                    if Some(narrowing.trigger) != resolution.expected_trigger {
                        violations
                            .push(GitHistoryComponentAccessibilityViolation::NarrowTriggerMismatch);
                    }
                    if narrowing.next_action != resolution.expected_next_action {
                        violations.push(
                            GitHistoryComponentAccessibilityViolation::NarrowNextActionMismatch,
                        );
                    }
                    if narrowing.preserved_truth_note.trim().is_empty() {
                        violations.push(
                            GitHistoryComponentAccessibilityViolation::NarrowPreservedTruthMissing,
                        );
                    }
                    if narrowing.next_action_label.trim().is_empty() {
                        violations.push(
                            GitHistoryComponentAccessibilityViolation::NarrowNextActionMissing,
                        );
                    }
                }
            }
        } else if row.narrowing.is_some() {
            violations.push(GitHistoryComponentAccessibilityViolation::ClaimNarrowingUnexpected);
        }

        if resolution.needs_topology_note && row.topology_note.trim().is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::TopologyNoteMissing);
        }
        if resolution.needs_recovery_note && row.recovery_note.trim().is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::RecoveryNoteMissing);
        }
        if resolution.needs_local_continue_note && row.local_continue_note.trim().is_empty() {
            violations.push(GitHistoryComponentAccessibilityViolation::LocalContinueNoteMissing);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(GitHistoryComponentAccessibilityViolation::CanonicalContractReferenceMissing);
        }

        seen_components.insert(row.component);
        seen_conditions.insert(row.condition);
        seen_tiers.insert(row.effective_claim);
    }

    // Coverage: every component, every condition, and every claim tier must appear.
    for component in M5GitHistoryComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(GitHistoryComponentAccessibilityViolation::ComponentCoverageMissing);
            break;
        }
    }
    for condition in GitHistoryClaimCondition::ALL {
        if !seen_conditions.contains(&condition) {
            violations.push(GitHistoryComponentAccessibilityViolation::ConditionCoverageMissing);
            break;
        }
    }
    for tier in GitHistoryClaimTier::ALL {
        if !seen_tiers.contains(&tier) {
            violations.push(GitHistoryComponentAccessibilityViolation::ClaimTierCoverageMissing);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
