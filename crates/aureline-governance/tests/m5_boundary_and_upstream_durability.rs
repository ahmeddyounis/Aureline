//! Protected tests binding the typed open/local-boundary and upstream-durability
//! matrix to the checked-in artifact, the frozen CI validation capture, and the
//! negative fixtures.
//!
//! The positive case is the checked-in matrix; the coverage check proves every
//! asset lane and every narrowing reason is wired; the capture cross-check proves
//! the typed model and the CI gate agree on the publication verdict, the
//! durable/narrowed counts, and the control/packet counts; the narrowing check
//! proves a durability-layer failure on a still-stable lane holds promotion while
//! inherited and waived narrowings stay gated upstream; the negative cases mutate
//! a parsed copy and read the checked-in fixtures to prove that an open-baseline
//! violation, a durable row with a gap, a narrowed row that stays above the
//! cutline, and a proceed verdict while a rule fires all fail validation.

use std::path::{Path, PathBuf};

use aureline_governance::m5_boundary_and_upstream_durability::{
    current_m5_boundary_and_upstream_durability, AssetLane, BoundaryDurabilityMatrix,
    BoundaryPosture, DurabilityReason, DurabilityState, LifecycleLabel, MatrixViolation,
    PublicationDecision, SupportClass, M5_BOUNDARY_AND_UPSTREAM_DURABILITY_RECORD_KIND,
    M5_BOUNDARY_AND_UPSTREAM_DURABILITY_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/captures/m5-boundary-and-upstream-durability_validation_capture.json"
));

