use super::*;

fn clean_grid_input() -> M5TrustFactGridResolutionInput {
    M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:test".to_owned(),
        actor_identity: "actor: test".to_owned(),
        object_identity: "workspace: test-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        root_trust: M5RootTrustState::RootTrusted,
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

fn clean_sheet_input() -> M5TrustElevationSheetResolutionInput {
    M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:test".to_owned(),
        actor_identity: "actor: test".to_owned(),
        object_identity: "workspace: test-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_trust_fact_grid_elevation_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_PACKET_ID
    );
}

#[test]
fn grid_clean_names_facts_and_is_legible() {
    let resolved = resolve_trust_fact_grid(clean_grid_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.all_facts_named);
    assert!(!resolved.collapses_per_root_into_uniform);
    assert_eq!(resolved.trust_scope, "trusted_workspace");
    assert_eq!(resolved.root_trust, "root_trusted");
    assert_eq!(resolved.grant_source, "user_explicit");
    assert_eq!(
        resolved.trust_disposition,
        Some(M5WorkspaceTrustRepairDisposition::Trusted)
    );
    assert_eq!(
        resolved.next_action,
        M5TrustFactGridElevationNextAction::OpenTrustDetail
    );
}

#[test]
fn grid_object_unstated_degrades() {
    let mut input = clean_grid_input();
    input.object_identity = "  ".to_owned();
    let resolved = resolve_trust_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustFactGridDegradeReason::ObjectIdentityUnstated)
    );
}

#[test]
fn grid_actor_unstated_degrades() {
    let mut input = clean_grid_input();
    input.actor_identity = "".to_owned();
    let resolved = resolve_trust_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustFactGridDegradeReason::ActorIdentityUnstated)
    );
}

#[test]
fn grid_scope_unknown_degrades_and_has_no_disposition() {
    let mut input = clean_grid_input();
    input.trust_scope = M5TrustScopeState::ScopeUnknown;
    let resolved = resolve_trust_fact_grid(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustFactGridDegradeReason::TrustScopeUnresolved)
    );
    assert_eq!(resolved.trust_disposition, None);
}

#[test]
fn grid_capability_unstated_degrades() {
    let mut input = clean_grid_input();
    input.capability_narrow = M5CapabilityNarrowState::ExecutionBlocked;
    input.capability_narrow_stated = false;
    let resolved = resolve_trust_fact_grid(input).unwrap();
    assert!(resolved.capability_narrowed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustFactGridDegradeReason::NarrowedCapabilityUnstated)
    );
}

#[test]
fn grid_mixed_root_collapsed_degrades() {
    let mut input = clean_grid_input();
    input.trust_scope = M5TrustScopeState::MixedRoot;
    input.reads_as_uniform_trust = true;
    let resolved = resolve_trust_fact_grid(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.collapses_per_root_into_uniform);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustFactGridDegradeReason::MixedRootCollapsedIntoUniform)
    );
}

