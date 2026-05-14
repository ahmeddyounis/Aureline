//! Protected alpha crash incident-trail tests.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_crash::{
    CrashDumpManifest, CrashEnvelope, CrashIncidentTrail, CrashIncidentTrailInputs,
    ModuleMappingQuality, NextSafeActionKind, SupportBundleLinkageState, SymbolicationReport,
    SymbolicationState,
};

const GENERATED_AT: &str = "2026-05-14T06:20:00Z";
const SUPPORT_BUNDLE_MANIFEST_REF: &str =
    "support.bundle.manifest.alpha_preview.renderer_panic.local_review";
const SUPPORT_PREVIEW_SNAPSHOT_REF: &str =
    "preview-snapshot:support-bundle:alpha-preview:renderer-panic";
const ALPHA_CHANNEL_REF: &str = "alpha-channel:preview:design-partner-linux";

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

fn inputs(report: Option<SymbolicationReport>) -> CrashIncidentTrailInputs {
    CrashIncidentTrailInputs {
        incident_trail_id: "crash-incident-trail:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        alpha_channel_ref: ALPHA_CHANNEL_REF.into(),
        crash_envelope: load_json::<CrashEnvelope>("crash_envelope.json"),
        crash_dump_manifest: load_json::<CrashDumpManifest>("crash_dump_manifest.json"),
        symbolication_report: report,
        support_bundle_manifest_ref: Some(SUPPORT_BUNDLE_MANIFEST_REF.into()),
        support_preview_snapshot_ref: Some(SUPPORT_PREVIEW_SNAPSHOT_REF.into()),
    }
}

fn exact_report() -> SymbolicationReport {
    load_json("symbolication_report_exact.json")
}

fn partial_report() -> SymbolicationReport {
    load_json("symbolication_report_partial.json")
}

#[test]
fn exact_build_symbolication_links_crash_to_support_bundle_manifest() {
    let trail = CrashIncidentTrail::from_inputs(inputs(Some(exact_report())));

    assert_eq!(trail.symbolication_state, SymbolicationState::Exact);
    assert_eq!(
        trail.support_bundle_linkage.linkage_state,
        SupportBundleLinkageState::Linked
    );
    assert_eq!(
        trail
            .support_bundle_linkage
            .support_bundle_manifest_ref
            .as_deref(),
        Some(SUPPORT_BUNDLE_MANIFEST_REF)
    );
    assert!(trail.is_support_bundle_linked());
    assert!(trail.labels_symbolication_honestly());
    assert!(trail.preserves_safe_next_actions());
    assert_eq!(trail.trace_ids.len(), 2);
    assert!(!trail.raw_dump_exported);
    assert!(trail
        .module_summaries
        .iter()
        .all(|module| module.mapping_quality == ModuleMappingQuality::Exact));
}

#[test]
fn partial_symbolication_is_labeled_without_breaking_bundle_linkage() {
    let trail = CrashIncidentTrail::from_inputs(inputs(Some(partial_report())));

    assert_eq!(trail.symbolication_state, SymbolicationState::Partial);
    assert_eq!(
        trail.support_bundle_linkage.linkage_state,
        SupportBundleLinkageState::Linked
    );
    assert!(trail
        .honesty_notes
        .iter()
        .any(|note| note.contains("partial")));
    assert!(trail.module_summaries.iter().any(|module| {
        module.module_id == "renderer.main.bundle.js"
            && module.mapping_quality == ModuleMappingQuality::Partial
            && module.unresolved_reason.as_deref()
                == Some("source_map_line_table_missing_for_renderer_frames")
    }));
    assert!(trail.preserves_safe_next_actions());
}

#[test]
fn missing_symbolication_keeps_evidence_and_safe_actions_visible() {
    let trail = CrashIncidentTrail::from_inputs(inputs(None));

    assert_eq!(trail.symbolication_state, SymbolicationState::Missing);
    assert!(trail.symbolication_report_ref.is_none());
    assert_eq!(
        trail.support_bundle_linkage.linkage_state,
        SupportBundleLinkageState::Linked
    );
    assert!(trail
        .honesty_notes
        .iter()
        .any(|note| note.contains("No symbolication report")));
    assert!(trail
        .module_summaries
        .iter()
        .all(|module| module.mapping_quality == ModuleMappingQuality::Missing));
    assert!(trail
        .next_safe_actions
        .iter()
        .any(|action| action.action_kind == NextSafeActionKind::ExportEvidence));
    assert!(trail.preserves_safe_next_actions());
}

#[test]
fn exact_build_mismatch_refuses_to_claim_exact_symbolication() {
    let mut report = exact_report();
    report.primary_exact_build_identity_ref =
        "build-id:aureline:preview:wrong-build:x86_64-unknown-linux-gnu:release:0000".into();

    let trail = CrashIncidentTrail::from_inputs(inputs(Some(report)));

    assert_eq!(trail.symbolication_state, SymbolicationState::BuildMismatch);
    assert!(trail
        .honesty_notes
        .iter()
        .any(|note| { note.contains("Exact-build identities differ") }));
    assert!(trail
        .module_summaries
        .iter()
        .any(|module| module.mapping_quality == ModuleMappingQuality::BuildMismatch));
    assert!(trail.preserves_safe_next_actions());
}

#[test]
fn support_bundle_manifest_ref_absence_is_honest_not_silent() {
    let mut test_inputs = inputs(Some(exact_report()));
    test_inputs.support_bundle_manifest_ref = None;

    let trail = CrashIncidentTrail::from_inputs(test_inputs);

    assert_eq!(trail.symbolication_state, SymbolicationState::Exact);
    assert_eq!(
        trail.support_bundle_linkage.linkage_state,
        SupportBundleLinkageState::MissingManifestRef
    );
    assert!(!trail.is_support_bundle_linked());
    assert!(trail
        .honesty_notes
        .iter()
        .any(|note| { note.contains("Support-bundle manifest linkage is missing") }));
}
