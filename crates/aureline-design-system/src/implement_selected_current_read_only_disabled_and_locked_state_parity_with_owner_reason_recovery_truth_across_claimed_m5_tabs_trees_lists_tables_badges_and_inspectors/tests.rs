use super::*;

fn selected_tab() -> M5SelectionOrLockResolutionInput {
    M5SelectionOrLockResolutionInput {
        item_kind: M5SelectionOrLockItemKind::Tab,
        selection_or_lock_state: M5SharedComponentStateClass::Selected,
        lock_owner: M5LockOwnerClass::NoLock,
        state_cause: M5StateCauseClass::UnknownCause,
        recovery_available: false,
        inspectable: true,
        high_contrast_active: false,
        item_identity_ref: "item:editor-tabs.readme".to_owned(),
        state_style_ref: "token:state.tab.selected".to_owned(),
        disclosure_ref: String::new(),
    }
}

fn locked_tree_item() -> M5SelectionOrLockResolutionInput {
    M5SelectionOrLockResolutionInput {
        item_kind: M5SelectionOrLockItemKind::TreeItem,
        selection_or_lock_state: M5SharedComponentStateClass::Locked,
        lock_owner: M5LockOwnerClass::PolicyLock,
        state_cause: M5StateCauseClass::PolicyCause,
        recovery_available: true,
        inspectable: false,
        high_contrast_active: false,
        item_identity_ref: "item:explorer-tree.protected-config".to_owned(),
        state_style_ref: "token:state.tree_item.locked".to_owned(),
        disclosure_ref: "policy:workspace.admin-lock".to_owned(),
    }
}

// ---- selection-or-lock-state resolver -----------------------------------

#[test]
fn selected_state_is_selected_treatment_with_selection_marker() {
    let resolved = resolve_selection_or_lock_state_contract(&selected_tab()).expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5SelectionOrLockPresentation::SelectedTreatment
    );
    assert!(!resolved.explainable);
    assert!(!resolved.owner_disclosed);
    assert_eq!(
        resolved.required_non_color_cues,
        vec![M5SelectionOrLockCue::SelectionMarker]
    );
    assert!(resolved.selected_and_current_stay_distinct);
    assert!(resolved.read_only_preserves_inspectability);
    assert!(resolved.lock_never_hidden_behind_disabled);
    assert!(resolved.no_color_only_signaling);
    assert!(resolved.names_owner_and_recovery_when_explainable);
    assert!(resolved.keyboard_and_screen_reader_explainable);
    assert!(resolved.driven_by_shared_state_contract);
}

#[test]
fn current_state_is_distinct_from_selected() {
    let selected = resolve_selection_or_lock_state_contract(&selected_tab()).expect("resolves");
    let current = resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
        selection_or_lock_state: M5SharedComponentStateClass::Current,
        ..selected_tab()
    })
    .expect("resolves");
    assert_ne!(selected.presentation, current.presentation);
    assert_eq!(
        current.presentation,
        M5SelectionOrLockPresentation::CurrentTreatment
    );
    assert_eq!(
        current.required_non_color_cues,
        vec![M5SelectionOrLockCue::CurrentLocationIndicator]
    );
    // Selected and current never share a cue, so they can never collapse.
    assert!(!current
        .required_non_color_cues
        .contains(&M5SelectionOrLockCue::SelectionMarker));
}

#[test]
fn locked_state_names_owner_and_recovery_disclosures() {
    let resolved = resolve_selection_or_lock_state_contract(&locked_tree_item()).expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5SelectionOrLockPresentation::LockedTreatment
    );
    assert!(resolved.explainable);
    assert!(resolved.owner_disclosed);
    assert_eq!(resolved.lock_owner, M5LockOwnerClass::PolicyLock);
    assert!(resolved
        .required_non_color_cues
        .contains(&M5SelectionOrLockCue::LockGlyphWithOwner));
    assert!(resolved
        .required_non_color_cues
        .contains(&M5SelectionOrLockCue::RecoveryAffordance));
    for trigger in [
        M5StateDisclosureTrigger::StateCauseRequired,
        M5StateDisclosureTrigger::OwnerRequired,
        M5StateDisclosureTrigger::BlockReasonRequired,
        M5StateDisclosureTrigger::RecoveryActionRequired,
        M5StateDisclosureTrigger::SilentStyleOnlyForbidden,
    ] {
        assert!(
            resolved.required_disclosures.contains(&trigger),
            "locked state missing disclosure {}",
            trigger.as_str()
        );
    }
}

