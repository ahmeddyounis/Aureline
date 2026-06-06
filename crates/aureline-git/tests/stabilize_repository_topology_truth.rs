//! Fixture-driven coverage for stable repository-topology truth.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use aureline_git::{
    CoverageClaimPosture, RepositoryTopologyClass, RepositoryTopologyTruthPacket,
    SurfaceResultTruth, TopologyActionApproval, TopologyActionClass, TopologyHonestyLabel,
    TopologySurface,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/review/m4/stabilize-repository-topology-truth")
}

fn load_packet(name: &str) -> RepositoryTopologyTruthPacket {
    let path = fixtures_dir().join(name);
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    RepositoryTopologyTruthPacket::parse_json(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse and validate: {error}"))
}

#[test]
fn stable_topology_packet_covers_required_descriptors_and_surfaces() {
    let packet = load_packet("stable_cross_surface_topology_packet.json");

    let classes: HashSet<_> = packet
        .descriptors
        .iter()
        .flat_map(|descriptor| descriptor.topology_classes.iter().copied())
        .collect();
    for required in [
        RepositoryTopologyClass::CurrentRepoRoot,
        RepositoryTopologyClass::WorksetRoot,
        RepositoryTopologyClass::SparseCheckoutRoot,
        RepositoryTopologyClass::PartialClonePromisorRoot,
        RepositoryTopologyClass::ShallowHistoryRoot,
        RepositoryTopologyClass::SubmoduleRoot,
        RepositoryTopologyClass::NestedIndependentRepoRoot,
        RepositoryTopologyClass::LfsHydrationBoundary,
        RepositoryTopologyClass::GeneratedVendorRoot,
    ] {
        assert!(
            classes.contains(&required),
            "fixture must cover {}",
            required.as_str()
        );
    }

    let surfaces: HashSet<_> = packet.surface_rows.iter().map(|row| row.surface).collect();
    for required in [
        TopologySurface::Search,
        TopologySurface::GitGraph,
        TopologySurface::Review,
        TopologySurface::Blame,
        TopologySurface::CodeActions,
        TopologySurface::AiContext,
        TopologySurface::RunDebug,
        TopologySurface::SupportExport,
    ] {
        assert!(
            surfaces.contains(&required),
            "fixture must cover {}",
            required.as_str()
        );
    }
}

#[test]
fn partial_rows_do_not_claim_complete_surface_truth() {
    let packet = load_packet("stable_cross_surface_topology_packet.json");

    for row in &packet.surface_rows {
        if !row.honesty_labels.is_empty() {
            assert_ne!(
                row.coverage_claim,
                CoverageClaimPosture::FullCoverageAllowed,
                "{} must not claim full coverage while carrying topology labels",
                row.row_id
            );
        }
    }

    let search = packet
        .surface_rows
        .iter()
        .find(|row| row.surface == TopologySurface::Search)
        .expect("search row exists");
    assert_eq!(search.result_truth, SurfaceResultTruth::OutsideCurrentSlice);

    let graph = packet
        .surface_rows
        .iter()
        .find(|row| row.surface == TopologySurface::GitGraph)
        .expect("git graph row exists");
    assert_eq!(graph.result_truth, SurfaceResultTruth::NotFetched);
}

#[test]
fn network_bearing_actions_keep_approval_or_policy_posture() {
    let packet = load_packet("stable_cross_surface_topology_packet.json");
    let mut observed_network_actions = HashSet::new();

    for action in packet
        .descriptors
        .iter()
        .flat_map(|descriptor| descriptor.allowed_actions.iter())
        .chain(
            packet
                .surface_rows
                .iter()
                .flat_map(|row| row.offered_actions.iter()),
        )
    {
        if action.action_class.is_network_bearing() {
            observed_network_actions.insert(action.action_class);
            assert!(
                matches!(
                    action.approval,
                    TopologyActionApproval::ApprovalRequired
                        | TopologyActionApproval::Approved
                        | TopologyActionApproval::PolicyBlocked
                ),
                "{} must carry approval or policy posture",
                action.action_class.as_str()
            );
        }
    }

    for required in [
        TopologyActionClass::FetchMissingObjects,
        TopologyActionClass::DeepenHistory,
        TopologyActionClass::InitializeSubmodule,
        TopologyActionClass::HydrateLfsObjects,
    ] {
        assert!(
            observed_network_actions.contains(&required),
            "fixture must cover network action {}",
            required.as_str()
        );
    }
}

#[test]
fn root_identity_and_lfs_hydration_states_remain_distinct() {
    let packet = load_packet("stable_cross_surface_topology_packet.json");

    let nested = packet
        .surface_rows
        .iter()
        .find(|row| row.row_id == "row:code-actions:nested-wrong-root")
        .expect("nested wrong-root row exists");
    assert_ne!(nested.active_root_ref, nested.authoritative_root_ref);
    assert_eq!(nested.coverage_claim, CoverageClaimPosture::DeniedWrongRoot);
    assert!(nested
        .honesty_labels
        .contains(&TopologyHonestyLabel::WrongTargetRoot));

    let pointer = packet
        .descriptors
        .iter()
        .find(|descriptor| descriptor.descriptor_id == "topology:lfs-pointer")
        .expect("pointer-only LFS descriptor exists");
    assert!(pointer.lfs_pointer_scope_ref.is_some());
    assert!(pointer
        .honesty_labels
        .contains(&TopologyHonestyLabel::PointerOnly));

    let hydrated = packet
        .descriptors
        .iter()
        .find(|descriptor| descriptor.descriptor_id == "topology:lfs-hydrated")
        .expect("hydrated LFS descriptor exists");
    assert!(hydrated.lfs_pointer_scope_ref.is_none());
    assert!(hydrated.honesty_labels.is_empty());
}

#[test]
fn support_export_preserves_reconstruction_fields() {
    let packet = load_packet("stable_cross_surface_topology_packet.json");
    let fields: HashSet<_> = packet
        .support_export
        .reconstruction_fields
        .iter()
        .map(String::as_str)
        .collect();

    for required in [
        "topology_class",
        "omitted_or_unfetched_scope",
        "chosen_action",
        "active_root_ref",
        "authoritative_root_ref",
        "parent_child_linkage",
        "shallow_boundary_ref",
        "lfs_pointer_scope_ref",
    ] {
        assert!(fields.contains(required), "support export keeps {required}");
    }

    assert!(packet.support_export.raw_paths_redacted);
    assert!(packet.support_export.raw_object_bytes_redacted);
}
