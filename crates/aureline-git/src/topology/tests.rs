use super::*;

const CANONICAL_MAP: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/git/m5/git_topology/topology_first_consumers.json"
));

const SUBMODULE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/topology-corpus/submodule_uninitialized_narrowed.json"
));

const LFS_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/topology-corpus/lfs_pointer_only.json"
));

fn baseline() -> RepositoryTopologyMap {
    serde_json::from_str(CANONICAL_MAP).expect("canonical map deserializes")
}

fn root<'a>(map: &'a RepositoryTopologyMap, root_id: &str) -> &'a TopologyRootDescriptor {
    map.roots
        .iter()
        .find(|root| root.root_id == root_id)
        .expect("root present")
}

fn binding<'a>(
    map: &'a RepositoryTopologyMap,
    surface: TopologyConsumerSurface,
    root_ref: &str,
    active_root_ref: &str,
) -> &'a SurfaceTopologyBinding {
    map.surface_bindings
        .iter()
        .find(|binding| {
            binding.surface == surface
                && binding.root_ref == root_ref
                && binding.active_root_ref == active_root_ref
        })
        .expect("binding present")
}

#[test]
fn checked_artifact_validates() {
    let map =
        current_git_topology_first_consumers_map().expect("checked topology map validates clean");
    assert_eq!(map.map_id, "git-topology-first-consumers:0001");
}

#[test]
fn canonical_map_validates_clean() {
    let map = baseline();
    assert!(map.validate().is_empty(), "{:?}", map.validate());
}

#[test]
fn canonical_map_round_trips() {
    let map = baseline();
    let reparsed = RepositoryTopologyMap::parse_json(&map.export_safe_json())
        .expect("export round-trips through parse_json");
    assert_eq!(map, reparsed);
}

#[test]
fn fixtures_validate() {
    for raw in [SUBMODULE_FIXTURE, LFS_FIXTURE] {
        let map = RepositoryTopologyMap::parse_json(raw).expect("fixture parses and validates");
        assert!(map.validate().is_empty(), "{:?}", map.validate());
    }
}

#[test]
fn every_consumer_surface_is_bound_for_each_root() {
    let map = baseline();
    for root in &map.roots {
        for surface in TopologyConsumerSurface::ALL {
            assert!(
                map.surface_bindings.iter().any(|binding| {
                    binding.surface == surface
                        && binding.root_ref == root.root_id
                        && binding.active_root_ref == root.root_id
                }),
                "missing {} binding for root {}",
                surface.as_str(),
                root.root_id
            );
        }
    }
}

#[test]
fn each_distinguishable_state_is_represented() {
    let map = baseline();
    let observed: Vec<SurfaceResultTruth> = map
        .surface_bindings
        .iter()
        .map(|binding| binding.result_truth)
        .collect();
    for required in [
        SurfaceResultTruth::Complete,
        SurfaceResultTruth::OutsideCurrentSlice,
        SurfaceResultTruth::NotFetched,
        SurfaceResultTruth::ShallowBoundary,
        SurfaceResultTruth::Uninitialized,
        SurfaceResultTruth::NestedRoot,
        SurfaceResultTruth::PointerOnly,
        SurfaceResultTruth::GeneratedOrExcluded,
        SurfaceResultTruth::WrongTargetRoot,
    ] {
        assert!(
            observed.contains(&required),
            "no binding renders {required:?}"
        );
    }
}

#[test]
fn projection_is_surface_aware() {
    let map = baseline();
    let sparse = root(&map, "sparse");
    // The sparse slice narrows path-scoped surfaces but not blame.
    assert_eq!(
        sparse.result_truth_for(TopologyConsumerSurface::SearchScope),
        SurfaceResultTruth::OutsideCurrentSlice
    );
    assert_eq!(
        sparse.result_truth_for(TopologyConsumerSurface::Blame),
        SurfaceResultTruth::Complete
    );
    let shallow = root(&map, "shallow");
    // The shallow boundary narrows history surfaces but not the working tree.
    assert_eq!(
        shallow.result_truth_for(TopologyConsumerSurface::Blame),
        SurfaceResultTruth::ShallowBoundary
    );
    assert_eq!(
        shallow.result_truth_for(TopologyConsumerSurface::GitStatus),
        SurfaceResultTruth::Complete
    );
}

