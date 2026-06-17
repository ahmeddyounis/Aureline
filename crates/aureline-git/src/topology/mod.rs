//! Explicit repository-topology descriptors and their first consumers.
//!
//! This module turns repository topology into a canonical, serde-serializable
//! substrate. Where the upstream topology-truth packet stored each boundary as
//! an opaque scope reference, this module records the topology of a root with
//! **explicit, structured fields**: the omitted-path set, the checkout filter
//! class, the history depth boundary, the parent/child repo identity, the
//! worktree root, the Git LFS object state, and any generated/vendor origin.
//!
//! The same [`TopologyRootDescriptor`] then *drives* every M5 Git-adjacent
//! surface through one deterministic projection
//! ([`TopologyRootDescriptor::project`]): Git status, review, blame, search
//! scope, AI-context assembly, and redaction-safe support/export all read the
//! identical truth instead of re-deriving topology from ambient assumptions.
//! Because the surface bindings are *derived* from the descriptor, a local or
//! provider overlay cannot quietly erase a boundary: omitted paths, unfetched
//! objects, shallow ancestry, parent/child identity, pointer-only assets, and
//! generated/vendor roots stay visible to the user, CLI, AI context, and
//! support/export tooling.
//!
//! Topology truth is never reduced to a badge. A pointer-only or unfetched
//! object never masquerades as "not found" or as fully hydrated truth, and a
//! parent and a child root never collapse into one bulk mutation scope; the
//! descriptors carry no raw paths or raw object bytes, only redaction-safe
//! refs.
//!
//! The boundary schema is
//! [`schemas/git/topology.schema.json`](../../../../schemas/git/topology.schema.json).
//! The protected fixture corpus is
//! [`fixtures/git/m5/topology-corpus/`](../../../../fixtures/git/m5/topology-corpus/).
//! The checked-in canonical map is
//! [`artifacts/git/m5/git_topology/topology_first_consumers.json`](../../../../artifacts/git/m5/git_topology/topology_first_consumers.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_repository_topology_truth::{
    CoverageClaimPosture, RepositoryTopologyClass, SurfaceResultTruth, TopologyHonestyLabel,
    TopologyOperationScope,
};

/// Schema version for [`RepositoryTopologyMap`].
pub const GIT_TOPOLOGY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`RepositoryTopologyMap`].
pub const GIT_TOPOLOGY_MAP_RECORD_KIND: &str = "git_topology_map";

/// Stable record-kind tag carried by [`TopologyRootDescriptor`].
pub const GIT_TOPOLOGY_ROOT_DESCRIPTOR_RECORD_KIND: &str = "git_topology_root_descriptor";

/// Stable record-kind tag carried by [`SurfaceTopologyBinding`].
pub const GIT_TOPOLOGY_SURFACE_BINDING_RECORD_KIND: &str = "git_topology_surface_binding";

/// Stable record-kind tag carried by [`TopologySupportExport`].
pub const GIT_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND: &str = "git_topology_support_export";

/// Repo-relative path of the boundary schema.
pub const GIT_TOPOLOGY_SCHEMA_REF: &str = "schemas/git/topology.schema.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const GIT_TOPOLOGY_FIXTURE_DIR: &str = "fixtures/git/m5/topology-corpus";

/// Repo-relative path of the checked-in canonical first-consumers map.
pub const GIT_TOPOLOGY_ARTIFACT_REF: &str =
    "artifacts/git/m5/git_topology/topology_first_consumers.json";

/// Reconstruction fields a support export must retain after redaction.
pub const GIT_TOPOLOGY_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 7] = [
    "filter_class",
    "omitted_path_set",
    "depth_boundary",
    "repo_identity",
    "worktree_root",
    "lfs_object_state",
    "generated_vendor_origin",
];

/// Consumer surface that reuses a topology descriptor instead of re-deriving it.
///
/// These are the first real consumers this lane wires up; each one must be able
/// to distinguish every topology state rather than flatten them into one badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyConsumerSurface {
    /// Git status, stage, and daily-loop rows.
    GitStatus,
    /// Review diff, summary, and publish rows.
    Review,
    /// Blame and file-history rows.
    Blame,
    /// Search scope and zero-result rows.
    SearchScope,
    /// AI-context assembly and evidence inspectors.
    AiContext,
    /// Redaction-safe support / export rows.
    SupportExport,
}

impl TopologyConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GitStatus,
        Self::Review,
        Self::Blame,
        Self::SearchScope,
        Self::AiContext,
        Self::SupportExport,
    ];

    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitStatus => "git_status",
            Self::Review => "review",
            Self::Blame => "blame",
            Self::SearchScope => "search_scope",
            Self::AiContext => "ai_context",
            Self::SupportExport => "support_export",
        }
    }

    /// Whether this surface can drive a content mutation (stage, apply, publish).
    pub const fn is_mutation_surface(self) -> bool {
        matches!(self, Self::GitStatus | Self::Review)
    }

    /// Whether this surface reads commit ancestry and so is bounded by a shallow
    /// or grafted history boundary.
    pub const fn reads_history(self) -> bool {
        matches!(self, Self::Blame | Self::AiContext | Self::SupportExport)
    }

    /// Whether this surface scopes by path and so is narrowed by an omitted
    /// sparse/workset slice.
    pub const fn scopes_by_path(self) -> bool {
        matches!(
            self,
            Self::GitStatus
                | Self::Review
                | Self::SearchScope
                | Self::AiContext
                | Self::SupportExport
        )
    }

    /// Whether this surface may embed object body bytes when content is hydrated;
    /// the support-export surface stays metadata-only.
    pub const fn allows_body_export(self) -> bool {
        !matches!(self, Self::SupportExport)
    }
}

