//! Protected tests for the depth-surface schema registry packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::{
    seeded_depth_surface_schema_registry_packet, ConsentStateClass, DepthSignalClass,
    DepthSurfaceClass, DepthSurfaceSchemaRegistryPacket, EndpointStateClass,
    DEPTH_SCHEMA_REGISTRY_ARTIFACT_REF, DEPTH_SCHEMA_REGISTRY_DOC_REF,
    DEPTH_SCHEMA_REGISTRY_FIXTURE_DIR, DEPTH_SCHEMA_REGISTRY_PACKET_RECORD_KIND,
    DEPTH_SCHEMA_REGISTRY_SCHEMA_REF, DEPTH_SCHEMA_REGISTRY_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(DEPTH_SCHEMA_REGISTRY_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> DepthSurfaceSchemaRegistryPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_validates_and_covers_all_required_surfaces_and_signals() {
    let packet = seeded_depth_surface_schema_registry_packet();
    assert_eq!(packet.record_kind, DEPTH_SCHEMA_REGISTRY_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, DEPTH_SCHEMA_REGISTRY_SCHEMA_VERSION);
    assert_eq!(packet.doc_ref, DEPTH_SCHEMA_REGISTRY_DOC_REF);
    assert_eq!(packet.schema_ref, DEPTH_SCHEMA_REGISTRY_SCHEMA_REF);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    for surface in DepthSurfaceClass::ALL {
        for signal in DepthSignalClass::ALL {
            assert!(
                packet
                    .schema_declarations
                    .iter()
                    .any(|row| row.surface == surface && row.signal_class == signal),
                "missing declaration for {} / {}",
                surface.as_str(),
                signal.as_str()
            );
        }
    }
}

#[test]
fn support_export_rows_remain_explicit_export_only() {
    let packet = seeded_depth_surface_schema_registry_packet();
    for row in packet
        .schema_declarations
        .iter()
        .filter(|row| row.signal_class == DepthSignalClass::SupportExport)
    {
        assert_eq!(
            row.active_consent_state,
            ConsentStateClass::ExplicitUserRequestRequired
        );
        assert_eq!(
            row.active_endpoint_state,
            EndpointStateClass::ManualExportOnly
        );
    }

    let support_binding = packet
        .consent_ledger_bindings
        .iter()
        .find(|row| row.signal_class == DepthSignalClass::SupportExport)
        .expect("support export binding");
    assert!(support_binding.explicit_export_not_ambient_telemetry);
}

#[test]
fn ordinary_rows_forbid_guardrail_content_classes() {
    let packet = seeded_depth_surface_schema_registry_packet();
    for row in packet
        .schema_declarations
        .iter()
        .filter(|row| row.signal_class != DepthSignalClass::SupportExport)
    {
        for required in [
            "source_code_bodies",
            "filenames_and_paths",
            "prompt_bodies",
            "terminal_contents",
            "secret_material",
            "clipboard_contents",
        ] {
            assert!(
                row.prohibited_content_classes
                    .iter()
                    .any(|value| value == required),
                "{} missing guardrail {}",
                row.schema_id,
                required,
            );
        }
    }
}

#[test]
fn consent_bindings_preserve_local_first_and_non_broadening_posture() {
    let packet = seeded_depth_surface_schema_registry_packet();
    for row in &packet.consent_ledger_bindings {
        assert!(row.open_source_local_default);
        assert!(row.managed_builds_may_narrow_but_not_broaden);
    }
}

#[test]
fn packet_classes_are_redaction_default_for_all_depth_surfaces() {
    let packet = seeded_depth_surface_schema_registry_packet();
    for row in &packet.packet_class_manifest {
        assert!(row.raw_source_code_forbidden);
        assert!(row.filenames_and_paths_forbidden);
        assert!(row.prompt_bodies_forbidden);
        assert!(row.terminal_contents_forbidden);
        assert!(row.secrets_forbidden);
        assert!(row.clipboard_contents_forbidden);
        assert_eq!(row.surfaces.len(), DepthSurfaceClass::ALL.len());
    }
}

#[test]
fn packet_round_trips_and_is_export_safe() {
    let packet = seeded_depth_surface_schema_registry_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let round: DepthSurfaceSchemaRegistryPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(round, packet);
    assert!(packet.is_export_safe());
    assert!(!json.contains("/Users/"));
    assert!(!json.contains("BEGIN PRIVATE KEY"));
}

#[test]
fn docs_schema_artifact_and_fixture_files_exist() {
    let root = repo_root();
    for rel in [
        DEPTH_SCHEMA_REGISTRY_SCHEMA_REF,
        DEPTH_SCHEMA_REGISTRY_DOC_REF,
        DEPTH_SCHEMA_REGISTRY_ARTIFACT_REF,
        "fixtures/support/m5/depth_surface_schema_registry/manifest.yaml",
        "fixtures/support/m5/depth_surface_schema_registry/README.md",
        "fixtures/support/m5/depth_surface_schema_registry/packet.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    let fixture = load_fixture("packet.json");
    let seeded = seeded_depth_surface_schema_registry_packet();
    assert_eq!(fixture, seeded);
}

#[test]
fn missing_schema_declaration_is_rejected() {
    let mut packet = seeded_depth_surface_schema_registry_packet();
    packet.schema_declarations.pop();
    let violations = packet.validate();
    assert!(
        violations
            .iter()
            .any(|row| row.message.contains("missing schema declaration")),
        "{violations:?}"
    );
}

#[test]
fn support_export_cannot_be_reclassified_as_ambient_telemetry() {
    let mut packet = seeded_depth_surface_schema_registry_packet();
    let binding = packet
        .consent_ledger_bindings
        .iter_mut()
        .find(|row| row.signal_class == DepthSignalClass::SupportExport)
        .expect("support export binding");
    binding.explicit_export_not_ambient_telemetry = false;
    let violations = packet.validate();
    assert!(
        violations
            .iter()
            .any(|row| row.message.contains("ambient telemetry")),
        "{violations:?}"
    );
}
