use super::*;

fn input(state: M5CompatibilityStateBadgeValue) -> M5CompatibilityStateBadgeInput {
    M5CompatibilityStateBadgeInput {
        subject_label: "aureline artifact: sample".to_owned(),
        state,
        reconciliation_detail_repr: None,
        last_evaluated_repr: "2026-07-01T00:00:00Z".to_owned(),
    }
}

fn reconciliation_input(
    state: M5CompatibilityStateBadgeValue,
    detail: &str,
) -> M5CompatibilityStateBadgeInput {
    M5CompatibilityStateBadgeInput {
        reconciliation_detail_repr: Some(detail.to_owned()),
        ..input(state)
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_exact_match_is_full_parity_with_no_note() {
    let resolved =
        resolve_compatibility_state_badge(&input(M5CompatibilityStateBadgeValue::ExactMatch))
            .expect("resolves");
    assert_eq!(
        resolved.compatibility_posture,
        M5CompatibilityPosture::FullParity
    );
    assert!(resolved.is_full_parity);
    assert!(!resolved.is_compatible_within_range);
    assert!(!resolved.requires_reconciliation);
    assert!(!resolved.is_reduced_capability);
    assert!(!resolved.is_hard_mismatch);
    assert!(resolved.reconciliation_note.is_none());
    assert_eq!(resolved.state, M5CompatibilityStateBadgeValue::ExactMatch);
}

#[test]
fn resolver_compatible_is_within_range_with_no_note() {
    let resolved =
        resolve_compatibility_state_badge(&input(M5CompatibilityStateBadgeValue::Compatible))
            .expect("resolves");
    assert_eq!(
        resolved.compatibility_posture,
        M5CompatibilityPosture::CompatibleWithinRange
    );
    assert!(resolved.is_compatible_within_range);
    assert!(!resolved.requires_reconciliation);
    assert!(resolved.reconciliation_note.is_none());
}

#[test]
fn resolver_limited_and_mismatch_disclose_reconciliation_and_preserve_state() {
    for (state, posture, class, action, residual, reduced, mismatch) in [
        (
            M5CompatibilityStateBadgeValue::Limited,
            M5CompatibilityPosture::ReducedCapability,
            M5CompatibilityGapClass::CapabilitySubsetReduced,
            M5CompatibilityRepairAction::CompareAndReviewReducedScope,
            M5CompatibilityResidualCapability::ContinuesWithReducedScope,
            true,
            false,
        ),
        (
            M5CompatibilityStateBadgeValue::Mismatch,
            M5CompatibilityPosture::IncompatibleAsClaimed,
            M5CompatibilityGapClass::VersionOrSchemaMismatch,
            M5CompatibilityRepairAction::RepairBeforeApply,
            M5CompatibilityResidualCapability::BlockedUntilReconciled,
            false,
            true,
        ),
    ] {
        let resolved =
            resolve_compatibility_state_badge(&reconciliation_input(state, "gap:token/example"))
                .expect("resolves");
        assert_eq!(
            resolved.compatibility_posture,
            posture,
            "{}",
            state.as_str()
        );
        assert!(resolved.requires_reconciliation, "{}", state.as_str());
        assert!(!resolved.is_full_parity);
        assert_eq!(
            resolved.is_reduced_capability,
            reduced,
            "{}",
            state.as_str()
        );
        assert_eq!(resolved.is_hard_mismatch, mismatch, "{}", state.as_str());
        let note = resolved
            .reconciliation_note
            .expect("reconciliation note present");
        assert_eq!(note.gap_class, class);
        assert_eq!(note.repair_action, action);
        assert_eq!(note.residual_capability, residual);
        // AC2: the badge names exactly what differs.
        assert_eq!(note.reconciliation_detail, "gap:token/example");
        // The underlying state context is preserved, not dropped.
        assert_eq!(note.preserved_state, state);
        assert!(!note.headline.trim().is_empty());
        assert!(note.headline.contains(state.as_str()));
    }
}

#[test]
fn resolver_rejects_reconciliation_state_without_detail() {
    // AC2: a Limited/Mismatch badge must never collapse into a generic warning — the
    // resolver refuses to build one without a reconciliation detail.
    let missing = input(M5CompatibilityStateBadgeValue::Mismatch);
    assert_eq!(
        resolve_compatibility_state_badge(&missing),
        Err(M5CompatibilityStateBadgeError::MissingReconciliationDetail)
    );

    let blank = reconciliation_input(M5CompatibilityStateBadgeValue::Limited, "   ");
    assert_eq!(
        resolve_compatibility_state_badge(&blank),
        Err(M5CompatibilityStateBadgeError::MissingReconciliationDetail)
    );
}

#[test]
fn resolver_posture_is_derived_from_state_alone() {
    // Each state maps to exactly one posture, and the posture never depends on anything
    // outside the state axis.
    for state in M5CompatibilityStateBadgeValue::ALL {
        let detail = match state {
            M5CompatibilityStateBadgeValue::Limited | M5CompatibilityStateBadgeValue::Mismatch => {
                Some("gap:token/example")
            }
            _ => None,
        };
        let resolved = resolve_compatibility_state_badge(&M5CompatibilityStateBadgeInput {
            reconciliation_detail_repr: detail.map(str::to_owned),
            ..input(state)
        })
        .expect("resolves");
        assert_eq!(resolved.state, state);
        // Reconciliation required exactly for Limited and Mismatch.
        let expects_reconciliation = matches!(
            state,
            M5CompatibilityStateBadgeValue::Limited | M5CompatibilityStateBadgeValue::Mismatch
        );
        assert_eq!(resolved.requires_reconciliation, expects_reconciliation);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_label = M5CompatibilityStateBadgeInput {
        subject_label: "  ".to_owned(),
        ..input(M5CompatibilityStateBadgeValue::ExactMatch)
    };
    assert_eq!(
        resolve_compatibility_state_badge(&empty_label),
        Err(M5CompatibilityStateBadgeError::EmptySubjectLabel)
    );

    let empty_ts = M5CompatibilityStateBadgeInput {
        last_evaluated_repr: "   ".to_owned(),
        ..input(M5CompatibilityStateBadgeValue::ExactMatch)
    };
    assert_eq!(
        resolve_compatibility_state_badge(&empty_ts),
        Err(M5CompatibilityStateBadgeError::EmptyLastEvaluated)
    );

    let forbidden = reconciliation_input(
        M5CompatibilityStateBadgeValue::Mismatch,
        "https://example.test/gap",
    );
    assert_eq!(
        resolve_compatibility_state_badge(&forbidden),
        Err(M5CompatibilityStateBadgeError::ForbiddenBadgeMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_compatibility_state_badge_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_compatibility_state_badge_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .badge_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5CompatibilityStateConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.badge_rows.len(),
        M5CompatibilityStateConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_explanation() {
    let packet = seeded_m5_compatibility_state_badge_primitive_packet();
    for row in &packet.badge_rows {
        for part in M5CompatibilityStateAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5CompatibilityStateExportField::MANDATORY {
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
    let packet = seeded_m5_compatibility_state_badge_primitive_packet();
    let cases: Vec<&M5CompatibilityStateResolutionCase> = packet
        .badge_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for state in M5CompatibilityStateBadgeValue::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.state == state),
            "no worked resolution exercises state {}",
            state.as_str()
        );
    }
    for posture in M5CompatibilityPosture::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.compatibility_posture == posture),
            "no worked resolution exercises posture {}",
            posture.as_str()
        );
    }
    for class in M5CompatibilityGapClass::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .reconciliation_note
                .as_ref()
                .is_some_and(|n| n.gap_class == class)),
            "no worked resolution exercises gap class {}",
            class.as_str()
        );
    }
    for residual in M5CompatibilityResidualCapability::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .reconciliation_note
                .as_ref()
                .is_some_and(|n| n.residual_capability == residual)),
            "no worked resolution exercises residual capability {}",
            residual.as_str()
        );
    }
    for action in M5CompatibilityRepairAction::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .reconciliation_note
                .as_ref()
                .is_some_and(|n| n.repair_action == action)),
            "no worked resolution exercises repair action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_compatibility_state_badge_primitive_packet();
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
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.badge_rows.retain(|row| {
        row.consumer_surface != M5CompatibilityStateConsumerSurface::ExtensionImportRow
    });
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.vocabulary_set.state_values.pop();
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.badge_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5CompatibilityStateAnatomyPart::ReconciliationDrawer);
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.badge_rows[0]
        .export_fields
        .retain(|f| *f != M5CompatibilityStateExportField::ReconciliationDetail);
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn explanation_drawer_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.badge_rows[0]
        .explanation_fields
        .retain(|f| *f != M5BadgeExplanationField::WhatItMeans);
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::ExplanationDrawerIncomplete));
}

