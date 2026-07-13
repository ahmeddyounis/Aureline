use super::*;

fn clean_toast_input() -> M5ToastResolutionInput {
    M5ToastResolutionInput {
        toast_id: "toast:test".to_owned(),
        toast_label: "Change approved — it is now in the review queue".to_owned(),
        toast_durability: M5ToastDurability::MirroredToActivityCenter,
        disposition: M5DecisionFeedbackDisposition::Success,
        surface_context: M5TransientSurfaceContext::ReviewWorkspace,
        acknowledgement_scope: M5ToastAcknowledgementScope::DurableOutcomeAck,
        backlink_target: M5ToastBacklinkTarget::ReviewQueue,
        acknowledges_transiently: true,
        outcome_matters_after_dismissal: true,
        durable_backlink_present: true,
        bounded_action_present: false,
        action_is_bounded: false,
        avoids_toast_only_truth: true,
        reconstructable_from_export: true,
        proof_fresh: true,
    }
}

fn clean_loading_input() -> M5LoadingStateResolutionInput {
    M5LoadingStateResolutionInput {
        loading_state_id: "loading:test".to_owned(),
        loading_label: "Refreshing the review index — the previous results stay visible".to_owned(),
        loading_fidelity: M5LoadingFidelity::PartialDataRetained,
        disposition: M5DecisionFeedbackDisposition::Pending,
        surface_context: M5TransientSurfaceContext::ReviewWorkspace,
        loading_treatment: M5LoadingTreatment::RetainedPreviousContent,
        readiness_posture: M5LoadingReadinessPosture::PartiallyReady,
        partial_content_available: true,
        partial_content_preserved: true,
        pane_blanked: false,
        overclaims_readiness: false,
        purpose_stated: true,
        reconstructable_from_export: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_toast_loading_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_TOAST_LOADING_CONTROLS_PACKET_ID);
}

#[test]
fn toast_clean_acknowledges_with_durable_backlink() {
    let resolved = resolve_toast(clean_toast_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.acknowledges_without_becoming_only_truth);
    assert!(!resolved.durability_is_toast_only);
    assert_eq!(resolved.toast_durability, "mirrored_to_activity_center");
    assert_eq!(resolved.surface_context, "review_workspace");
    assert_eq!(resolved.backlink_target, "review_queue");
    assert_eq!(
        resolved.next_action,
        M5ToastLoadingNextAction::ReviewAcknowledgedOutcome
    );
}

#[test]
fn toast_label_unstated_degrades() {
    let mut input = clean_toast_input();
    input.toast_label = "   ".to_owned();
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::ToastLabelUnstated)
    );
}

#[test]
fn toast_surface_and_durability_degrade() {
    let mut input = clean_toast_input();
    input.surface_context = M5TransientSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_toast_input();
    input.toast_durability = M5ToastDurability::ToastOnlyTruthDisallowed;
    let resolved = resolve_toast(input).unwrap();
    assert!(resolved.durability_is_toast_only);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ToastDegradeReason::DurabilityToastOnlyDisallowed)
    );
}

#[test]
fn toast_scope_and_transience_degrade() {
    let mut input = clean_toast_input();
    input.acknowledgement_scope = M5ToastAcknowledgementScope::ScopeUnknown;
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::AcknowledgementScopeUnresolved)
    );

    let mut input = clean_toast_input();
    input.acknowledges_transiently = false;
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::AcknowledgementNotShortLived)
    );
}

#[test]
fn toast_backlink_and_target_and_action_degrade() {
    let mut input = clean_toast_input();
    input.durable_backlink_present = false;
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::DurableBacklinkMissing)
    );

    let mut input = clean_toast_input();
    input.backlink_target = M5ToastBacklinkTarget::TargetUnknown;
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::BacklinkTargetUnresolved)
    );

    let mut input = clean_toast_input();
    input.bounded_action_present = true;
    input.action_is_bounded = false;
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::ActionNotBounded)
    );
}

#[test]
fn toast_only_truth_and_reconstructable_degrade() {
    let mut input = clean_toast_input();
    input.avoids_toast_only_truth = false;
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::ToastOnlyTruthUsed)
    );

    let mut input = clean_toast_input();
    input.reconstructable_from_export = false;
    assert_eq!(
        resolve_toast(input).unwrap().degrade_reason,
        Some(M5ToastDegradeReason::NotReconstructableFromExport)
    );
}

