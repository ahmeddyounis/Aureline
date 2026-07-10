//! Tests for the M05-1026 scaffold-component accessibility fallback capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the
//! drifted-template/partial-generation/unchecked-validation-never-qualified guarantee, no-loss
//! starter-source / recovery-boundary integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> ScaffoldComponentAccessibilityRow {
    seeded_m5_scaffold_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5ScaffoldComponentFamily::ALL.len()
    );
    // Six rows cover the six families one-to-one (one certified fully green — the verified
    // scaffold template card — the other five narrowed-yellow).
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5ScaffoldComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5ScaffoldComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_result_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5ScaffoldComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5ScaffoldConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_five_yellow_zero_red() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 5);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(
        packet.summary.family_count,
        M5ScaffoldComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_source_recovery_and_readiness_honesty() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    assert!(packet.summary.all_source_and_recovery_preserved);
    assert!(packet.summary.all_readiness_honesty_holds);
}

// --- AC1: drifted / partial / unchecked / secret-bound / blocked can no longer keep qualified ---

#[test]
fn verified_template_card_is_qualified_and_green() {
    let card = row("a11y:scaffold-template-card-verified");
    assert_eq!(
        card.full_scaffold_claim,
        M5ScaffoldComponentClaim::QualifiedStarter
    );
    assert_eq!(
        card.effective_claim(),
        M5ScaffoldComponentClaim::QualifiedStarter
    );
    assert!(card.claim_narrow.is_none());
    assert_eq!(card.status(), ScaffoldComponentAccessibilityStatus::Parity);
    assert!(card.effective_claim().asserts_qualified_starter());
}

#[test]
fn secret_bound_parameter_narrows_to_secret_bound_projection() {
    let row = row("a11y:starter-parameter-row-secret-bound");
    assert_eq!(
        row.effective_claim(),
        M5ScaffoldComponentClaim::SecretBoundParameterProjection
    );
    assert!(!row.effective_claim().asserts_qualified_starter());
    let narrow = row.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ScaffoldDowngradeTrigger::ParameterSourceUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5ScaffoldComponentClaimDimension::ParameterPortability
    );
    assert!(row.claim_is_honest());
    // A secret-bound parameter is an honest privacy operation, not a readiness overstatement.
    assert!(row.readiness_honesty_holds());
}

#[test]
fn blocked_prerequisite_narrows_to_blocked_prerequisite_projection() {
    let card = row("a11y:scaffold-preflight-card-blocked");
    assert_eq!(
        card.effective_claim(),
        M5ScaffoldComponentClaim::BlockedPrerequisiteProjection
    );
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.binding_dimension,
        M5ScaffoldComponentClaimDimension::PrerequisiteHealth
    );
    assert!(card.claim_is_honest());
    // A blocked prerequisite is an operational block, not a readiness overstatement.
    assert!(card.readiness_honesty_holds());
}

#[test]
fn drifted_template_narrows_to_drifted_template_projection() {
    let row = row("a11y:template-health-row-drifted");
    assert_eq!(
        row.effective_claim(),
        M5ScaffoldComponentClaim::DriftedTemplateProjection
    );
    assert!(!row.effective_claim().asserts_qualified_starter());
    let narrow = row.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ScaffoldDowngradeTrigger::HealthFreshnessStale
    );
    assert!(row.claim_is_honest());
    // A drifted template must never be shown as a fully qualified starter.
    assert!(row.readiness_honesty_holds());
}

#[test]
fn partial_generation_narrows_to_partial_generation_projection() {
    let card = row("a11y:generated-project-diff-card-partial");
    assert_eq!(
        card.effective_claim(),
        M5ScaffoldComponentClaim::PartialGenerationProjection
    );
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ScaffoldDowngradeTrigger::GeneratedBoundaryBlurred
    );
    assert!(card.claim_is_honest());
    assert!(card.readiness_honesty_holds());
}

#[test]
fn unchecked_validation_narrows_to_unchecked_validation_projection() {
    let banner = row("a11y:scaffold-handoff-banner-unchecked");
    assert_eq!(
        banner.effective_claim(),
        M5ScaffoldComponentClaim::UncheckedValidationProjection
    );
    assert!(!banner.effective_claim().asserts_qualified_starter());
    let narrow = banner.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ScaffoldDowngradeTrigger::HealthFreshnessStale
    );
    assert!(banner.claim_is_honest());
    assert!(banner.readiness_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a drifted template claiming
    // QualifiedStarter.
    let mut row = row("a11y:template-health-row-drifted");
    row.claim_narrow = None;
    assert!(!row.claim_is_honest());
    assert_eq!(row.status(), ScaffoldComponentAccessibilityStatus::Stranded);
}

