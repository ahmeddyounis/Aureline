//! Boundary records for the four repo-topology beta descriptors.
//!
//! Every descriptor mirrors the matching schema in
//! `schemas/workspace/`:
//!
//! - [`RepoRootDescriptor`] — `repo_root_descriptor.schema.json`
//! - [`FetchDepthDescriptor`] — `fetch_depth_descriptor.schema.json`
//! - [`SubmoduleLink`] — `submodule_link.schema.json`
//! - [`LfsHydrationDescriptor`] — `lfs_hydration_descriptor.schema.json`
//!
//! The records are intentionally narrow: they carry only opaque refs and
//! typed labels. Raw absolute paths, credentials, raw remote URLs, raw
//! file bodies, and raw object bytes never appear.

use serde::{Deserialize, Serialize};

use super::shared::{
    ClientScope, FixtureMetadata, FreshnessClass, RedactionClass, RepoTopologyClass,
    TopologyAffordanceClass,
};

pub const REPO_ROOT_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;
pub const FETCH_DEPTH_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;
pub const SUBMODULE_LINK_SCHEMA_VERSION: u32 = 1;
pub const LFS_HYDRATION_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

pub const REPO_ROOT_DESCRIPTOR_RECORD_KIND: &str = "repo_root_descriptor_record";
pub const FETCH_DEPTH_DESCRIPTOR_RECORD_KIND: &str = "fetch_depth_descriptor_record";
pub const SUBMODULE_LINK_RECORD_KIND: &str = "submodule_link_record";
pub const LFS_HYDRATION_DESCRIPTOR_RECORD_KIND: &str = "lfs_hydration_descriptor_record";

// ----------------------------------------------------------------------
// RepoRootDescriptor
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoRootDescriptorRecordKind {
    RepoRootDescriptorRecord,
}

/// Root-kind vocabulary the M03-222 spec freezes for every claimed beta
/// large-repo row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoRootKind {
    Primary,
    Nested,
    Submodule,
    Worktree,
}

