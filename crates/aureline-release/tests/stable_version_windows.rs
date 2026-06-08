//! Protected tests binding the typed stable version-window freeze to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the frozen, checked-in freeze; the capture cross-check proves
//! the typed model and the Python gate agree on the publication verdict, the
//! CLI/schema/API/manifest coverage counts, and the packet-freshness counts; the
//! negative cases mutate a parsed copy and the checked-in fixtures to prove that a row
//! which fails to narrow, a frozen row riding a breached packet, a freeze backed wider
//! than its public claim's ceiling, a disordered version window, and a publication
//! verdict that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::stable_version_windows::{
    current_stable_version_windows, GapReason, StableVersionWindows, StableVersionWindowsViolation,
    SurfaceKind, WindowState, STABLE_VERSION_WINDOWS_RECORD_KIND,
    STABLE_VERSION_WINDOWS_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stable_version_windows_validation_capture.json"
));

fn freeze() -> StableVersionWindows {
    current_stable_version_windows()
        .expect("checked-in stable version-window freeze parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_freeze_parses_and_validates() {
    let freeze = freeze();
    assert_eq!(freeze.schema_version, STABLE_VERSION_WINDOWS_SCHEMA_VERSION);
    assert_eq!(freeze.record_kind, STABLE_VERSION_WINDOWS_RECORD_KIND);
    let violations = freeze.validate();
    assert!(
        violations.is_empty(),
        "checked-in freeze must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_surface_kind() {
    let freeze = freeze();
    for kind in SurfaceKind::ALL {
        assert!(
            !freeze.rows_for_kind(kind).is_empty(),
            "surface kind {} must have at least one window row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let freeze = freeze();
    assert!(!freeze.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = freeze
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &freeze.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let freeze = freeze();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(freeze.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_surfaces"].as_u64().unwrap() as usize,
        freeze.rows.len(),
        "capture surface count must match the model"
    );
    assert_eq!(
        summary["surfaces_frozen_stable"].as_u64().unwrap() as usize,
        freeze.rows_frozen_stable().len(),
        "capture frozen count must match the model"
    );
    for (key, kind) in [
        ("cli_surfaces", SurfaceKind::Cli),
        ("schema_surfaces", SurfaceKind::Schema),
        ("api_surfaces", SurfaceKind::Api),
        ("manifest_surfaces", SurfaceKind::Manifest),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            freeze.rows_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        freeze.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        freeze.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        freeze.publication.decision,
        freeze.computed_publication_decision()
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
fn freeze_publishes_surfaces_without_narrowing() {
    let freeze = freeze();
    assert!(
        freeze.rows_narrowed().is_empty(),
        "clean freeze must not narrow a surface"
    );
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut freeze = freeze();
    let row = freeze
        .rows
        .iter_mut()
        .find(|row| row.holds_freeze())
        .expect("freeze has a frozen row");
    row.window_state = WindowState::UnfrozenStale;
    row.active_gap_reasons
        .push(GapReason::FreezePacketFreshnessBreached);
    row.frozen_label = StableClaimLevel::Stable;
    freeze.summary = freeze.computed_summary();
    freeze.publication.decision = freeze.computed_publication_decision();
    freeze.publication.blocking_rule_ids = freeze.computed_blocking_rule_ids();
    freeze.publication.blocking_window_ids = freeze.computed_blocking_window_ids();

    assert!(
        freeze.validate().iter().any(|v| matches!(
            v,
            StableVersionWindowsViolation::FrozenLabelNotNarrowed { .. }
        )),
        "a surface that is not frozen must narrow below the cutline"
    );
}

#[test]
fn frozen_row_on_a_breached_packet_fails() {
    let mut freeze = freeze();
    let row = freeze
        .rows
        .iter_mut()
        .find(|row| row.holds_freeze())
        .expect("freeze has a frozen row");
    row.freeze_packet.slo_state = FreshnessSloState::Breached;
    freeze.summary = freeze.computed_summary();

    assert!(
        freeze
            .validate()
            .iter()
            .any(|v| matches!(v, StableVersionWindowsViolation::HeldOnStalePacket { .. })),
        "a frozen row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_decision_mismatch_fails() {
    let mut freeze = freeze();
    freeze.publication.decision = PromotionDecision::Hold;

    assert!(
        freeze.validate().iter().any(|v| matches!(
            v,
            StableVersionWindowsViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication decision must agree with computed rules"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/stable_version_windows");
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
        let candidate: StableVersionWindows =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
