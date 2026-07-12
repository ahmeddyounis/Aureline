use super::*;

fn clean_band_input() -> M5ActivationBudgetBandResolutionInput {
    M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        cold_start_evidence: Some(M5ActivationCostLevel::Low),
        warm_start_evidence: Some(M5ActivationCostLevel::Low),
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    }
}

fn clean_card_input() -> M5InstalledStateDiagnosticsCardResolutionInput {
    M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:test".to_owned(),
        artifact_identity: "test-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::FileSystemRead],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_activation_diagnostics_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_ACTIVATION_DIAGNOSTICS_CONTROLS_PACKET_ID
    );
}

#[test]
fn band_clean_names_class_and_is_legible() {
    let resolved = resolve_activation_budget_band(clean_band_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fully_legible);
    assert!(!resolved.is_over_budget);
    assert!(resolved.has_cold_warm_evidence);
    assert_eq!(resolved.band_class, "low");
    assert!(!resolved.presents_over_budget_as_cost_free);
    assert!(!resolved.leaves_stale_certified_overclaim);
    assert_eq!(
        resolved.next_action,
        M5ActivationDiagnosticsNextAction::NoActionNeeded
    );
}

#[test]
fn band_near_budget_floors_class_at_medium() {
    let mut input = clean_band_input();
    input.certified_or_supported_claimed = false;
    input.budget_state = M5ActivationBudgetBandState::NearBudget;
    let resolved = resolve_activation_budget_band(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(resolved.band_class, "medium");
}

#[test]
fn band_within_budget_high_warm_reads_high() {
    let mut input = clean_band_input();
    input.certified_or_supported_claimed = false;
    input.warm_start_evidence = Some(M5ActivationCostLevel::High);
    let resolved = resolve_activation_budget_band(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(resolved.band_class, "high");
}

#[test]
fn band_over_budget_with_evidence_is_clean_over_budget_class() {
    let mut input = clean_band_input();
    input.certified_or_supported_claimed = false;
    input.budget_state = M5ActivationBudgetBandState::OverBudget;
    input.cold_start_evidence = Some(M5ActivationCostLevel::High);
    input.warm_start_evidence = Some(M5ActivationCostLevel::High);
    let resolved = resolve_activation_budget_band(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_over_budget);
    assert_eq!(resolved.band_class, "over_budget");
}

#[test]
fn band_unknown_state_degrades() {
    let mut input = clean_band_input();
    input.certified_or_supported_claimed = false;
    input.budget_state = M5ActivationBudgetBandState::BudgetUnknown;
    let resolved = resolve_activation_budget_band(input).unwrap();
    assert_eq!(resolved.band_class, "unknown");
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ActivationBudgetBandDegradeReason::BudgetBandUnresolved)
    );
}

#[test]
fn band_over_budget_cost_free_degrades() {
    let mut input = clean_band_input();
    input.certified_or_supported_claimed = false;
    input.budget_state = M5ActivationBudgetBandState::OverBudget;
    input.reads_over_budget_as_cost_free = true;
    let resolved = resolve_activation_budget_band(input).unwrap();
    assert!(resolved.presents_over_budget_as_cost_free);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ActivationBudgetBandDegradeReason::OverBudgetShownAsCostFree)
    );
    assert_eq!(
        resolved.next_action,
        M5ActivationDiagnosticsNextAction::ReviewActivationBudget
    );
}

#[test]
fn band_evidence_missing_after_degradation_degrades() {
    let mut input = clean_band_input();
    input.certified_or_supported_claimed = false;
    input.budget_state = M5ActivationBudgetBandState::Throttled;
    input.cold_start_evidence = None;
    input.warm_start_evidence = None;
    let resolved = resolve_activation_budget_band(input).unwrap();
    assert!(!resolved.has_cold_warm_evidence);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ActivationBudgetBandDegradeReason::ActivationEvidenceMissingAfterDegradation)
    );
    assert_eq!(
        resolved.next_action,
        M5ActivationDiagnosticsNextAction::ReviewColdWarmEvidence
    );
}

#[test]
fn band_stale_certified_overclaim_degrades() {
    let mut input = clean_band_input();
    input.certified_or_supported_claimed = true;
    input.evidence_fresh = false;
    let resolved = resolve_activation_budget_band(input).unwrap();
    assert!(resolved.leaves_stale_certified_overclaim);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ActivationBudgetBandDegradeReason::StaleEvidenceCertifiedOverclaim)
    );
    assert_eq!(
        resolved.next_action,
        M5ActivationDiagnosticsNextAction::ReviewEvidenceFreshness
    );
}

