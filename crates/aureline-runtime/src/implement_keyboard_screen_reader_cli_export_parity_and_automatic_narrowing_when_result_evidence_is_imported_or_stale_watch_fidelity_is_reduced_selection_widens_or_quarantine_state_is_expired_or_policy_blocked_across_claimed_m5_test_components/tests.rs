//! Tests for the M05-914 test-explorer / watch / triage component accessibility fallback
//! capstone: the honest auto-narrowing logic, the per-family parity contract, no-loss result /
//! attempt-lineage integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> TestComponentAccessibilityRow {
    seeded_m5_test_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5TestExplorerWatchTriageComponentFamily::ALL.len()
    );
    // One row per frozen family covers the seven families end-to-end.
    assert_eq!(packet.rows.len(), 7);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5TestComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5TestComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_test_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5TestComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5TestConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_three_green_four_yellow_zero_red() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 3);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 7);
    assert_eq!(
        packet.summary.family_count,
        M5TestExplorerWatchTriageComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_preserved() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    assert!(packet.summary.all_lineage_preserved);
}

#[test]
fn two_families_are_hierarchy_heavy() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    assert_eq!(packet.summary.hierarchy_heavy_family_count, 2);
    assert!(packet.summary.all_hierarchy_heavy_have_non_visual_fallback);
}

// --- AC1: imported/stale, reduced watch, widened selection, expired/blocked quarantine
//        can no longer keep a trusted-live label ---

#[test]
fn live_tree_row_is_trusted_live_and_green() {
    let tree = row("a11y:test-tree-row");
    assert_eq!(
        tree.full_test_claim,
        M5TestComponentClaim::TrustedLiveResult
    );
    assert_eq!(
        tree.effective_claim(),
        M5TestComponentClaim::TrustedLiveResult
    );
    assert!(tree.claim_narrow.is_none());
    assert_eq!(tree.status(), TestComponentAccessibilityStatus::Parity);
    assert!(tree.effective_claim().asserts_trusted_live());
}

#[test]
fn reviewable_triage_panel_is_reviewable_and_green() {
    let panel = row("a11y:failure-triage-panel-reviewable");
    assert_eq!(
        panel.effective_claim(),
        M5TestComponentClaim::ReviewableResult
    );
    assert!(panel.claim_narrow.is_none());
    assert_eq!(panel.status(), TestComponentAccessibilityStatus::Parity);
    assert!(panel.effective_claim().asserts_full_result());
    assert!(!panel.effective_claim().asserts_trusted_live());
}

#[test]
fn imported_evidence_narrows_to_imported_or_stale_result() {
    let marker = row("a11y:inline-result-marker-imported");
    assert_eq!(
        marker.effective_claim(),
        M5TestComponentClaim::ImportedOrStaleResult
    );
    assert!(!marker.effective_claim().asserts_trusted_live());
    let narrow = marker.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(narrow.trigger, M5TestDowngradeTrigger::ResultOriginUnstated);
    assert_eq!(
        narrow.binding_dimension,
        M5TestComponentClaimDimension::ResultEvidence
    );
    assert!(marker.claim_is_honest());
}

#[test]
fn widened_rerun_narrows_to_widened_selection_result() {
    let bar = row("a11y:session-summary-bar-widened");
    assert_eq!(
        bar.effective_claim(),
        M5TestComponentClaim::WidenedSelectionResult
    );
    let narrow = bar.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(narrow.trigger, M5TestDowngradeTrigger::RerunScopeWidened);
    assert!(bar.claim_is_honest());
}

#[test]
fn reduced_watch_narrows_to_reduced_watch_result() {
    let banner = row("a11y:watch-mode-banner-reduced");
    assert_eq!(
        banner.effective_claim(),
        M5TestComponentClaim::ReducedWatchResult
    );
    let narrow = banner.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TestDowngradeTrigger::WatchFidelityUnstated
    );
    assert!(banner.claim_is_honest());
}

