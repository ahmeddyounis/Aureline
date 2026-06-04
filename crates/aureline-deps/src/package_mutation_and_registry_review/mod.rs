//! Package-mutation, manifest-scope, registry-auth, and lockfile-review
//! packet for stable dependency-manager surfaces.
//!
//! This module publishes the canonical vocabulary and typed packet that keeps
//! package browser/search, workspace scope, package detail, operation review,
//! registry or mirror auth, grouped update, operation history, CLI/headless,
//! Help, and support-export surfaces aligned. The packet is review-first: it
//! distinguishes requested ranges and sources from resolved exact identities,
//! names the affected manifests and lockfiles, classifies registry credential
//! mode, quantifies lockfile impact, preserves script/native-build risk, and
//! requires validation plus rollback posture before a write-capable operation
//! can be considered stable.
//!
//! The checked-in packet lives at
//! `artifacts/deps/m4/package-mutation-and-registry-review.json` and is
//! embedded here so Rust consumers, support exports, and release evidence all
//! validate against the same source of truth.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported package-mutation review packet schema version.
pub const PACKAGE_MUTATION_AND_REGISTRY_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PACKAGE_MUTATION_AND_REGISTRY_REVIEW_RECORD_KIND: &str =
    "package_mutation_and_registry_review";

/// Repo-relative path to the checked-in packet.
pub const PACKAGE_MUTATION_AND_REGISTRY_REVIEW_PATH: &str =
    "artifacts/deps/m4/package-mutation-and-registry-review.json";

/// Embedded checked-in packet JSON.
pub const PACKAGE_MUTATION_AND_REGISTRY_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m4/package-mutation-and-registry-review.json"
));

/// Ecosystem class for stable package-manager rows.
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

/// Package operation class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationClass {
    /// Adds a direct package requirement.
    Add,
    /// Updates one or more package requirements or resolved versions.
    Update,
    /// Removes a package requirement.
    Remove,
    /// Regenerates or resolves lockfile state without a requested manifest bump.
    Resolve,
}

impl OperationClass {
    /// Every operation class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Add, Self::Update, Self::Remove, Self::Resolve];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Update => "update",
            Self::Remove => "remove",
            Self::Resolve => "resolve",
        }
    }
}

/// Search result state for package browser and detail surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchResultState {
    /// Results were found and are current enough for review.
    ResultsAvailable,
    /// Query completed with no matching packages.
    NoResults,
    /// Registry access requires credentials before results can be trusted.
    AuthRequired,
    /// Mirror data is reachable but stale.
    MirrorStale,
    /// Only an offline snapshot is available.
    OfflineSnapshotOnly,
    /// State is unknown or stale beyond claimable bounds.
    UnknownOrStale,
}

impl SearchResultState {
    /// Every search result state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResultsAvailable,
        Self::NoResults,
        Self::AuthRequired,
        Self::MirrorStale,
        Self::OfflineSnapshotOnly,
        Self::UnknownOrStale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResultsAvailable => "results_available",
            Self::NoResults => "no_results",
            Self::AuthRequired => "auth_required",
            Self::MirrorStale => "mirror_stale",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::UnknownOrStale => "unknown_or_stale",
        }
    }
}

/// Source kind for requested and resolved package identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// Public or private registry package source.
    Registry,
    /// Workspace-local package source.
    WorkspaceLocal,
    /// Filesystem path package source.
    Path,
    /// Version-control package source.
    Vcs,
    /// Policy-pinned source or version.
    PolicyPinned,
    /// Offline snapshot source.
    OfflineSnapshotOnly,
    /// Source could not be determined.
    UnknownOrStale,
}

impl SourceKind {
    /// Every source kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Registry,
        Self::WorkspaceLocal,
        Self::Path,
        Self::Vcs,
        Self::PolicyPinned,
        Self::OfflineSnapshotOnly,
        Self::UnknownOrStale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Registry => "registry",
            Self::WorkspaceLocal => "workspace_local",
            Self::Path => "path",
            Self::Vcs => "vcs",
            Self::PolicyPinned => "policy_pinned",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::UnknownOrStale => "unknown_or_stale",
        }
    }
}

/// Directness or dependency relation class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyRelationClass {
    /// Direct dependency in the target manifest.
    Direct,
    /// Transitive dependency resolved through another package.
    Transitive,
    /// Workspace-local member dependency.
    WorkspaceLocal,
    /// Path dependency.
    Path,
    /// VCS dependency.
    Vcs,
    /// Policy-pinned dependency.
    PolicyPinned,
    /// Offline-snapshot-only dependency.
    OfflineSnapshotOnly,
    /// Dependency state is unknown or stale.
    UnknownOrStale,
}

impl DependencyRelationClass {
    /// Every relation class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Direct,
        Self::Transitive,
        Self::WorkspaceLocal,
        Self::Path,
        Self::Vcs,
        Self::PolicyPinned,
        Self::OfflineSnapshotOnly,
        Self::UnknownOrStale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
            Self::WorkspaceLocal => "workspace_local",
            Self::Path => "path",
            Self::Vcs => "vcs",
            Self::PolicyPinned => "policy_pinned",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::UnknownOrStale => "unknown_or_stale",
        }
    }
}

/// Registry or mirror source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrySourceClass {
    /// Public upstream registry.
    PublicRegistry,
    /// Private registry.
    PrivateRegistry,
    /// Enterprise mirror of an upstream registry.
    EnterpriseMirror,
    /// Local cache.
    LocalCache,
    /// Offline snapshot.
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

/// Credential mode used for registry or mirror access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialMode {
    /// Anonymous registry access.
    Anonymous,
    /// Secret broker resolves credentials from the OS store.
    OsStore,
    /// Token-backed credential.
    Token,
    /// Browser or device-code sign-in.
    BrowserOrDeviceSignIn,
    /// Credential mode inherited from policy.
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

