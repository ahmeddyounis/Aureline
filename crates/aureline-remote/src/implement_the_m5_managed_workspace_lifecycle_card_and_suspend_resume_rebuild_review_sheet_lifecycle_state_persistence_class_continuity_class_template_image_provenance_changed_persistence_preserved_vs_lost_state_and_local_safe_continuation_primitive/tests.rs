use super::*;

use crate::managed_workspace_lifecycle::{
    ContinuityClass, ExpiryClass, LifecycleStateClass, PersistenceClass, ProvenanceClass,
    RecoveryOptionClass,
};

fn clean_card_input() -> M5ManagedWorkspaceLifecycleCardResolutionInput {
    M5ManagedWorkspaceLifecycleCardResolutionInput {
        card_id: "lifecycle-card:test".to_owned(),
        workspace_label: "web-frontend@managed-ws".to_owned(),
        lifecycle_state: LifecycleStateClass::Ready,
        state_disclosed: true,
        persistence_class: PersistenceClass::PersistentVolume,
        persistence_disclosed: true,
        continuity_class: ContinuityClass::ExactContinuity,
        continuity_disclosed: true,
        expiry_class: ExpiryClass::IdleWindow,
        expiry_disclosed: true,
        recovery_options: vec![],
        local_safe_offered: false,
        material_change_present: false,
        proof_fresh: true,
    }
}

fn clean_sheet_input() -> M5SuspendResumeRebuildReviewSheetResolutionInput {
    M5SuspendResumeRebuildReviewSheetResolutionInput {
        sheet_id: "review-sheet:test".to_owned(),
        workspace_label: "web-frontend@managed-ws".to_owned(),
        action: M5ManagedWorkspaceAction::Resume,
        action_disclosed: true,
        provenance_class: ProvenanceClass::PinnedDigest,
        provenance_disclosed: true,
        persistence_class: PersistenceClass::PersistentVolume,
        persistence_changed: false,
        persistence_change_disclosed: true,
        continuity_class: ContinuityClass::ExactContinuity,
        preserved_state_disclosed: true,
        lost_state_disclosed: true,
        consequences_disclosed: true,
        shown_before_commit: true,
        caveats: vec![],
        material_change_present: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_managed_lifecycle_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_MANAGED_LIFECYCLE_CONTROLS_PACKET_ID);
}

#[test]
fn card_ready_names_full_lifecycle() {
    let resolved = resolve_managed_workspace_lifecycle_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.state_disclosed);
    assert!(!resolved.misrepresents_continuity_or_local_safe());
    assert_eq!(resolved.lifecycle_state, "ready");
    assert_eq!(resolved.persistence_class, "persistent_volume");
    assert!(!resolved.is_outage_state);
    assert_eq!(
        resolved.next_action,
        M5ManagedLifecycleNextAction::NoActionNeeded
    );
}

#[test]
fn card_clean_examples_cover_every_lifecycle_state() {
    let packet = seeded_m5_managed_lifecycle_controls();
    let clean_states: BTreeSet<&str> = packet
        .controls_rows
        .iter()
        .flat_map(|row| row.lifecycle_card_examples.iter())
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.lifecycle_state.as_str())
        .collect();
    for state in BOUND_LIFECYCLE_STATES {
        assert!(
            clean_states.contains(state.as_str()),
            "missing clean state {}",
            state.as_str()
        );
    }
}

#[test]
fn card_state_undisclosed_degrades_ac1() {
    let mut input = clean_card_input();
    input.state_disclosed = false;
    let resolved = resolve_managed_workspace_lifecycle_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::LifecycleStateUnstated)
    );
}

#[test]
fn card_continuity_undisclosed_degrades() {
    let mut input = clean_card_input();
    input.continuity_disclosed = false;
    let resolved = resolve_managed_workspace_lifecycle_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::ContinuityUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5ManagedLifecycleNextAction::ReviewBeforeCommit
    );
}

#[test]
fn card_exact_continuity_over_material_change_degrades() {
    let mut input = clean_card_input();
    input.lifecycle_state = LifecycleStateClass::Resumed;
    input.material_change_present = true;
    input.continuity_class = ContinuityClass::ExactContinuity;
    input.expiry_class = ExpiryClass::None;
    let resolved = resolve_managed_workspace_lifecycle_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.implies_exact_continuity_after_material_change);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::ExactContinuityOverclaimed)
    );
}

