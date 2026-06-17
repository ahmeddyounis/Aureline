//! Durable package-mutation operation history and export-safe receipts.
//!
//! Where [`crate::reviewed_mutation_flows`] reviews a package mutation *before*
//! commit, this module owns what survives *after* it: a durable, inspectable
//! **operation history**. Each [`OperationHistoryEntry`] is a receipt for one
//! completed (or attempted) install, update, remove, or regenerate — the single
//! object the desktop history surface, the CLI/headless history listing, AI and
//! recipe proposal follow-ups, and support/export packets all render.
//!
//! A receipt is never a transient toast or an ecosystem-specific log line. Each
//! one preserves the things support and the operator need to answer *what
//! changed, which chain it affected, and how to revert it*:
//!
//! - the **manifest scope** the operation targeted and the ecosystem it ran in;
//! - the **origin** (desktop, CLI/headless, AI, or recipe) and the **result
//!   class** (applied, no-change, partially applied, rolled back, failed, or
//!   blocked by policy or auth) — a precise result code, never vague prose;
//! - the **manifest and lockfile identity** before and after, as redacted
//!   digests rather than full manifest bodies;
//! - the **resolver state** that produced the resolution;
//! - the **direct-versus-transitive impact chain** — every package the operation
//!   added, removed, upgraded, downgraded, re-pinned, or left unchanged, with its
//!   relation and the parent that pulled a transitive dependency in;
//! - the **validation outcome** recorded for the operation; and
//! - a **rollback handle** with the revert / open-diff / export-patch recovery
//!   actions and the durable checkpoint they reach, plus **evidence refs** back
//!   to the validation, lockfile-diff, and checkpoint proof.
//!
//! Receipts are redaction-default. The model carries no credential bodies,
//! registry tokens, raw provider payloads, private registry URLs, or full
//! manifest content; it stores ids and redacted digests, and a
//! [`RetentionPosture`] proves history retains neither raw secrets nor full
//! manifest bodies by default. Its retention subject binds to the frozen
//! [`crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::RetentionSubject::OperationHistory`]
//! rule, and every package-state label a receipt surfaces resolves to a frozen
//! state row through `references_matrix_id`.
//!
//! The checked-in packet lives at `artifacts/deps/m5/operation-history.json` and
//! is embedded here so Rust consumers, CLI/headless output, support exports, and
//! release evidence all validate against one source of truth.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::{
    current_m5_package_state_matrix, AuthMode, LockfileAuthority, ManifestScopeClass,
    PackageStateLabel, PackageSurface, RegistrySourceAuthority, ResolverIdentityClass, RetentionClass,
    RetentionSubject, RollbackClass, SurfaceWriteAuthority,
};
use crate::package_state_descriptors::{DependencyRelation, EcosystemKind, RequestedSourceKind};

/// Supported operation-history packet schema version.
pub const OPERATION_HISTORY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const OPERATION_HISTORY_RECORD_KIND: &str = "package_operation_history";

/// Repo-relative path to the checked-in packet.
pub const OPERATION_HISTORY_PATH: &str = "artifacts/deps/m5/operation-history.json";

/// Embedded checked-in packet JSON.
pub const OPERATION_HISTORY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/operation-history.json"
));

/// The package-mutation operation a receipt records.
///
/// These four kinds are deliberately distinct so an install, an update, a
/// remove, and a regenerate/resolve never collapse into one generic "change".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    /// Add a new direct package requirement.
    Install,
    /// Update one or more package requirements or resolved versions.
    Update,
    /// Remove a package requirement.
    Remove,
    /// Regenerate or re-resolve lockfile state without a requested range change.
    Regenerate,
}

impl OperationKind {
    /// Every operation kind, in declaration order.
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

    /// Whether this kind re-resolves the dependency set, so it discloses the
    /// resolver it used whenever it produced an impact.
    pub const fn re_resolves(self) -> bool {
        matches!(self, Self::Install | Self::Update | Self::Regenerate)
    }
}

/// The surface or actor that originated an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationOrigin {
    /// A person acting in the desktop workspace.
    DesktopManual,
    /// A CLI/headless invocation.
    CliHeadless,
    /// An AI proposal that was applied through review.
    AiProposal,
    /// A recipe or automation proposal that was applied through review.
    RecipeProposal,
}

impl OperationOrigin {
    /// Every origin, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopManual,
        Self::CliHeadless,
        Self::AiProposal,
        Self::RecipeProposal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopManual => "desktop_manual",
            Self::CliHeadless => "cli_headless",
            Self::AiProposal => "ai_proposal",
            Self::RecipeProposal => "recipe_proposal",
        }
    }

    /// Whether the operation originated from AI or a recipe.
    pub const fn is_automated(self) -> bool {
        matches!(self, Self::AiProposal | Self::RecipeProposal)
    }
}

/// The result class — the precise outcome code a receipt records.
///
/// The seven classes keep applied, no-change, partially applied, rolled back,
/// failed, policy-blocked, and auth-blocked outcomes distinct so a receipt never
/// collapses a failure into a generic "install failed" or a block into "not
/// found".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationResultClass {
    /// The mutation succeeded and changes were written.
    Applied,
    /// The mutation succeeded with nothing to change.
    NoChangeNeeded,
    /// The mutation wrote some changes and left a recovery pending.
    PartiallyApplied,
    /// The mutation was committed and then reverted to its checkpoint.
    RolledBack,
    /// The mutation failed before writing any change.
    FailedNoChange,
    /// Policy blocked the mutation; nothing was written.
    BlockedByPolicy,
    /// Registry authentication was required but unsatisfied; nothing was written.
    BlockedByAuth,
}

impl OperationResultClass {
    /// Every result class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Applied,
        Self::NoChangeNeeded,
        Self::PartiallyApplied,
        Self::RolledBack,
        Self::FailedNoChange,
        Self::BlockedByPolicy,
        Self::BlockedByAuth,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::NoChangeNeeded => "no_change_needed",
            Self::PartiallyApplied => "partially_applied",
            Self::RolledBack => "rolled_back",
            Self::FailedNoChange => "failed_no_change",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::BlockedByAuth => "blocked_by_auth",
        }
    }

    /// Whether this result wrote durable changes to a manifest or lockfile.
    pub const fn wrote_changes(self) -> bool {
        matches!(self, Self::Applied | Self::PartiallyApplied)
    }

    /// Whether the operation was reverted to its checkpoint.
    pub const fn reverted(self) -> bool {
        matches!(self, Self::RolledBack)
    }

    /// Whether this result produced an impact chain — it wrote changes or wrote
    /// then reverted them. Such operations must record a visible chain and a
    /// recoverable rollback handle.
    pub const fn produced_impact(self) -> bool {
        self.wrote_changes() || self.reverted()
    }

    /// Whether this result must leave the manifest and lockfile unchanged.
    pub const fn requires_no_write(self) -> bool {
        matches!(
            self,
            Self::NoChangeNeeded
                | Self::FailedNoChange
                | Self::BlockedByPolicy
                | Self::BlockedByAuth
        )
    }

    /// Whether the operation was blocked rather than attempted to completion.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BlockedByPolicy | Self::BlockedByAuth)
    }
}

