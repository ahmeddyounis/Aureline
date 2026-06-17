//! Cross-crate coverage for the topology descriptors and their first consumers.
//!
//! This exercises the public surface the way a downstream consumer (search,
//! review, blame, AI context, or support/export) would: load the canonical map,
//! ask each descriptor how a surface should render it, and confirm that topology
//! boundaries never flatten.

use std::path::{Path, PathBuf};

use aureline_git::{
    current_git_topology_first_consumers_map, CoverageClaimPosture, RepositoryTopologyMap,
    SurfaceResultTruth, TopologyConsumerSurface, TopologyOperationScope,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/m5/topology-corpus")
}

fn load_fixture(name: &str) -> RepositoryTopologyMap {
    let path = fixtures_dir().join(name);
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    RepositoryTopologyMap::parse_json(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse and validate: {error}"))
}

#[test]
fn checked_map_validates_and_drives_every_surface() {
    let map = current_git_topology_first_consumers_map().expect("checked map validates");
    for root in &map.roots {
        for surface in TopologyConsumerSurface::ALL {
            // Every descriptor can be projected onto every first consumer.
            let binding = root.project(surface, &root.root_id, "probe");
            assert_eq!(binding.surface, surface);
            assert_eq!(binding.authoritative_root_ref, root.root_id);
        }
    }
}

#[test]
fn each_topology_state_is_distinguishable_by_consumers() {
    let map = current_git_topology_first_consumers_map().expect("checked map validates");
    let truth = |root_id: &str, surface: TopologyConsumerSurface| {
        map.roots
            .iter()
            .find(|root| root.root_id == root_id)
            .expect("root present")
            .result_truth_for(surface)
    };

    assert_eq!(
        truth("sparse", TopologyConsumerSurface::SearchScope),
        SurfaceResultTruth::OutsideCurrentSlice
    );
    assert_eq!(
        truth("partial", TopologyConsumerSurface::GitStatus),
        SurfaceResultTruth::NotFetched
    );
    assert_eq!(
        truth("shallow", TopologyConsumerSurface::Blame),
        SurfaceResultTruth::ShallowBoundary
    );
    assert_eq!(
        truth("submodule", TopologyConsumerSurface::Review),
        SurfaceResultTruth::Uninitialized
    );
    assert_eq!(
        truth("lfs", TopologyConsumerSurface::AiContext),
        SurfaceResultTruth::PointerOnly
    );
    assert_eq!(
        truth("generated", TopologyConsumerSurface::Review),
        SurfaceResultTruth::GeneratedOrExcluded
    );
}

#[test]
fn parent_child_boundary_is_never_flattened_for_consumers() {
    let map = current_git_topology_first_consumers_map().expect("checked map validates");
    let submodule = map
        .roots
        .iter()
        .find(|root| root.root_id == "submodule")
        .expect("submodule present");

    // A mutate-class consumer that targets the child while the parent is active
    // is denied; the scopes never merge.
    let review = submodule.project(TopologyConsumerSurface::Review, "main", "probe");
    assert_eq!(review.result_truth, SurfaceResultTruth::WrongTargetRoot);
    assert_eq!(review.coverage_claim, CoverageClaimPosture::DeniedWrongRoot);
    assert!(!review.mutation_allowed);
    assert_eq!(
        review.mutation_scope,
        TopologyOperationScope::MutationDenied
    );
}

#[test]
fn pointer_only_fixture_never_claims_hydrated_truth() {
    let map = load_fixture("lfs_pointer_only.json");
    for binding in &map.surface_bindings {
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
fn submodule_fixture_blocks_mutation_for_every_consumer() {
    let map = load_fixture("submodule_uninitialized_narrowed.json");
    let submodule = map
        .roots
        .iter()
        .find(|root| root.root_id == "submodule")
        .expect("submodule present");
    assert!(!submodule.permits_mutation());
    for binding in map
        .surface_bindings
        .iter()
        .filter(|binding| binding.root_ref == "submodule")
    {
        assert!(!binding.mutation_allowed);
    }
}
