use super::*;

fn stale_session_input() -> M5ProviderRepairEntrypointResolutionInput {
    M5ProviderRepairEntrypointResolutionInput {
        boundary_class: M5ProviderBoundaryClass::AuthStaleSession,
        connection_state: M5AccountConnectionState::StaleSession,
        has_queued_drafts: true,
        has_cached_read: true,
        policy_escalation_available: false,
        boundary_label: "acme-eng session stale".to_owned(),
        repair_target_label: "acme-eng reauth handoff".to_owned(),
        repair_ref: "repair:acme-eng:reauth:1".to_owned(),
    }
}

fn policy_blocked_input() -> M5ProviderRepairEntrypointResolutionInput {
    M5ProviderRepairEntrypointResolutionInput {
        boundary_class: M5ProviderBoundaryClass::PolicyBlocked,
        connection_state: M5AccountConnectionState::PolicyBlocked,
        has_queued_drafts: false,
        has_cached_read: true,
        policy_escalation_available: true,
        boundary_label: "acme-eng policy blocks widen".to_owned(),
        repair_target_label: "acme-eng policy review".to_owned(),
        repair_ref: "repair:acme-eng:policy:1".to_owned(),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn stale_session_is_reauth_with_preserved_work_and_no_blind_reentry() {
    let resolved = resolve_provider_repair_entrypoint(&stale_session_input()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5ProviderRepairPosture::ReauthSessionRow
    );
    assert_eq!(
        resolved.repair_entrypoint,
        M5RepairEntrypointClass::OpenReauthHandoff
    );
    assert!(resolved.preserves_queued_work);
    assert!(resolved.preserves_cached_read_continuity);
    assert!(resolved.preserves_reviewed_export_path);
    assert!(!resolved.requires_blind_credential_reentry);
    assert!(!resolved.isolated_from_diagnostics);
    assert!(resolved.links_to_diagnostics);
    assert!(resolved
        .available_actions
        .contains(&M5ProviderRepairRowAction::OpenRepairEntrypoint));
    assert!(resolved
        .available_actions
        .contains(&M5ProviderRepairRowAction::OpenLinkedDiagnostics));
    assert!(resolved
        .available_actions
        .contains(&M5ProviderRepairRowAction::ExportRepairEvidence));
    assert!(!resolved
        .available_actions
        .contains(&M5ProviderRepairRowAction::RequestPolicyEscalation));
}

#[test]
fn posture_and_entrypoint_map_one_to_one_from_boundary() {
    let cases = [
        (
            M5ProviderBoundaryClass::NetworkEgressBlocked,
            M5ProviderRepairPosture::NetworkEgressRepairRow,
            M5RepairEntrypointClass::OpenNetworkEgressDiagnostics,
        ),
        (
            M5ProviderBoundaryClass::AuthStaleSession,
            M5ProviderRepairPosture::ReauthSessionRow,
            M5RepairEntrypointClass::OpenReauthHandoff,
        ),
        (
            M5ProviderBoundaryClass::AuthScopeLimited,
            M5ProviderRepairPosture::WidenScopeRow,
            M5RepairEntrypointClass::OpenScopeReview,
        ),
        (
            M5ProviderBoundaryClass::MappingBroken,
            M5ProviderRepairPosture::RemapTargetRow,
            M5RepairEntrypointClass::OpenMappingRepair,
        ),
        (
            M5ProviderBoundaryClass::ProviderIncompatible,
            M5ProviderRepairPosture::CompatibilityReviewRow,
            M5RepairEntrypointClass::OpenCompatibilityReport,
        ),
        (
            M5ProviderBoundaryClass::PolicyBlocked,
            M5ProviderRepairPosture::PolicyBlockedRow,
            M5RepairEntrypointClass::OpenPolicyReview,
        ),
    ];
    for (boundary, posture, entrypoint) in cases {
        let mut input = stale_session_input();
        input.boundary_class = boundary;
        input.policy_escalation_available = true;
        let resolved = resolve_provider_repair_entrypoint(&input).expect("resolves");
        assert_eq!(resolved.row_posture, posture, "posture for {boundary:?}");
        assert_eq!(
            resolved.repair_entrypoint, entrypoint,
            "entrypoint for {boundary:?}"
        );
    }
}

#[test]
fn every_repair_links_to_support_and_export_diagnostics() {
    for boundary in M5ProviderBoundaryClass::ALL {
        let mut input = stale_session_input();
        input.boundary_class = boundary;
        input.policy_escalation_available = true;
        let resolved = resolve_provider_repair_entrypoint(&input).expect("resolves");
        assert!(
            resolved
                .linked_diagnostics
                .contains(&M5LinkedDiagnosticClass::SupportBundleDiagnostic),
            "support bundle diagnostic missing for {boundary:?}"
        );
        assert!(
            resolved
                .linked_diagnostics
                .contains(&M5LinkedDiagnosticClass::ExportRedactionDiagnostic),
            "export redaction diagnostic missing for {boundary:?}"
        );
        assert!(!resolved.linked_diagnostics.is_empty());
    }
}

#[test]
fn network_boundary_links_network_diagnostic() {
    let mut input = stale_session_input();
    input.boundary_class = M5ProviderBoundaryClass::NetworkEgressBlocked;
    let resolved = resolve_provider_repair_entrypoint(&input).expect("resolves");
    assert!(resolved
        .linked_diagnostics
        .contains(&M5LinkedDiagnosticClass::NetworkEgressDiagnostic));
}

#[test]
fn mapping_boundary_links_compatibility_diagnostic() {
    let mut input = stale_session_input();
    input.boundary_class = M5ProviderBoundaryClass::MappingBroken;
    let resolved = resolve_provider_repair_entrypoint(&input).expect("resolves");
    assert!(resolved
        .linked_diagnostics
        .contains(&M5LinkedDiagnosticClass::ProviderCompatibilityDiagnostic));
}

#[test]
fn policy_blocked_offers_escalation_not_self_serve() {
    let resolved = resolve_provider_repair_entrypoint(&policy_blocked_input()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5ProviderRepairPosture::PolicyBlockedRow
    );
    assert!(resolved
        .available_actions
        .contains(&M5ProviderRepairRowAction::RequestPolicyEscalation));
    assert!(!resolved
        .available_actions
        .contains(&M5ProviderRepairRowAction::OpenRepairEntrypoint));
}

#[test]
fn continuity_guarantees_are_always_all_four() {
    let resolved = resolve_provider_repair_entrypoint(&stale_session_input()).expect("resolves");
    assert_eq!(
        resolved.continuity_guarantees,
        M5RepairContinuityGuarantee::ALL.to_vec()
    );
}

#[test]
fn no_entrypoint_requires_blind_credential_reentry() {
    for entrypoint in M5RepairEntrypointClass::ALL {
        assert!(!entrypoint.requires_blind_credential_reentry());
    }
}

#[test]
fn policy_blocked_without_escalation_is_error() {
    let mut input = policy_blocked_input();
    input.policy_escalation_available = false;
    assert_eq!(
        resolve_provider_repair_entrypoint(&input),
        Err(M5ProviderRepairEntrypointResolutionError::PolicyBlockedWithoutEscalationRoute)
    );
}

#[test]
fn empty_labels_and_refs_are_errors() {
    let mut input = stale_session_input();
    input.boundary_label = "  ".to_owned();
    assert_eq!(
        resolve_provider_repair_entrypoint(&input),
        Err(M5ProviderRepairEntrypointResolutionError::EmptyBoundaryLabel)
    );

    let mut input = stale_session_input();
    input.repair_target_label = String::new();
    assert_eq!(
        resolve_provider_repair_entrypoint(&input),
        Err(M5ProviderRepairEntrypointResolutionError::EmptyRepairTargetLabel)
    );

    let mut input = stale_session_input();
    input.repair_ref = String::new();
    assert_eq!(
        resolve_provider_repair_entrypoint(&input),
        Err(M5ProviderRepairEntrypointResolutionError::EmptyRepairRef)
    );
}

#[test]
fn forbidden_material_is_rejected() {
    let mut input = stale_session_input();
    input.repair_target_label = "https://acme.example/secret".to_owned();
    assert_eq!(
        resolve_provider_repair_entrypoint(&input),
        Err(M5ProviderRepairEntrypointResolutionError::ForbiddenRepairMaterial)
    );
}

#[test]
fn resolution_preserves_identity_exactly() {
    let input = stale_session_input();
    let resolved = resolve_provider_repair_entrypoint(&input).expect("resolves");
    assert_eq!(resolved.boundary_label, input.boundary_label);
    assert_eq!(resolved.repair_target_label, input.repair_target_label);
    assert_eq!(resolved.repair_ref, input.repair_ref);
}

// ---- packet validation --------------------------------------------------

#[test]
fn seed_packet_validates_clean() {
    let packet = seeded_m5_provider_repair_entrypoint_packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
}

#[test]
fn seed_packet_covers_every_consumer_surface() {
    let packet = seeded_m5_provider_repair_entrypoint_packet();
    assert_eq!(
        packet.rows.len(),
        M5ProviderRepairConsumerSurface::ALL.len()
    );
}

#[test]
fn wrong_record_kind_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::WrongRecordKind));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet
        .vocabulary_set
        .boundary_classes
        .push("bogus".to_owned());
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::VocabularySetDrift));
}