/// Checkout filter class that determines which objects and paths are present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutFilterClass {
    /// A full checkout with no sparse or partial filter.
    FullCheckout,
    /// A cone-mode sparse checkout.
    SparseCheckoutCone,
    /// A non-cone (pattern) sparse checkout.
    SparseCheckoutNonCone,
    /// A blobless partial clone (`--filter=blob:none`).
    PartialCloneBlobless,
    /// A treeless partial clone (`--filter=tree:0`).
    PartialCloneTreeless,
    /// A promisor-backed partial clone with another object filter.
    PartialClonePromisor,
}

impl CheckoutFilterClass {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullCheckout => "full_checkout",
            Self::SparseCheckoutCone => "sparse_checkout_cone",
            Self::SparseCheckoutNonCone => "sparse_checkout_non_cone",
            Self::PartialCloneBlobless => "partial_clone_blobless",
            Self::PartialCloneTreeless => "partial_clone_treeless",
            Self::PartialClonePromisor => "partial_clone_promisor",
        }
    }

    /// Whether this filter omits paths from the working tree (sparse classes).
    pub const fn omits_paths(self) -> bool {
        matches!(self, Self::SparseCheckoutCone | Self::SparseCheckoutNonCone)
    }

    /// Whether this filter is a partial clone whose objects may be unfetched.
    pub const fn is_partial_clone(self) -> bool {
        matches!(
            self,
            Self::PartialCloneBlobless | Self::PartialCloneTreeless | Self::PartialClonePromisor
        )
    }
}

/// Explicit set of paths omitted by the active sparse checkout or workset slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmittedPathSet {
    /// Redaction-safe refs for omitted top-level path scopes (never raw paths).
    pub path_refs: Vec<String>,
    /// Optional estimate of how many paths are omitted, for honest counts.
    pub omitted_estimate: Option<u64>,
}

impl OmittedPathSet {
    /// An omitted set that omits nothing.
    pub fn none() -> Self {
        Self {
            path_refs: Vec::new(),
            omitted_estimate: None,
        }
    }

    /// Whether this set omits any paths.
    pub fn is_empty(&self) -> bool {
        self.path_refs.is_empty()
    }
}

/// History depth class that bounds blame and log truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryDepthClass {
    /// Full history is present.
    FullHistory,
    /// History is limited by clone depth (`--depth`).
    ShallowDepth,
    /// History is grafted at a replacement boundary.
    Grafted,
    /// A single-branch clone bounds cross-branch ancestry.
    SingleBranchClone,
}

impl HistoryDepthClass {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullHistory => "full_history",
            Self::ShallowDepth => "shallow_depth",
            Self::Grafted => "grafted",
            Self::SingleBranchClone => "single_branch_clone",
        }
    }

    /// Whether this class bounds ancestry below full history.
    pub const fn is_bounded(self) -> bool {
        !matches!(self, Self::FullHistory)
    }
}

/// Explicit history depth boundary for a root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthBoundary {
    /// Depth class.
    pub depth_class: HistoryDepthClass,
    /// Redaction-safe refs for the commits at the shallow/graft boundary.
    pub shallow_boundary_refs: Vec<String>,
    /// Configured clone depth, when known.
    pub configured_depth: Option<u32>,
}

impl DepthBoundary {
    /// A full-history boundary with no shallow edge.
    pub fn full() -> Self {
        Self {
            depth_class: HistoryDepthClass::FullHistory,
            shallow_boundary_refs: Vec::new(),
            configured_depth: None,
        }
    }
}

/// Parent/child identity class for a repository root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoIdentityKind {
    /// A standalone repository with no parent and no tracked children.
    Standalone,
    /// A parent repository that owns one or more submodule children.
    ParentWithChildren,
    /// A submodule child pinned by a parent gitlink.
    SubmoduleChild,
    /// A nested repository whose `.git` is independent of the surrounding tree.
    NestedIndependent,
}

impl RepoIdentityKind {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standalone => "standalone",
            Self::ParentWithChildren => "parent_with_children",
            Self::SubmoduleChild => "submodule_child",
            Self::NestedIndependent => "nested_independent",
        }
    }

    /// Whether this identity sits below a parent root.
    pub const fn is_child(self) -> bool {
        matches!(self, Self::SubmoduleChild | Self::NestedIndependent)
    }
}

