//! Fixture replay and invariant tests for the shared config artifact header,
//! mode-switch, and layer-stack contract.

use std::collections::BTreeSet;

use aureline_config::structured_config_artifact_modes_and_layers::{
    seeded_structured_config_artifact_modes_and_layers, ConsumerSurfaceClass, HeaderFieldClass,
    LayerVocabularyField, ModeClass, StructuredConfigArtifactModesAndLayersPacket,
    STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_RECORD_KIND,
    STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF,
};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/config/structured_config_artifact_modes_and_layers/canonical.json",
);

fn load_packet() -> StructuredConfigArtifactModesAndLayersPacket {
    let body = std::fs::read_to_string(FIXTURE_PATH)
        .unwrap_or_else(|err| panic!("failed to read {FIXTURE_PATH}: {err}"));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {FIXTURE_PATH}: {err}"))
}

#[test]
fn fixture_matches_in_code_projection() {
    assert_eq!(
        load_packet(),
        seeded_structured_config_artifact_modes_and_layers(),
        "fixture drifted; re-emit with `cargo run -q -p aureline-config --bin aureline_config_structured_artifact_modes_and_layers -- json > fixtures/config/structured_config_artifact_modes_and_layers/canonical.json`",
    );
}

#[test]
fn packet_identity_and_surface_contract_refs_are_stable() {
    let packet = load_packet();
    assert_eq!(
        packet.record_kind,
        STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_RECORD_KIND
    );
    assert_eq!(
        packet.shared_contract_ref,
        STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF
    );
    assert_eq!(
        packet.summary.artifact_surface_count,
        packet.artifact_surfaces.len()
    );
}

#[test]
fn every_row_exposes_source_effective_and_live_switches() {
    let packet = load_packet();
    for row in &packet.artifact_surfaces {
        let modes: BTreeSet<ModeClass> = row.mode_switches.iter().map(|mode| mode.mode).collect();
        for required in ModeClass::ALL {
            assert!(
                modes.contains(&required),
                "{:?} missing mode {:?}",
                row.family,
                required
            );
        }
        let active_count = row.mode_switches.iter().filter(|mode| mode.active).count();
        assert_eq!(
            active_count, 1,
            "{:?} must have one active mode",
            row.family
        );
        assert!(
            row.mode_switches
                .iter()
                .any(|mode| mode.active && mode.mode == row.header.active_mode),
            "{:?} header active mode must match the active switch row",
            row.family
        );
    }
}

#[test]
fn env_bearing_rows_keep_layer_vocabulary_visible() {
    let packet = load_packet();
    for row in &packet.artifact_surfaces {
        if row.environment_stack_required {
            let stack = row
                .environment_layer_stack
                .as_ref()
                .expect("required stack present");
            assert!(stack.visible_without_leaving_ide);
            assert!(stack
                .layers
                .iter()
                .all(|layer| !layer.layer_label.is_empty()
                    && !layer.secret_bearing_note.is_empty()
                    && !layer.reset_action.action_label.is_empty()
                    && !layer.open_source_action.action_label.is_empty()));
        }
    }
}

#[test]
fn shared_surfaces_reuse_all_header_mode_and_layer_terms() {
    let packet = load_packet();
    let surfaces: BTreeSet<ConsumerSurfaceClass> = packet
        .surface_vocabulary
        .iter()
        .map(|row| row.surface)
        .collect();
    for required in ConsumerSurfaceClass::ALL {
        assert!(surfaces.contains(&required), "missing surface {required:?}");
    }
    for binding in &packet.surface_vocabulary {
        let header_fields: BTreeSet<HeaderFieldClass> =
            binding.header_fields.iter().copied().collect();
        for required in HeaderFieldClass::ALL {
            assert!(
                header_fields.contains(&required),
                "{:?} missing header field {:?}",
                binding.surface,
                required
            );
        }
        let layer_fields: BTreeSet<LayerVocabularyField> =
            binding.layer_fields.iter().copied().collect();
        for required in LayerVocabularyField::ALL {
            assert!(
                layer_fields.contains(&required),
                "{:?} missing layer field {:?}",
                binding.surface,
                required
            );
        }
    }
}
