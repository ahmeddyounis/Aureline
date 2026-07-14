use super::*;

fn clean_evidence_input() -> M5AcquisitionEvidenceEntryResolutionInput {
    M5AcquisitionEvidenceEntryResolutionInput {
        entry_id: "evidence:test".to_owned(),
        acquisition_path_id: "entry.acme.open-local".to_owned(),
        token_name: "acquisition.evidence.clone_fetch_transcript".to_owned(),
        semantic_role: M5RepositoryBootstrapRole::EvidencePacket,
        evidence_kind: M5AcquisitionEvidenceKind::CloneFetchTranscript,
        surface_context: M5RecoverySurfaceContext::ShellSurface,
        resolution_form_coverage: M5RecoveryResolutionForm::ALL.to_vec(),
        transcript_ref: "transcript.acme/clone-fetch-log".to_owned(),
        warnings_and_retries_ref: "warnings.acme/none-observed".to_owned(),
        resulting_root_identity_ref: "root-identity.acme/full-head".to_owned(),
        omitted_or_unfetched_ref: "omitted.acme/none-omitted".to_owned(),
        bootstrap_checkpoint_ref: "checkpoint.acme/complete".to_owned(),
        evidence_provenance: "evidence-provenance.acme.v3".to_owned(),
        bound_to_registry: true,
        partial_state_visible: true,
        describes_partial_state: false,
        partial_not_full_disclosed: true,
        proof_fresh: true,
    }
}

fn clean_recovery_input() -> M5PartialRecoveryEntryResolutionInput {
    M5PartialRecoveryEntryResolutionInput {
        entry_id: "recovery:test".to_owned(),
        source_ref: "entry.acme.clone-remote".to_owned(),
        token_name: "partial.recovery.resume_acquisition".to_owned(),
        semantic_role: M5RepositoryBootstrapRole::ResumableAcquisition,
        recovery_class: M5PartialRecoveryClass::ResumeAcquisition,
        surface_context: M5RecoverySurfaceContext::ShellSurface,
        resolution_form_coverage: M5RecoveryResolutionForm::ALL.to_vec(),
        recovery_action_kind: "recovery-action.resume-from-checkpoint".to_owned(),
        recovery_site: "site.worktree".to_owned(),
        state_consequence: "consequence.continues-partial-state".to_owned(),
        lineage_consequence: "consequence.preserves-transcript-lineage".to_owned(),
        explicit_action_requirement: "action.explicit-resume-required".to_owned(),
        attribution_ref: "attribution.acquisition-engine".to_owned(),
        identifies_recovery_site_and_state_effect: true,
        action_is_truthfully_typed: true,
        is_state_mutating_action: true,
        explicit_discard_or_cleanup_action_present: true,
        schedules_deferred_cleanup: false,
        cleanup_is_disclosed: false,
        discards_state_without_explicit_action: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_PACKET_ID
    );
}

#[test]
fn evidence_clean_names_meaning_and_is_bound() {
    let resolved = resolve_acquisition_evidence_entry(clean_evidence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.evidence_resolves_across_entry_flows);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.evidence_packet_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.evidence_kind_is_classified);
    assert!(resolved.partial_state_visible);
    assert_eq!(resolved.semantic_role, "evidence_packet");
    assert_eq!(resolved.evidence_kind, "clone_fetch_transcript");
    assert_eq!(
        resolved.canonical_evidence_mode,
        "clone_fetch_transcript_evidence"
    );
    assert_eq!(resolved.surface_context, "shell_surface");
    assert_eq!(
        resolved.next_action,
        M5RecoveryNextAction::ExpandRecoveryMeaning
    );
}

#[test]
fn evidence_token_unstated_degrades() {
    let mut input = clean_evidence_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_acquisition_evidence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceTokenUnstated)
    );
}

#[test]
fn evidence_unbound_and_unclassified_degrade() {
    let mut input = clean_evidence_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_acquisition_evidence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceNotBoundToRegistry)
    );

    let mut input = clean_evidence_input();
    input.evidence_kind = M5AcquisitionEvidenceKind::EvidenceUnclassified;
    assert_eq!(
        resolve_acquisition_evidence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceKindUnclassified)
    );
}

