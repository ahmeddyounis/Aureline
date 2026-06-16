//! Protected tests binding the typed critical-upstream health register to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the coverage check proves every upstream kind
//! is exercised and every narrowing reason is wired; the capture cross-check proves the typed
//! model and the CI gate agree on the promotion verdict, the scan/surface parity, and the
//! cleared/narrowed counts; the no-mask check proves a green upstream surface still narrows on
//! an abandoned maintainer base or an unowned dependency and that scan and surface agree on
//! every record; the narrowing check proves an upstream-health failure on a still-stable subject
//! holds promotion while inherited and waived narrowings stay gated upstream; the negative cases
//! mutate a parsed copy and read the checked-in fixtures to prove that a hidden ownership gap, a
//! green surface over a gapped scan, a narrowed record that stays above the cutline, and a
//! proceed verdict while a rule fires all fail validation.

use std::path::{Path, PathBuf};

use aureline_governance::m5_boundary_and_upstream_durability::{FreshnessSloState, LifecycleLabel};
use aureline_governance::m5_critical_upstream_health::{
    current_m5_critical_upstream_health, ControlDimension, CriticalUpstreamHealthRegister,
    HealthReason, HealthState, MaintainerRating, OwnershipState, Posture, PublicationDecision,
    RegisterViolation, UpstreamKind, M5_CRITICAL_UPSTREAM_HEALTH_RECORD_KIND,
    M5_CRITICAL_UPSTREAM_HEALTH_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/captures/m5-critical-upstream-health_validation_capture.json"
));

fn register() -> CriticalUpstreamHealthRegister {
    current_m5_critical_upstream_health().expect("checked-in register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_CRITICAL_UPSTREAM_HEALTH_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_CRITICAL_UPSTREAM_HEALTH_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_upstream_kind_and_every_reason_has_a_rule() {
    let r = register();
    for kind in UpstreamKind::ALL {
        assert!(
            !r.records_of_kind(kind).is_empty(),
            "upstream kind {} must have at least one record",
            kind.as_str()
        );
    }
    for rec in &r.records {
        for dimension in ControlDimension::ALL {
            assert_eq!(
                rec.controls
                    .iter()
                    .filter(|c| c.dimension == dimension)
                    .count(),
                1,
                "record {} must declare control {} exactly once",
                rec.record_id,
                dimension.as_str()
            );
        }
    }
    for reason in HealthReason::ALL {
        assert!(
            r.rules.iter().any(|rule| rule.trigger_reason == reason),
            "reason {} must be watched by a rule",
            reason.as_str()
        );
    }
}

#[test]
fn keeps_per_axis_state_not_one_global_flag() {
    let r = register();
    let states: std::collections::BTreeSet<HealthState> =
        r.records.iter().map(|x| x.health_state).collect();
    assert!(states.contains(&HealthState::Cleared));
    // Distinct narrowing axes coexist instead of collapsing into one pass/fail flag.
    assert!(states.contains(&HealthState::NarrowedMaintainer));
    assert!(states.contains(&HealthState::NarrowedSecurity));
    assert!(states.contains(&HealthState::NarrowedCadence));
    assert!(states.contains(&HealthState::NarrowedLicense));
    assert!(states.contains(&HealthState::NarrowedOwnership));
    assert!(states.contains(&HealthState::NarrowedStale));

    let reasons: std::collections::BTreeSet<HealthReason> = r
        .records
        .iter()
        .flat_map(|x| x.active_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&HealthReason::MaintainerAbandoned));
    assert!(reasons.contains(&HealthReason::SecurityUnpatchedCritical));
    assert!(reasons.contains(&HealthReason::UpstreamUnowned));
    assert!(reasons.contains(&HealthReason::ShiproomEscalationMissing));
}

#[test]
fn green_surface_never_masks_an_abandoned_or_unowned_upstream() {
    let r = register();
    // Every record's scan and surface agree, so a green surface can never sit over a scan that
    // found gaps.
    for rec in &r.records {
        assert!(
            rec.scan_surface_agree(),
            "record {} scan and surface must agree",
            rec.record_id
        );
        assert_eq!(rec.surface_posture, rec.computed_posture());
    }
    // An unowned upstream still narrows on the ownership axis and reports gaps on its surface.
    let unowned = r
        .records
        .iter()
        .find(|rec| rec.is_unowned())
        .expect("an unowned upstream exists");
    assert_eq!(unowned.health_state, HealthState::NarrowedOwnership);
    assert_eq!(unowned.surface_posture, Posture::GapsFound);
    // An abandoned upstream still narrows on the maintainer axis.
    let abandoned = r
        .records
        .iter()
        .find(|rec| rec.maintainer.rating == MaintainerRating::Abandoned)
        .expect("an abandoned upstream exists");
    assert_eq!(abandoned.health_state, HealthState::NarrowedMaintainer);
    assert_eq!(abandoned.surface_posture, Posture::GapsFound);
}

