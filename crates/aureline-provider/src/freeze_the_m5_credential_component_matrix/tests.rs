use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_credential_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CREDENTIAL_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_credential_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5CredentialComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5CredentialComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_credential_component_matrix();
    for row in &packet.component_rows {
        for label in M5CredentialRequiredLabel::MANDATORY {
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
        assert!(!row.degraded_states.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_credential_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.storage_modes.is_empty(),
            family.declares_storage_mode(),
            "storage_modes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.credential_classes.is_empty(),
            family.declares_credential_class(),
            "credential_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.reveal_postures.is_empty(),
            family.declares_reveal_posture(),
            "reveal_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.auth_handoff_classes.is_empty(),
            family.declares_auth_handoff_class(),
            "auth_handoff_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.delegated_identity_states.is_empty(),
            family.declares_delegated_identity_state(),
            "delegated_identity_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.lifecycle_states.is_empty(),
            family.declares_lifecycle_state(),
            "lifecycle_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.store_capabilities.is_empty(),
            family.declares_store_capability(),
            "store_capabilities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.export_safety_classes.is_empty(),
            family.declares_export_safety_class(),
            "export_safety_classes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_credential_component_matrix();
    for mode in M5CredentialStorageMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.storage_modes.contains(&mode)),
            "no component declares storage mode {}",
            mode.as_str()
        );
    }
    for class in M5CredentialClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.credential_classes.contains(&class)),
            "no component declares credential class {}",
            class.as_str()
        );
    }
    for posture in M5CredentialRevealPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.reveal_postures.contains(&posture)),
            "no component declares reveal posture {}",
            posture.as_str()
        );
    }
    for handoff in M5AuthHandoffClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.auth_handoff_classes.contains(&handoff)),
            "no component declares auth handoff class {}",
            handoff.as_str()
        );
    }
    for state in M5DelegatedIdentityState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.delegated_identity_states.contains(&state)),
            "no component declares delegated identity state {}",
            state.as_str()
        );
    }
    for state in M5CredentialLifecycleState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.lifecycle_states.contains(&state)),
            "no component declares lifecycle state {}",
            state.as_str()
        );
    }
    for capability in M5CredentialStoreCapability::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.store_capabilities.contains(&capability)),
            "no component declares store capability {}",
            capability.as_str()
        );
    }
    for class in M5CredentialExportSafetyClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.export_safety_classes.contains(&class)),
            "no component declares export safety class {}",
            class.as_str()
        );
    }
    for state in M5CredentialDegradedState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.degraded_states.contains(&state)),
            "no component declares degraded state {}",
            state.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5CredentialComponentFamily::VaultOrKeychainPicker);
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.vocabulary_set.storage_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5CredentialRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    let own = M5CredentialComponentFamily::CredentialStateRow.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CredentialComponentFamily::CredentialStateRow)
        .expect("credential-state row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn credential_state_row_vocab_missing_fails() {
    for clear in [0u8, 1, 2, 3] {
        let mut packet = seeded_m5_credential_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5CredentialComponentFamily::CredentialStateRow)
            .expect("credential-state row present");
        let expected = match clear {
            0 => {
                row.storage_modes.clear();
                M5CredentialComponentMatrixViolation::StorageModeMissing
            }
            1 => {
                row.credential_classes.clear();
                M5CredentialComponentMatrixViolation::CredentialClassMissing
            }
            2 => {
                row.reveal_postures.clear();
                M5CredentialComponentMatrixViolation::RevealPostureMissing
            }
            _ => {
                row.lifecycle_states.clear();
                M5CredentialComponentMatrixViolation::LifecycleStateMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn prompt_sheet_vocab_missing_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CredentialComponentFamily::SecretAccessPromptSheet)
        .expect("secret-access-prompt sheet present");
    row.auth_handoff_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::AuthHandoffClassMissing));
}

#[test]
fn vault_picker_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_credential_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5CredentialComponentFamily::VaultOrKeychainPicker)
            .expect("vault/keychain picker present");
        let expected = if clear == 0 {
            row.storage_modes.clear();
            M5CredentialComponentMatrixViolation::StorageModeMissing
        } else {
            row.store_capabilities.clear();
            M5CredentialComponentMatrixViolation::StoreCapabilityMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn delegated_row_vocab_missing_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CredentialComponentFamily::DelegatedCredentialRow)
        .expect("delegated-credential row present");
    row.delegated_identity_states.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::DelegatedIdentityStateMissing));
}

#[test]
fn export_safety_banner_vocab_missing_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CredentialComponentFamily::ExportSafetyBanner)
        .expect("export-safety banner present");
    row.export_safety_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ExportSafetyClassMissing));
}

#[test]
fn degraded_state_missing_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[3].degraded_states.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::DegradedStateMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[0].masks_storage_or_reveal_posture = true;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[5].hides_identity_delegation = true;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[2].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[7].implies_raw_secret_exportable = true;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CredentialComponentFamily::CredentialStateRow)
        .expect("credential-state row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_credential_component_matrix().render_markdown_summary();
    for family in M5CredentialComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_credential_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CredentialComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5CredentialComponentFamily::ALL {
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
    let packet = current_stable_m5_credential_component_matrix_export()
        .expect("checked M5 credential component matrix export validates");
    assert_eq!(packet.packet_id, M5_CREDENTIAL_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_credential_component_matrix_export()
        .expect("checked M5 credential component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_credential_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_credential_component_matrix_browser_device_code_handoff_card_beta_narrowed(),
        seeded_m5_credential_component_matrix_export_safety_banner_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5CredentialComponentFamily::ALL.len()
        );
    }

    let handoff =
        seeded_m5_credential_component_matrix_browser_device_code_handoff_card_beta_narrowed();
    let row = handoff
        .component_rows
        .iter()
        .find(|r| r.component_family == M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard)
        .expect("browser-device-code-handoff-card row present");
    assert_eq!(row.qualification, M5CredentialQualificationClass::Beta);

    let export = seeded_m5_credential_component_matrix_export_safety_banner_preview_narrowed();
    let row = export
        .component_rows
        .iter()
        .find(|r| r.component_family == M5CredentialComponentFamily::ExportSafetyBanner)
        .expect("export-safety-banner row present");
    assert_eq!(row.qualification, M5CredentialQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let handoff: M5CredentialComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-credential-components/browser_device_code_handoff_card_beta_narrowed.json"
    )))
    .expect("browser-device-code-handoff-card fixture parses");
    assert!(handoff.validate().is_empty());
    assert_eq!(
        handoff,
        seeded_m5_credential_component_matrix_browser_device_code_handoff_card_beta_narrowed()
    );

    let export: M5CredentialComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-credential-components/export_safety_banner_preview_narrowed.json"
    )))
    .expect("export-safety-banner fixture parses");
    assert!(export.validate().is_empty());
    assert_eq!(
        export,
        seeded_m5_credential_component_matrix_export_safety_banner_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_credential_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    // The controlled vocabulary intentionally uses the words `secret` and `credential`;
    // what must never appear is a raw secret *value* shape.
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_credential_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://vault.internal.example/secret leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentMatrixViolation::RawMaterialInExport));
}
