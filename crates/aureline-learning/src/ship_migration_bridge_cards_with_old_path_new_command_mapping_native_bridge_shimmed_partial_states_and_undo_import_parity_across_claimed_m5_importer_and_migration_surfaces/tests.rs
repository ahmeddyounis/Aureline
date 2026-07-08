use super::*;

fn exact_durable_bridge() -> M5MigrationBridgeCardResolutionInput {
    M5MigrationBridgeCardResolutionInput {
        mapping_class: M5MigrationMappingClass::Exact,
        source_tool: M5SourceToolClass::RivalIde,
        old_path_ref: "Ctrl+Shift+P (rival IDE command palette)".to_owned(),
        new_command_ref: Some("command:command-palette.open".to_owned()),
        affected_scope: "The command palette open shortcut".to_owned(),
        unsupported_edge_cases: vec![],
        import_created_durable_change: true,
        rollback_checkpoint_ref: Some("checkpoint:import:command-palette-shortcut:0001".to_owned()),
        bridge_identity_ref: "bridge:migration-report:command-palette".to_owned(),
    }
}

// ---- migration-bridge-card resolver -------------------------------------

#[test]
fn exact_durable_bridge_claims_parity_and_keeps_undo() {
    let resolved = resolve_migration_bridge_card(&exact_durable_bridge()).expect("resolves");
    assert_eq!(
        resolved.bridge_posture,
        M5MigrationBridgePosture::ExactParity
    );
    assert!(resolved.claims_exact_parity);
    assert!(resolved.is_faithful_mapping);
    assert!(!resolved.is_approximated_mapping);
    assert!(resolved.import_created_durable_change);
    assert!(resolved.has_rollback_checkpoint);
    assert!(resolved.undo_available);
    assert!(resolved.open_native_command_available);
    assert!(resolved.discloses_old_path_and_new_command);
    assert!(resolved.discloses_mapping_state_honestly);
    assert!(resolved.never_overstates_as_exact_parity);
    assert!(resolved.preserves_affected_scope);
    assert!(resolved.preserves_unsupported_edge_cases);
    assert!(resolved.preserves_import_rollback_linkage);
    assert!(resolved.keeps_undo_review_available_for_durable_changes);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5MigrationBridgeAction::ViewMappingDetails,
            M5MigrationBridgeAction::OpenNativeCommand,
            M5MigrationBridgeAction::UndoImportChanges,
            M5MigrationBridgeAction::ReviewImportCheckpoint,
        ]
    );
}

#[test]
fn approximated_and_partial_never_claim_exact_parity() {
    for mapping in [
        M5MigrationMappingClass::Bridge,
        M5MigrationMappingClass::Shimmed,
        M5MigrationMappingClass::Partial,
    ] {
        let resolved = resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            mapping_class: mapping,
            unsupported_edge_cases: vec!["some behavior is not covered".to_owned()],
            ..exact_durable_bridge()
        })
        .expect("resolves");
        assert!(
            !resolved.claims_exact_parity,
            "{} wrongly claimed exact parity",
            mapping.as_str()
        );
        assert!(!resolved.is_faithful_mapping);
        assert_ne!(
            resolved.bridge_posture,
            M5MigrationBridgePosture::ExactParity
        );
    }
}

#[test]
fn unsupported_mapping_offers_only_inspect_and_report() {
    let resolved = resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
        mapping_class: M5MigrationMappingClass::Unsupported,
        new_command_ref: None,
        unsupported_edge_cases: vec!["no equivalent exists".to_owned()],
        import_created_durable_change: false,
        rollback_checkpoint_ref: None,
        ..exact_durable_bridge()
    })
    .expect("resolves");
    assert_eq!(
        resolved.bridge_posture,
        M5MigrationBridgePosture::UnsupportedNoMapping
    );
    assert!(resolved.is_unsupported_mapping);
    assert!(!resolved.open_native_command_available);
    assert!(!resolved.undo_available);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5MigrationBridgeAction::ViewMappingDetails,
            M5MigrationBridgeAction::ReportUnsupportedEdgeCase,
        ]
    );
}

