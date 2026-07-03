//! Reviewed install / update / remove / regenerate mutation flows with
//! script-and-native-build labels, resolver identity, lockfile diff classes,
//! and durable rollback checkpoints.
//!
//! Where [`crate::package_mutation_and_registry_review`] reviews one operation
//! against the stable dependency-manager contract and
//! [`crate::grouped_update_and_rollback_review`] reviews a *grouped* update
//! plan, this module owns the canonical, preview-first **mutation review sheet**
//! the four package-mutation flows share. One [`MutationReviewSheet`] is the
//! single object the desktop review surface, the CLI/headless dry run, AI and
//! recipe proposals, and support/export packets all render, so a package
//! mutation is a checkpointed, exportable workflow rather than a side effect of
//! pressing update.
//!
//! Each sheet makes the things a package mutation must never hide explicit
//! before commit:
//!
//! - the **manifest scope** it targets and whether a whole-workspace change was
//!   confirmed;
//! - a **script / native-build label** that keeps *no scripts*, *known install
//!   scripts*, *native build required*, *unknown hook risk*, and *policy
//!   blocked* states distinct, so an operation can never masquerade as a
//!   harmless text edit;
//! - the **resolver identity and version** that produced the resolved set, which
//!   regenerate/resolve flows must always disclose;
//! - a **lockfile diff class** and quantified blast radius, with whether the
//!   lockfile stays authoritative for exact restore; and
//! - a durable **rollback checkpoint** that preserves the affected manifests,
//!   the lockfile identity before and after, the resulting state, and the
//!   revert / open-diff / export-patch recovery actions.
//!
//! A failed or partial mutation therefore leaves a durable [`RollbackReceipt`]
//! — a receipt, never a transient toast — and the commit gate refuses to record
//! a committed disposition while any block reason (policy-blocked or unknown
//! script risk, unsatisfied auth, a divergent lockfile, an unconfirmed
//! whole-workspace scope, or a missing/non-durable checkpoint) still holds.
//!
//! The packet reuses the frozen
//! [`crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix`]
//! vocabulary — manifest scope, registry source, auth mode, lockfile authority,
//! resolver identity, and rollback class — and binds every label it surfaces
//! back to a frozen state row through `references_matrix_id`.
//!
//! The checked-in packet lives at
//! `artifacts/deps/m5/reviewed-mutation-flows.json` and is embedded here so Rust
//! consumers, CLI/headless output, support exports, and release evidence all
//! validate against one source of truth. The model is metadata-only: it carries
//! no credential bodies, registry tokens, raw provider payloads, or private
//! registry URLs.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::{
    current_m5_package_state_matrix, AuthMode, LockfileAuthority, ManifestScopeClass,
    PackageStateLabel, PackageSurface, RegistrySourceAuthority, ResolverIdentityClass, RollbackClass,
    SurfaceWriteAuthority,
};
use crate::package_state_descriptors::{DependencyRelation, EcosystemKind, RequestedSourceKind};

/// Supported reviewed-mutation-flows packet schema version.
pub const REVIEWED_MUTATION_FLOWS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const REVIEWED_MUTATION_FLOWS_RECORD_KIND: &str = "reviewed_mutation_flows";

/// Repo-relative path to the checked-in packet.
pub const REVIEWED_MUTATION_FLOWS_PATH: &str = "artifacts/deps/m5/reviewed-mutation-flows.json";

/// Embedded checked-in packet JSON.
pub const REVIEWED_MUTATION_FLOWS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/reviewed-mutation-flows.json"
));

/// The package-mutation flow a review sheet represents.
///
/// These four flows are deliberately distinct so an install, an update, a
/// remove, and a regenerate/resolve never collapse into one generic "change".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationFlowClass {
    /// Add a new direct package requirement.
    Install,
    /// Update one or more package requirements or resolved versions.
    Update,
    /// Remove a package requirement.
    Remove,
    /// Regenerate or re-resolve lockfile state without a requested range change.
    Regenerate,
}

impl MutationFlowClass {
    /// Every mutation flow, in declaration order.
    pub const ALL: [Self; 4] = [Self::Install, Self::Update, Self::Remove, Self::Regenerate];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::Remove => "remove",
            Self::Regenerate => "regenerate",
        }
    }

    /// Whether this flow re-resolves the dependency set, so it must disclose the
    /// resolver version and any broad lockfile churn before write.
    pub const fn re_resolves(self) -> bool {
        matches!(self, Self::Install | Self::Update | Self::Regenerate)
    }

    /// The manifest-diff action class shown on reusable cards.
    pub const fn manifest_diff_action(self) -> ManifestDiffActionClass {
        match self {
            Self::Install => ManifestDiffActionClass::Add,
            Self::Update => ManifestDiffActionClass::Update,
            Self::Remove => ManifestDiffActionClass::Remove,
            Self::Regenerate => ManifestDiffActionClass::Resolve,
        }
    }
}

/// Add/update/remove class shown on a manifest-diff card.
///
/// `Resolve` is kept explicit for lockfile-only regenerate flows, so they do not
/// masquerade as dependency add/update/remove changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiffActionClass {
    /// Adds a manifest requirement.
    Add,
    /// Updates a manifest requirement or resolved set.
    Update,
    /// Removes a manifest requirement.
    Remove,
    /// Re-resolves or regenerates lockfile state.
    Resolve,
}

impl ManifestDiffActionClass {
    /// Stable token recorded in card projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Update => "update",
            Self::Remove => "remove",
            Self::Resolve => "resolve",
        }
    }
}

/// Script and native-build risk label for a mutation flow.
///
/// The five labels keep *no scripts*, *known install scripts*, *native build
/// required*, *unknown hook risk*, and *policy blocked* states distinct so a
/// mutation can never masquerade as a harmless text edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptBuildLabel {
    /// No install scripts or native build are expected.
    NoScripts,
    /// Known package lifecycle/install scripts will run.
    KnownInstallScripts,
    /// A native build (compiler/toolchain) is required.
    NativeBuildRequired,
    /// Script or hook behavior cannot be determined.
    UnknownHookRisk,
    /// Policy blocks the script or native-build behavior.
    PolicyBlocked,
}

impl ScriptBuildLabel {
    /// Every script/native-build label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoScripts,
        Self::KnownInstallScripts,
        Self::NativeBuildRequired,
        Self::UnknownHookRisk,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoScripts => "no_scripts",
            Self::KnownInstallScripts => "known_install_scripts",
            Self::NativeBuildRequired => "native_build_required",
            Self::UnknownHookRisk => "unknown_hook_risk",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Whether this label carries a script or native-build risk that review must
    /// surface with a disclosure note before commit.
    pub const fn is_risky(self) -> bool {
        !matches!(self, Self::NoScripts)
    }

    /// Whether this label blocks commit until resolved: an unknown hook risk or a
    /// policy block can never be applied.
    pub const fn blocks_commit(self) -> bool {
        matches!(self, Self::UnknownHookRisk | Self::PolicyBlocked)
    }

    /// Whether the operator must explicitly acknowledge the risk before commit.
    pub const fn requires_explicit_ack(self) -> bool {
        matches!(
            self,
            Self::KnownInstallScripts | Self::NativeBuildRequired | Self::UnknownHookRisk
        )
    }

    /// Whether policy may permit the behavior. A policy-blocked or unknown-risk
    /// label can never report that policy allows it.
    pub const fn may_be_policy_allowed(self) -> bool {
        !matches!(self, Self::PolicyBlocked | Self::UnknownHookRisk)
    }
}

/// The class of lockfile change a mutation flow would produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileDiffClass {
    /// The lockfile is not touched.
    NoLockfileChange,
    /// New lockfile entries are added.
    EntriesAdded,
    /// Existing lockfile entries are removed.
    EntriesRemoved,
    /// Existing lockfile entries are re-pinned to new exact versions.
    EntriesRepinned,
    /// A lockfile is created where none existed.
    LockfileCreated,
    /// The lockfile is regenerated wholesale.
    FullRegeneration,
}

impl LockfileDiffClass {
    /// Every lockfile diff class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoLockfileChange,
        Self::EntriesAdded,
        Self::EntriesRemoved,
        Self::EntriesRepinned,
        Self::LockfileCreated,
        Self::FullRegeneration,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLockfileChange => "no_lockfile_change",
            Self::EntriesAdded => "entries_added",
            Self::EntriesRemoved => "entries_removed",
            Self::EntriesRepinned => "entries_repinned",
            Self::LockfileCreated => "lockfile_created",
            Self::FullRegeneration => "full_regeneration",
        }
    }

    /// Whether the lockfile is changed at all.
    pub const fn changes_lockfile(self) -> bool {
        !matches!(self, Self::NoLockfileChange)
    }

    /// Whether this diff is a broad churn that a regenerate/resolve flow must
    /// disclose before write.
    pub const fn is_broad_churn(self) -> bool {
        matches!(self, Self::FullRegeneration)
    }
}