#[test]
fn missing_consumer_surface_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.rows.remove(0);
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::RequiredConsumerMissing));
}

#[test]
fn missing_source_contract_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet
        .source_contract_refs
        .retain(|r| r != M5_PROVIDER_REPAIR_ENTRYPOINT_REAUTH_REQUIREMENT_REF);
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::MissingSourceContracts));
}

#[test]
fn mandatory_anatomy_missing_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ProviderRepairRowAnatomyPart::RepairEntrypointCue);
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5ProviderRepairRowExportField::LinkedDiagnostics);
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::MandatoryExportMissing));
}

#[test]
fn missing_keyboard_route_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.rows[0]
        .accessibility_routes
        .retain(|r| *r != M5ProviderAccessibilityRoute::KeyboardFocusable);
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.rows[0].examples[0].resolved.row_posture = M5ProviderRepairPosture::PolicyBlockedRow;
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::ExampleResolutionDrift));
}

#[test]
fn row_invariant_violation_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.rows[0].loses_queued_work = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::RowInvariantViolated));
}

#[test]
fn boundary_class_coverage_is_enforced() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    // Drop every mapping-broken example so a boundary class goes unexercised.
    for row in &mut packet.rows {
        row.examples
            .retain(|c| c.resolved.boundary_class != M5ProviderBoundaryClass::MappingBroken);
    }
    let violations = packet.validate();
    assert!(
        violations.contains(&M5ProviderRepairEntrypointViolation::BoundaryClassCoverageUnproven)
    );
}

