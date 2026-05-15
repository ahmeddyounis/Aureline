//! Exact-build local symbolication integration tests.

use aureline_build_info::exact_build_identity_ref;
use aureline_crash::{
    symbolicate_exact_build, CrashDumpManifest, CrashEnvelope, CrashFrame, CrashIncidentTrail,
    CrashIncidentTrailInputs, CrashModule, CrashModuleIdentity, ExactBuildSymbolicationError,
    ExactBuildSymbolicationInput, InTreeSymbolFile, InTreeSymbolFrame, InTreeSymbolModule,
    ModuleMappingQuality, SymbolicationState,
};

const GENERATED_AT: &str = "2026-05-15T18:00:00Z";
const CAPTURED_AT: &str = "2026-05-15T17:59:30Z";
const CRASH_ENVELOPE_REF: &str = "crash-envelope:live-symbolication-alpha:renderer:0001";
const CRASH_DUMP_REF: &str = "crash-dump:live-symbolication-alpha:renderer:0001";
const CRASH_DUMP_MANIFEST_REF: &str =
    "fixtures/support/live_symbolication_alpha/crash_dump_manifest.json";
const SUPPORT_BUNDLE_REF: &str = "support_bundle.live_symbolication_alpha.local_review";
const SUPPORT_BUNDLE_MANIFEST_REF: &str =
    "support.bundle.manifest.live_symbolication_alpha.local_review";
const SUPPORT_PREVIEW_SNAPSHOT_REF: &str =
    "preview-snapshot:support-bundle:live-symbolication-alpha";
const SOURCE_MAP_DIGEST: &str =
    "sha256:liveexactbuildsymbolication0000000000000000000000000000000000000001";

fn crash_envelope(primary_exact_build_identity_ref: String) -> CrashEnvelope {
    CrashEnvelope {
        schema_version: 1,
        record_kind: "synthetic_crash_envelope".into(),
        fixture_id: Some("support.live_symbolication_alpha.renderer".into()),
        crash_envelope_ref: CRASH_ENVELOPE_REF.into(),
        captured_at: CAPTURED_AT.into(),
        chronology_capture_state: "captured_without_recording".into(),
        fault_domain_id: "fd.renderer.live_symbolication_alpha".into(),
        primary_exact_build_identity_ref: primary_exact_build_identity_ref.clone(),
        crash_dump_manifest_ref: CRASH_DUMP_MANIFEST_REF.into(),
        support_bundle_ref: SUPPORT_BUNDLE_REF.into(),
        trace_ids: vec!["trace:renderer:live-symbolication-alpha:0001".into()],
        modules: vec![CrashModule {
            module_id: "renderer.main.bundle.js".into(),
            module_kind: "web_bundle".into(),
            artifact_family_class: "source_map_bundle".into(),
            exact_build_identity_ref: format!("{primary_exact_build_identity_ref}:source-map"),
            module_identity: Some(CrashModuleIdentity {
                code_file_name: None,
                build_id: None,
                debug_id: None,
                image_base: None,
                bundle_revision_ref: Some("renderer-bundle:live-symbolication-alpha".into()),
                source_map_digest: Some(SOURCE_MAP_DIGEST.into()),
                generated_asset_ref: Some("dist/renderer.main.bundle.js".into()),
            }),
            faulting_frames: vec![
                CrashFrame {
                    frame_index: 0,
                    address: None,
                    generated_location: Some("renderer.main.bundle.js:120:17".into()),
                    symbol_hint: "renderProjectTree".into(),
                },
                CrashFrame {
                    frame_index: 1,
                    address: None,
                    generated_location: Some("renderer.main.bundle.js:188:9".into()),
                    symbol_hint: "reconcilePaneLayout".into(),
                },
            ],
        }],
    }
}

fn crash_dump_manifest(primary_exact_build_identity_ref: String) -> CrashDumpManifest {
    CrashDumpManifest {
        schema_version: 1,
        record_kind: "crash_dump_manifest".into(),
        crash_dump_ref: CRASH_DUMP_REF.into(),
        captured_at: CAPTURED_AT.into(),
        dump_format_class: "synthetic_stack".into(),
        record_class_id: "crash_diagnostic_payload".into(),
        data_class: "metadata_only".into(),
        redaction_class: "metadata_safe_default".into(),
        support_export_posture: "included_metadata_only".into(),
        storage_mode: "embedded_export_copy".into(),
        embedding_state: "embedded".into(),
        artifact_sha256: None,
        local_retention_ref: None,
        primary_exact_build_identity_ref,
        fault_domain_refs: vec!["fd.renderer.live_symbolication_alpha".into()],
        module_refs: vec!["renderer.main.bundle.js".into()],
        support_bundle_ref: SUPPORT_BUNDLE_REF.into(),
        notes: "Synthetic stack metadata only; no raw dump bytes are required.".into(),
    }
}

