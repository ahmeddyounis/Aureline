use super::*;

fn clean_banner_input() -> M5WorkspaceTrustBannerResolutionInput {
    M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:test".to_owned(),
        object_identity: "workspace: test-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_strip_input() -> M5RootTrustStripResolutionInput {
    M5RootTrustStripResolutionInput {
        strip_id: "root-strip:test".to_owned(),
        root_identity: "root: /src".to_owned(),
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_workspace_trust_root_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_WORKSPACE_TRUST_ROOT_CONTROLS_PACKET_ID);
}

#[test]
fn banner_clean_names_trust_class_and_is_legible() {
    let resolved = resolve_workspace_trust_banner(clean_banner_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.legible_at_a_glance);
    assert!(!resolved.collapses_mixed_root_into_uniform);
    assert_eq!(resolved.trust_scope, "trusted_workspace");
    assert_eq!(resolved.grant_source, "user_explicit");
    assert_eq!(
        resolved.trust_disposition,
        Some(M5WorkspaceTrustRepairDisposition::Trusted)
    );
    assert_eq!(
        resolved.next_action,
        M5WorkspaceTrustRootNextAction::OpenTrustDetail
    );
}

#[test]
fn banner_mixed_root_keeps_disposition_and_is_explicit() {
    let mut input = clean_banner_input();
    input.trust_scope = M5TrustScopeState::MixedRoot;
    let resolved = resolve_workspace_trust_banner(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_mixed_root);
    assert_eq!(
        resolved.trust_disposition,
        Some(M5WorkspaceTrustRepairDisposition::MixedRoot)
    );
}

#[test]
fn banner_object_identity_unstated_degrades() {
    let mut input = clean_banner_input();
    input.object_identity = "  ".to_owned();
    let resolved = resolve_workspace_trust_banner(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceTrustBannerDegradeReason::ObjectIdentityUnstated)
    );
}

#[test]
fn banner_scope_unknown_degrades_and_has_no_disposition() {
    let mut input = clean_banner_input();
    input.trust_scope = M5TrustScopeState::ScopeUnknown;
    let resolved = resolve_workspace_trust_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceTrustBannerDegradeReason::TrustScopeUnresolved)
    );
    assert_eq!(resolved.trust_disposition, None);
}

#[test]
fn banner_grant_unstated_degrades() {
    let mut input = clean_banner_input();
    input.grant_source = M5TrustGrantSourceClass::GrantSourceUnknown;
    input.grant_actor_stated = false;
    let resolved = resolve_workspace_trust_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceTrustBannerDegradeReason::GrantSourceUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5WorkspaceTrustRootNextAction::ReviewGrantSource
    );
}

#[test]
fn banner_policy_epoch_missing_degrades() {
    let mut input = clean_banner_input();
    input.grant_source = M5TrustGrantSourceClass::PolicyManaged;
    input.policy_epoch = "".to_owned();
    let resolved = resolve_workspace_trust_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceTrustBannerDegradeReason::PolicyEpochUnstated)
    );
}

#[test]
fn banner_narrowed_capability_unstated_degrades() {
    let mut input = clean_banner_input();
    input.capability_narrow = M5CapabilityNarrowState::ExecutionBlocked;
    input.capability_narrow_stated = false;
    let resolved = resolve_workspace_trust_banner(input).unwrap();
    assert!(resolved.capability_narrowed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceTrustBannerDegradeReason::NarrowedCapabilityUnstated)
    );
}

#[test]
fn banner_mixed_root_collapsed_degrades() {
    let mut input = clean_banner_input();
    input.trust_scope = M5TrustScopeState::MixedRoot;
    input.reads_as_uniform_trust = true;
    let resolved = resolve_workspace_trust_banner(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.collapses_mixed_root_into_uniform);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceTrustBannerDegradeReason::MixedRootCollapsedIntoUniform)
    );
    assert_eq!(
        resolved.next_action,
        M5WorkspaceTrustRootNextAction::InspectRootTrust
    );
}

#[test]
fn banner_detail_missing_degrades() {
    let mut input = clean_banner_input();
    input.detail_command_available = false;
    let resolved = resolve_workspace_trust_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WorkspaceTrustBannerDegradeReason::TrustDetailPathMissing)
    );
}

#[test]
fn banner_empty_id_and_forbidden_material_error() {
    let mut input = clean_banner_input();
    input.banner_id = "".to_owned();
    assert_eq!(
        resolve_workspace_trust_banner(input).unwrap_err(),
        M5WorkspaceTrustRootResolutionError::EmptyBannerId
    );

    let mut input = clean_banner_input();
    input.object_identity = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_workspace_trust_banner(input).unwrap_err(),
        M5WorkspaceTrustRootResolutionError::ForbiddenMaterial
    );
}

#[test]
fn strip_clean_names_per_root_trust() {
    let resolved = resolve_root_trust_strip(clean_strip_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.per_root_trust_explicit);
    assert!(!resolved.collapses_per_root_into_uniform);
    assert_eq!(resolved.root_trust, "root_trusted");
    assert_eq!(
        resolved.trust_disposition,
        Some(M5WorkspaceTrustRepairDisposition::Trusted)
    );
}

