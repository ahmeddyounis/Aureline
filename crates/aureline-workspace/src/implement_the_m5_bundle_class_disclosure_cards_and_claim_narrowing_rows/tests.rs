//! Tests for the M5 bundle class-disclosure primitive: the resolver, the parity matrix, and the
//! checked-in support export.

use super::*;

// --- resolver: AC1 recommendation reason + honest support strength ---

#[test]
fn resolver_preserves_disclosure_identity_across_surfaces() {
    let input = start_center_native_input();
    let resolved = resolve_bundle_class_disclosure(&input).expect("resolves");
    assert_eq!(resolved.disclosure_id, input.disclosure_id);
    assert_eq!(resolved.card.disclosure_id, input.disclosure_id);
    assert_eq!(resolved.row.disclosure_id, input.disclosure_id);
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_discloses_recommendation_and_strength() {
    let resolved = resolve_bundle_class_disclosure(&docs_community_input()).expect("resolves");
    assert!(resolved.recommendation_and_strength_disclosed());
    assert!(!resolved.card.reason_for_recommendation.trim().is_empty());
    assert_eq!(resolved.row.support_claim_strength, BundleScorecardClass::Community);
}

#[test]
fn resolver_rejects_empty_recommendation_reason() {
    let input = M5BundleClassDisclosureInput {
        reason_for_recommendation: "   ".to_owned(),
        ..start_center_native_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::EmptyRecommendationReason)
    );
}

#[test]
fn resolver_caps_support_strength_to_evidence() {
    // An imported, bridged, stale bundle can only present an imported-strength claim.
    let resolved = resolve_bundle_class_disclosure(&migration_imported_input()).expect("resolves");
    assert_eq!(resolved.row.support_claim_strength, BundleScorecardClass::Imported);
    assert!(resolved.row.is_narrowed);
    assert!(resolved.row.narrowing_reason.is_some());
}

// --- resolver: AC2 native parity not inherited when mapped / policy-bound ---

#[test]
fn resolver_lets_native_first_party_inherit_parity() {
    let resolved = resolve_bundle_class_disclosure(&start_center_native_input()).expect("resolves");
    assert!(resolved.row.inherits_native_parity);
    assert!(!resolved.row.is_narrowed);
    assert!(resolved.native_parity_not_overclaimed());
}

#[test]
fn resolver_rejects_native_parity_for_imported_bundle() {
    let input = M5BundleClassDisclosureInput {
        claims_full_native_parity: true,
        ..migration_imported_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::NativeParityOverclaimed)
    );
}

#[test]
fn resolver_rejects_native_parity_for_policy_bound_managed_bundle() {
    // Native capability, but policy-bound: full native parity is still an over-claim.
    let input = M5BundleClassDisclosureInput {
        claims_full_native_parity: true,
        ..diagnostics_managed_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::NativeParityOverclaimed)
    );
}

#[test]
fn resolver_managed_bundle_does_not_inherit_native_parity() {
    let resolved = resolve_bundle_class_disclosure(&diagnostics_managed_input()).expect("resolves");
    assert!(!resolved.row.inherits_native_parity);
    assert!(resolved.row.policy_bound);
    assert!(resolved.row.is_narrowed);
}

// --- resolver: dependency posture ---

#[test]
fn resolver_discloses_managed_dependencies() {
    let resolved = resolve_bundle_class_disclosure(&diagnostics_managed_input()).expect("resolves");
    assert!(resolved.card.depends_on_managed_registry);
    assert!(resolved.card.depends_on_org_identity);
    assert!(resolved.card.depends_on_mirror_freshness);
    assert!(resolved.card.depends_on_policy_availability);
    assert!(resolved.card.policy_owner.is_some());
    assert!(resolved.card.mirror_source.is_some());
    assert!(resolved.card.entitlement_dependency.is_some());
    assert!(!resolved.card.implies_standalone_local_completeness);
}

#[test]
fn resolver_rejects_standalone_completeness_with_dependencies() {
    let input = M5BundleClassDisclosureInput {
        implies_standalone_local_completeness: true,
        ..diagnostics_managed_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::StandaloneCompletenessOverclaimed)
    );
}

