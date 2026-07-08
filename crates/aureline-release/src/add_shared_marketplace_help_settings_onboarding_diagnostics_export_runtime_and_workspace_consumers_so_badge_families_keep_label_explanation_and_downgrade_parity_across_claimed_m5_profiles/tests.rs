use super::*;

fn full_input(consumer: M5BadgeConsumer, family: M5BadgeFamily) -> M5BadgeConsumerBindingInput {
    M5BadgeConsumerBindingInput {
        consumer,
        badge_family: family,
        parity_facets: M5BadgeParityFacet::ALL.to_vec(),
        render_mode: M5BadgeRenderMode::FullClaimScope,
        downgrade_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_scope_preserves_facets_with_no_banner() {
    let resolved = resolve_badge_consumer_binding(&full_input(
        M5BadgeConsumer::Marketplace,
        M5BadgeFamily::SupportClass,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.narrow_banner.is_none());
    assert_eq!(resolved.parity_state, M5BadgeParityState::FacetsPreserved);
    assert_eq!(
        resolved.canonical_schema_ref,
        badge_family_canonical_schema_ref(M5BadgeFamily::SupportClass)
    );
}

#[test]
fn resolver_narrowed_scope_discloses_self_contained_banner() {
    let input = M5BadgeConsumerBindingInput {
        render_mode: M5BadgeRenderMode::FreshnessNarrowed,
        downgrade_caveats: vec![M5BadgeDowngradeTrigger::ProofStale],
        ..full_input(
            M5BadgeConsumer::Onboarding,
            M5BadgeFamily::EvidenceFreshness,
        )
    };
    let resolved = resolve_badge_consumer_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.parity_state,
        M5BadgeParityState::FacetsDisclosedNarrowed
    );
    let banner = resolved.narrow_banner.expect("banner present");
    assert_eq!(banner.reason, M5BadgeNarrowReason::EvidenceStale);
    assert_eq!(
        banner.next_action,
        M5BadgeNarrowNextAction::RefreshStaleEvidence
    );
    // Facets stay preserved even under the narrowing.
    assert_eq!(banner.preserved_facets.len(), M5BadgeParityFacet::ALL.len());
    assert!(!banner.headline.trim().is_empty());
    // Not a generic "reduced" note.
    assert!(banner.headline.to_lowercase().contains("stale"));
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (mode, reason) in [
        (
            M5BadgeRenderMode::FreshnessNarrowed,
            M5BadgeNarrowReason::EvidenceStale,
        ),
        (
            M5BadgeRenderMode::ScopeNarrowed,
            M5BadgeNarrowReason::ScopeReduced,
        ),
        (
            M5BadgeRenderMode::ExportProjection,
            M5BadgeNarrowReason::ExportSnapshot,
        ),
    ] {
        let input = M5BadgeConsumerBindingInput {
            render_mode: mode,
            ..full_input(
                M5BadgeConsumer::Diagnostics,
                M5BadgeFamily::CompatibilityState,
            )
        };
        let resolved = resolve_badge_consumer_binding(&input).expect("resolves");
        assert_eq!(resolved.narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5BadgeConsumerBindingInput {
        parity_facets: vec![],
        ..full_input(M5BadgeConsumer::Marketplace, M5BadgeFamily::SupportClass)
    };
    assert_eq!(
        resolve_badge_consumer_binding(&empty),
        Err(M5BadgeConsumerBindingError::EmptyParityFacetSet)
    );

    let missing = M5BadgeConsumerBindingInput {
        parity_facets: vec![M5BadgeParityFacet::Label],
        ..full_input(M5BadgeConsumer::Marketplace, M5BadgeFamily::SupportClass)
    };
    assert_eq!(
        resolve_badge_consumer_binding(&missing),
        Err(M5BadgeConsumerBindingError::MissingRequiredFacet)
    );

    let forbidden = M5BadgeConsumerBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(M5BadgeConsumer::Marketplace, M5BadgeFamily::SupportClass)
    };
    assert_eq!(
        resolve_badge_consumer_binding(&forbidden),
        Err(M5BadgeConsumerBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces::M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF;
    use crate::implement_lifecycle_and_channel_badges_across_claimed_m5_command_feature_bundle_extension_and_install_surfaces::M5_MATURITY_BADGE_SCHEMA_REF;
    use crate::implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces::M5_BADGE_CLAIM_SCHEMA_REF;
    use crate::ship_compatibility_state_badges_and_mismatch_review_affordances_across_claimed_m5_workspace_toolchain_extension_bundle_and_artifact_flows::M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF;

    assert_eq!(
        badge_family_canonical_schema_ref(M5BadgeFamily::SupportClass),
        M5_BADGE_CLAIM_SCHEMA_REF
    );
    assert_eq!(
        badge_family_canonical_schema_ref(M5BadgeFamily::EvidenceFreshness),
        M5_BADGE_CLAIM_SCHEMA_REF
    );
    assert_eq!(
        badge_family_canonical_schema_ref(M5BadgeFamily::Lifecycle),
        M5_MATURITY_BADGE_SCHEMA_REF
    );
    assert_eq!(
        badge_family_canonical_schema_ref(M5BadgeFamily::Channel),
        M5_MATURITY_BADGE_SCHEMA_REF
    );
    assert_eq!(
        badge_family_canonical_schema_ref(M5BadgeFamily::DeploymentScope),
        M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF
    );
    assert_eq!(
        badge_family_canonical_schema_ref(M5BadgeFamily::CompatibilityState),
        M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_badge_family_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BADGE_FAMILY_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_badge_family_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5BadgeConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(packet.consumer_rows.len(), M5BadgeConsumer::ALL.len());
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_badge_family_consumer_packet();
    for family in M5BadgeFamily::ALL {
        let count = packet
            .consumer_rows
            .iter()
            .filter(|row| row.family_bindings.iter().any(|b| b.badge_family == family))
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
fn every_row_declares_mandatory_anatomy_export_and_facets() {
    let packet = seeded_m5_badge_family_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5BadgeConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5BadgeConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for facet in M5BadgeParityFacet::REQUIRED {
            assert!(row.parity_facets.contains(&facet));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable));
        assert!(!row.family_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_badge_family_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.family_bindings {
            assert_eq!(
                b.canonical_schema_ref,
                badge_family_canonical_schema_ref(b.badge_family)
            );
            assert_eq!(
                b.canonical_artifact_ref,
                badge_family_canonical_artifact_ref(b.badge_family)
            );
            assert!(b.references_canonical_not_local_prose);
        }
    }
}

#[test]
fn every_render_mode_and_reason_is_exercised() {
    let packet = seeded_m5_badge_family_consumer_packet();
    let cases: Vec<&M5BadgeConsumerBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.family_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for mode in M5BadgeRenderMode::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.render_mode == mode),
            "no worked binding exercises render mode {}",
            mode.as_str()
        );
    }
    for reason in M5BadgeNarrowReason::ALL {
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
    for state in M5BadgeParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_state == state),
            "no worked binding exercises parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_badge_family_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.family_bindings {
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
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5BadgeConsumer::Workspace);
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.vocabulary_set.render_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_rows[0].family_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_rows[0].family_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_facet_missing_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_rows[0]
        .parity_facets
        .retain(|f| *f != M5BadgeParityFacet::Explanation);
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::RequiredFacetMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5BadgeConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_rows[0].family_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_rows[1].family_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    // Strip every SupportClass binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.family_bindings.retain(|b| {
            if b.badge_family == M5BadgeFamily::SupportClass {
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
        .contains(&M5BadgeConsumerViolation::BadgeFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.family_bindings {
            b.example_bindings = vec![M5BadgeConsumerBindingCase::resolved(full_input(
                row.consumer,
                b.badge_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_rows[0].implies_freshness_from_support_class = true;
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet
        .governance_review
        .freshness_never_implied_from_support_class = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.consumer_projection.explanation_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn docs_help_reference_missing_fails() {
    let mut packet = seeded_m5_badge_family_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5BadgeConsumer::HelpAbout)
        .expect("help/about row present");
    row.family_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(violations.contains(&M5BadgeConsumerViolation::DocsHelpReferenceMissing));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_badge_family_consumer_packet().render_markdown_summary();
    for consumer in M5BadgeConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_badge_family_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5BadgeConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5BadgeConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_badge_family_consumer_export()
        .expect("checked M5 badge-family consumer export validates");
    assert_eq!(from_disk.packet_id, M5_BADGE_FAMILY_CONSUMER_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_badge_family_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_badge_family_consumer_diagnostics_freshness_beta_narrowed(),
        seeded_m5_badge_family_consumer_support_export_scope_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.consumer_rows.len(), M5BadgeConsumer::ALL.len());
    }

    let diagnostics = seeded_m5_badge_family_consumer_diagnostics_freshness_beta_narrowed();
    let row = diagnostics
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5BadgeConsumer::Diagnostics)
        .expect("diagnostics row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Beta);

    let support = seeded_m5_badge_family_consumer_support_export_scope_preview_narrowed();
    let row = support
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5BadgeConsumer::SupportExport)
        .expect("support-export row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let diagnostics: M5BadgeFamilyConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-badge-family-consumers/diagnostics_freshness_beta_narrowed.json"
    )))
    .expect("diagnostics fixture parses");
    assert!(diagnostics.validate().is_empty());
    assert_eq!(
        diagnostics,
        seeded_m5_badge_family_consumer_diagnostics_freshness_beta_narrowed()
    );

    let support: M5BadgeFamilyConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-badge-family-consumers/support_export_scope_preview_narrowed.json"
    )))
    .expect("support-export fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_badge_family_consumer_support_export_scope_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_badge_family_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
