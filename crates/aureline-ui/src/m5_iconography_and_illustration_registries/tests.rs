use super::*;

fn clean_icon_input() -> M5IconEntryResolutionInput {
    M5IconEntryResolutionInput {
        entry_id: "icon:test".to_owned(),
        token_name: "icon.action.save".to_owned(),
        semantic_role: M5VisualInteractionRole::Icon,
        iconography_role: M5IconographyRole::ActionIcon,
        meaning_class: M5IconMeaningClass::ActionIcon,
        surface_context: M5IconIllustrationSurfaceContext::Tab,
        has_accessible_text_equivalent: true,
        reuses_stable_metaphor: true,
        boundary_distinct: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_illustration_input() -> M5IllustrationEntryResolutionInput {
    M5IllustrationEntryResolutionInput {
        entry_id: "illustration:test".to_owned(),
        token_name: "illustration.onboarding.welcome".to_owned(),
        illustration_role: M5IllustrationRole::OnboardingIllustration,
        semantic_role: M5VisualInteractionRole::Illustration,
        placement: M5IllustrationPlacement::OnboardingSecondary,
        surface_context: M5IconIllustrationSurfaceContext::Onboarding,
        stays_secondary_to_content: true,
        never_impersonates_operational_or_security_truth: true,
        replaces_operational_messaging: false,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_iconography_and_illustration_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ICON_ILLUSTRATION_REGISTRIES_PACKET_ID);
}

#[test]
fn icon_clean_names_meaning_and_is_labeled() {
    let resolved = resolve_icon_entry(clean_icon_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.icon_semantics_hold);
    assert!(resolved.has_accessible_text_equivalent);
    assert!(resolved.reuses_stable_metaphor);
    assert!(resolved.boundary_distinct);
    assert!(!resolved.iconography_role_is_unlabeled_disallowed);
    assert!(resolved.meaning_class_is_classified);
    assert!(!resolved.meaning_class_is_boundary_sensitive);
    assert_eq!(resolved.semantic_role, "icon");
    assert_eq!(resolved.meaning_class, "action_icon");
    assert_eq!(resolved.surface_context, "tab");
    assert_eq!(
        resolved.next_action,
        M5IconIllustrationRegistryNextAction::InspectIconSemantics
    );
}

#[test]
fn icon_token_unstated_degrades() {
    let mut input = clean_icon_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_icon_entry(input).unwrap().degrade_reason,
        Some(M5IconEntryDegradeReason::TokenNameUnstated)
    );
}

#[test]
fn icon_unlabeled_and_role_disallowed_degrade() {
    let mut input = clean_icon_input();
    input.has_accessible_text_equivalent = false;
    assert_eq!(
        resolve_icon_entry(input).unwrap().degrade_reason,
        Some(M5IconEntryDegradeReason::UnlabeledIconForUncommonOrDestructive)
    );

    let mut input = clean_icon_input();
    input.iconography_role = M5IconographyRole::UnlabeledUncommonOrDestructiveDisallowed;
    let resolved = resolve_icon_entry(input).unwrap();
    assert!(resolved.iconography_role_is_unlabeled_disallowed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5IconEntryDegradeReason::UnlabeledIconForUncommonOrDestructive)
    );
}

#[test]
fn icon_private_grammar_unclassified_metaphor_and_boundary_degrade() {
    let mut input = clean_icon_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_icon_entry(input).unwrap().degrade_reason,
        Some(M5IconEntryDegradeReason::PrivateIconGrammarInsteadOfToken)
    );

    let mut input = clean_icon_input();
    input.meaning_class = M5IconMeaningClass::MeaningUnclassified;
    assert_eq!(
        resolve_icon_entry(input).unwrap().degrade_reason,
        Some(M5IconEntryDegradeReason::IconMeaningUnclassified)
    );

    let mut input = clean_icon_input();
    input.reuses_stable_metaphor = false;
    assert_eq!(
        resolve_icon_entry(input).unwrap().degrade_reason,
        Some(M5IconEntryDegradeReason::MetaphorReuseUnstable)
    );

    let mut input = clean_icon_input();
    input.boundary_distinct = false;
    assert_eq!(
        resolve_icon_entry(input).unwrap().degrade_reason,
        Some(M5IconEntryDegradeReason::FileTypeShellStatusBoundaryCollapsed)
    );

    let mut input = clean_icon_input();
    input.surface_context = M5IconIllustrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_icon_entry(input).unwrap().degrade_reason,
        Some(M5IconEntryDegradeReason::SurfaceContextUnresolved)
    );
}

