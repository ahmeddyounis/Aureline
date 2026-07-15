use super::*;

fn clean_descriptor_input() -> M5WideningDecisionEntryResolutionInput {
    M5WideningDecisionEntryResolutionInput {
        entry_id: "descriptor:test".to_owned(),
        widening_event_binding_id: "incident.lane.core-team-canary".to_owned(),
        token_name: "freeze.exception.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        widening_decision_packet_kind: M5WideningDecisionPacketKind::AlphaWideningDecision,
        surface_context: M5WideningDecisionSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5WideningDecisionResolutionForm::ALL.to_vec(),
        final_decision_reference: "repo.rows.core-team-canary-archetypes".to_owned(),
        open_risks_reference: "bundle.ids.canary-0007".to_owned(),
        narrowed_claims_reference: "install.topology.internal-dogfood-ring".to_owned(),
        on_call_roster_reference: "toolchain.envelope.pinned-canary".to_owned(),
        signoff_roster_reference: "known-limits.published.canary".to_owned(),
        evidence_snapshot_reference: "rollback.target.canary-previous-stable".to_owned(),
        decision_freshness_reference: "diagnostics.posture.full-telemetry".to_owned(),
        bound_to_registry: true,
        widening_decision_documented_before_widening: true,
        requires_documented_exception: false,
        attributable_asset_or_approved_exception: true,
        proof_fresh: true,
    }
}

fn clean_evidence_input() -> M5RingHistoryEntryResolutionInput {
    M5RingHistoryEntryResolutionInput {
        entry_id: "evidence:test".to_owned(),
        ring_history_ref: "incident.lane.core-team-canary".to_owned(),
        token_name: "go.no.go.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        ring_history_coverage: M5RingHistoryCoverageKind::RingHistoryScope,
        surface_context: M5WideningDecisionSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5WideningDecisionResolutionForm::ALL.to_vec(),
        resolved_coverage_identity: "transition-id.core-team-canary-0007".to_owned(),
        evidence_snapshot_ledger: "known-limits.ledger.canary".to_owned(),
        orr_signoff_reference: "rollback.target.ref.canary".to_owned(),
        on_call_roster_state: "rehearsal.currency.dogfood-ring-current".to_owned(),
        ring_history_freshness_state: "readiness.signoff.dogfood-reviewed".to_owned(),
        widening_stage_reference: "support.language.canary-bound-to-proof".to_owned(),
        last_ring_history_revision: "widening.revision.0007".to_owned(),
        keeps_evidence_snapshot_visible: true,
        ring_history_lineage_is_truthful: true,
        override_without_evidence_requested: false,
        blocked_until_evidence_linked: false,
        lineage_gap_present: false,
        lineage_gap_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_widening_decision_and_ring_history_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_PACKET_ID
    );
}

#[test]
fn descriptor_clean_names_meaning_and_is_bound() {
    let resolved = resolve_widening_decision_entry(clean_descriptor_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.widening_decision_resolves_across_classes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.widening_decision_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.widening_decision_packet_kind_is_classified);
    assert!(resolved.widening_decision_documented_before_widening);
    assert_eq!(resolved.semantic_role, "cohort_membership");
    assert_eq!(
        resolved.widening_decision_packet_kind,
        "alpha_widening_decision"
    );
    assert_eq!(
        resolved.canonical_widening_decision_packet_kind_mode,
        "alpha_widening_decision_kind"
    );
    assert_eq!(resolved.surface_context, "shiproom_surface");
    assert_eq!(
        resolved.next_action,
        M5WideningDecisionNextAction::ExpandWideningDecisionMeaning
    );
}

#[test]
fn widening_decision_token_unstated_degrades() {
    let mut input = clean_descriptor_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_widening_decision_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionTokenUnstated)
    );
}

#[test]
fn descriptor_unbound_and_unclassified_degrade() {
    let mut input = clean_descriptor_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_widening_decision_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionNotBoundToRegistry)
    );

    let mut input = clean_descriptor_input();
    input.widening_decision_packet_kind = M5WideningDecisionPacketKind::PacketKindUnclassified;
    assert_eq!(
        resolve_widening_decision_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionPacketKindUnclassified)
    );
}

