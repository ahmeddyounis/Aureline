use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_companion_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_COMPANION_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_companion_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5CompanionComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5CompanionComponentFamily::ALL.len()
    );
}

#[test]
fn disposition_vocabulary_carries_every_acceptance_criteria_token() {
    // AC1: consumers bind to ONE controlled vocabulary for review-only, comment-capable,
    // desktop required, cached, stale, policy blocked, and handoff ready.
    let tokens: Vec<&str> = M5CompanionComponentDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    for expected in [
        "review_only",
        "comment_capable",
        "desktop_required",
        "cached",
        "stale",
        "policy_blocked",
        "handoff_ready",
    ] {
        assert!(
            tokens.contains(&expected),
            "disposition vocabulary missing {expected}"
        );
    }
    assert_eq!(tokens.len(), 7);
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_companion_component_matrix();
    for row in &packet.component_rows {
        for label in M5CompanionRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.component_family
                    .canonical_component_schema_ref()
                    .to_owned()
            ),
            "component {} does not point at its canonical schema",
            row.component_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.object_kinds.is_empty());
        assert!(!row.client_scopes.is_empty());
        assert!(!row.freshness_classes.is_empty());
        assert!(!row.dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5CompanionAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_companion_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.severities.is_empty(),
            family.declares_severity(),
            "severities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.review_kinds.is_empty(),
            family.declares_review_kind(),
            "review_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.ci_statuses.is_empty(),
            family.declares_ci_status(),
            "ci_statuses presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.session_follow_states.is_empty(),
            family.declares_session_follow_state(),
            "session_follow_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.handoff_targets.is_empty(),
            family.declares_handoff_target(),
            "handoff_targets presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.notification_categories.is_empty(),
            family.declares_notification_category(),
            "notification_categories presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_companion_component_matrix();
    for kind in M5CompanionObjectKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.object_kinds.contains(&kind)),
            "no component declares object kind {}",
            kind.as_str()
        );
    }
    for scope in M5CompanionClientScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.client_scopes.contains(&scope)),
            "no component declares client scope {}",
            scope.as_str()
        );
    }
    for fresh in M5CompanionFreshness::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.freshness_classes.contains(&fresh)),
            "no component declares freshness {}",
            fresh.as_str()
        );
    }
    for disposition in M5CompanionComponentDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for severity in M5CompanionSeverity::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.severities.contains(&severity)),
            "no component declares severity {}",
            severity.as_str()
        );
    }
    for kind in M5CompanionReviewKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.review_kinds.contains(&kind)),
            "no component declares review kind {}",
            kind.as_str()
        );
    }
    for status in M5CompanionCiStatus::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.ci_statuses.contains(&status)),
            "no component declares CI status {}",
            status.as_str()
        );
    }
    for state in M5CompanionSessionFollowState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.session_follow_states.contains(&state)),
            "no component declares session-follow state {}",
            state.as_str()
        );
    }
    for target in M5CompanionHandoffTarget::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.handoff_targets.contains(&target)),
            "no component declares handoff target {}",
            target.as_str()
        );
    }
    for category in M5CompanionNotificationCategory::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.notification_categories.contains(&category)),
            "no component declares notification category {}",
            category.as_str()
        );
    }
    for reason in M5CompanionDegradedReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no component declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5CompanionComponentFamily::CiStatusCard);
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5CompanionRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    let own = M5CompanionComponentFamily::NotificationRow.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CompanionComponentFamily::NotificationRow)
        .expect("notification row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn universal_vocab_missing_fails() {
    for clear in [0u8, 1, 2, 3] {
        let mut packet = seeded_m5_companion_component_matrix();
        let expected = match clear {
            0 => {
                packet.component_rows[0].object_kinds.clear();
                M5CompanionComponentMatrixViolation::ObjectKindMissing
            }
            1 => {
                packet.component_rows[0].client_scopes.clear();
                M5CompanionComponentMatrixViolation::ClientScopeMissing
            }
            2 => {
                packet.component_rows[0].freshness_classes.clear();
                M5CompanionComponentMatrixViolation::FreshnessClassMissing
            }
            _ => {
                packet.component_rows[0].dispositions.clear();
                M5CompanionComponentMatrixViolation::DispositionMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn notification_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_companion_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5CompanionComponentFamily::NotificationRow)
            .expect("notification row present");
        let expected = if clear == 0 {
            row.severities.clear();
            M5CompanionComponentMatrixViolation::SeverityMissing
        } else {
            row.notification_categories.clear();
            M5CompanionComponentMatrixViolation::NotificationCategoryMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn mobile_review_card_vocab_missing_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CompanionComponentFamily::MobileReviewCard)
        .expect("mobile review card present");
    row.review_kinds.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ReviewKindMissing));
}

#[test]
fn ci_status_card_vocab_missing_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CompanionComponentFamily::CiStatusCard)
        .expect("ci-status card present");
    row.ci_statuses.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::CiStatusMissing));
}

#[test]
fn session_follow_tile_vocab_missing_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CompanionComponentFamily::SessionFollowTile)
        .expect("session-follow tile present");
    row.session_follow_states.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::SessionFollowStateMissing));
}

#[test]
fn desktop_handoff_sheet_vocab_missing_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CompanionComponentFamily::DesktopHandoffSheet)
        .expect("desktop-handoff sheet present");
    row.handoff_targets.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::HandoffTargetMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[2].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[0].masks_scope_or_freshness = true;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[3].hides_capability_boundary = true;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[2].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[5].implies_desktop_action_is_companion_safe = true;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CompanionComponentFamily::NotificationRow)
        .expect("notification row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_companion_component_matrix().render_markdown_summary();
    for family in M5CompanionComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_companion_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CompanionComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5CompanionComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_component_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_companion_component_matrix_export()
        .expect("checked M5 companion component matrix export validates");
    assert_eq!(packet.packet_id, M5_COMPANION_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_companion_component_matrix_export()
        .expect("checked M5 companion component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_companion_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_companion_component_matrix_session_follow_tile_beta_narrowed(),
        seeded_m5_companion_component_matrix_desktop_handoff_sheet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5CompanionComponentFamily::ALL.len()
        );
    }

    let follow = seeded_m5_companion_component_matrix_session_follow_tile_beta_narrowed();
    let row = follow
        .component_rows
        .iter()
        .find(|r| r.component_family == M5CompanionComponentFamily::SessionFollowTile)
        .expect("session-follow-tile row present");
    assert_eq!(row.qualification, M5CompanionQualificationClass::Beta);

    let handoff = seeded_m5_companion_component_matrix_desktop_handoff_sheet_preview_narrowed();
    let row = handoff
        .component_rows
        .iter()
        .find(|r| r.component_family == M5CompanionComponentFamily::DesktopHandoffSheet)
        .expect("desktop-handoff-sheet row present");
    assert_eq!(row.qualification, M5CompanionQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let follow: M5CompanionComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-companion-components/session_follow_tile_beta_narrowed.json"
    )))
    .expect("session-follow-tile fixture parses");
    assert!(follow.validate().is_empty());
    assert_eq!(
        follow,
        seeded_m5_companion_component_matrix_session_follow_tile_beta_narrowed()
    );

    let handoff: M5CompanionComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-companion-components/desktop_handoff_sheet_preview_narrowed.json"
    )))
    .expect("desktop-handoff-sheet fixture parses");
    assert!(handoff.validate().is_empty());
    assert_eq!(
        handoff,
        seeded_m5_companion_component_matrix_desktop_handoff_sheet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_companion_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentMatrixViolation::RawMaterialInExport));
}
