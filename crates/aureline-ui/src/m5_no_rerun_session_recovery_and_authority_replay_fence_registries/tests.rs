use super::*;

fn clean_posture_input() -> M5SessionRecoveryPostureEntryResolutionInput {
    M5SessionRecoveryPostureEntryResolutionInput {
        entry_id: "posture:test".to_owned(),
        recovery_target_id: "recovery.acme.warm".to_owned(),
        token_name: "recovery.posture.transcript_restored".to_owned(),
        semantic_role: M5WindowRestoreRole::SessionHydration,
        recovery_posture_state: M5SessionRecoveryPostureState::TranscriptRestored,
        surface_context: M5SessionRecoveryOrchestrationSurfaceContext::ShellSurface,
        resolution_form_coverage: M5SessionRecoveryOrchestrationResolutionForm::ALL.to_vec(),
        session_surface_id: "session-surface.terminal.main".to_owned(),
        session_scope: "session-scope.workspace".to_owned(),
        prior_authority_snapshot: "authority-snapshot.none".to_owned(),
        provenance_class: "provenance.stale-evidence".to_owned(),
        reconnect_plan_ref: "reconnect-plan.none".to_owned(),
        reauthorization_plan_ref: "reauth-plan.none".to_owned(),
        bound_to_registry: true,
        posture_decided_before_replay: true,
        requires_fresh_user_intent: false,
        reauthorization_disclosed_when_required: true,
        proof_fresh: true,
    }
}

fn clean_fence_input() -> M5AuthorityReplayFenceEntryResolutionInput {
    M5AuthorityReplayFenceEntryResolutionInput {
        entry_id: "fence:test".to_owned(),
        guarded_surface_id: "surface.terminal.main".to_owned(),
        token_name: "fence.privileged.no_reacquire".to_owned(),
        semantic_role: M5WindowRestoreRole::SessionHydration,
        fence_class: M5AuthorityReplayFenceClass::PrivilegedTicketOrRemoteAttach,
        surface_context: M5SessionRecoveryOrchestrationSurfaceContext::ShellSurface,
        resolution_form_coverage: M5SessionRecoveryOrchestrationResolutionForm::ALL.to_vec(),
        preserved_surface_role: "surface-role.terminal.main".to_owned(),
        prior_authority_class: "authority-class.none".to_owned(),
        provenance_hint: "provenance.live-session".to_owned(),
        preserves_surface_and_provenance: true,
        fence_is_truthful: true,
        authority_was_held_used: false,
        reauthorization_required_disclosed: false,
        privileged_flow_deferred: false,
        fresh_intent_required_disclosed: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_PACKET_ID
    );
}

#[test]
fn posture_clean_names_meaning_and_is_bound() {
    let resolved = resolve_session_recovery_posture_entry(clean_posture_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.posture_resolves_across_recoveries);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.recovery_posture_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.recovery_posture_state_is_classified);
    assert!(resolved.posture_decided_before_replay);
    assert_eq!(resolved.semantic_role, "session_hydration");
    assert_eq!(resolved.recovery_posture_state, "transcript_restored");
    assert_eq!(
        resolved.canonical_recovery_posture_mode,
        "transcript_restored"
    );
    assert_eq!(resolved.surface_context, "shell_surface");
    assert_eq!(
        resolved.next_action,
        M5SessionRecoveryOrchestrationNextAction::ExpandRecoveryMeaning
    );
}

#[test]
fn posture_token_unstated_degrades() {
    let mut input = clean_posture_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_session_recovery_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::PostureTokenUnstated)
    );
}

#[test]
fn posture_unbound_and_unclassified_degrade() {
    let mut input = clean_posture_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_session_recovery_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::PostureNotBoundToRegistry)
    );

    let mut input = clean_posture_input();
    input.recovery_posture_state = M5SessionRecoveryPostureState::PostureUnclassified;
    assert_eq!(
        resolve_session_recovery_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::RecoveryPostureStateUnclassified)
    );
}

#[test]
fn posture_object_incomplete_and_replay_first_and_form_degrade() {
    // An unstated session scope leaves the resolved object incomplete.
    let mut input = clean_posture_input();
    input.session_scope = "  ".to_owned();
    let resolved = resolve_session_recovery_posture_entry(input).unwrap();
    assert!(!resolved.recovery_posture_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::RecoveryPostureObjectIncomplete)
    );

    // Session-scoped work that replayed before the explicit posture was decided degrades.
    let mut input = clean_posture_input();
    input.posture_decided_before_replay = false;
    assert_eq!(
        resolve_session_recovery_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::ReplayPrecededPosture)
    );

    let mut input = clean_posture_input();
    input.resolution_form_coverage =
        vec![M5SessionRecoveryOrchestrationResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_session_recovery_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn posture_undisclosed_reauth_and_surface_and_proof_degrade() {
    let mut input = clean_posture_input();
    input.recovery_posture_state = M5SessionRecoveryPostureState::ReconnectAvailable;
    input.requires_fresh_user_intent = true;
    input.reauthorization_disclosed_when_required = false;
    // A fresh-intent posture that hides reauthorization first fails posture-precedes-replay.
    assert_eq!(
        resolve_session_recovery_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::ReplayPrecededPosture)
    );

    let mut input = clean_posture_input();
    input.surface_context = M5SessionRecoveryOrchestrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_session_recovery_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_posture_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_session_recovery_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionRecoveryPostureEntryDegradeReason::ProofStale)
    );
}