#[test]
fn card_expiry_undisclosed_degrades() {
    let mut input = clean_card_input();
    input.lifecycle_state = LifecycleStateClass::Suspended;
    input.expiry_class = ExpiryClass::HibernationWindow;
    input.expiry_disclosed = false;
    input.recovery_options = vec![RecoveryOptionClass::Resume];
    let resolved = resolve_managed_workspace_lifecycle_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::ExpiryTimingUnstated)
    );
}

#[test]
fn card_outage_hiding_local_safe_degrades() {
    let mut input = clean_card_input();
    input.lifecycle_state = LifecycleStateClass::Expired;
    input.persistence_class = PersistenceClass::LocalMirror;
    input.continuity_class = ContinuityClass::LocalSafeOnly;
    input.material_change_present = true;
    input.expiry_class = ExpiryClass::HardDeadline;
    input.local_safe_offered = false;
    input.recovery_options = vec![RecoveryOptionClass::Recreate];
    let resolved = resolve_managed_workspace_lifecycle_card(input).unwrap();
    assert!(resolved.is_outage_state);
    assert!(resolved.hides_local_safe_continuation);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::LocalSafeContinuationUnavailable)
    );
    assert_eq!(
        resolved.next_action,
        M5ManagedLifecycleNextAction::ContinueLocalSafe
    );
}

#[test]
fn card_outage_offering_local_safe_is_clean() {
    let mut input = clean_card_input();
    input.lifecycle_state = LifecycleStateClass::Reconnecting;
    input.expiry_class = ExpiryClass::ControlPlaneOutage;
    input.local_safe_offered = true;
    input.recovery_options = vec![
        RecoveryOptionClass::Reconnect,
        RecoveryOptionClass::LocalSafeContinue,
    ];
    let resolved = resolve_managed_workspace_lifecycle_card(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_outage_state);
    assert_eq!(
        resolved.next_action,
        M5ManagedLifecycleNextAction::ContinueLocalSafe
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "".to_owned();
    assert_eq!(
        resolve_managed_workspace_lifecycle_card(input).unwrap_err(),
        M5ManagedLifecycleResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.workspace_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_managed_workspace_lifecycle_card(input).unwrap_err(),
        M5ManagedLifecycleResolutionError::ForbiddenMaterial
    );
}

#[test]
fn sheet_resume_is_clean() {
    let resolved = resolve_suspend_resume_rebuild_review_sheet(clean_sheet_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.shown_before_commit);
    assert!(!resolved.misrepresents_review());
    assert_eq!(resolved.action, "resume");
    assert_eq!(
        resolved.next_action,
        M5ManagedLifecycleNextAction::NoActionNeeded
    );
}

#[test]
fn sheet_provenance_unstated_degrades() {
    let mut input = clean_sheet_input();
    input.provenance_disclosed = false;
    let resolved = resolve_suspend_resume_rebuild_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ProvenanceUnstated)
    );
}

#[test]
fn sheet_persistence_change_hidden_degrades() {
    let mut input = clean_sheet_input();
    input.action = M5ManagedWorkspaceAction::Rebuild;
    input.persistence_class = PersistenceClass::RebuiltFresh;
    input.persistence_changed = true;
    input.persistence_change_disclosed = false;
    let resolved = resolve_suspend_resume_rebuild_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::PersistenceChangeHidden)
    );
}

#[test]
fn sheet_preserved_vs_lost_unstated_degrades() {
    let mut input = clean_sheet_input();
    input.lost_state_disclosed = false;
    let resolved = resolve_suspend_resume_rebuild_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::PreservedVsLostStateUnstated)
    );
}

#[test]
fn sheet_consequences_unstated_degrades() {
    let mut input = clean_sheet_input();
    input.consequences_disclosed = false;
    let resolved = resolve_suspend_resume_rebuild_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ConsequencesUnstated)
    );
}

#[test]
fn sheet_exact_continuity_overclaim_degrades() {
    let mut input = clean_sheet_input();
    input.material_change_present = true;
    input.continuity_class = ContinuityClass::ExactContinuity;
    let resolved = resolve_suspend_resume_rebuild_review_sheet(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.overclaims_exact_continuity);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ExactContinuityOverclaimed)
    );
}

#[test]
fn sheet_shown_after_commit_degrades_ac2() {
    let mut input = clean_sheet_input();
    input.shown_before_commit = false;
    let resolved = resolve_suspend_resume_rebuild_review_sheet(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.shown_after_the_fact);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ReviewShownAfterCommit)
    );
}