#[test]
fn unprovable_state_shown_as_qualified_is_rejected() {
    // A drifted-template row whose narrow claims QualifiedStarter violates readiness honesty.
    let mut health = row("a11y:template-health-row-drifted");
    if let Some(narrow) = health.claim_narrow.as_mut() {
        narrow.narrowed_to = M5ScaffoldComponentClaim::QualifiedStarter;
    }
    assert!(!health.readiness_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_scaffold_component_a11y_fallback_packet();
        let idx = packet
            .rows
            .iter()
            .position(|r| r.row_id == "a11y:template-health-row-drifted")
            .expect("row present");
        packet.rows[idx] = health;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        ScaffoldComponentAccessibilityViolation::UnprovableStateShownAsQualified { .. }
    )));
}

#[test]
fn readiness_honesty_unproven_when_no_unprovable_row() {
    let mut packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    // Drop the three drifted-template / partial-generation / unchecked-validation rows.
    packet.rows.retain(|r| {
        r.row_id != "a11y:template-health-row-drifted"
            && r.row_id != "a11y:generated-project-diff-card-partial"
            && r.row_id != "a11y:scaffold-handoff-banner-unchecked"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ScaffoldComponentAccessibilityViolation::ReadinessHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_verified_row_is_rejected() {
    let mut card = row("a11y:scaffold-template-card-verified");
    card.claim_narrow = Some(ScaffoldComponentClaimAutoNarrow {
        narrowed_to: M5ScaffoldComponentClaim::UncheckedValidationProjection,
        binding_dimension: M5ScaffoldComponentClaimDimension::HandoffValidationClarity,
        trigger: M5ScaffoldDowngradeTrigger::HealthFreshnessStale,
        narrowed_label: "spurious narrowing that should not exist here".to_owned(),
        preserves_canonical_identity: true,
        preserves_source_and_recovery: true,
    });
    assert!(!card.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut row = row("a11y:template-health-row-drifted");
    if let Some(narrow) = row.claim_narrow.as_mut() {
        narrow.binding_dimension = M5ScaffoldComponentClaimDimension::ParameterPortability;
    }
    assert!(!row.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut row = row("a11y:template-health-row-drifted");
    if let Some(narrow) = row.claim_narrow.as_mut() {
        narrow.trigger = M5ScaffoldDowngradeTrigger::ParameterSourceUnstated;
    }
    assert!(!row.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut row = row("a11y:template-health-row-drifted");
    if let Some(narrow) = row.claim_narrow.as_mut() {
        narrow.narrowed_label = "drifted".to_owned();
    }
    assert!(!row.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5ScaffoldComponentClaim as S;
    use M5ScaffoldComponentConditionState as C;
    assert_eq!(
        C::StarterVerifiedReady.permitted_ceiling(),
        S::QualifiedStarter
    );
    assert_eq!(
        C::SecretBoundParameter.permitted_ceiling(),
        S::SecretBoundParameterProjection
    );
    assert_eq!(
        C::PrerequisiteBlocked.permitted_ceiling(),
        S::BlockedPrerequisiteProjection
    );
    assert_eq!(
        C::FreshnessDrifted.permitted_ceiling(),
        S::DriftedTemplateProjection
    );
    assert_eq!(
        C::GenerationDiffPartial.permitted_ceiling(),
        S::PartialGenerationProjection
    );
    assert_eq!(
        C::ValidationStale.permitted_ceiling(),
        S::UncheckedValidationProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5ScaffoldComponentConditionState as C;
    use M5ScaffoldDowngradeTrigger as T;
    assert_eq!(
        C::SecretBoundParameter.default_trigger(),
        T::ParameterSourceUnstated
    );
    assert_eq!(
        C::PrerequisiteBlocked.default_trigger(),
        T::HostBoundaryUnstated
    );
    assert_eq!(
        C::FreshnessDrifted.default_trigger(),
        T::HealthFreshnessStale
    );
    assert_eq!(
        C::GenerationDiffPartial.default_trigger(),
        T::GeneratedBoundaryBlurred
    );
    assert_eq!(
        C::ValidationStale.default_trigger(),
        T::HealthFreshnessStale
    );
}

#[test]
fn cannot_be_proven_states_are_flagged() {
    use M5ScaffoldComponentConditionState as C;
    assert!(C::FreshnessDrifted.cannot_be_proven_qualified());
    assert!(C::GenerationDiffPartial.cannot_be_proven_qualified());
    assert!(C::ValidationStale.cannot_be_proven_qualified());
    assert!(!C::SecretBoundParameter.cannot_be_proven_qualified());
    assert!(!C::PrerequisiteBlocked.cannot_be_proven_qualified());
    assert!(!C::StarterVerifiedReady.cannot_be_proven_qualified());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_scaffold_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_generated_diff_binds_a_non_visual_fallback() {
    let card = row("a11y:generated-project-diff-card-partial");
    assert!(card.is_hierarchy_heavy());
    assert!(card.has_non_visual_fallback());
    assert!(card
        .fallback_modalities
        .contains(&M5ScaffoldComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut card = row("a11y:scaffold-template-card-verified");
    card.keyboard_reach = ScaffoldComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!card.reaches_canonical_truth_via_at());
    assert_eq!(
        card.status(),
        ScaffoldComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn cli_trap_strands_a_row() {
    let mut row = row("a11y:starter-parameter-row-secret-bound");
    row.cli_reach = ScaffoldComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!row.reaches_canonical_truth_via_at());
    assert_eq!(row.status(), ScaffoldComponentAccessibilityStatus::Stranded);
}

#[test]
fn empty_scaffold_context_ref_strands_a_row() {
    let mut card = row("a11y:scaffold-template-card-verified");
    card.scaffold_context_ref = "  ".to_owned();
    assert!(!card.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_raw_value_is_rejected() {
    let mut card = row("a11y:scaffold-template-card-verified");
    card.export_summary = ScaffoldComponentExportSummaryState::RequiresRawValue;
    assert!(!card.export_preserves_meaning());
    assert_eq!(
        card.status(),
        ScaffoldComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut card = row("a11y:scaffold-template-card-verified");
    card.copy_export.formats.retain(|f| f != "markdown");
    assert!(!card.export_preserves_meaning());
}

#[test]
fn dropped_source_and_recovery_strands_a_row() {
    let mut card = row("a11y:generated-project-diff-card-partial");
    card.source_and_recovery_preserved = false;
    assert!(!card.preserves_source_and_recovery_continuity());
    assert_eq!(
        card.status(),
        ScaffoldComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_source_recovery_continuity_strands_a_row() {
    let mut card = row("a11y:generated-project-diff-card-partial");
    if let Some(narrow) = card.claim_narrow.as_mut() {
        narrow.preserves_source_and_recovery = false;
    }
    assert!(!card.preserves_source_and_recovery_continuity());
    assert!(!card.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut row = row("a11y:template-health-row-drifted");
    row.narrowing_disclosures.clear();
    assert!(!row.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut row = row("a11y:template-health-row-drifted");
    row.narrowing_disclosures[0].state = ScaffoldComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!row.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut row = row("a11y:template-health-row-drifted");
    row.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!row.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut card = row("a11y:scaffold-template-card-verified");
    card.required_labels
        .retain(|l| *l != M5ScaffoldRequiredLabel::Identity);
    assert!(!card.preserves_mandatory_labels());
    assert_eq!(
        card.status(),
        ScaffoldComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.component_family != M5ScaffoldComponentFamily::ScaffoldHandoffBanner);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ScaffoldComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5ScaffoldConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ScaffoldComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ScaffoldComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ScaffoldComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ScaffoldComponentAccessibilityViolation::RawStarterMaterialInExport
    )));
}

#[test]
fn secret_reference_token_is_not_forbidden_material() {
    // The governed `secret_reference` parameter source layer is a legitimate token, not a leaked
    // raw value — the seeded packet must stay clean.
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:template-health-row-drifted").chip_tokens();
    assert!(chip.contains("family=template_health_row"));
    assert!(chip.contains("effective_claim=drifted_template_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        SCAFFOLD_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_scaffold_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_scaffold_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_scaffold_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-scaffold-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_scaffold_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-scaffold-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_SCAFFOLD_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-templates generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_SCAFFOLD_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_scaffold_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-scaffold-component-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/release/m5-scaffold-component-accessibility-fallback.md"),
        &report,
    )
    .expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-scaffold-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
