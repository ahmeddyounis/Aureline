//! Protected tests binding the typed support-class ledger to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the frozen, checked-in ledger; the capture cross-check
//! proves the typed model and the Python gate agree on the publication verdict
//! and summary; the negative cases mutate a parsed copy and the checked-in
//! fixtures to prove that an entry which fails to narrow, a Certified claim with
//! no manifest entry, or a publication verdict that disagrees with the firing
//! downgrade rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::support_class_ledger::{
    current_support_class_ledger, DowngradeReason, LedgerState, PublicationDecision, SupportClass,
    SupportClassLedger, SupportClassLedgerViolation, SUPPORT_CLASS_LEDGER_RECORD_KIND,
    SUPPORT_CLASS_LEDGER_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/support_class_ledger_validation_capture.json"
));

fn ledger() -> SupportClassLedger {
    current_support_class_ledger().expect("checked-in support-class ledger parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_ledger_parses_and_validates() {
    let ledger = ledger();
    assert_eq!(ledger.schema_version, SUPPORT_CLASS_LEDGER_SCHEMA_VERSION);
    assert_eq!(ledger.record_kind, SUPPORT_CLASS_LEDGER_RECORD_KIND);
    let violations = ledger.validate();
    assert!(
        violations.is_empty(),
        "checked-in ledger must validate cleanly: {violations:#?}"
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let ledger = ledger();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(ledger.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        ledger.entries.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_published_as_claimed"].as_u64().unwrap() as usize,
        ledger.entries_holding().len(),
        "capture held count must match the model"
    );
    assert_eq!(
        summary["downgrade_rules_firing"].as_u64().unwrap() as usize,
        ledger
            .downgrade_rules
            .iter()
            .filter(|rule| ledger.downgrade_rule_fires(rule))
            .count(),
        "capture firing-rule count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        ledger.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        ledger.publication.decision,
        ledger.computed_publication_decision()
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
fn unqualified_entry_that_does_not_narrow_fails() {
    let mut ledger = ledger();
    let entry = ledger
        .entries
        .iter_mut()
        .find(|e| e.ledger_state == LedgerState::NarrowedUnqualified)
        .expect("ledger has a narrowed-unqualified entry");
    entry.effective_class = entry.claimed_class;
    ledger.summary = ledger.computed_summary();
    ledger.publication.decision = ledger.computed_publication_decision();
    ledger.publication.blocking_rule_ids = ledger.computed_blocking_rule_ids();
    ledger.publication.blocking_entry_ids = ledger.computed_blocking_entry_ids();

    assert!(
        ledger
            .validate()
            .iter()
            .any(|v| matches!(v, SupportClassLedgerViolation::EffectiveNotNarrowed { .. })),
        "a subject lacking qualification must narrow below its claimed class"
    );
}

#[test]
fn certified_claim_without_a_manifest_entry_fails() {
    let mut ledger = ledger();
    let entry = ledger
        .entries
        .iter_mut()
        .find(|e| e.claimed_class == SupportClass::Certified)
        .expect("ledger has a certified entry");
    entry.certified_archetype_ref = Some("certified_archetype:does_not_exist".to_owned());

    assert!(
        ledger.validate().iter().any(|v| matches!(
            v,
            SupportClassLedgerViolation::CertifiedArchetypeNotInManifest { .. }
        )),
        "a Certified claim must reference a manifest entry that exists"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut ledger = ledger();
    ledger.publication.decision = PublicationDecision::Proceed;

    assert!(
        ledger.validate().iter().any(|v| matches!(
            v,
            SupportClassLedgerViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking downgrade rule fires"
    );
}

#[test]
fn stripping_a_rule_for_an_active_reason_fails() {
    let mut ledger = ledger();
    ledger
        .downgrade_rules
        .retain(|rule| rule.trigger_reason != DowngradeReason::WaiverExpired);
    ledger.summary = ledger.computed_summary();
    ledger.publication.decision = ledger.computed_publication_decision();
    ledger.publication.blocking_rule_ids = ledger.computed_blocking_rule_ids();
    ledger.publication.blocking_entry_ids = ledger.computed_blocking_entry_ids();

    assert!(
        ledger
            .validate()
            .contains(&SupportClassLedgerViolation::DowngradeReasonWithoutRule {
                reason: DowngradeReason::WaiverExpired,
            }),
        "every downgrade reason must keep a rule watching for it"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/support_class_ledger");
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
        let candidate: SupportClassLedger =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
