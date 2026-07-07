use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_local_history_write_scope_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_local_history_write_scope_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5LocalHistoryWriteScopeComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5LocalHistoryWriteScopeComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_local_history_write_scope_component_matrix();
    for row in &packet.component_rows {
        for label in M5HistoryRequiredLabel::MANDATORY {
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
            .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_local_history_write_scope_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.snapshot_origins.is_empty(),
            family.is_local_history_row(),
            "snapshot_origins presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.actor_classes.is_empty(),
            family.is_local_history_row(),
            "actor_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.capture_fidelities.is_empty(),
            family.is_local_history_row(),
            "capture_fidelities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.checkpoint_lineage_classes.is_empty(),
            family.is_checkpoint_group_card(),
            "checkpoint_lineage_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.mutation_classes.is_empty(),
            family.is_checkpoint_group_card(),
            "mutation_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.restore_granularities.is_empty(),
            family.is_restore_preview_card(),
            "restore_granularities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.restore_drift_states.is_empty(),
            family.is_restore_preview_card(),
            "restore_drift_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.retention_postures.is_empty(),
            family.is_retention_export_card(),
            "retention_postures presence wrong for {}",
            family.as_str()
        );
        // Export-redaction posture is shared by the retention/export card and the
        // history-export manifest.
        assert_eq!(
            !row.export_redaction_postures.is_empty(),
            family.is_retention_export_card() || family.is_history_export_manifest(),
            "export_redaction_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.write_scope_classes.is_empty(),
            family.is_write_scope_preview_tree(),
            "write_scope_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.managed_file_caveats.is_empty(),
            family.is_write_scope_preview_tree(),
            "managed_file_caveats presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.restore_selection_modes.is_empty(),
            family.is_restore_granularity_selector(),
            "restore_selection_modes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.export_manifest_classes.is_empty(),
            family.is_history_export_manifest(),
            "export_manifest_classes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_local_history_write_scope_component_matrix();
    for origin in M5SnapshotOrigin::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.snapshot_origins.contains(&origin)),
            "no component declares snapshot origin {}",
            origin.as_str()
        );
    }
    for actor in M5HistoryActorClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.actor_classes.contains(&actor)),
            "no component declares actor class {}",
            actor.as_str()
        );
    }
    for fidelity in M5CaptureFidelity::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.capture_fidelities.contains(&fidelity)),
            "no component declares capture fidelity {}",
            fidelity.as_str()
        );
    }
    for lineage in M5CheckpointLineageClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.checkpoint_lineage_classes.contains(&lineage)),
            "no component declares checkpoint lineage {}",
            lineage.as_str()
        );
    }
    for class in M5MutationClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.mutation_classes.contains(&class)),
            "no component declares mutation class {}",
            class.as_str()
        );
    }
    for granularity in M5RestoreGranularity::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.restore_granularities.contains(&granularity)),
            "no component declares restore granularity {}",
            granularity.as_str()
        );
    }
    for drift in M5RestoreDriftState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.restore_drift_states.contains(&drift)),
            "no component declares restore drift state {}",
            drift.as_str()
        );
    }
    for posture in M5RetentionPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.retention_postures.contains(&posture)),
            "no component declares retention posture {}",
            posture.as_str()
        );
    }
    for posture in M5ExportRedactionPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.export_redaction_postures.contains(&posture)),
            "no component declares export-redaction posture {}",
            posture.as_str()
        );
    }
    for class in M5WriteScopeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.write_scope_classes.contains(&class)),
            "no component declares write-scope class {}",
            class.as_str()
        );
    }
    for caveat in M5ManagedFileCaveat::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.managed_file_caveats.contains(&caveat)),
            "no component declares managed-file caveat {}",
            caveat.as_str()
        );
    }
    for mode in M5RestoreSelectionMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.restore_selection_modes.contains(&mode)),
            "no component declares restore-selection mode {}",
            mode.as_str()
        );
    }
    for class in M5ExportManifestClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.export_manifest_classes.contains(&class)),
            "no component declares export-manifest class {}",
            class.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree
    });
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.vocabulary_set.snapshot_origins.pop();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5HistoryRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn local_history_row_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_local_history_write_scope_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow
            })
            .expect("local-history row present");
        let expected = match clear {
            0 => {
                row.snapshot_origins.clear();
                M5LocalHistoryWriteScopeComponentMatrixViolation::SnapshotOriginMissing
            }
            1 => {
                row.actor_classes.clear();
                M5LocalHistoryWriteScopeComponentMatrixViolation::ActorClassMissing
            }
            _ => {
                row.capture_fidelities.clear();
                M5LocalHistoryWriteScopeComponentMatrixViolation::CaptureFidelityMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn checkpoint_group_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_local_history_write_scope_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5LocalHistoryWriteScopeComponentFamily::CheckpointGroupCard
            })
            .expect("checkpoint-group card present");
        let expected = if clear == 0 {
            row.checkpoint_lineage_classes.clear();
            M5LocalHistoryWriteScopeComponentMatrixViolation::CheckpointLineageMissing
        } else {
            row.mutation_classes.clear();
            M5LocalHistoryWriteScopeComponentMatrixViolation::MutationClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn restore_preview_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_local_history_write_scope_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5LocalHistoryWriteScopeComponentFamily::RestorePreviewCard
            })
            .expect("restore-preview card present");
        let expected = if clear == 0 {
            row.restore_granularities.clear();
            M5LocalHistoryWriteScopeComponentMatrixViolation::RestoreGranularityMissing
        } else {
            row.restore_drift_states.clear();
            M5LocalHistoryWriteScopeComponentMatrixViolation::RestoreDriftStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn retention_export_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_local_history_write_scope_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5LocalHistoryWriteScopeComponentFamily::RetentionExportCard
            })
            .expect("retention/export card present");
        let expected = if clear == 0 {
            row.retention_postures.clear();
            M5LocalHistoryWriteScopeComponentMatrixViolation::RetentionPostureMissing
        } else {
            row.export_redaction_postures.clear();
            M5LocalHistoryWriteScopeComponentMatrixViolation::ExportRedactionPostureMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn write_scope_preview_tree_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_local_history_write_scope_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree
            })
            .expect("write-scope preview tree present");
        let expected = if clear == 0 {
            row.write_scope_classes.clear();
            M5LocalHistoryWriteScopeComponentMatrixViolation::WriteScopeClassMissing
        } else {
            row.managed_file_caveats.clear();
            M5LocalHistoryWriteScopeComponentMatrixViolation::ManagedFileCaveatMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn restore_granularity_selector_vocab_missing_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family
                == M5LocalHistoryWriteScopeComponentFamily::RestoreGranularitySelector
        })
        .expect("restore-granularity selector present");
    row.restore_selection_modes.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::RestoreSelectionModeMissing));
}

