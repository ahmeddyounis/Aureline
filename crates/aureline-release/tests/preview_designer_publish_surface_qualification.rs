//! Protected tests for the preview/designer/publish surface qualification packet.

use std::path::{Path, PathBuf};

use aureline_release::preview_designer_publish_surface_qualification::{
    current_preview_designer_publish_surface_qualification, ActionSafetyLineage,
    PreviewDesignerPublishQualificationViolation, PreviewDesignerPublishSurfaceKind,
    PreviewDesignerPublishSurfaceQualification, SafePreviewPosture, SourceMappingQuality,
    SourceSyncState, PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_RECORD_KIND,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};
use aureline_release::stable_claim_matrix::StableClaimLevel;
use serde::Deserialize;

fn packet() -> PreviewDesignerPublishSurfaceQualification {
    current_preview_designer_publish_surface_qualification()
        .expect("checked-in preview/designer/publish packet parses")
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
        PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_RECORD_KIND
    );
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in preview/designer/publish packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_matches_rows_and_narrowed_rows_do_not_widen() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.stable_surfaces().len(), 1);
    assert_eq!(packet.narrowed_surfaces().len(), 4);

    for surface in &packet.surfaces {
        assert!(
            surface.displayed_label.rank() <= surface.claim_label.rank(),
            "{} displays wider than its claim",
            surface.surface_id
        );
        if matches!(
            surface.surface_kind,
            PreviewDesignerPublishSurfaceKind::DesignerCanvas
                | PreviewDesignerPublishSurfaceKind::ShareExportSheet
                | PreviewDesignerPublishSurfaceKind::PublishDeployPreview
        ) {
            assert_ne!(
                surface.displayed_label,
                StableClaimLevel::Stable,
                "{} must not imply stable designer/export/publish depth",
                surface.surface_id
            );
        }
    }
}

#[test]
fn stable_rows_keep_canonical_source_safe_preview_and_browser_boundary() {
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
        assert_eq!(
            surface.source_mapping_quality,
            SourceMappingQuality::CanonicalSourceMapping
        );
        assert_eq!(surface.source_sync_state, SourceSyncState::InSync);
        assert!(
            !surface.visible_lineage_labels.is_empty(),
            "{} lacks generated/source truth labels",
            surface.surface_id
        );
        assert!(
            surface.fallback_paths.open_source
                && surface.fallback_paths.open_diff
                && surface.fallback_paths.rollback_lineage_export,
            "{} lacks source/diff/rollback fallback",
            surface.surface_id
        );
    }
}

#[test]
fn support_export_projection_preserves_row_truth() {
    let packet = packet();
    let projection = packet.support_export_projection();
    assert_eq!(
        projection.record_kind,
        "preview_designer_publish_surface_qualification_support_export"
    );
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.rows.len(), packet.surfaces.len());
    assert!(projection.rows.iter().any(|row| {
        row.surface_id == "preview_designer_publish:share_export_preview_sheet"
            && row.displayed_label == StableClaimLevel::Beta
            && row.rollback_lineage_exportable
    }));
}

#[test]
fn fixture_manifest_is_present_and_negative_drills_fire_expected_checks() {
    let fixture_path = repo_root()
        .join("fixtures/release/m4/preview-designer-publish-surface-qualification/cases.json");
    let payload = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", fixture_path.display()));
    let manifest: FixtureManifest =
        serde_json::from_str(&payload).expect("fixture manifest parses");
    assert_eq!(manifest.schema_version, 1);
    assert!(!manifest.cases.is_empty());

    for case in manifest.cases {
        let mut packet = packet();
        match case.case_id.as_str() {
            "stable_surface_without_canonical_mapping" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.source_mapping_quality = SourceMappingQuality::ApproximateMapping;
                    surface
                        .unsupported_construct_cards
                        .push("Approximate mapping must narrow.".to_string());
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    PreviewDesignerPublishQualificationViolation::StableSurfaceWithoutCanonicalMapping {
                        surface_id,
                    },
                );
            }
            "publish_without_dry_run" => {
                let borrowed_packet = packet.surfaces[0].qualification_packet.clone();
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| {
                            surface.surface_kind
                                == PreviewDesignerPublishSurfaceKind::PublishDeployPreview
                        })
                        .expect("publish surface exists");
                    surface.displayed_label = StableClaimLevel::Stable;
                    surface.qualification_packet = borrowed_packet;
                    surface.owner_signoff.signed_off = true;
                    surface.owner_signoff.signed_at = Some("2026-06-04".to_string());
                    surface.source_mapping_quality = SourceMappingQuality::CanonicalSourceMapping;
                    surface.source_sync_state = SourceSyncState::InSync;
                    surface
                        .action_safety_lineage
                        .retain(|lineage| !matches!(lineage, ActionSafetyLineage::DryRun));
                    surface.safe_preview_posture = SafePreviewPosture::ReviewRequiredPreview;
                    surface.surface_id.clone()
                };
                packet.summary = packet.computed_summary();
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    PreviewDesignerPublishQualificationViolation::SideEffectWithoutDryRun {
                        surface_id,
                    },
                );
            }
            "missing_generated_source_truth" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| {
                            surface.surface_id == "preview_designer_publish:device_viewport_preview"
                        })
                        .expect("device preview surface exists");
                    surface.visible_lineage_labels.clear();
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    PreviewDesignerPublishQualificationViolation::MissingGeneratedSourceTruth {
                        surface_id,
                    },
                );
            }
            "missing_preview_apply_revert_lineage" => {
                let borrowed_packet = packet.surfaces[0].qualification_packet.clone();
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| {
                            surface.surface_kind
                                == PreviewDesignerPublishSurfaceKind::ShareExportSheet
                        })
                        .expect("share/export surface exists");
                    surface.displayed_label = StableClaimLevel::Stable;
                    surface.qualification_packet = borrowed_packet;
                    surface.owner_signoff.signed_off = true;
                    surface.owner_signoff.signed_at = Some("2026-06-04".to_string());
                    surface.source_mapping_quality = SourceMappingQuality::CanonicalSourceMapping;
                    surface.source_sync_state = SourceSyncState::InSync;
                    surface.action_safety_lineage.retain(|lineage| {
                        !matches!(lineage, ActionSafetyLineage::RevertRollbackExportable)
                    });
                    surface.surface_id.clone()
                };
                packet.summary = packet.computed_summary();
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    PreviewDesignerPublishQualificationViolation::MissingPreviewApplyRevertLineage {
                        surface_id,
                    },
                );
            }
            other => panic!("unknown fixture case: {other}"),
        }
    }
}

fn assert_expected(
    expected_check_id: &str,
    violations: Vec<PreviewDesignerPublishQualificationViolation>,
    expected: PreviewDesignerPublishQualificationViolation,
) {
    assert!(
        violations.contains(&expected),
        "expected {expected_check_id} to fire {expected:?}, got {violations:#?}"
    );
}
