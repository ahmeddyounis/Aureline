use std::fs;

use super::{
    audit_structured_config_artifact_modes_and_layers,
    parse_structured_config_artifact_modes_and_layers,
    seeded_structured_config_artifact_modes_and_layers, ArtifactFamilyKind, ConsumerSurfaceClass,
    HeaderFieldClass, LayerVocabularyField, ModeClass, WriteEligibilityClass,
    STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_PATH,
};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/config/structured_config_artifact_modes_and_layers/canonical.json",
);

#[test]
fn seeded_packet_passes_validation() {
    let packet = seeded_structured_config_artifact_modes_and_layers();
    let defects = audit_structured_config_artifact_modes_and_layers(&packet);
    assert!(defects.is_empty(), "validation defects: {defects:?}");
}

#[test]
fn checked_in_artifact_matches_seeded_packet() {
    let path = format!(
        "{}/../../{}",
        env!("CARGO_MANIFEST_DIR"),
        STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_PATH
    );
    let body = fs::read_to_string(path).expect("artifact exists");
    let artifact =
        parse_structured_config_artifact_modes_and_layers(&body).expect("artifact parses");
    assert_eq!(
        artifact,
        seeded_structured_config_artifact_modes_and_layers()
    );
}

#[test]
fn checked_in_fixture_matches_seeded_packet() {
    let body = fs::read_to_string(FIXTURE_PATH).expect("fixture exists");
    let fixture = parse_structured_config_artifact_modes_and_layers(&body).expect("fixture parses");
    assert_eq!(
        fixture,
        seeded_structured_config_artifact_modes_and_layers()
    );
}

#[test]
fn required_families_modes_and_surfaces_are_present() {
    let packet = seeded_structured_config_artifact_modes_and_layers();

    let families: Vec<_> = packet
        .artifact_surfaces
        .iter()
        .map(|row| row.family)
        .collect();
    assert_eq!(families.len(), ArtifactFamilyKind::ALL.len());
    for family in ArtifactFamilyKind::ALL {
        assert!(families.contains(&family), "missing family {family:?}");
    }

    let surfaces: Vec<_> = packet
        .surface_vocabulary
        .iter()
        .map(|row| row.surface)
        .collect();
    assert_eq!(surfaces.len(), ConsumerSurfaceClass::ALL.len());
    for surface in ConsumerSurfaceClass::ALL {
        assert!(surfaces.contains(&surface), "missing surface {surface:?}");
    }

    for row in &packet.artifact_surfaces {
        let modes: Vec<_> = row.mode_switches.iter().map(|mode| mode.mode).collect();
        assert_eq!(modes.len(), ModeClass::ALL.len(), "{:?}", row.family);
        for mode in ModeClass::ALL {
            assert!(modes.contains(&mode), "{:?} missing {mode:?}", row.family);
        }
    }
}

#[test]
fn headers_and_surface_vocabularies_stay_complete() {
    let packet = seeded_structured_config_artifact_modes_and_layers();

    for row in &packet.artifact_surfaces {
        assert!(
            !row.header.identity_label.is_empty()
                && !row.header.identity_ref.is_empty()
                && !row.header.artifact_class_label.is_empty()
                && !row.header.canonical_source_note.is_empty()
                && !row.header.target_context_label.is_empty()
                && !row.header.validator_summary.is_empty()
        );
    }

    for binding in &packet.surface_vocabulary {
        for field in HeaderFieldClass::ALL {
            assert!(
                binding.header_fields.contains(&field),
                "{:?} missing {field:?}",
                binding.surface
            );
        }
        for field in LayerVocabularyField::ALL {
            assert!(
                binding.layer_fields.contains(&field),
                "{:?} missing {field:?}",
                binding.surface
            );
        }
        for mode in ModeClass::ALL {
            assert!(
                binding.mode_labels.contains(&mode),
                "{:?} missing {mode:?}",
                binding.surface
            );
        }
    }
}

#[test]
fn effective_and_live_modes_never_claim_canonical_writes() {
    let packet = seeded_structured_config_artifact_modes_and_layers();
    for row in &packet.artifact_surfaces {
        for mode_row in &row.mode_switches {
            if mode_row.mode != ModeClass::Source {
                assert_ne!(
                    mode_row.write_eligibility,
                    WriteEligibilityClass::WritableCanonicalSource,
                    "{:?} {:?} must not be writable canonical source",
                    row.family,
                    mode_row.mode
                );
            }
        }
    }
}

#[test]
fn environment_bearing_rows_expose_in_ide_layer_actions() {
    let packet = seeded_structured_config_artifact_modes_and_layers();
    for row in &packet.artifact_surfaces {
        if row.environment_stack_required {
            let stack = row
                .environment_layer_stack
                .as_ref()
                .expect("required stack present");
            assert!(stack.visible_without_leaving_ide);
            assert!(
                stack.layers.iter().any(|layer| layer.wins_effective_value),
                "{:?} must keep a winning layer visible",
                row.family
            );
            assert!(
                stack
                    .layers
                    .iter()
                    .any(|layer| layer.reset_action.available),
                "{:?} must expose a reset action",
                row.family
            );
            assert!(
                stack
                    .layers
                    .iter()
                    .any(|layer| layer.open_source_action.available),
                "{:?} must expose an open-source action",
                row.family
            );
        }
    }
}
