use super::*;

fn clean_tier_input() -> M5LayerTierEntryResolutionInput {
    M5LayerTierEntryResolutionInput {
        entry_id: "tier:test".to_owned(),
        token_name: "layer.menu.palette".to_owned(),
        semantic_role: M5VisualInteractionRole::Layer,
        layer_order_role: M5LayerOrderRole::OverlayTier,
        layer_tier: M5LayerTier::Menu,
        surface_context: M5LayerPortalSurfaceContext::Shell,
        hardcodes_always_on_top: false,
        stacks_under_shared_model: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_portal_input() -> M5PortalEntryResolutionInput {
    M5PortalEntryResolutionInput {
        entry_id: "portal:test".to_owned(),
        token_name: "portal.shell.palette".to_owned(),
        portal_ownership_role: M5PortalOwnershipRole::OwningSurfaceAttachment,
        semantic_role: M5VisualInteractionRole::Portal,
        layer_tier: M5LayerTier::Menu,
        attachment_mode: M5PortalAttachmentMode::OwningWindowAnchored,
        surface_context: M5LayerPortalSurfaceContext::Shell,
        attaches_to_owning_surface: true,
        restore_safe: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_layer_order_and_portal_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_LAYER_PORTAL_REGISTRIES_PACKET_ID);
}

#[test]
fn tier_clean_names_meaning_and_is_safe() {
    let resolved = resolve_layer_tier_entry(clean_tier_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.z_order_model_holds);
    assert!(resolved.stacks_under_shared_model);
    assert!(!resolved.hardcodes_always_on_top);
    assert!(!resolved.layer_order_role_is_private_bypass);
    assert!(resolved.layer_tier_is_classified);
    assert!(resolved.layer_tier_is_competing);
    assert_eq!(resolved.z_index, 3);
    assert_eq!(resolved.semantic_role, "layer");
    assert_eq!(resolved.layer_tier, "menu");
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5LayerPortalRegistryNextAction::InspectLayerTier
    );
}

#[test]
fn tier_token_unstated_degrades() {
    let mut input = clean_tier_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_layer_tier_entry(input).unwrap().degrade_reason,
        Some(M5LayerTierEntryDegradeReason::TokenNameUnstated)
    );
}

#[test]
fn tier_always_on_top_and_private_bypass_degrade() {
    let mut input = clean_tier_input();
    input.hardcodes_always_on_top = true;
    assert_eq!(
        resolve_layer_tier_entry(input).unwrap().degrade_reason,
        Some(M5LayerTierEntryDegradeReason::AlwaysOnTopBypassesSharedModel)
    );

    let mut input = clean_tier_input();
    input.layer_order_role = M5LayerOrderRole::PrivateLayerBypassDisallowed;
    let resolved = resolve_layer_tier_entry(input).unwrap();
    assert!(resolved.layer_order_role_is_private_bypass);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5LayerTierEntryDegradeReason::AlwaysOnTopBypassesSharedModel)
    );
}

#[test]
fn tier_raw_z_order_unclassified_and_not_stacked_degrade() {
    let mut input = clean_tier_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_layer_tier_entry(input).unwrap().degrade_reason,
        Some(M5LayerTierEntryDegradeReason::RawZOrderValueInlined)
    );

    let mut input = clean_tier_input();
    input.layer_tier = M5LayerTier::TierUnclassified;
    assert_eq!(
        resolve_layer_tier_entry(input).unwrap().degrade_reason,
        Some(M5LayerTierEntryDegradeReason::LayerTierUnclassified)
    );

    let mut input = clean_tier_input();
    input.stacks_under_shared_model = false;
    assert_eq!(
        resolve_layer_tier_entry(input).unwrap().degrade_reason,
        Some(M5LayerTierEntryDegradeReason::NotStackedUnderSharedModel)
    );

    let mut input = clean_tier_input();
    input.surface_context = M5LayerPortalSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_layer_tier_entry(input).unwrap().degrade_reason,
        Some(M5LayerTierEntryDegradeReason::SurfaceContextUnresolved)
    );
}