/// The validation outcome recorded for an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationResult {
    /// Validation ran and passed cleanly.
    Passed,
    /// Validation ran and passed with non-blocking warnings.
    PassedWithWarnings,
    /// Validation ran and failed.
    Failed,
    /// Validation was deliberately skipped for this operation.
    Skipped,
    /// Validation never ran because the operation was blocked or aborted first.
    NotRun,
}

impl ValidationResult {
    /// Every validation result, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Passed,
        Self::PassedWithWarnings,
        Self::Failed,
        Self::Skipped,
        Self::NotRun,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::PassedWithWarnings => "passed_with_warnings",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::NotRun => "not_run",
        }
    }

    /// Whether a validation decision was recorded at all (anything but
    /// [`ValidationResult::NotRun`]).
    pub const fn recorded(self) -> bool {
        !matches!(self, Self::NotRun)
    }

    /// Whether validation concluded with a pass (clean or with warnings).
    pub const fn passed(self) -> bool {
        matches!(self, Self::Passed | Self::PassedWithWarnings)
    }

    /// Whether validation concluded with a failure.
    pub const fn failed(self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// How one package in the impact chain changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactChangeKind {
    /// The package was newly added to the resolved set.
    Added,
    /// The package was removed from the resolved set.
    Removed,
    /// The package moved to a higher version.
    Upgraded,
    /// The package moved to a lower version.
    Downgraded,
    /// The package was re-pinned to a different exact resolution at a similar range.
    Repinned,
    /// The package's resolution did not change.
    Unchanged,
}

impl ImpactChangeKind {
    /// Every change kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Added,
        Self::Removed,
        Self::Upgraded,
        Self::Downgraded,
        Self::Repinned,
        Self::Unchanged,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Upgraded => "upgraded",
            Self::Downgraded => "downgraded",
            Self::Repinned => "repinned",
            Self::Unchanged => "unchanged",
        }
    }

    /// Whether this change actually moved the resolved set.
    pub const fn changed(self) -> bool {
        !matches!(self, Self::Unchanged)
    }
}

/// The kind of proof an evidence reference points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// A validation run log or report.
    ValidationLog,
    /// A lockfile diff produced by the operation.
    LockfileDiff,
    /// The durable rollback checkpoint.
    RollbackCheckpoint,
    /// A resolver trace recording how the set was resolved.
    ResolverTrace,
    /// A bundled support export referencing this operation.
    SupportBundle,
}

impl EvidenceKind {
    /// Every evidence kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ValidationLog,
        Self::LockfileDiff,
        Self::RollbackCheckpoint,
        Self::ResolverTrace,
        Self::SupportBundle,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidationLog => "validation_log",
            Self::LockfileDiff => "lockfile_diff",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::ResolverTrace => "resolver_trace",
            Self::SupportBundle => "support_bundle",
        }
    }
}

/// A recovery action a rollback handle offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevertActionKind {
    /// Revert the operation back to its checkpoint.
    Revert,
    /// Open the diff the operation produced.
    OpenDiff,
    /// Export the operation as a patch.
    ExportPatch,
}

impl RevertActionKind {
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

/// Stable surface contract: the surfaces that share this receipt model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationHistorySurfaceContract {
    /// Desktop operation-history surface.
    pub desktop_history_surface: String,
    /// CLI/headless history-listing surface.
    pub cli_history_surface: String,
    /// AI/recipe proposal follow-up surface.
    pub ai_recipe_surface: String,
    /// Rollback / recovery surface.
    pub rollback_surface: String,
    /// Help page describing the packet.
    pub help_page: String,
    /// Support-export channel.
    pub support_export_surface: String,
}

/// Which surfaces a receipt is mirrored to.
///
/// Operation history must read the same across desktop, the CLI/headless
/// listing, and support/export artifacts; every flag must stay `true`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistorySurfaceParity {
    /// Rendered on the desktop history surface.
    pub desktop: bool,
    /// Reproduced by the CLI/headless history listing.
    pub cli_headless: bool,
    /// Reproduced in support/export artifacts.
    pub support_export: bool,
}

impl HistorySurfaceParity {
    /// Whether the receipt is mirrored to every claimed surface.
    pub const fn is_consistent(&self) -> bool {
        self.desktop && self.cli_headless && self.support_export
    }
}

/// The manifest scope an operation targeted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestScopeRecord {
    /// Frozen manifest-scope class.
    pub scope_class: ManifestScopeClass,
    /// Ecosystem for this scope.
    pub ecosystem: EcosystemKind,
    /// Scope label shown to users.
    pub scope_label: String,
    /// Redacted manifest path; never a raw URL.
    pub redacted_manifest_path: String,
    /// Durable ids of every manifest the operation touched.
    pub affected_manifest_ids: Vec<String>,
}

/// The manifest and lockfile identity before and after an operation.
///
/// History stores redacted digests and ids, never full manifest bodies, so a
/// receipt can prove *which* manifest and lockfile moved without retaining their
/// content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestLockfileIdentity {
    /// Lockfile authority governing the resolved set.
    pub lockfile_authority: LockfileAuthority,
    /// Durable ids of every lockfile the operation touched.
    #[serde(default)]
    pub affected_lockfile_ids: Vec<String>,
    /// Redacted lockfile identity (digest/id) before the operation.
    pub lockfile_identity_before: String,
    /// Redacted lockfile identity (digest/id) after the operation.
    pub lockfile_identity_after: String,
    /// Redacted manifest digest before the operation; never the manifest body.
    pub manifest_digest_before: String,
    /// Redacted manifest digest after the operation; never the manifest body.
    pub manifest_digest_after: String,
}

impl ManifestLockfileIdentity {
    /// Whether the manifest or lockfile identity moved.
    pub fn identity_changed(&self) -> bool {
        self.lockfile_identity_before != self.lockfile_identity_after
            || self.manifest_digest_before != self.manifest_digest_after
    }
}

/// The requested package identity, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestedOperationIdentity {
    /// Ecosystem the request targeted.
    pub ecosystem: EcosystemKind,
    /// Package name or coordinate requested.
    pub package_name: String,
    /// Requested range, tag, source ref, or removal target; redacted.
    pub requested_ref: String,
    /// Requested source kind.
    pub requested_source: RequestedSourceKind,
    /// Whether the request was a policy-pinned constraint.
    #[serde(default)]
    pub policy_pinned: bool,
}

/// The resolver identity and version that produced the resolved set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolverState {
    /// Frozen resolver-identity class.
    pub resolver_class: ResolverIdentityClass,
    /// Resolver version string recorded in the receipt.
    pub resolver_version: String,
}

