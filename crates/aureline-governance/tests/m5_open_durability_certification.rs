//! Protected tests binding the typed open-durability certification register to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the coverage check proves every row kind is
//! exercised and every narrowing reason is wired; the capture cross-check proves the typed model and
//! the CI gate agree on the promotion verdict, the scan/surface parity, and the certified/narrowed
//! counts; the no-mask check proves a green certification surface still narrows on the three headline
//! guardrails (a hidden proprietary baseline, an ownerless critical import, a single-person emergency
//! authority) and that scan and surface agree on every row; the narrowing check proves a
//! certification failure on a still-stable row holds promotion while inherited and waived narrowings
//! stay gated upstream; the negative cases mutate a parsed copy and read the checked-in fixtures to
//! prove that a hidden axis gap, a single-person authority certified clean, a green surface over a
//! gapped scan, a narrowed row that stays above the cutline, and a proceed verdict while a rule fires
//! all fail validation.

use std::path::{Path, PathBuf};

use aureline_governance::m5_boundary_and_upstream_durability::{FreshnessSloState, LifecycleLabel};
use aureline_governance::m5_open_durability_certification::{
    current_m5_open_durability_certification, CertificationReason, CertificationState,
    ControlDimension, OpenDurabilityCertificationRegister, Posture, PublicationDecision,
    RegisterViolation, RowKind, M5_OPEN_DURABILITY_CERTIFICATION_RECORD_KIND,
    M5_OPEN_DURABILITY_CERTIFICATION_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/captures/m5-open-durability-certification_validation_capture.json"
));

