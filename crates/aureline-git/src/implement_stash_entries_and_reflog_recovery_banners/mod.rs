//! Stash entries and reflog-recovery banners with untracked/staged scope,
//! base-ref origin, apply/pop/drop/create-branch distinctions, and checkpoint
//! lineage across claimed M5 recovery lanes.
//!
//! This module narrows the two recovery components frozen in
//! [`crate::freeze_the_m5_git_history_sequence_component_matrix`] —
//! `stash_entry` and `reflog_recovery_banner` — into an implemented,
//! export-safe row contract. A [`StashEntryRow`] answers, from itself alone,
//! which message and created-from ref name the stash, whether untracked and/or
//! staged content is included, and which exact restore verb (apply, pop, drop,
//! or create-branch-from-stash) it exposes — never collapsing those verbs into
//! one ambiguous restore. A [`ReflogRecoveryBannerRow`] links a risky history
//! mutation back to a concrete checkpoint/recovery destination, spells out the
//! expiry state, and stays reachable from Git history, review, and help/support
//! surfaces until it is superseded or dismissed.
//!
//! Two honesty axes anchor the acceptance criteria. First, a `stash@{n}`-style
//! shorthand is never the only meaning-bearing label: the human message and the
//! created-from ref are always explicit, so one restore verb is never mistaken
//! for another (`StashShorthandOnlyLabel`, `RestoreVerbCoverageMissing`,
//! `RestoreVerbsCollapsed`). Second, a reachable recovery banner always carries
//! a concrete destination and spans history/review/help-support, and an expired
//! or pruned reflog point can never keep claiming to be reachable
//! (`RecoveryDestinationMissing`, `RecoveryNotReachableAcrossSurfaces`,
//! `ExpiredRecoveryStillReachable`).
//!
//! The shared downgrade vocabulary ([`GitHistoryDowngradeState`]) and the shared
//! consumer surfaces ([`ComponentConsumerSurface`]) are reused directly from the
//! frozen matrix so downgrades and parity read the same everywhere. Local-only
//! recovery stays explicit even when a provider-linked review state also exists.
//! Raw paths, raw object bytes, raw branch names, raw reflog/stash bodies, raw
//! provider payloads, and credentials stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-stash-reflog-recovery-component.schema.json`](../../../../schemas/ui/m5-stash-reflog-recovery-component.schema.json).
//! The contract doc is
//! [`docs/git/m5/implement_stash_entries_and_reflog_recovery_banners.md`](../../../../docs/git/m5/implement_stash_entries_and_reflog_recovery_banners.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-stash-reflog-recovery-components/`](../../../../fixtures/ui/m5-stash-reflog-recovery-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_git_history_sequence_component_matrix::{
    ComponentConsumerSurface, GitHistoryDowngradeState, M5GitHistoryComponent,
};

/// Stable record-kind tag carried by [`StashReflogRecoveryPacket`].
pub const STASH_REFLOG_RECOVERY_RECORD_KIND: &str = "git_stash_reflog_recovery_component_truth";

/// Schema version for stash / reflog-recovery component records.
pub const STASH_REFLOG_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const STASH_REFLOG_RECOVERY_SCHEMA_REF: &str =
    "schemas/ui/m5-stash-reflog-recovery-component.schema.json";

/// Repo-relative path of the contract doc.
pub const STASH_REFLOG_RECOVERY_DOC_REF: &str =
    "docs/git/m5/implement_stash_entries_and_reflog_recovery_banners.md";

/// Repo-relative path of the frozen component matrix this lane implements.
pub const STASH_REFLOG_RECOVERY_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-git-history-sequence-component-matrix.schema.json";

/// Repo-relative path of the canonical stash-entry contract.
pub const STASH_REFLOG_RECOVERY_STASH_CONTRACT_REF: &str = "schemas/git/stash_entry.schema.json";

