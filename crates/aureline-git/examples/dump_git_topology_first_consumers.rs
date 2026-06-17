//! Conformance dump for the explicit repository-topology descriptors and their
//! first consumers.
//!
//! Prints the canonical export-safe [`RepositoryTopologyMap`] as deterministic
//! JSON. The optional first argument selects a narrowed fixture variant:
//!
//! * (no argument) — the canonical first-consumers map
//! * `submodule` — an uninitialized submodule child narrowed to mutation-denied
//! * `lfs` — a pointer-only Git LFS root that never claims hydrated truth
//!
//! The canonical document is the source of the checked-in artifact, and the two
//! variants are the source of the protected narrowing fixtures.

use aureline_git::{
    CheckoutFilterClass, DepthBoundary, GeneratedVendorClass, GeneratedVendorOrigin,
    HistoryDepthClass, LfsObjectState, LfsState, ObjectAvailability, OmittedPathSet, RepoIdentity,
    RepoIdentityKind, RepositoryTopologyClass, RepositoryTopologyMap, SurfaceTopologyBinding,
    TopologyConsumerSurface, TopologyHonestyLabel, TopologyOperationScope, TopologyRootDescriptor,
    TopologySupportExport, WorktreeKind, WorktreeScope, GIT_TOPOLOGY_MAP_RECORD_KIND,
    GIT_TOPOLOGY_REQUIRED_RECONSTRUCTION_FIELDS, GIT_TOPOLOGY_ROOT_DESCRIPTOR_RECORD_KIND,
    GIT_TOPOLOGY_SCHEMA_VERSION, GIT_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND,
};

const STAMP: &str = "2026-06-17T00:00:00Z";

/// Convenience builder for one root descriptor.
#[allow(clippy::too_many_arguments)]
fn root(
    root_id: &str,
    topology_classes: Vec<RepositoryTopologyClass>,
    repo_identity: RepoIdentity,
    worktree: WorktreeScope,
    filter_class: CheckoutFilterClass,
    omitted_paths: OmittedPathSet,
    depth_boundary: DepthBoundary,
    object_availability: ObjectAvailability,
    lfs: LfsState,
    generated_vendor: Option<GeneratedVendorOrigin>,
    honesty_labels: Vec<TopologyHonestyLabel>,
    safe_operation_scope: TopologyOperationScope,
) -> TopologyRootDescriptor {
    TopologyRootDescriptor {
        record_kind: GIT_TOPOLOGY_ROOT_DESCRIPTOR_RECORD_KIND.to_owned(),
        root_id: root_id.to_owned(),
        root_path_ref: format!("path-ref:{root_id}"),
        topology_classes,
        repo_identity,
        worktree,
        filter_class,
        omitted_paths,
        depth_boundary,
        object_availability,
        lfs,
        generated_vendor,
        honesty_labels,
        safe_operation_scope,
    }
}

fn standalone(root_id: &str) -> RepoIdentity {
    RepoIdentity {
        kind: RepoIdentityKind::Standalone,
        root_id: root_id.to_owned(),
        parent_root_id: None,
        gitlink_path_ref: None,
        pinned_commit_ref: None,
        child_initialized: true,
    }
}

fn primary_worktree(root_id: &str) -> WorktreeScope {
    WorktreeScope {
        kind: WorktreeKind::Primary,
        worktree_root_ref: format!("worktree-ref:{root_id}"),
        common_dir_ref: None,
        locked: false,
    }
}

