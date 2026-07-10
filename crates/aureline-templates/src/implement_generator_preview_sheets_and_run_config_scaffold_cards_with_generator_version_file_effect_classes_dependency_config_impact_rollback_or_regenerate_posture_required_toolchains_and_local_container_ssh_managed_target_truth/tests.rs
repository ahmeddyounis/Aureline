use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_generator_run_config_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, GENERATOR_RUN_CONFIG_CONTROLS_PACKET_ID);
    assert_eq!(
        packet.record_kind,
        GENERATOR_RUN_CONFIG_CONTROLS_RECORD_KIND
    );
}

#[test]
fn every_component_carries_its_frozen_family() {
    let packet = seeded_generator_run_config_controls();
    assert!(!packet.generator_sheets.is_empty());
    assert!(!packet.run_config_cards.is_empty());
    for sheet in &packet.generator_sheets {
        assert_eq!(
            sheet.component,
            M5FrameworkComponentFamily::GeneratorPreviewSheet
        );
    }
    for card in &packet.run_config_cards {
        assert_eq!(
            card.component,
            M5FrameworkComponentFamily::RunConfigScaffoldCard
        );
    }
}

#[test]
fn ac_write_effect_posture_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin the exact write-effect labels: no-op preview, review-required
    // write, reversible applied, or unknown / blocked. Assert the exact tokens.
    let tokens: Vec<&str> = WriteEffectPosture::ALL.iter().map(|p| p.as_str()).collect();
    assert_eq!(
        tokens,
        vec![
            "no_op_preview",
            "review_required_write",
            "reversible_applied",
            "unknown_or_blocked"
        ]
    );
}

#[test]
fn ac_writing_generator_never_reads_as_no_op() {
    // AC #1: a generator that changes files, dependencies, or config must never present a no-op
    // write. Every writing impact resolves to a side effect and a non-no-op posture.
    for impact in [
        M5GeneratorImpactClass::FileWrite,
        M5GeneratorImpactClass::DependencyChange,
        M5GeneratorImpactClass::ConfigChange,
        M5GeneratorImpactClass::ScriptOrTaskChange,
    ] {
        let disclosure =
            resolve_generator_preview_posture(impact, M5GeneratorApplyPosture::PreviewReady);
        assert!(disclosure.has_side_effect, "{impact:?}");
        assert!(!disclosure.is_no_op, "{impact:?}");
    }
    let no_op = resolve_generator_preview_posture(
        M5GeneratorImpactClass::NoChange,
        M5GeneratorApplyPosture::PreviewReady,
    );
    assert!(no_op.is_no_op);
    assert!(!no_op.has_side_effect);
}

#[test]
fn ac_writing_run_config_never_reads_as_no_op() {
    // AC #1 for run-config: creating / editing config or adding a dependency is never a no-op.
    for mutation in [
        M5RunConfigMutationClass::CreatesConfigFile,
        M5RunConfigMutationClass::EditsConfigFile,
        M5RunConfigMutationClass::AddsDependency,
    ] {
        let disclosure = resolve_run_config_scaffold_posture(mutation);
        assert!(disclosure.has_side_effect, "{mutation:?}");
        assert!(!disclosure.is_no_op, "{mutation:?}");
    }
    let no_op = resolve_run_config_scaffold_posture(M5RunConfigMutationClass::NoWritePreview);
    assert!(no_op.is_no_op);
    assert!(!no_op.has_side_effect);
}

#[test]
fn posture_is_derived_never_asserted() {
    let packet = seeded_generator_run_config_controls();
    for sheet in &packet.generator_sheets {
        let disclosure = sheet.posture_disclosure();
        assert_eq!(sheet.write_effect_posture, disclosure.write_effect_posture);
        assert_eq!(sheet.claims_no_op_write, disclosure.is_no_op);
        assert_eq!(sheet.has_recovery_path, disclosure.has_recovery_path);
    }
    for card in &packet.run_config_cards {
        let disclosure = card.posture_disclosure();
        assert_eq!(card.write_effect_posture, disclosure.write_effect_posture);
        assert_eq!(card.claims_no_op_write, disclosure.is_no_op);
        assert_eq!(card.has_recovery_path, disclosure.has_recovery_path);
    }
}