/// Repo-relative path of the canonical recovery-checkpoint contract.
pub const STASH_REFLOG_RECOVERY_CHECKPOINT_CONTRACT_REF: &str =
    "schemas/git/recovery_checkpoint.schema.json";

/// Repo-relative path of the canonical stash-recovery review contract.
pub const STASH_REFLOG_RECOVERY_REVIEW_CONTRACT_REF: &str =
    "schemas/git/git_stash_recovery_review.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const STASH_REFLOG_RECOVERY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-stash-reflog-recovery-components";

/// Repo-relative path of the checked support-export artifact.
pub const STASH_REFLOG_RECOVERY_ARTIFACT_REF: &str =
    "artifacts/release/m5-stash-reflog-recovery-components-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const STASH_REFLOG_RECOVERY_SUMMARY_REF: &str =
    "artifacts/release/m5-stash-reflog-recovery-components-proof/summary.md";

/// The two recovery components this lane implements.
///
/// These are exactly the stash/reflog-recovery pair of the frozen matrix: a
/// stash-shelf entry with an explicit restore scope, and a reflog banner that
/// links a risky mutation back to a concrete checkpoint.
pub const STASH_REFLOG_RECOVERY_COMPONENTS: [M5GitHistoryComponent; 2] = [
    M5GitHistoryComponent::StashEntry,
    M5GitHistoryComponent::ReflogRecoveryBanner,
];

/// What content a stash entry captured: the untracked/staged scope axis.
///
/// The stash contents must be explicit so a restore never surprises the user
/// with untracked files it silently swept in or index state it kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StashContentScope {
    /// Tracked modified files only; index state was not preserved.
    TrackedModified,
    /// Tracked modified files plus preserved index/staged content.
    TrackedAndStaged,
    /// Tracked modified files plus swept-in untracked files.
    TrackedAndUntracked,
    /// Tracked, staged, and untracked content all captured together.
    TrackedStagedUntracked,
    /// Only the staged/index content was stashed (`--staged`).
    StagedOnly,
}

impl StashContentScope {
    /// Every scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TrackedModified,
        Self::TrackedAndStaged,
        Self::TrackedAndUntracked,
        Self::TrackedStagedUntracked,
        Self::StagedOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrackedModified => "tracked_modified",
            Self::TrackedAndStaged => "tracked_and_staged",
            Self::TrackedAndUntracked => "tracked_and_untracked",
            Self::TrackedStagedUntracked => "tracked_staged_untracked",
            Self::StagedOnly => "staged_only",
        }
    }

    /// Whether this scope swept in untracked files.
    pub const fn includes_untracked(self) -> bool {
        matches!(
            self,
            Self::TrackedAndUntracked | Self::TrackedStagedUntracked
        )
    }

    /// Whether this scope preserved staged/index content.
    pub const fn includes_staged(self) -> bool {
        matches!(
            self,
            Self::TrackedAndStaged | Self::TrackedStagedUntracked | Self::StagedOnly
        )
    }
}

/// A distinct stash restore verb.
///
/// Apply keeps the stash, pop restores and drops it, drop discards it without
/// restoring, and create-branch-from-stash spins a branch off the stashed
/// commit. These verbs are never collapsed into one ambiguous restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StashRestoreVerb {
    /// Restore the stash and keep it on the shelf.
    Apply,
    /// Restore the stash and remove it from the shelf.
    Pop,
    /// Discard the stash without restoring it.
    Drop,
    /// Create a new branch from the stashed commit and apply it there.
    CreateBranchFromStash,
}

impl StashRestoreVerb {
    /// Every restore verb, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Apply,
        Self::Pop,
        Self::Drop,
        Self::CreateBranchFromStash,
    ];

    /// The four verbs a stash entry must expose so none is mistaken for another.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Apply => "apply",
            Self::Pop => "pop",
            Self::Drop => "drop",
            Self::CreateBranchFromStash => "create_branch_from_stash",
        }
    }

    /// Whether this verb removes the stash from the shelf (pop and drop do).
    pub const fn removes_stash_from_shelf(self) -> bool {
        matches!(self, Self::Pop | Self::Drop)
    }

    /// Whether this verb discards work without restoring it (drop alone).
    pub const fn discards_without_restoring(self) -> bool {
        matches!(self, Self::Drop)
    }
}

