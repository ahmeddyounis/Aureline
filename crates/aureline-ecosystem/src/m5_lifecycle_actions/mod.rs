//! Canonical M5 lifecycle actions — disable, uninstall, rollback, quarantine,
//! and registry-status transitions modeled as distinct reviewed actions with
//! explicit continuity, data-retention, and restore semantics.
//!
//! Where the
//! [`install-governance matrix`](crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix)
//! freezes one governance row per marketed M5 artifact family, the
//! [`marketplace fact-views`](crate::m5_marketplace_fact_views) project that truth
//! into the storefront, and the [`install-review sheets`](crate::m5_install_review)
//! freeze how an install or update is reviewed *before commit*, this module freezes
//! what happens *after* a package is installed: how it is disabled, uninstalled,
//! rolled back, quarantined, or moved through a registry-status change such as
//! revocation, yank, deprecation, or publisher transfer.
//!
//! Each [`LifecycleActionRecord`] is honest about three things a generic warning
//! banner hides:
//!
//! - **continuity** — whether already-open views and background work
//!   [keep running temporarily](ContinuityDisposition::KeepsRunningTemporarily),
//!   [stop immediately](ContinuityDisposition::StopsImmediately), or
//!   [convert to placeholders at the next activation boundary](ContinuityDisposition::ConvertsToPlaceholderAtNextActivation),
//!   naming the M5-contributed surfaces affected rather than letting them silently
//!   disappear;
//! - **retained state** — a per-[`DataClass`] [`DataDisposition`] that makes
//!   user-owned recipes, docs-pack content, local history, and rollback checkpoints
//!   explicitly survivable, so an uninstall or disable can never silently delete
//!   protected data; and
//! - **rollback / restore** — a [`LifecycleRollbackNote`] carrying the
//!   last-known-good target, rollback posture, and a [`RollbackCompatibility`] note,
//!   plus a `restorable` flag and restore route, so rollback and restore are never
//!   implied to be risk free.
//!
//! The record is honest by construction. Its [`LifecycleReviewReason`] set and the
//! [`ActionDisposition`] it publishes are **not** stored by hand: they are
//! recomputed from the record's facts, and the stored values must equal that
//! recomputation or validation fails. This routes crash-loop, integrity-failure,
//! performance-budget, moderation, policy, revocation, yank, deprecation, and
//! publisher-transfer events through explicit lifecycle states instead of generic
//! banners: any reactive trigger forces at least
//! [`ActionDisposition::ReviewRequired`], an unconsented removal of protected
//! user-owned data or an irreversible rollback [`ActionDisposition::Blocked`]s the
//! action outright, and a clean user-initiated disable with full retention stays
//! [`ActionDisposition::ProceedAllowed`].
//!
//! Workspace- and global-scope actions stay visibly distinct: a
//! [`LifecycleActionKind::DisableWorkspace`] is bound to
//! [`InstallScope::Workspace`] and a [`LifecycleActionKind::DisableGlobal`] to
//! [`InstallScope::Global`], and every [`LifecycleOfferedAction`] carries the
//! record's scope so a workspace troubleshooting action can never silently apply
//! globally.
//!
//! The packet is checked in at `artifacts/ecosystem/m5/m5-lifecycle-actions.json`
//! and embedded here, so this typed consumer and any CI gate agree on every record
//! without a cargo build in CI. The model is metadata-only: every field is a typed
//! state or an opaque ref. It carries no credential bodies, raw provider payloads,
//! signing secrets, or registry tokens.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    ArtifactFamily, RollbackPosture,
};
use crate::m5_install_review::InstallScope;

/// Supported M5 lifecycle-actions schema version.
pub const M5_LIFECYCLE_ACTIONS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_LIFECYCLE_ACTIONS_RECORD_KIND: &str = "m5_lifecycle_actions";

/// Repo-relative path to the checked-in packet.
pub const M5_LIFECYCLE_ACTIONS_PATH: &str = "artifacts/ecosystem/m5/m5-lifecycle-actions.json";

/// Embedded checked-in packet JSON.
pub const M5_LIFECYCLE_ACTIONS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-lifecycle-actions.json"
));

/// A distinct, reviewed lifecycle action applied to an installed package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleActionKind {
    /// Disable the package for the current workspace only.
    DisableWorkspace,
    /// Disable the package globally across the install.
    DisableGlobal,
    /// Uninstall the package.
    Uninstall,
    /// Roll the package back to a last-known-good revision.
    Rollback,
    /// Quarantine the package pending review.
    Quarantine,
    /// Re-enable a previously disabled package.
    Reenable,
    /// Apply a registry- or publisher-imposed status change.
    ApplyRegistryStatus,
}

impl LifecycleActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DisableWorkspace,
        Self::DisableGlobal,
        Self::Uninstall,
        Self::Rollback,
        Self::Quarantine,
        Self::Reenable,
        Self::ApplyRegistryStatus,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisableWorkspace => "disable_workspace",
            Self::DisableGlobal => "disable_global",
            Self::Uninstall => "uninstall",
            Self::Rollback => "rollback",
            Self::Quarantine => "quarantine",
            Self::Reenable => "reenable",
            Self::ApplyRegistryStatus => "apply_registry_status",
        }
    }
}

/// What initiated a lifecycle action.
///
/// Reactive triggers route through explicit lifecycle states rather than a generic
/// warning banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleTrigger {
    /// A user or admin initiated the action.
    UserInitiated,
    /// The runtime crash-looped.
    CrashLoop,
    /// An integrity check failed.
    IntegrityFailure,
    /// The package exceeded its activation/performance budget.
    PerformanceBudgetExceeded,
    /// A moderation decision was applied.
    Moderation,
    /// A policy decision was applied.
    Policy,
    /// The registry revoked the package.
    Revocation,
    /// The registry yanked the revision.
    Yank,
    /// The publisher deprecated the package.
    Deprecation,
    /// Ownership transferred to a new publisher.
    PublisherTransfer,
}

