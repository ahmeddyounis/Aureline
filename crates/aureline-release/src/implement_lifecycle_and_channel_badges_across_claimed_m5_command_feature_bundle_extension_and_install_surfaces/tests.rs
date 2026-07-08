use super::*;

fn input(
    lifecycle: M5LifecycleBadgeValue,
    channel: M5ChannelBadgeValue,
) -> M5LifecycleChannelBadgeInput {
    M5LifecycleChannelBadgeInput {
        subject_label: "aureline capability: sample".to_owned(),
        lifecycle,
        channel,
        replacement_path_repr: None,
        last_evaluated_repr: "2026-07-01T00:00:00Z".to_owned(),
    }
}

fn sunsetting_input(
    lifecycle: M5LifecycleBadgeValue,
    channel: M5ChannelBadgeValue,
    replacement: &str,
) -> M5LifecycleChannelBadgeInput {
    M5LifecycleChannelBadgeInput {
        replacement_path_repr: Some(replacement.to_owned()),
        ..input(lifecycle, channel)
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_stable_lifecycle_is_stable_line_with_no_note() {
    let resolved = resolve_lifecycle_channel_badge(&input(
        M5LifecycleBadgeValue::Stable,
        M5ChannelBadgeValue::Stable,
    ))
    .expect("resolves");
    assert_eq!(
        resolved.effective_maturity,
        M5EffectiveMaturityPosture::MaturityStable
    );
    assert!(resolved.is_stable_line);
    assert!(!resolved.is_sunsetting);
    assert!(!resolved.is_prerelease);
    assert!(resolved.migration_note.is_none());
    // The channel is carried as its own field, unchanged.
    assert_eq!(resolved.channel, M5ChannelBadgeValue::Stable);
}

#[test]
fn resolver_deprecated_and_removal_scheduled_point_to_migration_and_preserve_channel() {
    for (lifecycle, reason, action, removal) in [
        (
            M5LifecycleBadgeValue::Deprecated,
            M5LifecycleSunsetReason::Deprecated,
            M5MaturityBadgeNextAction::FollowMigrationPath,
            false,
        ),
        (
            M5LifecycleBadgeValue::RemovalScheduled,
            M5LifecycleSunsetReason::RemovalScheduled,
            M5MaturityBadgeNextAction::CompleteMigrationBeforeRemoval,
            true,
        ),
    ] {
        let resolved = resolve_lifecycle_channel_badge(&sunsetting_input(
            lifecycle,
            M5ChannelBadgeValue::Beta,
            "migration:command/replacement",
        ))
        .expect("resolves");
        assert!(
            resolved.is_sunsetting,
            "{} should sunset",
            lifecycle.as_str()
        );
        assert!(!resolved.is_stable_line);
        let note = resolved.migration_note.expect("migration note present");
        assert_eq!(note.reason, reason);
        assert_eq!(note.next_action, action);
        assert_eq!(note.is_removal_scheduled, removal);
        // AC2: the badge points to a real replacement path, not an inert warning.
        assert_eq!(note.replacement_path, "migration:command/replacement");
        // The underlying channel context is preserved, not dropped.
        assert_eq!(note.preserved_channel, M5ChannelBadgeValue::Beta);
        assert!(!note.headline.trim().is_empty());
        assert!(note.headline.to_lowercase().contains("beta"));
    }
}

#[test]
fn resolver_rejects_sunsetting_without_replacement_path() {
    // AC2: a deprecated badge must never be an inert warning — the resolver refuses to
    // build one without a replacement/migration path.
    let missing = input(
        M5LifecycleBadgeValue::Deprecated,
        M5ChannelBadgeValue::Stable,
    );
    assert_eq!(
        resolve_lifecycle_channel_badge(&missing),
        Err(M5LifecycleChannelBadgeError::MissingReplacementPath)
    );

    let blank = sunsetting_input(
        M5LifecycleBadgeValue::RemovalScheduled,
        M5ChannelBadgeValue::Stable,
        "   ",
    );
    assert_eq!(
        resolve_lifecycle_channel_badge(&blank),
        Err(M5LifecycleChannelBadgeError::MissingReplacementPath)
    );
}

#[test]
fn resolver_maturity_is_independent_of_channel() {
    // The same lifecycle produces the same effective maturity regardless of channel:
    // maturity is never derived from the channel, and vice versa.
    for channel in M5ChannelBadgeValue::ALL {
        let stable =
            resolve_lifecycle_channel_badge(&input(M5LifecycleBadgeValue::Stable, channel))
                .expect("resolves");
        assert_eq!(
            stable.effective_maturity,
            M5EffectiveMaturityPosture::MaturityStable,
            "channel {} changed the maturity verdict",
            channel.as_str()
        );
        assert_eq!(stable.channel, channel);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_label = M5LifecycleChannelBadgeInput {
        subject_label: "  ".to_owned(),
        ..input(M5LifecycleBadgeValue::Stable, M5ChannelBadgeValue::Stable)
    };
    assert_eq!(
        resolve_lifecycle_channel_badge(&empty_label),
        Err(M5LifecycleChannelBadgeError::EmptySubjectLabel)
    );

    let empty_ts = M5LifecycleChannelBadgeInput {
        last_evaluated_repr: "   ".to_owned(),
        ..input(M5LifecycleBadgeValue::Stable, M5ChannelBadgeValue::Stable)
    };
    assert_eq!(
        resolve_lifecycle_channel_badge(&empty_ts),
        Err(M5LifecycleChannelBadgeError::EmptyLastEvaluated)
    );

    let forbidden = sunsetting_input(
        M5LifecycleBadgeValue::Deprecated,
        M5ChannelBadgeValue::Stable,
        "https://example.test/migration",
    );
    assert_eq!(
        resolve_lifecycle_channel_badge(&forbidden),
        Err(M5LifecycleChannelBadgeError::ForbiddenBadgeMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_maturity_badge_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_MATURITY_BADGE_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_maturity_badge_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .badge_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5MaturityBadgeConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.badge_rows.len(),
        M5MaturityBadgeConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_explanation() {
    let packet = seeded_m5_maturity_badge_primitive_packet();
    for row in &packet.badge_rows {
        for part in M5MaturityBadgeAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5MaturityBadgeExportField::MANDATORY {
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
    let packet = seeded_m5_maturity_badge_primitive_packet();
    let cases: Vec<&M5LifecycleChannelResolutionCase> = packet
        .badge_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for lifecycle in M5LifecycleBadgeValue::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.lifecycle == lifecycle),
            "no worked resolution exercises lifecycle {}",
            lifecycle.as_str()
        );
    }
    for channel in M5ChannelBadgeValue::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.channel == channel),
            "no worked resolution exercises channel {}",
            channel.as_str()
        );
    }
    for posture in M5EffectiveMaturityPosture::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.effective_maturity == posture),
            "no worked resolution exercises effective maturity {}",
            posture.as_str()
        );
    }
    for reason in M5LifecycleSunsetReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .migration_note
                .as_ref()
                .is_some_and(|n| n.reason == reason)),
            "no worked resolution exercises sunset reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_maturity_badge_primitive_packet();
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
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet
        .badge_rows
        .retain(|row| row.consumer_surface != M5MaturityBadgeConsumerSurface::WorkflowBundle);
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.vocabulary_set.lifecycle_values.pop();
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.badge_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5MaturityBadgeAnatomyPart::ChannelExplanationDrawer);
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.badge_rows[0]
        .export_fields
        .retain(|f| *f != M5MaturityBadgeExportField::ReplacementPath);
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn explanation_drawer_incomplete_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.badge_rows[0]
        .explanation_fields
        .retain(|f| *f != M5BadgeExplanationField::WhatItMeans);
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::ExplanationDrawerIncomplete));
}

