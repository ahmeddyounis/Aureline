//! Support-bundle alpha manifest reconstruction tests.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_commands::enablement::CommandEnablementContext;
use aureline_commands::registry::seeded_registry;
use aureline_runtime::RuntimeEvidenceAlphaPacket;
use aureline_shell::commands::review_enforcement::materialize_alpha_review_enforcement_snapshot;
use aureline_shell::commands::{argument_provenance_map_for, CommandReviewRuntimeInputs};
use aureline_shell::palette::materialize_invocation_session_for_review;
use aureline_shell::profiling_alpha::{ProfilingTraceActionKind, ProfilingTraceReplayAlphaSurface};
use aureline_shell::support_seed::SupportSeedSurface;
use aureline_support::bundle::{
    CrashDumpManifest, CrashEnvelope, CrashIncidentTrail, CrashIncidentTrailInputs,
    ExactBuildCapture, RedactionState, ReleaseChannelClass, SymbolicationReport,
    SymbolicationState, SUPPORT_ITEM_CRASH_INCIDENT_TRAIL,
};

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:abcdef123456",
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    )
}

fn incident_fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root")
        .join("fixtures")
        .join("support")
        .join("incident_trail_alpha")
}

fn load_incident_json<T>(name: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let path = incident_fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn crash_incident_trail_fixture() -> CrashIncidentTrail {
    CrashIncidentTrail::from_inputs(CrashIncidentTrailInputs {
        incident_trail_id: "crash-incident-trail:alpha-preview:renderer-panic:0001".into(),
        generated_at: "2026-05-14T06:30:00Z".into(),
        alpha_channel_ref: "alpha-channel:preview:design-partner-linux".into(),
        crash_envelope: load_incident_json::<CrashEnvelope>("crash_envelope.json"),
        crash_dump_manifest: load_incident_json::<CrashDumpManifest>("crash_dump_manifest.json"),
        symbolication_report: Some(load_incident_json::<SymbolicationReport>(
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
fn support_manifest_reconstructs_reviewed_command_route_and_policy_source() {
    let registry = seeded_registry();
    let entry = registry
        .get("cmd:workspace.import_profile")
        .expect("import command exists");
    let runtime = CommandReviewRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: "trusted",
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
    };
    let preflight = entry.preflight(&CommandEnablementContext {
        client_scope: "desktop_product".to_string(),
        workspace_trust_state: "trusted".to_string(),
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
        argument_provenance_map: argument_provenance_map_for(entry),
    });
    let mut session = materialize_invocation_session_for_review(
        entry,
        runtime,
        "command_palette",
        "user_initiated_local",
        &preflight,
    );
    session.approval_posture.approval_state = "approval_granted".to_string();

    let snapshot = materialize_alpha_review_enforcement_snapshot(registry);
    let row = snapshot
        .row_for_command("cmd:workspace.import_profile")
        .expect("review enforcement row exists");
    let surface = SupportSeedSurface::reviewed_command_route_preview(
        fixture_capture(),
        "2026-05-14T05:00:00Z",
        row,
        &session,
    )
    .expect("support preview builds");
    let manifest = surface.manifest();

    assert!(manifest.has_exact_build_identity());
    assert_eq!(manifest.preview_items.len(), 3);
    assert_eq!(
        manifest.redaction_controls.len(),
        manifest.preview_items.len()
    );
    assert!(manifest
        .redaction_controls
        .iter()
        .all(|control| !control.raw_content_export_allowed));
    assert!(manifest.redaction_controls.iter().any(|control| control
        .allowed_narrower_states
        .contains(&RedactionState::RetainedLocalOnly)));
    assert_eq!(
        manifest
            .preview_classification_summary
            .included_support_pack_item_ids
            .len(),
        3
    );

    let context = manifest
        .action_reconstruction_contexts
        .first()
        .expect("action reconstruction context present");
    assert_eq!(context.command_id, "cmd:workspace.import_profile");
    assert_eq!(context.command_descriptor_ref, row.command_revision_ref);
    assert_eq!(context.invocation_session_id, session.invocation_session_id);
    assert!(!context.target_identity_ref.trim().is_empty());
    assert_eq!(context.action_origin_class, "user_keystroke_local");
    assert_eq!(context.action_target_class, "local_host_target");
    assert_eq!(context.action_route_class, "in_process_route");
    assert_eq!(context.action_exposure_class, "workspace_visible_mutation");
    assert_eq!(
        context.policy_source.source_class,
        "invocation_policy_context"
    );
    assert_eq!(
        context.policy_source.policy_epoch,
        session.policy_context.policy_epoch
    );
    assert_eq!(context.exact_build_refs, fixture_capture().exact_build_refs);
    assert!(!context.raw_content_exported);
}

#[test]
fn shell_support_surface_consumes_crash_incident_trail() {
    let trail = crash_incident_trail_fixture();
    let surface = SupportSeedSurface::crash_incident_trail_preview(
        ExactBuildCapture::for_fixture(
            "build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:9f0e7d6c5b4a",
            "0.8.0-alpha.1",
            ReleaseChannelClass::Preview,
        ),
        "2026-05-14T06:31:00Z",
        &trail,
    )
    .expect("support preview builds");
    let manifest = surface.manifest();

    assert_eq!(trail.symbolication_state, SymbolicationState::Exact);
    assert!(trail.is_support_bundle_linked());
    assert!(trail.preserves_safe_next_actions());
    assert_eq!(surface.preview_row_count(), 1);
    assert_eq!(
        manifest.preview_items[0]
            .parity_binding
            .support_pack_item_id,
        SUPPORT_ITEM_CRASH_INCIDENT_TRAIL
    );
    assert!(manifest.has_exact_build_identity());
    assert!(manifest.preview_items[0]
        .redaction
        .visible_high_risk_label
        .is_none());
}

#[test]
fn runtime_evidence_surface_and_support_preview_preserve_import_view_only_truth() {
    let packet = RuntimeEvidenceAlphaPacket::import_view_only_baseline();
    let profiler_surface = ProfilingTraceReplayAlphaSurface::from_packet(&packet);

    assert!(profiler_surface.preserves_runtime_truth());
    assert_eq!(
        profiler_surface.profile_session.mapping_quality_token,
        "symbolized_with_partial_source_maps"
    );
    assert_eq!(
        profiler_surface.comparison.comparison_class_token,
        "import_view_only_not_comparable"
    );
    let live_replay = profiler_surface
        .find_action(ProfilingTraceActionKind::ReservedStartLiveReplay)
        .expect("reserved live replay action exists");
    assert!(!live_replay.is_live);

    let support_surface = SupportSeedSurface::runtime_evidence_preview(
        fixture_capture(),
        "2026-05-14T18:46:00Z",
        &packet.support_export,
    )
    .expect("runtime evidence support preview builds");
    let manifest = support_surface.manifest();

    assert!(manifest.has_exact_build_identity());
    assert_eq!(manifest.preview_items.len(), 3);
    let runtime_row = manifest
        .preview_items
        .iter()
        .find(|item| item.parity_binding.support_pack_item_id == "support.item.runtime_traces")
        .expect("runtime evidence preview row exists");
    assert_eq!(
        runtime_row.redaction.redaction_state,
        RedactionState::RetainedLocalOnly
    );
    assert!(runtime_row
        .redaction
        .visible_high_risk_label
        .as_deref()
        .expect("high-risk label")
        .contains("raw trace or transcript"));
    assert!(runtime_row
        .notes
        .contains("mapping symbolized_with_partial_source_maps"));
    assert!(runtime_row
        .notes
        .contains("comparison class import_view_only_not_comparable"));
    assert!(runtime_row.notes.contains("replay lane import_view_only"));
    assert!(manifest
        .preview_classification_summary
        .excluded_support_pack_item_ids
        .contains(&"support.item.runtime_traces".to_string()));
    assert!(manifest.excluded_classes.iter().any(|excluded| {
        excluded.support_pack_item_id.as_deref() == Some("support.item.runtime_traces")
    }));
}
