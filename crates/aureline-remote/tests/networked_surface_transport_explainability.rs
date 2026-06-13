//! Integration coverage that pins the committed networked-surface
//! transport-explainability fixtures to the seeded packet, so the checked-in
//! JSON and the typed truth model cannot drift apart silently.

use std::path::{Path, PathBuf};

use aureline_remote::{
    seeded_transport_explainability_page, ExplainQualificationClass, TransportExplainabilityPage,
    REQUIRED_NETWORKED_SURFACES,
};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join("fixtures/network/networked_surface_transport_explainability")
}

fn load_page(rel: &str) -> TransportExplainabilityPage {
    let path = fixture_root().join(rel);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} must read: {e}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|e| panic!("fixture {} must deserialize: {e}", path.display()))
}

#[test]
fn committed_page_fixture_matches_seeded_packet() {
    let on_disk = load_page("page.json");
    let seeded = seeded_transport_explainability_page();
    assert_eq!(
        on_disk, seeded,
        "fixtures/network/networked_surface_transport_explainability/page.json is stale; \
         regenerate with the dump_networked_surface_transport_explainability_fixtures example"
    );
}

#[test]
fn committed_page_fixture_is_stable_and_complete() {
    let page = load_page("page.json");
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(page.rows.len(), REQUIRED_NETWORKED_SURFACES.len());
    assert_eq!(
        page.posture_inspectors.len(),
        REQUIRED_NETWORKED_SURFACES.len()
    );
    assert_eq!(page.explain_sheets.len(), REQUIRED_NETWORKED_SURFACES.len());
    assert_eq!(
        page.event_ledger.entries.len(),
        REQUIRED_NETWORKED_SURFACES.len()
    );
    assert!(page.covers_all_required_surfaces());
    assert!(page.all_explain_sheets_at_parity());
    assert!(page.denied_events_carry_reasons());
    assert!(page.defects.is_empty());
}

#[test]
fn committed_drill_fixtures_carry_expected_qualifications() {
    let cases = [
        (
            "drills/drill_missing_surface_preview.json",
            ExplainQualificationClass::Preview,
        ),
        (
            "drills/drill_raw_material_withdrawn.json",
            ExplainQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_bypass_withdrawn.json",
            ExplainQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_denied_no_reason_beta.json",
            ExplainQualificationClass::Beta,
        ),
        (
            "drills/drill_stale_proof_beta.json",
            ExplainQualificationClass::Beta,
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
    for inspector in &page.posture_inspectors {
        assert!(
            inspector.raw_private_material_excluded,
            "posture inspector for '{}' must exclude raw private material",
            inspector.surface_token
        );
    }
    for entry in &page.event_ledger.entries {
        assert!(
            entry.raw_private_material_excluded,
            "ledger event '{}' must exclude raw private material",
            entry.event_id
        );
    }
    for sheet in &page.explain_sheets {
        assert!(
            sheet.raw_private_material_excluded,
            "explain sheet '{}' must exclude raw private material",
            sheet.action_id
        );
    }
}
