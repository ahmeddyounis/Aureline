//! Grouped update planner, conflict-card, lockfile-churn, native-build/install
//! disclosure, and rollback-checkpoint review packet for safe package mutation.
//!
//! This module publishes the canonical vocabulary and typed packet that keeps
//! the pre-apply review of grouped dependency updates honest across desktop,
//! CLI/headless dry run, and support/export surfaces. It exists so a package
//! mutation can never masquerade as a harmless text edit: every plan discloses
//! requested-versus-resolved versions, the manifests it touches, an estimated
//! lockfile-churn class, constraint/conflict cards, native-build or
//! install-script risk, the registry/auth source it would reach, and the
//! validation pack recommended before the change leaves review.
//!
//! Each [`UpdatePlan`] distinguishes the six grouped-update intents — direct
//! bump, security patch, grouped refresh, lockfile-only refresh, major-version
//! pilot, and workspace-wide convergence — and links to a durable
//! [`RollbackCheckpointReceipt`]. A checkpoint preserves the affected
//! manifests, the lockfile identity before and after, the validation outcome,
//! and the revert / open-diff / export-patch recovery actions, so a broken or
//! partial mutation leaves a checkpointed recovery path and a durable receipt
//! rather than a transient toast.
//!
//! The checked-in packet lives at
//! `artifacts/deps/m5/grouped-update-and-rollback-review.json` and is embedded
//! here so Rust consumers, CLI/headless output, support exports, and release
//! evidence all validate against the same source of truth.
//!
//! The enums [`EcosystemClass`], [`CredentialMode`], [`RegistrySourceClass`],
//! and [`ScriptNativeBuildRiskClass`] intentionally reuse the stable token
//! vocabulary already shipped by the package-mutation contract; they are kept
//! local to this packet so the review-and-rollback lane stays self-contained.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported grouped-update and rollback-review packet schema version.
pub const GROUPED_UPDATE_AND_ROLLBACK_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const GROUPED_UPDATE_AND_ROLLBACK_REVIEW_RECORD_KIND: &str =
    "grouped_update_and_rollback_review";

/// Repo-relative path to the checked-in packet.
pub const GROUPED_UPDATE_AND_ROLLBACK_REVIEW_PATH: &str =
    "artifacts/deps/m5/grouped-update-and-rollback-review.json";

/// Embedded checked-in packet JSON.
pub const GROUPED_UPDATE_AND_ROLLBACK_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/grouped-update-and-rollback-review.json"
));

/// Ecosystem class for a grouped-update plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EcosystemClass {
    /// Rust Cargo workspace and crate manifests.
    Cargo,
    /// Node package manifests using pnpm workspace semantics.
    NodePnpm,
}

impl EcosystemClass {
    /// Every ecosystem class claimed by this packet.
    pub const ALL: [Self; 2] = [Self::Cargo, Self::NodePnpm];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::NodePnpm => "node_pnpm",
        }
    }
}

/// The grouped-update intent a plan represents.
///
/// These six classes are deliberately distinct so a direct bump, a security
/// patch, a grouped refresh, a lockfile-only refresh, a major-version pilot,
/// and a workspace-wide convergence never collapse into one generic "update".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePlanClass {
    /// A single direct dependency version bump.
    DirectBump,
    /// A targeted security patch driven by an advisory.
    SecurityPatch,
    /// A grouped refresh of several related dependencies.
    GroupedRefresh,
    /// A lockfile-only re-resolution with no manifest range change.
    LockfileRefreshOnly,
    /// A piloted major-version upgrade behind explicit review.
    MajorVersionPilot,
    /// A workspace-wide convergence onto a single shared version.
    WorkspaceWideConvergence,
}

impl UpdatePlanClass {
    /// Every update-plan class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DirectBump,
        Self::SecurityPatch,
        Self::GroupedRefresh,
        Self::LockfileRefreshOnly,
        Self::MajorVersionPilot,
        Self::WorkspaceWideConvergence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectBump => "direct_bump",
            Self::SecurityPatch => "security_patch",
            Self::GroupedRefresh => "grouped_refresh",
            Self::LockfileRefreshOnly => "lockfile_refresh_only",
            Self::MajorVersionPilot => "major_version_pilot",
            Self::WorkspaceWideConvergence => "workspace_wide_convergence",
        }
    }
}

/// Estimated magnitude of lockfile churn a plan would produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileChurnClass {
    /// A single lockfile entry changes.
    SingleEntry,
    /// A localized cluster of entries changes.
    Localized,
    /// A moderate set of entries changes.
    Moderate,
    /// A broad set of entries changes.
    Broad,
    /// Churn reaches across the whole workspace lockfile.
    WorkspaceWide,
}

impl LockfileChurnClass {
    /// Every churn class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SingleEntry,
        Self::Localized,
        Self::Moderate,
        Self::Broad,
        Self::WorkspaceWide,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleEntry => "single_entry",
            Self::Localized => "localized",
            Self::Moderate => "moderate",
            Self::Broad => "broad",
            Self::WorkspaceWide => "workspace_wide",
        }
    }
}