#[test]
fn expired_quarantine_narrows_to_restricted_quarantine_result() {
    let sheet = row("a11y:quarantine-review-sheet-restricted");
    assert_eq!(
        sheet.effective_claim(),
        M5TestComponentClaim::RestrictedQuarantineResult
    );
    assert!(!sheet.effective_claim().asserts_full_result());
    let narrow = sheet.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TestDowngradeTrigger::QuarantineReleaseImpactHidden
    );
    assert!(sheet.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: an imported marker claiming
    // TrustedLiveResult.
    let mut marker = row("a11y:inline-result-marker-imported");
    marker.claim_narrow = None;
    assert!(!marker.claim_is_honest());
    assert_eq!(marker.status(), TestComponentAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_live_row_is_rejected() {
    let mut tree = row("a11y:test-tree-row");
    tree.claim_narrow = Some(TestComponentClaimAutoNarrow {
        narrowed_to: M5TestComponentClaim::ImportedOrStaleResult,
        binding_dimension: M5TestComponentClaimDimension::ResultEvidence,
        trigger: M5TestDowngradeTrigger::ResultOriginUnstated,
        narrowed_label: "spurious narrowing with no weak dimension behind it".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!tree.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut marker = row("a11y:inline-result-marker-imported");
    if let Some(narrow) = marker.claim_narrow.as_mut() {
        narrow.binding_dimension = M5TestComponentClaimDimension::QuarantineVisibility;
    }
    assert!(!marker.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut marker = row("a11y:inline-result-marker-imported");
    if let Some(narrow) = marker.claim_narrow.as_mut() {
        narrow.trigger = M5TestDowngradeTrigger::WatchFidelityUnstated;
    }
    assert!(!marker.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut marker = row("a11y:inline-result-marker-imported");
    if let Some(narrow) = marker.claim_narrow.as_mut() {
        narrow.narrowed_label = "imported".to_owned();
    }
    assert!(!marker.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5TestComponentClaim as S;
    use M5TestComponentConditionState as C;
    assert_eq!(
        C::ResultsLiveExact.permitted_ceiling(),
        S::TrustedLiveResult
    );
    assert_eq!(
        C::EvidenceImportedOrStale.permitted_ceiling(),
        S::ImportedOrStaleResult
    );
    assert_eq!(
        C::WatchFidelityReduced.permitted_ceiling(),
        S::ReducedWatchResult
    );
    assert_eq!(
        C::SelectionWidened.permitted_ceiling(),
        S::WidenedSelectionResult
    );
    assert_eq!(
        C::QuarantineExpiredOrBlocked.permitted_ceiling(),
        S::RestrictedQuarantineResult
    );
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5TestComponentClaimDimension as D;
    use M5TestDowngradeTrigger as T;
    assert_eq!(D::ResultEvidence.default_trigger(), T::ResultOriginUnstated);
    assert_eq!(D::WatchFidelity.default_trigger(), T::WatchFidelityUnstated);
    assert_eq!(D::SelectionScope.default_trigger(), T::RerunScopeWidened);
    assert_eq!(
        D::QuarantineVisibility.default_trigger(),
        T::QuarantineReleaseImpactHidden
    );
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_test_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_environment_card_binds_a_non_visual_fallback() {
    let card = row("a11y:environment-matrix-card");
    assert!(card.is_hierarchy_heavy());
    assert!(card.has_non_visual_fallback());
    assert!(card
        .fallback_modalities
        .contains(&M5TestComponentFallbackModality::Structured));
}

#[test]
fn hierarchy_heavy_triage_panel_binds_a_non_visual_fallback() {
    let panel = row("a11y:failure-triage-panel-reviewable");
    assert!(panel.is_hierarchy_heavy());
    assert!(panel.has_non_visual_fallback());
    assert!(panel
        .fallback_modalities
        .contains(&M5TestComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut tree = row("a11y:test-tree-row");
    tree.keyboard_reach = TestComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!tree.reaches_canonical_truth_via_at());
    assert_eq!(tree.status(), TestComponentAccessibilityStatus::Stranded);
}

#[test]
fn empty_test_context_ref_strands_a_row() {
    let mut tree = row("a11y:test-tree-row");
    tree.test_context_ref = "  ".to_owned();
    assert!(!tree.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut tree = row("a11y:test-tree-row");
    tree.export_summary = TestComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!tree.export_preserves_meaning());
    assert_eq!(tree.status(), TestComponentAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut tree = row("a11y:test-tree-row");
    tree.copy_export.formats.retain(|f| f != "markdown");
    assert!(!tree.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut marker = row("a11y:inline-result-marker-imported");
    marker.lineage_preserved = false;
    assert!(!marker.preserves_lineage_continuity());
    assert_eq!(marker.status(), TestComponentAccessibilityStatus::Stranded);
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut marker = row("a11y:inline-result-marker-imported");
    if let Some(narrow) = marker.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!marker.preserves_lineage_continuity());
    assert!(!marker.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut marker = row("a11y:inline-result-marker-imported");
    marker.narrowing_disclosures.clear();
    assert!(!marker.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut marker = row("a11y:inline-result-marker-imported");
    marker.narrowing_disclosures[0].state = TestComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!marker.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut marker = row("a11y:inline-result-marker-imported");
    marker.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!marker.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut tree = row("a11y:test-tree-row");
    tree.required_labels
        .retain(|l| *l != M5TestRequiredLabel::Identity);
    assert!(!tree.preserves_mandatory_labels());
    assert_eq!(tree.status(), TestComponentAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_test_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:environment-matrix-card");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        TestComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_test_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5TestConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        TestComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_test_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestComponentAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_test_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_test_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        TestComponentAccessibilityViolation::RawTestMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:inline-result-marker-imported").chip_tokens();
    assert!(chip.contains("family=inline_result_marker"));
    assert!(chip.contains("effective_claim=imported_or_stale_result"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    assert_eq!(packet.record_kind, TEST_COMPONENT_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_test_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_test_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_test_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_test_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_test_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_TEST_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-runtime generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_TEST_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_test_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest).join(
        "../../artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback",
    );
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest).join(
            "../../artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback.md",
        ),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-test-explorer-watch-triage-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
