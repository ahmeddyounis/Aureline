//! Inline tests for the M5 design-system style-drift-lint lane.

use super::*;

fn canonical() -> M5StyleDriftLintReport {
    seeded_m5_style_drift_lint_report()
}

fn has_check(outcome: &M5StyleDriftLintOutcome, check_id: &str) -> bool {
    outcome.findings.iter().any(|f| f.check_id == check_id)
}

fn blocking_with_check(outcome: &M5StyleDriftLintOutcome, check_id: &str) -> bool {
    outcome
        .findings
        .iter()
        .any(|f| f.check_id == check_id && f.is_blocking())
}

#[test]
fn canonical_report_validates() {
    let report = canonical();
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(report.record_kind, M5_STYLE_DRIFT_LINT_REPORT_RECORD_KIND);
    assert_eq!(report.report_id, M5_STYLE_DRIFT_LINT_REPORT_ID);
    assert_eq!(report.report_version, M5_STYLE_DRIFT_LINT_REPORT_VERSION);
}

#[test]
fn report_covers_every_protected_surface_class() {
    let report = canonical();
    for class in M5ProtectedSurfaceClass::ALL {
        let surface = report
            .surface(class)
            .unwrap_or_else(|| panic!("missing {}", class.as_str()));
        assert_eq!(surface.surface_class, class);
        // Every surface binds all four protected states.
        for state in PROTECTED_STATES {
            assert!(
                surface.binding(state).is_some(),
                "{} missing {}",
                class.as_str(),
                state.as_str()
            );
        }
    }
    assert_eq!(report.surfaces.len(), M5ProtectedSurfaceClass::ALL.len());
}

#[test]
fn conformant_report_lints_green() {
    let outcome = canonical().lint();
    assert_eq!(outcome.gate_decision, GateStateClass::Pass);
    assert_eq!(outcome.total_findings, 0);
    assert_eq!(outcome.blocking_finding_count, 0);
    assert!(!outcome.blocks_stable_promotion());
    assert_eq!(
        outcome.total_surfaces,
        M5ProtectedSurfaceClass::ALL.len() as u32
    );
    for gate in &outcome.surface_gates {
        assert_eq!(gate.gate_decision, GateStateClass::Pass);
    }
}

#[test]
fn drift_drill_is_structurally_valid_but_blocks() {
    let report = seeded_m5_style_drift_lint_report_drift();
    // A drift drill is a structurally valid report; the gate, not validation, rejects the drift.
    assert!(report.validate().is_empty(), "{:?}", report.validate());

    let outcome = report.lint();
    assert_eq!(outcome.gate_decision, GateStateClass::Block);
    assert!(outcome.blocks_stable_promotion());
    assert_eq!(
        outcome.blocked_surface_ids(),
        vec!["design-system:protected-surface:trust_prompt"]
    );

    // Every drift class is caught as a blocking finding.
    assert!(blocking_with_check(&outcome, CHECK_UNMANAGED_TOKEN_VALUE));
    assert!(blocking_with_check(
        &outcome,
        CHECK_FORBIDDEN_LOCAL_STYLE_FORK
    ));
    assert!(blocking_with_check(
        &outcome,
        CHECK_MISSING_STATE_SEMANTIC_BINDING
    ));
    assert!(blocking_with_check(
        &outcome,
        CHECK_COLOR_ONLY_STATE_MEANING
    ));
    assert!(blocking_with_check(&outcome, CHECK_SPINNER_ONLY_STATE));
    assert!(blocking_with_check(
        &outcome,
        CHECK_HOVER_ONLY_CRITICAL_ACTION
    ));

    // The two unmanaged token usages each produce a finding.
    let unmanaged = outcome
        .findings
        .iter()
        .filter(|f| f.check_id == CHECK_UNMANAGED_TOKEN_VALUE)
        .count();
    assert_eq!(unmanaged, 2);
}

#[test]
fn active_proof_tied_waivers_disclose_a_gap_without_blocking() {
    let outcome = seeded_m5_style_drift_lint_report_waived().lint();
    assert_eq!(outcome.gate_decision, GateStateClass::PassWithDisclosedGap);
    assert!(!outcome.blocks_stable_promotion());
    assert_eq!(outcome.blocking_finding_count, 0);
    assert!(outcome.waived_finding_count >= 6);
    // Every drift finding is suppressed by a named waiver, and no waiver is unused.
    assert!(outcome
        .findings
        .iter()
        .filter(|f| f.severity == FindingSeverity::Error)
        .all(|f| f.waived_by.is_some()));
    assert!(!has_check(&outcome, CHECK_WAIVER_UNUSED));
}

