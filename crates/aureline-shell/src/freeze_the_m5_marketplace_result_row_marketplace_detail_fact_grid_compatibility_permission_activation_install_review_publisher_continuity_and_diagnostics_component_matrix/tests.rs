use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_marketplace_install_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_marketplace_install_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5MarketplaceInstallComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5MarketplaceInstallComponentFamily::ALL.len()
    );
}

#[test]
fn frozen_disposition_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: public / mirrored / enterprise / side-load / verified
    // / transferred / deprecated / limited / incompatible / over-budget / throttled / quarantined /
    // disable-scope / rollback-compatibility stays in one controlled token set that no marketplace
    // or install surface reinvents.
    let tokens: Vec<&str> = M5MarketplaceInstallDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "public",
            "mirrored",
            "enterprise",
            "side_load",
            "verified",
            "transferred",
            "deprecated",
            "limited",
            "incompatible",
            "over_budget",
            "throttled",
            "quarantined",
            "disable_scope",
            "rollback_compatibility",
        ]
    );
    assert!(M5MarketplaceInstallDisposition::Verified.is_verified());
    assert!(!M5MarketplaceInstallDisposition::Incompatible.is_verified());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_marketplace_install_component_matrix();
    for row in &packet.component_rows {
        for label in M5MarketplaceInstallRequiredLabel::MANDATORY {
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
        assert!(!row.dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5MarketplaceInstallAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_marketplace_install_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.registry_source_classes.is_empty(),
            family.declares_registry_source(),
            "registry_source_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.compatibility_states.is_empty(),
            family.declares_compatibility(),
            "compatibility_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.host_runtime_models.is_empty(),
            family.declares_host_model(),
            "host_runtime_models presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.permission_postures.is_empty(),
            family.declares_permission_posture(),
            "permission_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.activation_budget_bands.is_empty(),
            family.declares_activation_budget(),
            "activation_budget_bands presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.publisher_continuity_states.is_empty(),
            family.declares_publisher_continuity(),
            "publisher_continuity_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.disable_scope_classes.is_empty(),
            family.declares_disable_scope(),
            "disable_scope_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.rollback_compatibility_states.is_empty(),
            family.declares_rollback_compat(),
            "rollback_compatibility_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.quarantine_states.is_empty(),
            family.declares_quarantine(),
            "quarantine_states presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_marketplace_install_component_matrix();
    for disposition in M5MarketplaceInstallDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for source in M5RegistrySourceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.registry_source_classes.contains(&source)),
            "no component declares registry source {}",
            source.as_str()
        );
    }
    for compat in M5CompatibilityState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.compatibility_states.contains(&compat)),
            "no component declares compatibility state {}",
            compat.as_str()
        );
    }
    for host in M5HostRuntimeModel::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.host_runtime_models.contains(&host)),
            "no component declares host model {}",
            host.as_str()
        );
    }
    for posture in M5PermissionPostureState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.permission_postures.contains(&posture)),
            "no component declares permission posture {}",
            posture.as_str()
        );
    }
    for band in M5ActivationBudgetBandState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.activation_budget_bands.contains(&band)),
            "no component declares activation-budget band {}",
            band.as_str()
        );
    }
    for continuity in M5PublisherContinuityState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.publisher_continuity_states.contains(&continuity)),
            "no component declares publisher continuity {}",
            continuity.as_str()
        );
    }
    for scope in M5DisableScopeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.disable_scope_classes.contains(&scope)),
            "no component declares disable scope {}",
            scope.as_str()
        );
    }
    for rollback in M5RollbackCompatibilityState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.rollback_compatibility_states.contains(&rollback)),
            "no component declares rollback compatibility {}",
            rollback.as_str()
        );
    }
    for quarantine in M5QuarantineState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.quarantine_states.contains(&quarantine)),
            "no component declares quarantine state {}",
            quarantine.as_str()
        );
    }
    for reason in M5MarketplaceInstallDegradedReason::ALL {
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
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5MarketplaceInstallComponentFamily::MarketplaceDetailFactGrid
    });
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5MarketplaceInstallRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    let own =
        M5MarketplaceInstallComponentFamily::MarketplaceResultRow.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5MarketplaceInstallComponentFamily::MarketplaceResultRow
        })
        .expect("marketplace-result row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn disposition_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::DispositionMissing));
}

