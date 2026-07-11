use super::*;

fn clean_card_input() -> M5AuthHandoffCardResolutionInput {
    M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:test".to_owned(),
        posture: M5AuthHandoffPosture::EmbeddedSignInCheckpoint,
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        provider_label: "Aureline identity provider".to_owned(),
        provider_domain_label: "id.aureline.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::RetryHandoffInApp,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    }
}

fn clean_header_input() -> M5RemoteServiceDashboardHeaderResolutionInput {
    M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:test".to_owned(),
        owner_class: WebviewOwnerClass::FirstPartyEmbedded,
        owner_origin_disclosed: true,
        service_identity_label: "Aureline build service".to_owned(),
        service_identity_stated: true,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        freshness_stated: true,
        export_action_available: true,
        open_console_action_available: true,
        primary_local_recovery_available: true,
        substitutes_for_local_recovery: false,
        allows_high_risk_approval_in_embedded_chrome: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_auth_dashboard_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_AUTH_DASHBOARD_CONTROLS_PACKET_ID);
}

#[test]
fn card_clean_embedded_checkpoint_is_capability_limited() {
    let resolved = resolve_auth_handoff_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.provider_or_domain_stated);
    assert!(resolved.local_continuity_stated);
    assert!(!resolved.hides_continuity_or_imitates());
    assert!(resolved.posture.is_embedded_checkpoint());
    assert_eq!(resolved.handoff_reason, "authenticate_with_provider");
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::CapabilityLimited
    );
    assert_eq!(
        resolved.next_action,
        M5AuthDashboardNextAction::NoActionNeeded
    );
}

#[test]
fn card_clean_external_handoff_is_browser_handoff_only() {
    let mut input = clean_card_input();
    input.posture = M5AuthHandoffPosture::SystemBrowserHandoff;
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::BrowserHandoffOnly
    );
}

#[test]
fn card_provider_unstated_degrades_ac1() {
    let mut input = clean_card_input();
    input.provider_label = String::new();
    input.provider_domain_label = "  ".to_owned();
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AuthHandoffCardDegradeReason::ProviderOrDomainUnstated)
    );
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::NotEvaluated
    );
}

#[test]
fn card_reason_unstated_degrades() {
    let mut input = clean_card_input();
    input.reason_stated = false;
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AuthHandoffCardDegradeReason::ReasonForHandoffUnstated)
    );
}

#[test]
fn card_continuity_unstated_degrades_ac1() {
    let mut input = clean_card_input();
    input.local_continuity_stated = false;
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert!(resolved.hides_continuity_or_imitates());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AuthHandoffCardDegradeReason::LocalContinuityUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5AuthDashboardNextAction::ReviewLocalContinuity
    );
}

#[test]
fn card_fallback_unstated_degrades() {
    let mut input = clean_card_input();
    input.fallback_stated = false;
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AuthHandoffCardDegradeReason::FallbackStateUnstated)
    );
}

#[test]
fn card_device_code_missing_disclosure_degrades() {
    let mut input = clean_card_input();
    input.posture = M5AuthHandoffPosture::DeviceCodeHandoff;
    input.handoff_kind = BrowserHandoffKind::DeviceCodeAuth;
    input.handoff_reason = HandoffReasonClass::AuthorizeDeviceCode;
    input.device_code_stated = false;
    input.expiry_disclosure = ExpiryDisclosureClass::NoExpiryApplicable;
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AuthHandoffCardDegradeReason::DeviceCodeOrExpiryUnstated)
    );
}

#[test]
fn card_device_code_with_disclosure_is_clean() {
    let mut input = clean_card_input();
    input.posture = M5AuthHandoffPosture::DeviceCodeHandoff;
    input.handoff_kind = BrowserHandoffKind::DeviceCodeAuth;
    input.handoff_reason = HandoffReasonClass::AuthorizeDeviceCode;
    input.device_code_stated = true;
    input.expiry_disclosure = ExpiryDisclosureClass::ExpiresWithCountdown;
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.device_code_disclosure_present);
}