impl LifecycleTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::UserInitiated,
        Self::CrashLoop,
        Self::IntegrityFailure,
        Self::PerformanceBudgetExceeded,
        Self::Moderation,
        Self::Policy,
        Self::Revocation,
        Self::Yank,
        Self::Deprecation,
        Self::PublisherTransfer,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserInitiated => "user_initiated",
            Self::CrashLoop => "crash_loop",
            Self::IntegrityFailure => "integrity_failure",
            Self::PerformanceBudgetExceeded => "performance_budget_exceeded",
            Self::Moderation => "moderation",
            Self::Policy => "policy",
            Self::Revocation => "revocation",
            Self::Yank => "yank",
            Self::Deprecation => "deprecation",
            Self::PublisherTransfer => "publisher_transfer",
        }
    }

    /// Whether this trigger is reactive (not directly user-initiated).
    pub const fn is_reactive(self) -> bool {
        !matches!(self, Self::UserInitiated)
    }

    /// Whether this trigger is an automated health signal.
    pub const fn is_automated_health(self) -> bool {
        matches!(
            self,
            Self::CrashLoop | Self::IntegrityFailure | Self::PerformanceBudgetExceeded
        )
    }

    /// Whether this trigger is a moderation or policy decision.
    pub const fn is_moderation_or_policy(self) -> bool {
        matches!(self, Self::Moderation | Self::Policy)
    }

    /// Whether this trigger is a registry-status change.
    pub const fn is_registry_status(self) -> bool {
        matches!(self, Self::Revocation | Self::Yank | Self::Deprecation)
    }
}

/// The lifecycle state a package occupies after an action resolves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStatus {
    /// Installed and active.
    Active,
    /// Disabled for the current workspace only.
    DisabledWorkspace,
    /// Disabled globally across the install.
    DisabledGlobal,
    /// Uninstalled.
    Uninstalled,
    /// Rolled back to a last-known-good revision.
    RolledBack,
    /// Quarantined pending review.
    Quarantined,
    /// Revoked by the registry.
    Revoked,
    /// Yanked from the registry.
    Yanked,
    /// Deprecated by the publisher.
    Deprecated,
    /// Moved to a new publisher.
    PublisherTransferred,
}

impl LifecycleStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::Active,
        Self::DisabledWorkspace,
        Self::DisabledGlobal,
        Self::Uninstalled,
        Self::RolledBack,
        Self::Quarantined,
        Self::Revoked,
        Self::Yanked,
        Self::Deprecated,
        Self::PublisherTransferred,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::DisabledWorkspace => "disabled_workspace",
            Self::DisabledGlobal => "disabled_global",
            Self::Uninstalled => "uninstalled",
            Self::RolledBack => "rolled_back",
            Self::Quarantined => "quarantined",
            Self::Revoked => "revoked",
            Self::Yanked => "yanked",
            Self::Deprecated => "deprecated",
            Self::PublisherTransferred => "publisher_transferred",
        }
    }
}

/// What happens to already-open views and background work when an action resolves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityDisposition {
    /// Open views keep running until the next activation boundary.
    KeepsRunningTemporarily,
    /// Open views stop immediately.
    StopsImmediately,
    /// Open views convert to placeholders at the next activation boundary.
    ConvertsToPlaceholderAtNextActivation,
    /// No open views or background work are affected.
    NotApplicable,
}

impl ContinuityDisposition {
    /// Every continuity disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::KeepsRunningTemporarily,
        Self::StopsImmediately,
        Self::ConvertsToPlaceholderAtNextActivation,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeepsRunningTemporarily => "keeps_running_temporarily",
            Self::StopsImmediately => "stops_immediately",
            Self::ConvertsToPlaceholderAtNextActivation => {
                "converts_to_placeholder_at_next_activation"
            }
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this disposition disrupts currently open work.
    ///
    /// Keeping views running temporarily preserves open work; stopping immediately
    /// or converting to placeholders disrupts it.
    pub const fn disrupts_open_work(self) -> bool {
        matches!(
            self,
            Self::StopsImmediately | Self::ConvertsToPlaceholderAtNextActivation
        )
    }

    /// Whether open views convert to placeholders.
    pub const fn converts_to_placeholder(self) -> bool {
        matches!(self, Self::ConvertsToPlaceholderAtNextActivation)
    }
}

/// A class of state a lifecycle action may retain or remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataClass {
    /// User-authored recipes.
    UserRecipes,
    /// Docs-pack content.
    DocsPackContent,
    /// Local edit/run history.
    LocalHistory,
    /// Rollback checkpoints.
    RollbackCheckpoints,
    /// Workspace-scoped settings.
    WorkspaceSettings,
    /// Regenerable cached indexes.
    CachedIndexes,
    /// Downloaded model weights.
    ModelWeights,
    /// Generated placeholder artifacts.
    GeneratedPlaceholders,
}

impl DataClass {
    /// Every data class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::UserRecipes,
        Self::DocsPackContent,
        Self::LocalHistory,
        Self::RollbackCheckpoints,
        Self::WorkspaceSettings,
        Self::CachedIndexes,
        Self::ModelWeights,
        Self::GeneratedPlaceholders,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserRecipes => "user_recipes",
            Self::DocsPackContent => "docs_pack_content",
            Self::LocalHistory => "local_history",
            Self::RollbackCheckpoints => "rollback_checkpoints",
            Self::WorkspaceSettings => "workspace_settings",
            Self::CachedIndexes => "cached_indexes",
            Self::ModelWeights => "model_weights",
            Self::GeneratedPlaceholders => "generated_placeholders",
        }
    }

    /// Whether this class is user-owned data the lane guardrail must protect.
    ///
    /// Recipes, docs-pack content, local history, and rollback checkpoints must
    /// never be silently deleted by an uninstall or disable.
    pub const fn is_protected(self) -> bool {
        matches!(
            self,
            Self::UserRecipes
                | Self::DocsPackContent
                | Self::LocalHistory
                | Self::RollbackCheckpoints
        )
    }
}

/// What a lifecycle action does to a given class of state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataDisposition {
    /// The state is retained intact.
    Retained,
    /// The state is removed only after explicit user consent.
    RemovedWithExplicitConsent,
    /// The state is a regenerable cache and is dropped without loss.
    RegenerableCache,
}

