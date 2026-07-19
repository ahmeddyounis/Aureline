//! Tests for the M5 start-center launch-wedge primitive: the resolver, the parity
//! matrix, and the checked-in support export.

use super::*;

// --- resolver: AC1 identity + entry-assurance disclosed ---

#[test]
fn resolver_preserves_wedge_identity_across_surfaces() {
    let input = certified_launch_wedge_input();
    let resolved = resolve_launch_wedge(&input).expect("resolves");
    assert_eq!(resolved.wedge_id, input.wedge_id);
    assert_eq!(resolved.bundle_card.wedge_id, input.wedge_id);
    assert_eq!(resolved.badge_group.wedge_id, input.wedge_id);
    assert!(resolved.identity_consistent());
    assert!(resolved.source_class_consistent());
}

#[test]
fn resolver_discloses_certified_entry_assurance_with_review_action() {
    let resolved = resolve_launch_wedge(&certified_launch_wedge_input()).expect("resolves");
    assert!(resolved.entry_assurance_disclosed());
    assert_eq!(
        resolved.bundle_card.entry_assurance_tier,
        M5EntryAssuranceTier::Certified
    );
    assert_eq!(
        resolved.bundle_card.source_class,
        CertificationTarget::Certified
    );
    assert!(resolved.bundle_card.review_action_present);
    assert!(resolved.bundle_card.discloses_signer_source);
    assert!(resolved.bundle_card.discloses_certification_freshness);
    assert_eq!(
        resolved.bundle_card.compatible_aureline_range,
        ">=2026.6, <2027.0"
    );
}

#[test]
fn resolver_marks_community_and_local_tiers_distinctly() {
    let community = resolve_launch_wedge(&community_aging_wedge_input()).expect("resolves");
    assert_eq!(
        community.bundle_card.entry_assurance_tier,
        M5EntryAssuranceTier::Approximate
    );
    let local = resolve_launch_wedge(&local_draft_wedge_input()).expect("resolves");
    assert_eq!(
        local.bundle_card.entry_assurance_tier,
        M5EntryAssuranceTier::LocalOnly
    );
}

// --- resolver: AC2 badges degrade visibly ---

#[test]
fn resolver_keeps_fresh_confirmed_badge_current() {
    let resolved = resolve_launch_wedge(&certified_launch_wedge_input()).expect("resolves");
    assert_eq!(
        resolved.badge_group.downgrade_state,
        M5ArchetypeBadgeDowngradeState::None
    );
    assert!(!resolved.badge_group.downgrade_state.is_degraded());
    assert!(resolved.badges_degrade_visibly());
}

#[test]
fn resolver_narrows_aging_badge_to_limited() {
    let resolved = resolve_launch_wedge(&community_aging_wedge_input()).expect("resolves");
    assert_eq!(
        resolved.badge_group.downgrade_state,
        M5ArchetypeBadgeDowngradeState::Limited
    );
    assert!(resolved.badge_group.downgrade_state.is_degraded());
    assert!(resolved.degraded.is_some());
}

#[test]
fn resolver_marks_stale_and_missing_badges_retest_pending() {
    let stale = resolve_launch_wedge(&imported_stale_wedge_input()).expect("resolves");
    assert_eq!(
        stale.badge_group.downgrade_state,
        M5ArchetypeBadgeDowngradeState::RetestPending
    );
    let missing = resolve_launch_wedge(&local_draft_wedge_input()).expect("resolves");
    assert_eq!(
        missing.badge_group.downgrade_state,
        M5ArchetypeBadgeDowngradeState::RetestPending
    );
}

#[test]
fn resolver_narrows_fresh_but_unconfirmed_archetype_to_limited() {
    let input = M5LaunchWedgeInput {
        certification_freshness: EvidenceFreshness::Fresh,
        archetype_confidence: ArchetypeConfidence::Probable,
        ..certified_launch_wedge_input()
    };
    let resolved = resolve_launch_wedge(&input).expect("resolves");
    assert_eq!(
        resolved.badge_group.downgrade_state,
        M5ArchetypeBadgeDowngradeState::Limited
    );
}

#[test]
fn resolver_rejects_stale_claim_shown_as_current() {
    let input = M5LaunchWedgeInput {
        claims_current_despite_stale: true,
        ..imported_stale_wedge_input()
    };
    assert_eq!(
        resolve_launch_wedge(&input),
        Err(M5LaunchWedgeResolutionError::StaleClaimShownAsCurrent)
    );
}

