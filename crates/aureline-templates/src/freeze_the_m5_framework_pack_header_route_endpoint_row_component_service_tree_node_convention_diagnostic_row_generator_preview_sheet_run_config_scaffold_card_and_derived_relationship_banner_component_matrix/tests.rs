use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_framework_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_FRAMEWORK_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_framework_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5FrameworkComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5FrameworkComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_dispositions_and_deployment_lines() {
    let packet = seeded_m5_framework_component_matrix();
    for row in &packet.component_rows {
        for label in M5FrameworkRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.dispositions.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_framework_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.pack_support_classes.is_empty(),
            family.is_framework_pack_header(),
            "pack_support_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.pack_identity_states.is_empty(),
            family.is_framework_pack_header(),
            "pack_identity_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.route_evidence_classes.is_empty(),
            family.is_route_endpoint_row(),
            "route_evidence_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.route_authorship_states.is_empty(),
            family.is_route_endpoint_row(),
            "route_authorship_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.topology_node_kinds.is_empty(),
            family.is_component_service_tree_node(),
            "topology_node_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.topology_evidence_classes.is_empty(),
            family.is_component_service_tree_node(),
            "topology_evidence_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.convention_confidence_classes.is_empty(),
            family.is_convention_diagnostic_row(),
            "convention_confidence_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.diagnostic_severities.is_empty(),
            family.is_convention_diagnostic_row(),
            "diagnostic_severities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.generator_impact_classes.is_empty(),
            family.is_generator_preview_sheet(),
            "generator_impact_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.generator_apply_postures.is_empty(),
            family.is_generator_preview_sheet(),
            "generator_apply_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.execution_boundary_classes.is_empty(),
            family.is_run_config_scaffold_card(),
            "execution_boundary_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.run_config_mutation_classes.is_empty(),
            family.is_run_config_scaffold_card(),
            "run_config_mutation_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.derived_relationship_classes.is_empty(),
            family.is_derived_relationship_banner(),
            "derived_relationship_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.relationship_proving_states.is_empty(),
            family.is_derived_relationship_banner(),
            "relationship_proving_states presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_framework_component_matrix();
    for disposition in M5FrameworkCertaintyDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for class in M5FrameworkPackSupportClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.pack_support_classes.contains(&class)),
            "no component declares pack support class {}",
            class.as_str()
        );
    }
    for state in M5FrameworkPackIdentityState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.pack_identity_states.contains(&state)),
            "no component declares pack identity state {}",
            state.as_str()
        );
    }
    for class in M5RouteEvidenceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.route_evidence_classes.contains(&class)),
            "no component declares route evidence class {}",
            class.as_str()
        );
    }
    for authorship in M5RouteAuthorship::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.route_authorship_states.contains(&authorship)),
            "no component declares route authorship {}",
            authorship.as_str()
        );
    }
    for kind in M5TopologyNodeKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.topology_node_kinds.contains(&kind)),
            "no component declares topology node kind {}",
            kind.as_str()
        );
    }
    for class in M5TopologyEvidenceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.topology_evidence_classes.contains(&class)),
            "no component declares topology evidence class {}",
            class.as_str()
        );
    }
    for class in M5ConventionConfidenceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.convention_confidence_classes.contains(&class)),
            "no component declares convention confidence class {}",
            class.as_str()
        );
    }
    for severity in M5ConventionDiagnosticSeverity::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.diagnostic_severities.contains(&severity)),
            "no component declares diagnostic severity {}",
            severity.as_str()
        );
    }
    for class in M5GeneratorImpactClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.generator_impact_classes.contains(&class)),
            "no component declares generator impact class {}",
            class.as_str()
        );
    }
    for posture in M5GeneratorApplyPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.generator_apply_postures.contains(&posture)),
            "no component declares generator apply posture {}",
            posture.as_str()
        );
    }
    for class in M5ExecutionBoundaryClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.execution_boundary_classes.contains(&class)),
            "no component declares execution boundary class {}",
            class.as_str()
        );
    }
    for class in M5RunConfigMutationClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.run_config_mutation_classes.contains(&class)),
            "no component declares run-config mutation class {}",
            class.as_str()
        );
    }
    for class in M5DerivedRelationshipClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.derived_relationship_classes.contains(&class)),
            "no component declares derived relationship class {}",
            class.as_str()
        );
    }
    for state in M5RelationshipProvingState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.relationship_proving_states.contains(&state)),
            "no component declares relationship proving state {}",
            state.as_str()
        );
    }
}

#[test]
fn ac_disposition_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin one controlled vocabulary; assert the exact tokens.
    let tokens: Vec<&str> = M5FrameworkCertaintyDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "core_native",
            "framework_pack",
            "bridge",
            "heuristic_convention",
            "verified",
            "derived_by_convention",
            "runtime_confirmed",
            "partial",
        ]
    );
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5FrameworkComponentFamily::GeneratorPreviewSheet);
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5FrameworkRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn dispositions_missing_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::DispositionsMissing));
}