#[test]
fn descriptor_object_incomplete_and_widen_fold_and_form_degrade() {
    // An unstated bundle IDs field leaves the resolved object incomplete.
    let mut input = clean_descriptor_input();
    input.open_risks_reference = "  ".to_owned();
    let resolved = resolve_widening_decision_entry(input).unwrap();
    assert!(!resolved.widening_decision_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WideningDecisionEntryDegradeReason::WideningDecisionObjectIncomplete)
    );

    // A cohort widening without a preserved rollback / diagnostics posture degrades with the structured blocker.
    let mut input = clean_descriptor_input();
    input.widening_decision_documented_before_widening = false;
    assert_eq!(
        resolve_widening_decision_entry(input).unwrap().degrade_reason,
        Some(
            M5WideningDecisionEntryDegradeReason::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof
        )
    );

    let mut input = clean_descriptor_input();
    input.resolution_form_coverage = vec![M5WideningDecisionResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_widening_decision_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WideningDecisionEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn descriptor_public_facing_and_surface_and_proof_degrade() {
    // A public-facing cohort running support language ahead of proof first fails the widen-boundary fold.
    let mut input = clean_descriptor_input();
    input.widening_decision_packet_kind =
        M5WideningDecisionPacketKind::LongTermSupportWideningDecision;
    input.requires_documented_exception = true;
    input.attributable_asset_or_approved_exception = false;
    assert_eq!(
        resolve_widening_decision_entry(input).unwrap().degrade_reason,
        Some(
            M5WideningDecisionEntryDegradeReason::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof
        )
    );

    let mut input = clean_descriptor_input();
    input.surface_context = M5WideningDecisionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_widening_decision_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WideningDecisionEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_descriptor_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_widening_decision_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5WideningDecisionEntryDegradeReason::ProofStale)
    );
}

#[test]
fn descriptor_empty_id_and_forbidden_material_error() {
    let mut input = clean_descriptor_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_widening_decision_entry(input).unwrap_err(),
        M5WideningDecisionResolutionError::EmptyWideningDecisionEntryId
    );

    let mut input = clean_descriptor_input();
    input.evidence_snapshot_reference = "see https://cohort.internal/leak".to_owned();
    assert_eq!(
        resolve_widening_decision_entry(input).unwrap_err(),
        M5WideningDecisionResolutionError::ForbiddenMaterial
    );
}