/// Where an operation resolved a package from, and the auth posture it used.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationRegistrySource {
    /// Registry or mirror source authority.
    pub source_class: RegistrySourceAuthority,
    /// Auth mode used to reach the source.
    pub auth_mode: AuthMode,
    /// Redacted source label safe for support exports; never a raw URL or token.
    pub redacted_source_label: String,
}

impl OperationRegistrySource {
    /// Whether trust was blocked because auth was required but unsatisfied.
    pub const fn trust_blocked(&self) -> bool {
        self.auth_mode.blocks_until_satisfied()
    }
}

/// One node in the direct-versus-transitive impact chain.
///
/// A direct (or workspace-local / path-VCS) link sits at depth zero with no
/// parent; a transitive link sits at depth one or more and names the parents
/// that pulled it in, so the chain reads as a graph rather than a flat list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImpactChainLink {
    /// Stable link id, unique within the receipt's chain.
    pub link_id: String,
    /// Package name; redaction-safe, never a raw URL.
    pub package_name: String,
    /// Dependency relation of the package to the target manifest.
    pub relation: DependencyRelation,
    /// How the package changed.
    pub change_kind: ImpactChangeKind,
    /// Depth in the dependency chain; zero for a direct relation.
    pub depth: u32,
    /// Link ids of the parents that pulled a transitive package in.
    #[serde(default)]
    pub parent_link_ids: Vec<String>,
    /// Redacted version before the change; absent when the package was added.
    #[serde(default)]
    pub version_before: Option<String>,
    /// Redacted version after the change; absent when the package was removed.
    #[serde(default)]
    pub version_after: Option<String>,
    /// Registry or mirror source the package resolved from.
    pub registry_source: RegistrySourceAuthority,
}

impl ImpactChainLink {
    /// Whether this link is a direct (depth-zero) relation.
    pub const fn is_direct(&self) -> bool {
        !matches!(self.relation, DependencyRelation::Transitive)
    }

    /// Whether this link is a transitive relation.
    pub const fn is_transitive(&self) -> bool {
        matches!(self.relation, DependencyRelation::Transitive)
    }

    /// Whether the link's topology and version movement are internally
    /// consistent with its relation and change kind.
    pub fn is_consistent(&self) -> bool {
        let topology_ok = if self.is_transitive() {
            self.depth >= 1 && !self.parent_link_ids.is_empty()
        } else {
            self.depth == 0 && self.parent_link_ids.is_empty()
        };
        let version_ok = match self.change_kind {
            ImpactChangeKind::Added => {
                self.version_before.is_none() && self.version_after.is_some()
            }
            ImpactChangeKind::Removed => {
                self.version_before.is_some() && self.version_after.is_none()
            }
            ImpactChangeKind::Upgraded
            | ImpactChangeKind::Downgraded
            | ImpactChangeKind::Repinned => {
                self.version_before.is_some() && self.version_after.is_some()
            }
            ImpactChangeKind::Unchanged => match (&self.version_before, &self.version_after) {
                (Some(before), Some(after)) => before == after,
                _ => false,
            },
        };
        topology_ok
            && version_ok
            && !self.link_id.trim().is_empty()
            && !self.package_name.trim().is_empty()
    }
}

/// The validation outcome recorded for an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationOutcomeRecord {
    /// The validation result.
    pub result: ValidationResult,
    /// The checks that ran (build, test, audit, …); may be empty when skipped.
    #[serde(default)]
    pub checks_run: Vec<String>,
    /// Redacted reference to the validation evidence; never a raw URL.
    pub redacted_evidence_ref: String,
    /// Concrete, non-vague summary of the outcome.
    pub summary: String,
}

/// A recovery action a rollback handle offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevertAction {
    /// Recovery action kind.
    pub kind: RevertActionKind,
    /// Redaction-safe target reference.
    pub redacted_target_ref: String,
}

/// The rollback handle a receipt carries — how to revert the operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackHandle {
    /// Rollback class.
    pub rollback_class: RollbackClass,
    /// Redacted reference to the durable checkpoint; empty when not applicable.
    #[serde(default)]
    pub checkpoint_ref: String,
    /// Whether a revert is still available against the checkpoint.
    pub revert_available: bool,
    /// Revert / open-diff / export-patch recovery actions.
    #[serde(default)]
    pub actions: Vec<RevertAction>,
    /// Human-readable rollback note.
    pub note: String,
}

impl RollbackHandle {
    /// Whether the rollback class is a real recovery path rather than no path.
    pub const fn is_recoverable(&self) -> bool {
        matches!(
            self.rollback_class,
            RollbackClass::ReversibleCheckpointed
                | RollbackClass::ReversibleManifestOnly
                | RollbackClass::CompensatingOnly
        )
    }

    /// Whether the handle offers revert, open-diff, and export-patch.
    pub fn offers_all_actions(&self) -> bool {
        let kinds: BTreeSet<RevertActionKind> = self.actions.iter().map(|a| a.kind).collect();
        RevertActionKind::ALL.iter().all(|k| kinds.contains(k))
    }

    /// Whether the handle is a durable, complete, recoverable path back to a
    /// checkpoint.
    pub fn is_durable_recovery(&self) -> bool {
        self.is_recoverable() && self.offers_all_actions() && !self.checkpoint_ref.trim().is_empty()
    }
}

/// A redacted reference to operation evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationEvidenceRef {
    /// Evidence kind.
    pub kind: EvidenceKind,
    /// Redacted reference; never a raw URL, token, or secret body.
    pub redacted_ref: String,
    /// Human-readable evidence label.
    pub label: String,
}

/// The privacy and retention posture of a receipt.
///
/// History is bounded-local by default and retains neither raw credentials nor
/// full manifest bodies; both flags must stay `false`, and the subject and class
/// bind to the frozen retention rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionPosture {
    /// Retention subject; must be [`RetentionSubject::OperationHistory`].
    pub subject: RetentionSubject,
    /// Retention class; must equal [`RetentionSubject::canonical_retention_class`].
    pub retention_class: RetentionClass,
    /// Whether full manifest bodies are retained; must be `false`.
    pub full_manifest_body_retained: bool,
    /// Whether raw credential material is retained; must be `false`.
    pub raw_credentials_retained: bool,
}

impl RetentionPosture {
    /// Whether the posture is the bounded-local, redaction-default posture
    /// history requires.
    pub fn is_consistent(&self) -> bool {
        self.subject == RetentionSubject::OperationHistory
            && self.retention_class == self.subject.canonical_retention_class()
            && !self.full_manifest_body_retained
            && !self.raw_credentials_retained
    }
}