#[test]
fn icon_empty_id_and_forbidden_material_error() {
    let mut input = clean_icon_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_icon_entry(input).unwrap_err(),
        M5IconIllustrationResolutionError::EmptyIconEntryId
    );

    let mut input = clean_icon_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_icon_entry(input).unwrap_err(),
        M5IconIllustrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn illustration_clean_stays_secondary() {
    let resolved = resolve_illustration_entry(clean_illustration_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.illustration_boundary_preserved);
    assert!(resolved.stays_secondary_to_content);
    assert!(resolved.never_impersonates_operational_or_security_truth);
    assert!(!resolved.replaces_operational_messaging);
    assert!(resolved.placement_present);
    assert!(!resolved.illustration_role_is_operational_truth_disallowed);
    assert_eq!(resolved.semantic_role, "illustration");
    assert_eq!(resolved.illustration_role, "onboarding_illustration");
    assert_eq!(resolved.surface_context, "onboarding");
    assert_eq!(
        resolved.next_action,
        M5IconIllustrationRegistryNextAction::ExpandIconMeaning
    );
}

#[test]
fn illustration_impersonates_and_role_disallowed_degrade() {
    let mut input = clean_illustration_input();
    input.never_impersonates_operational_or_security_truth = false;
    assert_eq!(
        resolve_illustration_entry(input).unwrap().degrade_reason,
        Some(M5IllustrationEntryDegradeReason::IllustrationImpersonatesOperationalOrSecurityTruth)
    );

    let mut input = clean_illustration_input();
    input.illustration_role = M5IllustrationRole::IllustrationAsOperationalTruthDisallowed;
    let resolved = resolve_illustration_entry(input).unwrap();
    assert!(resolved.illustration_role_is_operational_truth_disallowed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5IllustrationEntryDegradeReason::IllustrationImpersonatesOperationalOrSecurityTruth)
    );
}

#[test]
fn illustration_replaces_placement_secondary_and_id_degrade() {
    let mut input = clean_illustration_input();
    input.replaces_operational_messaging = true;
    assert_eq!(
        resolve_illustration_entry(input).unwrap().degrade_reason,
        Some(M5IllustrationEntryDegradeReason::ReplacesOperationalMessaging)
    );

    let mut input = clean_illustration_input();
    input.placement = M5IllustrationPlacement::NoneDisallowed;
    assert_eq!(
        resolve_illustration_entry(input).unwrap().degrade_reason,
        Some(M5IllustrationEntryDegradeReason::PlacementModeMissing)
    );

    let mut input = clean_illustration_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_illustration_entry(input).unwrap().degrade_reason,
        Some(M5IllustrationEntryDegradeReason::PrivateIllustrationGrammarInsteadOfToken)
    );

    let mut input = clean_illustration_input();
    input.stays_secondary_to_content = false;
    assert_eq!(
        resolve_illustration_entry(input).unwrap().degrade_reason,
        Some(M5IllustrationEntryDegradeReason::NotSecondaryToContent)
    );

    let mut input = clean_illustration_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_illustration_entry(input).unwrap_err(),
        M5IconIllustrationResolutionError::EmptyIllustrationEntryId
    );

    let mut input = clean_illustration_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_illustration_entry(input).unwrap_err(),
        M5IconIllustrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_iconography_and_illustration_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.vocabulary_set.meaning_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5IconIllustrationRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5IconIllustrationRegistryExportField::MeaningClasses);
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.registry_rows[0].illustration_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    // Force a clean icon entry to also drop its accessible label — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.icon_entries[0].degrade_reason = None;
    row.icon_entries[0].has_accessible_text_equivalent = false;
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_iconography_and_illustration_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.icon_uses_unlabeled_symbol_for_uncommon_or_destructive_action = true,
            1 => row.file_type_and_shell_status_meaning_collapsed = true,
            2 => row.illustration_impersonates_operational_or_security_truth = true,
            _ => row.private_icon_or_illustration_grammar_instead_of_token = true,
        }
        assert!(packet
            .validate()
            .contains(&M5IconIllustrationRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn first_consumers_not_proven_when_unlabeled_example_removed() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    for row in &mut packet.registry_rows {
        row.icon_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5IconEntryDegradeReason::UnlabeledIconForUncommonOrDestructive)
        });
    }
    assert!(packet.validate().contains(
        &M5IconIllustrationRegistriesViolation::FirstConsumersStableIconSemanticsNotProven
    ));
}

