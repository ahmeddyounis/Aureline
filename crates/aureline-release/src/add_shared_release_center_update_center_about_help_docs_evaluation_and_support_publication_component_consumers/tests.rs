use super::*;

fn full_input(
    consumer: M5PublicationComponentConsumer,
    family: M5PublicationComponentFamily,
) -> M5PublicationBindingInput {
    M5PublicationBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5PublicationDescriptor::ALL.to_vec(),
        client_scope_mode: M5ClientScopeMode::FullClientScope,
        handoff_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_scope_preserves_descriptors_with_no_banner() {
    let resolved = resolve_publication_binding(&full_input(
        M5PublicationComponentConsumer::ReleaseCenter,
        M5PublicationComponentFamily::ReleaseCandidateCard,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.reduced_scope_banner.is_none());
    assert_eq!(
        resolved.descriptor_parity_state,
        M5DescriptorParityState::DescriptorsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        M5PublicationComponentFamily::ReleaseCandidateCard.canonical_schema_ref()
    );
}

#[test]
fn resolver_narrowed_scope_discloses_self_contained_banner() {
    let input = M5PublicationBindingInput {
        client_scope_mode: M5ClientScopeMode::MirrorOfflineScope,
        handoff_caveats: vec![M5HandoffCaveat::OfflineSnapshot],
        ..full_input(
            M5PublicationComponentConsumer::DocsPortal,
            M5PublicationComponentFamily::PromotionRollbackHistory,
        )
    };
    let resolved = resolve_publication_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.descriptor_parity_state,
        M5DescriptorParityState::DescriptorsDisclosedNarrowed
    );
    let banner = resolved.reduced_scope_banner.expect("banner present");
    assert_eq!(banner.reason, M5ReducedScopeReason::MirrorOffline);
    assert_eq!(
        banner.next_action,
        M5ScopeNextAction::RefreshFromCanonicalSource
    );
    // Descriptors stay preserved even under the narrowing.
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5PublicationDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    // Not a generic "reduced" note.
    assert!(banner.headline.to_lowercase().contains("mirror"));
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (mode, reason) in [
        (
            M5ClientScopeMode::NarrowedClientScope,
            M5ReducedScopeReason::ClientNarrowed,
        ),
        (
            M5ClientScopeMode::MirrorOfflineScope,
            M5ReducedScopeReason::MirrorOffline,
        ),
        (
            M5ClientScopeMode::BrowserCompanionHandoff,
            M5ReducedScopeReason::BrowserCompanionHandoff,
        ),
    ] {
        let input = M5PublicationBindingInput {
            client_scope_mode: mode,
            ..full_input(
                M5PublicationComponentConsumer::UpdateCenter,
                M5PublicationComponentFamily::ReleaseCandidateCard,
            )
        };
        let resolved = resolve_publication_binding(&input).expect("resolves");
        assert_eq!(
            resolved.reduced_scope_banner.expect("banner").reason,
            reason
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5PublicationBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5PublicationComponentConsumer::ReleaseCenter,
            M5PublicationComponentFamily::ReleaseCandidateCard,
        )
    };
    assert_eq!(
        resolve_publication_binding(&empty),
        Err(M5PublicationBindingError::EmptyDescriptorSet)
    );

    let missing = M5PublicationBindingInput {
        descriptor_families: vec![M5PublicationDescriptor::Provenance],
        ..full_input(
            M5PublicationComponentConsumer::ReleaseCenter,
            M5PublicationComponentFamily::ReleaseCandidateCard,
        )
    };
    assert_eq!(
        resolve_publication_binding(&missing),
        Err(M5PublicationBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5PublicationBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5PublicationComponentConsumer::ReleaseCenter,
            M5PublicationComponentFamily::ReleaseCandidateCard,
        )
    };
    assert_eq!(
        resolve_publication_binding(&forbidden),
        Err(M5PublicationBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_artifact_provenance_bundle_cards_and_attestation_or_sbom_status_rows_across_claimed_m5_release_evaluation_support_surfaces::M5_PROVENANCE_BUNDLE_SCHEMA_REF;
    use crate::implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories::M5_RELEASE_HISTORY_STEP_SCHEMA_REF;
    use crate::implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces::M5_RELEASE_CANDIDATE_SCHEMA_REF;
    use crate::ship_version_bump_rows_and_publish_target_review_sheets_across_claimed_m5_publication_lanes::M5_PUBLICATION_REVIEW_SCHEMA_REF;

    assert_eq!(
        M5PublicationComponentFamily::ReleaseCandidateCard.canonical_schema_ref(),
        M5_RELEASE_CANDIDATE_SCHEMA_REF
    );
    assert_eq!(
        M5PublicationComponentFamily::VersionBumpPublishTarget.canonical_schema_ref(),
        M5_PUBLICATION_REVIEW_SCHEMA_REF
    );
    assert_eq!(
        M5PublicationComponentFamily::ArtifactProvenanceBundle.canonical_schema_ref(),
        M5_PROVENANCE_BUNDLE_SCHEMA_REF
    );
    assert_eq!(
        M5PublicationComponentFamily::PromotionRollbackHistory.canonical_schema_ref(),
        M5_RELEASE_HISTORY_STEP_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_publication_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_PUBLICATION_COMPONENT_CONSUMER_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_publication_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5PublicationComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5PublicationComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_publication_component_consumer_packet();
    for family in M5PublicationComponentFamily::ALL {
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
    let packet = seeded_m5_publication_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5PublicationConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5PublicationConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5PublicationDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_publication_component_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            assert_eq!(
                b.canonical_schema_ref,
                b.component_family.canonical_schema_ref()
            );
            assert_eq!(
                b.canonical_artifact_ref,
                b.component_family.canonical_artifact_ref()
            );
            assert!(b.references_canonical_not_local_prose);
        }
    }
}

#[test]
fn every_client_scope_mode_and_reason_is_exercised() {
    let packet = seeded_m5_publication_component_consumer_packet();
    let cases: Vec<&M5PublicationBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for mode in M5ClientScopeMode::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.client_scope_mode == mode),
            "no worked binding exercises client-scope mode {}",
            mode.as_str()
        );
    }
    for reason in M5ReducedScopeReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .reduced_scope_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked binding exercises reduced-scope reason {}",
            reason.as_str()
        );
    }
    for state in M5DescriptorParityState::ALL {
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
    let packet = seeded_m5_publication_component_consumer_packet();
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
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5PublicationComponentConsumer::SupportExport);
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.vocabulary_set.client_scope_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5PublicationDescriptor::Provenance);
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5PublicationConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    // Strip every ReleaseCandidateCard binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5PublicationComponentFamily::ReleaseCandidateCard {
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
        .contains(&M5PublicationConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5PublicationBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_rows[0].drops_provenance_or_freshness_when_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet
        .governance_review
        .mirror_offline_and_handoff_caveats_preserved = false;
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.consumer_projection.freshness_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_publication_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PublicationConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_publication_component_consumer_packet().render_markdown_summary();
    for consumer in M5PublicationComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_publication_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5PublicationComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5PublicationComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_publication_component_consumer_export()
        .expect("checked M5 publication-component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PUBLICATION_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_publication_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_publication_component_consumer_about_help_handoff_narrowed(),
        seeded_m5_publication_component_consumer_docs_mirror_offline_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5PublicationComponentConsumer::ALL.len()
        );
    }

    let about = seeded_m5_publication_component_consumer_about_help_handoff_narrowed();
    let row = about
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5PublicationComponentConsumer::AboutHelp)
        .expect("about/help row present");
    assert_eq!(row.qualification, M5ReleaseCenterQualificationClass::Beta);

    let docs = seeded_m5_publication_component_consumer_docs_mirror_offline_narrowed();
    let row = docs
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5PublicationComponentConsumer::DocsPortal)
        .expect("docs-portal row present");
    assert_eq!(
        row.qualification,
        M5ReleaseCenterQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let about: M5PublicationComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-publication-component-consumers/about_help_handoff_narrowed.json"
    )))
    .expect("about/help fixture parses");
    assert!(about.validate().is_empty());
    assert_eq!(
        about,
        seeded_m5_publication_component_consumer_about_help_handoff_narrowed()
    );

    let docs: M5PublicationComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-publication-component-consumers/docs_mirror_offline_narrowed.json"
    )))
    .expect("docs fixture parses");
    assert!(docs.validate().is_empty());
    assert_eq!(
        docs,
        seeded_m5_publication_component_consumer_docs_mirror_offline_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_publication_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
