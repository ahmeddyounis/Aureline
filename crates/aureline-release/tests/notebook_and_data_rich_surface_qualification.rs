//! Protected tests for the notebook/data-rich surface qualification packet.

use std::path::{Path, PathBuf};

use aureline_release::notebook_and_data_rich_surface_qualification::{
    current_notebook_data_rich_surface_qualification, NotebookDataQualificationViolation,
    NotebookDataRichSurfaceQualification, NotebookDataSurfaceKind,
    NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_RECORD_KIND,
    NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};
use aureline_release::stable_claim_matrix::StableClaimLevel;
use serde::Deserialize;

fn packet() -> NotebookDataRichSurfaceQualification {
    current_notebook_data_rich_surface_qualification()
        .expect("checked-in notebook/data-rich packet parses")
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
        NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_RECORD_KIND
    );
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in notebook/data-rich packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_matches_row_state_and_preview_rows_stay_narrow() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.stable_surfaces().len(), 3);
    assert_eq!(packet.narrowed_surfaces().len(), 5);

    for surface in &packet.surfaces {
        assert!(
            surface.displayed_label.rank() <= surface.claim_label.rank(),
            "{} displays wider than its claim",
            surface.surface_id
        );
        if matches!(
            surface.surface_kind,
            NotebookDataSurfaceKind::ResultGrid
                | NotebookDataSurfaceKind::ApiDatabaseResponseViewer
                | NotebookDataSurfaceKind::ExperimentHandoff
                | NotebookDataSurfaceKind::ChartSummary
        ) {
            assert_ne!(
                surface.displayed_label,
                StableClaimLevel::Stable,
                "{} must not imply stable data/database/experiment depth",
                surface.surface_id
            );
        }
    }
}

#[test]
fn stable_rows_have_required_notebook_and_projection_contracts() {
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
            surface.has_required_visible_state(),
            "{} lacks visible states",
            surface.surface_id
        );
        assert!(
            !surface.accessibility_refs.is_empty(),
            "{} lacks accessibility refs",
            surface.surface_id
        );
        assert!(
            !surface.snapshot_golden_refs.is_empty() && !surface.support_export_refs.is_empty(),
            "{} lacks review/support refs",
            surface.surface_id
        );
    }
}

#[test]
fn fixture_manifest_is_present_and_negative_drills_fire_expected_checks() {
    let fixture_path = repo_root()
        .join("fixtures/release/m4/notebook-and-data-rich-surface-qualification/cases.json");
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
                    NotebookDataQualificationViolation::StableSurfaceWithoutGreenPacket {
                        surface_id,
                    },
                );
            }
            "missing_visible_degrade_state" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.output_trust.requires_visible_state())
                        .expect("degraded surface exists");
                    surface.visible_state_labels.clear();
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    NotebookDataQualificationViolation::MissingVisibleDowngradeState { surface_id },
                );
            }
            "database_depth_overclaim" => {
                let borrowed_packet = packet.surfaces[0].qualification_packet.clone();
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.surface_kind == NotebookDataSurfaceKind::ResultGrid)
                        .expect("result-grid surface exists");
                    surface.claim_label = StableClaimLevel::Stable;
                    surface.displayed_label = StableClaimLevel::Stable;
                    surface.qualification_packet = borrowed_packet;
                    surface.surface_id.clone()
                };
                packet.summary = packet.computed_summary();
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    NotebookDataQualificationViolation::DatabaseDepthOverclaim { surface_id },
                );
            }
            "incomplete_projection" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.projection.support_export = false;
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    NotebookDataQualificationViolation::IncompleteProjection { surface_id },
                );
            }
            other => panic!("unknown fixture case: {other}"),
        }
    }
}

fn assert_expected(
    expected_check_id: &str,
    violations: Vec<NotebookDataQualificationViolation>,
    expected: NotebookDataQualificationViolation,
) {
    assert!(
        violations.contains(&expected),
        "expected {expected_check_id} to fire {expected:?}, got {violations:#?}"
    );
}
