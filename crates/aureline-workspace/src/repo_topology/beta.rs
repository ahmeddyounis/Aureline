//! Cross-surface beta projection of the four repo-topology descriptors.
//!
//! `RepoTopologyBetaProjection` is the single record workspace, search,
//! graph, blame, review, AI context, publish, support, and migration
//! surfaces consult before claiming a body, history, blame line, or
//! exported artifact is complete for a given root. It binds:
//!
//! - one [`RepoRootDescriptor`] (the active root identity / parent link /
//!   trust / policy / completeness state),
//! - one [`FetchDepthDescriptor`] (shallow / partial-clone / promisor
//!   posture for the same root),
//! - zero or more [`SubmoduleLink`]s the root participates in, and
//! - one [`LfsHydrationDescriptor`] for the LFS hydration posture.
//!
//! The projection deliberately never invents new descriptors. It only
//! quotes the four boundary records and adds typed predicates the surface
//! contract requires.

use serde::{Deserialize, Serialize};

use super::descriptors::{
    AssetClass, ChildDirtyClass, CompletenessStateClass, DriftClass, EditTargetClass,
    ExportBodyClass, FetchDepthDescriptor, FetchPolicyClass, HistoryDepthState,
    HydrationSummaryClass, InitClass, LfsHydrationDescriptor, ParentLinkageClass, PolicyClass,
    PromisorClass, RepoRootDescriptor, RepoRootKind, SubmoduleLink, TrustClass,
};
use super::shared::TopologyAffordanceClass;

/// Surface that consumes a [`RepoTopologyBetaProjection`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoTopologySurface {
    Workspace,
    Search,
    Graph,
    Blame,
    Review,
    Ai,
    Execution,
    Publish,
    Support,
    Migration,
}

impl RepoTopologySurface {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Search => "search",
            Self::Graph => "graph",
            Self::Blame => "blame",
            Self::Review => "review",
            Self::Ai => "ai",
            Self::Execution => "execution",
            Self::Publish => "publish",
            Self::Support => "support",
            Self::Migration => "migration",
        }
    }

    /// Surfaces that cannot truthfully claim "full coverage" if the
    /// underlying root is sparse, shallow, partial-clone, uninitialized,
    /// or pointer-only.
    pub const fn requires_full_coverage_for_claim(self) -> bool {
        matches!(
            self,
            Self::Search
                | Self::Graph
                | Self::Blame
                | Self::Review
                | Self::Ai
                | Self::Publish
                | Self::Migration
        )
    }
}

/// Mutation target a surface should resolve to before applying an edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationTarget {
    /// The parent root is the safe mutation target.
    ParentRoot,
    /// A child root (submodule, nested-independent, or worktree) is the
    /// safe mutation target.
    ChildRoot,
    /// Mutation is blocked until the caller selects a different root
    /// (e.g. the user opens the submodule or switches worktrees).
    SwitchRootRequired,
    /// Mutation is blocked because the asset is still pointer-only and
    /// must be hydrated first.
    ReadOnlyUntilHydrated,
    /// Mutation is blocked because the submodule has not been initialized
    /// and the parent gitlink is the only thing present.
    ReadOnlyUntilInitialized,
    /// Mutation is blocked by an active policy.
    PolicyBlocked,
}

/// Body-bytes posture used by publish, review, AI, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyExportPosture {
    /// Hydrated bytes may travel inside the packet.
    HydratedBytesAllowed,
    /// Only pointer metadata may travel; the packet records that the body
    /// was not embedded.
    PointerMetadataOnly,
    /// Body export is blocked by policy.
    BlockedByPolicy,
    /// Body export is unavailable (offline cache only, hydration failed).
    Unavailable,
}

/// Typed reason a "fully present local truth" claim is blocked. Surfaces
/// quote this enum verbatim when they downgrade their coverage claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FullCoverageBlocker {
    SparseOrWorksetNarrowed,
    ShallowHistoryPresent,
    PartialClonePromisorPresent,
    SubmoduleUninitialized,
    NestedIndependentBoundary,
    LfsPointerOnlyPresent,
    LfsPartiallyHydrated,
    PolicyBlocked,
    UnavailableUnknown,
}

impl FullCoverageBlocker {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SparseOrWorksetNarrowed => "sparse_or_workset_narrowed",
            Self::ShallowHistoryPresent => "shallow_history_present",
            Self::PartialClonePromisorPresent => "partial_clone_promisor_present",
            Self::SubmoduleUninitialized => "submodule_uninitialized",
            Self::NestedIndependentBoundary => "nested_independent_boundary",
            Self::LfsPointerOnlyPresent => "lfs_pointer_only_present",
            Self::LfsPartiallyHydrated => "lfs_partially_hydrated",
            Self::PolicyBlocked => "policy_blocked",
            Self::UnavailableUnknown => "unavailable_unknown",
        }
    }
}