#[test]
fn cohort_preserves_rollback_and_diagnostics_rejects_unpreserved() {
    assert!(widening_decision_stays_documented_before_widening(
        M5WideningDecisionPacketKind::AlphaWideningDecision,
        true,
        false,
        true
    ));
    assert!(!widening_decision_stays_documented_before_widening(
        M5WideningDecisionPacketKind::AlphaWideningDecision,
        false,
        false,
        true
    ));
    assert!(widening_decision_stays_documented_before_widening(
        M5WideningDecisionPacketKind::LongTermSupportWideningDecision,
        true,
        true,
        true
    ));
    assert!(!widening_decision_stays_documented_before_widening(
        M5WideningDecisionPacketKind::LongTermSupportWideningDecision,
        true,
        true,
        false
    ));
    assert!(!widening_decision_stays_documented_before_widening(
        M5WideningDecisionPacketKind::PacketKindUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn widening_decision_object_is_complete_requires_all_fields() {
    assert!(widening_decision_object_is_complete(
        M5WideningDecisionPacketKind::AlphaWideningDecision,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!widening_decision_object_is_complete(
        M5WideningDecisionPacketKind::AlphaWideningDecision,
        "repo.rows.core-team-canary-archetypes",
        "  ",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!widening_decision_object_is_complete(
        M5WideningDecisionPacketKind::PacketKindUnclassified,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
}

#[test]
fn evidence_clean_stays_honest() {
    let resolved = resolve_ring_history_entry(clean_evidence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.ring_history_safe_on_every_coverage);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_ring_history_record);
    assert!(resolved.ring_history_stays_honest);
    assert_eq!(resolved.ring_history_coverage, "ring_history_scope");
    assert_eq!(resolved.surface_context, "shiproom_surface");
}

#[test]
fn evidence_support_ahead_and_unclassified_degrade() {
    // Support language present but not bound to cohort proof runs support ahead of proof.
    let mut input = clean_evidence_input();
    input.override_without_evidence_requested = true;
    input.blocked_until_evidence_linked = false;
    let resolved = resolve_ring_history_entry(input).unwrap();
    assert!(!resolved.provides_complete_ring_history_record);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RingHistoryEntryDegradeReason::RingHistoryDropsEvidenceOrImpliesGreenWhileStale)
    );

    // A packet that hides the cohort evidence is also caught.
    let mut input = clean_evidence_input();
    input.keeps_evidence_snapshot_visible = false;
    assert_eq!(
        resolve_ring_history_entry(input).unwrap().degrade_reason,
        Some(M5RingHistoryEntryDegradeReason::RingHistoryDropsEvidenceOrImpliesGreenWhileStale)
    );

    // A known-limits gap masquerading as covered is also caught.
    let mut input = clean_evidence_input();
    input.lineage_gap_present = true;
    input.lineage_gap_flagged = false;
    assert_eq!(
        resolve_ring_history_entry(input).unwrap().degrade_reason,
        Some(M5RingHistoryEntryDegradeReason::RingHistoryDropsEvidenceOrImpliesGreenWhileStale)
    );

    let mut input = clean_evidence_input();
    input.ring_history_coverage = M5RingHistoryCoverageKind::CoverageUnclassified;
    assert_eq!(
        resolve_ring_history_entry(input).unwrap().degrade_reason,
        Some(M5RingHistoryEntryDegradeReason::RingHistoryCoverageUnclassified)
    );
}

#[test]
fn evidence_form_and_surface_and_id_and_material() {
    let mut input = clean_evidence_input();
    input.resolution_form_coverage = vec![M5WideningDecisionResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_ring_history_entry(input).unwrap().degrade_reason,
        Some(M5RingHistoryEntryDegradeReason::RingHistoryFormCoverageIncomplete)
    );

    let mut input = clean_evidence_input();
    input.surface_context = M5WideningDecisionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_ring_history_entry(input).unwrap().degrade_reason,
        Some(M5RingHistoryEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_evidence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_ring_history_entry(input).unwrap_err(),
        M5WideningDecisionResolutionError::EmptyRingHistoryEntryId
    );

    let mut input = clean_evidence_input();
    input.evidence_snapshot_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_ring_history_entry(input).unwrap_err(),
        M5WideningDecisionResolutionError::ForbiddenMaterial
    );
}

#[test]
fn evidence_bound_support_and_flagged_gap_stay_clean() {
    // Support language bound to cohort proof stays honest.
    let mut input = clean_evidence_input();
    input.override_without_evidence_requested = true;
    input.blocked_until_evidence_linked = true;
    assert!(resolve_ring_history_entry(input).unwrap().is_clean());

    // A known-limits gap flagged rather than masquerading stays honest.
    let mut input = clean_evidence_input();
    input.lineage_gap_present = true;
    input.lineage_gap_flagged = true;
    assert!(resolve_ring_history_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_widening_decision_and_ring_history_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.vocabulary_set.widening_decision_packet_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_WIDENING_DECISION_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_RING_HISTORY_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5WideningDecisionAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5WideningDecisionExportField::WideningDecisionPacketKinds);
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.registry_rows[0].ring_history_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    // Force a clean descriptor entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.widening_decision_entries[0].degrade_reason = None;
    row.widening_decision_entries[0].widening_decision_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.widens_a_stable_claim_without_a_durable_go_no_go_record = true,
            1 => row.drops_the_evidence_snapshot_or_roster_from_a_widening_record = true,
            2 => row.hides_the_ring_history_or_prior_blockers_before_widening = true,
            _ => row.implies_green_when_go_no_go_records_or_evidence_snapshots_are_stale = true,
        }
        assert!(packet
            .validate()
            .contains(&M5WideningDecisionRingHistoryRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn cohort_descriptor_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    for row in &mut packet.registry_rows {
        row.widening_decision_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5WideningDecisionEntryDegradeReason::WideningDecisionObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5WideningDecisionRingHistoryRegistriesViolation::WideningDecisionResolutionNotProven
    ));
}

#[test]
fn cohort_descriptor_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    // Drop every clean executive-steering-surface descriptor so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.widening_decision_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "executive_steering_surface"));
    }
    assert!(packet.validate().contains(
        &M5WideningDecisionRingHistoryRegistriesViolation::WideningDecisionResolutionNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_widen_fold_example_removed() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    for row in &mut packet.registry_rows {
        row.widening_decision_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5WideningDecisionEntryDegradeReason::WideningDecisionWidensScopeUndocumentedOrRunsClaimAheadOfProof,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5WideningDecisionRingHistoryRegistriesViolation::RingHistoryAttributionNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    for row in &mut packet.registry_rows {
        row.widening_decision_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5WideningDecisionEntryDegradeReason::WideningDecisionNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5WideningDecisionRingHistoryRegistriesViolation::RingHistoryAttributionNotProven
    ));
}

#[test]
fn ring_history_integrity_not_proven_when_support_ahead_example_removed() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    for row in &mut packet.registry_rows {
        row.ring_history_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5RingHistoryEntryDegradeReason::RingHistoryDropsEvidenceOrImpliesGreenWhileStale)
        });
    }
    assert!(packet.validate().contains(
        &M5WideningDecisionRingHistoryRegistriesViolation::RingHistoryIntegrityNotProven
    ));
}