#[test]
fn resolver_rejects_dependency_without_label() {
    let mut input = diagnostics_managed_input();
    input.dependencies.policy_owner = None;
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::DependencyDisclosureInconsistent)
    );
}

// --- resolver: honesty rules ---

#[test]
fn resolver_rejects_class_source_mismatch() {
    // A community class cannot back a certified source.
    let input = M5BundleClassDisclosureInput {
        disclosure_class: M5BundleDisclosureClass::Community,
        ..start_center_native_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::ClassSourceMismatch)
    );
}

#[test]
fn resolver_rejects_dishonest_capability_confidence() {
    // A bridged import cannot present a native capability claim.
    let input = M5BundleClassDisclosureInput {
        capability_confidence: M5CapabilityConfidence::Native,
        ..migration_imported_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::CapabilityConfidenceDishonest)
    );
}

#[test]
fn resolver_rejects_stale_claim_shown_as_current() {
    let input = M5BundleClassDisclosureInput {
        claims_current_despite_stale: true,
        ..support_replay_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::StaleClaimShownAsCurrent)
    );
}

#[test]
fn resolver_rejects_empty_disclosure_id() {
    let input = M5BundleClassDisclosureInput {
        disclosure_id: "  ".to_owned(),
        ..start_center_native_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::EmptyDisclosureId)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5BundleClassDisclosureInput {
        surface_label: "https://mirror.example/class".to_owned(),
        ..start_center_native_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5BundleClassDisclosureInput {
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::ImportedNotNative,
            degraded_label: "unsupported".to_owned(),
        }),
        ..migration_imported_input()
    };
    assert_eq!(
        resolve_bundle_class_disclosure(&input),
        Err(M5BundleClassDisclosureResolutionError::DegradedLabelGeneric)
    );
}

#[test]
fn resolver_builds_specific_narrowing_reason() {
    let resolved = resolve_bundle_class_disclosure(&support_replay_input()).expect("resolves");
    let reason = resolved.row.narrowing_reason.expect("narrowed");
    assert!(reason.starts_with("Claim narrowed:"));
    assert!(reason.contains("approximate"));
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_bundle_class_disclosure_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_bundle_class_disclosure_packet();
    let present: BTreeSet<M5BundleDisclosureSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5BundleDisclosureSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_cover_every_disclosure_class() {
    let packet = seeded_m5_bundle_class_disclosure_packet();
    let present: BTreeSet<M5BundleDisclosureClass> = packet
        .surface_rows
        .iter()
        .flat_map(|row| {
            row.example_disclosures
                .iter()
                .map(|case| case.resolved.card.disclosure_class)
        })
        .collect();
    for required in M5BundleDisclosureClass::ALL {
        assert!(present.contains(&required), "missing class {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_bundle_class_disclosure_packet();
    for row in &packet.surface_rows {
        for case in &row.example_disclosures {
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
    assert!(M5BundleDisclosureVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_bundle_class_disclosure_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_bundle_class_disclosure_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDisclosureViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_bundle_class_disclosure_packet();
    packet.surface_rows[0].collapses_class_to_generic = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDisclosureViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_bundle_class_disclosure_packet();
    packet.surface_rows[0].example_disclosures[0]
        .resolved
        .native_parity_not_overclaimed = !packet.surface_rows[0].example_disclosures[0]
        .resolved
        .native_parity_not_overclaimed;
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDisclosureViolation::ExampleDisclosureDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_bundle_class_disclosure_packet();
    packet.vocabulary_set.disclosure_classes.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDisclosureViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_bundle_class_disclosure_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5BundleDisclosureExportField::DependencyDisclosure);
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDisclosureViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_bundle_class_disclosure_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_bundle_class_disclosure_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_bundle_class_disclosure_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-bundle-class-disclosure-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_bundle_class_disclosure_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_bundle_class_disclosure_packet();
    assert_eq!(packet.record_kind, M5_BUNDLE_CLASS_DISCLOSURE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_VERSION);
}
