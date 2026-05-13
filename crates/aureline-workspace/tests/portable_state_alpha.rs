//! Protected fixtures for workspace portable-state alpha packages.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_workspace::{
    ExportMode, MachineLocalExclusionReason, PersistenceClassification, PortableStateAlphaPackage,
    PortableStateAlphaValidationError, RedactionRuleClass, SerializedStateClass,
    SurfaceRestorePosture,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(path: &str) -> String {
    let path = repo_root().join(path);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn fixture_package() -> PortableStateAlphaPackage {
    let payload =
        read_json("fixtures/workspace/portable_state_alpha/core_round_trip_changed_display.json");
    serde_json::from_str(&payload).expect("portable-state fixture must parse")
}

#[test]
fn fixture_round_trips_core_classes_placeholders_and_exclusions() {
    let package = fixture_package();
    package
        .validate()
        .expect("portable-state fixture must validate");

    let class_kinds: BTreeSet<_> = package
        .state_classes
        .iter()
        .map(|row| row.class_kind)
        .collect();
    for required in [
        SerializedStateClass::WorkspaceAuthority,
        SerializedStateClass::WindowTopology,
        SerializedStateClass::ProfileDefaults,
        SerializedStateClass::MachineLocalHints,
    ] {
        assert!(class_kinds.contains(&required), "missing {required:?}");
    }

    let classifications: BTreeSet<_> = package
        .state_classes
        .iter()
        .map(|row| row.classification)
        .collect();
    for required in [
        PersistenceClassification::LocalOnly,
        PersistenceClassification::Portable,
        PersistenceClassification::Shared,
        PersistenceClassification::MachineLocal,
    ] {
        assert!(classifications.contains(&required), "missing {required:?}");
    }

    let topology = package
        .state_classes
        .iter()
        .find(|row| row.class_kind == SerializedStateClass::WindowTopology)
        .expect("fixture must include topology");
    let postures: BTreeSet<_> = topology
        .pane_restore_postures
        .iter()
        .map(|pane| pane.restore_posture)
        .collect();
    assert!(postures.contains(&SurfaceRestorePosture::Live));
    assert!(postures.contains(&SurfaceRestorePosture::ContextOnly));
    assert!(postures.contains(&SurfaceRestorePosture::PlaceholderOnly));
    assert_eq!(
        topology.schema_binding.schema_ref,
        "schemas/workspace/pane_tree.schema.json"
    );

    let exclusion_reasons: BTreeSet<_> = package
        .machine_local_exclusions
        .iter()
        .map(|row| row.reason)
        .collect();
    assert!(exclusion_reasons.contains(&MachineLocalExclusionReason::ContainsLiveHandle));
    assert!(exclusion_reasons.contains(&MachineLocalExclusionReason::StateRootOnly));
    assert!(exclusion_reasons.contains(&MachineLocalExclusionReason::CredentialStoreOnly));
    assert!(exclusion_reasons.contains(&MachineLocalExclusionReason::DisplayHintBestEffortOnly));

    let json = serde_json::to_string(&package).expect("fixture must serialize");
    let reparsed: PortableStateAlphaPackage =
        serde_json::from_str(&json).expect("serialized package must parse");
    assert_eq!(package, reparsed);
}

#[test]
fn inspector_projection_names_classification_schema_last_write_and_actions() {
    let package = fixture_package();
    let inspector = package.inspector().expect("inspector must build");
    let rendered = inspector.render_plaintext();

    assert!(rendered.contains("schema_version=1"));
    assert!(rendered.contains("classification=portable"));
    assert!(rendered.contains("classification=local_only"));
    assert!(rendered.contains("classification=shared"));
    assert!(rendered.contains("classification=machine_local"));
    assert!(rendered.contains("last_written_at=mono:"));
    assert!(rendered.contains("actions=inspect,export,clear,compare"));
    assert!(rendered.contains("pane=pane-terminal-0001 posture=context_only"));
    assert!(rendered.contains("pane=pane-preview-ext-0001 posture=placeholder_only"));
}

#[test]
fn profile_defaults_remain_explicit_linked_artifact_refs() {
    let package = fixture_package();
    let profile_defaults = package
        .state_classes
        .iter()
        .find(|row| row.class_kind == SerializedStateClass::ProfileDefaults)
        .expect("profile defaults class must exist");

    assert_eq!(profile_defaults.export_mode, ExportMode::LinkedArtifactRef);
    assert!(!profile_defaults.linked_profile_artifact_refs.is_empty());
    for artifact_ref in &profile_defaults.linked_profile_artifact_refs {
        assert!(
            package
                .linked_profile_artifacts
                .iter()
                .any(|artifact| artifact.artifact_ref == *artifact_ref),
            "missing linked artifact {artifact_ref}"
        );
    }
}

#[test]
fn validator_rejects_missing_redaction_live_authority_rule() {
    let mut package = fixture_package();
    package
        .redaction_manifest
        .rules
        .retain(|rule| *rule != RedactionRuleClass::LiveAuthorityHandleExcluded);

    let err = package
        .validate()
        .expect_err("live authority redaction rule is required");
    assert!(matches!(
        err,
        PortableStateAlphaValidationError::MissingRedactionRule {
            rule: RedactionRuleClass::LiveAuthorityHandleExcluded
        }
    ));
}

#[test]
fn validator_rejects_machine_local_hints_exported_as_body() {
    let mut package = fixture_package();
    let machine_hints = package
        .state_classes
        .iter_mut()
        .find(|row| row.class_kind == SerializedStateClass::MachineLocalHints)
        .expect("machine hints class must exist");
    machine_hints.export_allowed = true;
    machine_hints.export_mode = ExportMode::CarriedBody;

    let err = package
        .validate()
        .expect_err("machine-local hints must not export as body");
    assert!(matches!(
        err,
        PortableStateAlphaValidationError::MachineLocalClassExported { .. }
    ));
}

#[test]
fn validator_rejects_display_topology_adjustment_without_visible_bounds() {
    let mut package = fixture_package();
    package.restore_provenance.topology_adjustments[0].visible_bounds_verified = false;

    let err = package
        .validate()
        .expect_err("changed display topology must verify visible bounds");
    assert!(matches!(
        err,
        PortableStateAlphaValidationError::DisplayTopologyAdjustmentUnverified { .. }
    ));
}
