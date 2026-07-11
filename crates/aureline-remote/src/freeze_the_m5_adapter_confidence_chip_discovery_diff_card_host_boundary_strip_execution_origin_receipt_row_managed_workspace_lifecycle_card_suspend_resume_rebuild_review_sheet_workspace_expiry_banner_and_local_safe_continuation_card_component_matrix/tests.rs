use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_build_remote_boundary_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_build_remote_boundary_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5BuildRemoteBoundaryComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5BuildRemoteBoundaryComponentFamily::ALL.len()
    );
}

#[test]
fn frozen_boundary_disposition_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: local/SSH/container/devcontainer/managed/
    // browser-bridge/service-plane execution stays distinguishable and a rebuilt, recreated, or
    // expired workspace never reads as exact continuity.
    let tokens: Vec<&str> = M5BuildRemoteBoundaryDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "local_execution",
            "ssh_execution",
            "container_execution",
            "devcontainer_execution",
            "managed_workspace",
            "browser_bridge",
            "service_plane",
            "suspended",
            "rebuilt",
            "recreated",
            "expired",
            "local_safe_continuation",
            "not_evaluated",
        ]
    );
    assert!(M5BuildRemoteBoundaryDisposition::LocalExecution.is_local_first_party());
    assert!(!M5BuildRemoteBoundaryDisposition::Rebuilt.is_local_first_party());
    assert!(M5BuildRemoteBoundaryDisposition::Rebuilt.breaks_exact_continuity());
    assert!(M5BuildRemoteBoundaryDisposition::Expired.breaks_exact_continuity());
    assert!(!M5BuildRemoteBoundaryDisposition::LocalExecution.breaks_exact_continuity());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_build_remote_boundary_component_matrix();
    for row in &packet.component_rows {
        for label in M5BuildRemoteRequiredLabel::MANDATORY {
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
        assert!(!row.boundary_dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5BuildRemoteAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_build_remote_boundary_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.adapter_confidences.is_empty(),
            family.declares_adapter_confidence(),
            "adapter_confidences presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.discovery_confidences.is_empty(),
            family.declares_discovery_confidence(),
            "discovery_confidences presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.host_kinds.is_empty(),
            family.declares_host_kind(),
            "host_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.origin_loci.is_empty(),
            family.declares_origin_locus(),
            "origin_loci presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.lifecycle_states.is_empty(),
            family.declares_lifecycle_state(),
            "lifecycle_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.persistence_classes.is_empty(),
            family.declares_persistence_class(),
            "persistence_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.continuity_classes.is_empty(),
            family.declares_continuity_class(),
            "continuity_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.expiry_classes.is_empty(),
            family.declares_expiry_class(),
            "expiry_classes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_build_remote_boundary_component_matrix();
    for disposition in M5BuildRemoteBoundaryDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.boundary_dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for confidence in AdapterConfidence::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.adapter_confidences.contains(&confidence)),
            "no component declares adapter confidence {}",
            confidence.as_str()
        );
    }
    for confidence in DiscoveryConfidence::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.discovery_confidences.contains(&confidence)),
            "no component declares discovery confidence {}",
            confidence.as_str()
        );
    }
    for kind in HostKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.host_kinds.contains(&kind)),
            "no component declares host kind {}",
            kind.as_str()
        );
    }
    for locus in OriginLocus::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.origin_loci.contains(&locus)),
            "no component declares origin locus {}",
            locus.as_str()
        );
    }
    for state in BOUND_LIFECYCLE_STATES {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.lifecycle_states.contains(&state)),
            "no component declares lifecycle state {}",
            state.as_str()
        );
    }
    for class in BOUND_PERSISTENCE_CLASSES {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.persistence_classes.contains(&class)),
            "no component declares persistence class {}",
            class.as_str()
        );
    }
    for class in BOUND_CONTINUITY_CLASSES {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.continuity_classes.contains(&class)),
            "no component declares continuity class {}",
            class.as_str()
        );
    }
    for class in BOUND_EXPIRY_CLASSES {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.expiry_classes.contains(&class)),
            "no component declares expiry class {}",
            class.as_str()
        );
    }
    for reason in M5BuildRemoteDegradedReason::ALL {
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
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5BuildRemoteBoundaryComponentFamily::HostBoundaryStrip
    });
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.vocabulary_set.boundary_dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5BuildRemoteRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    let own = M5BuildRemoteBoundaryComponentFamily::AdapterConfidenceChip
        .canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5BuildRemoteBoundaryComponentFamily::AdapterConfidenceChip
        })
        .expect("adapter-confidence-chip present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn boundary_disposition_missing_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[0].boundary_dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::BoundaryDispositionMissing));
}

#[test]
fn confidence_vocab_missing_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5BuildRemoteBoundaryComponentFamily::AdapterConfidenceChip
        })
        .expect("adapter-confidence-chip present")
        .adapter_confidences
        .clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::AdapterConfidenceMissing));

    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5BuildRemoteBoundaryComponentFamily::DiscoveryDiffCard)
        .expect("discovery-diff-card present")
        .discovery_confidences
        .clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::DiscoveryConfidenceMissing));
}