#[test]
fn read_only_state_preserves_inspectability_and_names_cause() {
    let resolved = resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
        item_kind: M5SelectionOrLockItemKind::TableRow,
        selection_or_lock_state: M5SharedComponentStateClass::ReadOnly,
        lock_owner: M5LockOwnerClass::SourceLock,
        state_cause: M5StateCauseClass::PreconditionCause,
        inspectable: true,
        item_identity_ref: "item:result-grid.derived-column".to_owned(),
        state_style_ref: "token:state.table_row.read_only".to_owned(),
        disclosure_ref: "readonly:generated-column.derived-from-query".to_owned(),
        ..selected_tab()
    })
    .expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5SelectionOrLockPresentation::ReadOnlyTreatment
    );
    assert!(resolved.explainable);
    assert!(!resolved.owner_disclosed);
    assert!(resolved
        .required_non_color_cues
        .contains(&M5SelectionOrLockCue::ReadOnlyGlyphInspectable));
    assert!(resolved
        .required_disclosures
        .contains(&M5StateDisclosureTrigger::StateCauseRequired));
}

#[test]
fn disabled_state_names_reason_without_a_lock_owner() {
    let resolved = resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
        item_kind: M5SelectionOrLockItemKind::ListRow,
        selection_or_lock_state: M5SharedComponentStateClass::Disabled,
        lock_owner: M5LockOwnerClass::NoLock,
        state_cause: M5StateCauseClass::PreconditionCause,
        inspectable: false,
        item_identity_ref: "item:results-list.unmet-prerequisite-row".to_owned(),
        state_style_ref: "token:state.list_row.disabled".to_owned(),
        disclosure_ref: "reason:prerequisite-unmet.select-target-first".to_owned(),
        ..selected_tab()
    })
    .expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5SelectionOrLockPresentation::DisabledTreatment
    );
    assert!(resolved
        .required_non_color_cues
        .contains(&M5SelectionOrLockCue::DisabledDimWithReason));
    assert!(resolved
        .required_disclosures
        .contains(&M5StateDisclosureTrigger::BlockReasonRequired));
    assert!(!resolved.owner_disclosed);
}

#[test]
fn disabled_masking_a_lock_is_rejected() {
    // The acceptance criterion: a disabled control must not hide a state that should be modeled as
    // locked.
    let err = resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
        selection_or_lock_state: M5SharedComponentStateClass::Disabled,
        lock_owner: M5LockOwnerClass::PolicyLock,
        disclosure_ref: "policy:workspace.admin-lock".to_owned(),
        ..selected_tab()
    });
    assert_eq!(
        err,
        Err(M5SelectionOrLockResolutionError::DisabledMaskingLock)
    );
}

#[test]
fn locked_without_owner_is_rejected() {
    let err = resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
        selection_or_lock_state: M5SharedComponentStateClass::Locked,
        lock_owner: M5LockOwnerClass::NoLock,
        disclosure_ref: "policy:workspace.admin-lock".to_owned(),
        ..selected_tab()
    });
    assert_eq!(err, Err(M5SelectionOrLockResolutionError::LockWithoutOwner));
}

#[test]
fn read_only_without_inspectability_is_rejected() {
    let err = resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
        selection_or_lock_state: M5SharedComponentStateClass::ReadOnly,
        inspectable: false,
        disclosure_ref: "readonly:x.y".to_owned(),
        ..selected_tab()
    });
    assert_eq!(
        err,
        Err(M5SelectionOrLockResolutionError::ReadOnlyNotInspectable)
    );
}

#[test]
fn explainable_state_without_disclosure_detail_is_rejected() {
    let err = resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
        selection_or_lock_state: M5SharedComponentStateClass::Locked,
        lock_owner: M5LockOwnerClass::PolicyLock,
        disclosure_ref: "   ".to_owned(),
        ..selected_tab()
    });
    assert_eq!(
        err,
        Err(M5SelectionOrLockResolutionError::MissingDisclosureDetail)
    );
}