#[test]
fn pointer_only_never_masquerades_as_hydrated() {
    let map = baseline();
    for binding in map
        .surface_bindings
        .iter()
        .filter(|binding| binding.root_ref == "lfs")
    {
        assert_eq!(binding.result_truth, SurfaceResultTruth::PointerOnly);
        assert_ne!(
            binding.coverage_claim,
            CoverageClaimPosture::FullCoverageAllowed
        );
        assert!(!binding.mutation_allowed);
        assert!(!binding.body_export_allowed);
    }
}

#[test]
fn unfetched_never_masquerades_as_not_found_or_complete() {
    let map = baseline();
    for binding in map
        .surface_bindings
        .iter()
        .filter(|binding| binding.root_ref == "partial")
    {
        assert_eq!(binding.result_truth, SurfaceResultTruth::NotFetched);
        assert_ne!(
            binding.coverage_claim,
            CoverageClaimPosture::FullCoverageAllowed
        );
        assert!(!binding.mutation_allowed);
    }
}

#[test]
fn parent_and_child_identity_stays_explicit_in_mutate_flows() {
    let map = baseline();
    // Targeting the submodule child while the parent is active is denied, not
    // flattened into the parent's mutation scope.
    let submodule_review = binding(&map, TopologyConsumerSurface::Review, "submodule", "main");
    assert_eq!(
        submodule_review.result_truth,
        SurfaceResultTruth::WrongTargetRoot
    );
    assert_eq!(
        submodule_review.coverage_claim,
        CoverageClaimPosture::DeniedWrongRoot
    );
    assert!(!submodule_review.mutation_allowed);
    assert_eq!(
        submodule_review.mutation_scope,
        TopologyOperationScope::MutationDenied
    );

    // The nested independent repo keeps its own boundary identity.
    let nested_status = binding(&map, TopologyConsumerSurface::GitStatus, "nested", "main");
    assert_eq!(nested_status.result_truth, SurfaceResultTruth::NestedRoot);
    assert!(nested_status
        .honesty_labels
        .contains(&TopologyHonestyLabel::NestedRepoBoundary));
}

#[test]
fn child_targeted_directly_is_complete_and_mutable() {
    let map = baseline();
    // A nested independent repo is a complete, mutable root when targeted as the
    // active root, distinct from the wrong-root denial above.
    let nested = binding(&map, TopologyConsumerSurface::Review, "nested", "nested");
    assert_eq!(nested.result_truth, SurfaceResultTruth::Complete);
    assert!(nested.mutation_allowed);
}

#[test]
fn project_is_deterministic() {
    let map = baseline();
    let nested = root(&map, "nested");
    let first = nested.project(TopologyConsumerSurface::Review, "main", "b");
    let second = nested.project(TopologyConsumerSurface::Review, "main", "b");
    assert_eq!(first, second);
}

#[test]
fn submodule_fixture_narrows_to_mutation_denied() {
    let map =
        RepositoryTopologyMap::parse_json(SUBMODULE_FIXTURE).expect("submodule fixture parses");
    let submodule = root(&map, "submodule");
    assert!(!submodule.permits_mutation());
    assert_eq!(
        submodule.safe_operation_scope,
        TopologyOperationScope::MutationDenied
    );
    for binding in map
        .surface_bindings
        .iter()
        .filter(|binding| binding.root_ref == "submodule" && binding.active_root_ref == "submodule")
    {
        assert_eq!(binding.result_truth, SurfaceResultTruth::Uninitialized);
        assert!(!binding.mutation_allowed);
    }
}

#[test]
fn lfs_fixture_is_pointer_only_everywhere() {
    let map = RepositoryTopologyMap::parse_json(LFS_FIXTURE).expect("lfs fixture parses");
    assert!(map
        .surface_bindings
        .iter()
        .all(|binding| binding.result_truth == SurfaceResultTruth::PointerOnly));
}