#[test]
fn non_color_encoding_missing_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.badge_rows[0]
        .accessibility_routes
        .retain(|r| *r != M5BadgeAccessibilityRoute::NonColorEncoded);
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.badge_rows[0].example_resolutions[0]
        .resolved
        .is_sunsetting = true;
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn distinction_unproven_fails_when_channel_always_matches_lifecycle() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    // Replace every example with a stable-on-stable one so neither distinction nor
    // sunsetting coverage is proven.
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![M5LifecycleChannelResolutionCase::resolved(input(
            M5LifecycleBadgeValue::Stable,
            M5ChannelBadgeValue::Stable,
        ))];
    }
    let violations = packet.validate();
    assert!(violations
        .contains(&M5MaturityBadgePrimitiveViolation::LifecycleChannelDistinctionUnproven));
    assert!(violations
        .contains(&M5MaturityBadgePrimitiveViolation::StableAndSunsettingCoverageUnproven));
}

#[test]
fn migration_path_preservation_unproven_fails_when_no_sunsetting_example() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![
            M5LifecycleChannelResolutionCase::resolved(input(
                M5LifecycleBadgeValue::Stable,
                M5ChannelBadgeValue::Preview,
            )),
            M5LifecycleChannelResolutionCase::resolved(input(
                M5LifecycleBadgeValue::Beta,
                M5ChannelBadgeValue::Beta,
            )),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::MigrationPathPreservationUnproven));
}

#[test]
fn badge_invariant_violation_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.badge_rows[0].collapses_lifecycle_and_channel_into_one_badge = true;
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::BadgeInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.badge_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet
        .governance_review
        .deprecated_or_removal_auto_points_to_migration_path = false;
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet
        .consumer_projection
        .channel_filter_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_maturity_badge_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MaturityBadgePrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_maturity_badge_primitive_packet().render_markdown_summary();
    for surface in M5MaturityBadgeConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_maturity_badge_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5MaturityBadgeConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5MaturityBadgeConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_maturity_badge_primitive_export()
        .expect("checked M5 maturity badge primitive export validates");
    assert_eq!(from_disk.packet_id, M5_MATURITY_BADGE_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_maturity_badge_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_maturity_badge_primitive_extension_install_row_beta_narrowed(),
        seeded_m5_maturity_badge_primitive_ecosystem_review_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.badge_rows.len(),
            M5MaturityBadgeConsumerSurface::ALL.len()
        );
    }

    let extension = seeded_m5_maturity_badge_primitive_extension_install_row_beta_narrowed();
    let row = extension
        .badge_rows
        .iter()
        .find(|r| r.consumer_surface == M5MaturityBadgeConsumerSurface::ExtensionInstallRow)
        .expect("extension row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Beta);

    let ecosystem = seeded_m5_maturity_badge_primitive_ecosystem_review_preview_narrowed();
    let row = ecosystem
        .badge_rows
        .iter()
        .find(|r| r.consumer_surface == M5MaturityBadgeConsumerSurface::EcosystemLifecycleReview)
        .expect("ecosystem row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let extension: M5MaturityBadgePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-lifecycle-and-channel-badges/extension_install_row_beta_narrowed.json"
    )))
    .expect("extension fixture parses");
    assert!(extension.validate().is_empty());
    assert_eq!(
        extension,
        seeded_m5_maturity_badge_primitive_extension_install_row_beta_narrowed()
    );

    let ecosystem: M5MaturityBadgePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-lifecycle-and-channel-badges/ecosystem_review_preview_narrowed.json"
    )))
    .expect("ecosystem fixture parses");
    assert!(ecosystem.validate().is_empty());
    assert_eq!(
        ecosystem,
        seeded_m5_maturity_badge_primitive_ecosystem_review_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_maturity_badge_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