/// The surface or actor that proposed a mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalSource {
    /// A person acting in the desktop workspace.
    DesktopManual,
    /// A CLI/headless dry run.
    CliHeadlessDryRun,
    /// An AI proposal.
    AiProposal,
    /// A recipe or automation proposal.
    RecipeProposal,
}

impl ProposalSource {
    /// Every proposal source, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopManual,
        Self::CliHeadlessDryRun,
        Self::AiProposal,
        Self::RecipeProposal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopManual => "desktop_manual",
            Self::CliHeadlessDryRun => "cli_headless_dry_run",
            Self::AiProposal => "ai_proposal",
            Self::RecipeProposal => "recipe_proposal",
        }
    }

    /// Whether the proposal came from AI or a recipe, so it must pass through the
    /// same review and never commit unreviewed.
    pub const fn is_automated(self) -> bool {
        matches!(self, Self::AiProposal | Self::RecipeProposal)
    }
}

/// The review disposition of a mutation sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDisposition {
    /// The sheet is awaiting review.
    PendingReview,
    /// The sheet has been reviewed and is ready to commit.
    ReviewedReady,
    /// The sheet is blocked until a risk, auth, or lockfile issue is resolved.
    BlockedUntilResolved,
    /// The mutation was committed after review.
    CommittedAfterReview,
    /// The mutation was committed and then rolled back.
    RolledBack,
}

impl ReviewDisposition {
    /// Every review disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PendingReview,
        Self::ReviewedReady,
        Self::BlockedUntilResolved,
        Self::CommittedAfterReview,
        Self::RolledBack,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::ReviewedReady => "reviewed_ready",
            Self::BlockedUntilResolved => "blocked_until_resolved",
            Self::CommittedAfterReview => "committed_after_review",
            Self::RolledBack => "rolled_back",
        }
    }
}

/// The state of a rollback checkpoint receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointState {
    /// A pre-commit checkpoint has been captured.
    Captured,
    /// The mutation committed and the checkpoint remains reversible.
    AppliedReversible,
    /// A failed or partial mutation left recovery pending against the receipt.
    PartialRecoveryPending,
    /// The mutation was reverted from this checkpoint.
    Reverted,
    /// The checkpoint was superseded by a newer sheet.
    Superseded,
}

impl CheckpointState {
    /// Every checkpoint state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Captured,
        Self::AppliedReversible,
        Self::PartialRecoveryPending,
        Self::Reverted,
        Self::Superseded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::AppliedReversible => "applied_reversible",
            Self::PartialRecoveryPending => "partial_recovery_pending",
            Self::Reverted => "reverted",
            Self::Superseded => "superseded",
        }
    }

    /// Whether this state represents a failed or partial mutation that still
    /// needs an operator-driven recovery.
    pub const fn is_recovery_pending(self) -> bool {
        matches!(self, Self::PartialRecoveryPending)
    }

    /// Whether this state is a committed (post-write) checkpoint state.
    pub const fn is_committed(self) -> bool {
        matches!(self, Self::AppliedReversible | Self::PartialRecoveryPending)
    }
}

/// A recovery action a checkpoint receipt offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionKind {
    /// Revert the mutation back to the checkpoint.
    Revert,
    /// Open the diff the mutation produced.
    OpenDiff,
    /// Export the mutation as a patch.
    ExportPatch,
}

impl RecoveryActionKind {
    /// Every recovery action kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::Revert, Self::OpenDiff, Self::ExportPatch];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Revert => "revert",
            Self::OpenDiff => "open_diff",
            Self::ExportPatch => "export_patch",
        }
    }
}

/// Stable surface contract: the surfaces that share this object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewedMutationSurfaceContract {
    /// Desktop review-sheet surface.
    pub review_sheet_surface: String,
    /// CLI/headless dry-run surface.
    pub cli_dry_run_surface: String,
    /// AI/recipe proposal surface.
    pub ai_recipe_surface: String,
    /// Rollback / checkpoint receipt surface.
    pub rollback_surface: String,
    /// Help page describing the packet.
    pub help_page: String,
    /// Support-export channel.
    pub support_export_surface: String,
}

/// Which surfaces a sheet's review is mirrored to.
///
/// Lockfile-safe review must behave consistently across desktop, the
/// CLI/headless dry run, and support/export artifacts; every flag must stay
/// `true`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceParity {
    /// Rendered on the desktop review surface.
    pub desktop: bool,
    /// Reproduced by the CLI/headless dry run.
    pub cli_headless_dry_run: bool,
    /// Reproduced in support/export artifacts.
    pub support_export: bool,
}

impl SurfaceParity {
    /// Whether the sheet is mirrored to every claimed surface.
    pub const fn is_consistent(&self) -> bool {
        self.desktop && self.cli_headless_dry_run && self.support_export
    }
}

/// The manifest scope a mutation targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestScopeTarget {
    /// Frozen manifest-scope class.
    pub scope_class: ManifestScopeClass,
    /// Ecosystem for this scope.
    pub ecosystem: EcosystemKind,
    /// Scope label shown to users.
    pub scope_label: String,
    /// Redacted manifest path; never a raw URL.
    pub redacted_manifest_path: String,
    /// Durable ids of every manifest the mutation would touch.
    pub affected_manifest_ids: Vec<String>,
    /// Whether a whole-workspace scope was explicitly confirmed.
    #[serde(default)]
    pub confirmed_explicitly: bool,
}

impl ManifestScopeTarget {
    /// Whether this scope must be confirmed explicitly before a bulk mutation.
    pub const fn requires_confirmation(&self) -> bool {
        self.scope_class.requires_explicit_confirmation()
    }

    /// Whether the confirmation requirement is satisfied.
    pub const fn confirmation_satisfied(&self) -> bool {
        !self.requires_confirmation() || self.confirmed_explicitly
    }
}

/// The requested package identity, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestedMutationIdentity {
    /// Ecosystem the request targets.
    pub ecosystem: EcosystemKind,
    /// Package name or coordinate requested.
    pub package_name: String,
    /// Requested range, tag, source ref, or removal target; redacted.
    pub requested_ref: String,
    /// Requested source kind.
    pub requested_source: RequestedSourceKind,
    /// Whether the request is a policy-pinned constraint.
    #[serde(default)]
    pub policy_pinned: bool,
}

/// The resolved package identity, after resolution.
///
/// Resolution may be absent — a removal produces no resolved identity, and an
/// auth-blocked or policy-blocked flow stays unresolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedMutationIdentity {
    /// Dependency relation of the resolved package.
    pub relation: DependencyRelation,
    /// Exact resolved version, commit, path, or snapshot id; redacted.
    pub resolved_ref: String,
    /// Registry or mirror source the resolution came from.
    pub registry_source: RegistrySourceAuthority,
}

/// The resolver identity and version that produced the resolved set.
///
/// Kept separate from [`ResolvedMutationIdentity`] so resolver identity is a
/// first-class disclosure: regenerate/resolve flows must always name the
/// resolver and its version before write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolverIdentity {
    /// Frozen resolver-identity class.
    pub resolver_class: ResolverIdentityClass,
    /// Resolver version string disclosed to review.
    pub resolver_version: String,
}

/// Where a flow resolves a package from, and whether that source can be trusted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistrySourceCue {
    /// Registry or mirror source authority.
    pub source_class: RegistrySourceAuthority,
    /// Auth mode used to reach the source.
    pub auth_mode: AuthMode,
    /// Redacted source label safe for support exports; never a raw URL or token.
    pub redacted_source_label: String,
}

impl RegistrySourceCue {
    /// Whether trust is blocked because auth is required but unsatisfied.
    pub const fn trust_blocked(&self) -> bool {
        self.auth_mode.blocks_until_satisfied()
    }
}

/// The script / native-build risk review for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptBuildReview {
    /// Script / native-build label.
    pub label: ScriptBuildLabel,
    /// Package refs that introduce the script or native-build behavior.
    #[serde(default)]
    pub source_package_refs: Vec<String>,
    /// Toolchains or runtimes a native build requires.
    #[serde(default)]
    pub required_toolchain_refs: Vec<String>,
    /// Whether policy allows the behavior.
    pub policy_allows: bool,
    /// Whether the operator must explicitly acknowledge the risk; must equal
    /// [`ScriptBuildLabel::requires_explicit_ack`].
    pub requires_explicit_ack: bool,
    /// Reviewer-facing disclosure note. Must be non-empty for any risky label.
    pub disclosure_note: String,
}