#[test]
fn toast_transient_outcome_needs_no_backlink() {
    // A toast whose outcome does not matter after dismissal stays clean without a durable backlink.
    let mut input = clean_toast_input();
    input.outcome_matters_after_dismissal = false;
    input.durable_backlink_present = false;
    input.backlink_target = M5ToastBacklinkTarget::TargetUnknown;
    input.acknowledgement_scope = M5ToastAcknowledgementScope::TransientConfirmation;
    let resolved = resolve_toast(input).unwrap();
    assert!(resolved.is_clean());
}

#[test]
fn toast_empty_id_and_forbidden_material_error() {
    let mut input = clean_toast_input();
    input.toast_id = "".to_owned();
    assert_eq!(
        resolve_toast(input).unwrap_err(),
        M5ToastLoadingResolutionError::EmptyToastId
    );

    let mut input = clean_toast_input();
    input.toast_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_toast(input).unwrap_err(),
        M5ToastLoadingResolutionError::ForbiddenMaterial
    );
}

#[test]
fn loading_clean_preserves_partial_and_readiness() {
    let resolved = resolve_loading_state(clean_loading_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.preserves_partial_and_readiness_honesty);
    assert!(!resolved.fidelity_is_full_screen_spinner);
    assert_eq!(resolved.loading_treatment, "retained_previous_content");
    assert_eq!(resolved.readiness_posture, "partially_ready");
    assert_eq!(
        resolved.next_action,
        M5ToastLoadingNextAction::WaitForReadyContent
    );
}

#[test]
fn loading_label_and_surface_and_fidelity_degrade() {
    let mut input = clean_loading_input();
    input.loading_label = "   ".to_owned();
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::LoadingLabelUnstated)
    );

    let mut input = clean_loading_input();
    input.surface_context = M5TransientSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_loading_input();
    input.loading_fidelity = M5LoadingFidelity::FullScreenSpinnerDisallowed;
    let resolved = resolve_loading_state(input).unwrap();
    assert!(resolved.fidelity_is_full_screen_spinner);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5LoadingStateDegradeReason::FidelityFullScreenSpinnerDisallowed)
    );
}

#[test]
fn loading_treatment_and_posture_degrade() {
    let mut input = clean_loading_input();
    input.loading_treatment = M5LoadingTreatment::TreatmentUnknown;
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::LoadingTreatmentUnresolved)
    );

    let mut input = clean_loading_input();
    input.readiness_posture = M5LoadingReadinessPosture::PostureUnknown;
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::ReadinessPostureUnresolved)
    );
}

#[test]
fn loading_blanked_and_partial_and_overclaim_degrade() {
    let mut input = clean_loading_input();
    input.pane_blanked = true;
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::UsefulPaneBlanked)
    );

    let mut input = clean_loading_input();
    input.partial_content_preserved = false;
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::PartialContentNotPreserved)
    );

    let mut input = clean_loading_input();
    input.overclaims_readiness = true;
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::ReadinessOverclaimed)
    );
}

#[test]
fn loading_purpose_and_reconstructable_degrade() {
    let mut input = clean_loading_input();
    input.purpose_stated = false;
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::PurposeUnstated)
    );

    let mut input = clean_loading_input();
    input.reconstructable_from_export = false;
    assert_eq!(
        resolve_loading_state(input).unwrap().degrade_reason,
        Some(M5LoadingStateDegradeReason::NotReconstructableFromExport)
    );
}

#[test]
fn loading_without_partial_content_stays_clean() {
    // A pane with no partial content available stays clean even without preserving content.
    let mut input = clean_loading_input();
    input.partial_content_available = false;
    input.partial_content_preserved = false;
    input.loading_treatment = M5LoadingTreatment::Skeleton;
    input.readiness_posture = M5LoadingReadinessPosture::WarmingNotReady;
    let resolved = resolve_loading_state(input).unwrap();
    assert!(resolved.is_clean());
}