#[test]
fn non_durable_bridge_offers_no_undo() {
    let resolved = resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
        mapping_class: M5MigrationMappingClass::Bridge,
        import_created_durable_change: false,
        rollback_checkpoint_ref: None,
        unsupported_edge_cases: vec![],
        ..exact_durable_bridge()
    })
    .expect("resolves");
    assert_eq!(
        resolved.bridge_posture,
        M5MigrationBridgePosture::BridgedApproximation
    );
    assert!(!resolved.undo_available);
    assert!(!resolved.has_rollback_checkpoint);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5MigrationBridgeAction::ViewMappingDetails,
            M5MigrationBridgeAction::OpenNativeCommand,
        ]
    );
}

#[test]
fn durable_change_without_rollback_is_rejected() {
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            import_created_durable_change: true,
            rollback_checkpoint_ref: None,
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::DurableChangeWithoutRollback)
    );
}

#[test]
fn mapped_state_without_new_command_is_rejected() {
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            new_command_ref: None,
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::MissingNewCommandForMappedState)
    );
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            new_command_ref: Some("  ".to_owned()),
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::MissingNewCommandForMappedState)
    );
}

#[test]
fn unsupported_state_with_new_command_is_rejected() {
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            mapping_class: M5MigrationMappingClass::Unsupported,
            new_command_ref: Some("command:something".to_owned()),
            unsupported_edge_cases: vec!["x".to_owned()],
            import_created_durable_change: false,
            rollback_checkpoint_ref: None,
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::NativeCommandOnUnsupportedState)
    );
}

#[test]
fn partial_and_unsupported_require_edge_cases() {
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            mapping_class: M5MigrationMappingClass::Partial,
            unsupported_edge_cases: vec![],
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::MissingUnsupportedEdgeCases)
    );
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            mapping_class: M5MigrationMappingClass::Unsupported,
            new_command_ref: None,
            unsupported_edge_cases: vec![],
            import_created_durable_change: false,
            rollback_checkpoint_ref: None,
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::MissingUnsupportedEdgeCases)
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            old_path_ref: " ".to_owned(),
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::EmptyOldPath)
    );
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            affected_scope: "".to_owned(),
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::EmptyAffectedScope)
    );
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            bridge_identity_ref: "".to_owned(),
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::EmptyBridgeIdentity)
    );
    assert_eq!(
        resolve_migration_bridge_card(&M5MigrationBridgeCardResolutionInput {
            new_command_ref: Some("command:https://evil.example/x".to_owned()),
            ..exact_durable_bridge()
        }),
        Err(M5MigrationBridgeCardResolutionError::ForbiddenBridgeMaterial)
    );
}

#[test]
fn posture_maps_one_to_one_from_mapping_class() {
    for mapping in M5MigrationMappingClass::ALL {
        assert_eq!(
            M5MigrationBridgePosture::from_mapping(mapping).as_str(),
            match mapping {
                M5MigrationMappingClass::Exact => "exact_parity",
                M5MigrationMappingClass::Native => "native_equivalent",
                M5MigrationMappingClass::Bridge => "bridged_approximation",
                M5MigrationMappingClass::Shimmed => "shimmed_compatibility",
                M5MigrationMappingClass::Partial => "partial_coverage",
                M5MigrationMappingClass::Unsupported => "unsupported_no_mapping",
            }
        );
    }
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_migration_bridge_card_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_MIGRATION_BRIDGE_CARD_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_migration_bridge_card_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5MigrationBridgeConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5MigrationBridgeConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_migration_bridge_card_packet();
    for row in &packet.rows {
        for part in M5MigrationBridgeAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5MigrationBridgeExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable));
        assert!(!row.bridge_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_migration_bridge_card_packet();
    let cases: Vec<&M5MigrationBridgeCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.bridge_examples.iter())
        .collect();

    for posture in M5MigrationBridgePosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.bridge_posture == posture),
            "no example exercises bridge posture {}",
            posture.as_str()
        );
    }
    for action in M5MigrationBridgeAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises action {}",
            action.as_str()
        );
    }
    for mapping in M5MigrationMappingClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.mapping_class == mapping),
            "no example exercises mapping class {}",
            mapping.as_str()
        );
    }
}

