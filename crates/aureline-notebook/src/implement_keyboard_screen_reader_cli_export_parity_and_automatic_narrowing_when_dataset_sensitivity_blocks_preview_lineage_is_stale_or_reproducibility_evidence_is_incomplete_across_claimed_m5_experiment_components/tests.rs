//! Tests for the M05-1018 experiment-component accessibility fallback capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the
//! partial-fingerprint/incomplete-comparison/stale-lineage/severed-provenance-never-comparable
//! guarantee, no-loss experiment-lineage integrity, and the checked-in support export / CSV /
//! report.

use super::*;

fn row(id: &str) -> ExperimentComponentAccessibilityRow {
    seeded_m5_experiment_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5ExperimentComponentFamily::ALL.len()
    );
    // Eight rows cover the eight families one-to-one (two certified fully green — the live
    // experiment run row and the reviewable result summary card — the other six narrowed-yellow).
    assert_eq!(packet.rows.len(), 8);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5ExperimentComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5ExperimentComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_result_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5ExperimentComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5ExperimentConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_six_yellow_zero_red() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 8);
    assert_eq!(
        packet.summary.family_count,
        M5ExperimentComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_and_comparable_result_honesty() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    assert!(packet.summary.all_lineage_preserved);
    assert!(packet.summary.all_comparable_result_honesty_holds);
}

// --- AC1: partial / incomplete / stale / severed / blocked can no longer keep exact comparable ---

#[test]
fn reviewable_result_summary_is_reviewable_and_green() {
    let card = row("a11y:result-summary-card-reviewable");
    assert_eq!(
        card.effective_claim(),
        M5ExperimentComponentClaim::ReviewableResult
    );
    assert!(card.claim_narrow.is_none());
    assert_eq!(
        card.status(),
        ExperimentComponentAccessibilityStatus::Parity
    );
    assert!(card.effective_claim().asserts_trustworthy_result());
    assert!(!card.effective_claim().asserts_exact_comparable_result());
}

#[test]
fn live_run_row_is_exact_comparable_and_green() {
    let run = row("a11y:experiment-run-row-live");
    assert_eq!(
        run.full_experiment_claim,
        M5ExperimentComponentClaim::ExactComparableResult
    );
    assert_eq!(
        run.effective_claim(),
        M5ExperimentComponentClaim::ExactComparableResult
    );
    assert!(run.claim_narrow.is_none());
    assert_eq!(run.status(), ExperimentComponentAccessibilityStatus::Parity);
    assert!(run.effective_claim().asserts_exact_comparable_result());
}

#[test]
fn partial_fingerprint_narrows_to_partial_fingerprint_projection() {
    let card = row("a11y:environment-fingerprint-card-partial");
    assert_eq!(
        card.effective_claim(),
        M5ExperimentComponentClaim::PartialFingerprintProjection
    );
    assert!(!card.effective_claim().asserts_exact_comparable_result());
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ExperimentDowngradeTrigger::EnvironmentFingerprintUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5ExperimentComponentClaimDimension::EnvironmentFingerprint
    );
    assert!(card.claim_is_honest());
    // A partial fingerprint cannot be proven an exact comparable result.
    assert!(card.comparable_result_honesty_holds());
}

#[test]
fn incomplete_comparison_narrows_to_incomparable_runs_projection() {
    let table = row("a11y:run-comparison-table-incomplete");
    assert_eq!(
        table.effective_claim(),
        M5ExperimentComponentClaim::IncomparableRunsProjection
    );
    assert!(!table.effective_claim().asserts_exact_comparable_result());
    let narrow = table.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ExperimentDowngradeTrigger::ComparabilityOverstated
    );
    assert!(table.claim_is_honest());
    assert!(table.comparable_result_honesty_holds());
}

#[test]
fn blocked_compare_guard_narrows_to_guard_blocked_projection() {
    let banner = row("a11y:compare-guard-banner-blocked");
    assert_eq!(
        banner.effective_claim(),
        M5ExperimentComponentClaim::GuardBlockedProjection
    );
    let narrow = banner.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.binding_dimension,
        M5ExperimentComponentClaimDimension::CompareGuardClearance
    );
    assert!(banner.claim_is_honest());
    // A blocked guard is an operational state, not an exactness overstatement.
    assert!(banner.comparable_result_honesty_holds());
}

#[test]
fn stale_lineage_narrows_to_stale_lineage_projection() {
    let panel = row("a11y:artifact-lineage-panel-stale");
    assert_eq!(
        panel.effective_claim(),
        M5ExperimentComponentClaim::StaleLineageProjection
    );
    let narrow = panel.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ExperimentDowngradeTrigger::CachedStateHidden
    );
    assert!(panel.claim_is_honest());
    // A stale lineage must never be shown as an exact comparable result.
    assert!(panel.comparable_result_honesty_holds());
}

