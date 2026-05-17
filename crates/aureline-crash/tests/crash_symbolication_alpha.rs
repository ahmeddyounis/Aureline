//! Protected alpha tests for the crash-envelope symbol-manifest binding.
//!
//! These tests exercise the boundary contract owned by
//! `aureline_crash::envelope`. They round-trip the release-side symbol
//! manifest, prove that the binding lane labels `linked`, `partial`,
//! `missing_manifest`, and `build_mismatch` honestly, and re-prove that
//! every emitted binding is metadata-safe.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_crash::{
    bind_crash_envelope, CrashEnvelope, CrashEnvelopeBindingInputs, ManifestArtifactFamilyClass,
    ManifestModuleKind, ManifestRedactionClass, ManifestStorageClass, ModuleBindingState,
    ReleaseChannelClass, SupportExportPostureClass, SymbolBindingState, SymbolManifest,
    CRASH_ENVELOPE_SYMBOL_BINDING_RECORD_KIND, SYMBOL_MANIFEST_DOC_REF,
    SYMBOL_MANIFEST_RECORD_KIND, SYMBOL_MANIFEST_SCHEMA_REF, SYMBOL_MANIFEST_SCHEMA_VERSION,
};

const GENERATED_AT: &str = "2026-05-16T00:30:00Z";
const SUPPORT_BUNDLE_MANIFEST_REF: &str =
    "support.bundle.manifest.alpha_preview.renderer_panic.local_review";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn load_json<T>(path: &Path) -> T
where
    T: serde::de::DeserializeOwned,
{
    let bytes =
        fs::read(path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn release_symbol_manifest_path() -> PathBuf {
    repo_root().join("artifacts/release/m3/symbol_manifest/symbol_manifest.json")
}

fn release_symbol_manifest() -> SymbolManifest {
    load_json(&release_symbol_manifest_path())
}

fn preview_alpha_symbol_manifest() -> SymbolManifest {
    load_json(
        &repo_root()
            .join("fixtures/support/crash_symbolication_alpha/symbol_manifest_preview_alpha.json"),
    )
}

fn linked_crash_envelope() -> CrashEnvelope {
    load_json(&repo_root().join("fixtures/support/incident_trail_alpha/crash_envelope.json"))
}

fn partial_crash_envelope() -> CrashEnvelope {
    load_json(
        &repo_root().join(
            "fixtures/support/crash_symbolication_alpha/crash_envelope_partial_renderer.json",
        ),
    )
}

fn partial_symbol_manifest() -> SymbolManifest {
    load_json(
        &repo_root()
            .join("fixtures/support/crash_symbolication_alpha/symbol_manifest_partial.json"),
    )
}

fn alpha_inputs<'a>(
    envelope: &'a CrashEnvelope,
    manifest: Option<&'a SymbolManifest>,
) -> CrashEnvelopeBindingInputs<'a> {
    CrashEnvelopeBindingInputs {
        binding_id: "crash-symbolication-binding:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        crash_envelope: envelope,
        symbol_manifest: manifest,
        symbolication_report: None,
        support_bundle_manifest_ref: Some(SUPPORT_BUNDLE_MANIFEST_REF.into()),
        support_export_posture: SupportExportPostureClass::MetadataOnlyDefault,
    }
}

#[test]
fn preview_symbol_manifest_matches_alpha_crash_envelope_identity() {
    let manifest = preview_alpha_symbol_manifest();
    let envelope = linked_crash_envelope();

    assert_eq!(manifest.record_kind, SYMBOL_MANIFEST_RECORD_KIND);
    assert_eq!(manifest.schema_version, SYMBOL_MANIFEST_SCHEMA_VERSION);
    assert_eq!(manifest.release_channel_class, ReleaseChannelClass::Preview);
    assert_eq!(
        manifest.redaction_class,
        ManifestRedactionClass::MetadataSafeDefault
    );
    assert!(manifest.raw_private_material_excluded);
    assert!(manifest.ambient_authority_excluded);
    assert_eq!(
        manifest.primary_exact_build_identity_ref,
        envelope.primary_exact_build_identity_ref
    );

    assert!(!manifest.modules.is_empty());
    for module in &manifest.modules {
        assert_eq!(
            module.storage_class,
            ManifestStorageClass::MetadataOnlyNoSymbolBytes
        );
        match module.module_kind {
            ManifestModuleKind::NativeBinary => {
                assert_eq!(
                    module.artifact_family_class,
                    ManifestArtifactFamilyClass::IdeBinary
                );
                assert!(module.build_id.is_some());
                assert!(module.debug_id.is_some());
                assert!(module.support_archive_identity_ref.is_some());
            }
            ManifestModuleKind::WebBundle => {
                assert_eq!(
                    module.artifact_family_class,
                    ManifestArtifactFamilyClass::SourceMapBundle
                );
                assert!(module.source_map_digest.is_some());
                assert!(module.bundle_revision_ref.is_some());
                assert!(module.generated_asset_ref.is_some());
            }
        }
    }

    let envelope_modules: std::collections::BTreeSet<&str> = envelope
        .modules
        .iter()
        .map(|m| m.module_id.as_str())
        .collect();
    let manifest_modules: std::collections::BTreeSet<&str> = manifest
        .modules
        .iter()
        .map(|m| m.module_id.as_str())
        .collect();
    assert_eq!(
        envelope_modules, manifest_modules,
        "preview alpha symbol manifest must cover every module declared by the alpha crash envelope"
    );
}

