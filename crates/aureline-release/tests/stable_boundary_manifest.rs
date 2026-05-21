//! Protected tests binding the typed stable boundary manifest to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the frozen, checked-in manifest; the capture cross-check
//! proves the typed model and the Python gate agree on the publication verdict,
//! the per-value-line rollups, and the packet-freshness counts; the negative cases
//! mutate a parsed copy and the checked-in fixtures to prove that a row which fails
//! to narrow, a published line riding a breached packet, a line published wider
//! than its subject's ceiling, and a publication verdict that disagrees with the
//! firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_boundary_manifest::{
    current_stable_boundary_manifest, BoundaryState, StableBoundaryManifest,
    StableBoundaryManifestViolation, ValueLine, STABLE_BOUNDARY_MANIFEST_RECORD_KIND,
    STABLE_BOUNDARY_MANIFEST_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stable_boundary_manifest_validation_capture.json"
));

fn manifest() -> StableBoundaryManifest {
    current_stable_boundary_manifest()
        .expect("checked-in stable boundary manifest parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_manifest_parses_and_validates() {
    let manifest = manifest();
    assert_eq!(
        manifest.schema_version,
        STABLE_BOUNDARY_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(manifest.record_kind, STABLE_BOUNDARY_MANIFEST_RECORD_KIND);
    let violations = manifest.validate();
    assert!(
        violations.is_empty(),
        "checked-in manifest must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_all_four_value_lines_for_every_subject() {
    let manifest = manifest();
    for line in ValueLine::ALL {
        assert!(
            !manifest.rows_for_line(line).is_empty(),
            "no rows for value line {}",
            line.as_str()
        );
    }
    // Three subjects x four value lines.
    assert_eq!(
        manifest.subjects().len() * ValueLine::ALL.len(),
        manifest.rows.len()
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let manifest = manifest();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(manifest.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_boundaries"].as_u64().unwrap() as usize,
        manifest.rows.len(),
        "capture boundary count must match the model"
    );
    assert_eq!(
        summary["boundaries_published_stable"].as_u64().unwrap() as usize,
        manifest.rows_published_stable().len(),
        "capture published count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        manifest.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        manifest.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        manifest.publication.decision,
        manifest.computed_publication_decision()
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
fn air_gapped_line_narrows_a_capability_gap() {
    let manifest = manifest();
    let narrowed = manifest
        .rows_for_line(ValueLine::AirGapped)
        .into_iter()
        .find(|row| {
            row.boundary_state == BoundaryState::NarrowedUnsupported && !row.publishes_stable()
        });
    assert!(
        narrowed.is_some(),
        "the air-gapped line must narrow at least one capability-gapped subject"
    );
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut manifest = manifest();
    let row = manifest
        .rows
        .iter_mut()
        .find(|row| {
            row.boundary_state == BoundaryState::NarrowedUnsupported
                && row.manifest_label == StableClaimLevel::Stable
        })
        .expect("manifest has a narrowed-unsupported row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    manifest.summary = manifest.computed_summary();
    manifest.publication.decision = manifest.computed_publication_decision();
    manifest.publication.blocking_rule_ids = manifest.computed_blocking_rule_ids();
    manifest.publication.blocking_boundary_ids = manifest.computed_blocking_boundary_ids();

    assert!(
        manifest.validate().iter().any(|v| matches!(
            v,
            StableBoundaryManifestViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a value line lacking support must narrow below the cutline"
    );
}

#[test]
fn published_line_on_a_breached_packet_fails() {
    let mut manifest = manifest();
    let row = manifest
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("manifest has a held row");
    row.boundary_packet.slo_state = FreshnessSloState::Breached;
    manifest.summary = manifest.computed_summary();

    assert!(
        manifest
            .validate()
            .iter()
            .any(|v| matches!(v, StableBoundaryManifestViolation::HeldOnStalePacket { .. })),
        "a published line may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut manifest = manifest();
    manifest.publication.decision = PromotionDecision::Proceed;

    assert!(
        manifest.validate().iter().any(|v| matches!(
            v,
            StableBoundaryManifestViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/stable_boundary_manifest");
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
        let candidate: StableBoundaryManifest =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