/// One operation-history entry — the durable receipt for a single mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationHistoryEntry {
    /// Stable operation id.
    pub operation_id: String,
    /// Operation kind.
    pub operation_kind: OperationKind,
    /// Origin surface or actor.
    pub origin: OperationOrigin,
    /// Result class.
    pub result_class: OperationResultClass,
    /// Human-readable, concrete entry label.
    pub entry_label: String,
    /// UTC date the operation ran (date only; redaction-safe).
    pub occurred_on: String,
    /// Manifest scope targeted.
    pub scope: ManifestScopeRecord,
    /// Requested identity.
    pub requested: RequestedOperationIdentity,
    /// Manifest/lockfile identity before and after.
    pub identity: ManifestLockfileIdentity,
    /// Resolver state.
    pub resolver: ResolverState,
    /// Registry/auth source.
    pub registry_source: OperationRegistrySource,
    /// Direct-versus-transitive impact chain.
    #[serde(default)]
    pub impact_chain: Vec<ImpactChainLink>,
    /// Validation outcome.
    pub validation: ValidationOutcomeRecord,
    /// Rollback handle.
    pub rollback: RollbackHandle,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<OperationEvidenceRef>,
    /// Retention posture.
    pub retention: RetentionPosture,
    /// Surface parity across desktop, CLI, and support/export.
    pub surface_parity: HistorySurfaceParity,
    /// Frozen package-state labels this receipt surfaces; each binds to the matrix.
    #[serde(default)]
    pub applicable_labels: Vec<PackageStateLabel>,
    /// Reviewer-facing note.
    pub note: String,
}

impl OperationHistoryEntry {
    /// Whether this operation produced an impact (wrote changes or reverted).
    pub const fn produced_impact(&self) -> bool {
        self.result_class.produced_impact()
    }

    /// Count of direct (depth-zero) links in the chain.
    pub fn direct_link_count(&self) -> usize {
        self.impact_chain.iter().filter(|l| l.is_direct()).count()
    }

    /// Count of transitive links in the chain.
    pub fn transitive_link_count(&self) -> usize {
        self.impact_chain
            .iter()
            .filter(|l| l.is_transitive())
            .count()
    }

    /// Whether the chain shows at least one changed link and at least one direct
    /// link — what a produced-impact receipt must always surface so the operator
    /// can see what changed and where it entered.
    pub fn has_visible_changed_chain(&self) -> bool {
        self.impact_chain.iter().any(|l| l.change_kind.changed())
            && self.impact_chain.iter().any(ImpactChainLink::is_direct)
    }

    /// Whether the receipt discloses everything support needs: a concrete label,
    /// the manifest scope, a resolver, the impact chain (when one was produced),
    /// a recorded validation outcome (when one was produced), and a rollback note.
    pub fn discloses_all_required(&self) -> bool {
        !self.entry_label.trim().is_empty()
            && !self.scope.scope_label.trim().is_empty()
            && !self.scope.affected_manifest_ids.is_empty()
            && !self.resolver.resolver_version.trim().is_empty()
            && !self.validation.summary.trim().is_empty()
            && !self.rollback.note.trim().is_empty()
            && (!self.produced_impact() || self.has_visible_changed_chain())
            && (!self.produced_impact() || self.validation.result.recorded())
    }

    /// Whether the rollback handle agrees with the result class.
    ///
    /// An operation that wrote changes must leave a durable, revertible recovery
    /// path; a rolled-back operation keeps the checkpoint and actions but may
    /// have already spent its revert; a no-write or blocked operation carries no
    /// rollback.
    pub fn rollback_consistent(&self) -> bool {
        match self.result_class {
            OperationResultClass::Applied | OperationResultClass::PartiallyApplied => {
                self.rollback.is_durable_recovery() && self.rollback.revert_available
            }
            OperationResultClass::RolledBack => self.rollback.is_durable_recovery(),
            OperationResultClass::NoChangeNeeded
            | OperationResultClass::FailedNoChange
            | OperationResultClass::BlockedByPolicy
            | OperationResultClass::BlockedByAuth => {
                self.rollback.rollback_class == RollbackClass::NotApplicable
                    && !self.rollback.revert_available
                    && self.rollback.actions.is_empty()
            }
        }
    }

    /// Whether the recorded identity movement agrees with the result class.
    ///
    /// A write moves the manifest or lockfile identity; a rollback returns it to
    /// where it started; a no-write leaves it untouched.
    pub fn identity_consistent(&self) -> bool {
        match self.result_class {
            OperationResultClass::Applied | OperationResultClass::PartiallyApplied => {
                self.identity.identity_changed()
            }
            OperationResultClass::RolledBack
            | OperationResultClass::NoChangeNeeded
            | OperationResultClass::FailedNoChange
            | OperationResultClass::BlockedByPolicy
            | OperationResultClass::BlockedByAuth => !self.identity.identity_changed(),
        }
    }

    /// Whether the validation outcome is compatible with the result class.
    ///
    /// An applied operation cannot carry a failed validation, and a rolled-back
    /// operation reverted because validation failed.
    pub fn validation_consistent(&self) -> bool {
        match self.result_class {
            OperationResultClass::Applied => !self.validation.result.failed(),
            OperationResultClass::RolledBack => self.validation.result.failed(),
            _ => true,
        }
    }
}

/// A redaction-safe view of one receipt, projected for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationHistoryEntrySurfaceProjection {
    /// Operation id.
    pub operation_id: String,
    /// Surface token.
    pub surface: String,
    /// Write authority this surface carries.
    pub write_authority: String,
    /// Whether this surface may revert the operation from here.
    pub can_revert_here: bool,
    /// Whether the projection is redacted for export.
    pub redacted: bool,
    /// Whether the direct-versus-transitive impact chain stays visible here.
    pub impact_chain_visible: bool,
    /// Direct link count surfaced.
    pub direct_links: usize,
    /// Transitive link count surfaced.
    pub transitive_links: usize,
}

/// Summary counts derived from the entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationHistorySummary {
    /// Total operations.
    pub total_operations: usize,
    /// Install operations.
    pub install_operations: usize,
    /// Update operations.
    pub update_operations: usize,
    /// Remove operations.
    pub remove_operations: usize,
    /// Regenerate operations.
    pub regenerate_operations: usize,
    /// Operations that applied changes cleanly.
    pub applied_operations: usize,
    /// Operations that found nothing to change.
    pub no_change_operations: usize,
    /// Operations that partially applied and left a recovery pending.
    pub partially_applied_operations: usize,
    /// Operations that were rolled back.
    pub rolled_back_operations: usize,
    /// Operations that failed before writing.
    pub failed_operations: usize,
    /// Operations blocked by policy.
    pub policy_blocked_operations: usize,
    /// Operations blocked by unsatisfied auth.
    pub auth_blocked_operations: usize,
    /// Operations carrying a recoverable rollback handle.
    pub recoverable_operations: usize,
    /// Total direct impact links across all chains.
    pub direct_impact_links: usize,
    /// Total transitive impact links across all chains.
    pub transitive_impact_links: usize,
    /// Operations whose validation failed.
    pub validation_failed_operations: usize,
}

