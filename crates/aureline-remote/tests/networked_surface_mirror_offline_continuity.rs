//! Integration coverage that pins the committed networked-surface mirror/offline
//! continuity fixtures to the seeded packet, so the checked-in JSON and the
//! typed truth model cannot drift apart silently.

use std::path::{Path, PathBuf};

use aureline_remote::{
    seeded_mirror_offline_continuity_page, ContinuityQualificationClass,
    MirrorOfflineContinuityPage, REQUIRED_ARTIFACT_FAMILIES,
};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join("fixtures/network/networked_surface_mirror_offline_continuity")
}

fn load_page(rel: &str) -> MirrorOfflineContinuityPage {
    let path = fixture_root().join(rel);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture {} must read: {e}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|e| panic!("fixture {} must deserialize: {e}", path.display()))
}

#[test]
fn committed_page_fixture_matches_seeded_packet() {
    let on_disk = load_page("page.json");
    let seeded = seeded_mirror_offline_continuity_page();
    assert_eq!(
        on_disk, seeded,
        "fixtures/network/networked_surface_mirror_offline_continuity/page.json is stale; \
         regenerate with the dump_networked_surface_mirror_offline_continuity_fixtures example"
    );
}

#[test]
fn committed_page_fixture_is_stable_and_complete() {
    let page = load_page("page.json");
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(page.rows.len(), REQUIRED_ARTIFACT_FAMILIES.len());
    assert_eq!(
        page.continuity_snapshot.records.len(),
        REQUIRED_ARTIFACT_FAMILIES.len()
    );
    assert!(page.covers_all_required_families());
    assert!(page.distinguishes_all_route_classes());
    assert!(page.no_record_allows_silent_public_fallback());
    assert!(page.replay_queues_are_idempotent_only());
    assert!(page.all_records_preserve_local_core());
    assert!(page.blocked_records_carry_reasons());
    assert!(page.all_fallback_rules_consistent());
    assert!(page.all_records_at_field_parity());
    assert!(page.defects.is_empty());
}

#[test]
fn committed_drill_fixtures_carry_expected_qualifications() {
    let cases = [
        (
            "drills/drill_missing_family_preview.json",
            ContinuityQualificationClass::Preview,
        ),
        (
            "drills/drill_raw_material_withdrawn.json",
            ContinuityQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_silent_fallback_withdrawn.json",
            ContinuityQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_non_idempotent_withdrawn.json",
            ContinuityQualificationClass::Withdrawn,
        ),
        (
            "drills/drill_blocked_no_reason_beta.json",
            ContinuityQualificationClass::Beta,
        ),
        (
            "drills/drill_stale_mirror_beta.json",
            ContinuityQualificationClass::Beta,
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
    for record in &page.continuity_snapshot.records {
        assert!(
            record.raw_private_material_excluded,
            "continuity record for '{}' must exclude raw private material",
            record.artifact_family_token
        );
    }
}