impl ScriptBuildReview {
    /// Whether the stored ack flag and policy posture agree with the label.
    pub fn is_consistent(&self) -> bool {
        self.requires_explicit_ack == self.label.requires_explicit_ack()
            && (self.label.may_be_policy_allowed() || !self.policy_allows)
            && (!self.label.is_risky() || !self.disclosure_note.trim().is_empty())
    }
}

/// The quantified lockfile blast radius for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockfileBlastRadius {
    /// Lockfile diff class.
    pub diff_class: LockfileDiffClass,
    /// Lockfile authority governing the resolved set.
    pub lockfile_authority: LockfileAuthority,
    /// Durable ids of every lockfile the mutation would touch.
    #[serde(default)]
    pub affected_lockfile_ids: Vec<String>,
    /// Lockfile entries added.
    pub entries_added: u32,
    /// Lockfile entries removed.
    pub entries_removed: u32,
    /// Lockfile entries re-pinned in place.
    pub entries_repinned: u32,
    /// Whether the lockfile stays authoritative for exact restore; must equal
    /// whether [`LockfileBlastRadius::lockfile_authority`] supports exact restore.
    pub exact_restore_supported: bool,
    /// Whether a broad churn was disclosed; must be `true` for a broad-churn diff.
    #[serde(default)]
    pub broad_churn_disclosed: bool,
    /// Reviewer-facing blast-radius note.
    pub note: String,
}

impl LockfileBlastRadius {
    /// Total touched lockfile entries.
    pub const fn churn_total(&self) -> u32 {
        self.entries_added + self.entries_removed + self.entries_repinned
    }

    /// Whether the stored authority supports authoritative exact restore.
    pub const fn authority_supports_exact_restore(&self) -> bool {
        matches!(
            self.lockfile_authority,
            LockfileAuthority::ExactLockfilePinned | LockfileAuthority::FrozenByPolicy
        )
    }

    /// Whether the blast radius is internally consistent.
    pub fn is_consistent(&self) -> bool {
        let churn_ok = if self.diff_class.changes_lockfile() {
            self.churn_total() > 0 && !self.affected_lockfile_ids.is_empty()
        } else {
            self.churn_total() == 0
        };
        let exact_ok = self.exact_restore_supported == self.authority_supports_exact_restore();
        let churn_disclosed = !self.diff_class.is_broad_churn() || self.broad_churn_disclosed;
        churn_ok && exact_ok && churn_disclosed
    }
}

/// A recovery action a checkpoint receipt offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryAction {
    /// Recovery action kind.
    pub kind: RecoveryActionKind,
    /// Redaction-safe target reference.
    pub target_ref: String,
}

/// A durable rollback checkpoint receipt for a mutation sheet.
///
/// A failed or partial mutation leaves one of these — a durable receipt, never a
/// transient toast — preserving the affected manifests, the lockfile identity
/// before and after, the resulting state, and the revert / open-diff /
/// export-patch recovery actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackReceipt {
    /// Stable checkpoint id.
    pub checkpoint_id: String,
    /// Sheet this checkpoint guards.
    pub sheet_id: String,
    /// Rollback class.
    pub rollback_class: RollbackClass,
    /// Checkpoint state.
    pub state: CheckpointState,
    /// Whether the receipt is durable (must always be `true`).
    pub durable: bool,
    /// Manifests covered by the checkpoint.
    pub affected_manifest_ids: Vec<String>,
    /// Redaction-safe lockfile identity before the change.
    pub lockfile_identity_before: String,
    /// Redaction-safe lockfile identity after the change.
    pub lockfile_identity_after: String,
    /// Resulting package state the receipt records.
    pub resulting_state: String,
    /// Human-readable receipt label.
    pub receipt_label: String,
    /// Revert / open-diff / export-patch recovery actions.
    pub recovery_actions: Vec<RecoveryAction>,
}

impl RollbackReceipt {
    /// Whether the receipt offers revert, open-diff, and export-patch recovery.
    pub fn offers_all_recovery_actions(&self) -> bool {
        let kinds: BTreeSet<RecoveryActionKind> =
            self.recovery_actions.iter().map(|a| a.kind).collect();
        RecoveryActionKind::ALL.iter().all(|k| kinds.contains(k))
    }

    /// Whether the rollback class is a real recovery path rather than no path.
    pub const fn is_recoverable(&self) -> bool {
        matches!(
            self.rollback_class,
            RollbackClass::ReversibleCheckpointed
                | RollbackClass::ReversibleManifestOnly
                | RollbackClass::CompensatingOnly
        )
    }

    /// Whether the receipt is a durable, complete, recoverable checkpoint.
    pub fn is_durable_recovery(&self) -> bool {
        self.durable && self.offers_all_recovery_actions() && self.is_recoverable()
    }
}

/// One mutation review sheet — the object the four flows share.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationReviewSheet {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Mutation flow.
    pub flow_class: MutationFlowClass,
    /// Source that proposed the mutation.
    pub proposal_source: ProposalSource,
    /// Review disposition.
    pub review_disposition: ReviewDisposition,
    /// Human-readable sheet label.
    pub sheet_label: String,
    /// Manifest scope targeted.
    pub manifest_scope: ManifestScopeTarget,
    /// Requested identity.
    pub requested: RequestedMutationIdentity,
    /// Resolved identity, when the flow resolved one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedMutationIdentity>,
    /// Resolver identity and version.
    pub resolver: ResolverIdentity,
    /// Registry/auth source cue.
    pub registry_source: RegistrySourceCue,
    /// Script / native-build review.
    pub script_build: ScriptBuildReview,
    /// Lockfile blast radius.
    pub lockfile: LockfileBlastRadius,
    /// Checkpoint receipt that guards this sheet.
    pub rollback_checkpoint_id: String,
    /// Surface parity across desktop, CLI dry run, and support/export.
    pub surface_parity: SurfaceParity,
    /// Frozen package-state labels this sheet surfaces; each binds to the matrix.
    #[serde(default)]
    pub applicable_labels: Vec<PackageStateLabel>,
    /// Reviewer-facing note.
    pub note: String,
}

impl MutationReviewSheet {
    /// Whether any intrinsic block reason holds, independent of the checkpoint.
    ///
    /// A mutation is blocked when its disposition is blocked, its script/build
    /// label blocks commit, registry trust is unsatisfied, the lockfile is
    /// divergent, or a whole-workspace scope was not confirmed.
    pub fn intrinsic_commit_blocked(&self) -> bool {
        self.review_disposition == ReviewDisposition::BlockedUntilResolved
            || self.script_build.label.blocks_commit()
            || self.registry_source.trust_blocked()
            || self.lockfile.lockfile_authority.blocks_until_reconciled()
            || !self.manifest_scope.confirmation_satisfied()
    }

    /// Whether the sheet discloses every field review must see before commit:
    /// manifest scope, script/native-build risk, resolver identity, lockfile
    /// blast radius, and a rollback checkpoint.
    pub fn discloses_all_required(&self) -> bool {
        !self.manifest_scope.scope_label.trim().is_empty()
            && !self.manifest_scope.affected_manifest_ids.is_empty()
            && !self.resolver.resolver_version.trim().is_empty()
            && (!self.script_build.label.is_risky()
                || !self.script_build.disclosure_note.trim().is_empty())
            && !self.lockfile.note.trim().is_empty()
            && !self.rollback_checkpoint_id.trim().is_empty()
    }

    /// The frozen labels this sheet surfaces.
    pub fn labels(&self) -> &[PackageStateLabel] {
        &self.applicable_labels
    }
}

fn constraint_note_for_sheet(sheet: &MutationReviewSheet) -> String {
    let mut notes = Vec::new();
    if !sheet.script_build.required_toolchain_refs.is_empty() {
        notes.push(format!(
            "Runtime/toolchain constraint: {}",
            sheet.script_build.required_toolchain_refs.join(", ")
        ));
    }
    if matches!(sheet.manifest_scope.ecosystem, EcosystemKind::NodePnpm)
        && sheet.flow_class == MutationFlowClass::Update
    {
        notes.push("Peer dependency compatibility reviewed for the updated package.".to_owned());
    }
    if notes.is_empty() {
        "No peer/runtime constraint changes claimed by this preview.".to_owned()
    } else {
        notes.join(" ")
    }
}

fn constraint_changes_for_sheet(sheet: &MutationReviewSheet) -> Vec<ConstraintChangeNote> {
    let mut changes = Vec::new();
    for toolchain in &sheet.script_build.required_toolchain_refs {
        changes.push(ConstraintChangeNote {
            constraint_kind: "toolchain".to_owned(),
            subject_ref: sheet.requested.package_name.clone(),
            from_ref: "not_required".to_owned(),
            to_ref: toolchain.clone(),
            compatibility_posture: "review_required".to_owned(),
        });
    }
    if matches!(sheet.manifest_scope.ecosystem, EcosystemKind::NodePnpm)
        && sheet.flow_class == MutationFlowClass::Update
    {
        changes.push(ConstraintChangeNote {
            constraint_kind: "peer".to_owned(),
            subject_ref: sheet.requested.package_name.clone(),
            from_ref: "previous_peer_set".to_owned(),
            to_ref: "resolved_peer_set".to_owned(),
            compatibility_posture: "review_required".to_owned(),
        });
    }
    changes
}