impl DataDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Retained,
        Self::RemovedWithExplicitConsent,
        Self::RegenerableCache,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retained => "retained",
            Self::RemovedWithExplicitConsent => "removed_with_explicit_consent",
            Self::RegenerableCache => "regenerable_cache",
        }
    }

    /// Whether this disposition removes user-owned state.
    pub const fn removes_owned_state(self) -> bool {
        matches!(self, Self::RemovedWithExplicitConsent)
    }
}

/// How safely a rollback or restore can be performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackCompatibility {
    /// The rollback is clean and fully reversible.
    Clean,
    /// The rollback requires a compatibility recheck before re-activation.
    RequiresRecompat,
    /// The rollback may lose state accumulated since the target revision.
    StateLossPossible,
    /// The rollback cannot be reversed once applied.
    NotReversible,
    /// No rollback applies to this action.
    NotApplicable,
}

impl RollbackCompatibility {
    /// Every compatibility class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Clean,
        Self::RequiresRecompat,
        Self::StateLossPossible,
        Self::NotReversible,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::RequiresRecompat => "requires_recompat",
            Self::StateLossPossible => "state_loss_possible",
            Self::NotReversible => "not_reversible",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether a rollback under this compatibility is not risk free.
    pub const fn is_not_risk_free(self) -> bool {
        matches!(self, Self::RequiresRecompat | Self::StateLossPossible)
    }
}

/// A reason a lifecycle action escalates above a direct, proceed-allowed action.
///
/// Each reason is recomputed from the record's facts; the record's stored
/// [`LifecycleActionRecord::review_reasons`] must equal the recomputed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleReviewReason {
    /// Protected user-owned data would be removed without captured consent.
    UnconsentedProtectedDataRemoval,
    /// The rollback cannot be reversed once applied.
    RollbackNotReversible,
    /// An automated health signal (crash loop, integrity, or budget) drove the action.
    AutomatedHealthTrigger,
    /// A moderation or policy decision drove the action.
    ModerationOrPolicyTrigger,
    /// A registry-status change (revocation, yank, or deprecation) drove the action.
    RegistryStatusTrigger,
    /// A publisher transfer drove the action.
    PublisherTransferTrigger,
    /// The action disrupts currently open work.
    OpenWorkDisruption,
    /// The action cannot be restored once applied.
    IrreversibleAction,
    /// The rollback is reversible but not risk free.
    RollbackNotRiskFree,
    /// Protected user-owned data is removed under disclosed, captured consent.
    DisclosedProtectedDataRemoval,
}

impl LifecycleReviewReason {
    /// Every review reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::UnconsentedProtectedDataRemoval,
        Self::RollbackNotReversible,
        Self::AutomatedHealthTrigger,
        Self::ModerationOrPolicyTrigger,
        Self::RegistryStatusTrigger,
        Self::PublisherTransferTrigger,
        Self::OpenWorkDisruption,
        Self::IrreversibleAction,
        Self::RollbackNotRiskFree,
        Self::DisclosedProtectedDataRemoval,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnconsentedProtectedDataRemoval => "unconsented_protected_data_removal",
            Self::RollbackNotReversible => "rollback_not_reversible",
            Self::AutomatedHealthTrigger => "automated_health_trigger",
            Self::ModerationOrPolicyTrigger => "moderation_or_policy_trigger",
            Self::RegistryStatusTrigger => "registry_status_trigger",
            Self::PublisherTransferTrigger => "publisher_transfer_trigger",
            Self::OpenWorkDisruption => "open_work_disruption",
            Self::IrreversibleAction => "irreversible_action",
            Self::RollbackNotRiskFree => "rollback_not_risk_free",
            Self::DisclosedProtectedDataRemoval => "disclosed_protected_data_removal",
        }
    }

    /// The minimum action disposition this reason forces.
    pub const fn min_disposition(self) -> ActionDisposition {
        match self {
            // Silently deleting protected data or an irreversible rollback are hard
            // stops: the action must not proceed as proposed.
            Self::UnconsentedProtectedDataRemoval | Self::RollbackNotReversible => {
                ActionDisposition::Blocked
            }
            // Every other reason forces an explicit review but does not block.
            Self::AutomatedHealthTrigger
            | Self::ModerationOrPolicyTrigger
            | Self::RegistryStatusTrigger
            | Self::PublisherTransferTrigger
            | Self::OpenWorkDisruption
            | Self::IrreversibleAction
            | Self::RollbackNotRiskFree
            | Self::DisclosedProtectedDataRemoval => ActionDisposition::ReviewRequired,
        }
    }
}

/// The disposition a lifecycle action publishes.
///
/// Ordered low-to-high by [`ActionDisposition::rank`]: a
/// [`ActionDisposition::ProceedAllowed`] action may be applied directly, and a
/// [`ActionDisposition::Blocked`] action must not be applied as proposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionDisposition {
    /// A direct action is allowed; no reason applies.
    ProceedAllowed,
    /// The action must be reviewed and acknowledged before it is applied.
    ReviewRequired,
    /// The action is blocked as proposed.
    Blocked,
}

impl ActionDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 3] = [Self::ProceedAllowed, Self::ReviewRequired, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedAllowed => "proceed_allowed",
            Self::ReviewRequired => "review_required",
            Self::Blocked => "blocked",
        }
    }

    /// Monotonic rank; higher means a stricter gate.
    pub const fn rank(self) -> u8 {
        match self {
            Self::ProceedAllowed => 0,
            Self::ReviewRequired => 1,
            Self::Blocked => 2,
        }
    }

    /// The stricter (higher-rank) of two dispositions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// One class of state and what the action does to it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedStateImpact {
    /// The class of state.
    pub data_class: DataClass,
    /// What the action does to it.
    pub disposition: DataDisposition,
    /// Opaque ref to the retained-state note for this class.
    pub note_ref: String,
}

impl RetainedStateImpact {
    /// Whether this impact removes protected user-owned data.
    pub const fn removes_protected_data(&self) -> bool {
        self.data_class.is_protected() && self.disposition.removes_owned_state()
    }
}

