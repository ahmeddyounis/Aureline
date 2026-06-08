//! Protected tests binding the typed open/paid boundary audit to the checked-in artifact,
//! the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in audit; the capture cross-check proves the typed
//! model and the Python gate agree on the publication verdict, the domain coverage counts,
//! the control counts, and the packet-freshness counts; the negative cases mutate a parsed
//! copy and the checked-in fixtures to prove that a row which fails to narrow, an attested
//! row riding a breached packet, a row carried wider than its public claim's ceiling, and a
//! publication verdict that disagrees with the firing rules all fail validation.
//!
//! Cross-artifact fixtures whose `expected_check_id` is a `ceiling.*` check are skipped in
//! the typed-model fixture loop: those flaws (a claim label that disagrees with the stable
//! claim manifest) are only observable by reading the neighbouring manifest, which the CI
//! gate does and the metadata-only typed model deliberately does not.

use std::path::{Path, PathBuf};

use aureline_release::open_paid_boundary_audit::{
    current_open_paid_boundary_audit, AuditDomain, AuditGapReason, AuditState,
    OpenPaidBoundaryAudit, OpenPaidBoundaryAuditViolation, OPEN_PAID_BOUNDARY_AUDIT_RECORD_KIND,
    OPEN_PAID_BOUNDARY_AUDIT_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/open_paid_boundary_audit_validation_capture.json"
));

fn audit() -> OpenPaidBoundaryAudit {
    current_open_paid_boundary_audit()
        .expect("checked-in open/paid boundary audit parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_audit_parses_and_validates() {
    let audit = audit();
    assert_eq!(
        audit.schema_version,
        OPEN_PAID_BOUNDARY_AUDIT_SCHEMA_VERSION
    );
    assert_eq!(audit.record_kind, OPEN_PAID_BOUNDARY_AUDIT_RECORD_KIND);
    let violations = audit.validate();
    assert!(
        violations.is_empty(),
        "checked-in audit must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_domain() {
    let audit = audit();
    for domain in AuditDomain::ALL {
        assert!(
            !audit.rows_for_domain(domain).is_empty(),
            "audit domain {} must have at least one row",
            domain.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_subject() {
    let audit = audit();
    assert!(!audit.release_blocking_audit_refs.is_empty());
    let covered: Vec<&str> = audit
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.entry_id.as_str())
        .collect();
    for declared in &audit.release_blocking_audit_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let audit = audit();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(audit.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_rows"].as_u64().unwrap() as usize,
        audit.rows.len(),
        "capture row count must match the model"
    );
    assert_eq!(
        summary["rows_attested"].as_u64().unwrap() as usize,
        audit.rows_attested().len(),
        "capture attested count must match the model"
    );
    assert_eq!(
        summary["total_controls"].as_u64().unwrap() as usize,
        audit.computed_summary().total_controls,
        "capture control total must match the model"
    );
    assert_eq!(
        summary["controls_unsatisfied"].as_u64().unwrap() as usize,
        audit.computed_summary().controls_unsatisfied,
        "capture unsatisfied-control count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        audit.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        audit.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        audit.publication.decision,
        audit.computed_publication_decision()
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
fn audit_attests_rows_without_narrowing() {
    let audit = audit();
    assert!(
        audit.rows_narrowed().is_empty(),
        "clean audit must not narrow a row"
    );
}

#[test]
fn audit_rows_have_satisfied_controls() {
    let audit = audit();
    assert_eq!(audit.computed_summary().controls_unsatisfied, 0);
    assert!(
        audit
            .rows
            .iter()
            .all(|row| row.unsatisfied_control_count() == 0),
        "clean audit must not carry unsatisfied controls"
    );
}

#[test]
fn audit_can_construct_a_control_narrowed_row() {
    let mut audit = audit();
    let narrowed = audit
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("audit has an attested row");
    narrowed.audit_state = AuditState::NarrowedUnbacked;
    narrowed.effective_label = StableClaimLevel::Beta;
    narrowed.audit_controls[0].satisfied = false;
    narrowed
        .active_gap_reasons
        .push(AuditGapReason::AuditControlUnsatisfied);
    assert!(narrowed.unsatisfied_control_count() > 0);
    assert!(!narrowed.holds_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut audit = audit();
    let row = audit
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("audit has an attested row");
    row.audit_state = AuditState::NarrowedStale;
    row.active_gap_reasons
        .push(AuditGapReason::AttestationPacketFreshnessBreached);
    row.effective_label = StableClaimLevel::Stable;
    audit.summary = audit.computed_summary();
    audit.publication.decision = audit.computed_publication_decision();
    audit.publication.blocking_rule_ids = audit.computed_blocking_rule_ids();
    audit.publication.blocking_entry_ids = audit.computed_blocking_entry_ids();

    assert!(
        audit.validate().iter().any(|v| matches!(
            v,
            OpenPaidBoundaryAuditViolation::EffectiveLabelNotNarrowed { .. }
        )),
        "a row that is not attested must narrow below the cutline"
    );
}

#[test]
fn attested_row_with_unsatisfied_control_fails() {
    let mut audit = audit();
    let row = audit
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("audit has an attested row");
    row.audit_controls[0].satisfied = false;
    audit.summary = audit.computed_summary();

    assert!(
        audit.validate().iter().any(|v| matches!(
            v,
            OpenPaidBoundaryAuditViolation::HeldWithUnsatisfiedControl { .. }
        )),
        "an attested row may not carry an unsatisfied required control"
    );
}

#[test]
fn attested_row_on_a_breached_packet_fails() {
    let mut audit = audit();
    let row = audit
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("audit has an attested row");
    row.attestation_packet.slo_state = FreshnessSloState::Breached;
    audit.summary = audit.computed_summary();

    assert!(
        audit
            .validate()
            .iter()
            .any(|v| matches!(v, OpenPaidBoundaryAuditViolation::HeldOnStalePacket { .. })),
        "an attested row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_decision_mismatch_fails() {
    let mut audit = audit();
    audit.publication.decision = PromotionDecision::Hold;

    assert!(
        audit.validate().iter().any(|v| matches!(
            v,
            OpenPaidBoundaryAuditViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication decision must agree with computed rules"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/open_paid_boundary_audit");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        // Cross-artifact (ceiling) flaws are only observable against the neighbouring claim
        // manifest, which the CI gate reads and the typed model does not.
        let expected = case["expected_check_id"].as_str().unwrap_or_default();
        if expected.starts_with("ceiling.") {
            continue;
        }
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: OpenPaidBoundaryAudit =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        model_checked += 1;
    }
    assert!(
        model_checked > 0,
        "at least one fixture must exercise a typed-model structural invariant"
    );
}