/// Explicit parent/child repo identity for a root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoIdentity {
    /// Identity class.
    pub kind: RepoIdentityKind,
    /// Stable id of this root; must match the owning descriptor.
    pub root_id: String,
    /// Parent root id, for child roots.
    pub parent_root_id: Option<String>,
    /// Redaction-safe ref to the parent gitlink, for submodule children.
    pub gitlink_path_ref: Option<String>,
    /// Redaction-safe ref to the commit a submodule gitlink pins.
    pub pinned_commit_ref: Option<String>,
    /// Whether a submodule child checkout is initialized.
    pub child_initialized: bool,
}

/// Worktree class for a root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorktreeKind {
    /// The primary worktree attached to the main `.git` directory.
    Primary,
    /// A linked worktree sharing the common Git directory.
    Linked,
}

impl WorktreeKind {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Linked => "linked",
        }
    }
}

/// Explicit worktree root for a descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorktreeScope {
    /// Worktree class.
    pub kind: WorktreeKind,
    /// Redaction-safe ref to the worktree root directory.
    pub worktree_root_ref: String,
    /// Redaction-safe ref to the shared common Git directory, for linked trees.
    pub common_dir_ref: Option<String>,
    /// Whether the worktree is locked.
    pub locked: bool,
}

/// Git LFS object state for a root's pointer-backed assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfsObjectState {
    /// No tracked Git LFS objects.
    NotApplicable,
    /// Only pointer metadata is local; content is not hydrated.
    PointerOnly,
    /// Some objects are hydrated and some remain pointer-only.
    PartiallyHydrated,
    /// All tracked objects are hydrated.
    Hydrated,
}

impl LfsObjectState {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::PointerOnly => "pointer_only",
            Self::PartiallyHydrated => "partially_hydrated",
            Self::Hydrated => "hydrated",
        }
    }
}

/// Explicit Git LFS object state for a root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LfsState {
    /// Object hydration state.
    pub state: LfsObjectState,
    /// Redaction-safe refs for pointer-only tracked paths.
    pub pointer_path_refs: Vec<String>,
}

impl LfsState {
    /// An LFS state for a root with no tracked LFS objects.
    pub fn not_applicable() -> Self {
        Self {
            state: LfsObjectState::NotApplicable,
            pointer_path_refs: Vec::new(),
        }
    }
}

/// Object availability for a root, separating present from promisor/unfetched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectAvailability {
    /// All referenced objects are materialized locally.
    FullyHydrated,
    /// Objects are backed by a promisor remote but currently present.
    PromisorBacked,
    /// Known objects are referenced but not materialized locally.
    MissingUnfetched,
}

impl ObjectAvailability {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyHydrated => "fully_hydrated",
            Self::PromisorBacked => "promisor_backed",
            Self::MissingUnfetched => "missing_unfetched",
        }
    }
}

/// Origin class for a generated or vendor root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedVendorClass {
    /// Content is generated by a build step or tool.
    Generated,
    /// Content is vendored from a third party into the tree.
    Vendored,
    /// Content is a tracked third-party import.
    ThirdPartyImport,
}

impl GeneratedVendorClass {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::Vendored => "vendored",
            Self::ThirdPartyImport => "third_party_import",
        }
    }
}

/// Explicit generated/vendor origin for a root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedVendorOrigin {
    /// Origin class.
    pub class: GeneratedVendorClass,
    /// Redaction-safe ref describing the origin (tool, upstream, or manifest).
    pub origin_ref: String,
    /// Whether content here is editable source truth; vendor/generated roots
    /// are intentionally outside editable truth.
    pub editable_truth: bool,
}

/// Root-level topology descriptor with explicit structured fields.
///
/// One descriptor exists per repository root. The structured fields are the
/// canonical substrate; [`TopologyRootDescriptor::project`] derives the truth a
/// surface renders, so every consumer reads the same boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyRootDescriptor {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable root id (referenced by bindings and child identities).
    pub root_id: String,
    /// Redaction-safe ref to the repository root directory.
    pub root_path_ref: String,
    /// Closed topology classes summarising this root's structured state.
    pub topology_classes: Vec<RepositoryTopologyClass>,
    /// Parent/child repo identity.
    pub repo_identity: RepoIdentity,
    /// Worktree root.
    pub worktree: WorktreeScope,
    /// Checkout filter class.
    pub filter_class: CheckoutFilterClass,
    /// Omitted-path set for sparse/workset slices.
    pub omitted_paths: OmittedPathSet,
    /// History depth boundary.
    pub depth_boundary: DepthBoundary,
    /// Object availability (present vs promisor/unfetched).
    pub object_availability: ObjectAvailability,
    /// Git LFS object state.
    pub lfs: LfsState,
    /// Generated/vendor origin, when the root is generated or vendored.
    pub generated_vendor: Option<GeneratedVendorOrigin>,
    /// Honesty labels every downstream binding must preserve.
    pub honesty_labels: Vec<TopologyHonestyLabel>,
    /// Safe mutation/export scope for this root.
    pub safe_operation_scope: TopologyOperationScope,
}

