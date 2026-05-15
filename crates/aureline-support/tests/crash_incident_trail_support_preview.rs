//! Support preview tests for crash incident-trail linkage.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::bundle::{
    crash_incident_trail_preview, CrashDumpManifest, CrashEnvelope, CrashIncidentTrail,
    CrashIncidentTrailInputs, ExactBuildCapture, RedactionState, ReleaseChannelClass,
    SupportBundleLinkageState, SymbolicationReport, SymbolicationState,
    SUPPORT_ITEM_CRASH_INCIDENT_TRAIL,
};

const GENERATED_AT: &str = "2026-05-14T06:25:00Z";
const FIXTURE_BUILD_ID: &str =
    "build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:9f0e7d6c5b4a";

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root")
        .join("fixtures")
        .join("support")
        .join("incident_trail_alpha")
}

fn load_json<T>(name: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn exact_build() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        FIXTURE_BUILD_ID,
        "0.8.0-alpha.1",
        ReleaseChannelClass::Preview,
    )
}

fn incident_trail() -> CrashIncidentTrail {
    CrashIncidentTrail::from_inputs(CrashIncidentTrailInputs {
        incident_trail_id: "crash-incident-trail:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        alpha_channel_ref: "alpha-channel:preview:design-partner-linux".into(),
        crash_envelope: load_json::<CrashEnvelope>("crash_envelope.json"),
        crash_dump_manifest: load_json::<CrashDumpManifest>("crash_dump_manifest.json"),
        symbolication_report: Some(load_json::<SymbolicationReport>(
            "symbolication_report_exact.json",
        )),
        support_bundle_manifest_ref: Some(
            "support.bundle.manifest.alpha_preview.renderer_panic.local_review".into(),
        ),
        support_preview_snapshot_ref: Some(
            "preview-snapshot:support-bundle:alpha-preview:renderer-panic".into(),
        ),
    })
}

#[test]
fn support_preview_embeds_incident_trail_linkage_as_metadata() {
    let trail = incident_trail();
    let preview =
        crash_incident_trail_preview(exact_build(), GENERATED_AT, &trail).expect("preview");
    let manifest = &preview.manifest;

    assert!(manifest.has_exact_build_identity());
    assert_eq!(manifest.preview_items.len(), 1);
    assert_eq!(trail.symbolication_state, SymbolicationState::Exact);
    assert_eq!(
        trail.support_bundle_linkage.linkage_state,
        SupportBundleLinkageState::Linked
    );

    let row = &manifest.preview_items[0];
    assert_eq!(
        row.parity_binding.support_pack_item_id,
        SUPPORT_ITEM_CRASH_INCIDENT_TRAIL
    );
    assert_eq!(
        row.redaction.redaction_state,
        RedactionState::NotRequiredMetadata
    );
    assert_eq!(
        row.file_section_identity.artifact_kind_class,
        "crash_incident_trail_alpha_record"
    );
    assert!(row
        .file_section_identity
        .source_refs
        .contains(&trail.crash_envelope_ref));
    assert!(row
        .file_section_identity
        .source_refs
        .iter()
        .any(|source_ref| {
            source_ref == "support.bundle.manifest.alpha_preview.renderer_panic.local_review"
        }));
    assert!(!trail.raw_dump_exported);
    assert!(manifest
        .preview_classification_summary
        .included_support_pack_item_ids
        .contains(&SUPPORT_ITEM_CRASH_INCIDENT_TRAIL.to_owned()));
    assert_eq!(manifest.crash_symbolication_frames.len(), 2);
    let renderer_frames = manifest
        .crash_symbolication_frames
        .iter()
        .find(|frames| frames.module_id == "renderer.main.bundle.js")
        .expect("renderer frame projection present");
    assert_eq!(renderer_frames.mapping_quality, "exact");
    assert_eq!(
        renderer_frames.resolved_frame_summary,
        vec![
            "2 renderProjectTree".to_owned(),
            "3 reconcilePaneLayout".to_owned()
        ]
    );
    assert!(!renderer_frames.raw_stack_body_exported);
    assert!(manifest
        .preview_export_parity
        .reconstruction_fields
        .iter()
        .any(|field| field == "crash_symbolication_frames[]"));
}
