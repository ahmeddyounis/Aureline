use super::*;

fn full_input(
    consumer: M5GovernanceDashboardConsumer,
    family: M5GovernanceDashboardComponentFamily,
) -> M5GovernanceBindingInput {
    M5GovernanceBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5GovernanceDescriptor::ALL.to_vec(),
        evidence_state: M5GovernanceEvidenceState::FullTruthFresh,
        readiness_vocab: M5GovernanceReadinessState::ALL.to_vec(),
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_truth_preserves_descriptors_with_no_banner() {
    let resolved = resolve_governance_consumer_binding(&full_input(
        M5GovernanceDashboardConsumer::AssuranceCenter,
        M5GovernanceDashboardComponentFamily::FitnessDashboardTile,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.narrow_banner.is_none());
    assert_eq!(
        resolved.descriptor_parity_state,
        M5GovernanceDescriptorParityState::DescriptorsPreserved
    );
    assert_eq!(
        resolved.projection_mode,
        M5GovernanceProjectionMode::FullParity
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        component_canonical_schema_ref(M5GovernanceDashboardComponentFamily::FitnessDashboardTile)
    );
}

#[test]
fn resolver_narrowed_evidence_discloses_self_contained_banner() {
    let input = M5GovernanceBindingInput {
        evidence_state: M5GovernanceEvidenceState::EvidenceStale,
        ..full_input(
            M5GovernanceDashboardConsumer::DocsPortal,
            M5GovernanceDashboardComponentFamily::GovernanceReportRow,
        )
    };
    let resolved = resolve_governance_consumer_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.descriptor_parity_state,
        M5GovernanceDescriptorParityState::DescriptorsDisclosedNarrowed
    );
    let banner = resolved.narrow_banner.expect("banner present");
    assert_eq!(banner.reason, M5GovernanceNarrowReason::EvidenceStale);
    assert_eq!(banner.next_action, M5GovernanceNextAction::RefreshEvidence);
    assert_eq!(
        banner.readiness_floor,
        M5GovernanceReadinessState::EvidenceStale
    );
    // Descriptors stay preserved even under the narrowing.
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5GovernanceDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    // Not a generic "degraded" note.
    assert!(banner.headline.to_lowercase().contains("stale"));
    // Never reads a clean pass — the floor is an explicit non-passing state.
    assert!(!banner.readiness_floor.is_clean_pass());
}

#[test]
fn resolver_each_narrowed_state_maps_to_its_reason_and_mode() {
    for (state, reason, mode) in [
        (
            M5GovernanceEvidenceState::EvidenceStale,
            M5GovernanceNarrowReason::EvidenceStale,
            M5GovernanceProjectionMode::StaleNarrowed,
        ),
        (
            M5GovernanceEvidenceState::WaiverExpiringOrExpired,
            M5GovernanceNarrowReason::WaiverExpiring,
            M5GovernanceProjectionMode::WaiverNarrowed,
        ),
        (
            M5GovernanceEvidenceState::OwnerCoverageMissing,
            M5GovernanceNarrowReason::OwnerCoverageMissing,
            M5GovernanceProjectionMode::OwnershipNarrowed,
        ),
        (
            M5GovernanceEvidenceState::ForumUnresolved,
            M5GovernanceNarrowReason::ForumUnresolved,
            M5GovernanceProjectionMode::ForumNarrowed,
        ),
        (
            M5GovernanceEvidenceState::NotEvaluatedHere,
            M5GovernanceNarrowReason::NotEvaluatedHere,
            M5GovernanceProjectionMode::NotEvaluatedNarrowed,
        ),
    ] {
        let input = M5GovernanceBindingInput {
            evidence_state: state,
            ..full_input(
                M5GovernanceDashboardConsumer::OperatorDashboard,
                M5GovernanceDashboardComponentFamily::ServiceOwnershipCard,
            )
        };
        let resolved = resolve_governance_consumer_binding(&input).expect("resolves");
        assert_eq!(resolved.projection_mode, mode);
        assert_eq!(resolved.narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5GovernanceBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5GovernanceDashboardConsumer::AssuranceCenter,
            M5GovernanceDashboardComponentFamily::FitnessDashboardTile,
        )
    };
    assert_eq!(
        resolve_governance_consumer_binding(&empty),
        Err(M5GovernanceBindingError::EmptyDescriptorSet)
    );

    let missing = M5GovernanceBindingInput {
        descriptor_families: vec![M5GovernanceDescriptor::Readiness],
        ..full_input(
            M5GovernanceDashboardConsumer::AssuranceCenter,
            M5GovernanceDashboardComponentFamily::FitnessDashboardTile,
        )
    };
    assert_eq!(
        resolve_governance_consumer_binding(&missing),
        Err(M5GovernanceBindingError::MissingRequiredDescriptor)
    );

    let no_vocab = M5GovernanceBindingInput {
        readiness_vocab: vec![],
        ..full_input(
            M5GovernanceDashboardConsumer::AssuranceCenter,
            M5GovernanceDashboardComponentFamily::FitnessDashboardTile,
        )
    };
    assert_eq!(
        resolve_governance_consumer_binding(&no_vocab),
        Err(M5GovernanceBindingError::EmptyReadinessVocab)
    );

    let forbidden = M5GovernanceBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5GovernanceDashboardConsumer::AssuranceCenter,
            M5GovernanceDashboardComponentFamily::FitnessDashboardTile,
        )
    };
    assert_eq!(
        resolve_governance_consumer_binding(&forbidden),
        Err(M5GovernanceBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_map_nine_families_to_four_controls() {
    use M5GovernanceDashboardComponentFamily as Family;
    assert_eq!(
        component_canonical_schema_ref(Family::FitnessDashboardTile),
        M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        component_canonical_schema_ref(Family::GovernanceReportRow),
        M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        component_canonical_schema_ref(Family::WaiverExpiryQueueItem),
        M5_WAIVER_GATE_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        component_canonical_schema_ref(Family::ReleaseGateBanner),
        M5_WAIVER_GATE_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        component_canonical_schema_ref(Family::MitigationNoteCard),
        M5_WAIVER_GATE_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        component_canonical_schema_ref(Family::ServiceOwnershipCard),
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        component_canonical_schema_ref(Family::OnCallStrip),
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        component_canonical_schema_ref(Family::DecisionRightCard),
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        component_canonical_schema_ref(Family::MilestoneDashboardRow),
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_governance_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_GOVERNANCE_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_governance_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5GovernanceDashboardConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5GovernanceDashboardConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_governance_component_consumer_packet();
    for family in M5GovernanceDashboardComponentFamily::ALL {
        let count = packet
            .consumer_rows
            .iter()
            .filter(|row| {
                row.component_bindings
                    .iter()
                    .any(|b| b.component_family == family)
            })
            .count();
        assert!(
            count >= 2,
            "family {} adopted by only {} consumer(s)",
            family.as_str(),
            count
        );
    }
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_descriptors() {
    let packet = seeded_m5_governance_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5GovernanceConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5GovernanceConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5GovernanceDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(!row.readiness_vocab.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5GovernanceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_governance_component_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            assert_eq!(
                b.canonical_schema_ref,
                component_canonical_schema_ref(b.component_family)
            );
            assert_eq!(
                b.canonical_artifact_ref,
                component_canonical_artifact_ref(b.component_family)
            );
            assert!(b.references_canonical_not_local_prose);
        }
    }
}

#[test]
fn every_evidence_state_reason_and_parity_is_exercised() {
    let packet = seeded_m5_governance_component_consumer_packet();
    let cases: Vec<&M5GovernanceBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for state in M5GovernanceEvidenceState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.evidence_state == state),
            "no worked binding exercises evidence state {}",
            state.as_str()
        );
    }
    for reason in M5GovernanceNarrowReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .narrow_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked binding exercises narrow reason {}",
            reason.as_str()
        );
    }
    for state in M5GovernanceDescriptorParityState::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.descriptor_parity_state == state),
            "no worked binding exercises descriptor-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_governance_component_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                assert!(
                    case.is_self_consistent(),
                    "worked binding for {} drifted from resolver output",
                    row.consumer.as_str()
                );
            }
        }
    }
}

