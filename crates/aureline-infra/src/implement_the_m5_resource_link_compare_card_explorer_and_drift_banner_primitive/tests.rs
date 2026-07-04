//! Tests for the M5 live-resource navigation primitive: the resolver, the parity
//! matrix, and the checked-in support export.

use super::*;

// --- resolver: AC1 identity + distinct source/live truth ---

#[test]
fn resolver_preserves_resource_identity_across_surfaces() {
    let input = source_to_live_in_sync_input();
    let resolved = resolve_live_resource_navigation(&input).expect("resolves");
    assert_eq!(resolved.resource_id, input.resource_id);
    assert_eq!(resolved.link_row.resource_id, input.resource_id);
    assert_eq!(resolved.compare_card.resource_id, input.resource_id);
    assert_eq!(resolved.explorer_row.resource_id, input.resource_id);
    assert_eq!(resolved.drift_banner.resource_id, input.resource_id);
    assert!(resolved.identity_consistent());
    assert!(resolved.truth_class_disclosed_consistently());
}

#[test]
fn resolver_keeps_source_and_live_distinct() {
    let resolved = resolve_live_resource_navigation(&source_to_live_in_sync_input())
        .expect("resolves");
    assert_ne!(resolved.link_row.from_truth, resolved.link_row.to_truth);
    assert_eq!(resolved.compare_card.rendered_side_truth, TruthMode::Rendered);
    assert_eq!(resolved.compare_card.live_side_truth, TruthMode::Live);
    assert!(resolved.source_and_live_distinct());
}

#[test]
fn resolver_rejects_blurred_link_truth_classes() {
    let input = M5LiveResourceInput {
        from_truth: TruthMode::Live,
        to_truth: TruthMode::Live,
        ..source_to_live_in_sync_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::BlurredLinkTruthClasses)
    );
}

// --- resolver: AC2 drift and unavailability visible before action ---

#[test]
fn resolver_surfaces_drift_before_action() {
    let resolved =
        resolve_live_resource_navigation(&rendered_live_drift_input()).expect("resolves");
    assert!(resolved.drift_banner.drift_present);
    assert!(resolved.drift_banner.banner_present);
    assert_eq!(
        resolved.drift_banner.banner_reason,
        Some(M5ManifestBuildDowngradeTrigger::DriftFromSource)
    );
    assert_eq!(
        resolved.drift_banner.what_diverged.as_deref(),
        Some("live replica count 5 diverges from rendered desired 3")
    );
    // The compare card is never marked current under drift, but stays inspectable.
    assert!(!resolved.compare_card.comparison_current);
    assert!(resolved.compare_card.safe_to_inspect);
    assert!(resolved.drift_visible_before_action());
}

#[test]
fn resolver_surfaces_unavailability_on_connector_loss() {
    let resolved =
        resolve_live_resource_navigation(&drift_banner_unavailable_input()).expect("resolves");
    assert!(resolved.drift_banner.unavailable);
    assert!(resolved.drift_banner.banner_present);
    assert_eq!(
        resolved.drift_banner.banner_reason,
        Some(M5ManifestBuildDowngradeTrigger::ConnectorLoss)
    );
    // The live side is not navigable once the connector is lost, but inspection of
    // the last cached snapshot stays safe.
    assert!(!resolved.link_row.to_side_navigable);
    assert!(resolved.link_row.from_side_navigable);
    assert!(resolved.drift_banner.safe_to_inspect);
    assert!(resolved.drift_visible_before_action());
    assert!(resolved.degraded.is_some());
}

#[test]
fn resolver_requires_divergence_detail_on_drift() {
    let input = M5LiveResourceInput {
        divergence_note: None,
        ..rendered_live_drift_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::DriftWithoutDivergenceDetail)
    );
}

// --- resolver: AC3 partial / limited never shown as current ---

#[test]
fn resolver_never_shows_cached_permission_limited_as_current() {
    let resolved =
        resolve_live_resource_navigation(&cluster_explorer_cached_input()).expect("resolves");
    assert!(!resolved.explorer_row.presents_as_current);
    assert!(resolved.drift_banner.what_stale);
    assert_eq!(
        resolved.explorer_row.freshness,
        M5ResourceFreshness::CachedStale
    );
    assert_eq!(
        resolved.drift_banner.banner_reason,
        Some(M5ManifestBuildDowngradeTrigger::PolicyBlock)
    );
    assert!(resolved.explorer_row.permission_connection_note.is_some());
    assert!(resolved.no_partial_shown_as_current());
}