/// Inputs used to assemble a beta projection. The projection itself
/// references each descriptor by id so consumers can re-resolve them via
/// the topology registry; the inputs let a caller hand the active set
/// directly without a separate lookup.
#[derive(Debug, Clone)]
pub struct RepoTopologyBetaInputs<'a> {
    pub repo_root: &'a RepoRootDescriptor,
    pub fetch_depth: Option<&'a FetchDepthDescriptor>,
    pub submodule_links: &'a [SubmoduleLink],
    pub lfs_hydration: Option<&'a LfsHydrationDescriptor>,
    pub surface: RepoTopologySurface,
}

/// Errors returned while assembling a beta projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoTopologyBetaError {
    /// The fetch/depth descriptor does not reference the active root
    /// descriptor.
    FetchDepthRootMismatch {
        expected_repo_root_descriptor_id: String,
        observed: String,
    },
    /// The LFS hydration descriptor does not reference the active root
    /// descriptor.
    LfsHydrationRootMismatch {
        expected_repo_root_descriptor_id: String,
        observed: String,
    },
    /// A supplied submodule link does not bind the active root as its
    /// parent.
    SubmoduleLinkParentMismatch {
        expected_repo_root_descriptor_id: String,
        observed: String,
    },
}

impl std::fmt::Display for RepoTopologyBetaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FetchDepthRootMismatch {
                expected_repo_root_descriptor_id,
                observed,
            } => write!(
                f,
                "fetch_depth descriptor references {observed}, expected {expected_repo_root_descriptor_id}"
            ),
            Self::LfsHydrationRootMismatch {
                expected_repo_root_descriptor_id,
                observed,
            } => write!(
                f,
                "lfs_hydration descriptor references {observed}, expected {expected_repo_root_descriptor_id}"
            ),
            Self::SubmoduleLinkParentMismatch {
                expected_repo_root_descriptor_id,
                observed,
            } => write!(
                f,
                "submodule link parent {observed}, expected {expected_repo_root_descriptor_id}"
            ),
        }
    }
}

impl std::error::Error for RepoTopologyBetaError {}

/// Beta truth one surface reads when it must agree with every other
/// surface about repo-root identity and incomplete-topology state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoTopologyBetaProjection {
    pub surface: RepoTopologySurface,
    pub workspace_id_ref: String,
    pub repo_topology_state_ref: String,
    pub repo_root_descriptor_ref: String,
    pub repo_root_kind: RepoRootKind,
    pub fetch_depth_descriptor_ref: Option<String>,
    pub submodule_link_refs: Vec<String>,
    pub lfs_hydration_descriptor_ref: Option<String>,
    pub may_claim_full_coverage: bool,
    pub full_coverage_blockers: Vec<FullCoverageBlocker>,
    pub required_affordances: Vec<TopologyAffordanceClass>,
    pub mutation_target: MutationTarget,
    pub body_export_posture: BodyExportPosture,
    pub honesty_labels: Vec<String>,
}

