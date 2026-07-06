use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_runtime_boundary_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RUNTIME_BOUNDARY_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_runtime_boundary_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5RuntimeBoundaryComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5RuntimeBoundaryComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_a_zone() {
    let packet = seeded_m5_runtime_boundary_component_matrix();
    for row in &packet.component_rows {
        for label in M5RuntimeBoundaryRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.responsive_classes.is_empty());
        assert!(!row.window_classes.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_runtime_boundary_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.shell_integration_qualities.is_empty(),
            family.is_terminal(),
            "shell_integration_qualities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.session_liveness_states.is_empty(),
            family.is_terminal(),
            "session_liveness_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.host_boundary_classes.is_empty(),
            family.is_remote_target(),
            "host_boundary_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.connection_states.is_empty(),
            family.is_remote_target(),
            "connection_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.runtime_source_classes.is_empty(),
            family.is_environment(),
            "runtime_source_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.toolchain_source_classes.is_empty(),
            family.is_toolchain(),
            "toolchain_source_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.toolchain_pin_states.is_empty(),
            family.is_toolchain(),
            "toolchain_pin_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.collaboration_roles.is_empty(),
            family.is_presence(),
            "collaboration_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.follow_states.is_empty(),
            family.is_presence(),
            "follow_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.repair_blast_radii.is_empty(),
            family.is_repair(),
            "repair_blast_radii presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.reversibility_classes.is_empty(),
            family.is_repair(),
            "reversibility_classes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_runtime_boundary_component_matrix();
    for quality in M5ShellIntegrationQuality::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.shell_integration_qualities.contains(&quality)),
            "no component declares shell-integration quality {}",
            quality.as_str()
        );
    }
    for state in M5TerminalSessionLiveness::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.session_liveness_states.contains(&state)),
            "no component declares session-liveness state {}",
            state.as_str()
        );
    }
    for class in M5HostBoundaryClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.host_boundary_classes.contains(&class)),
            "no component declares host-boundary class {}",
            class.as_str()
        );
    }
    for state in M5RemoteConnectionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.connection_states.contains(&state)),
            "no component declares connection state {}",
            state.as_str()
        );
    }
    for class in M5RuntimeSourceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.runtime_source_classes.contains(&class)),
            "no component declares runtime source class {}",
            class.as_str()
        );
    }
    for class in M5ToolchainSourceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.toolchain_source_classes.contains(&class)),
            "no component declares toolchain source class {}",
            class.as_str()
        );
    }
    for state in M5ToolchainPinState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.toolchain_pin_states.contains(&state)),
            "no component declares toolchain pin state {}",
            state.as_str()
        );
    }
    for role in M5CollaborationRole::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.collaboration_roles.contains(&role)),
            "no component declares collaboration role {}",
            role.as_str()
        );
    }
    for state in M5FollowState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.follow_states.contains(&state)),
            "no component declares follow state {}",
            state.as_str()
        );
    }
    for radius in M5RepairBlastRadius::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.repair_blast_radii.contains(&radius)),
            "no component declares repair blast radius {}",
            radius.as_str()
        );
    }
    for class in M5ReversibilityClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.reversibility_classes.contains(&class)),
            "no component declares reversibility class {}",
            class.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5RuntimeBoundaryComponentFamily::ToolchainPinRow);
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.vocabulary_set.host_boundary_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5RuntimeBoundaryRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn shell_integration_quality_missing_fails_for_terminal_tab() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::TerminalTab)
        .expect("terminal tab present");
    row.shell_integration_qualities.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ShellIntegrationQualityMissing));
}

#[test]
fn session_liveness_state_missing_fails_for_terminal_tab() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::TerminalTab)
        .expect("terminal tab present");
    row.session_liveness_states.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::SessionLivenessStateMissing));
}

#[test]
fn host_boundary_class_missing_fails_for_remote_pill() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::RemoteTargetPill)
        .expect("remote target pill present");
    row.host_boundary_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::HostBoundaryClassMissing));
}

#[test]
fn connection_state_missing_fails_for_remote_pill() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::RemoteTargetPill)
        .expect("remote target pill present");
    row.connection_states.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ConnectionStateMissing));
}