fn matrix() -> BoundaryDurabilityMatrix {
    current_m5_boundary_and_upstream_durability().expect("checked-in matrix parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_matrix_parses_and_validates() {
    let m = matrix();
    assert_eq!(
        m.schema_version,
        M5_BOUNDARY_AND_UPSTREAM_DURABILITY_SCHEMA_VERSION
    );
    assert_eq!(
        m.record_kind,
        M5_BOUNDARY_AND_UPSTREAM_DURABILITY_RECORD_KIND
    );
    let violations = m.validate();
    assert!(
        violations.is_empty(),
        "checked-in matrix must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_asset_lane_and_every_reason_has_a_rule() {
    let m = matrix();
    for lane in AssetLane::ALL {
        assert!(
            !m.rows_for_lane(lane).is_empty(),
            "asset lane {} must have at least one row",
            lane.as_str()
        );
    }
    for reason in DurabilityReason::ALL {
        assert!(
            m.rules.iter().any(|r| r.trigger_reason == reason),
            "reason {} must be watched by a rule",
            reason.as_str()
        );
    }
}

#[test]
fn must_remain_open_lanes_carry_an_open_baseline_or_narrow() {
    let m = matrix();
    let mut saw_open_baseline_must_open = false;
    for row in &m.rows {
        if row.must_remain_open {
            if row.boundary_posture.is_open_baseline() {
                saw_open_baseline_must_open = true;
            } else {
                assert!(
                    row.has_active_reason(DurabilityReason::BoundaryBaselineViolated),
                    "must-remain-open lane {} drifted off the baseline without narrowing",
                    row.entry_id
                );
            }
        }
    }
    assert!(
        saw_open_baseline_must_open,
        "the matrix must keep the must-remain-open core on an open baseline"
    );
}

#[test]
fn keeps_per_axis_state_not_one_global_flag() {
    let m = matrix();
    let states: std::collections::BTreeSet<DurabilityState> =
        m.rows.iter().map(|r| r.durability_state).collect();
    assert!(states.contains(&DurabilityState::Durable));
    // Distinct narrowing axes coexist (compliance, continuity, stale) instead of
    // collapsing into a single pass/fail flag.
    assert!(states.contains(&DurabilityState::NarrowedComplianceGap));
    assert!(states.contains(&DurabilityState::NarrowedContinuityGap));
    assert!(states.contains(&DurabilityState::NarrowedStale));

    let reasons: std::collections::BTreeSet<DurabilityReason> = m
        .rows
        .iter()
        .flat_map(|r| r.active_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&DurabilityReason::ComplianceControlUnsatisfied));
    assert!(reasons.contains(&DurabilityReason::SinglePointOfFailure));
    assert!(reasons.contains(&DurabilityReason::ProofFreshnessBreached));
}

#[test]
fn model_matches_frozen_validation_capture() {
    let m = matrix();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(m.as_of.as_str()));

    let summary = &capture["summary"];
    let computed = m.computed_summary();
    let u = |v: &serde_json::Value| v.as_u64().unwrap() as usize;
    assert_eq!(u(&summary["total_rows"]), computed.total_rows);
    assert_eq!(u(&summary["rows_durable"]), computed.rows_durable);
    assert_eq!(u(&summary["rows_narrowed"]), computed.rows_narrowed);
    assert_eq!(u(&summary["state_durable"]), computed.state_durable);
    assert_eq!(
        u(&summary["state_narrowed_compliance_gap"]),
        computed.state_narrowed_compliance_gap
    );
    assert_eq!(
        u(&summary["state_narrowed_continuity_gap"]),
        computed.state_narrowed_continuity_gap
    );
    assert_eq!(
        u(&summary["state_narrowed_stale"]),
        computed.state_narrowed_stale
    );
    assert_eq!(
        u(&summary["must_remain_open_rows"]),
        computed.must_remain_open_rows
    );
    assert_eq!(
        u(&summary["open_baseline_rows"]),
        computed.open_baseline_rows
    );
    assert_eq!(
        u(&summary["release_blocking_narrowed"]),
        computed.release_blocking_narrowed
    );
    assert_eq!(
        u(&summary["rows_on_active_waiver"]),
        computed.rows_on_active_waiver
    );
    assert_eq!(u(&summary["total_controls"]), computed.total_controls);
    assert_eq!(
        u(&summary["controls_unsatisfied"]),
        computed.controls_unsatisfied
    );
    assert_eq!(u(&summary["packets_breached"]), computed.packets_breached);
    assert_eq!(
        u(&summary["total_active_reasons"]),
        computed.total_active_reasons
    );
    assert_eq!(u(&summary["rules_firing"]), computed.rules_firing);

    assert_eq!(
        capture["publication"]["decision"].as_str().unwrap(),
        m.publication.decision.as_str()
    );
    assert_eq!(m.publication.decision, m.computed_decision());

    let captured_rules: Vec<&str> = capture["publication"]["blocking_rule_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_rules, m.computed_blocking_rule_ids());

    let captured_rows: Vec<&str> = capture["publication"]["blocking_row_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_rows, m.computed_blocking_row_ids());

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
fn durability_layer_failure_holds_promotion_inherited_does_not() {
    let m = matrix();
    assert_eq!(m.publication.decision, PublicationDecision::Hold);
    let blocking = m.computed_blocking_row_ids();
    assert!(
        !blocking.is_empty(),
        "a durability-layer failure on a still-stable lane must hold promotion"
    );
    for id in &blocking {
        let row = m.row(id).expect("blocking row exists");
        assert!(row.release_blocking);
        assert!(row.declares_at_or_above_cutline());
        assert!(!row.is_waived());
    }
    // The waived continuity lane is narrowed and visible, but gated upstream.
    let waived = m
        .row("boundary-critical-upstream-toolchain")
        .expect("critical-upstream lane exists");
    assert!(waived.durability_state.is_narrowed());
    assert!(waived.is_waived());
    assert!(!blocking.contains(&waived.entry_id));
    // The marketplace lane already sits below the cutline (Beta): inherited.
    let inherited = m
        .row("boundary-marketplace-protocol")
        .expect("marketplace lane exists");
    assert!(inherited.durability_state.is_narrowed());
    assert!(!inherited.declares_at_or_above_cutline());
    assert!(!blocking.contains(&inherited.entry_id));
}

#[test]
fn open_baseline_is_never_blurred_by_managed_value() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|r| r.must_remain_open && r.boundary_posture.is_open_baseline())
        .expect("an open must-remain-open lane exists");
    row.boundary_posture = BoundaryPosture::ManagedService;
    row.support_class = SupportClass::Managed;
    assert!(
        m.validate()
            .iter()
            .any(|v| matches!(v, MatrixViolation::MustRemainOpenViolated { .. })),
        "blurring the open baseline with a managed posture must fail validation"
    );
}

#[test]
fn durable_row_with_a_gap_fails() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|r| r.is_durable())
        .expect("a durable row exists");
    row.active_reasons
        .push(DurabilityReason::OwnerSignoffMissing);
    assert!(m
        .validate()
        .iter()
        .any(|v| matches!(v, MatrixViolation::DurableWithActiveReason { .. })));
}

#[test]
fn narrowed_row_above_the_cutline_fails() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|r| r.durability_state.is_narrowed())
        .expect("a narrowed row exists");
    row.effective_label = LifecycleLabel::Stable;
    assert!(m.validate().iter().any(|v| matches!(
        v,
        MatrixViolation::NarrowedAboveCutline { .. }
            | MatrixViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn proceed_while_a_rule_fires_fails() {
    let mut m = matrix();
    m.publication.decision = PublicationDecision::Proceed;
    assert!(m
        .validate()
        .iter()
        .any(|v| matches!(v, MatrixViolation::PublicationDecisionInconsistent)));
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/governance/m5-boundary-and-upstream-durability");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: BoundaryDurabilityMatrix =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        checked += 1;
    }
    assert!(checked > 0, "at least one fixture must be exercised");
}
