//! Fixture-driven coverage for the region / tenant / key-mode beta projection.
//!
//! These tests parse the protected fixtures under
//! `/fixtures/security/m3/region_tenant_key_mode`, validate the seeded page,
//! confirm that every drill axis is covered, and prove that the support-export
//! wrapper preserves the no-public-endpoint-fallback and local-editing
//! invariants.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    audit_region_tenant_key_mode_beta_page, validate_region_tenant_key_mode_beta_page,
    RegionTenantKeyModeBetaPage, RegionTenantKeyModeBetaProfileClass,
    RegionTenantKeyModeBetaSupportExport,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/security/m3/region_tenant_key_mode")
}

fn load_page(file_name: &str) -> RegionTenantKeyModeBetaPage {
    let path = fixture_dir().join(file_name);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a page: {err}"))
}

#[test]
fn seeded_page_fixture_validates_with_zero_defects() {
    let page = load_page("page.json");
    validate_region_tenant_key_mode_beta_page(&page).expect("seeded page validates");
    assert!(page.defects.is_empty());
    for profile in RegionTenantKeyModeBetaProfileClass::ALL {
        assert!(page
            .summary
            .profiles_present
            .iter()
            .any(|token| token == profile.as_str()));
    }
}

#[test]
fn fixture_covers_region_tenant_and_key_mode_drill_axes() {
    let page = load_page("page.json");
    let axes: BTreeSet<&str> = page
        .drill_packets
        .iter()
        .map(|packet| packet.axis_token.as_str())
        .collect();
    assert!(axes.contains("region"));
    assert!(axes.contains("tenant"));
    assert!(axes.contains("key_mode"));
}

#[test]
fn fixture_rows_preserve_no_public_fallback_and_local_editing() {
    let page = load_page("page.json");
    for row in &page.region_rows {
        assert!(row.no_public_endpoint_fallback);
        assert!(row.raw_private_material_excluded);
        assert!(row.local_editing_preserved);
    }
    for row in &page.tenant_rows {
        assert!(row.no_public_endpoint_fallback);
        assert!(row.raw_private_material_excluded);
        assert!(row.local_editing_preserved);
    }
    for row in &page.key_mode_rows {
        assert!(row.no_public_endpoint_fallback);
        assert!(row.raw_private_material_excluded);
        assert!(row.local_editing_preserved);
    }
}

#[test]
fn fixture_drill_packets_never_widen_sibling_lanes() {
    let page = load_page("page.json");
    for packet in &page.drill_packets {
        assert!(
            packet.sibling_lanes_unwidened,
            "drill {} widened a sibling lane",
            packet.drill_packet_id,
        );
        assert!(packet.local_editing_preserved);
        assert!(packet.raw_private_material_excluded);
    }
}

#[test]
fn support_export_round_trip_preserves_invariants() {
    let page = load_page("page.json");
    let export = RegionTenantKeyModeBetaSupportExport::from_page(
        "region-tenant-key-mode:support-export:fixture-001",
        "2026-05-16T05:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.no_public_endpoint_fallback_invariant);
    assert!(export.local_editing_preserved_invariant);
    assert!(export.defect_kinds_present.is_empty());
}

#[test]
fn fixture_audit_matches_validator_recompute() {
    let page = load_page("page.json");
    let recomputed = audit_region_tenant_key_mode_beta_page(
        &page.region_rows,
        &page.tenant_rows,
        &page.key_mode_rows,
        &page.drill_packets,
    );
    assert!(recomputed.is_empty(), "fixture must hold zero defects");
}