#[test]
fn missing_consumer_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5GovernanceDashboardConsumer::SupportExport);
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.vocabulary_set.evidence_states.pop();
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5GovernanceDescriptor::Readiness);
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn readiness_vocab_missing_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[0].readiness_vocab.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::ReadinessVocabMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5GovernanceConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    // Strip every FitnessDashboardTile binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5GovernanceDashboardComponentFamily::FitnessDashboardTile {
                if seen_first {
                    return false;
                }
                seen_first = true;
            }
            true
        });
    }
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5GovernanceBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[0].renders_waived_or_stale_as_clean_pass = true;
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.governance_review.waived_or_stale_never_reads_clean = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet
        .consumer_projection
        .evidence_freshness_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_governance_component_consumer_packet().render_markdown_summary();
    for consumer in M5GovernanceDashboardConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_governance_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5GovernanceDashboardConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5GovernanceDashboardConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_governance_component_consumer_export()
        .expect("checked M5 governance-dashboard-component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_GOVERNANCE_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_governance_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_governance_component_consumer_operator_ownership_narrowed(),
        seeded_m5_governance_component_consumer_docs_stale_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5GovernanceDashboardConsumer::ALL.len()
        );
    }

    let operator = seeded_m5_governance_component_consumer_operator_ownership_narrowed();
    let row = operator
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5GovernanceDashboardConsumer::OperatorDashboard)
        .expect("operator-dashboard row present");
    assert_eq!(row.qualification, M5GovernanceQualificationClass::Beta);

    let docs = seeded_m5_governance_component_consumer_docs_stale_narrowed();
    let row = docs
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5GovernanceDashboardConsumer::DocsPortal)
        .expect("docs-portal row present");
    assert_eq!(row.qualification, M5GovernanceQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let operator: M5GovernanceComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-governance-dashboard-component-consumers/operator_ownership_narrowed.json"
    )))
    .expect("operator fixture parses");
    assert!(operator.validate().is_empty());
    assert_eq!(
        operator,
        seeded_m5_governance_component_consumer_operator_ownership_narrowed()
    );

    let docs: M5GovernanceComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-governance-dashboard-component-consumers/docs_stale_narrowed.json"
    )))
    .expect("docs fixture parses");
    assert!(docs.validate().is_empty());
    assert_eq!(
        docs,
        seeded_m5_governance_component_consumer_docs_stale_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_governance_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_GOVERNANCE_DASHBOARD_COMPONENT_CONSUMER_ARTIFACTS` so ordinary
/// test runs never touch the working tree. Run in isolation with the env gate set,
/// then run the full suite.
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_GOVERNANCE_DASHBOARD_COMPONENT_CONSUMER_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = seeded_m5_governance_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-governance-dashboard-component-consumer-proof");
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
        .join("m5-governance-dashboard-component-consumers");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let operator = seeded_m5_governance_component_consumer_operator_ownership_narrowed();
    assert!(operator.validate().is_empty(), "{:?}", operator.validate());
    std::fs::write(
        fixture_dir.join("operator_ownership_narrowed.json"),
        format!("{}\n", operator.export_safe_json()),
    )
    .expect("write operator fixture");

    let docs = seeded_m5_governance_component_consumer_docs_stale_narrowed();
    assert!(docs.validate().is_empty(), "{:?}", docs.validate());
    std::fs::write(
        fixture_dir.join("docs_stale_narrowed.json"),
        format!("{}\n", docs.export_safe_json()),
    )
    .expect("write docs fixture");
}
