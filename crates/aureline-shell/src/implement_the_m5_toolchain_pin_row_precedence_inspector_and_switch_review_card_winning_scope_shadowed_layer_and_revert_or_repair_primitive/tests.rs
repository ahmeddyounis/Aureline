use super::*;

fn layer(
    scope: M5PinScope,
    source: M5ToolchainSourceClass,
    selection_repr: &str,
    present: bool,
) -> M5PinCandidateLayer {
    M5PinCandidateLayer {
        scope,
        source,
        selection_repr: selection_repr.to_owned(),
        present,
    }
}

fn resolved_project(title: &str) -> M5ToolchainSelectionResolutionInput {
    M5ToolchainSelectionResolutionInput {
        target_title: title.to_owned(),
        target_kind: M5ToolchainTargetKind::Interpreter,
        candidate_layers: vec![layer(
            M5PinScope::ProjectScope,
            M5ToolchainSourceClass::PinFile,
            "py-3.12",
            true,
        )],
        selection_health: M5SelectionHealth::Healthy,
        switch_request: None,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_lone_project_pin_is_pinned_resolved() {
    let resolved = resolve_toolchain_selection(&resolved_project("x")).expect("resolves");
    assert_eq!(resolved.winning_scope, M5PinScope::ProjectScope);
    assert_eq!(resolved.pin_state, M5ToolchainPinState::PinnedResolved);
    assert!(!resolved.selection_is_degraded);
    assert!(!resolved.shadows_durable_pin);
    assert!(resolved.discloses_shadowed_pins);
    assert_eq!(
        resolved.available_actions,
        vec![M5PinAction::ReviewPrecedence]
    );
    assert!(resolved.switch_review.is_none());
}

#[test]
fn resolver_winning_layer_is_lowest_precedence_rank() {
    let input = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![
            layer(
                M5PinScope::WorkspaceScope,
                M5ToolchainSourceClass::WorkspaceSetting,
                "py-3.11",
                true,
            ),
            layer(
                M5PinScope::PolicyScope,
                M5ToolchainSourceClass::WorkspaceSetting,
                "py-3.10",
                true,
            ),
            layer(
                M5PinScope::ProjectScope,
                M5ToolchainSourceClass::PinFile,
                "py-3.12",
                true,
            ),
        ],
        ..resolved_project("x")
    };
    let resolved = resolve_toolchain_selection(&input).expect("resolves");
    assert_eq!(resolved.winning_scope, M5PinScope::PolicyScope);
    assert!(resolved.ordered_layers[0].is_winner);
    assert_eq!(resolved.ordered_layers[0].scope, M5PinScope::PolicyScope);
    // Every non-winning layer carries an explicit shadow reason (AC1).
    for layer in resolved.ordered_layers.iter().filter(|l| !l.is_winner) {
        assert!(layer.shadow_reason.is_some());
    }
    // Ordered ascending by precedence rank.
    let ranks: Vec<u8> = resolved
        .ordered_layers
        .iter()
        .map(|l| l.precedence_rank)
        .collect();
    let mut sorted = ranks.clone();
    sorted.sort_unstable();
    assert_eq!(ranks, sorted);
}

#[test]
fn resolver_override_shadowing_durable_pin_is_pin_overridden_and_discloses_shadow() {
    let input = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![
            layer(
                M5PinScope::PolicyScope,
                M5ToolchainSourceClass::WorkspaceSetting,
                "py-3.11",
                true,
            ),
            layer(
                M5PinScope::ProjectScope,
                M5ToolchainSourceClass::PinFile,
                "py-3.12",
                true,
            ),
        ],
        ..resolved_project("x")
    };
    let resolved = resolve_toolchain_selection(&input).expect("resolves");
    assert_eq!(resolved.pin_state, M5ToolchainPinState::PinOverridden);
    assert!(resolved.shadows_durable_pin);
    assert!(resolved
        .available_actions
        .contains(&M5PinAction::ClearOverride));
    assert!(resolved
        .available_actions
        .contains(&M5PinAction::RevertToShadowedPin));
}