impl TopologyRootDescriptor {
    /// Whether this root's safe scope permits any content mutation.
    pub const fn permits_mutation(&self) -> bool {
        !matches!(
            self.safe_operation_scope,
            TopologyOperationScope::MetadataOnly | TopologyOperationScope::MutationDenied
        )
    }

    /// Result truth a surface renders for this root when it is targeted directly.
    ///
    /// The order is deterministic: universal content states (pointer-only,
    /// unfetched, uninitialized child, generated/vendor) win first, then the
    /// surface-specific history and path states. A pointer-only or unfetched
    /// object therefore never resolves to [`SurfaceResultTruth::Complete`].
    pub fn result_truth_for(&self, surface: TopologyConsumerSurface) -> SurfaceResultTruth {
        if self.lfs.state == LfsObjectState::PointerOnly {
            return SurfaceResultTruth::PointerOnly;
        }
        if self.object_availability == ObjectAvailability::MissingUnfetched {
            return SurfaceResultTruth::NotFetched;
        }
        if self.repo_identity.kind == RepoIdentityKind::SubmoduleChild
            && !self.repo_identity.child_initialized
        {
            return SurfaceResultTruth::Uninitialized;
        }
        if let Some(origin) = &self.generated_vendor {
            if !origin.editable_truth {
                return SurfaceResultTruth::GeneratedOrExcluded;
            }
        }
        if surface.reads_history() && self.depth_boundary.depth_class.is_bounded() {
            return SurfaceResultTruth::ShallowBoundary;
        }
        if surface.scopes_by_path() && self.filter_class.omits_paths() {
            return SurfaceResultTruth::OutsideCurrentSlice;
        }
        SurfaceResultTruth::Complete
    }

    /// Projects this descriptor onto one consumer surface for a caller's active
    /// root selection, producing the binding that surface renders.
    ///
    /// When the caller's `active_root_ref` is a different root than this one, the
    /// projection records a cross-root boundary (a nested-repo boundary or a
    /// wrong-target-root denial) rather than letting the surfaces flatten two
    /// roots into one scope.
    pub fn project(
        &self,
        surface: TopologyConsumerSurface,
        active_root_ref: &str,
        binding_id: impl Into<String>,
    ) -> SurfaceTopologyBinding {
        let authoritative_root_ref = self.root_id.clone();
        let wrong_root = active_root_ref != authoritative_root_ref;

        let (result_truth, coverage_claim) = if wrong_root {
            let truth = if self.repo_identity.kind == RepoIdentityKind::NestedIndependent {
                SurfaceResultTruth::NestedRoot
            } else {
                SurfaceResultTruth::WrongTargetRoot
            };
            let coverage = if surface.is_mutation_surface() {
                CoverageClaimPosture::DeniedWrongRoot
            } else {
                CoverageClaimPosture::NarrowedByTopology
            };
            (truth, coverage)
        } else {
            let truth = self.result_truth_for(surface);
            let coverage = if matches!(truth, SurfaceResultTruth::Complete) {
                CoverageClaimPosture::FullCoverageAllowed
            } else {
                CoverageClaimPosture::NarrowedByTopology
            };
            (truth, coverage)
        };

        let mut honesty_labels = self.honesty_labels.clone();
        if wrong_root {
            let extra = if self.repo_identity.kind == RepoIdentityKind::NestedIndependent {
                TopologyHonestyLabel::NestedRepoBoundary
            } else {
                TopologyHonestyLabel::WrongTargetRoot
            };
            if !honesty_labels.contains(&extra) {
                honesty_labels.push(extra);
            }
        }

        let mutation_scope = if wrong_root {
            TopologyOperationScope::MutationDenied
        } else {
            self.safe_operation_scope
        };

        let mutation_allowed = !wrong_root
            && surface.is_mutation_surface()
            && self.permits_mutation()
            && matches!(result_truth, SurfaceResultTruth::Complete);

        let body_export_allowed = surface.allows_body_export()
            && !wrong_root
            && matches!(result_truth, SurfaceResultTruth::Complete);

        SurfaceTopologyBinding {
            record_kind: GIT_TOPOLOGY_SURFACE_BINDING_RECORD_KIND.to_owned(),
            binding_id: binding_id.into(),
            surface,
            root_ref: self.root_id.clone(),
            active_root_ref: active_root_ref.to_owned(),
            authoritative_root_ref,
            result_truth,
            coverage_claim,
            honesty_labels,
            mutation_scope,
            mutation_allowed,
            body_export_allowed,
        }
    }
}

/// One consumer-surface binding derived from a [`TopologyRootDescriptor`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTopologyBinding {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable binding id.
    pub binding_id: String,
    /// Surface that renders this binding.
    pub surface: TopologyConsumerSurface,
    /// Referenced [`TopologyRootDescriptor::root_id`].
    pub root_ref: String,
    /// Root the caller selected as active.
    pub active_root_ref: String,
    /// Root that owns the content, result, or operation.
    pub authoritative_root_ref: String,
    /// Result truth rendered by the surface.
    pub result_truth: SurfaceResultTruth,
    /// Whether a complete-coverage claim is allowed.
    pub coverage_claim: CoverageClaimPosture,
    /// Honesty labels carried by the binding.
    pub honesty_labels: Vec<TopologyHonestyLabel>,
    /// Safe target scope for a mutation, export, or execution action.
    pub mutation_scope: TopologyOperationScope,
    /// Whether this binding may drive a content mutation.
    pub mutation_allowed: bool,
    /// Whether the surface may embed object body bytes in an export.
    pub body_export_allowed: bool,
}

