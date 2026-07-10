//! Tests for the M05-1042 framework-component accessibility fallback capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the
//! unproven-version-range/unlinked-source/heuristic-inference/partial-generator-effect-never-exact
//! guarantee, no-loss pack-source / recovery-boundary integrity, and the checked-in support export
//! / CSV / report.

use super::*;

fn row(id: &str) -> FrameworkComponentAccessibilityRow {
    seeded_m5_framework_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5FrameworkComponentFamily::ALL.len()
    );
    // Seven rows cover the seven families one-to-one (one certified fully green — the verified
    // run-config scaffold card — the other six narrowed-yellow).
    assert_eq!(packet.rows.len(), 7);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5FrameworkComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5FrameworkComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_a_permitted_ceiling() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    let ceilings = packet.represented_ceiling_claims();
    for claim in M5FrameworkComponentClaim::ALL {
        assert!(
            ceilings.contains(&claim),
            "claim tier {} missing from ceiling claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5FrameworkConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_six_yellow_zero_red() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 7);
    assert_eq!(
        packet.summary.family_count,
        M5FrameworkComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_source_recovery_and_exactness_honesty() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    assert!(packet.summary.all_source_and_recovery_preserved);
    assert!(packet.summary.all_exactness_honesty_holds);
}

// --- AC1: heuristic / unproven / partial can no longer keep exact framework truth ---

#[test]
fn verified_run_config_card_is_exact_and_green() {
    let card = row("a11y:run-config-scaffold-card-verified");
    assert_eq!(
        card.full_framework_claim,
        M5FrameworkComponentClaim::ExactFrameworkTruth
    );
    assert_eq!(
        card.effective_claim(),
        M5FrameworkComponentClaim::ExactFrameworkTruth
    );
    assert!(card.claim_narrow.is_none());
    assert_eq!(card.status(), FrameworkComponentAccessibilityStatus::Parity);
    assert!(card.effective_claim().asserts_exact_framework_truth());
}

#[test]
fn unverified_pack_header_narrows_to_unverified_pack_projection() {
    let header = row("a11y:framework-pack-header-unverified");
    assert_eq!(
        header.effective_claim(),
        M5FrameworkComponentClaim::UnverifiedPackProjection
    );
    assert!(!header.effective_claim().asserts_exact_framework_truth());
    let narrow = header.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5FrameworkDowngradeTrigger::SupportClassUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5FrameworkComponentClaimDimension::PackHealthIntegrity
    );
    assert!(header.claim_is_honest());
    // Pack health / support is an honest support-status disclosure, not an exactness overstatement.
    assert!(header.exactness_honesty_holds());
    // The pack header also carries its supported-version-range weakness as a secondary condition.
    assert_eq!(
        header.condition_for(M5FrameworkComponentClaimDimension::SupportedVersionRange),
        M5FrameworkComponentConditionState::VersionRangeUnproven
    );
}

#[test]
fn heuristic_route_narrows_to_heuristic_inference_projection() {
    let route = row("a11y:route-endpoint-row-heuristic");
    assert_eq!(
        route.effective_claim(),
        M5FrameworkComponentClaim::HeuristicInferenceProjection
    );
    assert!(!route.effective_claim().asserts_exact_framework_truth());
    let narrow = route.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated
    );
    assert!(route.claim_is_honest());
    // A heuristic route must never be shown as exact framework truth.
    assert!(route.exactness_honesty_holds());
}

#[test]
fn unlinked_tree_node_narrows_to_unlinked_source_projection() {
    let node = row("a11y:component-service-tree-node-unlinked");
    assert_eq!(
        node.effective_claim(),
        M5FrameworkComponentClaim::UnlinkedSourceProjection
    );
    let narrow = node.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted
    );
    assert_eq!(
        narrow.binding_dimension,
        M5FrameworkComponentClaimDimension::ProvingSourceLinkage
    );
    assert!(node.claim_is_honest());
    assert!(node.exactness_honesty_holds());
}

#[test]
fn heuristic_convention_narrows_to_heuristic_inference_projection() {
    let diag = row("a11y:convention-diagnostic-row-heuristic");
    assert_eq!(
        diag.effective_claim(),
        M5FrameworkComponentClaim::HeuristicInferenceProjection
    );
    assert!(diag.claim_is_honest());
    assert!(diag.exactness_honesty_holds());
}

#[test]
fn partial_generator_narrows_to_partial_generator_effect_projection() {
    let sheet = row("a11y:generator-preview-sheet-partial");
    assert_eq!(
        sheet.effective_claim(),
        M5FrameworkComponentClaim::PartialGeneratorEffectProjection
    );
    let narrow = sheet.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5FrameworkDowngradeTrigger::ImpactUndisclosed
    );
    assert!(sheet.claim_is_honest());
    assert!(sheet.exactness_honesty_holds());
}