/// Whether a reflog-recovery banner is still reachable.
///
/// A reachable banner keeps its recovery destination live across surfaces; a
/// superseded or dismissed banner is terminal and no longer claims reachability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryReachability {
    /// The recovery destination is still reachable and offered.
    Reachable,
    /// A newer recovery point superseded this one.
    Superseded,
    /// The user dismissed this recovery banner.
    Dismissed,
}

impl RecoveryReachability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Superseded => "superseded",
            Self::Dismissed => "dismissed",
        }
    }

    /// Whether the banner still offers a live recovery destination.
    pub const fn is_reachable(self) -> bool {
        matches!(self, Self::Reachable)
    }
}

/// Expiry state of the reflog/checkpoint a banner points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryExpiryState {
    /// The recovery point is well within its retention window.
    Fresh,
    /// The recovery point is approaching its reflog expiry.
    ExpiringSoon,
    /// The recovery point has passed its reflog expiry window.
    Expired,
    /// The recovery point was pruned by garbage collection.
    Pruned,
    /// The recovery point is pinned and will not expire.
    Pinned,
}

impl RecoveryExpiryState {
    /// Every expiry state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Fresh,
        Self::ExpiringSoon,
        Self::Expired,
        Self::Pruned,
        Self::Pinned,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
            Self::Pruned => "pruned",
            Self::Pinned => "pinned",
        }
    }

    /// Whether the recovery point is gone and can no longer be reached.
    pub const fn is_gone(self) -> bool {
        matches!(self, Self::Expired | Self::Pruned)
    }
}

/// A surface a reflog-recovery banner must stay reachable from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoverySurface {
    /// The Git history / graph surface.
    GitHistory,
    /// The review / publish surface.
    Review,
    /// The in-product help / support surface.
    HelpSupport,
}

impl RecoverySurface {
    /// Every surface recovery must remain reachable from, in declaration order.
    pub const ALL: [Self; 3] = [Self::GitHistory, Self::Review, Self::HelpSupport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitHistory => "git_history",
            Self::Review => "review",
            Self::HelpSupport => "help_support",
        }
    }
}

/// A direct action a reflog-recovery banner exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAction {
    /// Restore the working state to the concrete recovery destination.
    RestoreToCheckpoint,
    /// Open the underlying reflog entry in the workspace.
    OpenReflogEntry,
    /// Pin the recovery point so it will not expire.
    PinRecoveryPoint,
    /// Compare the recovery destination with the current tip.
    CompareWithCurrent,
    /// Dismiss the banner once recovery is no longer wanted.
    DismissBanner,
    /// Hand off to the hosted provider view in the browser.
    OpenProviderInBrowser,
}

impl RecoveryAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreToCheckpoint => "restore_to_checkpoint",
            Self::OpenReflogEntry => "open_reflog_entry",
            Self::PinRecoveryPoint => "pin_recovery_point",
            Self::CompareWithCurrent => "compare_with_current",
            Self::DismissBanner => "dismiss_banner",
            Self::OpenProviderInBrowser => "open_provider_in_browser",
        }
    }

    /// Whether this action stays inside the product rather than forcing raw-provider navigation.
    pub const fn is_in_product(self) -> bool {
        !matches!(self, Self::OpenProviderInBrowser)
    }
}

/// Disclosures a reflog-recovery banner must carry, derived from its
/// reachability and expiry state.
///
/// A reachable banner must show a concrete destination and span
/// history/review/help-support; every banner discloses its expiry state; and a
/// banner is only genuinely recoverable when reachable and not gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryBannerDisclosure {
    /// Whether the banner must show a concrete recovery destination.
    pub must_show_concrete_destination: bool,
    /// Whether the banner must stay reachable from history, review, and help/support.
    pub must_span_history_review_help_support: bool,
    /// Whether the banner must disclose its expiry state (always true).
    pub must_show_expiry_state: bool,
    /// Whether the recovery destination is genuinely still recoverable.
    pub is_still_recoverable: bool,
}