#[test]
fn host_and_origin_vocab_missing_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5BuildRemoteBoundaryComponentFamily::HostBoundaryStrip)
        .expect("host-boundary-strip present")
        .host_kinds
        .clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::HostKindMissing));

    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5BuildRemoteBoundaryComponentFamily::ExecutionOriginReceiptRow
        })
        .expect("execution-origin-receipt-row present")
        .origin_loci
        .clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::OriginLocusMissing));
}

#[test]
fn suspend_sheet_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_build_remote_boundary_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5BuildRemoteBoundaryComponentFamily::SuspendResumeRebuildReviewSheet
            })
            .expect("suspend-resume-rebuild-review-sheet present");
        let expected = match clear {
            0 => {
                row.lifecycle_states.clear();
                M5BuildRemoteBoundaryComponentMatrixViolation::LifecycleStateMissing
            }
            1 => {
                row.persistence_classes.clear();
                M5BuildRemoteBoundaryComponentMatrixViolation::PersistenceClassMissing
            }
            _ => {
                row.continuity_classes.clear();
                M5BuildRemoteBoundaryComponentMatrixViolation::ContinuityClassMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn expiry_vocab_missing_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5BuildRemoteBoundaryComponentFamily::WorkspaceExpiryBanner
        })
        .expect("workspace-expiry-banner present")
        .expiry_classes
        .clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ExpiryClassMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[2].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[5].implies_exact_continuity_after_material_change = true;
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[7].hides_local_safe_or_companion_handoff_in_overflow_only = true;
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[1].lower_confidence_overwrites_resolved_target_without_review = true;
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet
        .governance_review
        .no_card_implies_exact_continuity_after_material_change = false;
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_build_remote_source = false;
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_build_remote_boundary_component_matrix().render_markdown_summary();
    for family in M5BuildRemoteBoundaryComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_build_remote_boundary_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5BuildRemoteBoundaryComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5BuildRemoteBoundaryComponentFamily::ALL {
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
    let packet = current_stable_m5_build_remote_boundary_component_matrix_export()
        .expect("checked M5 build/remote-boundary component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_build_remote_boundary_component_matrix_export()
        .expect("checked M5 build/remote-boundary component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_build_remote_boundary_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_build_remote_boundary_component_matrix_adapter_confidence_chip_beta_narrowed(),
        seeded_m5_build_remote_boundary_component_matrix_suspend_resume_rebuild_review_sheet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5BuildRemoteBoundaryComponentFamily::ALL.len()
        );
    }

    let chip =
        seeded_m5_build_remote_boundary_component_matrix_adapter_confidence_chip_beta_narrowed();
    let row = chip
        .component_rows
        .iter()
        .find(|r| r.component_family == M5BuildRemoteBoundaryComponentFamily::AdapterConfidenceChip)
        .expect("adapter-confidence-chip row present");
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Beta);

    let sheet =
        seeded_m5_build_remote_boundary_component_matrix_suspend_resume_rebuild_review_sheet_preview_narrowed();
    let row = sheet
        .component_rows
        .iter()
        .find(|r| {
            r.component_family
                == M5BuildRemoteBoundaryComponentFamily::SuspendResumeRebuildReviewSheet
        })
        .expect("suspend-resume-rebuild-review-sheet row present");
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let chip: M5BuildRemoteBoundaryComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-build-remote-boundary-components/adapter_confidence_chip_beta_narrowed.json"
    )))
    .expect("adapter-confidence-chip fixture parses");
    assert!(chip.validate().is_empty());
    assert_eq!(
        chip,
        seeded_m5_build_remote_boundary_component_matrix_adapter_confidence_chip_beta_narrowed()
    );

    let sheet: M5BuildRemoteBoundaryComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-build-remote-boundary-components/suspend_resume_rebuild_review_sheet_preview_narrowed.json"
    )))
    .expect("suspend-resume-rebuild-review-sheet fixture parses");
    assert!(sheet.validate().is_empty());
    assert_eq!(
        sheet,
        seeded_m5_build_remote_boundary_component_matrix_suspend_resume_rebuild_review_sheet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_build_remote_boundary_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://workspace.example/runtime leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5BuildRemoteBoundaryComponentMatrixViolation::RawMaterialInExport));
}

#[test]
fn binding_refs_point_at_execution_and_lifecycle_object_models() {
    assert!(M5_BUILD_REMOTE_BOUNDARY_BINDING_REFS.contains(&M5_BUILD_AND_HOST_GOVERNANCE_PATH));
    assert!(M5_BUILD_REMOTE_BOUNDARY_BINDING_REFS.contains(&M5_HOST_BOUNDARY_PATH));
    assert!(M5_BUILD_REMOTE_BOUNDARY_BINDING_REFS.contains(&M5_TARGET_DISCOVERY_PATH));
    assert!(M5_BUILD_REMOTE_BOUNDARY_BINDING_REFS.contains(&MANAGED_WORKSPACE_LIFECYCLE_DOC_REF));
    let packet = seeded_m5_build_remote_boundary_component_matrix();
    for binding in M5_BUILD_REMOTE_BOUNDARY_BINDING_REFS {
        assert!(
            packet.source_contract_refs.iter().any(|r| r == binding),
            "matrix omits binding ref {binding}"
        );
    }
}