/// Registry freshness state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryFreshnessState {
    /// Registry or mirror metadata is current.
    Current,
    /// Metadata is stale.
    Stale,
    /// Registry access requires authentication.
    AuthRequired,
    /// Only offline snapshot metadata is available.
    OfflineSnapshotOnly,
    /// Metadata freshness is unknown.
    Unknown,
}

impl RegistryFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Stale,
        Self::AuthRequired,
        Self::OfflineSnapshotOnly,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::AuthRequired => "auth_required",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::Unknown => "unknown",
        }
    }
}

/// Registry reachability state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryReachabilityState {
    /// Registry is reachable.
    Reachable,
    /// Registry or mirror is unreachable.
    Unreachable,
    /// Registry requires authentication before reachability can be proven.
    AuthRequired,
    /// Registry is blocked by policy.
    PolicyBlocked,
    /// Offline mode is active.
    Offline,
}

impl RegistryReachabilityState {
    /// Every reachability state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Reachable,
        Self::Unreachable,
        Self::AuthRequired,
        Self::PolicyBlocked,
        Self::Offline,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Unreachable => "unreachable",
            Self::AuthRequired => "auth_required",
            Self::PolicyBlocked => "policy_blocked",
            Self::Offline => "offline",
        }
    }
}

/// Lockfile impact class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileImpactClass {
    /// Direct dependency bump.
    DirectBump,
    /// Security patch.
    SecurityPatch,
    /// Grouped refresh across related packages.
    GroupedRefresh,
    /// Lockfile-only refresh.
    LockfileRefreshOnly,
    /// Major-version pilot.
    MajorVersionPilot,
    /// Workspace-wide convergence operation.
    WorkspaceWideConvergence,
}

impl LockfileImpactClass {
    /// Every lockfile impact class, in declaration order.
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

/// Script and native-build risk class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptNativeBuildRiskClass {
    /// No scripts or native builds are expected.
    NoneKnown,
    /// Lifecycle scripts are expected.
    PackageScripts,
    /// Native build or compiler/toolchain work is expected.
    NativeBuild,
    /// New egress may occur during restore or build.
    NewEgress,
    /// Script or native-build behavior is blocked by policy.
    PolicyBlocked,
    /// Script or native-build state is unknown.
    Unknown,
}

impl ScriptNativeBuildRiskClass {
    /// Every risk class, in declaration order.
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

    /// Whether the risk class is apply blocking.
    pub const fn blocks_apply(self) -> bool {
        matches!(self, Self::PolicyBlocked | Self::Unknown)
    }
}

/// Source that proposed the operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationSourceClass {
    /// User selected the operation manually.
    Manual,
    /// AI suggested the operation.
    AiSuggested,
    /// Automation or recipe suggested the operation.
    AutomationSuggested,
}

impl AutomationSourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 3] = [Self::Manual, Self::AiSuggested, Self::AutomationSuggested];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::AiSuggested => "ai_suggested",
            Self::AutomationSuggested => "automation_suggested",
        }
    }
}

/// Write posture for an operation review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritePosture {
    /// Operation is read-only inspection.
    InspectOnly,
    /// Operation can only proceed through explicit review.
    ReviewRequired,
    /// Operation is blocked by policy, auth, or risk state.
    ApplyBlocked,
    /// Operation was applied after review.
    AppliedAfterReview,
}

impl WritePosture {
    /// Every write posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InspectOnly,
        Self::ReviewRequired,
        Self::ApplyBlocked,
        Self::AppliedAfterReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::ReviewRequired => "review_required",
            Self::ApplyBlocked => "apply_blocked",
            Self::AppliedAfterReview => "applied_after_review",
        }
    }
}

/// Stable surface contract for UI, CLI/headless, docs/help, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableSurfaceContract {
    /// Package browser/search surface label.
    pub package_browser_surface: String,
    /// Workspace scope bar surface label.
    pub workspace_scope_surface: String,
    /// Package detail sheet surface label.
    pub package_detail_surface: String,
    /// Operation review surface label.
    pub operation_review_surface: String,
    /// Registry or mirror auth panel surface label.
    pub registry_auth_surface: String,
    /// Operation history and recovery lane surface label.
    pub history_recovery_surface: String,
    /// CLI/headless command or inspect surface.
    pub cli_headless_surface: String,
    /// Help page path.
    pub help_page: String,
    /// Support export packet path or projection ref.
    pub support_export_surface: String,
}

/// Manifest scope selected for a package operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestScope {
    /// Ecosystem for this scope.
    pub ecosystem: EcosystemClass,
    /// Scope label shown to users.
    pub scope_label: String,
    /// Manifest path that will be modified or inspected.
    pub manifest_path: String,
    /// Workspace member or module label, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_member: Option<String>,
    /// Lockfiles coupled to the manifest scope.
    #[serde(default)]
    pub lockfile_paths: Vec<String>,
    /// Registry or mirror auth panel ref inherited by this scope.
    pub registry_auth_ref: String,
}

/// Requested package identity before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageIdentityRequest {
    /// Package name or coordinate requested by the user or automation.
    pub package_name: String,
    /// Requested range, tag, source ref, or removal target.
    pub requested_range_or_source: String,
    /// Requested source kind.
    pub requested_source_kind: SourceKind,
}