#[test]
fn band_empty_id_and_forbidden_material_error() {
    let mut input = clean_band_input();
    input.band_id = "".to_owned();
    assert_eq!(
        resolve_activation_budget_band(input).unwrap_err(),
        M5ActivationDiagnosticsResolutionError::EmptyBandId
    );

    let mut input = clean_band_input();
    input.artifact_identity = "see internal://notes".to_owned();
    assert_eq!(
        resolve_activation_budget_band(input).unwrap_err(),
        M5ActivationDiagnosticsResolutionError::ForbiddenMaterial
    );
}

#[test]
fn card_clean_names_triggers_and_capabilities() {
    let resolved = resolve_installed_state_diagnostics_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fully_legible);
    assert!(!resolved.is_actionable);
    assert!(resolved.offers_disable);
    assert!(resolved.offers_retry);
    assert!(resolved.disable_retry_parity);
    assert!(!resolved.hides_quarantine_history);
    assert_eq!(resolved.activation_triggers, vec!["on_startup"]);
    assert_eq!(resolved.exercised_capabilities, vec!["file_system_read"]);
}

#[test]
fn card_throttled_with_reason_and_parity_is_clean() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.budget_state = M5ActivationBudgetBandState::Throttled;
    input.throttle_quarantine_reason = Some(M5ThrottleQuarantineReason::ActivationBudgetExceeded);
    input.remediation_actions = vec![
        M5DiagnosticsRemediationAction::RetryActivation,
        M5DiagnosticsRemediationAction::DisableWorkspace,
    ];
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_actionable);
    assert!(resolved.disable_retry_parity);
    assert_eq!(
        resolved.throttle_quarantine_reason,
        Some("activation_budget_exceeded".to_owned())
    );
}

#[test]
fn card_budget_unresolved_degrades() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.budget_state = M5ActivationBudgetBandState::BudgetUnknown;
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstalledStateDiagnosticsCardDegradeReason::BudgetBandUnresolved)
    );
}

#[test]
fn card_quarantine_unresolved_degrades() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.quarantine_state = M5QuarantineState::QuarantineUnknown;
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstalledStateDiagnosticsCardDegradeReason::QuarantineStateUnresolved)
    );
}

#[test]
fn card_triggers_unstated_degrades() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.activation_triggers = vec![];
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstalledStateDiagnosticsCardDegradeReason::ActivationTriggersUnstated)
    );
}

#[test]
fn card_capabilities_unstated_degrades() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.exercised_capabilities = vec![];
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstalledStateDiagnosticsCardDegradeReason::ExercisedCapabilitiesUnstated)
    );
}

#[test]
fn card_quarantine_hidden_degrades() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.quarantine_state = M5QuarantineState::QuarantinedActive;
    input.throttle_quarantine_reason = Some(M5ThrottleQuarantineReason::ManualQuarantine);
    input.reads_quarantine_as_healthy = true;
    input.remediation_actions = vec![
        M5DiagnosticsRemediationAction::RetryActivation,
        M5DiagnosticsRemediationAction::DisableWorkspace,
    ];
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert!(resolved.hides_quarantine_history);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstalledStateDiagnosticsCardDegradeReason::QuarantineHistoryHidden)
    );
    assert_eq!(
        resolved.next_action,
        M5ActivationDiagnosticsNextAction::ReviewQuarantineReason
    );
}

#[test]
fn card_actionable_without_reason_degrades() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.budget_state = M5ActivationBudgetBandState::Throttled;
    input.throttle_quarantine_reason = None;
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert!(resolved.is_actionable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstalledStateDiagnosticsCardDegradeReason::ThrottleQuarantineReasonMissing)
    );
}

#[test]
fn card_actionable_without_disable_retry_parity_degrades() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.quarantine_state = M5QuarantineState::QuarantinedActive;
    input.throttle_quarantine_reason = Some(M5ThrottleQuarantineReason::PolicyViolation);
    input.remediation_actions = vec![
        M5DiagnosticsRemediationAction::RetryActivation,
        M5DiagnosticsRemediationAction::ViewLogs,
    ];
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert!(resolved.is_actionable);
    assert!(resolved.offers_retry);
    assert!(!resolved.offers_disable);
    assert!(!resolved.disable_retry_parity);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstalledStateDiagnosticsCardDegradeReason::DisableRetryActionsMissing)
    );
    assert_eq!(
        resolved.next_action,
        M5ActivationDiagnosticsNextAction::ReviewDisableRetryActions
    );
}

#[test]
fn card_released_from_quarantine_healthy_is_clean() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = false;
    input.quarantine_state = M5QuarantineState::ReleasedFromQuarantine;
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.is_actionable);
}

