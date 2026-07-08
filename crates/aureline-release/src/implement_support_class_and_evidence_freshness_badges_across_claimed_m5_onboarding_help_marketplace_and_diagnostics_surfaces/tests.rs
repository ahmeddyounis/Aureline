use super::*;

fn input(
    support_class: M5SupportClassBadgeValue,
    freshness: M5EvidenceFreshnessValue,
) -> M5BadgeClaimInput {
    M5BadgeClaimInput {
        subject_label: "aureline capability: sample".to_owned(),
        support_class,
        freshness,
        evidence_source_repr: "evidence-source:cert-suite:sample".to_owned(),
        last_evaluated_repr: "2026-07-01T00:00:00Z".to_owned(),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_fresh_evidence_is_current_with_no_note() {
    let resolved = resolve_badge_claim(&input(
        M5SupportClassBadgeValue::Certified,
        M5EvidenceFreshnessValue::Fresh,
    ))
    .expect("resolves");
    assert_eq!(
        resolved.effective_claim,
        M5EffectiveClaimPosture::ClaimCurrent
    );
    assert!(resolved.is_current);
    assert!(!resolved.is_narrowed);
    assert!(!resolved.is_retest_pending);
    assert!(resolved.narrowing_note.is_none());
    // The support class is carried as its own field, unchanged.
    assert_eq!(resolved.support_class, M5SupportClassBadgeValue::Certified);
}

#[test]
fn resolver_stale_and_imported_evidence_narrow_and_preserve_support_class() {
    for (freshness, reason, action) in [
        (
            M5EvidenceFreshnessValue::EvidenceStale,
            M5FreshnessReducesClaimReason::EvidenceStale,
            M5BadgeNextAction::RefreshEvidence,
        ),
        (
            M5EvidenceFreshnessValue::ImportedEvidence,
            M5FreshnessReducesClaimReason::ImportedEvidence,
            M5BadgeNextAction::ReverifyImportedEvidence,
        ),
    ] {
        let resolved = resolve_badge_claim(&input(M5SupportClassBadgeValue::Certified, freshness))
            .expect("resolves");
        assert!(resolved.is_narrowed, "{} should narrow", freshness.as_str());
        assert!(!resolved.is_current);
        let note = resolved.narrowing_note.expect("narrowing note present");
        assert_eq!(note.reason, reason);
        assert_eq!(note.next_action, action);
        assert!(note.narrows_claim);
        // AC2: the underlying support-class context is preserved, not dropped.
        assert_eq!(
            note.preserved_support_class,
            M5SupportClassBadgeValue::Certified
        );
        assert!(!note.headline.trim().is_empty());
        assert!(note.headline.to_lowercase().contains("certified"));
    }
}

#[test]
fn resolver_retest_pending_flags_but_does_not_narrow() {
    let resolved = resolve_badge_claim(&input(
        M5SupportClassBadgeValue::Limited,
        M5EvidenceFreshnessValue::RetestPending,
    ))
    .expect("resolves");
    assert_eq!(
        resolved.effective_claim,
        M5EffectiveClaimPosture::ClaimRetestPending
    );
    assert!(resolved.is_retest_pending);
    assert!(!resolved.is_narrowed);
    let note = resolved.narrowing_note.expect("note present");
    assert_eq!(note.reason, M5FreshnessReducesClaimReason::RetestPending);
    assert_eq!(note.next_action, M5BadgeNextAction::AwaitRetest);
    assert!(!note.narrows_claim);
}

#[test]
fn resolver_freshness_is_independent_of_support_class() {
    // The same freshness value produces the same effective claim regardless of support
    // class: freshness is never derived from support class, and vice versa.
    for support in M5SupportClassBadgeValue::ALL {
        let stale = resolve_badge_claim(&input(support, M5EvidenceFreshnessValue::EvidenceStale))
            .expect("resolves");
        assert_eq!(
            stale.effective_claim,
            M5EffectiveClaimPosture::ClaimNarrowedEvidenceStale,
            "support class {} changed the freshness verdict",
            support.as_str()
        );
        assert_eq!(stale.support_class, support);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_label = M5BadgeClaimInput {
        subject_label: "  ".to_owned(),
        ..input(
            M5SupportClassBadgeValue::Supported,
            M5EvidenceFreshnessValue::Fresh,
        )
    };
    assert_eq!(
        resolve_badge_claim(&empty_label),
        Err(M5BadgeClaimError::EmptySubjectLabel)
    );

    let empty_source = M5BadgeClaimInput {
        evidence_source_repr: "".to_owned(),
        ..input(
            M5SupportClassBadgeValue::Supported,
            M5EvidenceFreshnessValue::Fresh,
        )
    };
    assert_eq!(
        resolve_badge_claim(&empty_source),
        Err(M5BadgeClaimError::EmptyEvidenceSource)
    );

    let empty_ts = M5BadgeClaimInput {
        last_evaluated_repr: "   ".to_owned(),
        ..input(
            M5SupportClassBadgeValue::Supported,
            M5EvidenceFreshnessValue::Fresh,
        )
    };
    assert_eq!(
        resolve_badge_claim(&empty_ts),
        Err(M5BadgeClaimError::EmptyLastEvaluated)
    );

    let forbidden = M5BadgeClaimInput {
        evidence_source_repr: "https://example.test/evidence".to_owned(),
        ..input(
            M5SupportClassBadgeValue::Supported,
            M5EvidenceFreshnessValue::Fresh,
        )
    };
    assert_eq!(
        resolve_badge_claim(&forbidden),
        Err(M5BadgeClaimError::ForbiddenBadgeMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_badge_claim_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BADGE_CLAIM_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_badge_claim_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .badge_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5BadgeClaimConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.badge_rows.len(),
        M5BadgeClaimConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_explanation() {
    let packet = seeded_m5_badge_claim_primitive_packet();
    for row in &packet.badge_rows {
        for part in M5BadgeClaimAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5BadgeClaimExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for field in M5BadgeExplanationField::MANDATORY {
            assert!(row.explanation_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable));
        assert!(row
            .accessibility_routes
            .contains(&M5BadgeAccessibilityRoute::NonColorEncoded));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_badge_claim_primitive_packet();
    let cases: Vec<&M5BadgeClaimResolutionCase> = packet
        .badge_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for support in M5SupportClassBadgeValue::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.support_class == support),
            "no worked resolution exercises support class {}",
            support.as_str()
        );
    }
    for freshness in M5EvidenceFreshnessValue::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.freshness == freshness),
            "no worked resolution exercises freshness {}",
            freshness.as_str()
        );
    }
    for posture in M5EffectiveClaimPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.effective_claim == posture),
            "no worked resolution exercises effective claim {}",
            posture.as_str()
        );
    }
    for reason in M5FreshnessReducesClaimReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .narrowing_note
                .as_ref()
                .is_some_and(|n| n.reason == reason)),
            "no worked resolution exercises narrowing reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_badge_claim_primitive_packet();
    for row in &packet.badge_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet
        .badge_rows
        .retain(|row| row.consumer_surface != M5BadgeClaimConsumerSurface::DiagnosticsReport);
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.vocabulary_set.support_class_values.pop();
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.badge_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5BadgeClaimAnatomyPart::FreshnessExplanationDrawer);
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.badge_rows[0]
        .export_fields
        .retain(|f| *f != M5BadgeClaimExportField::Freshness);
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn explanation_drawer_incomplete_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.badge_rows[0]
        .explanation_fields
        .retain(|f| *f != M5BadgeExplanationField::WhatItMeans);
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::ExplanationDrawerIncomplete));
}

