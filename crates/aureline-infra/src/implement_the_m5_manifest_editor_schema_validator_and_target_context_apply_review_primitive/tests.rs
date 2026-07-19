//! Tests for the M5 manifest-authoring primitive: the resolver, the parity
//! matrix, and the checked-in support export.

use super::*;

// --- resolver: AC1 identity + truth across surfaces ---

#[test]
fn resolver_preserves_authoring_identity_across_surfaces() {
    let input = apply_ready_input();
    let resolved = resolve_manifest_authoring(&input).expect("resolves");
    assert_eq!(resolved.authoring_id, input.authoring_id);
    assert_eq!(resolved.header.authoring_id, input.authoring_id);
    assert_eq!(resolved.schema_row.authoring_id, input.authoring_id);
    assert_eq!(resolved.context_chips.authoring_id, input.authoring_id);
    assert_eq!(resolved.apply_banner.authoring_id, input.authoring_id);
    assert!(resolved.identity_consistent());
    assert!(resolved.truth_class_consistent());
}

#[test]
fn resolver_discloses_environment_and_schema_source() {
    let resolved = resolve_manifest_authoring(&apply_ready_input()).expect("resolves");
    assert!(resolved.environment_disclosed());
    assert_eq!(
        resolved.context_chips.target_identity_ref,
        "target:prod-us-east"
    );
    assert!(resolved.schema_row.schema_source.is_explicit());
    assert!(resolved.context_chips.context_complete);
}

// --- resolver: AC2 states explicit before mutation ---

#[test]
fn resolver_offers_apply_only_when_fully_gated() {
    let resolved = resolve_manifest_authoring(&apply_ready_input()).expect("resolves");
    assert!(resolved.apply_banner.apply_available);
    assert!(resolved.header.apply_available);
    assert!(resolved.header.preview_available);
    assert!(resolved.apply_banner.counts_known);
    assert_eq!(resolved.apply_banner.apply_blocked_reason, None);
    assert!(resolved.states_explicit_before_mutation());
}

#[test]
fn resolver_gates_apply_when_connector_lost() {
    let resolved = resolve_manifest_authoring(&apply_review_degraded_input()).expect("resolves");
    assert!(!resolved.apply_banner.apply_available);
    assert_eq!(
        resolved.apply_banner.apply_blocked_reason,
        Some(M5ManifestBuildDowngradeTrigger::ConnectorLoss)
    );
    // The preview path stays available so the user can still inspect the plan.
    assert!(resolved.header.preview_available);
    assert!(resolved.states_explicit_before_mutation());
    assert!(resolved.degraded.is_some());
}

#[test]
fn resolver_blocks_apply_on_validation_errors() {
    let input = M5ManifestAuthoringInput {
        validation_state: M5SchemaValidationState::Errors,
        ..apply_ready_input()
    };
    let resolved = resolve_manifest_authoring(&input).expect("resolves");
    assert!(resolved.schema_row.blocks_apply);
    assert!(!resolved.apply_banner.apply_available);
    assert_eq!(
        resolved.apply_banner.apply_blocked_reason,
        Some(M5ManifestBuildDowngradeTrigger::SchemaStale)
    );
    assert!(resolved.states_explicit_before_mutation());
}

#[test]
fn resolver_blocks_apply_when_dry_run_policy_blocked() {
    let input = M5ManifestAuthoringInput {
        dry_run: M5DryRunAvailability::UnavailablePolicyBlocked,
        ..apply_ready_input()
    };
    let resolved = resolve_manifest_authoring(&input).expect("resolves");
    assert!(!resolved.apply_banner.apply_available);
    assert_eq!(
        resolved.apply_banner.apply_blocked_reason,
        Some(M5ManifestBuildDowngradeTrigger::PolicyBlock)
    );
}

// --- resolver: AC3 schema freshness visible ---