#[test]
fn ring_history_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    // Drop every clean ring-history-signoff evidence so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.ring_history_entries
            .retain(|ex| !(ex.is_clean() && ex.ring_history_coverage == "packet_freshness_scope"));
    }
    assert!(packet.validate().contains(
        &M5WideningDecisionRingHistoryRegistriesViolation::RingHistoryIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet
        .governance_review
        .scope_cannot_widen_without_documented_widening_decision = false;
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://cohort.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5WideningDecisionRingHistoryRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_widening_decision_and_ring_history_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_widening_decision_and_ring_history_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_widening_decision_and_ring_history_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn cohort_descriptor_table_lists_only_clean_descriptors() {
    let packet = seeded_m5_widening_decision_and_ring_history_registries();
    let table = packet.render_widening_decision_table();
    // The clean canary and migration descriptors are rendered from the registry.
    assert!(table.contains("alpha_widening_decision_kind"));
    assert!(table.contains("beta_widening_decision_kind"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains(":incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_widening_decision_and_ring_history_registries_export()
        .expect("checked M5 widening-decision / ring-history registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_widening_decision_and_ring_history_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_widening_decision_and_ring_history_registries_widening_decision_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .unwrap();
    assert_eq!(row.qualification, M5LaunchControlQualificationClass::Beta);

    let preview =
        seeded_m5_widening_decision_and_ring_history_registries_ring_history_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5LaunchControlConsumerSurface::ReleaseCenter)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5LaunchControlQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5WideningDecisionRingHistoryRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-widening-decision-and-ring-history-registries/widening_decision_beta_narrowed.json"
    )))
    .expect("widening-decision fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_widening_decision_and_ring_history_registries_widening_decision_beta_narrowed()
    );

    let preview: M5WideningDecisionRingHistoryRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-widening-decision-and-ring-history-registries/ring_history_preview_narrowed.json"
    )))
    .expect("ring-history fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_widening_decision_and_ring_history_registries_ring_history_preview_narrowed()
    );
}

#[test]
fn implemented_ring_stages_is_all_five_widening_stages() {
    assert_eq!(
        IMPLEMENTED_WIDENING_DECISION_STAGES,
        [
            M5LaunchControlWideningStage::Alpha,
            M5LaunchControlWideningStage::Beta,
            M5LaunchControlWideningStage::ReleaseCandidate,
            M5LaunchControlWideningStage::Stable,
            M5LaunchControlWideningStage::LongTermSupport,
        ]
    );
}
