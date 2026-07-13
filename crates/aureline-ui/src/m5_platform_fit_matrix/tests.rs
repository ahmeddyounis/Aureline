use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_platform_fit_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PLATFORM_FIT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_platform_fit_family() {
    let packet = seeded_m5_platform_fit_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .platform_fit_rows
        .iter()
        .map(|r| r.platform_fit_family)
        .collect();
    for family in M5PlatformFitFamily::ALL {
        assert!(
            present.contains(&family),
            "missing platform-fit family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.platform_fit_rows.len(),
        M5PlatformFitFamily::ALL.len()
    );
}

#[test]
fn frozen_platform_fit_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: shortcut / window_menu / path_terminology / appearance /
    // credential_wording / input_fidelity / command_stability stays in one controlled token set that no
    // macOS, Windows, Linux, docs, or support surface reinvents.
    let tokens: Vec<&str> = M5PlatformFitRole::ALL.iter().map(|r| r.as_str()).collect();
    assert_eq!(
        tokens,
        vec![
            "shortcut",
            "window_menu",
            "path_terminology",
            "appearance",
            "credential_wording",
            "input_fidelity",
            "command_stability",
        ]
    );
    assert!(M5PlatformFitRole::Shortcut.must_preserve_command_identity_under_platform_adaptation());
    assert!(
        M5PlatformFitRole::WindowMenu.must_preserve_command_identity_under_platform_adaptation()
    );
    assert!(
        M5PlatformFitRole::InputFidelity.must_preserve_command_identity_under_platform_adaptation()
    );
    assert!(M5PlatformFitRole::CommandStability
        .must_preserve_command_identity_under_platform_adaptation());
    assert!(!M5PlatformFitRole::PathTerminology
        .must_preserve_command_identity_under_platform_adaptation());
    assert!(
        !M5PlatformFitRole::Appearance.must_preserve_command_identity_under_platform_adaptation()
    );
    assert!(!M5PlatformFitRole::CredentialWording
        .must_preserve_command_identity_under_platform_adaptation());
}

#[test]
fn every_family_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_platform_fit_matrix();
    for row in &packet.platform_fit_rows {
        for label in M5PlatformFitRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "family {} missing mandatory label {}",
                row.platform_fit_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.platform_fit_family
                    .canonical_domain_schema_ref()
                    .to_owned()
            ),
            "family {} does not point at its canonical schema",
            row.platform_fit_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5PlatformFitAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_platform_fit_matrix();
    for row in &packet.platform_fit_rows {
        let family = row.platform_fit_family;
        assert_eq!(
            !row.platform_convention_roles.is_empty(),
            family.declares_platform_convention_roles(),
            "platform_convention_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.shortcut_notation_roles.is_empty(),
            family.declares_shortcut_notation_roles(),
            "shortcut_notation_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.file_path_reveal_roles.is_empty(),
            family.declares_file_path_reveal_roles(),
            "file_path_reveal_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.theme_contrast_live_change_roles.is_empty(),
            family.declares_theme_contrast_live_change_roles(),
            "theme_contrast_live_change_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.credential_store_wording_roles.is_empty(),
            family.declares_credential_store_wording_roles(),
            "credential_store_wording_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.input_method_roles.is_empty(),
            family.declares_input_method_roles(),
            "input_method_roles presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_family() {
    let packet = seeded_m5_platform_fit_matrix();
    for role in M5PlatformFitRole::ALL {
        assert!(
            packet
                .platform_fit_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no family declares platform-fit role {}",
            role.as_str()
        );
    }
    for role in M5PlatformConventionRole::ALL {
        assert!(
            packet
                .platform_fit_rows
                .iter()
                .any(|row| row.platform_convention_roles.contains(&role)),
            "no family declares platform-convention role {}",
            role.as_str()
        );
    }
    for role in M5ShortcutNotationRole::ALL {
        assert!(
            packet
                .platform_fit_rows
                .iter()
                .any(|row| row.shortcut_notation_roles.contains(&role)),
            "no family declares shortcut-notation role {}",
            role.as_str()
        );
    }
    for role in M5FilePathRevealRole::ALL {
        assert!(
            packet
                .platform_fit_rows
                .iter()
                .any(|row| row.file_path_reveal_roles.contains(&role)),
            "no family declares file-path-reveal role {}",
            role.as_str()
        );
    }
    for role in M5ThemeContrastLiveChangeRole::ALL {
        assert!(
            packet
                .platform_fit_rows
                .iter()
                .any(|row| row.theme_contrast_live_change_roles.contains(&role)),
            "no family declares theme-contrast-live-change role {}",
            role.as_str()
        );
    }
    for role in M5CredentialStoreWordingRole::ALL {
        assert!(
            packet
                .platform_fit_rows
                .iter()
                .any(|row| row.credential_store_wording_roles.contains(&role)),
            "no family declares credential-store-wording role {}",
            role.as_str()
        );
    }
    for role in M5InputMethodRole::ALL {
        assert!(
            packet
                .platform_fit_rows
                .iter()
                .any(|row| row.input_method_roles.contains(&role)),
            "no family declares input-method role {}",
            role.as_str()
        );
    }
    for reason in M5PlatformFitDegradedReason::ALL {
        assert!(
            packet
                .platform_fit_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no family declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_platform_fit_family_fails_validation() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet
        .platform_fit_rows
        .retain(|row| row.platform_fit_family != M5PlatformFitFamily::InputMethod);
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[0]
        .required_labels
        .retain(|label| *label != M5PlatformFitRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    let own = M5PlatformFitFamily::ShortcutNotation.canonical_domain_schema_ref();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::ShortcutNotation)
        .expect("shortcut-notation row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::SemanticRoleMissing));
}

#[test]
fn platform_convention_role_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::PlatformConvention)
        .expect("platform-convention present");
    row.platform_convention_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::PlatformConventionRoleMissing));
}