/// The rollback / restore note attached to a lifecycle action.
///
/// The note keeps recovery honest: it names the last-known-good target, the
/// rollback posture, and a compatibility note, so rollback and restore are never
/// implied to be risk free.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleRollbackNote {
    /// How safely the rollback or restore can be performed.
    pub rollback_compatibility: RollbackCompatibility,
    /// Rollback posture of the action.
    pub rollback_posture: RollbackPosture,
    /// Opaque ref to the last-known-good target, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_ref: Option<String>,
    /// Opaque ref to the retained-state note that accompanies the rollback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retained_state_note_ref: Option<String>,
}

/// A scoped action offered through the lifecycle record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleOfferedAction {
    /// The kind of action.
    pub action_kind: LifecycleActionKind,
    /// The scope the action applies to; must equal the record's scope.
    pub scope: InstallScope,
    /// Opaque ref to the action.
    pub action_ref: String,
    /// Whether the action is currently enabled.
    pub enabled: bool,
}

/// One lifecycle action record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleActionRecord {
    /// Stable record id.
    pub record_id: String,
    /// Opaque ref to the catalog listing under action.
    pub listing_ref: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Ref to the governance-matrix family this listing resolves through.
    pub governance_family_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// The lifecycle action.
    pub action_kind: LifecycleActionKind,
    /// The scope the action applies to.
    pub scope: InstallScope,
    /// What initiated the action.
    pub trigger: LifecycleTrigger,
    /// The lifecycle state after the action resolves.
    pub resulting_status: LifecycleStatus,
    /// What happens to already-open views and background work.
    pub continuity: ContinuityDisposition,
    /// Opaque refs to the M5-contributed surfaces affected by the continuity change.
    #[serde(default)]
    pub contributed_surface_refs: Vec<String>,
    /// Per-class retained-state impacts.
    #[serde(default)]
    pub retained_state: Vec<RetainedStateImpact>,
    /// Whether explicit consent was captured for any protected-data removal.
    pub destructive_consent_captured: bool,
    /// Rollback / restore note.
    pub rollback: LifecycleRollbackNote,
    /// Whether the package can be restored after this action.
    pub restorable: bool,
    /// Opaque ref to the restore route, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_path_ref: Option<String>,
    /// Scoped actions offered through the record.
    #[serde(default)]
    pub offered_actions: Vec<LifecycleOfferedAction>,
    /// Review reasons; must equal the recomputed set.
    #[serde(default)]
    pub review_reasons: Vec<LifecycleReviewReason>,
    /// Action disposition; must equal the recomputed value.
    pub action_disposition: ActionDisposition,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl LifecycleActionRecord {
    /// Whether protected user-owned data would be removed without captured consent.
    pub fn removes_protected_data_unconsented(&self) -> bool {
        !self.destructive_consent_captured
            && self
                .retained_state
                .iter()
                .any(RetainedStateImpact::removes_protected_data)
    }

    /// Whether protected user-owned data is removed under disclosed, captured consent.
    pub fn removes_protected_data_disclosed(&self) -> bool {
        self.destructive_consent_captured
            && self
                .retained_state
                .iter()
                .any(RetainedStateImpact::removes_protected_data)
    }

    /// Whether every protected data class is preserved by this action.
    pub fn preserves_protected_data(&self) -> bool {
        !self
            .retained_state
            .iter()
            .any(RetainedStateImpact::removes_protected_data)
    }

    /// Whether the rollback cannot be reversed once applied.
    pub fn rollback_not_reversible(&self) -> bool {
        self.action_kind == LifecycleActionKind::Rollback
            && self.rollback.rollback_compatibility == RollbackCompatibility::NotReversible
    }

    /// Whether the rollback is reversible but not risk free.
    pub fn rollback_not_risk_free(&self) -> bool {
        self.action_kind == LifecycleActionKind::Rollback
            && self.rollback.rollback_compatibility.is_not_risk_free()
    }

    /// Whether this action disrupts currently open work.
    pub fn disrupts_open_work(&self) -> bool {
        self.continuity.disrupts_open_work()
    }

    /// The review reasons recomputed from this record's facts, in canonical order.
    pub fn computed_review_reasons(&self) -> Vec<LifecycleReviewReason> {
        let mut reasons = Vec::new();
        if self.removes_protected_data_unconsented() {
            reasons.push(LifecycleReviewReason::UnconsentedProtectedDataRemoval);
        }
        if self.rollback_not_reversible() {
            reasons.push(LifecycleReviewReason::RollbackNotReversible);
        }
        if self.trigger.is_automated_health() {
            reasons.push(LifecycleReviewReason::AutomatedHealthTrigger);
        }
        if self.trigger.is_moderation_or_policy() {
            reasons.push(LifecycleReviewReason::ModerationOrPolicyTrigger);
        }
        if self.trigger.is_registry_status() {
            reasons.push(LifecycleReviewReason::RegistryStatusTrigger);
        }
        if self.trigger == LifecycleTrigger::PublisherTransfer {
            reasons.push(LifecycleReviewReason::PublisherTransferTrigger);
        }
        if self.disrupts_open_work() {
            reasons.push(LifecycleReviewReason::OpenWorkDisruption);
        }
        if !self.restorable {
            reasons.push(LifecycleReviewReason::IrreversibleAction);
        }
        if self.rollback_not_risk_free() {
            reasons.push(LifecycleReviewReason::RollbackNotRiskFree);
        }
        if self.removes_protected_data_disclosed() {
            reasons.push(LifecycleReviewReason::DisclosedProtectedDataRemoval);
        }
        reasons
    }

    /// The action disposition recomputed from this record's reasons.
    pub fn computed_action_disposition(&self) -> ActionDisposition {
        self.computed_review_reasons()
            .into_iter()
            .fold(ActionDisposition::ProceedAllowed, |disposition, reason| {
                disposition.widen(reason.min_disposition())
            })
    }

    /// Whether the stored reasons and disposition agree with the recomputed values.
    pub fn gate_consistent(&self) -> bool {
        self.review_reasons == self.computed_review_reasons()
            && self.action_disposition == self.computed_action_disposition()
    }

    /// Whether a direct action is allowed.
    pub fn proceeds_directly(&self) -> bool {
        self.action_disposition == ActionDisposition::ProceedAllowed
    }

    /// The offered action matching this record's action kind, if any.
    pub fn primary_action(&self) -> Option<&LifecycleOfferedAction> {
        self.offered_actions
            .iter()
            .find(|a| a.action_kind == self.action_kind)
    }

    /// Whether the record offers an action of the given kind.
    pub fn offers_action(&self, kind: LifecycleActionKind) -> bool {
        self.offered_actions.iter().any(|a| a.action_kind == kind)
    }

    /// The lifecycle status the action kind must resolve to, when fixed.
    ///
    /// Registry-status actions resolve to one of several statuses keyed by trigger,
    /// so they return [`None`] here and are checked separately.
    const fn expected_status(self_kind: LifecycleActionKind) -> Option<LifecycleStatus> {
        match self_kind {
            LifecycleActionKind::DisableWorkspace => Some(LifecycleStatus::DisabledWorkspace),
            LifecycleActionKind::DisableGlobal => Some(LifecycleStatus::DisabledGlobal),
            LifecycleActionKind::Uninstall => Some(LifecycleStatus::Uninstalled),
            LifecycleActionKind::Rollback => Some(LifecycleStatus::RolledBack),
            LifecycleActionKind::Quarantine => Some(LifecycleStatus::Quarantined),
            LifecycleActionKind::Reenable => Some(LifecycleStatus::Active),
            LifecycleActionKind::ApplyRegistryStatus => None,
        }
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5LifecycleActionsSummary {
    /// Total lifecycle records.
    pub total_records: usize,
    /// Records that allow a direct action.
    pub proceed_allowed_records: usize,
    /// Records that require review.
    pub review_required_records: usize,
    /// Records whose action is blocked.
    pub blocked_records: usize,
    /// Records that disable for a workspace.
    pub workspace_disable_records: usize,
    /// Records that disable globally.
    pub global_disable_records: usize,
    /// Records that uninstall.
    pub uninstall_records: usize,
    /// Records that roll back.
    pub rollback_records: usize,
    /// Records that quarantine.
    pub quarantine_records: usize,
    /// Records that re-enable.
    pub reenable_records: usize,
    /// Records that apply a registry-status change.
    pub registry_status_records: usize,
    /// Records driven by a reactive trigger.
    pub reactive_trigger_records: usize,
    /// Records that convert open views to placeholders.
    pub placeholder_conversion_records: usize,
    /// Records that preserve every protected data class.
    pub protected_data_preserved_records: usize,
    /// Records that remove protected data under disclosed consent.
    pub disclosed_protected_removal_records: usize,
    /// Records that can be restored after the action.
    pub restorable_records: usize,
    /// Distinct package kinds across records.
    pub distinct_package_kinds: usize,
}

/// A redaction-safe export row projected from a lifecycle record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleActionsExportRow {
    /// Record id.
    pub record_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Action-kind token.
    pub action_kind: String,
    /// Scope token.
    pub scope: String,
    /// Trigger token.
    pub trigger: String,
    /// Resulting-status token.
    pub resulting_status: String,
    /// Continuity token.
    pub continuity: String,
    /// Action-disposition token.
    pub action_disposition: String,
    /// Review-reason tokens.
    pub review_reasons: Vec<String>,
    /// Rollback-compatibility token.
    pub rollback_compatibility: String,
    /// Last-known-good target ref, when one applies.
    pub last_known_good_ref: Option<String>,
    /// Whether the package can be restored after the action.
    pub restorable: bool,
    /// Whether every protected data class is preserved.
    pub preserves_protected_data: bool,
    /// Governance-matrix family ref.
    pub governance_family_ref: String,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleActionsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5LifecycleActionsExportRow>,
    /// Whether every record's gate is consistent with its recomputation.
    pub all_gates_consistent: bool,
    /// Records that require review.
    pub review_required_count: usize,
    /// Records whose action is blocked.
    pub blocked_count: usize,
}

/// The typed M5 lifecycle-actions packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5LifecycleActions {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed package-kind vocabulary (reused from the governance matrix).
    pub package_kinds: Vec<ArtifactFamily>,
    /// Closed scope vocabulary (reused from the install-review sheets).
    pub scopes: Vec<InstallScope>,
    /// Closed rollback-posture vocabulary (reused from the governance matrix).
    pub rollback_postures: Vec<RollbackPosture>,
    /// Closed action-kind vocabulary.
    pub action_kinds: Vec<LifecycleActionKind>,
    /// Closed trigger vocabulary.
    pub triggers: Vec<LifecycleTrigger>,
    /// Closed lifecycle-status vocabulary.
    pub lifecycle_statuses: Vec<LifecycleStatus>,
    /// Closed continuity-disposition vocabulary.
    pub continuity_dispositions: Vec<ContinuityDisposition>,
    /// Closed data-class vocabulary.
    pub data_classes: Vec<DataClass>,
    /// Closed data-disposition vocabulary.
    pub data_dispositions: Vec<DataDisposition>,
    /// Closed rollback-compatibility vocabulary.
    pub rollback_compatibilities: Vec<RollbackCompatibility>,
    /// Closed review-reason vocabulary.
    pub review_reasons: Vec<LifecycleReviewReason>,
    /// Closed action-disposition vocabulary.
    pub action_dispositions: Vec<ActionDisposition>,
    /// The lifecycle action records.
    #[serde(default)]
    pub records: Vec<LifecycleActionRecord>,
    /// Summary counts.
    pub summary: M5LifecycleActionsSummary,
}