/// Constraint/conflict class surfaced on a conflict card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictClass {
    /// Constraints conflict and cannot be unified.
    VersionConflict,
    /// A peer or shared-range requirement conflicts.
    PeerRequirementConflict,
    /// Multiple resolved versions would coexist for one package.
    DuplicateVersions,
    /// Multiple requests would unify onto a single resolved version.
    FeatureUnification,
    /// A target version carries an advisory or has been yanked.
    AdvisoryOrYank,
}

impl ConflictClass {
    /// Every conflict class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::VersionConflict,
        Self::PeerRequirementConflict,
        Self::DuplicateVersions,
        Self::FeatureUnification,
        Self::AdvisoryOrYank,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VersionConflict => "version_conflict",
            Self::PeerRequirementConflict => "peer_requirement_conflict",
            Self::DuplicateVersions => "duplicate_versions",
            Self::FeatureUnification => "feature_unification",
            Self::AdvisoryOrYank => "advisory_or_yank",
        }
    }
}

/// Native-build or install-script risk class disclosed before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptNativeBuildRiskClass {
    /// No script or native-build risk is known.
    NoneKnown,
    /// The change introduces or retains package install/lifecycle scripts.
    PackageScripts,
    /// The change requires a native build (compiler/toolchain).
    NativeBuild,
    /// The change introduces or widens network egress.
    NewEgress,
    /// Policy blocks the script or native-build behavior.
    PolicyBlocked,
    /// Script or native-build behavior cannot be determined.
    Unknown,
}

impl ScriptNativeBuildRiskClass {
    /// Every script/native-build risk class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoneKnown,
        Self::PackageScripts,
        Self::NativeBuild,
        Self::NewEgress,
        Self::PolicyBlocked,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneKnown => "none_known",
            Self::PackageScripts => "package_scripts",
            Self::NativeBuild => "native_build",
            Self::NewEgress => "new_egress",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unknown => "unknown",
        }
    }

    /// Whether the class discloses a script or native-build risk that review
    /// must surface before apply.
    pub const fn is_risky(self) -> bool {
        !matches!(self, Self::NoneKnown)
    }
}

/// Registry or mirror source a plan would reach to resolve a change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrySourceClass {
    /// A public package registry.
    PublicRegistry,
    /// A private package registry.
    PrivateRegistry,
    /// An enterprise mirror.
    EnterpriseMirror,
    /// A local on-disk cache.
    LocalCache,
    /// An offline snapshot.
    OfflineSnapshot,
}

impl RegistrySourceClass {
    /// Every registry source class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PublicRegistry,
        Self::PrivateRegistry,
        Self::EnterpriseMirror,
        Self::LocalCache,
        Self::OfflineSnapshot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicRegistry => "public_registry",
            Self::PrivateRegistry => "private_registry",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::LocalCache => "local_cache",
            Self::OfflineSnapshot => "offline_snapshot",
        }
    }
}

/// Credential mode a plan's registry source uses.
///
/// This records only the *mode*; no credential body is ever carried by the
/// packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialMode {
    /// Anonymous, no credential required.
    Anonymous,
    /// Credential resolved from the OS secret store.
    OsStore,
    /// A token credential.
    Token,
    /// Browser or device interactive sign-in.
    BrowserOrDeviceSignIn,
    /// Credential inherited from policy.
    PolicyInherited,
}

impl CredentialMode {
    /// Every credential mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Anonymous,
        Self::OsStore,
        Self::Token,
        Self::BrowserOrDeviceSignIn,
        Self::PolicyInherited,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Anonymous => "anonymous",
            Self::OsStore => "os_store",
            Self::Token => "token",
            Self::BrowserOrDeviceSignIn => "browser_or_device_sign_in",
            Self::PolicyInherited => "policy_inherited",
        }
    }
}

/// Outcome of a recommended validation pack, before or after apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationOutcomeClass {
    /// The validation pack has not run yet.
    NotRun,
    /// The validation pack passed.
    Passed,
    /// The validation pack failed.
    Failed,
    /// The validation pack partially passed.
    Partial,
    /// The validation pack was skipped by policy.
    SkippedByPolicy,
}

impl ValidationOutcomeClass {
    /// Every validation outcome, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotRun,
        Self::Passed,
        Self::Failed,
        Self::Partial,
        Self::SkippedByPolicy,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRun => "not_run",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Partial => "partial",
            Self::SkippedByPolicy => "skipped_by_policy",
        }
    }
}

/// Review disposition of a grouped-update plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDisposition {
    /// The plan is awaiting review.
    PendingReview,
    /// The plan has been reviewed and is ready to apply.
    ReviewedReady,
    /// The plan is blocked until a conflict or risk is resolved.
    BlockedUntilResolved,
    /// The plan has been applied.
    Applied,
    /// The plan was applied and then rolled back.
    RolledBack,
}

impl ReviewDisposition {
    /// Every review disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PendingReview,
        Self::ReviewedReady,
        Self::BlockedUntilResolved,
        Self::Applied,
        Self::RolledBack,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::ReviewedReady => "reviewed_ready",
            Self::BlockedUntilResolved => "blocked_until_resolved",
            Self::Applied => "applied",
            Self::RolledBack => "rolled_back",
        }
    }
}

