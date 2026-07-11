use super::*;

use crate::managed_workspace_lifecycle::{
    ContinuityClass, ExpiryClass, PersistenceClass, RecoveryOptionClass, TransitionReasonClass,
};

fn clean_banner_input() -> M5WorkspaceExpiryBannerResolutionInput {
    M5WorkspaceExpiryBannerResolutionInput {
        banner_id: "expiry-banner:test".to_owned(),
        workspace_label: "web-frontend@managed-ws".to_owned(),
        expiry_class: ExpiryClass::HardDeadline,
        expiry_disclosed: true,
        triggering_reason: TransitionReasonClass::ExpiryDeadlineReached,
        triggering_source_disclosed: true,
        affected_capabilities: vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Kernels,
        ],
        capabilities_disclosed: true,
        offered_actions: vec![
            M5WorkspaceExpiryAction::ExportBeforeLoss,
            M5WorkspaceExpiryAction::ContinueLocalSafe,
        ],
        renew_reopen_allowed: false,
        continuity_class: ContinuityClass::LocalSafeOnly,
        material_change_present: true,
        proof_fresh: true,
    }
}

fn clean_card_input() -> M5LocalSafeContinuationCardResolutionInput {
    M5LocalSafeContinuationCardResolutionInput {
        card_id: "local-safe-card:test".to_owned(),
        workspace_label: "web-frontend@managed-ws".to_owned(),
        persistence_class: PersistenceClass::LocalMirror,
        continuity_class: ContinuityClass::LocalSafeOnly,
        preserved_context: vec![
            M5PreservedContextClass::WorkingTreeFiles,
            M5PreservedContextClass::UnsavedEdits,
        ],
        preserved_disclosed: true,
        lost_live_state: vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Kernels,
        ],
        lost_disclosed: true,
        next_actions: vec![
            RecoveryOptionClass::LocalSafeContinue,
            RecoveryOptionClass::Reconnect,
        ],
        next_actions_disclosed: true,
        material_change_present: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_expiry_continuation_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EXPIRY_CONTINUATION_CONTROLS_PACKET_ID);
}

#[test]
fn banner_hard_deadline_names_full_expiry() {
    let resolved = resolve_workspace_expiry_banner(clean_banner_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.expiry_disclosed);
    assert!(resolved.triggering_source_disclosed);
    assert!(!resolved.misrepresents_expiry());
    assert_eq!(resolved.expiry_class, "hard_deadline");
    assert_eq!(resolved.triggering_reason, "expiry_deadline_reached");
    assert_eq!(
        resolved.next_action,
        M5ExpiryContinuationNextAction::ExportBeforeLoss
    );
}

#[test]
fn banner_clean_examples_cover_every_expiry_window() {
    let packet = seeded_m5_expiry_continuation_controls();
    let clean_windows: BTreeSet<&str> = packet
        .controls_rows
        .iter()
        .flat_map(|row| row.expiry_banner_examples.iter())
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.expiry_class.as_str())
        .collect();
    for window in EXPIRY_WINDOW_CLASSES {
        assert!(
            clean_windows.contains(window.as_str()),
            "missing clean window {}",
            window.as_str()
        );
    }
}

#[test]
fn banner_timing_undisclosed_degrades_ac1() {
    let mut input = clean_banner_input();
    input.expiry_disclosed = false;
    let resolved = resolve_workspace_expiry_banner(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.appears_as_generic_disconnect_or_silent_loss);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceExpiryBannerDegradeReason::ExpiryTimingUnstated)
    );
}

#[test]
fn banner_source_undisclosed_degrades_ac1() {
    let mut input = clean_banner_input();
    input.triggering_source_disclosed = false;
    let resolved = resolve_workspace_expiry_banner(input).unwrap();
    assert!(resolved.appears_as_generic_disconnect_or_silent_loss);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceExpiryBannerDegradeReason::TriggeringSourceUnstated)
    );
}

#[test]
fn banner_capabilities_undisclosed_degrades() {
    let mut input = clean_banner_input();
    input.capabilities_disclosed = false;
    let resolved = resolve_workspace_expiry_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceExpiryBannerDegradeReason::AffectedCapabilitiesUnstated)
    );
}

#[test]
fn banner_empty_capabilities_degrades() {
    let mut input = clean_banner_input();
    input.affected_capabilities = vec![];
    let resolved = resolve_workspace_expiry_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceExpiryBannerDegradeReason::AffectedCapabilitiesUnstated)
    );
}

#[test]
fn banner_no_action_degrades() {
    let mut input = clean_banner_input();
    input.offered_actions = vec![];
    let resolved = resolve_workspace_expiry_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceExpiryBannerDegradeReason::ExportOrRenewActionMissing)
    );
    assert_eq!(
        resolved.next_action,
        M5ExpiryContinuationNextAction::ExportBeforeLoss
    );
}

#[test]
fn banner_exact_continuity_over_material_change_degrades() {
    let mut input = clean_banner_input();
    input.continuity_class = ContinuityClass::ExactContinuity;
    input.material_change_present = true;
    let resolved = resolve_workspace_expiry_banner(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.implies_exact_continuity_after_material_change);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceExpiryBannerDegradeReason::ExactContinuityOverclaimed)
    );
}

