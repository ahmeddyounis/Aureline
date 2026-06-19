use super::*;

const BOUNDARY_ID: &str = "docs-preview-boundary:readme:split:0001";

fn packet() -> RenderedPreviewBoundary {
    seeded_stable_rendered_preview_boundary()
}

fn boundary_index(packet: &RenderedPreviewBoundary, kind: PreviewCapabilityKind) -> usize {
    packet
        .capability_boundaries
        .iter()
        .position(|boundary| boundary.capability_kind == kind)
        .expect("capability present")
}

#[test]
fn seeded_boundary_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_boundary_round_trips_through_json() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: RenderedPreviewBoundary =
        serde_json::from_str(&json).expect("boundary round-trips");
    assert_eq!(packet, parsed);
}

#[test]
fn seeded_boundary_covers_every_capability_once() {
    let packet = packet();
    for kind in PreviewCapabilityKind::all() {
        assert!(packet.boundary(kind).is_some(), "missing {kind:?}");
    }
    assert_eq!(
        packet.capability_boundaries.len(),
        PreviewCapabilityKind::all().len()
    );
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "nope".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = packet();
    packet.schema_version = 99;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut packet = packet();
    packet.surface_refs.clear();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::MissingIdentity));
}

#[test]
fn missing_origin_disclosure_fails() {
    let mut packet = packet();
    packet.origin_disclosure = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::MissingIdentity));
}

#[test]
fn dropping_a_capability_fails_coverage() {
    let mut packet = packet();
    packet
        .capability_boundaries
        .retain(|boundary| boundary.capability_kind != PreviewCapabilityKind::Math);
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::CapabilityCoverageIncomplete));
}

#[test]
fn duplicate_capability_fails_coverage() {
    let mut packet = packet();
    let clone = packet.capability_boundaries[0].clone();
    packet.capability_boundaries.push(clone);
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::CapabilityCoverageIncomplete));
}

#[test]
fn missing_boundary_cue_fails() {
    let mut packet = packet();
    packet.capability_boundaries[0].boundary_cue = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::BoundaryCueMissing));
}

#[test]
fn capability_without_escape_fails() {
    let mut packet = packet();
    packet.capability_boundaries[0].escape_to_source_available = false;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::EscapeRouteMissing));
}

#[test]
fn capability_expanding_authority_fails() {
    let mut packet = packet();
    packet.capability_boundaries[0].authority_posture =
        CapabilityAuthority::ImpersonatesNativeApproval;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::CapabilityExpandsAuthority));
}

#[test]
fn browser_control_plane_authority_fails() {
    let mut packet = packet();
    packet.capability_boundaries[0].authority_posture =
        CapabilityAuthority::ClaimsBrowserControlPlane;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::CapabilityExpandsAuthority));
}

#[test]
fn missing_no_authority_expansion_note_fails() {
    let mut packet = packet();
    packet.no_authority_expansion_note = String::new();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::NoAuthorityExpansionNoteMissing));
}

#[test]
fn impersonated_native_shell_owner_fails() {
    let mut packet = packet();
    packet.surface_owner = PreviewSurfaceOwner::ImpersonatedNativeShell;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::SurfaceOwnerImpersonatesAuthority));
}

#[test]
fn impersonated_browser_control_plane_owner_fails() {
    let mut packet = packet();
    packet.surface_owner = PreviewSurfaceOwner::ImpersonatedBrowserControlPlane;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::SurfaceOwnerImpersonatesAuthority));
}

#[test]
fn disclosed_browser_companion_owner_is_legitimate() {
    let mut packet = packet();
    packet.surface_owner = PreviewSurfaceOwner::DisclosedBrowserCompanion;
    assert!(!packet
        .validate()
        .contains(&PreviewBoundaryViolation::SurfaceOwnerImpersonatesAuthority));
}

#[test]
fn active_render_without_grant_fails() {
    let mut packet = packet();
    let idx = boundary_index(&packet, PreviewCapabilityKind::Diagrams);
    packet.capability_boundaries[idx].request_state =
        CapabilityRequestState::RequestedAwaitingConsent;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::ActiveCapabilityWithoutGrant));
}

#[test]
fn active_render_without_consent_ref_fails() {
    let mut packet = packet();
    let idx = boundary_index(&packet, PreviewCapabilityKind::Diagrams);
    packet.capability_boundaries[idx].consent_ref = None;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::ActiveCapabilityWithoutGrant));
}

#[test]
fn grant_without_active_render_fails() {
    let mut packet = packet();
    let idx = boundary_index(&packet, PreviewCapabilityKind::FrontMatter);
    // Grant the capability but leave it rendering only as static — the grant and
    // the active render must stay in lock-step.
    packet.capability_boundaries[idx].request_state = CapabilityRequestState::GrantedSandboxed;
    packet.capability_boundaries[idx].consent_ref = Some("consent:front_matter".to_owned());
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::ActiveCapabilityWithoutGrant));
}

#[test]
fn blocked_capability_without_note_fails() {
    let mut packet = packet();
    let idx = boundary_index(&packet, PreviewCapabilityKind::RemoteAssets);
    packet.capability_boundaries[idx].note = None;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::DegradationNotDisclosed));
}