fn roots() -> Vec<TopologyRootDescriptor> {
    vec![
        // Parent repository that owns the submodule and contains the nested repo.
        root(
            "main",
            vec![RepositoryTopologyClass::CurrentRepoRoot],
            RepoIdentity {
                kind: RepoIdentityKind::ParentWithChildren,
                root_id: "main".to_owned(),
                parent_root_id: None,
                gitlink_path_ref: None,
                pinned_commit_ref: None,
                child_initialized: true,
            },
            primary_worktree("main"),
            CheckoutFilterClass::FullCheckout,
            OmittedPathSet::none(),
            DepthBoundary::full(),
            ObjectAvailability::FullyHydrated,
            LfsState::not_applicable(),
            None,
            vec![],
            TopologyOperationScope::ActiveRootOnly,
        ),
        // Sparse checkout: paths outside the active slice are omitted, not missing.
        root(
            "sparse",
            vec![RepositoryTopologyClass::SparseCheckoutRoot],
            standalone("sparse"),
            primary_worktree("sparse"),
            CheckoutFilterClass::SparseCheckoutCone,
            OmittedPathSet {
                path_refs: vec![
                    "omitted-ref:vendor".to_owned(),
                    "omitted-ref:docs".to_owned(),
                ],
                omitted_estimate: Some(412),
            },
            DepthBoundary::full(),
            ObjectAvailability::FullyHydrated,
            LfsState::not_applicable(),
            None,
            vec![TopologyHonestyLabel::OutsideCurrentSlice],
            TopologyOperationScope::ActiveRootOnly,
        ),
        // Promisor partial clone: known objects are referenced but not fetched.
        root(
            "partial",
            vec![RepositoryTopologyClass::PartialClonePromisorRoot],
            standalone("partial"),
            primary_worktree("partial"),
            CheckoutFilterClass::PartialCloneBlobless,
            OmittedPathSet::none(),
            DepthBoundary::full(),
            ObjectAvailability::MissingUnfetched,
            LfsState::not_applicable(),
            None,
            vec![TopologyHonestyLabel::NotFetched],
            TopologyOperationScope::ActiveRootOnly,
        ),
        // Shallow history: blame and log stop at the shallow boundary.
        root(
            "shallow",
            vec![RepositoryTopologyClass::ShallowHistoryRoot],
            standalone("shallow"),
            primary_worktree("shallow"),
            CheckoutFilterClass::FullCheckout,
            OmittedPathSet::none(),
            DepthBoundary {
                depth_class: HistoryDepthClass::ShallowDepth,
                shallow_boundary_refs: vec!["boundary-ref:grafted-head".to_owned()],
                configured_depth: Some(50),
            },
            ObjectAvailability::FullyHydrated,
            LfsState::not_applicable(),
            None,
            vec![TopologyHonestyLabel::ShallowBoundary],
            TopologyOperationScope::ActiveRootOnly,
        ),
        // Submodule child pinned by the parent gitlink, currently uninitialized.
        root(
            "submodule",
            vec![RepositoryTopologyClass::SubmoduleRoot],
            RepoIdentity {
                kind: RepoIdentityKind::SubmoduleChild,
                root_id: "submodule".to_owned(),
                parent_root_id: Some("main".to_owned()),
                gitlink_path_ref: Some("gitlink-ref:libs/widget".to_owned()),
                pinned_commit_ref: Some("pin-ref:0".to_owned()),
                child_initialized: false,
            },
            primary_worktree("submodule"),
            CheckoutFilterClass::FullCheckout,
            OmittedPathSet::none(),
            DepthBoundary::full(),
            ObjectAvailability::FullyHydrated,
            LfsState::not_applicable(),
            None,
            vec![TopologyHonestyLabel::SubmoduleNotInitialized],
            TopologyOperationScope::MutationDenied,
        ),
        // Nested independent repository inside the parent working tree.
        root(
            "nested",
            vec![RepositoryTopologyClass::NestedIndependentRepoRoot],
            RepoIdentity {
                kind: RepoIdentityKind::NestedIndependent,
                root_id: "nested".to_owned(),
                parent_root_id: Some("main".to_owned()),
                gitlink_path_ref: None,
                pinned_commit_ref: None,
                child_initialized: true,
            },
            primary_worktree("nested"),
            CheckoutFilterClass::FullCheckout,
            OmittedPathSet::none(),
            DepthBoundary::full(),
            ObjectAvailability::FullyHydrated,
            LfsState::not_applicable(),
            None,
            vec![TopologyHonestyLabel::NestedRepoBoundary],
            TopologyOperationScope::ChildRootOnly,
        ),
        // Git LFS pointer-only asset root: only pointer metadata is local.
        root(
            "lfs",
            vec![RepositoryTopologyClass::LfsHydrationBoundary],
            standalone("lfs"),
            primary_worktree("lfs"),
            CheckoutFilterClass::FullCheckout,
            OmittedPathSet::none(),
            DepthBoundary::full(),
            ObjectAvailability::FullyHydrated,
            LfsState {
                state: LfsObjectState::PointerOnly,
                pointer_path_refs: vec!["pointer-ref:assets/model.bin".to_owned()],
            },
            None,
            vec![TopologyHonestyLabel::PointerOnly],
            TopologyOperationScope::MetadataOnly,
        ),
        // Generated / vendor root: intentionally outside editable source truth.
        root(
            "generated",
            vec![RepositoryTopologyClass::GeneratedVendorRoot],
            standalone("generated"),
            primary_worktree("generated"),
            CheckoutFilterClass::FullCheckout,
            OmittedPathSet::none(),
            DepthBoundary::full(),
            ObjectAvailability::FullyHydrated,
            LfsState::not_applicable(),
            Some(GeneratedVendorOrigin {
                class: GeneratedVendorClass::Generated,
                origin_ref: "origin-ref:codegen-pipeline".to_owned(),
                editable_truth: false,
            }),
            vec![TopologyHonestyLabel::GeneratedOrExcluded],
            TopologyOperationScope::MutationDenied,
        ),
        // Linked worktree: an explicit alternate working tree of the main repo.
        root(
            "worktree",
            vec![RepositoryTopologyClass::WorktreeRoot],
            standalone("worktree"),
            WorktreeScope {
                kind: WorktreeKind::Linked,
                worktree_root_ref: "worktree-ref:feature".to_owned(),
                common_dir_ref: Some("common-dir-ref:main".to_owned()),
                locked: false,
            },
            CheckoutFilterClass::FullCheckout,
            OmittedPathSet::none(),
            DepthBoundary::full(),
            ObjectAvailability::FullyHydrated,
            LfsState::not_applicable(),
            None,
            vec![],
            TopologyOperationScope::ActiveRootOnly,
        ),
    ]
}