impl M5LifecycleActions {
    /// Returns the record with the given id.
    pub fn record(&self, record_id: &str) -> Option<&LifecycleActionRecord> {
        self.records.iter().find(|r| r.record_id == record_id)
    }

    /// Records scoped to the given scope.
    pub fn records_in_scope(
        &self,
        scope: InstallScope,
    ) -> impl Iterator<Item = &LifecycleActionRecord> {
        self.records.iter().filter(move |r| r.scope == scope)
    }

    /// Records that require review or are blocked.
    pub fn records_requiring_review(&self) -> impl Iterator<Item = &LifecycleActionRecord> {
        self.records.iter().filter(|r| !r.proceeds_directly())
    }

    /// Whether every record's stored gate agrees with its recomputation.
    pub fn all_gates_consistent(&self) -> bool {
        self.records
            .iter()
            .all(LifecycleActionRecord::gate_consistent)
    }

    /// Recomputes the summary block from the records.
    pub fn computed_summary(&self) -> M5LifecycleActionsSummary {
        let count_disposition = |disposition: ActionDisposition| {
            self.records
                .iter()
                .filter(|r| r.action_disposition == disposition)
                .count()
        };
        let count_action = |kind: LifecycleActionKind| {
            self.records
                .iter()
                .filter(|r| r.action_kind == kind)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.records.iter().map(|r| r.package_kind).collect();
        M5LifecycleActionsSummary {
            total_records: self.records.len(),
            proceed_allowed_records: count_disposition(ActionDisposition::ProceedAllowed),
            review_required_records: count_disposition(ActionDisposition::ReviewRequired),
            blocked_records: count_disposition(ActionDisposition::Blocked),
            workspace_disable_records: count_action(LifecycleActionKind::DisableWorkspace),
            global_disable_records: count_action(LifecycleActionKind::DisableGlobal),
            uninstall_records: count_action(LifecycleActionKind::Uninstall),
            rollback_records: count_action(LifecycleActionKind::Rollback),
            quarantine_records: count_action(LifecycleActionKind::Quarantine),
            reenable_records: count_action(LifecycleActionKind::Reenable),
            registry_status_records: count_action(LifecycleActionKind::ApplyRegistryStatus),
            reactive_trigger_records: self
                .records
                .iter()
                .filter(|r| r.trigger.is_reactive())
                .count(),
            placeholder_conversion_records: self
                .records
                .iter()
                .filter(|r| r.continuity.converts_to_placeholder())
                .count(),
            protected_data_preserved_records: self
                .records
                .iter()
                .filter(|r| r.preserves_protected_data())
                .count(),
            disclosed_protected_removal_records: self
                .records
                .iter()
                .filter(|r| r.removes_protected_data_disclosed())
                .count(),
            restorable_records: self.records.iter().filter(|r| r.restorable).count(),
            distinct_package_kinds: package_kinds.len(),
        }
    }

    /// Produces an export projection that downstream surfaces — support exports,
    /// docs/help, and release/public-truth packets — render instead of restating
    /// lifecycle state, continuity, and disposition text by hand.
    pub fn export_projection(&self) -> M5LifecycleActionsExportProjection {
        let rows = self
            .records
            .iter()
            .map(|r| M5LifecycleActionsExportRow {
                record_id: r.record_id.clone(),
                package_kind: r.package_kind.as_str().to_owned(),
                action_kind: r.action_kind.as_str().to_owned(),
                scope: r.scope.as_str().to_owned(),
                trigger: r.trigger.as_str().to_owned(),
                resulting_status: r.resulting_status.as_str().to_owned(),
                continuity: r.continuity.as_str().to_owned(),
                action_disposition: r.action_disposition.as_str().to_owned(),
                review_reasons: r
                    .review_reasons
                    .iter()
                    .map(|reason| reason.as_str().to_owned())
                    .collect(),
                rollback_compatibility: r.rollback.rollback_compatibility.as_str().to_owned(),
                last_known_good_ref: r.rollback.last_known_good_ref.clone(),
                restorable: r.restorable,
                preserves_protected_data: r.preserves_protected_data(),
                governance_family_ref: r.governance_family_ref.clone(),
                summary: format!(
                    "{}: {} in {} scope via {}, status {}, continuity {}, disposition {}",
                    r.package_kind.as_str(),
                    r.action_kind.as_str(),
                    r.scope.as_str(),
                    r.trigger.as_str(),
                    r.resulting_status.as_str(),
                    r.continuity.as_str(),
                    r.action_disposition.as_str(),
                ),
            })
            .collect();
        M5LifecycleActionsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_gates_consistent: self.all_gates_consistent(),
            review_required_count: self
                .records
                .iter()
                .filter(|r| r.action_disposition == ActionDisposition::ReviewRequired)
                .count(),
            blocked_count: self
                .records
                .iter()
                .filter(|r| r.action_disposition == ActionDisposition::Blocked)
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5LifecycleActionsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen = BTreeSet::new();
        for record in &self.records {
            if !seen.insert(record.record_id.clone()) {
                violations.push(M5LifecycleActionsViolation::DuplicateRecordId {
                    record_id: record.record_id.clone(),
                });
            }
            self.validate_record(record, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5LifecycleActionsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5LifecycleActionsViolation>) {
        if self.schema_version != M5_LIFECYCLE_ACTIONS_SCHEMA_VERSION {
            violations.push(M5LifecycleActionsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_LIFECYCLE_ACTIONS_RECORD_KIND {
            violations.push(M5LifecycleActionsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5LifecycleActionsViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "package_kinds",
                self.package_kinds == ArtifactFamily::ALL.to_vec(),
            ),
            ("scopes", self.scopes == InstallScope::ALL.to_vec()),
            (
                "rollback_postures",
                self.rollback_postures == RollbackPosture::ALL.to_vec(),
            ),
            (
                "action_kinds",
                self.action_kinds == LifecycleActionKind::ALL.to_vec(),
            ),
            ("triggers", self.triggers == LifecycleTrigger::ALL.to_vec()),
            (
                "lifecycle_statuses",
                self.lifecycle_statuses == LifecycleStatus::ALL.to_vec(),
            ),
            (
                "continuity_dispositions",
                self.continuity_dispositions == ContinuityDisposition::ALL.to_vec(),
            ),
            ("data_classes", self.data_classes == DataClass::ALL.to_vec()),
            (
                "data_dispositions",
                self.data_dispositions == DataDisposition::ALL.to_vec(),
            ),
            (
                "rollback_compatibilities",
                self.rollback_compatibilities == RollbackCompatibility::ALL.to_vec(),
            ),
            (
                "review_reasons",
                self.review_reasons == LifecycleReviewReason::ALL.to_vec(),
            ),
            (
                "action_dispositions",
                self.action_dispositions == ActionDisposition::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5LifecycleActionsViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_record(
        &self,
        record: &LifecycleActionRecord,
        violations: &mut Vec<M5LifecycleActionsViolation>,
    ) {
        for (field, value) in [
            ("record_id", &record.record_id),
            ("listing_ref", &record.listing_ref),
            ("display_label", &record.display_label),
            ("governance_family_ref", &record.governance_family_ref),
            ("summary", &record.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5LifecycleActionsViolation::EmptyField {
                    id: record.record_id.clone(),
                    field_name: field,
                });
            }
        }
        for surface in &record.contributed_surface_refs {
            if surface.trim().is_empty() {
                violations.push(M5LifecycleActionsViolation::EmptyField {
                    id: record.record_id.clone(),
                    field_name: "contributed_surface_refs",
                });
            }
        }
        for impact in &record.retained_state {
            if impact.note_ref.trim().is_empty() {
                violations.push(M5LifecycleActionsViolation::EmptyField {
                    id: record.record_id.clone(),
                    field_name: "retained_state.note_ref",
                });
            }
        }

        self.validate_record_states(record, violations);
        self.validate_record_rollback(record, violations);
        self.validate_record_actions(record, violations);
        self.validate_record_gate(record, violations);
    }

    fn validate_record_states(
        &self,
        record: &LifecycleActionRecord,
        violations: &mut Vec<M5LifecycleActionsViolation>,
    ) {
        // Workspace/global disable actions are bound to their scope, so a workspace
        // troubleshooting action can never silently apply globally and vice versa.
        let scope_ok = match record.action_kind {
            LifecycleActionKind::DisableWorkspace => record.scope == InstallScope::Workspace,
            LifecycleActionKind::DisableGlobal => record.scope == InstallScope::Global,
            _ => true,
        };
        if !scope_ok {
            violations.push(M5LifecycleActionsViolation::ActionScopeKindMismatch {
                record_id: record.record_id.clone(),
                action: record.action_kind.as_str(),
            });
        }

        // The resulting status must match the action kind, so a disable can never be
        // recorded as an uninstall or vice versa.
        if let Some(expected) = LifecycleActionRecord::expected_status(record.action_kind) {
            if record.resulting_status != expected {
                violations.push(M5LifecycleActionsViolation::StatusMismatch {
                    record_id: record.record_id.clone(),
                    action: record.action_kind.as_str(),
                    status: record.resulting_status.as_str(),
                });
            }
        } else {
            // A registry-status action must pair its status with the matching trigger.
            let pairing_ok = matches!(
                (record.resulting_status, record.trigger),
                (LifecycleStatus::Revoked, LifecycleTrigger::Revocation)
                    | (LifecycleStatus::Yanked, LifecycleTrigger::Yank)
                    | (LifecycleStatus::Deprecated, LifecycleTrigger::Deprecation)
                    | (
                        LifecycleStatus::PublisherTransferred,
                        LifecycleTrigger::PublisherTransfer
                    )
            );
            if !pairing_ok {
                violations.push(M5LifecycleActionsViolation::RegistryStatusPairingMismatch {
                    record_id: record.record_id.clone(),
                    status: record.resulting_status.as_str(),
                    trigger: record.trigger.as_str(),
                });
            }
        }

        // A quarantine is a health/moderation/policy hold, never a registry-status
        // change in disguise.
        if record.action_kind == LifecycleActionKind::Quarantine
            && (record.trigger.is_registry_status()
                || record.trigger == LifecycleTrigger::PublisherTransfer)
        {
            violations.push(M5LifecycleActionsViolation::QuarantineTriggerMismatch {
                record_id: record.record_id.clone(),
                trigger: record.trigger.as_str(),
            });
        }

        // A revoked or yanked package cannot be self-restored, so it must not claim
        // to be restorable.
        if matches!(
            record.resulting_status,
            LifecycleStatus::Revoked | LifecycleStatus::Yanked
        ) && record.restorable
        {
            violations.push(M5LifecycleActionsViolation::RevokedRestorableInconsistent {
                record_id: record.record_id.clone(),
            });
        }
    }

    fn validate_record_rollback(
        &self,
        record: &LifecycleActionRecord,
        violations: &mut Vec<M5LifecycleActionsViolation>,
    ) {
        if record.action_kind == LifecycleActionKind::Rollback {
            if record.rollback.rollback_compatibility == RollbackCompatibility::NotApplicable {
                violations.push(M5LifecycleActionsViolation::RollbackTargetMissing {
                    record_id: record.record_id.clone(),
                    field_name: "rollback_compatibility",
                });
            }
            if record
                .rollback
                .last_known_good_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
            {
                violations.push(M5LifecycleActionsViolation::RollbackTargetMissing {
                    record_id: record.record_id.clone(),
                    field_name: "last_known_good_ref",
                });
            }
        }
    }

    fn validate_record_actions(
        &self,
        record: &LifecycleActionRecord,
        violations: &mut Vec<M5LifecycleActionsViolation>,
    ) {
        for action in &record.offered_actions {
            if action.action_ref.trim().is_empty() {
                violations.push(M5LifecycleActionsViolation::EmptyField {
                    id: record.record_id.clone(),
                    field_name: "offered_actions.action_ref",
                });
            }
            // Every offered action must carry the record's scope, so a workspace
            // troubleshooting action can never silently apply at global scope.
            if action.scope != record.scope {
                violations.push(M5LifecycleActionsViolation::OfferedActionScopeMismatch {
                    record_id: record.record_id.clone(),
                    action: action.action_kind.as_str(),
                });
            }
        }

        // The record must offer its own primary action.
        if !record.offers_action(record.action_kind) {
            violations.push(M5LifecycleActionsViolation::MissingPrimaryAction {
                record_id: record.record_id.clone(),
                action: record.action_kind.as_str(),
            });
        }

        // A blocked action must not expose an enabled primary action.
        if record.action_disposition == ActionDisposition::Blocked {
            if let Some(primary) = record.primary_action() {
                if primary.enabled {
                    violations.push(M5LifecycleActionsViolation::BlockedActionEnabled {
                        record_id: record.record_id.clone(),
                    });
                }
            }
        }
    }

    fn validate_record_gate(
        &self,
        record: &LifecycleActionRecord,
        violations: &mut Vec<M5LifecycleActionsViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for reason in &record.review_reasons {
            if !seen.insert(*reason) {
                violations.push(M5LifecycleActionsViolation::DuplicateReviewReason {
                    record_id: record.record_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The recorded reasons must equal the recomputed set, so a reactive trigger,
        // protected-data loss, or irreversible rollback can never be asserted or
        // hidden by hand.
        if record.review_reasons != record.computed_review_reasons() {
            violations.push(M5LifecycleActionsViolation::ReviewReasonsMismatch {
                record_id: record.record_id.clone(),
            });
        }

        // The published disposition must equal the recomputed gate.
        let computed = record.computed_action_disposition();
        if record.action_disposition != computed {
            violations.push(M5LifecycleActionsViolation::ActionDispositionMismatch {
                record_id: record.record_id.clone(),
                stored: record.action_disposition.as_str(),
                computed: computed.as_str(),
            });
        }
    }
}

/// A validation violation for the M5 lifecycle-actions packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5LifecycleActionsViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Record or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A record id appears more than once.
    DuplicateRecordId {
        /// Duplicate record id.
        record_id: String,
    },
    /// An action kind disagrees with its bound scope.
    ActionScopeKindMismatch {
        /// Record id.
        record_id: String,
        /// Action token.
        action: &'static str,
    },
    /// A resulting status disagrees with its action kind.
    StatusMismatch {
        /// Record id.
        record_id: String,
        /// Action token.
        action: &'static str,
        /// Status token.
        status: &'static str,
    },
    /// A registry-status action pairs its status with the wrong trigger.
    RegistryStatusPairingMismatch {
        /// Record id.
        record_id: String,
        /// Status token.
        status: &'static str,
        /// Trigger token.
        trigger: &'static str,
    },
    /// A quarantine record carries a registry/publisher trigger.
    QuarantineTriggerMismatch {
        /// Record id.
        record_id: String,
        /// Trigger token.
        trigger: &'static str,
    },
    /// A revoked or yanked record claims to be restorable.
    RevokedRestorableInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A rollback record is missing a durable target.
    RollbackTargetMissing {
        /// Record id.
        record_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An offered action's scope disagrees with its record's scope.
    OfferedActionScopeMismatch {
        /// Record id.
        record_id: String,
        /// Action token.
        action: &'static str,
    },
    /// A record does not offer its own primary action.
    MissingPrimaryAction {
        /// Record id.
        record_id: String,
        /// Action token.
        action: &'static str,
    },
    /// A blocked record exposes an enabled primary action.
    BlockedActionEnabled {
        /// Record id.
        record_id: String,
    },
    /// A record lists a review reason more than once.
    DuplicateReviewReason {
        /// Record id.
        record_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A record's review reasons disagree with the recomputed set.
    ReviewReasonsMismatch {
        /// Record id.
        record_id: String,
    },
    /// A record's action disposition disagrees with the recomputed gate.
    ActionDispositionMismatch {
        /// Record id.
        record_id: String,
        /// Stored disposition token.
        stored: &'static str,
        /// Recomputed disposition token.
        computed: &'static str,
    },
    /// The summary counts disagree with the records.
    SummaryMismatch,
}

impl fmt::Display for M5LifecycleActionsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRecordId { record_id } => {
                write!(f, "duplicate lifecycle record id {record_id}")
            }
            Self::ActionScopeKindMismatch { record_id, action } => {
                write!(
                    f,
                    "record {record_id} action {action} disagrees with its bound scope"
                )
            }
            Self::StatusMismatch {
                record_id,
                action,
                status,
            } => {
                write!(
                    f,
                    "record {record_id} action {action} resolves to status {status} but the action fixes a different status"
                )
            }
            Self::RegistryStatusPairingMismatch {
                record_id,
                status,
                trigger,
            } => {
                write!(
                    f,
                    "record {record_id} registry status {status} does not pair with trigger {trigger}"
                )
            }
            Self::QuarantineTriggerMismatch { record_id, trigger } => {
                write!(
                    f,
                    "record {record_id} quarantine carries registry/publisher trigger {trigger}"
                )
            }
            Self::RevokedRestorableInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} is revoked or yanked but claims to be restorable"
                )
            }
            Self::RollbackTargetMissing {
                record_id,
                field_name,
            } => {
                write!(f, "record {record_id} rollback is missing {field_name}")
            }
            Self::OfferedActionScopeMismatch { record_id, action } => {
                write!(
                    f,
                    "record {record_id} offered action {action} is scoped outside the record's scope"
                )
            }
            Self::MissingPrimaryAction { record_id, action } => {
                write!(
                    f,
                    "record {record_id} does not offer its primary action {action}"
                )
            }
            Self::BlockedActionEnabled { record_id } => {
                write!(
                    f,
                    "record {record_id} blocks the action but exposes an enabled primary action"
                )
            }
            Self::DuplicateReviewReason { record_id, reason } => {
                write!(f, "record {record_id} repeats review reason {reason}")
            }
            Self::ReviewReasonsMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} review reasons disagree with the recomputed set"
                )
            }
            Self::ActionDispositionMismatch {
                record_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "record {record_id} publishes disposition {stored} but the recomputed gate is {computed}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the records")
            }
        }
    }
}

impl Error for M5LifecycleActionsViolation {}

/// Loads the embedded M5 lifecycle-actions packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5LifecycleActions`].
pub fn current_m5_lifecycle_actions() -> Result<M5LifecycleActions, serde_json::Error> {
    serde_json::from_str(M5_LIFECYCLE_ACTIONS_JSON)
}

#[cfg(test)]
mod tests;