#[test]
fn first_consumers_not_proven_when_first_surface_collapses() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    // Drop every clean entry rendered on the onboarding surface so the first-consumer surface set collapses.
    for row in &mut packet.registry_rows {
        row.icon_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "onboarding"));
        row.illustration_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "onboarding"));
    }
    assert!(packet.validate().contains(
        &M5IconIllustrationRegistriesViolation::FirstConsumersStableIconSemanticsNotProven
    ));
}

#[test]
fn boundary_not_proven_when_collapse_example_removed() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    for row in &mut packet.registry_rows {
        row.icon_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5IconEntryDegradeReason::FileTypeShellStatusBoundaryCollapsed)
        });
    }
    assert!(packet.validate().contains(
        &M5IconIllustrationRegistriesViolation::FileTypeVersusShellStatusDistinctNotProven
    ));
}

#[test]
fn boundary_not_proven_when_boundary_class_dropped() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    // Drop every clean file-type icon so boundary-sensitive coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.icon_entries
            .retain(|ex| !(ex.is_clean() && ex.meaning_class == "file_type_icon"));
    }
    assert!(packet.validate().contains(
        &M5IconIllustrationRegistriesViolation::FileTypeVersusShellStatusDistinctNotProven
    ));
}

#[test]
fn illustration_boundary_not_proven_when_impersonates_example_removed() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    for row in &mut packet.registry_rows {
        row.illustration_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5IllustrationEntryDegradeReason::IllustrationImpersonatesOperationalOrSecurityTruth,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5IconIllustrationRegistriesViolation::IllustrationNeverReplacesOperationalTruthNotProven
    ));
}

#[test]
fn illustration_boundary_not_proven_when_not_secondary_dropped() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    // Drop every not-secondary drift example so drift is no longer visible.
    for row in &mut packet.registry_rows {
        row.illustration_entries.retain(|ex| {
            ex.degrade_reason != Some(M5IllustrationEntryDegradeReason::NotSecondaryToContent)
        });
    }
    assert!(packet.validate().contains(
        &M5IconIllustrationRegistriesViolation::IllustrationNeverReplacesOperationalTruthNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet
        .governance_review
        .no_unlabeled_icon_for_uncommon_or_destructive_action = false;
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_iconography_and_illustration_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5IconIllustrationRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_iconography_and_illustration_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_iconography_and_illustration_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_iconography_and_illustration_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_iconography_and_illustration_registries_export()
        .expect("checked M5 iconography and illustration registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_ICON_ILLUSTRATION_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_iconography_and_illustration_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_iconography_and_illustration_registries_shell_ui_beta_narrowed();
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

    let preview =
        seeded_m5_iconography_and_illustration_registries_onboarding_ui_preview_narrowed();
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
    let beta: M5IconIllustrationRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-iconography-and-illustration-registries/shell_ui_beta_narrowed.json"
    )))
    .expect("shell-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_iconography_and_illustration_registries_shell_ui_beta_narrowed()
    );

    let preview: M5IconIllustrationRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-iconography-and-illustration-registries/onboarding_ui_preview_narrowed.json"
    )))
    .expect("onboarding-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_iconography_and_illustration_registries_onboarding_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_iconography_and_illustration() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5VisualInteractionFamily::Iconography,
            M5VisualInteractionFamily::Illustration
        ]
    );
}
