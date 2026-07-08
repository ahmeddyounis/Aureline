use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_badge_family_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BADGE_FAMILY_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_badge_family() {
    let packet = seeded_m5_badge_family_matrix();
    let present: std::collections::BTreeSet<_> =
        packet.badge_rows.iter().map(|r| r.badge_family).collect();
    for family in M5BadgeFamily::ALL {
        assert!(
            present.contains(&family),
            "missing badge family {}",
            family.as_str()
        );
    }
    assert_eq!(packet.badge_rows.len(), M5BadgeFamily::ALL.len());
}

#[test]
fn every_badge_declares_mandatory_labels_explanation_and_deployment_lines() {
    let packet = seeded_m5_badge_family_matrix();
    for row in &packet.badge_rows {
        for label in M5BadgeRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "badge {} missing mandatory label {}",
                row.badge_family.as_str(),
                label.as_str()
            );
        }
        for field in M5BadgeExplanationField::MANDATORY {
            assert!(
                row.explanation_fields.contains(&field),
                "badge {} missing mandatory explanation field {}",
                row.badge_family.as_str(),
                field.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable));
        assert!(row
            .accessibility_routes
            .contains(&M5BadgeAccessibilityRoute::NonColorEncoded));
    }
}

#[test]
fn family_specific_values_are_declared_only_where_applicable() {
    let packet = seeded_m5_badge_family_matrix();
    for row in &packet.badge_rows {
        let family = row.badge_family;
        assert_eq!(
            !row.support_class_values.is_empty(),
            family.is_support_class(),
            "support_class_values presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.evidence_freshness_values.is_empty(),
            family.is_evidence_freshness(),
            "evidence_freshness_values presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.lifecycle_values.is_empty(),
            family.is_lifecycle(),
            "lifecycle_values presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.channel_values.is_empty(),
            family.is_channel(),
            "channel_values presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.deployment_scope_values.is_empty(),
            family.is_deployment_scope(),
            "deployment_scope_values presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.compatibility_state_values.is_empty(),
            family.is_compatibility_state(),
            "compatibility_state_values presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_value_token_is_declared_by_some_badge() {
    let packet = seeded_m5_badge_family_matrix();
    for value in M5SupportClassBadge::ALL {
        assert!(
            packet
                .badge_rows
                .iter()
                .any(|row| row.support_class_values.contains(&value)),
            "no badge declares support class value {}",
            value.as_str()
        );
    }
    for value in M5EvidenceFreshnessBadge::ALL {
        assert!(
            packet
                .badge_rows
                .iter()
                .any(|row| row.evidence_freshness_values.contains(&value)),
            "no badge declares evidence freshness value {}",
            value.as_str()
        );
    }
    for value in M5LifecycleBadge::ALL {
        assert!(
            packet
                .badge_rows
                .iter()
                .any(|row| row.lifecycle_values.contains(&value)),
            "no badge declares lifecycle value {}",
            value.as_str()
        );
    }
    for value in M5ChannelBadge::ALL {
        assert!(
            packet
                .badge_rows
                .iter()
                .any(|row| row.channel_values.contains(&value)),
            "no badge declares channel value {}",
            value.as_str()
        );
    }
    for value in M5DeploymentScopeBadge::ALL {
        assert!(
            packet
                .badge_rows
                .iter()
                .any(|row| row.deployment_scope_values.contains(&value)),
            "no badge declares deployment scope value {}",
            value.as_str()
        );
    }
    for value in M5CompatibilityStateBadge::ALL {
        assert!(
            packet
                .badge_rows
                .iter()
                .any(|row| row.compatibility_state_values.contains(&value)),
            "no badge declares compatibility state value {}",
            value.as_str()
        );
    }
}

#[test]
fn axis_separation_rules_are_canonical() {
    let packet = seeded_m5_badge_family_matrix();
    let expected: Vec<String> = M5BadgeAxisSeparationRule::ALL
        .iter()
        .map(|rule| rule.as_str().to_owned())
        .collect();
    assert_eq!(packet.axis_separation_rules, expected);
    assert!(packet
        .axis_separation_rules
        .contains(&"support_class_does_not_imply_freshness".to_owned()));
    assert!(packet
        .axis_separation_rules
        .contains(&"deployment_scope_does_not_imply_lifecycle".to_owned()));
}

#[test]
fn missing_badge_family_fails_validation() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet
        .badge_rows
        .retain(|row| row.badge_family != M5BadgeFamily::Channel);
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::RequiredBadgeFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.vocabulary_set.support_class_values.pop();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::VocabularySetDrift));
}