#[test]
fn resolver_two_durable_pins_disagree_is_pin_conflict() {
    let input = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![
            layer(
                M5PinScope::WorkspaceScope,
                M5ToolchainSourceClass::WorkspaceSetting,
                "bash-5.2",
                true,
            ),
            layer(
                M5PinScope::UserScope,
                M5ToolchainSourceClass::VersionManager,
                "zsh-5.9",
                true,
            ),
        ],
        ..resolved_project("x")
    };
    let resolved = resolve_toolchain_selection(&input).expect("resolves");
    assert_eq!(resolved.winning_scope, M5PinScope::WorkspaceScope);
    assert_eq!(resolved.pin_state, M5ToolchainPinState::PinConflict);
    assert!(resolved.shadows_durable_pin);
}

#[test]
fn resolver_only_default_present_is_unpinned() {
    let input = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![layer(
            M5PinScope::GlobalDefaultScope,
            M5ToolchainSourceClass::SystemInstalled,
            "py-lts",
            true,
        )],
        ..resolved_project("x")
    };
    let resolved = resolve_toolchain_selection(&input).expect("resolves");
    assert_eq!(resolved.pin_state, M5ToolchainPinState::Unpinned);
    assert_eq!(resolved.winning_scope, M5PinScope::GlobalDefaultScope);
}

#[test]
fn resolver_missing_selection_is_pinned_missing_fallback_with_repair() {
    let input = M5ToolchainSelectionResolutionInput {
        selection_health: M5SelectionHealth::MissingUnavailable,
        ..resolved_project("x")
    };
    let resolved = resolve_toolchain_selection(&input).expect("resolves");
    assert_eq!(
        resolved.pin_state,
        M5ToolchainPinState::PinnedMissingFallback
    );
    assert!(resolved.selection_is_degraded);
    assert!(resolved
        .available_actions
        .contains(&M5PinAction::RepairSelection));
}

#[test]
fn resolver_degraded_health_always_keeps_repair_action() {
    for health in [
        M5SelectionHealth::DegradedStale,
        M5SelectionHealth::MismatchedVersion,
        M5SelectionHealth::MissingUnavailable,
    ] {
        let input = M5ToolchainSelectionResolutionInput {
            selection_health: health,
            ..resolved_project("x")
        };
        let resolved = resolve_toolchain_selection(&input).expect("resolves");
        assert!(resolved.selection_is_degraded);
        assert!(
            resolved
                .available_actions
                .contains(&M5PinAction::RepairSelection),
            "health {} dropped the repair action",
            health.as_str()
        );
    }
}

