use super::*;

fn full_input(
    consumer: M5ProviderComponentConsumer,
    family: M5ProviderAccountOfflineComponentFamily,
) -> M5ProviderComponentBindingInput {
    M5ProviderComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5ProviderComponentDescriptor::ALL.to_vec(),
        parity_health: M5ProviderConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_provider_component_binding(&full_input(
        M5ProviderComponentConsumer::WorkItemDetail,
        M5ProviderAccountOfflineComponentFamily::ProviderAccountRow,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_cached_or_offline_state);
    assert!(resolved.asserts_provider_committed);
    assert_eq!(
        resolved.claim_parity_state,
        M5ProviderClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5ProviderAccountOfflineComponentFamily::ProviderAccountRow)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5ProviderComponentBindingInput {
        parity_health: M5ProviderConsumerParityHealth::ScopeLimitedNarrowed,
        export_caveats: vec![M5ProviderConsumerExportCaveat::ScopeLimitedReadOnly],
        ..full_input(
            M5ProviderComponentConsumer::WorkItemDetail,
            M5ProviderAccountOfflineComponentFamily::SyncBehaviorRow,
        )
    };
    let resolved = resolve_provider_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.asserts_provider_committed);
    assert_eq!(
        resolved.claim_parity_state,
        M5ProviderClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5ProviderConsumerNarrowingReason::ProviderScopeLimited
    );
    assert_eq!(
        banner.recovery_action,
        M5ProviderConsumerRecoveryAction::ReauthorizeForFullScope
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5ProviderComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("provider scope"));
}