#[test]
fn grid_empty_id_and_forbidden_material_error() {
    let mut input = clean_grid_input();
    input.grid_id = "".to_owned();
    assert_eq!(
        resolve_trust_fact_grid(input).unwrap_err(),
        M5TrustFactGridElevationResolutionError::EmptyGridId
    );

    let mut input = clean_grid_input();
    input.object_identity = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_trust_fact_grid(input).unwrap_err(),
        M5TrustFactGridElevationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn sheet_clean_names_reduced_mode_and_effect() {
    let resolved = resolve_trust_elevation_sheet(clean_sheet_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.reviewed_before_approval);
    assert!(resolved.reduced_mode_alternative_stated);
    assert!(resolved.effect_lasting);
    assert!(!resolved.implies_ambient_scope);
    assert_eq!(resolved.effect_class, "lasting_until_revoked");
    assert_eq!(
        resolved.trust_disposition,
        Some(M5WorkspaceTrustRepairDisposition::Trusted)
    );
    assert_eq!(
        resolved.next_action,
        M5TrustFactGridElevationNextAction::OpenTrustDetail
    );
}

#[test]
fn sheet_trusted_root_scope_is_distinct_from_workspace() {
    let mut input = clean_sheet_input();
    input.trust_scope = M5TrustScopeState::TrustedRoot;
    input.effect_class = M5TrustElevationEffectClass::OneTimeThisSession;
    let resolved = resolve_trust_elevation_sheet(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(resolved.trust_scope, "trusted_root");
    assert!(!resolved.effect_lasting);
    // A trusted root still resolves to the trusted disposition but keeps its narrow scope token.
    assert_eq!(
        resolved.trust_disposition,
        Some(M5WorkspaceTrustRepairDisposition::Trusted)
    );
}

#[test]
fn sheet_capability_delta_missing_degrades() {
    let mut input = clean_sheet_input();
    input.capability_narrow = M5CapabilityNarrowState::ExtensionBlocked;
    input.capability_delta_stated = false;
    let resolved = resolve_trust_elevation_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustElevationSheetDegradeReason::CapabilityDeltaUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5TrustFactGridElevationNextAction::ReviewCapabilityDelta
    );
}

#[test]
fn sheet_reduced_mode_missing_degrades() {
    let mut input = clean_sheet_input();
    input.reduced_mode_alternative_stated = false;
    let resolved = resolve_trust_elevation_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustElevationSheetDegradeReason::ReducedModeAlternativeUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5TrustFactGridElevationNextAction::ContinueInReducedMode
    );
}

#[test]
fn sheet_effect_unknown_degrades() {
    let mut input = clean_sheet_input();
    input.effect_class = M5TrustElevationEffectClass::EffectUnknown;
    let resolved = resolve_trust_elevation_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustElevationSheetDegradeReason::EffectDurationUnstated)
    );
}

#[test]
fn sheet_ambient_scope_degrades() {
    let mut input = clean_sheet_input();
    input.implies_ambient_grant = true;
    let resolved = resolve_trust_elevation_sheet(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.implies_ambient_scope);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustElevationSheetDegradeReason::AmbientScopeImplied)
    );
}

#[test]
fn sheet_detail_missing_degrades() {
    let mut input = clean_sheet_input();
    input.detail_command_available = false;
    let resolved = resolve_trust_elevation_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TrustElevationSheetDegradeReason::TrustDetailPathMissing)
    );
}

#[test]
fn sheet_empty_id_and_forbidden_material_error() {
    let mut input = clean_sheet_input();
    input.sheet_id = "   ".to_owned();
    assert_eq!(
        resolve_trust_elevation_sheet(input).unwrap_err(),
        M5TrustFactGridElevationResolutionError::EmptySheetId
    );

    let mut input = clean_sheet_input();
    input.actor_identity = "bearer abc".to_owned();
    assert_eq!(
        resolve_trust_elevation_sheet(input).unwrap_err(),
        M5TrustFactGridElevationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_trust_fact_grid_elevation_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.vocabulary_set.effect_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_TRUST_FACT_GRID_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5TrustFactGridElevationAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5TrustFactGridElevationExportField::TrustDispositions);
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.controls_rows[0]
        .trust_elevation_sheet_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_sheet_example_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    // Force a clean sheet to also read as implying ambient scope — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.trust_elevation_sheet_examples[0].degrade_reason = None;
    row.trust_elevation_sheet_examples[0].implies_ambient_scope = true;
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.implies_ambient_or_inherited_grant_beyond_reviewed_object = true,
            1 => row.hides_policy_source_or_capability_delta_in_menus_only = true,
            2 => row.collapses_reduced_mode_alternative_into_generic_chrome = true,
            _ => row.collapses_effect_duration_into_generic_grant = true,
        }
        assert!(packet
            .validate()
            .contains(&M5TrustFactGridElevationControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn no_ambient_grant_not_proven_when_ambient_example_removed() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    for row in &mut packet.controls_rows {
        row.trust_elevation_sheet_examples.retain(|ex| {
            ex.degrade_reason != Some(M5TrustElevationSheetDegradeReason::AmbientScopeImplied)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::NoAmbientGrantNotProven));
}

#[test]
fn no_ambient_grant_not_proven_when_trusted_root_scope_uncovered() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    // Drop every clean trusted-root sheet so the required scope coverage breaks.
    for row in &mut packet.controls_rows {
        row.trust_elevation_sheet_examples
            .retain(|ex| !(ex.is_clean() && ex.trust_scope == "trusted_root"));
    }
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::NoAmbientGrantNotProven));
}