/// Resolved package identity after package-manager resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedPackageIdentity {
    /// Resolved package name or coordinate.
    pub package_name: String,
    /// Exact resolved version, commit, path, or snapshot id.
    pub resolved_exact_version_or_source: String,
    /// Resolved source kind.
    pub resolved_source_kind: SourceKind,
    /// Directness or dependency relation.
    pub relation_class: DependencyRelationClass,
    /// Registry, mirror, path, VCS, or workspace source ref.
    pub source_ref: String,
}

/// Registry or mirror auth panel state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryAuthPanel {
    /// Stable auth panel id.
    pub registry_auth_id: String,
    /// Ecosystem this panel applies to.
    pub ecosystem: EcosystemClass,
    /// Source class.
    pub source_class: RegistrySourceClass,
    /// Credential mode.
    pub credential_mode: CredentialMode,
    /// Freshness state.
    pub freshness_state: RegistryFreshnessState,
    /// Reachability state.
    pub reachability_state: RegistryReachabilityState,
    /// Whether policy locks the registry source or credential mode.
    pub policy_locked: bool,
    /// Scope shown to users.
    pub scope: String,
    /// Redacted source label safe for support exports.
    pub redacted_source_label: String,
    /// Whether raw secrets are included in exports.
    pub raw_secrets_exported: bool,
    /// Reviewer-facing note.
    pub note: String,
}

/// Quantified lockfile impact for an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockfileImpactReview {
    /// Lockfile impact class.
    pub impact_class: LockfileImpactClass,
    /// Affected manifest paths.
    #[serde(default)]
    pub affected_manifests: Vec<String>,
    /// Affected lockfile paths.
    #[serde(default)]
    pub affected_lockfiles: Vec<String>,
    /// Number of direct dependencies changed.
    pub direct_package_changes: u32,
    /// Number of transitive dependencies changed.
    pub transitive_package_changes: u32,
    /// Reviewer-facing blast-radius note.
    pub quantified_note: String,
}

/// Script or native-build risk review for an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptNativeBuildRiskReview {
    /// Risk class.
    pub risk_class: ScriptNativeBuildRiskClass,
    /// Package refs that may introduce script or native-build behavior.
    #[serde(default)]
    pub source_package_refs: Vec<String>,
    /// Toolchains or runtimes required by the risk.
    #[serde(default)]
    pub required_toolchain_refs: Vec<String>,
    /// Egress classes introduced or widened.
    #[serde(default)]
    pub egress_class_refs: Vec<String>,
    /// Whether policy allows this risk.
    pub policy_allows: bool,
    /// Reviewer-facing note.
    pub note: String,
}

/// Validation pack that must run before or after package mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationPack {
    /// Stable validation pack id.
    pub validation_pack_id: String,
    /// Validation commands, checks, or packet refs.
    #[serde(default)]
    pub checks: Vec<String>,
    /// Whether validation is required before stable apply.
    pub required_before_apply: bool,
    /// Whether validation has passed.
    pub passed: bool,
}

/// Rollback checkpoint for a package operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackCheckpoint {
    /// Checkpoint id or VCS ref.
    pub checkpoint_id: String,
    /// Paths covered by rollback.
    #[serde(default)]
    pub covered_paths: Vec<String>,
    /// Whether rollback is available.
    pub rollback_available: bool,
    /// Reviewer-facing note.
    pub note: String,
}

/// One package operation review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationReviewRow {
    /// Stable operation id.
    pub operation_id: String,
    /// Source that proposed the operation.
    pub source_class: AutomationSourceClass,
    /// Operation class.
    pub operation_class: OperationClass,
    /// Write posture.
    pub write_posture: WritePosture,
    /// Package browser/search state.
    pub search_result_state: SearchResultState,
    /// Manifest scope.
    pub manifest_scope: ManifestScope,
    /// Requested identity.
    pub requested: PackageIdentityRequest,
    /// Resolved identity.
    pub resolved: ResolvedPackageIdentity,
    /// Registry auth panel ref.
    pub registry_auth_ref: String,
    /// Lockfile impact review.
    pub lockfile_impact: LockfileImpactReview,
    /// Script and native-build risk review.
    pub script_native_build_risk: ScriptNativeBuildRiskReview,
    /// Peer or runtime constraint shifts.
    #[serde(default)]
    pub peer_or_runtime_constraint_shifts: Vec<String>,
    /// Advisory refs used in the review.
    #[serde(default)]
    pub advisory_refs: Vec<String>,
    /// Grouped-update plan ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_update_ref: Option<String>,
    /// Validation pack.
    pub validation_pack: ValidationPack,
    /// Rollback checkpoint.
    pub rollback_checkpoint: RollbackCheckpoint,
    /// Reviewer-facing note.
    pub note: String,
}

/// Grouped update planner row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupedUpdatePlan {
    /// Stable grouped-update id.
    pub grouped_update_id: String,
    /// Ecosystem for the plan.
    pub ecosystem: EcosystemClass,
    /// Manifest scope label.
    pub scope_label: String,
    /// Operation refs included in the group.
    #[serde(default)]
    pub operation_refs: Vec<String>,
    /// Reason for grouping.
    pub reason: String,
    /// Lockfile impact class.
    pub lockfile_impact_class: LockfileImpactClass,
    /// Validation pack id that gates the group.
    pub validation_pack_id: String,
    /// Checkpoint id for group rollback.
    pub checkpoint_id: String,
}