/// The checkpoint state a disposition expects.
const fn disposition_allows_checkpoint(
    disposition: ReviewDisposition,
    state: CheckpointState,
) -> bool {
    match disposition {
        ReviewDisposition::CommittedAfterReview => state.is_committed(),
        ReviewDisposition::RolledBack => matches!(state, CheckpointState::Reverted),
        ReviewDisposition::PendingReview
        | ReviewDisposition::ReviewedReady
        | ReviewDisposition::BlockedUntilResolved => {
            matches!(
                state,
                CheckpointState::Captured | CheckpointState::Superseded
            )
        }
    }
}

/// A redaction-safe view of one mutation sheet, projected for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationReviewSheetSurfaceProjection {
    /// Sheet id.
    pub sheet_id: String,
    /// Surface token.
    pub surface: String,
    /// Whether this surface may commit an unblocked sheet.
    pub can_commit_here: bool,
    /// Whether the projection is redacted for export.
    pub redacted: bool,
    /// Whether the commit gate is blocked for this sheet.
    pub commit_blocked: bool,
}

/// Manifest-diff preview honesty state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiffPreviewState {
    /// A full manifest/lockfile preview is available.
    Available,
    /// The surface only has a narrowed preview and must say so.
    NarrowedFallback,
    /// No manifest/lockfile preview is available.
    NoPreview,
}

impl ManifestDiffPreviewState {
    /// Stable token recorded in card projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::NarrowedFallback => "narrowed_fallback",
            Self::NoPreview => "no_preview",
        }
    }
}

/// Checkpoint state shown before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiffCheckpointState {
    /// A durable checkpoint exists before apply.
    Available,
    /// No checkpoint exists and apply must remain blocked or narrowed.
    Missing,
    /// The surface narrowed because it cannot provide a durable checkpoint.
    NarrowedNoCheckpoint,
}

impl ManifestDiffCheckpointState {
    /// Stable token recorded in card projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Missing => "missing",
            Self::NarrowedNoCheckpoint => "narrowed_no_checkpoint",
        }
    }
}

/// Rollback state shown before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiffRollbackState {
    /// A durable rollback path exists.
    Available,
    /// Only a compensating rollback path exists.
    CompensatingOnly,
    /// No rollback path exists.
    Unavailable,
}

impl ManifestDiffRollbackState {
    /// Stable token recorded in card projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::CompensatingOnly => "compensating_only",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Apply action shown by a manifest-diff card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiffApplyAction {
    /// The current surface may apply the mutation.
    Apply,
    /// The current surface may stage the mutation for review, but not apply.
    StageForReview,
    /// The current surface may inspect/export only.
    InspectOnly,
    /// Apply is blocked by review, policy, preview, checkpoint, or validation state.
    Blocked,
}

impl ManifestDiffApplyAction {
    /// Stable token recorded in card projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Apply => "apply",
            Self::StageForReview => "stage_for_review",
            Self::InspectOnly => "inspect_only",
            Self::Blocked => "blocked",
        }
    }
}

/// Peer/runtime constraint disclosure shown on a manifest-diff card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConstraintChangeNote {
    /// Constraint kind, such as peer, runtime, engine, or toolchain.
    pub constraint_kind: String,
    /// Package or runtime the note speaks for.
    pub subject_ref: String,
    /// Redaction-safe before value.
    pub from_ref: String,
    /// Redaction-safe after value.
    pub to_ref: String,
    /// Compatibility posture shown to review.
    pub compatibility_posture: String,
}

/// Reusable manifest-diff card rendered before any package write.
///
/// The card is a projection, not a second packet. It is derived from the
/// canonical review sheet plus its checkpoint receipt, and automation surfaces
/// may attach the validation selection that governed the proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestDiffCard {
    /// Stable card id.
    pub card_id: String,
    /// Review sheet this card renders.
    pub sheet_ref: String,
    /// Source that proposed the mutation.
    pub proposal_source: ProposalSource,
    /// Add/update/remove/resolve class.
    pub action_class: ManifestDiffActionClass,
    /// Package or workspace target.
    pub package_ref: String,
    /// Manifests touched or claimed by the preview.
    pub affected_manifest_refs: Vec<String>,
    /// Lockfiles touched or claimed by the preview. Empty is paired with
    /// `lockfile_touch_note` for no-lockfile-change honesty.
    pub affected_lockfile_refs: Vec<String>,
    /// Explicit note for touched lockfiles, including no-change cases.
    pub lockfile_touch_note: String,
    /// Explicit scripts/hooks/native-build note.
    pub scripts_hooks_note: String,
    /// Explicit peer/runtime constraint note.
    pub peer_runtime_constraints_note: String,
    /// Constraint changes, if any.
    pub constraint_changes: Vec<ConstraintChangeNote>,
    /// Preview state.
    pub preview_state: ManifestDiffPreviewState,
    /// Checkpoint state before apply.
    pub checkpoint_state: ManifestDiffCheckpointState,
    /// Rollback state before apply.
    pub rollback_state: ManifestDiffRollbackState,
    /// Redaction-safe checkpoint ref, or a missing-state token.
    pub checkpoint_ref: String,
    /// Redaction-safe rollback ref, or a missing-state token.
    pub rollback_ref: String,
    /// Validation-task selection ref, when the card is rendered from automation.
    pub validation_selection_ref: Option<String>,
    /// Selected validation-task tokens, when the card is rendered from automation.
    pub selected_validation_tasks: Vec<String>,
    /// Apply action for this card on its current surface.
    pub apply_action: ManifestDiffApplyAction,
    /// Consumer surfaces proven to use this card grammar.
    pub consumer_surfaces: Vec<String>,
}

impl ManifestDiffCard {
    /// Whether the card names the files, risks, constraint posture, and
    /// checkpoint/rollback posture required before apply.
    pub fn discloses_apply_boundary(&self) -> bool {
        !self.affected_manifest_refs.is_empty()
            && !self.lockfile_touch_note.trim().is_empty()
            && !self.scripts_hooks_note.trim().is_empty()
            && !self.peer_runtime_constraints_note.trim().is_empty()
            && !self.checkpoint_ref.trim().is_empty()
            && !self.rollback_ref.trim().is_empty()
    }

    /// Whether the card honestly blocks or narrows when preview/checkpoint state
    /// is unavailable.
    pub const fn fallback_honest(&self) -> bool {
        match (self.preview_state, self.checkpoint_state, self.apply_action) {
            (ManifestDiffPreviewState::Available, ManifestDiffCheckpointState::Available, _) => {
                true
            }
            (_, _, ManifestDiffApplyAction::Blocked | ManifestDiffApplyAction::InspectOnly) => true,
            _ => false,
        }
    }
}

impl MutationReviewSheet {
    /// Builds the reusable manifest-diff card for this sheet.
    pub fn manifest_diff_card(
        &self,
        receipt: Option<&RollbackReceipt>,
        commit_blocked: bool,
    ) -> ManifestDiffCard {
        let (checkpoint_state, rollback_state, checkpoint_ref, rollback_ref) = match receipt {
            Some(receipt) if receipt.is_durable_recovery() => {
                let rollback_state = if receipt.rollback_class == RollbackClass::CompensatingOnly {
                    ManifestDiffRollbackState::CompensatingOnly
                } else {
                    ManifestDiffRollbackState::Available
                };
                (
                    ManifestDiffCheckpointState::Available,
                    rollback_state,
                    receipt.checkpoint_id.clone(),
                    receipt.checkpoint_id.clone(),
                )
            }
            Some(receipt) => (
                ManifestDiffCheckpointState::Missing,
                ManifestDiffRollbackState::Unavailable,
                receipt.checkpoint_id.clone(),
                "rollback:unavailable".to_owned(),
            ),
            None => (
                ManifestDiffCheckpointState::Missing,
                ManifestDiffRollbackState::Unavailable,
                "checkpoint:missing".to_owned(),
                "rollback:missing".to_owned(),
            ),
        };

        let apply_action =
            if commit_blocked || checkpoint_state != ManifestDiffCheckpointState::Available {
                ManifestDiffApplyAction::Blocked
            } else {
                ManifestDiffApplyAction::Apply
            };

        let lockfile_touch_note = if self.lockfile.affected_lockfile_ids.is_empty() {
            format!("No lockfile touched: {}", self.lockfile.note)
        } else {
            self.lockfile.note.clone()
        };

        ManifestDiffCard {
            card_id: format!("mdc:{}", self.sheet_id),
            sheet_ref: self.sheet_id.clone(),
            proposal_source: self.proposal_source,
            action_class: self.flow_class.manifest_diff_action(),
            package_ref: self.requested.package_name.clone(),
            affected_manifest_refs: self.manifest_scope.affected_manifest_ids.clone(),
            affected_lockfile_refs: self.lockfile.affected_lockfile_ids.clone(),
            lockfile_touch_note,
            scripts_hooks_note: self.script_build.disclosure_note.clone(),
            peer_runtime_constraints_note: constraint_note_for_sheet(self),
            constraint_changes: constraint_changes_for_sheet(self),
            preview_state: ManifestDiffPreviewState::Available,
            checkpoint_state,
            rollback_state,
            checkpoint_ref,
            rollback_ref,
            validation_selection_ref: None,
            selected_validation_tasks: Vec::new(),
            apply_action,
            consumer_surfaces: vec![
                "package_manager".to_owned(),
                "review_pane".to_owned(),
                "ai_recipe_cli".to_owned(),
                "support_export".to_owned(),
                "release_proof".to_owned(),
            ],
        }
    }
}