/// Projects each root onto every surface as the directly-targeted (same-root)
/// view, then adds cross-root bindings that expose parent/child boundaries.
fn bindings(roots: &[TopologyRootDescriptor]) -> Vec<SurfaceTopologyBinding> {
    let mut out = Vec::new();
    for root in roots {
        for surface in TopologyConsumerSurface::ALL {
            let binding_id = format!(
                "binding-{}-{}-active-{}",
                surface.as_str(),
                root.root_id,
                root.root_id
            );
            out.push(root.project(surface, &root.root_id, binding_id));
        }
    }

    // Cross-root: the parent's active scope encounters its children and its
    // alternate worktree. These never flatten into one mutation scope.
    let by_id = |id: &str| roots.iter().find(|root| root.root_id == id).unwrap();
    let cross = [
        ("submodule", TopologyConsumerSurface::GitStatus),
        ("submodule", TopologyConsumerSurface::Review),
        ("submodule", TopologyConsumerSurface::AiContext),
        ("nested", TopologyConsumerSurface::GitStatus),
        ("nested", TopologyConsumerSurface::Review),
        ("nested", TopologyConsumerSurface::AiContext),
        ("worktree", TopologyConsumerSurface::GitStatus),
        ("worktree", TopologyConsumerSurface::Review),
    ];
    for (child_id, surface) in cross {
        let binding_id = format!("binding-{}-{}-active-main", surface.as_str(), child_id);
        out.push(by_id(child_id).project(surface, "main", binding_id));
    }
    out
}

fn support_export(
    roots: &[TopologyRootDescriptor],
    bindings: &[SurfaceTopologyBinding],
) -> TopologySupportExport {
    TopologySupportExport {
        record_kind: GIT_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: "git-topology-first-consumers-export:0001".to_owned(),
        root_refs: roots.iter().map(|root| root.root_id.clone()).collect(),
        binding_refs: bindings
            .iter()
            .map(|binding| binding.binding_id.clone())
            .collect(),
        reconstruction_fields: GIT_TOPOLOGY_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_object_bytes_redacted: true,
    }
}

fn canonical_map() -> RepositoryTopologyMap {
    let roots = roots();
    let bindings = bindings(&roots);
    let support_export = support_export(&roots, &bindings);
    RepositoryTopologyMap {
        record_kind: GIT_TOPOLOGY_MAP_RECORD_KIND.to_owned(),
        schema_version: GIT_TOPOLOGY_SCHEMA_VERSION,
        map_id: "git-topology-first-consumers:0001".to_owned(),
        generated_at: STAMP.to_owned(),
        roots,
        surface_bindings: bindings,
        support_export,
    }
}

/// A map restricted to the parent and the uninitialized submodule, narrowed to
/// mutation-denied truth across its surfaces.
fn submodule_variant() -> RepositoryTopologyMap {
    let mut map = canonical_map();
    map.map_id = "git-topology-first-consumers:submodule-uninitialized:0001".to_owned();
    map.roots
        .retain(|root| root.root_id == "main" || root.root_id == "submodule");
    map.surface_bindings
        .retain(|binding| binding.root_ref == "main" || binding.root_ref == "submodule");
    rebuild_support_export(&mut map);
    map
}

/// A map restricted to the pointer-only Git LFS root, which never claims
/// hydrated truth on any surface.
fn lfs_variant() -> RepositoryTopologyMap {
    let mut map = canonical_map();
    map.map_id = "git-topology-first-consumers:lfs-pointer-only:0001".to_owned();
    map.roots.retain(|root| root.root_id == "lfs");
    map.surface_bindings
        .retain(|binding| binding.root_ref == "lfs");
    rebuild_support_export(&mut map);
    map
}

fn rebuild_support_export(map: &mut RepositoryTopologyMap) {
    map.support_export.root_refs = map.roots.iter().map(|root| root.root_id.clone()).collect();
    map.support_export.binding_refs = map
        .surface_bindings
        .iter()
        .map(|binding| binding.binding_id.clone())
        .collect();
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let map = match variant.as_str() {
        "submodule" => submodule_variant(),
        "lfs" => lfs_variant(),
        _ => canonical_map(),
    };
    let violations = map.validate();
    assert!(
        violations.is_empty(),
        "topology map invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", map.render_markdown_summary());
    } else {
        println!("{}", map.export_safe_json());
    }
}