#[test]
fn field_parity_not_proven_when_reduced_mode_example_removed() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    for row in &mut packet.controls_rows {
        row.trust_elevation_sheet_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5TrustElevationSheetDegradeReason::ReducedModeAlternativeUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::FieldAndReducedModeParityNotProven));
}

#[test]
fn field_parity_not_proven_when_one_time_effect_uncovered() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    // Drop every clean one-time / single-action sheet so the lasting-versus-one-time grammar breaks.
    for row in &mut packet.controls_rows {
        row.trust_elevation_sheet_examples
            .retain(|ex| !ex.is_clean() || ex.effect_lasting);
    }
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::FieldAndReducedModeParityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet
        .governance_review
        .no_prompt_implies_ambient_grant_beyond_object = false;
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet
        .consumer_projection
        .scope_and_source_inspectable_before_approval = false;
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5TrustFactGridElevationControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_trust_fact_grid_elevation_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_trust_fact_grid_elevation_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_trust_fact_grid_elevation_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_trust_fact_grid_elevation_controls_export()
        .expect("checked M5 trust-fact-grid / trust-elevation-sheet controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_trust_fact_grid_elevation_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_trust_fact_grid_elevation_controls_workspace_trust_ui_beta_narrowed();
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

    let preview = seeded_m5_trust_fact_grid_elevation_controls_safe_mode_ui_preview_narrowed();
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
    let beta: M5TrustFactGridElevationControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-trust-fact-grid-trust-elevation-sheet-controls/workspace_trust_ui_beta_narrowed.json"
    )))
    .expect("workspace-trust-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_trust_fact_grid_elevation_controls_workspace_trust_ui_beta_narrowed()
    );

    let preview: M5TrustFactGridElevationControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-trust-fact-grid-trust-elevation-sheet-controls/safe_mode_ui_preview_narrowed.json"
    )))
    .expect("safe-mode-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_trust_fact_grid_elevation_controls_safe_mode_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_the_two_trust_review_components() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5WorkspaceTrustRepairComponentFamily::TrustFactGrid,
            M5WorkspaceTrustRepairComponentFamily::TrustElevationSheet,
        ]
    );
}

/// One-shot generator for the checked proof bundle and narrowed fixtures. Run
/// with `GEN_TRUST_ELEVATION_CONTROL_ARTIFACTS=1 cargo test -p aureline-shell
/// trust_fact_grid_and_trust_elevation_sheet::tests::generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_TRUST_ELEVATION_CONTROL_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_trust_fact_grid_elevation_controls();
    assert!(packet.validate().is_empty());
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let artifact_dir = manifest
        .join("../../artifacts/release/m5-trust-fact-grid-trust-elevation-sheet-controls-proof");
    fs::create_dir_all(&artifact_dir).expect("create trust-elevation artifact directory");
    fs::write(
        artifact_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write trust-elevation support export");
    fs::write(artifact_dir.join("matrix.csv"), packet.render_matrix_csv())
        .expect("write trust-elevation matrix");
    fs::write(
        artifact_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write trust-elevation summary");

    let fixture_dir =
        manifest.join("../../fixtures/ui/m5-trust-fact-grid-trust-elevation-sheet-controls");
    fs::create_dir_all(&fixture_dir).expect("create trust-elevation fixture directory");
    for (name, narrowed) in [
        (
            "workspace_trust_ui_beta_narrowed.json",
            seeded_m5_trust_fact_grid_elevation_controls_workspace_trust_ui_beta_narrowed(),
        ),
        (
            "safe_mode_ui_preview_narrowed.json",
            seeded_m5_trust_fact_grid_elevation_controls_safe_mode_ui_preview_narrowed(),
        ),
    ] {
        assert!(narrowed.validate().is_empty());
        fs::write(
            fixture_dir.join(name),
            format!(
                "{}\n",
                serde_json::to_string_pretty(&narrowed)
                    .expect("trust-elevation fixture serializes")
            ),
        )
        .expect("write trust-elevation fixture");
    }
}