#[test]
fn resolver_keeps_schema_freshness_visible_on_header_and_row() {
    let resolved = resolve_manifest_authoring(&plan_preview_stale_input()).expect("resolves");
    assert_eq!(resolved.header.schema_freshness, M5SchemaFreshness::Stale);
    assert_eq!(
        resolved.schema_row.schema_freshness,
        M5SchemaFreshness::Stale
    );
    assert!(resolved.schema_freshness_disclosed());
    assert!(!resolved.header.schema_freshness.is_current());
    // A stale (but resolvable) schema still permits apply — it is disclosed, not
    // blocking.
    assert!(resolved.apply_banner.apply_available);
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_write_posture_on_unresolved_target() {
    let input = M5ManifestAuthoringInput {
        target_context: M5TargetContextChips {
            account: None,
            project: None,
            cluster: None,
            namespace: None,
        },
        ..apply_ready_input()
    };
    assert_eq!(
        resolve_manifest_authoring(&input),
        Err(M5ManifestAuthoringResolutionError::ApplyPostureOnUnresolvedTarget)
    );
}

#[test]
fn resolver_rejects_writable_manifest_without_schema() {
    let input = M5ManifestAuthoringInput {
        schema_freshness: M5SchemaFreshness::Unavailable,
        validation_state: M5SchemaValidationState::SchemaUnavailable,
        ..apply_ready_input()
    };
    assert_eq!(
        resolve_manifest_authoring(&input),
        Err(M5ManifestAuthoringResolutionError::WritableManifestWithoutSchema)
    );
}

#[test]
fn resolver_rejects_mutation_counts_without_write_path() {
    let input = M5ManifestAuthoringInput {
        mutation_counts: Some(M5MutationCounts {
            creates: 1,
            updates: 0,
            deletes: 0,
        }),
        ..read_only_rendered_input()
    };
    assert_eq!(
        resolve_manifest_authoring(&input),
        Err(M5ManifestAuthoringResolutionError::MutationCountsWithoutWritePath)
    );
}

#[test]
fn resolver_rejects_empty_authoring_id() {
    let input = M5ManifestAuthoringInput {
        authoring_id: "   ".to_owned(),
        ..apply_ready_input()
    };
    assert_eq!(
        resolve_manifest_authoring(&input),
        Err(M5ManifestAuthoringResolutionError::EmptyAuthoringId)
    );
}

#[test]
fn resolver_rejects_empty_target_identity_ref() {
    let input = M5ManifestAuthoringInput {
        target_identity_ref: "".to_owned(),
        ..read_only_rendered_input()
    };
    assert_eq!(
        resolve_manifest_authoring(&input),
        Err(M5ManifestAuthoringResolutionError::EmptyTargetIdentityRef)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5ManifestAuthoringInput {
        manifest_label: "see https://example.com/secrets".to_owned(),
        ..read_only_rendered_input()
    };
    assert_eq!(
        resolve_manifest_authoring(&input),
        Err(M5ManifestAuthoringResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5ManifestAuthoringInput {
        degraded: Some(DegradedState {
            trigger: M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            degraded_label: "unavailable".to_owned(),
        }),
        ..apply_ready_input()
    };
    assert_eq!(
        resolve_manifest_authoring(&input),
        Err(M5ManifestAuthoringResolutionError::DegradedLabelGeneric)
    );
}

#[test]
fn resolver_read_only_offers_no_apply() {
    let resolved = resolve_manifest_authoring(&live_explorer_input()).expect("resolves");
    assert!(!resolved.apply_banner.apply_available);
    assert!(!resolved.header.apply_available);
    assert!(!resolved.apply_banner.counts_known);
    assert!(resolved.states_explicit_before_mutation());
    // A read-only surface has no write path, so no blocked reason is surfaced.
    assert_eq!(resolved.apply_banner.apply_blocked_reason, None);
}

#[test]
fn target_context_completeness_requires_account_and_scope() {
    assert!(full_context("x").is_complete());
    let account_only = M5TargetContextChips {
        account: Some("a".to_owned()),
        project: None,
        cluster: None,
        namespace: None,
    };
    assert!(!account_only.is_complete());
    let scope_only = M5TargetContextChips {
        account: None,
        project: Some("p".to_owned()),
        cluster: None,
        namespace: None,
    };
    assert!(!scope_only.is_complete());
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_manifest_authoring_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_manifest_authoring_packet();
    let present: BTreeSet<M5ManifestAuthoringSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5ManifestAuthoringSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_manifest_authoring_packet();
    for row in &packet.surface_rows {
        for case in &row.example_authoring {
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
    assert!(M5ManifestAuthoringVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_manifest_authoring_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_manifest_authoring_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5ManifestAuthoringViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_manifest_authoring_packet();
    packet.surface_rows[0].offers_apply_before_review = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5ManifestAuthoringViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_manifest_authoring_packet();
    packet.surface_rows[0].example_authoring[0]
        .resolved
        .apply_banner
        .apply_available = !packet.surface_rows[0].example_authoring[0]
        .resolved
        .apply_banner
        .apply_available;
    let violations = packet.validate();
    assert!(violations.contains(&M5ManifestAuthoringViolation::ExampleAuthoringDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_manifest_authoring_packet();
    packet.vocabulary_set.source_types.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5ManifestAuthoringViolation::VocabularySetDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_manifest_authoring_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_manifest_authoring_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_manifest_authoring_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_manifest_authoring_packet();
    assert_eq!(packet.record_kind, M5_MANIFEST_AUTHORING_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_MANIFEST_AUTHORING_SCHEMA_VERSION);
}