#[test]
fn resolver_rejects_non_selection_or_lock_state() {
    for state in [
        M5SharedComponentStateClass::Default,
        M5SharedComponentStateClass::Hover,
        M5SharedComponentStateClass::FocusVisible,
        M5SharedComponentStateClass::PressedActive,
        M5SharedComponentStateClass::Loading,
        M5SharedComponentStateClass::Pending,
        M5SharedComponentStateClass::WarningError,
        M5SharedComponentStateClass::Degraded,
    ] {
        assert_eq!(
            resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
                selection_or_lock_state: state,
                ..selected_tab()
            }),
            Err(M5SelectionOrLockResolutionError::NonSelectionOrLockState),
            "state {} was not rejected as non-selection-or-lock",
            state.as_str()
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
            item_identity_ref: " ".to_owned(),
            ..selected_tab()
        }),
        Err(M5SelectionOrLockResolutionError::EmptyItemIdentity)
    );
    assert_eq!(
        resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
            state_style_ref: "".to_owned(),
            ..selected_tab()
        }),
        Err(M5SelectionOrLockResolutionError::EmptyStateStyleRef)
    );
    assert_eq!(
        resolve_selection_or_lock_state_contract(&M5SelectionOrLockResolutionInput {
            selection_or_lock_state: M5SharedComponentStateClass::Locked,
            lock_owner: M5LockOwnerClass::PolicyLock,
            disclosure_ref: "policy:https://evil.example/x".to_owned(),
            ..selected_tab()
        }),
        Err(M5SelectionOrLockResolutionError::ForbiddenStateMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_selection_or_lock_state_contract_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_item_kind() {
    let packet = seeded_m5_selection_or_lock_state_contract_packet();
    let present: std::collections::BTreeSet<_> = packet.rows.iter().map(|r| r.item_kind).collect();
    for item in M5SelectionOrLockItemKind::ALL {
        assert!(
            present.contains(&item),
            "missing item kind {}",
            item.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5SelectionOrLockItemKind::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_labels() {
    let packet = seeded_m5_selection_or_lock_state_contract_packet();
    for row in &packet.rows {
        for part in M5SelectionOrLockAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5SelectionOrLockExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for label in M5ComponentStateRequiredLabel::MANDATORY {
            assert!(row.required_labels.contains(&label));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable));
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded));
        assert!(!row.state_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_selection_or_lock_state_contract_packet();
    let cases: Vec<&M5SelectionOrLockResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .collect();

    for state in selection_or_lock_states() {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.selection_or_lock_state == state),
            "no example exercises selection-or-lock state {}",
            state.as_str()
        );
    }
    for posture in M5SelectionOrLockPresentation::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.presentation == posture),
            "no example exercises presentation {}",
            posture.as_str()
        );
    }
    for cue in M5SelectionOrLockCue::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.required_non_color_cues.contains(&cue)),
            "no example exercises non-color cue {}",
            cue.as_str()
        );
    }
    for trigger in M5StateDisclosureTrigger::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.required_disclosures.contains(&trigger)),
            "no example exercises disclosure {}",
            trigger.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity_and_guarantees() {
    let packet = seeded_m5_selection_or_lock_state_contract_packet();
    for row in &packet.rows {
        for case in &row.state_examples {
            assert!(
                case.is_self_consistent(),
                "state case for {} drifted",
                row.item_kind.as_str()
            );
            assert!(
                case.preserves_identity(),
                "state case for {} lost identity",
                row.item_kind.as_str()
            );
            assert!(
                case.preserves_guarantees(),
                "state case for {} lost a guarantee",
                row.item_kind.as_str()
            );
        }
    }
}

#[test]
fn missing_item_kind_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet
        .rows
        .retain(|row| row.item_kind != M5SelectionOrLockItemKind::Badge);
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::RequiredItemMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.vocabulary_set.presentations.pop();
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5SelectionOrLockAnatomyPart::StateCauseCue);
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5SelectionOrLockExportField::StateCause);
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::MandatoryExportMissing));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.rows[0]
        .required_labels
        .retain(|l| *l != M5ComponentStateRequiredLabel::KeyboardRoute);
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::MandatoryLabelMissing));
}

