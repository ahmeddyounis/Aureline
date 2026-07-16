use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_historical_reference_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_HISTORICAL_REFERENCE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_historical_reference_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .historical_reference_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5HistoricalReferenceObject::ALL {
        assert!(
            present.contains(&class),
            "missing object class {}",
            class.as_str()
        );
    }
    assert_eq!(
        packet.historical_reference_rows.len(),
        M5HistoricalReferenceObject::ALL.len()
    );
}

#[test]
fn frozen_retirement_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5HistoricalReferenceRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "snapshot_labeling",
            "capture_time_attribution",
            "provenance_attribution",
            "mutation_blocked_posture",
            "live_target_handoff",
            "imported_offline_disclosure",
            "expiry_removal_handling",
        ]
    );
    assert!(M5HistoricalReferenceRole::SnapshotLabeling
        .must_be_present_before_surfacing_as_non_live_evidence());
    assert!(M5HistoricalReferenceRole::CaptureTimeAttribution
        .must_be_present_before_surfacing_as_non_live_evidence());
    assert!(M5HistoricalReferenceRole::ProvenanceAttribution
        .must_be_present_before_surfacing_as_non_live_evidence());
    assert!(M5HistoricalReferenceRole::MutationBlockedPosture
        .must_be_present_before_surfacing_as_non_live_evidence());
    assert!(!M5HistoricalReferenceRole::LiveTargetHandoff
        .must_be_present_before_surfacing_as_non_live_evidence());
    assert!(!M5HistoricalReferenceRole::ImportedOfflineDisclosure
        .must_be_present_before_surfacing_as_non_live_evidence());
    assert!(!M5HistoricalReferenceRole::ExpiryRemovalHandling
        .must_be_present_before_surfacing_as_non_live_evidence());
}

