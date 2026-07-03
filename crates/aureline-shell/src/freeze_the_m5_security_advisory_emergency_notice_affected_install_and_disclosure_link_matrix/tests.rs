use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_advisory_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_advisory_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5AdvisoryComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5AdvisoryComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_severity_and_a_zone() {
    let packet = seeded_m5_advisory_component_matrix();
    for row in &packet.component_rows {
        for label in M5AdvisoryRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.severity_classes.is_empty());
        assert!(!row.projection_surfaces.is_empty());
        assert!(!row.responsive_classes.is_empty());
        assert!(!row.window_classes.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_advisory_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.anatomy_fields.is_empty(),
            family.is_advisory_card(),
            "anatomy_fields presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.action_states.is_empty(),
            family.carries_action(),
            "action_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.required_actions.is_empty(),
            family.carries_action(),
            "required_actions presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.dismissal_states.is_empty(),
            family.is_emergency_notice(),
            "dismissal_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.continuity_claims.is_empty(),
            family.assesses_install(),
            "continuity_claims presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.delivery_profiles.is_empty(),
            family.assesses_install(),
            "delivery_profiles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.freshness_states.is_empty(),
            family.assesses_install(),
            "freshness_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.disclosure_fields.is_empty(),
            family.discloses_history(),
            "disclosure_fields presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.notification_behaviors.is_empty(),
            family.hands_off_native(),
            "notification_behaviors presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.export_fields.is_empty(),
            family.is_activity_row(),
            "export_fields presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_advisory_component_matrix();
    for class in M5AdvisorySeverityClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.severity_classes.contains(&class)),
            "no component declares severity class {}",
            class.as_str()
        );
    }
    for field in M5AdvisoryAnatomyField::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.anatomy_fields.contains(&field)),
            "no component declares anatomy field {}",
            field.as_str()
        );
    }
    for state in M5AdvisoryActionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.action_states.contains(&state)),
            "no component declares action state {}",
            state.as_str()
        );
    }
    for action in M5AdvisoryRequiredAction::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.required_actions.contains(&action)),
            "no component declares required action {}",
            action.as_str()
        );
    }
    for state in M5AdvisoryDismissalState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dismissal_states.contains(&state)),
            "no component declares dismissal state {}",
            state.as_str()
        );
    }
    for claim in M5AdvisoryContinuityClaim::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.continuity_claims.contains(&claim)),
            "no component declares continuity claim {}",
            claim.as_str()
        );
    }
    for profile in M5AdvisoryDeliveryProfile::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.delivery_profiles.contains(&profile)),
            "no component declares delivery profile {}",
            profile.as_str()
        );
    }
    for state in M5AdvisoryFreshnessState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.freshness_states.contains(&state)),
            "no component declares freshness state {}",
            state.as_str()
        );
    }
    for field in M5AdvisoryDisclosureField::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.disclosure_fields.contains(&field)),
            "no component declares disclosure field {}",
            field.as_str()
        );
    }
    for behavior in M5AdvisoryNotificationBehavior::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.notification_behaviors.contains(&behavior)),
            "no component declares notification behavior {}",
            behavior.as_str()
        );
    }
    for field in M5AdvisoryExportField::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.export_fields.contains(&field)),
            "no component declares export field {}",
            field.as_str()
        );
    }
    for surface in M5AdvisoryProjectionSurface::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.projection_surfaces.contains(&surface)),
            "no component declares projection surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5AdvisoryComponentFamily::DisclosureBlock);
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.vocabulary_set.severity_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5AdvisoryRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn severity_missing_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.component_rows[0].severity_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::SeverityClassMissing));
}

#[test]
fn projection_surface_missing_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.component_rows[0].projection_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::ProjectionSurfaceMissing));
}

#[test]
fn anatomy_field_missing_fails_for_advisory_card() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::AdvisoryCard);
    row.anatomy_fields.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::AnatomyFieldMissing));
}

#[test]
fn action_state_missing_fails_for_action_bearing_family() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::EmergencyNotice);
    row.action_states.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::ActionStateMissing));
}