#[test]
fn evidence_packet_incomplete_and_overclaim_and_form_degrade() {
    // An unstated transcript reference leaves the resolved packet incomplete.
    let mut input = clean_evidence_input();
    input.transcript_ref = "  ".to_owned();
    let resolved = resolve_acquisition_evidence_entry(input).unwrap();
    assert!(!resolved.evidence_packet_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5AcquisitionEvidenceEntryDegradeReason::EvidencePacketIncomplete)
    );

    // A partial-describing packet with no partial-not-full disclosure overclaims a full checkout and degrades.
    let mut input = clean_evidence_input();
    input.evidence_kind = M5AcquisitionEvidenceKind::OmittedOrUnfetchedState;
    input.describes_partial_state = true;
    input.partial_not_full_disclosed = false;
    assert_eq!(
        resolve_acquisition_evidence_entry(input).unwrap().degrade_reason,
        Some(
            M5AcquisitionEvidenceEntryDegradeReason::EvidenceOverclaimsFullCheckoutOrHidesPartialState
        )
    );

    let mut input = clean_evidence_input();
    input.resolution_form_coverage = vec![M5RecoveryResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_acquisition_evidence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AcquisitionEvidenceEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn evidence_invisible_and_surface_and_proof_degrade() {
    let mut input = clean_evidence_input();
    input.partial_state_visible = false;
    // A packet that leaves the partial state invisible first fails visibility.
    assert_eq!(
        resolve_acquisition_evidence_entry(input).unwrap().degrade_reason,
        Some(
            M5AcquisitionEvidenceEntryDegradeReason::EvidenceOverclaimsFullCheckoutOrHidesPartialState
        )
    );

    let mut input = clean_evidence_input();
    input.surface_context = M5RecoverySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_acquisition_evidence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AcquisitionEvidenceEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_evidence_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_acquisition_evidence_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5AcquisitionEvidenceEntryDegradeReason::ProofStale)
    );
}

#[test]
fn evidence_empty_id_and_forbidden_material_error() {
    let mut input = clean_evidence_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_acquisition_evidence_entry(input).unwrap_err(),
        M5RecoveryResolutionError::EmptyAcquisitionEvidenceEntryId
    );

    let mut input = clean_evidence_input();
    input.evidence_provenance = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_acquisition_evidence_entry(input).unwrap_err(),
        M5RecoveryResolutionError::ForbiddenMaterial
    );
}