impl RepoTopologyBetaProjection {
    /// Builds a projection from the four descriptors. Validates that the
    /// supplied fetch/depth, submodule, and LFS records all bind the same
    /// root descriptor.
    pub fn project(inputs: RepoTopologyBetaInputs<'_>) -> Result<Self, RepoTopologyBetaError> {
        let RepoTopologyBetaInputs {
            repo_root,
            fetch_depth,
            submodule_links,
            lfs_hydration,
            surface,
        } = inputs;

        let root_id = repo_root.repo_root_descriptor_id.as_str();

        if let Some(fetch_depth) = fetch_depth {
            if fetch_depth.repo_root_descriptor_ref != root_id {
                return Err(RepoTopologyBetaError::FetchDepthRootMismatch {
                    expected_repo_root_descriptor_id: root_id.to_owned(),
                    observed: fetch_depth.repo_root_descriptor_ref.clone(),
                });
            }
        }
        if let Some(lfs) = lfs_hydration {
            if lfs.repo_root_descriptor_ref != root_id {
                return Err(RepoTopologyBetaError::LfsHydrationRootMismatch {
                    expected_repo_root_descriptor_id: root_id.to_owned(),
                    observed: lfs.repo_root_descriptor_ref.clone(),
                });
            }
        }
        for link in submodule_links {
            if link.parent_repo_root_descriptor_ref != root_id {
                return Err(RepoTopologyBetaError::SubmoduleLinkParentMismatch {
                    expected_repo_root_descriptor_id: root_id.to_owned(),
                    observed: link.parent_repo_root_descriptor_ref.clone(),
                });
            }
        }

        let blockers =
            full_coverage_blockers(repo_root, fetch_depth, submodule_links, lfs_hydration);
        let required = required_affordances(&blockers);
        let mutation_target = resolve_mutation_target(repo_root, submodule_links, lfs_hydration);
        let body_export_posture = resolve_body_export_posture(repo_root, lfs_hydration);
        let honesty_labels = honesty_labels_for(&blockers, repo_root);

        let may_claim_full_coverage = blockers.is_empty()
            && repo_root.completeness_state.may_claim_full_coverage
            && matches!(
                repo_root.completeness_state.completeness_class,
                CompletenessStateClass::FullyPresentLocalTruth
            );

        Ok(Self {
            surface,
            workspace_id_ref: repo_root.workspace_id_ref.clone(),
            repo_topology_state_ref: repo_root.repo_topology_state_ref.clone(),
            repo_root_descriptor_ref: repo_root.repo_root_descriptor_id.clone(),
            repo_root_kind: repo_root.root_kind,
            fetch_depth_descriptor_ref: fetch_depth
                .map(|fd| fd.fetch_depth_descriptor_id.clone()),
            submodule_link_refs: submodule_links
                .iter()
                .map(|link| link.submodule_link_id.clone())
                .collect(),
            lfs_hydration_descriptor_ref: lfs_hydration
                .map(|lfs| lfs.lfs_hydration_descriptor_id.clone()),
            may_claim_full_coverage,
            full_coverage_blockers: blockers,
            required_affordances: required,
            mutation_target,
            body_export_posture,
            honesty_labels,
        })
    }
}

fn full_coverage_blockers(
    repo_root: &RepoRootDescriptor,
    fetch_depth: Option<&FetchDepthDescriptor>,
    submodule_links: &[SubmoduleLink],
    lfs_hydration: Option<&LfsHydrationDescriptor>,
) -> Vec<FullCoverageBlocker> {
    let mut blockers = Vec::new();

    match repo_root.completeness_state.completeness_class {
        CompletenessStateClass::SparseOrWorksetNarrowed => {
            blockers.push(FullCoverageBlocker::SparseOrWorksetNarrowed)
        }
        CompletenessStateClass::ShallowHistoryPresent => {
            blockers.push(FullCoverageBlocker::ShallowHistoryPresent)
        }
        CompletenessStateClass::PartialClonePromisorPresent => {
            blockers.push(FullCoverageBlocker::PartialClonePromisorPresent)
        }
        CompletenessStateClass::SubmoduleUninitialized => {
            blockers.push(FullCoverageBlocker::SubmoduleUninitialized)
        }
        CompletenessStateClass::NestedIndependentBoundary => {
            blockers.push(FullCoverageBlocker::NestedIndependentBoundary)
        }
        CompletenessStateClass::LfsPointerOnlyPresent => {
            blockers.push(FullCoverageBlocker::LfsPointerOnlyPresent)
        }
        CompletenessStateClass::LfsPartiallyHydrated => {
            blockers.push(FullCoverageBlocker::LfsPartiallyHydrated)
        }
        CompletenessStateClass::UnavailableUnknown => {
            blockers.push(FullCoverageBlocker::UnavailableUnknown)
        }
        CompletenessStateClass::FullyPresentLocalTruth => {}
    }

    if let Some(fetch_depth) = fetch_depth {
        match fetch_depth.history_depth.depth_state {
            HistoryDepthState::ShallowBoundaryPresent => {
                push_unique(&mut blockers, FullCoverageBlocker::ShallowHistoryPresent)
            }
            HistoryDepthState::HistoryUnavailable | HistoryDepthState::Unknown => {
                push_unique(&mut blockers, FullCoverageBlocker::UnavailableUnknown)
            }
            HistoryDepthState::FullHistoryAvailable => {}
        }
        match fetch_depth.promisor_state.promisor_class {
            PromisorClass::PromisorRemoteConfigured
            | PromisorClass::PartialCloneFilterActive
            | PromisorClass::PromisorUnreachable => push_unique(
                &mut blockers,
                FullCoverageBlocker::PartialClonePromisorPresent,
            ),
            PromisorClass::FetchForbiddenByPolicy => {
                push_unique(&mut blockers, FullCoverageBlocker::PolicyBlocked)
            }
            PromisorClass::Unknown => {
                push_unique(&mut blockers, FullCoverageBlocker::UnavailableUnknown)
            }
            PromisorClass::NotPromisorFullClone => {}
        }
        if matches!(
            fetch_depth.fetch_posture.fetch_policy_class,
            FetchPolicyClass::FetchBlockedPolicy
        ) {
            push_unique(&mut blockers, FullCoverageBlocker::PolicyBlocked);
        }
    }

    for link in submodule_links {
        if !matches!(link.init_state.init_class, InitClass::Initialized) {
            push_unique(&mut blockers, FullCoverageBlocker::SubmoduleUninitialized);
        }
    }

    if let Some(lfs) = lfs_hydration {
        match lfs.hydration_summary {
            HydrationSummaryClass::LfsPointerOnlyPresent => {
                push_unique(&mut blockers, FullCoverageBlocker::LfsPointerOnlyPresent)
            }
            HydrationSummaryClass::LfsPartiallyHydrated => {
                push_unique(&mut blockers, FullCoverageBlocker::LfsPartiallyHydrated)
            }
            HydrationSummaryClass::HydrationBlockedOrUnknown => {
                push_unique(&mut blockers, FullCoverageBlocker::UnavailableUnknown)
            }
            HydrationSummaryClass::LfsFullyHydrated
            | HydrationSummaryClass::NoLfsObjectsSeen
            | HydrationSummaryClass::NotApplicable => {}
        }
        if lfs
            .asset_buckets
            .iter()
            .any(|bucket| matches!(bucket.asset_class, AssetClass::HydrationBlockedObjects))
        {
            push_unique(&mut blockers, FullCoverageBlocker::PolicyBlocked);
        }
    }

    match repo_root.policy_posture.policy_class {
        PolicyClass::PolicyBlockedFetch
        | PolicyClass::PolicyBlockedDeepen
        | PolicyClass::PolicyBlockedInit
        | PolicyClass::PolicyBlockedHydrate
        | PolicyClass::PolicyBlockedExport
        | PolicyClass::PolicyRestrictedMutation => {
            push_unique(&mut blockers, FullCoverageBlocker::PolicyBlocked)
        }
        PolicyClass::NoPolicyActive | PolicyClass::PolicyObserved => {}
    }

    if matches!(
        repo_root.trust_posture.trust_class,
        TrustClass::UntrustedPendingReview
    ) {
        push_unique(&mut blockers, FullCoverageBlocker::UnavailableUnknown);
    }

    blockers
}