#[test]
fn tier_empty_id_and_forbidden_material_error() {
    let mut input = clean_tier_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_layer_tier_entry(input).unwrap_err(),
        M5LayerPortalResolutionError::EmptyLayerTierEntryId
    );

    let mut input = clean_tier_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_layer_tier_entry(input).unwrap_err(),
        M5LayerPortalResolutionError::ForbiddenMaterial
    );
}

#[test]
fn portal_clean_stays_attached_to_owning_surface() {
    let resolved = resolve_portal_entry(clean_portal_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.owning_surface_attachment_preserved);
    assert!(resolved.attaches_to_owning_surface);
    assert!(resolved.restore_safe);
    assert!(resolved.attachment_mode_present);
    assert!(!resolved.portal_role_is_detached);
    assert!(resolved.layer_tier_is_classified);
    assert_eq!(resolved.z_index, 3);
    assert_eq!(resolved.semantic_role, "portal");
    assert_eq!(resolved.portal_ownership_role, "owning_surface_attachment");
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5LayerPortalRegistryNextAction::ExpandLayerMeaning
    );
}

#[test]
fn portal_detached_and_role_detached_degrade() {
    let mut input = clean_portal_input();
    input.attaches_to_owning_surface = false;
    assert_eq!(
        resolve_portal_entry(input).unwrap().degrade_reason,
        Some(M5PortalEntryDegradeReason::PortalDetachedFromOwningSurface)
    );

    let mut input = clean_portal_input();
    input.portal_ownership_role = M5PortalOwnershipRole::DetachedPortalDisallowed;
    let resolved = resolve_portal_entry(input).unwrap();
    assert!(resolved.portal_role_is_detached);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PortalEntryDegradeReason::PortalDetachedFromOwningSurface)
    );
}

#[test]
fn portal_attachment_restore_tier_and_id_degrade() {
    let mut input = clean_portal_input();
    input.attachment_mode = M5PortalAttachmentMode::NoneDisallowed;
    assert_eq!(
        resolve_portal_entry(input).unwrap().degrade_reason,
        Some(M5PortalEntryDegradeReason::AttachmentModeMissing)
    );

    let mut input = clean_portal_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_portal_entry(input).unwrap().degrade_reason,
        Some(M5PortalEntryDegradeReason::RawZOrderValueInlined)
    );

    let mut input = clean_portal_input();
    input.restore_safe = false;
    assert_eq!(
        resolve_portal_entry(input).unwrap().degrade_reason,
        Some(M5PortalEntryDegradeReason::RestoreUnsafeOnOwnerChange)
    );

    let mut input = clean_portal_input();
    input.layer_tier = M5LayerTier::TierUnclassified;
    assert_eq!(
        resolve_portal_entry(input).unwrap().degrade_reason,
        Some(M5PortalEntryDegradeReason::LayerTierUnclassified)
    );

    let mut input = clean_portal_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_portal_entry(input).unwrap_err(),
        M5LayerPortalResolutionError::EmptyPortalEntryId
    );

    let mut input = clean_portal_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_portal_entry(input).unwrap_err(),
        M5LayerPortalResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_layer_order_and_portal_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.vocabulary_set.layer_tiers.pop();
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5LayerPortalRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5LayerPortalRegistryExportField::LayerTiers);
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.registry_rows[0].portal_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    // Force a clean layer-tier entry to also hard-code always-on-top — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.layer_tier_entries[0].degrade_reason = None;
    row.layer_tier_entries[0].hardcodes_always_on_top = true;
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_layer_order_and_portal_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.overlay_hardcodes_always_on_top = true,
            1 => row.portal_detaches_from_owning_surface = true,
            2 => row.raw_z_order_value_inlined_instead_of_token = true,
            _ => row.layer_order_bypasses_shared_z_order_model = true,
        }
        assert!(packet
            .validate()
            .contains(&M5LayerPortalRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn first_consumers_not_proven_when_raw_z_order_example_removed() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    for row in &mut packet.registry_rows {
        row.layer_tier_entries.retain(|ex| {
            ex.degrade_reason != Some(M5LayerTierEntryDegradeReason::RawZOrderValueInlined)
        });
    }
    assert!(packet.validate().contains(
        &M5LayerPortalRegistriesViolation::FirstConsumersObeyCanonicalLayerModelNotProven
    ));
}

#[test]
fn first_consumers_not_proven_when_first_surface_collapses() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    // Drop every clean entry rendered on the embedded surface so the first-consumer surface set collapses.
    for row in &mut packet.registry_rows {
        row.layer_tier_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "embedded"));
        row.portal_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "embedded"));
    }
    assert!(packet.validate().contains(
        &M5LayerPortalRegistriesViolation::FirstConsumersObeyCanonicalLayerModelNotProven
    ));
}