#[test]
fn resolver_switch_derives_blast_radius_and_reversibility() {
    // Reconnect → multi-target, manual reversal.
    let reconnect = M5ToolchainSelectionResolutionInput {
        switch_request: Some(M5SwitchRequest {
            to_scope: M5PinScope::ProjectScope,
            to_source: M5ToolchainSourceClass::PinFile,
            to_selection_repr: "py-3.11".to_owned(),
            requires_restart: true,
            requires_reconnect: true,
            newly_blocked_actions: vec!["debug".to_owned()],
            safe_local_only_fallback: false,
        }),
        ..resolved_project("x")
    };
    let review = resolve_toolchain_selection(&reconnect)
        .unwrap()
        .switch_review
        .expect("switch review present");
    assert_eq!(review.blast_radius, M5RepairBlastRadius::MultiTargetScoped);
    assert_eq!(
        review.reversibility,
        M5ReversibilityClass::ReversalRequiresManualSteps
    );
    assert!(review.blocks_actions_after_switch);

    // Safe local-only fallback, no reconnect → fully reversible checkpoint.
    let fallback = M5ToolchainSelectionResolutionInput {
        switch_request: Some(M5SwitchRequest {
            to_scope: M5PinScope::WorkspaceScope,
            to_source: M5ToolchainSourceClass::WorkspaceSetting,
            to_selection_repr: "py-3.12".to_owned(),
            requires_restart: true,
            requires_reconnect: false,
            newly_blocked_actions: vec![],
            safe_local_only_fallback: true,
        }),
        ..resolved_project("x")
    };
    let review = resolve_toolchain_selection(&fallback)
        .unwrap()
        .switch_review
        .expect("switch review present");
    assert_eq!(review.blast_radius, M5RepairBlastRadius::ToolchainScoped);
    assert_eq!(
        review.reversibility,
        M5ReversibilityClass::FullyReversibleCheckpoint
    );
    assert!(!review.blocks_actions_after_switch);
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_title = M5ToolchainSelectionResolutionInput {
        target_title: "  ".to_owned(),
        ..resolved_project("x")
    };
    assert_eq!(
        resolve_toolchain_selection(&empty_title),
        Err(M5ToolchainSelectionResolutionError::EmptyTargetTitle)
    );

    let no_layers = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![],
        ..resolved_project("x")
    };
    assert_eq!(
        resolve_toolchain_selection(&no_layers),
        Err(M5ToolchainSelectionResolutionError::EmptyCandidateLayers)
    );

    let none_present = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![layer(
            M5PinScope::ProjectScope,
            M5ToolchainSourceClass::PinFile,
            "py-3.12",
            false,
        )],
        ..resolved_project("x")
    };
    assert_eq!(
        resolve_toolchain_selection(&none_present),
        Err(M5ToolchainSelectionResolutionError::NoPresentLayer)
    );

    let dup_scope = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![
            layer(
                M5PinScope::ProjectScope,
                M5ToolchainSourceClass::PinFile,
                "py-3.12",
                true,
            ),
            layer(
                M5PinScope::ProjectScope,
                M5ToolchainSourceClass::VersionManager,
                "py-3.11",
                true,
            ),
        ],
        ..resolved_project("x")
    };
    assert_eq!(
        resolve_toolchain_selection(&dup_scope),
        Err(M5ToolchainSelectionResolutionError::DuplicateScope)
    );

    let empty_selection = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![layer(
            M5PinScope::ProjectScope,
            M5ToolchainSourceClass::PinFile,
            "  ",
            true,
        )],
        ..resolved_project("x")
    };
    assert_eq!(
        resolve_toolchain_selection(&empty_selection),
        Err(M5ToolchainSelectionResolutionError::EmptySelection)
    );

    let empty_switch = M5ToolchainSelectionResolutionInput {
        switch_request: Some(M5SwitchRequest {
            to_scope: M5PinScope::WorkspaceScope,
            to_source: M5ToolchainSourceClass::WorkspaceSetting,
            to_selection_repr: " ".to_owned(),
            requires_restart: false,
            requires_reconnect: false,
            newly_blocked_actions: vec![],
            safe_local_only_fallback: true,
        }),
        ..resolved_project("x")
    };
    assert_eq!(
        resolve_toolchain_selection(&empty_switch),
        Err(M5ToolchainSelectionResolutionError::EmptySwitchSelection)
    );

    let forbidden = M5ToolchainSelectionResolutionInput {
        candidate_layers: vec![layer(
            M5PinScope::ProjectScope,
            M5ToolchainSourceClass::PinFile,
            "py from https://example.test",
            true,
        )],
        ..resolved_project("x")
    };
    assert_eq!(
        resolve_toolchain_selection(&forbidden),
        Err(M5ToolchainSelectionResolutionError::ForbiddenToolchainMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_selector_surface() {
    let packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .selector_rows
        .iter()
        .map(|r| r.selector_surface)
        .collect();
    for surface in M5EnvironmentSelectorSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing selector surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.selector_rows.len(),
        M5EnvironmentSelectorSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    for row in &packet.selector_rows {
        for part in M5ToolchainPinRowPart::MANDATORY {
            assert!(row.pin_row_parts.contains(&part));
        }
        for part in M5PrecedenceInspectorPart::MANDATORY {
            assert!(row.inspector_parts.contains(&part));
        }
        for part in M5SwitchReviewCardPart::MANDATORY {
            assert!(row.switch_card_parts.contains(&part));
        }
        for field in M5ToolchainSelectionExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    let cases: Vec<&M5ToolchainSelectionResolutionCase> = packet
        .selector_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for kind in M5ToolchainTargetKind::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.target_kind == kind),
            "no worked resolution exercises target kind {}",
            kind.as_str()
        );
    }
    for state in M5ToolchainPinState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.pin_state == state),
            "no worked resolution exercises pin state {}",
            state.as_str()
        );
    }
    for scope in M5PinScope::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.winning_scope == scope),
            "no worked resolution exercises winning scope {}",
            scope.as_str()
        );
    }
    for health in M5SelectionHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.selection_health == health),
            "no worked resolution exercises selection health {}",
            health.as_str()
        );
    }
    for source in M5ToolchainSourceClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.winning_source == source),
            "no worked resolution exercises winning source {}",
            source.as_str()
        );
    }
    for action in M5PinAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no worked resolution exercises pin action {}",
            action.as_str()
        );
    }
    // The reachable switch blast radii all appear.
    let radii: std::collections::BTreeSet<_> = cases
        .iter()
        .filter_map(|c| c.resolved.switch_review.as_ref())
        .map(|r| r.blast_radius)
        .collect();
    for expected in [
        M5RepairBlastRadius::WorkspaceScoped,
        M5RepairBlastRadius::ToolchainScoped,
        M5RepairBlastRadius::HostEnvironmentScoped,
        M5RepairBlastRadius::MultiTargetScoped,
    ] {
        assert!(
            radii.contains(&expected),
            "no switch review exercises blast radius {}",
            expected.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    for row in &packet.selector_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.selector_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_selector_surface_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet
        .selector_rows
        .retain(|row| row.selector_surface != M5EnvironmentSelectorSurface::SdkSelector);
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::RequiredSelectorMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.vocabulary_set.pin_scopes.pop();
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_pin_row_part_missing_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.selector_rows[0]
        .pin_row_parts
        .retain(|p| *p != M5ToolchainPinRowPart::WinningScope);
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::MandatoryPinRowPartMissing));
}

