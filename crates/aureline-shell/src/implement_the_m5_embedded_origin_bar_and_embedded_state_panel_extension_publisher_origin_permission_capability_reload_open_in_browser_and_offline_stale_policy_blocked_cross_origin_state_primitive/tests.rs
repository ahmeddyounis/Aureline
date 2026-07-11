use super::*;

fn clean_bar_input() -> M5EmbeddedOriginBarResolutionInput {
    M5EmbeddedOriginBarResolutionInput {
        bar_id: "origin-bar:test".to_owned(),
        owner_class: WebviewOwnerClass::ExtensionOwned,
        origin_disclosure: OriginDisclosureClass::NamedExtensionOrigin,
        extension_name: "Acme Language Pack".to_owned(),
        publisher: "Acme Tools".to_owned(),
        owner_origin_disclosed: true,
        permission_state: WebviewPermissionState::ScopedPermissionsGranted,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        capability_limits_disclosed: true,
        reload_available: true,
        open_in_browser_kind: Some(BrowserHandoffKind::ProviderContentView),
        open_in_browser_available: true,
        imitates_native_permission_ui: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    }
}

fn clean_panel_input() -> M5EmbeddedStatePanelResolutionInput {
    M5EmbeddedStatePanelResolutionInput {
        panel_id: "state-panel:test".to_owned(),
        state_class: M5EmbeddedStateClass::PolicyBlocked,
        owner_class: WebviewOwnerClass::ProviderOwned,
        state_explained: true,
        severity_and_support_boundary_shared: true,
        recovery_action_available: true,
        shown_as_fresh_first_party: false,
        imitates_native_permission_ui: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_embedded_origin_state_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_EMBEDDED_ORIGIN_STATE_CONTROLS_PACKET_ID
    );
}

#[test]
fn bar_clean_names_owner_and_capability() {
    let resolved = resolve_embedded_origin_bar(clean_bar_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.owner_origin_disclosed);
    assert!(resolved.capability_limits_disclosed);
    assert!(!resolved.imitates_native_permission_ui);
    assert!(!resolved.hides_owner_or_capability());
    assert_eq!(resolved.owner_origin, "extension_owned");
    assert_eq!(resolved.origin_disclosure, "named_extension_origin");
    assert_eq!(resolved.permission_state, "scoped_permissions_granted");
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::CapabilityLimited
    );
    assert_eq!(
        resolved.next_action,
        M5EmbeddedOriginStateNextAction::NoActionNeeded
    );
}

#[test]
fn bar_owner_undisclosed_degrades_ac1() {
    let mut input = clean_bar_input();
    input.owner_origin_disclosed = false;
    let resolved = resolve_embedded_origin_bar(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.hides_owner_or_capability());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedOriginBarDegradeReason::OwnerOrOriginUnstated)
    );
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::NotEvaluated
    );
}

#[test]
fn bar_untrusted_origin_degrades() {
    let mut input = clean_bar_input();
    input.owner_class = WebviewOwnerClass::UnknownUntrusted;
    input.origin_disclosure = OriginDisclosureClass::UndisclosedOriginBlocked;
    let resolved = resolve_embedded_origin_bar(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedOriginBarDegradeReason::OwnerOrOriginUnstated)
    );
}

#[test]
fn bar_publisher_missing_degrades() {
    let mut input = clean_bar_input();
    input.publisher = "  ".to_owned();
    let resolved = resolve_embedded_origin_bar(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedOriginBarDegradeReason::PublisherOrExtensionUnstated)
    );
}

#[test]
fn bar_capability_undisclosed_degrades_ac1() {
    let mut input = clean_bar_input();
    input.capability_limits_disclosed = false;
    let resolved = resolve_embedded_origin_bar(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.hides_owner_or_capability());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedOriginBarDegradeReason::CapabilityLimitsUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5EmbeddedOriginStateNextAction::ViewCapabilityLimits
    );
}

#[test]
fn bar_empty_capability_degrades_ac1() {
    let mut input = clean_bar_input();
    input.capability_limits = vec![];
    let resolved = resolve_embedded_origin_bar(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedOriginBarDegradeReason::CapabilityLimitsUnstated)
    );
}

