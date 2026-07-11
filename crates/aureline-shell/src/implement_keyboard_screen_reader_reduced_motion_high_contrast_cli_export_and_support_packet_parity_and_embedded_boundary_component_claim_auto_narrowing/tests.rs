//! Tests for the M05-1073 embedded-boundary component accessibility parity capstone:
//! the honest auto-narrowing logic, the per-family parity contract, and the checked-in
//! support export / CSV / report.

use super::*;

fn row(id: &str) -> EmbeddedBoundaryAccessibilityRow {
    seeded_m5_embedded_boundary_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5EmbeddedBoundaryComponentFamily::ALL.len()
    );
    assert_eq!(
        packet.rows.len(),
        M5EmbeddedBoundaryComponentFamily::ALL.len()
    );
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5EmbeddedClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5EmbeddedAccessClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5_EMBEDDED_BOUNDARY_A11Y_CONSUMER_SURFACES {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_six_yellow_zero_red() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC1: a stale / offline / provider-blocked / partial boundary state can no longer
// keep an old fresh first-party full-truth label ---

#[test]
fn intact_docs_pane_header_is_full_truth_and_green() {
    let header = row("a11y:docs-pane-header");
    assert_eq!(header.full_support_claim, M5EmbeddedAccessClaim::FullTruth);
    assert_eq!(header.effective_claim(), M5EmbeddedAccessClaim::FullTruth);
    assert!(header.claim_narrow.is_none());
    assert_eq!(header.status(), EmbeddedAccessibilityStatus::Parity);
    assert!(header.effective_claim().asserts_live_truth());
}

#[test]
fn intact_origin_bar_is_resolved_and_green() {
    let bar = row("a11y:embedded-origin-bar");
    assert_eq!(bar.effective_claim(), M5EmbeddedAccessClaim::ResolvedTruth);
    assert!(bar.claim_narrow.is_none());
    assert_eq!(bar.status(), EmbeddedAccessibilityStatus::Parity);
    assert!(bar.effective_claim().asserts_full_self_sufficiency());
    assert!(!bar.effective_claim().asserts_live_truth());
}

#[test]
fn partial_boundary_fact_grid_narrows_to_degraded() {
    let grid = row("a11y:boundary-fact-grid");
    assert_eq!(grid.effective_claim(), M5EmbeddedAccessClaim::Degraded);
    assert!(!grid.effective_claim().asserts_full_self_sufficiency());
    let narrow = grid.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5EmbeddedDowngradeTrigger::DataBoundaryUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5EmbeddedClaimDimension::DataBoundaryTruth
    );
    assert!(grid.claim_is_honest());
}

#[test]
fn stale_remote_dashboard_header_narrows_to_stale() {
    let header = row("a11y:remote-service-dashboard-header");
    assert_eq!(header.effective_claim(), M5EmbeddedAccessClaim::Stale);
    assert!(!header.effective_claim().asserts_live_truth());
    assert!(header.claim_is_honest());
}

#[test]
fn offline_auth_handoff_card_narrows_to_offline() {
    let card = row("a11y:auth-handoff-card");
    assert_eq!(card.effective_claim(), M5EmbeddedAccessClaim::Offline);
    assert!(!card.effective_claim().asserts_full_self_sufficiency());
    assert!(card.claim_is_honest());
}

