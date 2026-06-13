//! Integration coverage that pins the committed M5 transport-governance
//! certification fixtures to the seeded packet, so the checked-in JSON and the
//! typed truth model cannot drift apart silently.

use std::path::{Path, PathBuf};

use aureline_remote::{
    seeded_m5_transport_governance_certification_page, CertificationVerdictClass,
    M5TransportGovernanceCertificationPage, REQUIRED_DIMENSIONS, REQUIRED_PROFILES,
};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join("fixtures/network/m5_transport_governance_certification")
}

fn load_page(rel: &str) -> M5TransportGovernanceCertificationPage {
    let path = fixture_root().join(rel);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} must read: {e}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|e| panic!("fixture {} must deserialize: {e}", path.display()))
}

#[test]
fn committed_page_fixture_matches_seeded_packet() {
    let on_disk = load_page("page.json");
    let seeded = seeded_m5_transport_governance_certification_page();
    assert_eq!(
        on_disk, seeded,
        "fixtures/network/m5_transport_governance_certification/page.json is stale; \
         regenerate with the dump_m5_transport_governance_certification_fixtures example"
    );
}

#[test]
fn committed_page_fixture_is_certified_and_complete() {
    let page = load_page("page.json");
    assert!(page.is_certified());
    assert!(page.no_withdrawn_rows());
    assert_eq!(page.rows.len(), REQUIRED_PROFILES.len());
    assert!(page.covers_all_required_profiles());
    assert!(page.covers_all_dimensions());
    assert!(page.binds_all_dimensions());
    assert!(page.all_cells_at_field_parity());
    assert!(page.raw_private_material_excluded());
    assert!(page.defects.is_empty());
    assert_eq!(page.dimension_bindings.len(), REQUIRED_DIMENSIONS.len());
}

#[test]
fn committed_drill_fixtures_carry_expected_verdicts() {
    let cases = [
        (
            "drills/drill_stale_narrowed.json",
            CertificationVerdictClass::Narrowed,
        ),
        (
            "drills/drill_missing_continuity_held.json",
            CertificationVerdictClass::HeldBack,
        ),
        (
            "drills/drill_missing_profile_held.json",
            CertificationVerdictClass::HeldBack,
        ),
        (
            "drills/drill_raw_material_withdrawn.json",
            CertificationVerdictClass::Withdrawn,
        ),
        (
            "drills/drill_fallthrough_withdrawn.json",
            CertificationVerdictClass::Withdrawn,
        ),
    ];
    for (rel, expected) in cases {
        let page = load_page(rel);
        assert_eq!(
            page.summary.overall_verdict_token,
            expected.as_str(),
            "{rel} should resolve to {}",
            expected.as_str()
        );
    }
}

#[test]
fn no_committed_fixture_carries_raw_private_material() {
    let page = load_page("page.json");
    assert!(page.raw_private_material_excluded());
    for profile in &page.certification_snapshot.profiles {
        assert!(
            profile.raw_private_material_excluded,
            "profile '{}' must exclude raw private material",
            profile.profile_token
        );
    }
}
