use super::*;

fn full_input(
    consumer: M5CompanionComponentConsumer,
    family: M5CompanionComponentFamily,
) -> M5CompanionComponentBindingInput {
    M5CompanionComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5CompanionComponentDescriptor::ALL.to_vec(),
        parity_health: M5CompanionConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_companion_component_binding(&full_input(
        M5CompanionComponentConsumer::Inbox,
        M5CompanionComponentFamily::NotificationRow,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_stale_desktop_or_policy_blocked_state);
    assert!(resolved.asserts_live_and_companion_safe);
    assert_eq!(
        resolved.claim_parity_state,
        M5CompanionClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5CompanionComponentFamily::NotificationRow)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5CompanionComponentBindingInput {
        parity_health: M5CompanionConsumerParityHealth::PolicyBlockedNarrowed,
        export_caveats: vec![M5CompanionConsumerExportCaveat::PolicyBlockedNotCompanionSafe],
        ..full_input(
            M5CompanionComponentConsumer::Advisory,
            M5CompanionComponentFamily::IncidentSnapshotCard,
        )
    };
    let resolved = resolve_companion_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.asserts_live_and_companion_safe);
    assert_eq!(
        resolved.claim_parity_state,
        M5CompanionClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5CompanionConsumerNarrowingReason::PolicyBlockedOnCompanion
    );
    assert_eq!(
        banner.recovery_action,
        M5CompanionConsumerRecoveryAction::RequestPolicyGrantOrUseDesktop
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5CompanionComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("policy"));
}

#[test]
fn resolver_cached_narrows_but_stays_safe_and_not_stale() {
    // A cached value narrows freshness but does NOT make the component stale, desktop-required, or
    // policy-blocked, so it must not be counted against the live-safety-honesty invariant.
    let input = M5CompanionComponentBindingInput {
        parity_health: M5CompanionConsumerParityHealth::CachedNarrowed,
        export_caveats: vec![M5CompanionConsumerExportCaveat::CachedNotLive],
        ..full_input(
            M5CompanionComponentConsumer::Inbox,
            M5CompanionComponentFamily::MobileReviewCard,
        )
    };
    let resolved = resolve_companion_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.reflects_stale_desktop_or_policy_blocked_state);
    assert!(!resolved.asserts_live_and_companion_safe);
    assert_eq!(
        resolved.auto_narrow_banner.expect("banner").reason,
        M5CompanionConsumerNarrowingReason::ShowingCachedValue
    );
}