#[test]
fn upstream_health_and_contingency_truth_is_recorded() {
    let r = register();
    // The maintainer, security, cadence, license, and ownership axes actually carry gaps.
    assert!(
        r.summary.maintainer_gaps > 0,
        "must record a maintainer gap"
    );
    assert!(r.summary.security_gaps > 0, "must record a security gap");
    assert!(r.summary.cadence_gaps > 0, "must record a cadence gap");
    assert!(r.summary.license_gaps > 0, "must record a license gap");
    assert!(r.summary.ownership_gaps > 0, "must record an ownership gap");
    // Red-risk upstreams are tracked, escalations are required, and at least one plan is recorded.
    assert!(r.summary.red_risk_total > 0);
    assert!(r.summary.escalations_required > 0);
    assert!(r.summary.contingency_plans_recorded > 0);
    for rec in &r.records {
        if rec.maintainer.rating.is_abandoned() {
            assert!(rec.has_active_reason(HealthReason::MaintainerAbandoned));
        }
        if rec.escalation_missing() {
            assert!(rec.has_active_reason(HealthReason::ShiproomEscalationMissing));
        }
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let r = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(r.as_of.as_str()));

    let summary = &capture["summary"];
    let computed = r.computed_summary();
    let u = |v: &serde_json::Value| v.as_u64().unwrap() as usize;
    assert_eq!(u(&summary["total_records"]), computed.total_records);
    assert_eq!(u(&summary["records_cleared"]), computed.records_cleared);
    assert_eq!(u(&summary["records_narrowed"]), computed.records_narrowed);
    assert_eq!(u(&summary["state_cleared"]), computed.state_cleared);
    assert_eq!(
        u(&summary["state_narrowed_maintainer"]),
        computed.state_narrowed_maintainer
    );
    assert_eq!(
        u(&summary["state_narrowed_security"]),
        computed.state_narrowed_security
    );
    assert_eq!(
        u(&summary["state_narrowed_cadence"]),
        computed.state_narrowed_cadence
    );
    assert_eq!(
        u(&summary["state_narrowed_license"]),
        computed.state_narrowed_license
    );
    assert_eq!(
        u(&summary["state_narrowed_ownership"]),
        computed.state_narrowed_ownership
    );
    assert_eq!(
        u(&summary["state_narrowed_stale"]),
        computed.state_narrowed_stale
    );
    assert_eq!(
        u(&summary["release_blocking_narrowed"]),
        computed.release_blocking_narrowed
    );
    assert_eq!(
        u(&summary["records_on_active_waiver"]),
        computed.records_on_active_waiver
    );
    assert_eq!(u(&summary["maintainer_gaps"]), computed.maintainer_gaps);
    assert_eq!(u(&summary["security_gaps"]), computed.security_gaps);
    assert_eq!(u(&summary["cadence_gaps"]), computed.cadence_gaps);
    assert_eq!(u(&summary["license_gaps"]), computed.license_gaps);
    assert_eq!(u(&summary["ownership_gaps"]), computed.ownership_gaps);
    assert_eq!(u(&summary["red_risk_total"]), computed.red_risk_total);
    assert_eq!(u(&summary["unowned_total"]), computed.unowned_total);
    assert_eq!(
        u(&summary["escalations_required"]),
        computed.escalations_required
    );
    assert_eq!(
        u(&summary["escalations_raised"]),
        computed.escalations_raised
    );
    assert_eq!(
        u(&summary["contingency_plans_recorded"]),
        computed.contingency_plans_recorded
    );
    assert_eq!(
        u(&summary["total_active_reasons"]),
        computed.total_active_reasons
    );
    assert_eq!(u(&summary["rules_firing"]), computed.rules_firing);

    let parity = &capture["scan_surface_parity"];
    let computed_parity = r.computed_scan_surface_parity();
    assert_eq!(
        u(&parity["subjects_in_agreement"]),
        computed_parity.subjects_in_agreement
    );
    assert_eq!(
        u(&parity["subjects_in_disagreement"]),
        computed_parity.subjects_in_disagreement
    );
    assert_eq!(
        u(&parity["subjects_with_gaps"]),
        computed_parity.subjects_with_gaps
    );
    assert_eq!(
        parity["all_subjects_agree"].as_bool(),
        Some(computed_parity.all_subjects_agree)
    );

    assert_eq!(
        capture["publication"]["decision"].as_str().unwrap(),
        r.publication.decision.as_str()
    );
    assert_eq!(r.publication.decision, r.computed_decision());

    let captured_rules: Vec<&str> = capture["publication"]["blocking_rule_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_rules, r.computed_blocking_rule_ids());

    let captured_records: Vec<&str> = capture["publication"]["blocking_record_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_records, r.computed_blocking_record_ids());

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
fn health_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, PublicationDecision::Hold);
    let blocking = r.computed_blocking_record_ids();
    assert!(
        !blocking.is_empty(),
        "an upstream-health failure on a still-stable subject must hold promotion"
    );
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
    }
    // The abandoned notebook upstream holds promotion on the maintainer axis.
    let notebook = r
        .record("upstream-notebook-render-kernel")
        .expect("notebook record exists");
    assert_eq!(notebook.health_state, HealthState::NarrowedMaintainer);
    assert!(blocking.contains(&notebook.record_id));
    // The unowned managed-depth upstream holds promotion on the ownership axis.
    let unowned = r
        .record("upstream-managed_depth-object-store")
        .expect("unowned record exists");
    assert_eq!(unowned.health_state, HealthState::NarrowedOwnership);
    assert!(blocking.contains(&unowned.record_id));
    // The red-risk, unowned, plan-and-escalation-pending vector index is the headline guardrail.
    let blocked = r
        .record("upstream-ai_adjacent-vector-index")
        .expect("blocked record exists");
    assert_eq!(blocked.health_state, HealthState::NarrowedOwnership);
    assert!(blocked.has_active_reason(HealthReason::ContingencyPlanMissing));
    assert!(blocked.has_active_reason(HealthReason::ShiproomEscalationMissing));
    assert!(blocking.contains(&blocked.record_id));
    // The single-maintainer queue protocol is narrowed and visible, but held by a waiver.
    let waived = r
        .record("upstream-managed_depth-queue-protocol")
        .expect("waived record exists");
    assert_eq!(waived.health_state, HealthState::NarrowedMaintainer);
    assert!(waived.is_waived());
    assert!(!blocking.contains(&waived.record_id));
    // The diff protocol already sits below the cutline (Beta): inherited.
    let beta = r
        .record("upstream-review-diff-protocol")
        .expect("beta record exists");
    assert!(beta.health_state.is_narrowed());
    assert!(!beta.declares_at_or_above_cutline());
    assert!(!blocking.contains(&beta.record_id));
}