#[test]
fn derived_banner_narrows_to_unlinked_source_projection() {
    let banner = row("a11y:derived-relationship-banner-unlinked");
    assert_eq!(
        banner.effective_claim(),
        M5FrameworkComponentClaim::UnlinkedSourceProjection
    );
    assert!(!banner.effective_claim().asserts_exact_framework_truth());
    assert!(banner.claim_is_honest());
    assert!(banner.exactness_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a heuristic route with no narrow.
    let mut route = row("a11y:route-endpoint-row-heuristic");
    route.claim_narrow = None;
    assert!(!route.claim_is_honest());
    assert_eq!(
        route.status(),
        FrameworkComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn unprovable_state_shown_as_exact_is_rejected() {
    // A heuristic route whose narrow claims ExactFrameworkTruth violates exactness honesty.
    let mut route = row("a11y:route-endpoint-row-heuristic");
    if let Some(narrow) = route.claim_narrow.as_mut() {
        narrow.narrowed_to = M5FrameworkComponentClaim::ExactFrameworkTruth;
    }
    assert!(!route.exactness_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_framework_component_a11y_fallback_packet();
        let idx = packet
            .rows
            .iter()
            .position(|r| r.row_id == "a11y:route-endpoint-row-heuristic")
            .expect("row present");
        packet.rows[idx] = route;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        FrameworkComponentAccessibilityViolation::UnprovableStateShownAsExact { .. }
    )));
}

#[test]
fn exactness_honesty_unproven_when_no_unprovable_row() {
    let mut packet = seeded_m5_framework_component_a11y_fallback_packet();
    // Drop every row carrying an unproven-version-range / unlinked-source / heuristic-inference /
    // partial-generator-effect state, keeping only the green card and the pack-health header.
    packet.rows.retain(|r| {
        r.row_id == "a11y:run-config-scaffold-card-verified"
            || r.row_id == "a11y:framework-pack-header-unverified"
    });
    // Also strip the pack header's version-range weakness so no cannot-be-proven-exact state remains.
    if let Some(header) = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "a11y:framework-pack-header-unverified")
    {
        header
            .claim_conditions
            .retain(|c| c.dimension != M5FrameworkComponentClaimDimension::SupportedVersionRange);
    }
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        FrameworkComponentAccessibilityViolation::ExactnessHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_verified_row_is_rejected() {
    let mut card = row("a11y:run-config-scaffold-card-verified");
    card.claim_narrow = Some(FrameworkComponentClaimAutoNarrow {
        narrowed_to: M5FrameworkComponentClaim::HeuristicInferenceProjection,
        binding_dimension: M5FrameworkComponentClaimDimension::HeuristicInferenceBoundary,
        trigger: M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        narrowed_label: "spurious narrowing that should not exist here".to_owned(),
        preserves_canonical_identity: true,
        preserves_source_and_recovery: true,
    });
    assert!(!card.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut route = row("a11y:route-endpoint-row-heuristic");
    if let Some(narrow) = route.claim_narrow.as_mut() {
        narrow.binding_dimension = M5FrameworkComponentClaimDimension::ProvingSourceLinkage;
    }
    assert!(!route.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut route = row("a11y:route-endpoint-row-heuristic");
    if let Some(narrow) = route.claim_narrow.as_mut() {
        narrow.trigger = M5FrameworkDowngradeTrigger::PackIdentityUnstated;
    }
    assert!(!route.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut route = row("a11y:route-endpoint-row-heuristic");
    if let Some(narrow) = route.claim_narrow.as_mut() {
        narrow.narrowed_label = "heuristic".to_owned();
    }
    assert!(!route.claim_is_honest());
}

#[test]
fn multi_condition_pack_header_binds_the_lowest_ceiling() {
    // The pack header carries both pack-health-unproven (rank 3) and version-range-unproven
    // (rank 4); the lower-rank pack-health ceiling binds.
    let header = row("a11y:framework-pack-header-unverified");
    assert_eq!(
        header.permitted_claim(),
        M5FrameworkComponentClaim::UnverifiedPackProjection
    );
    assert_eq!(
        header.binding_dimension(),
        Some(M5FrameworkComponentClaimDimension::PackHealthIntegrity)
    );
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5FrameworkComponentClaim as S;
    use M5FrameworkComponentConditionState as C;
    assert_eq!(
        C::FrameworkVerifiedExact.permitted_ceiling(),
        S::ExactFrameworkTruth
    );
    assert_eq!(
        C::PackHealthUnproven.permitted_ceiling(),
        S::UnverifiedPackProjection
    );
    assert_eq!(
        C::VersionRangeUnproven.permitted_ceiling(),
        S::UnprovenVersionRangeProjection
    );
    assert_eq!(
        C::SourceLinkageUnproven.permitted_ceiling(),
        S::UnlinkedSourceProjection
    );
    assert_eq!(
        C::HeuristicInferenceOnly.permitted_ceiling(),
        S::HeuristicInferenceProjection
    );
    assert_eq!(
        C::GeneratorEffectPartial.permitted_ceiling(),
        S::PartialGeneratorEffectProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5FrameworkComponentConditionState as C;
    use M5FrameworkDowngradeTrigger as T;
    assert_eq!(
        C::PackHealthUnproven.default_trigger(),
        T::SupportClassUnstated
    );
    assert_eq!(
        C::VersionRangeUnproven.default_trigger(),
        T::PackIdentityUnstated
    );
    assert_eq!(
        C::SourceLinkageUnproven.default_trigger(),
        T::ProvingSourceOmitted
    );
    assert_eq!(
        C::HeuristicInferenceOnly.default_trigger(),
        T::ExactVersusHeuristicUnstated
    );
    assert_eq!(
        C::GeneratorEffectPartial.default_trigger(),
        T::ImpactUndisclosed
    );
}

#[test]
fn cannot_be_proven_states_are_flagged() {
    use M5FrameworkComponentConditionState as C;
    assert!(C::VersionRangeUnproven.cannot_be_proven_exact());
    assert!(C::SourceLinkageUnproven.cannot_be_proven_exact());
    assert!(C::HeuristicInferenceOnly.cannot_be_proven_exact());
    assert!(C::GeneratorEffectPartial.cannot_be_proven_exact());
    assert!(!C::PackHealthUnproven.cannot_be_proven_exact());
    assert!(!C::FrameworkVerifiedExact.cannot_be_proven_exact());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_framework_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_tree_node_binds_a_non_visual_fallback() {
    let node = row("a11y:component-service-tree-node-unlinked");
    assert!(node.is_hierarchy_heavy());
    assert!(node.has_non_visual_fallback());
    assert!(node
        .fallback_modalities
        .contains(&M5FrameworkComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut card = row("a11y:run-config-scaffold-card-verified");
    card.keyboard_reach = FrameworkComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!card.reaches_canonical_truth_via_at());
    assert_eq!(
        card.status(),
        FrameworkComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn cli_trap_strands_a_row() {
    let mut route = row("a11y:route-endpoint-row-heuristic");
    route.cli_reach = FrameworkComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!route.reaches_canonical_truth_via_at());
    assert_eq!(
        route.status(),
        FrameworkComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_framework_context_ref_strands_a_row() {
    let mut card = row("a11y:run-config-scaffold-card-verified");
    card.framework_context_ref = "  ".to_owned();
    assert!(!card.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_raw_value_is_rejected() {
    let mut card = row("a11y:run-config-scaffold-card-verified");
    card.export_summary = FrameworkComponentExportSummaryState::RequiresRawValue;
    assert!(!card.export_preserves_meaning());
    assert_eq!(
        card.status(),
        FrameworkComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut card = row("a11y:run-config-scaffold-card-verified");
    card.copy_export.formats.retain(|f| f != "markdown");
    assert!(!card.export_preserves_meaning());
}

#[test]
fn dropped_source_and_recovery_strands_a_row() {
    let mut node = row("a11y:component-service-tree-node-unlinked");
    node.source_and_recovery_preserved = false;
    assert!(!node.preserves_source_and_recovery_continuity());
    assert_eq!(
        node.status(),
        FrameworkComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_source_recovery_continuity_strands_a_row() {
    let mut node = row("a11y:component-service-tree-node-unlinked");
    if let Some(narrow) = node.claim_narrow.as_mut() {
        narrow.preserves_source_and_recovery = false;
    }
    assert!(!node.preserves_source_and_recovery_continuity());
    assert!(!node.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut route = row("a11y:route-endpoint-row-heuristic");
    route.narrowing_disclosures.clear();
    assert!(!route.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut route = row("a11y:route-endpoint-row-heuristic");
    route.narrowing_disclosures[0].state =
        FrameworkComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!route.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut route = row("a11y:route-endpoint-row-heuristic");
    route.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!route.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut card = row("a11y:run-config-scaffold-card-verified");
    card.required_labels
        .retain(|l| *l != M5FrameworkRequiredLabel::Identity);
    assert!(!card.preserves_mandatory_labels());
    assert_eq!(
        card.status(),
        FrameworkComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_framework_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.component_family != M5FrameworkComponentFamily::DerivedRelationshipBanner);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        FrameworkComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_framework_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5FrameworkConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        FrameworkComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_framework_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        FrameworkComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_framework_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, FrameworkComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_framework_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        FrameworkComponentAccessibilityViolation::RawFrameworkMaterialInExport
    )));
}

#[test]
fn secret_token_is_forbidden_material() {
    // The framework vocabulary never legitimately names a raw secret, so a leaked "secret" token
    // is forbidden material.
    let mut packet = seeded_m5_framework_component_a11y_fallback_packet();
    packet.rows[0].source_refs.push("secret=abc123".to_owned());
    assert!(json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn seeded_packet_carries_no_forbidden_material() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:route-endpoint-row-heuristic").chip_tokens();
    assert!(chip.contains("family=route_endpoint_row"));
    assert!(chip.contains("effective_claim=heuristic_inference_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        FRAMEWORK_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_framework_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_framework_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_framework_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-framework-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_framework_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-framework-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_FRAMEWORK_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-templates generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_FRAMEWORK_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_framework_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-framework-component-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/release/m5-framework-component-accessibility-fallback.md"),
        &report,
    )
    .expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-framework-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