/// State of a rollback checkpoint receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointState {
    /// A pre-apply checkpoint has been captured.
    Captured,
    /// The change applied and the checkpoint remains reversible.
    AppliedReversible,
    /// A broken or partial mutation left recovery pending.
    PartialRecoveryPending,
    /// The change was reverted from this checkpoint.
    Reverted,
    /// The checkpoint was superseded by a newer plan.
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

    /// Whether the state represents a broken or partial mutation that still
    /// needs an operator-driven recovery.
    pub const fn is_recovery_pending(self) -> bool {
        matches!(self, Self::PartialRecoveryPending)
    }
}

/// Recovery action a checkpoint receipt offers.
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

/// Stable surface contract: which surfaces ingest this packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupedUpdateSurfaceContract {
    /// Desktop grouped-update review surface.
    pub review_surface: String,
    /// Desktop conflict-card surface.
    pub conflict_card_surface: String,
    /// Rollback / checkpoint receipt surface.
    pub rollback_surface: String,
    /// CLI/headless dry-run surface.
    pub cli_dry_run_surface: String,
    /// Help page describing the packet.
    pub help_page: String,
    /// Support-export channel.
    pub support_export_surface: String,
}

/// Which surfaces a plan's review is mirrored to.
///
/// Grouped-update review must behave consistently across desktop, the
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
    /// Whether the plan is mirrored to every claimed surface.
    pub const fn is_consistent(&self) -> bool {
        self.desktop && self.cli_headless_dry_run && self.support_export
    }
}

/// One package's requested-versus-resolved change within a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageVersionChange {
    /// Stable package identity.
    pub package_id: String,
    /// Human-readable package name.
    pub package_name: String,
    /// Ecosystem this package belongs to.
    pub ecosystem: EcosystemClass,
    /// Manifest that declares the change.
    pub manifest_path: String,
    /// Requested range or source as it would be written.
    pub requested_range_or_source: String,
    /// Currently resolved version (before the change).
    pub from_resolved_version: String,
    /// Resolved version the change would produce.
    pub to_resolved_version: String,
}

/// One constraint/conflict card shown before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConflictCard {
    /// Stable card id.
    pub card_id: String,
    /// Conflict class.
    pub conflict_class: ConflictClass,
    /// Package the card is about.
    pub package_id: String,
    /// Human-readable disclosure of the conflict.
    pub disclosure: String,
    /// Suggested resolution hint.
    pub resolution_hint: String,
}

/// Native-build or install-script risk disclosure for a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptNativeBuildDisclosure {
    /// Risk class.
    pub risk_class: ScriptNativeBuildRiskClass,
    /// Package refs that introduce the risk.
    #[serde(default)]
    pub source_package_refs: Vec<String>,
    /// Whether policy allows the risk.
    pub policy_allows: bool,
    /// Whether the operator must explicitly acknowledge the risk before apply.
    pub requires_explicit_ack: bool,
    /// Reviewer-facing disclosure note.
    pub disclosure_note: String,
}

/// The registry/auth source a plan would reach.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistrySource {
    /// Registry source class.
    pub source_class: RegistrySourceClass,
    /// Credential mode (mode only; never a credential body).
    pub credential_mode: CredentialMode,
    /// Redaction-safe source label.
    pub source_label: String,
}

/// Validation pack recommended for a plan, with its current outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationPackRecommendation {
    /// Stable validation-pack id.
    pub validation_pack_id: String,
    /// Recommended checks or commands.
    pub recommended_checks: Vec<String>,
    /// Whether the pack must pass before apply.
    pub required_before_apply: bool,
    /// Current outcome of the pack.
    pub outcome: ValidationOutcomeClass,
}

/// One grouped-update plan, with full pre-apply disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdatePlan {
    /// Stable plan id.
    pub plan_id: String,
    /// Update-plan class.
    pub plan_class: UpdatePlanClass,
    /// Ecosystem the plan targets.
    pub ecosystem: EcosystemClass,
    /// Human-readable plan label.
    pub plan_label: String,
    /// Manifests this plan would touch.
    pub manifests_touched: Vec<String>,
    /// Per-package requested-versus-resolved changes.
    pub package_changes: Vec<PackageVersionChange>,
    /// Estimated lockfile-churn class.
    pub lockfile_churn_class: LockfileChurnClass,
    /// Lockfile entries the plan would add.
    pub lockfile_entries_added: u32,
    /// Lockfile entries the plan would remove.
    pub lockfile_entries_removed: u32,
    /// Lockfile entries the plan would change in place.
    pub lockfile_entries_changed: u32,
    /// Reviewer-facing blast-radius note.
    pub blast_radius_note: String,
    /// Constraint/conflict cards (empty when the plan is clean).
    #[serde(default)]
    pub conflict_cards: Vec<ConflictCard>,
    /// Native-build or install-script risk disclosure.
    pub script_native_build: ScriptNativeBuildDisclosure,
    /// Registry/auth source the plan would reach.
    pub registry_source: RegistrySource,
    /// Recommended validation pack.
    pub validation_pack: ValidationPackRecommendation,
    /// Review disposition.
    pub review_disposition: ReviewDisposition,
    /// Surface parity across desktop, CLI dry run, and support/export.
    pub surface_parity: SurfaceParity,
    /// Checkpoint receipt that guards this plan.
    pub checkpoint_id: String,
}