/// Resolves the disclosures a reflog-recovery banner must carry from its
/// reachability and expiry state.
///
/// A reachable banner always shows a concrete destination and spans all three
/// recovery surfaces, so recovery stays reachable from Git history, review, and
/// help/support after a risky mutation until superseded or dismissed. The expiry
/// state is always disclosed, and a banner is only recoverable when reachable
/// and the recovery point has not expired or been pruned.
pub fn resolve_recovery_banner_disclosure(
    reachability: RecoveryReachability,
    expiry: RecoveryExpiryState,
) -> RecoveryBannerDisclosure {
    RecoveryBannerDisclosure {
        must_show_concrete_destination: reachability.is_reachable(),
        must_span_history_review_help_support: reachability.is_reachable(),
        must_show_expiry_state: true,
        is_still_recoverable: reachability.is_reachable() && !expiry.is_gone(),
    }
}

/// One stash-shelf entry with its explicit restore scope and verbs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashEntryRow {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component this row implements; must be `stash_entry`.
    pub component: M5GitHistoryComponent,
    /// The `stash@{n}`-style shorthand; never the only meaning-bearing label.
    pub stash_shorthand: String,
    /// Human-readable stash message.
    pub message: String,
    /// The ref the stash was created from.
    pub created_from_ref: String,
    /// What content the stash captured (untracked/staged scope).
    pub content_scope: StashContentScope,
    /// Human-readable disclosure of what the scope includes.
    pub scope_disclosure: String,
    /// The distinct restore verbs the entry exposes, in display order.
    pub restore_verbs: Vec<StashRestoreVerb>,
    /// How the stashed work stays reflog/recovery reachable after a restore.
    pub recovery_reflog_note: String,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl StashEntryRow {
    /// Whether the message and created-from ref give the entry a meaning-bearing
    /// label beyond the raw `stash@{n}` shorthand.
    pub fn has_meaning_beyond_shorthand(&self) -> bool {
        let message = self.message.trim();
        !message.is_empty()
            && message != self.stash_shorthand.trim()
            && !self.created_from_ref.trim().is_empty()
    }

    /// Whether the entry exposes every required distinct restore verb.
    pub fn covers_required_restore_verbs(&self) -> bool {
        StashRestoreVerb::REQUIRED
            .iter()
            .all(|verb| self.restore_verbs.contains(verb))
    }

    /// Whether the restore verbs stay distinct (no verb aliased/collapsed).
    pub fn restore_verbs_stay_distinct(&self) -> bool {
        let unique: BTreeSet<StashRestoreVerb> = self.restore_verbs.iter().copied().collect();
        unique.len() == self.restore_verbs.len()
    }
}

/// One reflog-recovery banner linking a risky mutation to a concrete checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflogRecoveryBannerRow {
    /// Stable row id.
    pub row_id: String,
    /// Canonical component this row implements; must be `reflog_recovery_banner`.
    pub component: M5GitHistoryComponent,
    /// The risky history mutation this banner recovers from.
    pub mutation_label: String,
    /// The concrete checkpoint/recovery destination (for example `main@{1}`).
    pub recovery_destination: String,
    /// Whether the banner is still reachable.
    pub reachability: RecoveryReachability,
    /// Expiry state of the reflog/checkpoint the banner points at.
    pub expiry_state: RecoveryExpiryState,
    /// Human-readable disclosure of the expiry state.
    pub expiry_disclosure: String,
    /// Surfaces the banner stays reachable from; must span history/review/help-support when reachable.
    pub reachable_from_surfaces: Vec<RecoverySurface>,
    /// Direct actions the banner exposes, in display order.
    pub restore_actions: Vec<RecoveryAction>,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Component fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
}