#[test]
fn non_live_evidence_is_mechanically_distinct_from_other_evidence_states() {
    let tokens: Vec<&str> = M5HistoricalReferenceEvidenceState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "live_object",
            "cached_current_state",
            "restore_capable_workspace",
            "archived_snapshot",
            "imported_offline_evidence",
        ]
    );
    assert!(M5HistoricalReferenceEvidenceState::ArchivedSnapshot.is_non_live_evidence());
    assert!(M5HistoricalReferenceEvidenceState::ImportedOfflineEvidence.is_non_live_evidence());
    assert!(!M5HistoricalReferenceEvidenceState::LiveObject.is_non_live_evidence());
    assert!(!M5HistoricalReferenceEvidenceState::CachedCurrentState.is_non_live_evidence());
    assert!(!M5HistoricalReferenceEvidenceState::RestoreCapableWorkspace.is_non_live_evidence());
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_horizon_stages() {
    let packet = seeded_m5_historical_reference_matrix();
    for row in &packet.historical_reference_rows {
        for label in M5HistoricalReferenceRequiredLabel::MANDATORY {
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
        assert!(!row.capture_lifecycle_stages.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5HistoricalReferenceAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_visible_state() {
    let packet = seeded_m5_historical_reference_matrix();
    for row in &packet.historical_reference_rows {
        let tr = &row.required_visible_state;
        for field in [
            &tr.snapshot_label,
            &tr.capture_time,
            &tr.provenance,
            &tr.live_target_availability,
            &tr.imported_offline_status,
            &tr.mutation_blocked_posture,
            &tr.expiry_removal_state,
            &tr.live_target_handoff_or_exit,
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
    let packet = seeded_m5_historical_reference_matrix();
    for row in &packet.historical_reference_rows {
        let class = row.object_class;
        assert_eq!(
            !row.retirement_snapshot_roles.is_empty(),
            class.declares_retirement_snapshot_roles(),
            "retirement_snapshot_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.support_export_evidence_roles.is_empty(),
            class.declares_support_export_evidence_roles(),
            "support_export_evidence_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.archived_runbook_packet_roles.is_empty(),
            class.declares_archived_runbook_packet_roles(),
            "archived_runbook_packet_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.imported_offline_route_evidence_roles.is_empty(),
            class.declares_imported_offline_route_evidence_roles(),
            "imported_offline_route_evidence_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.review_incident_snapshot_roles.is_empty(),
            class.declares_review_incident_snapshot_roles(),
            "review_incident_snapshot_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_historical_reference_matrix();
    for role in M5HistoricalReferenceRole::ALL {
        assert!(
            packet
                .historical_reference_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares retirement role {}",
            role.as_str()
        );
    }
    for role in M5HistoricalReferenceRetirementSnapshotRole::ALL {
        assert!(
            packet
                .historical_reference_rows
                .iter()
                .any(|row| row.retirement_snapshot_roles.contains(&role)),
            "no class declares retirement_snapshot_role {}",
            role.as_str()
        );
    }
    for role in M5HistoricalReferenceSupportExportEvidenceRole::ALL {
        assert!(
            packet
                .historical_reference_rows
                .iter()
                .any(|row| row.support_export_evidence_roles.contains(&role)),
            "no class declares support_export_evidence_role {}",
            role.as_str()
        );
    }
    for role in M5HistoricalReferenceArchivedRunbookPacketRole::ALL {
        assert!(
            packet
                .historical_reference_rows
                .iter()
                .any(|row| row.archived_runbook_packet_roles.contains(&role)),
            "no class declares archived_runbook_packet_role {}",
            role.as_str()
        );
    }
    for role in M5HistoricalReferenceImportedOfflineRouteEvidenceRole::ALL {
        assert!(
            packet
                .historical_reference_rows
                .iter()
                .any(|row| row.imported_offline_route_evidence_roles.contains(&role)),
            "no class declares imported_offline_route_evidence_role {}",
            role.as_str()
        );
    }
    for role in M5HistoricalReferenceReviewIncidentSnapshotRole::ALL {
        assert!(
            packet
                .historical_reference_rows
                .iter()
                .any(|row| row.review_incident_snapshot_roles.contains(&role)),
            "no class declares review_incident_snapshot_role {}",
            role.as_str()
        );
    }
    for reason in M5HistoricalReferenceDegradedReason::ALL {
        assert!(
            packet
                .historical_reference_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet
        .historical_reference_rows
        .retain(|row| row.object_class != M5HistoricalReferenceObject::ReviewIncidentSnapshot);
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[0]
        .required_labels
        .retain(|label| *label != M5HistoricalReferenceRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    let own = M5HistoricalReferenceObject::SupportExportEvidence.canonical_domain_schema_ref();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::SupportExportEvidence)
        .expect("support-export-evidence row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::SemanticRoleMissing));
}

#[test]
fn retirement_snapshot_roles_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::RetirementSnapshot)
        .expect("RetirementSnapshot row present");
    row.retirement_snapshot_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::RetirementSnapshotRoleMissing));
}

#[test]
fn support_export_evidence_roles_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::SupportExportEvidence)
        .expect("SupportExportEvidence row present");
    row.support_export_evidence_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::SupportExportEvidenceRoleMissing));
}

#[test]
fn archived_runbook_packet_roles_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::ArchivedRunbookPacket)
        .expect("ArchivedRunbookPacket row present");
    row.archived_runbook_packet_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::ArchivedRunbookPacketRoleMissing));
}

#[test]
fn imported_offline_route_evidence_roles_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::ImportedOfflineRouteEvidence)
        .expect("ImportedOfflineRouteEvidence row present");
    row.imported_offline_route_evidence_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::ImportedOfflineRouteEvidenceRoleMissing));
}

#[test]
fn review_incident_snapshot_roles_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::ReviewIncidentSnapshot)
        .expect("ReviewIncidentSnapshot row present");
    row.review_incident_snapshot_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::ReviewIncidentSnapshotRoleMissing));
}

#[test]
fn visible_state_incomplete_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[0]
        .required_visible_state
        .snapshot_label
        .clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::VisibleStateIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[0]
        .backup_owner_role
        .clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::HistoricalReferenceRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[4].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::DegradedReasonMissing));
}