#[test]
fn shortcut_notation_role_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::ShortcutNotation)
        .expect("shortcut-notation present");
    row.shortcut_notation_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::ShortcutNotationRoleMissing));
}

#[test]
fn file_path_reveal_role_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::FilePathReveal)
        .expect("file-path-reveal present");
    row.file_path_reveal_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::FilePathRevealRoleMissing));
}

#[test]
fn theme_contrast_live_change_role_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::ThemeContrastLiveChange)
        .expect("theme-contrast-live-change present");
    row.theme_contrast_live_change_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::ThemeContrastLiveChangeRoleMissing));
}

#[test]
fn credential_store_wording_role_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::CredentialStoreWording)
        .expect("credential-store-wording present");
    row.credential_store_wording_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::CredentialStoreWordingRoleMissing));
}

#[test]
fn input_method_role_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::InputMethod)
        .expect("input-method present");
    row.input_method_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::InputMethodRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::DegradedReasonMissing));
}

#[test]
fn platform_fit_invariant_violation_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[0].platform_wording_changes_command_or_permission_meaning = true;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::PlatformFitInvariantViolated));

    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[0].hides_primary_action_only_in_os_chrome = true;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::PlatformFitInvariantViolated));

    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[4].falls_back_to_plaintext_secret_storage_silently = true;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::PlatformFitInvariantViolated));

    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[5].input_method_corrupts_text_or_trust_fidelity = true;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::PlatformFitInvariantViolated));

    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[1].screenshot_or_docs_mislabels_shortcut_or_path_verb = true;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::PlatformFitInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    let row = packet
        .platform_fit_rows
        .iter_mut()
        .find(|row| row.platform_fit_family == M5PlatformFitFamily::ShortcutNotation)
        .expect("shortcut-notation row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet
        .governance_review
        .command_ids_stable_while_labels_adapt = false;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_platform_fit_source = false;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_platform_fit_family() {
    let summary = seeded_m5_platform_fit_matrix().render_markdown_summary();
    for family in M5PlatformFitFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_platform_fit_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5PlatformFitFamily::ALL.len());
    assert!(lines[0].starts_with("platform_fit_family,qualification,owner,canonical_schema,"));
    for family in M5PlatformFitFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing family {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_platform_fit_matrix_export()
        .expect("checked M5 platform-fit matrix export validates");
    assert_eq!(packet.packet_id, M5_PLATFORM_FIT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_platform_fit_matrix_export()
        .expect("checked M5 platform-fit matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_platform_fit_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_platform_fit_matrix_theme_contrast_live_change_beta_narrowed(),
        seeded_m5_platform_fit_matrix_input_method_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.platform_fit_rows.len(),
            M5PlatformFitFamily::ALL.len()
        );
    }

    let theme = seeded_m5_platform_fit_matrix_theme_contrast_live_change_beta_narrowed();
    let row = theme
        .platform_fit_rows
        .iter()
        .find(|r| r.platform_fit_family == M5PlatformFitFamily::ThemeContrastLiveChange)
        .expect("theme-contrast-live-change row present");
    assert_eq!(row.qualification, M5PlatformFitQualificationClass::Beta);

    let input = seeded_m5_platform_fit_matrix_input_method_preview_narrowed();
    let row = input
        .platform_fit_rows
        .iter()
        .find(|r| r.platform_fit_family == M5PlatformFitFamily::InputMethod)
        .expect("input-method row present");
    assert_eq!(row.qualification, M5PlatformFitQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let theme: M5PlatformFitMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-desktop-fit/theme_contrast_live_change_beta_narrowed.json"
    )))
    .expect("theme-contrast fixture parses");
    assert!(theme.validate().is_empty());
    assert_eq!(
        theme,
        seeded_m5_platform_fit_matrix_theme_contrast_live_change_beta_narrowed()
    );

    let input: M5PlatformFitMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-desktop-fit/input_method_preview_narrowed.json"
    )))
    .expect("input-method fixture parses");
    assert!(input.validate().is_empty());
    assert_eq!(
        input,
        seeded_m5_platform_fit_matrix_input_method_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_platform_fit_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_platform_fit_matrix();
    packet.platform_fit_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PlatformFitMatrixViolation::RawMaterialInExport));
}
