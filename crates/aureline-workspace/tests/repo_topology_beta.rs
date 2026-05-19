//! Fixture-driven coverage for the M3 repo-topology beta truth lane.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_workspace::{
    BodyExportPosture, FetchDepthDescriptor, FullCoverageBlocker, LfsHydrationDescriptor,
    MutationTarget, RepoRootDescriptor, RepoTopologyBetaInputs, RepoTopologyBetaProjection,
    RepoTopologySurface, SubmoduleLink, TopologyAffordanceClass,
};

#[derive(Debug, Clone, Deserialize)]
struct BetaFixture {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    surface: String,
    repo_root: RepoRootDescriptor,
    #[serde(default)]
    fetch_depth: Option<FetchDepthDescriptor>,
    #[serde(default)]
    submodule_links: Vec<SubmoduleLink>,
    #[serde(default)]
    lfs_hydration: Option<LfsHydrationDescriptor>,
    expect: ExpectedProjection,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    doc_sections: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedProjection {
    may_claim_full_coverage: bool,
    full_coverage_blockers: Vec<String>,
    required_affordances: Vec<String>,
    mutation_target: String,
    body_export_posture: String,
    honesty_labels: Vec<String>,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m3/repo_topology_and_partial_clone")
}

fn load_fixtures() -> Vec<(PathBuf, BetaFixture)> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("repo_topology beta fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let parsed: BetaFixture = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
            (path, parsed)
        })
        .collect()
}

fn surface_from_token(token: &str) -> RepoTopologySurface {
    match token {
        "workspace" => RepoTopologySurface::Workspace,
        "search" => RepoTopologySurface::Search,
        "graph" => RepoTopologySurface::Graph,
        "blame" => RepoTopologySurface::Blame,
        "review" => RepoTopologySurface::Review,
        "ai" => RepoTopologySurface::Ai,
        "execution" => RepoTopologySurface::Execution,
        "publish" => RepoTopologySurface::Publish,
        "support" => RepoTopologySurface::Support,
        "migration" => RepoTopologySurface::Migration,
        other => panic!("unknown surface token {other}"),
    }
}

fn blocker_as_str(blocker: &FullCoverageBlocker) -> &'static str {
    blocker.as_str()
}

fn affordance_as_str(affordance: &TopologyAffordanceClass) -> &'static str {
    affordance.as_str()
}

fn mutation_target_token(target: MutationTarget) -> &'static str {
    match target {
        MutationTarget::ParentRoot => "parent_root",
        MutationTarget::ChildRoot => "child_root",
        MutationTarget::SwitchRootRequired => "switch_root_required",
        MutationTarget::ReadOnlyUntilHydrated => "read_only_until_hydrated",
        MutationTarget::ReadOnlyUntilInitialized => "read_only_until_initialized",
        MutationTarget::PolicyBlocked => "policy_blocked",
    }
}

fn body_export_token(posture: BodyExportPosture) -> &'static str {
    match posture {
        BodyExportPosture::HydratedBytesAllowed => "hydrated_bytes_allowed",
        BodyExportPosture::PointerMetadataOnly => "pointer_metadata_only",
        BodyExportPosture::BlockedByPolicy => "blocked_by_policy",
        BodyExportPosture::Unavailable => "unavailable",
    }
}