#[test]
fn severed_provenance_narrows_to_unprovenanced_data_projection() {
    let card = row("a11y:dataset-provenance-card-severed");
    assert_eq!(
        card.effective_claim(),
        M5ExperimentComponentClaim::UnprovenancedDataProjection
    );
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ExperimentDowngradeTrigger::DatasetProvenanceSevered
    );
    assert!(card.claim_is_honest());
    assert!(card.comparable_result_honesty_holds());
}

#[test]
fn sensitivity_blocks_preview_narrows_to_blocked_preview_projection() {
    let banner = row("a11y:sensitivity-sharing-banner-blocked-preview");
    assert_eq!(
        banner.effective_claim(),
        M5ExperimentComponentClaim::BlockedPreviewProjection
    );
    assert!(!banner.effective_claim().asserts_trustworthy_result());
    let narrow = banner.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ExperimentDowngradeTrigger::SensitivityClassUnstated
    );
    assert!(banner.claim_is_honest());
    // Blocking preview for sensitivity is honest privacy narrowing, not an exactness overstatement.
    assert!(banner.comparable_result_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a partial fingerprint claiming
    // ExactComparableResult.
    let mut card = row("a11y:environment-fingerprint-card-partial");
    card.claim_narrow = None;
    assert!(!card.claim_is_honest());
    assert_eq!(
        card.status(),
        ExperimentComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn unprovable_state_shown_as_comparable_is_rejected() {
    // A stale-lineage row whose narrow claims ExactComparableResult violates comparable-result
    // honesty.
    let mut panel = row("a11y:artifact-lineage-panel-stale");
    if let Some(narrow) = panel.claim_narrow.as_mut() {
        narrow.narrowed_to = M5ExperimentComponentClaim::ExactComparableResult;
    }
    assert!(!panel.comparable_result_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_experiment_component_a11y_fallback_packet();
        let idx = packet
            .rows
            .iter()
            .position(|r| r.row_id == "a11y:artifact-lineage-panel-stale")
            .expect("row present");
        packet.rows[idx] = panel;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        ExperimentComponentAccessibilityViolation::UnprovableStateShownAsComparable { .. }
    )));
}

