use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_governance_dashboard_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_GOVERNANCE_DASHBOARD_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_governance_dashboard_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5GovernanceDashboardComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5GovernanceDashboardComponentFamily::ALL.len()
    );
}

/// The one acceptance-criteria vocabulary: the governance readiness states are
/// frozen for shared reuse. This test pins the exact token list so no later row can
/// silently add, drop, or rename a status word.
#[test]
fn frozen_governance_readiness_state_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5GovernanceReadinessState::ALL
        .iter()
        .map(|state| state.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "passing",
            "warning",
            "blocked",
            "waived",
            "expired_waiver",
            "evidence_stale",
            "owner_unresolved",
            "forum_unresolved",
            "not_evaluated",
        ]
    );
    // Only `passing` is a clean pass; every other state must never read as one.
    for state in M5GovernanceReadinessState::ALL {
        assert_eq!(
            state.is_clean_pass(),
            state == M5GovernanceReadinessState::Passing,
            "is_clean_pass wrong for {}",
            state.as_str()
        );
    }
}

#[test]
fn every_component_declares_readiness_states_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_governance_dashboard_component_matrix();
    for row in &packet.component_rows {
        for label in M5GovernanceRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            !row.readiness_states.is_empty(),
            "component {} declares no readiness states",
            row.component_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_governance_dashboard_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.fitness_provenance_classes.is_empty(),
            family.is_fitness_tile(),
            "fitness_provenance_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.report_scopes.is_empty(),
            family.is_report_row(),
            "report_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.waiver_expiry_states.is_empty(),
            family.is_waiver_item(),
            "waiver_expiry_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.release_gate_decisions.is_empty(),
            family.is_release_gate(),
            "release_gate_decisions presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.mitigation_postures.is_empty(),
            family.is_mitigation_card(),
            "mitigation_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.ownership_coverage_states.is_empty(),
            family.is_ownership_card(),
            "ownership_coverage_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.on_call_coverage_states.is_empty(),
            family.is_on_call(),
            "on_call_coverage_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.escalation_route_classes.is_empty(),
            family.is_on_call(),
            "escalation_route_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.decision_forum_classes.is_empty(),
            family.is_decision_right(),
            "decision_forum_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.decision_right_states.is_empty(),
            family.is_decision_right(),
            "decision_right_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.milestone_gate_states.is_empty(),
            family.is_milestone_row(),
            "milestone_gate_states presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_governance_dashboard_component_matrix();
    for state in M5GovernanceReadinessState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.readiness_states.contains(&state)),
            "no component declares readiness state {}",
            state.as_str()
        );
    }
    for class in M5FitnessProvenanceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.fitness_provenance_classes.contains(&class)),
            "no component declares fitness provenance class {}",
            class.as_str()
        );
    }
    for scope in M5GovernanceReportScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.report_scopes.contains(&scope)),
            "no component declares report scope {}",
            scope.as_str()
        );
    }
    for state in M5WaiverExpiryState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.waiver_expiry_states.contains(&state)),
            "no component declares waiver expiry state {}",
            state.as_str()
        );
    }
    for decision in M5ReleaseGateDecision::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.release_gate_decisions.contains(&decision)),
            "no component declares release-gate decision {}",
            decision.as_str()
        );
    }
    for posture in M5MitigationPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.mitigation_postures.contains(&posture)),
            "no component declares mitigation posture {}",
            posture.as_str()
        );
    }
    for state in M5OwnershipCoverageState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.ownership_coverage_states.contains(&state)),
            "no component declares ownership coverage state {}",
            state.as_str()
        );
    }
    for state in M5OnCallCoverageState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.on_call_coverage_states.contains(&state)),
            "no component declares on-call coverage state {}",
            state.as_str()
        );
    }
    for class in M5EscalationRouteClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.escalation_route_classes.contains(&class)),
            "no component declares escalation route class {}",
            class.as_str()
        );
    }
    for class in M5DecisionForumClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.decision_forum_classes.contains(&class)),
            "no component declares decision forum class {}",
            class.as_str()
        );
    }
    for state in M5DecisionRightState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.decision_right_states.contains(&state)),
            "no component declares decision-right state {}",
            state.as_str()
        );
    }
    for state in M5MilestoneGateState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.milestone_gate_states.contains(&state)),
            "no component declares milestone gate state {}",
            state.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5GovernanceDashboardComponentFamily::OnCallStrip);
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.vocabulary_set.readiness_states.pop();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5GovernanceRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn readiness_state_missing_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.component_rows[0].readiness_states.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ReadinessStateMissing));
}

#[test]
fn fitness_vocab_missing_fails_for_fitness_tile() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5GovernanceDashboardComponentFamily::FitnessDashboardTile
        })
        .expect("fitness tile present");
    row.fitness_provenance_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::FitnessProvenanceMissing));
}

#[test]
fn waiver_vocab_missing_fails_for_waiver_item() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5GovernanceDashboardComponentFamily::WaiverExpiryQueueItem
        })
        .expect("waiver item present");
    row.waiver_expiry_states.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::WaiverExpiryStateMissing));
}

#[test]
fn release_gate_vocab_missing_fails_for_release_gate_banner() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5GovernanceDashboardComponentFamily::ReleaseGateBanner)
        .expect("release gate banner present");
    row.release_gate_decisions.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ReleaseGateDecisionMissing));
}