impl ReflogRecoveryBannerRow {
    /// Disclosures this banner must carry, derived from its reachability and expiry.
    pub fn disclosure(&self) -> RecoveryBannerDisclosure {
        resolve_recovery_banner_disclosure(self.reachability, self.expiry_state)
    }

    /// Whether the banner exposes at least one in-product action.
    pub fn has_in_product_action(&self) -> bool {
        self.restore_actions
            .iter()
            .any(|action| action.is_in_product())
    }

    /// Whether the banner stays reachable from all three required surfaces.
    pub fn spans_required_surfaces(&self) -> bool {
        RecoverySurface::ALL
            .iter()
            .all(|surface| self.reachable_from_surfaces.contains(surface))
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashReflogRecoveryTrustReview {
    /// A `stash@{n}` shorthand is never the only meaning-bearing label.
    pub stash_shorthand_never_only_label: bool,
    /// The stash message and created-from ref are always explicit.
    pub stash_message_and_origin_explicit: bool,
    /// The untracked/staged scope of a stash is always explicit.
    pub stash_scope_explicit: bool,
    /// Apply/pop/drop/create-branch verbs stay distinct, never collapsed.
    pub restore_verbs_stay_distinct: bool,
    /// A recovery banner always names a concrete recovery destination.
    pub recovery_destination_always_concrete: bool,
    /// Recovery stays reachable until it is superseded or dismissed.
    pub recovery_reachable_until_superseded_or_dismissed: bool,
    /// The reflog/checkpoint expiry state is always disclosed.
    pub expiry_state_always_disclosed: bool,
    /// Local-only recovery stays explicit even with provider review state.
    pub local_only_recovery_stays_explicit: bool,
    /// One component contract is reused with no hidden per-surface meaning.
    pub one_component_contract_no_hidden_meaning: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified components automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl StashReflogRecoveryTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.stash_shorthand_never_only_label
            && self.stash_message_and_origin_explicit
            && self.stash_scope_explicit
            && self.restore_verbs_stay_distinct
            && self.recovery_destination_always_concrete
            && self.recovery_reachable_until_superseded_or_dismissed
            && self.expiry_state_always_disclosed
            && self.local_only_recovery_stays_explicit
            && self.one_component_contract_no_hidden_meaning
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashReflogRecoveryConsumerProjection {
    /// Git-history surfaces reuse one component contract.
    pub git_history_reuses_one_contract: bool,
    /// Review surfaces reuse one component contract.
    pub review_reuses_one_contract: bool,
    /// Help/support surfaces reuse one component contract.
    pub help_support_reuses_one_contract: bool,
    /// Support/export surfaces reuse one component contract.
    pub support_export_reuses_one_contract: bool,
    /// Recovery stays reachable across history, review, and help/support surfaces.
    pub recovery_reachable_across_surfaces: bool,
    /// CLI / headless shows component truth.
    pub cli_headless_shows_truth: bool,
    /// Provider overlay shows component truth without overwriting local truth.
    pub provider_overlay_shows_truth: bool,
    /// AI-context assembly shows component truth.
    pub ai_context_shows_truth: bool,
}

impl StashReflogRecoveryConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.git_history_reuses_one_contract
            && self.review_reuses_one_contract
            && self.help_support_reuses_one_contract
            && self.support_export_reuses_one_contract
            && self.recovery_reachable_across_surfaces
            && self.cli_headless_shows_truth
            && self.provider_overlay_shows_truth
            && self.ai_context_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashReflogRecoveryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`StashReflogRecoveryPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StashReflogRecoveryPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Stash-entry rows.
    pub stash_entries: Vec<StashEntryRow>,
    /// Reflog-recovery banner rows.
    pub reflog_banners: Vec<ReflogRecoveryBannerRow>,
    /// Shared downgrade states that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryDowngradeState>,
    /// Consumer surfaces that must reuse this component contract.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: StashReflogRecoveryTrustReview,
    /// Consumer projection block.
    pub consumer_projection: StashReflogRecoveryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: StashReflogRecoveryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe stash / reflog-recovery component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashReflogRecoveryPacket {
    /// Record kind; must equal [`STASH_REFLOG_RECOVERY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`STASH_REFLOG_RECOVERY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Stash-entry rows.
    pub stash_entries: Vec<StashEntryRow>,
    /// Reflog-recovery banner rows.
    pub reflog_banners: Vec<ReflogRecoveryBannerRow>,
    /// Shared downgrade states that apply to this lane.
    pub downgrade_triggers: Vec<GitHistoryDowngradeState>,
    /// Consumer surfaces that must reuse this component contract.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: StashReflogRecoveryTrustReview,
    /// Consumer projection block.
    pub consumer_projection: StashReflogRecoveryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: StashReflogRecoveryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl StashReflogRecoveryPacket {
    /// Builds a stash / reflog-recovery component packet from stable-lane input.
    pub fn new(input: StashReflogRecoveryPacketInput) -> Self {
        Self {
            record_kind: STASH_REFLOG_RECOVERY_RECORD_KIND.to_owned(),
            schema_version: STASH_REFLOG_RECOVERY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            stash_entries: input.stash_entries,
            reflog_banners: input.reflog_banners,
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

    /// Validates the stash / reflog-recovery invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<StashReflogRecoveryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != STASH_REFLOG_RECOVERY_RECORD_KIND {
            violations.push(StashReflogRecoveryViolation::WrongRecordKind);
        }
        if self.schema_version != STASH_REFLOG_RECOVERY_SCHEMA_VERSION {
            violations.push(StashReflogRecoveryViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(StashReflogRecoveryViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(StashReflogRecoveryViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(StashReflogRecoveryViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_stash_entries(self, &mut violations);
        validate_reflog_banners(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(StashReflogRecoveryViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(StashReflogRecoveryViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(StashReflogRecoveryViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("stash reflog recovery packet serializes"),
        ) {
            violations.push(StashReflogRecoveryViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("stash reflog recovery packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let reachable = self
            .reflog_banners
            .iter()
            .filter(|banner| banner.reachability.is_reachable())
            .count();
        let untracked = self
            .stash_entries
            .iter()
            .filter(|entry| entry.content_scope.includes_untracked())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Stash Entries and Reflog-Recovery Banners: Restore-Scope and Checkpoint Truth\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Stash entries: {} ({} include untracked content); reflog banners: {} ({} still reachable)\n",
            self.stash_entries.len(),
            untracked,
            self.reflog_banners.len(),
            reachable
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Stash entries\n\n");
        for entry in &self.stash_entries {
            let verbs = entry
                .restore_verbs
                .iter()
                .map(|verb| verb.as_str())
                .collect::<Vec<_>>()
                .join("/");
            out.push_str(&format!(
                "- `{}` — \"{}\" from `{}` [scope `{}`]: verbs {}\n",
                entry.stash_shorthand,
                entry.message,
                entry.created_from_ref,
                entry.content_scope.as_str(),
                verbs
            ));
        }

        out.push_str("\n## Reflog-recovery banners\n\n");
        for banner in &self.reflog_banners {
            out.push_str(&format!(
                "- **{}** → `{}` [{}, expiry `{}`]\n",
                banner.mutation_label,
                banner.recovery_destination,
                banner.reachability.as_str(),
                banner.expiry_state.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stash / reflog-recovery export.
#[derive(Debug)]
pub enum StashReflogRecoveryArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<StashReflogRecoveryViolation>),
}

impl fmt::Display for StashReflogRecoveryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "stash reflog recovery export parse failed: {error}"
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
                    "stash reflog recovery export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for StashReflogRecoveryArtifactError {}

/// Validation failures emitted by [`StashReflogRecoveryPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StashReflogRecoveryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No stash entries are present.
    StashEntriesMissing,
    /// No reflog-recovery banners are present.
    ReflogBannersMissing,
    /// A stash-entry row is incomplete.
    StashRowIncomplete,
    /// A stash-entry row's component is not `stash_entry`.
    WrongComponentForStashRow,
    /// A stash entry lets the `stash@{n}` shorthand be its only meaning-bearing label.
    StashShorthandOnlyLabel,
    /// A stash entry does not disclose its untracked/staged scope.
    StashScopeDisclosureMissing,
    /// A stash entry is missing one of apply/pop/drop/create-branch-from-stash.
    RestoreVerbCoverageMissing,
    /// A stash entry collapses/aliases its distinct restore verbs.
    RestoreVerbsCollapsed,
    /// A stash entry does not name how the stashed work stays recoverable.
    StashRecoveryNoteMissing,
    /// A reflog-recovery banner row is incomplete.
    BannerRowIncomplete,
    /// A banner row's component is not `reflog_recovery_banner`.
    WrongComponentForBannerRow,
    /// A reachable banner does not name a concrete recovery destination.
    RecoveryDestinationMissing,
    /// A reachable banner does not span history, review, and help/support surfaces.
    RecoveryNotReachableAcrossSurfaces,
    /// A banner does not disclose its expiry state.
    ExpiryStateUndisclosed,
    /// An expired or pruned recovery point still claims to be reachable.
    ExpiredRecoveryStillReachable,
    /// A banner exposes no in-product action and forces raw-provider navigation.
    ForcedRawProviderNavigation,
    /// No reachable recovery banner is present to prove recovery stays reachable.
    RecoveryReachabilityCoverageMissing,
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

impl StashReflogRecoveryViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::StashEntriesMissing => "stash_entries_missing",
            Self::ReflogBannersMissing => "reflog_banners_missing",
            Self::StashRowIncomplete => "stash_row_incomplete",
            Self::WrongComponentForStashRow => "wrong_component_for_stash_row",
            Self::StashShorthandOnlyLabel => "stash_shorthand_only_label",
            Self::StashScopeDisclosureMissing => "stash_scope_disclosure_missing",
            Self::RestoreVerbCoverageMissing => "restore_verb_coverage_missing",
            Self::RestoreVerbsCollapsed => "restore_verbs_collapsed",
            Self::StashRecoveryNoteMissing => "stash_recovery_note_missing",
            Self::BannerRowIncomplete => "banner_row_incomplete",
            Self::WrongComponentForBannerRow => "wrong_component_for_banner_row",
            Self::RecoveryDestinationMissing => "recovery_destination_missing",
            Self::RecoveryNotReachableAcrossSurfaces => "recovery_not_reachable_across_surfaces",
            Self::ExpiryStateUndisclosed => "expiry_state_undisclosed",
            Self::ExpiredRecoveryStillReachable => "expired_recovery_still_reachable",
            Self::ForcedRawProviderNavigation => "forced_raw_provider_navigation",
            Self::RecoveryReachabilityCoverageMissing => "recovery_reachability_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable stash / reflog-recovery export.
///
/// # Errors
///
/// Returns [`StashReflogRecoveryArtifactError`] when the checked-in export fails
/// to parse or violates the contract.
pub fn current_stash_reflog_recovery_export(
) -> Result<StashReflogRecoveryPacket, StashReflogRecoveryArtifactError> {
    let packet: StashReflogRecoveryPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-stash-reflog-recovery-components-proof/support_export.json"
    )))
    .map_err(StashReflogRecoveryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(StashReflogRecoveryArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &StashReflogRecoveryPacket,
    violations: &mut Vec<StashReflogRecoveryViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        STASH_REFLOG_RECOVERY_SCHEMA_REF,
        STASH_REFLOG_RECOVERY_DOC_REF,
        STASH_REFLOG_RECOVERY_COMPONENT_MATRIX_CONTRACT_REF,
        STASH_REFLOG_RECOVERY_STASH_CONTRACT_REF,
        STASH_REFLOG_RECOVERY_CHECKPOINT_CONTRACT_REF,
        STASH_REFLOG_RECOVERY_REVIEW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(StashReflogRecoveryViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_stash_entries(
    packet: &StashReflogRecoveryPacket,
    violations: &mut Vec<StashReflogRecoveryViolation>,
) {
    if packet.stash_entries.is_empty() {
        violations.push(StashReflogRecoveryViolation::StashEntriesMissing);
        return;
    }

    for entry in &packet.stash_entries {
        if entry.row_id.trim().is_empty()
            || entry.stash_shorthand.trim().is_empty()
            || entry.restore_verbs.is_empty()
            || entry.downgrade_vocab.is_empty()
            || entry.fields_shown.is_empty()
            || entry.source_contract_refs.is_empty()
        {
            violations.push(StashReflogRecoveryViolation::StashRowIncomplete);
        }

        if entry.component != M5GitHistoryComponent::StashEntry {
            violations.push(StashReflogRecoveryViolation::WrongComponentForStashRow);
        }

        if !entry.has_meaning_beyond_shorthand() {
            violations.push(StashReflogRecoveryViolation::StashShorthandOnlyLabel);
        }
        if entry.scope_disclosure.trim().is_empty() {
            violations.push(StashReflogRecoveryViolation::StashScopeDisclosureMissing);
        }
        if !entry.covers_required_restore_verbs() {
            violations.push(StashReflogRecoveryViolation::RestoreVerbCoverageMissing);
        }
        if !entry.restore_verbs_stay_distinct() {
            violations.push(StashReflogRecoveryViolation::RestoreVerbsCollapsed);
        }
        if entry.recovery_reflog_note.trim().is_empty() {
            violations.push(StashReflogRecoveryViolation::StashRecoveryNoteMissing);
        }
    }
}

fn validate_reflog_banners(
    packet: &StashReflogRecoveryPacket,
    violations: &mut Vec<StashReflogRecoveryViolation>,
) {
    if packet.reflog_banners.is_empty() {
        violations.push(StashReflogRecoveryViolation::ReflogBannersMissing);
        return;
    }

    let mut any_reachable = false;

    for banner in &packet.reflog_banners {
        if banner.reachability.is_reachable() {
            any_reachable = true;
        }

        if banner.row_id.trim().is_empty()
            || banner.mutation_label.trim().is_empty()
            || banner.restore_actions.is_empty()
            || banner.downgrade_vocab.is_empty()
            || banner.fields_shown.is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations.push(StashReflogRecoveryViolation::BannerRowIncomplete);
        }

        if banner.component != M5GitHistoryComponent::ReflogRecoveryBanner {
            violations.push(StashReflogRecoveryViolation::WrongComponentForBannerRow);
        }

        let disclosure = banner.disclosure();

        if disclosure.must_show_concrete_destination
            && banner.recovery_destination.trim().is_empty()
        {
            violations.push(StashReflogRecoveryViolation::RecoveryDestinationMissing);
        }
        if disclosure.must_span_history_review_help_support && !banner.spans_required_surfaces() {
            violations.push(StashReflogRecoveryViolation::RecoveryNotReachableAcrossSurfaces);
        }
        if banner.expiry_disclosure.trim().is_empty() {
            violations.push(StashReflogRecoveryViolation::ExpiryStateUndisclosed);
        }
        if banner.reachability.is_reachable() && banner.expiry_state.is_gone() {
            violations.push(StashReflogRecoveryViolation::ExpiredRecoveryStillReachable);
        }
        if !banner.has_in_product_action() {
            violations.push(StashReflogRecoveryViolation::ForcedRawProviderNavigation);
        }
    }

    if !any_reachable {
        violations.push(StashReflogRecoveryViolation::RecoveryReachabilityCoverageMissing);
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