/// Redaction-safe support-export projection for a topology map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologySupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Root ids included in the export.
    pub root_refs: Vec<String>,
    /// Binding ids included in the export.
    pub binding_refs: Vec<String>,
    /// Structured fields retained after redaction.
    pub reconstruction_fields: Vec<String>,
    /// True when no raw paths are embedded.
    pub raw_paths_redacted: bool,
    /// True when no raw object bytes are embedded.
    pub raw_object_bytes_redacted: bool,
}

/// Top-level canonical map binding explicit descriptors to their first consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryTopologyMap {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable map id.
    pub map_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Root-level explicit descriptors.
    pub roots: Vec<TopologyRootDescriptor>,
    /// Per-surface bindings derived from the descriptors.
    pub surface_bindings: Vec<SurfaceTopologyBinding>,
    /// Redaction-safe support-export projection.
    pub support_export: TopologySupportExport,
}

impl RepositoryTopologyMap {
    /// Parses a map from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`GitTopologyError`] when the JSON is invalid or the parsed map
    /// violates the topology contract.
    pub fn parse_json(input: &str) -> Result<Self, GitTopologyError> {
        let map: Self = serde_json::from_str(input).map_err(GitTopologyError::Json)?;
        let violations = map.validate();
        if violations.is_empty() {
            Ok(map)
        } else {
            Err(GitTopologyError::Validation(violations))
        }
    }

