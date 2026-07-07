use super::*;

fn full_input(
    consumer: M5SupportIntakeComponentConsumer,
    family: M5SupportIntakeEscalationComponentFamily,
) -> M5SupportIntakeBindingInput {
    M5SupportIntakeBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5SupportIntakeComponentDescriptor::ALL.to_vec(),
        parity_health: M5SupportIntakeConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_support_intake_binding(&full_input(
        M5SupportIntakeComponentConsumer::DoctorResults,
        M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert_eq!(
        resolved.claim_parity_state,
        M5SupportIntakeClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(
            M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow
        )
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5SupportIntakeBindingInput {
        parity_health: M5SupportIntakeConsumerParityHealth::ScenarioUncertainNarrowed,
        export_caveats: vec![M5SupportIntakeConsumerExportCaveat::ScenarioUncertainLocalOnly],
        ..full_input(
            M5SupportIntakeComponentConsumer::DoctorResults,
            M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary,
        )
    };
    let resolved = resolve_support_intake_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.claim_parity_state,
        M5SupportIntakeClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5SupportIntakeConsumerNarrowingReason::ScenarioClassificationUncertain
    );
    assert_eq!(
        banner.recovery_action,
        M5SupportIntakeConsumerRecoveryAction::ClassifyScenarioBeforeEscalating
    );
    // Descriptors stay preserved even under the narrowing.
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5SupportIntakeComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    // Not a generic "degraded" note.
    assert!(banner
        .headline
        .to_lowercase()
        .contains("scenario classification"));
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5SupportIntakeConsumerParityHealth::ScenarioUncertainNarrowed,
            M5SupportIntakeConsumerNarrowingReason::ScenarioClassificationUncertain,
        ),
        (
            M5SupportIntakeConsumerParityHealth::EvidenceIncompleteNarrowed,
            M5SupportIntakeConsumerNarrowingReason::EvidenceClassesIncomplete,
        ),
        (
            M5SupportIntakeConsumerParityHealth::DestinationUnavailableNarrowed,
            M5SupportIntakeConsumerNarrowingReason::PacketDestinationUnavailable,
        ),
        (
            M5SupportIntakeConsumerParityHealth::RedactionPendingNarrowed,
            M5SupportIntakeConsumerNarrowingReason::RedactionReviewRequired,
        ),
    ] {
        let input = M5SupportIntakeBindingInput {
            parity_health: health,
            ..full_input(
                M5SupportIntakeComponentConsumer::SupportExport,
                M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary,
            )
        };
        let resolved = resolve_support_intake_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5SupportIntakeBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5SupportIntakeComponentConsumer::DoctorResults,
            M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow,
        )
    };
    assert_eq!(
        resolve_support_intake_binding(&empty),
        Err(M5SupportIntakeBindingError::EmptyDescriptorSet)
    );

    let missing = M5SupportIntakeBindingInput {
        descriptor_families: vec![M5SupportIntakeComponentDescriptor::ScenarioCode],
        ..full_input(
            M5SupportIntakeComponentConsumer::DoctorResults,
            M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow,
        )
    };
    assert_eq!(
        resolve_support_intake_binding(&missing),
        Err(M5SupportIntakeBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5SupportIntakeBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5SupportIntakeComponentConsumer::DoctorResults,
            M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow,
        )
    };
    assert_eq!(
        resolve_support_intake_binding(&forbidden),
        Err(M5SupportIntakeBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_escalation_packet_summaries_and_handoff_timeline_rows_with_packet_id_scenario_code_finding_repair_lineage_owner_destination_and_next_step_truth_across_claimed_m5_support_lanes::M5_ESCALATION_HANDOFF_SCHEMA_REF;
    use crate::implement_issue_report_builder_steps_and_evidence_class_selectors_with_included_excluded_redaction_repro_and_local_only_preview_truth_across_claimed_m5_support_flows::M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF;
    use crate::implement_support_scenario_picker_rows_and_seeded_symptom_scope_cues_with_start_diagnosis_parity_across_claimed_m5_support_intake_surfaces::M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF;
    use crate::implement_unsafe_fix_blocked_notes_and_approved_repair_guidance_with_blocked_action_block_reason_safer_repair_blast_radius_and_rollback_evidence_preservation_truth_across_claimed_m5_doctor_and_support_surfaces::M5_UNSAFE_REPAIR_SCHEMA_REF;
    use M5SupportIntakeEscalationComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::SupportScenarioPickerRow),
        M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::IssueReportBuilderStep),
        M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::EscalationPacketSummary),
        M5_ESCALATION_HANDOFF_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::HandoffTimelineRow),
        M5_ESCALATION_HANDOFF_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::UnsafeFixBlockedNote),
        M5_UNSAFE_REPAIR_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5SupportIntakeComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5SupportIntakeComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    for family in M5SupportIntakeEscalationComponentFamily::ALL {
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
    let packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5SupportIntakeConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5SupportIntakeConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5SupportIntakeComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5SupportAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            assert_eq!(
                b.canonical_schema_ref,
                family_canonical_schema_ref(b.component_family)
            );
            assert_eq!(
                b.canonical_artifact_ref,
                family_canonical_artifact_ref(b.component_family)
            );
            assert!(b.references_canonical_not_local_prose);
        }
    }
}