#[test]
fn release_symbol_manifest_matches_beta_artifact_graph_identity() {
    let manifest = release_symbol_manifest();
    let graph: serde_json::Value =
        load_json(&repo_root().join("artifacts/release/m3/artifact_graph.json"));
    let graph_exact_build_identity_ref = graph["exact_build_identities"][0]
        ["exact_build_identity_ref"]
        .as_str()
        .expect("graph exact-build identity");
    let graph_version = graph["candidate"]["version"]
        .as_str()
        .expect("graph candidate version");

    assert_eq!(manifest.release_channel_class, ReleaseChannelClass::Beta);
    assert_eq!(
        manifest.primary_exact_build_identity_ref,
        graph_exact_build_identity_ref
    );
    assert_eq!(manifest.workspace_version, graph_version);
    assert_eq!(
        graph["source_contract_refs"]["symbol_manifest"]
            .as_str()
            .expect("symbol manifest graph ref"),
        "artifacts/release/m3/symbol_manifest/symbol_manifest.json"
    );

    let module_ids: std::collections::BTreeSet<&str> = manifest
        .modules
        .iter()
        .map(|module| module.module_id.as_str())
        .collect();
    assert!(module_ids.contains("aureline-shell"));
    assert!(module_ids.contains("aureline-commands"));
    assert!(module_ids.contains("renderer.main.bundle.js"));

    for module in &manifest.modules {
        assert_eq!(
            module.storage_class,
            ManifestStorageClass::MetadataOnlyNoSymbolBytes
        );
        match module.module_kind {
            ManifestModuleKind::NativeBinary => {
                assert!(matches!(
                    module.artifact_family_class,
                    ManifestArtifactFamilyClass::IdeBinary | ManifestArtifactFamilyClass::CliBinary
                ));
                assert_eq!(
                    module.exact_build_identity_ref,
                    graph_exact_build_identity_ref
                );
                assert!(module.build_id.is_some());
                assert!(module.debug_id.is_some());
                assert!(module.support_archive_identity_ref.is_some());
            }
            ManifestModuleKind::WebBundle => {
                assert_eq!(
                    module.artifact_family_class,
                    ManifestArtifactFamilyClass::SourceMapBundle
                );
                assert!(module
                    .exact_build_identity_ref
                    .starts_with(graph_exact_build_identity_ref));
                assert!(module.source_map_digest.is_some());
                assert!(module.bundle_revision_ref.is_some());
                assert!(module.generated_asset_ref.is_some());
            }
        }
    }
}

#[test]
fn linked_manifest_binds_crash_envelope_to_exact_build_symbols() {
    let manifest = preview_alpha_symbol_manifest();
    let envelope = linked_crash_envelope();
    let binding = bind_crash_envelope(alpha_inputs(&envelope, Some(&manifest)));

    assert_eq!(
        binding.record_kind,
        CRASH_ENVELOPE_SYMBOL_BINDING_RECORD_KIND
    );
    assert_eq!(binding.schema_version, SYMBOL_MANIFEST_SCHEMA_VERSION);
    assert_eq!(binding.binding_state, SymbolBindingState::Linked);
    assert!(binding.is_linked());
    assert_eq!(binding.doc_ref, SYMBOL_MANIFEST_DOC_REF);
    assert_eq!(binding.schema_ref, SYMBOL_MANIFEST_SCHEMA_REF);
    assert_eq!(
        binding.symbol_manifest_ref.as_deref(),
        Some(manifest.manifest_id.as_str())
    );
    assert_eq!(
        binding.manifest_primary_exact_build_identity_ref.as_deref(),
        Some(manifest.primary_exact_build_identity_ref.as_str())
    );
    assert_eq!(
        binding.support_bundle_manifest_ref.as_deref(),
        Some(SUPPORT_BUNDLE_MANIFEST_REF)
    );
    assert_eq!(
        binding.support_export_posture,
        SupportExportPostureClass::MetadataOnlyDefault
    );

    for row in &binding.module_bindings {
        assert_eq!(row.binding_state, ModuleBindingState::Linked);
        assert!(row.mismatch_reason.is_none());
        assert!(row.manifest_symbolication_identity_ref.is_some());
    }

    assert!(binding
        .honesty_notes
        .iter()
        .any(|note| note.contains("agree on exact-build identity")));
}