#[test]
fn blocked_write_keeps_no_recovery_note_and_no_reversible_path() {
    // A blocked write (side effect present, apply blocked) must not claim a reversible recovery path
    // and must name why it has no automatic undo.
    let disclosure = resolve_generator_preview_posture(
        M5GeneratorImpactClass::ConfigChange,
        M5GeneratorApplyPosture::Blocked,
    );
    assert!(disclosure.has_side_effect);
    assert!(!disclosure.has_recovery_path);
    assert!(disclosure.needs_no_recovery_note);
    assert_eq!(
        disclosure.write_effect_posture,
        WriteEffectPosture::UnknownOrBlocked
    );
}

#[test]
fn reversible_applied_generator_has_recovery_path() {
    let disclosure = resolve_generator_preview_posture(
        M5GeneratorImpactClass::ScriptOrTaskChange,
        M5GeneratorApplyPosture::RollbackAvailable,
    );
    assert_eq!(
        disclosure.write_effect_posture,
        WriteEffectPosture::ReversibleApplied
    );
    assert!(disclosure.is_reversible_applied);
    assert!(disclosure.has_recovery_path);
    assert!(disclosure.needs_applied_recovery_note);
}

#[test]
fn components_cover_every_frozen_and_derived_vocabulary() {
    let packet = seeded_generator_run_config_controls();
    for impact in M5GeneratorImpactClass::ALL {
        assert!(
            packet
                .generator_sheets
                .iter()
                .any(|s| s.generator_impact_class == impact),
            "missing impact {}",
            impact.as_str()
        );
    }
    for apply in M5GeneratorApplyPosture::ALL {
        assert!(
            packet
                .generator_sheets
                .iter()
                .any(|s| s.generator_apply_posture == apply),
            "missing apply {}",
            apply.as_str()
        );
    }
    for effect in FileEffectClass::ALL {
        assert!(
            packet
                .generator_sheets
                .iter()
                .any(|s| s.file_effect_class == effect),
            "missing file effect {}",
            effect.as_str()
        );
    }
    for ownership in FileOwnershipClass::ALL {
        assert!(
            packet
                .generator_sheets
                .iter()
                .any(|s| s.file_ownership_class == ownership),
            "missing ownership {}",
            ownership.as_str()
        );
    }
    for mutation in M5RunConfigMutationClass::ALL {
        assert!(
            packet
                .run_config_cards
                .iter()
                .any(|c| c.run_config_mutation_class == mutation),
            "missing mutation {}",
            mutation.as_str()
        );
    }
    for boundary in M5ExecutionBoundaryClass::ALL {
        assert!(
            packet
                .run_config_cards
                .iter()
                .any(|c| c.execution_boundary_class == boundary),
            "missing boundary {}",
            boundary.as_str()
        );
    }
    for target in RunTargetKind::ALL {
        assert!(
            packet
                .run_config_cards
                .iter()
                .any(|c| c.run_target_kind == target),
            "missing target {}",
            target.as_str()
        );
    }
    for profile in LaunchProfileClass::ALL {
        assert!(
            packet
                .run_config_cards
                .iter()
                .any(|c| c.launch_profile_class == profile),
            "missing profile {}",
            profile.as_str()
        );
    }
    for toolchain in ToolchainReadiness::ALL {
        assert!(
            packet
                .run_config_cards
                .iter()
                .any(|c| c.toolchain_readiness == toolchain),
            "missing toolchain {}",
            toolchain.as_str()
        );
    }
    for posture in WriteEffectPosture::ALL {
        assert!(
            packet
                .generator_sheets
                .iter()
                .any(|s| s.write_effect_posture == posture)
                || packet
                    .run_config_cards
                    .iter()
                    .any(|c| c.write_effect_posture == posture),
            "missing write-effect posture {}",
            posture.as_str()
        );
    }
    for recovery in RecoveryPath::ALL {
        assert!(
            packet
                .generator_sheets
                .iter()
                .any(|s| s.recovery_kind == recovery)
                || packet
                    .run_config_cards
                    .iter()
                    .any(|c| c.recovery_kind == recovery),
            "missing recovery path {}",
            recovery.as_str()
        );
    }
}

