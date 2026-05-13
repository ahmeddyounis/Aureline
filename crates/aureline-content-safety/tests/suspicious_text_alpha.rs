use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use aureline_content_safety::{
    project_suspicious_text_core_surfaces, SuspiciousTextProjectionSeed,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    case_id: String,
    language_id: String,
    content: String,
    surface_refs: SurfaceRefs,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct SurfaceRefs {
    editor: String,
    diff: String,
    search: String,
    review: String,
}

#[derive(Debug, Deserialize)]
struct Expected {
    surfaces: Vec<String>,
    warning_classes: Vec<String>,
    safe_actions: Vec<String>,
    warning_continuity_required: bool,
    normalization_applied: bool,
}

#[test]
fn suspicious_text_alpha_fixtures_preserve_cross_surface_warnings() {
    let fixture_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/content_safety/suspicious_text_alpha");
    let mut fixture_paths = fs::read_dir(&fixture_dir)
        .expect("fixture dir")
        .map(|entry| entry.expect("fixture entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect::<Vec<_>>();
    fixture_paths.sort();
    assert!(
        !fixture_paths.is_empty(),
        "expected suspicious-text fixtures"
    );

    for fixture_path in fixture_paths {
        let raw = fs::read_to_string(&fixture_path).expect("read fixture");
        let fixture: Fixture = serde_json::from_str(&raw).expect("parse fixture");
        let seed = SuspiciousTextProjectionSeed {
            case_id: &fixture.case_id,
            content: &fixture.content,
            editor_subject_ref: &fixture.surface_refs.editor,
            diff_hunk_ref: &fixture.surface_refs.diff,
            search_row_ref: &fixture.surface_refs.search,
            review_anchor_ref: &fixture.surface_refs.review,
        };

        let packet = project_suspicious_text_core_surfaces(&seed);
        assert_eq!(
            packet.normalization_applied, fixture.expected.normalization_applied,
            "normalization posture mismatch for {}",
            fixture.language_id
        );
        assert!(packet.covers_core_source_surfaces());
        assert!(packet.all_surfaces_share_warning_classes());
        assert!(packet.all_warnings_offer_raw_and_escaped_reveal());
        assert!(packet.all_surfaces_offer_safe_representation_path());
        assert!(packet.all_transfers_preserve_warning_refs());

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
        assert_eq!(
            actual_surfaces, expected_surfaces,
            "surface coverage mismatch for {}",
            fixture.language_id
        );

        assert_eq!(
            packet.finding_classes, fixture.expected.warning_classes,
            "warning class mismatch for {}",
            fixture.language_id
        );

        for surface in &packet.surfaces {
            let action_ids = surface
                .copy_choices
                .iter()
                .map(|choice| choice.action_id.as_str())
                .chain(
                    surface
                        .safe_export
                        .iter()
                        .map(|export| export.action_id.as_str()),
                )
                .collect::<BTreeSet<_>>();
            let expected_actions = fixture
                .expected
                .safe_actions
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>();
            assert_eq!(
                action_ids,
                expected_actions,
                "safe action mismatch for {} on {}",
                fixture.language_id,
                surface.surface.as_str()
            );

            if fixture.expected.warning_continuity_required {
                let continuity_refs = surface
                    .warnings
                    .iter()
                    .map(|warning| warning.anchor.continuity_ref.as_str())
                    .collect::<BTreeSet<_>>();
                assert_eq!(
                    continuity_refs.len(),
                    fixture.expected.warning_classes.len(),
                    "continuity refs mismatch for {} on {}",
                    fixture.language_id,
                    surface.surface.as_str()
                );
            }
        }
    }
}
