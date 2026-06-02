//! Protected tests binding the typed shiproom dashboard to the checked-in artifact, the
//! frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in dashboard; the capture cross-check proves the typed
//! model and the Python gate agree on the publication verdict, the
//! claim-truth/qualification/public-proof/maintenance coverage counts, the
//! packet-freshness counts, and the fitness-function counts; the negative cases mutate a
//! parsed copy and the checked-in fixtures to prove that a panel which fails to narrow, a
//! green panel riding a breached packet, a panel rendered wider than its public claim's
//! ceiling, a fitness status that disagrees with its measurement, and a publication verdict
//! that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::shiproom_dashboard::{
    current_shiproom_dashboard, FitnessStatus, PanelKind, PanelState, ShiproomDashboard,
    ShiproomDashboardViolation, SHIPROOM_DASHBOARD_RECORD_KIND, SHIPROOM_DASHBOARD_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/shiproom_dashboard_validation_capture.json"
));

fn dashboard() -> ShiproomDashboard {
    current_shiproom_dashboard().expect("checked-in shiproom dashboard parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_dashboard_parses_and_validates() {
    let dashboard = dashboard();
    assert_eq!(dashboard.schema_version, SHIPROOM_DASHBOARD_SCHEMA_VERSION);
    assert_eq!(dashboard.record_kind, SHIPROOM_DASHBOARD_RECORD_KIND);
    let violations = dashboard.validate();
    assert!(
        violations.is_empty(),
        "checked-in dashboard must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_panel_kind() {
    let dashboard = dashboard();
    for kind in PanelKind::ALL {
        assert!(
            !dashboard.panels_for_kind(kind).is_empty(),
            "panel kind {} must have at least one dashboard row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_panel() {
    let dashboard = dashboard();
    assert!(!dashboard.release_blocking_panel_refs.is_empty());
    let covered: Vec<&str> = dashboard
        .release_blocking_panels()
        .into_iter()
        .map(|panel| panel.panel_ref.as_str())
        .collect();
    for declared in &dashboard.release_blocking_panel_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let dashboard = dashboard();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(dashboard.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_panels"].as_u64().unwrap() as usize,
        dashboard.panels.len(),
        "capture panel count must match the model"
    );
    assert_eq!(
        summary["panels_green_stable"].as_u64().unwrap() as usize,
        dashboard.panels_green_stable().len(),
        "capture green count must match the model"
    );
    for (key, kind) in [
        ("claim_truth_panels", PanelKind::ClaimTruth),
        ("qualification_panels", PanelKind::Qualification),
        ("public_proof_panels", PanelKind::PublicProof),
        ("maintenance_panels", PanelKind::Maintenance),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            dashboard.panels_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    let computed = dashboard.computed_summary();
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        computed.packets_breached,
        "capture breached-packet count must match the model"
    );
    for (key, value) in [
        ("fitness_pass", computed.fitness_pass),
        ("fitness_warn", computed.fitness_warn),
        ("fitness_fail", computed.fitness_fail),
        ("fitness_unmeasured", computed.fitness_unmeasured),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            value,
            "capture {key} must match the model"
        );
    }

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        dashboard.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        dashboard.publication.decision,
        dashboard.computed_publication_decision()
    );

    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
    let fixtures = capture["fixture_cases"].as_array().unwrap();
    assert!(!fixtures.is_empty(), "capture must record fixture cases");
    for case in fixtures {
        assert_eq!(
            case["status"].as_str(),
            Some("passed"),
            "frozen capture fixture case {} must have passed",
            case["case_id"]
        );
    }
}

#[test]
fn dashboard_narrows_a_panel_under_a_still_stable_claim() {
    let dashboard = dashboard();
    // A release-blocking panel whose public claim is still published Stable but is itself
    // narrowed — the shiproom-level truth beneath an optimistic claim.
    let narrowed = dashboard.panels.iter().find(|panel| {
        panel.release_blocking
            && panel.claim_holds_stable()
            && !panel.renders_stable()
            && panel.panel_state != PanelState::NarrowedClaimNarrowed
    });
    assert!(
        narrowed.is_some(),
        "the dashboard must narrow at least one release-blocking panel under a still-stable claim"
    );
}

#[test]
fn dashboard_exercises_a_failing_and_an_unmeasured_fitness_function() {
    let dashboard = dashboard();
    let statuses: Vec<FitnessStatus> = dashboard
        .panels
        .iter()
        .flat_map(|panel| panel.fitness_functions.iter().map(|f| f.status))
        .collect();
    assert!(
        statuses.contains(&FitnessStatus::Fail),
        "the dashboard must exercise a failing fitness function"
    );
    assert!(
        statuses.contains(&FitnessStatus::Unmeasured),
        "the dashboard must exercise an unmeasured fitness function"
    );
    assert!(
        statuses.contains(&FitnessStatus::Warn),
        "the dashboard must exercise a warn-band fitness function"
    );
}

#[test]
fn narrowing_panel_that_does_not_narrow_fails() {
    let mut dashboard = dashboard();
    let panel = dashboard
        .panels
        .iter_mut()
        .find(|panel| {
            panel.panel_state == PanelState::NarrowedStale
                && panel.claim_label == StableClaimLevel::Stable
        })
        .expect("dashboard has a narrowed-stale panel under a stable ceiling");
    panel.displayed_label = StableClaimLevel::Stable;
    dashboard.summary = dashboard.computed_summary();
    dashboard.publication.decision = dashboard.computed_publication_decision();
    dashboard.publication.blocking_rule_ids = dashboard.computed_blocking_rule_ids();
    dashboard.publication.blocking_panel_ids = dashboard.computed_blocking_panel_ids();

    assert!(
        dashboard.validate().iter().any(|v| matches!(
            v,
            ShiproomDashboardViolation::DisplayedLabelNotNarrowed { .. }
        )),
        "a panel that is not green must narrow below the cutline"
    );
}

#[test]
fn green_panel_on_a_breached_packet_fails() {
    let mut dashboard = dashboard();
    let panel = dashboard
        .panels
        .iter_mut()
        .find(|panel| panel.renders_green())
        .expect("dashboard has a green panel");
    panel.freshness_packet.slo_state = FreshnessSloState::Breached;
    dashboard.summary = dashboard.computed_summary();

    assert!(
        dashboard
            .validate()
            .iter()
            .any(|v| matches!(v, ShiproomDashboardViolation::GreenOnStalePacket { .. })),
        "a green panel may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn fitness_status_disagreeing_with_measurement_fails() {
    let mut dashboard = dashboard();
    let panel = dashboard
        .panels
        .iter_mut()
        .find(|panel| {
            panel
                .fitness_functions
                .iter()
                .any(|f| f.status == FitnessStatus::Pass)
        })
        .expect("dashboard has a passing fitness function");
    let function = panel
        .fitness_functions
        .iter_mut()
        .find(|f| f.status == FitnessStatus::Pass)
        .expect("a passing fitness function exists");
    function.status = FitnessStatus::Fail;

    assert!(
        dashboard.validate().iter().any(|v| matches!(
            v,
            ShiproomDashboardViolation::FitnessStatusInconsistent { .. }
        )),
        "a fitness function's status must agree with its measurement"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut dashboard = dashboard();
    dashboard.publication.decision = PromotionDecision::Proceed;

    assert!(
        dashboard.validate().iter().any(|v| matches!(
            v,
            ShiproomDashboardViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/shiproom_dashboard");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: ShiproomDashboard =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