#[test]
fn marketplace_detail_fact_grid_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_marketplace_install_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5MarketplaceInstallComponentFamily::MarketplaceDetailFactGrid
            })
            .expect("marketplace-detail fact grid present");
        let expected = if clear == 0 {
            row.registry_source_classes.clear();
            M5MarketplaceInstallComponentMatrixViolation::RegistrySourceMissing
        } else {
            row.permission_postures.clear();
            M5MarketplaceInstallComponentMatrixViolation::PermissionPostureMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn compatibility_label_strip_vocab_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5MarketplaceInstallComponentFamily::CompatibilityLabelStrip
        })
        .expect("compatibility-label strip present");
    row.host_runtime_models.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::HostModelMissing));
}

#[test]
fn permission_manifest_summary_vocab_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5MarketplaceInstallComponentFamily::PermissionManifestSummary
        })
        .expect("permission-manifest summary present");
    row.permission_postures.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::PermissionPostureMissing));
}

#[test]
fn activation_budget_band_vocab_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5MarketplaceInstallComponentFamily::ActivationBudgetBand
        })
        .expect("activation-budget band present");
    row.activation_budget_bands.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ActivationBudgetMissing));
}

#[test]
fn install_review_sheet_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_marketplace_install_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5MarketplaceInstallComponentFamily::InstallUpdateDisableRollbackReviewSheet
            })
            .expect("install-review sheet present");
        let expected = if clear == 0 {
            row.disable_scope_classes.clear();
            M5MarketplaceInstallComponentMatrixViolation::DisableScopeMissing
        } else {
            row.rollback_compatibility_states.clear();
            M5MarketplaceInstallComponentMatrixViolation::RollbackCompatibilityMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn publisher_continuity_row_vocab_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5MarketplaceInstallComponentFamily::PublisherContinuityRow
        })
        .expect("publisher-continuity row present");
    row.publisher_continuity_states.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::PublisherContinuityMissing));
}

#[test]
fn installed_state_diagnostics_card_quarantine_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family
                == M5MarketplaceInstallComponentFamily::InstalledStateDiagnosticsCard
        })
        .expect("installed-state diagnostics card present");
    row.quarantine_states.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::QuarantineStateMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[2].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[0].hides_permission_widening_or_activation_cost = true;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[5].hides_publisher_transfer_disable_scope_or_rollback_incompatibility =
        true;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[6].collapses_registry_source_class_across_public_mirrored_enterprise =
        true;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[7].presents_incompatible_or_over_budget_as_ready = true;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5MarketplaceInstallComponentFamily::MarketplaceResultRow
        })
        .expect("marketplace-result row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet
        .governance_review
        .registry_source_class_always_explicit = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_marketplace_source = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_marketplace_install_component_matrix().render_markdown_summary();
    for family in M5MarketplaceInstallComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_marketplace_install_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5MarketplaceInstallComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5MarketplaceInstallComponentFamily::ALL {
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
    let packet = current_stable_m5_marketplace_install_component_matrix_export()
        .expect("checked M5 marketplace-install component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_marketplace_install_component_matrix_export()
        .expect("checked M5 marketplace-install component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_marketplace_install_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_marketplace_install_component_matrix_compatibility_label_strip_beta_narrowed(),
        seeded_m5_marketplace_install_component_matrix_install_review_sheet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5MarketplaceInstallComponentFamily::ALL.len()
        );
    }

    let strip =
        seeded_m5_marketplace_install_component_matrix_compatibility_label_strip_beta_narrowed();
    let row = strip
        .component_rows
        .iter()
        .find(|r| {
            r.component_family == M5MarketplaceInstallComponentFamily::CompatibilityLabelStrip
        })
        .expect("compatibility-label strip row present");
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Beta
    );

    let sheet =
        seeded_m5_marketplace_install_component_matrix_install_review_sheet_preview_narrowed();
    let row = sheet
        .component_rows
        .iter()
        .find(|r| {
            r.component_family
                == M5MarketplaceInstallComponentFamily::InstallUpdateDisableRollbackReviewSheet
        })
        .expect("install-review sheet row present");
    assert_eq!(
        row.qualification,
        M5MarketplaceInstallQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let strip: M5MarketplaceInstallComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-marketplace-install-components/compatibility_label_strip_beta_narrowed.json"
    )))
    .expect("compatibility-label-strip fixture parses");
    assert!(strip.validate().is_empty());
    assert_eq!(
        strip,
        seeded_m5_marketplace_install_component_matrix_compatibility_label_strip_beta_narrowed()
    );

    let sheet: M5MarketplaceInstallComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-marketplace-install-components/install_review_sheet_preview_narrowed.json"
    )))
    .expect("install-review-sheet fixture parses");
    assert!(sheet.validate().is_empty());
    assert_eq!(
        sheet,
        seeded_m5_marketplace_install_component_matrix_install_review_sheet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_marketplace_install_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceInstallComponentMatrixViolation::RawMaterialInExport));
}