#[test]
fn competing_tiers_not_proven_when_always_on_top_example_removed() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    for row in &mut packet.registry_rows {
        row.layer_tier_entries.retain(|ex| {
            ex.degrade_reason != Some(M5LayerTierEntryDegradeReason::AlwaysOnTopBypassesSharedModel)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::CompetingTiersNoAdHocZOrderNotProven));
}

#[test]
fn competing_tiers_not_proven_when_competing_tier_dropped() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    // Drop every clean menu tier so competing coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.layer_tier_entries
            .retain(|ex| !(ex.is_clean() && ex.layer_tier == "menu"));
    }
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::CompetingTiersNoAdHocZOrderNotProven));
}

#[test]
fn portal_continuity_not_proven_when_detached_example_removed() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    for row in &mut packet.registry_rows {
        row.portal_entries.retain(|ex| {
            ex.degrade_reason != Some(M5PortalEntryDegradeReason::PortalDetachedFromOwningSurface)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::PortalContinuityAndDriftVisibleNotProven));
}

#[test]
fn portal_continuity_not_proven_when_tier_drift_dropped() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    // Drop every tier-unclassified drift example so drift is no longer visible.
    for row in &mut packet.registry_rows {
        row.layer_tier_entries.retain(|ex| {
            ex.degrade_reason != Some(M5LayerTierEntryDegradeReason::LayerTierUnclassified)
        });
        row.portal_entries.retain(|ex| {
            ex.degrade_reason != Some(M5PortalEntryDegradeReason::LayerTierUnclassified)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::PortalContinuityAndDriftVisibleNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.governance_review.no_overlay_hardcodes_always_on_top = false;
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_layer_order_and_portal_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5LayerPortalRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_layer_order_and_portal_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_layer_order_and_portal_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_layer_order_and_portal_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_layer_order_and_portal_registries_export()
        .expect("checked M5 layer-order and portal registries export validates");
    assert_eq!(from_disk.packet_id, M5_LAYER_PORTAL_REGISTRIES_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_layer_order_and_portal_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_layer_order_and_portal_registries_shell_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualInteractionConsumerSurface::ShellUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualInteractionQualificationClass::Beta
    );

    let preview = seeded_m5_layer_order_and_portal_registries_onboarding_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualInteractionConsumerSurface::OnboardingUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualInteractionQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5LayerPortalRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-layer-order-and-portal-registries/shell_ui_beta_narrowed.json"
    )))
    .expect("shell-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_layer_order_and_portal_registries_shell_ui_beta_narrowed()
    );

    let preview: M5LayerPortalRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-layer-order-and-portal-registries/onboarding_ui_preview_narrowed.json"
    )))
    .expect("onboarding-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_layer_order_and_portal_registries_onboarding_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_layer_order_and_portal_ownership() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5VisualInteractionFamily::LayerOrder,
            M5VisualInteractionFamily::PortalOwnership
        ]
    );
}