impl RepoRootKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Nested => "nested",
            Self::Submodule => "submodule",
            Self::Worktree => "worktree",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VcsProviderClass {
    GitLocal,
    GitRemoteManaged,
    GitRemoteUnmanaged,
    NonGitVcs,
    NoVcs,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteRoleClass {
    PrimaryOrigin,
    PromisorRemote,
    ReviewRemote,
    PublishRemote,
    MirrorRemote,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReachabilityClass {
    Reachable,
    WarmCachedOnly,
    Unreachable,
    PolicyBlocked,
    NotAttempted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustClass {
    TrustedLocal,
    TrustedManaged,
    TrustedWithReview,
    UntrustedPendingReview,
    PolicyRestricted,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyClass {
    NoPolicyActive,
    PolicyObserved,
    PolicyRestrictedMutation,
    PolicyBlockedFetch,
    PolicyBlockedDeepen,
    PolicyBlockedInit,
    PolicyBlockedHydrate,
    PolicyBlockedExport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessStateClass {
    FullyPresentLocalTruth,
    SparseOrWorksetNarrowed,
    ShallowHistoryPresent,
    PartialClonePromisorPresent,
    SubmoduleUninitialized,
    NestedIndependentBoundary,
    LfsPointerOnlyPresent,
    LfsPartiallyHydrated,
    UnavailableUnknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParentLinkageClass {
    NoParentRoot,
    WorksetMember,
    SparseProjection,
    SubmoduleChild,
    NestedIndependentChild,
    WorktreeSharedObjectStoreChild,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorktreeKindClass {
    MainRepositoryCheckout,
    LinkedSecondaryCheckout,
    ManagedRemoteCheckout,
    ImportedSnapshotViewOnly,
    SyntheticRecoveryViewOnly,
    NotAWorktree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportSurfaceClass {
    SearchExport,
    NavigationDeepLink,
    BlameView,
    ReviewPack,
    AiEvidencePacket,
    ExecutionContext,
    PublishPack,
    SupportBundle,
    MigrationPacket,
    ProjectDoctor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconstructionField {
    RepoIdentity,
    WorktreeIdentity,
    ParentLink,
    RemoteSummary,
    TrustPosture,
    PolicyPosture,
    CompletenessState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPosture {
    MetadataOnly,
    LabelsAndCounts,
    ByReference,
    BlockedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoIdentity {
    pub repo_root_ref: String,
    pub repo_identity_ref: String,
    pub vcs_provider_class: VcsProviderClass,
    pub vcs_provider_ref: Option<String>,
    pub object_store_ref: Option<String>,
    pub default_branch_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorktreeIdentity {
    pub worktree_id_ref: Option<String>,
    pub worktree_kind_class: WorktreeKindClass,
    pub head_revision_id_ref: Option<String>,
    pub branch_or_detached_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentLink {
    pub linkage_class: ParentLinkageClass,
    pub parent_root_ref: Option<String>,
    pub link_path_ref: Option<String>,
    pub pinned_commit_ref: Option<String>,
    pub submodule_link_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteSummaryEntry {
    pub remote_ref: String,
    pub remote_role: RemoteRoleClass,
    pub reachability: ReachabilityClass,
    pub last_observed_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteSummary {
    pub remotes: Vec<RemoteSummaryEntry>,
    pub promisor_remote_ref: Option<String>,
    pub publish_remote_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustPosture {
    pub trust_class: TrustClass,
    pub review_required: bool,
    pub review_ticket_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPosture {
    pub policy_class: PolicyClass,
    pub policy_ref: Option<String>,
    #[serde(default)]
    pub policy_blocks: Vec<TopologyAffordanceClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletenessState {
    pub completeness_class: CompletenessStateClass,
    pub fetch_depth_descriptor_ref: Option<String>,
    #[serde(default)]
    pub submodule_link_refs: Vec<String>,
    pub lfs_hydration_descriptor_ref: Option<String>,
    pub may_claim_full_coverage: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoTopologyExportSupportRequirements {
    pub packet_surfaces: Vec<ExportSurfaceClass>,
    pub reconstruction_fields: Vec<ReconstructionField>,
    pub redaction_posture: RedactionPosture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoRootDescriptor {
    #[serde(rename = "__fixture__", default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<FixtureMetadata>,
    pub record_kind: RepoRootDescriptorRecordKind,
    pub repo_root_descriptor_schema_version: u32,
    pub repo_root_descriptor_id: String,
    pub repo_topology_state_ref: String,
    pub workspace_id_ref: String,
    pub root_kind: RepoRootKind,
    pub topology_classes: Vec<RepoTopologyClass>,
    pub repo_identity: RepoIdentity,
    pub worktree_identity: WorktreeIdentity,
    pub parent_link: ParentLink,
    pub remote_summary: RemoteSummary,
    pub trust_posture: TrustPosture,
    pub policy_posture: PolicyPosture,
    pub completeness_state: CompletenessState,
    pub supported_affordances: Vec<TopologyAffordanceClass>,
    pub export_support_requirements: RepoTopologyExportSupportRequirements,
    pub freshness_class: FreshnessClass,
    pub client_scopes: Vec<ClientScope>,
    pub redaction_class: RedactionClass,
    pub created_at: String,
    pub updated_at: String,
}

// ----------------------------------------------------------------------
// FetchDepthDescriptor
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FetchDepthDescriptorRecordKind {
    FetchDepthDescriptorRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryDepthState {
    FullHistoryAvailable,
    ShallowBoundaryPresent,
    HistoryUnavailable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepenPolicyClass {
    DeepenOnDemandAllowed,
    DeepenRequiresApproval,
    DeepenBlockedPolicy,
    OfflineCacheOnly,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialCloneFilterClass {
    None,
    BlobNone,
    BlobLimit,
    TreeDepth,
    SparseOid,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromisorClass {
    NotPromisorFullClone,
    PromisorRemoteConfigured,
    PartialCloneFilterActive,
    PromisorUnreachable,
    FetchForbiddenByPolicy,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FetchPolicyClass {
    FetchOnDemandAllowed,
    FetchRequiresApproval,
    FetchBlockedPolicy,
    OfflineCacheOnly,
    ManualFetchOnly,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FetchDenialReason {
    ShallowBoundary,
    NotFetched,
    PolicyBlocked,
    PromisorUnreachable,
    OfflineCacheOnly,
    ManualFetchRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryDepth {
    pub depth_state: HistoryDepthState,
    pub depth_commits: Option<u32>,
    pub boundary_commit_ref: Option<String>,
    pub deepen_policy_class: DeepenPolicyClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialCloneFilter {
    pub filter_class: PartialCloneFilterClass,
    pub missing_object_count_known: bool,
    pub missing_object_count: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromisorState {
    pub promisor_class: PromisorClass,
    pub promisor_remote_ref: Option<String>,
    pub promisor_reachability: ReachabilityClass,
    pub last_fetch_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchPosture {
    pub fetch_policy_class: FetchPolicyClass,
    pub blocks_full_coverage_claim: bool,
    pub denial_reason: Option<FetchDenialReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchDepthDescriptor {
    #[serde(rename = "__fixture__", default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<FixtureMetadata>,
    pub record_kind: FetchDepthDescriptorRecordKind,
    pub fetch_depth_descriptor_schema_version: u32,
    pub fetch_depth_descriptor_id: String,
    pub repo_root_descriptor_ref: String,
    pub repo_topology_state_ref: String,
    pub history_depth: HistoryDepth,
    pub partial_clone_filter: PartialCloneFilter,
    pub promisor_state: PromisorState,
    pub fetch_posture: FetchPosture,
    pub allowed_affordances: Vec<TopologyAffordanceClass>,
    pub freshness_class: FreshnessClass,
    pub client_scopes: Vec<ClientScope>,
    pub redaction_class: RedactionClass,
    pub created_at: String,
    pub updated_at: String,
}

impl FetchDepthDescriptor {
    /// Returns true when this descriptor's depth or promisor state blocks
    /// a "full local coverage" claim for the bound root.
    pub fn blocks_full_coverage(&self) -> bool {
        self.fetch_posture.blocks_full_coverage_claim
            || !matches!(
                self.history_depth.depth_state,
                HistoryDepthState::FullHistoryAvailable
            )
            || !matches!(
                self.promisor_state.promisor_class,
                PromisorClass::NotPromisorFullClone
            )
    }
}

// ----------------------------------------------------------------------
// SubmoduleLink
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmoduleLinkRecordKind {
    SubmoduleLinkRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinnedByClass {
    SubmoduleGitlink,
    ReviewBundleBase,
    ManagedSnapshot,
    SupportReplay,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InitClass {
    Initialized,
    NotInitialized,
    InitFailed,
    InitBlockedPolicy,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InitPolicyClass {
    InitOnDemandAllowed,
    InitRequiresApproval,
    InitBlockedPolicy,
    OfflineCacheOnly,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChildDirtyClass {
    Clean,
    Dirty,
    DirtyUnknownUninitialized,
    NotAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftClass {
    ChildAtPinnedCommit,
    ChildHeadAheadOfPin,
    ChildHeadBehindPin,
    ChildDivergedFromPin,
    DriftUnknownUninitialized,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParentMutationPostureClass {
    ParentOwnsGitlinkOnly,
    ChildContentReadOnlyUntilInitialized,
    ChildRootMutationOnly,
    ParentMutationDeniedWrongTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmoduleDenialReason {
    WrongTargetRoot,
    SubmoduleNotInitialized,
    PolicyBlocked,
    DriftUnresolved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmodulePinnedCommit {
    pub commit_ref: String,
    pub pinned_by_class: PinnedByClass,
    pub pin_freshness_class: FreshnessClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitState {
    pub init_class: InitClass,
    pub init_policy_class: InitPolicyClass,
    pub last_init_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildDirtyState {
    pub dirty_class: ChildDirtyClass,
    pub dirty_summary_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftState {
    pub drift_class: DriftClass,
    pub observed_child_commit_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentMutationPosture {
    pub posture_class: ParentMutationPostureClass,
    pub denial_reason: Option<SubmoduleDenialReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmoduleLink {
    #[serde(rename = "__fixture__", default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<FixtureMetadata>,
    pub record_kind: SubmoduleLinkRecordKind,
    pub submodule_link_schema_version: u32,
    pub submodule_link_id: String,
    pub parent_repo_root_descriptor_ref: String,
    pub child_repo_root_descriptor_ref: Option<String>,
    pub repo_topology_state_ref: String,
    pub link_path_ref: String,
    pub pinned_commit: SubmodulePinnedCommit,
    pub init_state: InitState,
    pub child_dirty_state: ChildDirtyState,
    pub drift_state: DriftState,
    pub allowed_affordances: Vec<TopologyAffordanceClass>,
    pub parent_mutation_posture: ParentMutationPosture,
    pub freshness_class: FreshnessClass,
    pub client_scopes: Vec<ClientScope>,
    pub redaction_class: RedactionClass,
    pub created_at: String,
    pub updated_at: String,
}

impl SubmoduleLink {
    /// True when the link still externalizes its child content. Workspace,
    /// review, blame, AI, and publish surfaces use this to decide whether
    /// to render submodule content as gitlink-only.
    pub fn child_content_externalized(&self) -> bool {
        !matches!(self.init_state.init_class, InitClass::Initialized)
    }
}

// ----------------------------------------------------------------------
// LfsHydrationDescriptor
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfsHydrationDescriptorRecordKind {
    LfsHydrationDescriptorRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HydrationSummaryClass {
    NoLfsObjectsSeen,
    LfsPointerOnlyPresent,
    LfsPartiallyHydrated,
    LfsFullyHydrated,
    HydrationBlockedOrUnknown,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    PointerOnlyKnownObjects,
    HydratedObjects,
    HydrationPendingObjects,
    HydrationFailedObjects,
    HydrationBlockedObjects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SizeBand {
    Unknown,
    Small,
    Medium,
    Large,
    VeryLarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfsHydratePolicyClass {
    HydrateOnDemandAllowed,
    HydrateRequiresApproval,
    HydrateBlockedPolicy,
    OfflineCacheOnly,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfsLockPostureClass {
    NotLocked,
    LockedByCurrentActor,
    LockedByOtherActor,
    LockStateUnavailable,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkCostBand {
    None,
    Low,
    Medium,
    High,
    Network,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditTargetClass {
    PointerOnly,
    HydratedAsset,
    EditDeniedUntilHydrated,
    EditDeniedPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewTargetClass {
    PointerMetadataOnly,
    HydratedAsset,
    PreviewUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportBodyClass {
    PointerMetadataOnly,
    HydratedAsset,
    ExportBlockedPolicy,
    ExportUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfsPreviewExportDenial {
    PointerOnly,
    PolicyBlocked,
    LockHeldByOtherActor,
    HydrationFailed,
    OfflineCacheOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetBucket {
    pub asset_class: AssetClass,
    pub object_count_known: bool,
    pub object_count: Option<u32>,
    pub size_band: SizeBand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HydratePosture {
    pub hydrate_policy_class: LfsHydratePolicyClass,
    pub lock_posture_class: LfsLockPostureClass,
    pub network_cost_band: NetworkCostBand,
    pub last_hydration_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewExportPosture {
    pub edit_target_class: EditTargetClass,
    pub preview_target_class: PreviewTargetClass,
    pub export_body_class: ExportBodyClass,
    pub denial_reason: Option<LfsPreviewExportDenial>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LfsHydrationDescriptor {
    #[serde(rename = "__fixture__", default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<FixtureMetadata>,
    pub record_kind: LfsHydrationDescriptorRecordKind,
    pub lfs_hydration_descriptor_schema_version: u32,
    pub lfs_hydration_descriptor_id: String,
    pub repo_root_descriptor_ref: String,
    pub repo_topology_state_ref: String,
    pub hydration_summary: HydrationSummaryClass,
    pub asset_buckets: Vec<AssetBucket>,
    pub hydrate_posture: HydratePosture,
    pub preview_export_posture: PreviewExportPosture,
    pub allowed_affordances: Vec<TopologyAffordanceClass>,
    pub freshness_class: FreshnessClass,
    pub client_scopes: Vec<ClientScope>,
    pub redaction_class: RedactionClass,
    pub created_at: String,
    pub updated_at: String,
}

impl LfsHydrationDescriptor {
    /// True when any object under this descriptor is still pointer-only.
    pub fn has_pointer_only_assets(&self) -> bool {
        matches!(
            self.hydration_summary,
            HydrationSummaryClass::LfsPointerOnlyPresent
                | HydrationSummaryClass::LfsPartiallyHydrated
        ) || self
            .asset_buckets
            .iter()
            .any(|bucket| matches!(bucket.asset_class, AssetClass::PointerOnlyKnownObjects))
    }

    /// True when this descriptor permits edit/export to operate on
    /// hydrated bytes rather than pointer metadata.
    pub fn permits_hydrated_body_export(&self) -> bool {
        matches!(
            self.preview_export_posture.export_body_class,
            ExportBodyClass::HydratedAsset
        )
    }
}
