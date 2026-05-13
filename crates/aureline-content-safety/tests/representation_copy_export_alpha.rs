use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_content_safety::{
    RepresentationCopyExportAlphaPacket, RepresentationCopyExportCase,
    INTERACTION_SAFETY_COPY_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    #[serde(flatten)]
    case: RepresentationCopyExportCase,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Expected {
    surfaces: Vec<String>,
    validation_status: String,
    default_actions_are_raw_or_plain_safe: bool,
    sensitive_actions_require_preview: bool,
    interaction_safety_records_present: bool,
    reconciled_group_refs: Vec<String>,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/content_safety/representation_copy_export_alpha")
}

#[test]
fn protected_copy_export_fixture_validates_cross_surface_labels_and_recovery() {
    let mut fixture_paths = fs::read_dir(fixtures_dir())
        .expect("fixture dir")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect::<Vec<_>>();
    fixture_paths.sort();
    assert!(
        !fixture_paths.is_empty(),
        "expected representation copy/export fixtures"
    );

    for fixture_path in fixture_paths {
        let raw = fs::read_to_string(&fixture_path)
            .unwrap_or_else(|err| panic!("fixture {fixture_path:?} must read: {err}"));
        let fixture: Fixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {fixture_path:?} must parse: {err}"));
        let packet = RepresentationCopyExportAlphaPacket::from_case(fixture.case);
        let report = packet.validate();

        assert_eq!(
            report.status, fixture.expected.validation_status,
            "{fixture_path:?}: validation status mismatch: {:?}",
            report.violations
        );
        assert!(report.passed(), "{fixture_path:?}: {:?}", report.violations);
        assert!(packet.covers_protected_surfaces());

        let expected_surfaces = fixture
            .expected
            .surfaces
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let actual_surfaces = packet
            .surfaces
            .iter()
            .map(|surface| surface.surface.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(actual_surfaces, expected_surfaces);

        assert_eq!(
            all_default_actions_are_raw_or_plain_safe(&packet),
            fixture.expected.default_actions_are_raw_or_plain_safe
        );
        assert_eq!(
            all_sensitive_actions_require_preview(&packet),
            fixture.expected.sensitive_actions_require_preview
        );
        assert_eq!(
            all_actions_have_interaction_safety_records(&packet),
            fixture.expected.interaction_safety_records_present
        );

        let expected_groups = fixture
            .expected
            .reconciled_group_refs
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let actual_groups = report
            .reconciled_groups
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        assert_eq!(actual_groups, expected_groups);
    }
}

fn all_default_actions_are_raw_or_plain_safe(packet: &RepresentationCopyExportAlphaPacket) -> bool {
    packet.surfaces.iter().all(|surface| {
        surface
            .actions
            .iter()
            .filter(|action| action.default_for_surface)
            .count()
            == 1
            && surface
                .actions
                .iter()
                .filter(|action| action.default_for_surface)
                .all(|action| {
                    matches!(action.representation_class.as_str(), "raw" | "escaped")
                        && matches!(action.payload_mode.as_str(), "plain_text" | "raw")
                        && !action.includes_context
                })
    })
}

fn all_sensitive_actions_require_preview(packet: &RepresentationCopyExportAlphaPacket) -> bool {
    packet.surfaces.iter().all(|surface| {
        surface.actions.iter().all(|action| {
            !action.carries_sensitive_value || action.preview_required_before_clipboard
        })
    })
}

fn all_actions_have_interaction_safety_records(
    packet: &RepresentationCopyExportAlphaPacket,
) -> bool {
    packet.surfaces.iter().all(|surface| {
        surface.actions.iter().all(|action| {
            let record = &action.interaction_copy_export_record;
            record.record_kind == INTERACTION_SAFETY_COPY_EXPORT_RECORD_KIND
                && record.source_surface_class == surface.interaction_surface_class
                && record.source_target_identity_ref == surface.source_target_identity_ref
                && record.representation_class == action.representation_class
                && record.action_kind == action.action_kind
        })
    })
}