/// One row of the redaction-safe export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationHistoryExportRow {
    /// Operation id.
    pub operation_id: String,
    /// Operation kind token.
    pub operation_kind: String,
    /// Origin token.
    pub origin: String,
    /// Result class token.
    pub result_class: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Entry label.
    pub label: String,
    /// Direct link count.
    pub direct_links: usize,
    /// Transitive link count.
    pub transitive_links: usize,
    /// Rollback class token.
    pub rollback_class: String,
    /// Whether the operation is recoverable.
    pub recoverable: bool,
    /// Validation result token.
    pub validation: String,
    /// Concrete, non-vague summary.
    pub summary: String,
}

/// Redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationHistoryExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<OperationHistoryExportRow>,
    /// Whether every receipt binds to the frozen matrix.
    pub all_bind_matrix: bool,
    /// Whether every produced-impact receipt keeps its chain visible.
    pub all_chains_visible: bool,
    /// Whether every receipt is redaction-safe (no full manifest body or raw secret).
    pub all_redaction_safe: bool,
}

/// Typed operation-history packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageOperationHistory {
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
    pub surface_contract: OperationHistorySurfaceContract,
    /// Closed operation-kind vocabulary.
    pub operation_kinds: Vec<OperationKind>,
    /// Closed origin vocabulary.
    pub operation_origins: Vec<OperationOrigin>,
    /// Closed result-class vocabulary.
    pub result_classes: Vec<OperationResultClass>,
    /// Closed validation-result vocabulary.
    pub validation_results: Vec<ValidationResult>,
    /// Closed impact-change-kind vocabulary.
    pub impact_change_kinds: Vec<ImpactChangeKind>,
    /// Closed evidence-kind vocabulary.
    pub evidence_kinds: Vec<EvidenceKind>,
    /// Closed revert-action-kind vocabulary.
    pub revert_action_kinds: Vec<RevertActionKind>,
    /// Retention subject this lane is governed by; pinned to operation history.
    pub retention_subject: RetentionSubject,
    /// Operation-history entries.
    #[serde(default)]
    pub entries: Vec<OperationHistoryEntry>,
    /// Summary counts.
    pub summary: OperationHistorySummary,
}

impl PackageOperationHistory {
    /// Returns the entry for `operation_id`.
    pub fn entry(&self, operation_id: &str) -> Option<&OperationHistoryEntry> {
        self.entries
            .iter()
            .find(|row| row.operation_id == operation_id)
    }

    /// Whether the packet binds to the matrix and every label resolves in it.
    pub fn all_bind_matrix(&self) -> bool {
        let Ok(matrix) = current_m5_package_state_matrix() else {
            return false;
        };
        if self.references_matrix_id != matrix.packet_id {
            return false;
        }
        self.entries
            .iter()
            .flat_map(|e| e.applicable_labels.iter())
            .all(|label| matrix.state(*label).is_some())
    }

    /// Whether every produced-impact receipt keeps a visible changed chain.
    pub fn all_chains_visible(&self) -> bool {
        self.entries
            .iter()
            .all(|e| !e.produced_impact() || e.has_visible_changed_chain())
    }

    /// Whether every receipt is redaction-safe and discloses what support needs.
    pub fn all_redaction_safe(&self) -> bool {
        self.entries.iter().all(|e| e.retention.is_consistent())
    }

    /// Projects a receipt for a marketed surface with the write authority that
    /// surface may carry, pinned from the frozen matrix. The impact chain stays
    /// visible on every surface, including redacted support exports.
    pub fn surface_projection(
        &self,
        operation_id: &str,
        surface: PackageSurface,
    ) -> Option<OperationHistoryEntrySurfaceProjection> {
        let entry = self.entry(operation_id)?;
        let authority = surface.canonical_write_authority();
        Some(OperationHistoryEntrySurfaceProjection {
            operation_id: entry.operation_id.clone(),
            surface: surface.as_str().to_owned(),
            write_authority: authority.as_str().to_owned(),
            can_revert_here: authority.can_mutate()
                && entry.rollback.revert_available
                && entry.rollback.is_recoverable(),
            redacted: matches!(authority, SurfaceWriteAuthority::RedactedExport),
            impact_chain_visible: true,
            direct_links: entry.direct_link_count(),
            transitive_links: entry.transitive_link_count(),
        })
    }

    /// Recomputes the summary block from the entries.
    pub fn computed_summary(&self) -> OperationHistorySummary {
        let kind_count = |kind: OperationKind| {
            self.entries
                .iter()
                .filter(|e| e.operation_kind == kind)
                .count()
        };
        let result_count = |result: OperationResultClass| {
            self.entries
                .iter()
                .filter(|e| e.result_class == result)
                .count()
        };
        OperationHistorySummary {
            total_operations: self.entries.len(),
            install_operations: kind_count(OperationKind::Install),
            update_operations: kind_count(OperationKind::Update),
            remove_operations: kind_count(OperationKind::Remove),
            regenerate_operations: kind_count(OperationKind::Regenerate),
            applied_operations: result_count(OperationResultClass::Applied),
            no_change_operations: result_count(OperationResultClass::NoChangeNeeded),
            partially_applied_operations: result_count(OperationResultClass::PartiallyApplied),
            rolled_back_operations: result_count(OperationResultClass::RolledBack),
            failed_operations: result_count(OperationResultClass::FailedNoChange),
            policy_blocked_operations: result_count(OperationResultClass::BlockedByPolicy),
            auth_blocked_operations: result_count(OperationResultClass::BlockedByAuth),
            recoverable_operations: self
                .entries
                .iter()
                .filter(|e| e.rollback.is_recoverable())
                .count(),
            direct_impact_links: self.entries.iter().map(|e| e.direct_link_count()).sum(),
            transitive_impact_links: self.entries.iter().map(|e| e.transitive_link_count()).sum(),
            validation_failed_operations: self
                .entries
                .iter()
                .filter(|e| e.validation.result.failed())
                .count(),
        }
    }

