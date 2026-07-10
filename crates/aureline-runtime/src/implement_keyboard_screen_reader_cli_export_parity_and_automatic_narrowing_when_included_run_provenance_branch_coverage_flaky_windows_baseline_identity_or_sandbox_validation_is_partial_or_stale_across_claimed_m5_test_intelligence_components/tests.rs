//! Tests for the M05-1034 test-intelligence component accessibility fallback capstone: the
//! honest auto-narrowing logic, the per-family parity contract, no-loss provenance / baseline /
//! assumption lineage integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> IntelComponentAccessibilityRow {
    seeded_m5_test_intel_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5TestIntelligenceComponentFamily::ALL.len()
    );
    // One row per frozen family covers the seven families end-to-end.
    assert_eq!(packet.rows.len(), 7);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5IntelComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5IntelComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_evidence_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5IntelComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5TestIntelligenceConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_five_yellow_zero_red() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 5);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 7);
    assert_eq!(
        packet.summary.family_count,
        M5TestIntelligenceComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_preserved() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    assert!(packet.summary.all_lineage_preserved);
}

#[test]
fn two_families_are_hierarchy_heavy() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    assert_eq!(packet.summary.hierarchy_heavy_family_count, 2);
    assert!(packet.summary.all_hierarchy_heavy_have_non_visual_fallback);
}

// --- AC1: imported/stale provenance, partial branch coverage, insufficient flaky windows,
//        unverified baselines, unproven sandbox validation can no longer keep a
//        verified-current label ---

#[test]
fn live_retry_row_is_verified_current_and_green() {
    let retry = row("a11y:retry-history-row-current");
    assert_eq!(
        retry.full_test_claim,
        M5IntelComponentClaim::VerifiedCurrentEvidence
    );
    assert_eq!(
        retry.effective_claim(),
        M5IntelComponentClaim::VerifiedCurrentEvidence
    );
    assert!(retry.claim_narrow.is_none());
    assert_eq!(retry.status(), IntelComponentAccessibilityStatus::Parity);
    assert!(retry.effective_claim().asserts_verified_current());
}

#[test]
fn reviewable_snapshot_card_is_reviewable_and_green() {
    let card = row("a11y:snapshot-review-card-reviewable");
    assert_eq!(
        card.effective_claim(),
        M5IntelComponentClaim::ReviewableEvidence
    );
    assert!(card.claim_narrow.is_none());
    assert_eq!(card.status(), IntelComponentAccessibilityStatus::Parity);
    assert!(card.effective_claim().asserts_full_evidence());
    assert!(!card.effective_claim().asserts_verified_current());
}

#[test]
fn imported_provenance_narrows_to_imported_or_stale_result() {
    let bar = row("a11y:coverage-summary-bar-imported");
    assert_eq!(
        bar.effective_claim(),
        M5IntelComponentClaim::ImportedOrStaleEvidence
    );
    assert!(!bar.effective_claim().asserts_verified_current());
    let narrow = bar.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TestIntelligenceDowngradeTrigger::ProvenanceClassUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5IntelComponentClaimDimension::IncludedRunProvenance
    );
    assert!(bar.claim_is_honest());
}

#[test]
fn partial_branch_coverage_narrows_to_partial_condition_result() {
    let overlay = row("a11y:coverage-overlay-marker-partial");
    assert_eq!(
        overlay.effective_claim(),
        M5IntelComponentClaim::PartialConditionEvidence
    );
    let narrow = overlay.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TestIntelligenceDowngradeTrigger::LineVersusBranchUnstated
    );
    assert!(overlay.claim_is_honest());
}

#[test]
fn insufficient_flaky_window_narrows_to_unconfirmed_flaky_result() {
    let badge = row("a11y:flaky-state-badge-unconfirmed");
    assert_eq!(
        badge.effective_claim(),
        M5IntelComponentClaim::UnconfirmedFlakyEvidence
    );
    let narrow = badge.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TestIntelligenceDowngradeTrigger::FlakyConfidenceOverstated
    );
    assert!(badge.claim_is_honest());
}

#[test]
fn unverified_baseline_narrows_to_unverified_baseline_result() {
    let sheet = row("a11y:coverage-import-merge-sheet-unverified");
    assert_eq!(
        sheet.effective_claim(),
        M5IntelComponentClaim::UnverifiedBaselineEvidence
    );
    let narrow = sheet.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TestIntelligenceDowngradeTrigger::SnapshotBaselineUnstated
    );
    assert!(sheet.claim_is_honest());
}

