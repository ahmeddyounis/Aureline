use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_content_safety::{
    RepresentationLabelsBetaCase, RepresentationLabelsBetaPacket, REPRESENTATION_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    #[serde(flatten)]
    case: RepresentationLabelsBetaCase,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Expected {
    validation_status: String,
    surfaces: Vec<String>,
    content_classes: Vec<String>,
    representations: Vec<String>,
}

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/content/m3/representation_copy_export")
}

#[test]
fn representation_label_beta_fixture_validates_risky_surfaces() {
    let mut fixture_paths = fs::read_dir(fixture_dir())
        .expect("fixture dir")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect::<Vec<_>>();
    fixture_paths.sort();
    assert!(
        !fixture_paths.is_empty(),
        "expected representation-label beta fixtures"
    );

    for fixture_path in fixture_paths {
        let raw = fs::read_to_string(&fixture_path)
            .unwrap_or_else(|err| panic!("fixture {fixture_path:?} must read: {err}"));
        let fixture: Fixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {fixture_path:?} must parse: {err}"));
        let packet = RepresentationLabelsBetaPacket::from_case(fixture.case);
        let report = packet.validate();

        assert_eq!(
            report.status, fixture.expected.validation_status,
            "{fixture_path:?}: {:#?}",
            report.violations
        );
        assert!(
            report.is_green(),
            "{fixture_path:?}: {:#?}",
            report.violations
        );
        assert!(packet.beta_gate_is_green());

        assert_eq!(
            report
                .observed_surface_tokens
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            fixture
                .expected
                .surfaces
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            report
                .observed_content_classes
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            fixture
                .expected
                .content_classes
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            report.observed_representation_tokens,
            fixture.expected.representations
        );

        for surface in &packet.surfaces {
            assert!(surface.covers_required_actions());
            assert!(surface.actions.iter().all(|action| {
                let record = &action.representation_export_record;
                record.record_kind == REPRESENTATION_EXPORT_RECORD_KIND
                    && record.source_surface == surface.surface_token
                    && record.source_subject_ref == surface.subject_ref
                    && record.source_trust_class == surface.source_trust_class
                    && record.representation_class == action.representation_class
                    && record.action_kind == action.action_kind
                    && record.trust_class_badge_visible
                    && record.representation_label_visible
                    && record.source_trust_class_visible
            }));
        }
    }
}