#[test]
fn axis_separation_rules_drift_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.axis_separation_rules.pop();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::AxisSeparationRulesDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.badge_rows[0]
        .required_labels
        .retain(|label| *label != M5BadgeRequiredLabel::AxisName);
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn explanation_drawer_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.badge_rows[0]
        .explanation_fields
        .retain(|field| *field != M5BadgeExplanationField::WhatItMeans);
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::ExplanationDrawerIncomplete));
}

#[test]
fn support_class_values_missing_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::SupportClass)
        .expect("support class badge present");
    row.support_class_values.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::SupportClassValueMissing));
}

#[test]
fn evidence_freshness_values_missing_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::EvidenceFreshness)
        .expect("evidence freshness badge present");
    row.evidence_freshness_values.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::EvidenceFreshnessValueMissing));
}

#[test]
fn lifecycle_values_missing_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::Lifecycle)
        .expect("lifecycle badge present");
    row.lifecycle_values.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::LifecycleValueMissing));
}

#[test]
fn channel_values_missing_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::Channel)
        .expect("channel badge present");
    row.channel_values.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::ChannelValueMissing));
}

#[test]
fn deployment_scope_values_missing_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::DeploymentScope)
        .expect("deployment scope badge present");
    row.deployment_scope_values.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::DeploymentScopeValueMissing));
}

#[test]
fn compatibility_state_values_missing_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::CompatibilityState)
        .expect("compatibility state badge present");
    row.compatibility_state_values.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::CompatibilityStateValueMissing));
}

#[test]
fn badge_invariant_violation_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.badge_rows[0].collapses_multiple_axes_into_one_pill = true;
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::BadgeInvariantViolated));

    let mut packet = seeded_m5_badge_family_matrix();
    packet.badge_rows[0].implies_freshness_from_support_class = true;
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::BadgeInvariantViolated));

    let mut packet = seeded_m5_badge_family_matrix();
    packet.badge_rows[4].implies_lifecycle_from_deployment_scope = true;
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::BadgeInvariantViolated));

    let mut packet = seeded_m5_badge_family_matrix();
    packet.badge_rows[1].drops_badge_meaning_in_export = true;
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::BadgeInvariantViolated));
}

#[test]
fn stable_badge_missing_proof_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.badge_family == M5BadgeFamily::SupportClass)
        .expect("support class badge present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::StableBadgeMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.badge_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.badge_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet
        .governance_review
        .support_class_never_implies_freshness = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet
        .consumer_projection
        .filters_read_single_source_per_axis = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_badge_family_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BadgeFamilyMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_badge_family() {
    let summary = seeded_m5_badge_family_matrix().render_markdown_summary();
    for family in M5BadgeFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing badge {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_badge_family() {
    let csv = seeded_m5_badge_family_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5BadgeFamily::ALL.len());
    assert!(lines[0].starts_with("badge_family,qualification,owner,"));
    for family in M5BadgeFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing badge {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_badge_family_matrix_export()
        .expect("checked M5 badge family matrix export validates");
    assert_eq!(packet.packet_id, M5_BADGE_FAMILY_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_badge_family_matrix_export()
        .expect("checked M5 badge family matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_badge_family_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_badge_family_matrix_channel_badge_beta_narrowed(),
        seeded_m5_badge_family_matrix_compatibility_state_badge_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.badge_rows.len(), M5BadgeFamily::ALL.len());
    }

    let channel = seeded_m5_badge_family_matrix_channel_badge_beta_narrowed();
    let row = channel
        .badge_rows
        .iter()
        .find(|r| r.badge_family == M5BadgeFamily::Channel)
        .expect("channel badge row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Beta);

    let compat = seeded_m5_badge_family_matrix_compatibility_state_badge_preview_narrowed();
    let row = compat
        .badge_rows
        .iter()
        .find(|r| r.badge_family == M5BadgeFamily::CompatibilityState)
        .expect("compatibility-state badge row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let channel: M5BadgeFamilyMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-badge-family-consumers/channel_badge_beta_narrowed.json"
    )))
    .expect("channel fixture parses");
    assert!(channel.validate().is_empty());
    assert_eq!(
        channel,
        seeded_m5_badge_family_matrix_channel_badge_beta_narrowed()
    );

    let compat: M5BadgeFamilyMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-badge-family-consumers/compatibility_state_badge_preview_narrowed.json"
    )))
    .expect("compatibility fixture parses");
    assert!(compat.validate().is_empty());
    assert_eq!(
        compat,
        seeded_m5_badge_family_matrix_compatibility_state_badge_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_badge_family_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