// --- resolver: AC3 source class named, never inherited ---

#[test]
fn resolver_rejects_hidden_marketplace_inheritance() {
    let input = M5LaunchWedgeInput {
        inherits_hidden_marketplace_assumption: true,
        ..certified_launch_wedge_input()
    };
    assert_eq!(
        resolve_launch_wedge(&input),
        Err(M5LaunchWedgeResolutionError::HiddenMarketplaceInheritance)
    );
}

#[test]
fn resolver_names_source_class_on_every_case() {
    for input in [
        certified_launch_wedge_input(),
        community_aging_wedge_input(),
        local_draft_wedge_input(),
    ] {
        let resolved = resolve_launch_wedge(&input).expect("resolves");
        assert!(resolved.source_class_not_inherited());
        assert!(resolved.bundle_card.source_class_named);
    }
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_empty_wedge_id() {
    let input = M5LaunchWedgeInput {
        wedge_id: "  ".to_owned(),
        ..certified_launch_wedge_input()
    };
    assert_eq!(
        resolve_launch_wedge(&input),
        Err(M5LaunchWedgeResolutionError::EmptyWedgeId)
    );
}

#[test]
fn resolver_rejects_empty_review_action() {
    let input = M5LaunchWedgeInput {
        review_action_ref: String::new(),
        ..certified_launch_wedge_input()
    };
    assert_eq!(
        resolve_launch_wedge(&input),
        Err(M5LaunchWedgeResolutionError::EmptyReviewAction)
    );
}

#[test]
fn resolver_rejects_empty_compatible_range() {
    let input = M5LaunchWedgeInput {
        compatible_aureline_range: "   ".to_owned(),
        ..certified_launch_wedge_input()
    };
    assert_eq!(
        resolve_launch_wedge(&input),
        Err(M5LaunchWedgeResolutionError::EmptyCompatibleRange)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5LaunchWedgeInput {
        supported_platform_envelope_ref: "https://mirror.example/envelope".to_owned(),
        ..certified_launch_wedge_input()
    };
    assert_eq!(
        resolve_launch_wedge(&input),
        Err(M5LaunchWedgeResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5LaunchWedgeInput {
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::StaleCertification,
            degraded_label: "unsupported".to_owned(),
        }),
        ..certified_launch_wedge_input()
    };
    assert_eq!(
        resolve_launch_wedge(&input),
        Err(M5LaunchWedgeResolutionError::DegradedLabelGeneric)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_start_center_launch_wedge_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_start_center_launch_wedge_packet();
    let present: BTreeSet<M5LaunchWedgeSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5LaunchWedgeSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_start_center_launch_wedge_packet();
    for row in &packet.surface_rows {
        for case in &row.example_wedges {
            assert!(
                case.is_self_consistent(),
                "case drifted on {:?}",
                row.surface_family
            );
        }
    }
}

#[test]
fn vocabulary_set_matches_canonical() {
    assert!(M5LaunchWedgeVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_start_center_launch_wedge_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_start_center_launch_wedge_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5LaunchWedgeViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_start_center_launch_wedge_packet();
    packet.surface_rows[0].hides_entry_assurance = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5LaunchWedgeViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_start_center_launch_wedge_packet();
    packet.surface_rows[0].example_wedges[0]
        .resolved
        .badges_degrade_visibly = !packet.surface_rows[0].example_wedges[0]
        .resolved
        .badges_degrade_visibly;
    let violations = packet.validate();
    assert!(violations.contains(&M5LaunchWedgeViolation::ExampleWedgeDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_start_center_launch_wedge_packet();
    packet
        .vocabulary_set
        .source_classes
        .push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5LaunchWedgeViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_start_center_launch_wedge_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5LaunchWedgeExportField::SourceClass);
    let violations = packet.validate();
    assert!(violations.contains(&M5LaunchWedgeViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_start_center_launch_wedge_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_start_center_launch_wedge_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_start_center_launch_wedge_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-start-center-launch-wedge-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_start_center_launch_wedge_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_start_center_launch_wedge_packet();
    assert_eq!(packet.record_kind, M5_START_CENTER_WEDGE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_START_CENTER_WEDGE_SCHEMA_VERSION);
}