#[test]
fn unproven_sandbox_narrows_to_unvalidated_generated_result() {
    let gen = row("a11y:test-generation-suggestion-card-unvalidated");
    assert_eq!(
        gen.effective_claim(),
        M5IntelComponentClaim::UnvalidatedGeneratedEvidence
    );
    assert!(!gen.effective_claim().asserts_full_evidence());
    let narrow = gen.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TestIntelligenceDowngradeTrigger::GeneratedAssumptionHidden
    );
    assert!(gen.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: an imported coverage bar claiming
    // VerifiedCurrentEvidence.
    let mut bar = row("a11y:coverage-summary-bar-imported");
    bar.claim_narrow = None;
    assert!(!bar.claim_is_honest());
    assert_eq!(bar.status(), IntelComponentAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_live_row_is_rejected() {
    let mut retry = row("a11y:retry-history-row-current");
    retry.claim_narrow = Some(IntelComponentClaimAutoNarrow {
        narrowed_to: M5IntelComponentClaim::ImportedOrStaleEvidence,
        binding_dimension: M5IntelComponentClaimDimension::IncludedRunProvenance,
        trigger: M5TestIntelligenceDowngradeTrigger::ProvenanceClassUnstated,
        narrowed_label: "spurious narrowing with no weak dimension behind it".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!retry.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut bar = row("a11y:coverage-summary-bar-imported");
    if let Some(narrow) = bar.claim_narrow.as_mut() {
        narrow.binding_dimension = M5IntelComponentClaimDimension::SandboxValidation;
    }
    assert!(!bar.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut bar = row("a11y:coverage-summary-bar-imported");
    if let Some(narrow) = bar.claim_narrow.as_mut() {
        narrow.trigger = M5TestIntelligenceDowngradeTrigger::LineVersusBranchUnstated;
    }
    assert!(!bar.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut bar = row("a11y:coverage-summary-bar-imported");
    if let Some(narrow) = bar.claim_narrow.as_mut() {
        narrow.narrowed_label = "imported".to_owned();
    }
    assert!(!bar.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5IntelComponentClaim as S;
    use M5IntelComponentConditionState as C;
    assert_eq!(
        C::EvidenceCurrentExact.permitted_ceiling(),
        S::VerifiedCurrentEvidence
    );
    assert_eq!(
        C::ProvenanceImportedOrStale.permitted_ceiling(),
        S::ImportedOrStaleEvidence
    );
    assert_eq!(
        C::BranchConditionPartial.permitted_ceiling(),
        S::PartialConditionEvidence
    );
    assert_eq!(
        C::FlakyWindowInsufficient.permitted_ceiling(),
        S::UnconfirmedFlakyEvidence
    );
    assert_eq!(
        C::BaselineIdentityUnverified.permitted_ceiling(),
        S::UnverifiedBaselineEvidence
    );
    assert_eq!(
        C::SandboxValidationUnproven.permitted_ceiling(),
        S::UnvalidatedGeneratedEvidence
    );
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5IntelComponentClaimDimension as D;
    use M5TestIntelligenceDowngradeTrigger as T;
    assert_eq!(
        D::IncludedRunProvenance.default_trigger(),
        T::ProvenanceClassUnstated
    );
    assert_eq!(
        D::BranchConditionCoverage.default_trigger(),
        T::LineVersusBranchUnstated
    );
    assert_eq!(
        D::FlakyEvidenceWindow.default_trigger(),
        T::FlakyConfidenceOverstated
    );
    assert_eq!(
        D::BaselineScopeIdentity.default_trigger(),
        T::SnapshotBaselineUnstated
    );
    assert_eq!(
        D::SandboxValidation.default_trigger(),
        T::GeneratedAssumptionHidden
    );
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_test_intel_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_merge_sheet_binds_a_non_visual_fallback() {
    let sheet = row("a11y:coverage-import-merge-sheet-unverified");
    assert!(sheet.is_hierarchy_heavy());
    assert!(sheet.has_non_visual_fallback());
    assert!(sheet
        .fallback_modalities
        .contains(&M5IntelComponentFallbackModality::Structured));
}

#[test]
fn hierarchy_heavy_snapshot_card_binds_a_non_visual_fallback() {
    let card = row("a11y:snapshot-review-card-reviewable");
    assert!(card.is_hierarchy_heavy());
    assert!(card.has_non_visual_fallback());
    assert!(card
        .fallback_modalities
        .contains(&M5IntelComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut retry = row("a11y:retry-history-row-current");
    retry.keyboard_reach = IntelComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!retry.reaches_canonical_truth_via_at());
    assert_eq!(retry.status(), IntelComponentAccessibilityStatus::Stranded);
}

#[test]
fn empty_test_context_ref_strands_a_row() {
    let mut retry = row("a11y:retry-history-row-current");
    retry.test_context_ref = "  ".to_owned();
    assert!(!retry.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut retry = row("a11y:retry-history-row-current");
    retry.export_summary = IntelComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!retry.export_preserves_meaning());
    assert_eq!(retry.status(), IntelComponentAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut retry = row("a11y:retry-history-row-current");
    retry.copy_export.formats.retain(|f| f != "markdown");
    assert!(!retry.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut bar = row("a11y:coverage-summary-bar-imported");
    bar.lineage_preserved = false;
    assert!(!bar.preserves_lineage_continuity());
    assert_eq!(bar.status(), IntelComponentAccessibilityStatus::Stranded);
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut bar = row("a11y:coverage-summary-bar-imported");
    if let Some(narrow) = bar.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!bar.preserves_lineage_continuity());
    assert!(!bar.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut bar = row("a11y:coverage-summary-bar-imported");
    bar.narrowing_disclosures.clear();
    assert!(!bar.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut bar = row("a11y:coverage-summary-bar-imported");
    bar.narrowing_disclosures[0].state = IntelComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!bar.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut bar = row("a11y:coverage-summary-bar-imported");
    bar.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!bar.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut retry = row("a11y:retry-history-row-current");
    retry
        .required_labels
        .retain(|l| *l != M5TestIntelligenceRequiredLabel::Identity);
    assert!(!retry.preserves_mandatory_labels());
    assert_eq!(retry.status(), IntelComponentAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:test-generation-suggestion-card-unvalidated");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        IntelComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5TestIntelligenceConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        IntelComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelComponentAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        IntelComponentAccessibilityViolation::RawTestMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:coverage-summary-bar-imported").chip_tokens();
    assert!(chip.contains("family=coverage_summary_bar"));
    assert!(chip.contains("effective_claim=imported_or_stale_evidence"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        TEST_INTEL_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_test_intel_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(
        packet,
        seeded_m5_test_intel_component_a11y_fallback_packet()
    );
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_test_intel_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-intelligence-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_test_intel_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-intelligence-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_TEST_INTEL_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-runtime generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_TEST_INTEL_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_test_intel_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-test-intelligence-component-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest).join(
            "../../artifacts/release/m5-test-intelligence-component-accessibility-fallback.md",
        ),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-test-intelligence-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