#[test]
fn acquisition_evidence_discloses_partial_state_rejects_overclaim() {
    assert!(acquisition_evidence_discloses_partial_state(
        M5AcquisitionEvidenceKind::CloneFetchTranscript,
        true,
        false,
        true
    ));
    assert!(!acquisition_evidence_discloses_partial_state(
        M5AcquisitionEvidenceKind::CloneFetchTranscript,
        false,
        false,
        true
    ));
    assert!(acquisition_evidence_discloses_partial_state(
        M5AcquisitionEvidenceKind::OmittedOrUnfetchedState,
        true,
        true,
        true
    ));
    assert!(!acquisition_evidence_discloses_partial_state(
        M5AcquisitionEvidenceKind::OmittedOrUnfetchedState,
        true,
        true,
        false
    ));
    assert!(!acquisition_evidence_discloses_partial_state(
        M5AcquisitionEvidenceKind::EvidenceUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn acquisition_evidence_object_is_complete_requires_all_fields() {
    assert!(acquisition_evidence_object_is_complete(
        M5AcquisitionEvidenceKind::CloneFetchTranscript,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/none-observed",
        "root-identity.acme/full-head",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    ));
    assert!(!acquisition_evidence_object_is_complete(
        M5AcquisitionEvidenceKind::CloneFetchTranscript,
        "transcript.acme/clone-fetch-log",
        "  ",
        "root-identity.acme/full-head",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    ));
    assert!(!acquisition_evidence_object_is_complete(
        M5AcquisitionEvidenceKind::EvidenceUnclassified,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/none-observed",
        "root-identity.acme/full-head",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    ));
}

#[test]
fn recovery_clean_preserves_lineage() {
    let resolved = resolve_partial_recovery_entry(clean_recovery_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.recovery_safe_on_every_source);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_partial_recovery);
    assert!(resolved.partial_recovery_action_preserves_lineage);
    assert!(resolved.recovery_class_is_state_mutating);
    assert!(!resolved.discards_state_without_explicit_action);
    assert_eq!(resolved.recovery_class, "resume_acquisition");
    assert_eq!(resolved.surface_context, "shell_surface");
}

#[test]
fn recovery_discard_without_action_and_unclassified_degrade() {
    // A state-mutating action that discards state without an explicit action breaks the recovery.
    let mut input = clean_recovery_input();
    input.discards_state_without_explicit_action = true;
    let resolved = resolve_partial_recovery_entry(input).unwrap();
    assert!(!resolved.provides_complete_partial_recovery);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PartialRecoveryEntryDegradeReason::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction)
    );

    // A state-mutating action that is not gated behind an explicit discard or cleanup also breaks.
    let mut input = clean_recovery_input();
    input.explicit_discard_or_cleanup_action_present = false;
    assert_eq!(
        resolve_partial_recovery_entry(input).unwrap().degrade_reason,
        Some(M5PartialRecoveryEntryDegradeReason::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction)
    );

    // A hidden action / consequence also breaks the recovery action.
    let mut input = clean_recovery_input();
    input.identifies_recovery_site_and_state_effect = false;
    assert_eq!(
        resolve_partial_recovery_entry(input).unwrap().degrade_reason,
        Some(M5PartialRecoveryEntryDegradeReason::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction)
    );

    // An undisclosed scheduled cleanup also breaks the recovery action.
    let mut input = clean_recovery_input();
    input.schedules_deferred_cleanup = true;
    input.cleanup_is_disclosed = false;
    assert_eq!(
        resolve_partial_recovery_entry(input).unwrap().degrade_reason,
        Some(M5PartialRecoveryEntryDegradeReason::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction)
    );

    let mut input = clean_recovery_input();
    input.recovery_class = M5PartialRecoveryClass::RecoveryUnclassified;
    assert_eq!(
        resolve_partial_recovery_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PartialRecoveryEntryDegradeReason::PartialRecoveryClassUnclassified)
    );
}

#[test]
fn recovery_form_and_surface_and_id_and_material() {
    let mut input = clean_recovery_input();
    input.resolution_form_coverage = vec![M5RecoveryResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_partial_recovery_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PartialRecoveryEntryDegradeReason::RecoveryFormCoverageIncomplete)
    );

    let mut input = clean_recovery_input();
    input.surface_context = M5RecoverySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_partial_recovery_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PartialRecoveryEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_recovery_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_partial_recovery_entry(input).unwrap_err(),
        M5RecoveryResolutionError::EmptyPartialRecoveryEntryId
    );

    let mut input = clean_recovery_input();
    input.attribution_ref = "see internal://notes".to_owned();
    assert_eq!(
        resolve_partial_recovery_entry(input).unwrap_err(),
        M5RecoveryResolutionError::ForbiddenMaterial
    );
}

#[test]
fn recovery_read_only_and_gated_mutating_stay_clean() {
    // An open-read-only-partial-root action is not state-mutating and stays clean without a gate.
    let mut input = clean_recovery_input();
    input.recovery_class = M5PartialRecoveryClass::OpenReadOnlyPartialRoot;
    input.is_state_mutating_action = false;
    input.explicit_discard_or_cleanup_action_present = false;
    let resolved = resolve_partial_recovery_entry(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.recovery_class_is_state_mutating);

    // A state-mutating action with a disclosed cleanup stays clean.
    let mut input = clean_recovery_input();
    input.schedules_deferred_cleanup = true;
    input.cleanup_is_disclosed = true;
    assert!(resolve_partial_recovery_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_acquisition_evidence_and_partial_recovery_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.vocabulary_set.evidence_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5AcquisitionEvidencePartialRecoveryRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5RecoveryAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5RecoveryExportField::EvidenceKinds);
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.registry_rows[0].partial_recovery_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5AcquisitionEvidencePartialRecoveryRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    // Force a clean evidence entry to also read as packet-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.acquisition_evidence_entries[0].degrade_reason = None;
    row.acquisition_evidence_entries[0].evidence_packet_complete = false;
    assert!(packet
        .validate()
        .contains(&M5AcquisitionEvidencePartialRecoveryRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.presents_partial_acquisition_as_healthy_full_checkout = true,
            1 => row.discards_partial_state_or_lineage_without_explicit_action = true,
            2 => row.hides_what_a_recovery_action_would_do_or_its_state_or_lineage_effect = true,
            _ => row.leaves_partial_or_interrupted_state_invisible_or_unrecoverable = true,
        }
        assert!(packet.validate().contains(
            &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn acquisition_evidence_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    for row in &mut packet.registry_rows {
        row.acquisition_evidence_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5AcquisitionEvidenceEntryDegradeReason::EvidencePacketIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::AcquisitionEvidenceResolutionNotProven
    ));
}

#[test]
fn acquisition_evidence_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    // Drop every clean admin-surface evidence so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.acquisition_evidence_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::AcquisitionEvidenceResolutionNotProven
    ));
}

