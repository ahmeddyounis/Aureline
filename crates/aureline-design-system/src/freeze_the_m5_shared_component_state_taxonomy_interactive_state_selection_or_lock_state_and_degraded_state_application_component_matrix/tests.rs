use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_shared_component_state_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SHARED_COMPONENT_STATE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_contract_family() {
    let packet = seeded_m5_shared_component_state_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5SharedComponentStateFamily::ALL {
        assert!(
            present.contains(&family),
            "missing contract family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5SharedComponentStateFamily::ALL.len()
    );
}

#[test]
fn taxonomy_names_all_thirteen_canonical_states() {
    let packet = seeded_m5_shared_component_state_matrix();
    let taxonomy = packet
        .component_rows
        .iter()
        .find(|r| r.component_family == M5SharedComponentStateFamily::SharedComponentStateTaxonomy)
        .expect("taxonomy row present");
    for state in M5SharedComponentStateClass::ALL {
        assert!(
            taxonomy.state_classes.contains(&state),
            "taxonomy missing canonical state {}",
            state.as_str()
        );
    }
    assert_eq!(
        taxonomy.state_classes.len(),
        M5SharedComponentStateClass::ALL.len()
    );
}

#[test]
fn every_family_governs_exactly_its_partition_of_states() {
    let packet = seeded_m5_shared_component_state_matrix();
    for row in &packet.component_rows {
        let present: std::collections::BTreeSet<_> = row.state_classes.iter().copied().collect();
        let expected: std::collections::BTreeSet<_> = row
            .component_family
            .governed_states()
            .iter()
            .copied()
            .collect();
        assert_eq!(
            present,
            expected,
            "family {} governs the wrong state subset",
            row.component_family.as_str()
        );
    }
}

#[test]
fn interactive_and_selection_partitions_are_disjoint() {
    let interactive: std::collections::BTreeSet<_> = M5SharedComponentStateFamily::InteractiveState
        .governed_states()
        .iter()
        .copied()
        .collect();
    let selection: std::collections::BTreeSet<_> =
        M5SharedComponentStateFamily::SelectionOrLockState
            .governed_states()
            .iter()
            .copied()
            .collect();
    let degraded: std::collections::BTreeSet<_> =
        M5SharedComponentStateFamily::DegradedStateApplication
            .governed_states()
            .iter()
            .copied()
            .collect();
    assert!(interactive.is_disjoint(&selection));
    assert!(interactive.is_disjoint(&degraded));
    assert!(selection.is_disjoint(&degraded));
    // The three sub-contracts partition the full taxonomy exactly.
    let union: std::collections::BTreeSet<_> = interactive
        .union(&selection)
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .union(&degraded)
        .copied()
        .collect();
    let all: std::collections::BTreeSet<_> =
        M5SharedComponentStateClass::ALL.iter().copied().collect();
    assert_eq!(union, all);
}

#[test]
fn every_contract_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_shared_component_state_matrix();
    for row in &packet.component_rows {
        for label in M5ComponentStateRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "contract {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable));
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_shared_component_state_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.precedence_rules.is_empty(),
            family.is_shared_component_state_taxonomy(),
            "precedence_rules presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.disclosure_triggers.is_empty(),
            family.is_shared_component_state_taxonomy(),
            "disclosure_triggers presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.interaction_input_routes.is_empty(),
            family.is_interactive_state(),
            "interaction_input_routes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.lock_owner_classes.is_empty(),
            family.is_selection_or_lock_state(),
            "lock_owner_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.recovery_disclosure_classes.is_empty(),
            family.is_degraded_state_application(),
            "recovery_disclosure_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.state_cause_classes.is_empty(),
            family.is_selection_or_lock_state() || family.is_degraded_state_application(),
            "state_cause_classes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_contract() {
    let packet = seeded_m5_shared_component_state_matrix();
    for state in M5SharedComponentStateClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.state_classes.contains(&state)),
            "no contract declares state class {}",
            state.as_str()
        );
    }
    for rule in M5StatePrecedenceRule::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.precedence_rules.contains(&rule)),
            "no contract declares precedence rule {}",
            rule.as_str()
        );
    }
    for trigger in M5StateDisclosureTrigger::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.disclosure_triggers.contains(&trigger)),
            "no contract declares disclosure trigger {}",
            trigger.as_str()
        );
    }
    for route in M5InteractionInputRoute::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.interaction_input_routes.contains(&route)),
            "no contract declares interaction input route {}",
            route.as_str()
        );
    }
    for owner in M5LockOwnerClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.lock_owner_classes.contains(&owner)),
            "no contract declares lock owner class {}",
            owner.as_str()
        );
    }
    for class in M5RecoveryDisclosureClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.recovery_disclosure_classes.contains(&class)),
            "no contract declares recovery disclosure class {}",
            class.as_str()
        );
    }
    for cause in M5StateCauseClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.state_cause_classes.contains(&cause)),
            "no contract declares state cause class {}",
            cause.as_str()
        );
    }
    for label in M5ComponentStateRequiredLabel::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.required_labels.contains(&label)),
            "no contract declares required label {}",
            label.as_str()
        );
    }
}

