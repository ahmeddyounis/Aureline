use super::*;

fn full_input(
    consumer: M5CredentialComponentConsumer,
    family: M5CredentialComponentFamily,
) -> M5CredentialComponentBindingInput {
    M5CredentialComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5CredentialComponentDescriptor::ALL.to_vec(),
        parity_health: M5CredentialConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_credential_component_binding(&full_input(
        M5CredentialComponentConsumer::Settings,
        M5CredentialComponentFamily::CredentialStateRow,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_unusable_or_forwarded_state);
    assert!(resolved.asserts_credential_usable_and_local);
    assert_eq!(
        resolved.claim_parity_state,
        M5CredentialClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5CredentialComponentFamily::CredentialStateRow)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5CredentialComponentBindingInput {
        parity_health: M5CredentialConsumerParityHealth::SessionOnlyOrPolicyBlockedNarrowed,
        export_caveats: vec![
            M5CredentialConsumerExportCaveat::SessionOnlyOrPolicyBlockedNotDurable,
        ],
        ..full_input(
            M5CredentialComponentConsumer::Registry,
            M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard,
        )
    };
    let resolved = resolve_credential_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.asserts_credential_usable_and_local);
    assert_eq!(
        resolved.claim_parity_state,
        M5CredentialClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5CredentialConsumerNarrowingReason::SessionOnlyOrPolicyBlocked
    );
    assert_eq!(
        banner.recovery_action,
        M5CredentialConsumerRecoveryAction::StoreDurablyOrRequestPolicyGrant
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5CredentialComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("session"));
}

#[test]
fn resolver_handle_only_narrows_but_stays_usable_and_not_unusable() {
    // A handle-only path narrows the reveal posture but does NOT make the credential unusable or
    // forwarded, so it must not be counted against the usability-honesty invariant.
    let input = M5CredentialComponentBindingInput {
        parity_health: M5CredentialConsumerParityHealth::HandleOnlyNarrowed,
        export_caveats: vec![M5CredentialConsumerExportCaveat::HandleOnlyNoRawExport],
        ..full_input(
            M5CredentialComponentConsumer::Request,
            M5CredentialComponentFamily::SecretAccessPromptSheet,
        )
    };
    let resolved = resolve_credential_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.reflects_unusable_or_forwarded_state);
    assert!(!resolved.asserts_credential_usable_and_local);
    assert_eq!(
        resolved.auto_narrow_banner.expect("banner").reason,
        M5CredentialConsumerNarrowingReason::HandleOnlyPath
    );
}

