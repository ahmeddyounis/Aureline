use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_collaboration_control_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_COLLABORATION_CONTROL_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_collaboration_control_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .collaboration_control_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5CollaborationControlObject::ALL {
        assert!(
            present.contains(&class),
            "missing object class {}",
            class.as_str()
        );
    }
    assert_eq!(
        packet.collaboration_control_rows.len(),
        M5CollaborationControlObject::ALL.len()
    );
}

#[test]
fn frozen_collaboration_control_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5CollaborationControlRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "control_authority_disclosure",
            "active_driver_disclosure",
            "view_first_default_disclosure",
            "consent_scope_disclosure",
            "recording_retention_state_disclosure",
            "paste_secret_guard_disclosure",
            "replay_free_restore_disclosure",
        ]
    );
    assert!(M5CollaborationControlRole::ControlAuthorityDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_control_result());
    assert!(M5CollaborationControlRole::ActiveDriverDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_control_result());
    assert!(M5CollaborationControlRole::ViewFirstDefaultDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_control_result());
    assert!(M5CollaborationControlRole::ConsentScopeDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_control_result());
    assert!(
        !M5CollaborationControlRole::RecordingRetentionStateDisclosure
            .must_be_present_before_surfacing_as_a_collaboration_control_result()
    );
    assert!(!M5CollaborationControlRole::PasteSecretGuardDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_control_result());
    assert!(!M5CollaborationControlRole::ReplayFreeRestoreDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_control_result());
}

#[test]
fn active_driver_is_mechanically_distinct_from_viewer() {
    let tokens: Vec<&str> = M5CollaborationControlState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "viewer",
            "commenter",
            "editor",
            "driver",
            "navigator",
            "presenter_moderator",
            "live_only",
            "metadata_audit",
            "replayable_text_comment_timeline",
            "elevated_support_evidence",
            "control_requested",
            "control_granted",
            "control_expired",
            "recording_active",
            "consent_renewal_required",
            "restore_view_only",
        ]
    );
    assert!(M5CollaborationControlState::Driver.is_active_driver());
    for state in M5CollaborationControlState::ALL {
        if state != M5CollaborationControlState::Driver {
            assert!(
                !state.is_active_driver(),
                "state {} must not be the active driver",
                state.as_str()
            );
        }
    }
}

#[test]
fn control_authority_source_keeps_the_four_kinds_distinct() {
    let tokens: Vec<&str> = M5CollaborationControlAuthoritySource::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "granted_by_explicit_control_grant",
            "delegated_by_presenter_token",
            "inferred_from_presence_or_follow",
            "expired_or_revoked_grant",
        ]
    );
    assert!(
        M5CollaborationControlAuthoritySource::GrantedByExplicitControlGrant
            .is_explicitly_granted()
    );
    assert!(
        !M5CollaborationControlAuthoritySource::DelegatedByPresenterToken.is_explicitly_granted()
    );
    assert!(
        !M5CollaborationControlAuthoritySource::InferredFromPresenceOrFollow
            .is_explicitly_granted()
    );
    assert!(!M5CollaborationControlAuthoritySource::ExpiredOrRevokedGrant.is_explicitly_granted());
}