#[test]
fn accessibility_route_missing_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.rows[0]
        .accessibility_routes
        .retain(|r| *r != M5ComponentStateAccessibilityRoute::NonColorEncoded);
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.rows[0].state_examples[0].resolved.owner_disclosed = true;
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::ExampleResolutionDrift));
}

#[test]
fn state_example_missing_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.rows[1].state_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::StateExampleMissing));
}

#[test]
fn selection_or_lock_state_coverage_unproven_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    for row in &mut packet.rows {
        row.state_examples = vec![M5SelectionOrLockResolutionCase::resolved(selected_tab())];
    }
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::SelectionOrLockStateCoverageUnproven));
}

#[test]
fn presentation_and_cue_and_disclosure_coverage_unproven_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    // Every example a selected tab → no current/disabled/read-only/locked posture, no lock/read-only
    // cue, no cause/owner/block/recovery disclosure.
    for row in &mut packet.rows {
        row.state_examples = vec![M5SelectionOrLockResolutionCase::resolved(selected_tab())];
    }
    let violations = packet.validate();
    assert!(
        violations.contains(&M5SelectionOrLockStateContractViolation::PresentationCoverageUnproven)
    );
    assert!(violations.contains(&M5SelectionOrLockStateContractViolation::CueCoverageUnproven));
    assert!(
        violations.contains(&M5SelectionOrLockStateContractViolation::DisclosureCoverageUnproven)
    );
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.rows[0].collapses_selected_and_current = true;
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::RowInvariantViolated));
}

#[test]
fn stable_item_missing_proof_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::StableItemMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.governance_review.locked_never_hidden_behind_disabled = false;
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet
        .consumer_projection
        .disclosure_set_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_selection_or_lock_state_contract_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SelectionOrLockStateContractViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_item_kind() {
    let summary = seeded_m5_selection_or_lock_state_contract_packet().render_markdown_summary();
    for item in M5SelectionOrLockItemKind::ALL {
        assert!(
            summary.contains(item.label()),
            "summary missing item {}",
            item.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_item() {
    let csv = seeded_m5_selection_or_lock_state_contract_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5SelectionOrLockItemKind::ALL.len());
    assert!(lines[0].starts_with("item_kind,qualification,owner,"));
    for item in M5SelectionOrLockItemKind::ALL {
        assert!(
            csv.contains(item.as_str()),
            "csv missing item {}",
            item.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_selection_or_lock_state_contract_export()
        .expect("checked M5 selection or lock state contract primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_selection_or_lock_state_contract_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_items_visible() {
    for packet in [
        seeded_m5_selection_or_lock_state_contract_badge_beta_narrowed(),
        seeded_m5_selection_or_lock_state_contract_inspector_entry_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5SelectionOrLockItemKind::ALL.len());
    }

    let badge = seeded_m5_selection_or_lock_state_contract_badge_beta_narrowed();
    let row = badge
        .rows
        .iter()
        .find(|r| r.item_kind == M5SelectionOrLockItemKind::Badge)
        .expect("badge row present");
    assert_eq!(row.qualification, M5ComponentStateQualificationClass::Beta);

    let inspector = seeded_m5_selection_or_lock_state_contract_inspector_entry_preview_narrowed();
    let row = inspector
        .rows
        .iter()
        .find(|r| r.item_kind == M5SelectionOrLockItemKind::InspectorEntry)
        .expect("inspector-entry row present");
    assert_eq!(
        row.qualification,
        M5ComponentStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let badge: M5SelectionOrLockStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-selection-lock-state-contract-primitive/badge_beta_narrowed.json"
    )))
    .expect("badge fixture parses");
    assert!(badge.validate().is_empty());
    assert_eq!(
        badge,
        seeded_m5_selection_or_lock_state_contract_badge_beta_narrowed()
    );

    let inspector: M5SelectionOrLockStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-selection-lock-state-contract-primitive/inspector_entry_preview_narrowed.json"
    )))
    .expect("inspector-entry fixture parses");
    assert!(inspector.validate().is_empty());
    assert_eq!(
        inspector,
        seeded_m5_selection_or_lock_state_contract_inspector_entry_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_selection_or_lock_state_contract_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