#[test]
fn card_imitates_native_degrades_ac1() {
    let mut input = clean_card_input();
    input.imitates_native_approval_ui = true;
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert!(resolved.hides_continuity_or_imitates());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AuthHandoffCardDegradeReason::ImitatesNativeApprovalUi)
    );
}

#[test]
fn card_high_risk_embedded_degrades() {
    let mut input = clean_card_input();
    input.embeds_high_risk_approval_without_step_up = true;
    let resolved = resolve_auth_handoff_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AuthHandoffCardDegradeReason::HighRiskApprovalEmbeddedWithoutStepUp)
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "".to_owned();
    assert_eq!(
        resolve_auth_handoff_card(input).unwrap_err(),
        M5AuthDashboardResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.provider_domain_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_auth_handoff_card(input).unwrap_err(),
        M5AuthDashboardResolutionError::ForbiddenMaterial
    );
}

#[test]
fn header_clean_first_party_is_hosted() {
    let resolved = resolve_remote_service_dashboard_header(clean_header_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.service_identity_stated);
    assert!(resolved.owner_origin_disclosed);
    assert!(resolved.freshness_stated);
    assert!(resolved.primary_local_recovery_available);
    assert!(!resolved.substitutes_or_hides_boundaries());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::LiveFirstPartyHosted
    );
}

#[test]
fn header_identity_unstated_degrades_ac2() {
    let mut input = clean_header_input();
    input.service_identity_stated = false;
    let resolved = resolve_remote_service_dashboard_header(input).unwrap();
    assert!(resolved.substitutes_or_hides_boundaries());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RemoteServiceDashboardHeaderDegradeReason::ServiceIdentityUnstated)
    );
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::NotEvaluated
    );
}

#[test]
fn header_untrusted_owner_degrades() {
    let mut input = clean_header_input();
    input.owner_class = WebviewOwnerClass::UnknownUntrusted;
    let resolved = resolve_remote_service_dashboard_header(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RemoteServiceDashboardHeaderDegradeReason::OwnershipBoundaryUnstated)
    );
}

#[test]
fn header_freshness_unstated_degrades_ac2() {
    let mut input = clean_header_input();
    input.freshness = M5EmbeddedFreshnessState::FreshnessUnknown;
    input.freshness_stated = false;
    let resolved = resolve_remote_service_dashboard_header(input).unwrap();
    assert!(resolved.substitutes_or_hides_boundaries());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RemoteServiceDashboardHeaderDegradeReason::FreshnessOrOfflineUnstated)
    );
}

#[test]
fn header_substitutes_for_local_recovery_degrades_ac2() {
    let mut input = clean_header_input();
    input.substitutes_for_local_recovery = true;
    input.primary_local_recovery_available = false;
    let resolved = resolve_remote_service_dashboard_header(input).unwrap();
    assert!(resolved.substitutes_or_hides_boundaries());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RemoteServiceDashboardHeaderDegradeReason::SubstitutesForLocalRecovery)
    );
    assert_eq!(
        resolved.next_action,
        M5AuthDashboardNextAction::ReviewLocalContinuity
    );
}

#[test]
fn header_no_export_or_console_degrades() {
    let mut input = clean_header_input();
    input.export_action_available = false;
    input.open_console_action_available = false;
    let resolved = resolve_remote_service_dashboard_header(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RemoteServiceDashboardHeaderDegradeReason::ExportOrConsoleActionUnavailable)
    );
    assert_eq!(
        resolved.next_action,
        M5AuthDashboardNextAction::ExportOrOpenConsole
    );
}

#[test]
fn header_high_risk_in_embedded_chrome_degrades() {
    let mut input = clean_header_input();
    input.allows_high_risk_approval_in_embedded_chrome = true;
    let resolved = resolve_remote_service_dashboard_header(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RemoteServiceDashboardHeaderDegradeReason::HighRiskApprovalInEmbeddedChrome)
    );
}

