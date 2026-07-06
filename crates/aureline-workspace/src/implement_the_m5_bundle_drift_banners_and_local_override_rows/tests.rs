//! Tests for the M5 bundle drift / override primitive: the resolver, the parity matrix, and the
//! checked-in support export.

use super::*;

// --- resolver: AC1 reviewable at detail ---

#[test]
fn resolver_preserves_drift_identity_across_surfaces() {
    let input = harmless_and_significant_drift_input();
    let resolved = resolve_bundle_drift(&input).expect("resolves");
    assert_eq!(resolved.drift_id, input.drift_id);
    assert_eq!(resolved.banner.drift_id, input.drift_id);
    assert_eq!(resolved.override_list.drift_id, input.drift_id);
    assert_eq!(resolved.rollback_remove_card.drift_id, input.drift_id);
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_enumerates_distinct_drift_kinds_at_detail() {
    let resolved = resolve_bundle_drift(&harmless_and_significant_drift_input()).expect("resolves");
    assert!(resolved.reviewable_at_detail());
    assert!(!resolved.banner.reads_like_generic_update);
    // local_only_edit + bundle_version_drift + missing_artifact = three distinct kinds.
    assert_eq!(resolved.banner.distinct_drift_kinds.len(), 3);
    assert!(resolved.has_enumerated_drift());
    assert!(resolved.banner.has_missing_artifacts);
}

#[test]
fn resolver_reports_overrides_at_field_package_task_granularity() {
    let resolved = resolve_bundle_drift(&remove_rollback_input()).expect("resolves");
    // Neither collapses into one opaque "customized" label.
    assert!(!resolved.override_list.collapses_to_opaque_customized);
    assert!(resolved
        .override_list
        .granularities_present
        .contains(&M5DriftGranularity::Package));
    assert!(resolved
        .override_list
        .granularities_present
        .contains(&M5DriftGranularity::Field));
}

#[test]
fn resolver_rejects_banner_that_reads_like_generic_update() {
    let input = M5BundleDriftInput {
        reads_like_generic_update: true,
        ..harmless_and_significant_drift_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::ReadsLikeGenericUpdate)
    );
}

#[test]
fn resolver_rejects_empty_drift_signals() {
    let input = M5BundleDriftInput {
        local_overrides: vec![],
        missing_artifacts: vec![],
        ..harmless_and_significant_drift_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::EmptyDriftSignals)
    );
}

// --- resolver: AC2 harmless vs support-significant ---

#[test]
fn resolver_distinguishes_harmless_from_support_significant() {
    let resolved = resolve_bundle_drift(&harmless_and_significant_drift_input()).expect("resolves");
    assert!(resolved.significance_distinguished());
    assert!(resolved.separates_harmless_from_significant());
    assert_eq!(
        resolved.banner.highest_significance,
        M5DriftSignificance::SupportSignificant
    );
    let harmless = &resolved.override_list.overrides[0];
    assert_eq!(
        harmless.significance,
        M5DriftSignificance::HarmlessLocalPreference
    );
}

#[test]
fn missing_artifact_only_drift_is_support_significant() {
    let resolved = resolve_bundle_drift(&missing_artifact_only_input()).expect("resolves");
    assert!(resolved.override_list.overrides.is_empty());
    assert!(resolved.banner.has_missing_artifacts);
    assert_eq!(
        resolved.banner.highest_significance,
        M5DriftSignificance::SupportSignificant
    );
}

#[test]
fn resolver_rejects_support_significant_claimed_harmless() {
    let input = M5BundleDriftInput {
        claims_harmless_despite_significant: true,
        ..version_drift_rebase_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::SignificanceMislabeled)
    );
}

#[test]
fn override_significance_must_match_drift_kind() {
    let mut input = harmless_and_significant_drift_input();
    // Paint a local-only edit as support-significant.
    input.local_overrides[0].significance = M5DriftSignificance::SupportSignificant;
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::OverrideRowIncomplete)
    );
}

// --- resolver: AC3 attributable without reset + rollback ---

#[test]
fn resolver_keeps_overrides_attributable_without_reset() {
    let resolved = resolve_bundle_drift(&imported_gap_input()).expect("resolves");
    assert!(resolved.overrides_attributable_without_reset());
    assert!(resolved.override_list.preserves_local_overrides);
    assert!(resolved.override_list.attributable_and_exportable);
    assert!(!resolved.rollback_remove_card.forces_reset);
}

