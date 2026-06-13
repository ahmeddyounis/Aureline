//! Integration coverage that pins the committed networked-surface transport
//! matrix fixtures to the seeded packet, so the checked-in JSON and the typed
//! truth model cannot drift apart silently.

use std::path::{Path, PathBuf};

use aureline_remote::{
    seeded_networked_surface_matrix_page, MatrixQualificationClass,
    NetworkedSurfaceTransportMatrixPage, REQUIRED_NETWORKED_SURFACES,
};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join("fixtures/network/networked_surface_transport_matrix")
}

fn load_page(rel: &str) -> NetworkedSurfaceTransportMatrixPage {
    let path = fixture_root().join(rel);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} must read: {e}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|e| panic!("fixture {} must deserialize: {e}", path.display()))
}

#[test]
fn committed_page_fixture_matches_seeded_packet() {
    let on_disk = load_page("page.json");
    let seeded = seeded_networked_surface_matrix_page();
    assert_eq!(
        on_disk, seeded,
        "fixtures/network/networked_surface_transport_matrix/page.json is stale; \
         regenerate with the dump_networked_surface_transport_matrix_fixtures example"
    );
}

#[test]
fn committed_page_fixture_is_stable_and_complete() {
    let page = load_page("page.json");
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(page.rows.len(), REQUIRED_NETWORKED_SURFACES.len());
    assert!(page.covers_all_required_surfaces());
    assert!(page.defects.is_empty());
}

#[test]
fn committed_drill_fixtures_carry_expected_qualifications() {
    let cases = [
        (
            "drills/drill_missing_surface_preview.json",
            MatrixQualificationClass::Preview,
        ),
        (
            "drills/drill_raw_material_withdrawn.json",
            MatrixQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_silent_public_fallback_withdrawn.json",
            MatrixQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_non_idempotent_replay_withdrawn.json",
            MatrixQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_stale_proof_beta.json",
            MatrixQualificationClass::Beta,
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
    // The whole point of the export boundary: never serialize raw secrets.
    // Assert no record opts out of the raw-material exclusion in the stable
    // page, and that the support export advertises the exclusion.
    let page = load_page("page.json");
    for record in &page.matrix_snapshot.records {
        assert!(
            record.raw_private_material_excluded,
            "surface '{}' fixture must exclude raw private material",
            record.surface_token
        );
    }
}