#[test]
fn tampered_binding_fails_validation() {
    let mut map = baseline();
    let target = map
        .surface_bindings
        .iter_mut()
        .find(|binding| binding.root_ref == "partial")
        .expect("partial binding present");
    // Forge a complete claim over an unfetched object.
    target.result_truth = SurfaceResultTruth::Complete;
    target.coverage_claim = CoverageClaimPosture::FullCoverageAllowed;
    let violations = map.validate();
    assert!(violations.iter().any(|error| matches!(
        error,
        GitTopologyValidationError::BindingDoesNotMatchDescriptor { .. }
    )));
}

#[test]
fn read_only_root_permitting_mutation_fails() {
    let mut map = baseline();
    root_mut(&mut map, "lfs").safe_operation_scope = TopologyOperationScope::ActiveRootOnly;
    assert!(map.validate().iter().any(|error| matches!(
        error,
        GitTopologyValidationError::ReadOnlyRootPermitsMutation { .. }
    )));
}

#[test]
fn missing_honesty_label_fails() {
    let mut map = baseline();
    root_mut(&mut map, "lfs").honesty_labels.clear();
    assert!(map.validate().iter().any(|error| matches!(
        error,
        GitTopologyValidationError::RootMissingHonestyLabel { .. }
    )));
}

#[test]
fn omitted_filter_without_set_fails() {
    let mut map = baseline();
    root_mut(&mut map, "sparse").omitted_paths = OmittedPathSet::none();
    assert!(map
        .validate()
        .iter()
        .any(|error| matches!(error, GitTopologyValidationError::OmittedSetMissing { .. })));
}

#[test]
fn bounded_history_without_boundary_edge_fails() {
    let mut map = baseline();
    root_mut(&mut map, "shallow")
        .depth_boundary
        .shallow_boundary_refs
        .clear();
    assert!(map.validate().iter().any(|error| matches!(
        error,
        GitTopologyValidationError::DepthBoundaryMissingEdge { .. }
    )));
}

#[test]
fn duplicate_root_fails() {
    let mut map = baseline();
    let dup = root(&map, "lfs").clone();
    map.roots.push(dup);
    assert!(map
        .validate()
        .iter()
        .any(|error| matches!(error, GitTopologyValidationError::DuplicateRootId { .. })));
}

#[test]
fn unknown_binding_root_fails() {
    let mut map = baseline();
    map.surface_bindings[0].root_ref = "ghost".to_owned();
    assert!(map
        .validate()
        .iter()
        .any(|error| matches!(error, GitTopologyValidationError::UnknownBindingRoot { .. })));
}

#[test]
fn support_export_missing_field_fails() {
    let mut map = baseline();
    map.support_export
        .reconstruction_fields
        .retain(|field| field != "lfs_object_state");
    assert!(map.validate().iter().any(|error| matches!(
        error,
        GitTopologyValidationError::SupportExportMissingField { .. }
    )));
}

#[test]
fn support_export_unredacted_fails() {
    let mut map = baseline();
    map.support_export.raw_paths_redacted = false;
    assert!(map
        .validate()
        .contains(&GitTopologyValidationError::SupportExportEmbedsRawMaterial));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut map = baseline();
    root_mut(&mut map, "lfs").root_path_ref = "leak bearer abc123".to_owned();
    assert!(map
        .validate()
        .contains(&GitTopologyValidationError::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_every_root() {
    let summary = baseline().render_markdown_summary();
    for root_id in [
        "main",
        "sparse",
        "partial",
        "shallow",
        "submodule",
        "nested",
        "lfs",
        "generated",
        "worktree",
    ] {
        assert!(summary.contains(root_id), "summary missing root {root_id}");
    }
}

fn root_mut<'a>(
    map: &'a mut RepositoryTopologyMap,
    root_id: &str,
) -> &'a mut TopologyRootDescriptor {
    map.roots
        .iter_mut()
        .find(|root| root.root_id == root_id)
        .expect("root present")
}
