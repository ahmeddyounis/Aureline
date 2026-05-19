//! Repository-topology beta truth.
//!
//! The four boundary descriptors in this module bind a single root's
//! identity, fetch/depth posture, submodule links, and Git LFS hydration
//! into one truth that every workspace, search, graph, blame, review,
//! AI context, execution, publish, and support surface consults before it
//! claims that a local view is "fully present". The [`beta`] submodule
//! adds the cross-surface projection: typed honesty labels, required
//! affordances (widen, deepen, init, fetch, hydrate, switch root), the
//! safe mutation target, and the body-export posture.
//!
//! Boundary schemas:
//!
//! - `schemas/workspace/repo_root_descriptor.schema.json`
//! - `schemas/workspace/fetch_depth_descriptor.schema.json`
//! - `schemas/workspace/submodule_link.schema.json`
//! - `schemas/workspace/lfs_hydration_descriptor.schema.json`
//!
//! Worked fixtures live under
//! `fixtures/workspace/m3/repo_topology_and_partial_clone/`.

pub mod beta;
pub mod descriptors;
pub mod shared;

pub use beta::{
    surface_must_downgrade_claim, BodyExportPosture, FullCoverageBlocker, MutationTarget,
    RepoTopologyBetaError, RepoTopologyBetaInputs, RepoTopologyBetaProjection,
    RepoTopologySurface,
};

pub use descriptors::{
    AssetBucket, AssetClass, ChildDirtyClass, ChildDirtyState, CompletenessState,
    CompletenessStateClass, DeepenPolicyClass, DriftClass, DriftState, EditTargetClass,
    ExportBodyClass, ExportSurfaceClass, FetchDenialReason, FetchDepthDescriptor,
    FetchDepthDescriptorRecordKind, FetchPolicyClass, FetchPosture, HistoryDepth,
    HistoryDepthState, HydratePosture, HydrationSummaryClass, InitClass, InitPolicyClass,
    InitState, LfsHydratePolicyClass, LfsHydrationDescriptor, LfsHydrationDescriptorRecordKind,
    LfsLockPostureClass, LfsPreviewExportDenial, NetworkCostBand, ParentLink, ParentLinkageClass,
    ParentMutationPosture, ParentMutationPostureClass, PartialCloneFilter,
    PartialCloneFilterClass, PinnedByClass, PolicyClass, PolicyPosture, PreviewExportPosture,
    PreviewTargetClass, PromisorClass, PromisorState, ReachabilityClass, ReconstructionField,
    RedactionPosture, RemoteRoleClass, RemoteSummary, RemoteSummaryEntry, RepoIdentity,
    RepoRootDescriptor, RepoRootDescriptorRecordKind, RepoRootKind,
    RepoTopologyExportSupportRequirements, SizeBand, SubmoduleDenialReason, SubmoduleLink,
    SubmoduleLinkRecordKind, SubmodulePinnedCommit, TrustClass, TrustPosture, VcsProviderClass,
    WorktreeIdentity, WorktreeKindClass, FETCH_DEPTH_DESCRIPTOR_RECORD_KIND,
    FETCH_DEPTH_DESCRIPTOR_SCHEMA_VERSION, LFS_HYDRATION_DESCRIPTOR_RECORD_KIND,
    LFS_HYDRATION_DESCRIPTOR_SCHEMA_VERSION, REPO_ROOT_DESCRIPTOR_RECORD_KIND,
    REPO_ROOT_DESCRIPTOR_SCHEMA_VERSION, SUBMODULE_LINK_RECORD_KIND,
    SUBMODULE_LINK_SCHEMA_VERSION,
};

pub use shared::{
    ClientScope as RepoTopologyClientScope, FixtureMetadata as RepoTopologyFixtureMetadata,
    FreshnessClass as RepoTopologyFreshnessClass, RedactionClass as RepoTopologyRedactionClass,
    RepoTopologyClass, TopologyAffordanceClass,
};