fn required_affordances(blockers: &[FullCoverageBlocker]) -> Vec<TopologyAffordanceClass> {
    let mut required = Vec::new();
    for blocker in blockers {
        match blocker {
            FullCoverageBlocker::SparseOrWorksetNarrowed => {
                push_unique(&mut required, TopologyAffordanceClass::WidenWorksetScope)
            }
            FullCoverageBlocker::ShallowHistoryPresent => {
                push_unique(&mut required, TopologyAffordanceClass::DeepenHistory)
            }
            FullCoverageBlocker::PartialClonePromisorPresent => {
                push_unique(&mut required, TopologyAffordanceClass::FetchMissingObjects)
            }
            FullCoverageBlocker::SubmoduleUninitialized => {
                push_unique(&mut required, TopologyAffordanceClass::InitSubmodule)
            }
            FullCoverageBlocker::NestedIndependentBoundary => {
                push_unique(&mut required, TopologyAffordanceClass::SwitchTargetRoot)
            }
            FullCoverageBlocker::LfsPointerOnlyPresent
            | FullCoverageBlocker::LfsPartiallyHydrated => {
                push_unique(&mut required, TopologyAffordanceClass::HydrateLfsObjects)
            }
            FullCoverageBlocker::PolicyBlocked | FullCoverageBlocker::UnavailableUnknown => {}
        }
    }
    if required.is_empty() {
        required.push(TopologyAffordanceClass::NoneAvailable);
    }
    required
}