#[test]
fn mandatory_inspector_part_missing_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.selector_rows[0]
        .inspector_parts
        .retain(|p| *p != M5PrecedenceInspectorPart::ShadowExplanation);
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::MandatoryInspectorPartMissing));
}

#[test]
fn mandatory_switch_card_part_missing_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.selector_rows[0]
        .switch_card_parts
        .retain(|p| *p != M5SwitchReviewCardPart::SafeLocalOnlyFallback);
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::MandatorySwitchCardPartMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.selector_rows[0]
        .export_fields
        .retain(|f| *f != M5ToolchainSelectionExportField::PinState);
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.selector_rows[0].example_resolutions[0]
        .resolved
        .pin_state = M5ToolchainPinState::PinConflict;
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn shadow_disclosure_unproven_fails_when_no_shadow_example_present() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    // Replace every example with a lone-pin resolution that shadows nothing.
    for row in &mut packet.selector_rows {
        row.example_resolutions = vec![M5ToolchainSelectionResolutionCase::resolved(
            M5ToolchainSelectionResolutionInput {
                target_title: "lone".to_owned(),
                target_kind: M5ToolchainTargetKind::Interpreter,
                candidate_layers: vec![layer(
                    M5PinScope::ProjectScope,
                    M5ToolchainSourceClass::PinFile,
                    "py-3.12",
                    true,
                )],
                selection_health: M5SelectionHealth::MissingUnavailable,
                switch_request: Some(M5SwitchRequest {
                    to_scope: M5PinScope::WorkspaceScope,
                    to_source: M5ToolchainSourceClass::WorkspaceSetting,
                    to_selection_repr: "py-3.11".to_owned(),
                    requires_restart: true,
                    requires_reconnect: false,
                    newly_blocked_actions: vec![],
                    safe_local_only_fallback: true,
                }),
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::ShadowDisclosureUnproven));
}

#[test]
fn switch_blast_radius_unproven_fails_when_no_switch_example_present() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    for row in &mut packet.selector_rows {
        for case in &mut row.example_resolutions {
            case.input.switch_request = None;
            case.resolved = resolve_toolchain_selection(&case.input).unwrap();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::SwitchBlastRadiusUnproven));
}