#[test]
fn some_durable_change_keeps_undo_available() {
    let packet = seeded_m5_migration_bridge_card_packet();
    assert!(packet
        .rows
        .iter()
        .flat_map(|row| row.bridge_examples.iter())
        .any(|c| c.resolved.import_created_durable_change
            && c.resolved.undo_available
            && c.resolved.has_rollback_checkpoint));
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity_and_reversibility() {
    let packet = seeded_m5_migration_bridge_card_packet();
    for row in &packet.rows {
        for case in &row.bridge_examples {
            assert!(
                case.is_self_consistent(),
                "bridge case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "bridge case for {} lost identity",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_reversibility(),
                "bridge case for {} lost reversibility",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5MigrationBridgeConsumerSurface::ImportDiffRow);
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.vocabulary_set.bridge_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5MigrationBridgeAnatomyPart::MappingStateCue);
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5MigrationBridgeExportField::MappingClass);
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.rows[0].bridge_examples[0]
        .resolved
        .claims_exact_parity = false;
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::ExampleResolutionDrift));
}

#[test]
fn bridge_example_missing_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.rows[1].bridge_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::BridgeExampleMissing));
}

#[test]
fn mapping_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    for row in &mut packet.rows {
        row.bridge_examples = vec![M5MigrationBridgeCardResolutionCase::resolved(
            exact_durable_bridge(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::MappingClassCoverageUnproven));
}

#[test]
fn posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    // Every example exact-parity → no approximated, partial, or unsupported posture.
    for row in &mut packet.rows {
        row.bridge_examples = vec![M5MigrationBridgeCardResolutionCase::resolved(
            exact_durable_bridge(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::PostureCoverageUnproven));
}

#[test]
fn action_coverage_unproven_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    for row in &mut packet.rows {
        row.bridge_examples = vec![M5MigrationBridgeCardResolutionCase::resolved(
            exact_durable_bridge(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::ActionCoverageUnproven));
}

#[test]
fn undo_parity_unproven_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    // Only non-durable bridge examples → no durable change proves undo parity.
    let non_durable =
        M5MigrationBridgeCardResolutionCase::resolved(M5MigrationBridgeCardResolutionInput {
            mapping_class: M5MigrationMappingClass::Bridge,
            import_created_durable_change: false,
            rollback_checkpoint_ref: None,
            unsupported_edge_cases: vec![],
            ..exact_durable_bridge()
        });
    for row in &mut packet.rows {
        row.bridge_examples = vec![non_durable.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::UndoParityUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.rows[0].overstates_as_exact_parity = true;
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet
        .governance_review
        .undo_review_available_where_import_changed_durable_behavior = false;
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.consumer_projection.action_set_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MigrationBridgeCardViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_migration_bridge_card_packet().render_markdown_summary();
    for surface in M5MigrationBridgeConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_migration_bridge_card_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5MigrationBridgeConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5MigrationBridgeConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_migration_bridge_card_export()
        .expect("checked M5 migration bridge card primitive export validates");
    assert_eq!(from_disk.packet_id, M5_MIGRATION_BRIDGE_CARD_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_migration_bridge_card_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_migration_bridge_card_keybinding_migration_notice_beta_narrowed(),
        seeded_m5_migration_bridge_card_support_migration_export_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5MigrationBridgeConsumerSurface::ALL.len()
        );
    }

    let keybinding = seeded_m5_migration_bridge_card_keybinding_migration_notice_beta_narrowed();
    let row = keybinding
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5MigrationBridgeConsumerSurface::KeybindingMigrationNotice)
        .expect("keybinding-migration-notice row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Beta);

    let support = seeded_m5_migration_bridge_card_support_migration_export_preview_narrowed();
    let row = support
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5MigrationBridgeConsumerSurface::SupportMigrationExport)
        .expect("support-migration-export row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let keybinding: M5MigrationBridgeCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-migration-bridge-card-primitive/keybinding_migration_notice_beta_narrowed.json"
    )))
    .expect("keybinding-migration-notice fixture parses");
    assert!(keybinding.validate().is_empty());
    assert_eq!(
        keybinding,
        seeded_m5_migration_bridge_card_keybinding_migration_notice_beta_narrowed()
    );

    let support: M5MigrationBridgeCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-migration-bridge-card-primitive/support_migration_export_preview_narrowed.json"
    )))
    .expect("support-migration-export fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_migration_bridge_card_support_migration_export_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_migration_bridge_card_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
