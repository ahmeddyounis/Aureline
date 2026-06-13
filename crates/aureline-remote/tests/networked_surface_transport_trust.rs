//! Integration coverage that pins the committed networked-surface
//! transport-trust fixtures to the seeded packet, so the checked-in JSON and the
//! typed truth model cannot drift apart silently.

use std::path::{Path, PathBuf};

use aureline_remote::{
    seeded_transport_trust_page, TransportTrustPage, TrustQualificationClass,
    REQUIRED_NETWORKED_SURFACES,
};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join("fixtures/network/networked_surface_transport_trust")
}

fn load_page(rel: &str) -> TransportTrustPage {
    let path = fixture_root().join(rel);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} must read: {e}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|e| panic!("fixture {} must deserialize: {e}", path.display()))
}

#[test]
fn committed_page_fixture_matches_seeded_packet() {
    let on_disk = load_page("page.json");
    let seeded = seeded_transport_trust_page();
    assert_eq!(
        on_disk, seeded,
        "fixtures/network/networked_surface_transport_trust/page.json is stale; \
         regenerate with the dump_networked_surface_transport_trust_fixtures example"
    );
}

#[test]
fn committed_page_fixture_is_stable_and_complete() {
    let page = load_page("page.json");
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(page.rows.len(), REQUIRED_NETWORKED_SURFACES.len());
    assert!(page.covers_all_required_surfaces());
    assert!(page.no_record_ships_direct_ca_override());
    assert!(page.no_record_allows_silent_trust_downgrade());
    assert!(page.all_records_expose_host_proof_state());
    assert!(page.all_records_expose_trust_inputs());
    assert!(page.rotation_cues_consistent());
    assert!(page.defects.is_empty());
}

#[test]
fn committed_drill_fixtures_carry_expected_qualifications() {
    let cases = [
        (
            "drills/drill_missing_surface_preview.json",
            TrustQualificationClass::Preview,
        ),
        (
            "drills/drill_raw_material_withdrawn.json",
            TrustQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_private_key_withdrawn.json",
            TrustQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_ca_override_withdrawn.json",
            TrustQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_silent_downgrade_withdrawn.json",
            TrustQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_denied_no_reason_beta.json",
            TrustQualificationClass::Beta,
        ),
        (
            "drills/drill_missing_rotation_cue_beta.json",
            TrustQualificationClass::Beta,
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
fn no_committed_fixture_carries_raw_trust_or_private_key_material() {
    let page = load_page("page.json");
    for record in &page.trust_snapshot.records {
        assert!(
            record.raw_trust_material_excluded,
            "record '{}' fixture must exclude raw trust material",
            record.record_id
        );
        assert!(
            record.private_key_material_excluded,
            "record '{}' fixture must exclude raw private-key material",
            record.record_id
        );
        assert!(
            record.ca_bundle.bundle_handle.starts_with("trust_bundle:"),
            "record '{}' CA bundle must be an opaque handle, never raw CA bytes",
            record.record_id
        );
        assert!(
            record.host_proof.proof_handle.starts_with("host_proof:"),
            "record '{}' host proof must be an opaque handle, never a raw host key",
            record.record_id
        );
    }
}