    /// Validates every descriptor, binding, and support-export invariant.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<GitTopologyValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != GIT_TOPOLOGY_MAP_RECORD_KIND {
            errors.push(GitTopologyValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != GIT_TOPOLOGY_SCHEMA_VERSION {
            errors.push(GitTopologyValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.map_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            errors.push(GitTopologyValidationError::MissingIdentity);
        }
        if self.roots.is_empty() {
            errors.push(GitTopologyValidationError::NoRoots);
        }

        let mut root_ids: HashSet<&str> = HashSet::new();
        for root in &self.roots {
            if !root_ids.insert(root.root_id.as_str()) {
                errors.push(GitTopologyValidationError::DuplicateRootId {
                    root_id: root.root_id.clone(),
                });
            }
            validate_root(root, &mut errors);
        }

        let mut binding_ids: HashSet<&str> = HashSet::new();
        for binding in &self.surface_bindings {
            if !binding_ids.insert(binding.binding_id.as_str()) {
                errors.push(GitTopologyValidationError::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            validate_binding(binding, &self.roots, &mut errors);
        }

        validate_support_export(self, &root_ids, &binding_ids, &mut errors);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("git topology map serializes"),
        ) {
            errors.push(GitTopologyValidationError::RawBoundaryMaterialInExport);
        }

        errors
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only map fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("git topology map serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Repository Topology Descriptors and First Consumers\n\n");
        out.push_str(&format!("- Map: `{}`\n", self.map_id));
        out.push_str(&format!(
            "- Roots: {} / Surface bindings: {}\n",
            self.roots.len(),
            self.surface_bindings.len()
        ));

        out.push_str("\n## Roots\n\n");
        for root in &self.roots {
            out.push_str(&format!(
                "- **{}** ({}): filter `{}`, depth `{}`, lfs `{}`, scope `{}`\n",
                root.root_id,
                root.repo_identity.kind.as_str(),
                root.filter_class.as_str(),
                root.depth_boundary.depth_class.as_str(),
                root.lfs.state.as_str(),
                operation_scope_token(root.safe_operation_scope),
            ));
        }

        out.push_str("\n## Surface bindings\n\n");
        for binding in &self.surface_bindings {
            out.push_str(&format!(
                "- **{}** → `{}`: truth `{}`, coverage `{}`, mutation {}\n",
                binding.surface.as_str(),
                binding.root_ref,
                surface_result_token(binding.result_truth),
                coverage_token(binding.coverage_claim),
                binding.mutation_allowed,
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical first-consumers map.
///
/// # Errors
///
/// Returns [`GitTopologyError`] when the checked-in map fails to parse or
/// violates the topology contract.
pub fn current_git_topology_first_consumers_map() -> Result<RepositoryTopologyMap, GitTopologyError>
{
    RepositoryTopologyMap::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/git_topology/topology_first_consumers.json"
    )))
}

fn validate_root(root: &TopologyRootDescriptor, errors: &mut Vec<GitTopologyValidationError>) {
    if root.record_kind != GIT_TOPOLOGY_ROOT_DESCRIPTOR_RECORD_KIND {
        errors.push(GitTopologyValidationError::WrongRecordKind {
            observed: root.record_kind.clone(),
        });
    }
    if root.topology_classes.is_empty() {
        errors.push(GitTopologyValidationError::RootMissingTopologyClass {
            root_id: root.root_id.clone(),
        });
    }
    if root.repo_identity.root_id != root.root_id {
        errors.push(GitTopologyValidationError::IdentityRootIdMismatch {
            root_id: root.root_id.clone(),
        });
    }
    if root.repo_identity.kind.is_child() && root.repo_identity.parent_root_id.is_none() {
        errors.push(GitTopologyValidationError::ChildMissingParent {
            root_id: root.root_id.clone(),
        });
    }
    if root.repo_identity.kind == RepoIdentityKind::SubmoduleChild
        && root.repo_identity.gitlink_path_ref.is_none()
    {
        errors.push(GitTopologyValidationError::SubmoduleMissingGitlink {
            root_id: root.root_id.clone(),
        });
    }

    let labels: HashSet<TopologyHonestyLabel> = root.honesty_labels.iter().copied().collect();
    let require = |label: TopologyHonestyLabel,
                   present: bool,
                   errors: &mut Vec<GitTopologyValidationError>| {
        if present && !labels.contains(&label) {
            errors.push(GitTopologyValidationError::RootMissingHonestyLabel {
                root_id: root.root_id.clone(),
                label,
            });
        }
    };
    require(
        TopologyHonestyLabel::PointerOnly,
        root.lfs.state == LfsObjectState::PointerOnly,
        errors,
    );
    require(
        TopologyHonestyLabel::NotFetched,
        root.object_availability == ObjectAvailability::MissingUnfetched,
        errors,
    );
    require(
        TopologyHonestyLabel::SubmoduleNotInitialized,
        root.repo_identity.kind == RepoIdentityKind::SubmoduleChild
            && !root.repo_identity.child_initialized,
        errors,
    );
    require(
        TopologyHonestyLabel::ShallowBoundary,
        root.depth_boundary.depth_class.is_bounded(),
        errors,
    );
    require(
        TopologyHonestyLabel::OutsideCurrentSlice,
        root.filter_class.omits_paths(),
        errors,
    );
    require(
        TopologyHonestyLabel::GeneratedOrExcluded,
        root.generated_vendor
            .as_ref()
            .is_some_and(|origin| !origin.editable_truth),
        errors,
    );
    require(
        TopologyHonestyLabel::NestedRepoBoundary,
        root.repo_identity.kind == RepoIdentityKind::NestedIndependent,
        errors,
    );

    // A filter that omits paths must record the omitted set so counts stay honest.
    if root.filter_class.omits_paths() && root.omitted_paths.is_empty() {
        errors.push(GitTopologyValidationError::OmittedSetMissing {
            root_id: root.root_id.clone(),
        });
    }
    // Shallow/grafted history must record its boundary edge.
    if root.depth_boundary.depth_class.is_bounded()
        && root.depth_boundary.shallow_boundary_refs.is_empty()
    {
        errors.push(GitTopologyValidationError::DepthBoundaryMissingEdge {
            root_id: root.root_id.clone(),
        });
    }

    // Scope guardrails: content that cannot be edited must not advertise mutation.
    let read_only_required = root.lfs.state == LfsObjectState::PointerOnly
        || root
            .generated_vendor
            .as_ref()
            .is_some_and(|origin| !origin.editable_truth)
        || (root.repo_identity.kind == RepoIdentityKind::SubmoduleChild
            && !root.repo_identity.child_initialized);
    if read_only_required && root.permits_mutation() {
        errors.push(GitTopologyValidationError::ReadOnlyRootPermitsMutation {
            root_id: root.root_id.clone(),
        });
    }
}

fn validate_binding(
    binding: &SurfaceTopologyBinding,
    roots: &[TopologyRootDescriptor],
    errors: &mut Vec<GitTopologyValidationError>,
) {
    if binding.record_kind != GIT_TOPOLOGY_SURFACE_BINDING_RECORD_KIND {
        errors.push(GitTopologyValidationError::WrongRecordKind {
            observed: binding.record_kind.clone(),
        });
    }
    let Some(root) = roots.iter().find(|root| root.root_id == binding.root_ref) else {
        errors.push(GitTopologyValidationError::UnknownBindingRoot {
            binding_id: binding.binding_id.clone(),
            root_ref: binding.root_ref.clone(),
        });
        return;
    };

    // The binding must equal the deterministic projection of its descriptor; this
    // is what proves the same descriptors drive every surface.
    let expected = root.project(
        binding.surface,
        &binding.active_root_ref,
        binding.binding_id.clone(),
    );
    if &expected != binding {
        errors.push(GitTopologyValidationError::BindingDoesNotMatchDescriptor {
            binding_id: binding.binding_id.clone(),
        });
    }

    // Guardrail: a pointer-only or unfetched binding never claims complete or
    // fully-hydrated truth, and never permits mutation.
    let partial_truth = matches!(
        binding.result_truth,
        SurfaceResultTruth::PointerOnly | SurfaceResultTruth::NotFetched
    );
    let claims_complete = matches!(
        binding.coverage_claim,
        CoverageClaimPosture::FullCoverageAllowed
    ) || binding.mutation_allowed
        || binding.body_export_allowed;
    if partial_truth && claims_complete {
        errors.push(GitTopologyValidationError::PartialBindingClaimsComplete {
            binding_id: binding.binding_id.clone(),
        });
    }

    // Guardrail: a wrong-root binding never mutates and never flattens roots.
    if binding.active_root_ref != binding.authoritative_root_ref && binding.mutation_allowed {
        errors.push(GitTopologyValidationError::WrongRootPermitsMutation {
            binding_id: binding.binding_id.clone(),
        });
    }
}

fn validate_support_export(
    map: &RepositoryTopologyMap,
    root_ids: &HashSet<&str>,
    binding_ids: &HashSet<&str>,
    errors: &mut Vec<GitTopologyValidationError>,
) {
    let export = &map.support_export;
    if export.record_kind != GIT_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(GitTopologyValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for root_ref in &export.root_refs {
        if !root_ids.contains(root_ref.as_str()) {
            errors.push(GitTopologyValidationError::UnknownSupportRootRef {
                root_ref: root_ref.clone(),
            });
        }
    }
    for binding_ref in &export.binding_refs {
        if !binding_ids.contains(binding_ref.as_str()) {
            errors.push(GitTopologyValidationError::UnknownSupportBindingRef {
                binding_ref: binding_ref.clone(),
            });
        }
    }
    for required in GIT_TOPOLOGY_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(GitTopologyValidationError::SupportExportMissingField {
                field: required.to_string(),
            });
        }
    }
    if !export.raw_paths_redacted || !export.raw_object_bytes_redacted {
        errors.push(GitTopologyValidationError::SupportExportEmbedsRawMaterial);
    }
}

/// Stable token for a [`TopologyOperationScope`] reused from the topology packet.
fn operation_scope_token(scope: TopologyOperationScope) -> &'static str {
    match scope {
        TopologyOperationScope::ActiveRootOnly => "active_root_only",
        TopologyOperationScope::ChildRootOnly => "child_root_only",
        TopologyOperationScope::ExplicitMultiRootPreviewRequired => {
            "explicit_multi_root_preview_required"
        }
        TopologyOperationScope::MetadataOnly => "metadata_only",
        TopologyOperationScope::MutationDenied => "mutation_denied",
    }
}

/// Stable token for a [`SurfaceResultTruth`] reused from the topology packet.
fn surface_result_token(truth: SurfaceResultTruth) -> &'static str {
    match truth {
        SurfaceResultTruth::Complete => "complete",
        SurfaceResultTruth::OutsideCurrentSlice => "outside_current_slice",
        SurfaceResultTruth::NotFetched => "not_fetched",
        SurfaceResultTruth::ShallowBoundary => "shallow_boundary",
        SurfaceResultTruth::Uninitialized => "uninitialized",
        SurfaceResultTruth::NestedRoot => "nested_root",
        SurfaceResultTruth::PointerOnly => "pointer_only",
        SurfaceResultTruth::GeneratedOrExcluded => "generated_or_excluded",
        SurfaceResultTruth::WrongTargetRoot => "wrong_target_root",
        SurfaceResultTruth::Unavailable => "unavailable",
    }
}

/// Stable token for a [`CoverageClaimPosture`] reused from the topology packet.
fn coverage_token(coverage: CoverageClaimPosture) -> &'static str {
    match coverage {
        CoverageClaimPosture::FullCoverageAllowed => "full_coverage_allowed",
        CoverageClaimPosture::NarrowedByTopology => "narrowed_by_topology",
        CoverageClaimPosture::DeniedWrongRoot => "denied_wrong_root",
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Error returned while parsing a topology map.
#[derive(Debug)]
pub enum GitTopologyError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<GitTopologyValidationError>),
}