#[test]
fn strip_mixed_children_disposition_is_mixed_root() {
    let mut input = clean_strip_input();
    input.root_trust = M5RootTrustState::RootMixedChildren;
    let resolved = resolve_root_trust_strip(input).unwrap();
    assert_eq!(
        resolved.trust_disposition,
        Some(M5WorkspaceTrustRepairDisposition::MixedRoot)
    );
}

#[test]
fn strip_root_unknown_degrades() {
    let mut input = clean_strip_input();
    input.root_trust = M5RootTrustState::RootUnknown;
    let resolved = resolve_root_trust_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RootTrustStripDegradeReason::RootTrustUnresolved)
    );
    assert_eq!(resolved.trust_disposition, None);
}

#[test]
fn strip_collapsed_degrades() {
    let mut input = clean_strip_input();
    input.reads_as_uniform_with_siblings = true;
    let resolved = resolve_root_trust_strip(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.collapses_per_root_into_uniform);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RootTrustStripDegradeReason::PerRootTrustCollapsedIntoUniform)
    );
}

#[test]
fn strip_root_identity_unstated_degrades() {
    let mut input = clean_strip_input();
    input.root_identity = "".to_owned();
    let resolved = resolve_root_trust_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RootTrustStripDegradeReason::RootIdentityUnstated)
    );
}

#[test]
fn strip_empty_id_and_forbidden_material_error() {
    let mut input = clean_strip_input();
    input.strip_id = "   ".to_owned();
    assert_eq!(
        resolve_root_trust_strip(input).unwrap_err(),
        M5WorkspaceTrustRootResolutionError::EmptyStripId
    );

    let mut input = clean_strip_input();
    input.strip_id = "root://leak".to_owned();
    assert_eq!(
        resolve_root_trust_strip(input).unwrap_err(),
        M5WorkspaceTrustRootResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_workspace_trust_root_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.vocabulary_set.trust_scopes.pop();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5WorkspaceTrustRootAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5WorkspaceTrustRootExportField::TrustDispositions);
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.controls_rows[0].root_trust_strip_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    // Force a clean banner to also read as collapsing mixed-root — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.workspace_trust_banner_examples[0].degrade_reason = None;
    row.workspace_trust_banner_examples[0].collapses_mixed_root_into_uniform = true;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_workspace_trust_root_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.implies_blanket_trust_across_roots_or_routes = true,
            1 => row.hides_grant_source_or_policy_epoch_in_menus_only = true,
            2 => row.collapses_mixed_root_into_uniform_trust = true,
            _ => row.hides_narrowed_capability_behind_generic_chrome = true,
        }
        assert!(packet
            .validate()
            .contains(&M5WorkspaceTrustRootControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn mixed_root_honesty_not_proven_when_collapse_example_removed() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    for row in &mut packet.controls_rows {
        row.workspace_trust_banner_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5WorkspaceTrustBannerDegradeReason::MixedRootCollapsedIntoUniform)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::MixedRootHonestyNotProven));
}

#[test]
fn mixed_root_honesty_not_proven_when_scope_uncovered() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    // Drop every clean mixed-root banner so the required scope coverage breaks.
    for row in &mut packet.controls_rows {
        row.workspace_trust_banner_examples
            .retain(|ex| !(ex.is_clean() && ex.trust_scope == "mixed_root"));
    }
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::MixedRootHonestyNotProven));
}

#[test]
fn trust_traceability_not_proven_when_detail_missing_example_removed() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    for row in &mut packet.controls_rows {
        row.workspace_trust_banner_examples.retain(|ex| {
            ex.degrade_reason != Some(M5WorkspaceTrustBannerDegradeReason::TrustDetailPathMissing)
        });
        row.root_trust_strip_examples.retain(|ex| {
            ex.degrade_reason != Some(M5RootTrustStripDegradeReason::TrustDetailPathMissing)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::TrustTraceabilityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet
        .governance_review
        .mixed_root_always_explicit_never_uniform = false;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet
        .consumer_projection
        .support_export_reads_single_trust_source = false;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5WorkspaceTrustRootControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_workspace_trust_root_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_workspace_trust_root_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_workspace_trust_root_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_workspace_trust_root_controls_export()
        .expect("checked M5 workspace-trust-root controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_WORKSPACE_TRUST_ROOT_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_workspace_trust_root_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_workspace_trust_root_controls_workspace_trust_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WorkspaceTrustRepairQualificationClass::Beta
    );

    let preview = seeded_m5_workspace_trust_root_controls_safe_mode_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::SafeModeUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WorkspaceTrustRepairQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5WorkspaceTrustRootControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-trust-banner-root-trust-strip-controls/workspace_trust_ui_beta_narrowed.json"
    )))
    .expect("workspace-trust-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_workspace_trust_root_controls_workspace_trust_ui_beta_narrowed()
    );

    let preview: M5WorkspaceTrustRootControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-trust-banner-root-trust-strip-controls/safe_mode_ui_preview_narrowed.json"
    )))
    .expect("safe-mode-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_workspace_trust_root_controls_safe_mode_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_the_two_trust_components() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5WorkspaceTrustRepairComponentFamily::WorkspaceTrustBanner,
            M5WorkspaceTrustRepairComponentFamily::RootTrustStrip,
        ]
    );
}
