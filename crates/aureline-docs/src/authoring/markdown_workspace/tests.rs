use super::*;

const WORKSPACE_ID: &str = "docs-workspace:readme:split:0001";

fn packet() -> MarkdownAuthoringWorkspace {
    seeded_stable_markdown_authoring_workspace()
}

#[test]
fn seeded_workspace_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_workspace_round_trips_through_json() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: MarkdownAuthoringWorkspace =
        serde_json::from_str(&json).expect("workspace round-trips");
    assert_eq!(packet, parsed);
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "nope".to_owned();
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::WrongRecordKind));
}

#[test]
fn missing_identity_fails() {
    let mut packet = packet();
    packet.surface_refs.clear();
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::MissingIdentity));
}

#[test]
fn commonmark_baseline_must_be_declared() {
    let mut packet = packet();
    packet.commonmark_baseline = false;
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::CommonMarkBaselineMissing));
}

#[test]
fn mode_commands_must_cover_every_mode() {
    let mut packet = packet();
    packet
        .mode_commands
        .retain(|command| command.target_mode != DocsPreviewMode::Rendered);
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::ModeCommandsIncomplete));
}

#[test]
fn mode_command_without_keyboard_reach_fails() {
    let mut packet = packet();
    packet.mode_commands[0].keyboard_reachable = false;
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::ModeCommandsIncomplete));
}

#[test]
fn recovery_command_must_revert_to_source() {
    let mut packet = packet();
    packet.recover_to_source_command.reverts_to_mode = DocsPreviewMode::Rendered;
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::RecoveryCommandInvalid));
}

#[test]
fn remembered_preference_must_have_a_command() {
    let mut packet = packet();
    // Drop the split command while split stays the remembered preference.
    packet
        .mode_commands
        .retain(|command| command.target_mode != DocsPreviewMode::Split);
    // Re-add the modes that keep mode-command coverage from being the only failure.
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::RememberedModeUnsupported));
}

#[test]
fn hidden_extension_fails() {
    let mut packet = packet();
    packet
        .active_extensions
        .push("undeclared_extension".to_owned());
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::HiddenExtensionActive));
}

#[test]
fn source_mode_must_not_render_content() {
    let mut packet = packet();
    packet.active_mode = DocsPreviewMode::Source;
    packet.remembered_mode_preference = DocsPreviewMode::Source;
    // Sanitization stays sanitized_safe while nothing should render.
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::SourceModeRendersContent));
}

#[test]
fn rendering_mode_without_concrete_sanitization_fails() {
    let mut packet = packet();
    packet.sanitization_state = DocsPreviewSanitizationState::NotApplicable;
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::UnsafePreviewDefault));
}

#[test]
fn rendering_mode_without_canonical_disclosure_fails() {
    let mut packet = packet();
    packet.rendered_is_not_canonical_note = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::RenderedNotDisclosedAsNonCanonical));
}

#[test]
fn allowed_raw_html_requires_disclosure_note() {
    let mut packet = packet();
    packet.sanitization_state = DocsPreviewSanitizationState::RawHtmlAllowedDisclosed;
    packet.sanitization_note = None;
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::SanitizationNoteMissing));
}

#[test]
fn incomplete_source_version_badge_fails() {
    let mut packet = packet();
    packet.source_version_badge.source_class_token = String::new();
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::SourceVersionBadgeIncomplete));
}

#[test]
fn open_source_action_must_be_keyboard_reachable() {
    let mut packet = packet();
    packet.open_source_action.keyboard_reachable = false;
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::OpenSourceActionInvalid));
}

#[test]
fn handoff_unavailable_must_not_offer_a_browser_action() {
    let mut packet = packet();
    packet.browser_handoff_availability = BrowserHandoffAvailability::UnavailableOffline;
    // open_browser_action is still Some, which contradicts the availability.
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::BrowserHandoffActionMismatch));
}

#[test]
fn handoff_available_requires_a_browser_action() {
    let mut packet = packet();
    packet.open_browser_action = None;
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::BrowserHandoffActionMismatch));
}

#[test]
fn malformed_anchor_fails() {
    let mut packet = packet();
    if let Some(anchor) = packet.anchor_context.as_mut() {
        anchor.preserved_across_modes = false;
    }
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::AnchorContextInvalid));
}

#[test]
fn forbidden_boundary_material_is_rejected() {
    let mut packet = packet();
    packet.commonmark_baseline_note = "api_key=AKIA-do-not-ship".to_owned();
    assert!(packet
        .validate()
        .contains(&MarkdownWorkspaceViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_modes_and_source_truth() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains(WORKSPACE_ID));
    assert!(summary.contains("Active mode"));
    assert!(summary.contains(MODE_SOURCE_COMMAND_ID));
    assert!(summary.contains(RECOVER_SOURCE_COMMAND_ID));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_markdown_authoring_workspace_export()
        .expect("checked markdown workspace export validates");
    assert_eq!(packet.workspace_id, WORKSPACE_ID);
}

#[test]
fn checked_support_export_matches_seeded_packet() {
    let packet = current_stable_markdown_authoring_workspace_export()
        .expect("checked markdown workspace export validates");
    assert_eq!(packet, seeded_stable_markdown_authoring_workspace());
}

#[test]
fn checked_mode_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/markdown-workspace-modes/source_mode_local.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/markdown-workspace-modes/rendered_mode_mirrored_offline.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/markdown-workspace-modes/rendered_raw_html_disclosed.json"
        )),
    ] {
        let packet: MarkdownAuthoringWorkspace =
            serde_json::from_str(raw).expect("fixture parses as workspace packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