#[test]
fn governance_review_incomplete_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet
        .governance_review
        .settings_never_isolated_from_diagnostics = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::GovernanceReviewIncomplete));
}

#[test]
fn release_posture_incomplete_is_flagged() {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderRepairEntrypointViolation::ReleasePostureIncomplete));
}

// ---- renders ------------------------------------------------------------

#[test]
fn csv_lists_every_consumer() {
    let csv = seeded_m5_provider_repair_entrypoint_packet().render_matrix_csv();
    for surface in M5ProviderRepairConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn markdown_summary_names_the_primitive() {
    let md = seeded_m5_provider_repair_entrypoint_packet().render_markdown_summary();
    assert!(md.contains("Repair-Entrypoint Row Primitive"));
    assert!(md.contains("open_reauth_handoff"));
}

// ---- artifacts / fixtures ----------------------------------------------

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_provider_repair_entrypoint_export()
        .expect("checked M5 repair entrypoint row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_PROVIDER_REPAIR_ENTRYPOINT_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_provider_repair_entrypoint_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_provider_repair_entrypoint_sync_behavior_beta_narrowed(),
        seeded_m5_provider_repair_entrypoint_privacy_redaction_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5ProviderRepairConsumerSurface::ALL.len()
        );
    }

    let sync = seeded_m5_provider_repair_entrypoint_sync_behavior_beta_narrowed();
    let row = sync
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5ProviderRepairConsumerSurface::SyncBehaviorRow)
        .expect("sync-behavior row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Beta);

    let privacy = seeded_m5_provider_repair_entrypoint_privacy_redaction_preview_narrowed();
    let row = privacy
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5ProviderRepairConsumerSurface::PrivacyRedactionRow)
        .expect("privacy-redaction row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let sync: M5ProviderRepairEntrypointPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-settings-repair-entrypoint-row/sync_behavior_beta_narrowed.json"
    )))
    .expect("sync-behavior fixture parses");
    assert!(sync.validate().is_empty());
    assert_eq!(
        sync,
        seeded_m5_provider_repair_entrypoint_sync_behavior_beta_narrowed()
    );

    let privacy: M5ProviderRepairEntrypointPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-settings-repair-entrypoint-row/privacy_redaction_preview_narrowed.json"
    )))
    .expect("privacy-redaction fixture parses");
    assert!(privacy.validate().is_empty());
    assert_eq!(
        privacy,
        seeded_m5_provider_repair_entrypoint_privacy_redaction_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_provider_repair_entrypoint_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
