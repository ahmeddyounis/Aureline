use super::*;

fn clean_channel_input() -> M5ChannelIsolationEntryResolutionInput {
    M5ChannelIsolationEntryResolutionInput {
        entry_id: "channel:test".to_owned(),
        profile_id: "profile.side_by_side_stable".to_owned(),
        token_name: "channel.side_by_side.stable".to_owned(),
        semantic_role: M5InstallTopologyRole::WritableStateRoots,
        channel: M5SideBySideChannel::Stable,
        surface_context: M5ChannelSurfaceContext::InstallerFlow,
        presentation_form_coverage: M5ChannelPresentationForm::ALL.to_vec(),
        channel_root: r"%LOCALAPPDATA%\Aureline\Stable".to_owned(),
        state_namespace_root: r"%LOCALAPPDATA%\Aureline\Stable\state".to_owned(),
        secrets_namespace_root: r"%LOCALAPPDATA%\Aureline\Stable\secrets".to_owned(),
        isolation_fields_covered: M5ChannelIsolationField::ALL.to_vec(),
        containment: M5ChannelStateContainment::Isolated,
        bound_to_registry: true,
        namespace_reuse_used: false,
        namespace_isolation_enforced: true,
        proof_fresh: true,
    }
}

fn clean_precedence_input() -> M5PrecedenceRollbackEntryResolutionInput {
    M5PrecedenceRollbackEntryResolutionInput {
        entry_id: "precedence:test".to_owned(),
        profile_id: "profile.side_by_side_stable".to_owned(),
        token_name: "precedence.file_association".to_owned(),
        semantic_role: M5InstallTopologyRole::RollbackTarget,
        precedence_domain: M5PrecedenceReviewDomain::FileAssociation,
        surface_context: M5ChannelSurfaceContext::InstallerFlow,
        presentation_form_coverage: M5ChannelPresentationForm::ALL.to_vec(),
        association_owner: "channel.stable".to_owned(),
        rollback_artifact_graph_root: r"%LOCALAPPDATA%\Aureline\Stable\rollback\artifact-graph"
            .to_owned(),
        disclosed_fields: M5PrecedenceReviewField::ALL.to_vec(),
        rollback_posture: M5RollbackCompletenessPosture::FullArtifactGraphBound,
        rollback_artifact_graph_continuity_documented: true,
        precedence_ownership_disclosed: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_PACKET_ID
    );
}

#[test]
fn channel_clean_names_meaning_and_is_bound() {
    let resolved = resolve_channel_isolation_entry(clean_channel_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.channel_resolves_across_profiles);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.channel_isolation_complete);
    assert!(resolved.channel_state_isolated);
    assert!(resolved.bound_to_registry);
    assert!(resolved.channel_is_isolatable);
    assert!(resolved.containment_is_disclosed);
    assert_eq!(resolved.semantic_role, "writable_state_roots");
    assert_eq!(resolved.channel, "stable");
    assert_eq!(resolved.containment, "isolated");
    assert_eq!(resolved.surface_context, "installer_flow");
    assert_eq!(
        resolved.next_action,
        M5ChannelNextAction::ExpandChannelMeaning
    );
}

#[test]
fn channel_token_unstated_degrades() {
    let mut input = clean_channel_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::ChannelTokenUnstated)
    );
}

#[test]
fn channel_unbound_and_unclassified_degrade() {
    let mut input = clean_channel_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::ChannelNotBoundToRegistry)
    );

    let mut input = clean_channel_input();
    input.channel = M5SideBySideChannel::ChannelUnclassified;
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::ChannelUnclassified)
    );
}

#[test]
fn channel_inventory_and_reuse_and_containment_and_form_degrade() {
    // A dropped mandatory isolation field leaves the inventory incomplete.
    let mut input = clean_channel_input();
    input.isolation_fields_covered = vec![
        M5ChannelIsolationField::ChannelRoot,
        M5ChannelIsolationField::StateNamespace,
        M5ChannelIsolationField::SecretsNamespace,
    ];
    let resolved = resolve_channel_isolation_entry(input).unwrap();
    assert!(!resolved.channel_isolation_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::ChannelNamespaceInventoryIncomplete)
    );

    // A preview channel reusing the stable namespace degrades.
    let mut input = clean_channel_input();
    input.namespace_reuse_used = true;
    input.namespace_isolation_enforced = false;
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::PreviewCorruptedStableDurableState)
    );

    // An unenforced (unproven) isolation posture degrades even without an actual reuse.
    let mut input = clean_channel_input();
    input.namespace_isolation_enforced = false;
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::PreviewCorruptedStableDurableState)
    );

    // An ambiguous containment degrades.
    let mut input = clean_channel_input();
    input.containment = M5ChannelStateContainment::ContainmentAmbiguous;
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::ContainmentAmbiguous)
    );

    let mut input = clean_channel_input();
    input.presentation_form_coverage = vec![M5ChannelPresentationForm::CanonicalObject];
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::PresentationFormCoverageIncomplete)
    );
}