#[test]
fn retention_gate_names_blocked_states() {
    let tokens: Vec<&str> = M5CollaborationControlRetentionGate::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "consent_current_recording_allowed",
            "blocked_by_missing_join_consent",
            "blocked_by_consent_renewal_required",
            "blocked_by_retention_scope_widening",
            "blocked_by_guest_scope_or_route_expansion",
        ]
    );
    assert!(
        !M5CollaborationControlRetentionGate::ConsentCurrentRecordingAllowed
            .is_blocked_from_recording_or_retention()
    );
    assert!(
        M5CollaborationControlRetentionGate::BlockedByMissingJoinConsent
            .is_blocked_from_recording_or_retention()
    );
    assert!(
        M5CollaborationControlRetentionGate::BlockedByConsentRenewalRequired
            .is_blocked_from_recording_or_retention()
    );
    assert!(
        M5CollaborationControlRetentionGate::BlockedByRetentionScopeWidening
            .is_blocked_from_recording_or_retention()
    );
    assert!(
        M5CollaborationControlRetentionGate::BlockedByGuestScopeOrRouteExpansion
            .is_blocked_from_recording_or_retention()
    );
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_stages() {
    let packet = seeded_m5_collaboration_control_matrix();
    for row in &packet.collaboration_control_rows {
        for label in M5CollaborationControlRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "class {} missing mandatory label {}",
                row.object_class.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs
                .contains(&row.object_class.canonical_domain_schema_ref().to_owned()),
            "class {} does not point at its canonical schema",
            row.object_class.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.classification_stages.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5CollaborationControlAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_visible_state() {
    let packet = seeded_m5_collaboration_control_matrix();
    for row in &packet.collaboration_control_rows {
        let tr = &row.required_visible_state;
        for field in [
            &tr.surface_label,
            &tr.control_authority,
            &tr.active_driver,
            &tr.participant_roster_and_roles,
            &tr.session_state_summary,
            &tr.consent_and_retention_state,
            &tr.guard_and_restore_evidence,
        ] {
            assert!(
                !field.trim().is_empty(),
                "visible-state field empty on {}",
                row.object_class.as_str()
            );
        }
    }
}

#[test]
fn class_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_collaboration_control_matrix();
    for row in &packet.collaboration_control_rows {
        let class = row.object_class;
        assert_eq!(
            !row.shared_terminal_debug_view_roles.is_empty(),
            class.declares_shared_terminal_debug_view_roles(),
            "shared_terminal_debug_view_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.control_grant_roles.is_empty(),
            class.declares_control_grant_roles(),
            "control_grant_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.presenter_token_roles.is_empty(),
            class.declares_presenter_token_roles(),
            "presenter_token_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.consent_envelope_roles.is_empty(),
            class.declares_consent_envelope_roles(),
            "consent_envelope_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.retention_review_roles.is_empty(),
            class.declares_retention_review_roles(),
            "retention_review_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.session_restore_view_roles.is_empty(),
            class.declares_session_restore_view_roles(),
            "session_restore_view_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_collaboration_control_matrix();
    for role in M5CollaborationControlRole::ALL {
        assert!(
            packet
                .collaboration_control_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares collaboration-control role {}",
            role.as_str()
        );
    }
    for role in M5SharedTerminalDebugViewRole::ALL {
        assert!(
            packet
                .collaboration_control_rows
                .iter()
                .any(|row| row.shared_terminal_debug_view_roles.contains(&role)),
            "no class declares shared_terminal_debug_view_role {}",
            role.as_str()
        );
    }
    for role in M5ControlGrantRole::ALL {
        assert!(
            packet
                .collaboration_control_rows
                .iter()
                .any(|row| row.control_grant_roles.contains(&role)),
            "no class declares control_grant_role {}",
            role.as_str()
        );
    }
    for role in M5PresenterTokenRole::ALL {
        assert!(
            packet
                .collaboration_control_rows
                .iter()
                .any(|row| row.presenter_token_roles.contains(&role)),
            "no class declares presenter_token_role {}",
            role.as_str()
        );
    }
    for role in M5ConsentEnvelopeRole::ALL {
        assert!(
            packet
                .collaboration_control_rows
                .iter()
                .any(|row| row.consent_envelope_roles.contains(&role)),
            "no class declares consent_envelope_role {}",
            role.as_str()
        );
    }
    for role in M5RetentionReviewRole::ALL {
        assert!(
            packet
                .collaboration_control_rows
                .iter()
                .any(|row| row.retention_review_roles.contains(&role)),
            "no class declares retention_review_role {}",
            role.as_str()
        );
    }
    for role in M5SessionRestoreViewRole::ALL {
        assert!(
            packet
                .collaboration_control_rows
                .iter()
                .any(|row| row.session_restore_view_roles.contains(&role)),
            "no class declares session_restore_view_role {}",
            role.as_str()
        );
    }
    for reason in M5CollaborationControlDegradedReason::ALL {
        assert!(
            packet
                .collaboration_control_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet
        .collaboration_control_rows
        .retain(|row| row.object_class != M5CollaborationControlObject::PresenterToken);
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[0]
        .required_labels
        .retain(|label| *label != M5CollaborationControlRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    let own = M5CollaborationControlObject::ControlGrant.canonical_domain_schema_ref();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::ControlGrant)
        .expect("control-grant row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::SemanticRoleMissing));
}

#[test]
fn shared_terminal_debug_view_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::SharedTerminalDebugView)
        .expect("SharedTerminalDebugView row present");
    row.shared_terminal_debug_view_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::SharedTerminalDebugViewRoleMissing));
}

#[test]
fn control_grant_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::ControlGrant)
        .expect("ControlGrant row present");
    row.control_grant_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::ControlGrantRoleMissing));
}

#[test]
fn presenter_token_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::PresenterToken)
        .expect("PresenterToken row present");
    row.presenter_token_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::PresenterTokenRoleMissing));
}

#[test]
fn consent_envelope_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::ConsentEnvelope)
        .expect("ConsentEnvelope row present");
    row.consent_envelope_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::ConsentEnvelopeRoleMissing));
}