#[test]
fn stale_and_missing_proof_narrow_on_the_stale_axis() {
    let r = register();
    let stale = r
        .record("upstream-framework-build-toolchain")
        .expect("stale-proof record exists");
    assert_eq!(stale.health_state, HealthState::NarrowedStale);
    assert_eq!(stale.proof_packet.slo_state, FreshnessSloState::Breached);
    let missing = r
        .record("upstream-companion-packager")
        .expect("missing-proof record exists");
    assert_eq!(missing.health_state, HealthState::NarrowedStale);
    assert_eq!(missing.proof_packet.slo_state, FreshnessSloState::Missing);
}

#[test]
fn hidden_ownership_gap_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    rec.ownership.ownership_state = OwnershipState::Unowned;
    rec.ownership.owner_ref = String::new();
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            RegisterViolation::GapWithoutReason {
                reason: HealthReason::UpstreamUnowned,
                ..
            }
        )),
        "a hidden ownership gap must fail validation"
    );
}

#[test]
fn green_surface_over_a_gapped_scan_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.health_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.surface_posture = Posture::Clear;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::ScanSurfaceDisagreement { .. }
            | RegisterViolation::PostureMismatch { .. }
    )));
}

#[test]
fn narrowed_record_above_the_cutline_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.health_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.publication.decision = PublicationDecision::Proceed;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, RegisterViolation::PublicationDecisionInconsistent)));
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/governance/m5-critical-upstream-health");
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
        let candidate: CriticalUpstreamHealthRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        checked += 1;
    }
    assert!(checked > 0, "at least one fixture must be exercised");
}