#[test]
fn history_export_manifest_vocab_missing_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5LocalHistoryWriteScopeComponentFamily::HistoryExportManifest
        })
        .expect("history-export manifest present");
    row.export_manifest_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ExportManifestClassMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.component_rows[0].masks_actor_or_timestamp = true;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.component_rows[4].hides_generated_or_managed_caveat = true;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.component_rows[2].invents_private_history_grammar = true;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.component_rows[5].bypasses_restore_scope_review = true;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow
        })
        .expect("local-history row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet
        .governance_review
        .partial_restore_never_shown_as_whole_snapshot = false;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet
        .consumer_projection
        .refactor_and_ai_surfaces_read_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryWriteScopeComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_local_history_write_scope_component_matrix().render_markdown_summary();
    for family in M5LocalHistoryWriteScopeComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_local_history_write_scope_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5LocalHistoryWriteScopeComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5LocalHistoryWriteScopeComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_local_history_write_scope_component_matrix_export()
        .expect("checked M5 local-history write-scope component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_local_history_write_scope_component_matrix_export()
        .expect("checked M5 local-history write-scope component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_local_history_write_scope_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_local_history_write_scope_component_matrix_write_scope_preview_tree_beta_narrowed(),
        seeded_m5_local_history_write_scope_component_matrix_history_export_manifest_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5LocalHistoryWriteScopeComponentFamily::ALL.len()
        );
    }

    let scope =
        seeded_m5_local_history_write_scope_component_matrix_write_scope_preview_tree_beta_narrowed(
        );
    let row = scope
        .component_rows
        .iter()
        .find(|r| {
            r.component_family == M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree
        })
        .expect("write-scope-preview-tree row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Beta);

    let manifest =
        seeded_m5_local_history_write_scope_component_matrix_history_export_manifest_preview_narrowed();
    let row = manifest
        .component_rows
        .iter()
        .find(|r| {
            r.component_family == M5LocalHistoryWriteScopeComponentFamily::HistoryExportManifest
        })
        .expect("history-export-manifest row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let scope: M5LocalHistoryWriteScopeComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-local-history-write-scope-components/write_scope_preview_tree_beta_narrowed.json"
        )))
        .expect("write-scope-preview-tree fixture parses");
    assert!(scope.validate().is_empty());
    assert_eq!(
        scope,
        seeded_m5_local_history_write_scope_component_matrix_write_scope_preview_tree_beta_narrowed(
        )
    );

    let manifest: M5LocalHistoryWriteScopeComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-local-history-write-scope-components/history_export_manifest_preview_narrowed.json"
        )))
        .expect("history-export-manifest fixture parses");
    assert!(manifest.validate().is_empty());
    assert_eq!(
        manifest,
        seeded_m5_local_history_write_scope_component_matrix_history_export_manifest_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_local_history_write_scope_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