#[test]
fn retention_review_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::RetentionReview)
        .expect("RetentionReview row present");
    row.retention_review_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::RetentionReviewRoleMissing));
}

#[test]
fn session_restore_view_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::SessionRestoreView)
        .expect("SessionRestoreView row present");
    row.session_restore_view_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::SessionRestoreViewRoleMissing));
}

#[test]
fn visible_state_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[0]
        .required_visible_state
        .surface_label
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::VisibleStateIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[0]
        .backup_owner_role
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::CollaborationControlRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[3]
        .degraded_reasons
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::DegradedReasonMissing));
}

#[test]
fn collaboration_control_invariant_violation_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[0]
        .acquires_control_from_presence_or_follow_without_an_explicit_grant = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::CollaborationControlInvariantViolated));

    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[1]
        .allows_more_than_one_active_driver_on_a_sensitive_surface = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::CollaborationControlInvariantViolated));

    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[2]
        .starts_recording_transcript_retention_or_guest_scope_widening_silently = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::CollaborationControlInvariantViolated));

    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[3].replays_prior_terminal_or_debug_input_on_join_or_restore =
        true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::CollaborationControlInvariantViolated));

    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[4]
        .reveals_raw_secrets_command_text_or_clipboard_without_a_guard_and_consent_posture = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::CollaborationControlInvariantViolated));
}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::ControlGrant)
        .expect("control-grant row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_classification_stages_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[1]
        .classification_stages
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::ClassificationStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[1]
        .consumer_surfaces
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet
        .governance_review
        .active_driver_state_is_mechanically_distinct_from_viewer = false;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_collaboration_control_source = false;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_collaboration_control_matrix().render_markdown_summary();
    for class in M5CollaborationControlObject::ALL {
        assert!(
            summary.contains(class.as_str()),
            "summary missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_collaboration_control_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CollaborationControlObject::ALL.len());
    assert!(lines[0].starts_with(
        "object_class,qualification,session_state,owner,backup_owner,canonical_schema,"
    ));
    for class in M5CollaborationControlObject::ALL {
        assert!(
            csv.contains(class.as_str()),
            "csv missing class {}",
            class.as_str()
        );
        assert!(
            csv.contains(class.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            class.as_str()
        );
    }
}

#[test]
fn dashboard_json_names_every_class_and_matches_checked_in_file() {
    let rendered: serde_json::Value =
        serde_json::from_str(&seeded_m5_collaboration_control_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5CollaborationControlObject::ALL {
        assert!(
            rendered["objects"]
                .as_array()
                .expect("objects array")
                .iter()
                .any(|c| c["object_class"] == class.as_str()),
            "dashboard missing class {}",
            class.as_str()
        );
    }
    let from_disk: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../dashboards/m5-collaboration-control-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked collaboration-control-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_collaboration_control_matrix_export()
        .expect("checked M5 collaboration-control matrix export validates");
    assert_eq!(packet.packet_id, M5_COLLABORATION_CONTROL_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_collaboration_control_matrix_export()
        .expect("checked M5 collaboration-control matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_collaboration_control_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_collaboration_control_matrix_control_grant_beta_narrowed(),
        seeded_m5_collaboration_control_matrix_session_restore_view_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.collaboration_control_rows.len(),
            M5CollaborationControlObject::ALL.len()
        );
    }

    let beta = seeded_m5_collaboration_control_matrix_control_grant_beta_narrowed();
    let row = beta
        .collaboration_control_rows
        .iter()
        .find(|r| r.object_class == M5CollaborationControlObject::ControlGrant)
        .expect("control-grant row present");
    assert_eq!(
        row.qualification,
        M5CollaborationControlQualificationClass::Beta
    );

    let preview = seeded_m5_collaboration_control_matrix_session_restore_view_preview_narrowed();
    let row = preview
        .collaboration_control_rows
        .iter()
        .find(|r| r.object_class == M5CollaborationControlObject::SessionRestoreView)
        .expect("session-restore-view row present");
    assert_eq!(
        row.qualification,
        M5CollaborationControlQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5CollaborationControlMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/collaboration/m5-shared-control/control_grant_beta_narrowed.json"
    )))
    .expect("control-grant fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_collaboration_control_matrix_control_grant_beta_narrowed()
    );

    let preview: M5CollaborationControlMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/collaboration/m5-shared-control/session_restore_view_preview_narrowed.json"
    )))
    .expect("session-restore-view fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_collaboration_control_matrix_session_restore_view_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_collaboration_control_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.collaboration_control_rows[0].scope_summary =
        "raw endpoint https://relay.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CollaborationControlMatrixViolation::RawMaterialInExport));
}