#[test]
fn on_call_vocab_missing_fails_for_on_call_strip() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5GovernanceDashboardComponentFamily::OnCallStrip)
        .expect("on-call strip present");
    row.on_call_coverage_states.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::OnCallCoverageStateMissing));

    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5GovernanceDashboardComponentFamily::OnCallStrip)
        .expect("on-call strip present");
    row.escalation_route_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::EscalationRouteClassMissing));
}

#[test]
fn decision_right_vocab_missing_fails_for_decision_right_card() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5GovernanceDashboardComponentFamily::DecisionRightCard)
        .expect("decision-right card present");
    row.decision_forum_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::DecisionForumClassMissing));

    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5GovernanceDashboardComponentFamily::DecisionRightCard)
        .expect("decision-right card present");
    row.decision_right_states.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::DecisionRightStateMissing));
}

#[test]
fn milestone_vocab_missing_fails_for_milestone_row() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5GovernanceDashboardComponentFamily::MilestoneDashboardRow
        })
        .expect("milestone row present");
    row.milestone_gate_states.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::MilestoneGateStateMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.component_rows[0].renders_waived_or_stale_as_clean_pass = true;
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.component_rows[5].lets_ownerless_or_forumless_blocker_read_resolved = true;
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.component_rows[4].hides_mitigation_behind_internal_jargon = true;
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.component_rows[1].invents_private_governance_status_grammar = true;
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5GovernanceDashboardComponentFamily::FitnessDashboardTile
        })
        .expect("fitness tile present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.governance_review.waived_or_stale_never_clean_pass = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet
        .consumer_projection
        .ownership_and_on_call_surfaces_consume_coverage_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceDashboardMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_governance_dashboard_component_matrix().render_markdown_summary();
    for family in M5GovernanceDashboardComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_governance_dashboard_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5GovernanceDashboardComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,readiness_states,"));
    for family in M5GovernanceDashboardComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_governance_dashboard_component_matrix_export()
        .expect("checked M5 governance dashboard matrix export validates");
    assert_eq!(packet.packet_id, M5_GOVERNANCE_DASHBOARD_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_governance_dashboard_component_matrix_export()
        .expect("checked M5 governance dashboard matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_governance_dashboard_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_governance_dashboard_component_matrix_service_ownership_card_beta_narrowed(),
        seeded_m5_governance_dashboard_component_matrix_release_gate_banner_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5GovernanceDashboardComponentFamily::ALL.len()
        );
    }

    let ownership =
        seeded_m5_governance_dashboard_component_matrix_service_ownership_card_beta_narrowed();
    let row = ownership
        .component_rows
        .iter()
        .find(|r| r.component_family == M5GovernanceDashboardComponentFamily::ServiceOwnershipCard)
        .expect("service-ownership-card row present");
    assert_eq!(row.qualification, M5GovernanceQualificationClass::Beta);

    let release_gate =
        seeded_m5_governance_dashboard_component_matrix_release_gate_banner_preview_narrowed();
    let row = release_gate
        .component_rows
        .iter()
        .find(|r| r.component_family == M5GovernanceDashboardComponentFamily::ReleaseGateBanner)
        .expect("release-gate-banner row present");
    assert_eq!(row.qualification, M5GovernanceQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let ownership: M5GovernanceDashboardMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-governance-dashboard-components/service_ownership_card_beta_narrowed.json"
    )))
    .expect("service-ownership fixture parses");
    assert!(ownership.validate().is_empty());
    assert_eq!(
        ownership,
        seeded_m5_governance_dashboard_component_matrix_service_ownership_card_beta_narrowed()
    );

    let release_gate: M5GovernanceDashboardMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-governance-dashboard-components/release_gate_banner_preview_narrowed.json"
    )))
    .expect("release-gate fixture parses");
    assert!(release_gate.validate().is_empty());
    assert_eq!(
        release_gate,
        seeded_m5_governance_dashboard_component_matrix_release_gate_banner_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_governance_dashboard_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_GOVERNANCE_DASHBOARD_COMPONENT_MATRIX_ARTIFACTS` so ordinary
/// test runs never touch the working tree. Run in isolation with the env gate set,
/// then run the full suite.
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_GOVERNANCE_DASHBOARD_COMPONENT_MATRIX_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = seeded_m5_governance_dashboard_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-governance-dashboard-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(proof_dir.join("matrix.csv"), packet.render_matrix_csv())
        .expect("write matrix csv");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-governance-dashboard-components");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let ownership =
        seeded_m5_governance_dashboard_component_matrix_service_ownership_card_beta_narrowed();
    assert!(
        ownership.validate().is_empty(),
        "{:?}",
        ownership.validate()
    );
    std::fs::write(
        fixture_dir.join("service_ownership_card_beta_narrowed.json"),
        format!("{}\n", ownership.export_safe_json()),
    )
    .expect("write service-ownership fixture");

    let release_gate =
        seeded_m5_governance_dashboard_component_matrix_release_gate_banner_preview_narrowed();
    assert!(
        release_gate.validate().is_empty(),
        "{:?}",
        release_gate.validate()
    );
    std::fs::write(
        fixture_dir.join("release_gate_banner_preview_narrowed.json"),
        format!("{}\n", release_gate.export_safe_json()),
    )
    .expect("write release-gate fixture");
}
