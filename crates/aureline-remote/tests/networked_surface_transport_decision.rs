//! Integration coverage that pins the committed networked-surface
//! transport-decision fixtures to the seeded packet, so the checked-in JSON and
//! the typed truth model cannot drift apart silently.

use std::path::{Path, PathBuf};

use aureline_remote::{
    seeded_transport_decision_page, DecisionQualificationClass, TransportDecisionLogPage,
    REQUIRED_NETWORKED_SURFACES,
};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join("fixtures/network/networked_surface_transport_decision")
}

fn load_page(rel: &str) -> TransportDecisionLogPage {
    let path = fixture_root().join(rel);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} must read: {e}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|e| panic!("fixture {} must deserialize: {e}", path.display()))
}

#[test]
fn committed_page_fixture_matches_seeded_packet() {
    let on_disk = load_page("page.json");
    let seeded = seeded_transport_decision_page();
    assert_eq!(
        on_disk, seeded,
        "fixtures/network/networked_surface_transport_decision/page.json is stale; \
         regenerate with the dump_networked_surface_transport_decision_fixtures example"
    );
}

#[test]
fn committed_page_fixture_is_stable_and_complete() {
    let page = load_page("page.json");
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(page.rows.len(), REQUIRED_NETWORKED_SURFACES.len());
    assert!(page.covers_all_required_surfaces());
    assert!(page.no_decision_bypasses_governance());
    assert!(page.defects.is_empty());
}

#[test]
fn committed_drill_fixtures_carry_expected_qualifications() {
    let cases = [
        (
            "drills/drill_missing_surface_preview.json",
            DecisionQualificationClass::Preview,
        ),
        (
            "drills/drill_raw_material_withdrawn.json",
            DecisionQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_bypass_withdrawn.json",
            DecisionQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_silent_public_fallback_withdrawn.json",
            DecisionQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_non_idempotent_replay_withdrawn.json",
            DecisionQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_denied_no_reason_beta.json",
            DecisionQualificationClass::Beta,
        ),
        (
            "drills/drill_stale_proof_beta.json",
            DecisionQualificationClass::Beta,
        ),
    ];
    for (rel, expected) in cases {
        let page = load_page(rel);
        assert_eq!(
            page.summary.overall_qualification_token,
            expected.as_str(),
            "{rel} should qualify {}",
            expected.as_str()
        );
    }
}

#[test]
fn no_committed_fixture_carries_raw_private_material() {
    let page = load_page("page.json");
    for decision in &page.decision_snapshot.decisions {
        assert!(
            decision.raw_private_material_excluded,
            "decision '{}' fixture must exclude raw private material",
            decision.decision_id
        );
        assert!(
            decision.endpoint.endpoint_handle.starts_with("endpoint:"),
            "decision '{}' endpoint must be an opaque handle, never a raw URL",
            decision.decision_id
        );
    }
}