fn resolve_mutation_target(
    repo_root: &RepoRootDescriptor,
    submodule_links: &[SubmoduleLink],
    lfs_hydration: Option<&LfsHydrationDescriptor>,
) -> MutationTarget {
    if matches!(
        repo_root.policy_posture.policy_class,
        PolicyClass::PolicyRestrictedMutation
            | PolicyClass::PolicyBlockedFetch
            | PolicyClass::PolicyBlockedDeepen
            | PolicyClass::PolicyBlockedInit
            | PolicyClass::PolicyBlockedHydrate
            | PolicyClass::PolicyBlockedExport
    ) {
        return MutationTarget::PolicyBlocked;
    }

    if let Some(lfs) = lfs_hydration {
        if matches!(
            lfs.preview_export_posture.edit_target_class,
            EditTargetClass::EditDeniedUntilHydrated | EditTargetClass::PointerOnly
        ) {
            return MutationTarget::ReadOnlyUntilHydrated;
        }
        if matches!(
            lfs.preview_export_posture.edit_target_class,
            EditTargetClass::EditDeniedPolicy
        ) {
            return MutationTarget::PolicyBlocked;
        }
    }

    match repo_root.parent_link.linkage_class {
        ParentLinkageClass::NestedIndependentChild => return MutationTarget::SwitchRootRequired,
        ParentLinkageClass::WorktreeSharedObjectStoreChild => return MutationTarget::ChildRoot,
        _ => {}
    }

    for link in submodule_links {
        match link.init_state.init_class {
            InitClass::NotInitialized | InitClass::InitFailed | InitClass::InitBlockedPolicy => {
                return MutationTarget::ReadOnlyUntilInitialized;
            }
            InitClass::Initialized
                if !matches!(
                    link.drift_state.drift_class,
                    DriftClass::ChildAtPinnedCommit | DriftClass::NotApplicable
                ) =>
            {
                return MutationTarget::ChildRoot;
            }
            _ => {}
        }
        if !matches!(link.child_dirty_state.dirty_class, ChildDirtyClass::Clean) {
            return MutationTarget::ChildRoot;
        }
    }

    if matches!(repo_root.root_kind, RepoRootKind::Submodule | RepoRootKind::Nested) {
        return MutationTarget::ChildRoot;
    }

    MutationTarget::ParentRoot
}

fn resolve_body_export_posture(
    repo_root: &RepoRootDescriptor,
    lfs_hydration: Option<&LfsHydrationDescriptor>,
) -> BodyExportPosture {
    if matches!(
        repo_root.policy_posture.policy_class,
        PolicyClass::PolicyBlockedExport
    ) {
        return BodyExportPosture::BlockedByPolicy;
    }
    if let Some(lfs) = lfs_hydration {
        return match lfs.preview_export_posture.export_body_class {
            ExportBodyClass::HydratedAsset => BodyExportPosture::HydratedBytesAllowed,
            ExportBodyClass::PointerMetadataOnly => BodyExportPosture::PointerMetadataOnly,
            ExportBodyClass::ExportBlockedPolicy => BodyExportPosture::BlockedByPolicy,
            ExportBodyClass::ExportUnavailable => BodyExportPosture::Unavailable,
        };
    }
    BodyExportPosture::HydratedBytesAllowed
}

fn honesty_labels_for(
    blockers: &[FullCoverageBlocker],
    repo_root: &RepoRootDescriptor,
) -> Vec<String> {
    let mut labels: Vec<String> = Vec::new();
    for blocker in blockers {
        let label = match blocker {
            FullCoverageBlocker::SparseOrWorksetNarrowed => "outside_current_slice",
            FullCoverageBlocker::ShallowHistoryPresent => "shallow_boundary",
            FullCoverageBlocker::PartialClonePromisorPresent => "not_fetched",
            FullCoverageBlocker::SubmoduleUninitialized => "submodule_not_initialized",
            FullCoverageBlocker::NestedIndependentBoundary => "nested_repo_boundary",
            FullCoverageBlocker::LfsPointerOnlyPresent
            | FullCoverageBlocker::LfsPartiallyHydrated => "pointer_only",
            FullCoverageBlocker::PolicyBlocked => "policy_excluded",
            FullCoverageBlocker::UnavailableUnknown => "unavailable",
        };
        if !labels.iter().any(|existing| existing == label) {
            labels.push(label.to_string());
        }
    }
    if matches!(repo_root.root_kind, RepoRootKind::Nested)
        && !labels.iter().any(|l| l == "nested_repo_boundary")
    {
        labels.push("nested_repo_boundary".to_string());
    }
    labels
}

fn push_unique<T: PartialEq>(slot: &mut Vec<T>, value: T) {
    if !slot.contains(&value) {
        slot.push(value);
    }
}

/// Predicate: a search/blame/AI/review/publish surface MAY NOT claim
/// "fully covered" without first running every required affordance.
pub fn surface_must_downgrade_claim(projection: &RepoTopologyBetaProjection) -> bool {
    projection.surface.requires_full_coverage_for_claim() && !projection.may_claim_full_coverage
}