#[test]
fn required_action_missing_fails_for_action_bearing_family() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::AdvisoryCard);
    row.required_actions.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::RequiredActionMissing));
}

#[test]
fn dismissal_state_missing_fails_for_emergency_notice() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::EmergencyNotice);
    row.dismissal_states.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::DismissalStateMissing));
}

#[test]
fn continuity_and_freshness_missing_fail_for_affected_install_panel() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::AffectedInstallPanel);
    row.continuity_claims.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::ContinuityClaimMissing));

    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::AffectedInstallPanel);
    row.freshness_states.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::FreshnessStateMissing));

    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::AffectedInstallPanel);
    row.delivery_profiles.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::DeliveryProfileMissing));
}

#[test]
fn disclosure_field_missing_fails_for_disclosure_block() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::DisclosureBlock);
    row.disclosure_fields.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::DisclosureFieldMissing));
}

#[test]
fn notification_behavior_missing_fails_for_native_handoff() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(
        &mut packet,
        M5AdvisoryComponentFamily::NativeNotificationHandoff,
    );
    row.notification_behaviors.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::NotificationBehaviorMissing));
}

#[test]
fn export_field_missing_fails_for_activity_row() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::AdvisoryActivityRow);
    row.export_fields.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::ExportFieldMissing));
}

#[test]
fn advisory_invariant_violation_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.component_rows[0].hides_affected_scope = true;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::AdvisoryInvariantViolated));

    let mut packet = seeded_m5_advisory_component_matrix();
    packet.component_rows[1].hides_local_continuity = true;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::AdvisoryInvariantViolated));

    let mut packet = seeded_m5_advisory_component_matrix();
    packet.component_rows[2].invents_generic_advisory_language = true;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::AdvisoryInvariantViolated));

    let mut packet = seeded_m5_advisory_component_matrix();
    packet.component_rows[3].stays_silent_on_stale_or_unsigned = true;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::AdvisoryInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    let row = find_mut(&mut packet, M5AdvisoryComponentFamily::AdvisoryCard);
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet
        .governance_review
        .no_component_invents_generic_advisory_language = false;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet
        .consumer_projection
        .mirror_offline_drills_read_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_advisory_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_advisory_component_matrix().render_markdown_summary();
    for family in M5AdvisoryComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_advisory_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AdvisoryComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5AdvisoryComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_advisory_component_matrix_export()
        .expect("checked M5 advisory component matrix export validates");
    assert_eq!(packet.packet_id, M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_advisory_component_matrix_export()
        .expect("checked M5 advisory component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_advisory_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_advisory_component_matrix_emergency_notice_beta_narrowed(),
        seeded_m5_advisory_component_matrix_affected_install_panel_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5AdvisoryComponentFamily::ALL.len()
        );
    }

    let emergency = seeded_m5_advisory_component_matrix_emergency_notice_beta_narrowed();
    let row = emergency
        .component_rows
        .iter()
        .find(|r| r.component_family == M5AdvisoryComponentFamily::EmergencyNotice)
        .expect("emergency-notice row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Beta);

    let install = seeded_m5_advisory_component_matrix_affected_install_panel_preview_narrowed();
    let row = install
        .component_rows
        .iter()
        .find(|r| r.component_family == M5AdvisoryComponentFamily::AffectedInstallPanel)
        .expect("affected-install-panel row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let emergency: M5AdvisoryComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-advisory-scenarios/emergency_notice_beta_narrowed.json"
    )))
    .expect("emergency fixture parses");
    assert!(emergency.validate().is_empty());
    assert_eq!(
        emergency,
        seeded_m5_advisory_component_matrix_emergency_notice_beta_narrowed()
    );

    let install: M5AdvisoryComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-advisory-scenarios/affected_install_panel_preview_narrowed.json"
    )))
    .expect("install fixture parses");
    assert!(install.validate().is_empty());
    assert_eq!(
        install,
        seeded_m5_advisory_component_matrix_affected_install_panel_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_advisory_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}

fn find_mut(
    packet: &mut M5AdvisoryComponentMatrixPacket,
    family: M5AdvisoryComponentFamily,
) -> &mut M5AdvisoryComponentRow {
    packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == family)
        .expect("component family present")
}