#[test]
fn posture_empty_id_and_forbidden_material_error() {
    let mut input = clean_posture_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_session_recovery_posture_entry(input).unwrap_err(),
        M5SessionRecoveryOrchestrationResolutionError::EmptyRecoveryPostureEntryId
    );

    let mut input = clean_posture_input();
    input.provenance_class = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_session_recovery_posture_entry(input).unwrap_err(),
        M5SessionRecoveryOrchestrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn posture_precedes_replay_rejects_replay_first() {
    assert!(posture_precedes_replay(
        M5SessionRecoveryPostureState::TranscriptRestored,
        true,
        false,
        true
    ));
    assert!(!posture_precedes_replay(
        M5SessionRecoveryPostureState::TranscriptRestored,
        false,
        false,
        true
    ));
    assert!(posture_precedes_replay(
        M5SessionRecoveryPostureState::ReconnectAvailable,
        true,
        true,
        true
    ));
    assert!(!posture_precedes_replay(
        M5SessionRecoveryPostureState::ReconnectAvailable,
        true,
        true,
        false
    ));
    assert!(!posture_precedes_replay(
        M5SessionRecoveryPostureState::PostureUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn recovery_posture_object_is_complete_requires_all_fields() {
    assert!(recovery_posture_object_is_complete(
        M5SessionRecoveryPostureState::TranscriptRestored,
        "session-surface.terminal.main",
        "session-scope.workspace",
        "authority-snapshot.none",
        "provenance.stale-evidence",
        "reconnect-plan.none",
        "reauth-plan.none",
    ));
    assert!(!recovery_posture_object_is_complete(
        M5SessionRecoveryPostureState::TranscriptRestored,
        "session-surface.terminal.main",
        "  ",
        "authority-snapshot.none",
        "provenance.stale-evidence",
        "reconnect-plan.none",
        "reauth-plan.none",
    ));
    assert!(!recovery_posture_object_is_complete(
        M5SessionRecoveryPostureState::PostureUnclassified,
        "session-surface.terminal.main",
        "session-scope.workspace",
        "authority-snapshot.none",
        "provenance.stale-evidence",
        "reconnect-plan.none",
        "reauth-plan.none",
    ));
}

#[test]
fn fence_clean_holds_no_reacquire() {
    let resolved = resolve_authority_replay_fence_entry(clean_fence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.fence_holds_on_every_surface);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_disclosure_triple);
    assert!(resolved.fence_holds_no_reacquire);
    assert_eq!(resolved.fence_class, "privileged_ticket_or_remote_attach");
    assert_eq!(resolved.surface_context, "shell_surface");
}

#[test]
fn fence_reacquires_and_unclassified_degrade() {
    // A previously held authority that is not reauthorization-disclosed is a silent reacquisition.
    let mut input = clean_fence_input();
    input.authority_was_held_used = true;
    input.reauthorization_required_disclosed = false;
    let resolved = resolve_authority_replay_fence_entry(input).unwrap();
    assert!(!resolved.provides_complete_disclosure_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceReacquiresOrOverclaims)
    );

    // A fence that no longer preserves the surface role and provenance is also a reacquisition / overclaim.
    let mut input = clean_fence_input();
    input.preserves_surface_and_provenance = false;
    assert_eq!(
        resolve_authority_replay_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceReacquiresOrOverclaims)
    );

    // An overclaimed deferred fresh intent is also a reacquisition / overclaim.
    let mut input = clean_fence_input();
    input.privileged_flow_deferred = true;
    input.fresh_intent_required_disclosed = false;
    assert_eq!(
        resolve_authority_replay_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceReacquiresOrOverclaims)
    );

    let mut input = clean_fence_input();
    input.fence_class = M5AuthorityReplayFenceClass::FenceClassUnclassified;
    assert_eq!(
        resolve_authority_replay_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceClassUnclassified)
    );
}