#[cfg(test)]
mod tests {
    use super::super::descriptors::{
        AssetBucket, AssetClass, ChildDirtyClass, ChildDirtyState, CompletenessState,
        CompletenessStateClass, DeepenPolicyClass, DriftClass, DriftState, EditTargetClass,
        ExportBodyClass, ExportSurfaceClass, FetchDepthDescriptor,
        FetchDepthDescriptorRecordKind, FetchPolicyClass, FetchPosture, HistoryDepth,
        HistoryDepthState, HydratePosture, HydrationSummaryClass, InitClass, InitPolicyClass,
        InitState, LfsHydratePolicyClass, LfsHydrationDescriptor,
        LfsHydrationDescriptorRecordKind, LfsLockPostureClass, NetworkCostBand, ParentLink,
        ParentLinkageClass, PartialCloneFilter, PartialCloneFilterClass, PinnedByClass,
        PolicyClass, PolicyPosture, PreviewExportPosture, PreviewTargetClass, PromisorClass,
        PromisorState, ReachabilityClass, ReconstructionField, RedactionPosture,
        RemoteSummary, RepoIdentity, RepoRootDescriptor, RepoRootDescriptorRecordKind,
        RepoRootKind, RepoTopologyExportSupportRequirements, SizeBand, SubmoduleDenialReason,
        SubmoduleLink, SubmoduleLinkRecordKind, SubmodulePinnedCommit, TrustClass, TrustPosture,
        VcsProviderClass, WorktreeIdentity, WorktreeKindClass, ParentMutationPosture,
        ParentMutationPostureClass, FETCH_DEPTH_DESCRIPTOR_SCHEMA_VERSION,
        LFS_HYDRATION_DESCRIPTOR_SCHEMA_VERSION, REPO_ROOT_DESCRIPTOR_SCHEMA_VERSION,
        SUBMODULE_LINK_SCHEMA_VERSION,
    };
    use super::super::shared::{
        ClientScope, FreshnessClass, RedactionClass, RepoTopologyClass, TopologyAffordanceClass,
    };
    use super::*;

    fn now() -> String {
        "2026-05-19T00:00:00Z".to_string()
    }

    fn primary_root(completeness: CompletenessStateClass) -> RepoRootDescriptor {
        RepoRootDescriptor {
            fixture: None,
            record_kind: RepoRootDescriptorRecordKind::RepoRootDescriptorRecord,
            repo_root_descriptor_schema_version: REPO_ROOT_DESCRIPTOR_SCHEMA_VERSION,
            repo_root_descriptor_id: "rrd:primary".to_string(),
            repo_topology_state_ref: "topology:primary".to_string(),
            workspace_id_ref: "workspace:test".to_string(),
            root_kind: RepoRootKind::Primary,
            topology_classes: vec![RepoTopologyClass::CurrentRepoRoot],
            repo_identity: RepoIdentity {
                repo_root_ref: "root:primary".to_string(),
                repo_identity_ref: "repo:primary".to_string(),
                vcs_provider_class: VcsProviderClass::GitLocal,
                vcs_provider_ref: Some("vcs:git".to_string()),
                object_store_ref: Some("os:primary".to_string()),
                default_branch_ref: Some("ref:main".to_string()),
            },
            worktree_identity: WorktreeIdentity {
                worktree_id_ref: Some("worktree:primary".to_string()),
                worktree_kind_class: WorktreeKindClass::MainRepositoryCheckout,
                head_revision_id_ref: Some("commit:head".to_string()),
                branch_or_detached_ref: Some("ref:main".to_string()),
            },
            parent_link: ParentLink {
                linkage_class: ParentLinkageClass::NoParentRoot,
                parent_root_ref: None,
                link_path_ref: None,
                pinned_commit_ref: None,
                submodule_link_ref: None,
            },
            remote_summary: RemoteSummary {
                remotes: Vec::new(),
                promisor_remote_ref: None,
                publish_remote_ref: None,
            },
            trust_posture: TrustPosture {
                trust_class: TrustClass::TrustedLocal,
                review_required: false,
                review_ticket_ref: None,
            },
            policy_posture: PolicyPosture {
                policy_class: PolicyClass::NoPolicyActive,
                policy_ref: None,
                policy_blocks: Vec::new(),
            },
            completeness_state: CompletenessState {
                completeness_class: completeness,
                fetch_depth_descriptor_ref: None,
                submodule_link_refs: Vec::new(),
                lfs_hydration_descriptor_ref: None,
                may_claim_full_coverage: matches!(
                    completeness,
                    CompletenessStateClass::FullyPresentLocalTruth
                ),
            },
            supported_affordances: vec![TopologyAffordanceClass::ExportTopologyPacket],
            export_support_requirements: RepoTopologyExportSupportRequirements {
                packet_surfaces: vec![ExportSurfaceClass::SupportBundle],
                reconstruction_fields: vec![ReconstructionField::RepoIdentity],
                redaction_posture: RedactionPosture::LabelsAndCounts,
            },
            freshness_class: FreshnessClass::AuthoritativeLive,
            client_scopes: vec![ClientScope::DesktopProduct],
            redaction_class: RedactionClass::MetadataSafeDefault,
            created_at: now(),
            updated_at: now(),
        }
    }