#[test]
fn expired_waivers_do_not_suppress_and_still_block() {
    let outcome = seeded_m5_style_drift_lint_report_expired_waiver().lint();
    assert_eq!(outcome.gate_decision, GateStateClass::Block);
    assert!(outcome.blocks_stable_promotion());
    // Expired waivers suppress nothing, so the blocking findings remain.
    assert!(outcome.blocking_finding_count >= 6);
    assert!(outcome
        .findings
        .iter()
        .filter(|f| f.severity == FindingSeverity::Error)
        .all(|f| f.waived_by.is_none()));
    // An expired waiver matches a finding structurally, so it is not reported as unused.
    assert!(!has_check(&outcome, CHECK_WAIVER_UNUSED));
}

#[test]
fn unmanaged_token_detection_distinguishes_governed_from_raw_values() {
    assert!(token_value_is_managed("al.color.surface.raised"));
    assert!(token_value_is_managed("space.4"));
    assert!(token_value_is_managed("motion_standard"));
    assert!(token_value_is_managed("icon.metaphor.lock"));
    assert!(token_value_is_managed("typography.body"));

    assert!(token_value_is_raw_literal("#0A84FF"));
    assert!(token_value_is_raw_literal("12px"));
    assert!(token_value_is_raw_literal("1.5rem"));
    assert!(token_value_is_raw_literal("rgb(10, 132, 255)"));
    assert!(token_value_is_raw_literal("0A84FF"));
    assert!(!token_value_is_raw_literal("al.color.surface.raised"));
    assert!(!token_value_is_managed("brand_blue_500"));
}

#[test]
fn state_semantic_audit_catches_spinner_color_and_hover_regressions() {
    use CanonicalStateClass as S;
    // Spinner-only is allowed for loading but rejected for pending/degraded/blocked.
    let mut report = canonical();
    let surface = &mut report.surfaces[0];
    surface
        .binding(S::Loading)
        .map(|b| assert!(!b.spinner_only))
        .unwrap();
    // Loading spinner-only is exempt.
    surface
        .state_bindings
        .iter_mut()
        .find(|b| b.state_class == S::Loading)
        .unwrap()
        .spinner_only = true;
    // Pending spinner-only is a regression.
    surface
        .state_bindings
        .iter_mut()
        .find(|b| b.state_class == S::Pending)
        .unwrap()
        .spinner_only = true;
    let outcome = report.lint();
    let spinner_findings: Vec<&M5StyleDriftFinding> = outcome
        .findings
        .iter()
        .filter(|f| f.check_id == CHECK_SPINNER_ONLY_STATE)
        .collect();
    assert_eq!(spinner_findings.len(), 1);
    assert_eq!(spinner_findings[0].state_class, Some(S::Pending));
}

#[test]
fn state_semantic_audit_catches_unlabeled_and_color_only() {
    use CanonicalStateClass as S;
    let mut report = canonical();
    let degraded = report.surfaces[1]
        .state_bindings
        .iter_mut()
        .find(|b| b.state_class == S::Degraded)
        .unwrap();
    degraded.screen_reader_label.clear();
    degraded.non_color_cues.clear();
    let outcome = report.lint();
    assert!(blocking_with_check(&outcome, CHECK_UNLABELED_STATE));
    assert!(blocking_with_check(
        &outcome,
        CHECK_COLOR_ONLY_STATE_MEANING
    ));
}

#[test]
fn missing_protected_state_binding_blocks() {
    use CanonicalStateClass as S;
    let mut report = canonical();
    report.surfaces[2]
        .state_bindings
        .retain(|b| b.state_class != S::Blocked);
    let outcome = report.lint();
    assert!(blocking_with_check(
        &outcome,
        CHECK_MISSING_STATE_SEMANTIC_BINDING
    ));
    let missing = outcome
        .findings
        .iter()
        .find(|f| f.check_id == CHECK_MISSING_STATE_SEMANTIC_BINDING)
        .unwrap();
    assert_eq!(missing.state_class, Some(S::Blocked));
}

#[test]
fn unused_waiver_is_reported_as_a_non_blocking_warning() {
    let mut report = canonical();
    // Add a well-formed waiver to a conformant surface; it suppresses nothing.
    report.surfaces[0].waivers.push(M5StyleDriftWaiver {
        waiver_id: "stale".to_owned(),
        waived_check_id: CHECK_FORBIDDEN_LOCAL_STYLE_FORK.to_owned(),
        waived_state_class: None,
        waived_subject_id: None,
        reason_message_id: format!(
            "{}trust_prompt.waiver.stale",
            M5_STYLE_DRIFT_LINT_MESSAGE_ID_PREFIX
        ),
        expires_at: "2026-09-01T00:00:00Z".to_owned(),
        proof_packet_ref: format!(
            "{}style-drift-lint-outcome.json",
            M5_DESIGN_SYSTEM_PROOF_DIR
        ),
    });
    let outcome = report.lint();
    assert_eq!(outcome.gate_decision, GateStateClass::Warn);
    assert!(!outcome.blocks_stable_promotion());
    assert!(has_check(&outcome, CHECK_WAIVER_UNUSED));
}