#[test]
fn channel_surface_and_proof_degrade() {
    let mut input = clean_channel_input();
    input.surface_context = M5ChannelSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_channel_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_channel_isolation_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ChannelIsolationEntryDegradeReason::ProofStale)
    );
}

#[test]
fn channel_empty_id_and_forbidden_material_error() {
    let mut input = clean_channel_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_channel_isolation_entry(input).unwrap_err(),
        M5ChannelResolutionError::EmptyChannelEntryId
    );

    let mut input = clean_channel_input();
    input.channel_root = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_channel_isolation_entry(input).unwrap_err(),
        M5ChannelResolutionError::ForbiddenMaterial
    );
}

#[test]
fn channel_state_is_isolated_requires_no_reuse_and_full_inventory() {
    assert!(channel_state_is_isolated(
        M5SideBySideChannel::Stable,
        r"%LOCALAPPDATA%\Aureline\Stable",
        r"%LOCALAPPDATA%\Aureline\Stable\state",
        r"%LOCALAPPDATA%\Aureline\Stable\secrets",
        &M5ChannelIsolationField::ALL,
        false,
        true,
    ));
    // A reused stable namespace breaks isolation.
    assert!(!channel_state_is_isolated(
        M5SideBySideChannel::Preview,
        r"%LOCALAPPDATA%\Aureline\Preview",
        r"%LOCALAPPDATA%\Aureline\Preview\state",
        r"%LOCALAPPDATA%\Aureline\Preview\secrets",
        &M5ChannelIsolationField::ALL,
        true,
        true,
    ));
    // An unenforced isolation posture is not proven isolated.
    assert!(!channel_state_is_isolated(
        M5SideBySideChannel::Preview,
        r"%LOCALAPPDATA%\Aureline\Preview",
        r"%LOCALAPPDATA%\Aureline\Preview\state",
        r"%LOCALAPPDATA%\Aureline\Preview\secrets",
        &M5ChannelIsolationField::ALL,
        false,
        false,
    ));
    // An unclassified channel is never isolated.
    assert!(!channel_state_is_isolated(
        M5SideBySideChannel::ChannelUnclassified,
        r"%LOCALAPPDATA%\Aureline\Stable",
        r"%LOCALAPPDATA%\Aureline\Stable\state",
        r"%LOCALAPPDATA%\Aureline\Stable\secrets",
        &M5ChannelIsolationField::ALL,
        false,
        true,
    ));
}

#[test]
fn precedence_clean_is_inspectable_and_full_graph() {
    let resolved = resolve_precedence_and_rollback_entry(clean_precedence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.precedence_and_rollback_inspectable_on_every_profile);
    assert!(resolved.covers_all_presentation_forms);
    assert!(resolved.handler_precedence_inspectable);
    assert!(resolved.rollback_full_artifact_graph);
    assert_eq!(resolved.precedence_domain, "file_association");
    assert_eq!(resolved.rollback_posture, "full_artifact_graph_bound");
}

#[test]
fn precedence_inspectability_and_rollback_and_domain_degrade() {
    // A dropped mandatory field leaves the precedence rule not inspectable.
    let mut input = clean_precedence_input();
    input.disclosed_fields = vec![
        M5PrecedenceReviewField::OwnerChannel,
        M5PrecedenceReviewField::PrecedenceRank,
        M5PrecedenceReviewField::ConflictResolution,
        M5PrecedenceReviewField::RollbackArtifactGraph,
    ];
    let resolved = resolve_precedence_and_rollback_entry(input).unwrap();
    assert!(!resolved.handler_precedence_inspectable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PrecedenceRollbackEntryDegradeReason::HandlerPrecedenceNotInspectable)
    );

    // An undisclosed precedence ownership is also not inspectable.
    let mut input = clean_precedence_input();
    input.precedence_ownership_disclosed = false;
    assert_eq!(
        resolve_precedence_and_rollback_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PrecedenceRollbackEntryDegradeReason::HandlerPrecedenceNotInspectable)
    );

    // An undocumented rollback artifact-graph continuity degrades.
    let mut input = clean_precedence_input();
    input.rollback_artifact_graph_continuity_documented = false;
    assert_eq!(
        resolve_precedence_and_rollback_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PrecedenceRollbackEntryDegradeReason::RollbackArtifactGraphIncomplete)
    );

    // An unclassified rollback posture also breaks the full-graph binding.
    let mut input = clean_precedence_input();
    input.rollback_posture = M5RollbackCompletenessPosture::PostureUnclassified;
    assert_eq!(
        resolve_precedence_and_rollback_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PrecedenceRollbackEntryDegradeReason::RollbackArtifactGraphIncomplete)
    );

    let mut input = clean_precedence_input();
    input.precedence_domain = M5PrecedenceReviewDomain::DomainUnclassified;
    assert_eq!(
        resolve_precedence_and_rollback_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PrecedenceRollbackEntryDegradeReason::PrecedenceDomainUnclassified)
    );
}