    fn shallow_fetch_depth() -> FetchDepthDescriptor {
        FetchDepthDescriptor {
            fixture: None,
            record_kind: FetchDepthDescriptorRecordKind::FetchDepthDescriptorRecord,
            fetch_depth_descriptor_schema_version: FETCH_DEPTH_DESCRIPTOR_SCHEMA_VERSION,
            fetch_depth_descriptor_id: "fdd:primary".to_string(),
            repo_root_descriptor_ref: "rrd:primary".to_string(),
            repo_topology_state_ref: "topology:primary".to_string(),
            history_depth: HistoryDepth {
                depth_state: HistoryDepthState::ShallowBoundaryPresent,
                depth_commits: Some(50),
                boundary_commit_ref: Some("commit:boundary".to_string()),
                deepen_policy_class: DeepenPolicyClass::DeepenOnDemandAllowed,
            },
            partial_clone_filter: PartialCloneFilter {
                filter_class: PartialCloneFilterClass::None,
                missing_object_count_known: false,
                missing_object_count: None,
            },
            promisor_state: PromisorState {
                promisor_class: PromisorClass::NotPromisorFullClone,
                promisor_remote_ref: None,
                promisor_reachability: ReachabilityClass::Reachable,
                last_fetch_at: None,
            },
            fetch_posture: FetchPosture {
                fetch_policy_class: FetchPolicyClass::FetchOnDemandAllowed,
                blocks_full_coverage_claim: true,
                denial_reason: Some(super::super::descriptors::FetchDenialReason::ShallowBoundary),
            },
            allowed_affordances: vec![TopologyAffordanceClass::DeepenHistory],
            freshness_class: FreshnessClass::AuthoritativeLive,
            client_scopes: vec![ClientScope::DesktopProduct],
            redaction_class: RedactionClass::MetadataSafeDefault,
            created_at: now(),
            updated_at: now(),
        }
    }