#[test]
fn runtime_source_class_missing_fails_for_environment_strip() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5RuntimeBoundaryComponentFamily::EnvironmentStatusStrip
        })
        .expect("environment status strip present");
    row.runtime_source_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::RuntimeSourceClassMissing));
}

#[test]
fn toolchain_vocab_missing_fails_for_toolchain_row() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::ToolchainPinRow)
        .expect("toolchain pin row present");
    row.toolchain_source_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ToolchainSourceClassMissing));

    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::ToolchainPinRow)
        .expect("toolchain pin row present");
    row.toolchain_pin_states.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ToolchainPinStateMissing));
}

#[test]
fn presence_vocab_missing_fails_for_presence_stack() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::PresenceAvatarStack)
        .expect("presence avatar stack present");
    row.collaboration_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::CollaborationRoleMissing));

    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::PresenceAvatarStack)
        .expect("presence avatar stack present");
    row.follow_states.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::FollowStateMissing));
}

#[test]
fn repair_vocab_missing_fails_for_repair_card() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::RepairActionCard)
        .expect("repair action card present");
    row.repair_blast_radii.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::RepairBlastRadiusMissing));

    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::RepairActionCard)
        .expect("repair action card present");
    row.reversibility_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ReversibilityClassMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.component_rows[0].masks_host_or_runtime_boundary = true;
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.component_rows[0].conflates_live_and_restored_session = true;
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.component_rows[2].invents_private_status_grammar = true;
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.component_rows[5].overstates_reversibility_or_drops_audit_truth = true;
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::TerminalTab)
        .expect("terminal tab present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet
        .governance_review
        .no_component_invents_second_status_grammar = false;
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet
        .consumer_projection
        .repair_surfaces_consume_reversibility_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RuntimeBoundaryMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_runtime_boundary_component_matrix().render_markdown_summary();
    for family in M5RuntimeBoundaryComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_runtime_boundary_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5RuntimeBoundaryComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5RuntimeBoundaryComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_runtime_boundary_component_matrix_export()
        .expect("checked M5 runtime boundary matrix export validates");
    assert_eq!(packet.packet_id, M5_RUNTIME_BOUNDARY_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_runtime_boundary_component_matrix_export()
        .expect("checked M5 runtime boundary matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_runtime_boundary_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_runtime_boundary_component_matrix_presence_avatar_stack_beta_narrowed(),
        seeded_m5_runtime_boundary_component_matrix_repair_action_card_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5RuntimeBoundaryComponentFamily::ALL.len()
        );
    }

    let presence =
        seeded_m5_runtime_boundary_component_matrix_presence_avatar_stack_beta_narrowed();
    let row = presence
        .component_rows
        .iter()
        .find(|r| r.component_family == M5RuntimeBoundaryComponentFamily::PresenceAvatarStack)
        .expect("presence-avatar-stack row present");
    assert_eq!(row.qualification, M5RuntimeBoundaryQualificationClass::Beta);

    let repair = seeded_m5_runtime_boundary_component_matrix_repair_action_card_preview_narrowed();
    let row = repair
        .component_rows
        .iter()
        .find(|r| r.component_family == M5RuntimeBoundaryComponentFamily::RepairActionCard)
        .expect("repair-action-card row present");
    assert_eq!(
        row.qualification,
        M5RuntimeBoundaryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let presence: M5RuntimeBoundaryMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-runtime-boundary-components/presence_avatar_stack_beta_narrowed.json"
    )))
    .expect("presence fixture parses");
    assert!(presence.validate().is_empty());
    assert_eq!(
        presence,
        seeded_m5_runtime_boundary_component_matrix_presence_avatar_stack_beta_narrowed()
    );

    let repair: M5RuntimeBoundaryMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-runtime-boundary-components/repair_action_card_preview_narrowed.json"
    )))
    .expect("repair fixture parses");
    assert!(repair.validate().is_empty());
    assert_eq!(
        repair,
        seeded_m5_runtime_boundary_component_matrix_repair_action_card_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_runtime_boundary_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