/// A recovery action offered by a checkpoint receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryAction {
    /// Recovery action kind.
    pub kind: RecoveryActionKind,
    /// Redaction-safe target reference.
    pub target_ref: String,
}

/// A durable rollback checkpoint receipt for a grouped-update plan.
///
/// Broken or partial mutations leave one of these — a durable receipt, never a
/// transient toast — preserving the affected manifests, the lockfile identity
/// before and after, the validation outcome, and the revert / open-diff /
/// export-patch recovery actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackCheckpointReceipt {
    /// Stable checkpoint id.
    pub checkpoint_id: String,
    /// Plan this checkpoint guards.
    pub plan_id: String,
    /// Checkpoint state.
    pub state: CheckpointState,
    /// Manifests covered by the checkpoint.
    pub affected_manifests: Vec<String>,
    /// Redaction-safe lockfile identity before the change.
    pub lockfile_identity_before: String,
    /// Redaction-safe lockfile identity after the change.
    pub lockfile_identity_after: String,
    /// Validation outcome captured with the receipt.
    pub validation_outcome: ValidationOutcomeClass,
    /// Whether the receipt is durable (must always be `true`).
    pub durable: bool,
    /// Human-readable receipt label.
    pub receipt_label: String,
    /// Revert / open-diff / export-patch recovery actions.
    pub recovery_actions: Vec<RecoveryAction>,
}

/// Summary counts derived from the rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupedUpdateAndRollbackReviewSummary {
    /// Total update-plan rows.
    pub plan_rows: usize,
    /// Direct-bump plans.
    pub direct_bump_plans: usize,
    /// Security-patch plans.
    pub security_patch_plans: usize,
    /// Grouped-refresh plans.
    pub grouped_refresh_plans: usize,
    /// Lockfile-only refresh plans.
    pub lockfile_refresh_only_plans: usize,
    /// Major-version pilot plans.
    pub major_version_pilot_plans: usize,
    /// Workspace-wide convergence plans.
    pub workspace_wide_convergence_plans: usize,
    /// Plans carrying at least one conflict card.
    pub plans_with_conflict_cards: usize,
    /// Plans disclosing a script or native-build risk.
    pub plans_with_script_or_native_risk: usize,
    /// Total conflict cards across all plans.
    pub conflict_card_rows: usize,
    /// Total checkpoint receipts.
    pub checkpoint_rows: usize,
    /// Durable checkpoint receipts.
    pub durable_checkpoint_rows: usize,
    /// Checkpoints in a partial-recovery-pending state.
    pub recovery_pending_checkpoint_rows: usize,
    /// Checkpoints that have been reverted.
    pub reverted_checkpoint_rows: usize,
}

/// One row of the redaction-safe export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupedUpdateAndRollbackReviewExportRow {
    /// Row id (plan id or checkpoint id).
    pub row_id: String,
    /// Row kind discriminator.
    pub row_kind: String,
    /// Ecosystem token, or a marker for checkpoint rows.
    pub ecosystem: String,
    /// Plan or checkpoint label.
    pub label: String,
    /// Effective state token.
    pub effective_state: String,
    /// Human-readable summary.
    pub summary: String,
}

/// Redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupedUpdateAndRollbackReviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<GroupedUpdateAndRollbackReviewExportRow>,
}

/// Typed grouped-update and rollback-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupedUpdateAndRollbackReview {
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
    /// Stable surface contract.
    pub surface_contract: GroupedUpdateSurfaceContract,
    /// Closed update-plan-class vocabulary.
    pub update_plan_classes: Vec<UpdatePlanClass>,
    /// Closed lockfile-churn-class vocabulary.
    pub lockfile_churn_classes: Vec<LockfileChurnClass>,
    /// Closed conflict-class vocabulary.
    pub conflict_classes: Vec<ConflictClass>,
    /// Closed script/native-build risk vocabulary.
    pub script_native_build_risk_classes: Vec<ScriptNativeBuildRiskClass>,
    /// Closed registry-source vocabulary.
    pub registry_source_classes: Vec<RegistrySourceClass>,
    /// Closed credential-mode vocabulary.
    pub credential_modes: Vec<CredentialMode>,
    /// Closed validation-outcome vocabulary.
    pub validation_outcome_classes: Vec<ValidationOutcomeClass>,
    /// Closed review-disposition vocabulary.
    pub review_dispositions: Vec<ReviewDisposition>,
    /// Closed checkpoint-state vocabulary.
    pub checkpoint_states: Vec<CheckpointState>,
    /// Closed recovery-action vocabulary.
    pub recovery_action_kinds: Vec<RecoveryActionKind>,
    /// Stable ecosystems claimed by this packet.
    pub claimed_stable_ecosystems: Vec<EcosystemClass>,
    /// Grouped-update plan rows.
    #[serde(default)]
    pub update_plans: Vec<UpdatePlan>,
    /// Rollback checkpoint receipt rows.
    #[serde(default)]
    pub checkpoints: Vec<RollbackCheckpointReceipt>,
    /// Summary counts.
    pub summary: GroupedUpdateAndRollbackReviewSummary,
}