    #[test]
    fn primary_root_full_coverage_passes_through() {
        let root = primary_root(CompletenessStateClass::FullyPresentLocalTruth);
        let projection = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
            repo_root: &root,
            fetch_depth: None,
            submodule_links: &[],
            lfs_hydration: None,
            surface: RepoTopologySurface::Search,
        })
        .expect("must project");

        assert!(projection.may_claim_full_coverage);
        assert!(projection.full_coverage_blockers.is_empty());
        assert!(!surface_must_downgrade_claim(&projection));
        assert!(matches!(projection.mutation_target, MutationTarget::ParentRoot));
    }

    #[test]
    fn shallow_history_forces_deepen_affordance_and_blocks_blame_claim() {
        let root = primary_root(CompletenessStateClass::ShallowHistoryPresent);
        let fetch_depth = shallow_fetch_depth();
        let projection = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
            repo_root: &root,
            fetch_depth: Some(&fetch_depth),
            submodule_links: &[],
            lfs_hydration: None,
            surface: RepoTopologySurface::Blame,
        })
        .expect("must project");

        assert!(!projection.may_claim_full_coverage);
        assert!(projection
            .full_coverage_blockers
            .contains(&FullCoverageBlocker::ShallowHistoryPresent));
        assert!(projection
            .required_affordances
            .contains(&TopologyAffordanceClass::DeepenHistory));
        assert!(surface_must_downgrade_claim(&projection));
    }

    #[test]
    fn uninitialized_submodule_forces_init_and_read_only_mutation_target() {
        let root = primary_root(CompletenessStateClass::SubmoduleUninitialized);
        let link = SubmoduleLink {
            fixture: None,
            record_kind: SubmoduleLinkRecordKind::SubmoduleLinkRecord,
            submodule_link_schema_version: SUBMODULE_LINK_SCHEMA_VERSION,
            submodule_link_id: "sml:vendor".to_string(),
            parent_repo_root_descriptor_ref: "rrd:primary".to_string(),
            child_repo_root_descriptor_ref: None,
            repo_topology_state_ref: "topology:primary".to_string(),
            link_path_ref: "path:vendor".to_string(),
            pinned_commit: SubmodulePinnedCommit {
                commit_ref: "commit:pin".to_string(),
                pinned_by_class: PinnedByClass::SubmoduleGitlink,
                pin_freshness_class: FreshnessClass::AuthoritativeLive,
            },
            init_state: InitState {
                init_class: InitClass::NotInitialized,
                init_policy_class: InitPolicyClass::InitOnDemandAllowed,
                last_init_at: None,
            },
            child_dirty_state: ChildDirtyState {
                dirty_class: ChildDirtyClass::DirtyUnknownUninitialized,
                dirty_summary_ref: None,
            },
            drift_state: DriftState {
                drift_class: DriftClass::DriftUnknownUninitialized,
                observed_child_commit_ref: None,
            },
            allowed_affordances: vec![TopologyAffordanceClass::InitSubmodule],
            parent_mutation_posture: ParentMutationPosture {
                posture_class: ParentMutationPostureClass::ChildContentReadOnlyUntilInitialized,
                denial_reason: Some(SubmoduleDenialReason::SubmoduleNotInitialized),
            },
            freshness_class: FreshnessClass::AuthoritativeLive,
            client_scopes: vec![ClientScope::DesktopProduct],
            redaction_class: RedactionClass::MetadataSafeDefault,
            created_at: now(),
            updated_at: now(),
        };
        let projection = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
            repo_root: &root,
            fetch_depth: None,
            submodule_links: std::slice::from_ref(&link),
            lfs_hydration: None,
            surface: RepoTopologySurface::Review,
        })
        .expect("must project");

        assert!(projection
            .required_affordances
            .contains(&TopologyAffordanceClass::InitSubmodule));
        assert!(matches!(
            projection.mutation_target,
            MutationTarget::ReadOnlyUntilInitialized
        ));
        assert!(link.child_content_externalized());
    }

    #[test]
    fn pointer_only_lfs_blocks_full_coverage_and_export_carries_metadata_only() {
        let root = primary_root(CompletenessStateClass::LfsPointerOnlyPresent);
        let lfs = LfsHydrationDescriptor {
            fixture: None,
            record_kind: LfsHydrationDescriptorRecordKind::LfsHydrationDescriptorRecord,
            lfs_hydration_descriptor_schema_version: LFS_HYDRATION_DESCRIPTOR_SCHEMA_VERSION,
            lfs_hydration_descriptor_id: "lfs:primary".to_string(),
            repo_root_descriptor_ref: "rrd:primary".to_string(),
            repo_topology_state_ref: "topology:primary".to_string(),
            hydration_summary: HydrationSummaryClass::LfsPointerOnlyPresent,
            asset_buckets: vec![AssetBucket {
                asset_class: AssetClass::PointerOnlyKnownObjects,
                object_count_known: true,
                object_count: Some(12),
                size_band: SizeBand::Large,
            }],
            hydrate_posture: HydratePosture {
                hydrate_policy_class: LfsHydratePolicyClass::HydrateOnDemandAllowed,
                lock_posture_class: LfsLockPostureClass::NotLocked,
                network_cost_band: NetworkCostBand::Network,
                last_hydration_at: None,
            },
            preview_export_posture: PreviewExportPosture {
                edit_target_class: EditTargetClass::PointerOnly,
                preview_target_class: PreviewTargetClass::PointerMetadataOnly,
                export_body_class: ExportBodyClass::PointerMetadataOnly,
                denial_reason: Some(
                    super::super::descriptors::LfsPreviewExportDenial::PointerOnly,
                ),
            },
            allowed_affordances: vec![TopologyAffordanceClass::HydrateLfsObjects],
            freshness_class: FreshnessClass::AuthoritativeLive,
            client_scopes: vec![ClientScope::DesktopProduct],
            redaction_class: RedactionClass::MetadataSafeDefault,
            created_at: now(),
            updated_at: now(),
        };

        let projection = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
            repo_root: &root,
            fetch_depth: None,
            submodule_links: &[],
            lfs_hydration: Some(&lfs),
            surface: RepoTopologySurface::Publish,
        })
        .expect("must project");

        assert!(!projection.may_claim_full_coverage);
        assert!(projection
            .full_coverage_blockers
            .contains(&FullCoverageBlocker::LfsPointerOnlyPresent));
        assert!(projection
            .required_affordances
            .contains(&TopologyAffordanceClass::HydrateLfsObjects));
        assert!(matches!(
            projection.body_export_posture,
            BodyExportPosture::PointerMetadataOnly
        ));
        assert!(matches!(
            projection.mutation_target,
            MutationTarget::ReadOnlyUntilHydrated
        ));
    }

    #[test]
    fn root_id_mismatch_is_rejected() {
        let root = primary_root(CompletenessStateClass::FullyPresentLocalTruth);
        let mut fetch_depth = shallow_fetch_depth();
        fetch_depth.repo_root_descriptor_ref = "rrd:other".to_string();
        let err = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
            repo_root: &root,
            fetch_depth: Some(&fetch_depth),
            submodule_links: &[],
            lfs_hydration: None,
            surface: RepoTopologySurface::Search,
        })
        .expect_err("mismatch must be rejected");

        assert!(matches!(
            err,
            RepoTopologyBetaError::FetchDepthRootMismatch { .. }
        ));
    }
}