#[test]
fn comparable_result_honesty_unproven_when_no_unprovable_row() {
    let mut packet = seeded_m5_experiment_component_a11y_fallback_packet();
    // Drop the four partial-fingerprint / incomplete-comparison / stale-lineage /
    // severed-provenance rows.
    packet.rows.retain(|r| {
        r.row_id != "a11y:environment-fingerprint-card-partial"
            && r.row_id != "a11y:run-comparison-table-incomplete"
            && r.row_id != "a11y:artifact-lineage-panel-stale"
            && r.row_id != "a11y:dataset-provenance-card-severed"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExperimentComponentAccessibilityViolation::ComparableResultHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_exact_row_is_rejected() {
    let mut run = row("a11y:experiment-run-row-live");
    run.claim_narrow = Some(ExperimentComponentClaimAutoNarrow {
        narrowed_to: M5ExperimentComponentClaim::BlockedPreviewProjection,
        binding_dimension: M5ExperimentComponentClaimDimension::SensitivityDisclosure,
        trigger: M5ExperimentDowngradeTrigger::SensitivityClassUnstated,
        narrowed_label: "spurious narrowing that should not exist here".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!run.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut card = row("a11y:environment-fingerprint-card-partial");
    if let Some(narrow) = card.claim_narrow.as_mut() {
        narrow.binding_dimension = M5ExperimentComponentClaimDimension::DatasetProvenance;
    }
    assert!(!card.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut card = row("a11y:environment-fingerprint-card-partial");
    if let Some(narrow) = card.claim_narrow.as_mut() {
        narrow.trigger = M5ExperimentDowngradeTrigger::DatasetProvenanceSevered;
    }
    assert!(!card.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut card = row("a11y:environment-fingerprint-card-partial");
    if let Some(narrow) = card.claim_narrow.as_mut() {
        narrow.narrowed_label = "partial".to_owned();
    }
    assert!(!card.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5ExperimentComponentClaim as S;
    use M5ExperimentComponentConditionState as C;
    assert_eq!(
        C::LiveExactResult.permitted_ceiling(),
        S::ExactComparableResult
    );
    assert_eq!(
        C::FingerprintPartial.permitted_ceiling(),
        S::PartialFingerprintProjection
    );
    assert_eq!(
        C::ComparabilityIncomplete.permitted_ceiling(),
        S::IncomparableRunsProjection
    );
    assert_eq!(
        C::CompareGuardBlocked.permitted_ceiling(),
        S::GuardBlockedProjection
    );
    assert_eq!(
        C::LineageStale.permitted_ceiling(),
        S::StaleLineageProjection
    );
    assert_eq!(
        C::ProvenanceSevered.permitted_ceiling(),
        S::UnprovenancedDataProjection
    );
    assert_eq!(
        C::SensitivityBlocksPreview.permitted_ceiling(),
        S::BlockedPreviewProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5ExperimentComponentConditionState as C;
    use M5ExperimentDowngradeTrigger as T;
    assert_eq!(
        C::FingerprintPartial.default_trigger(),
        T::EnvironmentFingerprintUnstated
    );
    assert_eq!(
        C::ComparabilityIncomplete.default_trigger(),
        T::ComparabilityOverstated
    );
    assert_eq!(
        C::CompareGuardBlocked.default_trigger(),
        T::ComparabilityOverstated
    );
    assert_eq!(C::LineageStale.default_trigger(), T::CachedStateHidden);
    assert_eq!(
        C::ProvenanceSevered.default_trigger(),
        T::DatasetProvenanceSevered
    );
    assert_eq!(
        C::SensitivityBlocksPreview.default_trigger(),
        T::SensitivityClassUnstated
    );
}

#[test]
fn cannot_be_proven_states_are_flagged() {
    use M5ExperimentComponentConditionState as C;
    assert!(C::FingerprintPartial.cannot_be_proven_exact());
    assert!(C::ComparabilityIncomplete.cannot_be_proven_exact());
    assert!(C::LineageStale.cannot_be_proven_exact());
    assert!(C::ProvenanceSevered.cannot_be_proven_exact());
    assert!(!C::CompareGuardBlocked.cannot_be_proven_exact());
    assert!(!C::SensitivityBlocksPreview.cannot_be_proven_exact());
    assert!(!C::LiveExactResult.cannot_be_proven_exact());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_experiment_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_artifact_lineage_binds_a_non_visual_fallback() {
    let panel = row("a11y:artifact-lineage-panel-stale");
    assert!(panel.is_hierarchy_heavy());
    assert!(panel.has_non_visual_fallback());
    assert!(panel
        .fallback_modalities
        .contains(&M5ExperimentComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut run = row("a11y:experiment-run-row-live");
    run.keyboard_reach = ExperimentComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!run.reaches_canonical_truth_via_at());
    assert_eq!(
        run.status(),
        ExperimentComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn cli_trap_strands_a_row() {
    let mut card = row("a11y:dataset-provenance-card-severed");
    card.cli_reach = ExperimentComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!card.reaches_canonical_truth_via_at());
    assert_eq!(
        card.status(),
        ExperimentComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_experiment_context_ref_strands_a_row() {
    let mut run = row("a11y:experiment-run-row-live");
    run.experiment_context_ref = "  ".to_owned();
    assert!(!run.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_raw_payload_is_rejected() {
    let mut run = row("a11y:experiment-run-row-live");
    run.export_summary = ExperimentComponentExportSummaryState::RequiresRawPayload;
    assert!(!run.export_preserves_meaning());
    assert_eq!(
        run.status(),
        ExperimentComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut run = row("a11y:experiment-run-row-live");
    run.copy_export.formats.retain(|f| f != "markdown");
    assert!(!run.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut panel = row("a11y:artifact-lineage-panel-stale");
    panel.lineage_preserved = false;
    assert!(!panel.preserves_lineage_continuity());
    assert_eq!(
        panel.status(),
        ExperimentComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut panel = row("a11y:artifact-lineage-panel-stale");
    if let Some(narrow) = panel.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!panel.preserves_lineage_continuity());
    assert!(!panel.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut card = row("a11y:environment-fingerprint-card-partial");
    card.narrowing_disclosures.clear();
    assert!(!card.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut card = row("a11y:environment-fingerprint-card-partial");
    card.narrowing_disclosures[0].state =
        ExperimentComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!card.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut card = row("a11y:environment-fingerprint-card-partial");
    card.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!card.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut run = row("a11y:experiment-run-row-live");
    run.required_labels
        .retain(|l| *l != M5ExperimentRequiredLabel::Identity);
    assert!(!run.preserves_mandatory_labels());
    assert_eq!(
        run.status(),
        ExperimentComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_experiment_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.component_family != M5ExperimentComponentFamily::ResultSummaryCard);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExperimentComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_experiment_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5ExperimentConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExperimentComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_experiment_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExperimentComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_experiment_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExperimentComponentAccessibilityViolation::SummaryMismatch
    )));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_experiment_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExperimentComponentAccessibilityViolation::RawExperimentMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:environment-fingerprint-card-partial").chip_tokens();
    assert!(chip.contains("family=environment_fingerprint_card"));
    assert!(chip.contains("effective_claim=partial_fingerprint_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        EXPERIMENT_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_experiment_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(
        packet,
        seeded_m5_experiment_component_a11y_fallback_packet()
    );
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_experiment_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-experiment-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_experiment_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-experiment-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_EXPERIMENT_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-notebook generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_EXPERIMENT_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_experiment_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-experiment-component-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/release/m5-experiment-component-accessibility-fallback.md"),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-experiment-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