#[test]
fn resolver_rejects_forced_reset_to_export() {
    let input = M5BundleDriftInput {
        forces_reset_to_export: true,
        ..imported_gap_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::ForcesResetToExport)
    );
}

#[test]
fn resolver_rejects_removing_user_protected_override() {
    let mut input = imported_gap_input();
    // An adopted (user-protected) override marked for removal.
    input.local_overrides[0].resolution = ResolutionChoice::RemoveBundleOwned;
    // resolution_safe catches this first as an incomplete row.
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::OverrideRowIncomplete)
    );
}

#[test]
fn resolver_requires_checkpoint_for_mutating_remove() {
    let input = M5BundleDriftInput {
        rollback_checkpoint: None,
        ..remove_rollback_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::MutatingOpWithoutCheckpoint)
    );
}

#[test]
fn resolver_creates_checkpoint_only_for_mutating_ops() {
    let mutating = resolve_bundle_drift(&remove_rollback_input()).expect("resolves");
    assert!(mutating.rollback_remove_card.creates_rollback_checkpoint);
    assert!(mutating.rollback_remove_card.rollback_checkpoint.is_some());

    let read_only = resolve_bundle_drift(&missing_artifact_only_input()).expect("resolves");
    assert!(!read_only.rollback_remove_card.creates_rollback_checkpoint);
    assert!(read_only.rollback_remove_card.rollback_checkpoint.is_none());
}

// --- resolver: recommended choices + structural rules ---

#[test]
fn resolver_requires_actionable_recommended_choices() {
    let empty = M5BundleDriftInput {
        recommended_choices: vec![],
        ..harmless_and_significant_drift_input()
    };
    assert_eq!(
        resolve_bundle_drift(&empty),
        Err(M5BundleDriftResolutionError::MissingRecommendedChoices)
    );

    let non_actionable = M5BundleDriftInput {
        recommended_choices: vec![ResolutionChoice::NotApplicable],
        ..harmless_and_significant_drift_input()
    };
    assert_eq!(
        resolve_bundle_drift(&non_actionable),
        Err(M5BundleDriftResolutionError::NonActionableRecommendedChoice)
    );
}

#[test]
fn resolver_rejects_stale_claim_shown_as_current() {
    let input = M5BundleDriftInput {
        claims_current_despite_stale: true,
        ..imported_gap_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::StaleClaimShownAsCurrent)
    );
}

#[test]
fn resolver_rejects_empty_drift_id() {
    let input = M5BundleDriftInput {
        drift_id: "  ".to_owned(),
        ..harmless_and_significant_drift_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::EmptyDriftId)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5BundleDriftInput {
        surface_label: "https://mirror.example/drift".to_owned(),
        ..harmless_and_significant_drift_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5BundleDriftInput {
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
            degraded_label: "unsupported".to_owned(),
        }),
        ..harmless_and_significant_drift_input()
    };
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::DegradedLabelGeneric)
    );
}

#[test]
fn resolver_rejects_incomplete_missing_artifact() {
    let mut input = missing_artifact_only_input();
    input.missing_artifacts[0].artifact_ref = String::new();
    assert_eq!(
        resolve_bundle_drift(&input),
        Err(M5BundleDriftResolutionError::MissingArtifactIncomplete)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_bundle_drift_override_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_bundle_drift_override_packet();
    let present: BTreeSet<M5BundleDriftSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5BundleDriftSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_bundle_drift_override_packet();
    for row in &packet.surface_rows {
        for case in &row.example_drifts {
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
    assert!(M5BundleDriftVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_bundle_drift_override_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_bundle_drift_override_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDriftViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_bundle_drift_override_packet();
    packet.surface_rows[0].collapses_to_opaque_customized = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDriftViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_bundle_drift_override_packet();
    packet.surface_rows[0].example_drifts[0]
        .resolved
        .reviewable_at_detail = !packet.surface_rows[0].example_drifts[0]
        .resolved
        .reviewable_at_detail;
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDriftViolation::ExampleDriftDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_bundle_drift_override_packet();
    packet.vocabulary_set.drift_kinds.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDriftViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_bundle_drift_override_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5BundleDriftExportField::LocalOverrides);
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleDriftViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_bundle_drift_override_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_bundle_drift_override_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_bundle_drift_override_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-bundle-drift-override-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_bundle_drift_override_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_bundle_drift_override_packet();
    assert_eq!(packet.record_kind, M5_BUNDLE_DRIFT_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_BUNDLE_DRIFT_SCHEMA_VERSION);
}