#[test]
fn every_component_offers_mandatory_actions_labels_and_keyboard_route() {
    let packet = seeded_generator_run_config_controls();
    for sheet in &packet.generator_sheets {
        for action in GeneratorSheetAction::MANDATORY {
            assert!(sheet.sheet_actions.contains(&action));
        }
        assert!(sheet.declares_mandatory_labels());
        assert!(sheet
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
    for card in &packet.run_config_cards {
        for action in RunConfigCardAction::MANDATORY {
            assert!(card.card_actions.contains(&action));
        }
        assert!(card.declares_mandatory_labels());
        assert!(card
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn misrepresented_generator_posture_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.generator_sheets[1].claims_no_op_write = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::GeneratorPostureMisrepresented));
}

#[test]
fn writing_generator_claiming_no_op_fails() {
    let mut packet = seeded_generator_run_config_controls();
    let sheet = packet
        .generator_sheets
        .iter_mut()
        .find(|s| s.posture_disclosure().has_side_effect)
        .expect("a writing generator");
    sheet.claims_no_op_write = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::WriteClaimsNoOp));
}

#[test]
fn writing_run_config_claiming_no_op_fails() {
    let mut packet = seeded_generator_run_config_controls();
    let card = packet
        .run_config_cards
        .iter_mut()
        .find(|c| c.posture_disclosure().has_side_effect)
        .expect("a writing run-config card");
    card.claims_no_op_write = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::WriteClaimsNoOp));
}

#[test]
fn generator_claiming_recovery_without_path_fails() {
    let mut packet = seeded_generator_run_config_controls();
    let sheet = packet
        .generator_sheets
        .iter_mut()
        .find(|s| !s.has_recovery_path)
        .expect("a sheet without a recovery path");
    sheet.recovery_kind = RecoveryPath::Rollback;
    sheet.recovery_ref = "diff:fake".to_owned();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::RecoveryClaimedWithoutPath));
}

#[test]
fn generator_with_path_but_no_recovery_kind_fails() {
    let mut packet = seeded_generator_run_config_controls();
    let sheet = packet
        .generator_sheets
        .iter_mut()
        .find(|s| s.has_recovery_path)
        .expect("a sheet with a recovery path");
    sheet.recovery_kind = RecoveryPath::ForwardFixOnly;
    sheet.recovery_ref = String::new();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::RecoveryUnresolved));
}

#[test]
fn missing_no_recovery_note_fails() {
    let mut packet = seeded_generator_run_config_controls();
    let sheet = packet
        .generator_sheets
        .iter_mut()
        .find(|s| s.posture_disclosure().needs_no_recovery_note)
        .expect("a blocked write with no recovery");
    sheet.no_recovery_note = String::new();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::NoRecoveryNoteMissing));
}

#[test]
fn missing_impact_label_fails() {
    let mut packet = seeded_generator_run_config_controls();
    let sheet = packet
        .generator_sheets
        .iter_mut()
        .find(|s| s.posture_disclosure().has_side_effect)
        .expect("a writing generator");
    sheet.dependency_config_impact_label = String::new();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::ImpactLabelMissing));
}

#[test]
fn missing_ownership_label_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.generator_sheets[0].ownership_label = String::new();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::OwnershipLabelMissing));
}

#[test]
fn file_effect_count_mismatch_fails() {
    let mut packet = seeded_generator_run_config_controls();
    // A creates-file sheet with zero created paths is a mismatch.
    let sheet = packet
        .generator_sheets
        .iter_mut()
        .find(|s| s.file_effect_class == FileEffectClass::CreatesFile)
        .expect("a creates-file sheet");
    sheet.created_path_count = 0;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::FileEffectCountMismatch));
}

#[test]
fn missing_launch_command_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.run_config_cards[0].launch_command_label = String::new();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::LaunchCommandMissing));
}