impl GroupedUpdateAndRollbackReview {
    /// Returns the plan for `plan_id`.
    pub fn plan(&self, plan_id: &str) -> Option<&UpdatePlan> {
        self.update_plans.iter().find(|row| row.plan_id == plan_id)
    }

    /// Returns the checkpoint receipt for `checkpoint_id`.
    pub fn checkpoint(&self, checkpoint_id: &str) -> Option<&RollbackCheckpointReceipt> {
        self.checkpoints
            .iter()
            .find(|row| row.checkpoint_id == checkpoint_id)
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> GroupedUpdateAndRollbackReviewSummary {
        let plan_class_count = |class: UpdatePlanClass| {
            self.update_plans
                .iter()
                .filter(|row| row.plan_class == class)
                .count()
        };
        GroupedUpdateAndRollbackReviewSummary {
            plan_rows: self.update_plans.len(),
            direct_bump_plans: plan_class_count(UpdatePlanClass::DirectBump),
            security_patch_plans: plan_class_count(UpdatePlanClass::SecurityPatch),
            grouped_refresh_plans: plan_class_count(UpdatePlanClass::GroupedRefresh),
            lockfile_refresh_only_plans: plan_class_count(UpdatePlanClass::LockfileRefreshOnly),
            major_version_pilot_plans: plan_class_count(UpdatePlanClass::MajorVersionPilot),
            workspace_wide_convergence_plans: plan_class_count(
                UpdatePlanClass::WorkspaceWideConvergence,
            ),
            plans_with_conflict_cards: self
                .update_plans
                .iter()
                .filter(|row| !row.conflict_cards.is_empty())
                .count(),
            plans_with_script_or_native_risk: self
                .update_plans
                .iter()
                .filter(|row| row.script_native_build.risk_class.is_risky())
                .count(),
            conflict_card_rows: self
                .update_plans
                .iter()
                .map(|row| row.conflict_cards.len())
                .sum(),
            checkpoint_rows: self.checkpoints.len(),
            durable_checkpoint_rows: self.checkpoints.iter().filter(|row| row.durable).count(),
            recovery_pending_checkpoint_rows: self
                .checkpoints
                .iter()
                .filter(|row| row.state.is_recovery_pending())
                .count(),
            reverted_checkpoint_rows: self
                .checkpoints
                .iter()
                .filter(|row| row.state == CheckpointState::Reverted)
                .count(),
        }
    }

    /// Produces a redaction-safe export projection for UI, CLI, support, docs,
    /// release, and public proof consumers.
    pub fn export_projection(&self) -> GroupedUpdateAndRollbackReviewExportProjection {
        let mut rows = Vec::new();
        for plan in &self.update_plans {
            rows.push(GroupedUpdateAndRollbackReviewExportRow {
                row_id: plan.plan_id.clone(),
                row_kind: "update_plan".to_owned(),
                ecosystem: plan.ecosystem.as_str().to_owned(),
                label: plan.plan_label.clone(),
                effective_state: plan.review_disposition.as_str().to_owned(),
                summary: format!(
                    "{} churn {} (+{}/-{}/~{}) conflicts {} script {} validation {}",
                    plan.plan_class.as_str(),
                    plan.lockfile_churn_class.as_str(),
                    plan.lockfile_entries_added,
                    plan.lockfile_entries_removed,
                    plan.lockfile_entries_changed,
                    plan.conflict_cards.len(),
                    plan.script_native_build.risk_class.as_str(),
                    plan.validation_pack.outcome.as_str(),
                ),
            });
        }
        for checkpoint in &self.checkpoints {
            rows.push(GroupedUpdateAndRollbackReviewExportRow {
                row_id: checkpoint.checkpoint_id.clone(),
                row_kind: "checkpoint".to_owned(),
                ecosystem: "workspace".to_owned(),
                label: checkpoint.receipt_label.clone(),
                effective_state: checkpoint.state.as_str().to_owned(),
                summary: format!(
                    "{} durable {} validation {} actions {}",
                    checkpoint.plan_id,
                    checkpoint.durable,
                    checkpoint.validation_outcome.as_str(),
                    checkpoint.recovery_actions.len(),
                ),
            });
        }
        GroupedUpdateAndRollbackReviewExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<GroupedUpdateAndRollbackReviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_plans(&mut violations);
        self.validate_checkpoints(&mut violations);
        if self.summary != self.computed_summary() {
            violations.push(GroupedUpdateAndRollbackReviewViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<GroupedUpdateAndRollbackReviewViolation>) {
        if self.schema_version != GROUPED_UPDATE_AND_ROLLBACK_REVIEW_SCHEMA_VERSION {
            violations.push(
                GroupedUpdateAndRollbackReviewViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != GROUPED_UPDATE_AND_ROLLBACK_REVIEW_RECORD_KIND {
            violations.push(
                GroupedUpdateAndRollbackReviewViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, value) in [
            ("review_surface", &self.surface_contract.review_surface),
            (
                "conflict_card_surface",
                &self.surface_contract.conflict_card_surface,
            ),
            ("rollback_surface", &self.surface_contract.rollback_surface),
            (
                "cli_dry_run_surface",
                &self.surface_contract.cli_dry_run_surface,
            ),
            ("help_page", &self.surface_contract.help_page),
            (
                "support_export_surface",
                &self.surface_contract.support_export_surface,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                    id: "<surface_contract>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab_checks: [(&'static str, bool); 11] = [
            (
                "update_plan_classes",
                self.update_plan_classes == UpdatePlanClass::ALL.to_vec(),
            ),
            (
                "lockfile_churn_classes",
                self.lockfile_churn_classes == LockfileChurnClass::ALL.to_vec(),
            ),
            (
                "conflict_classes",
                self.conflict_classes == ConflictClass::ALL.to_vec(),
            ),
            (
                "script_native_build_risk_classes",
                self.script_native_build_risk_classes == ScriptNativeBuildRiskClass::ALL.to_vec(),
            ),
            (
                "registry_source_classes",
                self.registry_source_classes == RegistrySourceClass::ALL.to_vec(),
            ),
            (
                "credential_modes",
                self.credential_modes == CredentialMode::ALL.to_vec(),
            ),
            (
                "validation_outcome_classes",
                self.validation_outcome_classes == ValidationOutcomeClass::ALL.to_vec(),
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
            (
                "claimed_stable_ecosystems",
                self.claimed_stable_ecosystems == EcosystemClass::ALL.to_vec(),
            ),
        ];
        for (field, ok) in vocab_checks {
            if !ok {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::ClosedVocabularyMismatch { field },
                );
            }
        }
    }

    fn validate_plans(&self, violations: &mut Vec<GroupedUpdateAndRollbackReviewViolation>) {
        let mut seen = BTreeSet::new();
        let mut plan_classes_seen = BTreeSet::new();
        let mut churn_seen = BTreeSet::new();
        let mut conflict_seen = BTreeSet::new();
        let mut script_seen = BTreeSet::new();
        let mut validation_seen = BTreeSet::new();
        let mut disposition_seen = BTreeSet::new();
        for plan in &self.update_plans {
            if !seen.insert(plan.plan_id.clone()) {
                violations.push(GroupedUpdateAndRollbackReviewViolation::DuplicateRowId {
                    row_id: plan.plan_id.clone(),
                    row_kind: "update_plan",
                });
            }
            plan_classes_seen.insert(plan.plan_class);
            churn_seen.insert(plan.lockfile_churn_class);
            script_seen.insert(plan.script_native_build.risk_class);
            validation_seen.insert(plan.validation_pack.outcome);
            disposition_seen.insert(plan.review_disposition);
            for card in &plan.conflict_cards {
                conflict_seen.insert(card.conflict_class);
            }
            self.validate_plan(plan, violations);
        }
        for required in UpdatePlanClass::ALL {
            if !plan_classes_seen.contains(&required) {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::MissingCorpusState {
                        field: "plan_class",
                        state: required.as_str(),
                    },
                );
            }
        }
        for required in LockfileChurnClass::ALL {
            if !churn_seen.contains(&required) {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::MissingCorpusState {
                        field: "lockfile_churn_class",
                        state: required.as_str(),
                    },
                );
            }
        }
        for required in ConflictClass::ALL {
            if !conflict_seen.contains(&required) {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::MissingCorpusState {
                        field: "conflict_class",
                        state: required.as_str(),
                    },
                );
            }
        }
        for required in ScriptNativeBuildRiskClass::ALL {
            if !script_seen.contains(&required) {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::MissingCorpusState {
                        field: "script_native_build_risk_class",
                        state: required.as_str(),
                    },
                );
            }
        }
        for required in ValidationOutcomeClass::ALL {
            if !validation_seen.contains(&required) {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::MissingCorpusState {
                        field: "validation_outcome_class",
                        state: required.as_str(),
                    },
                );
            }
        }
        for required in ReviewDisposition::ALL {
            if !disposition_seen.contains(&required) {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::MissingCorpusState {
                        field: "review_disposition",
                        state: required.as_str(),
                    },
                );
            }
        }
    }

    fn validate_plan(
        &self,
        plan: &UpdatePlan,
        violations: &mut Vec<GroupedUpdateAndRollbackReviewViolation>,
    ) {
        for (field, value) in [
            ("plan_id", &plan.plan_id),
            ("plan_label", &plan.plan_label),
            ("blast_radius_note", &plan.blast_radius_note),
            ("checkpoint_id", &plan.checkpoint_id),
        ] {
            if value.trim().is_empty() {
                violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                    id: plan.plan_id.clone(),
                    field_name: field,
                });
            }
        }
        if plan.manifests_touched.is_empty() {
            violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                id: plan.plan_id.clone(),
                field_name: "manifests_touched",
            });
        }
        if plan.package_changes.is_empty() {
            violations.push(
                GroupedUpdateAndRollbackReviewViolation::MissingPackageChange {
                    plan_id: plan.plan_id.clone(),
                },
            );
        }
        for change in &plan.package_changes {
            for (field, value) in [
                ("package_id", &change.package_id),
                ("package_name", &change.package_name),
                ("manifest_path", &change.manifest_path),
                (
                    "requested_range_or_source",
                    &change.requested_range_or_source,
                ),
                ("from_resolved_version", &change.from_resolved_version),
                ("to_resolved_version", &change.to_resolved_version),
            ] {
                if value.trim().is_empty() {
                    violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                        id: plan.plan_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
        // Lockfile-churn estimate must be non-zero: every claimed churn class
        // implies at least one touched lockfile entry.
        let churn_total = plan.lockfile_entries_added
            + plan.lockfile_entries_removed
            + plan.lockfile_entries_changed;
        if churn_total == 0 {
            violations.push(GroupedUpdateAndRollbackReviewViolation::ZeroLockfileChurn {
                plan_id: plan.plan_id.clone(),
            });
        }
        // Every conflict card must carry a disclosure and resolution hint.
        let mut card_ids = BTreeSet::new();
        for card in &plan.conflict_cards {
            if !card_ids.insert(card.card_id.clone()) {
                violations.push(GroupedUpdateAndRollbackReviewViolation::DuplicateRowId {
                    row_id: card.card_id.clone(),
                    row_kind: "conflict_card",
                });
            }
            for (field, value) in [
                ("card_id", &card.card_id),
                ("package_id", &card.package_id),
                ("disclosure", &card.disclosure),
                ("resolution_hint", &card.resolution_hint),
            ] {
                if value.trim().is_empty() {
                    violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                        id: plan.plan_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
        // A disclosed script/native-build risk must carry a disclosure note:
        // package operations may not masquerade as harmless text edits.
        if plan.script_native_build.risk_class.is_risky()
            && plan.script_native_build.disclosure_note.trim().is_empty()
        {
            violations.push(
                GroupedUpdateAndRollbackReviewViolation::ScriptRiskUndisclosed {
                    plan_id: plan.plan_id.clone(),
                },
            );
        }
        if plan.registry_source.source_label.trim().is_empty() {
            violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                id: plan.plan_id.clone(),
                field_name: "registry_source.source_label",
            });
        }
        if plan.validation_pack.validation_pack_id.trim().is_empty() {
            violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                id: plan.plan_id.clone(),
                field_name: "validation_pack.validation_pack_id",
            });
        }
        if plan.validation_pack.recommended_checks.is_empty() {
            violations.push(
                GroupedUpdateAndRollbackReviewViolation::MissingValidationPack {
                    plan_id: plan.plan_id.clone(),
                },
            );
        }
        // Review must be consistent across desktop, CLI dry run, and export.
        if !plan.surface_parity.is_consistent() {
            violations.push(
                GroupedUpdateAndRollbackReviewViolation::SurfaceParityBroken {
                    plan_id: plan.plan_id.clone(),
                },
            );
        }
        // Every plan must link to an existing checkpoint receipt.
        if self.checkpoint(&plan.checkpoint_id).is_none() {
            violations.push(
                GroupedUpdateAndRollbackReviewViolation::DanglingCheckpointRef {
                    plan_id: plan.plan_id.clone(),
                    checkpoint_ref: plan.checkpoint_id.clone(),
                },
            );
        }
    }

    fn validate_checkpoints(&self, violations: &mut Vec<GroupedUpdateAndRollbackReviewViolation>) {
        let mut seen = BTreeSet::new();
        let mut states_seen = BTreeSet::new();
        for checkpoint in &self.checkpoints {
            if !seen.insert(checkpoint.checkpoint_id.clone()) {
                violations.push(GroupedUpdateAndRollbackReviewViolation::DuplicateRowId {
                    row_id: checkpoint.checkpoint_id.clone(),
                    row_kind: "checkpoint",
                });
            }
            states_seen.insert(checkpoint.state);
            for (field, value) in [
                ("checkpoint_id", &checkpoint.checkpoint_id),
                ("plan_id", &checkpoint.plan_id),
                (
                    "lockfile_identity_before",
                    &checkpoint.lockfile_identity_before,
                ),
                (
                    "lockfile_identity_after",
                    &checkpoint.lockfile_identity_after,
                ),
                ("receipt_label", &checkpoint.receipt_label),
            ] {
                if value.trim().is_empty() {
                    violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                        id: checkpoint.checkpoint_id.clone(),
                        field_name: field,
                    });
                }
            }
            if checkpoint.affected_manifests.is_empty() {
                violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                    id: checkpoint.checkpoint_id.clone(),
                    field_name: "affected_manifests",
                });
            }
            // A checkpoint receipt must be durable, never a transient toast.
            if !checkpoint.durable {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::NonDurableCheckpoint {
                        checkpoint_id: checkpoint.checkpoint_id.clone(),
                    },
                );
            }
            // Every receipt must offer revert, open-diff, and export-patch.
            let action_kinds: BTreeSet<RecoveryActionKind> = checkpoint
                .recovery_actions
                .iter()
                .map(|action| action.kind)
                .collect();
            for required in RecoveryActionKind::ALL {
                if !action_kinds.contains(&required) {
                    violations.push(
                        GroupedUpdateAndRollbackReviewViolation::MissingRecoveryAction {
                            checkpoint_id: checkpoint.checkpoint_id.clone(),
                            kind: required.as_str(),
                        },
                    );
                }
            }
            for action in &checkpoint.recovery_actions {
                if action.target_ref.trim().is_empty() {
                    violations.push(GroupedUpdateAndRollbackReviewViolation::EmptyField {
                        id: checkpoint.checkpoint_id.clone(),
                        field_name: "recovery_action.target_ref",
                    });
                }
            }
            // The guarded plan must exist and point back at this checkpoint.
            match self.plan(&checkpoint.plan_id) {
                None => violations.push(GroupedUpdateAndRollbackReviewViolation::DanglingPlanRef {
                    checkpoint_id: checkpoint.checkpoint_id.clone(),
                    plan_ref: checkpoint.plan_id.clone(),
                }),
                Some(plan) => {
                    if plan.checkpoint_id != checkpoint.checkpoint_id {
                        violations.push(
                            GroupedUpdateAndRollbackReviewViolation::CheckpointPlanMismatch {
                                checkpoint_id: checkpoint.checkpoint_id.clone(),
                                plan_id: checkpoint.plan_id.clone(),
                            },
                        );
                    }
                }
            }
        }
        for required in CheckpointState::ALL {
            if !states_seen.contains(&required) {
                violations.push(
                    GroupedUpdateAndRollbackReviewViolation::MissingCorpusState {
                        field: "checkpoint_state",
                        state: required.as_str(),
                    },
                );
            }
        }
    }
}

