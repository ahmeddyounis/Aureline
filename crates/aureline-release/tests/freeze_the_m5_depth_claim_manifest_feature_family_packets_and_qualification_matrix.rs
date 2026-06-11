//! Protected tests binding the typed M5 depth-claim manifest to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in manifest; the capture cross-check proves
//! the typed model and the CI gate agree on the promotion verdict, the
//! qualification-cell counts, and the packet-freshness counts; the negative
//! cases mutate a parsed copy and the checked-in fixtures to prove that a packet
//! that fails to narrow, a held packet with an active gap, a packet carried
//! wider than its depth claim's ceiling, a qualification matrix missing a
//! dimension, and a promotion verdict that disagrees with the firing rules all
//! fail validation.

use std::path::{Path, PathBuf};

use aureline_release::freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix::{
    current_m5_depth_claim_manifest, DepthClaimManifest, DepthClaimManifestViolation, FamilyKind,
    NarrowingReason, PacketState, QualificationDimension,
    FREEZE_M5_DEPTH_CLAIM_MANIFEST_RECORD_KIND, FREEZE_M5_DEPTH_CLAIM_MANIFEST_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix_validation_capture.json"
));

fn manifest() -> DepthClaimManifest {
    current_m5_depth_claim_manifest().expect("checked-in manifest parses into the model")
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
        FREEZE_M5_DEPTH_CLAIM_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(m.record_kind, FREEZE_M5_DEPTH_CLAIM_MANIFEST_RECORD_KIND);
    let violations = m.validate();
    assert!(
        violations.is_empty(),
        "checked-in manifest must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_family_kind_and_dimension() {
    let m = manifest();
    for kind in FamilyKind::ALL {
        assert!(
            !m.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one packet",
            kind.as_str()
        );
    }
    for row in &m.rows {
        for dimension in QualificationDimension::ALL {
            assert!(
                row.cell(dimension).is_some(),
                "packet {} must cover dimension {}",
                row.entry_id,
                dimension.as_str()
            );
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_family() {
    let m = manifest();
    assert!(!m.release_blocking_family_refs.is_empty());
    let covered: Vec<&str> = m
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.family_ref.as_str())
        .collect();
    for declared in &m.release_blocking_family_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking packet"
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
    let computed = m.computed_summary();
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        m.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_qualified"].as_u64().unwrap() as usize,
        m.rows_published_stable().len(),
        "capture qualified count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed"].as_u64().unwrap() as usize,
        m.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["packets_missing"].as_u64().unwrap() as usize,
        computed.packets_missing,
        "capture missing-packet count must match the model"
    );
    assert_eq!(
        summary["total_qualification_cells"].as_u64().unwrap() as usize,
        computed.total_qualification_cells,
        "capture total-cell count must match the model"
    );
    assert_eq!(
        summary["cells_qualified"].as_u64().unwrap() as usize,
        computed.cells_qualified,
        "capture qualified-cell count must match the model"
    );
    assert_eq!(
        summary["cells_missing"].as_u64().unwrap() as usize,
        computed.cells_missing,
        "capture missing-cell count must match the model"
    );
    assert_eq!(
        summary["rules_firing"].as_u64().unwrap() as usize,
        computed.rules_firing,
        "capture firing-rule count must match the model"
    );

    let captured_decision = capture["promotion"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        m.promotion.decision.as_str(),
        "capture promotion decision must match the model"
    );
    assert_eq!(m.promotion.decision, m.computed_promotion_decision());

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
fn manifest_narrows_a_release_blocking_family() {
    let m = manifest();
    let narrowed = m
        .rows
        .iter()
        .find(|row| row.release_blocking && row.claim_holds_stable() && !row.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the manifest must narrow at least one release-blocking family under a still-stable claim"
    );
}

#[test]
fn manifest_shows_a_family_on_waiver() {
    let m = manifest();
    let on_waiver = m
        .rows
        .iter()
        .find(|row| row.packet_state == PacketState::OnWaiver)
        .expect("the manifest must show a family on waiver");
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn narrowing_family_that_does_not_narrow_fails() {
    let mut m = manifest();
    let row = m
        .rows
        .iter_mut()
        .find(|row| !row.holds_label() && row.claim_label == StableClaimLevel::Stable)
        .expect("manifest has a narrowed packet under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    m.summary = m.computed_summary();
    m.promotion.decision = m.computed_promotion_decision();
    m.promotion.blocking_rule_ids = m.computed_blocking_rule_ids();
    m.promotion.blocking_claim_ids = m.computed_blocking_entry_ids();

    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            DepthClaimManifestViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a family that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_family_with_active_gap_fails() {
    let mut m = manifest();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("manifest has a backed packet");
    row.active_narrowing_reasons
        .push(NarrowingReason::LineageMissing);
    m.summary = m.computed_summary();

    assert!(
        m.validate()
            .iter()
            .any(|v| matches!(v, DepthClaimManifestViolation::HeldWithActiveGap { .. })),
        "a backed family may not carry an active narrowing reason"
    );
}

#[test]
fn backed_family_on_a_breached_packet_fails() {
    let mut m = manifest();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("manifest has a backed packet");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    m.summary = m.computed_summary();

    assert!(
        m.validate()
            .iter()
            .any(|v| matches!(v, DepthClaimManifestViolation::HeldOnStalePacket { .. })),
        "a backed family may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut m = manifest();
    m.promotion.decision = PromotionDecision::Proceed;

    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            DepthClaimManifestViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join(
        "fixtures/release/m5/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix",
    );
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: DepthClaimManifest =
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