#[test]
fn validation_rejects_proofless_waiver() {
    let mut report = seeded_m5_style_drift_lint_report_waived();
    let trust = &mut report.surfaces[0];
    trust.waivers[0].proof_packet_ref = "artifacts/somewhere/else.json".to_owned();
    assert!(report
        .validate()
        .contains(&M5StyleDriftLintViolation::WaiverMalformed));
}

#[test]
fn validation_rejects_waiver_on_unknown_check() {
    let mut report = seeded_m5_style_drift_lint_report_waived();
    report.surfaces[0].waivers[0].waived_check_id = "style_drift.not_a_real_check".to_owned();
    assert!(report
        .validate()
        .contains(&M5StyleDriftLintViolation::WaiverMalformed));
}

#[test]
fn validation_rejects_missing_surface_class() {
    let mut report = canonical();
    report.surfaces.pop();
    assert!(report
        .validate()
        .contains(&M5StyleDriftLintViolation::RequiredSurfaceClassMissing));
}

#[test]
fn validation_rejects_duplicate_surface_class() {
    let mut report = canonical();
    let extra = report.surfaces[0].clone();
    report.surfaces.push(extra);
    let violations = report.validate();
    assert!(violations.contains(&M5StyleDriftLintViolation::DuplicateSurfaceClass));
}

#[test]
fn validation_rejects_bad_report_version() {
    let mut report = canonical();
    report.report_version = "1.0".to_owned();
    assert!(report
        .validate()
        .contains(&M5StyleDriftLintViolation::BadReportVersion));
}

#[test]
fn export_import_round_trips_and_revalidates() {
    let report = canonical();
    let json = report.export_safe_json();
    let imported = M5StyleDriftLintReport::from_json(&json).expect("imports");
    assert_eq!(imported, report);
    assert!(imported.validate().is_empty());
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = canonical().export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("authorization"));
    assert!(!json.contains("bearer "));
}

#[test]
fn release_packet_summarizes_each_surface_and_overall_gate() {
    let report = canonical();
    let release = report.release_packet();
    assert_eq!(release.gate_decision, GateStateClass::Pass);
    assert_eq!(
        release.surface_gates.len(),
        M5ProtectedSurfaceClass::ALL.len()
    );
    assert_eq!(release.blocking_finding_count, 0);

    let drift_release = seeded_m5_style_drift_lint_report_drift().release_packet();
    assert_eq!(drift_release.gate_decision, GateStateClass::Block);
    assert!(drift_release.blocking_finding_count >= 6);
}

#[test]
fn checked_report_fixture_matches_seed_and_validates() {
    let from_disk = current_stable_m5_style_drift_lint_report().expect("checked report validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked lint report drifted from the seed builder"
    );
}

#[test]
fn checked_outcome_matches_computed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/style-drift-lint-outcome.json"
    ));
    let from_disk: M5StyleDriftLintOutcome =
        serde_json::from_str(raw).expect("outcome packet parses");
    assert_eq!(
        from_disk,
        seeded_m5_style_drift_lint_report().lint(),
        "checked lint outcome drifted from the computed outcome"
    );
}

#[test]
fn checked_release_packet_matches_computed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/style-drift-lint-release.json"
    ));
    let from_disk: M5StyleDriftLintReleasePacket =
        serde_json::from_str(raw).expect("release packet parses");
    assert_eq!(
        from_disk,
        seeded_m5_style_drift_lint_report().release_packet(),
        "checked release packet drifted from the computed release packet"
    );
}

#[test]
fn checked_drill_fixtures_match_seed_and_gate_as_expected() {
    macro_rules! check_drill {
        ($builder:expr, $file:literal, $gate:expr) => {{
            let raw = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-style-drift-lint/",
                $file
            ));
            let from_disk: M5StyleDriftLintReport =
                serde_json::from_str(raw).expect("drill fixture parses");
            let expected = $builder;
            assert_eq!(from_disk, expected, "{} fixture drifted", $file);
            assert!(
                from_disk.validate().is_empty(),
                "{} fixture invalid: {:?}",
                $file,
                from_disk.validate()
            );
            assert_eq!(from_disk.lint().gate_decision, $gate, "{} gate", $file);
        }};
    }
    check_drill!(
        seeded_m5_style_drift_lint_report_drift(),
        "lint-report-drift.json",
        GateStateClass::Block
    );
    check_drill!(
        seeded_m5_style_drift_lint_report_waived(),
        "lint-report-waived.json",
        GateStateClass::PassWithDisclosedGap
    );
    check_drill!(
        seeded_m5_style_drift_lint_report_expired_waiver(),
        "lint-report-expired-waiver.json",
        GateStateClass::Block
    );
}