#[test]
fn resolver_stale_desktop_or_policy_blocked_state_never_asserts_live_and_safe() {
    for (health, reason) in [
        (
            M5CompanionConsumerParityHealth::StaleNarrowed,
            M5CompanionConsumerNarrowingReason::StaleBeyondWindow,
        ),
        (
            M5CompanionConsumerParityHealth::DesktopRequiredNarrowed,
            M5CompanionConsumerNarrowingReason::DesktopRequiredAction,
        ),
        (
            M5CompanionConsumerParityHealth::PolicyBlockedNarrowed,
            M5CompanionConsumerNarrowingReason::PolicyBlockedOnCompanion,
        ),
    ] {
        let input = M5CompanionComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5CompanionComponentConsumer::Review,
                M5CompanionComponentFamily::DesktopHandoffSheet,
            )
        };
        let resolved = resolve_companion_component_binding(&input).expect("resolves");
        assert!(resolved.reflects_stale_desktop_or_policy_blocked_state);
        assert!(!resolved.asserts_live_and_companion_safe);
        assert!(resolved.is_narrowed);
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5CompanionConsumerParityHealth::CachedNarrowed,
            M5CompanionConsumerNarrowingReason::ShowingCachedValue,
        ),
        (
            M5CompanionConsumerParityHealth::StaleNarrowed,
            M5CompanionConsumerNarrowingReason::StaleBeyondWindow,
        ),
        (
            M5CompanionConsumerParityHealth::DesktopRequiredNarrowed,
            M5CompanionConsumerNarrowingReason::DesktopRequiredAction,
        ),
        (
            M5CompanionConsumerParityHealth::PolicyBlockedNarrowed,
            M5CompanionConsumerNarrowingReason::PolicyBlockedOnCompanion,
        ),
    ] {
        let input = M5CompanionComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5CompanionComponentConsumer::Support,
                M5CompanionComponentFamily::NotificationRow,
            )
        };
        let resolved = resolve_companion_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5CompanionComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5CompanionComponentConsumer::Inbox,
            M5CompanionComponentFamily::NotificationRow,
        )
    };
    assert_eq!(
        resolve_companion_component_binding(&empty),
        Err(M5CompanionComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5CompanionComponentBindingInput {
        descriptor_families: vec![M5CompanionComponentDescriptor::ObjectIdentity],
        ..full_input(
            M5CompanionComponentConsumer::Inbox,
            M5CompanionComponentFamily::NotificationRow,
        )
    };
    assert_eq!(
        resolve_companion_component_binding(&missing),
        Err(M5CompanionComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5CompanionComponentBindingInput {
        note_repr: Some("internal://relay/leak".to_owned()),
        ..full_input(
            M5CompanionComponentConsumer::Inbox,
            M5CompanionComponentFamily::NotificationRow,
        )
    };
    assert_eq!(
        resolve_companion_component_binding(&forbidden),
        Err(M5CompanionComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_map_to_the_three_narrowed_primitives() {
    use crate::implement_ci_status_cards_and_session_follow_tiles_with_provider_source_run_or_session_identity_stale_state_labeling_and_follow_or_handoff_continuity::CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_REF;
    use crate::implement_incident_snapshot_cards_and_desktop_handoff_sheets_with_service_run_identity_severity_status_target_identity_auth_tenant_reminder_and_open_on_desktop_truth::INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_REF;
    use crate::implement_notification_rows_and_mobile_review_cards_with_object_identity_client_scope_freshness_severity_unread_and_desktop_handoff_truth::NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_REF;
    use M5CompanionComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::NotificationRow),
        NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::MobileReviewCard),
        NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::CiStatusCard),
        CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SessionFollowTile),
        CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::IncidentSnapshotCard),
        INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::DesktopHandoffSheet),
        INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_companion_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_COMPANION_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_companion_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5CompanionComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5CompanionComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_companion_component_consumer_packet();
    for family in M5CompanionComponentFamily::ALL {
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
    let packet = seeded_m5_companion_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5CompanionConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5CompanionConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5CompanionComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5CompanionAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_companion_component_consumer_packet();
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
    let packet = seeded_m5_companion_component_consumer_packet();
    let cases: Vec<&M5CompanionComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5CompanionConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5CompanionConsumerNarrowingReason::ALL {
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
    for state in M5CompanionClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn stale_desktop_or_policy_blocked_bindings_never_assert_live_and_safe() {
    let packet = seeded_m5_companion_component_consumer_packet();
    let mut seen = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_stale_desktop_or_policy_blocked_state {
                    seen = true;
                    assert!(!case.resolved.asserts_live_and_companion_safe);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen,
        "no stale / desktop-required / policy-blocked binding present to prove live-safety honesty"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_companion_component_consumer_packet();
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
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5CompanionComponentConsumer::Advisory);
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5CompanionComponentDescriptor::HandoffTarget);
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5CompanionConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    // Strip every NotificationRow binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5CompanionComponentFamily::NotificationRow {
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
        .contains(&M5CompanionComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5CompanionComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(
        violations.contains(&M5CompanionComponentConsumerViolation::NarrowingDisclosureUnproven)
    );
}

#[test]
fn live_safety_honesty_unproven_fails_when_no_stale_or_desktop_example_present() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    // Replace every binding with a full-parity case: no stale / desktop / policy state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5CompanionComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::LiveSafetyHonestyUnproven));
}

#[test]
fn live_safety_honesty_unproven_fails_when_stale_state_claims_live_and_safe() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    // Find a stale / desktop / policy binding and force it to assert live-and-companion-safe.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_stale_desktop_or_policy_blocked_state {
                    case.resolved.asserts_live_and_companion_safe = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::LiveSafetyHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.consumer_rows[0].shows_stale_or_desktop_required_state_as_live_and_companion_safe = true;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn help_support_export_reference_missing_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5CompanionComponentConsumer::Support)
        .expect("support row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(violations
        .contains(&M5CompanionComponentConsumerViolation::HelpSupportExportReferenceMissing));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet
        .governance_review
        .stale_or_desktop_required_state_never_shown_as_live_and_companion_safe = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet
        .consumer_projection
        .handoff_target_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_companion_component_consumer_packet().render_markdown_summary();
    for consumer in M5CompanionComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_companion_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CompanionComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5CompanionComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_companion_component_consumer_export()
        .expect("checked M5 companion component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_COMPANION_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_companion_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_companion_component_consumer_advisory_beta_narrowed(),
        seeded_m5_companion_component_consumer_handoff_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5CompanionComponentConsumer::ALL.len()
        );
    }

    let advisory = seeded_m5_companion_component_consumer_advisory_beta_narrowed();
    let row = advisory
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5CompanionComponentConsumer::Advisory)
        .expect("advisory row present");
    assert_eq!(row.qualification, M5CompanionQualificationClass::Beta);

    let handoff = seeded_m5_companion_component_consumer_handoff_preview_narrowed();
    let row = handoff
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5CompanionComponentConsumer::Handoff)
        .expect("handoff row present");
    assert_eq!(row.qualification, M5CompanionQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let advisory: M5CompanionComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-companion-component-consumers/advisory_beta_narrowed.json"
    )))
    .expect("advisory fixture parses");
    assert!(advisory.validate().is_empty());
    assert_eq!(
        advisory,
        seeded_m5_companion_component_consumer_advisory_beta_narrowed()
    );

    let handoff: M5CompanionComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-companion-component-consumers/handoff_preview_narrowed.json"
    )))
    .expect("handoff fixture parses");
    assert!(handoff.validate().is_empty());
    assert_eq!(
        handoff,
        seeded_m5_companion_component_consumer_handoff_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_companion_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}
