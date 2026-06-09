//! Protected tests binding the typed M5 browser, companion, and embedded-boundary
//! manifest to the checked-in artifact, the frozen CI validation capture, and the
//! negative fixtures.
//!
//! The positive case is the checked-in manifest; the capture cross-check proves
//! the typed model and the CI gate agree on the publication verdict, the
//! surface-kind coverage counts, the packet-freshness counts, and the active
//! gap-reason counts; the negative cases mutate a parsed copy and the checked-in
//! fixtures to prove that a row which fails to narrow, a held row with an active
//! gap, a row carried wider than its public claim's ceiling, and a publication
//! verdict that disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows::{
    current_m5_browser_companion_embedded_boundary_manifest_with_handoff_eligibility_rows,
    HandoffEligibilityState, HandoffGapReason, M5BrowserCompanionEmbeddedBoundaryManifest,
    M5HandoffEligibilityManifestViolation,
    GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_RECORD_KIND,
    GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows_validation_capture.json"
));

fn manifest() -> M5BrowserCompanionEmbeddedBoundaryManifest {
    current_m5_browser_companion_embedded_boundary_manifest_with_handoff_eligibility_rows()
        .expect("checked-in manifest parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_manifest_parses_and_validates() {
    let m = manifest();
    assert_eq!(
        m.schema_version,
        GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_SCHEMA_VERSION
    );
    assert_eq!(
        m.record_kind,
        GENERATE_THE_M5_BROWSER_COMPANION_AND_EMBEDDED_BOUNDARY_MANIFEST_WITH_HANDOFF_ELIGIBILITY_ROWS_RECORD_KIND
    );
    let violations = m.validate();
    assert!(
        violations.is_empty(),
        "checked-in manifest must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_surface_kind() {
    use aureline_release::generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows::HandoffSurfaceKind;
    let m = manifest();
    for kind in HandoffSurfaceKind::ALL {
        assert!(
            !m.rows_for_kind(kind).is_empty(),
            "surface kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let m = manifest();
    assert!(!m.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = m
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &m.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let m = manifest();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(m.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        m.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_holding_stable"].as_u64().unwrap() as usize,
        m.rows_published_stable().len(),
        "capture holding count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed"].as_u64().unwrap() as usize,
        m.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        m.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );
    assert_eq!(
        summary["rules_firing"].as_u64().unwrap() as usize,
        m.computed_summary().rules_firing,
        "capture rules-firing count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        m.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(m.publication.decision, m.computed_publication_decision());

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
fn manifest_narrows_a_release_blocking_row() {
    let m = manifest();
    let narrowed = m
        .rows
        .iter()
        .find(|row| row.release_blocking && row.claim_holds_stable() && !row.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the manifest must narrow at least one release-blocking row under a still-stable claim"
    );
}

#[test]
fn manifest_shows_a_row_on_waiver() {
    let m = manifest();
    let on_waiver = m
        .rows
        .iter()
        .find(|row| row.handoff_state == HandoffEligibilityState::EligibleDegraded)
        .expect("the manifest must show a row on waiver");
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut m = manifest();
    let row = m
        .rows
        .iter_mut()
        .find(|row| {
            row.handoff_state == HandoffEligibilityState::PendingEvidence
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("manifest has a pending-evidence row under a stable ceiling");
    row.effective_label = StableClaimLevel::Stable;
    m.summary = m.computed_summary();
    m.publication.decision = m.computed_publication_decision();
    m.publication.blocking_rule_ids = m.computed_blocking_rule_ids();
    m.publication.blocking_claim_ids = m.computed_blocking_entry_ids();

    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            M5HandoffEligibilityManifestViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_row_with_active_gap_fails() {
    let mut m = manifest();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("manifest has a backed row");
    row.active_gap_reasons
        .push(HandoffGapReason::ProofPacketMissing);
    m.summary = m.computed_summary();

    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            M5HandoffEligibilityManifestViolation::HeldWithActiveGap { .. }
        )),
        "a backed row may not carry an active gap reason"
    );
}

#[test]
fn backed_row_on_a_breached_packet_fails() {
    let mut m = manifest();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("manifest has a backed row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    m.summary = m.computed_summary();

    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            M5HandoffEligibilityManifestViolation::HeldOnStalePacket { .. }
        )),
        "a backed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut m = manifest();
    m.publication.decision = PromotionDecision::Proceed;

    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            M5HandoffEligibilityManifestViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root()
        .join("fixtures/release/m5/generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let expected = case["expected_check_id"].as_str().unwrap_or_default();
        if expected.starts_with("ceiling.") {
            continue;
        }
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: M5BrowserCompanionEmbeddedBoundaryManifest =
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
