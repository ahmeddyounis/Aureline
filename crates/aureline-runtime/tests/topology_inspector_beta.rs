use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    seeded_host_topology_inspector, seeded_lane_filtered_event_viewer,
    seeded_reattach_review_sheet, FaultDomainNextSafeActionClass, FaultDomainRestartCard,
    HostLaneFamily, HostLaneHealthClass, ReattachDriftFieldClass, ReattachReviewDecisionClass,
    RestartBudgetStateClass, RuntimeSurfaceClass, HOST_TOPOLOGY_SCHEMA_VERSION,
    TOPOLOGY_INSPECTOR_RECORD_KIND,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn topology_inspector_maps_required_surfaces_to_plain_language_host_lanes() {
    let inspector = seeded_host_topology_inspector();

    assert_eq!(inspector.record_kind, TOPOLOGY_INSPECTOR_RECORD_KIND);
    assert_eq!(inspector.schema_version, HOST_TOPOLOGY_SCHEMA_VERSION);
    assert_eq!(inspector.validate(), Vec::new());
    assert_eq!(
        inspector.missing_required_surface_tokens(),
        Vec::<String>::new()
    );

    let labels = inspector
        .lanes
        .iter()
        .map(|lane| lane.family_label.as_str())
        .collect::<Vec<_>>();
    for expected in [
        "Local shell service",
        "Language/analysis host",
        "Extension sandbox host",
        "Debug/task adapter host",
        "Notebook kernel",
        "Remote workspace agent",
        "Managed service lane",
    ] {
        assert!(
            labels.contains(&expected),
            "missing family label {expected}"
        );
    }

    for surface in RuntimeSurfaceClass::REQUIRED_INLINE_BADGE_SURFACES {
        let rows = inspector.results_for_surface(surface);
        assert!(!rows.is_empty(), "missing surface {}", surface.as_str());
        assert!(
            rows.iter().all(|row| !row.host_badge_groups.is_empty()),
            "surface {} must have host badges",
            surface.as_str()
        );
    }
}

#[test]
fn restart_cards_distinguish_analysis_refresh_from_mutating_reattach() {
    let inspector = seeded_host_topology_inspector();
    let analysis = inspector
        .lane("lane:language-analysis")
        .expect("analysis lane exists");
    let analysis_card = FaultDomainRestartCard::from_lane("card:analysis", analysis);
    assert_eq!(analysis.family, HostLaneFamily::LanguageAnalysisHost);
    assert_eq!(
        analysis_card.restart_budget_state,
        RestartBudgetStateClass::WithinBudget
    );
    assert!(analysis_card
        .next_safe_actions
        .contains(&FaultDomainNextSafeActionClass::RefreshAnalysis));

    let debug = inspector
        .lane("lane:debug-task-adapter")
        .expect("debug lane exists");
    let debug_card = FaultDomainRestartCard::from_lane("card:debug", debug);
    assert_eq!(debug.health_class, HostLaneHealthClass::Disconnected);
    assert_eq!(
        debug_card.restart_budget_state,
        RestartBudgetStateClass::ReattachReviewRequired
    );
    assert!(debug_card
        .next_safe_actions
        .contains(&FaultDomainNextSafeActionClass::ReviewReattach));

    let notebook = inspector
        .lane("lane:notebook-kernel")
        .expect("notebook lane exists");
    let notebook_card = FaultDomainRestartCard::from_lane("card:notebook", notebook);
    assert_eq!(
        notebook_card.restart_budget_state,
        RestartBudgetStateClass::Quarantined
    );
    assert!(notebook_card.blocks_healthy_claim());
}

#[test]
fn reattach_review_blocks_current_claim_on_host_policy_and_auth_drift() {
    let review = seeded_reattach_review_sheet();

    assert_eq!(
        review.decision,
        ReattachReviewDecisionClass::ReapprovalRequired
    );
    assert!(!review.current_lane_accepted);
    assert!(review.policy_drift_present);
    assert!(review.auth_drift_present);
    assert!(!review.preserved_state_refs.is_empty());
    assert!(!review.lost_state_refs.is_empty());

    let fields = review
        .drift_rows
        .iter()
        .map(|row| row.field)
        .collect::<Vec<_>>();
    assert!(fields.contains(&ReattachDriftFieldClass::TargetRef));
    assert!(fields.contains(&ReattachDriftFieldClass::PolicyEpoch));
    assert!(fields.contains(&ReattachDriftFieldClass::AuthScope));
}

#[test]
fn lane_filtered_events_preserve_restart_markers_and_provenance_links() {
    let viewer = seeded_lane_filtered_event_viewer();
    assert!(viewer.rows_preserve_provenance());
    assert!(viewer.restart_marker_count >= 4);
    assert!(viewer
        .rows
        .iter()
        .any(|row| row.restart_marker_token == "quarantine_entered"));
    assert!(viewer
        .rows
        .iter()
        .any(|row| row.restart_marker_token == "reattached_after_review"));

    let analysis_only = viewer.filter_by_lane("lane:language-analysis");
    assert!(analysis_only
        .rows
        .iter()
        .all(|row| row.host_lane_ref == "lane:language-analysis"));
    assert!(analysis_only
        .rows
        .iter()
        .any(|row| row.restart_marker_token == "restart_scheduled"));
}

#[test]
fn checked_in_topology_fixture_manifest_covers_required_drills() {
    let manifest_path =
        repo_root().join("fixtures/runtime/host_topology_and_reattach/manifest.yaml");
    let payload = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", manifest_path.display()));
    let manifest: FixtureManifest = serde_yaml::from_str(&payload).expect("manifest parses");

    assert_eq!(manifest.schema_version, HOST_TOPOLOGY_SCHEMA_VERSION);
    assert_eq!(
        manifest.record_kind,
        "host_topology_and_reattach_fixture_manifest"
    );
    for surface in RuntimeSurfaceClass::REQUIRED_INLINE_BADGE_SURFACES {
        assert!(manifest
            .required_surface_tokens
            .contains(&surface.as_str().to_owned()));
    }
    assert!(manifest.acceptance.values().all(|value| *value));
    for case_ref in manifest.case_refs {
        assert!(
            repo_root().join(&case_ref).is_file(),
            "fixture {case_ref} must exist"
        );
    }
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    record_kind: String,
    case_refs: Vec<String>,
    required_surface_tokens: Vec<String>,
    acceptance: BTreeMap<String, bool>,
}