#[test]
fn every_fixture_projects_to_expected_beta_truth() {
    let fixtures = load_fixtures();
    assert!(
        fixtures.len() >= 7,
        "beta fixture suite must keep at least 7 scenarios (found {})",
        fixtures.len()
    );

    for (path, fixture) in fixtures {
        let surface = surface_from_token(&fixture.surface);
        let projection = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
            repo_root: &fixture.repo_root,
            fetch_depth: fixture.fetch_depth.as_ref(),
            submodule_links: &fixture.submodule_links,
            lfs_hydration: fixture.lfs_hydration.as_ref(),
            surface,
        })
        .unwrap_or_else(|err| panic!("fixture {} failed to project: {err}", fixture.fixture.name));

        assert!(
            !fixture.fixture.scenario.is_empty(),
            "{path:?}: scenario must not be empty"
        );
        assert!(
            !fixture.fixture.doc_sections.is_empty(),
            "{path:?}: doc_sections must not be empty"
        );

        assert_eq!(
            projection.may_claim_full_coverage, fixture.expect.may_claim_full_coverage,
            "{}: may_claim_full_coverage mismatch",
            fixture.fixture.name
        );

        let observed_blockers: Vec<String> = projection
            .full_coverage_blockers
            .iter()
            .map(|b| blocker_as_str(b).to_string())
            .collect();
        assert_eq!(
            observed_blockers, fixture.expect.full_coverage_blockers,
            "{}: full_coverage_blockers mismatch",
            fixture.fixture.name
        );

        let observed_affordances: Vec<String> = projection
            .required_affordances
            .iter()
            .map(|a| affordance_as_str(a).to_string())
            .collect();
        assert_eq!(
            observed_affordances, fixture.expect.required_affordances,
            "{}: required_affordances mismatch",
            fixture.fixture.name
        );

        assert_eq!(
            mutation_target_token(projection.mutation_target),
            fixture.expect.mutation_target,
            "{}: mutation_target mismatch",
            fixture.fixture.name
        );

        assert_eq!(
            body_export_token(projection.body_export_posture),
            fixture.expect.body_export_posture,
            "{}: body_export_posture mismatch",
            fixture.fixture.name
        );

        assert_eq!(
            projection.honesty_labels, fixture.expect.honesty_labels,
            "{}: honesty_labels mismatch",
            fixture.fixture.name
        );

        let round_trip_root = serde_json::to_value(&fixture.repo_root)
            .and_then(serde_json::from_value::<RepoRootDescriptor>)
            .expect("repo_root must round-trip");
        assert_eq!(round_trip_root, fixture.repo_root);

        if let Some(fetch_depth) = &fixture.fetch_depth {
            let round_trip = serde_json::to_value(fetch_depth)
                .and_then(serde_json::from_value::<FetchDepthDescriptor>)
                .expect("fetch_depth must round-trip");
            assert_eq!(&round_trip, fetch_depth);
        }
        if let Some(lfs) = &fixture.lfs_hydration {
            let round_trip = serde_json::to_value(lfs)
                .and_then(serde_json::from_value::<LfsHydrationDescriptor>)
                .expect("lfs_hydration must round-trip");
            assert_eq!(&round_trip, lfs);
        }
        for link in &fixture.submodule_links {
            let round_trip = serde_json::to_value(link)
                .and_then(serde_json::from_value::<SubmoduleLink>)
                .expect("submodule_link must round-trip");
            assert_eq!(&round_trip, link);
        }
    }
}

#[test]
fn search_surface_with_partial_coverage_must_downgrade() {
    let fixtures = load_fixtures();
    let (_, sparse) = fixtures
        .iter()
        .find(|(_, f)| f.fixture.name == "linked_worktree_root")
        .expect("linked_worktree_root fixture must exist");

    let projection = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
        repo_root: &sparse.repo_root,
        fetch_depth: sparse.fetch_depth.as_ref(),
        submodule_links: &sparse.submodule_links,
        lfs_hydration: sparse.lfs_hydration.as_ref(),
        surface: RepoTopologySurface::Search,
    })
    .expect("must project");

    assert!(!projection.may_claim_full_coverage);
    assert!(aureline_workspace::surface_must_downgrade_claim(&projection));
}

#[test]
fn primary_full_local_truth_search_does_not_downgrade() {
    let fixtures = load_fixtures();
    let (_, full) = fixtures
        .iter()
        .find(|(_, f)| f.fixture.name == "primary_full_local_truth")
        .expect("primary_full_local_truth fixture must exist");

    let projection = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
        repo_root: &full.repo_root,
        fetch_depth: None,
        submodule_links: &[],
        lfs_hydration: None,
        surface: RepoTopologySurface::Search,
    })
    .expect("must project");

    assert!(projection.may_claim_full_coverage);
    assert!(!aureline_workspace::surface_must_downgrade_claim(&projection));
}