fn register() -> OpenDurabilityCertificationRegister {
    current_m5_open_durability_certification().expect("checked-in register parses into the model")
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
    assert_eq!(
        r.schema_version,
        M5_OPEN_DURABILITY_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_OPEN_DURABILITY_CERTIFICATION_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_row_kind_and_every_reason_has_a_rule() {
    let r = register();
    for kind in RowKind::ALL {
        assert!(
            !r.records_of_kind(kind).is_empty(),
            "row kind {} must have at least one record",
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
    for reason in CertificationReason::ALL {
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
    let states: std::collections::BTreeSet<CertificationState> =
        r.records.iter().map(|x| x.certification_state).collect();
    assert!(states.contains(&CertificationState::Certified));
    // Distinct narrowing axes coexist instead of collapsing into one pass/fail flag.
    assert!(states.contains(&CertificationState::NarrowedBoundary));
    assert!(states.contains(&CertificationState::NarrowedCompliance));
    assert!(states.contains(&CertificationState::NarrowedImport));
    assert!(states.contains(&CertificationState::NarrowedAuthority));
    assert!(states.contains(&CertificationState::NarrowedEmergency));
    assert!(states.contains(&CertificationState::NarrowedUpstream));
    assert!(states.contains(&CertificationState::NarrowedStale));

    let reasons: std::collections::BTreeSet<CertificationReason> = r
        .records
        .iter()
        .flat_map(|x| x.active_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&CertificationReason::HiddenProprietaryBaseline));
    assert!(reasons.contains(&CertificationReason::OwnerlessCriticalImport));
    assert!(reasons.contains(&CertificationReason::SinglePersonEmergencyAuthority));
    assert!(reasons.contains(&CertificationReason::NoticeBindingMissing));
    assert!(reasons.contains(&CertificationReason::CriticalUpstreamUnhealthy));
    assert!(reasons.contains(&CertificationReason::EmergencyResponseStale));
}

#[test]
fn green_surface_never_masks_a_guardrail_gap() {
    let r = register();
    // Every record's scan and surface agree, so a green surface can never sit over a scan that found
    // gaps.
    for rec in &r.records {
        assert!(
            rec.scan_surface_agree(),
            "record {} scan and surface must agree",
            rec.record_id
        );
        assert_eq!(rec.surface_posture, rec.computed_posture());
    }
    // The three headline guardrails each narrow on their axis and report gaps on the surface.
    let hidden = r
        .records
        .iter()
        .find(|rec| rec.has_active_reason(CertificationReason::HiddenProprietaryBaseline))
        .expect("a hidden proprietary baseline exists");
    assert_eq!(
        hidden.certification_state,
        CertificationState::NarrowedBoundary
    );
    assert_eq!(hidden.surface_posture, Posture::GapsFound);
    let ownerless = r
        .records
        .iter()
        .find(|rec| rec.has_active_reason(CertificationReason::OwnerlessCriticalImport))
        .expect("an ownerless critical import exists");
    assert_eq!(
        ownerless.certification_state,
        CertificationState::NarrowedImport
    );
    assert_eq!(ownerless.surface_posture, Posture::GapsFound);
    let single = r
        .records
        .iter()
        .find(|rec| rec.has_active_reason(CertificationReason::SinglePersonEmergencyAuthority))
        .expect("a single-person emergency authority exists");
    assert_eq!(
        single.certification_state,
        CertificationState::NarrowedAuthority
    );
    assert_eq!(single.surface_posture, Posture::GapsFound);
}

#[test]
fn every_axis_truth_is_recorded() {
    let r = register();
    assert!(r.summary.boundary_gaps > 0, "must record a boundary gap");
    assert!(
        r.summary.compliance_gaps > 0,
        "must record a compliance gap"
    );
    assert!(r.summary.import_gaps > 0, "must record an import gap");
    assert!(r.summary.authority_gaps > 0, "must record an authority gap");
    assert!(r.summary.emergency_gaps > 0, "must record an emergency gap");
    assert!(r.summary.upstream_gaps > 0, "must record an upstream gap");
    // The three "do not certify" guardrails are tracked as first-class counts.
    assert!(r.summary.hidden_proprietary_baseline_gaps > 0);
    assert!(r.summary.ownerless_critical_import_gaps > 0);
    assert!(r.summary.single_person_authority_gaps > 0);
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
    assert_eq!(u(&summary["records_certified"]), computed.records_certified);
    assert_eq!(u(&summary["records_narrowed"]), computed.records_narrowed);
    assert_eq!(u(&summary["state_certified"]), computed.state_certified);
    assert_eq!(
        u(&summary["state_narrowed_boundary"]),
        computed.state_narrowed_boundary
    );
    assert_eq!(
        u(&summary["state_narrowed_compliance"]),
        computed.state_narrowed_compliance
    );
    assert_eq!(
        u(&summary["state_narrowed_import"]),
        computed.state_narrowed_import
    );
    assert_eq!(
        u(&summary["state_narrowed_authority"]),
        computed.state_narrowed_authority
    );
    assert_eq!(
        u(&summary["state_narrowed_emergency"]),
        computed.state_narrowed_emergency
    );
    assert_eq!(
        u(&summary["state_narrowed_upstream"]),
        computed.state_narrowed_upstream
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
    assert_eq!(u(&summary["boundary_gaps"]), computed.boundary_gaps);
    assert_eq!(u(&summary["compliance_gaps"]), computed.compliance_gaps);
    assert_eq!(u(&summary["import_gaps"]), computed.import_gaps);
    assert_eq!(u(&summary["authority_gaps"]), computed.authority_gaps);
    assert_eq!(u(&summary["emergency_gaps"]), computed.emergency_gaps);
    assert_eq!(u(&summary["upstream_gaps"]), computed.upstream_gaps);
    assert_eq!(
        u(&summary["hidden_proprietary_baseline_gaps"]),
        computed.hidden_proprietary_baseline_gaps
    );
    assert_eq!(
        u(&summary["ownerless_critical_import_gaps"]),
        computed.ownerless_critical_import_gaps
    );
    assert_eq!(
        u(&summary["single_person_authority_gaps"]),
        computed.single_person_authority_gaps
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
fn certification_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, PublicationDecision::Hold);
    let blocking = r.computed_blocking_record_ids();
    assert!(
        !blocking.is_empty(),
        "a certification failure on a still-stable row must hold promotion"
    );
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
    }
    // The hidden proprietary baseline holds promotion on the boundary axis.
    let hidden = r
        .record("cert-ai_adjacent-ecosystem")
        .expect("hidden-baseline record exists");
    assert_eq!(
        hidden.certification_state,
        CertificationState::NarrowedBoundary
    );
    assert!(hidden.has_active_reason(CertificationReason::HiddenProprietaryBaseline));
    assert!(blocking.contains(&hidden.record_id));
    // The ownerless critical import holds promotion on the import axis.
    let ownerless = r
        .record("cert-framework-ecosystem")
        .expect("ownerless-import record exists");
    assert_eq!(
        ownerless.certification_state,
        CertificationState::NarrowedImport
    );
    assert!(ownerless.has_active_reason(CertificationReason::OwnerlessCriticalImport));
    assert!(blocking.contains(&ownerless.record_id));
    // The single-person emergency authority is the headline durability guardrail.
    let single = r
        .record("cert-data_rich-ecosystem")
        .expect("single-person record exists");
    assert_eq!(
        single.certification_state,
        CertificationState::NarrowedAuthority
    );
    assert!(single.has_active_reason(CertificationReason::SinglePersonEmergencyAuthority));
    assert!(blocking.contains(&single.record_id));
    // The boundary gap held by an unexpired waiver is narrowed and visible, but not held.
    let waived = r
        .record("cert-review-managed-release")
        .expect("waived record exists");
    assert_eq!(
        waived.certification_state,
        CertificationState::NarrowedBoundary
    );
    assert!(waived.is_waived());
    assert!(!blocking.contains(&waived.record_id));
    // The companion preview row already sits below the cutline (Beta): inherited.
    let beta = r
        .record("cert-companion-preview-ecosystem")
        .expect("beta record exists");
    assert!(beta.certification_state.is_narrowed());
    assert!(!beta.declares_at_or_above_cutline());
    assert!(!blocking.contains(&beta.record_id));
}

#[test]
fn stale_and_missing_proof_narrow_on_the_stale_axis() {
    let r = register();
    let stale = r
        .record("cert-managed_depth-ecosystem")
        .expect("stale-proof record exists");
    assert_eq!(stale.certification_state, CertificationState::NarrowedStale);
    assert_eq!(stale.proof_packet.slo_state, FreshnessSloState::Breached);
    let missing = r
        .record("cert-companion-release")
        .expect("missing-proof record exists");
    assert_eq!(
        missing.certification_state,
        CertificationState::NarrowedStale
    );
    assert_eq!(missing.proof_packet.slo_state, FreshnessSloState::Missing);
}

#[test]
fn hidden_axis_gap_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_certified())
        .expect("a certified record exists");
    rec.import_durability.state =
        aureline_governance::m5_open_durability_certification::ImportEvidenceState::OwnerlessCriticalImport;
    rec.import_durability.critical_import_owned = false;
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            RegisterViolation::GapWithoutReason {
                reason: CertificationReason::OwnerlessCriticalImport,
                ..
            } | RegisterViolation::ControlStateInconsistent { .. }
        )),
        "a hidden ownerless-import gap must fail validation"
    );
}

#[test]
fn green_surface_over_a_gapped_scan_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.certification_state.is_narrowed())
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
        .find(|x| x.certification_state.is_narrowed())
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
    let fixtures_dir = repo_root().join("fixtures/governance/m5-open-durability-certification");
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
        let candidate: OpenDurabilityCertificationRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        checked += 1;
    }
    assert!(checked > 0, "at least one fixture must be exercised");
}