/// Summary counts derived from the rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewedMutationFlowsSummary {
    /// Total mutation sheets.
    pub total_sheets: usize,
    /// Install sheets.
    pub install_sheets: usize,
    /// Update sheets.
    pub update_sheets: usize,
    /// Remove sheets.
    pub remove_sheets: usize,
    /// Regenerate sheets.
    pub regenerate_sheets: usize,
    /// Sheets carrying a script or native-build risk.
    pub script_risk_sheets: usize,
    /// Sheets requiring a native build.
    pub native_build_sheets: usize,
    /// Sheets blocked by policy.
    pub policy_blocked_sheets: usize,
    /// Sheets whose commit gate is blocked.
    pub commit_blocked_sheets: usize,
    /// Sheets proposed by AI or a recipe.
    pub automated_proposal_sheets: usize,
    /// Sheets disclosing a broad lockfile churn.
    pub broad_churn_sheets: usize,
    /// Total checkpoint receipts.
    pub total_checkpoints: usize,
    /// Durable checkpoint receipts.
    pub durable_checkpoints: usize,
    /// Checkpoints with recovery pending.
    pub recovery_pending_checkpoints: usize,
    /// Checkpoints that have been reverted.
    pub reverted_checkpoints: usize,
}

/// One row of the redaction-safe export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewedMutationFlowsExportRow {
    /// Row id (sheet id or checkpoint id).
    pub row_id: String,
    /// Row kind discriminator.
    pub row_kind: String,
    /// Ecosystem token, or a marker for checkpoint rows.
    pub ecosystem: String,
    /// Sheet or checkpoint label.
    pub label: String,
    /// Effective state token.
    pub effective_state: String,
    /// Whether the row blocks commit.
    pub blocks_commit: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// Redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewedMutationFlowsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<ReviewedMutationFlowsExportRow>,
    /// Whether any sheet blocks commit.
    pub blocks_any_commit: bool,
    /// Whether every sheet discloses all required disclosures.
    pub all_disclose_required: bool,
    /// Whether every sheet's labels bind to the frozen matrix.
    pub all_bind_matrix: bool,
}

/// Typed reviewed-mutation-flows packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewedMutationFlows {
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
    /// The frozen matrix this packet binds to.
    pub references_matrix_id: String,
    /// Stable surface contract.
    pub surface_contract: ReviewedMutationSurfaceContract,
    /// Closed mutation-flow vocabulary.
    pub flow_classes: Vec<MutationFlowClass>,
    /// Closed script/native-build label vocabulary.
    pub script_build_labels: Vec<ScriptBuildLabel>,
    /// Closed lockfile-diff-class vocabulary.
    pub lockfile_diff_classes: Vec<LockfileDiffClass>,
    /// Closed proposal-source vocabulary.
    pub proposal_sources: Vec<ProposalSource>,
    /// Closed review-disposition vocabulary.
    pub review_dispositions: Vec<ReviewDisposition>,
    /// Closed checkpoint-state vocabulary.
    pub checkpoint_states: Vec<CheckpointState>,
    /// Closed recovery-action vocabulary.
    pub recovery_action_kinds: Vec<RecoveryActionKind>,
    /// Mutation review sheets.
    #[serde(default)]
    pub sheets: Vec<MutationReviewSheet>,
    /// Rollback checkpoint receipts.
    #[serde(default)]
    pub checkpoints: Vec<RollbackReceipt>,
    /// Summary counts.
    pub summary: ReviewedMutationFlowsSummary,
}

impl ReviewedMutationFlows {
    /// Returns the sheet for `sheet_id`.
    pub fn sheet(&self, sheet_id: &str) -> Option<&MutationReviewSheet> {
        self.sheets.iter().find(|row| row.sheet_id == sheet_id)
    }

    /// Returns the checkpoint receipt for `checkpoint_id`.
    pub fn checkpoint(&self, checkpoint_id: &str) -> Option<&RollbackReceipt> {
        self.checkpoints
            .iter()
            .find(|row| row.checkpoint_id == checkpoint_id)
    }

    /// Builds the reusable manifest-diff card for `sheet_id`.
    pub fn manifest_diff_card(&self, sheet_id: &str) -> Option<ManifestDiffCard> {
        let sheet = self.sheet(sheet_id)?;
        Some(sheet.manifest_diff_card(
            self.checkpoint(&sheet.rollback_checkpoint_id),
            self.commit_blocked(sheet),
        ))
    }

    /// Builds manifest-diff cards for every reviewed mutation sheet.
    pub fn manifest_diff_cards(&self) -> Vec<ManifestDiffCard> {
        self.sheets
            .iter()
            .map(|sheet| {
                sheet.manifest_diff_card(
                    self.checkpoint(&sheet.rollback_checkpoint_id),
                    self.commit_blocked(sheet),
                )
            })
            .collect()
    }

    /// Whether the sheet's commit gate is blocked.
    ///
    /// A sheet is gated when an intrinsic block reason holds or its checkpoint is
    /// missing, non-durable, incomplete, or non-recoverable. A failed/partial
    /// mutation must not be silently committed without a durable recovery path.
    pub fn commit_blocked(&self, sheet: &MutationReviewSheet) -> bool {
        if sheet.intrinsic_commit_blocked() {
            return true;
        }
        match self.checkpoint(&sheet.rollback_checkpoint_id) {
            None => true,
            Some(receipt) => !receipt.is_durable_recovery(),
        }
    }

    /// Whether any sheet blocks commit.
    pub fn blocks_any_commit(&self) -> bool {
        self.sheets.iter().any(|s| self.commit_blocked(s))
    }

    /// Whether every sheet discloses all required disclosures.
    pub fn all_disclose_required(&self) -> bool {
        self.sheets.iter().all(|s| s.discloses_all_required())
    }

    /// Whether the packet binds to the matrix and every label resolves in it.
    pub fn all_bind_matrix(&self) -> bool {
        let Ok(matrix) = current_m5_package_state_matrix() else {
            return false;
        };
        if self.references_matrix_id != matrix.packet_id {
            return false;
        }
        self.sheets
            .iter()
            .flat_map(|s| s.applicable_labels.iter())
            .all(|label| matrix.state(*label).is_some())
    }

