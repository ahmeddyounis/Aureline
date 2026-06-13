//! Integration coverage that pins the committed networked-surface
//! proxy-resolution fixtures to the seeded packet, so the checked-in JSON and
//! the typed truth model cannot drift apart silently.

use std::path::{Path, PathBuf};

use aureline_remote::{
    seeded_proxy_resolution_page, ProxyQualificationClass, ProxyResolutionGovernancePage,
    REQUIRED_NETWORKED_SURFACES,
};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join("fixtures/network/networked_surface_proxy_resolution")
}

fn load_page(rel: &str) -> ProxyResolutionGovernancePage {
    let path = fixture_root().join(rel);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} must read: {e}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|e| panic!("fixture {} must deserialize: {e}", path.display()))
}

#[test]
fn committed_page_fixture_matches_seeded_packet() {
    let on_disk = load_page("page.json");
    let seeded = seeded_proxy_resolution_page();
    assert_eq!(
        on_disk, seeded,
        "fixtures/network/networked_surface_proxy_resolution/page.json is stale; \
         regenerate with the dump_networked_surface_proxy_resolution_fixtures example"
    );
}

#[test]
fn committed_page_fixture_is_stable_and_complete() {
    let page = load_page("page.json");
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(page.rows.len(), REQUIRED_NETWORKED_SURFACES.len());
    assert!(page.covers_all_required_surfaces());
    assert!(page.no_record_ships_private_proxy_stack());
    assert!(page.no_record_ships_direct_ca_override());
    assert!(page.no_record_allows_silent_direct_fallback());
    assert!(page.all_records_respect_precedence());
    assert!(page.defects.is_empty());
}

#[test]
fn committed_drill_fixtures_carry_expected_qualifications() {
    let cases = [
        (
            "drills/drill_missing_surface_preview.json",
            ProxyQualificationClass::Preview,
        ),
        (
            "drills/drill_raw_material_withdrawn.json",
            ProxyQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_private_stack_withdrawn.json",
            ProxyQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_ca_override_withdrawn.json",
            ProxyQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_silent_fallback_withdrawn.json",
            ProxyQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_denied_no_reason_beta.json",
            ProxyQualificationClass::Beta,
        ),
        (
            "drills/drill_precedence_not_respected_beta.json",
            ProxyQualificationClass::Beta,
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
    for record in &page.resolution_snapshot.records {
        assert!(
            record.raw_private_material_excluded,
            "record '{}' fixture must exclude raw private material",
            record.record_id
        );
        for candidate in &record.candidates {
            assert!(
                candidate.candidate_handle.starts_with("proxy_source:"),
                "record '{}' candidate must be an opaque handle, never a raw proxy host",
                record.record_id
            );
        }
    }
}