/// Operation history and recovery lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationHistoryRow {
    /// History row id.
    pub history_id: String,
    /// Operation ref.
    pub operation_ref: String,
    /// UTC timestamp for the history event.
    pub timestamp: String,
    /// Result class shown to users.
    pub result_class: String,
    /// Recovery action exposed to users.
    pub recovery_action: String,
    /// Support-export row ref.
    pub support_export_ref: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageMutationAndRegistryReviewSummary {
    /// Total operation rows.
    pub total_operation_rows: usize,
    /// Review-required operation rows.
    pub review_required_rows: usize,
    /// Apply-blocked operation rows.
    pub apply_blocked_rows: usize,
    /// AI or automation suggested rows.
    pub automation_suggested_rows: usize,
    /// Registry auth panel count.
    pub registry_auth_panel_rows: usize,
    /// Operation rows with non-empty lockfile impact.
    pub lockfile_impact_rows: usize,
    /// Operation rows with script or native-build risk.
    pub script_or_native_build_risk_rows: usize,
    /// Grouped update plan rows.
    pub grouped_update_plan_rows: usize,
    /// Operation history rows.
    pub operation_history_rows: usize,
}

/// Redaction-safe export row projected from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMutationAndRegistryReviewExportRow {
    /// Row id.
    pub row_id: String,
    /// Row kind discriminator.
    pub row_kind: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Manifest scope label.
    pub scope_label: String,
    /// Effective state token.
    pub effective_state: String,
    /// Whether the row blocks stable write claims.
    pub blocks_stable_write: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// Redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMutationAndRegistryReviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<PackageMutationAndRegistryReviewExportRow>,
    /// Whether any row blocks stable write claims.
    pub blocks_stable_write: bool,
}

/// Typed package-mutation and registry-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageMutationAndRegistryReview {
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
    pub surface_contract: StableSurfaceContract,
    /// Closed ecosystem vocabulary.
    pub ecosystem_classes: Vec<EcosystemClass>,
    /// Closed operation vocabulary.
    pub operation_classes: Vec<OperationClass>,
    /// Closed search-state vocabulary.
    pub search_result_states: Vec<SearchResultState>,
    /// Closed source-kind vocabulary.
    pub source_kinds: Vec<SourceKind>,
    /// Closed dependency relation vocabulary.
    pub dependency_relation_classes: Vec<DependencyRelationClass>,
    /// Closed registry source vocabulary.
    pub registry_source_classes: Vec<RegistrySourceClass>,
    /// Closed credential-mode vocabulary.
    pub credential_modes: Vec<CredentialMode>,
    /// Closed registry freshness vocabulary.
    pub registry_freshness_states: Vec<RegistryFreshnessState>,
    /// Closed registry reachability vocabulary.
    pub registry_reachability_states: Vec<RegistryReachabilityState>,
    /// Closed lockfile-impact vocabulary.
    pub lockfile_impact_classes: Vec<LockfileImpactClass>,
    /// Closed script/native-build-risk vocabulary.
    pub script_native_build_risk_classes: Vec<ScriptNativeBuildRiskClass>,
    /// Closed automation-source vocabulary.
    pub automation_source_classes: Vec<AutomationSourceClass>,
    /// Closed write-posture vocabulary.
    pub write_postures: Vec<WritePosture>,
    /// Stable ecosystems claimed by this packet.
    pub claimed_stable_ecosystems: Vec<EcosystemClass>,
    /// Registry auth panel rows.
    #[serde(default)]
    pub registry_auth_panels: Vec<RegistryAuthPanel>,
    /// Package operation review rows.
    #[serde(default)]
    pub operation_reviews: Vec<OperationReviewRow>,
    /// Grouped update plans.
    #[serde(default)]
    pub grouped_update_plans: Vec<GroupedUpdatePlan>,
    /// Operation history and recovery rows.
    #[serde(default)]
    pub operation_history: Vec<OperationHistoryRow>,
    /// Summary counts.
    pub summary: PackageMutationAndRegistryReviewSummary,
}

impl PackageMutationAndRegistryReview {
    /// Returns the registry auth panel for `registry_auth_id`.
    pub fn registry_auth_panel(&self, registry_auth_id: &str) -> Option<&RegistryAuthPanel> {
        self.registry_auth_panels
            .iter()
            .find(|row| row.registry_auth_id == registry_auth_id)
    }

    /// Returns the operation review row for `operation_id`.
    pub fn operation_review(&self, operation_id: &str) -> Option<&OperationReviewRow> {
        self.operation_reviews
            .iter()
            .find(|row| row.operation_id == operation_id)
    }