#[test]
fn provider_blocked_embedded_state_panel_narrows_to_provider_blocked() {
    let panel = row("a11y:embedded-state-panel");
    assert_eq!(
        panel.effective_claim(),
        M5EmbeddedAccessClaim::ProviderBlocked
    );
    assert!(!panel.effective_claim().asserts_full_self_sufficiency());
    assert!(panel.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a partial boundary-fact grid
    // claiming full truth.
    let mut grid = row("a11y:boundary-fact-grid");
    grid.claim_narrow = None;
    assert!(!grid.claim_is_honest());
    assert_eq!(grid.status(), EmbeddedAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_intact_row_is_rejected() {
    let mut header = row("a11y:docs-pane-header");
    header.claim_narrow = Some(EmbeddedClaimAutoNarrow {
        narrowed_to: M5EmbeddedAccessClaim::Degraded,
        binding_dimension: M5EmbeddedClaimDimension::FreshnessTruth,
        trigger: M5EmbeddedDowngradeTrigger::FreshnessOrLastUpdatedUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!header.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut grid = row("a11y:boundary-fact-grid");
    if let Some(narrow) = grid.claim_narrow.as_mut() {
        narrow.binding_dimension = M5EmbeddedClaimDimension::OwnerOriginTruth;
    }
    assert!(!grid.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut grid = row("a11y:boundary-fact-grid");
    if let Some(narrow) = grid.claim_narrow.as_mut() {
        narrow.trigger = M5EmbeddedDowngradeTrigger::OwnerOrOriginUnstated;
    }
    assert!(!grid.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut grid = row("a11y:boundary-fact-grid");
    if let Some(narrow) = grid.claim_narrow.as_mut() {
        narrow.narrowed_label = "degraded".to_owned();
    }
    assert!(!grid.claim_is_honest());
    // Generic offline / stale wording is also rejected.
    let mut grid2 = grid.clone();
    if let Some(narrow) = grid2.claim_narrow.as_mut() {
        narrow.narrowed_label = "Offline".to_owned();
    }
    assert!(!grid2.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5EmbeddedAccessClaim as S;
    use M5EmbeddedConditionState as C;
    assert_eq!(C::Intact.permitted_ceiling(), S::FullTruth);
    assert_eq!(C::Partial.permitted_ceiling(), S::Degraded);
    assert_eq!(C::Stale.permitted_ceiling(), S::Stale);
    assert_eq!(C::Offline.permitted_ceiling(), S::Offline);
    assert_eq!(C::ProviderBlocked.permitted_ceiling(), S::ProviderBlocked);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5EmbeddedClaimDimension as D;
    use M5EmbeddedDowngradeTrigger as T;
    assert_eq!(
        D::OwnerOriginTruth.default_trigger(),
        T::OwnerOrOriginUnstated
    );
    assert_eq!(
        D::DataBoundaryTruth.default_trigger(),
        T::DataBoundaryUnstated
    );
    assert_eq!(
        D::BrowserFallbackTruth.default_trigger(),
        T::BrowserFallbackHiddenInMenusOnly
    );
    assert_eq!(
        D::CapabilityLimitTruth.default_trigger(),
        T::CapabilityLimitsUnstated
    );
    assert_eq!(
        D::FreshnessTruth.default_trigger(),
        T::FreshnessOrLastUpdatedUnstated
    );
    assert_eq!(
        D::AccountScopeTruth.default_trigger(),
        T::AccountScopeUnstated
    );
}

// --- AC2: accessibility / CLI / export reach the same canonical truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_embedded_boundary_a11y_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_boundary_fact_grid_binds_a_non_visual_fallback() {
    let grid = row("a11y:boundary-fact-grid");
    assert!(grid.is_hierarchy_heavy());
    assert!(grid.has_non_visual_fallback());
    assert!(grid
        .fallback_modalities
        .contains(&M5EmbeddedFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut header = row("a11y:docs-pane-header");
    header.keyboard_reach = EmbeddedNonVisualReachState::ViewOnlyTrap;
    assert!(!header.reaches_canonical_truth_via_at());
    assert_eq!(header.status(), EmbeddedAccessibilityStatus::Stranded);
}

#[test]
fn empty_boundary_context_ref_strands_a_row() {
    let mut header = row("a11y:docs-pane-header");
    header.boundary_context_ref = "  ".to_owned();
    assert!(!header.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut header = row("a11y:docs-pane-header");
    header.export_summary = EmbeddedExportSummaryState::AbsentNeedsScreenshot;
    assert!(!header.export_preserves_meaning());
    assert_eq!(header.status(), EmbeddedAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut header = row("a11y:docs-pane-header");
    header.copy_export.formats.retain(|f| f != "markdown");
    assert!(!header.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut grid = row("a11y:boundary-fact-grid");
    grid.narrowing_disclosures.clear();
    assert!(!grid.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut grid = row("a11y:boundary-fact-grid");
    grid.narrowing_disclosures[0].state = EmbeddedNarrowingDisclosureState::SilentlyDropped;
    assert!(!grid.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut grid = row("a11y:boundary-fact-grid");
    grid.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!grid.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut header = row("a11y:docs-pane-header");
    header
        .required_labels
        .retain(|l| *l != M5EmbeddedRequiredLabel::Identity);
    assert!(!header.preserves_mandatory_labels());
    assert_eq!(header.status(), EmbeddedAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_embedded_boundary_a11y_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:boundary-fact-grid");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedBoundaryAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_embedded_boundary_a11y_packet();
    packet.rows[0].consumer_surfaces = vec![M5EmbeddedConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedBoundaryAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_embedded_boundary_a11y_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedBoundaryAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_embedded_boundary_a11y_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedBoundaryAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_embedded_boundary_a11y_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedBoundaryAccessibilityViolation::RawEmbeddedMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:auth-handoff-card").chip_tokens();
    assert!(chip.contains("family=auth_handoff_card"));
    assert!(chip.contains("effective_claim=offline"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    assert_eq!(packet.record_kind, EMBEDDED_BOUNDARY_A11Y_RECORD_KIND);
    assert_eq!(packet.schema_version, EMBEDDED_BOUNDARY_A11Y_SCHEMA_VERSION);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_embedded_boundary_a11y_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_embedded_boundary_a11y_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_embedded_boundary_a11y_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_embedded_boundary_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-embedded-boundary-component-accessibility-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_embedded_boundary_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-embedded-boundary-component-accessibility-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it
/// never runs in the normal suite. Run with
/// `GEN_EMBEDDED_BOUNDARY_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_EMBEDDED_BOUNDARY_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_embedded_boundary_a11y_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-embedded-boundary-component-accessibility-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-embedded-boundary-component-accessibility-parity");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 embedded-boundary component accessibility parity fixtures\n\n\
         Mirror of `artifacts/release/m5-embedded-boundary-component-accessibility-proof/`.\n\
         Regenerate with `GEN_EMBEDDED_BOUNDARY_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