#[test]
fn card_stale_certified_overclaim_degrades() {
    let mut input = clean_card_input();
    input.certified_or_supported_claimed = true;
    input.evidence_fresh = false;
    let resolved = resolve_installed_state_diagnostics_card(input).unwrap();
    assert!(resolved.leaves_stale_certified_overclaim);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5InstalledStateDiagnosticsCardDegradeReason::StaleEvidenceCertifiedOverclaim)
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "   ".to_owned();
    assert_eq!(
        resolve_installed_state_diagnostics_card(input).unwrap_err(),
        M5ActivationDiagnosticsResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.artifact_identity = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_installed_state_diagnostics_card(input).unwrap_err(),
        M5ActivationDiagnosticsResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_activation_diagnostics_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.vocabulary_set.band_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ActivationDiagnosticsAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5ActivationDiagnosticsExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.controls_rows[0]
        .installed_state_diagnostics_card_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_band_example_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    let row = &mut packet.controls_rows[0];
    row.activation_budget_band_examples[0].degrade_reason = None;
    row.activation_budget_band_examples[0].leaves_stale_certified_overclaim = true;
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::DishonestExample));
}

#[test]
fn dishonest_clean_card_example_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    let row = &mut packet.controls_rows[0];
    row.installed_state_diagnostics_card_examples[0].degrade_reason = None;
    row.installed_state_diagnostics_card_examples[0].hides_quarantine_history = true;
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_activation_diagnostics_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.hides_activation_cost_or_over_budget_band = true,
            1 => row.hides_throttling_or_quarantine_reason = true,
            2 => row.collapses_disable_and_retry_into_generic_action = true,
            _ => row.leaves_stale_evidence_certified_or_supported = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ActivationDiagnosticsControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn budget_legibility_not_proven_when_over_budget_evidence_band_removed() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    for row in &mut packet.controls_rows {
        row.activation_budget_band_examples
            .retain(|ex| !(ex.is_clean() && ex.is_over_budget && ex.has_cold_warm_evidence));
    }
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::BudgetLegibilityNotProven));
}

#[test]
fn budget_legibility_not_proven_when_evidence_missing_band_removed() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    for row in &mut packet.controls_rows {
        row.activation_budget_band_examples.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5ActivationBudgetBandDegradeReason::ActivationEvidenceMissingAfterDegradation,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::BudgetLegibilityNotProven));
}

#[test]
fn quarantine_reason_and_disable_retry_not_proven_when_disable_retry_example_removed() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    for row in &mut packet.controls_rows {
        row.installed_state_diagnostics_card_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5InstalledStateDiagnosticsCardDegradeReason::DisableRetryActionsMissing)
        });
    }
    assert!(packet.validate().contains(
        &M5ActivationDiagnosticsControlsViolation::QuarantineReasonAndDisableRetryNotProven
    ));
}

#[test]
fn quarantine_reason_and_disable_retry_not_proven_when_quarantine_hidden_removed() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    for row in &mut packet.controls_rows {
        row.installed_state_diagnostics_card_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5InstalledStateDiagnosticsCardDegradeReason::QuarantineHistoryHidden)
        });
    }
    assert!(packet.validate().contains(
        &M5ActivationDiagnosticsControlsViolation::QuarantineReasonAndDisableRetryNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.governance_review.disable_retry_pair_always_intact = false;
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ActivationDiagnosticsControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_activation_diagnostics_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_activation_diagnostics_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_activation_diagnostics_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_activation_diagnostics_controls_export()
        .expect("checked M5 activation-diagnostics controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_ACTIVATION_DIAGNOSTICS_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_activation_diagnostics_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_activation_diagnostics_controls_marketplace_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5MarketplaceInstallConsumerSurface::MarketplaceUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Beta
    );

    let preview = seeded_m5_activation_diagnostics_controls_installed_state_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5MarketplaceInstallConsumerSurface::InstallReviewUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ActivationDiagnosticsControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-activation-budget-band-installed-state-diagnostics-card-controls/marketplace_ui_beta_narrowed.json"
    )))
    .expect("marketplace-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_activation_diagnostics_controls_marketplace_ui_beta_narrowed()
    );

    let preview: M5ActivationDiagnosticsControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-activation-budget-band-installed-state-diagnostics-card-controls/registry_ui_preview_narrowed.json"
    )))
    .expect("install-review fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_activation_diagnostics_controls_installed_state_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_the_two_performance_components() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5MarketplaceInstallComponentFamily::ActivationBudgetBand,
            M5MarketplaceInstallComponentFamily::InstalledStateDiagnosticsCard,
        ]
    );
}
