use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_content_safety::{
    current_content_integrity_beta_packet, ContentIntegrityBetaCase, ContentIntegrityBetaPacket,
    CONTENT_INTEGRITY_BETA_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    #[serde(flatten)]
    case: ContentIntegrityBetaCase,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Expected {
    validation_status: String,
    surfaces: Vec<String>,
    warning_classes: Vec<String>,
    representations: Vec<String>,
}

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/content_safety/m3/shared_detector")
}

#[test]
fn shared_detector_beta_fixtures_validate_green() {
    let mut fixture_paths = fs::read_dir(fixture_dir())
        .expect("fixture dir")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect::<Vec<_>>();
    fixture_paths.sort();
    assert!(
        !fixture_paths.is_empty(),
        "expected shared detector beta fixtures"
    );

    for fixture_path in fixture_paths {
        let raw = fs::read_to_string(&fixture_path)
            .unwrap_or_else(|err| panic!("fixture {fixture_path:?} must read: {err}"));
        let fixture: Fixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {fixture_path:?} must parse: {err}"));
        let packet = ContentIntegrityBetaPacket::from_case(fixture.case);
        let report = packet.validate();

        assert_eq!(
            packet.record_kind,
            CONTENT_INTEGRITY_BETA_PACKET_RECORD_KIND
        );
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
            report.observed_warning_class_tokens,
            fixture.expected.warning_classes
        );
        assert_eq!(
            report.observed_representation_tokens,
            fixture.expected.representations
        );

        for surface in &packet.surfaces {
            assert_eq!(surface.warning_class_tokens, packet.finding_class_tokens);
            assert!(surface.operator_truth.is_green());
            assert!(surface.offers_required_representations());
            assert!(surface
                .content_integrity_warnings
                .iter()
                .all(|warning| !warning.continuity_ref.is_empty()));
            assert!(surface.representation_choices.iter().all(|choice| {
                surface.content_integrity_warnings.iter().all(|warning| {
                    choice
                        .attached_warning_refs
                        .iter()
                        .any(|reference| reference == &warning.continuity_ref)
                })
            }));
        }
    }
}

#[test]
fn checked_in_content_integrity_beta_packet_is_green() {
    let packet = current_content_integrity_beta_packet().expect("checked-in packet validates");
    assert!(packet.beta_gate_is_green());
}
