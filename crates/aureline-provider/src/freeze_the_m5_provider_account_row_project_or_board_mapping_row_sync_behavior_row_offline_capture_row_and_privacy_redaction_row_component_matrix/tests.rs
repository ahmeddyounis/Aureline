use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_provider_account_offline_capture_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_provider_account_offline_capture_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5ProviderAccountOfflineComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5ProviderAccountOfflineComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_provider_account_offline_capture_component_matrix();
    for row in &packet.component_rows {
        for label in M5ProviderRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_provider_account_offline_capture_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.provider_identity_classes.is_empty(),
            family.is_provider_account_row(),
            "provider_identity_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.account_connection_states.is_empty(),
            family.is_provider_account_row(),
            "account_connection_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.tenant_scopes.is_empty(),
            family.is_provider_account_row(),
            "tenant_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.mapping_origins.is_empty(),
            family.is_project_or_board_mapping_row(),
            "mapping_origins presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.mapping_target_kinds.is_empty(),
            family.is_project_or_board_mapping_row(),
            "mapping_target_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.sync_modes.is_empty(),
            family.is_sync_behavior_row(),
            "sync_modes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.write_scopes.is_empty(),
            family.is_sync_behavior_row(),
            "write_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.offline_capture_states.is_empty(),
            family.is_offline_capture_row(),
            "offline_capture_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.queued_draft_states.is_empty(),
            family.is_sync_behavior_row() || family.is_offline_capture_row(),
            "queued_draft_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.redaction_classes.is_empty(),
            family.is_privacy_redaction_row(),
            "redaction_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.export_boundaries.is_empty(),
            family.is_privacy_redaction_row(),
            "export_boundaries presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_provider_account_offline_capture_component_matrix();
    for identity in M5ProviderIdentityClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.provider_identity_classes.contains(&identity)),
            "no component declares provider identity class {}",
            identity.as_str()
        );
    }
    for state in M5AccountConnectionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.account_connection_states.contains(&state)),
            "no component declares account connection state {}",
            state.as_str()
        );
    }
    for scope in M5TenantScopeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.tenant_scopes.contains(&scope)),
            "no component declares tenant scope {}",
            scope.as_str()
        );
    }
    for origin in M5MappingOriginClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.mapping_origins.contains(&origin)),
            "no component declares mapping origin {}",
            origin.as_str()
        );
    }
    for target in M5MappingTargetKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.mapping_target_kinds.contains(&target)),
            "no component declares mapping target kind {}",
            target.as_str()
        );
    }
    for mode in M5ProviderSyncMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.sync_modes.contains(&mode)),
            "no component declares sync mode {}",
            mode.as_str()
        );
    }
    for scope in M5ProviderWriteScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.write_scopes.contains(&scope)),
            "no component declares write scope {}",
            scope.as_str()
        );
    }
    for state in M5OfflineCaptureState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.offline_capture_states.contains(&state)),
            "no component declares offline capture state {}",
            state.as_str()
        );
    }
    for state in M5QueuedDraftState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.queued_draft_states.contains(&state)),
            "no component declares queued draft state {}",
            state.as_str()
        );
    }
    for class in M5ProviderRedactionClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.redaction_classes.contains(&class)),
            "no component declares redaction class {}",
            class.as_str()
        );
    }
    for boundary in M5ExportBoundaryClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.export_boundaries.contains(&boundary)),
            "no component declares export boundary {}",
            boundary.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5ProviderAccountOfflineComponentFamily::SyncBehaviorRow
    });
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.vocabulary_set.account_connection_states.pop();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5ProviderRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn provider_account_row_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ProviderAccountOfflineComponentFamily::ProviderAccountRow
            })
            .expect("provider-account row present");
        let expected = match clear {
            0 => {
                row.provider_identity_classes.clear();
                M5ProviderAccountOfflineComponentMatrixViolation::ProviderIdentityClassMissing
            }
            1 => {
                row.account_connection_states.clear();
                M5ProviderAccountOfflineComponentMatrixViolation::AccountConnectionStateMissing
            }
            _ => {
                row.tenant_scopes.clear();
                M5ProviderAccountOfflineComponentMatrixViolation::TenantScopeMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn mapping_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5ProviderAccountOfflineComponentFamily::ProjectOrBoardMappingRow
            })
            .expect("mapping row present");
        let expected = if clear == 0 {
            row.mapping_origins.clear();
            M5ProviderAccountOfflineComponentMatrixViolation::MappingOriginMissing
        } else {
            row.mapping_target_kinds.clear();
            M5ProviderAccountOfflineComponentMatrixViolation::MappingTargetKindMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn sync_behavior_row_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ProviderAccountOfflineComponentFamily::SyncBehaviorRow
            })
            .expect("sync-behavior row present");
        let expected = match clear {
            0 => {
                row.sync_modes.clear();
                M5ProviderAccountOfflineComponentMatrixViolation::SyncModeMissing
            }
            1 => {
                row.write_scopes.clear();
                M5ProviderAccountOfflineComponentMatrixViolation::WriteScopeMissing
            }
            _ => {
                row.queued_draft_states.clear();
                M5ProviderAccountOfflineComponentMatrixViolation::QueuedDraftStateMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn offline_capture_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ProviderAccountOfflineComponentFamily::OfflineCaptureRow
            })
            .expect("offline-capture row present");
        let expected = if clear == 0 {
            row.offline_capture_states.clear();
            M5ProviderAccountOfflineComponentMatrixViolation::OfflineCaptureStateMissing
        } else {
            row.queued_draft_states.clear();
            M5ProviderAccountOfflineComponentMatrixViolation::QueuedDraftStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn privacy_redaction_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ProviderAccountOfflineComponentFamily::PrivacyRedactionRow
            })
            .expect("privacy-redaction row present");
        let expected = if clear == 0 {
            row.redaction_classes.clear();
            M5ProviderAccountOfflineComponentMatrixViolation::RedactionClassMissing
        } else {
            row.export_boundaries.clear();
            M5ProviderAccountOfflineComponentMatrixViolation::ExportBoundaryMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.component_rows[0].masks_connection_or_scope = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.component_rows[4].hides_export_or_redaction_boundary = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.component_rows[2].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.component_rows[1].assumes_default_destination_silently = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5ProviderAccountOfflineComponentFamily::ProviderAccountRow
        })
        .expect("provider-account row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountOfflineComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary =
        seeded_m5_provider_account_offline_capture_component_matrix().render_markdown_summary();
    for family in M5ProviderAccountOfflineComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_provider_account_offline_capture_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5ProviderAccountOfflineComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5ProviderAccountOfflineComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_provider_account_offline_capture_component_matrix_export()
        .expect("checked M5 provider-account offline-capture component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_provider_account_offline_capture_component_matrix_export()
        .expect("checked M5 provider-account offline-capture component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_provider_account_offline_capture_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_provider_account_offline_capture_component_matrix_offline_capture_row_beta_narrowed(),
        seeded_m5_provider_account_offline_capture_component_matrix_privacy_redaction_row_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5ProviderAccountOfflineComponentFamily::ALL.len()
        );
    }

    let offline =
        seeded_m5_provider_account_offline_capture_component_matrix_offline_capture_row_beta_narrowed();
    let row = offline
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ProviderAccountOfflineComponentFamily::OfflineCaptureRow)
        .expect("offline-capture-row row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Beta);

    let privacy =
        seeded_m5_provider_account_offline_capture_component_matrix_privacy_redaction_row_preview_narrowed();
    let row = privacy
        .component_rows
        .iter()
        .find(|r| {
            r.component_family == M5ProviderAccountOfflineComponentFamily::PrivacyRedactionRow
        })
        .expect("privacy-redaction-row row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let offline: M5ProviderAccountOfflineComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-provider-account-offline-capture-components/offline_capture_row_beta_narrowed.json"
        )))
        .expect("offline-capture-row fixture parses");
    assert!(offline.validate().is_empty());
    assert_eq!(
        offline,
        seeded_m5_provider_account_offline_capture_component_matrix_offline_capture_row_beta_narrowed()
    );

    let privacy: M5ProviderAccountOfflineComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-provider-account-offline-capture-components/privacy_redaction_row_preview_narrowed.json"
        )))
        .expect("privacy-redaction-row fixture parses");
    assert!(privacy.validate().is_empty());
    assert_eq!(
        privacy,
        seeded_m5_provider_account_offline_capture_component_matrix_privacy_redaction_row_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_provider_account_offline_capture_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