impl fmt::Display for GitTopologyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(formatter, "failed to parse git topology map JSON: {error}")
            }
            Self::Validation(errors) => {
                write!(formatter, "git topology map has validation errors: ")?;
                for (index, error) in errors.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, "; ")?;
                    }
                    write!(formatter, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for GitTopologyError {}

/// Cross-row validation error for a topology map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitTopologyValidationError {
    /// A record-kind tag does not match the stable contract.
    WrongRecordKind {
        /// Observed record-kind tag.
        observed: String,
    },
    /// The map schema version is unsupported.
    WrongSchemaVersion {
        /// Observed schema version.
        observed: u32,
    },
    /// A required identity field is missing.
    MissingIdentity,
    /// The map carries no roots.
    NoRoots,
    /// A root id is declared more than once.
    DuplicateRootId {
        /// Duplicated root id.
        root_id: String,
    },
    /// A binding id is declared more than once.
    DuplicateBindingId {
        /// Duplicated binding id.
        binding_id: String,
    },
    /// A root carries no topology class.
    RootMissingTopologyClass {
        /// Root id.
        root_id: String,
    },
    /// A root's identity root id does not match the descriptor id.
    IdentityRootIdMismatch {
        /// Root id.
        root_id: String,
    },
    /// A child root does not name its parent.
    ChildMissingParent {
        /// Root id.
        root_id: String,
    },
    /// A submodule child does not reference its parent gitlink.
    SubmoduleMissingGitlink {
        /// Root id.
        root_id: String,
    },
    /// A root omits an honesty label its structured state requires.
    RootMissingHonestyLabel {
        /// Root id.
        root_id: String,
        /// Required honesty label.
        label: TopologyHonestyLabel,
    },
    /// A path-omitting filter does not record its omitted set.
    OmittedSetMissing {
        /// Root id.
        root_id: String,
    },
    /// A bounded-history root does not record its boundary edge.
    DepthBoundaryMissingEdge {
        /// Root id.
        root_id: String,
    },
    /// A read-only root advertises a mutating scope.
    ReadOnlyRootPermitsMutation {
        /// Root id.
        root_id: String,
    },
    /// A binding references an unknown root.
    UnknownBindingRoot {
        /// Binding id.
        binding_id: String,
        /// Unknown root ref.
        root_ref: String,
    },
    /// A binding does not equal the projection of its descriptor.
    BindingDoesNotMatchDescriptor {
        /// Binding id.
        binding_id: String,
    },
    /// A pointer-only or unfetched binding claims complete or hydrated truth.
    PartialBindingClaimsComplete {
        /// Binding id.
        binding_id: String,
    },
    /// A wrong-root binding permits mutation.
    WrongRootPermitsMutation {
        /// Binding id.
        binding_id: String,
    },
    /// A support-export root ref is unknown.
    UnknownSupportRootRef {
        /// Unknown root ref.
        root_ref: String,
    },
    /// A support-export binding ref is unknown.
    UnknownSupportBindingRef {
        /// Unknown binding ref.
        binding_ref: String,
    },
    /// The support export omits a required reconstruction field.
    SupportExportMissingField {
        /// Missing reconstruction field.
        field: String,
    },
    /// The support export embeds raw paths or raw object bytes.
    SupportExportEmbedsRawMaterial,
    /// The export contains obviously forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl fmt::Display for GitTopologyValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { observed } => {
                write!(formatter, "unexpected record kind {observed}")
            }
            Self::WrongSchemaVersion { observed } => {
                write!(formatter, "unsupported schema version {observed}")
            }
            Self::MissingIdentity => write!(formatter, "map is missing identity fields"),
            Self::NoRoots => write!(formatter, "map carries no roots"),
            Self::DuplicateRootId { root_id } => {
                write!(formatter, "root id {root_id} is declared more than once")
            }
            Self::DuplicateBindingId { binding_id } => {
                write!(
                    formatter,
                    "binding id {binding_id} is declared more than once"
                )
            }
            Self::RootMissingTopologyClass { root_id } => {
                write!(formatter, "root {root_id} has no topology class")
            }
            Self::IdentityRootIdMismatch { root_id } => {
                write!(formatter, "root {root_id} identity root id mismatch")
            }
            Self::ChildMissingParent { root_id } => {
                write!(formatter, "child root {root_id} does not name a parent")
            }
            Self::SubmoduleMissingGitlink { root_id } => {
                write!(formatter, "submodule root {root_id} has no gitlink ref")
            }
            Self::RootMissingHonestyLabel { root_id, label } => write!(
                formatter,
                "root {root_id} is missing required honesty label {}",
                label.as_str()
            ),
            Self::OmittedSetMissing { root_id } => {
                write!(
                    formatter,
                    "root {root_id} omits paths without an omitted set"
                )
            }
            Self::DepthBoundaryMissingEdge { root_id } => {
                write!(
                    formatter,
                    "root {root_id} is bounded without a boundary edge"
                )
            }
            Self::ReadOnlyRootPermitsMutation { root_id } => {
                write!(formatter, "read-only root {root_id} permits mutation")
            }
            Self::UnknownBindingRoot {
                binding_id,
                root_ref,
            } => write!(
                formatter,
                "binding {binding_id} references unknown root {root_ref}"
            ),
            Self::BindingDoesNotMatchDescriptor { binding_id } => write!(
                formatter,
                "binding {binding_id} does not match its descriptor projection"
            ),
            Self::PartialBindingClaimsComplete { binding_id } => write!(
                formatter,
                "partial binding {binding_id} claims complete or hydrated truth"
            ),
            Self::WrongRootPermitsMutation { binding_id } => {
                write!(
                    formatter,
                    "wrong-root binding {binding_id} permits mutation"
                )
            }
            Self::UnknownSupportRootRef { root_ref } => {
                write!(
                    formatter,
                    "support export references unknown root {root_ref}"
                )
            }
            Self::UnknownSupportBindingRef { binding_ref } => write!(
                formatter,
                "support export references unknown binding {binding_ref}"
            ),
            Self::SupportExportMissingField { field } => {
                write!(
                    formatter,
                    "support export missing reconstruction field {field}"
                )
            }
            Self::SupportExportEmbedsRawMaterial => {
                write!(
                    formatter,
                    "support export embeds raw paths or raw object bytes"
                )
            }
            Self::RawBoundaryMaterialInExport => {
                write!(formatter, "export contains forbidden boundary material")
            }
        }
    }
}