    /// Returns the grouped update plan for `grouped_update_id`.
    pub fn grouped_update_plan(&self, grouped_update_id: &str) -> Option<&GroupedUpdatePlan> {
        self.grouped_update_plans
            .iter()
            .find(|row| row.grouped_update_id == grouped_update_id)
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> PackageMutationAndRegistryReviewSummary {
        PackageMutationAndRegistryReviewSummary {
            total_operation_rows: self.operation_reviews.len(),
            review_required_rows: self
                .operation_reviews
                .iter()
                .filter(|row| row.write_posture == WritePosture::ReviewRequired)
                .count(),
            apply_blocked_rows: self
                .operation_reviews
                .iter()
                .filter(|row| row.write_posture == WritePosture::ApplyBlocked)
                .count(),
            automation_suggested_rows: self
                .operation_reviews
                .iter()
                .filter(|row| {
                    matches!(
                        row.source_class,
                        AutomationSourceClass::AiSuggested
                            | AutomationSourceClass::AutomationSuggested
                    )
                })
                .count(),
            registry_auth_panel_rows: self.registry_auth_panels.len(),
            lockfile_impact_rows: self
                .operation_reviews
                .iter()
                .filter(|row| {
                    !row.lockfile_impact.affected_manifests.is_empty()
                        || !row.lockfile_impact.affected_lockfiles.is_empty()
                })
                .count(),
            script_or_native_build_risk_rows: self
                .operation_reviews
                .iter()
                .filter(|row| {
                    row.script_native_build_risk.risk_class != ScriptNativeBuildRiskClass::NoneKnown
                })
                .count(),
            grouped_update_plan_rows: self.grouped_update_plans.len(),
            operation_history_rows: self.operation_history.len(),
        }
    }

    /// Whether any row blocks stable write claims.
    pub fn blocks_stable_write(&self) -> bool {
        self.operation_reviews.iter().any(|row| {
            row.write_posture == WritePosture::ApplyBlocked
                || row.script_native_build_risk.risk_class.blocks_apply()
                || !row.rollback_checkpoint.rollback_available
                || (row.validation_pack.required_before_apply && !row.validation_pack.passed)
        })
    }

    /// Produces a redaction-safe export projection for UI, CLI, support, docs,
    /// release, and public proof consumers.
    pub fn export_projection(&self) -> PackageMutationAndRegistryReviewExportProjection {
        let registry_by_id = self
            .registry_auth_panels
            .iter()
            .map(|row| (row.registry_auth_id.as_str(), row))
            .collect::<BTreeMap<_, _>>();
        let mut rows = Vec::new();
        for operation in &self.operation_reviews {
            let auth = registry_by_id.get(operation.registry_auth_ref.as_str());
            let auth_state = auth
                .map(|row| row.freshness_state.as_str())
                .unwrap_or("missing_auth_panel");
            let blocks = operation.write_posture == WritePosture::ApplyBlocked
                || operation.script_native_build_risk.risk_class.blocks_apply()
                || !operation.rollback_checkpoint.rollback_available
                || (operation.validation_pack.required_before_apply
                    && !operation.validation_pack.passed);
            rows.push(PackageMutationAndRegistryReviewExportRow {
                row_id: operation.operation_id.clone(),
                row_kind: "operation_review".to_owned(),
                ecosystem: operation.manifest_scope.ecosystem.as_str().to_owned(),
                scope_label: operation.manifest_scope.scope_label.clone(),
                effective_state: operation.write_posture.as_str().to_owned(),
                blocks_stable_write: blocks,
                summary: format!(
                    "{} {} requested {} resolved {} auth {} lockfile {} rollback {}",
                    operation.operation_class.as_str(),
                    operation.requested.package_name,
                    operation.requested.requested_range_or_source,
                    operation.resolved.resolved_exact_version_or_source,
                    auth_state,
                    operation.lockfile_impact.impact_class.as_str(),
                    operation.rollback_checkpoint.checkpoint_id
                ),
            });
        }
        for auth in &self.registry_auth_panels {
            rows.push(PackageMutationAndRegistryReviewExportRow {
                row_id: auth.registry_auth_id.clone(),
                row_kind: "registry_auth_panel".to_owned(),
                ecosystem: auth.ecosystem.as_str().to_owned(),
                scope_label: auth.scope.clone(),
                effective_state: auth.freshness_state.as_str().to_owned(),
                blocks_stable_write: auth.raw_secrets_exported
                    || auth.reachability_state == RegistryReachabilityState::PolicyBlocked,
                summary: format!(
                    "{} via {} credential {} reachability {}",
                    auth.redacted_source_label,
                    auth.source_class.as_str(),
                    auth.credential_mode.as_str(),
                    auth.reachability_state.as_str()
                ),
            });
        }
        PackageMutationAndRegistryReviewExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            blocks_stable_write: self.blocks_stable_write(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<PackageMutationAndRegistryReviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_registry_auth_panels(&mut violations);
        self.validate_operation_reviews(&mut violations);
        self.validate_grouped_update_plans(&mut violations);
        self.validate_operation_history(&mut violations);
        if self.summary != self.computed_summary() {
            violations.push(PackageMutationAndRegistryReviewViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<PackageMutationAndRegistryReviewViolation>) {
        if self.schema_version != PACKAGE_MUTATION_AND_REGISTRY_REVIEW_SCHEMA_VERSION {
            violations.push(
                PackageMutationAndRegistryReviewViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != PACKAGE_MUTATION_AND_REGISTRY_REVIEW_RECORD_KIND {
            violations.push(
                PackageMutationAndRegistryReviewViolation::UnsupportedRecordKind {
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
                violations.push(PackageMutationAndRegistryReviewViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, value) in [
            (
                "package_browser_surface",
                &self.surface_contract.package_browser_surface,
            ),
            (
                "workspace_scope_surface",
                &self.surface_contract.workspace_scope_surface,
            ),
            (
                "package_detail_surface",
                &self.surface_contract.package_detail_surface,
            ),
            (
                "operation_review_surface",
                &self.surface_contract.operation_review_surface,
            ),
            (
                "registry_auth_surface",
                &self.surface_contract.registry_auth_surface,
            ),
            (
                "history_recovery_surface",
                &self.surface_contract.history_recovery_surface,
            ),
            (
                "cli_headless_surface",
                &self.surface_contract.cli_headless_surface,
            ),
            ("help_page", &self.surface_contract.help_page),
            (
                "support_export_surface",
                &self.surface_contract.support_export_surface,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageMutationAndRegistryReviewViolation::EmptyField {
                    id: "<surface_contract>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.ecosystem_classes != EcosystemClass::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "ecosystem_classes",
                },
            );
        }
        if self.operation_classes != OperationClass::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "operation_classes",
                },
            );
        }
        if self.search_result_states != SearchResultState::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "search_result_states",
                },
            );
        }
        if self.source_kinds != SourceKind::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "source_kinds",
                },
            );
        }
        if self.dependency_relation_classes != DependencyRelationClass::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "dependency_relation_classes",
                },
            );
        }
        if self.registry_source_classes != RegistrySourceClass::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "registry_source_classes",
                },
            );
        }
        if self.credential_modes != CredentialMode::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "credential_modes",
                },
            );
        }
        if self.registry_freshness_states != RegistryFreshnessState::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "registry_freshness_states",
                },
            );
        }
        if self.registry_reachability_states != RegistryReachabilityState::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "registry_reachability_states",
                },
            );
        }
        if self.lockfile_impact_classes != LockfileImpactClass::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "lockfile_impact_classes",
                },
            );
        }
        if self.script_native_build_risk_classes != ScriptNativeBuildRiskClass::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "script_native_build_risk_classes",
                },
            );
        }
        if self.automation_source_classes != AutomationSourceClass::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "automation_source_classes",
                },
            );
        }
        if self.write_postures != WritePosture::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "write_postures",
                },
            );
        }
        if self.claimed_stable_ecosystems != EcosystemClass::ALL.to_vec() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::ClosedVocabularyMismatch {
                    field: "claimed_stable_ecosystems",
                },
            );
        }
    }

    fn validate_registry_auth_panels(
        &self,
        violations: &mut Vec<PackageMutationAndRegistryReviewViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for row in &self.registry_auth_panels {
            if !seen.insert(row.registry_auth_id.clone()) {
                violations.push(PackageMutationAndRegistryReviewViolation::DuplicateRowId {
                    row_id: row.registry_auth_id.clone(),
                    row_kind: "registry_auth_panel",
                });
            }
            for (field, value) in [
                ("registry_auth_id", &row.registry_auth_id),
                ("scope", &row.scope),
                ("redacted_source_label", &row.redacted_source_label),
                ("note", &row.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(PackageMutationAndRegistryReviewViolation::EmptyField {
                        id: row.registry_auth_id.clone(),
                        field_name: field,
                    });
                }
            }
            if row.raw_secrets_exported {
                violations.push(
                    PackageMutationAndRegistryReviewViolation::RawSecretsExported {
                        registry_auth_id: row.registry_auth_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_operation_reviews(
        &self,
        violations: &mut Vec<PackageMutationAndRegistryReviewViolation>,
    ) {
        let mut seen = BTreeSet::new();
        let mut search_states_seen = BTreeSet::new();
        let mut lockfile_classes_seen = BTreeSet::new();
        for row in &self.operation_reviews {
            if !seen.insert(row.operation_id.clone()) {
                violations.push(PackageMutationAndRegistryReviewViolation::DuplicateRowId {
                    row_id: row.operation_id.clone(),
                    row_kind: "operation_review",
                });
            }
            search_states_seen.insert(row.search_result_state);
            lockfile_classes_seen.insert(row.lockfile_impact.impact_class);
            self.validate_operation_review(row, violations);
        }
        for required in [
            SearchResultState::NoResults,
            SearchResultState::AuthRequired,
            SearchResultState::MirrorStale,
            SearchResultState::OfflineSnapshotOnly,
        ] {
            if !search_states_seen.contains(&required) {
                violations.push(
                    PackageMutationAndRegistryReviewViolation::MissingCorpusState {
                        field: "search_result_state",
                        state: required.as_str(),
                    },
                );
            }
        }
        for required in LockfileImpactClass::ALL {
            if !lockfile_classes_seen.contains(&required) {
                violations.push(
                    PackageMutationAndRegistryReviewViolation::MissingCorpusState {
                        field: "lockfile_impact_class",
                        state: required.as_str(),
                    },
                );
            }
        }
    }

    fn validate_operation_review(
        &self,
        row: &OperationReviewRow,
        violations: &mut Vec<PackageMutationAndRegistryReviewViolation>,
    ) {
        for (field, value) in [
            ("operation_id", &row.operation_id),
            ("note", &row.note),
            ("scope_label", &row.manifest_scope.scope_label),
            ("manifest_path", &row.manifest_scope.manifest_path),
            ("package_name", &row.requested.package_name),
            (
                "requested_range_or_source",
                &row.requested.requested_range_or_source,
            ),
            ("resolved_package_name", &row.resolved.package_name),
            (
                "resolved_exact_version_or_source",
                &row.resolved.resolved_exact_version_or_source,
            ),
            ("source_ref", &row.resolved.source_ref),
            ("quantified_note", &row.lockfile_impact.quantified_note),
            ("script_note", &row.script_native_build_risk.note),
            (
                "validation_pack_id",
                &row.validation_pack.validation_pack_id,
            ),
            ("checkpoint_id", &row.rollback_checkpoint.checkpoint_id),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageMutationAndRegistryReviewViolation::EmptyField {
                    id: row.operation_id.clone(),
                    field_name: field,
                });
            }
        }
        if row.manifest_scope.registry_auth_ref != row.registry_auth_ref {
            violations.push(
                PackageMutationAndRegistryReviewViolation::RegistryAuthScopeMismatch {
                    operation_id: row.operation_id.clone(),
                },
            );
        }
        if self.registry_auth_panel(&row.registry_auth_ref).is_none() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::DanglingRegistryAuthRef {
                    operation_id: row.operation_id.clone(),
                    registry_auth_ref: row.registry_auth_ref.clone(),
                },
            );
        }
        if row.lockfile_impact.affected_manifests.is_empty() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::MissingAffectedManifest {
                    operation_id: row.operation_id.clone(),
                },
            );
        }
        if row.lockfile_impact.affected_lockfiles.is_empty() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::MissingLockfileImpact {
                    operation_id: row.operation_id.clone(),
                },
            );
        }
        if row.validation_pack.checks.is_empty() {
            violations.push(
                PackageMutationAndRegistryReviewViolation::MissingValidationPack {
                    operation_id: row.operation_id.clone(),
                },
            );
        }
        if row.rollback_checkpoint.covered_paths.is_empty()
            || !row.rollback_checkpoint.rollback_available
        {
            violations.push(
                PackageMutationAndRegistryReviewViolation::MissingRollbackCheckpoint {
                    operation_id: row.operation_id.clone(),
                },
            );
        }
        if matches!(
            row.source_class,
            AutomationSourceClass::AiSuggested | AutomationSourceClass::AutomationSuggested
        ) && row.write_posture != WritePosture::ReviewRequired
        {
            violations.push(
                PackageMutationAndRegistryReviewViolation::AutomationBypassedReview {
                    operation_id: row.operation_id.clone(),
                },
            );
        }
        if row.script_native_build_risk.risk_class.blocks_apply()
            && row.write_posture != WritePosture::ApplyBlocked
        {
            violations.push(
                PackageMutationAndRegistryReviewViolation::BlockingRiskNotBlocked {
                    operation_id: row.operation_id.clone(),
                },
            );
        }
        if let Some(grouped_ref) = &row.grouped_update_ref {
            if self.grouped_update_plan(grouped_ref).is_none() {
                violations.push(
                    PackageMutationAndRegistryReviewViolation::DanglingGroupedUpdateRef {
                        operation_id: row.operation_id.clone(),
                        grouped_update_ref: grouped_ref.clone(),
                    },
                );
            }
        }
    }

    fn validate_grouped_update_plans(
        &self,
        violations: &mut Vec<PackageMutationAndRegistryReviewViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for row in &self.grouped_update_plans {
            if !seen.insert(row.grouped_update_id.clone()) {
                violations.push(PackageMutationAndRegistryReviewViolation::DuplicateRowId {
                    row_id: row.grouped_update_id.clone(),
                    row_kind: "grouped_update_plan",
                });
            }
            for operation_ref in &row.operation_refs {
                if self.operation_review(operation_ref).is_none() {
                    violations.push(
                        PackageMutationAndRegistryReviewViolation::DanglingOperationRef {
                            row_id: row.grouped_update_id.clone(),
                            operation_ref: operation_ref.clone(),
                        },
                    );
                }
            }
            for (field, value) in [
                ("grouped_update_id", &row.grouped_update_id),
                ("scope_label", &row.scope_label),
                ("reason", &row.reason),
                ("validation_pack_id", &row.validation_pack_id),
                ("checkpoint_id", &row.checkpoint_id),
            ] {
                if value.trim().is_empty() {
                    violations.push(PackageMutationAndRegistryReviewViolation::EmptyField {
                        id: row.grouped_update_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_operation_history(
        &self,
        violations: &mut Vec<PackageMutationAndRegistryReviewViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for row in &self.operation_history {
            if !seen.insert(row.history_id.clone()) {
                violations.push(PackageMutationAndRegistryReviewViolation::DuplicateRowId {
                    row_id: row.history_id.clone(),
                    row_kind: "operation_history",
                });
            }
            if self.operation_review(&row.operation_ref).is_none() {
                violations.push(
                    PackageMutationAndRegistryReviewViolation::DanglingOperationRef {
                        row_id: row.history_id.clone(),
                        operation_ref: row.operation_ref.clone(),
                    },
                );
            }
            for (field, value) in [
                ("history_id", &row.history_id),
                ("operation_ref", &row.operation_ref),
                ("timestamp", &row.timestamp),
                ("result_class", &row.result_class),
                ("recovery_action", &row.recovery_action),
                ("support_export_ref", &row.support_export_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(PackageMutationAndRegistryReviewViolation::EmptyField {
                        id: row.history_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }
}

/// A validation violation for the package-mutation review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageMutationAndRegistryReviewViolation {
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
    /// An operation references a missing registry auth panel.
    DanglingRegistryAuthRef {
        /// Operation id carrying the ref.
        operation_id: String,
        /// Unresolvable registry auth ref.
        registry_auth_ref: String,
    },
    /// A grouped update or history row references a missing operation.
    DanglingOperationRef {
        /// Row id carrying the ref.
        row_id: String,
        /// Unresolvable operation ref.
        operation_ref: String,
    },
    /// An operation references a missing grouped-update plan.
    DanglingGroupedUpdateRef {
        /// Operation id carrying the ref.
        operation_id: String,
        /// Unresolvable grouped-update ref.
        grouped_update_ref: String,
    },
    /// Operation-level and manifest-scope registry auth refs disagree.
    RegistryAuthScopeMismatch {
        /// Operation id.
        operation_id: String,
    },
    /// An operation lacks affected manifest truth.
    MissingAffectedManifest {
        /// Operation id.
        operation_id: String,
    },
    /// An operation lacks affected lockfile truth.
    MissingLockfileImpact {
        /// Operation id.
        operation_id: String,
    },
    /// An operation lacks a validation pack.
    MissingValidationPack {
        /// Operation id.
        operation_id: String,
    },
    /// An operation lacks rollback checkpoint coverage.
    MissingRollbackCheckpoint {
        /// Operation id.
        operation_id: String,
    },
    /// AI or automation bypassed the same review surface as manual actions.
    AutomationBypassedReview {
        /// Operation id.
        operation_id: String,
    },
    /// A blocking script/native-build risk is not reflected as apply-blocked.
    BlockingRiskNotBlocked {
        /// Operation id.
        operation_id: String,
    },
    /// Raw secrets would be included in support or proof exports.
    RawSecretsExported {
        /// Registry auth panel id.
        registry_auth_id: String,
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

impl fmt::Display for PackageMutationAndRegistryReviewViolation {
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
            Self::DanglingRegistryAuthRef {
                operation_id,
                registry_auth_ref,
            } => write!(
                f,
                "operation {operation_id} references missing registry auth panel {registry_auth_ref}"
            ),
            Self::DanglingOperationRef {
                row_id,
                operation_ref,
            } => write!(f, "row {row_id} references missing operation {operation_ref}"),
            Self::DanglingGroupedUpdateRef {
                operation_id,
                grouped_update_ref,
            } => write!(
                f,
                "operation {operation_id} references missing grouped update plan {grouped_update_ref}"
            ),
            Self::RegistryAuthScopeMismatch { operation_id } => write!(
                f,
                "operation {operation_id} registry_auth_ref disagrees with manifest scope"
            ),
            Self::MissingAffectedManifest { operation_id } => {
                write!(f, "operation {operation_id} lacks affected manifest truth")
            }
            Self::MissingLockfileImpact { operation_id } => {
                write!(f, "operation {operation_id} lacks affected lockfile truth")
            }
            Self::MissingValidationPack { operation_id } => {
                write!(f, "operation {operation_id} lacks validation pack checks")
            }
            Self::MissingRollbackCheckpoint { operation_id } => {
                write!(f, "operation {operation_id} lacks rollback checkpoint coverage")
            }
            Self::AutomationBypassedReview { operation_id } => write!(
                f,
                "operation {operation_id} is AI/automation-suggested but did not require review"
            ),
            Self::BlockingRiskNotBlocked { operation_id } => write!(
                f,
                "operation {operation_id} has blocking script/native-build risk without apply block"
            ),
            Self::RawSecretsExported { registry_auth_id } => write!(
                f,
                "registry auth panel {registry_auth_id} would export raw secrets"
            ),
            Self::MissingCorpusState { field, state } => {
                write!(f, "packet corpus does not exercise {field} state {state}")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for PackageMutationAndRegistryReviewViolation {}

/// Loads the embedded package-mutation and registry-review packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`PackageMutationAndRegistryReview`].
pub fn current_package_mutation_and_registry_review(
) -> Result<PackageMutationAndRegistryReview, serde_json::Error> {
    serde_json::from_str(PACKAGE_MUTATION_AND_REGISTRY_REVIEW_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet() -> PackageMutationAndRegistryReview {
        current_package_mutation_and_registry_review().expect("packet parses")
    }

    #[test]
    fn embedded_packet_parses_and_validates() {
        let packet = packet();
        assert_eq!(
            packet.schema_version,
            PACKAGE_MUTATION_AND_REGISTRY_REVIEW_SCHEMA_VERSION
        );
        assert_eq!(
            packet.record_kind,
            PACKAGE_MUTATION_AND_REGISTRY_REVIEW_RECORD_KIND
        );
        assert_eq!(packet.validate(), Vec::new());
    }

    #[test]
    fn requested_and_resolved_identity_stay_separate() {
        let packet = packet();
        assert!(packet.operation_reviews.iter().any(|row| {
            row.requested.requested_range_or_source != row.resolved.resolved_exact_version_or_source
        }));
        assert!(packet.operation_reviews.iter().all(|row| {
            !row.requested.package_name.is_empty()
                && !row.requested.requested_range_or_source.is_empty()
                && !row.resolved.resolved_exact_version_or_source.is_empty()
        }));
    }

    #[test]
    fn every_operation_has_manifest_lockfile_validation_and_rollback() {
        let packet = packet();
        for row in &packet.operation_reviews {
            assert!(!row.lockfile_impact.affected_manifests.is_empty());
            assert!(!row.lockfile_impact.affected_lockfiles.is_empty());
            assert!(!row.validation_pack.checks.is_empty());
            assert!(row.rollback_checkpoint.rollback_available);
            assert!(!row.rollback_checkpoint.covered_paths.is_empty());
        }
    }

    #[test]
    fn auth_states_are_distinct_and_secret_safe() {
        let packet = packet();
        assert!(packet
            .registry_auth_panels
            .iter()
            .all(|row| !row.raw_secrets_exported));
        assert!(packet
            .operation_reviews
            .iter()
            .any(|row| row.search_result_state == SearchResultState::NoResults));
        assert!(packet
            .operation_reviews
            .iter()
            .any(|row| row.search_result_state == SearchResultState::AuthRequired));
        assert!(packet
            .operation_reviews
            .iter()
            .any(|row| row.search_result_state == SearchResultState::MirrorStale));
        assert!(packet
            .operation_reviews
            .iter()
            .any(|row| row.search_result_state == SearchResultState::OfflineSnapshotOnly));
    }

    #[test]
    fn automation_uses_same_review_surface() {
        let packet = packet();
        assert!(packet.operation_reviews.iter().any(|row| {
            row.source_class == AutomationSourceClass::AiSuggested
                && row.write_posture == WritePosture::ReviewRequired
        }));
        assert!(packet.operation_reviews.iter().any(|row| {
            row.source_class == AutomationSourceClass::AutomationSuggested
                && row.write_posture == WritePosture::ReviewRequired
        }));
    }

    #[test]
    fn grouped_updates_and_history_resolve() {
        let packet = packet();
        for plan in &packet.grouped_update_plans {
            assert!(!plan.operation_refs.is_empty());
            for operation_ref in &plan.operation_refs {
                assert!(packet.operation_review(operation_ref).is_some());
            }
        }
        for history in &packet.operation_history {
            assert!(packet.operation_review(&history.operation_ref).is_some());
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let packet = packet();
        assert_eq!(packet.summary, packet.computed_summary());
    }

    #[test]
    fn export_projection_is_redaction_safe_and_complete() {
        let packet = packet();
        let projection = packet.export_projection();
        assert_eq!(
            projection.rows.len(),
            packet.operation_reviews.len() + packet.registry_auth_panels.len()
        );
        assert!(!projection
            .rows
            .iter()
            .any(|row| row.summary.to_lowercase().contains("token:")));
    }
}