#[test]
fn non_color_encoding_missing_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.badge_rows[0]
        .accessibility_routes
        .retain(|r| *r != M5BadgeAccessibilityRoute::NonColorEncoded);
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.badge_rows[0].example_resolutions[0]
        .resolved
        .is_full_parity = false;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn preflight_posture_disclosure_unproven_fails_without_both_ends() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    // Replace every example with an exact-match (parity-clean) one so no Limited/Mismatch
    // example remains and neither reconciliation preservation nor limited/mismatch coverage
    // is proven.
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![M5CompatibilityStateResolutionCase::resolved(input(
            M5CompatibilityStateBadgeValue::ExactMatch,
        ))];
    }
    let violations = packet.validate();
    assert!(violations.contains(
        &M5CompatibilityStateBadgePrimitiveViolation::PreflightPostureDisclosureUnproven
    ));
    assert!(violations.contains(
        &M5CompatibilityStateBadgePrimitiveViolation::LimitedAndMismatchCoverageUnproven
    ));
}

#[test]
fn repair_compare_detail_preservation_unproven_fails_when_no_reconciliation_example() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![M5CompatibilityStateResolutionCase::resolved(input(
            M5CompatibilityStateBadgeValue::ExactMatch,
        ))];
    }
    assert!(packet.validate().contains(
        &M5CompatibilityStateBadgePrimitiveViolation::RepairCompareDetailPreservationUnproven
    ));
}