#[test]
fn resolver_presents_live_fresh_full_access_as_current() {
    let resolved = resolve_live_resource_navigation(&source_to_live_in_sync_input())
        .expect("resolves");
    assert!(resolved.explorer_row.presents_as_current);
    assert!(resolved.compare_card.comparison_current);
    assert!(!resolved.drift_banner.banner_present);
    assert!(resolved.no_partial_shown_as_current());
}

#[test]
fn resolver_requires_note_when_permission_limited() {
    let input = M5LiveResourceInput {
        permission_connection_note: None,
        ..cluster_explorer_cached_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::PermissionLimitedWithoutNote)
    );
}

#[test]
fn resolver_rejects_live_fresh_with_non_live_truth() {
    let input = M5LiveResourceInput {
        freshness: M5ResourceFreshness::LiveFresh,
        truth_mode: TruthMode::Rendered,
        ..source_to_live_in_sync_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::LiveFreshTruthMismatch)
    );
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_empty_resource_id() {
    let input = M5LiveResourceInput {
        resource_id: "   ".to_owned(),
        ..source_to_live_in_sync_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::EmptyResourceId)
    );
}

#[test]
fn resolver_rejects_empty_resource_identity() {
    let input = M5LiveResourceInput {
        identity: M5ResourceIdentity {
            resource_kind: M5ResourceKind::Workload,
            stable_id: "".to_owned(),
            namespace: None,
            project: None,
        },
        ..source_to_live_in_sync_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::EmptyResourceIdentity)
    );
}

#[test]
fn resolver_rejects_empty_target_identity_ref() {
    let input = M5LiveResourceInput {
        target_identity_ref: "".to_owned(),
        ..source_to_live_in_sync_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::EmptyTargetIdentityRef)
    );
}

#[test]
fn resolver_rejects_no_actions() {
    let input = M5LiveResourceInput {
        available_actions: vec![],
        ..source_to_live_in_sync_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::NoActionsOffered)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5LiveResourceInput {
        resource_label: "see https://example.com/secrets".to_owned(),
        ..source_to_live_in_sync_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5LiveResourceInput {
        degraded: Some(DegradedState {
            trigger: M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            degraded_label: "unavailable".to_owned(),
        }),
        ..drift_banner_unavailable_input()
    };
    assert_eq!(
        resolve_live_resource_navigation(&input),
        Err(M5LiveResourceResolutionError::DegradedLabelGeneric)
    );
}

#[test]
fn resolver_provider_overlay_hands_off_without_claiming_current() {
    let resolved = resolve_live_resource_navigation(&provider_console_input()).expect("resolves");
    assert!(!resolved.explorer_row.presents_as_current);
    assert!(resolved.drift_banner.banner_present);
    assert_eq!(resolved.drift_banner.banner_reason, None);
    assert_eq!(resolved.compare_card.truth_mode, TruthMode::ProviderOverlay);
    assert!(resolved
        .explorer_row
        .actions
        .contains(&M5ResourceActionKind::OpenInProviderConsole));
    assert!(resolved.no_partial_shown_as_current());
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_live_resource_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_live_resource_packet();
    let present: BTreeSet<M5LiveResourceSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5LiveResourceSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_live_resource_packet();
    for row in &packet.surface_rows {
        for case in &row.example_navigation {
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
    assert!(M5LiveResourceVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_live_resource_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_live_resource_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5LiveResourceViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_live_resource_packet();
    packet.surface_rows[0].presents_partial_as_current = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5LiveResourceViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_live_resource_packet();
    packet.surface_rows[0].example_navigation[0]
        .resolved
        .explorer_row
        .presents_as_current = !packet.surface_rows[0].example_navigation[0]
        .resolved
        .explorer_row
        .presents_as_current;
    let violations = packet.validate();
    assert!(violations.contains(&M5LiveResourceViolation::ExampleNavigationDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_live_resource_packet();
    packet.vocabulary_set.resource_kinds.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5LiveResourceViolation::VocabularySetDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_live_resource_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_live_resource_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_live_resource_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_live_resource_packet();
    assert_eq!(packet.record_kind, M5_LIVE_RESOURCE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_LIVE_RESOURCE_SCHEMA_VERSION);
}