#[test]
fn resolver_unusable_or_forwarded_state_never_asserts_usable_and_local() {
    for (health, reason) in [
        (
            M5CredentialConsumerParityHealth::ExpiredOrRevokedNarrowed,
            M5CredentialConsumerNarrowingReason::CredentialExpiredOrRevoked,
        ),
        (
            M5CredentialConsumerParityHealth::DelegatedOrForwardedNarrowed,
            M5CredentialConsumerNarrowingReason::IdentityForwardedOrDelegated,
        ),
        (
            M5CredentialConsumerParityHealth::SessionOnlyOrPolicyBlockedNarrowed,
            M5CredentialConsumerNarrowingReason::SessionOnlyOrPolicyBlocked,
        ),
    ] {
        let input = M5CredentialComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5CredentialComponentConsumer::Database,
                M5CredentialComponentFamily::DelegatedCredentialRow,
            )
        };
        let resolved = resolve_credential_component_binding(&input).expect("resolves");
        assert!(resolved.reflects_unusable_or_forwarded_state);
        assert!(!resolved.asserts_credential_usable_and_local);
        assert!(resolved.is_narrowed);
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5CredentialConsumerParityHealth::HandleOnlyNarrowed,
            M5CredentialConsumerNarrowingReason::HandleOnlyPath,
        ),
        (
            M5CredentialConsumerParityHealth::ExpiredOrRevokedNarrowed,
            M5CredentialConsumerNarrowingReason::CredentialExpiredOrRevoked,
        ),
        (
            M5CredentialConsumerParityHealth::DelegatedOrForwardedNarrowed,
            M5CredentialConsumerNarrowingReason::IdentityForwardedOrDelegated,
        ),
        (
            M5CredentialConsumerParityHealth::SessionOnlyOrPolicyBlockedNarrowed,
            M5CredentialConsumerNarrowingReason::SessionOnlyOrPolicyBlocked,
        ),
    ] {
        let input = M5CredentialComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5CredentialComponentConsumer::Support,
                M5CredentialComponentFamily::CredentialStateRow,
            )
        };
        let resolved = resolve_credential_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5CredentialComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5CredentialComponentConsumer::Settings,
            M5CredentialComponentFamily::CredentialStateRow,
        )
    };
    assert_eq!(
        resolve_credential_component_binding(&empty),
        Err(M5CredentialComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5CredentialComponentBindingInput {
        descriptor_families: vec![M5CredentialComponentDescriptor::StorageMode],
        ..full_input(
            M5CredentialComponentConsumer::Settings,
            M5CredentialComponentFamily::CredentialStateRow,
        )
    };
    assert_eq!(
        resolve_credential_component_binding(&missing),
        Err(M5CredentialComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5CredentialComponentBindingInput {
        note_repr: Some("internal://vault/leak".to_owned()),
        ..full_input(
            M5CredentialComponentConsumer::Settings,
            M5CredentialComponentFamily::CredentialStateRow,
        )
    };
    assert_eq!(
        resolve_credential_component_binding(&forbidden),
        Err(M5CredentialComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_map_to_the_four_narrowed_primitives() {
    use crate::implement_browser_or_device_code_handoff_cards_and_delegated_credential_rows_with_handoff_boundary_and_delegated_identity_origin_truth::BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF;
    use crate::implement_credential_state_rows_and_vault_or_keychain_pickers_with_source_target_boundary_expiry_portability_and_rotate_revoke_test_truth::CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF;
    use crate::implement_rotation_revoke_event_rows_and_export_safety_banners_with_impacted_workflow_remembered_decision_and_raw_secret_excluded_continuity_truth::ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF;
    use crate::implement_secret_access_prompt_sheets_and_credential_store_capability_rows_with_actor_scope_handle_only_and_session_fallback_truth::SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF;
    use M5CredentialComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::CredentialStateRow),
        CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::VaultOrKeychainPicker),
        CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SecretAccessPromptSheet),
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::CredentialStoreCapabilityRow),
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::BrowserDeviceCodeHandoffCard),
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::DelegatedCredentialRow),
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RotationRevokeEventRow),
        ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ExportSafetyBanner),
        ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_credential_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CREDENTIAL_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_credential_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5CredentialComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5CredentialComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_credential_component_consumer_packet();
    for family in M5CredentialComponentFamily::ALL {
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
    let packet = seeded_m5_credential_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5CredentialConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5CredentialConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5CredentialComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_credential_component_consumer_packet();
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
    let packet = seeded_m5_credential_component_consumer_packet();
    let cases: Vec<&M5CredentialComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5CredentialConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5CredentialConsumerNarrowingReason::ALL {
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
    for state in M5CredentialClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn unusable_or_forwarded_bindings_never_assert_usable_and_local() {
    let packet = seeded_m5_credential_component_consumer_packet();
    let mut seen = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_unusable_or_forwarded_state {
                    seen = true;
                    assert!(!case.resolved.asserts_credential_usable_and_local);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen,
        "no unusable / forwarded binding present to prove usability honesty"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_credential_component_consumer_packet();
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
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5CredentialComponentConsumer::Registry);
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5CredentialComponentDescriptor::ExportSafety);
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5CredentialConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    // Strip every CredentialStateRow binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5CredentialComponentFamily::CredentialStateRow {
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
        .contains(&M5CredentialComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5CredentialComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(
        violations.contains(&M5CredentialComponentConsumerViolation::NarrowingDisclosureUnproven)
    );
}

#[test]
fn usability_honesty_unproven_fails_when_no_unusable_or_forwarded_example_present() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    // Replace every binding with a full-parity case: no unusable / forwarded state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5CredentialComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::UsabilityHonestyUnproven));
}

#[test]
fn usability_honesty_unproven_fails_when_forwarded_state_claims_usable_and_local() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    // Find an unusable / forwarded binding and force it to assert usable-and-local.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_unusable_or_forwarded_state {
                    case.resolved.asserts_credential_usable_and_local = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::UsabilityHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_rows[0].shows_unusable_or_forwarded_state_as_usable_and_local = true;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn help_support_export_reference_missing_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5CredentialComponentConsumer::Support)
        .expect("support row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(violations
        .contains(&M5CredentialComponentConsumerViolation::HelpSupportExportReferenceMissing));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet
        .governance_review
        .unusable_or_forwarded_state_never_shown_as_usable_and_local = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.consumer_projection.export_safety_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_credential_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_credential_component_consumer_packet().render_markdown_summary();
    for consumer in M5CredentialComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_credential_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CredentialComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5CredentialComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_credential_component_consumer_export()
        .expect("checked M5 credential component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_CREDENTIAL_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_credential_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_credential_component_consumer_registry_beta_narrowed(),
        seeded_m5_credential_component_consumer_database_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5CredentialComponentConsumer::ALL.len()
        );
    }

    let registry = seeded_m5_credential_component_consumer_registry_beta_narrowed();
    let row = registry
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5CredentialComponentConsumer::Registry)
        .expect("registry row present");
    assert_eq!(row.qualification, M5CredentialQualificationClass::Beta);

    let database = seeded_m5_credential_component_consumer_database_preview_narrowed();
    let row = database
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5CredentialComponentConsumer::Database)
        .expect("database row present");
    assert_eq!(row.qualification, M5CredentialQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let registry: M5CredentialComponentConsumerPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-credential-component-consumers/registry_beta_narrowed.json"
        )))
        .expect("registry fixture parses");
    assert!(registry.validate().is_empty());
    assert_eq!(
        registry,
        seeded_m5_credential_component_consumer_registry_beta_narrowed()
    );

    let database: M5CredentialComponentConsumerPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-credential-component-consumers/database_preview_narrowed.json"
        )))
        .expect("database fixture parses");
    assert!(database.validate().is_empty());
    assert_eq!(
        database,
        seeded_m5_credential_component_consumer_database_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_credential_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}