#[test]
fn missing_manifest_keeps_envelope_refs_without_implying_coverage() {
    let envelope = linked_crash_envelope();
    let binding = bind_crash_envelope(alpha_inputs(&envelope, None));

    assert_eq!(binding.binding_state, SymbolBindingState::MissingManifest);
    assert!(binding.is_missing_manifest());
    assert!(binding.symbol_manifest_ref.is_none());
    assert!(binding.manifest_primary_exact_build_identity_ref.is_none());

    for row in &binding.module_bindings {
        assert_eq!(row.binding_state, ModuleBindingState::ExtraInEnvelope);
        assert!(row.manifest_symbolication_identity_ref.is_none());
    }
    assert!(binding
        .honesty_notes
        .iter()
        .any(|note| note.contains("No symbol manifest is bound")));
    assert!(binding
        .honesty_notes
        .iter()
        .any(|note| note.contains("Release-side symbol manifest is absent")));
}

#[test]
fn partial_manifest_labels_missing_modules_without_claiming_exact_state() {
    let envelope = linked_crash_envelope();
    let manifest = partial_symbol_manifest();
    let binding = bind_crash_envelope(alpha_inputs(&envelope, Some(&manifest)));

    assert_eq!(binding.binding_state, SymbolBindingState::Partial);
    let renderer_row = binding
        .module_bindings
        .iter()
        .find(|row| row.module_id == "renderer.main.bundle.js")
        .expect("renderer module bound");
    assert_eq!(
        renderer_row.binding_state,
        ModuleBindingState::MissingFromManifest
    );
    assert_eq!(
        renderer_row.mismatch_reason.as_deref(),
        Some("module_absent_from_symbol_manifest")
    );

    let shell_row = binding
        .module_bindings
        .iter()
        .find(|row| row.module_id == "aureline-shell")
        .expect("shell module bound");
    assert_eq!(shell_row.binding_state, ModuleBindingState::Linked);

    // partial envelope: manifest carries extra module the envelope does not name
    let partial_envelope = partial_crash_envelope();
    let release_manifest = preview_alpha_symbol_manifest();
    let partial_binding =
        bind_crash_envelope(alpha_inputs(&partial_envelope, Some(&release_manifest)));
    assert_eq!(partial_binding.binding_state, SymbolBindingState::Partial);
    let extra_shell_row = partial_binding
        .module_bindings
        .iter()
        .find(|row| row.module_id == "aureline-shell")
        .expect("shell appears as extra-in-envelope row");
    assert_eq!(
        extra_shell_row.binding_state,
        ModuleBindingState::ExtraInEnvelope
    );
    assert_eq!(
        extra_shell_row.envelope_exact_build_identity_ref, "",
        "envelope identity is empty for manifest-only rows"
    );
}

#[test]
fn build_mismatch_refuses_to_claim_exact_symbolication() {
    let envelope = linked_crash_envelope();
    let mut manifest = preview_alpha_symbol_manifest();
    manifest.primary_exact_build_identity_ref =
        "build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:deadbeefdead"
            .into();
    let binding = bind_crash_envelope(alpha_inputs(&envelope, Some(&manifest)));

    assert_eq!(binding.binding_state, SymbolBindingState::BuildMismatch);
    assert!(binding.is_build_mismatch());
    assert!(binding
        .honesty_notes
        .iter()
        .any(|note| note.contains("disagree on exact-build identity")));
    for row in &binding.module_bindings {
        if row.binding_state == ModuleBindingState::IdentityMismatch {
            assert_eq!(
                row.mismatch_reason.as_deref(),
                Some("primary_exact_build_identity_mismatch")
            );
        }
    }
}

#[test]
fn binding_is_metadata_safe_by_construction() {
    let manifest = preview_alpha_symbol_manifest();
    let envelope = linked_crash_envelope();
    let binding = bind_crash_envelope(alpha_inputs(&envelope, Some(&manifest)));

    assert!(binding.is_export_safe());
    assert!(binding.raw_private_material_excluded);
    assert!(binding.ambient_authority_excluded);
    assert!(!binding.raw_dump_exported);
    // serde round-trip preserves every field; reviewers consume the same shape verbatim.
    let json = serde_json::to_string(&binding).expect("serialize binding");
    let restored: aureline_crash::CrashEnvelopeSymbolBinding =
        serde_json::from_str(&json).expect("deserialize binding");
    assert_eq!(restored, binding);
}

#[test]
fn boundary_schema_and_doc_refs_exist_on_disk() {
    let schema_path = repo_root().join(SYMBOL_MANIFEST_SCHEMA_REF);
    let doc_path = repo_root().join(SYMBOL_MANIFEST_DOC_REF);
    let release_manifest = release_symbol_manifest_path();
    let release_readme = repo_root().join("artifacts/release/m3/symbol_manifest/README.md");

    assert!(
        schema_path.exists(),
        "schema must exist: {}",
        schema_path.display()
    );
    assert!(doc_path.exists(), "doc must exist: {}", doc_path.display());
    assert!(
        release_manifest.exists(),
        "release symbol manifest must exist: {}",
        release_manifest.display()
    );
    assert!(
        release_readme.exists(),
        "release symbol manifest README must exist: {}",
        release_readme.display()
    );
}