#[test]
fn precedence_form_and_id_and_material() {
    let mut input = clean_precedence_input();
    input.presentation_form_coverage = vec![M5ChannelPresentationForm::CanonicalObject];
    assert_eq!(
        resolve_precedence_and_rollback_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PrecedenceRollbackEntryDegradeReason::PrecedenceFormCoverageIncomplete)
    );

    let mut input = clean_precedence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_precedence_and_rollback_entry(input).unwrap_err(),
        M5ChannelResolutionError::EmptyPrecedenceEntryId
    );

    let mut input = clean_precedence_input();
    input.rollback_artifact_graph_root = "see internal://notes".to_owned();
    assert_eq!(
        resolve_precedence_and_rollback_entry(input).unwrap_err(),
        M5ChannelResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_channel_isolation_precedence_review_and_rollback_targets()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.vocabulary_set.channels.pop();
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::VocabularySetDrift
    ));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ChannelAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ChannelExportField::Containments);
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.registry_rows[0].precedence_rollback_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    // Force a clean channel entry to also read as inventory-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.channel_isolation_entries[0].degrade_reason = None;
    row.channel_isolation_entries[0].channel_isolation_complete = false;
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::DishonestExample
    ));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.preview_or_beta_reused_stable_state_namespace = true,
            1 => row.handler_ownership_became_last_writer_wins = true,
            2 => row.rollback_targeted_primary_executable_only = true,
            _ => row.channel_precedence_or_rollback_drifted_from_matrix = true,
        }
        assert!(packet.validate().contains(
            &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn channel_isolation_contract_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    for row in &mut packet.registry_rows {
        row.channel_isolation_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ChannelIsolationEntryDegradeReason::ChannelNamespaceInventoryIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ChannelIsolationContractNotProven
    ));
}

#[test]
fn channel_isolation_contract_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    // Drop every clean admin channel so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.channel_isolation_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ChannelIsolationContractNotProven
    ));
}

#[test]
fn handler_precedence_inspectability_not_proven_when_ambiguous_example_removed() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    for row in &mut packet.registry_rows {
        row.channel_isolation_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ChannelIsolationEntryDegradeReason::ContainmentAmbiguous)
        });
    }
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::HandlerPrecedenceInspectabilityNotProven
    ));
}

#[test]
fn handler_precedence_inspectability_not_proven_when_domain_dropped() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    // Drop every clean deep-link precedence so the canonical domain coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.precedence_rollback_entries
            .retain(|ex| !(ex.is_clean() && ex.precedence_domain == "deep_link"));
    }
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::HandlerPrecedenceInspectabilityNotProven
    ));
}

#[test]
fn rollback_completeness_not_proven_when_reuse_example_removed() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    for row in &mut packet.registry_rows {
        row.channel_isolation_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ChannelIsolationEntryDegradeReason::PreviewCorruptedStableDurableState)
        });
    }
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::RollbackArtifactGraphCompletenessNotProven
    ));
}

#[test]
fn rollback_completeness_not_proven_when_rollback_example_removed() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    for row in &mut packet.registry_rows {
        row.precedence_rollback_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5PrecedenceRollbackEntryDegradeReason::RollbackArtifactGraphIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::RollbackArtifactGraphCompletenessNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet
        .governance_review
        .preview_never_reuses_stable_state_namespace = false;
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet.validate().contains(
        &M5ChannelIsolationPrecedenceReviewAndRollbackTargetsViolation::RawMaterialInExport
    ));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_channel_isolation_precedence_review_and_rollback_targets().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn isolation_table_lists_only_clean_channels() {
    let packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    let table = packet.render_channel_isolation_table();
    // The clean stable and lts channels are rendered from the registry.
    assert!(table.contains("stable"));
    assert!(table.contains("lts"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("inventory-incomplete"));
    // An ambiguous containment never leaks into the generated table.
    assert!(!table.contains("containment_ambiguous"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_channel_isolation_precedence_review_and_rollback_targets_export()
            .expect("checked M5 channel-isolation / precedence / rollback export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_channel_isolation_precedence_review_and_rollback_targets(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_channel_isolation_precedence_review_and_rollback_targets_side_by_side_channel_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5InstallTopologyConsumerSurface::Admin)
        .unwrap();
    assert_eq!(row.qualification, M5InstallTopologyQualificationClass::Beta);

    let preview =
        seeded_m5_channel_isolation_precedence_review_and_rollback_targets_offline_airgap_bundle_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5InstallTopologyConsumerSurface::UpdaterService)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5InstallTopologyQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-channel-isolation-precedence-review-and-rollback-targets/side_by_side_channel_beta_narrowed.json"
    )))
    .expect("side-by-side-channel fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_channel_isolation_precedence_review_and_rollback_targets_side_by_side_channel_beta_narrowed()
    );

    let preview: M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-channel-isolation-precedence-review-and-rollback-targets/offline_airgap_bundle_preview_narrowed.json"
    )))
    .expect("offline fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_channel_isolation_precedence_review_and_rollback_targets_offline_airgap_bundle_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_side_by_side() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5InstallTopologyFamily::SideBySideStablePreview]
    );
}
