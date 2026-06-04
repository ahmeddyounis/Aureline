//! Protected tests for the browser/mobile companion qualification packet.

use std::path::{Path, PathBuf};

use aureline_release::browser_mobile_companion_surface_qualification::{
    current_browser_mobile_companion_surface_qualification,
    BrowserMobileCompanionSurfaceQualification, CompanionFreshness,
    CompanionQualificationViolation, BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_RECORD_KIND,
    BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};
use aureline_release::stable_claim_matrix::StableClaimLevel;
use serde::Deserialize;

fn packet() -> BrowserMobileCompanionSurfaceQualification {
    current_browser_mobile_companion_surface_qualification()
        .expect("checked-in browser/mobile companion packet parses")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    cases: Vec<FixtureCase>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    case_id: String,
    expected_check_id: String,
}

#[test]
fn checked_in_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_RECORD_KIND
    );
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in browser/mobile companion packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_matches_row_state_and_preview_rows_stay_narrow() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.stable_surfaces().len(), 3);
    assert_eq!(packet.narrowed_surfaces().len(), 3);

    for surface in &packet.surfaces {
        assert!(
            surface.displayed_label.rank() <= surface.claim_label.rank(),
            "{} displays wider than its claim",
            surface.surface_id
        );
        if !surface.renders_stable() {
            assert!(
                surface.visible_label.is_narrow(),
                "{} must carry a visible preview/labs/unsupported label",
                surface.surface_id
            );
        }
    }
}

#[test]
fn stable_rows_have_scope_freshness_handoff_and_validation_evidence() {
    let packet = packet();
    for surface in packet.stable_surfaces() {
        assert!(
            surface.has_green_packet(),
            "{} lacks green packet",
            surface.surface_id
        );
        assert!(
            surface.owner_signoff.signed_off,
            "{} lacks owner sign-off",
            surface.surface_id
        );
        assert!(
            surface.has_scope_truth(),
            "{} lacks scope truth",
            surface.surface_id
        );
        assert!(
            surface.has_freshness_truth(),
            "{} lacks freshness cueing",
            surface.surface_id
        );
        assert!(
            !surface.accessibility_refs.is_empty()
                && !surface.privacy_refs.is_empty()
                && !surface.support_export_refs.is_empty(),
            "{} lacks validation refs",
            surface.surface_id
        );
    }
}

#[test]
fn fixture_manifest_is_present_and_negative_drills_fire_expected_checks() {
    let fixture_path = repo_root()
        .join("fixtures/release/m4/browser-mobile-companion-surface-qualification/cases.json");
    let payload = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", fixture_path.display()));
    let manifest: FixtureManifest =
        serde_json::from_str(&payload).expect("fixture manifest parses");
    assert_eq!(manifest.schema_version, 1);
    assert!(!manifest.cases.is_empty());

    for case in manifest.cases {
        let mut packet = packet();
        match case.case_id.as_str() {
            "stable_surface_without_green_packet" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.qualification_packet = None;
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    CompanionQualificationViolation::StableSurfaceWithoutGreenPacket { surface_id },
                );
            }
            "missing_scope_truth" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.unsupported_actions.clear();
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    CompanionQualificationViolation::MissingScopeTruth { surface_id },
                );
            }
            "missing_freshness_cue" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.freshness = CompanionFreshness::Stale;
                    surface.freshness_cues.clear();
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    CompanionQualificationViolation::MissingFreshnessCue { surface_id },
                );
            }
            "incomplete_desktop_handoff" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.desktop_handoff.return_anchor = false;
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    CompanionQualificationViolation::IncompleteDesktopHandoff { surface_id },
                );
            }
            "companion_surface_overclaim" => {
                let borrowed_packet = packet.surfaces[0].qualification_packet.clone();
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| !surface.renders_stable())
                        .expect("preview surface exists");
                    surface.claim_label = StableClaimLevel::Stable;
                    surface.displayed_label = StableClaimLevel::Stable;
                    surface.qualification_packet = borrowed_packet;
                    surface.surface_id.clone()
                };
                packet.summary = packet.computed_summary();
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    CompanionQualificationViolation::StableSurfaceLacksStableLabel { surface_id },
                );
            }
            other => panic!("unknown fixture case: {other}"),
        }
    }
}

fn assert_expected(
    expected_check_id: &str,
    violations: Vec<CompanionQualificationViolation>,
    expected: CompanionQualificationViolation,
) {
    assert!(
        violations.contains(&expected),
        "expected {expected_check_id} to fire {expected:?}, got {violations:#?}"
    );
}