#[test]
fn loading_empty_id_and_forbidden_material_error() {
    let mut input = clean_loading_input();
    input.loading_state_id = "   ".to_owned();
    assert_eq!(
        resolve_loading_state(input).unwrap_err(),
        M5ToastLoadingResolutionError::EmptyLoadingStateId
    );

    let mut input = clean_loading_input();
    input.loading_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_loading_state(input).unwrap_err(),
        M5ToastLoadingResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_toast_loading_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.vocabulary_set.loading_treatments.pop();
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_TOAST_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ToastLoadingAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5ToastLoadingExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.controls_rows[0].loading_state_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    // Force a clean toast to also read as the only durable truth — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.toast_examples[0].degrade_reason = None;
    row.toast_examples[0].avoids_toast_only_truth = false;
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_toast_loading_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.toast_represents_durable_work_as_toast_only = true,
            1 => row.toast_lacks_durable_backlink_when_outcome_matters = true,
            2 => row.loading_blanks_useful_pane = true,
            _ => row.loading_uses_full_screen_spinner_when_partial_capable = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ToastLoadingControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn durable_backlink_not_proven_when_backlink_missing_example_removed() {
    let mut packet = seeded_m5_toast_loading_controls();
    for row in &mut packet.controls_rows {
        row.toast_examples
            .retain(|ex| ex.degrade_reason != Some(M5ToastDegradeReason::DurableBacklinkMissing));
    }
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::DurableBacklinkWhenOutcomeMattersNotProven));
}

#[test]
fn durable_backlink_not_proven_when_toast_only_example_removed() {
    let mut packet = seeded_m5_toast_loading_controls();
    for row in &mut packet.controls_rows {
        row.toast_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ToastDegradeReason::ToastOnlyTruthUsed)
                && ex.degrade_reason != Some(M5ToastDegradeReason::DurabilityToastOnlyDisallowed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::DurableBacklinkWhenOutcomeMattersNotProven));
}

#[test]
fn partial_readiness_not_proven_when_blanked_example_removed() {
    let mut packet = seeded_m5_toast_loading_controls();
    for row in &mut packet.controls_rows {
        row.loading_state_examples
            .retain(|ex| ex.degrade_reason != Some(M5LoadingStateDegradeReason::UsefulPaneBlanked));
    }
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::PartialContentAndReadinessHonestyNotProven));
}

#[test]
fn partial_readiness_not_proven_when_treatment_coverage_collapses() {
    let mut packet = seeded_m5_toast_loading_controls();
    // Drop every clean example carrying the blocked-waiting treatment so the treatment grammar no longer
    // covers it.
    for row in &mut packet.controls_rows {
        row.loading_state_examples
            .retain(|ex| !(ex.is_clean() && ex.loading_treatment == "blocked_waiting"));
    }
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::PartialContentAndReadinessHonestyNotProven));
}

#[test]
fn reconstructable_not_proven_when_toast_screenshot_example_removed() {
    let mut packet = seeded_m5_toast_loading_controls();
    for row in &mut packet.controls_rows {
        row.toast_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ToastDegradeReason::NotReconstructableFromExport)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::ReconstructableFromExportNotProven));
}

#[test]
fn reconstructable_not_proven_when_loading_screenshot_example_removed() {
    let mut packet = seeded_m5_toast_loading_controls();
    for row in &mut packet.controls_rows {
        row.loading_state_examples.retain(|ex| {
            ex.degrade_reason != Some(M5LoadingStateDegradeReason::NotReconstructableFromExport)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::ReconstructableFromExportNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet
        .governance_review
        .loading_state_never_blanks_useful_pane = false;
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet
        .consumer_projection
        .support_export_reads_single_toast_loading_source = false;
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ToastLoadingControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_toast_loading_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_toast_loading_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_toast_loading_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_toast_loading_controls_export()
        .expect("checked M5 toast / loading-state controls export validates");
    assert_eq!(from_disk.packet_id, M5_TOAST_LOADING_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_toast_loading_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_toast_loading_controls_review_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5DecisionFeedbackConsumerSurface::ReviewUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Beta
    );

    let preview = seeded_m5_toast_loading_controls_support_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5DecisionFeedbackConsumerSurface::SupportUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ToastLoadingControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-toast-and-loading-state-controls/review_ui_beta_narrowed.json"
    )))
    .expect("review-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_toast_loading_controls_review_ui_beta_narrowed()
    );

    let preview: M5ToastLoadingControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-toast-and-loading-state-controls/support_ui_preview_narrowed.json"
    )))
    .expect("support-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_toast_loading_controls_support_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_toast_and_loading_state() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5DecisionFeedbackFamily::Toast,
            M5DecisionFeedbackFamily::LoadingState
        ]
    );
}