#[test]
fn banner_empty_id_and_forbidden_material_error() {
    let mut input = clean_banner_input();
    input.banner_id = "".to_owned();
    assert_eq!(
        resolve_workspace_expiry_banner(input).unwrap_err(),
        M5ExpiryContinuationResolutionError::EmptyBannerId
    );

    let mut input = clean_banner_input();
    input.workspace_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_workspace_expiry_banner(input).unwrap_err(),
        M5ExpiryContinuationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn card_local_safe_names_preserved_and_lost() {
    let resolved = resolve_local_safe_continuation_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.preserved_disclosed);
    assert!(resolved.lost_disclosed);
    assert!(resolved.offers_continue_locally);
    assert!(!resolved.misrepresents_continuation());
    assert_eq!(
        resolved.next_action,
        M5ExpiryContinuationNextAction::ContinueLocalSafe
    );
}

#[test]
fn card_preserved_undisclosed_degrades_ac2() {
    let mut input = clean_card_input();
    input.preserved_disclosed = false;
    let resolved = resolve_local_safe_continuation_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5LocalSafeContinuationCardDegradeReason::PreservedContextUnstated)
    );
}

#[test]
fn card_lost_undisclosed_degrades_ac2() {
    let mut input = clean_card_input();
    input.lost_disclosed = false;
    let resolved = resolve_local_safe_continuation_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5LocalSafeContinuationCardDegradeReason::LostLiveStateUnstated)
    );
}

#[test]
fn card_next_actions_undisclosed_degrades() {
    let mut input = clean_card_input();
    input.next_actions_disclosed = false;
    let resolved = resolve_local_safe_continuation_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5LocalSafeContinuationCardDegradeReason::NextSafeActionsUnstated)
    );
}

#[test]
fn card_without_local_safe_route_degrades() {
    let mut input = clean_card_input();
    input.next_actions = vec![
        RecoveryOptionClass::Reconnect,
        RecoveryOptionClass::ContactOperator,
    ];
    let resolved = resolve_local_safe_continuation_card(input).unwrap();
    assert!(!resolved.offers_continue_locally);
    assert!(resolved.local_safe_continuation_unavailable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5LocalSafeContinuationCardDegradeReason::LocalSafeContinuationUnavailable)
    );
    assert_eq!(
        resolved.next_action,
        M5ExpiryContinuationNextAction::ContinueLocalSafe
    );
}

#[test]
fn card_exact_continuity_over_material_change_degrades() {
    let mut input = clean_card_input();
    input.continuity_class = ContinuityClass::ExactContinuity;
    input.material_change_present = true;
    let resolved = resolve_local_safe_continuation_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.overclaims_exact_continuity);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5LocalSafeContinuationCardDegradeReason::ExactContinuityOverclaimed)
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "   ".to_owned();
    assert_eq!(
        resolve_local_safe_continuation_card(input).unwrap_err(),
        M5ExpiryContinuationResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.workspace_label = "ssh://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_local_safe_continuation_card(input).unwrap_err(),
        M5ExpiryContinuationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_expiry_continuation_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.vocabulary_set.expiry_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ExpiryContinuationAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5ExpiryContinuationExportField::ExpiryClasses);
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.controls_rows[0].local_safe_card_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    // Force a clean banner to also read as a generic disconnect — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.expiry_banner_examples[0].degrade_reason = None;
    row.expiry_banner_examples[0].appears_as_generic_disconnect_or_silent_loss = true;
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_expiry_continuation_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.implies_exact_continuity_after_material_change = true,
            1 => row.hides_local_safe_or_companion_handoff_in_overflow_only = true,
            2 => row.expiry_appears_as_generic_disconnect_or_silent_loss = true,
            _ => row.conceals_preserved_vs_lost_state_or_next_safe_actions = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ExpiryContinuationControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_a_window_uncovered() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    // Drop every clean control-plane-outage banner so the required window coverage breaks.
    for row in &mut packet.controls_rows {
        row.expiry_banner_examples
            .retain(|ex| !(ex.is_clean() && ex.expiry_class == "control_plane_outage"));
    }
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::Ac1NotProven));
}

#[test]
fn ac1_not_proven_when_export_action_example_removed() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    for row in &mut packet.controls_rows {
        row.expiry_banner_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5WorkspaceExpiryBannerDegradeReason::ExportOrRenewActionMissing)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_local_safe_example_removed() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    for row in &mut packet.controls_rows {
        row.local_safe_card_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5LocalSafeContinuationCardDegradeReason::LocalSafeContinuationUnavailable)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::Ac2NotProven));
}

#[test]
fn ac2_not_proven_when_overclaim_example_removed() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    for row in &mut packet.controls_rows {
        row.local_safe_card_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5LocalSafeContinuationCardDegradeReason::ExactContinuityOverclaimed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet
        .governance_review
        .material_change_never_implies_exact_continuity = false;
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet
        .consumer_projection
        .companion_surfaces_reuse_expiry_banner_and_continuation_cards = false;
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ExpiryContinuationControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_expiry_continuation_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_expiry_continuation_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_expiry_continuation_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_expiry_continuation_controls_export()
        .expect("checked M5 expiry-continuation controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_EXPIRY_CONTINUATION_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_expiry_continuation_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_expiry_continuation_controls_expiry_banner_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildRemoteConsumerSurface::ShellUi)
        .unwrap();
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Beta);

    let preview = seeded_m5_expiry_continuation_controls_local_safe_card_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildRemoteConsumerSurface::PreviewUi)
        .unwrap();
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ExpiryContinuationControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-expiry-banner-local-safe-continuation-card-controls/expiry_banner_beta_narrowed.json"
    )))
    .expect("expiry-banner fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_expiry_continuation_controls_expiry_banner_beta_narrowed()
    );

    let preview: M5ExpiryContinuationControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-expiry-banner-local-safe-continuation-card-controls/local_safe_card_preview_narrowed.json"
    )))
    .expect("local-safe-card fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_expiry_continuation_controls_local_safe_card_preview_narrowed()
    );
}