#[test]
fn fence_form_and_surface_and_id_and_material() {
    let mut input = clean_fence_input();
    input.resolution_form_coverage =
        vec![M5SessionRecoveryOrchestrationResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_authority_replay_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AuthorityReplayFenceEntryDegradeReason::FenceFormCoverageIncomplete)
    );

    let mut input = clean_fence_input();
    input.surface_context = M5SessionRecoveryOrchestrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_authority_replay_fence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AuthorityReplayFenceEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_fence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_authority_replay_fence_entry(input).unwrap_err(),
        M5SessionRecoveryOrchestrationResolutionError::EmptyAuthorityReplayFenceEntryId
    );

    let mut input = clean_fence_input();
    input.preserved_surface_role = "see internal://notes".to_owned();
    assert_eq!(
        resolve_authority_replay_fence_entry(input).unwrap_err(),
        M5SessionRecoveryOrchestrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn fence_disclosed_reauth_and_deferred_stay_clean() {
    // A reauthorization-disclosed previously held authority holds no-reacquire.
    let mut input = clean_fence_input();
    input.authority_was_held_used = true;
    input.reauthorization_required_disclosed = true;
    assert!(resolve_authority_replay_fence_entry(input)
        .unwrap()
        .is_clean());

    // A disclosed deferred privileged flow holds no-reacquire.
    let mut input = clean_fence_input();
    input.privileged_flow_deferred = true;
    input.fresh_intent_required_disclosed = true;
    assert!(resolve_authority_replay_fence_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.vocabulary_set.recovery_posture_states.pop();
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::VocabularySetDrift
    ));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_RESTORE_FIDELITY_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5SessionRecoveryOrchestrationAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5SessionRecoveryOrchestrationExportField::RecoveryPostureStates);
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.registry_rows[0]
        .authority_replay_fence_entries
        .clear();
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::ExamplesMissing
    ));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    // Force a clean posture entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.recovery_posture_entries[0].degrade_reason = None;
    row.recovery_posture_entries[0].recovery_posture_object_complete = false;
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::DishonestExample
    ));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet =
            seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => {
                row.reruns_session_scoped_work_or_reacquires_authority_automatically_after_restore =
                    true
            }
            1 => row.hides_that_reauthorization_is_required = true,
            2 => row.merges_recovery_posture_and_authority_fence_into_one_opaque_blob = true,
            _ => row.overclaims_live_continuity_when_only_context_or_evidence_restored = true,
        }
        assert!(packet.validate().contains(
            &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn recovery_posture_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    for row in &mut packet.registry_rows {
        row.recovery_posture_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SessionRecoveryPostureEntryDegradeReason::RecoveryPostureObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::RecoveryPostureResolutionNotProven
    ));
}

#[test]
fn recovery_posture_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    // Drop every clean admin posture so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.recovery_posture_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::RecoveryPostureResolutionNotProven
    ));
}

#[test]
fn posture_before_replay_not_proven_when_replay_first_example_removed() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    for row in &mut packet.registry_rows {
        row.recovery_posture_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SessionRecoveryPostureEntryDegradeReason::ReplayPrecededPosture)
        });
    }
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::PostureBeforeReplayNotProven
    ));
}

#[test]
fn posture_before_replay_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    for row in &mut packet.registry_rows {
        row.recovery_posture_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SessionRecoveryPostureEntryDegradeReason::PostureNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::PostureBeforeReplayNotProven
    ));
}

#[test]
fn fence_continuity_not_proven_when_reacquires_example_removed() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    for row in &mut packet.registry_rows {
        row.authority_replay_fence_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceReacquiresOrOverclaims,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::AuthorityFenceContinuityNotProven
    ));
}

#[test]
fn fence_continuity_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    // Drop every clean shared-control fence so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.authority_replay_fence_entries
            .retain(|ex| !(ex.is_clean() && ex.fence_class == "shared_control_grant"));
    }
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::AuthorityFenceContinuityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet
        .governance_review
        .posture_decided_before_authority_replay = false;
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet.validate().contains(
        &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::RawMaterialInExport
    ));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries()
        .export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn recovery_posture_table_lists_only_clean_postures() {
    let packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    let table = packet.render_recovery_posture_table();
    // The clean transcript and reconnect postures are rendered from the registry.
    assert!(table.contains("transcript_restored"));
    assert!(table.contains("reconnect_available"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_export()
            .expect(
                "checked M5 recovery-posture / authority-replay-fence registries export validates",
            );
    assert_eq!(
        from_disk.packet_id,
        M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_reconnect_posture_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5WindowRestoreConsumerSurface::RestoreCoordinator)
        .unwrap();
    assert_eq!(row.qualification, M5WindowRestoreQualificationClass::Beta);

    let preview =
        seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_context_only_continuity_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5WindowRestoreConsumerSurface::Diagnostics)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WindowRestoreQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-no-rerun-session-recovery-and-authority-replay-fence-registries/reconnect_posture_beta_narrowed.json"
    )))
    .expect("reconnect-posture fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_reconnect_posture_beta_narrowed()
    );

    let preview: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-no-rerun-session-recovery-and-authority-replay-fence-registries/context_only_continuity_preview_narrowed.json"
    )))
    .expect("context-only fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_context_only_continuity_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_no_rerun_session_hydration() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5WindowRestoreFamily::NoRerunSessionHydration]
    );
}