#[test]
fn missing_contract_family_fails_validation() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5SharedComponentStateFamily::InteractiveState);
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.vocabulary_set.state_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5ComponentStateRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn state_subset_mismatch_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5SharedComponentStateFamily::InteractiveState)
        .expect("interactive-state row present");
    // Add a state the interactive contract does not govern.
    row.state_classes.push(M5SharedComponentStateClass::Locked);
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::StateSubsetMismatch));
}

#[test]
fn taxonomy_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_shared_component_state_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5SharedComponentStateFamily::SharedComponentStateTaxonomy
            })
            .expect("taxonomy row present");
        let expected = if clear == 0 {
            row.precedence_rules.clear();
            M5SharedComponentStateMatrixViolation::PrecedenceRuleMissing
        } else {
            row.disclosure_triggers.clear();
            M5SharedComponentStateMatrixViolation::DisclosureTriggerMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn interactive_vocab_missing_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5SharedComponentStateFamily::InteractiveState)
        .expect("interactive-state row present");
    row.interaction_input_routes.clear();
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::InteractionInputRouteMissing));
}

#[test]
fn selection_or_lock_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_shared_component_state_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5SharedComponentStateFamily::SelectionOrLockState)
            .expect("selection-or-lock row present");
        let expected = if clear == 0 {
            row.lock_owner_classes.clear();
            M5SharedComponentStateMatrixViolation::LockOwnerClassMissing
        } else {
            row.state_cause_classes.clear();
            M5SharedComponentStateMatrixViolation::StateCauseClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn degraded_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_shared_component_state_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5SharedComponentStateFamily::DegradedStateApplication
            })
            .expect("degraded row present");
        let expected = if clear == 0 {
            row.recovery_disclosure_classes.clear();
            M5SharedComponentStateMatrixViolation::RecoveryDisclosureClassMissing
        } else {
            row.state_cause_classes.clear();
            M5SharedComponentStateMatrixViolation::StateCauseClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.component_rows[2].collapses_current_and_selected = true;
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.component_rows[2].masks_lock_behind_disabled = true;
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.component_rows[3].presents_pending_as_generic_loading = true;
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.component_rows[3].omits_consequence_or_recovery_on_degraded = true;
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_contract_missing_proof_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5SharedComponentStateFamily::SharedComponentStateTaxonomy
        })
        .expect("taxonomy row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.governance_review.current_and_selected_never_collapse = false;
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_shared_component_state_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SharedComponentStateMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_contract_family() {
    let summary = seeded_m5_shared_component_state_matrix().render_markdown_summary();
    for family in M5SharedComponentStateFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing contract {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_contract() {
    let csv = seeded_m5_shared_component_state_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5SharedComponentStateFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,state_classes,"));
    for family in M5SharedComponentStateFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing contract {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_shared_component_state_matrix_export()
        .expect("checked M5 shared-component-state matrix export validates");
    assert_eq!(packet.packet_id, M5_SHARED_COMPONENT_STATE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_shared_component_state_matrix_export()
        .expect("checked M5 shared-component-state matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_shared_component_state_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_contracts_visible() {
    for packet in [
        seeded_m5_shared_component_state_matrix_interactive_state_beta_narrowed(),
        seeded_m5_shared_component_state_matrix_degraded_state_application_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5SharedComponentStateFamily::ALL.len()
        );
    }

    let interactive = seeded_m5_shared_component_state_matrix_interactive_state_beta_narrowed();
    let row = interactive
        .component_rows
        .iter()
        .find(|r| r.component_family == M5SharedComponentStateFamily::InteractiveState)
        .expect("interactive-state row present");
    assert_eq!(row.qualification, M5ComponentStateQualificationClass::Beta);

    let degraded =
        seeded_m5_shared_component_state_matrix_degraded_state_application_preview_narrowed();
    let row = degraded
        .component_rows
        .iter()
        .find(|r| r.component_family == M5SharedComponentStateFamily::DegradedStateApplication)
        .expect("degraded row present");
    assert_eq!(
        row.qualification,
        M5ComponentStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let interactive: M5SharedComponentStateMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-shared-state-taxonomy/interactive_state_beta_narrowed.json"
        )))
        .expect("interactive-state fixture parses");
    assert!(interactive.validate().is_empty());
    assert_eq!(
        interactive,
        seeded_m5_shared_component_state_matrix_interactive_state_beta_narrowed()
    );

    let degraded: M5SharedComponentStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shared-state-taxonomy/degraded_state_application_preview_narrowed.json"
    )))
    .expect("degraded-state-application fixture parses");
    assert!(degraded.validate().is_empty());
    assert_eq!(
        degraded,
        seeded_m5_shared_component_state_matrix_degraded_state_application_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_shared_component_state_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