#[test]
fn header_offline_reads_as_offline_snapshot() {
    let mut input = clean_header_input();
    input.freshness = M5EmbeddedFreshnessState::OfflineSnapshot;
    let resolved = resolve_remote_service_dashboard_header(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::OfflineSnapshot
    );
}

#[test]
fn header_empty_id_and_forbidden_material_error() {
    let mut input = clean_header_input();
    input.header_id = "   ".to_owned();
    assert_eq!(
        resolve_remote_service_dashboard_header(input).unwrap_err(),
        M5AuthDashboardResolutionError::EmptyHeaderId
    );

    let mut input = clean_header_input();
    input.service_identity_label = "service://leak".to_owned();
    assert_eq!(
        resolve_remote_service_dashboard_header(input).unwrap_err(),
        M5AuthDashboardResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_auth_dashboard_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.vocabulary_set.handoff_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5AuthDashboardAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5AuthDashboardExportField::BoundaryDispositions);
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.controls_rows[0]
        .remote_service_dashboard_header_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    // Force a clean card to also read as imitating native approval — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.auth_handoff_card_examples[0].degrade_reason = None;
    row.auth_handoff_card_examples[0].imitates_native_approval_ui = true;
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_auth_dashboard_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.masquerades_as_native_approval_chrome = true,
            1 => row.hides_owner_origin_or_handoff_in_menus_only = true,
            2 => row.renders_stale_or_blocked_as_fresh_first_party_truth = true,
            _ => row.embeds_high_risk_approval_without_native_step_up = true,
        }
        assert!(packet
            .validate()
            .contains(&M5AuthDashboardControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_imitation_example_removed() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    for row in &mut packet.controls_rows {
        row.auth_handoff_card_examples.retain(|ex| {
            ex.degrade_reason != Some(M5AuthHandoffCardDegradeReason::ImitatesNativeApprovalUi)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::Ac1NotProven));
}

#[test]
fn ac1_not_proven_when_continuity_example_removed() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    for row in &mut packet.controls_rows {
        row.auth_handoff_card_examples.retain(|ex| {
            ex.degrade_reason != Some(M5AuthHandoffCardDegradeReason::LocalContinuityUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_substitution_example_removed() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    for row in &mut packet.controls_rows {
        row.remote_service_dashboard_header_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5RemoteServiceDashboardHeaderDegradeReason::SubstitutesForLocalRecovery)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::Ac2NotProven));
}

#[test]
fn ac2_not_proven_when_freshness_example_removed() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    for row in &mut packet.controls_rows {
        row.remote_service_dashboard_header_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5RemoteServiceDashboardHeaderDegradeReason::FreshnessOrOfflineUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet
        .governance_review
        .no_high_risk_approval_in_embedded_chrome = false;
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet
        .consumer_projection
        .support_export_reads_single_boundary_source = false;
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AuthDashboardControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_auth_dashboard_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_auth_dashboard_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_auth_dashboard_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_auth_dashboard_controls_export()
        .expect("checked M5 auth/dashboard controls export validates");
    assert_eq!(from_disk.packet_id, M5_AUTH_DASHBOARD_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_auth_dashboard_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_auth_dashboard_controls_auth_handoff_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EmbeddedConsumerSurface::AuthHandoffUi)
        .unwrap();
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Beta);

    let preview = seeded_m5_auth_dashboard_controls_remote_dashboard_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EmbeddedConsumerSurface::RemoteDashboardUi)
        .unwrap();
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5AuthDashboardControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-auth-handoff-card-remote-service-dashboard-header-controls/auth_handoff_beta_narrowed.json"
    )))
    .expect("auth-handoff fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_auth_dashboard_controls_auth_handoff_beta_narrowed()
    );

    let preview: M5AuthDashboardControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-auth-handoff-card-remote-service-dashboard-header-controls/remote_dashboard_preview_narrowed.json"
    )))
    .expect("remote-dashboard fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_auth_dashboard_controls_remote_dashboard_preview_narrowed()
    );
}