#[test]
fn sheet_action_unstated_degrades() {
    let mut input = clean_sheet_input();
    input.action_disclosed = false;
    let resolved = resolve_suspend_resume_rebuild_review_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ActionClassUnstated)
    );
}

#[test]
fn sheet_rebuild_with_material_change_is_clean() {
    let mut input = clean_sheet_input();
    input.action = M5ManagedWorkspaceAction::Rebuild;
    input.provenance_class = ProvenanceClass::SuccessorImage;
    input.persistence_class = PersistenceClass::RebuiltFresh;
    input.persistence_changed = true;
    input.continuity_class = ContinuityClass::MaterialChange;
    input.material_change_present = true;
    let resolved = resolve_suspend_resume_rebuild_review_sheet(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.overclaims_exact_continuity);
    assert!(resolved.persistence_changed);
}

#[test]
fn sheet_empty_id_and_forbidden_material_error() {
    let mut input = clean_sheet_input();
    input.sheet_id = "   ".to_owned();
    assert_eq!(
        resolve_suspend_resume_rebuild_review_sheet(input).unwrap_err(),
        M5ManagedLifecycleResolutionError::EmptySheetId
    );

    let mut input = clean_sheet_input();
    input.workspace_label = "ssh://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_suspend_resume_rebuild_review_sheet(input).unwrap_err(),
        M5ManagedLifecycleResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_managed_lifecycle_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.vocabulary_set.lifecycle_states.pop();
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ManagedLifecycleAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5ManagedLifecycleExportField::LifecycleStates);
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.controls_rows[0].review_sheet_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    // Force a clean sheet to also read as overclaiming continuity — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.review_sheet_examples[0].degrade_reason = None;
    row.review_sheet_examples[0].overclaims_exact_continuity = true;
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_managed_lifecycle_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.implies_exact_continuity_after_material_change = true,
            1 => row.hides_local_safe_or_companion_handoff_in_overflow_only = true,
            2 => row.review_sheet_appears_after_the_fact = true,
            _ => row.conceals_lifecycle_or_continuity_in_generic_status_wording = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ManagedLifecycleControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_a_state_uncovered() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    // Drop every clean local-safe-continuation card so the required state coverage breaks.
    for row in &mut packet.controls_rows {
        row.lifecycle_card_examples
            .retain(|ex| !(ex.is_clean() && ex.lifecycle_state == "local_safe_continuation"));
    }
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::Ac1NotProven));
}

#[test]
fn ac1_not_proven_when_local_safe_example_removed() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    for row in &mut packet.controls_rows {
        row.lifecycle_card_examples.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5ManagedWorkspaceLifecycleCardDegradeReason::LocalSafeContinuationUnavailable,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_after_commit_example_removed() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    for row in &mut packet.controls_rows {
        row.review_sheet_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ReviewShownAfterCommit)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::Ac2NotProven));
}

#[test]
fn ac2_not_proven_when_overclaim_example_removed() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    for row in &mut packet.controls_rows {
        row.review_sheet_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ExactContinuityOverclaimed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet
        .governance_review
        .material_change_never_implies_exact_continuity = false;
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet
        .consumer_projection
        .companion_surfaces_reuse_lifecycle_cards_and_review_language = false;
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ManagedLifecycleControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_managed_lifecycle_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_managed_lifecycle_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_managed_lifecycle_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_managed_lifecycle_controls_export()
        .expect("checked M5 managed-lifecycle controls export validates");
    assert_eq!(from_disk.packet_id, M5_MANAGED_LIFECYCLE_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_managed_lifecycle_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_managed_lifecycle_controls_lifecycle_card_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildRemoteConsumerSurface::RunTestDebugUi)
        .unwrap();
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Beta);

    let preview = seeded_m5_managed_lifecycle_controls_review_sheet_preview_narrowed();
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
    let beta: M5ManagedLifecycleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls/lifecycle_card_beta_narrowed.json"
    )))
    .expect("lifecycle-card fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_managed_lifecycle_controls_lifecycle_card_beta_narrowed()
    );

    let preview: M5ManagedLifecycleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls/review_sheet_preview_narrowed.json"
    )))
    .expect("review-sheet fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_managed_lifecycle_controls_review_sheet_preview_narrowed()
    );
}