#[test]
fn degraded_repair_unproven_fails_when_no_degraded_example_present() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    for row in &mut packet.selector_rows {
        for case in &mut row.example_resolutions {
            case.input.selection_health = M5SelectionHealth::Healthy;
            case.resolved = resolve_toolchain_selection(&case.input).unwrap();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::DegradedRepairUnproven));
}

#[test]
fn selector_invariant_violation_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.selector_rows[0].silently_shadows_durable_pin = true;
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::SelectorInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.selector_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet
        .governance_review
        .override_never_silently_shadows_durable_pin = false;
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet
        .consumer_projection
        .pin_resolver_reads_single_precedence_source = false;
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ToolchainPinSwitchReviewPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_selector_surface() {
    let summary =
        seeded_m5_toolchain_pin_switch_review_primitive_packet().render_markdown_summary();
    for surface in M5EnvironmentSelectorSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing selector surface {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_toolchain_pin_switch_review_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5EnvironmentSelectorSurface::ALL.len());
    assert!(lines[0].starts_with("selector_surface,qualification,owner,"));
    for surface in M5EnvironmentSelectorSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing selector surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_toolchain_pin_switch_review_primitive_export()
        .expect("checked M5 toolchain-pin / switch-review primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_toolchain_pin_switch_review_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_toolchain_pin_switch_review_primitive_repair_panel_beta_narrowed(),
        seeded_m5_toolchain_pin_switch_review_primitive_runtime_target_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.selector_rows.len(),
            M5EnvironmentSelectorSurface::ALL.len()
        );
    }

    let repair = seeded_m5_toolchain_pin_switch_review_primitive_repair_panel_beta_narrowed();
    let row = repair
        .selector_rows
        .iter()
        .find(|r| r.selector_surface == M5EnvironmentSelectorSurface::RepairPanelSelector)
        .expect("repair-panel selector row present");
    assert_eq!(row.qualification, M5RuntimeBoundaryQualificationClass::Beta);

    let runtime = seeded_m5_toolchain_pin_switch_review_primitive_runtime_target_preview_narrowed();
    let row = runtime
        .selector_rows
        .iter()
        .find(|r| r.selector_surface == M5EnvironmentSelectorSurface::RuntimeTargetSwitcher)
        .expect("runtime-target switcher row present");
    assert_eq!(
        row.qualification,
        M5RuntimeBoundaryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let repair: M5ToolchainPinSwitchReviewPrimitivePacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-toolchain-pin-switch-review-primitive/repair_panel_beta_narrowed.json"
        )
    ))
    .expect("repair fixture parses");
    assert!(repair.validate().is_empty());
    assert_eq!(
        repair,
        seeded_m5_toolchain_pin_switch_review_primitive_repair_panel_beta_narrowed()
    );

    let runtime: M5ToolchainPinSwitchReviewPrimitivePacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-toolchain-pin-switch-review-primitive/runtime_target_preview_narrowed.json"
        )
    ))
    .expect("runtime fixture parses");
    assert!(runtime.validate().is_empty());
    assert_eq!(
        runtime,
        seeded_m5_toolchain_pin_switch_review_primitive_runtime_target_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_toolchain_pin_switch_review_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