#[test]
fn non_color_encoding_missing_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.badge_rows[0]
        .accessibility_routes
        .retain(|r| *r != M5BadgeAccessibilityRoute::NonColorEncoded);
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.badge_rows[0].example_resolutions[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn distinct_cues_unproven_fails_when_no_high_support_narrowed_example() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    // Replace every example with a clean current one so the distinct-cues lint fires.
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![M5BadgeClaimResolutionCase::resolved(input(
            M5SupportClassBadgeValue::Certified,
            M5EvidenceFreshnessValue::Fresh,
        ))];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5BadgeClaimPrimitiveViolation::DistinctCuesUnproven));
    assert!(violations.contains(&M5BadgeClaimPrimitiveViolation::FreshAndNarrowedCoverageUnproven));
}

#[test]
fn context_preservation_unproven_fails_when_no_narrowed_example() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![
            M5BadgeClaimResolutionCase::resolved(input(
                M5SupportClassBadgeValue::Certified,
                M5EvidenceFreshnessValue::Fresh,
            )),
            M5BadgeClaimResolutionCase::resolved(input(
                M5SupportClassBadgeValue::Limited,
                M5EvidenceFreshnessValue::RetestPending,
            )),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::ContextPreservationUnproven));
}

#[test]
fn badge_invariant_violation_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.badge_rows[0].collapses_support_and_freshness_into_one_badge = true;
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::BadgeInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.badge_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet
        .governance_review
        .narrowing_preserves_support_class_context = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet
        .consumer_projection
        .freshness_filter_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeClaimPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_badge_claim_primitive_packet().render_markdown_summary();
    for surface in M5BadgeClaimConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_badge_claim_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5BadgeClaimConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5BadgeClaimConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_badge_claim_primitive_export()
        .expect("checked M5 badge-claim primitive export validates");
    assert_eq!(from_disk.packet_id, M5_BADGE_CLAIM_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_badge_claim_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_badge_claim_primitive_marketplace_listing_beta_narrowed(),
        seeded_m5_badge_claim_primitive_certification_record_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.badge_rows.len(),
            M5BadgeClaimConsumerSurface::ALL.len()
        );
    }

    let marketplace = seeded_m5_badge_claim_primitive_marketplace_listing_beta_narrowed();
    let row = marketplace
        .badge_rows
        .iter()
        .find(|r| r.consumer_surface == M5BadgeClaimConsumerSurface::MarketplaceListing)
        .expect("marketplace row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Beta);

    let certification = seeded_m5_badge_claim_primitive_certification_record_preview_narrowed();
    let row = certification
        .badge_rows
        .iter()
        .find(|r| r.consumer_surface == M5BadgeClaimConsumerSurface::CertificationRecord)
        .expect("certification row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let marketplace: M5BadgeClaimPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-class-and-evidence-freshness-badges/marketplace_listing_beta_narrowed.json"
    )))
    .expect("marketplace fixture parses");
    assert!(marketplace.validate().is_empty());
    assert_eq!(
        marketplace,
        seeded_m5_badge_claim_primitive_marketplace_listing_beta_narrowed()
    );

    let certification: M5BadgeClaimPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-support-class-and-evidence-freshness-badges/certification_record_preview_narrowed.json"
    )))
    .expect("certification fixture parses");
    assert!(certification.validate().is_empty());
    assert_eq!(
        certification,
        seeded_m5_badge_claim_primitive_certification_record_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_badge_claim_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