#[test]
fn every_parity_health_mode_reason_and_parity_state_is_exercised() {
    let packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    let cases: Vec<&M5SupportIntakeBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5SupportIntakeConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5SupportIntakeConsumerNarrowingReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked binding exercises narrowing reason {}",
            reason.as_str()
        );
    }
    for state in M5SupportIntakeClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_support_intake_escalation_component_consumer_packet();
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
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5SupportIntakeComponentConsumer::Bisect);
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5SupportIntakeComponentDescriptor::RedactionClass);
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5SupportIntakeConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    // Strip every SupportScenarioPickerRow binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family
                == M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow
            {
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
        .contains(&M5SupportIntakeComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5SupportIntakeBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.consumer_rows[0].inherits_stronger_label_from_healthier_profile = true;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5SupportIntakeComponentConsumer::SupportExport)
        .expect("support / export row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(violations
        .contains(&M5SupportIntakeComponentConsumerViolation::SupportExportReferenceMissing));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.governance_review.degraded_state_auto_narrows_claim = false;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet
        .consumer_projection
        .redaction_class_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SupportIntakeComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary =
        seeded_m5_support_intake_escalation_component_consumer_packet().render_markdown_summary();
    for consumer in M5SupportIntakeComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_support_intake_escalation_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5SupportIntakeComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5SupportIntakeComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_support_intake_escalation_component_consumer_export()
        .expect("checked M5 support-intake escalation component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_support_intake_escalation_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_support_intake_escalation_component_consumer_bisect_preview_narrowed(),
        seeded_m5_support_intake_escalation_component_consumer_docs_help_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5SupportIntakeComponentConsumer::ALL.len()
        );
    }

    let bisect = seeded_m5_support_intake_escalation_component_consumer_bisect_preview_narrowed();
    let row = bisect
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5SupportIntakeComponentConsumer::Bisect)
        .expect("bisect row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Preview);

    let docs = seeded_m5_support_intake_escalation_component_consumer_docs_help_beta_narrowed();
    let row = docs
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5SupportIntakeComponentConsumer::DocsHelp)
        .expect("docs / help row present");
    assert_eq!(row.qualification, M5SupportQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let bisect: M5SupportIntakeComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-intake-escalation-component-consumers/bisect_preview_narrowed.json"
    )))
    .expect("bisect fixture parses");
    assert!(bisect.validate().is_empty());
    assert_eq!(
        bisect,
        seeded_m5_support_intake_escalation_component_consumer_bisect_preview_narrowed()
    );

    let docs: M5SupportIntakeComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-intake-escalation-component-consumers/docs_help_beta_narrowed.json"
    )))
    .expect("docs / help fixture parses");
    assert!(docs.validate().is_empty());
    assert_eq!(
        docs,
        seeded_m5_support_intake_escalation_component_consumer_docs_help_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_support_intake_escalation_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