    /// Projects a sheet for a marketed surface with the write authority that
    /// surface may carry, pinned from the frozen matrix.
    pub fn surface_projection(
        &self,
        sheet_id: &str,
        surface: PackageSurface,
    ) -> Option<MutationReviewSheetSurfaceProjection> {
        let sheet = self.sheet(sheet_id)?;
        let authority = surface.canonical_write_authority();
        let blocked = self.commit_blocked(sheet);
        Some(MutationReviewSheetSurfaceProjection {
            sheet_id: sheet.sheet_id.clone(),
            surface: surface.as_str().to_owned(),
            can_commit_here: authority.can_mutate() && !blocked,
            redacted: matches!(authority, SurfaceWriteAuthority::RedactedExport),
            commit_blocked: blocked,
        })
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> ReviewedMutationFlowsSummary {
        let flow_count =
            |flow: MutationFlowClass| self.sheets.iter().filter(|s| s.flow_class == flow).count();
        ReviewedMutationFlowsSummary {
            total_sheets: self.sheets.len(),
            install_sheets: flow_count(MutationFlowClass::Install),
            update_sheets: flow_count(MutationFlowClass::Update),
            remove_sheets: flow_count(MutationFlowClass::Remove),
            regenerate_sheets: flow_count(MutationFlowClass::Regenerate),
            script_risk_sheets: self
                .sheets
                .iter()
                .filter(|s| s.script_build.label.is_risky())
                .count(),
            native_build_sheets: self
                .sheets
                .iter()
                .filter(|s| s.script_build.label == ScriptBuildLabel::NativeBuildRequired)
                .count(),
            policy_blocked_sheets: self
                .sheets
                .iter()
                .filter(|s| s.script_build.label == ScriptBuildLabel::PolicyBlocked)
                .count(),
            commit_blocked_sheets: self
                .sheets
                .iter()
                .filter(|s| self.commit_blocked(s))
                .count(),
            automated_proposal_sheets: self
                .sheets
                .iter()
                .filter(|s| s.proposal_source.is_automated())
                .count(),
            broad_churn_sheets: self
                .sheets
                .iter()
                .filter(|s| s.lockfile.diff_class.is_broad_churn())
                .count(),
            total_checkpoints: self.checkpoints.len(),
            durable_checkpoints: self.checkpoints.iter().filter(|c| c.durable).count(),
            recovery_pending_checkpoints: self
                .checkpoints
                .iter()
                .filter(|c| c.state.is_recovery_pending())
                .count(),
            reverted_checkpoints: self
                .checkpoints
                .iter()
                .filter(|c| c.state == CheckpointState::Reverted)
                .count(),
        }
    }

    /// Produces a redaction-safe export projection for UI, CLI, support, docs,
    /// release, and public proof consumers.
    pub fn export_projection(&self) -> ReviewedMutationFlowsExportProjection {
        let mut rows = Vec::new();
        for sheet in &self.sheets {
            let blocked = self.commit_blocked(sheet);
            rows.push(ReviewedMutationFlowsExportRow {
                row_id: sheet.sheet_id.clone(),
                row_kind: "review_sheet".to_owned(),
                ecosystem: sheet.manifest_scope.ecosystem.as_str().to_owned(),
                label: sheet.sheet_label.clone(),
                effective_state: sheet.review_disposition.as_str().to_owned(),
                blocks_commit: blocked,
                summary: format!(
                    "{} {} script {} lockfile {} (+{}/-{}/~{}) resolver {} {} rollback {}",
                    sheet.flow_class.as_str(),
                    sheet.requested.package_name,
                    sheet.script_build.label.as_str(),
                    sheet.lockfile.diff_class.as_str(),
                    sheet.lockfile.entries_added,
                    sheet.lockfile.entries_removed,
                    sheet.lockfile.entries_repinned,
                    sheet.resolver.resolver_class.as_str(),
                    sheet.resolver.resolver_version,
                    sheet.rollback_checkpoint_id,
                ),
            });
        }
        for checkpoint in &self.checkpoints {
            rows.push(ReviewedMutationFlowsExportRow {
                row_id: checkpoint.checkpoint_id.clone(),
                row_kind: "checkpoint".to_owned(),
                ecosystem: "workspace".to_owned(),
                label: checkpoint.receipt_label.clone(),
                effective_state: checkpoint.state.as_str().to_owned(),
                blocks_commit: false,
                summary: format!(
                    "{} rollback {} durable {} actions {} resulting {}",
                    checkpoint.sheet_id,
                    checkpoint.rollback_class.as_str(),
                    checkpoint.durable,
                    checkpoint.recovery_actions.len(),
                    checkpoint.resulting_state,
                ),
            });
        }
        ReviewedMutationFlowsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            blocks_any_commit: self.blocks_any_commit(),
            all_disclose_required: self.all_disclose_required(),
            all_bind_matrix: self.all_bind_matrix(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ReviewedMutationFlowsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_sheets(&mut violations);
        self.validate_checkpoints(&mut violations);
        if self.summary != self.computed_summary() {
            violations.push(ReviewedMutationFlowsViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ReviewedMutationFlowsViolation>) {
        if self.schema_version != REVIEWED_MUTATION_FLOWS_SCHEMA_VERSION {
            violations.push(ReviewedMutationFlowsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != REVIEWED_MUTATION_FLOWS_RECORD_KIND {
            violations.push(ReviewedMutationFlowsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("references_matrix_id", &self.references_matrix_id),
        ] {
            if value.trim().is_empty() {
                violations.push(ReviewedMutationFlowsViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, value) in [
            (
                "review_sheet_surface",
                &self.surface_contract.review_sheet_surface,
            ),
            (
                "cli_dry_run_surface",
                &self.surface_contract.cli_dry_run_surface,
            ),
            (
                "ai_recipe_surface",
                &self.surface_contract.ai_recipe_surface,
            ),
            ("rollback_surface", &self.surface_contract.rollback_surface),
            ("help_page", &self.surface_contract.help_page),
            (
                "support_export_surface",
                &self.surface_contract.support_export_surface,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(ReviewedMutationFlowsViolation::EmptyField {
                    id: "<surface_contract>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab_checks: [(&'static str, bool); 7] = [
            (
                "flow_classes",
                self.flow_classes == MutationFlowClass::ALL.to_vec(),
            ),
            (
                "script_build_labels",
                self.script_build_labels == ScriptBuildLabel::ALL.to_vec(),
            ),
            (
                "lockfile_diff_classes",
                self.lockfile_diff_classes == LockfileDiffClass::ALL.to_vec(),
            ),
            (
                "proposal_sources",
                self.proposal_sources == ProposalSource::ALL.to_vec(),
            ),
            (
                "review_dispositions",
                self.review_dispositions == ReviewDisposition::ALL.to_vec(),
            ),
            (
                "checkpoint_states",
                self.checkpoint_states == CheckpointState::ALL.to_vec(),
            ),
            (
                "recovery_action_kinds",
                self.recovery_action_kinds == RecoveryActionKind::ALL.to_vec(),
            ),
        ];
        for (field, ok) in vocab_checks {
            if !ok {
                violations.push(ReviewedMutationFlowsViolation::ClosedVocabularyMismatch { field });
            }
        }
        // The matrix this packet claims to bind to must be the frozen matrix.
        if let Ok(matrix) = current_m5_package_state_matrix() {
            if self.references_matrix_id != matrix.packet_id {
                violations.push(ReviewedMutationFlowsViolation::MatrixBindingMismatch {
                    referenced: self.references_matrix_id.clone(),
                });
            }
        }
    }

    fn validate_sheets(&self, violations: &mut Vec<ReviewedMutationFlowsViolation>) {
        let matrix = current_m5_package_state_matrix().ok();
        let mut seen = BTreeSet::new();
        for sheet in &self.sheets {
            if !seen.insert(sheet.sheet_id.clone()) {
                violations.push(ReviewedMutationFlowsViolation::DuplicateRowId {
                    row_id: sheet.sheet_id.clone(),
                    row_kind: "review_sheet",
                });
            }
            self.validate_sheet(sheet, matrix.as_ref(), violations);
        }
    }

    /// Returns the corpus-coverage gaps: any flow, script/native-build label,
    /// lockfile diff class, proposal source, review disposition, or checkpoint
    /// state the rows do not exercise.
    ///
    /// This is checked against the canonical embedded packet, not per-row, so a
    /// single-scenario fixture can still validate without exercising the whole
    /// vocabulary.
    pub fn corpus_coverage_gaps(&self) -> Vec<ReviewedMutationFlowsViolation> {
        let mut gaps = Vec::new();
        let flows: BTreeSet<MutationFlowClass> = self.sheets.iter().map(|s| s.flow_class).collect();
        for required in MutationFlowClass::ALL {
            if !flows.contains(&required) {
                gaps.push(ReviewedMutationFlowsViolation::MissingCorpusState {
                    field: "flow_class",
                    state: required.as_str(),
                });
            }
        }
        let labels: BTreeSet<ScriptBuildLabel> =
            self.sheets.iter().map(|s| s.script_build.label).collect();
        for required in ScriptBuildLabel::ALL {
            if !labels.contains(&required) {
                gaps.push(ReviewedMutationFlowsViolation::MissingCorpusState {
                    field: "script_build_label",
                    state: required.as_str(),
                });
            }
        }
        let lockfiles: BTreeSet<LockfileDiffClass> =
            self.sheets.iter().map(|s| s.lockfile.diff_class).collect();
        for required in LockfileDiffClass::ALL {
            if !lockfiles.contains(&required) {
                gaps.push(ReviewedMutationFlowsViolation::MissingCorpusState {
                    field: "lockfile_diff_class",
                    state: required.as_str(),
                });
            }
        }
        let sources: BTreeSet<ProposalSource> =
            self.sheets.iter().map(|s| s.proposal_source).collect();
        for required in ProposalSource::ALL {
            if !sources.contains(&required) {
                gaps.push(ReviewedMutationFlowsViolation::MissingCorpusState {
                    field: "proposal_source",
                    state: required.as_str(),
                });
            }
        }
        let dispositions: BTreeSet<ReviewDisposition> =
            self.sheets.iter().map(|s| s.review_disposition).collect();
        for required in ReviewDisposition::ALL {
            if !dispositions.contains(&required) {
                gaps.push(ReviewedMutationFlowsViolation::MissingCorpusState {
                    field: "review_disposition",
                    state: required.as_str(),
                });
            }
        }
        let states: BTreeSet<CheckpointState> = self.checkpoints.iter().map(|c| c.state).collect();
        for required in CheckpointState::ALL {
            if !states.contains(&required) {
                gaps.push(ReviewedMutationFlowsViolation::MissingCorpusState {
                    field: "checkpoint_state",
                    state: required.as_str(),
                });
            }
        }
        gaps
    }

    fn validate_sheet(
        &self,
        sheet: &MutationReviewSheet,
        matrix: Option<&crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::M5PackageStateMatrix>,
        violations: &mut Vec<ReviewedMutationFlowsViolation>,
    ) {
        for (field, value) in [
            ("sheet_id", &sheet.sheet_id),
            ("sheet_label", &sheet.sheet_label),
            ("scope_label", &sheet.manifest_scope.scope_label),
            (
                "redacted_manifest_path",
                &sheet.manifest_scope.redacted_manifest_path,
            ),
            ("package_name", &sheet.requested.package_name),
            ("requested_ref", &sheet.requested.requested_ref),
            ("resolver_version", &sheet.resolver.resolver_version),
            (
                "redacted_source_label",
                &sheet.registry_source.redacted_source_label,
            ),
            ("lockfile_note", &sheet.lockfile.note),
            ("rollback_checkpoint_id", &sheet.rollback_checkpoint_id),
            ("note", &sheet.note),
        ] {
            if value.trim().is_empty() {
                violations.push(ReviewedMutationFlowsViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: field,
                });
            }
        }
        if sheet.manifest_scope.affected_manifest_ids.is_empty() {
            violations.push(ReviewedMutationFlowsViolation::EmptyField {
                id: sheet.sheet_id.clone(),
                field_name: "affected_manifest_ids",
            });
        }
        // Redaction: no field may leak a raw URL.
        for (field, value) in [
            (
                "redacted_manifest_path",
                &sheet.manifest_scope.redacted_manifest_path,
            ),
            ("requested_ref", &sheet.requested.requested_ref),
            (
                "redacted_source_label",
                &sheet.registry_source.redacted_source_label,
            ),
        ] {
            if value.contains("://") {
                violations.push(ReviewedMutationFlowsViolation::RawUrlLeak {
                    id: sheet.sheet_id.clone(),
                    field_name: field,
                });
            }
        }
        if let Some(resolved) = &sheet.resolved {
            if resolved.resolved_ref.contains("://") {
                violations.push(ReviewedMutationFlowsViolation::RawUrlLeak {
                    id: sheet.sheet_id.clone(),
                    field_name: "resolved_ref",
                });
            }
        }
        // Manifest-scope confirmation flag must match the frozen scope class.
        if sheet.manifest_scope.confirmed_explicitly
            && !sheet.manifest_scope.requires_confirmation()
        {
            violations.push(
                ReviewedMutationFlowsViolation::ScopeConfirmationFlagMismatch {
                    sheet_id: sheet.sheet_id.clone(),
                },
            );
        }
        // Script / native-build risk must be disclosed and policy-consistent.
        if !sheet.script_build.is_consistent() {
            violations.push(ReviewedMutationFlowsViolation::ScriptRiskInconsistent {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        if sheet.script_build.label.is_risky()
            && sheet.script_build.disclosure_note.trim().is_empty()
        {
            violations.push(ReviewedMutationFlowsViolation::ScriptRiskUndisclosed {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        // Resolver version must be disclosed for any re-resolving flow.
        if sheet.flow_class.re_resolves() && sheet.resolver.resolver_version.trim().is_empty() {
            violations.push(ReviewedMutationFlowsViolation::MissingResolverVersion {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        // Lockfile blast radius must be internally consistent.
        if !sheet.lockfile.is_consistent() {
            violations.push(
                ReviewedMutationFlowsViolation::LockfileBlastRadiusInconsistent {
                    sheet_id: sheet.sheet_id.clone(),
                },
            );
        }
        if sheet.lockfile.diff_class.is_broad_churn() && !sheet.lockfile.broad_churn_disclosed {
            violations.push(ReviewedMutationFlowsViolation::BroadChurnUndisclosed {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        // Surface parity must hold across desktop, CLI dry run, and export.
        if !sheet.surface_parity.is_consistent() {
            violations.push(ReviewedMutationFlowsViolation::SurfaceParityBroken {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        // Every surfaced label must resolve in the frozen matrix.
        if let Some(matrix) = matrix {
            for label in &sheet.applicable_labels {
                if matrix.state(*label).is_none() {
                    violations.push(ReviewedMutationFlowsViolation::UnboundLabel {
                        sheet_id: sheet.sheet_id.clone(),
                        label: label.as_str(),
                    });
                }
            }
        }
        // The sheet must link to an existing checkpoint receipt.
        match self.checkpoint(&sheet.rollback_checkpoint_id) {
            None => violations.push(ReviewedMutationFlowsViolation::DanglingCheckpointRef {
                sheet_id: sheet.sheet_id.clone(),
                checkpoint_ref: sheet.rollback_checkpoint_id.clone(),
            }),
            Some(receipt) => {
                if !disposition_allows_checkpoint(sheet.review_disposition, receipt.state) {
                    violations.push(
                        ReviewedMutationFlowsViolation::DispositionCheckpointMismatch {
                            sheet_id: sheet.sheet_id.clone(),
                            checkpoint_id: receipt.checkpoint_id.clone(),
                        },
                    );
                }
            }
        }
        let card = sheet.manifest_diff_card(
            self.checkpoint(&sheet.rollback_checkpoint_id),
            self.commit_blocked(sheet),
        );
        if !card.discloses_apply_boundary() {
            violations.push(ReviewedMutationFlowsViolation::ManifestDiffCardIncomplete {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        if !card.fallback_honest() {
            violations.push(
                ReviewedMutationFlowsViolation::ManifestDiffFallbackDishonest {
                    sheet_id: sheet.sheet_id.clone(),
                },
            );
        }
        // The commit gate: a committed sheet may carry no live block reason.
        if sheet.review_disposition == ReviewDisposition::CommittedAfterReview
            && self.commit_blocked(sheet)
        {
            violations.push(ReviewedMutationFlowsViolation::CommitGateViolated {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }

    fn validate_checkpoints(&self, violations: &mut Vec<ReviewedMutationFlowsViolation>) {
        let mut seen = BTreeSet::new();
        for receipt in &self.checkpoints {
            if !seen.insert(receipt.checkpoint_id.clone()) {
                violations.push(ReviewedMutationFlowsViolation::DuplicateRowId {
                    row_id: receipt.checkpoint_id.clone(),
                    row_kind: "checkpoint",
                });
            }
            for (field, value) in [
                ("checkpoint_id", &receipt.checkpoint_id),
                ("sheet_id", &receipt.sheet_id),
                (
                    "lockfile_identity_before",
                    &receipt.lockfile_identity_before,
                ),
                ("lockfile_identity_after", &receipt.lockfile_identity_after),
                ("resulting_state", &receipt.resulting_state),
                ("receipt_label", &receipt.receipt_label),
            ] {
                if value.trim().is_empty() {
                    violations.push(ReviewedMutationFlowsViolation::EmptyField {
                        id: receipt.checkpoint_id.clone(),
                        field_name: field,
                    });
                }
            }
            if receipt.affected_manifest_ids.is_empty() {
                violations.push(ReviewedMutationFlowsViolation::EmptyField {
                    id: receipt.checkpoint_id.clone(),
                    field_name: "affected_manifest_ids",
                });
            }
            // A receipt must be durable, never a transient toast.
            if !receipt.durable {
                violations.push(ReviewedMutationFlowsViolation::NonDurableCheckpoint {
                    checkpoint_id: receipt.checkpoint_id.clone(),
                });
            }
            // A receipt must leave a real recovery path.
            if !receipt.is_recoverable() {
                violations.push(ReviewedMutationFlowsViolation::NonRecoverableRollback {
                    checkpoint_id: receipt.checkpoint_id.clone(),
                });
            }
            // Every receipt must offer revert, open-diff, and export-patch.
            let kinds: BTreeSet<RecoveryActionKind> =
                receipt.recovery_actions.iter().map(|a| a.kind).collect();
            for required in RecoveryActionKind::ALL {
                if !kinds.contains(&required) {
                    violations.push(ReviewedMutationFlowsViolation::MissingRecoveryAction {
                        checkpoint_id: receipt.checkpoint_id.clone(),
                        kind: required.as_str(),
                    });
                }
            }
            for action in &receipt.recovery_actions {
                if action.target_ref.trim().is_empty() {
                    violations.push(ReviewedMutationFlowsViolation::EmptyField {
                        id: receipt.checkpoint_id.clone(),
                        field_name: "recovery_action.target_ref",
                    });
                }
            }
            for (field, value) in [
                (
                    "lockfile_identity_before",
                    &receipt.lockfile_identity_before,
                ),
                ("lockfile_identity_after", &receipt.lockfile_identity_after),
            ] {
                if value.contains("://") {
                    violations.push(ReviewedMutationFlowsViolation::RawUrlLeak {
                        id: receipt.checkpoint_id.clone(),
                        field_name: field,
                    });
                }
            }
            // The guarded sheet must exist and point back at this checkpoint.
            match self.sheet(&receipt.sheet_id) {
                None => violations.push(ReviewedMutationFlowsViolation::DanglingSheetRef {
                    checkpoint_id: receipt.checkpoint_id.clone(),
                    sheet_ref: receipt.sheet_id.clone(),
                }),
                Some(sheet) => {
                    if sheet.rollback_checkpoint_id != receipt.checkpoint_id {
                        violations.push(ReviewedMutationFlowsViolation::CheckpointSheetMismatch {
                            checkpoint_id: receipt.checkpoint_id.clone(),
                            sheet_id: receipt.sheet_id.clone(),
                        });
                    }
                }
            }
        }
    }
}

/// A validation violation for the reviewed-mutation-flows packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewedMutationFlowsViolation {
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
        /// Row, section, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once within its kind.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
        /// Row kind discriminator.
        row_kind: &'static str,
    },
    /// A required corpus state is missing.
    MissingCorpusState {
        /// Field that must exercise the state.
        field: &'static str,
        /// Missing state token.
        state: &'static str,
    },
    /// A scope confirmation flag disagrees with the frozen scope class.
    ScopeConfirmationFlagMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's script/native-build review is internally inconsistent.
    ScriptRiskInconsistent {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet discloses script/native-build risk without a disclosure note.
    ScriptRiskUndisclosed {
        /// Sheet id.
        sheet_id: String,
    },
    /// A re-resolving flow does not disclose its resolver version.
    MissingResolverVersion {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's lockfile blast radius is internally inconsistent.
    LockfileBlastRadiusInconsistent {
        /// Sheet id.
        sheet_id: String,
    },
    /// A broad-churn diff is not disclosed before write.
    BroadChurnUndisclosed {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's review is not mirrored to every claimed surface.
    SurfaceParityBroken {
        /// Sheet id.
        sheet_id: String,
    },
    /// A surfaced label does not resolve in the frozen matrix.
    UnboundLabel {
        /// Sheet id.
        sheet_id: String,
        /// Unbound label token.
        label: &'static str,
    },
    /// A sheet references a missing checkpoint.
    DanglingCheckpointRef {
        /// Sheet id carrying the ref.
        sheet_id: String,
        /// Unresolvable checkpoint ref.
        checkpoint_ref: String,
    },
    /// A checkpoint references a missing sheet.
    DanglingSheetRef {
        /// Checkpoint id carrying the ref.
        checkpoint_id: String,
        /// Unresolvable sheet ref.
        sheet_ref: String,
    },
    /// A checkpoint and its sheet disagree about the link between them.
    CheckpointSheetMismatch {
        /// Checkpoint id.
        checkpoint_id: String,
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's disposition disagrees with its checkpoint state.
    DispositionCheckpointMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Checkpoint id.
        checkpoint_id: String,
    },
    /// A checkpoint receipt is not durable.
    NonDurableCheckpoint {
        /// Checkpoint id.
        checkpoint_id: String,
    },
    /// A checkpoint receipt leaves no real recovery path.
    NonRecoverableRollback {
        /// Checkpoint id.
        checkpoint_id: String,
    },
    /// A checkpoint lacks a required recovery action.
    MissingRecoveryAction {
        /// Checkpoint id.
        checkpoint_id: String,
        /// Missing recovery action kind token.
        kind: &'static str,
    },
    /// A committed sheet still carries a live block reason.
    CommitGateViolated {
        /// Sheet id.
        sheet_id: String,
    },
    /// A redacted field leaks a raw URL.
    RawUrlLeak {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// The packet binds to a matrix id other than the frozen matrix.
    MatrixBindingMismatch {
        /// Referenced matrix id.
        referenced: String,
    },
    /// The derived manifest-diff card omits a required pre-apply disclosure.
    ManifestDiffCardIncomplete {
        /// Sheet id.
        sheet_id: String,
    },
    /// The derived manifest-diff card failed to block or narrow a fallback.
    ManifestDiffFallbackDishonest {
        /// Sheet id.
        sheet_id: String,
    },
    /// Summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for ReviewedMutationFlowsViolation {
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
            Self::DuplicateRowId { row_id, row_kind } => {
                write!(f, "duplicate {row_kind} row id {row_id}")
            }
            Self::MissingCorpusState { field, state } => {
                write!(f, "packet corpus does not exercise {field} state {state}")
            }
            Self::ScopeConfirmationFlagMismatch { sheet_id } => write!(
                f,
                "sheet {sheet_id} confirms a scope that needs no confirmation"
            ),
            Self::ScriptRiskInconsistent { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} script/native-build review is inconsistent"
                )
            }
            Self::ScriptRiskUndisclosed { sheet_id } => write!(
                f,
                "sheet {sheet_id} discloses script/native-build risk without a note"
            ),
            Self::MissingResolverVersion { sheet_id } => {
                write!(f, "sheet {sheet_id} re-resolves without a resolver version")
            }
            Self::LockfileBlastRadiusInconsistent { sheet_id } => {
                write!(f, "sheet {sheet_id} lockfile blast radius is inconsistent")
            }
            Self::BroadChurnUndisclosed { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} does not disclose a broad lockfile churn"
                )
            }
            Self::SurfaceParityBroken { sheet_id } => write!(
                f,
                "sheet {sheet_id} review is not mirrored to every claimed surface"
            ),
            Self::UnboundLabel { sheet_id, label } => {
                write!(f, "sheet {sheet_id} surfaces unbound label {label}")
            }
            Self::DanglingCheckpointRef {
                sheet_id,
                checkpoint_ref,
            } => write!(
                f,
                "sheet {sheet_id} references missing checkpoint {checkpoint_ref}"
            ),
            Self::DanglingSheetRef {
                checkpoint_id,
                sheet_ref,
            } => write!(
                f,
                "checkpoint {checkpoint_id} references missing sheet {sheet_ref}"
            ),
            Self::CheckpointSheetMismatch {
                checkpoint_id,
                sheet_id,
            } => write!(
                f,
                "checkpoint {checkpoint_id} and sheet {sheet_id} disagree about their link"
            ),
            Self::DispositionCheckpointMismatch {
                sheet_id,
                checkpoint_id,
            } => write!(
                f,
                "sheet {sheet_id} disposition disagrees with checkpoint {checkpoint_id} state"
            ),
            Self::NonDurableCheckpoint { checkpoint_id } => {
                write!(f, "checkpoint {checkpoint_id} is not a durable receipt")
            }
            Self::NonRecoverableRollback { checkpoint_id } => {
                write!(f, "checkpoint {checkpoint_id} leaves no real recovery path")
            }
            Self::MissingRecoveryAction {
                checkpoint_id,
                kind,
            } => write!(f, "checkpoint {checkpoint_id} lacks {kind} recovery action"),
            Self::CommitGateViolated { sheet_id } => write!(
                f,
                "sheet {sheet_id} is committed while a block reason still holds"
            ),
            Self::RawUrlLeak { id, field_name } => {
                write!(f, "{id} field {field_name} leaks a raw URL")
            }
            Self::MatrixBindingMismatch { referenced } => {
                write!(f, "packet binds to non-frozen matrix {referenced}")
            }
            Self::ManifestDiffCardIncomplete { sheet_id } => write!(
                f,
                "sheet {sheet_id} manifest-diff card omits a required pre-apply disclosure"
            ),
            Self::ManifestDiffFallbackDishonest { sheet_id } => write!(
                f,
                "sheet {sheet_id} manifest-diff card does not honestly block or narrow a fallback"
            ),
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for ReviewedMutationFlowsViolation {}

/// Loads the embedded reviewed-mutation-flows packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`ReviewedMutationFlows`].
pub fn current_reviewed_mutation_flows() -> Result<ReviewedMutationFlows, serde_json::Error> {
    serde_json::from_str(REVIEWED_MUTATION_FLOWS_JSON)
}

#[cfg(test)]
mod tests;
