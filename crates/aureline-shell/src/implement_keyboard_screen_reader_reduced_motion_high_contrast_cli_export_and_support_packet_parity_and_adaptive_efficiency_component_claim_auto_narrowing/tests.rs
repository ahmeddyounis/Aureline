//! Tests for the M05-1065 adaptive-efficiency component accessibility parity capstone:
//! the honest auto-narrowing logic, the per-family parity contract, and the checked-in
//! support export / CSV / report.

use super::*;

fn row(id: &str) -> EfficiencyAccessibilityRow {
    seeded_m5_efficiency_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_efficiency_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_efficiency_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5EfficiencyComponentFamily::ALL.len()
    );
    assert_eq!(packet.rows.len(), M5EfficiencyComponentFamily::ALL.len());
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_efficiency_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5EfficiencyClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_efficiency_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5EfficiencyAccessClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_efficiency_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5_EFFICIENCY_A11Y_CONSUMER_SURFACES {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_six_yellow_zero_red() {
    let packet = seeded_m5_efficiency_a11y_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC1: a stale / partial / deferred efficiency state can no longer keep an old
// full-truth label ---

#[test]
fn intact_power_state_indicator_is_full_truth_and_green() {
    let indicator = row("a11y:power-state-indicator");
    assert_eq!(
        indicator.full_support_claim,
        M5EfficiencyAccessClaim::FullTruth
    );
    assert_eq!(
        indicator.effective_claim(),
        M5EfficiencyAccessClaim::FullTruth
    );
    assert!(indicator.claim_narrow.is_none());
    assert_eq!(indicator.status(), EfficiencyAccessibilityStatus::Parity);
    assert!(indicator.effective_claim().asserts_live_truth());
}

#[test]
fn intact_background_banner_is_resolved_and_green() {
    let banner = row("a11y:background-work-banner");
    assert_eq!(
        banner.effective_claim(),
        M5EfficiencyAccessClaim::ResolvedTruth
    );
    assert!(banner.claim_narrow.is_none());
    assert_eq!(banner.status(), EfficiencyAccessibilityStatus::Parity);
    assert!(banner.effective_claim().asserts_full_self_sufficiency());
    assert!(!banner.effective_claim().asserts_live_truth());
}

#[test]
fn partial_throttled_row_narrows_to_degraded() {
    let row = row("a11y:throttled-subsystem-row");
    assert_eq!(row.effective_claim(), M5EfficiencyAccessClaim::Degraded);
    assert!(!row.effective_claim().asserts_full_self_sufficiency());
    let narrow = row.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5EfficiencyDowngradeTrigger::SlowedVersusPausedAmbiguous
    );
    assert_eq!(
        narrow.binding_dimension,
        M5EfficiencyClaimDimension::WorkDispositionTruth
    );
    assert!(row.claim_is_honest());
}

#[test]
fn deferred_background_row_narrows_to_deferred() {
    let row = row("a11y:background-work-row");
    assert_eq!(row.effective_claim(), M5EfficiencyAccessClaim::Deferred);
    assert!(!row.effective_claim().asserts_live_truth());
    assert!(row.claim_is_honest());
}

#[test]
fn stale_result_note_narrows_to_stale_shown() {
    let note = row("a11y:stale-result-continuity-note");
    assert_eq!(note.effective_claim(), M5EfficiencyAccessClaim::StaleShown);
    assert!(!note.effective_claim().asserts_full_self_sufficiency());
    assert!(note.claim_is_honest());
}

#[test]
fn policy_blocked_override_sheet_narrows_to_policy_blocked() {
    let sheet = row("a11y:per-workspace-override-sheet");
    assert_eq!(
        sheet.effective_claim(),
        M5EfficiencyAccessClaim::PolicyBlocked
    );
    assert!(!sheet.effective_claim().asserts_full_self_sufficiency());
    assert!(sheet.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a partial throttled row
    // claiming full truth.
    let mut row = row("a11y:throttled-subsystem-row");
    row.claim_narrow = None;
    assert!(!row.claim_is_honest());
    assert_eq!(row.status(), EfficiencyAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_intact_row_is_rejected() {
    let mut indicator = row("a11y:power-state-indicator");
    indicator.claim_narrow = Some(EfficiencyClaimAutoNarrow {
        narrowed_to: M5EfficiencyAccessClaim::Degraded,
        binding_dimension: M5EfficiencyClaimDimension::PressureSourceTruth,
        trigger: M5EfficiencyDowngradeTrigger::SourceOfChangeUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!indicator.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut row = row("a11y:throttled-subsystem-row");
    if let Some(narrow) = row.claim_narrow.as_mut() {
        narrow.binding_dimension = M5EfficiencyClaimDimension::PressureSourceTruth;
    }
    assert!(!row.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut row = row("a11y:throttled-subsystem-row");
    if let Some(narrow) = row.claim_narrow.as_mut() {
        narrow.trigger = M5EfficiencyDowngradeTrigger::SourceOfChangeUnstated;
    }
    assert!(!row.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut row = row("a11y:throttled-subsystem-row");
    if let Some(narrow) = row.claim_narrow.as_mut() {
        narrow.narrowed_label = "degraded".to_owned();
    }
    assert!(!row.claim_is_honest());
    // Generic low-power wording is also rejected.
    let mut row2 = row.clone();
    if let Some(narrow) = row2.claim_narrow.as_mut() {
        narrow.narrowed_label = "Low power".to_owned();
    }
    assert!(!row2.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5EfficiencyAccessClaim as S;
    use M5EfficiencyConditionState as C;
    assert_eq!(C::Intact.permitted_ceiling(), S::FullTruth);
    assert_eq!(C::Partial.permitted_ceiling(), S::Degraded);
    assert_eq!(C::Deferred.permitted_ceiling(), S::Deferred);
    assert_eq!(C::StaleShown.permitted_ceiling(), S::StaleShown);
    assert_eq!(C::PolicyBlocked.permitted_ceiling(), S::PolicyBlocked);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5EfficiencyClaimDimension as D;
    use M5EfficiencyDowngradeTrigger as T;
    assert_eq!(
        D::PressureSourceTruth.default_trigger(),
        T::SourceOfChangeUnstated
    );
    assert_eq!(
        D::WorkDispositionTruth.default_trigger(),
        T::SlowedVersusPausedAmbiguous
    );
    assert_eq!(
        D::OverrideAvailabilityTruth.default_trigger(),
        T::OverrideAvailabilityUnstated
    );
    assert_eq!(
        D::PolicyOwnerTruth.default_trigger(),
        T::PolicyOwnerUnstated
    );
    assert_eq!(
        D::ResumeBacklogTruth.default_trigger(),
        T::ResumeBacklogHidden
    );
    assert_eq!(
        D::StaleResultContinuityTruth.default_trigger(),
        T::StaleResultContinuityCleared
    );
}

// --- AC2: accessibility / CLI / export reach the same canonical truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_efficiency_a11y_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_override_sheet_binds_a_non_visual_fallback() {
    let sheet = row("a11y:per-workspace-override-sheet");
    assert!(sheet.is_hierarchy_heavy());
    assert!(sheet.has_non_visual_fallback());
    assert!(sheet
        .fallback_modalities
        .contains(&M5EfficiencyFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut indicator = row("a11y:power-state-indicator");
    indicator.keyboard_reach = EfficiencyNonVisualReachState::ViewOnlyTrap;
    assert!(!indicator.reaches_canonical_truth_via_at());
    assert_eq!(indicator.status(), EfficiencyAccessibilityStatus::Stranded);
}

#[test]
fn empty_efficiency_context_ref_strands_a_row() {
    let mut indicator = row("a11y:power-state-indicator");
    indicator.efficiency_context_ref = "  ".to_owned();
    assert!(!indicator.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut indicator = row("a11y:power-state-indicator");
    indicator.export_summary = EfficiencyExportSummaryState::AbsentNeedsScreenshot;
    assert!(!indicator.export_preserves_meaning());
    assert_eq!(indicator.status(), EfficiencyAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut indicator = row("a11y:power-state-indicator");
    indicator.copy_export.formats.retain(|f| f != "markdown");
    assert!(!indicator.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut row = row("a11y:throttled-subsystem-row");
    row.narrowing_disclosures.clear();
    assert!(!row.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut row = row("a11y:throttled-subsystem-row");
    row.narrowing_disclosures[0].state = EfficiencyNarrowingDisclosureState::SilentlyDropped;
    assert!(!row.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut row = row("a11y:throttled-subsystem-row");
    row.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!row.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut indicator = row("a11y:power-state-indicator");
    indicator
        .required_labels
        .retain(|l| *l != M5EfficiencyRequiredLabel::Identity);
    assert!(!indicator.preserves_mandatory_labels());
    assert_eq!(indicator.status(), EfficiencyAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_efficiency_a11y_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:throttled-subsystem-row");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EfficiencyAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_efficiency_a11y_packet();
    packet.rows[0].consumer_surfaces = vec![M5EfficiencyConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EfficiencyAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_efficiency_a11y_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EfficiencyAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_efficiency_a11y_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EfficiencyAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_efficiency_a11y_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EfficiencyAccessibilityViolation::RawEfficiencyMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:background-work-row").chip_tokens();
    assert!(chip.contains("family=background_work_row"));
    assert!(chip.contains("effective_claim=deferred"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_efficiency_a11y_packet();
    assert_eq!(packet.record_kind, EFFICIENCY_A11Y_RECORD_KIND);
    assert_eq!(packet.schema_version, EFFICIENCY_A11Y_SCHEMA_VERSION);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_efficiency_a11y_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_efficiency_a11y_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_efficiency_a11y_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_efficiency_a11y_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_efficiency_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-efficiency-component-accessibility-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_efficiency_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-efficiency-component-accessibility-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it
/// never runs in the normal suite. Run with
/// `GEN_EFFICIENCY_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_EFFICIENCY_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_efficiency_a11y_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-efficiency-component-accessibility-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-efficiency-component-accessibility-parity");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 adaptive-efficiency component accessibility parity fixtures\n\n\
         Mirror of `artifacts/release/m5-efficiency-component-accessibility-proof/`.\n\
         Regenerate with `GEN_EFFICIENCY_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