#[test]
fn bar_imitates_native_degrades_ac2() {
    let mut input = clean_bar_input();
    input.imitates_native_permission_ui = true;
    let resolved = resolve_embedded_origin_bar(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.imitates_native_permission_ui);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedOriginBarDegradeReason::ImitatesNativePermissionUi)
    );
    assert_eq!(
        resolved.next_action,
        M5EmbeddedOriginStateNextAction::ReviewPermissions
    );
}

#[test]
fn bar_stale_freshness_never_reads_fresh_first_party() {
    let mut input = clean_bar_input();
    input.owner_class = WebviewOwnerClass::FirstPartyEmbedded;
    input.origin_disclosure = OriginDisclosureClass::FirstPartyOrigin;
    input.freshness = M5EmbeddedFreshnessState::StaleSnapshot;
    let resolved = resolve_embedded_origin_bar(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::StaleSnapshot
    );
}

#[test]
fn bar_empty_id_and_forbidden_material_error() {
    let mut input = clean_bar_input();
    input.bar_id = "".to_owned();
    assert_eq!(
        resolve_embedded_origin_bar(input).unwrap_err(),
        M5EmbeddedOriginStateResolutionError::EmptyBarId
    );

    let mut input = clean_bar_input();
    input.publisher = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_embedded_origin_bar(input).unwrap_err(),
        M5EmbeddedOriginStateResolutionError::ForbiddenMaterial
    );
}

#[test]
fn panel_clean_names_state_and_boundary() {
    let resolved = resolve_embedded_state_panel(clean_panel_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.imitates_native_permission_ui);
    assert!(!resolved.renders_blocked_as_fresh);
    assert_eq!(resolved.state_class, "policy_blocked");
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::ProviderBlocked
    );
}

#[test]
fn panel_cross_origin_is_capability_limited() {
    let mut input = clean_panel_input();
    input.state_class = M5EmbeddedStateClass::CrossOriginLimited;
    let resolved = resolve_embedded_state_panel(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::CapabilityLimited
    );
}

#[test]
fn panel_state_unknown_degrades() {
    let mut input = clean_panel_input();
    input.state_class = M5EmbeddedStateClass::StateUnknown;
    let resolved = resolve_embedded_state_panel(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedStatePanelDegradeReason::StateClassUnstated)
    );
}

#[test]
fn panel_not_explained_degrades() {
    let mut input = clean_panel_input();
    input.state_explained = false;
    let resolved = resolve_embedded_state_panel(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedStatePanelDegradeReason::StateNotExplained)
    );
}

#[test]
fn panel_forked_vocabulary_degrades() {
    let mut input = clean_panel_input();
    input.severity_and_support_boundary_shared = false;
    let resolved = resolve_embedded_state_panel(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedStatePanelDegradeReason::SupportBoundaryOrSeverityUnstated)
    );
}

#[test]
fn panel_blocked_shown_as_fresh_degrades() {
    let mut input = clean_panel_input();
    input.state_class = M5EmbeddedStateClass::StaleSnapshot;
    input.shown_as_fresh_first_party = true;
    let resolved = resolve_embedded_state_panel(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.renders_blocked_as_fresh);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedStatePanelDegradeReason::BlockedShownAsFresh)
    );
    assert_eq!(
        resolved.next_action,
        M5EmbeddedOriginStateNextAction::OpenInBrowser
    );
}

#[test]
fn panel_live_shown_as_fresh_is_not_blocked() {
    let mut input = clean_panel_input();
    input.state_class = M5EmbeddedStateClass::LiveHealthy;
    input.owner_class = WebviewOwnerClass::FirstPartyEmbedded;
    input.shown_as_fresh_first_party = true;
    let resolved = resolve_embedded_state_panel(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.renders_blocked_as_fresh);
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::LiveFirstPartyLocal
    );
}