#[test]
fn partial_state_not_proven_when_overclaim_example_removed() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    for row in &mut packet.registry_rows {
        row.acquisition_evidence_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5AcquisitionEvidenceEntryDegradeReason::EvidenceOverclaimsFullCheckoutOrHidesPartialState,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::PartialStateVisibilityNotProven
    ));
}

#[test]
fn partial_state_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    for row in &mut packet.registry_rows {
        row.acquisition_evidence_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5AcquisitionEvidenceEntryDegradeReason::EvidenceNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::PartialStateVisibilityNotProven
    ));
}

#[test]
fn partial_recovery_gating_not_proven_when_discard_example_removed() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    for row in &mut packet.registry_rows {
        row.partial_recovery_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5PartialRecoveryEntryDegradeReason::PartialRecoveryDiscardsStateOrLineageWithoutExplicitAction,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::PartialRecoveryGatingNotProven
    ));
}

#[test]
fn partial_recovery_gating_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    // Drop every clean open-read-only-partial-root action so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.partial_recovery_entries
            .retain(|ex| !(ex.is_clean() && ex.recovery_class == "open_read_only_partial_root"));
    }
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::PartialRecoveryGatingNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet
        .governance_review
        .acquisition_evidence_stays_visible_no_full_checkout_overclaim = false;
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5AcquisitionEvidencePartialRecoveryRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://clone.example/repo leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AcquisitionEvidencePartialRecoveryRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_acquisition_evidence_and_partial_recovery_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn partial_recovery_table_lists_only_clean_recovery_items() {
    let packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    let table = packet.render_partial_recovery_table();
    // The clean resume and discard recovery actions are rendered from the registry.
    assert!(table.contains("resume_acquisition"));
    assert!(table.contains("discard_partial_state"));
    // A degraded, state-discarding entry never leaks into the generated table.
    assert!(!table.contains("discard-without-action"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_acquisition_evidence_and_partial_recovery_registries_export()
        .expect("checked M5 acquisition-evidence / partial-recovery registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_acquisition_evidence_and_partial_recovery_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_acquisition_evidence_and_partial_recovery_registries_resume_partial_beta_narrowed(
        );
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepositoryBootstrapConsumerSurface::TrustService)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Beta
    );

    let preview =
        seeded_m5_acquisition_evidence_and_partial_recovery_registries_discard_cleanup_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepositoryBootstrapConsumerSurface::Diagnostics)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5AcquisitionEvidencePartialRecoveryRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-acquisition-evidence-and-partial-recovery-registries/resume_partial_beta_narrowed.json"
    )))
    .expect("resume-partial fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_acquisition_evidence_and_partial_recovery_registries_resume_partial_beta_narrowed(
        )
    );

    let preview: M5AcquisitionEvidencePartialRecoveryRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-acquisition-evidence-and-partial-recovery-registries/discard_cleanup_preview_narrowed.json"
    )))
    .expect("discard-cleanup fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_acquisition_evidence_and_partial_recovery_registries_discard_cleanup_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_all_five_acquisition_verbs() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5RepositoryBootstrapFamily::OpenLocal,
            M5RepositoryBootstrapFamily::CloneRemote,
            M5RepositoryBootstrapFamily::OpenArchive,
            M5RepositoryBootstrapFamily::ImportBundle,
            M5RepositoryBootstrapFamily::ResumeSnapshot,
        ]
    );
}