fn symbol_file(runtime_identity_ref: String) -> InTreeSymbolFile {
    InTreeSymbolFile {
        schema_version: 1,
        record_kind: "in_tree_symbol_file".into(),
        fixture_id: Some("support.live_symbolication_alpha.renderer".into()),
        symbolication_report_ref: "symbolication-report:live-symbolication-alpha:renderer:0001"
            .into(),
        generated_at: GENERATED_AT.into(),
        runtime_identity_ref: runtime_identity_ref.clone(),
        support_bundle_ref: SUPPORT_BUNDLE_REF.into(),
        release_evidence_packet_ref: Some(
            "release-evidence:aureline:live-symbolication-alpha".into(),
        ),
        claim_row_refs: vec!["claim_row:build.exact_build_identity".into()],
        retention_seed_ref: Some("artifacts/support/crash_artifact_retention_seed.json".into()),
        modules: vec![InTreeSymbolModule {
            module_id: "renderer.main.bundle.js".into(),
            module_kind: "web_bundle".into(),
            symbolication_identity_ref: format!("{runtime_identity_ref}:source-map"),
            support_archive_identity_ref: None,
            matched_symbol_tag: format!("symbol-tag:source-map-digest:{SOURCE_MAP_DIGEST}"),
            frames: vec![
                InTreeSymbolFrame {
                    frame_index: 0,
                    address: None,
                    generated_location: Some("renderer.main.bundle.js:120:17".into()),
                    symbol_name: "renderProjectTree".into(),
                    source_location: "crates/aureline-shell/src/explorer/tree.rs:42:9".into(),
                    resolved_frame_summary:
                        "0 crates/aureline-shell/src/explorer/tree.rs:42:9 renderProjectTree".into(),
                },
                InTreeSymbolFrame {
                    frame_index: 1,
                    address: None,
                    generated_location: Some("renderer.main.bundle.js:188:9".into()),
                    symbol_name: "reconcilePaneLayout".into(),
                    source_location: "crates/aureline-shell/src/layout/split_tree.rs:88:13".into(),
                    resolved_frame_summary:
                        "1 crates/aureline-shell/src/layout/split_tree.rs:88:13 reconcilePaneLayout"
                            .into(),
                },
            ],
        }],
    }
}

#[test]
fn current_exact_build_symbolicates_synthetic_renderer_frames() {
    let current_identity = exact_build_identity_ref();
    let envelope = crash_envelope(current_identity.clone());
    let dump_manifest = crash_dump_manifest(current_identity.clone());
    let symbol_file = symbol_file(current_identity.clone());

    let report = symbolicate_exact_build(ExactBuildSymbolicationInput {
        crash_envelope: &envelope,
        crash_dump_manifest: &dump_manifest,
        symbol_file: &symbol_file,
    })
    .expect("current exact-build identity symbolicates");

    assert_eq!(report.primary_exact_build_identity_ref, current_identity);
    assert_eq!(report.result_state, "exact_match");
    assert_eq!(report.module_results.len(), 1);

    let renderer = &report.module_results[0];
    assert_eq!(renderer.mapping_state, "exact");
    assert_eq!(
        renderer.symbolication_identity_ref.as_deref(),
        Some(symbol_file.modules[0].symbolication_identity_ref.as_str())
    );
    assert_eq!(
        renderer.resolved_frame_summary,
        vec![
            "0 crates/aureline-shell/src/explorer/tree.rs:42:9 renderProjectTree".to_owned(),
            "1 crates/aureline-shell/src/layout/split_tree.rs:88:13 reconcilePaneLayout".to_owned(),
        ]
    );

    let trail = CrashIncidentTrail::from_inputs(CrashIncidentTrailInputs {
        incident_trail_id: "crash-incident-trail:live-symbolication-alpha:renderer:0001".into(),
        generated_at: GENERATED_AT.into(),
        alpha_channel_ref: "alpha-channel:local-dev".into(),
        crash_envelope: envelope,
        crash_dump_manifest: dump_manifest,
        symbolication_report: Some(report),
        support_bundle_manifest_ref: Some(SUPPORT_BUNDLE_MANIFEST_REF.into()),
        support_preview_snapshot_ref: Some(SUPPORT_PREVIEW_SNAPSHOT_REF.into()),
    });

    assert_eq!(trail.symbolication_state, SymbolicationState::Exact);
    assert_eq!(
        trail.module_summaries[0].mapping_quality,
        ModuleMappingQuality::Exact
    );
    assert_eq!(
        trail.module_summaries[0].resolved_frame_summary,
        vec![
            "0 crates/aureline-shell/src/explorer/tree.rs:42:9 renderProjectTree".to_owned(),
            "1 crates/aureline-shell/src/layout/split_tree.rs:88:13 reconcilePaneLayout".to_owned(),
        ]
    );
}

#[test]
fn mismatched_exact_build_identity_is_rejected_with_typed_error() {
    let current_identity = exact_build_identity_ref();
    let mismatched_identity = format!("{current_identity}:different-build");
    let envelope = crash_envelope(mismatched_identity.clone());
    let dump_manifest = crash_dump_manifest(mismatched_identity.clone());
    let symbol_file = symbol_file(current_identity.clone());

    let error = symbolicate_exact_build(ExactBuildSymbolicationInput {
        crash_envelope: &envelope,
        crash_dump_manifest: &dump_manifest,
        symbol_file: &symbol_file,
    })
    .expect_err("mismatched exact-build identity fails closed");

    assert_eq!(
        error,
        ExactBuildSymbolicationError::PrimaryIdentityMismatch {
            expected: current_identity,
            actual: mismatched_identity,
        }
    );
}