#[test]
fn panel_imitates_native_degrades_ac2() {
    let mut input = clean_panel_input();
    input.state_class = M5EmbeddedStateClass::LiveHealthy;
    input.imitates_native_permission_ui = true;
    let resolved = resolve_embedded_state_panel(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.imitates_native_permission_ui);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmbeddedStatePanelDegradeReason::ImitatesNativePermissionUi)
    );
}

#[test]
fn panel_empty_id_and_forbidden_material_error() {
    let mut input = clean_panel_input();
    input.panel_id = "   ".to_owned();
    assert_eq!(
        resolve_embedded_state_panel(input).unwrap_err(),
        M5EmbeddedOriginStateResolutionError::EmptyPanelId
    );

    let mut input = clean_panel_input();
    input.panel_id = "panel://leak".to_owned();
    assert_eq!(
        resolve_embedded_state_panel(input).unwrap_err(),
        M5EmbeddedOriginStateResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_embedded_origin_state_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.vocabulary_set.state_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5EmbeddedOriginStateAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5EmbeddedOriginStateExportField::BoundaryDispositions);
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.controls_rows[0]
        .embedded_state_panel_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    // Force a clean panel to also read as imitating native UI — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.embedded_state_panel_examples[0].degrade_reason = None;
    row.embedded_state_panel_examples[0].imitates_native_permission_ui = true;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_embedded_origin_state_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.masquerades_as_native_approval_chrome = true,
            1 => row.hides_owner_origin_or_handoff_in_menus_only = true,
            2 => row.renders_stale_or_blocked_as_fresh_first_party_truth = true,
            _ => row.embeds_high_risk_approval_without_native_step_up = true,
        }
        assert!(packet
            .validate()
            .contains(&M5EmbeddedOriginStateControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_undisclosed_capability_example_removed() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    for row in &mut packet.controls_rows {
        row.embedded_origin_bar_examples.retain(|ex| {
            ex.degrade_reason != Some(M5EmbeddedOriginBarDegradeReason::CapabilityLimitsUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::Ac1NotProven));
}

#[test]
fn ac1_not_proven_when_a_required_state_uncovered() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    // Drop every clean cross-origin-limited panel so the required state coverage breaks.
    for row in &mut packet.controls_rows {
        row.embedded_state_panel_examples
            .retain(|ex| !(ex.is_clean() && ex.state_class == "cross_origin_limited"));
    }
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_bar_imitation_example_removed() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    for row in &mut packet.controls_rows {
        row.embedded_origin_bar_examples.retain(|ex| {
            ex.degrade_reason != Some(M5EmbeddedOriginBarDegradeReason::ImitatesNativePermissionUi)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::Ac2NotProven));
}

#[test]
fn ac2_not_proven_when_panel_imitation_example_removed() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    for row in &mut packet.controls_rows {
        row.embedded_state_panel_examples.retain(|ex| {
            ex.degrade_reason != Some(M5EmbeddedStatePanelDegradeReason::ImitatesNativePermissionUi)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.governance_review.no_surface_imitates_native_ui = false;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet
        .consumer_projection
        .support_export_reads_single_boundary_source = false;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EmbeddedOriginStateControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_embedded_origin_state_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_embedded_origin_state_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_embedded_origin_state_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_embedded_origin_state_controls_export()
        .expect("checked M5 embedded-origin-state controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_EMBEDDED_ORIGIN_STATE_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_embedded_origin_state_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_embedded_origin_state_controls_embedded_webview_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EmbeddedConsumerSurface::EmbeddedWebviewUi)
        .unwrap();
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Beta);

    let preview = seeded_m5_embedded_origin_state_controls_remote_dashboard_preview_narrowed();
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
    let beta: M5EmbeddedOriginStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-embedded-origin-bar-state-panel-controls/embedded_webview_beta_narrowed.json"
    )))
    .expect("embedded-webview fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_embedded_origin_state_controls_embedded_webview_beta_narrowed()
    );

    let preview: M5EmbeddedOriginStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-embedded-origin-bar-state-panel-controls/remote_dashboard_preview_narrowed.json"
    )))
    .expect("remote-dashboard fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_embedded_origin_state_controls_remote_dashboard_preview_narrowed()
    );
}