    /// Produces a redaction-safe export projection for UI, CLI, support, docs,
    /// release, and public-proof consumers.
    pub fn export_projection(&self) -> OperationHistoryExportProjection {
        let rows = self
            .entries
            .iter()
            .map(|entry| OperationHistoryExportRow {
                operation_id: entry.operation_id.clone(),
                operation_kind: entry.operation_kind.as_str().to_owned(),
                origin: entry.origin.as_str().to_owned(),
                result_class: entry.result_class.as_str().to_owned(),
                ecosystem: entry.scope.ecosystem.as_str().to_owned(),
                label: entry.entry_label.clone(),
                direct_links: entry.direct_link_count(),
                transitive_links: entry.transitive_link_count(),
                rollback_class: entry.rollback.rollback_class.as_str().to_owned(),
                recoverable: entry.rollback.is_recoverable(),
                validation: entry.validation.result.as_str().to_owned(),
                summary: format!(
                    "{} {} result {} scope {} lockfile {} direct {} transitive {} rollback {}",
                    entry.operation_kind.as_str(),
                    entry.requested.package_name,
                    entry.result_class.as_str(),
                    entry.scope.scope_class.as_str(),
                    entry.identity.lockfile_authority.as_str(),
                    entry.direct_link_count(),
                    entry.transitive_link_count(),
                    entry.rollback.rollback_class.as_str(),
                ),
            })
            .collect();
        OperationHistoryExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_bind_matrix: self.all_bind_matrix(),
            all_chains_visible: self.all_chains_visible(),
            all_redaction_safe: self.all_redaction_safe(),
        }
    }

    /// Returns the corpus-coverage gaps: any operation kind, origin, result
    /// class, validation result, impact change kind, or evidence kind the
    /// entries do not exercise.
    pub fn corpus_coverage_gaps(&self) -> Vec<PackageOperationHistoryViolation> {
        let mut gaps = Vec::new();
        let kinds: BTreeSet<OperationKind> =
            self.entries.iter().map(|e| e.operation_kind).collect();
        for required in OperationKind::ALL {
            if !kinds.contains(&required) {
                gaps.push(PackageOperationHistoryViolation::MissingCorpusState {
                    field: "operation_kind",
                    state: required.as_str(),
                });
            }
        }
        let origins: BTreeSet<OperationOrigin> = self.entries.iter().map(|e| e.origin).collect();
        for required in OperationOrigin::ALL {
            if !origins.contains(&required) {
                gaps.push(PackageOperationHistoryViolation::MissingCorpusState {
                    field: "operation_origin",
                    state: required.as_str(),
                });
            }
        }
        let results: BTreeSet<OperationResultClass> =
            self.entries.iter().map(|e| e.result_class).collect();
        for required in OperationResultClass::ALL {
            if !results.contains(&required) {
                gaps.push(PackageOperationHistoryViolation::MissingCorpusState {
                    field: "result_class",
                    state: required.as_str(),
                });
            }
        }
        let validations: BTreeSet<ValidationResult> =
            self.entries.iter().map(|e| e.validation.result).collect();
        for required in ValidationResult::ALL {
            if !validations.contains(&required) {
                gaps.push(PackageOperationHistoryViolation::MissingCorpusState {
                    field: "validation_result",
                    state: required.as_str(),
                });
            }
        }
        let changes: BTreeSet<ImpactChangeKind> = self
            .entries
            .iter()
            .flat_map(|e| e.impact_chain.iter().map(|l| l.change_kind))
            .collect();
        for required in ImpactChangeKind::ALL {
            if !changes.contains(&required) {
                gaps.push(PackageOperationHistoryViolation::MissingCorpusState {
                    field: "impact_change_kind",
                    state: required.as_str(),
                });
            }
        }
        let evidence: BTreeSet<EvidenceKind> = self
            .entries
            .iter()
            .flat_map(|e| e.evidence_refs.iter().map(|r| r.kind))
            .collect();
        for required in EvidenceKind::ALL {
            if !evidence.contains(&required) {
                gaps.push(PackageOperationHistoryViolation::MissingCorpusState {
                    field: "evidence_kind",
                    state: required.as_str(),
                });
            }
        }
        gaps
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<PackageOperationHistoryViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_entries(&mut violations);
        if self.summary != self.computed_summary() {
            violations.push(PackageOperationHistoryViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<PackageOperationHistoryViolation>) {
        if self.schema_version != OPERATION_HISTORY_SCHEMA_VERSION {
            violations.push(PackageOperationHistoryViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != OPERATION_HISTORY_RECORD_KIND {
            violations.push(PackageOperationHistoryViolation::UnsupportedRecordKind {
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
                violations.push(PackageOperationHistoryViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, value) in [
            (
                "desktop_history_surface",
                &self.surface_contract.desktop_history_surface,
            ),
            (
                "cli_history_surface",
                &self.surface_contract.cli_history_surface,
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
                violations.push(PackageOperationHistoryViolation::EmptyField {
                    id: "<surface_contract>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab_checks: [(&'static str, bool); 7] = [
            (
                "operation_kinds",
                self.operation_kinds == OperationKind::ALL.to_vec(),
            ),
            (
                "operation_origins",
                self.operation_origins == OperationOrigin::ALL.to_vec(),
            ),
            (
                "result_classes",
                self.result_classes == OperationResultClass::ALL.to_vec(),
            ),
            (
                "validation_results",
                self.validation_results == ValidationResult::ALL.to_vec(),
            ),
            (
                "impact_change_kinds",
                self.impact_change_kinds == ImpactChangeKind::ALL.to_vec(),
            ),
            (
                "evidence_kinds",
                self.evidence_kinds == EvidenceKind::ALL.to_vec(),
            ),
            (
                "revert_action_kinds",
                self.revert_action_kinds == RevertActionKind::ALL.to_vec(),
            ),
        ];
        for (field, ok) in vocab_checks {
            if !ok {
                violations
                    .push(PackageOperationHistoryViolation::ClosedVocabularyMismatch { field });
            }
        }
        if self.retention_subject != RetentionSubject::OperationHistory {
            violations.push(PackageOperationHistoryViolation::RetentionSubjectMismatch {
                actual: self.retention_subject.as_str(),
            });
        }
        if let Ok(matrix) = current_m5_package_state_matrix() {
            if self.references_matrix_id != matrix.packet_id {
                violations.push(PackageOperationHistoryViolation::MatrixBindingMismatch {
                    referenced: self.references_matrix_id.clone(),
                });
            }
        }
    }

    fn validate_entries(&self, violations: &mut Vec<PackageOperationHistoryViolation>) {
        let matrix = current_m5_package_state_matrix().ok();
        let mut seen = BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.operation_id.clone()) {
                violations.push(PackageOperationHistoryViolation::DuplicateRowId {
                    row_id: entry.operation_id.clone(),
                });
            }
            self.validate_entry(entry, matrix.as_ref(), violations);
        }
    }

    fn validate_entry(
        &self,
        entry: &OperationHistoryEntry,
        matrix: Option<&crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::M5PackageStateMatrix>,
        violations: &mut Vec<PackageOperationHistoryViolation>,
    ) {
        for (field, value) in [
            ("operation_id", &entry.operation_id),
            ("entry_label", &entry.entry_label),
            ("occurred_on", &entry.occurred_on),
            ("scope_label", &entry.scope.scope_label),
            (
                "redacted_manifest_path",
                &entry.scope.redacted_manifest_path,
            ),
            ("package_name", &entry.requested.package_name),
            ("requested_ref", &entry.requested.requested_ref),
            (
                "lockfile_identity_before",
                &entry.identity.lockfile_identity_before,
            ),
            (
                "lockfile_identity_after",
                &entry.identity.lockfile_identity_after,
            ),
            (
                "manifest_digest_before",
                &entry.identity.manifest_digest_before,
            ),
            (
                "manifest_digest_after",
                &entry.identity.manifest_digest_after,
            ),
            ("resolver_version", &entry.resolver.resolver_version),
            (
                "redacted_source_label",
                &entry.registry_source.redacted_source_label,
            ),
            (
                "validation_evidence_ref",
                &entry.validation.redacted_evidence_ref,
            ),
            ("validation_summary", &entry.validation.summary),
            ("rollback_note", &entry.rollback.note),
            ("note", &entry.note),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageOperationHistoryViolation::EmptyField {
                    id: entry.operation_id.clone(),
                    field_name: field,
                });
            }
        }
        if entry.scope.affected_manifest_ids.is_empty() {
            violations.push(PackageOperationHistoryViolation::EmptyField {
                id: entry.operation_id.clone(),
                field_name: "affected_manifest_ids",
            });
        }

        // Redaction: no field may leak a raw URL.
        for (field, value) in [
            (
                "redacted_manifest_path",
                &entry.scope.redacted_manifest_path,
            ),
            ("requested_ref", &entry.requested.requested_ref),
            (
                "redacted_source_label",
                &entry.registry_source.redacted_source_label,
            ),
            (
                "lockfile_identity_before",
                &entry.identity.lockfile_identity_before,
            ),
            (
                "lockfile_identity_after",
                &entry.identity.lockfile_identity_after,
            ),
            (
                "manifest_digest_before",
                &entry.identity.manifest_digest_before,
            ),
            (
                "manifest_digest_after",
                &entry.identity.manifest_digest_after,
            ),
            (
                "validation_evidence_ref",
                &entry.validation.redacted_evidence_ref,
            ),
            ("checkpoint_ref", &entry.rollback.checkpoint_ref),
        ] {
            if value.contains("://") {
                violations.push(PackageOperationHistoryViolation::RawUrlLeak {
                    id: entry.operation_id.clone(),
                    field_name: field,
                });
            }
        }
        for link in &entry.impact_chain {
            for value in [
                &link.package_name,
                link.version_before.as_deref().unwrap_or(""),
                link.version_after.as_deref().unwrap_or(""),
            ] {
                if value.contains("://") {
                    violations.push(PackageOperationHistoryViolation::RawUrlLeak {
                        id: entry.operation_id.clone(),
                        field_name: "impact_chain_link",
                    });
                }
            }
        }
        for action in &entry.rollback.actions {
            if action.redacted_target_ref.contains("://") {
                violations.push(PackageOperationHistoryViolation::RawUrlLeak {
                    id: entry.operation_id.clone(),
                    field_name: "rollback_action_target",
                });
            }
            if action.redacted_target_ref.trim().is_empty() {
                violations.push(PackageOperationHistoryViolation::EmptyField {
                    id: entry.operation_id.clone(),
                    field_name: "rollback_action_target",
                });
            }
        }
        for evidence in &entry.evidence_refs {
            if evidence.redacted_ref.contains("://") {
                violations.push(PackageOperationHistoryViolation::RawUrlLeak {
                    id: entry.operation_id.clone(),
                    field_name: "evidence_ref",
                });
            }
            if evidence.redacted_ref.trim().is_empty() || evidence.label.trim().is_empty() {
                violations.push(PackageOperationHistoryViolation::EmptyField {
                    id: entry.operation_id.clone(),
                    field_name: "evidence_ref",
                });
            }
        }

        // Impact-chain topology and version movement must be consistent, and
        // every transitive parent must resolve within the chain.
        let link_ids: BTreeSet<&str> = entry
            .impact_chain
            .iter()
            .map(|l| l.link_id.as_str())
            .collect();
        let mut seen_links = BTreeSet::new();
        for link in &entry.impact_chain {
            if !seen_links.insert(link.link_id.as_str()) {
                violations.push(PackageOperationHistoryViolation::DuplicateImpactLink {
                    operation_id: entry.operation_id.clone(),
                    link_id: link.link_id.clone(),
                });
            }
            if !link.is_consistent() {
                violations.push(
                    PackageOperationHistoryViolation::ImpactChainLinkInconsistent {
                        operation_id: entry.operation_id.clone(),
                        link_id: link.link_id.clone(),
                    },
                );
            }
            for parent in &link.parent_link_ids {
                if !link_ids.contains(parent.as_str()) {
                    violations.push(PackageOperationHistoryViolation::DanglingParentLink {
                        operation_id: entry.operation_id.clone(),
                        link_id: link.link_id.clone(),
                        parent_ref: parent.clone(),
                    });
                }
            }
        }

        // Chain visibility: a produced-impact receipt must show a changed chain;
        // a no-write receipt must not claim any change.
        if entry.produced_impact() {
            if !entry.has_visible_changed_chain() {
                violations.push(PackageOperationHistoryViolation::MissingImpactChain {
                    operation_id: entry.operation_id.clone(),
                });
            }
        } else if entry.impact_chain.iter().any(|l| l.change_kind.changed()) {
            violations.push(
                PackageOperationHistoryViolation::UnexpectedImpactOnNoWrite {
                    operation_id: entry.operation_id.clone(),
                },
            );
        }

        // Identity movement, rollback handle, and validation must each agree with
        // the result class.
        if !entry.identity_consistent() {
            violations.push(PackageOperationHistoryViolation::IdentityResultMismatch {
                operation_id: entry.operation_id.clone(),
            });
        }
        if !entry.rollback_consistent() {
            violations.push(PackageOperationHistoryViolation::RollbackInconsistent {
                operation_id: entry.operation_id.clone(),
            });
        }
        if entry.produced_impact() && !entry.validation.result.recorded() {
            violations.push(PackageOperationHistoryViolation::ValidationNotRecorded {
                operation_id: entry.operation_id.clone(),
            });
        }
        if !entry.validation_consistent() {
            violations.push(
                PackageOperationHistoryViolation::ValidationContradictsResult {
                    operation_id: entry.operation_id.clone(),
                },
            );
        }

        // Auth-blocked receipts must carry an auth-blocked source posture.
        if entry.result_class == OperationResultClass::BlockedByAuth
            && !entry.registry_source.trust_blocked()
        {
            violations.push(PackageOperationHistoryViolation::AuthBlockSourceMismatch {
                operation_id: entry.operation_id.clone(),
            });
        }

        // Retention: bounded-local, no full manifest bodies, no raw credentials.
        if entry.retention.subject != RetentionSubject::OperationHistory
            || entry.retention.retention_class
                != entry.retention.subject.canonical_retention_class()
        {
            violations.push(PackageOperationHistoryViolation::RetentionInconsistent {
                operation_id: entry.operation_id.clone(),
            });
        }
        if entry.retention.full_manifest_body_retained {
            violations.push(PackageOperationHistoryViolation::FullManifestBodyRetained {
                operation_id: entry.operation_id.clone(),
            });
        }
        if entry.retention.raw_credentials_retained {
            violations.push(PackageOperationHistoryViolation::RawCredentialRetained {
                operation_id: entry.operation_id.clone(),
            });
        }

        // Surface parity must hold across desktop, CLI, and export.
        if !entry.surface_parity.is_consistent() {
            violations.push(PackageOperationHistoryViolation::SurfaceParityBroken {
                operation_id: entry.operation_id.clone(),
            });
        }

        // Every surfaced label must resolve in the frozen matrix.
        if let Some(matrix) = matrix {
            for label in &entry.applicable_labels {
                if matrix.state(*label).is_none() {
                    violations.push(PackageOperationHistoryViolation::UnboundLabel {
                        operation_id: entry.operation_id.clone(),
                        label: label.as_str(),
                    });
                }
            }
        }
    }
}

/// A validation violation for the operation-history packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageOperationHistoryViolation {
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
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// An impact-chain link id appears more than once within a receipt.
    DuplicateImpactLink {
        /// Operation id.
        operation_id: String,
        /// Duplicate link id.
        link_id: String,
    },
    /// A required corpus state is missing.
    MissingCorpusState {
        /// Field that must exercise the state.
        field: &'static str,
        /// Missing state token.
        state: &'static str,
    },
    /// An impact-chain link's topology or version movement is inconsistent.
    ImpactChainLinkInconsistent {
        /// Operation id.
        operation_id: String,
        /// Link id.
        link_id: String,
    },
    /// A transitive link names a parent that is not in the chain.
    DanglingParentLink {
        /// Operation id.
        operation_id: String,
        /// Link id carrying the ref.
        link_id: String,
        /// Unresolvable parent link id.
        parent_ref: String,
    },
    /// A produced-impact receipt does not surface a visible changed chain.
    MissingImpactChain {
        /// Operation id.
        operation_id: String,
    },
    /// A no-write receipt claims an impact change.
    UnexpectedImpactOnNoWrite {
        /// Operation id.
        operation_id: String,
    },
    /// The recorded identity movement disagrees with the result class.
    IdentityResultMismatch {
        /// Operation id.
        operation_id: String,
    },
    /// The rollback handle disagrees with the result class.
    RollbackInconsistent {
        /// Operation id.
        operation_id: String,
    },
    /// A produced-impact receipt records no validation outcome.
    ValidationNotRecorded {
        /// Operation id.
        operation_id: String,
    },
    /// The validation outcome contradicts the result class.
    ValidationContradictsResult {
        /// Operation id.
        operation_id: String,
    },
    /// An auth-blocked receipt does not carry an auth-blocked source posture.
    AuthBlockSourceMismatch {
        /// Operation id.
        operation_id: String,
    },
    /// A receipt's retention subject or class is not the operation-history rule.
    RetentionInconsistent {
        /// Operation id.
        operation_id: String,
    },
    /// A receipt retains a full manifest body.
    FullManifestBodyRetained {
        /// Operation id.
        operation_id: String,
    },
    /// A receipt retains raw credential material.
    RawCredentialRetained {
        /// Operation id.
        operation_id: String,
    },
    /// A receipt's surface parity is broken.
    SurfaceParityBroken {
        /// Operation id.
        operation_id: String,
    },
    /// A surfaced label does not resolve in the frozen matrix.
    UnboundLabel {
        /// Operation id.
        operation_id: String,
        /// Unbound label token.
        label: &'static str,
    },
    /// A redacted field leaks a raw URL.
    RawUrlLeak {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// The packet's retention subject is not operation history.
    RetentionSubjectMismatch {
        /// Subject found in the packet.
        actual: &'static str,
    },
    /// The packet binds to a matrix id other than the frozen matrix.
    MatrixBindingMismatch {
        /// Referenced matrix id.
        referenced: String,
    },
    /// Summary counts disagree with the entries.
    SummaryMismatch,
}

impl fmt::Display for PackageOperationHistoryViolation {
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
            Self::DuplicateRowId { row_id } => write!(f, "duplicate operation row id {row_id}"),
            Self::DuplicateImpactLink {
                operation_id,
                link_id,
            } => write!(
                f,
                "operation {operation_id} duplicates impact link {link_id}"
            ),
            Self::MissingCorpusState { field, state } => {
                write!(f, "packet corpus does not exercise {field} state {state}")
            }
            Self::ImpactChainLinkInconsistent {
                operation_id,
                link_id,
            } => write!(
                f,
                "operation {operation_id} impact link {link_id} is inconsistent"
            ),
            Self::DanglingParentLink {
                operation_id,
                link_id,
                parent_ref,
            } => write!(
                f,
                "operation {operation_id} link {link_id} references missing parent {parent_ref}"
            ),
            Self::MissingImpactChain { operation_id } => write!(
                f,
                "operation {operation_id} produced an impact but surfaces no changed chain"
            ),
            Self::UnexpectedImpactOnNoWrite { operation_id } => write!(
                f,
                "operation {operation_id} claims an impact change without writing"
            ),
            Self::IdentityResultMismatch { operation_id } => write!(
                f,
                "operation {operation_id} manifest/lockfile identity disagrees with its result"
            ),
            Self::RollbackInconsistent { operation_id } => write!(
                f,
                "operation {operation_id} rollback handle disagrees with its result"
            ),
            Self::ValidationNotRecorded { operation_id } => write!(
                f,
                "operation {operation_id} produced an impact without recording validation"
            ),
            Self::ValidationContradictsResult { operation_id } => write!(
                f,
                "operation {operation_id} validation outcome contradicts its result"
            ),
            Self::AuthBlockSourceMismatch { operation_id } => write!(
                f,
                "operation {operation_id} is auth-blocked without an auth-blocked source"
            ),
            Self::RetentionInconsistent { operation_id } => write!(
                f,
                "operation {operation_id} retention is not the bounded-local history rule"
            ),
            Self::FullManifestBodyRetained { operation_id } => {
                write!(f, "operation {operation_id} retains a full manifest body")
            }
            Self::RawCredentialRetained { operation_id } => {
                write!(
                    f,
                    "operation {operation_id} retains raw credential material"
                )
            }
            Self::SurfaceParityBroken { operation_id } => write!(
                f,
                "operation {operation_id} history is not mirrored to every claimed surface"
            ),
            Self::UnboundLabel {
                operation_id,
                label,
            } => write!(f, "operation {operation_id} surfaces unbound label {label}"),
            Self::RawUrlLeak { id, field_name } => {
                write!(f, "{id} field {field_name} leaks a raw URL")
            }
            Self::RetentionSubjectMismatch { actual } => {
                write!(
                    f,
                    "packet retention_subject {actual} is not operation_history"
                )
            }
            Self::MatrixBindingMismatch { referenced } => {
                write!(f, "packet binds to non-frozen matrix {referenced}")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the entries"),
        }
    }
}

impl Error for PackageOperationHistoryViolation {}

/// Loads the embedded operation-history packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`PackageOperationHistory`].
pub fn current_package_operation_history() -> Result<PackageOperationHistory, serde_json::Error> {
    serde_json::from_str(OPERATION_HISTORY_JSON)
}

#[cfg(test)]
mod tests;