#[test]
fn framework_pack_header_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_framework_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5FrameworkComponentFamily::FrameworkPackHeader)
            .expect("framework-pack-header row present");
        let expected = if clear == 0 {
            row.pack_support_classes.clear();
            M5FrameworkComponentMatrixViolation::PackSupportClassMissing
        } else {
            row.pack_identity_states.clear();
            M5FrameworkComponentMatrixViolation::PackIdentityStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn route_endpoint_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_framework_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5FrameworkComponentFamily::RouteEndpointRow)
            .expect("route-endpoint-row row present");
        let expected = if clear == 0 {
            row.route_evidence_classes.clear();
            M5FrameworkComponentMatrixViolation::RouteEvidenceClassMissing
        } else {
            row.route_authorship_states.clear();
            M5FrameworkComponentMatrixViolation::RouteAuthorshipMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_service_tree_node_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_framework_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5FrameworkComponentFamily::ComponentServiceTreeNode
            })
            .expect("component-service-tree-node row present");
        let expected = if clear == 0 {
            row.topology_node_kinds.clear();
            M5FrameworkComponentMatrixViolation::TopologyNodeKindMissing
        } else {
            row.topology_evidence_classes.clear();
            M5FrameworkComponentMatrixViolation::TopologyEvidenceClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn convention_diagnostic_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_framework_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5FrameworkComponentFamily::ConventionDiagnosticRow)
            .expect("convention-diagnostic-row row present");
        let expected = if clear == 0 {
            row.convention_confidence_classes.clear();
            M5FrameworkComponentMatrixViolation::ConventionConfidenceClassMissing
        } else {
            row.diagnostic_severities.clear();
            M5FrameworkComponentMatrixViolation::DiagnosticSeverityMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn generator_preview_sheet_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_framework_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5FrameworkComponentFamily::GeneratorPreviewSheet)
            .expect("generator-preview-sheet row present");
        let expected = if clear == 0 {
            row.generator_impact_classes.clear();
            M5FrameworkComponentMatrixViolation::GeneratorImpactClassMissing
        } else {
            row.generator_apply_postures.clear();
            M5FrameworkComponentMatrixViolation::GeneratorApplyPostureMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn run_config_scaffold_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_framework_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5FrameworkComponentFamily::RunConfigScaffoldCard)
            .expect("run-config-scaffold-card row present");
        let expected = if clear == 0 {
            row.execution_boundary_classes.clear();
            M5FrameworkComponentMatrixViolation::ExecutionBoundaryClassMissing
        } else {
            row.run_config_mutation_classes.clear();
            M5FrameworkComponentMatrixViolation::RunConfigMutationClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn derived_relationship_banner_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_framework_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5FrameworkComponentFamily::DerivedRelationshipBanner
            })
            .expect("derived-relationship-banner row present");
        let expected = if clear == 0 {
            row.derived_relationship_classes.clear();
            M5FrameworkComponentMatrixViolation::DerivedRelationshipClassMissing
        } else {
            row.relationship_proving_states.clear();
            M5FrameworkComponentMatrixViolation::RelationshipProvingStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[0].hides_pack_identity_version_or_support_class = true;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[1].lets_heuristic_masquerade_as_exact = true;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[4].implies_no_op_write_while_mutating_config_or_dependencies = true;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[5].hides_local_container_ssh_or_managed_boundary = true;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[6].omits_proving_source_or_rollback_path = true;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[3].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5FrameworkComponentFamily::FrameworkPackHeader)
        .expect("framework-pack-header row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.governance_review.no_heuristic_masquerades_as_exact = false;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_framework_component_matrix().render_markdown_summary();
    for family in M5FrameworkComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_framework_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5FrameworkComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,dispositions,"));
    for family in M5FrameworkComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_framework_component_matrix_export()
        .expect("checked M5 framework component matrix export validates");
    assert_eq!(packet.packet_id, M5_FRAMEWORK_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_framework_component_matrix_export()
        .expect("checked M5 framework component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_framework_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_framework_component_matrix_route_endpoint_row_beta_narrowed(),
        seeded_m5_framework_component_matrix_generator_preview_sheet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5FrameworkComponentFamily::ALL.len()
        );
    }

    let route = seeded_m5_framework_component_matrix_route_endpoint_row_beta_narrowed();
    let row = route
        .component_rows
        .iter()
        .find(|r| r.component_family == M5FrameworkComponentFamily::RouteEndpointRow)
        .expect("route-endpoint-row row present");
    assert_eq!(row.qualification, M5FrameworkQualificationClass::Beta);

    let generator = seeded_m5_framework_component_matrix_generator_preview_sheet_preview_narrowed();
    let row = generator
        .component_rows
        .iter()
        .find(|r| r.component_family == M5FrameworkComponentFamily::GeneratorPreviewSheet)
        .expect("generator-preview-sheet row present");
    assert_eq!(row.qualification, M5FrameworkQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let route: M5FrameworkComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-framework-components/route_endpoint_row_beta_narrowed.json"
    )))
    .expect("route-endpoint-row fixture parses");
    assert!(route.validate().is_empty());
    assert_eq!(
        route,
        seeded_m5_framework_component_matrix_route_endpoint_row_beta_narrowed()
    );

    let generator: M5FrameworkComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-framework-components/generator_preview_sheet_preview_narrowed.json"
    )))
    .expect("generator-preview-sheet fixture parses");
    assert!(generator.validate().is_empty());
    assert_eq!(
        generator,
        seeded_m5_framework_component_matrix_generator_preview_sheet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_framework_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