#[test]
fn historical_reference_invariant_violation_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[0]
        .lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission = true;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::HistoricalReferenceInvariantViolated));

    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[1].reopens_a_live_target_from_a_snapshot_without_validating_identity_trust_route_and_authority = true;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::HistoricalReferenceInvariantViolated));

    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[2].dead_links_an_expired_or_removed_artifact_instead_of_showing_metadata_provenance_or_cleanup_state = true;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::HistoricalReferenceInvariantViolated));

    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[3].leaves_non_live_evidence_unjoined_to_capture_time_provenance_retention_state_or_live_target_mismatch = true;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::HistoricalReferenceInvariantViolated));

    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[4].presents_a_snapshot_or_imported_packet_as_a_current_live_object_or_reopens_through_an_ambiguous_route = true;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::HistoricalReferenceInvariantViolated));
}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::SupportExportEvidence)
        .expect("support-export-evidence row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_horizon_stages_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[1]
        .capture_lifecycle_stages
        .clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::CaptureLifecycleStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[1]
        .consumer_surfaces
        .clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet
        .governance_review
        .non_live_evidence_is_mechanically_distinct_from_live_cached_and_restore_capable_state =
        false;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_historical_reference_source = false;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_historical_reference_matrix().render_markdown_summary();
    for class in M5HistoricalReferenceObject::ALL {
        assert!(
            summary.contains(class.as_str()),
            "summary missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_historical_reference_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5HistoricalReferenceObject::ALL.len());
    assert!(lines[0].starts_with(
        "object_class,qualification,evidence_state,owner,backup_owner,canonical_schema,"
    ));
    for class in M5HistoricalReferenceObject::ALL {
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
        serde_json::from_str(&seeded_m5_historical_reference_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5HistoricalReferenceObject::ALL {
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
        "/../../dashboards/m5-historical-evidence-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked historical-evidence-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_historical_reference_matrix_export()
        .expect("checked M5 historical-reference matrix export validates");
    assert_eq!(packet.packet_id, M5_HISTORICAL_REFERENCE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_historical_reference_matrix_export()
        .expect("checked M5 historical-reference matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_historical_reference_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_historical_reference_matrix_imported_offline_route_evidence_beta_narrowed(),
        seeded_m5_historical_reference_matrix_review_incident_snapshot_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.historical_reference_rows.len(),
            M5HistoricalReferenceObject::ALL.len()
        );
    }

    let beta =
        seeded_m5_historical_reference_matrix_imported_offline_route_evidence_beta_narrowed();
    let row = beta
        .historical_reference_rows
        .iter()
        .find(|r| r.object_class == M5HistoricalReferenceObject::ImportedOfflineRouteEvidence)
        .expect("imported-offline-route-evidence row present");
    assert_eq!(
        row.qualification,
        M5HistoricalReferenceQualificationClass::Beta
    );

    let preview = seeded_m5_historical_reference_matrix_review_incident_snapshot_preview_narrowed();
    let row = preview
        .historical_reference_rows
        .iter()
        .find(|r| r.object_class == M5HistoricalReferenceObject::ReviewIncidentSnapshot)
        .expect("review-incident-snapshot row present");
    assert_eq!(
        row.qualification,
        M5HistoricalReferenceQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5HistoricalReferenceMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/recovery/m5-historical-snapshots/imported_offline_route_evidence_beta_narrowed.json"
    )))
    .expect("imported-offline-route-evidence fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_historical_reference_matrix_imported_offline_route_evidence_beta_narrowed()
    );

    let preview: M5HistoricalReferenceMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/recovery/m5-historical-snapshots/review_incident_snapshot_preview_narrowed.json"
    )))
    .expect("review-incident-snapshot fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_historical_reference_matrix_review_incident_snapshot_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_historical_reference_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.historical_reference_rows[0].scope_summary =
        "raw endpoint https://line.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5HistoricalReferenceMatrixViolation::RawMaterialInExport));
}