/// A validation violation for the grouped-update and rollback-review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupedUpdateAndRollbackReviewViolation {
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
    /// A plan carries no package changes.
    MissingPackageChange {
        /// Plan id.
        plan_id: String,
    },
    /// A plan claims a churn class but estimates zero touched lockfile entries.
    ZeroLockfileChurn {
        /// Plan id.
        plan_id: String,
    },
    /// A plan discloses script/native-build risk without a disclosure note.
    ScriptRiskUndisclosed {
        /// Plan id.
        plan_id: String,
    },
    /// A plan recommends no validation pack checks.
    MissingValidationPack {
        /// Plan id.
        plan_id: String,
    },
    /// A plan's review is not mirrored to every claimed surface.
    SurfaceParityBroken {
        /// Plan id.
        plan_id: String,
    },
    /// A plan references a missing checkpoint.
    DanglingCheckpointRef {
        /// Plan id carrying the ref.
        plan_id: String,
        /// Unresolvable checkpoint ref.
        checkpoint_ref: String,
    },
    /// A checkpoint references a missing plan.
    DanglingPlanRef {
        /// Checkpoint id carrying the ref.
        checkpoint_id: String,
        /// Unresolvable plan ref.
        plan_ref: String,
    },
    /// A checkpoint and its plan disagree about the link between them.
    CheckpointPlanMismatch {
        /// Checkpoint id.
        checkpoint_id: String,
        /// Plan id.
        plan_id: String,
    },
    /// A checkpoint receipt is not durable.
    NonDurableCheckpoint {
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
    /// A required corpus state is missing.
    MissingCorpusState {
        /// Field that must exercise the state.
        field: &'static str,
        /// Missing state token.
        state: &'static str,
    },
    /// Summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for GroupedUpdateAndRollbackReviewViolation {
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
            Self::MissingPackageChange { plan_id } => {
                write!(f, "plan {plan_id} carries no package changes")
            }
            Self::ZeroLockfileChurn { plan_id } => write!(
                f,
                "plan {plan_id} claims a churn class but estimates zero lockfile entries"
            ),
            Self::ScriptRiskUndisclosed { plan_id } => write!(
                f,
                "plan {plan_id} discloses script/native-build risk without a note"
            ),
            Self::MissingValidationPack { plan_id } => {
                write!(f, "plan {plan_id} recommends no validation checks")
            }
            Self::SurfaceParityBroken { plan_id } => write!(
                f,
                "plan {plan_id} review is not mirrored to every claimed surface"
            ),
            Self::DanglingCheckpointRef {
                plan_id,
                checkpoint_ref,
            } => write!(
                f,
                "plan {plan_id} references missing checkpoint {checkpoint_ref}"
            ),
            Self::DanglingPlanRef {
                checkpoint_id,
                plan_ref,
            } => write!(
                f,
                "checkpoint {checkpoint_id} references missing plan {plan_ref}"
            ),
            Self::CheckpointPlanMismatch {
                checkpoint_id,
                plan_id,
            } => write!(
                f,
                "checkpoint {checkpoint_id} and plan {plan_id} disagree about their link"
            ),
            Self::NonDurableCheckpoint { checkpoint_id } => {
                write!(f, "checkpoint {checkpoint_id} is not a durable receipt")
            }
            Self::MissingRecoveryAction {
                checkpoint_id,
                kind,
            } => write!(f, "checkpoint {checkpoint_id} lacks {kind} recovery action"),
            Self::MissingCorpusState { field, state } => {
                write!(f, "packet corpus does not exercise {field} state {state}")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for GroupedUpdateAndRollbackReviewViolation {}

/// Loads the embedded grouped-update and rollback-review packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`GroupedUpdateAndRollbackReview`].
pub fn current_grouped_update_and_rollback_review(
) -> Result<GroupedUpdateAndRollbackReview, serde_json::Error> {
    serde_json::from_str(GROUPED_UPDATE_AND_ROLLBACK_REVIEW_JSON)
}

#[cfg(test)]
mod tests;