#[test]
fn missing_required_toolchain_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.run_config_cards[0].required_toolchain_label = String::new();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::RequiredToolchainMissing));
}

#[test]
fn misrepresented_execution_boundary_fails() {
    let mut packet = seeded_generator_run_config_controls();
    // Flip the local-execution flag on a local-process card.
    let card = packet
        .run_config_cards
        .iter_mut()
        .find(|c| c.execution_boundary_class == M5ExecutionBoundaryClass::LocalProcess)
        .expect("a local-process card");
    card.is_local_execution = false;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::ExecutionBoundaryMisrepresented));
}

#[test]
fn missing_mandatory_generator_action_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.generator_sheets[0]
        .sheet_actions
        .retain(|a| *a != GeneratorSheetAction::ReviewCreatedAndModifiedDiff);
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::GeneratorSheetActionsIncomplete));
}

#[test]
fn missing_mandatory_run_config_action_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.run_config_cards[0]
        .card_actions
        .retain(|a| *a != RunConfigCardAction::InspectExecutionBoundary);
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::RunConfigCardActionsIncomplete));
}

#[test]
fn each_generator_hard_invariant_fails_when_set() {
    let mut packet = seeded_generator_run_config_controls();
    packet.generator_sheets[0].implies_no_op_write_without_review = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::ImpliesNoOpWriteWithoutReview));

    let mut packet = seeded_generator_run_config_controls();
    packet.generator_sheets[0].hides_dependency_or_config_impact = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::DependencyOrConfigImpactHidden));

    let mut packet = seeded_generator_run_config_controls();
    packet.generator_sheets[0].omits_rollback_or_regenerate_path = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::RecoveryPathOmitted));

    let mut packet = seeded_generator_run_config_controls();
    packet.generator_sheets[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::AlternateStateLabelInvented));
}

#[test]
fn run_config_hides_boundary_or_toolchain_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.run_config_cards[0].hides_execution_boundary_or_toolchain = true;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::ExecutionBoundaryOrToolchainHidden));
}

#[test]
fn missing_write_effect_note_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.generator_sheets[0].write_effect_note = String::new();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::WriteEffectNoteMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.run_config_cards[0].context_note = String::new();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::ContextNoteMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet
        .generator_run_config_review
        .execution_boundary_always_visible_before_dispatch = false;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet
        .consumer_projection
        .execution_boundary_and_toolchain_visible_before_dispatch = false;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_generator_run_config_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GeneratorRunConfigControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let packet = seeded_generator_run_config_controls();
    let summary = packet.render_markdown_summary();
    for sheet in &packet.generator_sheets {
        assert!(summary.contains(&sheet.generator_name_label));
    }
    for card in &packet.run_config_cards {
        assert!(summary.contains(&card.config_name_label));
    }
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let packet = seeded_generator_run_config_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + packet.generator_sheets.len() + packet.run_config_cards.len()
    );
    assert!(lines[0].starts_with("component,id,primary_class,"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_generator_run_config_controls_export()
        .expect("checked generator run config controls export validates");
    assert_eq!(
        from_disk,
        seeded_generator_run_config_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn scenario_fixtures_validate_and_keep_full_coverage() {
    for packet in [
        seeded_generator_run_config_controls_writing_generator(),
        seeded_generator_run_config_controls_remote_run_config(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn checked_scenario_fixtures_validate_and_match_seed_builders() {
    let writing: GeneratorPreviewRunConfigControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-generator-preview-run-config-controls/writing_generator.json"
        )))
        .expect("writing-generator fixture parses");
    assert!(writing.validate().is_empty());
    assert_eq!(
        writing,
        seeded_generator_run_config_controls_writing_generator()
    );

    let remote: GeneratorPreviewRunConfigControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-generator-preview-run-config-controls/remote_run_config.json"
        )))
        .expect("remote-run-config fixture parses");
    assert!(remote.validate().is_empty());
    assert_eq!(
        remote,
        seeded_generator_run_config_controls_remote_run_config()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_generator_run_config_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("secret"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
