use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_discoverability_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DISCOVERABILITY_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_surface_family() {
    let packet = seeded_m5_discoverability_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for family in M5CommandSurfaceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing surface family {}",
            family.as_str()
        );
    }
    assert_eq!(packet.surface_rows.len(), M5CommandSurfaceFamily::ALL.len());
}

#[test]
fn every_surface_declares_mandatory_labels_and_a_command_binding() {
    let packet = seeded_m5_discoverability_matrix();
    for row in &packet.surface_rows {
        for label in M5RequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "surface {} missing mandatory label {}",
                row.surface_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row
            .canonical_command_binding
            .command_id_field
            .trim()
            .is_empty());
        assert!(!row
            .canonical_command_binding
            .help_anchor_ref
            .trim()
            .is_empty());
        assert!(!row.unavailable_reasons.is_empty());
        assert!(!row.stale_target_states.is_empty());
        assert!(!row.parity_surfaces.is_empty());
    }
}

#[test]
fn shortcut_resolving_surfaces_declare_source_classes() {
    let packet = seeded_m5_discoverability_matrix();
    for row in &packet.surface_rows {
        if row.surface_family.resolves_shortcuts() {
            assert!(
                !row.shortcut_source_classes.is_empty(),
                "surface {} resolves shortcuts but declares none",
                row.surface_family.as_str()
            );
        }
        if row.surface_family.reviews_conflicts() {
            assert!(
                !row.conflict_reasons.is_empty(),
                "surface {} reviews conflicts but declares none",
                row.surface_family.as_str()
            );
        }
        if row.surface_family.translates_imports() {
            assert!(
                !row.import_translation_states.is_empty(),
                "surface {} translates imports but declares none",
                row.surface_family.as_str()
            );
        }
    }
}

#[test]
fn every_shortcut_source_class_is_declared_by_some_surface() {
    let packet = seeded_m5_discoverability_matrix();
    for class in M5ShortcutSourceClass::ALL {
        assert!(
            packet
                .surface_rows
                .iter()
                .any(|row| row.shortcut_source_classes.contains(&class)),
            "no surface declares shortcut-source class {}",
            class.as_str()
        );
    }
}

#[test]
fn precedence_ranks_are_monotonic_across_the_layer_stack() {
    // The declared order of ALL is the precedence stack; ranks must be
    // non-decreasing so keybinding help can name a deterministic winner.
    let mut last = 0u8;
    for class in M5ShortcutSourceClass::ALL {
        assert!(
            class.precedence_rank() >= last,
            "shortcut-source precedence is not monotonic at {}",
            class.as_str()
        );
        last = class.precedence_rank();
    }
}

#[test]
fn missing_surface_family_fails_validation() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet
        .surface_rows
        .retain(|row| row.surface_family != M5CommandSurfaceFamily::ImportBridgeRow);
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.vocabulary_set.shortcut_source_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.surface_rows[0]
        .required_labels
        .retain(|label| *label != M5RequiredLabel::CommandId);
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn command_binding_incomplete_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.surface_rows[0]
        .canonical_command_binding
        .command_id_field
        .clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::CommandBindingIncomplete));
}

#[test]
fn shortcut_source_missing_fails_for_resolver_surface() {
    let mut packet = seeded_m5_discoverability_matrix();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::KeybindingResolverLayer)
        .expect("resolver row present");
    row.shortcut_source_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::ShortcutSourceMissing));
}

#[test]
fn conflict_reason_missing_fails_for_conflict_sheet() {
    let mut packet = seeded_m5_discoverability_matrix();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::ConflictReviewSheet)
        .expect("conflict sheet present");
    row.conflict_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::ConflictReasonMissing));
}

#[test]
fn import_translation_missing_fails_for_import_bridge() {
    let mut packet = seeded_m5_discoverability_matrix();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::ImportBridgeRow)
        .expect("import-bridge row present");
    row.import_translation_states.clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::ImportTranslationMissing));
}

#[test]
fn unavailable_reason_missing_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.surface_rows[0].unavailable_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::UnavailableReasonMissing));
}

#[test]
fn stale_target_missing_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.surface_rows[0].stale_target_states.clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::StaleTargetMissing));
}

#[test]
fn surface_invariant_violation_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.surface_rows[0].invents_alternate_label = true;
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::SurfaceInvariantViolated));

    let mut packet = seeded_m5_discoverability_matrix();
    packet.surface_rows[0].masks_preview_or_approval = true;
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::SurfaceInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5CommandSurfaceFamily::MenuItem)
        .expect("menu-item row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.surface_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.governance_review.no_surface_widens_authority = false;
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet
        .consumer_projection
        .keybinding_help_shows_source_and_conflicts = false;
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_discoverability_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DiscoverabilityMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface_family() {
    let summary = seeded_m5_discoverability_matrix().render_markdown_summary();
    for family in M5CommandSurfaceFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing surface {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_discoverability_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CommandSurfaceFamily::ALL.len());
    assert!(lines[0].starts_with("surface_family,qualification,owner,"));
    for family in M5CommandSurfaceFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing surface {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_discoverability_matrix_export()
        .expect("checked M5 discoverability matrix export validates");
    assert_eq!(packet.packet_id, M5_DISCOVERABILITY_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_discoverability_matrix_export()
        .expect("checked M5 discoverability matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_discoverability_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_discoverability_matrix_imported_keymap_approximated_narrowed(),
        seeded_m5_discoverability_matrix_leader_sequence_help_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.surface_rows.len(), M5CommandSurfaceFamily::ALL.len());
    }

    let imported = seeded_m5_discoverability_matrix_imported_keymap_approximated_narrowed();
    let row = imported
        .surface_rows
        .iter()
        .find(|r| r.surface_family == M5CommandSurfaceFamily::ImportBridgeRow)
        .expect("import-bridge row present");
    assert_eq!(row.qualification, M5SurfaceQualificationClass::Beta);

    let leader = seeded_m5_discoverability_matrix_leader_sequence_help_preview_narrowed();
    let row = leader
        .surface_rows
        .iter()
        .find(|r| r.surface_family == M5CommandSurfaceFamily::LeaderSequenceHelp)
        .expect("leader-sequence-help row present");
    assert_eq!(row.qualification, M5SurfaceQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let imported: M5DiscoverabilityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/commands/m5-discoverability-affordances/imported_keymap_approximated_narrowed.json"
    )))
    .expect("imported fixture parses");
    assert!(imported.validate().is_empty());
    assert_eq!(
        imported,
        seeded_m5_discoverability_matrix_imported_keymap_approximated_narrowed()
    );

    let leader: M5DiscoverabilityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/commands/m5-discoverability-affordances/leader_sequence_help_preview_narrowed.json"
    )))
    .expect("leader fixture parses");
    assert!(leader.validate().is_empty());
    assert_eq!(
        leader,
        seeded_m5_discoverability_matrix_leader_sequence_help_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_discoverability_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
