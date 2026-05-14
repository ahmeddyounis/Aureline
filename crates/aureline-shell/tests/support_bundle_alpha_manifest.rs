//! Support-bundle alpha manifest reconstruction tests.

use aureline_commands::enablement::CommandEnablementContext;
use aureline_commands::registry::seeded_registry;
use aureline_shell::commands::review_enforcement::materialize_alpha_review_enforcement_snapshot;
use aureline_shell::commands::{argument_provenance_map_for, CommandReviewRuntimeInputs};
use aureline_shell::palette::materialize_invocation_session_for_review;
use aureline_shell::support_seed::SupportSeedSurface;
use aureline_support::bundle::{ExactBuildCapture, RedactionState, ReleaseChannelClass};

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:abcdef123456",
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    )
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