#[test]
fn resolver_cached_or_offline_state_never_asserts_committed() {
    for (health, reason) in [
        (
            M5ProviderConsumerParityHealth::SessionStaleNarrowed,
            M5ProviderConsumerNarrowingReason::SessionStale,
        ),
        (
            M5ProviderConsumerParityHealth::PacketLocalOnlyNarrowed,
            M5ProviderConsumerNarrowingReason::PacketLocalOnly,
        ),
    ] {
        let input = M5ProviderComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5ProviderComponentConsumer::BrowserHandoff,
                M5ProviderAccountOfflineComponentFamily::ProviderAccountRow,
            )
        };
        let resolved = resolve_provider_component_binding(&input).expect("resolves");
        assert!(resolved.reflects_cached_or_offline_state);
        assert!(!resolved.asserts_provider_committed);
        assert!(resolved.is_narrowed);
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5ProviderConsumerParityHealth::ScopeLimitedNarrowed,
            M5ProviderConsumerNarrowingReason::ProviderScopeLimited,
        ),
        (
            M5ProviderConsumerParityHealth::SessionStaleNarrowed,
            M5ProviderConsumerNarrowingReason::SessionStale,
        ),
        (
            M5ProviderConsumerParityHealth::MappingPolicyLockedNarrowed,
            M5ProviderConsumerNarrowingReason::MappingPolicyLocked,
        ),
        (
            M5ProviderConsumerParityHealth::PacketLocalOnlyNarrowed,
            M5ProviderConsumerNarrowingReason::PacketLocalOnly,
        ),
    ] {
        let input = M5ProviderComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5ProviderComponentConsumer::SupportExport,
                M5ProviderAccountOfflineComponentFamily::ProviderAccountRow,
            )
        };
        let resolved = resolve_provider_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5ProviderComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5ProviderComponentConsumer::WorkItemDetail,
            M5ProviderAccountOfflineComponentFamily::ProviderAccountRow,
        )
    };
    assert_eq!(
        resolve_provider_component_binding(&empty),
        Err(M5ProviderComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5ProviderComponentBindingInput {
        descriptor_families: vec![M5ProviderComponentDescriptor::AccountState],
        ..full_input(
            M5ProviderComponentConsumer::WorkItemDetail,
            M5ProviderAccountOfflineComponentFamily::ProviderAccountRow,
        )
    };
    assert_eq!(
        resolve_provider_component_binding(&missing),
        Err(M5ProviderComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5ProviderComponentBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5ProviderComponentConsumer::WorkItemDetail,
            M5ProviderAccountOfflineComponentFamily::ProviderAccountRow,
        )
    };
    assert_eq!(
        resolve_provider_component_binding(&forbidden),
        Err(M5ProviderComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_offline_capture_rows_and_privacy_redaction_rows_with_packet_destination_queued_draft_count_export_clear_actions_and_metadata_safe_boundary_truth_across_claimed_m5_provider_workflows::M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF;
    use crate::implement_provider_account_rows_with_signed_in_limited_scope_stale_session_offline_cached_policy_blocked_truth_and_sign_in_retry_remove_parity_across_claimed_m5_provider_surfaces::M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF;
    use crate::ship_project_or_board_mapping_rows_and_sync_behavior_rows_with_inherited_local_policy_scope_read_only_comment_transition_sync_modes_and_change_reset_parity_across_claimed_m5_provider_lanes::M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF;
    use M5ProviderAccountOfflineComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::ProviderAccountRow),
        M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ProjectOrBoardMappingRow),
        M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SyncBehaviorRow),
        M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::OfflineCaptureRow),
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::PrivacyRedactionRow),
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_provider_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PROVIDER_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_provider_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5ProviderComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5ProviderComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_provider_component_consumer_packet();
    for family in M5ProviderAccountOfflineComponentFamily::ALL {
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
    let packet = seeded_m5_provider_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5ProviderConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ProviderConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5ProviderComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_provider_component_consumer_packet();
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
    let packet = seeded_m5_provider_component_consumer_packet();
    let cases: Vec<&M5ProviderComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5ProviderConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5ProviderConsumerNarrowingReason::ALL {
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
    for state in M5ProviderClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn cached_or_offline_bindings_never_assert_committed() {
    let packet = seeded_m5_provider_component_consumer_packet();
    let mut seen_cached = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_cached_or_offline_state {
                    seen_cached = true;
                    assert!(!case.resolved.asserts_provider_committed);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen_cached,
        "no cached / offline binding present to prove AC2"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_provider_component_consumer_packet();
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
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5ProviderComponentConsumer::IssueIntake);
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5ProviderComponentDescriptor::RedactionPosture);
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5ProviderConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    // Strip every ProviderAccountRow binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5ProviderAccountOfflineComponentFamily::ProviderAccountRow {
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
        .contains(&M5ProviderComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5ProviderComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5ProviderComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn commit_honesty_unproven_fails_when_no_cached_or_offline_example_present() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    // Replace every binding with a full-parity case: no cached / offline state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5ProviderComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::CommitHonestyUnproven));
}

#[test]
fn commit_honesty_unproven_fails_when_cached_state_claims_committed() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    // Find a cached / offline binding and force it to assert committed state.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_cached_or_offline_state {
                    case.resolved.asserts_provider_committed = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::CommitHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.consumer_rows[0].shows_cached_or_offline_state_as_committed = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5ProviderComponentConsumer::SupportExport)
        .expect("support / export row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5ProviderComponentConsumerViolation::SupportExportReferenceMissing)
    );
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet
        .governance_review
        .cached_or_offline_state_never_shown_as_committed = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet
        .consumer_projection
        .redaction_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_provider_component_consumer_packet().render_markdown_summary();
    for consumer in M5ProviderComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_provider_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ProviderComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5ProviderComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_provider_component_consumer_export()
        .expect("checked M5 provider component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PROVIDER_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_provider_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_provider_component_consumer_browser_handoff_beta_narrowed(),
        seeded_m5_provider_component_consumer_issue_intake_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5ProviderComponentConsumer::ALL.len()
        );
    }

    let browser = seeded_m5_provider_component_consumer_browser_handoff_beta_narrowed();
    let row = browser
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5ProviderComponentConsumer::BrowserHandoff)
        .expect("browser-handoff row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Beta);

    let intake = seeded_m5_provider_component_consumer_issue_intake_preview_narrowed();
    let row = intake
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5ProviderComponentConsumer::IssueIntake)
        .expect("issue-intake row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let browser: M5ProviderComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-account-offline-capture-component-consumers/browser_handoff_beta_narrowed.json"
    )))
    .expect("browser-handoff fixture parses");
    assert!(browser.validate().is_empty());
    assert_eq!(
        browser,
        seeded_m5_provider_component_consumer_browser_handoff_beta_narrowed()
    );

    let intake: M5ProviderComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-account-offline-capture-component-consumers/issue_intake_preview_narrowed.json"
    )))
    .expect("issue-intake fixture parses");
    assert!(intake.validate().is_empty());
    assert_eq!(
        intake,
        seeded_m5_provider_component_consumer_issue_intake_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_provider_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