#[test]
fn external_open_available_requires_action() {
    let mut packet = packet();
    let idx = boundary_index(&packet, PreviewCapabilityKind::RemoteAssets);
    packet.capability_boundaries[idx].open_externally_action = None;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::ExternalOpenActionMismatch));
}

#[test]
fn external_open_action_without_availability_fails() {
    let mut packet = packet();
    let idx = boundary_index(&packet, PreviewCapabilityKind::CustomComponents);
    packet.capability_boundaries[idx].open_externally_action = Some(DocsMaintenanceAction::new(
        "docs.preview.open_external",
        "Open",
    ));
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::ExternalOpenActionMismatch));
}

#[test]
fn recovery_command_must_revert_to_source() {
    let mut packet = packet();
    packet.recover_to_source_command.reverts_to_mode = DocsPreviewMode::Rendered;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::RecoveryCommandInvalid));
}

#[test]
fn recovery_command_must_be_keyboard_reachable() {
    let mut packet = packet();
    packet.recover_to_source_command.keyboard_reachable = false;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::RecoveryCommandInvalid));
}

#[test]
fn open_source_action_must_be_keyboard_reachable() {
    let mut packet = packet();
    packet.open_source_action.keyboard_reachable = false;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::OpenSourceActionInvalid));
}

#[test]
fn source_mode_must_not_render_content() {
    let mut packet = packet();
    packet.active_mode = DocsPreviewMode::Source;
    // Capabilities still describe active rendering while nothing should render.
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::SourceModeRendersContent));
}

#[test]
fn source_mode_with_inert_boundaries_validates() {
    let mut packet = packet();
    packet.active_mode = DocsPreviewMode::Source;
    packet.sanitization_state = DocsPreviewSanitizationState::NotApplicable;
    packet.sanitization_note = None;
    packet.capability_boundaries = not_applicable_capability_boundaries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn rendering_mode_without_concrete_sanitization_fails() {
    let mut packet = packet();
    packet.sanitization_state = DocsPreviewSanitizationState::NotApplicable;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::UnsafePreviewDefault));
}

#[test]
fn rendering_mode_with_all_capabilities_inert_fails() {
    let mut packet = packet();
    packet.capability_boundaries = not_applicable_capability_boundaries();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::UnsafePreviewDefault));
}

#[test]
fn rendering_mode_without_canonical_disclosure_fails() {
    let mut packet = packet();
    packet.rendered_is_not_canonical_note = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::RenderedNotDisclosedAsNonCanonical));
}

#[test]
fn allowed_raw_html_requires_disclosure_note() {
    let mut packet = packet();
    packet.sanitization_state = DocsPreviewSanitizationState::RawHtmlAllowedDisclosed;
    packet.sanitization_note = None;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::SanitizationNoteMissing));
}

#[test]
fn missing_keyboard_parity_fails() {
    let mut packet = packet();
    packet.accessibility_parity.keyboard_parity = false;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::AccessibilityKeyboardParityMissing));
}

#[test]
fn degraded_parity_without_note_fails() {
    let mut packet = packet();
    packet.accessibility_parity.zoom_parity = false;
    packet.accessibility_parity.parity_note = None;
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::AccessibilityParityDegradedWithoutNote));
}

#[test]
fn degraded_parity_with_note_validates() {
    let mut packet = packet();
    packet.accessibility_parity.zoom_parity = false;
    packet.accessibility_parity.parity_note =
        Some("Zoom parity is degraded; raw source stays at the active zoom.".to_owned());
    assert!(!packet
        .validate()
        .contains(&PreviewBoundaryViolation::AccessibilityParityDegradedWithoutNote));
}

#[test]
fn incomplete_source_version_badge_fails() {
    let mut packet = packet();
    packet.source_version_badge.source_class_token = String::new();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::SourceVersionBadgeIncomplete));
}

#[test]
fn forbidden_boundary_material_is_rejected() {
    let mut packet = packet();
    packet.origin_disclosure = "api_key=AKIA-do-not-ship".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewBoundaryViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_capabilities_and_escapes() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains(BOUNDARY_ID));
    assert!(summary.contains("Capability boundaries"));
    assert!(summary.contains("remote_assets"));
    assert!(summary.contains(RECOVER_SOURCE_COMMAND_ID));
    assert!(summary.contains(OPEN_SOURCE_ACTION_REF));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_rendered_preview_boundary_export()
        .expect("checked rendered preview boundary export validates");
    assert_eq!(packet.boundary_id, BOUNDARY_ID);
}

#[test]
fn checked_support_export_matches_seeded_packet() {
    let packet = current_stable_rendered_preview_boundary_export()
        .expect("checked rendered preview boundary export validates");
    assert_eq!(packet, seeded_stable_rendered_preview_boundary());
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/rendered-preview-boundaries/source_mode_local.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/rendered-preview-boundaries/rendered_mirrored_offline.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/rendered-preview-boundaries/rendered_raw_html_disclosed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/rendered-preview-boundaries/rendered_degraded_accessibility.json"
        )),
    ] {
        let packet: RenderedPreviewBoundary =
            serde_json::from_str(raw).expect("fixture parses as rendered preview boundary packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