#[test]
fn limited_and_mismatch_coverage_unproven_fails_without_mismatch() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    // Only exact-match + limited examples: parity-clean is proven and reconciliation
    // preservation holds, but the Mismatch reading is absent.
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![
            M5CompatibilityStateResolutionCase::resolved(input(
                M5CompatibilityStateBadgeValue::ExactMatch,
            )),
            M5CompatibilityStateResolutionCase::resolved(reconciliation_input(
                M5CompatibilityStateBadgeValue::Limited,
                "capability:subset-4of6/skips-remote-eval",
            )),
        ];
    }
    let violations = packet.validate();
    assert!(violations.contains(
        &M5CompatibilityStateBadgePrimitiveViolation::LimitedAndMismatchCoverageUnproven
    ));
    // Preflight disclosure and reconciliation preservation still hold.
    assert!(!violations.contains(
        &M5CompatibilityStateBadgePrimitiveViolation::PreflightPostureDisclosureUnproven
    ));
    assert!(!violations.contains(
        &M5CompatibilityStateBadgePrimitiveViolation::RepairCompareDetailPreservationUnproven
    ));
}

#[test]
fn badge_invariant_violation_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.badge_rows[0].collapses_state_into_support_lifecycle_or_channel = true;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::BadgeInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.badge_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet
        .governance_review
        .mismatch_auto_discloses_reconciliation_detail = false;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet
        .consumer_projection
        .compatibility_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_compatibility_state_badge_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CompatibilityStateBadgePrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_compatibility_state_badge_primitive_packet().render_markdown_summary();
    for surface in M5CompatibilityStateConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_compatibility_state_badge_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5CompatibilityStateConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5CompatibilityStateConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_compatibility_state_badge_primitive_export()
        .expect("checked M5 compatibility state badge primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_compatibility_state_badge_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_compatibility_state_badge_primitive_compare_review_panel_beta_narrowed(),
        seeded_m5_compatibility_state_badge_primitive_support_export_row_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.badge_rows.len(),
            M5CompatibilityStateConsumerSurface::ALL.len()
        );
    }

    let compare =
        seeded_m5_compatibility_state_badge_primitive_compare_review_panel_beta_narrowed();
    let row = compare
        .badge_rows
        .iter()
        .find(|r| r.consumer_surface == M5CompatibilityStateConsumerSurface::CompareReviewPanel)
        .expect("compare row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Beta);

    let support =
        seeded_m5_compatibility_state_badge_primitive_support_export_row_preview_narrowed();
    let row = support
        .badge_rows
        .iter()
        .find(|r| r.consumer_surface == M5CompatibilityStateConsumerSurface::SupportExportRow)
        .expect("support export row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let compare: M5CompatibilityStateBadgePrimitivePacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-compatibility-state-badges/compare_review_panel_beta_narrowed.json"
        )))
        .expect("compare fixture parses");
    assert!(compare.validate().is_empty());
    assert_eq!(
        compare,
        seeded_m5_compatibility_state_badge_primitive_compare_review_panel_beta_narrowed()
    );

    let support: M5CompatibilityStateBadgePrimitivePacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-compatibility-state-badges/support_export_row_preview_narrowed.json"
    )))
        .expect("support export fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_compatibility_state_badge_primitive_support_export_row_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_compatibility_state_badge_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
